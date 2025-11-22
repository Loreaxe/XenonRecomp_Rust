pub fn sub_82D55608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55608 size=4
    let mut pc: u32 = 0x82D55608;
    'dispatch: loop {
        match pc {
            0x82D55608 => {
    //   block [0x82D55608..0x82D5560C)
	// 82D55608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55610 size=12
    let mut pc: u32 = 0x82D55610;
    'dispatch: loop {
        match pc {
            0x82D55610 => {
    //   block [0x82D55610..0x82D5561C)
	// 82D55610: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D55614: 386B39E0  addi r3, r11, 0x39e0
	ctx.r[3].s64 = ctx.r[11].s64 + 14816;
	// 82D55618: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D55620 size=152
    let mut pc: u32 = 0x82D55620;
    'dispatch: loop {
        match pc {
            0x82D55620 => {
    //   block [0x82D55620..0x82D556B8)
	// 82D55620: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D55624: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D55628: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5562C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D55630: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D55634: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D55638: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D5563C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D55640: 48007579  bl 0x82d5cbb8
	ctx.lr = 0x82D55644;
	sub_82D5CBB8(ctx, base);
	// 82D55644: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D55648: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82D5564C: 419A0020  beq cr6, 0x82d5566c
	if ctx.cr[6].eq {
	pc = 0x82D5566C; continue 'dispatch;
	}
	// 82D55650: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D55654: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D55658: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5565C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D55660: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D55664: 4E800421  bctrl
	ctx.lr = 0x82D55668;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D55668: 48000024  b 0x82d5568c
	pc = 0x82D5568C; continue 'dispatch;
	// 82D5566C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D55670: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82D55674: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82D55678: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D5567C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D55680: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55684: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D55688: 4E800421  bctrl
	ctx.lr = 0x82D5568C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5568C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D55690: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D55694: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D55698: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5569C: 4E800421  bctrl
	ctx.lr = 0x82D556A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D556A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D556A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D556A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D556AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D556B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D556B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D556B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D556B8 size=20
    let mut pc: u32 = 0x82D556B8;
    'dispatch: loop {
        match pc {
            0x82D556B8 => {
    //   block [0x82D556B8..0x82D556CC)
	// 82D556B8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82D556BC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82D556C0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D556C4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82D556C8: 480073F0  b 0x82d5cab8
	sub_82D5CAB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D556D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D556D0 size=128
    let mut pc: u32 = 0x82D556D0;
    'dispatch: loop {
        match pc {
            0x82D556D0 => {
    //   block [0x82D556D0..0x82D55750)
	// 82D556D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D556D4: 4BF53D35  bl 0x82ca9408
	ctx.lr = 0x82D556D8;
	sub_82CA93D0(ctx, base);
	// 82D556D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D556DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D556E0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D556E4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D556E8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D556EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D556F0: 40990034  ble cr6, 0x82d55724
	if !ctx.cr[6].gt {
	pc = 0x82D55724; continue 'dispatch;
	}
	// 82D556F4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D556F8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D556FC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D55700: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D55704: 480032D5  bl 0x82d589d8
	ctx.lr = 0x82D55708;
	sub_82D589D8(ctx, base);
	// 82D55708: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5570C: 419A0024  beq cr6, 0x82d55730
	if ctx.cr[6].eq {
	pc = 0x82D55730; continue 'dispatch;
	}
	// 82D55710: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55714: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82D55718: 3BBD000C  addi r29, r29, 0xc
	ctx.r[29].s64 = ctx.r[29].s64 + 12;
	// 82D5571C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D55720: 4198FFD8  blt cr6, 0x82d556f8
	if ctx.cr[6].lt {
	pc = 0x82D556F8; continue 'dispatch;
	}
	// 82D55724: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D55728: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5572C: 4BF53D2C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D55730: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D55734: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D55738: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82D5573C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D55740: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D55744: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82D55748: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5574C: 4BF53D0C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55750 size=8
    let mut pc: u32 = 0x82D55750;
    'dispatch: loop {
        match pc {
            0x82D55750 => {
    //   block [0x82D55750..0x82D55758)
	// 82D55750: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D55754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55758 size=8
    let mut pc: u32 = 0x82D55758;
    'dispatch: loop {
        match pc {
            0x82D55758 => {
    //   block [0x82D55758..0x82D55760)
	// 82D55758: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5575C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55760 size=8
    let mut pc: u32 = 0x82D55760;
    'dispatch: loop {
        match pc {
            0x82D55760 => {
    //   block [0x82D55760..0x82D55768)
	// 82D55760: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55768 size=16
    let mut pc: u32 = 0x82D55768;
    'dispatch: loop {
        match pc {
            0x82D55768 => {
    //   block [0x82D55768..0x82D55778)
	// 82D55768: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5576C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D55770: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D55774: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55778 size=20
    let mut pc: u32 = 0x82D55778;
    'dispatch: loop {
        match pc {
            0x82D55778 => {
    //   block [0x82D55778..0x82D5578C)
	// 82D55778: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5577C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82D55780: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D55784: 409AFFF4  bne cr6, 0x82d55778
	if !ctx.cr[6].eq {
	pc = 0x82D55778; continue 'dispatch;
	}
	// 82D55788: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55790 size=40
    let mut pc: u32 = 0x82D55790;
    'dispatch: loop {
        match pc {
            0x82D55790 => {
    //   block [0x82D55790..0x82D557B8)
	// 82D55790: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82D55794: 419A0018  beq cr6, 0x82d557ac
	if ctx.cr[6].eq {
	pc = 0x82D557AC; continue 'dispatch;
	}
	// 82D55798: 7F052040  cmplw cr6, r5, r4
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82D5579C: 419A001C  beq cr6, 0x82d557b8
	if ctx.cr[6].eq {
		sub_82D557B8(ctx, base);
		return;
	}
	// 82D557A0: 80A50004  lwz r5, 4(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D557A4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82D557A8: 409AFFF0  bne cr6, 0x82d55798
	if !ctx.cr[6].eq {
	pc = 0x82D55798; continue 'dispatch;
	}
	// 82D557AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D557B0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D557B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D557B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D557B8 size=12
    let mut pc: u32 = 0x82D557B8;
    'dispatch: loop {
        match pc {
            0x82D557B8 => {
    //   block [0x82D557B8..0x82D557C4)
	// 82D557B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D557BC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D557C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D557C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D557C8 size=16
    let mut pc: u32 = 0x82D557C8;
    'dispatch: loop {
        match pc {
            0x82D557C8 => {
    //   block [0x82D557C8..0x82D557D8)
	// 82D557C8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D557CC: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D557D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D557D4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D557D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D557D8 size=24
    let mut pc: u32 = 0x82D557D8;
    'dispatch: loop {
        match pc {
            0x82D557D8 => {
    //   block [0x82D557D8..0x82D557F0)
	// 82D557D8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D557DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D557E0: 7C6A1A14  add r3, r10, r3
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82D557E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D557E8: 409AFFF0  bne cr6, 0x82d557d8
	if !ctx.cr[6].eq {
	pc = 0x82D557D8; continue 'dispatch;
	}
	// 82D557EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D557F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D557F0 size=8
    let mut pc: u32 = 0x82D557F0;
    'dispatch: loop {
        match pc {
            0x82D557F0 => {
    //   block [0x82D557F0..0x82D557F8)
	// 82D557F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D557F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D557F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D557F8 size=8
    let mut pc: u32 = 0x82D557F8;
    'dispatch: loop {
        match pc {
            0x82D557F8 => {
    //   block [0x82D557F8..0x82D55800)
	// 82D557F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D557FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55800 size=8
    let mut pc: u32 = 0x82D55800;
    'dispatch: loop {
        match pc {
            0x82D55800 => {
    //   block [0x82D55800..0x82D55808)
	// 82D55800: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D55804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55808 size=16
    let mut pc: u32 = 0x82D55808;
    'dispatch: loop {
        match pc {
            0x82D55808 => {
    //   block [0x82D55808..0x82D55818)
	// 82D55808: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5580C: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D55810: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D55814: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55818 size=24
    let mut pc: u32 = 0x82D55818;
    'dispatch: loop {
        match pc {
            0x82D55818 => {
    //   block [0x82D55818..0x82D55830)
	// 82D55818: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D5581C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55820: 7C6A1A14  add r3, r10, r3
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82D55824: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D55828: 409AFFF0  bne cr6, 0x82d55818
	if !ctx.cr[6].eq {
	pc = 0x82D55818; continue 'dispatch;
	}
	// 82D5582C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55830 size=80
    let mut pc: u32 = 0x82D55830;
    'dispatch: loop {
        match pc {
            0x82D55830 => {
    //   block [0x82D55830..0x82D55880)
	// 82D55830: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55834: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D55838: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5583C: 419A0018  beq cr6, 0x82d55854
	if ctx.cr[6].eq {
	pc = 0x82D55854; continue 'dispatch;
	}
	// 82D55840: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D55844: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55848: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D5584C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D55850: 409AFFF0  bne cr6, 0x82d55840
	if !ctx.cr[6].eq {
	pc = 0x82D55840; continue 'dispatch;
	}
	// 82D55854: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82D55858: 7D692050  subf r11, r9, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 82D5585C: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D55860: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82D55864: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D55868: 40980018  bge cr6, 0x82d55880
	if !ctx.cr[6].lt {
		sub_82D55880(ctx, base);
		return;
	}
	// 82D5586C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55870: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D55874: 409AFFE8  bne cr6, 0x82d5585c
	if !ctx.cr[6].eq {
	pc = 0x82D5585C; continue 'dispatch;
	}
	// 82D55878: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5587C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55880 size=24
    let mut pc: u32 = 0x82D55880;
    'dispatch: loop {
        match pc {
            0x82D55880 => {
    //   block [0x82D55880..0x82D55898)
	// 82D55880: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D55884: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D55888: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D5588C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D55890: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D55894: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D55898 size=200
    let mut pc: u32 = 0x82D55898;
    'dispatch: loop {
        match pc {
            0x82D55898 => {
    //   block [0x82D55898..0x82D55960)
	// 82D55898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5589C: 4BF53B6D  bl 0x82ca9408
	ctx.lr = 0x82D558A0;
	sub_82CA93D0(ctx, base);
	// 82D558A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D558A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D558A8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D558AC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D558B0: 4BFFFF59  bl 0x82d55808
	ctx.lr = 0x82D558B4;
	sub_82D55808(ctx, base);
	// 82D558B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D558B8: 40990078  ble cr6, 0x82d55930
	if !ctx.cr[6].gt {
	pc = 0x82D55930; continue 'dispatch;
	}
	// 82D558BC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D558C0: 813E0014  lwz r9, 0x14(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D558C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D558C8: 419A0018  beq cr6, 0x82d558e0
	if ctx.cr[6].eq {
	pc = 0x82D558E0; continue 'dispatch;
	}
	// 82D558CC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D558D0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D558D4: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D558D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D558DC: 409AFFF0  bne cr6, 0x82d558cc
	if !ctx.cr[6].eq {
	pc = 0x82D558CC; continue 'dispatch;
	}
	// 82D558E0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82D558E4: 7D69E850  subf r11, r9, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[9].s64;
	// 82D558E8: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D558EC: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82D558F0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D558F4: 40980048  bge cr6, 0x82d5593c
	if !ctx.cr[6].lt {
	pc = 0x82D5593C; continue 'dispatch;
	}
	// 82D558F8: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D558FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D55900: 409AFFE8  bne cr6, 0x82d558e8
	if !ctx.cr[6].eq {
	pc = 0x82D558E8; continue 'dispatch;
	}
	// 82D55904: 83FE0010  lwz r31, 0x10(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D55908: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D5590C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D55910: 480030C9  bl 0x82d589d8
	ctx.lr = 0x82D55914;
	sub_82D589D8(ctx, base);
	// 82D55914: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D55918: 419A003C  beq cr6, 0x82d55954
	if ctx.cr[6].eq {
	pc = 0x82D55954; continue 'dispatch;
	}
	// 82D5591C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D55920: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82D55924: 4BFFFEE5  bl 0x82d55808
	ctx.lr = 0x82D55928;
	sub_82D55808(ctx, base);
	// 82D55928: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82D5592C: 4198FF90  blt cr6, 0x82d558bc
	if ctx.cr[6].lt {
	pc = 0x82D558BC; continue 'dispatch;
	}
	// 82D55930: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D55934: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D55938: 4BF53B20  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D5593C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D55940: 814A0010  lwz r10, 0x10(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D55944: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D55948: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5594C: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D55950: 4BFFFFB8  b 0x82d55908
	pc = 0x82D55908; continue 'dispatch;
	// 82D55954: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D55958: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5595C: 4BF53AFC  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55960 size=16
    let mut pc: u32 = 0x82D55960;
    'dispatch: loop {
        match pc {
            0x82D55960 => {
    //   block [0x82D55960..0x82D55970)
	// 82D55960: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55964: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D55968: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5596C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55970 size=24
    let mut pc: u32 = 0x82D55970;
    'dispatch: loop {
        match pc {
            0x82D55970 => {
    //   block [0x82D55970..0x82D55988)
	// 82D55970: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D55974: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55978: 7C6A1A14  add r3, r10, r3
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82D5597C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D55980: 409AFFF0  bne cr6, 0x82d55970
	if !ctx.cr[6].eq {
	pc = 0x82D55970; continue 'dispatch;
	}
	// 82D55984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55988 size=80
    let mut pc: u32 = 0x82D55988;
    'dispatch: loop {
        match pc {
            0x82D55988 => {
    //   block [0x82D55988..0x82D559D8)
	// 82D55988: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5598C: 8123001C  lwz r9, 0x1c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D55990: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D55994: 419A0018  beq cr6, 0x82d559ac
	if ctx.cr[6].eq {
	pc = 0x82D559AC; continue 'dispatch;
	}
	// 82D55998: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D5599C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D559A0: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D559A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D559A8: 409AFFF0  bne cr6, 0x82d55998
	if !ctx.cr[6].eq {
	pc = 0x82D55998; continue 'dispatch;
	}
	// 82D559AC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82D559B0: 7D692050  subf r11, r9, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 82D559B4: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D559B8: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82D559BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D559C0: 40980018  bge cr6, 0x82d559d8
	if !ctx.cr[6].lt {
		sub_82D559D8(ctx, base);
		return;
	}
	// 82D559C4: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D559C8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D559CC: 409AFFE8  bne cr6, 0x82d559b4
	if !ctx.cr[6].eq {
	pc = 0x82D559B4; continue 'dispatch;
	}
	// 82D559D0: 80630018  lwz r3, 0x18(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D559D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D559D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D559D8 size=24
    let mut pc: u32 = 0x82D559D8;
    'dispatch: loop {
        match pc {
            0x82D559D8 => {
    //   block [0x82D559D8..0x82D559F0)
	// 82D559D8: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D559DC: 814A0018  lwz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D559E0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D559E4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D559E8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D559EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D559F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D559F0 size=4
    let mut pc: u32 = 0x82D559F0;
    'dispatch: loop {
        match pc {
            0x82D559F0 => {
    //   block [0x82D559F0..0x82D559F4)
	// 82D559F0: 4BFFFF98  b 0x82d55988
	sub_82D55988(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D559F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D559F8 size=8
    let mut pc: u32 = 0x82D559F8;
    'dispatch: loop {
        match pc {
            0x82D559F8 => {
    //   block [0x82D559F8..0x82D55A00)
	// 82D559F8: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D559FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55A00 size=24
    let mut pc: u32 = 0x82D55A00;
    'dispatch: loop {
        match pc {
            0x82D55A00 => {
    //   block [0x82D55A00..0x82D55A18)
	// 82D55A00: 548B083C  slwi r11, r4, 1
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D55A04: 81430018  lwz r10, 0x18(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D55A08: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 82D55A0C: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D55A10: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D55A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D55A18 size=200
    let mut pc: u32 = 0x82D55A18;
    'dispatch: loop {
        match pc {
            0x82D55A18 => {
    //   block [0x82D55A18..0x82D55AE0)
	// 82D55A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D55A1C: 4BF539ED  bl 0x82ca9408
	ctx.lr = 0x82D55A20;
	sub_82CA93D0(ctx, base);
	// 82D55A20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D55A24: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D55A28: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D55A2C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D55A30: 4BFFFF31  bl 0x82d55960
	ctx.lr = 0x82D55A34;
	sub_82D55960(ctx, base);
	// 82D55A34: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D55A38: 40990078  ble cr6, 0x82d55ab0
	if !ctx.cr[6].gt {
	pc = 0x82D55AB0; continue 'dispatch;
	}
	// 82D55A3C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55A40: 813E001C  lwz r9, 0x1c(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D55A44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D55A48: 419A0018  beq cr6, 0x82d55a60
	if ctx.cr[6].eq {
	pc = 0x82D55A60; continue 'dispatch;
	}
	// 82D55A4C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D55A50: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55A54: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82D55A58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D55A5C: 409AFFF0  bne cr6, 0x82d55a4c
	if !ctx.cr[6].eq {
	pc = 0x82D55A4C; continue 'dispatch;
	}
	// 82D55A60: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 82D55A64: 7D69E850  subf r11, r9, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[9].s64;
	// 82D55A68: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D55A6C: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82D55A70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D55A74: 40980048  bge cr6, 0x82d55abc
	if !ctx.cr[6].lt {
	pc = 0x82D55ABC; continue 'dispatch;
	}
	// 82D55A78: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55A7C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D55A80: 409AFFE8  bne cr6, 0x82d55a68
	if !ctx.cr[6].eq {
	pc = 0x82D55A68; continue 'dispatch;
	}
	// 82D55A84: 83FE0018  lwz r31, 0x18(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D55A88: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D55A8C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D55A90: 48002F49  bl 0x82d589d8
	ctx.lr = 0x82D55A94;
	sub_82D589D8(ctx, base);
	// 82D55A94: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D55A98: 419A003C  beq cr6, 0x82d55ad4
	if ctx.cr[6].eq {
	pc = 0x82D55AD4; continue 'dispatch;
	}
	// 82D55A9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D55AA0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82D55AA4: 4BFFFEBD  bl 0x82d55960
	ctx.lr = 0x82D55AA8;
	sub_82D55960(ctx, base);
	// 82D55AA8: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82D55AAC: 4198FF90  blt cr6, 0x82d55a3c
	if ctx.cr[6].lt {
	pc = 0x82D55A3C; continue 'dispatch;
	}
	// 82D55AB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D55AB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D55AB8: 4BF539A0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D55ABC: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D55AC0: 814A0018  lwz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D55AC4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D55AC8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D55ACC: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D55AD0: 4BFFFFB8  b 0x82d55a88
	pc = 0x82D55A88; continue 'dispatch;
	// 82D55AD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D55AD8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D55ADC: 4BF5397C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55AE0 size=8
    let mut pc: u32 = 0x82D55AE0;
    'dispatch: loop {
        match pc {
            0x82D55AE0 => {
    //   block [0x82D55AE0..0x82D55AE8)
	// 82D55AE0: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D55AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55AE8 size=8
    let mut pc: u32 = 0x82D55AE8;
    'dispatch: loop {
        match pc {
            0x82D55AE8 => {
    //   block [0x82D55AE8..0x82D55AF0)
	// 82D55AE8: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82D55AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55AF0 size=56
    let mut pc: u32 = 0x82D55AF0;
    'dispatch: loop {
        match pc {
            0x82D55AF0 => {
    //   block [0x82D55AF0..0x82D55B28)
	// 82D55AF0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82D55AF4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55AF8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D55AFC: 419A0014  beq cr6, 0x82d55b10
	if ctx.cr[6].eq {
	pc = 0x82D55B10; continue 'dispatch;
	}
	// 82D55B00: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82D55B04: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55B08: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D55B0C: 409AFFF4  bne cr6, 0x82d55b00
	if !ctx.cr[6].eq {
	pc = 0x82D55B00; continue 'dispatch;
	}
	// 82D55B10: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D55B14: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D55B18: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D55B1C: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 82D55B20: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D55B24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55B28 size=80
    let mut pc: u32 = 0x82D55B28;
    'dispatch: loop {
        match pc {
            0x82D55B28 => {
    //   block [0x82D55B28..0x82D55B78)
	// 82D55B28: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82D55B2C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55B30: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D55B34: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D55B38: 419A0018  beq cr6, 0x82d55b50
	if ctx.cr[6].eq {
	pc = 0x82D55B50; continue 'dispatch;
	}
	// 82D55B3C: 810B001C  lwz r8, 0x1c(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D55B40: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55B44: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 82D55B48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D55B4C: 409AFFF0  bne cr6, 0x82d55b3c
	if !ctx.cr[6].eq {
	pc = 0x82D55B3C; continue 'dispatch;
	}
	// 82D55B50: 7D692050  subf r11, r9, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[9].s64;
	// 82D55B54: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D55B58: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82D55B5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D55B60: 40980018  bge cr6, 0x82d55b78
	if !ctx.cr[6].lt {
		sub_82D55B78(ctx, base);
		return;
	}
	// 82D55B64: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55B68: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D55B6C: 409AFFE8  bne cr6, 0x82d55b54
	if !ctx.cr[6].eq {
	pc = 0x82D55B54; continue 'dispatch;
	}
	// 82D55B70: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D55B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55B78 size=68
    let mut pc: u32 = 0x82D55B78;
    'dispatch: loop {
        match pc {
            0x82D55B78 => {
    //   block [0x82D55B78..0x82D55BBC)
	// 82D55B78: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D55B7C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D55B80: 419AFFF0  beq cr6, 0x82d55b70
	if ctx.cr[6].eq {
		sub_82D55B28(ctx, base);
		return;
	}
	// 82D55B84: 5568103A  slwi r8, r11, 2
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D55B88: 7D08482E  lwzx r8, r8, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D55B8C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82D55B90: 4198FFE0  blt cr6, 0x82d55b70
	if ctx.cr[6].lt {
		sub_82D55B28(ctx, base);
		return;
	}
	// 82D55B94: 7D294214  add r9, r9, r8
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[8].u64;
	// 82D55B98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D55B9C: 91250000  stw r9, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D55BA0: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D55BA4: 814A0018  lwz r10, 0x18(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D55BA8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D55BAC: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D55BB0: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D55BB4: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D55BB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D55BC0 size=88
    let mut pc: u32 = 0x82D55BC0;
    'dispatch: loop {
        match pc {
            0x82D55BC0 => {
    //   block [0x82D55BC0..0x82D55C18)
	// 82D55BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D55BC4: 4BF53849  bl 0x82ca940c
	ctx.lr = 0x82D55BC8;
	sub_82CA93D0(ctx, base);
	// 82D55BC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D55BCC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D55BD0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82D55BD4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82D55BD8: 4BFFFF51  bl 0x82d55b28
	ctx.lr = 0x82D55BDC;
	sub_82D55B28(ctx, base);
	// 82D55BDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D55BE0: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D55BE4: 409A0028  bne cr6, 0x82d55c0c
	if !ctx.cr[6].eq {
	pc = 0x82D55C0C; continue 'dispatch;
	}
	// 82D55BE8: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D55BEC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D55BF0: 480071B1  bl 0x82d5cda0
	ctx.lr = 0x82D55BF4;
	sub_82D5CDA0(ctx, base);
	// 82D55BF4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D55BF8: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D55BFC: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D55C00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D55C04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D55C08: 4E800421  bctrl
	ctx.lr = 0x82D55C0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D55C0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D55C10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D55C14: 4BF53848  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55C18 size=16
    let mut pc: u32 = 0x82D55C18;
    'dispatch: loop {
        match pc {
            0x82D55C18 => {
    //   block [0x82D55C18..0x82D55C28)
	// 82D55C18: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D55C1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D55C20: 419A0008  beq cr6, 0x82d55c28
	if ctx.cr[6].eq {
		sub_82D55C28(ctx, base);
		return;
	}
	// 82D55C24: 4BFFFAAC  b 0x82d556d0
	sub_82D556D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55C28 size=8
    let mut pc: u32 = 0x82D55C28;
    'dispatch: loop {
        match pc {
            0x82D55C28 => {
    //   block [0x82D55C28..0x82D55C30)
	// 82D55C28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D55C2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55C30 size=8
    let mut pc: u32 = 0x82D55C30;
    'dispatch: loop {
        match pc {
            0x82D55C30 => {
    //   block [0x82D55C30..0x82D55C38)
	// 82D55C30: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 82D55C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55C38 size=8
    let mut pc: u32 = 0x82D55C38;
    'dispatch: loop {
        match pc {
            0x82D55C38 => {
    //   block [0x82D55C38..0x82D55C40)
	// 82D55C38: 38630028  addi r3, r3, 0x28
	ctx.r[3].s64 = ctx.r[3].s64 + 40;
	// 82D55C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D55C40 size=76
    let mut pc: u32 = 0x82D55C40;
    'dispatch: loop {
        match pc {
            0x82D55C40 => {
    //   block [0x82D55C40..0x82D55C8C)
	// 82D55C40: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82D55C44: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82D55C48: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D55C4C: 80E1005C  lwz r7, 0x5c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82D55C50: 80C10064  lwz r6, 0x64(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82D55C54: 8101006C  lwz r8, 0x6c(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82D55C58: 91230010  stw r9, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82D55C5C: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82D55C60: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82D55C64: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D55C68: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82D55C6C: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82D55C70: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82D55C74: 90E3001C  stw r7, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[7].u32 ) };
	// 82D55C78: 90C30020  stw r6, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 82D55C7C: 91030024  stw r8, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[8].u32 ) };
	// 82D55C80: 91230028  stw r9, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 82D55C84: 9143002C  stw r10, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 82D55C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D55C90 size=184
    let mut pc: u32 = 0x82D55C90;
    'dispatch: loop {
        match pc {
            0x82D55C90 => {
    //   block [0x82D55C90..0x82D55D48)
	// 82D55C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D55C94: 4BF53775  bl 0x82ca9408
	ctx.lr = 0x82D55C98;
	sub_82CA93D0(ctx, base);
	// 82D55C98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D55C9C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82D55CA0: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82D55CA4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82D55CA8: 4BFFFE81  bl 0x82d55b28
	ctx.lr = 0x82D55CAC;
	sub_82D55B28(ctx, base);
	// 82D55CAC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D55CB0: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82D55CB4: 409A0034  bne cr6, 0x82d55ce8
	if !ctx.cr[6].eq {
	pc = 0x82D55CE8; continue 'dispatch;
	}
	// 82D55CB8: 83C10054  lwz r30, 0x54(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D55CBC: 88BE000C  lbz r5, 0xc(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D55CC0: 2B050014  cmplwi cr6, r5, 0x14
	ctx.cr[6].compare_u32(ctx.r[5].u32, 20 as u32, &mut ctx.xer);
	// 82D55CC4: 409A0030  bne cr6, 0x82d55cf4
	if !ctx.cr[6].eq {
	pc = 0x82D55CF4; continue 'dispatch;
	}
	// 82D55CC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D55CCC: 48007095  bl 0x82d5cd60
	ctx.lr = 0x82D55CD0;
	sub_82D5CD60(ctx, base);
	// 82D55CD0: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D55CD4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D55CD8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82D55CDC: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82D55CE0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82D55CE4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D55CE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D55CEC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D55CF0: 4BF53768  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D55CF4: 2B050018  cmplwi cr6, r5, 0x18
	ctx.cr[6].compare_u32(ctx.r[5].u32, 24 as u32, &mut ctx.xer);
	// 82D55CF8: 409A003C  bne cr6, 0x82d55d34
	if !ctx.cr[6].eq {
	pc = 0x82D55D34; continue 'dispatch;
	}
	// 82D55CFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D55D00: 48007069  bl 0x82d5cd68
	ctx.lr = 0x82D55D04;
	sub_82D5CD68(ctx, base);
	// 82D55D04: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D55D08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D55D0C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D55D10: 48007361  bl 0x82d5d070
	ctx.lr = 0x82D55D14;
	sub_82D5D070(ctx, base);
	// 82D55D14: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D55D18: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82D55D1C: 939F0014  stw r28, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[28].u32 ) };
	// 82D55D20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D55D24: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82D55D28: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D55D2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D55D30: 4BF53728  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D55D34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D55D38: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D55D3C: 480002E5  bl 0x82d56020
	ctx.lr = 0x82D55D40;
	sub_82D56020(ctx, base);
	// 82D55D40: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D55D44: 4BF53714  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D55D48 size=588
    let mut pc: u32 = 0x82D55D48;
    'dispatch: loop {
        match pc {
            0x82D55D48 => {
    //   block [0x82D55D48..0x82D55F94)
	// 82D55D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D55D4C: 4BF536A5  bl 0x82ca93f0
	ctx.lr = 0x82D55D50;
	sub_82CA93D0(ctx, base);
	// 82D55D50: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D55D54: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D55D58: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D55D5C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82D55D60: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D55D64: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82D55D68: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82D55D6C: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D55D70: 480081A1  bl 0x82d5df10
	ctx.lr = 0x82D55D74;
	sub_82D5DF10(ctx, base);
	// 82D55D74: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D55D78: 809A000C  lwz r4, 0xc(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D55D7C: 4800841D  bl 0x82d5e198
	ctx.lr = 0x82D55D80;
	sub_82D5E198(ctx, base);
	// 82D55D80: 817A0014  lwz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D55D84: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D55D88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D55D8C: 4099002C  ble cr6, 0x82d55db8
	if !ctx.cr[6].gt {
	pc = 0x82D55DB8; continue 'dispatch;
	}
	// 82D55D90: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82D55D94: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D55D98: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82D55D9C: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82D55DA0: 48008841  bl 0x82d5e5e0
	ctx.lr = 0x82D55DA4;
	sub_82D5E5E0(ctx, base);
	// 82D55DA4: 817A0014  lwz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D55DA8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82D55DAC: 3BDE0014  addi r30, r30, 0x14
	ctx.r[30].s64 = ctx.r[30].s64 + 20;
	// 82D55DB0: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D55DB4: 4198FFE0  blt cr6, 0x82d55d94
	if ctx.cr[6].lt {
	pc = 0x82D55D94; continue 'dispatch;
	}
	// 82D55DB8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D55DBC: 809A0014  lwz r4, 0x14(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D55DC0: 480083D9  bl 0x82d5e198
	ctx.lr = 0x82D55DC4;
	sub_82D5E198(ctx, base);
	// 82D55DC4: 817A001C  lwz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D55DC8: 3AC00000  li r22, 0
	ctx.r[22].s64 = 0;
	// 82D55DCC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D55DD0: 409901A8  ble cr6, 0x82d55f78
	if !ctx.cr[6].gt {
	pc = 0x82D55F78; continue 'dispatch;
	}
	// 82D55DD4: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82D55DD8: 817A0018  lwz r11, 0x18(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D55DDC: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 82D55DE0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 82D55DE4: 7F6BC214  add r27, r11, r24
	ctx.r[27].u64 = ctx.r[11].u64 + ctx.r[24].u64;
	// 82D55DE8: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82D55DEC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82D55DF0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D55DF4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D55DF8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82D55DFC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D55E00: 4200FFF0  bdnz 0x82d55df0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82D55DF0; continue 'dispatch;
	}
	// 82D55E04: 8B21007C  lbz r25, 0x7c(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 82D55E08: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82D55E0C: 2B0B0018  cmplwi cr6, r11, 0x18
	ctx.cr[6].compare_u32(ctx.r[11].u32, 24 as u32, &mut ctx.xer);
	// 82D55E10: 419A000C  beq cr6, 0x82d55e1c
	if ctx.cr[6].eq {
	pc = 0x82D55E1C; continue 'dispatch;
	}
	// 82D55E14: 2B0B001F  cmplwi cr6, r11, 0x1f
	ctx.cr[6].compare_u32(ctx.r[11].u32, 31 as u32, &mut ctx.xer);
	// 82D55E18: 409A0034  bne cr6, 0x82d55e4c
	if !ctx.cr[6].eq {
	pc = 0x82D55E4C; continue 'dispatch;
	}
	// 82D55E1C: 8861007D  lbz r3, 0x7d(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(125 as u32) ) } as u64;
	// 82D55E20: 48006F19  bl 0x82d5cd38
	ctx.lr = 0x82D55E24;
	sub_82D5CD38(ctx, base);
	// 82D55E24: A1430008  lhz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D55E28: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82D55E2C: A1610080  lhz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D55E30: 7D4A0734  extsh r10, r10
	ctx.r[10].s64 = ctx.r[10].s16 as i64;
	// 82D55E34: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D55E38: 9BC1007D  stb r30, 0x7d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(125 as u32), ctx.r[30].u8 ) };
	// 82D55E3C: 7D4B5A78  xor r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 ^ ctx.r[11].u64;
	// 82D55E40: 557C043E  clrlwi r28, r11, 0x10
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82D55E44: B3810080  sth r28, 0x80(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u16 ) };
	// 82D55E48: 4800000C  b 0x82d55e54
	pc = 0x82D55E54; continue 'dispatch;
	// 82D55E4C: A3810080  lhz r28, 0x80(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D55E50: 8BC1007D  lbz r30, 0x7d(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(125 as u32) ) } as u64;
	// 82D55E54: 578B043E  clrlwi r11, r28, 0x10
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x0000FFFFu64;
	// 82D55E58: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D55E5C: 556A056A  rlwinm r10, r11, 0, 0x15, 0x15
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D55E60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D55E64: 419A0030  beq cr6, 0x82d55e94
	if ctx.cr[6].eq {
	pc = 0x82D55E94; continue 'dispatch;
	}
	// 82D55E68: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82D55E6C: 57CA063E  clrlwi r10, r30, 0x18
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82D55E70: 697C0400  xori r28, r11, 0x400
	ctx.r[28].u64 = ctx.r[11].u64 ^ 1024;
	// 82D55E74: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D55E78: B3810080  sth r28, 0x80(r1)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[28].u16 ) };
	// 82D55E7C: 419A0008  beq cr6, 0x82d55e84
	if ctx.cr[6].eq {
	pc = 0x82D55E84; continue 'dispatch;
	}
	// 82D55E80: 57DD063E  clrlwi r29, r30, 0x18
	ctx.r[29].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82D55E84: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 82D55E88: 3B200013  li r25, 0x13
	ctx.r[25].s64 = 19;
	// 82D55E8C: 9BC1007D  stb r30, 0x7d(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(125 as u32), ctx.r[30].u8 ) };
	// 82D55E90: 9B21007C  stb r25, 0x7c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[25].u8 ) };
	// 82D55E94: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82D55E98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D55E9C: 419A0048  beq cr6, 0x82d55ee4
	if ctx.cr[6].eq {
	pc = 0x82D55EE4; continue 'dispatch;
	}
	// 82D55EA0: 897B000C  lbz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D55EA4: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 82D55EA8: 419A003C  beq cr6, 0x82d55ee4
	if ctx.cr[6].eq {
	pc = 0x82D55EE4; continue 'dispatch;
	}
	// 82D55EAC: 897B000D  lbz r11, 0xd(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(13 as u32) ) } as u64;
	// 82D55EB0: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 82D55EB4: 419A0030  beq cr6, 0x82d55ee4
	if ctx.cr[6].eq {
	pc = 0x82D55EE4; continue 'dispatch;
	}
	// 82D55EB8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82D55EBC: 48006EA5  bl 0x82d5cd60
	ctx.lr = 0x82D55EC0;
	sub_82D5CD60(ctx, base);
	// 82D55EC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D55EC4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D55EC8: 419A001C  beq cr6, 0x82d55ee4
	if ctx.cr[6].eq {
	pc = 0x82D55EE4; continue 'dispatch;
	}
	// 82D55ECC: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82D55ED0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D55ED4: 4BFFFE75  bl 0x82d55d48
	ctx.lr = 0x82D55ED8;
	sub_82D55D48(ctx, base);
	// 82D55ED8: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55EDC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D55EE0: 409AFFEC  bne cr6, 0x82d55ecc
	if !ctx.cr[6].eq {
	pc = 0x82D55ECC; continue 'dispatch;
	}
	// 82D55EE4: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82D55EE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D55EEC: 419A0014  beq cr6, 0x82d55f00
	if ctx.cr[6].eq {
	pc = 0x82D55F00; continue 'dispatch;
	}
	// 82D55EF0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82D55EF4: 48006E75  bl 0x82d5cd68
	ctx.lr = 0x82D55EF8;
	sub_82D5CD68(ctx, base);
	// 82D55EF8: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82D55EFC: 480086E5  bl 0x82d5e5e0
	ctx.lr = 0x82D55F00;
	sub_82D5E5E0(ctx, base);
	// 82D55F00: 83E10070  lwz r31, 0x70(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D55F04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D55F08: 48002D21  bl 0x82d58c28
	ctx.lr = 0x82D55F0C;
	sub_82D58C28(ctx, base);
	// 82D55F0C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D55F10: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D55F14: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D55F18: 48007EF1  bl 0x82d5de08
	ctx.lr = 0x82D55F1C;
	sub_82D5DE08(ctx, base);
	// 82D55F1C: 5724063E  clrlwi r4, r25, 0x18
	ctx.r[4].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	// 82D55F20: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D55F24: 48008215  bl 0x82d5e138
	ctx.lr = 0x82D55F28;
	sub_82D5E138(ctx, base);
	// 82D55F28: 57C4063E  clrlwi r4, r30, 0x18
	ctx.r[4].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	// 82D55F2C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D55F30: 48008209  bl 0x82d5e138
	ctx.lr = 0x82D55F34;
	sub_82D5E138(ctx, base);
	// 82D55F34: 7FAB0734  extsh r11, r29
	ctx.r[11].s64 = ctx.r[29].s16 as i64;
	// 82D55F38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D55F3C: 419A0010  beq cr6, 0x82d55f4c
	if ctx.cr[6].eq {
	pc = 0x82D55F4C; continue 'dispatch;
	}
	// 82D55F40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D55F44: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D55F48: 480081F1  bl 0x82d5e138
	ctx.lr = 0x82D55F4C;
	sub_82D5E138(ctx, base);
	// 82D55F4C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D55F50: A081007E  lhz r4, 0x7e(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(126 as u32) ) } as u64;
	// 82D55F54: 480081E5  bl 0x82d5e138
	ctx.lr = 0x82D55F58;
	sub_82D5E138(ctx, base);
	// 82D55F58: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D55F5C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D55F60: 480081D9  bl 0x82d5e138
	ctx.lr = 0x82D55F64;
	sub_82D5E138(ctx, base);
	// 82D55F64: 817A001C  lwz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D55F68: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 82D55F6C: 3B180018  addi r24, r24, 0x18
	ctx.r[24].s64 = ctx.r[24].s64 + 24;
	// 82D55F70: 7F165800  cmpw cr6, r22, r11
	ctx.cr[6].compare_i32(ctx.r[22].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D55F74: 4198FE64  blt cr6, 0x82d55dd8
	if ctx.cr[6].lt {
	pc = 0x82D55DD8; continue 'dispatch;
	}
	// 82D55F78: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D55F7C: 809A001C  lwz r4, 0x1c(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D55F80: 48008219  bl 0x82d5e198
	ctx.lr = 0x82D55F84;
	sub_82D5E198(ctx, base);
	// 82D55F84: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D55F88: 48008131  bl 0x82d5e0b8
	ctx.lr = 0x82D55F8C;
	sub_82D5E0B8(ctx, base);
	// 82D55F8C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82D55F90: 4BF534B0  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D55F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D55F98 size=132
    let mut pc: u32 = 0x82D55F98;
    'dispatch: loop {
        match pc {
            0x82D55F98 => {
    //   block [0x82D55F98..0x82D5601C)
	// 82D55F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D55F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D55FA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D55FA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D55FA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D55FAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D55FB0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D55FB4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D55FB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D55FBC: 4800892D  bl 0x82d5e8e8
	ctx.lr = 0x82D55FC0;
	sub_82D5E8E8(ctx, base);
	// 82D55FC0: 7FCBF0F8  nor r11, r30, r30
	ctx.r[11].u64 = !(ctx.r[30].u64 | ctx.r[30].u64);
	// 82D55FC4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D55FC8: 556B07FE  clrlwi r11, r11, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 82D55FCC: 419A0030  beq cr6, 0x82d55ffc
	if ctx.cr[6].eq {
	pc = 0x82D55FFC; continue 'dispatch;
	}
	// 82D55FD0: 557E063E  clrlwi r30, r11, 0x18
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82D55FD4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D55FD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D55FDC: 4BFFFD6D  bl 0x82d55d48
	ctx.lr = 0x82D55FE0;
	sub_82D55D48(ctx, base);
	// 82D55FE0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D55FE4: 419A000C  beq cr6, 0x82d55ff0
	if ctx.cr[6].eq {
	pc = 0x82D55FF0; continue 'dispatch;
	}
	// 82D55FE8: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D55FEC: 48000008  b 0x82d55ff4
	pc = 0x82D55FF4; continue 'dispatch;
	// 82D55FF0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82D55FF4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D55FF8: 409AFFDC  bne cr6, 0x82d55fd4
	if !ctx.cr[6].eq {
	pc = 0x82D55FD4; continue 'dispatch;
	}
	// 82D55FFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D56000: 480088D9  bl 0x82d5e8d8
	ctx.lr = 0x82D56004;
	sub_82D5E8D8(ctx, base);
	// 82D56004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D56008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5600C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D56010: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D56014: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D56018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D56020 size=96
    let mut pc: u32 = 0x82D56020;
    'dispatch: loop {
        match pc {
            0x82D56020 => {
    //   block [0x82D56020..0x82D56080)
	// 82D56020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D56024: 4BF533E9  bl 0x82ca940c
	ctx.lr = 0x82D56028;
	sub_82CA93D0(ctx, base);
	// 82D56028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5602C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D56030: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D56034: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D56038: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D5603C: 48006CFD  bl 0x82d5cd38
	ctx.lr = 0x82D56040;
	sub_82D5CD38(ctx, base);
	// 82D56040: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D56044: A14B0008  lhz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D56048: 2B0A0040  cmplwi cr6, r10, 0x40
	ctx.cr[6].compare_u32(ctx.r[10].u32, 64 as u32, &mut ctx.xer);
	// 82D5604C: 41990028  bgt cr6, 0x82d56074
	if ctx.cr[6].gt {
	pc = 0x82D56074; continue 'dispatch;
	}
	// 82D56050: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82D56054: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D56058: A16B0008  lhz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5605C: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82D56060: 7D650734  extsh r5, r11
	ctx.r[5].s64 = ctx.r[11].s16 as i64;
	// 82D56064: 48002CCD  bl 0x82d58d30
	ctx.lr = 0x82D56068;
	sub_82D58D30(ctx, base);
	// 82D56068: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5606C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D56070: 4BF533EC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82D56074: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D56078: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5607C: 4BF533E0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D56080 size=16
    let mut pc: u32 = 0x82D56080;
    'dispatch: loop {
        match pc {
            0x82D56080 => {
    //   block [0x82D56080..0x82D56090)
	// 82D56080: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82D56084: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82D56088: 80850000  lwz r4, 0(r5)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5608C: 480089CC  b 0x82d5ea58
	sub_82D5EA58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D56090 size=112
    let mut pc: u32 = 0x82D56090;
    'dispatch: loop {
        match pc {
            0x82D56090 => {
    //   block [0x82D56090..0x82D56100)
	// 82D56090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D56094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D56098: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5609C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D560A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D560A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D560A8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82D560AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D560B0: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82D560B4: 48008C45  bl 0x82d5ecf8
	ctx.lr = 0x82D560B8;
	sub_82D5ECF8(ctx, base);
	// 82D560B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D560BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D560C0: 419A0024  beq cr6, 0x82d560e4
	if ctx.cr[6].eq {
	pc = 0x82D560E4; continue 'dispatch;
	}
	// 82D560C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D560C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D560CC: 419A0010  beq cr6, 0x82d560dc
	if ctx.cr[6].eq {
	pc = 0x82D560DC; continue 'dispatch;
	}
	// 82D560D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D560D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D560D8: 4E800421  bctrl
	ctx.lr = 0x82D560DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D560DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D560E0: 48000008  b 0x82d560e8
	pc = 0x82D560E8; continue 'dispatch;
	// 82D560E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D560E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D560EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D560F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D560F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D560F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D560FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D56100 size=104
    let mut pc: u32 = 0x82D56100;
    'dispatch: loop {
        match pc {
            0x82D56100 => {
    //   block [0x82D56100..0x82D56168)
	// 82D56100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D56104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D56108: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5610C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D56110: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D56114: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D56118: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82D5611C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D56120: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82D56124: 48008BD5  bl 0x82d5ecf8
	ctx.lr = 0x82D56128;
	sub_82D5ECF8(ctx, base);
	// 82D56128: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5612C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D56130: 419A001C  beq cr6, 0x82d5614c
	if ctx.cr[6].eq {
	pc = 0x82D5614C; continue 'dispatch;
	}
	// 82D56134: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D56138: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5613C: 419A0010  beq cr6, 0x82d5614c
	if ctx.cr[6].eq {
	pc = 0x82D5614C; continue 'dispatch;
	}
	// 82D56140: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D56144: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D56148: 4E800421  bctrl
	ctx.lr = 0x82D5614C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5614C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D56150: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D56154: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D56158: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5615C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D56160: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D56164: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D56168 size=104
    let mut pc: u32 = 0x82D56168;
    'dispatch: loop {
        match pc {
            0x82D56168 => {
    //   block [0x82D56168..0x82D561D0)
	// 82D56168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5616C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D56170: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D56174: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D56178: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5617C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D56180: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82D56184: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82D56188: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D5618C: 4BFFF0BD  bl 0x82d55248
	ctx.lr = 0x82D56190;
	sub_82D55248(ctx, base);
	// 82D56190: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D56194: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D56198: 396B3A08  addi r11, r11, 0x3a08
	ctx.r[11].s64 = ctx.r[11].s64 + 14856;
	// 82D5619C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82D561A0: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D561A4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82D561A8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D561AC: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82D561B0: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82D561B4: 48008E75  bl 0x82d5f028
	ctx.lr = 0x82D561B8;
	sub_82D5F028(ctx, base);
	// 82D561B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D561BC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D561C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D561C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D561C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D561CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D561D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D561D0 size=72
    let mut pc: u32 = 0x82D561D0;
    'dispatch: loop {
        match pc {
            0x82D561D0 => {
    //   block [0x82D561D0..0x82D56218)
	// 82D561D0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82D561D4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D561D8: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82D561DC: 40990030  ble cr6, 0x82d5620c
	if !ctx.cr[6].gt {
	pc = 0x82D5620C; continue 'dispatch;
	}
	// 82D561E0: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 82D561E4: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D561E8: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D561EC: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82D561F0: 55080050  rlwinm r8, r8, 0, 1, 8
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82D561F4: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D561F8: 419A0020  beq cr6, 0x82d56218
	if ctx.cr[6].eq {
		sub_82D56218(ctx, base);
		return;
	}
	// 82D561FC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D56200: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D56204: 7F0A2800  cmpw cr6, r10, r5
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82D56208: 4198FFDC  blt cr6, 0x82d561e4
	if ctx.cr[6].lt {
	pc = 0x82D561E4; continue 'dispatch;
	}
	// 82D5620C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D56210: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D56214: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D56218 size=12
    let mut pc: u32 = 0x82D56218;
    'dispatch: loop {
        match pc {
            0x82D56218 => {
    //   block [0x82D56218..0x82D56224)
	// 82D56218: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5621C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D56220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D56228 size=64
    let mut pc: u32 = 0x82D56228;
    'dispatch: loop {
        match pc {
            0x82D56228 => {
    //   block [0x82D56228..0x82D56268)
	// 82D56228: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82D5622C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D56230: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 82D56234: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D56238: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D5623C: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82D56240: 55080050  rlwinm r8, r8, 0, 1, 8
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82D56244: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D56248: 419A0020  beq cr6, 0x82d56268
	if ctx.cr[6].eq {
		sub_82D56268(ctx, base);
		return;
	}
	// 82D5624C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D56250: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D56254: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 82D56258: 4198FFDC  blt cr6, 0x82d56234
	if ctx.cr[6].lt {
	pc = 0x82D56234; continue 'dispatch;
	}
	// 82D5625C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D56260: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D56264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D56268 size=12
    let mut pc: u32 = 0x82D56268;
    'dispatch: loop {
        match pc {
            0x82D56268 => {
    //   block [0x82D56268..0x82D56274)
	// 82D56268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5626C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D56270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D56278 size=64
    let mut pc: u32 = 0x82D56278;
    'dispatch: loop {
        match pc {
            0x82D56278 => {
    //   block [0x82D56278..0x82D562B8)
	// 82D56278: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82D5627C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D56280: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 82D56284: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D56288: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D5628C: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82D56290: 55080050  rlwinm r8, r8, 0, 1, 8
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82D56294: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D56298: 419A0020  beq cr6, 0x82d562b8
	if ctx.cr[6].eq {
		sub_82D562B8(ctx, base);
		return;
	}
	// 82D5629C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D562A0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D562A4: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82D562A8: 4198FFDC  blt cr6, 0x82d56284
	if ctx.cr[6].lt {
	pc = 0x82D56284; continue 'dispatch;
	}
	// 82D562AC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D562B0: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D562B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D562B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D562B8 size=12
    let mut pc: u32 = 0x82D562B8;
    'dispatch: loop {
        match pc {
            0x82D562B8 => {
    //   block [0x82D562B8..0x82D562C4)
	// 82D562B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D562BC: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D562C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D562C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D562C8 size=112
    let mut pc: u32 = 0x82D562C8;
    'dispatch: loop {
        match pc {
            0x82D562C8 => {
    //   block [0x82D562C8..0x82D56338)
	// 82D562C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D562CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D562D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D562D4: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82D562D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D562DC: 4BFFFF4D  bl 0x82d56228
	ctx.lr = 0x82D562E0;
	sub_82D56228(ctx, base);
	// 82D562E0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D562E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D562E8: 419A0034  beq cr6, 0x82d5631c
	if ctx.cr[6].eq {
	pc = 0x82D5631C; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D56338 size=112
    let mut pc: u32 = 0x82D56338;
    'dispatch: loop {
        match pc {
            0x82D56338 => {
    //   block [0x82D56338..0x82D563A8)
	// 82D56338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5633C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D56340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D56344: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82D56348: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5634C: 4BFFFF2D  bl 0x82d56278
	ctx.lr = 0x82D56350;
	sub_82D56278(ctx, base);
	// 82D56350: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D56354: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D56358: 419A0034  beq cr6, 0x82d5638c
	if ctx.cr[6].eq {
	pc = 0x82D5638C; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D563A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D563A8 size=68
    let mut pc: u32 = 0x82D563A8;
    'dispatch: loop {
        match pc {
            0x82D563A8 => {
    //   block [0x82D563A8..0x82D563EC)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D563F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D563F0 size=80
    let mut pc: u32 = 0x82D563F0;
    'dispatch: loop {
        match pc {
            0x82D563F0 => {
    //   block [0x82D563F0..0x82D56440)
	// 82D563F0: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D56440 size=56
    let mut pc: u32 = 0x82D56440;
    'dispatch: loop {
        match pc {
            0x82D56440 => {
    //   block [0x82D56440..0x82D56478)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D56478 size=68
    let mut pc: u32 = 0x82D56478;
    'dispatch: loop {
        match pc {
            0x82D56478 => {
    //   block [0x82D56478..0x82D564BC)
	// 82D56478: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D564C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D564C0 size=56
    let mut pc: u32 = 0x82D564C0;
    'dispatch: loop {
        match pc {
            0x82D564C0 => {
    //   block [0x82D564C0..0x82D564F8)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D564F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D564F8 size=116
    let mut pc: u32 = 0x82D564F8;
    'dispatch: loop {
        match pc {
            0x82D564F8 => {
    //   block [0x82D564F8..0x82D5656C)
	// 82D564F8: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D56570 size=144
    let mut pc: u32 = 0x82D56570;
    'dispatch: loop {
        match pc {
            0x82D56570 => {
    //   block [0x82D56570..0x82D56600)
	// 82D56570: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D56600 size=132
    let mut pc: u32 = 0x82D56600;
    'dispatch: loop {
        match pc {
            0x82D56600 => {
    //   block [0x82D56600..0x82D56684)
	// 82D56600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D56604: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D56608: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5660C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D56610: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82D56614: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D56618: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D5661C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D56620: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D56624: C00B0BFC  lfs f0, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D56628: EFE10032  fmuls f31, f1, f0
	ctx.f[31].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D5662C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82D56630: 4B4E3961  bl 0x82239f90
	ctx.lr = 0x82D56634;
	sub_82239F90(ctx, base);
	// 82D56634: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 82D56638: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D56688 size=156
    let mut pc: u32 = 0x82D56688;
    'dispatch: loop {
        match pc {
            0x82D56688 => {
    //   block [0x82D56688..0x82D56724)
	// 82D56688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5668C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D56690: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D56694: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D56698: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5669C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D566A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D566A4: 4BFFFBD5  bl 0x82d56278
	ctx.lr = 0x82D566A8;
	sub_82D56278(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D56728 size=476
    let mut pc: u32 = 0x82D56728;
    'dispatch: loop {
        match pc {
            0x82D56728 => {
    //   block [0x82D56728..0x82D56904)
	// 82D56728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5672C: 4BF52CDD  bl 0x82ca9408
	ctx.lr = 0x82D56730;
	sub_82CA93D0(ctx, base);
	// 82D56730: C1A40014  lfs f13, 0x14(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D56734: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D56738: C1840000  lfs f12, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D5673C: EC0C682A  fadds f0, f12, f13
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[13].f64) as f32) as f64;
	// 82D56740: C1440028  lfs f10, 0x28(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82D56744: C16B0C18  lfs f11, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82D56748: EC00502A  fadds f0, f0, f10
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[10].f64) as f32) as f64;
	// 82D5674C: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82D56750: 40990074  ble cr6, 0x82d567c4
	if !ctx.cr[6].gt {
	pc = 0x82D567C4; continue 'dispatch;
	}
	// 82D56754: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D56758: C1840024  lfs f12, 0x24(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D5675C: C1640020  lfs f11, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82D56760: C1AB0C14  lfs f13, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D56764: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D56768: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 82D5676C: C1A40018  lfs f13, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D56770: ED4D6028  fsubs f10, f13, f12
	ctx.f[10].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 82D56774: C1A40008  lfs f13, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D56778: ED6B6828  fsubs f11, f11, f13
	ctx.f[11].f64 = (((ctx.f[11].f64 - ctx.f[13].f64) as f32) as f64);
	// 82D5677C: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D56780: C1A40010  lfs f13, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D56784: ED2C6828  fsubs f9, f12, f13
	ctx.f[9].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 82D56788: C18B0BFC  lfs f12, 0xbfc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3068 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D5678C: EDA0002C  fsqrts f13, f0
	ctx.f[13].f64 = ((ctx.f[0].f64).sqrt() as f32) as f64;
	// 82D56790: EC0C6824  fdivs f0, f12, f13
	ctx.f[0].f64 = ((ctx.f[12].f64 / ctx.f[13].f64) as f32) as f64;
	// 82D56794: EDAD0332  fmuls f13, f13, f12
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
	// 82D56798: D1A1FFCC  stfs f13, -0x34(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-52 as u32), tmp.u32 ) };
	// 82D5679C: EDAA0032  fmuls f13, f10, f0
	ctx.f[13].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D567A0: D1A1FFC0  stfs f13, -0x40(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), tmp.u32 ) };
	// 82D567A4: EDAB0032  fmuls f13, f11, f0
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D567A8: D1A1FFC4  stfs f13, -0x3c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-60 as u32), tmp.u32 ) };
	// 82D567AC: EC090032  fmuls f0, f9, f0
	ctx.f[0].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D567B0: D001FFC8  stfs f0, -0x38(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), tmp.u32 ) };
	// 82D567B4: 3961FFC0  addi r11, r1, -0x40
	ctx.r[11].s64 = ctx.r[1].s64 + -64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D56908 size=4
    let mut pc: u32 = 0x82D56908;
    'dispatch: loop {
        match pc {
            0x82D56908 => {
    //   block [0x82D56908..0x82D5690C)
	// 82D56908: 4BFFFE20  b 0x82d56728
	sub_82D56728(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D56910 size=92
    let mut pc: u32 = 0x82D56910;
    'dispatch: loop {
        match pc {
            0x82D56910 => {
    //   block [0x82D56910..0x82D5696C)
	// 82D56910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D56914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D56918: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5691C: 4BFFFE0D  bl 0x82d56728
	ctx.lr = 0x82D56920;
	sub_82D56728(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D56970 size=344
    let mut pc: u32 = 0x82D56970;
    'dispatch: loop {
        match pc {
            0x82D56970 => {
    //   block [0x82D56970..0x82D56AC8)
	// 82D56970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D56974: 4BF52A99  bl 0x82ca940c
	ctx.lr = 0x82D56978;
	sub_82CA93D0(ctx, base);
	// 82D56978: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 82D5697C: 4BF5735D  bl 0x82cadcd8
	ctx.lr = 0x82D56980;
	sub_82CADCA0(ctx, base);
	// 82D56980: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D56984: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D56988: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82D5698C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D56990: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D56994: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D56AC8 size=220
    let mut pc: u32 = 0x82D56AC8;
    'dispatch: loop {
        match pc {
            0x82D56AC8 => {
    //   block [0x82D56AC8..0x82D56BA4)
	// 82D56AC8: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D56ACC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D56AD0: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D56AD4: FC000210  fabs f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 & !0x8000_0000_0000_0000u64;
	// 82D56AD8: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82D56ADC: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D56AE0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D56AE4: FD806210  fabs f12, f12
	ctx.f[12].u64 = ctx.f[12].u64 & !0x8000_0000_0000_0000u64;
	// 82D56AE8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82D56AEC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82D56AF0: 40980010  bge cr6, 0x82d56b00
	if !ctx.cr[6].lt {
	pc = 0x82D56B00; continue 'dispatch;
	}
	// 82D56AF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D56AF8: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 82D56AFC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D56B00: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 82D56B04: 40980008  bge cr6, 0x82d56b0c
	if !ctx.cr[6].lt {
	pc = 0x82D56B0C; continue 'dispatch;
	}
	// 82D56B08: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82D56B0C: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
	// 82D56B10: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D56B14: 38E1FFE0  addi r7, r1, -0x20
	ctx.r[7].s64 = ctx.r[1].s64 + -32;
	// 82D56B18: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D56B1C: 3901FFE0  addi r8, r1, -0x20
	ctx.r[8].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D56BA8 size=484
    let mut pc: u32 = 0x82D56BA8;
    'dispatch: loop {
        match pc {
            0x82D56BA8 => {
    //   block [0x82D56BA8..0x82D56D8C)
	// 82D56BA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D56BAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D56BB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D56BB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D56BB8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82D56BBC: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D56D90 size=384
    let mut pc: u32 = 0x82D56D90;
    'dispatch: loop {
        match pc {
            0x82D56D90 => {
    //   block [0x82D56D90..0x82D56F10)
	// 82D56D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D56D94: 4BF52675  bl 0x82ca9408
	ctx.lr = 0x82D56D98;
	sub_82CA93D0(ctx, base);
	// 82D56D98: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82D56D9C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D56DA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D56DA4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82D56DA8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D56DAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D56DB0: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D56F10 size=136
    let mut pc: u32 = 0x82D56F10;
    'dispatch: loop {
        match pc {
            0x82D56F10 => {
    //   block [0x82D56F10..0x82D56F98)
	// 82D56F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D56F14: 4BF524ED  bl 0x82ca9400
	ctx.lr = 0x82D56F18;
	sub_82CA93D0(ctx, base);
	// 82D56F18: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D56F1C: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D56F20: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82D56F24: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D56F28: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82D56F2C: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D56F30: 7C9BF1D6  mullw r4, r27, r30
	ctx.r[4].s64 = (ctx.r[27].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82D56F34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D56F38: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D56F3C: 4BFFE30D  bl 0x82d55248
	ctx.lr = 0x82D56F40;
	sub_82D55248(ctx, base);
	// 82D56F40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D56F44: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82D56F48: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D56F4C: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82D56F50: 4BF52531  bl 0x82ca9480
	ctx.lr = 0x82D56F54;
	sub_82CA9480(ctx, base);
	// 82D56F54: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D56F58: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D56F5C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D56F60: 409A001C  bne cr6, 0x82d56f7c
	if !ctx.cr[6].eq {
	pc = 0x82D56F7C; continue 'dispatch;
	}
	// 82D56F64: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D56F68: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D56F6C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D56F70: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D56F74: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82D56F78: 4BFFE351  bl 0x82d552c8
	ctx.lr = 0x82D56F7C;
	sub_82D552C8(ctx, base);
	// 82D56F7C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D56F80: 935F0000  stw r26, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82D56F84: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D56F88: 7D6BDB78  or r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[27].u64;
	// 82D56F8C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D56F90: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D56F94: 4BF524BC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D56F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D56F98 size=152
    let mut pc: u32 = 0x82D56F98;
    'dispatch: loop {
        match pc {
            0x82D56F98 => {
    //   block [0x82D56F98..0x82D57030)
	// 82D56F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D56F9C: 4BF52465  bl 0x82ca9400
	ctx.lr = 0x82D56FA0;
	sub_82CA93D0(ctx, base);
	// 82D56FA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D56FA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D56FA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D56FAC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D56FB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D56FB4: 557A083C  slwi r26, r11, 1
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 82D56FB8: 409A0008  bne cr6, 0x82d56fc0
	if !ctx.cr[6].eq {
	pc = 0x82D56FC0; continue 'dispatch;
	}
	// 82D56FBC: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82D56FC0: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D56FC4: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82D56FC8: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D56FCC: 7C9AF1D6  mullw r4, r26, r30
	ctx.r[4].s64 = (ctx.r[26].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82D56FD0: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D56FD4: 4BFFE275  bl 0x82d55248
	ctx.lr = 0x82D56FD8;
	sub_82D55248(ctx, base);
	// 82D56FD8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D56FDC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D56FE0: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82D56FE4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82D56FE8: 4BF52499  bl 0x82ca9480
	ctx.lr = 0x82D56FEC;
	sub_82CA9480(ctx, base);
	// 82D56FEC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D56FF0: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D56FF4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D56FF8: 409A001C  bne cr6, 0x82d57014
	if !ctx.cr[6].eq {
	pc = 0x82D57014; continue 'dispatch;
	}
	// 82D56FFC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D57000: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57004: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D57008: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D5700C: 7CABF1D6  mullw r5, r11, r30
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[30].s32 as i64);
	// 82D57010: 4BFFE2B9  bl 0x82d552c8
	ctx.lr = 0x82D57014;
	sub_82D552C8(ctx, base);
	// 82D57014: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D57018: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82D5701C: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D57020: 7D6BD378  or r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[26].u64;
	// 82D57024: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D57028: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D5702C: 4BF52424  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57030 size=176
    let mut pc: u32 = 0x82D57030;
    'dispatch: loop {
        match pc {
            0x82D57030 => {
    //   block [0x82D57030..0x82D570E0)
	// 82D57030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57034: 4BF523D1  bl 0x82ca9404
	ctx.lr = 0x82D57038;
	sub_82CA93D0(ctx, base);
	// 82D57038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5703C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D57040: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D57044: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82D57048: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82D5704C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82D57050: 419A001C  beq cr6, 0x82d5706c
	if ctx.cr[6].eq {
	pc = 0x82D5706C; continue 'dispatch;
	}
	// 82D57054: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D57058: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82D5705C: 40980010  bge cr6, 0x82d5706c
	if !ctx.cr[6].lt {
	pc = 0x82D5706C; continue 'dispatch;
	}
	// 82D57060: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D57064: 3F608000  lis r27, -0x8000
	ctx.r[27].s64 = -2147483648;
	// 82D57068: 48000020  b 0x82d57088
	pc = 0x82D57088; continue 'dispatch;
	// 82D5706C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57070: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D57074: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D57078: 7C9DE1D6  mullw r4, r29, r28
	ctx.r[4].s64 = (ctx.r[29].s32 as i64) * (ctx.r[28].s32 as i64);
	// 82D5707C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D57080: 4BFFE1C9  bl 0x82d55248
	ctx.lr = 0x82D57084;
	sub_82D55248(ctx, base);
	// 82D57084: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D57088: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5708C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D57090: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57094: 7CABE9D6  mullw r5, r11, r29
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[29].s32 as i64);
	// 82D57098: 48001C99  bl 0x82d58d30
	ctx.lr = 0x82D5709C;
	sub_82D58D30(ctx, base);
	// 82D5709C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D570A0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D570A4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D570A8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D570AC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D570B0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D570B4: 7CABE9D6  mullw r5, r11, r29
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[29].s32 as i64);
	// 82D570B8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D570BC: 4BFFE20D  bl 0x82d552c8
	ctx.lr = 0x82D570C0;
	sub_82D552C8(ctx, base);
	// 82D570C0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D570C4: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82D570C8: 556B0042  rlwinm r11, r11, 0, 1, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D570CC: 7D6BDB78  or r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[27].u64;
	// 82D570D0: 7D6BE378  or r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[28].u64;
	// 82D570D4: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D570D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D570DC: 4BF52378  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D570E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D570E0 size=80
    let mut pc: u32 = 0x82D570E0;
    'dispatch: loop {
        match pc {
            0x82D570E0 => {
    //   block [0x82D570E0..0x82D57130)
	// 82D570E0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D570E4: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 82D570E8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D570EC: C0090C18  lfs f0, 0xc18(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D570F0: C1ABFFF8  lfs f13, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D570F4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D570F8: D1A40000  stfs f13, 0(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D570FC: C1ABFFFC  lfs f13, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D57100: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D57104: D1A40004  stfs f13, 4(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D57108: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5710C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D57110: D1A40008  stfs f13, 8(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D57114: D004000C  stfs f0, 0xc(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D57118: 38840010  addi r4, r4, 0x10
	ctx.r[4].s64 = ctx.r[4].s64 + 16;
	// 82D5711C: 409AFFD4  bne cr6, 0x82d570f0
	if !ctx.cr[6].eq {
	pc = 0x82D570F0; continue 'dispatch;
	}
	// 82D57120: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D57124: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D57128: D004FFFC  stfs f0, -4(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D5712C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D57130 size=80
    let mut pc: u32 = 0x82D57130;
    'dispatch: loop {
        match pc {
            0x82D57130 => {
    //   block [0x82D57130..0x82D57180)
	// 82D57130: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82D57134: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D57138: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D5713C: C0090C18  lfs f0, 0xc18(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(3096 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D57140: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D57144: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D57148: D1AB0000  stfs f13, 0(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82D5714C: C1A40004  lfs f13, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D57150: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D57154: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82D57158: C1A40008  lfs f13, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D5715C: 38840010  addi r4, r4, 0x10
	ctx.r[4].s64 = ctx.r[4].s64 + 16;
	// 82D57160: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82D57164: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82D57168: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 82D5716C: 409AFFD4  bne cr6, 0x82d57140
	if !ctx.cr[6].eq {
	pc = 0x82D57140; continue 'dispatch;
	}
	// 82D57170: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82D57174: C00A0C14  lfs f0, 0xc14(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D57178: D00BFFFC  stfs f0, -4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D5717C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D57180 size=64
    let mut pc: u32 = 0x82D57180;
    'dispatch: loop {
        match pc {
            0x82D57180 => {
    //   block [0x82D57180..0x82D571C0)
	// 82D57180: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82D57184: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D57188: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 82D5718C: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D57190: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D57194: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82D57198: 55080050  rlwinm r8, r8, 0, 1, 8
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5719C: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D571A0: 419A0020  beq cr6, 0x82d571c0
	if ctx.cr[6].eq {
		sub_82D571C0(ctx, base);
		return;
	}
	// 82D571A4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D571A8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82D571AC: 2F0A0010  cmpwi cr6, r10, 0x10
	ctx.cr[6].compare_i32(ctx.r[10].s32, 16, &mut ctx.xer);
	// 82D571B0: 4198FFDC  blt cr6, 0x82d5718c
	if ctx.cr[6].lt {
	pc = 0x82D5718C; continue 'dispatch;
	}
	// 82D571B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D571B8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D571BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D571C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D571C0 size=12
    let mut pc: u32 = 0x82D571C0;
    'dispatch: loop {
        match pc {
            0x82D571C0 => {
    //   block [0x82D571C0..0x82D571CC)
	// 82D571C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D571C4: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D571C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D571D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D571D0 size=176
    let mut pc: u32 = 0x82D571D0;
    'dispatch: loop {
        match pc {
            0x82D571D0 => {
    //   block [0x82D571D0..0x82D57280)
	// 82D571D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D571D4: 4BF52239  bl 0x82ca940c
	ctx.lr = 0x82D571D8;
	sub_82CA93D0(ctx, base);
	// 82D571D8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82D571DC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D571E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D571E4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82D571E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D571EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D571F0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D571F4: 480081C5  bl 0x82d5f3b8
	ctx.lr = 0x82D571F8;
	sub_82D5F3B8(ctx, base);
	// 82D571F8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D571FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D57200: 419A0068  beq cr6, 0x82d57268
	if ctx.cr[6].eq {
	pc = 0x82D57268; continue 'dispatch;
	}
	// 82D57204: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D57208: D3E10060  stfs f31, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82D5720C: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82D57210: 119F038C  vspltisw v12, -1
	for i in 0..4 {
		ctx.v[12].u32[i] = 4294967295;
	}
	// 82D57214: 396B14A0  addi r11, r11, 0x14a0
	ctx.r[11].s64 = ctx.r[11].s64 + 5280;
	// 82D57218: 392000E0  li r9, 0xe0
	ctx.r[9].s64 = 224;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57280 size=128
    let mut pc: u32 = 0x82D57280;
    'dispatch: loop {
        match pc {
            0x82D57280 => {
    //   block [0x82D57280..0x82D57300)
	// 82D57280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D57288: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5728C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D57290: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D57294: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D57298: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5729C: 4800802D  bl 0x82d5f2c8
	ctx.lr = 0x82D572A0;
	sub_82D5F2C8(ctx, base);
	// 82D572A0: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D57300 size=108
    let mut pc: u32 = 0x82D57300;
    'dispatch: loop {
        match pc {
            0x82D57300 => {
    //   block [0x82D57300..0x82D5736C)
	// 82D57300: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D57370 size=156
    let mut pc: u32 = 0x82D57370;
    'dispatch: loop {
        match pc {
            0x82D57370 => {
    //   block [0x82D57370..0x82D5740C)
	// 82D57370: 38E40010  addi r7, r4, 0x10
	ctx.r[7].s64 = ctx.r[4].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57410 size=100
    let mut pc: u32 = 0x82D57410;
    'dispatch: loop {
        match pc {
            0x82D57410 => {
    //   block [0x82D57410..0x82D57474)
	// 82D57410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D57418: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5741C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57478 size=80
    let mut pc: u32 = 0x82D57478;
    'dispatch: loop {
        match pc {
            0x82D57478 => {
    //   block [0x82D57478..0x82D574C8)
	// 82D57478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5747C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D57480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D57484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D57488: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5748C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D57490: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D57494: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82D57498: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5749C: 4BFFFDE5  bl 0x82d57280
	ctx.lr = 0x82D574A0;
	sub_82D57280(ctx, base);
	// 82D574A0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82D574A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D574A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D574AC: 4BFFFE55  bl 0x82d57300
	ctx.lr = 0x82D574B0;
	sub_82D57300(ctx, base);
	// 82D574B0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82D574B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D574B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D574BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D574C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D574C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D574C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D574C8 size=64
    let mut pc: u32 = 0x82D574C8;
    'dispatch: loop {
        match pc {
            0x82D574C8 => {
    //   block [0x82D574C8..0x82D57508)
	// 82D574C8: 3D40832F  lis r10, -0x7cd1
	ctx.r[10].s64 = -2094071808;
	// 82D574CC: 3D603E39  lis r11, 0x3e39
	ctx.r[11].s64 = 1043922944;
	// 82D574D0: 6169B193  ori r9, r11, 0xb193
	ctx.r[9].u64 = ctx.r[11].u64 | 45459;
	// 82D574D4: 816A6EA0  lwz r11, 0x6ea0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28320 as u32) ) } as u64;
	// 82D574D8: 7D6B49D6  mullw r11, r11, r9
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[9].s32 as i64);
	// 82D574DC: 216B3039  subfic r11, r11, 0x3039
	ctx.xer.ca = ctx.r[11].u32 <= 12345 as u32;
	ctx.r[11].s64 = (12345 as i64) - ctx.r[11].s64;
	// 82D574E0: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 82D574E4: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 82D574E8: 916A6EA0  stw r11, 0x6ea0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28320 as u32), ctx.r[11].u32 ) };
	// 82D574EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82D574F0: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D574F4: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82D574F8: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82D574FC: C00B6D54  lfs f0, 0x6d54(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27988 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D57500: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D57504: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D57508 size=136
    let mut pc: u32 = 0x82D57508;
    'dispatch: loop {
        match pc {
            0x82D57508 => {
    //   block [0x82D57508..0x82D57590)
	// 82D57508: D021FFF0  stfs f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D5750C: 3CE0FF80  lis r7, -0x80
	ctx.r[7].s64 = -8388608;
	// 82D57510: 3CC00080  lis r6, 0x80
	ctx.r[6].s64 = 8388608;
	// 82D57514: 8121FFF0  lwz r9, -0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82D57518: 552B4E3E  rlwinm r11, r9, 9, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x007FFFFFu64;
	// 82D5751C: 552A007E  clrlwi r10, r9, 1
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x7FFFFFFFu64;
	// 82D57520: 396BFF81  addi r11, r11, -0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + -127;
	// 82D57524: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D57528: 210B0017  subfic r8, r11, 0x17
	ctx.xer.ca = ctx.r[11].u32 <= 23 as u32;
	ctx.r[8].s64 = (23 as i64) - ctx.r[11].s64;
	// 82D5752C: 7D05FE70  srawi r5, r8, 0x1f
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[8].s32 >> 31) as i64;
	// 82D57530: 7D44FE70  srawi r4, r10, 0x1f
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[10].s32 >> 31) as i64;
	// 82D57534: 7CAA28F8  nor r10, r5, r5
	ctx.r[10].u64 = !(ctx.r[5].u64 | ctx.r[5].u64);
	// 82D57538: 7D292078  andc r9, r9, r4
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[4].u64;
	// 82D5753C: 7D484038  and r8, r10, r8
	ctx.r[8].u64 = ctx.r[10].u64 & ctx.r[8].u64;
	// 82D57540: 71450017  andi. r5, r10, 0x17
	ctx.r[5].u64 = ctx.r[10].u64 & 23;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82D57544: 7D082850  subf r8, r8, r5
	ctx.r[8].s64 = ctx.r[5].s64 - ctx.r[8].s64;
	// 82D57548: 7CE84630  sraw r8, r7, r8
	tmp.u32 = ctx.r[8].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[7].s32 >> tmp.u32) as i64;
	// 82D5754C: 7D0A5338  orc r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 | ~ctx.r[10].u64;
	// 82D57550: 7D28FE70  srawi r8, r9, 0x1f
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[9].s32 >> 31) as i64;
	// 82D57554: 7D67FE70  srawi r7, r11, 0x1f
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 31) as i64;
	// 82D57558: 7CC65E30  sraw r6, r6, r11
	tmp.u32 = ctx.r[11].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[6].s32 >> tmp.u32) as i64;
	// 82D5755C: 7D2B5078  andc r11, r9, r10
	ctx.r[11].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 82D57560: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D57564: 7CCB5878  andc r11, r6, r11
	ctx.r[11].u64 = ctx.r[6].u64 & !ctx.r[11].u64;
	// 82D57568: 7CE64038  and r6, r7, r8
	ctx.r[6].u64 = ctx.r[7].u64 & ctx.r[8].u64;
	// 82D5756C: 7D6B4038  and r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[8].u64;
	// 82D57570: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82D57574: 7D6B3878  andc r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 & !ctx.r[7].u64;
	// 82D57578: 7D6B5038  and r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[10].u64;
	// 82D5757C: 74C8BF80  andis. r8, r6, 0xbf80
	ctx.r[8].u64 = ctx.r[6].u64 & 3212836864;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82D57580: 7D6B4378  or r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[8].u64;
	// 82D57584: 9161FFF0  stw r11, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u32 ) };
	// 82D57588: C021FFF0  lfs f1, -0x10(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82D5758C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D57590 size=184
    let mut pc: u32 = 0x82D57590;
    'dispatch: loop {
        match pc {
            0x82D57590 => {
    //   block [0x82D57590..0x82D57648)
	// 82D57590: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82D57594: D021FFF0  stfs f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D57598: 3C80FF80  lis r4, -0x80
	ctx.r[4].s64 = -8388608;
	// 82D5759C: 3C600080  lis r3, 0x80
	ctx.r[3].s64 = 8388608;
	// 82D575A0: 8121FFF0  lwz r9, -0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82D575A4: 552B4E3E  rlwinm r11, r9, 9, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x007FFFFFu64;
	// 82D575A8: 552A007E  clrlwi r10, r9, 1
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x7FFFFFFFu64;
	// 82D575AC: 396BFF81  addi r11, r11, -0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + -127;
	// 82D575B0: 38EAFFFF  addi r7, r10, -1
	ctx.r[7].s64 = ctx.r[10].s64 + -1;
	// 82D575B4: 390BFFE9  addi r8, r11, -0x17
	ctx.r[8].s64 = ctx.r[11].s64 + -23;
	// 82D575B8: 3948FFFF  addi r10, r8, -1
	ctx.r[10].s64 = ctx.r[8].s64 + -1;
	// 82D575BC: 7D4AFE70  srawi r10, r10, 0x1f
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 31) as i64;
	// 82D575C0: 20CB0017  subfic r6, r11, 0x17
	ctx.xer.ca = ctx.r[11].u32 <= 23 as u32;
	ctx.r[6].s64 = (23 as i64) - ctx.r[11].s64;
	// 82D575C4: 7CE7FE70  srawi r7, r7, 0x1f
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 31) as i64;
	// 82D575C8: 7D293878  andc r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[7].u64;
	// 82D575CC: 7CC75038  and r7, r6, r10
	ctx.r[7].u64 = ctx.r[6].u64 & ctx.r[10].u64;
	// 82D575D0: 7D4650F8  nor r6, r10, r10
	ctx.r[6].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82D575D4: 71450017  andi. r5, r10, 0x17
	ctx.r[5].u64 = ctx.r[10].u64 & 23;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82D575D8: 7D472850  subf r10, r7, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[7].s64;
	// 82D575DC: 7CC54038  and r5, r6, r8
	ctx.r[5].u64 = ctx.r[6].u64 & ctx.r[8].u64;
	// 82D575E0: 7C845630  sraw r4, r4, r10
	tmp.u32 = ctx.r[10].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[4].s64 = (ctx.r[4].s32 >> tmp.u32) as i64;
	// 82D575E4: 7D2AFE70  srawi r10, r9, 0x1f
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[9].s32 >> 31) as i64;
	// 82D575E8: 7D68FE70  srawi r8, r11, 0x1f
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 31) as i64;
	// 82D575EC: 7C863378  or r6, r4, r6
	ctx.r[6].u64 = ctx.r[4].u64 | ctx.r[6].u64;
	// 82D575F0: 7D1F5038  and r31, r8, r10
	ctx.r[31].u64 = ctx.r[8].u64 & ctx.r[10].u64;
	// 82D575F4: 7D243078  andc r4, r9, r6
	ctx.r[4].u64 = ctx.r[9].u64 & !ctx.r[6].u64;
	// 82D575F8: 7C635E30  sraw r3, r3, r11
	tmp.u32 = ctx.r[11].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[3].s32 >> tmp.u32) as i64;
	// 82D575FC: 7D0B40F8  nor r11, r8, r8
	ctx.r[11].u64 = !(ctx.r[8].u64 | ctx.r[8].u64);
	// 82D57600: 3904FFFF  addi r8, r4, -1
	ctx.r[8].s64 = ctx.r[4].s64 + -1;
	// 82D57604: 7FE45B78  or r4, r31, r11
	ctx.r[4].u64 = ctx.r[31].u64 | ctx.r[11].u64;
	// 82D57608: 7C684078  andc r8, r3, r8
	ctx.r[8].u64 = ctx.r[3].u64 & !ctx.r[8].u64;
	// 82D5760C: 7D6B4838  and r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[9].u64;
	// 82D57610: 7D085038  and r8, r8, r10
	ctx.r[8].u64 = ctx.r[8].u64 & ctx.r[10].u64;
	// 82D57614: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D57618: 508B0210  rlwimi r11, r4, 0, 8, 8
	ctx.r[11].u64 = (((ctx.r[4].u32).rotate_left(0) as u64) & 0x0000000000800000) | (ctx.r[11].u64 & 0xFFFFFFFFFF7FFFFF);
	// 82D5761C: 556B023E  clrlwi r11, r11, 8
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00FFFFFFu64;
	// 82D57620: 7D6B3038  and r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 & ctx.r[6].u64;
	// 82D57624: 5569083C  slwi r9, r11, 1
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82D57628: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82D5762C: 7D2A5378  or r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 | ctx.r[10].u64;
	// 82D57630: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82D57634: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D57638: 7D6B3E30  sraw r11, r11, r7
	tmp.u32 = ctx.r[7].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> tmp.u32) as i64;
	// 82D5763C: 7D632830  slw r3, r11, r5
	if (ctx.r[5].u8 & 0x20) != 0 {
		ctx.r[3].u64 = 0;
	} else {
		ctx.r[3].u64 = ((ctx.r[11].u32) << ((ctx.r[5].u8 & 0x1F) as u32)) as u64;
	}
	// 82D57640: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82D57644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D57648 size=140
    let mut pc: u32 = 0x82D57648;
    'dispatch: loop {
        match pc {
            0x82D57648 => {
    //   block [0x82D57648..0x82D576D4)
	// 82D57648: D021FFF0  stfs f1, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D5764C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D57650: 3CA0FF80  lis r5, -0x80
	ctx.r[5].s64 = -8388608;
	// 82D57654: 8121FFF0  lwz r9, -0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82D57658: 552B4E3E  rlwinm r11, r9, 9, 0x18, 0x1f
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x007FFFFFu64;
	// 82D5765C: 552A007E  clrlwi r10, r9, 1
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x7FFFFFFFu64;
	// 82D57660: 396BFF81  addi r11, r11, -0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + -127;
	// 82D57664: 38EAFFFF  addi r7, r10, -1
	ctx.r[7].s64 = ctx.r[10].s64 + -1;
	// 82D57668: 390BFFE9  addi r8, r11, -0x17
	ctx.r[8].s64 = ctx.r[11].s64 + -23;
	// 82D5766C: 3948FFFF  addi r10, r8, -1
	ctx.r[10].s64 = ctx.r[8].s64 + -1;
	// 82D57670: 7D4AFE70  srawi r10, r10, 0x1f
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 31) as i64;
	// 82D57674: 20CB0017  subfic r6, r11, 0x17
	ctx.xer.ca = ctx.r[11].u32 <= 23 as u32;
	ctx.r[6].s64 = (23 as i64) - ctx.r[11].s64;
	// 82D57678: 7CE7FE70  srawi r7, r7, 0x1f
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 31) as i64;
	// 82D5767C: 7D293878  andc r9, r9, r7
	ctx.r[9].u64 = ctx.r[9].u64 & !ctx.r[7].u64;
	// 82D57680: 7CC75038  and r7, r6, r10
	ctx.r[7].u64 = ctx.r[6].u64 & ctx.r[10].u64;
	// 82D57684: 7D4650F8  nor r6, r10, r10
	ctx.r[6].u64 = !(ctx.r[10].u64 | ctx.r[10].u64);
	// 82D57688: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 82D5768C: 5083B810  rlwimi r3, r4, 0x17, 0, 8
	ctx.r[3].u64 = (((ctx.r[4].u32).rotate_left(23) as u64) & 0x00000000FF800000) | (ctx.r[3].u64 & 0xFFFFFFFF007FFFFF);
	// 82D57690: 714A0017  andi. r10, r10, 0x17
	ctx.r[10].u64 = ctx.r[10].u64 & 23;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D57694: 7C875050  subf r4, r7, r10
	ctx.r[4].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 82D57698: 7CCA4038  and r10, r6, r8
	ctx.r[10].u64 = ctx.r[6].u64 & ctx.r[8].u64;
	// 82D5769C: 7CA82630  sraw r8, r5, r4
	tmp.u32 = ctx.r[4].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[5].s32 >> tmp.u32) as i64;
	// 82D576A0: 7D29FE70  srawi r9, r9, 0x1f
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 31) as i64;
	// 82D576A4: 7D083378  or r8, r8, r6
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[6].u64;
	// 82D576A8: 7D66FE70  srawi r6, r11, 0x1f
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 31) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 31) as i64;
	// 82D576AC: 7C6B4038  and r11, r3, r8
	ctx.r[11].u64 = ctx.r[3].u64 & ctx.r[8].u64;
	// 82D576B0: 5568083C  slwi r8, r11, 1
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82D576B4: 3908FFFF  addi r8, r8, -1
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	// 82D576B8: 7D094B78  or r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 82D576BC: 7D6B4850  subf r11, r11, r9
	ctx.r[11].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 82D576C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D576C4: 7D6B3078  andc r11, r11, r6
	ctx.r[11].u64 = ctx.r[11].u64 & !ctx.r[6].u64;
	// 82D576C8: 7D6B3E30  sraw r11, r11, r7
	tmp.u32 = ctx.r[7].u32 & 0x3F;
	if tmp.u32 > 0x1F { tmp.u32 = 0x1F; }
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << tmp.u32) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> tmp.u32) as i64;
	// 82D576CC: 7D635030  slw r3, r11, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[3].u64 = 0;
	} else {
		ctx.r[3].u64 = ((ctx.r[11].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 82D576D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D576D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D576D8 size=68
    let mut pc: u32 = 0x82D576D8;
    'dispatch: loop {
        match pc {
            0x82D576D8 => {
    //   block [0x82D576D8..0x82D5771C)
	// 82D576D8: FC000A10  fabs f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[1].u64 & !0x8000_0000_0000_0000u64;
	// 82D576DC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D576E0: FDA01210  fabs f13, f2
	ctx.f[13].u64 = ctx.f[2].u64 & !0x8000_0000_0000_0000u64;
	// 82D576E4: C18B0C38  lfs f12, 0xc38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3128 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D576E8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D576EC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82D576F0: 4199002C  bgt cr6, 0x82d5771c
	if ctx.cr[6].gt {
		sub_82D5771C(ctx, base);
		return;
	}
	// 82D576F4: EDAD602A  fadds f13, f13, f12
	ctx.f[13].f64 = ((ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64;
	// 82D576F8: C18B3A64  lfs f12, 0x3a64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14948 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D576FC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D57700: C16B3A60  lfs f11, 0x3a60(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14944 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82D57704: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82D57708: EDA00032  fmuls f13, f0, f0
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D5770C: ED4D0032  fmuls f10, f13, f0
	ctx.f[10].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D57710: EC0D033C  fnmsubs f0, f13, f12, f0
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82D57714: EC0A02FC  fnmsubs f0, f10, f11, f0
	ctx.f[0].f64 = -(((ctx.f[10].f64 * ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 82D57718: 48000034  b 0x82d5774c
	sub_82D5771C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5771C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5771C size=92
    let mut pc: u32 = 0x82D5771C;
    'dispatch: loop {
        match pc {
            0x82D5771C => {
    //   block [0x82D5771C..0x82D57778)
	// 82D5771C: EC00602A  fadds f0, f0, f12
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 82D57720: C18B3A64  lfs f12, 0x3a64(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14948 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D57724: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D57728: C16B3A60  lfs f11, 0x3a60(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14944 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82D5772C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82D57730: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82D57734: EDA00032  fmuls f13, f0, f0
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D57738: ED4D0032  fmuls f10, f13, f0
	ctx.f[10].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82D5773C: EC0D033C  fnmsubs f0, f13, f12, f0
	ctx.f[0].f64 = -(((ctx.f[13].f64 * ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82D57740: EDAA02FC  fnmsubs f13, f10, f11, f0
	ctx.f[13].f64 = -(((ctx.f[10].f64 * ctx.f[11].f64 - ctx.f[0].f64) as f32) as f64);
	// 82D57744: C00B692C  lfs f0, 0x692c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26924 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D57748: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 82D5774C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D57750: C18B0C18  lfs f12, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D57754: FF026000  fcmpu cr6, f2, f12
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[12].f64);
	// 82D57758: 40980010  bge cr6, 0x82d57768
	if !ctx.cr[6].lt {
	pc = 0x82D57768; continue 'dispatch;
	}
	// 82D5775C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D57760: C1AB0B40  lfs f13, 0xb40(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2880 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D57764: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82D57768: FF016000  fcmpu cr6, f1, f12
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[12].f64);
	// 82D5776C: 4098000C  bge cr6, 0x82d57778
	if !ctx.cr[6].lt {
		sub_82D57778(ctx, base);
		return;
	}
	// 82D57770: FC200050  fneg f1, f0
	ctx.f[1].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82D57774: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D57778 size=8
    let mut pc: u32 = 0x82D57778;
    'dispatch: loop {
        match pc {
            0x82D57778 => {
    //   block [0x82D57778..0x82D57780)
	// 82D57778: FC200090  fmr f1, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[0].f64;
	// 82D5777C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D57780 size=232
    let mut pc: u32 = 0x82D57780;
    'dispatch: loop {
        match pc {
            0x82D57780 => {
    //   block [0x82D57780..0x82D57868)
	// 82D57780: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D57784: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D57788: C1840004  lfs f12, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82D5778C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82D57790: C1240008  lfs f9, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 82D57794: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82D57798: C164000C  lfs f11, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82D5779C: C1AB0C18  lfs f13, 0xc18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3096 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D577A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D577A4: D1A1FFDC  stfs f13, -0x24(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 82D577A8: D1A1FFEC  stfs f13, -0x14(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 82D577AC: D1A1FFFC  stfs f13, -4(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 82D577B0: C1AB0C4C  lfs f13, 0xc4c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3148 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82D577B4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D577B8: ED000372  fmuls f8, f0, f13
	ctx.f[8].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D577BC: ED4C0372  fmuls f10, f12, f13
	ctx.f[10].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D577C0: EDA90372  fmuls f13, f9, f13
	ctx.f[13].f64 = (((ctx.f[9].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D577C4: ECE00232  fmuls f7, f0, f8
	ctx.f[7].f64 = (((ctx.f[0].f64 * ctx.f[8].f64) as f32) as f64);
	// 82D577C8: ECCC02B2  fmuls f6, f12, f10
	ctx.f[6].f64 = (((ctx.f[12].f64 * ctx.f[10].f64) as f32) as f64);
	// 82D577CC: ED290372  fmuls f9, f9, f13
	ctx.f[9].f64 = (((ctx.f[9].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D577D0: ED8C0372  fmuls f12, f12, f13
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D577D4: ED0B0232  fmuls f8, f11, f8
	ctx.f[8].f64 = (((ctx.f[11].f64 * ctx.f[8].f64) as f32) as f64);
	// 82D577D8: ECA002B2  fmuls f5, f0, f10
	ctx.f[5].f64 = (((ctx.f[0].f64 * ctx.f[10].f64) as f32) as f64);
	// 82D577DC: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D577E0: ED4B02B2  fmuls f10, f11, f10
	ctx.f[10].f64 = (((ctx.f[11].f64 * ctx.f[10].f64) as f32) as f64);
	// 82D577E4: EDAB0372  fmuls f13, f11, f13
	ctx.f[13].f64 = (((ctx.f[11].f64 * ctx.f[13].f64) as f32) as f64);
	// 82D577E8: ED66382A  fadds f11, f6, f7
	ctx.f[11].f64 = ((ctx.f[6].f64 + ctx.f[7].f64) as f32) as f64;
	// 82D577EC: ECC9302A  fadds f6, f9, f6
	ctx.f[6].f64 = ((ctx.f[9].f64 + ctx.f[6].f64) as f32) as f64;
	// 82D577F0: ED29382A  fadds f9, f9, f7
	ctx.f[9].f64 = ((ctx.f[9].f64 + ctx.f[7].f64) as f32) as f64;
	// 82D577F4: ECE8602A  fadds f7, f8, f12
	ctx.f[7].f64 = ((ctx.f[8].f64 + ctx.f[12].f64) as f32) as f64;
	// 82D577F8: D0E1FFE8  stfs f7, -0x18(r1)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
	// 82D577FC: ECE05028  fsubs f7, f0, f10
	ctx.f[7].f64 = (((ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 82D57800: D0E1FFD8  stfs f7, -0x28(r1)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 82D57804: EC0A002A  fadds f0, f10, f0
	ctx.f[0].f64 = ((ctx.f[10].f64 + ctx.f[0].f64) as f32) as f64;
	// 82D57808: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D5780C: EC0C4028  fsubs f0, f12, f8
	ctx.f[0].f64 = (((ctx.f[12].f64 - ctx.f[8].f64) as f32) as f64);
	// 82D57810: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 82D57814: ECED282A  fadds f7, f13, f5
	ctx.f[7].f64 = ((ctx.f[13].f64 + ctx.f[5].f64) as f32) as f64;
	// 82D57818: C00B0C14  lfs f0, 0xc14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5781C: EDA56828  fsubs f13, f5, f13
	ctx.f[13].f64 = (((ctx.f[5].f64 - ctx.f[13].f64) as f32) as f64);
	// 82D57820: D1A1FFE0  stfs f13, -0x20(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 82D57824: EDA03028  fsubs f13, f0, f6
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[6].f64) as f32) as f64);
	// 82D57828: D1A1FFD0  stfs f13, -0x30(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 82D5782C: D0E1FFD4  stfs f7, -0x2c(r1)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-44 as u32), tmp.u32 ) };
	// 82D57830: 3961FFD0  addi r11, r1, -0x30
	ctx.r[11].s64 = ctx.r[1].s64 + -48;
	// 82D57834: EDA04828  fsubs f13, f0, f9
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[9].f64) as f32) as f64);
	// 82D57838: D1A1FFE4  stfs f13, -0x1c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 82D5783C: EC005828  fsubs f0, f0, f11
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[11].f64) as f32) as f64);
	// 82D57840: D001FFF8  stfs f0, -8(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57868 size=60
    let mut pc: u32 = 0x82D57868;
    'dispatch: loop {
        match pc {
            0x82D57868 => {
    //   block [0x82D57868..0x82D578A4)
	// 82D57868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5786C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D57870: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D57874: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D57878: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5787C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57880: 4BFFED81  bl 0x82d56600
	ctx.lr = 0x82D57884;
	sub_82D56600(ctx, base);
	// 82D57884: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D57888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5788C: 4BFFFEF5  bl 0x82d57780
	ctx.lr = 0x82D57890;
	sub_82D57780(ctx, base);
	// 82D57890: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D57894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D57898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5789C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D578A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D578A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D578A8 size=56
    let mut pc: u32 = 0x82D578A8;
    'dispatch: loop {
        match pc {
            0x82D578A8 => {
    //   block [0x82D578A8..0x82D578E0)
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D578E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D578E0 size=200
    let mut pc: u32 = 0x82D578E0;
    'dispatch: loop {
        match pc {
            0x82D578E0 => {
    //   block [0x82D578E0..0x82D579A8)
	// 82D578E0: 39440010  addi r10, r4, 0x10
	ctx.r[10].s64 = ctx.r[4].s64 + 16;
	// 82D578E4: 3961FFE0  addi r11, r1, -0x20
	ctx.r[11].s64 = ctx.r[1].s64 + -32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D579A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D579A8 size=64
    let mut pc: u32 = 0x82D579A8;
    'dispatch: loop {
        match pc {
            0x82D579A8 => {
    //   block [0x82D579A8..0x82D579E8)
	// 82D579A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D579AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D579B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D579B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D579B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D579BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D579C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D579C4: 4BFFEF4D  bl 0x82d56910
	ctx.lr = 0x82D579C8;
	sub_82D56910(ctx, base);
	// 82D579C8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D579CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D579D0: 4BFFFDB1  bl 0x82d57780
	ctx.lr = 0x82D579D4;
	sub_82D57780(ctx, base);
	// 82D579D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D579D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D579DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D579E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D579E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D579E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D579E8 size=140
    let mut pc: u32 = 0x82D579E8;
    'dispatch: loop {
        match pc {
            0x82D579E8 => {
    //   block [0x82D579E8..0x82D57A74)
	// 82D579E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D579EC: 4BF51A21  bl 0x82ca940c
	ctx.lr = 0x82D579F0;
	sub_82CA93D0(ctx, base);
	// 82D579F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D579F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D579F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D579FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D57A00: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D57A04: 48000895  bl 0x82d58298
	ctx.lr = 0x82D57A08;
	sub_82D58298(ctx, base);
	// 82D57A08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 82D57A0C: 7D7EEA14  add r11, r30, r29
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[29].u64;
	// 82D57A10: 394A3A6C  addi r10, r10, 0x3a6c
	ctx.r[10].s64 = ctx.r[10].s64 + 14956;
	// 82D57A14: 386BFFE0  addi r3, r11, -0x20
	ctx.r[3].s64 = ctx.r[11].s64 + -32;
	// 82D57A18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D57A1C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D57A20: 419A0028  beq cr6, 0x82d57a48
	if ctx.cr[6].eq {
	pc = 0x82D57A48; continue 'dispatch;
	}
	// 82D57A24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D57A28: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D57A2C: 38BDFFE0  addi r5, r29, -0x20
	ctx.r[5].s64 = ctx.r[29].s64 + -32;
	// 82D57A30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D57A34: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82D57A38: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57A3C: 480082A5  bl 0x82d5fce0
	ctx.lr = 0x82D57A40;
	sub_82D5FCE0(ctx, base);
	// 82D57A40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D57A44: 48000008  b 0x82d57a4c
	pc = 0x82D57A4C; continue 'dispatch;
	// 82D57A48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D57A4C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D57A50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D57A54: A14B0004  lhz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D57A58: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D57A5C: 419A0010  beq cr6, 0x82d57a6c
	if ctx.cr[6].eq {
	pc = 0x82D57A6C; continue 'dispatch;
	}
	// 82D57A60: A14B0006  lhz r10, 6(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D57A64: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D57A68: B14B0006  sth r10, 6(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D57A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D57A70: 4BF519EC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57A78 size=264
    let mut pc: u32 = 0x82D57A78;
    'dispatch: loop {
        match pc {
            0x82D57A78 => {
    //   block [0x82D57A78..0x82D57B80)
	// 82D57A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57A7C: 4BF51975  bl 0x82ca93f0
	ctx.lr = 0x82D57A80;
	sub_82CA93D0(ctx, base);
	// 82D57A80: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D57A84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D57A88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D57A8C: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82D57A90: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82D57A94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57A98: 4BFFFF51  bl 0x82d579e8
	ctx.lr = 0x82D57A9C;
	sub_82D579E8(ctx, base);
	// 82D57A9C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D57AA0: A15F0006  lhz r10, 6(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D57AA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57AA8: 388B3B74  addi r4, r11, 0x3b74
	ctx.r[4].s64 = ctx.r[11].s64 + 15220;
	// 82D57AAC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D57AB0: 7D560734  extsh r22, r10
	ctx.r[22].s64 = ctx.r[10].s16 as i64;
	// 82D57AB4: 3BAB3B3C  addi r29, r11, 0x3b3c
	ctx.r[29].s64 = ctx.r[11].s64 + 15164;
	// 82D57AB8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D57ABC: 3B8B3B20  addi r28, r11, 0x3b20
	ctx.r[28].s64 = ctx.r[11].s64 + 15136;
	// 82D57AC0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D57AC4: 3B6B3AEC  addi r27, r11, 0x3aec
	ctx.r[27].s64 = ctx.r[11].s64 + 15084;
	// 82D57AC8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D57ACC: 3B4B3AB4  addi r26, r11, 0x3ab4
	ctx.r[26].s64 = ctx.r[11].s64 + 15028;
	// 82D57AD0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D57AD4: 3B2B3AB0  addi r25, r11, 0x3ab0
	ctx.r[25].s64 = ctx.r[11].s64 + 15024;
	// 82D57AD8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D57ADC: 3B0B3AA8  addi r24, r11, 0x3aa8
	ctx.r[24].s64 = ctx.r[11].s64 + 15016;
	// 82D57AE0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D57AE4: 3AEB3A94  addi r23, r11, 0x3a94
	ctx.r[23].s64 = ctx.r[11].s64 + 14996;
	// 82D57AE8: 48000309  bl 0x82d57df0
	ctx.lr = 0x82D57AEC;
	sub_82D57DF0(ctx, base);
	// 82D57AEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D57AF0: 480001C1  bl 0x82d57cb0
	ctx.lr = 0x82D57AF4;
	sub_82D57CB0(ctx, base);
	// 82D57AF4: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82D57AF8: 480002F9  bl 0x82d57df0
	ctx.lr = 0x82D57AFC;
	sub_82D57DF0(ctx, base);
	// 82D57AFC: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82D57B00: 48000429  bl 0x82d57f28
	ctx.lr = 0x82D57B04;
	sub_82D57F28(ctx, base);
	// 82D57B04: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82D57B08: 480002E9  bl 0x82d57df0
	ctx.lr = 0x82D57B0C;
	sub_82D57DF0(ctx, base);
	// 82D57B0C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D57B10: 480002E1  bl 0x82d57df0
	ctx.lr = 0x82D57B14;
	sub_82D57DF0(ctx, base);
	// 82D57B14: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82D57B18: 480002D9  bl 0x82d57df0
	ctx.lr = 0x82D57B1C;
	sub_82D57DF0(ctx, base);
	// 82D57B1C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82D57B20: 480002D1  bl 0x82d57df0
	ctx.lr = 0x82D57B24;
	sub_82D57DF0(ctx, base);
	// 82D57B24: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82D57B28: 480002C9  bl 0x82d57df0
	ctx.lr = 0x82D57B2C;
	sub_82D57DF0(ctx, base);
	// 82D57B2C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D57B30: 480002C1  bl 0x82d57df0
	ctx.lr = 0x82D57B34;
	sub_82D57DF0(ctx, base);
	// 82D57B34: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D57B38: 480002B9  bl 0x82d57df0
	ctx.lr = 0x82D57B3C;
	sub_82D57DF0(ctx, base);
	// 82D57B3C: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D57B40: 3CA02C66  lis r5, 0x2c66
	ctx.r[5].s64 = 744882176;
	// 82D57B44: 3900001E  li r8, 0x1e
	ctx.r[8].s64 = 30;
	// 82D57B48: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82D57B4C: 60A5F2D8  ori r5, r5, 0xf2d8
	ctx.r[5].u64 = ctx.r[5].u64 | 62168;
	// 82D57B50: 806B7630  lwz r3, 0x7630(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82D57B54: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D57B58: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82D57B5C: 38EB3A78  addi r7, r11, 0x3a78
	ctx.r[7].s64 = ctx.r[11].s64 + 14968;
	// 82D57B60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57B64: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D57B68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D57B6C: 4E800421  bctrl
	ctx.lr = 0x82D57B70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D57B70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57B74: 480008BD  bl 0x82d58430
	ctx.lr = 0x82D57B78;
	sub_82D58430(ctx, base);
	// 82D57B78: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82D57B7C: 4BF518C4  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57B80 size=128
    let mut pc: u32 = 0x82D57B80;
    'dispatch: loop {
        match pc {
            0x82D57B80 => {
    //   block [0x82D57B80..0x82D57C00)
	// 82D57B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D57B88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D57B8C: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D57B90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D57B94: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82D57B98: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82D57B9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57BA0: 4BFFFE49  bl 0x82d579e8
	ctx.lr = 0x82D57BA4;
	sub_82D579E8(ctx, base);
	// 82D57BA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D57BA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57BAC: 48000245  bl 0x82d57df0
	ctx.lr = 0x82D57BB0;
	sub_82D57DF0(ctx, base);
	// 82D57BB0: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D57BB4: 3CA02636  lis r5, 0x2636
	ctx.r[5].s64 = 641073152;
	// 82D57BB8: 39000025  li r8, 0x25
	ctx.r[8].s64 = 37;
	// 82D57BBC: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82D57BC0: 60A5FE25  ori r5, r5, 0xfe25
	ctx.r[5].u64 = ctx.r[5].u64 | 65061;
	// 82D57BC4: 806B7630  lwz r3, 0x7630(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30256 as u32) ) } as u64;
	// 82D57BC8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D57BCC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82D57BD0: 38EB3A78  addi r7, r11, 0x3a78
	ctx.r[7].s64 = ctx.r[11].s64 + 14968;
	// 82D57BD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57BD8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D57BDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D57BE0: 4E800421  bctrl
	ctx.lr = 0x82D57BE4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D57BE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57BE8: 48000849  bl 0x82d58430
	ctx.lr = 0x82D57BEC;
	sub_82D58430(ctx, base);
	// 82D57BEC: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82D57BF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D57BF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D57BF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D57BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57C00 size=112
    let mut pc: u32 = 0x82D57C00;
    'dispatch: loop {
        match pc {
            0x82D57C00 => {
    //   block [0x82D57C00..0x82D57C70)
	// 82D57C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57C04: 4BF51809  bl 0x82ca940c
	ctx.lr = 0x82D57C08;
	sub_82CA93D0(ctx, base);
	// 82D57C08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D57C0C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D57C10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D57C14: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D57C18: 419A0030  beq cr6, 0x82d57c48
	if ctx.cr[6].eq {
	pc = 0x82D57C48; continue 'dispatch;
	}
	// 82D57C1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D57C20: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57C24: 48001005  bl 0x82d58c28
	ctx.lr = 0x82D57C28;
	sub_82D58C28(ctx, base);
	// 82D57C28: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D57C2C: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D57C30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D57C34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D57C38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D57C3C: 4E800421  bctrl
	ctx.lr = 0x82D57C40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D57C40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D57C44: 4BF51818  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82D57C48: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57C4C: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 82D57C50: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82D57C54: 388B3D20  addi r4, r11, 0x3d20
	ctx.r[4].s64 = ctx.r[11].s64 + 15648;
	// 82D57C58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D57C5C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D57C60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D57C64: 4E800421  bctrl
	ctx.lr = 0x82D57C68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D57C68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D57C6C: 4BF517F0  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57C70 size=64
    let mut pc: u32 = 0x82D57C70;
    'dispatch: loop {
        match pc {
            0x82D57C70 => {
    //   block [0x82D57C70..0x82D57CB0)
	// 82D57C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D57C78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D57C7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D57C80: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D57C84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D57C88: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57C8C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D57C90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D57C94: 4E800421  bctrl
	ctx.lr = 0x82D57C98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D57C98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D57C9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D57CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D57CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D57CA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D57CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57CB0 size=100
    let mut pc: u32 = 0x82D57CB0;
    'dispatch: loop {
        match pc {
            0x82D57CB0 => {
    //   block [0x82D57CB0..0x82D57D14)
	// 82D57CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57CB4: 4BF51759  bl 0x82ca940c
	ctx.lr = 0x82D57CB8;
	sub_82CA93D0(ctx, base);
	// 82D57CB8: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82D57CBC: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82D57CC0: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D57CC4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82D57CC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D57CCC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82D57CD0: 38ABB228  addi r5, r11, -0x4dd8
	ctx.r[5].s64 = ctx.r[11].s64 + -19928;
	// 82D57CD4: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82D57CD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57CDC: 48000C6D  bl 0x82d58948
	ctx.lr = 0x82D57CE0;
	sub_82D58948(ctx, base);
	// 82D57CE0: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D57CE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57CE8: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57CEC: 48000F3D  bl 0x82d58c28
	ctx.lr = 0x82D57CF0;
	sub_82D58C28(ctx, base);
	// 82D57CF0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D57CF4: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D57CF8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D57CFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D57D00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D57D04: 4E800421  bctrl
	ctx.lr = 0x82D57D08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D57D08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D57D0C: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82D57D10: 4BF5174C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57D18 size=132
    let mut pc: u32 = 0x82D57D18;
    'dispatch: loop {
        match pc {
            0x82D57D18 => {
    //   block [0x82D57D18..0x82D57D9C)
	// 82D57D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57D1C: 4BF516ED  bl 0x82ca9408
	ctx.lr = 0x82D57D20;
	sub_82CA93D0(ctx, base);
	// 82D57D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D57D24: 7C8B0774  extsb r11, r4
	ctx.r[11].s64 = ctx.r[4].s8 as i64;
	// 82D57D28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D57D2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D57D30: 419A0010  beq cr6, 0x82d57d40
	if ctx.cr[6].eq {
	pc = 0x82D57D40; continue 'dispatch;
	}
	// 82D57D34: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D57D38: 3BCB0F40  addi r30, r11, 0xf40
	ctx.r[30].s64 = ctx.r[11].s64 + 3904;
	// 82D57D3C: 4800000C  b 0x82d57d48
	pc = 0x82D57D48; continue 'dispatch;
	// 82D57D40: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82D57D44: 3BCB1700  addi r30, r11, 0x1700
	ctx.r[30].s64 = ctx.r[11].s64 + 5888;
	// 82D57D48: 83FD0008  lwz r31, 8(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D57D4C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D57D50: 419A0020  beq cr6, 0x82d57d70
	if ctx.cr[6].eq {
	pc = 0x82D57D70; continue 'dispatch;
	}
	// 82D57D54: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D57D58: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57D5C: 48000ECD  bl 0x82d58c28
	ctx.lr = 0x82D57D60;
	sub_82D58C28(ctx, base);
	// 82D57D60: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D57D64: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D57D68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D57D6C: 48000018  b 0x82d57d84
	pc = 0x82D57D84; continue 'dispatch;
	// 82D57D70: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57D74: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 82D57D78: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82D57D7C: 388B3D20  addi r4, r11, 0x3d20
	ctx.r[4].s64 = ctx.r[11].s64 + 15648;
	// 82D57D80: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D57D84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D57D88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D57D8C: 4E800421  bctrl
	ctx.lr = 0x82D57D90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D57D90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D57D94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D57D98: 4BF516C0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57DA0 size=76
    let mut pc: u32 = 0x82D57DA0;
    'dispatch: loop {
        match pc {
            0x82D57DA0 => {
    //   block [0x82D57DA0..0x82D57DEC)
	// 82D57DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D57DA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D57DAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D57DB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D57DB4: 9881007F  stb r4, 0x7f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(127 as u32), ctx.r[4].u8 ) };
	// 82D57DB8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D57DBC: 3881007F  addi r4, r1, 0x7f
	ctx.r[4].s64 = ctx.r[1].s64 + 127;
	// 82D57DC0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D57DC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57DC8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D57DCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D57DD0: 4E800421  bctrl
	ctx.lr = 0x82D57DD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D57DD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D57DD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D57DDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D57DE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D57DE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D57DE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57DF0 size=104
    let mut pc: u32 = 0x82D57DF0;
    'dispatch: loop {
        match pc {
            0x82D57DF0 => {
    //   block [0x82D57DF0..0x82D57E58)
	// 82D57DF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57DF4: 4BF51615  bl 0x82ca9408
	ctx.lr = 0x82D57DF8;
	sub_82CA93D0(ctx, base);
	// 82D57DF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D57DFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D57E00: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D57E04: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82D57E08: 83FE0008  lwz r31, 8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D57E0C: 419A0020  beq cr6, 0x82d57e2c
	if ctx.cr[6].eq {
	pc = 0x82D57E2C; continue 'dispatch;
	}
	// 82D57E10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D57E14: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57E18: 48000E11  bl 0x82d58c28
	ctx.lr = 0x82D57E1C;
	sub_82D58C28(ctx, base);
	// 82D57E1C: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D57E20: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D57E24: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D57E28: 48000018  b 0x82d57e40
	pc = 0x82D57E40; continue 'dispatch;
	// 82D57E2C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57E30: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 82D57E34: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82D57E38: 388B3D20  addi r4, r11, 0x3d20
	ctx.r[4].s64 = ctx.r[11].s64 + 15648;
	// 82D57E3C: 816A0010  lwz r11, 0x10(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D57E40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D57E44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D57E48: 4E800421  bctrl
	ctx.lr = 0x82D57E4C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D57E4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D57E50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D57E54: 4BF51604  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57E58 size=100
    let mut pc: u32 = 0x82D57E58;
    'dispatch: loop {
        match pc {
            0x82D57E58 => {
    //   block [0x82D57E58..0x82D57EBC)
	// 82D57E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57E5C: 4BF515B1  bl 0x82ca940c
	ctx.lr = 0x82D57E60;
	sub_82CA93D0(ctx, base);
	// 82D57E60: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82D57E64: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82D57E68: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D57E6C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D57E70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D57E74: 7C860734  extsh r6, r4
	ctx.r[6].s64 = ctx.r[4].s16 as i64;
	// 82D57E78: 38AB3B98  addi r5, r11, 0x3b98
	ctx.r[5].s64 = ctx.r[11].s64 + 15256;
	// 82D57E7C: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82D57E80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57E84: 48000AC5  bl 0x82d58948
	ctx.lr = 0x82D57E88;
	sub_82D58948(ctx, base);
	// 82D57E88: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D57E8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57E90: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57E94: 48000D95  bl 0x82d58c28
	ctx.lr = 0x82D57E98;
	sub_82D58C28(ctx, base);
	// 82D57E98: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D57E9C: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D57EA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D57EA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D57EA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D57EAC: 4E800421  bctrl
	ctx.lr = 0x82D57EB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D57EB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D57EB4: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82D57EB8: 4BF515A4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57EC0 size=100
    let mut pc: u32 = 0x82D57EC0;
    'dispatch: loop {
        match pc {
            0x82D57EC0 => {
    //   block [0x82D57EC0..0x82D57F24)
	// 82D57EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57EC4: 4BF51549  bl 0x82ca940c
	ctx.lr = 0x82D57EC8;
	sub_82CA93D0(ctx, base);
	// 82D57EC8: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82D57ECC: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82D57ED0: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D57ED4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82D57ED8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D57EDC: 5486043E  clrlwi r6, r4, 0x10
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82D57EE0: 38AB8864  addi r5, r11, -0x779c
	ctx.r[5].s64 = ctx.r[11].s64 + -30620;
	// 82D57EE4: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82D57EE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57EEC: 48000A5D  bl 0x82d58948
	ctx.lr = 0x82D57EF0;
	sub_82D58948(ctx, base);
	// 82D57EF0: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D57EF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57EF8: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57EFC: 48000D2D  bl 0x82d58c28
	ctx.lr = 0x82D57F00;
	sub_82D58C28(ctx, base);
	// 82D57F00: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D57F04: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D57F08: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D57F0C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D57F10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D57F14: 4E800421  bctrl
	ctx.lr = 0x82D57F18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D57F18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D57F1C: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82D57F20: 4BF5153C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57F28 size=100
    let mut pc: u32 = 0x82D57F28;
    'dispatch: loop {
        match pc {
            0x82D57F28 => {
    //   block [0x82D57F28..0x82D57F8C)
	// 82D57F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57F2C: 4BF514E1  bl 0x82ca940c
	ctx.lr = 0x82D57F30;
	sub_82CA93D0(ctx, base);
	// 82D57F30: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82D57F34: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82D57F38: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D57F3C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D57F40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D57F44: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82D57F48: 38AB3B98  addi r5, r11, 0x3b98
	ctx.r[5].s64 = ctx.r[11].s64 + 15256;
	// 82D57F4C: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82D57F50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57F54: 480009F5  bl 0x82d58948
	ctx.lr = 0x82D57F58;
	sub_82D58948(ctx, base);
	// 82D57F58: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D57F5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57F60: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57F64: 48000CC5  bl 0x82d58c28
	ctx.lr = 0x82D57F68;
	sub_82D58C28(ctx, base);
	// 82D57F68: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D57F6C: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D57F70: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D57F74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D57F78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D57F7C: 4E800421  bctrl
	ctx.lr = 0x82D57F80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D57F80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D57F84: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82D57F88: 4BF514D4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57F90 size=100
    let mut pc: u32 = 0x82D57F90;
    'dispatch: loop {
        match pc {
            0x82D57F90 => {
    //   block [0x82D57F90..0x82D57FF4)
	// 82D57F90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57F94: 4BF51479  bl 0x82ca940c
	ctx.lr = 0x82D57F98;
	sub_82CA93D0(ctx, base);
	// 82D57F98: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82D57F9C: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82D57FA0: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D57FA4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82D57FA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D57FAC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82D57FB0: 38AB8864  addi r5, r11, -0x779c
	ctx.r[5].s64 = ctx.r[11].s64 + -30620;
	// 82D57FB4: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82D57FB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57FBC: 4800098D  bl 0x82d58948
	ctx.lr = 0x82D57FC0;
	sub_82D58948(ctx, base);
	// 82D57FC0: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D57FC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D57FC8: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D57FCC: 48000C5D  bl 0x82d58c28
	ctx.lr = 0x82D57FD0;
	sub_82D58C28(ctx, base);
	// 82D57FD0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D57FD4: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D57FD8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D57FDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D57FE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D57FE4: 4E800421  bctrl
	ctx.lr = 0x82D57FE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D57FE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D57FEC: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82D57FF0: 4BF5146C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D57FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D57FF8 size=104
    let mut pc: u32 = 0x82D57FF8;
    'dispatch: loop {
        match pc {
            0x82D57FF8 => {
    //   block [0x82D57FF8..0x82D58060)
	// 82D57FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D57FFC: 4BF51411  bl 0x82ca940c
	ctx.lr = 0x82D58000;
	sub_82CA93D0(ctx, base);
	// 82D58000: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82D58004: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82D58008: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5800C: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 82D58010: D8210028  stfd f1, 0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.f[1].u64 ) };
	// 82D58014: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D58018: E8C10028  ld r6, 0x28(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(40 as u32) ) };
	// 82D5801C: 38ABFB84  addi r5, r11, -0x47c
	ctx.r[5].s64 = ctx.r[11].s64 + -1148;
	// 82D58020: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82D58024: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D58028: 48000921  bl 0x82d58948
	ctx.lr = 0x82D5802C;
	sub_82D58948(ctx, base);
	// 82D5802C: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58030: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D58034: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58038: 48000BF1  bl 0x82d58c28
	ctx.lr = 0x82D5803C;
	sub_82D58C28(ctx, base);
	// 82D5803C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D58040: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D58044: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D58048: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5804C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D58050: 4E800421  bctrl
	ctx.lr = 0x82D58054;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D58054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D58058: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82D5805C: 4BF51400  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58060 size=100
    let mut pc: u32 = 0x82D58060;
    'dispatch: loop {
        match pc {
            0x82D58060 => {
    //   block [0x82D58060..0x82D580C4)
	// 82D58060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D58064: 4BF513A9  bl 0x82ca940c
	ctx.lr = 0x82D58068;
	sub_82CA93D0(ctx, base);
	// 82D58068: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82D5806C: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82D58070: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D58074: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D58078: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5807C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82D58080: 38AB3B9C  addi r5, r11, 0x3b9c
	ctx.r[5].s64 = ctx.r[11].s64 + 15260;
	// 82D58084: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82D58088: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5808C: 480008BD  bl 0x82d58948
	ctx.lr = 0x82D58090;
	sub_82D58948(ctx, base);
	// 82D58090: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58094: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D58098: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5809C: 48000B8D  bl 0x82d58c28
	ctx.lr = 0x82D580A0;
	sub_82D58C28(ctx, base);
	// 82D580A0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D580A4: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D580A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D580AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D580B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D580B4: 4E800421  bctrl
	ctx.lr = 0x82D580B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D580B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D580BC: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82D580C0: 4BF5139C  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D580C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D580C8 size=100
    let mut pc: u32 = 0x82D580C8;
    'dispatch: loop {
        match pc {
            0x82D580C8 => {
    //   block [0x82D580C8..0x82D5812C)
	// 82D580C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D580CC: 4BF51341  bl 0x82ca940c
	ctx.lr = 0x82D580D0;
	sub_82CA93D0(ctx, base);
	// 82D580D0: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82D580D4: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82D580D8: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D580DC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82D580E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D580E4: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 82D580E8: 38AB681C  addi r5, r11, 0x681c
	ctx.r[5].s64 = ctx.r[11].s64 + 26652;
	// 82D580EC: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82D580F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D580F4: 48000855  bl 0x82d58948
	ctx.lr = 0x82D580F8;
	sub_82D58948(ctx, base);
	// 82D580F8: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D580FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D58100: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58104: 48000B25  bl 0x82d58c28
	ctx.lr = 0x82D58108;
	sub_82D58C28(ctx, base);
	// 82D58108: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D5810C: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D58110: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D58114: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D58118: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5811C: 4E800421  bctrl
	ctx.lr = 0x82D58120;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D58120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D58124: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82D58128: 4BF51334  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58130 size=152
    let mut pc: u32 = 0x82D58130;
    'dispatch: loop {
        match pc {
            0x82D58130 => {
    //   block [0x82D58130..0x82D581C8)
	// 82D58130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D58134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D58138: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5813C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D58140: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82D58144: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82D58148: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82D5814C: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82D58150: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82D58154: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82D58158: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 82D5815C: E981E000  ld r12, -0x2000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8192 as u32) ) };
	// 82D58160: 9421D860  stwu r1, -0x27a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-10144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D58164: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D58168: 394127C0  addi r10, r1, 0x27c0
	ctx.r[10].s64 = ctx.r[1].s64 + 10176;
	// 82D5816C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D58170: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82D58174: 38802728  li r4, 0x2728
	ctx.r[4].s64 = 10024;
	// 82D58178: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5817C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D58180: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D58184: 480007BD  bl 0x82d58940
	ctx.lr = 0x82D58188;
	sub_82D58940(ctx, base);
	// 82D58188: 83FF0008  lwz r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5818C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D58190: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58194: 48000A95  bl 0x82d58c28
	ctx.lr = 0x82D58198;
	sub_82D58C28(ctx, base);
	// 82D58198: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D5819C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D581A0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82D581A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D581A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D581AC: 4E800421  bctrl
	ctx.lr = 0x82D581B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D581B0: 382127A0  addi r1, r1, 0x27a0
	ctx.r[1].s64 = ctx.r[1].s64 + 10144;
	// 82D581B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D581B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D581BC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D581C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D581C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D581C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D581C8 size=20
    let mut pc: u32 = 0x82D581C8;
    'dispatch: loop {
        match pc {
            0x82D581C8 => {
    //   block [0x82D581C8..0x82D581DC)
	// 82D581C8: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D581CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D581D0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D581D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D581D8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D581E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D581E0 size=20
    let mut pc: u32 = 0x82D581E0;
    'dispatch: loop {
        match pc {
            0x82D581E0 => {
    //   block [0x82D581E0..0x82D581F4)
	// 82D581E0: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D581E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D581E8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D581EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D581F0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D581F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D581F8 size=96
    let mut pc: u32 = 0x82D581F8;
    'dispatch: loop {
        match pc {
            0x82D581F8 => {
    //   block [0x82D581F8..0x82D58258)
	// 82D581F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D581FC: 4BF5120D  bl 0x82ca9408
	ctx.lr = 0x82D58200;
	sub_82CA93D0(ctx, base);
	// 82D58200: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D58204: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D58208: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82D5820C: 3BABD400  addi r29, r11, -0x2c00
	ctx.r[29].s64 = ctx.r[11].s64 + -11264;
	// 82D58210: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58214: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D58218: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5821C: 48000A0D  bl 0x82d58c28
	ctx.lr = 0x82D58220;
	sub_82D58C28(ctx, base);
	// 82D58220: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82D58224: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D58228: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D5822C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D58230: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D58234: 4E800421  bctrl
	ctx.lr = 0x82D58238;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D58238: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5823C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58240: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D58244: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D58248: 4E800421  bctrl
	ctx.lr = 0x82D5824C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5824C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D58250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D58254: 4BF51204  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58258 size=64
    let mut pc: u32 = 0x82D58258;
    'dispatch: loop {
        match pc {
            0x82D58258 => {
    //   block [0x82D58258..0x82D58298)
	// 82D58258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5825C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D58260: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D58264: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D58268: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5826C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58270: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58274: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D58278: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5827C: 4E800421  bctrl
	ctx.lr = 0x82D58280;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D58280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D58284: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D58288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5828C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D58290: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D58294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58298 size=32
    let mut pc: u32 = 0x82D58298;
    'dispatch: loop {
        match pc {
            0x82D58298 => {
    //   block [0x82D58298..0x82D582B8)
	// 82D58298: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5829C: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82D582A0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D582A4: 396B3BD4  addi r11, r11, 0x3bd4
	ctx.r[11].s64 = ctx.r[11].s64 + 15316;
	// 82D582A8: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82D582AC: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D582B0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D582B4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D582B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D582B8 size=12
    let mut pc: u32 = 0x82D582B8;
    'dispatch: loop {
        match pc {
            0x82D582B8 => {
    //   block [0x82D582B8..0x82D582C4)
	// 82D582B8: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D582BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D582C0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D582C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D582C4 size=16
    let mut pc: u32 = 0x82D582C4;
    'dispatch: loop {
        match pc {
            0x82D582C4 => {
    //   block [0x82D582C4..0x82D582D4)
	// 82D582C4: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D582C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D582CC: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D582D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D582D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D582D8 size=96
    let mut pc: u32 = 0x82D582D8;
    'dispatch: loop {
        match pc {
            0x82D582D8 => {
    //   block [0x82D582D8..0x82D58338)
	// 82D582D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D582DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D582E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D582E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D582E8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D582EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D582F0: 396B3BD4  addi r11, r11, 0x3bd4
	ctx.r[11].s64 = ctx.r[11].s64 + 15316;
	// 82D582F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D582F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D582FC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D58300: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D58304: 806B7700  lwz r3, 0x7700(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30464 as u32) ) } as u64;
	// 82D58308: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5830C: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D58310: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D58314: 4E800421  bctrl
	ctx.lr = 0x82D58318;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D58318: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5831C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D58320: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D58324: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D58328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5832C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D58330: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D58334: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58338 size=116
    let mut pc: u32 = 0x82D58338;
    'dispatch: loop {
        match pc {
            0x82D58338 => {
    //   block [0x82D58338..0x82D583AC)
	// 82D58338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5833C: 4BF510CD  bl 0x82ca9408
	ctx.lr = 0x82D58340;
	sub_82CA93D0(ctx, base);
	// 82D58340: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D58344: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D58348: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5834C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D58350: 396B3BD4  addi r11, r11, 0x3bd4
	ctx.r[11].s64 = ctx.r[11].s64 + 15316;
	// 82D58354: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D58358: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 82D5835C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D58360: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D58364: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 82D58368: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5836C: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82D58370: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82D58374: 7C68502E  lwzx r3, r8, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D58378: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82D5837C: 4BFFCECD  bl 0x82d55248
	ctx.lr = 0x82D58380;
	sub_82D55248(ctx, base);
	// 82D58380: 3960001C  li r11, 0x1c
	ctx.r[11].s64 = 28;
	// 82D58384: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82D58388: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82D5838C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D58390: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82D58394: 4800794D  bl 0x82d5fce0
	ctx.lr = 0x82D58398;
	sub_82D5FCE0(ctx, base);
	// 82D58398: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5839C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D583A0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D583A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D583A8: 4BF510B0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D583B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D583B0 size=128
    let mut pc: u32 = 0x82D583B0;
    'dispatch: loop {
        match pc {
            0x82D583B0 => {
    //   block [0x82D583B0..0x82D58430)
	// 82D583B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D583B4: 4BF51055  bl 0x82ca9408
	ctx.lr = 0x82D583B8;
	sub_82CA93D0(ctx, base);
	// 82D583B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D583BC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D583C0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D583C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D583C8: 396B3BD4  addi r11, r11, 0x3bd4
	ctx.r[11].s64 = ctx.r[11].s64 + 15316;
	// 82D583CC: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 82D583D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D583D4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D583D8: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 82D583DC: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82D583E0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D583E4: B39E0006  sth r28, 6(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(6 as u32), ctx.r[28].u16 ) };
	// 82D583E8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D583EC: 4BFFCE5D  bl 0x82d55248
	ctx.lr = 0x82D583F0;
	sub_82D55248(ctx, base);
	// 82D583F0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D583F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D583F8: 396B3BA8  addi r11, r11, 0x3ba8
	ctx.r[11].s64 = ctx.r[11].s64 + 15272;
	// 82D583FC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82D58400: B39F0006  sth r28, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[28].u16 ) };
	// 82D58404: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D58408: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82D5840C: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82D58410: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D58414: 939F0010  stw r28, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[28].u32 ) };
	// 82D58418: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D5841C: 4800017D  bl 0x82d58598
	ctx.lr = 0x82D58420;
	sub_82D58598(ctx, base);
	// 82D58420: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D58424: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82D58428: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5842C: 4BF5102C  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58430 size=132
    let mut pc: u32 = 0x82D58430;
    'dispatch: loop {
        match pc {
            0x82D58430 => {
    //   block [0x82D58430..0x82D584B4)
	// 82D58430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D58434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D58438: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5843C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D58440: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D58444: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D58448: 396B3BD4  addi r11, r11, 0x3bd4
	ctx.r[11].s64 = ctx.r[11].s64 + 15316;
	// 82D5844C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58450: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D58454: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D58458: 419A003C  beq cr6, 0x82d58494
	if ctx.cr[6].eq {
	pc = 0x82D58494; continue 'dispatch;
	}
	// 82D5845C: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D58460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D58464: 419A0030  beq cr6, 0x82d58494
	if ctx.cr[6].eq {
	pc = 0x82D58494; continue 'dispatch;
	}
	// 82D58468: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D5846C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D58470: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D58474: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D58478: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D5847C: 409A0018  bne cr6, 0x82d58494
	if !ctx.cr[6].eq {
	pc = 0x82D58494; continue 'dispatch;
	}
	// 82D58480: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58484: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D58488: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5848C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D58490: 4E800421  bctrl
	ctx.lr = 0x82D58494;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D58494: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D58498: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D5849C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D584A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D584A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D584A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D584AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D584B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D584B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D584B8 size=140
    let mut pc: u32 = 0x82D584B8;
    'dispatch: loop {
        match pc {
            0x82D584B8 => {
    //   block [0x82D584B8..0x82D58544)
	// 82D584B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D584BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D584C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D584C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D584C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D584CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D584D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D584D4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D584D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D584DC: 419A0010  beq cr6, 0x82d584ec
	if ctx.cr[6].eq {
	pc = 0x82D584EC; continue 'dispatch;
	}
	// 82D584E0: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D584E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D584E8: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D584EC: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D584F0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D584F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D584F8: 419A0030  beq cr6, 0x82d58528
	if ctx.cr[6].eq {
	pc = 0x82D58528; continue 'dispatch;
	}
	// 82D584FC: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D58500: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D58504: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D58508: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5850C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D58510: 409A0018  bne cr6, 0x82d58528
	if !ctx.cr[6].eq {
	pc = 0x82D58528; continue 'dispatch;
	}
	// 82D58514: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58518: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D5851C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58520: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D58524: 4E800421  bctrl
	ctx.lr = 0x82D58528;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D58528: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82D5852C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D58530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D58534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D58538: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D5853C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D58540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58548 size=76
    let mut pc: u32 = 0x82D58548;
    'dispatch: loop {
        match pc {
            0x82D58548 => {
    //   block [0x82D58548..0x82D58594)
	// 82D58548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5854C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D58550: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D58554: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D58558: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5855C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D58560: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58564: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 82D58568: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5856C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58570: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D58574: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D58578: 4E800421  bctrl
	ctx.lr = 0x82D5857C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5857C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D58580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D58584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D58588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5858C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D58590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58598 size=116
    let mut pc: u32 = 0x82D58598;
    'dispatch: loop {
        match pc {
            0x82D58598 => {
    //   block [0x82D58598..0x82D5860C)
	// 82D58598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5859C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D585A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D585A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D585A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D585AC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D585B0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D585B4: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D585B8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D585BC: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82D585C0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D585C4: 40980020  bge cr6, 0x82d585e4
	if !ctx.cr[6].lt {
	pc = 0x82D585E4; continue 'dispatch;
	}
	// 82D585C8: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D585CC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D585D0: 40980008  bge cr6, 0x82d585d8
	if !ctx.cr[6].lt {
	pc = 0x82D585D8; continue 'dispatch;
	}
	// 82D585D4: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82D585D8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D585DC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D585E0: 4BFFE931  bl 0x82d56f10
	ctx.lr = 0x82D585E4;
	sub_82D56F10(ctx, base);
	// 82D585E4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D585E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D585EC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D585F0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D585F4: 7D4959AE  stbx r10, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 82D585F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D585FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D58600: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D58604: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D58608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58610 size=20
    let mut pc: u32 = 0x82D58610;
    'dispatch: loop {
        match pc {
            0x82D58610 => {
    //   block [0x82D58610..0x82D58624)
	// 82D58610: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58614: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D58618: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D5861C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D58620: 4BFFFF78  b 0x82d58598
	sub_82D58598(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58628 size=212
    let mut pc: u32 = 0x82D58628;
    'dispatch: loop {
        match pc {
            0x82D58628 => {
    //   block [0x82D58628..0x82D586FC)
	// 82D58628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5862C: 4BF50DDD  bl 0x82ca9408
	ctx.lr = 0x82D58630;
	sub_82CA93D0(ctx, base);
	// 82D58630: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D58634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D58638: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D5863C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D58640: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58644: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D58648: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5864C: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D58650: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D58654: 40990058  ble cr6, 0x82d586ac
	if !ctx.cr[6].gt {
	pc = 0x82D586AC; continue 'dispatch;
	}
	// 82D58658: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D5865C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58660: 7FABF214  add r29, r11, r30
	ctx.r[29].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D58664: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82D58668: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82D5866C: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D58670: 40980020  bge cr6, 0x82d58690
	if !ctx.cr[6].lt {
	pc = 0x82D58690; continue 'dispatch;
	}
	// 82D58674: 554A083C  slwi r10, r10, 1
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D58678: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D5867C: 40980008  bge cr6, 0x82d58684
	if !ctx.cr[6].lt {
	pc = 0x82D58684; continue 'dispatch;
	}
	// 82D58680: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82D58684: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D58688: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D5868C: 4BFFE885  bl 0x82d56f10
	ctx.lr = 0x82D58690;
	sub_82D56F10(ctx, base);
	// 82D58690: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58694: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D58698: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82D5869C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D586A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D586A4: 7D4BE9AE  stbx r10, r11, r29
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[29].u32), ctx.r[10].u8) };
	// 82D586A8: 48000020  b 0x82d586c8
	pc = 0x82D586C8; continue 'dispatch;
	// 82D586AC: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D586B0: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82D586B4: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D586B8: 40990010  ble cr6, 0x82d586c8
	if !ctx.cr[6].gt {
	pc = 0x82D586C8; continue 'dispatch;
	}
	// 82D586BC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D586C0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D586C4: 7D2A59AE  stbx r9, r10, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u8) };
	// 82D586C8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D586CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D586D0: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D586D4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D586D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D586DC: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D586E0: 48000651  bl 0x82d58d30
	ctx.lr = 0x82D586E4;
	sub_82D58D30(ctx, base);
	// 82D586E4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D586E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D586EC: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82D586F0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82D586F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D586F8: 4BF50D60  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58700 size=12
    let mut pc: u32 = 0x82D58700;
    'dispatch: loop {
        match pc {
            0x82D58700 => {
    //   block [0x82D58700..0x82D5870C)
	// 82D58700: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D58704: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D58708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58710 size=236
    let mut pc: u32 = 0x82D58710;
    'dispatch: loop {
        match pc {
            0x82D58710 => {
    //   block [0x82D58710..0x82D587FC)
	// 82D58710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D58714: 4BF50CF1  bl 0x82ca9404
	ctx.lr = 0x82D58718;
	sub_82CA93D0(ctx, base);
	// 82D58718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5871C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82D58720: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 82D58724: 817B000C  lwz r11, 0xc(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D58728: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82D5872C: 41980028  blt cr6, 0x82d58754
	if ctx.cr[6].lt {
	pc = 0x82D58754; continue 'dispatch;
	}
	// 82D58730: 419A001C  beq cr6, 0x82d5874c
	if ctx.cr[6].eq {
	pc = 0x82D5874C; continue 'dispatch;
	}
	// 82D58734: 2B050003  cmplwi cr6, r5, 3
	ctx.cr[6].compare_u32(ctx.r[5].u32, 3 as u32, &mut ctx.xer);
	// 82D58738: 40980020  bge cr6, 0x82d58758
	if !ctx.cr[6].lt {
	pc = 0x82D58758; continue 'dispatch;
	}
	// 82D5873C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58740: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D58744: 7F845850  subf r28, r4, r11
	ctx.r[28].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 82D58748: 48000010  b 0x82d58758
	pc = 0x82D58758; continue 'dispatch;
	// 82D5874C: 7F8B2214  add r28, r11, r4
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82D58750: 48000008  b 0x82d58758
	pc = 0x82D58758; continue 'dispatch;
	// 82D58754: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D58758: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82D5875C: 41980094  blt cr6, 0x82d587f0
	if ctx.cr[6].lt {
	pc = 0x82D587F0; continue 'dispatch;
	}
	// 82D58760: 83DB0008  lwz r30, 8(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58764: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D58768: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5876C: 40990074  ble cr6, 0x82d587e0
	if !ctx.cr[6].gt {
	pc = 0x82D587E0; continue 'dispatch;
	}
	// 82D58770: 3BFC0001  addi r31, r28, 1
	ctx.r[31].s64 = ctx.r[28].s64 + 1;
	// 82D58774: 557D003E  slwi r29, r11, 0
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82D58778: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82D5877C: 40990058  ble cr6, 0x82d587d4
	if !ctx.cr[6].gt {
	pc = 0x82D587D4; continue 'dispatch;
	}
	// 82D58780: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58784: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D58788: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D5878C: 40980024  bge cr6, 0x82d587b0
	if !ctx.cr[6].lt {
	pc = 0x82D587B0; continue 'dispatch;
	}
	// 82D58790: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D58794: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D58798: 41980008  blt cr6, 0x82d587a0
	if ctx.cr[6].lt {
	pc = 0x82D587A0; continue 'dispatch;
	}
	// 82D5879C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D587A0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D587A4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D587A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D587AC: 4BFFE765  bl 0x82d56f10
	ctx.lr = 0x82D587B0;
	sub_82D56F10(ctx, base);
	// 82D587B0: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82D587B4: 7F1DF800  cmpw cr6, r29, r31
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D587B8: 4098001C  bge cr6, 0x82d587d4
	if !ctx.cr[6].lt {
	pc = 0x82D587D4; continue 'dispatch;
	}
	// 82D587BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D587C0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D587C4: 7D4B49AE  stbx r10, r11, r9
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[10].u8) };
	// 82D587C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D587CC: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D587D0: 4198FFF0  blt cr6, 0x82d587c0
	if ctx.cr[6].lt {
	pc = 0x82D587C0; continue 'dispatch;
	}
	// 82D587D4: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82D587D8: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D587DC: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82D587E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D587E4: 939B000C  stw r28, 0xc(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82D587E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D587EC: 4BF50C68  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D587F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D587F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D587F8: 4BF50C5C  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58800 size=100
    let mut pc: u32 = 0x82D58800;
    'dispatch: loop {
        match pc {
            0x82D58800 => {
    //   block [0x82D58800..0x82D58864)
	// 82D58800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D58804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D58808: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5880C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D58810: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D58814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D58818: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5881C: 4BFFFC15  bl 0x82d58430
	ctx.lr = 0x82D58820;
	sub_82D58430(ctx, base);
	// 82D58820: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D58824: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D58828: 419A0020  beq cr6, 0x82d58848
	if ctx.cr[6].eq {
	pc = 0x82D58848; continue 'dispatch;
	}
	// 82D5882C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58830: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D58834: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 82D58838: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5883C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D58840: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D58844: 4BFFCA85  bl 0x82d552c8
	ctx.lr = 0x82D58848;
	sub_82D552C8(ctx, base);
	// 82D58848: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5884C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D58850: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D58854: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D58858: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D5885C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D58860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58868 size=152
    let mut pc: u32 = 0x82D58868;
    'dispatch: loop {
        match pc {
            0x82D58868 => {
    //   block [0x82D58868..0x82D58900)
	// 82D58868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5886C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D58870: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D58874: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D58878: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5887C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D58880: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D58884: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D58888: 396B3BA8  addi r11, r11, 0x3ba8
	ctx.r[11].s64 = ctx.r[11].s64 + 15272;
	// 82D5888C: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D58890: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D58894: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D58898: 409A0018  bne cr6, 0x82d588b0
	if !ctx.cr[6].eq {
	pc = 0x82D588B0; continue 'dispatch;
	}
	// 82D5889C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D588A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D588A4: 419A000C  beq cr6, 0x82d588b0
	if ctx.cr[6].eq {
	pc = 0x82D588B0; continue 'dispatch;
	}
	// 82D588A8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D588AC: 480DF375  bl 0x82e37c20
	ctx.lr = 0x82D588B0;
	sub_82E37C20(ctx, base);
	// 82D588B0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D588B4: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D588B8: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D588BC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D588C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D588C4: 419A0020  beq cr6, 0x82d588e4
	if ctx.cr[6].eq {
	pc = 0x82D588E4; continue 'dispatch;
	}
	// 82D588C8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D588CC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D588D0: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 82D588D4: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D588D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D588DC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D588E0: 4BFFC9E9  bl 0x82d552c8
	ctx.lr = 0x82D588E4;
	sub_82D552C8(ctx, base);
	// 82D588E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D588E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D588EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D588F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D588F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D588F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D588FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58900 size=12
    let mut pc: u32 = 0x82D58900;
    'dispatch: loop {
        match pc {
            0x82D58900 => {
    //   block [0x82D58900..0x82D5890C)
	// 82D58900: 7C6B0774  extsb r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	// 82D58904: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 82D58908: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5890C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5890C size=8
    let mut pc: u32 = 0x82D5890C;
    'dispatch: loop {
        match pc {
            0x82D5890C => {
    //   block [0x82D5890C..0x82D58914)
	// 82D5890C: 2F0B007A  cmpwi cr6, r11, 0x7a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 122, &mut ctx.xer);
	// 82D58910: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58914(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58914 size=12
    let mut pc: u32 = 0x82D58914;
    'dispatch: loop {
        match pc {
            0x82D58914 => {
    //   block [0x82D58914..0x82D58920)
	// 82D58914: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 82D58918: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82D5891C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58920 size=12
    let mut pc: u32 = 0x82D58920;
    'dispatch: loop {
        match pc {
            0x82D58920 => {
    //   block [0x82D58920..0x82D5892C)
	// 82D58920: 7C6B0774  extsb r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	// 82D58924: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82D58928: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5892C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5892C size=8
    let mut pc: u32 = 0x82D5892C;
    'dispatch: loop {
        match pc {
            0x82D5892C => {
    //   block [0x82D5892C..0x82D58934)
	// 82D5892C: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82D58930: 4D990020  bgtlr cr6
	if ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58934(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58934 size=12
    let mut pc: u32 = 0x82D58934;
    'dispatch: loop {
        match pc {
            0x82D58934 => {
    //   block [0x82D58934..0x82D58940)
	// 82D58934: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82D58938: 7D630774  extsb r3, r11
	ctx.r[3].s64 = ctx.r[11].s8 as i64;
	// 82D5893C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58940 size=4
    let mut pc: u32 = 0x82D58940;
    'dispatch: loop {
        match pc {
            0x82D58940 => {
    //   block [0x82D58940..0x82D58944)
	// 82D58940: 4BF58BD0  b 0x82cb1510
	sub_82CB1510(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58948 size=68
    let mut pc: u32 = 0x82D58948;
    'dispatch: loop {
        match pc {
            0x82D58948 => {
    //   block [0x82D58948..0x82D5898C)
	// 82D58948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5894C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D58950: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82D58954: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82D58958: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82D5895C: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82D58960: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82D58964: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D58968: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D5896C: 39410088  addi r10, r1, 0x88
	ctx.r[10].s64 = ctx.r[1].s64 + 136;
	// 82D58970: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D58974: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D58978: 4BF58B99  bl 0x82cb1510
	ctx.lr = 0x82D5897C;
	sub_82CB1510(ctx, base);
	// 82D5897C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D58980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D58984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D58988: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58990 size=72
    let mut pc: u32 = 0x82D58990;
    'dispatch: loop {
        match pc {
            0x82D58990 => {
    //   block [0x82D58990..0x82D589D8)
	// 82D58990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D58994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D58998: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82D5899C: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82D589A0: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82D589A4: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82D589A8: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82D589AC: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82D589B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D589B4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D589B8: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82D589BC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D589C0: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D589C4: 4B46DE8D  bl 0x821c6850
	ctx.lr = 0x82D589C8;
	sub_821C6850(ctx, base);
	// 82D589C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D589CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D589D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D589D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D589D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D589D8 size=24
    let mut pc: u32 = 0x82D589D8;
    'dispatch: loop {
        match pc {
            0x82D589D8 => {
    //   block [0x82D589D8..0x82D589F0)
	// 82D589D8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82D589DC: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D589E0: 89240000  lbz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D589E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D589E8: 7C695850  subf r3, r9, r11
	ctx.r[3].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 82D589EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D589F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D589F0 size=20
    let mut pc: u32 = 0x82D589F0;
    'dispatch: loop {
        match pc {
            0x82D589F0 => {
    //   block [0x82D589F0..0x82D58A04)
	// 82D589F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D589F4: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82D589F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D589FC: 419AFFE0  beq cr6, 0x82d589dc
	if ctx.cr[6].eq {
		sub_82D589D8(ctx, base);
		return;
	}
	// 82D58A00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58A08 size=4
    let mut pc: u32 = 0x82D58A08;
    'dispatch: loop {
        match pc {
            0x82D58A08 => {
    //   block [0x82D58A08..0x82D58A0C)
	// 82D58A08: 4BF516E8  b 0x82caa0f0
	sub_82CAA0F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58A10 size=60
    let mut pc: u32 = 0x82D58A10;
    'dispatch: loop {
        match pc {
            0x82D58A10 => {
    //   block [0x82D58A10..0x82D58A4C)
	// 82D58A10: 7CE41850  subf r7, r4, r3
	ctx.r[7].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	// 82D58A14: 7D4720AE  lbzx r10, r7, r4
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[4].u32)) } as u64;
	// 82D58A18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D58A1C: 409A0010  bne cr6, 0x82d58a2c
	if !ctx.cr[6].eq {
	pc = 0x82D58A2C; continue 'dispatch;
	}
	// 82D58A20: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58A24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D58A28: 419A00B8  beq cr6, 0x82d58ae0
	if ctx.cr[6].eq {
		sub_82D58AE0(ctx, base);
		return;
	}
	// 82D58A2C: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 82D58A30: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82D58A34: 41980018  blt cr6, 0x82d58a4c
	if ctx.cr[6].lt {
		sub_82D58A4C(ctx, base);
		return;
	}
	// 82D58A38: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82D58A3C: 41990010  bgt cr6, 0x82d58a4c
	if ctx.cr[6].gt {
		sub_82D58A4C(ctx, base);
		return;
	}
	// 82D58A40: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82D58A44: 7D680774  extsb r8, r11
	ctx.r[8].s64 = ctx.r[11].s8 as i64;
	// 82D58A48: 48000008  b 0x82d58a50
	sub_82D58A4C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58A4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58A4C size=40
    let mut pc: u32 = 0x82D58A4C;
    'dispatch: loop {
        match pc {
            0x82D58A4C => {
    //   block [0x82D58A4C..0x82D58A74)
	// 82D58A4C: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82D58A50: 89240000  lbz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58A54: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 82D58A58: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82D58A5C: 41980018  blt cr6, 0x82d58a74
	if ctx.cr[6].lt {
		sub_82D58A74(ctx, base);
		return;
	}
	// 82D58A60: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82D58A64: 41990010  bgt cr6, 0x82d58a74
	if ctx.cr[6].gt {
		sub_82D58A74(ctx, base);
		return;
	}
	// 82D58A68: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82D58A6C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D58A70: 48000008  b 0x82d58a78
	sub_82D58A74(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58A74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58A74 size=80
    let mut pc: u32 = 0x82D58A74;
    'dispatch: loop {
        match pc {
            0x82D58A74 => {
    //   block [0x82D58A74..0x82D58AC4)
	// 82D58A74: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82D58A78: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D58A7C: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 82D58A80: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D58A84: 41980064  blt cr6, 0x82d58ae8
	if ctx.cr[6].lt {
		sub_82D58AE8(ctx, base);
		return;
	}
	// 82D58A88: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 82D58A8C: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82D58A90: 41980014  blt cr6, 0x82d58aa4
	if ctx.cr[6].lt {
	pc = 0x82D58AA4; continue 'dispatch;
	}
	// 82D58A94: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82D58A98: 4199000C  bgt cr6, 0x82d58aa4
	if ctx.cr[6].gt {
	pc = 0x82D58AA4; continue 'dispatch;
	}
	// 82D58A9C: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82D58AA0: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82D58AA4: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 82D58AA8: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82D58AAC: 41980018  blt cr6, 0x82d58ac4
	if ctx.cr[6].lt {
		sub_82D58AC4(ctx, base);
		return;
	}
	// 82D58AB0: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82D58AB4: 41990010  bgt cr6, 0x82d58ac4
	if ctx.cr[6].gt {
		sub_82D58AC4(ctx, base);
		return;
	}
	// 82D58AB8: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82D58ABC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D58AC0: 48000008  b 0x82d58ac8
	sub_82D58AC4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58AC4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58AC4 size=28
    let mut pc: u32 = 0x82D58AC4;
    'dispatch: loop {
        match pc {
            0x82D58AC4 => {
    //   block [0x82D58AC4..0x82D58AE0)
	// 82D58AC4: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82D58AC8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D58ACC: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82D58AD0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D58AD4: 4199001C  bgt cr6, 0x82d58af0
	if ctx.cr[6].gt {
		sub_82D58AF0(ctx, base);
		return;
	}
	// 82D58AD8: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82D58ADC: 4BFFFF38  b 0x82d58a14
	sub_82D58A10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58AE0 size=8
    let mut pc: u32 = 0x82D58AE0;
    'dispatch: loop {
        match pc {
            0x82D58AE0 => {
    //   block [0x82D58AE0..0x82D58AE8)
	// 82D58AE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D58AE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58AE8 size=8
    let mut pc: u32 = 0x82D58AE8;
    'dispatch: loop {
        match pc {
            0x82D58AE8 => {
    //   block [0x82D58AE8..0x82D58AF0)
	// 82D58AE8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82D58AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58AF0 size=8
    let mut pc: u32 = 0x82D58AF0;
    'dispatch: loop {
        match pc {
            0x82D58AF0 => {
    //   block [0x82D58AF0..0x82D58AF8)
	// 82D58AF0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D58AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58AF8 size=76
    let mut pc: u32 = 0x82D58AF8;
    'dispatch: loop {
        match pc {
            0x82D58AF8 => {
    //   block [0x82D58AF8..0x82D58B44)
	// 82D58AF8: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 82D58AFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82D58B00: 7C841850  subf r4, r4, r3
	ctx.r[4].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	// 82D58B04: 7D4438AE  lbzx r10, r4, r7
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[4].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 82D58B08: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D58B0C: 409A0010  bne cr6, 0x82d58b1c
	if !ctx.cr[6].eq {
	pc = 0x82D58B1C; continue 'dispatch;
	}
	// 82D58B10: 89670000  lbz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58B14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D58B18: 419A00D4  beq cr6, 0x82d58bec
	if ctx.cr[6].eq {
		sub_82D58BEC(ctx, base);
		return;
	}
	// 82D58B1C: 7F062800  cmpw cr6, r6, r5
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82D58B20: 409800CC  bge cr6, 0x82d58bec
	if !ctx.cr[6].lt {
		sub_82D58BEC(ctx, base);
		return;
	}
	// 82D58B24: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 82D58B28: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82D58B2C: 41980018  blt cr6, 0x82d58b44
	if ctx.cr[6].lt {
		sub_82D58B44(ctx, base);
		return;
	}
	// 82D58B30: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82D58B34: 41990010  bgt cr6, 0x82d58b44
	if ctx.cr[6].gt {
		sub_82D58B44(ctx, base);
		return;
	}
	// 82D58B38: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82D58B3C: 7D680774  extsb r8, r11
	ctx.r[8].s64 = ctx.r[11].s8 as i64;
	// 82D58B40: 48000008  b 0x82d58b48
	sub_82D58B44(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58B44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58B44 size=40
    let mut pc: u32 = 0x82D58B44;
    'dispatch: loop {
        match pc {
            0x82D58B44 => {
    //   block [0x82D58B44..0x82D58B6C)
	// 82D58B44: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82D58B48: 89270000  lbz r9, 0(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58B4C: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 82D58B50: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82D58B54: 41980018  blt cr6, 0x82d58b6c
	if ctx.cr[6].lt {
		sub_82D58B6C(ctx, base);
		return;
	}
	// 82D58B58: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82D58B5C: 41990010  bgt cr6, 0x82d58b6c
	if ctx.cr[6].gt {
		sub_82D58B6C(ctx, base);
		return;
	}
	// 82D58B60: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82D58B64: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D58B68: 48000008  b 0x82d58b70
	sub_82D58B6C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58B6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58B6C size=80
    let mut pc: u32 = 0x82D58B6C;
    'dispatch: loop {
        match pc {
            0x82D58B6C => {
    //   block [0x82D58B6C..0x82D58BBC)
	// 82D58B6C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82D58B70: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D58B74: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 82D58B78: 7F085800  cmpw cr6, r8, r11
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D58B7C: 41980060  blt cr6, 0x82d58bdc
	if ctx.cr[6].lt {
		sub_82D58BDC(ctx, base);
		return;
	}
	// 82D58B80: 7D4B0774  extsb r11, r10
	ctx.r[11].s64 = ctx.r[10].s8 as i64;
	// 82D58B84: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82D58B88: 41980014  blt cr6, 0x82d58b9c
	if ctx.cr[6].lt {
	pc = 0x82D58B9C; continue 'dispatch;
	}
	// 82D58B8C: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82D58B90: 4199000C  bgt cr6, 0x82d58b9c
	if ctx.cr[6].gt {
	pc = 0x82D58B9C; continue 'dispatch;
	}
	// 82D58B94: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82D58B98: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 82D58B9C: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 82D58BA0: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82D58BA4: 41980018  blt cr6, 0x82d58bbc
	if ctx.cr[6].lt {
		sub_82D58BBC(ctx, base);
		return;
	}
	// 82D58BA8: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82D58BAC: 41990010  bgt cr6, 0x82d58bbc
	if ctx.cr[6].gt {
		sub_82D58BBC(ctx, base);
		return;
	}
	// 82D58BB0: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82D58BB4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D58BB8: 48000008  b 0x82d58bc0
	sub_82D58BBC(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58BBC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58BBC size=32
    let mut pc: u32 = 0x82D58BBC;
    'dispatch: loop {
        match pc {
            0x82D58BBC => {
    //   block [0x82D58BBC..0x82D58BDC)
	// 82D58BBC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82D58BC0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D58BC4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82D58BC8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D58BCC: 41990018  bgt cr6, 0x82d58be4
	if ctx.cr[6].gt {
		sub_82D58BE4(ctx, base);
		return;
	}
	// 82D58BD0: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 82D58BD4: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82D58BD8: 4BFFFF2C  b 0x82d58b04
	sub_82D58AF8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58BDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58BDC size=8
    let mut pc: u32 = 0x82D58BDC;
    'dispatch: loop {
        match pc {
            0x82D58BDC => {
    //   block [0x82D58BDC..0x82D58BE4)
	// 82D58BDC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82D58BE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58BE4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58BE4 size=8
    let mut pc: u32 = 0x82D58BE4;
    'dispatch: loop {
        match pc {
            0x82D58BE4 => {
    //   block [0x82D58BE4..0x82D58BEC)
	// 82D58BE4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D58BE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58BEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58BEC size=8
    let mut pc: u32 = 0x82D58BEC;
    'dispatch: loop {
        match pc {
            0x82D58BEC => {
    //   block [0x82D58BEC..0x82D58BF4)
	// 82D58BEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D58BF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58BF8 size=28
    let mut pc: u32 = 0x82D58BF8;
    'dispatch: loop {
        match pc {
            0x82D58BF8 => {
    //   block [0x82D58BF8..0x82D58C14)
	// 82D58BF8: 7D441850  subf r10, r4, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[4].s64;
	// 82D58BFC: 89640000  lbz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58C00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D58C04: 7D6A21AE  stbx r11, r10, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32), ctx.r[11].u8) };
	// 82D58C08: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 82D58C0C: 409AFFF0  bne cr6, 0x82d58bfc
	if !ctx.cr[6].eq {
	pc = 0x82D58BFC; continue 'dispatch;
	}
	// 82D58C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58C18 size=8
    let mut pc: u32 = 0x82D58C18;
    'dispatch: loop {
        match pc {
            0x82D58C18 => {
    //   block [0x82D58C18..0x82D58C20)
	// 82D58C18: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82D58C1C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58C20 size=8
    let mut pc: u32 = 0x82D58C20;
    'dispatch: loop {
        match pc {
            0x82D58C20 => {
    //   block [0x82D58C20..0x82D58C28)
	// 82D58C20: 4B574488  b 0x822cd0a8
	sub_822CD0A8(ctx, base);
	return;
	// 82D58C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58C28 size=36
    let mut pc: u32 = 0x82D58C28;
    'dispatch: loop {
        match pc {
            0x82D58C28 => {
    //   block [0x82D58C28..0x82D58C4C)
	// 82D58C28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D58C2C: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58C30: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 82D58C34: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D58C38: 409AFFF4  bne cr6, 0x82d58c2c
	if !ctx.cr[6].eq {
	pc = 0x82D58C2C; continue 'dispatch;
	}
	// 82D58C3C: 7D6B1850  subf r11, r11, r3
	ctx.r[11].s64 = ctx.r[3].s64 - ctx.r[11].s64;
	// 82D58C40: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D58C44: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82D58C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58C50 size=12
    let mut pc: u32 = 0x82D58C50;
    'dispatch: loop {
        match pc {
            0x82D58C50 => {
    //   block [0x82D58C50..0x82D58C5C)
	// 82D58C50: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D58C54: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D58C58: 4BF53ED0  b 0x82cacb28
	sub_82CACB28(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58C60 size=40
    let mut pc: u32 = 0x82D58C60;
    'dispatch: loop {
        match pc {
            0x82D58C60 => {
    //   block [0x82D58C60..0x82D58C88)
	// 82D58C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D58C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D58C68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D58C6C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D58C70: 4BF57521  bl 0x82cb0190
	ctx.lr = 0x82D58C74;
	sub_82CB0190(ctx, base);
	// 82D58C74: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82D58C78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D58C7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D58C80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D58C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58C88 size=4
    let mut pc: u32 = 0x82D58C88;
    'dispatch: loop {
        match pc {
            0x82D58C88 => {
    //   block [0x82D58C88..0x82D58C8C)
	// 82D58C88: 4B48DB40  b 0x821e67c8
	sub_821E67C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58C90 size=4
    let mut pc: u32 = 0x82D58C90;
    'dispatch: loop {
        match pc {
            0x82D58C90 => {
    //   block [0x82D58C90..0x82D58C94)
	// 82D58C90: 4BF51580  b 0x82caa210
	sub_82CAA210(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58C98 size=4
    let mut pc: u32 = 0x82D58C98;
    'dispatch: loop {
        match pc {
            0x82D58C98 => {
    //   block [0x82D58C98..0x82D58C9C)
	// 82D58C98: 4BF54BF8  b 0x82cad890
	sub_82CAD890(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58CA0 size=12
    let mut pc: u32 = 0x82D58CA0;
    'dispatch: loop {
        match pc {
            0x82D58CA0 => {
    //   block [0x82D58CA0..0x82D58CAC)
	// 82D58CA0: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58CA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D58CA8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58CAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58CAC size=60
    let mut pc: u32 = 0x82D58CAC;
    'dispatch: loop {
        match pc {
            0x82D58CAC => {
    //   block [0x82D58CAC..0x82D58CE8)
	// 82D58CAC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82D58CB0: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58CB4: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 82D58CB8: 2F0B0041  cmpwi cr6, r11, 0x41
	ctx.cr[6].compare_i32(ctx.r[11].s32, 65, &mut ctx.xer);
	// 82D58CBC: 41980014  blt cr6, 0x82d58cd0
	if ctx.cr[6].lt {
	pc = 0x82D58CD0; continue 'dispatch;
	}
	// 82D58CC0: 2F0B005A  cmpwi cr6, r11, 0x5a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 90, &mut ctx.xer);
	// 82D58CC4: 4199000C  bgt cr6, 0x82d58cd0
	if ctx.cr[6].gt {
	pc = 0x82D58CD0; continue 'dispatch;
	}
	// 82D58CC8: 396B0020  addi r11, r11, 0x20
	ctx.r[11].s64 = ctx.r[11].s64 + 32;
	// 82D58CCC: 7D690774  extsb r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	// 82D58CD0: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82D58CD4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D58CD8: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58CDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D58CE0: 409AFFD0  bne cr6, 0x82d58cb0
	if !ctx.cr[6].eq {
	pc = 0x82D58CB0; continue 'dispatch;
	}
	// 82D58CE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58CE8 size=12
    let mut pc: u32 = 0x82D58CE8;
    'dispatch: loop {
        match pc {
            0x82D58CE8 => {
    //   block [0x82D58CE8..0x82D58CF4)
	// 82D58CE8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58CEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D58CF0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58CF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58CF4 size=60
    let mut pc: u32 = 0x82D58CF4;
    'dispatch: loop {
        match pc {
            0x82D58CF4 => {
    //   block [0x82D58CF4..0x82D58D30)
	// 82D58CF4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82D58CF8: 892A0000  lbz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58CFC: 7D2B0774  extsb r11, r9
	ctx.r[11].s64 = ctx.r[9].s8 as i64;
	// 82D58D00: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 82D58D04: 41980014  blt cr6, 0x82d58d18
	if ctx.cr[6].lt {
	pc = 0x82D58D18; continue 'dispatch;
	}
	// 82D58D08: 2F0B007A  cmpwi cr6, r11, 0x7a
	ctx.cr[6].compare_i32(ctx.r[11].s32, 122, &mut ctx.xer);
	// 82D58D0C: 4199000C  bgt cr6, 0x82d58d18
	if ctx.cr[6].gt {
	pc = 0x82D58D18; continue 'dispatch;
	}
	// 82D58D10: 396BFFE0  addi r11, r11, -0x20
	ctx.r[11].s64 = ctx.r[11].s64 + -32;
	// 82D58D14: 7D690774  extsb r9, r11
	ctx.r[9].s64 = ctx.r[11].s8 as i64;
	// 82D58D18: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82D58D1C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D58D20: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58D24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D58D28: 409AFFD0  bne cr6, 0x82d58cf8
	if !ctx.cr[6].eq {
	pc = 0x82D58CF8; continue 'dispatch;
	}
	// 82D58D2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58D30 size=4
    let mut pc: u32 = 0x82D58D30;
    'dispatch: loop {
        match pc {
            0x82D58D30 => {
    //   block [0x82D58D30..0x82D58D34)
	// 82D58D30: 4BF50750  b 0x82ca9480
	sub_82CA9480(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58D38 size=4
    let mut pc: u32 = 0x82D58D38;
    'dispatch: loop {
        match pc {
            0x82D58D38 => {
    //   block [0x82D58D38..0x82D58D3C)
	// 82D58D38: 4BF57DF8  b 0x82cb0b30
	sub_82CB0B30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58D40 size=4
    let mut pc: u32 = 0x82D58D40;
    'dispatch: loop {
        match pc {
            0x82D58D40 => {
    //   block [0x82D58D40..0x82D58D44)
	// 82D58D40: 4BF50C70  b 0x82ca99b0
	sub_82CA99B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58D48 size=20
    let mut pc: u32 = 0x82D58D48;
    'dispatch: loop {
        match pc {
            0x82D58D48 => {
    //   block [0x82D58D48..0x82D58D5C)
	// 82D58D48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D58D4C: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82D58D50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D58D54: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82D58D58: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58D5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58D5C size=20
    let mut pc: u32 = 0x82D58D5C;
    'dispatch: loop {
        match pc {
            0x82D58D5C => {
    //   block [0x82D58D5C..0x82D58D70)
	// 82D58D5C: 7D2B2A14  add r9, r11, r5
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82D58D60: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58D64: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58D68: 7C674051  subf. r3, r7, r8
	ctx.r[3].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D58D6C: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58D70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58D70 size=20
    let mut pc: u32 = 0x82D58D70;
    'dispatch: loop {
        match pc {
            0x82D58D70 => {
    //   block [0x82D58D70..0x82D58D84)
	// 82D58D70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D58D74: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82D58D78: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D58D7C: 409AFFE4  bne cr6, 0x82d58d60
	if !ctx.cr[6].eq {
		sub_82D58D5C(ctx, base);
		return;
	}
	// 82D58D80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58D88 size=128
    let mut pc: u32 = 0x82D58D88;
    'dispatch: loop {
        match pc {
            0x82D58D88 => {
    //   block [0x82D58D88..0x82D58E08)
	// 82D58D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D58D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D58D90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D58D94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D58D98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D58D9C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D58DA0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82D58DA4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58DA8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D58DAC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D58DB0: 409AFFF4  bne cr6, 0x82d58da4
	if !ctx.cr[6].eq {
	pc = 0x82D58DA4; continue 'dispatch;
	}
	// 82D58DB4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D58DB8: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58DBC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D58DC0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D58DC4: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82D58DC8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D58DCC: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	// 82D58DD0: 7C6A482E  lwzx r3, r10, r9
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82D58DD4: 4BFFC6A5  bl 0x82d55478
	ctx.lr = 0x82D58DD8;
	sub_82D55478(ctx, base);
	// 82D58DD8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D58DDC: 7D3F1850  subf r9, r31, r3
	ctx.r[9].s64 = ctx.r[3].s64 - ctx.r[31].s64;
	// 82D58DE0: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58DE4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D58DE8: 7D4959AE  stbx r10, r9, r11
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u8) };
	// 82D58DEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D58DF0: 409AFFF0  bne cr6, 0x82d58de0
	if !ctx.cr[6].eq {
	pc = 0x82D58DE0; continue 'dispatch;
	}
	// 82D58DF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D58DF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D58DFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D58E00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D58E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58E08 size=132
    let mut pc: u32 = 0x82D58E08;
    'dispatch: loop {
        match pc {
            0x82D58E08 => {
    //   block [0x82D58E08..0x82D58E8C)
	// 82D58E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D58E0C: 4BF50601  bl 0x82ca940c
	ctx.lr = 0x82D58E10;
	sub_82CA93D0(ctx, base);
	// 82D58E10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D58E14: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D58E18: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82D58E1C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82D58E20: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58E24: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D58E28: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D58E2C: 409AFFF4  bne cr6, 0x82d58e20
	if !ctx.cr[6].eq {
	pc = 0x82D58E20; continue 'dispatch;
	}
	// 82D58E30: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D58E34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D58E38: 557F003E  slwi r31, r11, 0
	ctx.r[31].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82D58E3C: 7F1F2000  cmpw cr6, r31, r4
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82D58E40: 40990008  ble cr6, 0x82d58e48
	if !ctx.cr[6].gt {
	pc = 0x82D58E48; continue 'dispatch;
	}
	// 82D58E44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D58E48: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58E4C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D58E50: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82D58E54: 389F0001  addi r4, r31, 1
	ctx.r[4].s64 = ctx.r[31].s64 + 1;
	// 82D58E58: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D58E5C: 4BFFC61D  bl 0x82d55478
	ctx.lr = 0x82D58E60;
	sub_82D55478(ctx, base);
	// 82D58E60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D58E64: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D58E68: 419A0010  beq cr6, 0x82d58e78
	if ctx.cr[6].eq {
	pc = 0x82D58E78; continue 'dispatch;
	}
	// 82D58E6C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D58E70: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D58E74: 4B574235  bl 0x822cd0a8
	ctx.lr = 0x82D58E78;
	sub_822CD0A8(ctx, base);
	// 82D58E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D58E7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D58E80: 7D7EF9AE  stbx r11, r30, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32), ctx.r[11].u8) };
	// 82D58E84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D58E88: 4BF505D4  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D58E90 size=344
    let mut pc: u32 = 0x82D58E90;
    'dispatch: loop {
        match pc {
            0x82D58E90 => {
    //   block [0x82D58E90..0x82D58FE8)
	// 82D58E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D58E94: 4BF50575  bl 0x82ca9408
	ctx.lr = 0x82D58E98;
	sub_82CA93D0(ctx, base);
	// 82D58E98: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 82D58E9C: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 82D58EA0: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 82D58EA4: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 82D58EA8: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 82D58EAC: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 82D58EB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D58EB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D58EB8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82D58EBC: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58EC0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D58EC4: 2F0B003F  cmpwi cr6, r11, 0x3f
	ctx.cr[6].compare_i32(ctx.r[11].s32, 63, &mut ctx.xer);
	// 82D58EC8: 4098002C  bge cr6, 0x82d58ef4
	if !ctx.cr[6].lt {
	pc = 0x82D58EF4; continue 'dispatch;
	}
	// 82D58ECC: 2F0B0100  cmpwi cr6, r11, 0x100
	ctx.cr[6].compare_i32(ctx.r[11].s32, 256, &mut ctx.xer);
	// 82D58ED0: 40980024  bge cr6, 0x82d58ef4
	if !ctx.cr[6].lt {
	pc = 0x82D58EF4; continue 'dispatch;
	}
	// 82D58ED4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D58ED8: 2F0B0100  cmpwi cr6, r11, 0x100
	ctx.cr[6].compare_i32(ctx.r[11].s32, 256, &mut ctx.xer);
	// 82D58EDC: 41990008  bgt cr6, 0x82d58ee4
	if ctx.cr[6].gt {
	pc = 0x82D58EE4; continue 'dispatch;
	}
	// 82D58EE0: 39600100  li r11, 0x100
	ctx.r[11].s64 = 256;
	// 82D58EE4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D58EE8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D58EEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D58EF0: 4BFFE021  bl 0x82d56f10
	ctx.lr = 0x82D58EF4;
	sub_82D56F10(ctx, base);
	// 82D58EF4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D58EF8: 394100A0  addi r10, r1, 0xa0
	ctx.r[10].s64 = ctx.r[1].s64 + 160;
	// 82D58EFC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82D58F00: 83A10050  lwz r29, 0x50(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D58F04: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58F08: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82D58F0C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82D58F10: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D58F14: 557F00BE  clrlwi r31, r11, 2
	ctx.r[31].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D58F18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D58F1C: 4BF585F5  bl 0x82cb1510
	ctx.lr = 0x82D58F20;
	sub_82CB1510(ctx, base);
	// 82D58F20: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D58F24: 41980048  blt cr6, 0x82d58f6c
	if ctx.cr[6].lt {
	pc = 0x82D58F6C; continue 'dispatch;
	}
	// 82D58F28: 7F03F800  cmpw cr6, r3, r31
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D58F2C: 4198008C  blt cr6, 0x82d58fb8
	if ctx.cr[6].lt {
	pc = 0x82D58FB8; continue 'dispatch;
	}
	// 82D58F30: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58F34: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82D58F38: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D58F3C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D58F40: 40980024  bge cr6, 0x82d58f64
	if !ctx.cr[6].lt {
	pc = 0x82D58F64; continue 'dispatch;
	}
	// 82D58F44: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D58F48: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D58F4C: 41980008  blt cr6, 0x82d58f54
	if ctx.cr[6].lt {
	pc = 0x82D58F54; continue 'dispatch;
	}
	// 82D58F50: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D58F54: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D58F58: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D58F5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D58F60: 4BFFDFB1  bl 0x82d56f10
	ctx.lr = 0x82D58F64;
	sub_82D56F10(ctx, base);
	// 82D58F64: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82D58F68: 4BFFFF9C  b 0x82d58f04
	pc = 0x82D58F04; continue 'dispatch;
	// 82D58F6C: 57EB083C  slwi r11, r31, 1
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D58F70: 2F0B00FF  cmpwi cr6, r11, 0xff
	ctx.cr[6].compare_i32(ctx.r[11].s32, 255, &mut ctx.xer);
	// 82D58F74: 41990008  bgt cr6, 0x82d58f7c
	if ctx.cr[6].gt {
	pc = 0x82D58F7C; continue 'dispatch;
	}
	// 82D58F78: 396000FF  li r11, 0xff
	ctx.r[11].s64 = 255;
	// 82D58F7C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58F80: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 82D58F84: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82D58F88: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D58F8C: 4098FFD8  bge cr6, 0x82d58f64
	if !ctx.cr[6].lt {
	pc = 0x82D58F64; continue 'dispatch;
	}
	// 82D58F90: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D58F94: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D58F98: 4198FFBC  blt cr6, 0x82d58f54
	if ctx.cr[6].lt {
	pc = 0x82D58F54; continue 'dispatch;
	}
	// 82D58F9C: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D58FA0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D58FA4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D58FA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D58FAC: 4BFFDF65  bl 0x82d56f10
	ctx.lr = 0x82D58FB0;
	sub_82D56F10(ctx, base);
	// 82D58FB0: 93FE0004  stw r31, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82D58FB4: 4BFFFF50  b 0x82d58f04
	pc = 0x82D58F04; continue 'dispatch;
	// 82D58FB8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D58FBC: 38C30001  addi r6, r3, 1
	ctx.r[6].s64 = ctx.r[3].s64 + 1;
	// 82D58FC0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D58FC4: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D58FC8: 90DE0004  stw r6, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82D58FCC: 41990014  bgt cr6, 0x82d58fe0
	if ctx.cr[6].gt {
	pc = 0x82D58FE0; continue 'dispatch;
	}
	// 82D58FD0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D58FD4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D58FD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D58FDC: 4BFFE055  bl 0x82d57030
	ctx.lr = 0x82D58FE0;
	sub_82D57030(ctx, base);
	// 82D58FE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D58FE4: 4BF50474  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D58FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D58FE8 size=72
    let mut pc: u32 = 0x82D58FE8;
    'dispatch: loop {
        match pc {
            0x82D58FE8 => {
    //   block [0x82D58FE8..0x82D59030)
	// 82D58FE8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D58FEC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82D58FF0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D58FF4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D58FF8: 40980030  bge cr6, 0x82d59028
	if !ctx.cr[6].lt {
	pc = 0x82D59028; continue 'dispatch;
	}
	// 82D58FFC: 7F0B3000  cmpw cr6, r11, r6
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[6].s32, &mut ctx.xer);
	// 82D59000: 40980028  bge cr6, 0x82d59028
	if !ctx.cr[6].lt {
	pc = 0x82D59028; continue 'dispatch;
	}
	// 82D59004: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59008: 7C880774  extsb r8, r4
	ctx.r[8].s64 = ctx.r[4].s8 as i64;
	// 82D5900C: 7D2958AE  lbzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59010: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 82D59014: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82D59018: 419A0018  beq cr6, 0x82d59030
	if ctx.cr[6].eq {
		sub_82D59030(ctx, base);
		return;
	}
	// 82D5901C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D59020: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D59024: 4198FFD8  blt cr6, 0x82d58ffc
	if ctx.cr[6].lt {
	pc = 0x82D58FFC; continue 'dispatch;
	}
	// 82D59028: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82D5902C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59030 size=8
    let mut pc: u32 = 0x82D59030;
    'dispatch: loop {
        match pc {
            0x82D59030 => {
    //   block [0x82D59030..0x82D59038)
	// 82D59030: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D59034: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59038 size=76
    let mut pc: u32 = 0x82D59038;
    'dispatch: loop {
        match pc {
            0x82D59038 => {
    //   block [0x82D59038..0x82D59084)
	// 82D59038: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5903C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D59040: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D59044: 40990008  ble cr6, 0x82d5904c
	if !ctx.cr[6].gt {
	pc = 0x82D5904C; continue 'dispatch;
	}
	// 82D59048: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 82D5904C: 3966FFFF  addi r11, r6, -1
	ctx.r[11].s64 = ctx.r[6].s64 + -1;
	// 82D59050: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82D59054: 41980028  blt cr6, 0x82d5907c
	if ctx.cr[6].lt {
	pc = 0x82D5907C; continue 'dispatch;
	}
	// 82D59058: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5905C: 7C890774  extsb r9, r4
	ctx.r[9].s64 = ctx.r[4].s8 as i64;
	// 82D59060: 7D0A58AE  lbzx r8, r10, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59064: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 82D59068: 7F084800  cmpw cr6, r8, r9
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D5906C: 419A0018  beq cr6, 0x82d59084
	if ctx.cr[6].eq {
		sub_82D59084(ctx, base);
		return;
	}
	// 82D59070: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D59074: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82D59078: 4098FFE8  bge cr6, 0x82d59060
	if !ctx.cr[6].lt {
	pc = 0x82D59060; continue 'dispatch;
	}
	// 82D5907C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82D59080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59084(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59084 size=8
    let mut pc: u32 = 0x82D59084;
    'dispatch: loop {
        match pc {
            0x82D59084 => {
    //   block [0x82D59084..0x82D5908C)
	// 82D59084: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82D59088: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D59090 size=128
    let mut pc: u32 = 0x82D59090;
    'dispatch: loop {
        match pc {
            0x82D59090 => {
    //   block [0x82D59090..0x82D59110)
	// 82D59090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D59094: 4BF50371  bl 0x82ca9404
	ctx.lr = 0x82D59098;
	sub_82CA93D0(ctx, base);
	// 82D59098: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5909C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D590A0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D590A4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D590A8: 815D0004  lwz r10, 4(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D590AC: 3B8BFFFF  addi r28, r11, -1
	ctx.r[28].s64 = ctx.r[11].s64 + -1;
	// 82D590B0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D590B4: 3B6AFFFF  addi r27, r10, -1
	ctx.r[27].s64 = ctx.r[10].s64 + -1;
	// 82D590B8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D590BC: 7D5BE214  add r10, r27, r28
	ctx.r[10].u64 = ctx.r[27].u64 + ctx.r[28].u64;
	// 82D590C0: 3BCA0001  addi r30, r10, 1
	ctx.r[30].s64 = ctx.r[10].s64 + 1;
	// 82D590C4: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D590C8: 40980024  bge cr6, 0x82d590ec
	if !ctx.cr[6].lt {
	pc = 0x82D590EC; continue 'dispatch;
	}
	// 82D590CC: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D590D0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D590D4: 41980008  blt cr6, 0x82d590dc
	if ctx.cr[6].lt {
	pc = 0x82D590DC; continue 'dispatch;
	}
	// 82D590D8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D590DC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D590E0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D590E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D590E8: 4BFFDE29  bl 0x82d56f10
	ctx.lr = 0x82D590EC;
	sub_82D56F10(ctx, base);
	// 82D590EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D590F0: 38BB0001  addi r5, r27, 1
	ctx.r[5].s64 = ctx.r[27].s64 + 1;
	// 82D590F4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82D590F8: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82D590FC: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59100: 4BF50381  bl 0x82ca9480
	ctx.lr = 0x82D59104;
	sub_82CA9480(ctx, base);
	// 82D59104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D59108: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5910C: 4BF50348  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D59110 size=156
    let mut pc: u32 = 0x82D59110;
    'dispatch: loop {
        match pc {
            0x82D59110 => {
    //   block [0x82D59110..0x82D591AC)
	// 82D59110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D59114: 4BF502F1  bl 0x82ca9404
	ctx.lr = 0x82D59118;
	sub_82CA93D0(ctx, base);
	// 82D59118: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5911C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D59120: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82D59124: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 82D59128: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5912C: 3B8AFFFF  addi r28, r10, -1
	ctx.r[28].s64 = ctx.r[10].s64 + -1;
	// 82D59130: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82D59134: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59138: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5913C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D59140: 409AFFF4  bne cr6, 0x82d59134
	if !ctx.cr[6].eq {
	pc = 0x82D59134; continue 'dispatch;
	}
	// 82D59144: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D59148: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5914C: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 82D59150: 554B00BE  clrlwi r11, r10, 2
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82D59154: 553D003E  slwi r29, r9, 0
	ctx.r[29].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82D59158: 7D5DE214  add r10, r29, r28
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 82D5915C: 3BCA0001  addi r30, r10, 1
	ctx.r[30].s64 = ctx.r[10].s64 + 1;
	// 82D59160: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82D59164: 40980024  bge cr6, 0x82d59188
	if !ctx.cr[6].lt {
	pc = 0x82D59188; continue 'dispatch;
	}
	// 82D59168: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5916C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D59170: 41980008  blt cr6, 0x82d59178
	if ctx.cr[6].lt {
	pc = 0x82D59178; continue 'dispatch;
	}
	// 82D59174: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D59178: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5917C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D59180: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D59184: 4BFFDD8D  bl 0x82d56f10
	ctx.lr = 0x82D59188;
	sub_82D56F10(ctx, base);
	// 82D59188: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5918C: 38BD0001  addi r5, r29, 1
	ctx.r[5].s64 = ctx.r[29].s64 + 1;
	// 82D59190: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82D59194: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82D59198: 7C6BE214  add r3, r11, r28
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82D5919C: 4BF502E5  bl 0x82ca9480
	ctx.lr = 0x82D591A0;
	sub_82CA9480(ctx, base);
	// 82D591A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D591A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D591A8: 4BF502AC  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D591B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D591B0 size=80
    let mut pc: u32 = 0x82D591B0;
    'dispatch: loop {
        match pc {
            0x82D591B0 => {
    //   block [0x82D591B0..0x82D59200)
	// 82D591B0: 89450000  lbz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D591B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D591B8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D591BC: 419A0038  beq cr6, 0x82d591f4
	if ctx.cr[6].eq {
	pc = 0x82D591F4; continue 'dispatch;
	}
	// 82D591C0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D591C4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D591C8: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D591CC: 40980034  bge cr6, 0x82d59200
	if !ctx.cr[6].lt {
		sub_82D59200(ctx, base);
		return;
	}
	// 82D591D0: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D591D4: 7D0B28AE  lbzx r8, r11, r5
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82D591D8: 7D2958AE  lbzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D591DC: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82D591E0: 409A0020  bne cr6, 0x82d59200
	if !ctx.cr[6].eq {
		sub_82D59200(ctx, base);
		return;
	}
	// 82D591E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D591E8: 7D2B28AE  lbzx r9, r11, r5
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 82D591EC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D591F0: 409AFFD8  bne cr6, 0x82d591c8
	if !ctx.cr[6].eq {
	pc = 0x82D591C8; continue 'dispatch;
	}
	// 82D591F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D591F8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D591FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59200 size=12
    let mut pc: u32 = 0x82D59200;
    'dispatch: loop {
        match pc {
            0x82D59200 => {
    //   block [0x82D59200..0x82D5920C)
	// 82D59200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D59204: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D59208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59210 size=36
    let mut pc: u32 = 0x82D59210;
    'dispatch: loop {
        match pc {
            0x82D59210 => {
    //   block [0x82D59210..0x82D59234)
	// 82D59210: 81650004  lwz r11, 4(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59214: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59218: 390BFFFF  addi r8, r11, -1
	ctx.r[8].s64 = ctx.r[11].s64 + -1;
	// 82D5921C: 392AFFFF  addi r9, r10, -1
	ctx.r[9].s64 = ctx.r[10].s64 + -1;
	// 82D59220: 7F094000  cmpw cr6, r9, r8
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82D59224: 40980010  bge cr6, 0x82d59234
	if !ctx.cr[6].lt {
		sub_82D59234(ctx, base);
		return;
	}
	// 82D59228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5922C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D59230: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59234(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59234 size=68
    let mut pc: u32 = 0x82D59234;
    'dispatch: loop {
        match pc {
            0x82D59234 => {
    //   block [0x82D59234..0x82D59278)
	// 82D59234: 7D4B5050  subf r10, r11, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82D59238: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5923C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82D59240: 4099002C  ble cr6, 0x82d5926c
	if !ctx.cr[6].gt {
	pc = 0x82D5926C; continue 'dispatch;
	}
	// 82D59244: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59248: 80E50000  lwz r7, 0(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5924C: 7D495214  add r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82D59250: 7D2758AE  lbzx r9, r7, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59254: 7CCA58AE  lbzx r6, r10, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59258: 7F064840  cmplw cr6, r6, r9
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D5925C: 409AFFCC  bne cr6, 0x82d59228
	if !ctx.cr[6].eq {
		sub_82D59210(ctx, base);
		return;
	}
	// 82D59260: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D59264: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82D59268: 4198FFE8  blt cr6, 0x82d59250
	if ctx.cr[6].lt {
	pc = 0x82D59250; continue 'dispatch;
	}
	// 82D5926C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D59270: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D59274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59278 size=72
    let mut pc: u32 = 0x82D59278;
    'dispatch: loop {
        match pc {
            0x82D59278 => {
    //   block [0x82D59278..0x82D592C0)
	// 82D59278: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82D5927C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 82D59280: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82D59284: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59288: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5928C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82D59290: 409AFFF4  bne cr6, 0x82d59284
	if !ctx.cr[6].eq {
	pc = 0x82D59284; continue 'dispatch;
	}
	// 82D59294: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D59298: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5929C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D592A0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D592A4: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82D592A8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D592AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D592B0: 40980010  bge cr6, 0x82d592c0
	if !ctx.cr[6].lt {
		sub_82D592C0(ctx, base);
		return;
	}
	// 82D592B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D592B8: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D592BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D592C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D592C0 size=76
    let mut pc: u32 = 0x82D592C0;
    'dispatch: loop {
        match pc {
            0x82D592C0 => {
    //   block [0x82D592C0..0x82D5930C)
	// 82D592C0: 89490000  lbz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D592C4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82D592C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D592CC: 419A0034  beq cr6, 0x82d59300
	if ctx.cr[6].eq {
	pc = 0x82D59300; continue 'dispatch;
	}
	// 82D592D0: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D592D4: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D592D8: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D592DC: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 82D592E0: 7F085000  cmpw cr6, r8, r10
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D592E4: 409AFFD0  bne cr6, 0x82d592b4
	if !ctx.cr[6].eq {
		sub_82D59278(ctx, base);
		return;
	}
	// 82D592E8: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82D592EC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D592F0: 89490000  lbz r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D592F4: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82D592F8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D592FC: 409AFFDC  bne cr6, 0x82d592d8
	if !ctx.cr[6].eq {
	pc = 0x82D592D8; continue 'dispatch;
	}
	// 82D59300: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D59304: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D59308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D59310 size=128
    let mut pc: u32 = 0x82D59310;
    'dispatch: loop {
        match pc {
            0x82D59310 => {
    //   block [0x82D59310..0x82D59390)
	// 82D59310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D59314: 4BF500E9  bl 0x82ca93fc
	ctx.lr = 0x82D59318;
	sub_82CA93D0(ctx, base);
	// 82D59318: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5931C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82D59320: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82D59324: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D59328: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5932C: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82D59330: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59334: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59338: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82D5933C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59340: 3B2AFFFF  addi r25, r10, -1
	ctx.r[25].s64 = ctx.r[10].s64 + -1;
	// 82D59344: 7C69582E  lwzx r3, r9, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59348: 7D79F214  add r11, r25, r30
	ctx.r[11].u64 = ctx.r[25].u64 + ctx.r[30].u64;
	// 82D5934C: 3BAB0001  addi r29, r11, 1
	ctx.r[29].s64 = ctx.r[11].s64 + 1;
	// 82D59350: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D59354: 4BFFBEF5  bl 0x82d55248
	ctx.lr = 0x82D59358;
	sub_82D55248(ctx, base);
	// 82D59358: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5935C: 809B0000  lwz r4, 0(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59360: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82D59364: 4BF5011D  bl 0x82ca9480
	ctx.lr = 0x82D59368;
	sub_82CA9480(ctx, base);
	// 82D59368: 38B90001  addi r5, r25, 1
	ctx.r[5].s64 = ctx.r[25].s64 + 1;
	// 82D5936C: 7C7CF214  add r3, r28, r30
	ctx.r[3].u64 = ctx.r[28].u64 + ctx.r[30].u64;
	// 82D59370: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59374: 4BF5010D  bl 0x82ca9480
	ctx.lr = 0x82D59378;
	sub_82CA9480(ctx, base);
	// 82D59378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5937C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82D59380: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82D59384: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82D59388: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D5938C: 4BF500C0  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D59390 size=156
    let mut pc: u32 = 0x82D59390;
    'dispatch: loop {
        match pc {
            0x82D59390 => {
    //   block [0x82D59390..0x82D5942C)
	// 82D59390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D59394: 4BF50069  bl 0x82ca93fc
	ctx.lr = 0x82D59398;
	sub_82CA93D0(ctx, base);
	// 82D59398: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5939C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82D593A0: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82D593A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D593A8: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82D593AC: 815A0004  lwz r10, 4(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D593B0: 3B8AFFFF  addi r28, r10, -1
	ctx.r[28].s64 = ctx.r[10].s64 + -1;
	// 82D593B4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82D593B8: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D593BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D593C0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82D593C4: 409AFFF4  bne cr6, 0x82d593b8
	if !ctx.cr[6].eq {
	pc = 0x82D593B8; continue 'dispatch;
	}
	// 82D593C8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82D593CC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D593D0: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D593D4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D593D8: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82D593DC: 557B003E  slwi r27, r11, 0
	ctx.r[27].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 82D593E0: 7D7BE214  add r11, r27, r28
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[28].u64;
	// 82D593E4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D593E8: 3BCB0001  addi r30, r11, 1
	ctx.r[30].s64 = ctx.r[11].s64 + 1;
	// 82D593EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D593F0: 4BFFBE59  bl 0x82d55248
	ctx.lr = 0x82D593F4;
	sub_82D55248(ctx, base);
	// 82D593F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82D593F8: 809A0000  lwz r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D593FC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D59400: 4BF50081  bl 0x82ca9480
	ctx.lr = 0x82D59404;
	sub_82CA9480(ctx, base);
	// 82D59404: 38BB0001  addi r5, r27, 1
	ctx.r[5].s64 = ctx.r[27].s64 + 1;
	// 82D59408: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82D5940C: 7C7DE214  add r3, r29, r28
	ctx.r[3].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 82D59410: 4BF50071  bl 0x82ca9480
	ctx.lr = 0x82D59414;
	sub_82CA9480(ctx, base);
	// 82D59414: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D59418: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82D5941C: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82D59420: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82D59424: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D59428: 4BF50024  b 0x82ca944c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D59430 size=200
    let mut pc: u32 = 0x82D59430;
    'dispatch: loop {
        match pc {
            0x82D59430 => {
    //   block [0x82D59430..0x82D594F8)
	// 82D59430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D59434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D59438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5943C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D59440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D59444: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59448: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D5944C: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D59450: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82D59454: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D59458: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5945C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59460: 4BFFBDE9  bl 0x82d55248
	ctx.lr = 0x82D59464;
	sub_82D55248(ctx, base);
	// 82D59464: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59468: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82D5946C: 354BFFFF  addic. r10, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D59470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D59474: 40810048  ble 0x82d594bc
	if !ctx.cr[0].gt {
	pc = 0x82D594BC; continue 'dispatch;
	}
	// 82D59478: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5947C: 7D2A58AE  lbzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59480: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 82D59484: 2F0A0061  cmpwi cr6, r10, 0x61
	ctx.cr[6].compare_i32(ctx.r[10].s32, 97, &mut ctx.xer);
	// 82D59488: 41980018  blt cr6, 0x82d594a0
	if ctx.cr[6].lt {
	pc = 0x82D594A0; continue 'dispatch;
	}
	// 82D5948C: 2F0A007A  cmpwi cr6, r10, 0x7a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 122, &mut ctx.xer);
	// 82D59490: 41990010  bgt cr6, 0x82d594a0
	if ctx.cr[6].gt {
	pc = 0x82D594A0; continue 'dispatch;
	}
	// 82D59494: 394AFFE0  addi r10, r10, -0x20
	ctx.r[10].s64 = ctx.r[10].s64 + -32;
	// 82D59498: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82D5949C: 48000008  b 0x82d594a4
	pc = 0x82D594A4; continue 'dispatch;
	// 82D594A0: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82D594A4: 7D4B41AE  stbx r10, r11, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u8) };
	// 82D594A8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D594AC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D594B0: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D594B4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D594B8: 4198FFC0  blt cr6, 0x82d59478
	if ctx.cr[6].lt {
	pc = 0x82D59478; continue 'dispatch;
	}
	// 82D594BC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D594C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D594C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D594C8: 911E0000  stw r8, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82D594CC: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D594D0: 994BFFFF  stb r10, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[10].u8 ) };
	// 82D594D4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D594D8: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D594DC: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D594E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D594E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D594E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D594EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D594F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D594F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D594F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D594F8 size=200
    let mut pc: u32 = 0x82D594F8;
    'dispatch: loop {
        match pc {
            0x82D594F8 => {
    //   block [0x82D594F8..0x82D595C0)
	// 82D594F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D594FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D59500: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D59504: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D59508: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5950C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59510: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D59514: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D59518: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82D5951C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D59520: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59524: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59528: 4BFFBD21  bl 0x82d55248
	ctx.lr = 0x82D5952C;
	sub_82D55248(ctx, base);
	// 82D5952C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59530: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82D59534: 354BFFFF  addic. r10, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D59538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5953C: 40810048  ble 0x82d59584
	if !ctx.cr[0].gt {
	pc = 0x82D59584; continue 'dispatch;
	}
	// 82D59540: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59544: 7D2A58AE  lbzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59548: 7D2A0774  extsb r10, r9
	ctx.r[10].s64 = ctx.r[9].s8 as i64;
	// 82D5954C: 2F0A0041  cmpwi cr6, r10, 0x41
	ctx.cr[6].compare_i32(ctx.r[10].s32, 65, &mut ctx.xer);
	// 82D59550: 41980018  blt cr6, 0x82d59568
	if ctx.cr[6].lt {
	pc = 0x82D59568; continue 'dispatch;
	}
	// 82D59554: 2F0A005A  cmpwi cr6, r10, 0x5a
	ctx.cr[6].compare_i32(ctx.r[10].s32, 90, &mut ctx.xer);
	// 82D59558: 41990010  bgt cr6, 0x82d59568
	if ctx.cr[6].gt {
	pc = 0x82D59568; continue 'dispatch;
	}
	// 82D5955C: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 82D59560: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82D59564: 48000008  b 0x82d5956c
	pc = 0x82D5956C; continue 'dispatch;
	// 82D59568: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82D5956C: 7D4B41AE  stbx r10, r11, r8
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u8) };
	// 82D59570: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D59574: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59578: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D5957C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D59580: 4198FFC0  blt cr6, 0x82d59540
	if ctx.cr[6].lt {
	pc = 0x82D59540; continue 'dispatch;
	}
	// 82D59584: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59588: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D5958C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D59590: 911E0000  stw r8, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82D59594: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82D59598: 994BFFFF  stb r10, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[10].u8 ) };
	// 82D5959C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D595A0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D595A4: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D595A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D595AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D595B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D595B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D595B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D595BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D595C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D595C0 size=184
    let mut pc: u32 = 0x82D595C0;
    'dispatch: loop {
        match pc {
            0x82D595C0 => {
    //   block [0x82D595C0..0x82D59678)
	// 82D595C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D595C4: 4BF4FE3D  bl 0x82ca9400
	ctx.lr = 0x82D595C8;
	sub_82CA93D0(ctx, base);
	// 82D595C8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D595CC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D595D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D595D4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D595D8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82D595DC: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82D595E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82D595E4: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D595E8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82D595EC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D595F0: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82D595F4: 4BFFBC55  bl 0x82d55248
	ctx.lr = 0x82D595F8;
	sub_82D55248(ctx, base);
	// 82D595F8: 80BF0004  lwz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D595FC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59600: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D59604: 4BF4FE7D  bl 0x82ca9480
	ctx.lr = 0x82D59608;
	sub_82CA9480(ctx, base);
	// 82D59608: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5960C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D59610: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D59614: 40810038  ble 0x82d5964c
	if !ctx.cr[0].gt {
	pc = 0x82D5964C; continue 'dispatch;
	}
	// 82D59618: 7F890774  extsb r9, r28
	ctx.r[9].s64 = ctx.r[28].s8 as i64;
	// 82D5961C: 7D4BF0AE  lbzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82D59620: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 82D59624: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82D59628: 409A0010  bne cr6, 0x82d59638
	if !ctx.cr[6].eq {
	pc = 0x82D59638; continue 'dispatch;
	}
	// 82D5962C: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82D59630: 7F6BF1AE  stbx r27, r11, r30
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[27].u8) };
	// 82D59634: 419A0018  beq cr6, 0x82d5964c
	if ctx.cr[6].eq {
	pc = 0x82D5964C; continue 'dispatch;
	}
	// 82D59638: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5963C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D59640: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82D59644: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D59648: 4198FFD4  blt cr6, 0x82d5961c
	if ctx.cr[6].lt {
	pc = 0x82D5961C; continue 'dispatch;
	}
	// 82D5964C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59650: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82D59654: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D59658: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82D5965C: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82D59660: 994BFFFF  stb r10, -1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-1 as u32), ctx.r[10].u8 ) };
	// 82D59664: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59668: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D5966C: 917D0008  stw r11, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82D59670: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D59674: 4BF4FDDC  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D59678 size=708
    let mut pc: u32 = 0x82D59678;
    'dispatch: loop {
        match pc {
            0x82D59678 => {
    //   block [0x82D59678..0x82D5993C)
	// 82D59678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5967C: 4BF4FD5D  bl 0x82ca93d8
	ctx.lr = 0x82D59680;
	sub_82CA93D0(ctx, base);
	// 82D59680: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D59684: 820D0000  lwz r16, 0(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59688: 3A200004  li r17, 4
	ctx.r[17].s64 = 4;
	// 82D5968C: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82D59690: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 82D59694: 38A00016  li r5, 0x16
	ctx.r[5].s64 = 22;
	// 82D59698: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D5969C: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 82D596A0: 7C71802E  lwzx r3, r17, r16
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[17].u32.wrapping_add(ctx.r[16].u32)) } as u64;
	// 82D596A4: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82D596A8: 7CF53B78  mr r21, r7
	ctx.r[21].u64 = ctx.r[7].u64;
	// 82D596AC: 4BFFBB9D  bl 0x82d55248
	ctx.lr = 0x82D596B0;
	sub_82D55248(ctx, base);
	// 82D596B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D596B4: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82D596B8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82D596BC: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 82D596C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82D596C4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82D596C8: 9B630000  stb r27, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[27].u8 ) };
	// 82D596CC: 82F90000  lwz r23, 0(r25)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D596D0: 82760000  lwz r19, 0(r22)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D596D4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82D596D8: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 82D596DC: 4B48D0ED  bl 0x821e67c8
	ctx.lr = 0x82D596E0;
	sub_821E67C8(ctx, base);
	// 82D596E0: 3E408000  lis r18, -0x8000
	ctx.r[18].s64 = -2147483648;
	// 82D596E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D596E8: 419A01E0  beq cr6, 0x82d598c8
	if ctx.cr[6].eq {
	pc = 0x82D598C8; continue 'dispatch;
	}
	// 82D596EC: 81790004  lwz r11, 4(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D596F0: 7D571850  subf r10, r23, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[23].s64;
	// 82D596F4: 7D7C5850  subf r11, r28, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[28].s64;
	// 82D596F8: 7F5C5050  subf r26, r28, r10
	ctx.r[26].s64 = ctx.r[10].s64 - ctx.r[28].s64;
	// 82D596FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D59700: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 82D59704: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D59708: 40990008  ble cr6, 0x82d59710
	if !ctx.cr[6].gt {
	pc = 0x82D59710; continue 'dispatch;
	}
	// 82D5970C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 82D59710: 3BDF0001  addi r30, r31, 1
	ctx.r[30].s64 = ctx.r[31].s64 + 1;
	// 82D59714: 83B90000  lwz r29, 0(r25)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59718: 93610060  stw r27, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u32 ) };
	// 82D5971C: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82D59720: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D59724: 92410068  stw r18, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[18].u32 ) };
	// 82D59728: 4099001C  ble cr6, 0x82d59744
	if !ctx.cr[6].gt {
	pc = 0x82D59744; continue 'dispatch;
	}
	// 82D5972C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82D59730: 41980008  blt cr6, 0x82d59738
	if ctx.cr[6].lt {
	pc = 0x82D59738; continue 'dispatch;
	}
	// 82D59734: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D59738: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5973C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D59740: 4BFFD7D1  bl 0x82d56f10
	ctx.lr = 0x82D59744;
	sub_82D56F10(ctx, base);
	// 82D59744: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D59748: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D5974C: 7C9DE214  add r4, r29, r28
	ctx.r[4].u64 = ctx.r[29].u64 + ctx.r[28].u64;
	// 82D59750: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82D59754: 4BF4FD2D  bl 0x82ca9480
	ctx.lr = 0x82D59758;
	sub_82CA9480(ctx, base);
	// 82D59758: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D5975C: 7F6BF9AE  stbx r27, r11, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[27].u8) };
	// 82D59760: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D59764: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82D59768: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82D5976C: 3BABFFFF  addi r29, r11, -1
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	// 82D59770: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D59774: 7D5DF214  add r10, r29, r30
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 82D59778: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5977C: 3BEA0001  addi r31, r10, 1
	ctx.r[31].s64 = ctx.r[10].s64 + 1;
	// 82D59780: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D59784: 40980024  bge cr6, 0x82d597a8
	if !ctx.cr[6].lt {
	pc = 0x82D597A8; continue 'dispatch;
	}
	// 82D59788: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5978C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D59790: 41980008  blt cr6, 0x82d59798
	if ctx.cr[6].lt {
	pc = 0x82D59798; continue 'dispatch;
	}
	// 82D59794: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D59798: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5979C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D597A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D597A4: 4BFFD76D  bl 0x82d56f10
	ctx.lr = 0x82D597A8;
	sub_82D56F10(ctx, base);
	// 82D597A8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D597AC: 38BD0001  addi r5, r29, 1
	ctx.r[5].s64 = ctx.r[29].s64 + 1;
	// 82D597B0: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D597B4: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82D597B8: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D597BC: 4BF4FCC5  bl 0x82ca9480
	ctx.lr = 0x82D597C0;
	sub_82CA9480(ctx, base);
	// 82D597C0: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82D597C4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D597C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D597CC: 409A0018  bne cr6, 0x82d597e4
	if !ctx.cr[6].eq {
	pc = 0x82D597E4; continue 'dispatch;
	}
	// 82D597D0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D597D4: 7C71802E  lwzx r3, r17, r16
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[17].u32.wrapping_add(ctx.r[16].u32)) } as u64;
	// 82D597D8: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D597DC: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D597E0: 4BFFBAE9  bl 0x82d552c8
	ctx.lr = 0x82D597E4;
	sub_82D552C8(ctx, base);
	// 82D597E4: 81580004  lwz r10, 4(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D597E8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D597EC: 3BAAFFFF  addi r29, r10, -1
	ctx.r[29].s64 = ctx.r[10].s64 + -1;
	// 82D597F0: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82D597F4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D597F8: 7D5DF214  add r10, r29, r30
	ctx.r[10].u64 = ctx.r[29].u64 + ctx.r[30].u64;
	// 82D597FC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D59800: 3BEA0001  addi r31, r10, 1
	ctx.r[31].s64 = ctx.r[10].s64 + 1;
	// 82D59804: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D59808: 40980024  bge cr6, 0x82d5982c
	if !ctx.cr[6].lt {
	pc = 0x82D5982C; continue 'dispatch;
	}
	// 82D5980C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D59810: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D59814: 41980008  blt cr6, 0x82d5981c
	if ctx.cr[6].lt {
	pc = 0x82D5981C; continue 'dispatch;
	}
	// 82D59818: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D5981C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D59820: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D59824: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D59828: 4BFFD6E9  bl 0x82d56f10
	ctx.lr = 0x82D5982C;
	sub_82D56F10(ctx, base);
	// 82D5982C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D59830: 38BD0001  addi r5, r29, 1
	ctx.r[5].s64 = ctx.r[29].s64 + 1;
	// 82D59834: 80980000  lwz r4, 0(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59838: 7C7E5A14  add r3, r30, r11
	ctx.r[3].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 82D5983C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D59840: 4BF4FC41  bl 0x82ca9480
	ctx.lr = 0x82D59844;
	sub_82CA9480(ctx, base);
	// 82D59844: 81760004  lwz r11, 4(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59848: 2F150000  cmpwi cr6, r21, 0
	ctx.cr[6].compare_i32(ctx.r[21].s32, 0, &mut ctx.xer);
	// 82D5984C: 7D6BD214  add r11, r11, r26
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82D59850: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82D59854: 3B8BFFFF  addi r28, r11, -1
	ctx.r[28].s64 = ctx.r[11].s64 + -1;
	// 82D59858: 419A0018  beq cr6, 0x82d59870
	if ctx.cr[6].eq {
	pc = 0x82D59870; continue 'dispatch;
	}
	// 82D5985C: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 82D59860: 7C77E214  add r3, r23, r28
	ctx.r[3].u64 = ctx.r[23].u64 + ctx.r[28].u64;
	// 82D59864: 4B48CF65  bl 0x821e67c8
	ctx.lr = 0x82D59868;
	sub_821E67C8(ctx, base);
	// 82D59868: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D5986C: 409AFE80  bne cr6, 0x82d596ec
	if !ctx.cr[6].eq {
	pc = 0x82D596EC; continue 'dispatch;
	}
	// 82D59870: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82D59874: 419A0054  beq cr6, 0x82d598c8
	if ctx.cr[6].eq {
	pc = 0x82D598C8; continue 'dispatch;
	}
	// 82D59878: 7C97E214  add r4, r23, r28
	ctx.r[4].u64 = ctx.r[23].u64 + ctx.r[28].u64;
	// 82D5987C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D59880: 4BFFF891  bl 0x82d59110
	ctx.lr = 0x82D59884;
	sub_82D59110(ctx, base);
	// 82D59884: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D59888: 93740000  stw r27, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82D5988C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82D59890: 93740004  stw r27, 4(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82D59894: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82D59898: 92540008  stw r18, 8(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(8 as u32), ctx.r[18].u32 ) };
	// 82D5989C: 40990020  ble cr6, 0x82d598bc
	if !ctx.cr[6].gt {
	pc = 0x82D598BC; continue 'dispatch;
	}
	// 82D598A0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82D598A4: 41980008  blt cr6, 0x82d598ac
	if ctx.cr[6].lt {
	pc = 0x82D598AC; continue 'dispatch;
	}
	// 82D598A8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82D598AC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D598B0: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82D598B4: 4BFFD65D  bl 0x82d56f10
	ctx.lr = 0x82D598B8;
	sub_82D56F10(ctx, base);
	// 82D598B8: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D598BC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D598C0: 93F40004  stw r31, 4(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82D598C4: 48000040  b 0x82d59904
	pc = 0x82D59904; continue 'dispatch;
	// 82D598C8: 93740000  stw r27, 0(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82D598CC: 93740004  stw r27, 4(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 82D598D0: 92540008  stw r18, 8(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(8 as u32), ctx.r[18].u32 ) };
	// 82D598D4: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D598D8: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82D598DC: 4099001C  ble cr6, 0x82d598f8
	if !ctx.cr[6].gt {
	pc = 0x82D598F8; continue 'dispatch;
	}
	// 82D598E0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82D598E4: 41980008  blt cr6, 0x82d598ec
	if ctx.cr[6].lt {
	pc = 0x82D598EC; continue 'dispatch;
	}
	// 82D598E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D598EC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D598F0: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82D598F4: 4BFFD61D  bl 0x82d56f10
	ctx.lr = 0x82D598F8;
	sub_82D56F10(ctx, base);
	// 82D598F8: 93F40004  stw r31, 4(r20)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[20].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82D598FC: 80B90004  lwz r5, 4(r25)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59900: 80990000  lwz r4, 0(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59904: 80740000  lwz r3, 0(r20)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59908: 4BF4FB79  bl 0x82ca9480
	ctx.lr = 0x82D5990C;
	sub_82CA9480(ctx, base);
	// 82D5990C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D59910: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D59914: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D59918: 409A0018  bne cr6, 0x82d59930
	if !ctx.cr[6].eq {
	pc = 0x82D59930; continue 'dispatch;
	}
	// 82D5991C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D59920: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D59924: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D59928: 7C71802E  lwzx r3, r17, r16
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[17].u32.wrapping_add(ctx.r[16].u32)) } as u64;
	// 82D5992C: 4BFFB99D  bl 0x82d552c8
	ctx.lr = 0x82D59930;
	sub_82D552C8(ctx, base);
	// 82D59930: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 82D59934: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82D59938: 4BF4FAF0  b 0x82ca9428
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59940 size=4
    let mut pc: u32 = 0x82D59940;
    'dispatch: loop {
        match pc {
            0x82D59940 => {
    //   block [0x82D59940..0x82D59944)
	// 82D59940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59948 size=4
    let mut pc: u32 = 0x82D59948;
    'dispatch: loop {
        match pc {
            0x82D59948 => {
    //   block [0x82D59948..0x82D5994C)
	// 82D59948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59950 size=4
    let mut pc: u32 = 0x82D59950;
    'dispatch: loop {
        match pc {
            0x82D59950 => {
    //   block [0x82D59950..0x82D59954)
	// 82D59950: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59958 size=4
    let mut pc: u32 = 0x82D59958;
    'dispatch: loop {
        match pc {
            0x82D59958 => {
    //   block [0x82D59958..0x82D5995C)
	// 82D59958: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59960 size=8
    let mut pc: u32 = 0x82D59960;
    'dispatch: loop {
        match pc {
            0x82D59960 => {
    //   block [0x82D59960..0x82D59968)
	// 82D59960: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D59964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59968 size=8
    let mut pc: u32 = 0x82D59968;
    'dispatch: loop {
        match pc {
            0x82D59968 => {
    //   block [0x82D59968..0x82D59970)
	// 82D59968: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82D5996C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59970 size=12
    let mut pc: u32 = 0x82D59970;
    'dispatch: loop {
        match pc {
            0x82D59970 => {
    //   block [0x82D59970..0x82D5997C)
	// 82D59970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D59974: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D59978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59980 size=4
    let mut pc: u32 = 0x82D59980;
    'dispatch: loop {
        match pc {
            0x82D59980 => {
    //   block [0x82D59980..0x82D59984)
	// 82D59980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D59988 size=40
    let mut pc: u32 = 0x82D59988;
    'dispatch: loop {
        match pc {
            0x82D59988 => {
    //   block [0x82D59988..0x82D599B0)
	// 82D59988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5998C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D59990: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D59994: 4BFFB305  bl 0x82d54c98
	ctx.lr = 0x82D59998;
	sub_82D54C98(ctx, base);
	// 82D59998: 48006B21  bl 0x82d604b8
	ctx.lr = 0x82D5999C;
	sub_82D604B8(ctx, base);
	// 82D5999C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D599A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D599A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D599A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D599AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D599B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D599B0 size=88
    let mut pc: u32 = 0x82D599B0;
    'dispatch: loop {
        match pc {
            0x82D599B0 => {
    //   block [0x82D599B0..0x82D59A08)
	// 82D599B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D599B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D599B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D599BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D599C0: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D599C4: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82D599C8: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D599CC: 48006935  bl 0x82d60300
	ctx.lr = 0x82D599D0;
	sub_82D60300(ctx, base);
	// 82D599D0: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82D599D4: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D599D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D599DC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D599E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D599E4: 4E800421  bctrl
	ctx.lr = 0x82D599E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D599E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D599EC: 4BFFB2AD  bl 0x82d54c98
	ctx.lr = 0x82D599F0;
	sub_82D54C98(ctx, base);
	// 82D599F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D599F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D599F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D599FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D59A00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D59A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D59A08 size=16
    let mut pc: u32 = 0x82D59A08;
    'dispatch: loop {
        match pc {
            0x82D59A08 => {
    //   block [0x82D59A08..0x82D59A18)
	// 82D59A08: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D59A0C: 896B76F8  lbz r11, 0x76f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(30456 as u32) ) } as u64;
	// 82D59A10: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D59A14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59A18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D59A18 size=80
    let mut pc: u32 = 0x82D59A18;
    'dispatch: loop {
        match pc {
            0x82D59A18 => {
    //   block [0x82D59A18..0x82D59A68)
	// 82D59A18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D59A1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D59A20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D59A24: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59A28: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D59A2C: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82D59A30: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82D59A34: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59A38: 4BFFB811  bl 0x82d55248
	ctx.lr = 0x82D59A3C;
	sub_82D55248(ctx, base);
	// 82D59A3C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D59A40: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82D59A44: 396B41FC  addi r11, r11, 0x41fc
	ctx.r[11].s64 = ctx.r[11].s64 + 16892;
	// 82D59A48: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D59A4C: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82D59A50: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D59A54: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82D59A58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D59A5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D59A60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D59A64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D59A68 size=372
    let mut pc: u32 = 0x82D59A68;
    'dispatch: loop {
        match pc {
            0x82D59A68 => {
    //   block [0x82D59A68..0x82D59BDC)
	// 82D59A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D59A6C: 4BF4F99D  bl 0x82ca9408
	ctx.lr = 0x82D59A70;
	sub_82CA93D0(ctx, base);
	// 82D59A70: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D59A74: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D59A78: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D59A7C: 3BCB76FC  addi r30, r11, 0x76fc
	ctx.r[30].s64 = ctx.r[11].s64 + 30460;
	// 82D59A80: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82D59A84: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82D59A88: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59A8C: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82D59A90: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82D59A94: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D59A98: 419A00C0  beq cr6, 0x82d59b58
	if ctx.cr[6].eq {
	pc = 0x82D59B58; continue 'dispatch;
	}
	// 82D59A9C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D59AA0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59AA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D59AA8: 409A0020  bne cr6, 0x82d59ac8
	if !ctx.cr[6].eq {
	pc = 0x82D59AC8; continue 'dispatch;
	}
	// 82D59AAC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59AB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D59AB4: 4E800421  bctrl
	ctx.lr = 0x82D59AB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D59AB8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D59ABC: 419A00D0  beq cr6, 0x82d59b8c
	if ctx.cr[6].eq {
	pc = 0x82D59B8C; continue 'dispatch;
	}
	// 82D59AC0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D59AC4: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D59AC8: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82D59ACC: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59AD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D59AD4: 409AFFC8  bne cr6, 0x82d59a9c
	if !ctx.cr[6].eq {
	pc = 0x82D59A9C; continue 'dispatch;
	}
	// 82D59AD8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D59ADC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D59AE0: 419A0078  beq cr6, 0x82d59b58
	if ctx.cr[6].eq {
	pc = 0x82D59B58; continue 'dispatch;
	}
	// 82D59AE4: 3B8BFFFF  addi r28, r11, -1
	ctx.r[28].s64 = ctx.r[11].s64 + -1;
	// 82D59AE8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82D59AEC: 41980060  blt cr6, 0x82d59b4c
	if ctx.cr[6].lt {
	pc = 0x82D59B4C; continue 'dispatch;
	}
	// 82D59AF0: 579D103A  slwi r29, r28, 2
	ctx.r[29].u32 = ctx.r[28].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82D59AF4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D59AF8: 7FFD582E  lwzx r31, r29, r11
	ctx.r[31].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59AFC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59B00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D59B04: 4E800421  bctrl
	ctx.lr = 0x82D59B08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D59B08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D59B0C: 419A0030  beq cr6, 0x82d59b3c
	if ctx.cr[6].eq {
	pc = 0x82D59B3C; continue 'dispatch;
	}
	// 82D59B10: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D59B14: 906B0000  stw r3, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82D59B18: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82D59B1C: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 82D59B20: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D59B24: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D59B28: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82D59B2C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82D59B30: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D59B34: 7D4A582E  lwzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59B38: 7D5D592E  stwx r10, r29, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 82D59B3C: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82D59B40: 3BBDFFFC  addi r29, r29, -4
	ctx.r[29].s64 = ctx.r[29].s64 + -4;
	// 82D59B44: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82D59B48: 4098FFAC  bge cr6, 0x82d59af4
	if !ctx.cr[6].lt {
	pc = 0x82D59AF4; continue 'dispatch;
	}
	// 82D59B4C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D59B50: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D59B54: 409AFF90  bne cr6, 0x82d59ae4
	if !ctx.cr[6].eq {
	pc = 0x82D59AE4; continue 'dispatch;
	}
	// 82D59B58: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D59B5C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D59B60: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D59B64: 409A0020  bne cr6, 0x82d59b84
	if !ctx.cr[6].eq {
	pc = 0x82D59B84; continue 'dispatch;
	}
	// 82D59B68: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59B6C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D59B70: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D59B74: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D59B78: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D59B7C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D59B80: 4BFFB749  bl 0x82d552c8
	ctx.lr = 0x82D59B84;
	sub_82D552C8(ctx, base);
	// 82D59B84: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D59B88: 4BF4F8D0  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
	// 82D59B8C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D59B90: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D59B94: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D59B98: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D59B9C: 409A0010  bne cr6, 0x82d59bac
	if !ctx.cr[6].eq {
	pc = 0x82D59BAC; continue 'dispatch;
	}
	// 82D59BA0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82D59BA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D59BA8: 4BFFD3F1  bl 0x82d56f98
	ctx.lr = 0x82D59BAC;
	sub_82D56F98(ctx, base);
	// 82D59BAC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D59BB0: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D59BB4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D59BB8: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82D59BBC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D59BC0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D59BC4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82D59BC8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59BCC: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59BD0: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82D59BD4: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82D59BD8: 4BFFFEF8  b 0x82d59ad0
	pc = 0x82D59AD0; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D59BE0 size=300
    let mut pc: u32 = 0x82D59BE0;
    'dispatch: loop {
        match pc {
            0x82D59BE0 => {
    //   block [0x82D59BE0..0x82D59D0C)
	// 82D59BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D59BE4: 4BF4F829  bl 0x82ca940c
	ctx.lr = 0x82D59BE8;
	sub_82CA93D0(ctx, base);
	// 82D59BE8: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D59BEC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D59BF0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D59BF4: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82D59BF8: 614A0080  ori r10, r10, 0x80
	ctx.r[10].u64 = ctx.r[10].u64 | 128;
	// 82D59BFC: 83EB76FC  lwz r31, 0x76fc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30460 as u32) ) } as u64;
	// 82D59C00: 3961005C  addi r11, r1, 0x5c
	ctx.r[11].s64 = ctx.r[1].s64 + 92;
	// 82D59C04: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D59C08: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82D59C0C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82D59C10: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82D59C14: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82D59C18: 419A0048  beq cr6, 0x82d59c60
	if ctx.cr[6].eq {
	pc = 0x82D59C60; continue 'dispatch;
	}
	// 82D59C1C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D59C20: 554A00BE  clrlwi r10, r10, 2
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x3FFFFFFFu64;
	// 82D59C24: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82D59C28: 409A0014  bne cr6, 0x82d59c3c
	if !ctx.cr[6].eq {
	pc = 0x82D59C3C; continue 'dispatch;
	}
	// 82D59C2C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82D59C30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D59C34: 4BFFD365  bl 0x82d56f98
	ctx.lr = 0x82D59C38;
	sub_82D56F98(ctx, base);
	// 82D59C38: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D59C3C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D59C40: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D59C44: 7FEB512E  stwx r31, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[31].u32) };
	// 82D59C48: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82D59C4C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D59C50: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82D59C54: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59C58: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D59C5C: 409AFFC0  bne cr6, 0x82d59c1c
	if !ctx.cr[6].eq {
	pc = 0x82D59C1C; continue 'dispatch;
	}
	// 82D59C60: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82D59C64: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D59C68: 41980070  blt cr6, 0x82d59cd8
	if ctx.cr[6].lt {
	pc = 0x82D59CD8; continue 'dispatch;
	}
	// 82D59C6C: 57DF103A  slwi r31, r30, 2
	ctx.r[31].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82D59C70: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D59C74: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59C78: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D59C7C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59C80: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59C84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D59C88: 419A0030  beq cr6, 0x82d59cb8
	if ctx.cr[6].eq {
	pc = 0x82D59CB8; continue 'dispatch;
	}
	// 82D59C8C: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D59C90: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D59C94: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D59C98: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D59C9C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D59CA0: 409A0018  bne cr6, 0x82d59cb8
	if !ctx.cr[6].eq {
	pc = 0x82D59CB8; continue 'dispatch;
	}
	// 82D59CA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59CA8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D59CAC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59CB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D59CB4: 4E800421  bctrl
	ctx.lr = 0x82D59CB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D59CB8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D59CBC: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82D59CC0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82D59CC4: 7D7F582E  lwzx r11, r31, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59CC8: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 82D59CCC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D59CD0: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82D59CD4: 4098FF9C  bge cr6, 0x82d59c70
	if !ctx.cr[6].lt {
	pc = 0x82D59C70; continue 'dispatch;
	}
	// 82D59CD8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D59CDC: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D59CE0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D59CE4: 409A0020  bne cr6, 0x82d59d04
	if !ctx.cr[6].eq {
	pc = 0x82D59D04; continue 'dispatch;
	}
	// 82D59CE8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59CEC: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D59CF0: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D59CF4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D59CF8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D59CFC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D59D00: 4BFFB5C9  bl 0x82d552c8
	ctx.lr = 0x82D59D04;
	sub_82D552C8(ctx, base);
	// 82D59D04: 38210280  addi r1, r1, 0x280
	ctx.r[1].s64 = ctx.r[1].s64 + 640;
	// 82D59D08: 4BF4F754  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D59D10 size=312
    let mut pc: u32 = 0x82D59D10;
    'dispatch: loop {
        match pc {
            0x82D59D10 => {
    //   block [0x82D59D10..0x82D59E48)
	// 82D59D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D59D14: 4BF4F6F1  bl 0x82ca9404
	ctx.lr = 0x82D59D18;
	sub_82CA93D0(ctx, base);
	// 82D59D18: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D59D1C: 3F608333  lis r27, -0x7ccd
	ctx.r[27].s64 = -2093809664;
	// 82D59D20: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D59D24: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D59D28: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82D59D2C: 897B76F8  lbz r11, 0x76f8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(30456 as u32) ) } as u64;
	// 82D59D30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D59D34: 409A00A8  bne cr6, 0x82d59ddc
	if !ctx.cr[6].eq {
	pc = 0x82D59DDC; continue 'dispatch;
	}
	// 82D59D38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D59D3C: 419A00DC  beq cr6, 0x82d59e18
	if ctx.cr[6].eq {
	pc = 0x82D59E18; continue 'dispatch;
	}
	// 82D59D40: 4BFFACB1  bl 0x82d549f0
	ctx.lr = 0x82D59D44;
	sub_82D549F0(ctx, base);
	// 82D59D44: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82D59D48: 419A00A0  beq cr6, 0x82d59de8
	if ctx.cr[6].eq {
	pc = 0x82D59DE8; continue 'dispatch;
	}
	// 82D59D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D59D50: 4BFFAF49  bl 0x82d54c98
	ctx.lr = 0x82D59D54;
	sub_82D54C98(ctx, base);
	// 82D59D54: 48006765  bl 0x82d604b8
	ctx.lr = 0x82D59D58;
	sub_82D604B8(ctx, base);
	// 82D59D58: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59D5C: 3BE00004  li r31, 4
	ctx.r[31].s64 = 4;
	// 82D59D60: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82D59D64: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82D59D68: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82D59D6C: 4BFFB4DD  bl 0x82d55248
	ctx.lr = 0x82D59D70;
	sub_82D55248(ctx, base);
	// 82D59D70: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 82D59D74: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D59D78: 396B41E4  addi r11, r11, 0x41e4
	ctx.r[11].s64 = ctx.r[11].s64 + 16868;
	// 82D59D7C: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82D59D80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D59D84: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D59D88: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D59D8C: 480001ED  bl 0x82d59f78
	ctx.lr = 0x82D59D90;
	sub_82D59F78(ctx, base);
	// 82D59D90: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 82D59D94: 7C7FF02E  lwzx r3, r31, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82D59D98: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82D59D9C: 4BFFB4AD  bl 0x82d55248
	ctx.lr = 0x82D59DA0;
	sub_82D55248(ctx, base);
	// 82D59DA0: 39600028  li r11, 0x28
	ctx.r[11].s64 = 40;
	// 82D59DA4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82D59DA8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D59DAC: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82D59DB0: 480002A9  bl 0x82d5a058
	ctx.lr = 0x82D59DB4;
	sub_82D5A058(ctx, base);
	// 82D59DB4: 4B6FBC35  bl 0x824559e8
	ctx.lr = 0x82D59DB8;
	sub_824559E8(ctx, base);
	// 82D59DB8: 4BFFFCB1  bl 0x82d59a68
	ctx.lr = 0x82D59DBC;
	sub_82D59A68(ctx, base);
	// 82D59DBC: 3D608333  lis r11, -0x7ccd
	ctx.r[11].s64 = -2093809664;
	// 82D59DC0: 806B76F4  lwz r3, 0x76f4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30452 as u32) ) } as u64;
	// 82D59DC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59DC8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D59DCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D59DD0: 4E800421  bctrl
	ctx.lr = 0x82D59DD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D59DD4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D59DD8: 997B76F8  stb r11, 0x76f8(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(30456 as u32), ctx.r[11].u8 ) };
	// 82D59DDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D59DE0: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82D59DE4: 4BF4F670  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D59DE8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D59DEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D59DF0: 388B4380  addi r4, r11, 0x4380
	ctx.r[4].s64 = ctx.r[11].s64 + 17280;
	// 82D59DF4: 38A00078  li r5, 0x78
	ctx.r[5].s64 = 120;
	// 82D59DF8: 4BF4F689  bl 0x82ca9480
	ctx.lr = 0x82D59DFC;
	sub_82CA9480(ctx, base);
	// 82D59DFC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D59E00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D59E04: 7FA903A6  mtctr r29
	ctx.ctr.u64 = ctx.r[29].u64;
	// 82D59E08: 4E800421  bctrl
	ctx.lr = 0x82D59E0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D59E0C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D59E10: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82D59E14: 4BF4F640  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
	// 82D59E18: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D59E1C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D59E20: 388B42A0  addi r4, r11, 0x42a0
	ctx.r[4].s64 = ctx.r[11].s64 + 17056;
	// 82D59E24: 38A000DC  li r5, 0xdc
	ctx.r[5].s64 = 220;
	// 82D59E28: 4BF4F659  bl 0x82ca9480
	ctx.lr = 0x82D59E2C;
	sub_82CA9480(ctx, base);
	// 82D59E2C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D59E30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D59E34: 7FA903A6  mtctr r29
	ctx.ctr.u64 = ctx.r[29].u64;
	// 82D59E38: 4E800421  bctrl
	ctx.lr = 0x82D59E3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D59E3C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D59E40: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82D59E44: 4BF4F610  b 0x82ca9454
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D59E48 size=300
    let mut pc: u32 = 0x82D59E48;
    'dispatch: loop {
        match pc {
            0x82D59E48 => {
    //   block [0x82D59E48..0x82D59F74)
	// 82D59E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D59E4C: 4BF4F5BD  bl 0x82ca9408
	ctx.lr = 0x82D59E50;
	sub_82CA93D0(ctx, base);
	// 82D59E50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D59E54: 3F808333  lis r28, -0x7ccd
	ctx.r[28].s64 = -2093809664;
	// 82D59E58: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82D59E5C: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82D59E60: 897C76F8  lbz r11, 0x76f8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(30456 as u32) ) } as u64;
	// 82D59E64: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82D59E68: 409A0100  bne cr6, 0x82d59f68
	if !ctx.cr[6].eq {
	pc = 0x82D59F68; continue 'dispatch;
	}
	// 82D59E6C: 4BFFFD75  bl 0x82d59be0
	ctx.lr = 0x82D59E70;
	sub_82D59BE0(ctx, base);
	// 82D59E70: 3FE08333  lis r31, -0x7ccd
	ctx.r[31].s64 = -2093809664;
	// 82D59E74: 897F7764  lbz r11, 0x7764(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(30564 as u32) ) } as u64;
	// 82D59E78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D59E7C: 419A0028  beq cr6, 0x82d59ea4
	if ctx.cr[6].eq {
	pc = 0x82D59EA4; continue 'dispatch;
	}
	// 82D59E80: 3D60832F  lis r11, -0x7cd1
	ctx.r[11].s64 = -2094071808;
	// 82D59E84: 814B764C  lwz r10, 0x764c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30284 as u32) ) } as u64;
	// 82D59E88: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D59E8C: 419A0018  beq cr6, 0x82d59ea4
	if ctx.cr[6].eq {
	pc = 0x82D59EA4; continue 'dispatch;
	}
	// 82D59E90: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D59E94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D59E98: 4E800421  bctrl
	ctx.lr = 0x82D59E9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D59E9C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D59EA0: 997F7764  stb r11, 0x7764(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(30564 as u32), ctx.r[11].u8 ) };
	// 82D59EA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D59EA8: 4B6FBB41  bl 0x824559e8
	ctx.lr = 0x82D59EAC;
	sub_824559E8(ctx, base);
	// 82D59EAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D59EB0: 480000C9  bl 0x82d59f78
	ctx.lr = 0x82D59EB4;
	sub_82D59F78(ctx, base);
	// 82D59EB4: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59EB8: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82D59EBC: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59EC0: 48006441  bl 0x82d60300
	ctx.lr = 0x82D59EC4;
	sub_82D60300(ctx, base);
	// 82D59EC4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82D59EC8: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D59ECC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59ED0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D59ED4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D59ED8: 4E800421  bctrl
	ctx.lr = 0x82D59EDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D59EDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D59EE0: 4BFFADB9  bl 0x82d54c98
	ctx.lr = 0x82D59EE4;
	sub_82D54C98(ctx, base);
	// 82D59EE4: 3FE08333  lis r31, -0x7ccd
	ctx.r[31].s64 = -2093809664;
	// 82D59EE8: 807F7414  lwz r3, 0x7414(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(29716 as u32) ) } as u64;
	// 82D59EEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59EF0: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 82D59EF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D59EF8: 4E800421  bctrl
	ctx.lr = 0x82D59EFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D59EFC: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82D59F00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D59F04: 419A0054  beq cr6, 0x82d59f58
	if ctx.cr[6].eq {
	pc = 0x82D59F58; continue 'dispatch;
	}
	// 82D59F08: 807F7414  lwz r3, 0x7414(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(29716 as u32) ) } as u64;
	// 82D59F0C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82D59F10: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D59F14: 93C10058  stw r30, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[30].u32 ) };
	// 82D59F18: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82D59F1C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82D59F20: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82D59F24: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82D59F28: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59F2C: 9BA10050  stb r29, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u8 ) };
	// 82D59F30: 88AB0000  lbz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59F34: 816A0080  lwz r11, 0x80(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D59F38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D59F3C: 4E800421  bctrl
	ctx.lr = 0x82D59F40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D59F40: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D59F44: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D59F48: 41990008  bgt cr6, 0x82d59f50
	if ctx.cr[6].gt {
	pc = 0x82D59F50; continue 'dispatch;
	}
	// 82D59F4C: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82D59F50: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82D59F54: 4800610D  bl 0x82d60060
	ctx.lr = 0x82D59F58;
	sub_82D60060(ctx, base);
	// 82D59F58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D59F5C: 4BFFAA95  bl 0x82d549f0
	ctx.lr = 0x82D59F60;
	sub_82D549F0(ctx, base);
	// 82D59F60: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82D59F64: 997C76F8  stb r11, 0x76f8(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(30456 as u32), ctx.r[11].u8 ) };
	// 82D59F68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82D59F6C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D59F70: 4BF4F4E8  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D59F78 size=124
    let mut pc: u32 = 0x82D59F78;
    'dispatch: loop {
        match pc {
            0x82D59F78 => {
    //   block [0x82D59F78..0x82D59FF4)
	// 82D59F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D59F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D59F80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D59F84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D59F88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D59F8C: 3FE08333  lis r31, -0x7ccd
	ctx.r[31].s64 = -2093809664;
	// 82D59F90: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D59F94: 807F7700  lwz r3, 0x7700(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(30464 as u32) ) } as u64;
	// 82D59F98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D59F9C: 419A003C  beq cr6, 0x82d59fd8
	if ctx.cr[6].eq {
	pc = 0x82D59FD8; continue 'dispatch;
	}
	// 82D59FA0: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D59FA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D59FA8: 419A0030  beq cr6, 0x82d59fd8
	if ctx.cr[6].eq {
	pc = 0x82D59FD8; continue 'dispatch;
	}
	// 82D59FAC: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D59FB0: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D59FB4: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D59FB8: 556B043E  clrlwi r11, r11, 0x10
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	// 82D59FBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D59FC0: 409A0018  bne cr6, 0x82d59fd8
	if !ctx.cr[6].eq {
	pc = 0x82D59FD8; continue 'dispatch;
	}
	// 82D59FC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59FC8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D59FCC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D59FD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D59FD4: 4E800421  bctrl
	ctx.lr = 0x82D59FD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D59FD8: 93DF7700  stw r30, 0x7700(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(30464 as u32), ctx.r[30].u32 ) };
	// 82D59FDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D59FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D59FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D59FE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D59FEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D59FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D59FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D59FF8 size=72
    let mut pc: u32 = 0x82D59FF8;
    'dispatch: loop {
        match pc {
            0x82D59FF8 => {
    //   block [0x82D59FF8..0x82D5A040)
	// 82D59FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D59FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5A000: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5A004: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5A008: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5A00C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A010: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82D5A014: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5A018: 4E800421  bctrl
	ctx.lr = 0x82D5A01C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5A01C: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82D5A020: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82D5A024: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D5A028: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D5A02C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5A030: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5A034: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5A038: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5A03C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5A040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5A040 size=24
    let mut pc: u32 = 0x82D5A040;
    'dispatch: loop {
        match pc {
            0x82D5A040 => {
    //   block [0x82D5A040..0x82D5A058)
	// 82D5A040: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A044: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D5A048: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5A04C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82D5A050: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5A054: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5A058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5A058 size=92
    let mut pc: u32 = 0x82D5A058;
    'dispatch: loop {
        match pc {
            0x82D5A058 => {
    //   block [0x82D5A058..0x82D5A0B4)
	// 82D5A058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5A05C: 4BF4F3B1  bl 0x82ca940c
	ctx.lr = 0x82D5A060;
	sub_82CA93D0(ctx, base);
	// 82D5A060: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5A064: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5A068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5A06C: 396B4210  addi r11, r11, 0x4210
	ctx.r[11].s64 = ctx.r[11].s64 + 16912;
	// 82D5A070: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D5A074: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82D5A078: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5A07C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82D5A080: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5A084: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D5A088: 4B50CEB9  bl 0x82266f40
	ctx.lr = 0x82D5A08C;
	sub_82266F40(ctx, base);
	// 82D5A08C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5A090: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82D5A094: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5A098: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82D5A09C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82D5A0A0: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82D5A0A4: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82D5A0A8: 93BF0024  stw r29, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 82D5A0AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5A0B0: 4BF4F3AC  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5A0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5A0B8 size=364
    let mut pc: u32 = 0x82D5A0B8;
    'dispatch: loop {
        match pc {
            0x82D5A0B8 => {
    //   block [0x82D5A0B8..0x82D5A224)
	// 82D5A0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5A0BC: 4BF4F335  bl 0x82ca93f0
	ctx.lr = 0x82D5A0C0;
	sub_82CA93D0(ctx, base);
	// 82D5A0C0: 9421FCB0  stwu r1, -0x350(r1)
	ea = ctx.r[1].u32.wrapping_add(-848 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5A0C4: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82D5A0C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5A0CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5A0D0: 388B6E98  addi r4, r11, 0x6e98
	ctx.r[4].s64 = ctx.r[11].s64 + 28312;
	// 82D5A0D4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82D5A0D8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82D5A0DC: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82D5A0E0: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82D5A0E4: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82D5A0E8: 4BFFE8A9  bl 0x82d58990
	ctx.lr = 0x82D5A0EC;
	sub_82D58990(ctx, base);
	// 82D5A0EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D5A0F0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82D5A0F4: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 82D5A0F8: 388100F0  addi r4, r1, 0xf0
	ctx.r[4].s64 = ctx.r[1].s64 + 240;
	// 82D5A0FC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82D5A100: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82D5A104: 88CB0000  lbz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A108: 48005BD9  bl 0x82d5fce0
	ctx.lr = 0x82D5A10C;
	sub_82D5FCE0(ctx, base);
	// 82D5A10C: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82D5A110: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5A114: 4BFFE185  bl 0x82d58298
	ctx.lr = 0x82D5A118;
	sub_82D58298(ctx, base);
	// 82D5A118: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5A11C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D5A120: 3B2B4258  addi r25, r11, 0x4258
	ctx.r[25].s64 = ctx.r[11].s64 + 16984;
	// 82D5A124: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5A128: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5A12C: 3B0B4250  addi r24, r11, 0x4250
	ctx.r[24].s64 = ctx.r[11].s64 + 16976;
	// 82D5A130: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82D5A134: 3B810070  addi r28, r1, 0x70
	ctx.r[28].s64 = ctx.r[1].s64 + 112;
	// 82D5A138: 3AEB6EA0  addi r23, r11, 0x6ea0
	ctx.r[23].s64 = ctx.r[11].s64 + 28320;
	// 82D5A13C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5A140: 3ACB4248  addi r22, r11, 0x4248
	ctx.r[22].s64 = ctx.r[11].s64 + 16968;
	// 82D5A144: 4BFFDCAD  bl 0x82d57df0
	ctx.lr = 0x82D5A148;
	sub_82D57DF0(ctx, base);
	// 82D5A148: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 82D5A14C: 4BFFDC55  bl 0x82d57da0
	ctx.lr = 0x82D5A150;
	sub_82D57DA0(ctx, base);
	// 82D5A150: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82D5A154: 4BFFDDD5  bl 0x82d57f28
	ctx.lr = 0x82D5A158;
	sub_82D57F28(ctx, base);
	// 82D5A158: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82D5A15C: 4BFFDC95  bl 0x82d57df0
	ctx.lr = 0x82D5A160;
	sub_82D57DF0(ctx, base);
	// 82D5A160: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D5A164: 4BFFDC8D  bl 0x82d57df0
	ctx.lr = 0x82D5A168;
	sub_82D57DF0(ctx, base);
	// 82D5A168: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 82D5A16C: 4BFFDC85  bl 0x82d57df0
	ctx.lr = 0x82D5A170;
	sub_82D57DF0(ctx, base);
	// 82D5A170: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D5A174: 4BFFDC7D  bl 0x82d57df0
	ctx.lr = 0x82D5A178;
	sub_82D57DF0(ctx, base);
	// 82D5A178: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82D5A17C: 4BFFDC75  bl 0x82d57df0
	ctx.lr = 0x82D5A180;
	sub_82D57DF0(ctx, base);
	// 82D5A180: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82D5A184: 4BFFDC6D  bl 0x82d57df0
	ctx.lr = 0x82D5A188;
	sub_82D57DF0(ctx, base);
	// 82D5A188: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82D5A18C: 4BFFDC65  bl 0x82d57df0
	ctx.lr = 0x82D5A190;
	sub_82D57DF0(ctx, base);
	// 82D5A190: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D5A194: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D5A198: 386100F0  addi r3, r1, 0xf0
	ctx.r[3].s64 = ctx.r[1].s64 + 240;
	// 82D5A19C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5A1A0: 4E800421  bctrl
	ctx.lr = 0x82D5A1A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5A1A4: 7F4B0774  extsb r11, r26
	ctx.r[11].s64 = ctx.r[26].s8 as i64;
	// 82D5A1A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5A1AC: 419A0060  beq cr6, 0x82d5a20c
	if ctx.cr[6].eq {
	pc = 0x82D5A20C; continue 'dispatch;
	}
	// 82D5A1B0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82D5A1B4: 48006C4D  bl 0x82d60e00
	ctx.lr = 0x82D5A1B8;
	sub_82D60E00(ctx, base);
	// 82D5A1B8: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 82D5A1BC: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82D5A1C0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82D5A1C4: 48006C35  bl 0x82d60df8
	ctx.lr = 0x82D5A1C8;
	sub_82D60DF8(ctx, base);
	// 82D5A1C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D5A1CC: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 82D5A1D0: 40990034  ble cr6, 0x82d5a204
	if !ctx.cr[6].gt {
	pc = 0x82D5A204; continue 'dispatch;
	}
	// 82D5A1D4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5A1D8: 809F0024  lwz r4, 0x24(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D5A1DC: 386B4234  addi r3, r11, 0x4234
	ctx.r[3].s64 = ctx.r[11].s64 + 16948;
	// 82D5A1E0: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D5A1E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5A1E8: 4E800421  bctrl
	ctx.lr = 0x82D5A1EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5A1EC: 38BEFFFE  addi r5, r30, -2
	ctx.r[5].s64 = ctx.r[30].s64 + -2;
	// 82D5A1F0: 80FF0024  lwz r7, 0x24(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82D5A1F4: 388100A8  addi r4, r1, 0xa8
	ctx.r[4].s64 = ctx.r[1].s64 + 168;
	// 82D5A1F8: 80DF0020  lwz r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82D5A1FC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82D5A200: 48006BE1  bl 0x82d60de0
	ctx.lr = 0x82D5A204;
	sub_82D60DE0(ctx, base);
	// 82D5A204: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82D5A208: 48006C11  bl 0x82d60e18
	ctx.lr = 0x82D5A20C;
	sub_82D60E18(ctx, base);
	// 82D5A20C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5A210: 4BFFE221  bl 0x82d58430
	ctx.lr = 0x82D5A214;
	sub_82D58430(ctx, base);
	// 82D5A214: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82D5A218: 48005CE1  bl 0x82d5fef8
	ctx.lr = 0x82D5A21C;
	sub_82D5FEF8(ctx, base);
	// 82D5A21C: 38210350  addi r1, r1, 0x350
	ctx.r[1].s64 = ctx.r[1].s64 + 848;
	// 82D5A220: 4BF4F220  b 0x82ca9440
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5A228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5A228 size=20
    let mut pc: u32 = 0x82D5A228;
    'dispatch: loop {
        match pc {
            0x82D5A228 => {
    //   block [0x82D5A228..0x82D5A23C)
	// 82D5A228: 7CAB0774  extsb r11, r5
	ctx.r[11].s64 = ctx.r[5].s8 as i64;
	// 82D5A22C: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82D5A230: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5A234: 419A0008  beq cr6, 0x82d5a23c
	if ctx.cr[6].eq {
		sub_82D5A23C(ctx, base);
		return;
	}
	// 82D5A238: 48006A30  b 0x82d60c68
	sub_82D60C68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5A23C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5A23C size=8
    let mut pc: u32 = 0x82D5A23C;
    'dispatch: loop {
        match pc {
            0x82D5A23C => {
    //   block [0x82D5A23C..0x82D5A244)
	// 82D5A23C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5A240: 4B50B9C0  b 0x82265c00
	sub_82265C00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5A248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5A248 size=80
    let mut pc: u32 = 0x82D5A248;
    'dispatch: loop {
        match pc {
            0x82D5A248 => {
    //   block [0x82D5A248..0x82D5A298)
	// 82D5A248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5A24C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5A250: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5A254: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5A258: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82D5A25C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5A260: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82D5A264: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82D5A268: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82D5A26C: 4800641D  bl 0x82d60688
	ctx.lr = 0x82D5A270;
	sub_82D60688(ctx, base);
	// 82D5A270: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5A274: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5A278: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82D5A27C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D5A280: 997F0000  stb r11, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5A284: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5A288: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5A28C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5A290: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5A294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5A298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5A298 size=8
    let mut pc: u32 = 0x82D5A298;
    'dispatch: loop {
        match pc {
            0x82D5A298 => {
    //   block [0x82D5A298..0x82D5A2A0)
	// 82D5A298: 38630008  addi r3, r3, 8
	ctx.r[3].s64 = ctx.r[3].s64 + 8;
	// 82D5A29C: 480062E4  b 0x82d60580
	sub_82D60580(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5A2A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5A2A0 size=292
    let mut pc: u32 = 0x82D5A2A0;
    'dispatch: loop {
        match pc {
            0x82D5A2A0 => {
    //   block [0x82D5A2A0..0x82D5A35C)
	// 82D5A2A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5A2A4: 4BF4F15D  bl 0x82ca9400
	ctx.lr = 0x82D5A2A8;
	sub_82CA93D0(ctx, base);
	// 82D5A2A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5A2AC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82D5A2B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5A2B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82D5A2B8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82D5A2BC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 82D5A2C0: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 82D5A2C4: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82D5A2C8: 409A0024  bne cr6, 0x82d5a2ec
	if !ctx.cr[6].eq {
	pc = 0x82D5A2EC; continue 'dispatch;
	}
	// 82D5A2CC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D5A2D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5A2D4: 419A0018  beq cr6, 0x82d5a2ec
	if ctx.cr[6].eq {
	pc = 0x82D5A2EC; continue 'dispatch;
	}
	// 82D5A2D8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5A2DC: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D5A2E0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5A2E4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82D5A2E8: 83CBFFFC  lwz r30, -4(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82D5A2EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A2F0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5A2F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5A2F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5A2FC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D5A300: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5A304: 4E800421  bctrl
	ctx.lr = 0x82D5A308;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5A308: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A30C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5A310: 409A0010  bne cr6, 0x82d5a320
	if !ctx.cr[6].eq {
	pc = 0x82D5A320; continue 'dispatch;
	}
	// 82D5A314: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5A318: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D5A31C: 4BF4F134  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
	// 82D5A320: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D5A324: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82D5A328: 388B0CA0  addi r4, r11, 0xca0
	ctx.r[4].s64 = ctx.r[11].s64 + 3232;
	// 82D5A32C: 2B1D0003  cmplwi cr6, r29, 3
	ctx.cr[6].compare_u32(ctx.r[29].u32, 3 as u32, &mut ctx.xer);
	// 82D5A330: 4199005C  bgt cr6, 0x82d5a38c
	if ctx.cr[6].gt {
	pc = 0x82D5A38C; continue 'dispatch;
	}
	// 82D5A334: 3D8082D6  lis r12, -0x7d2a
	ctx.r[12].s64 = -2099904512;
	// 82D5A338: 398CA34C  addi r12, r12, -0x5cb4
	ctx.r[12].s64 = ctx.r[12].s64 + -23732;
	// 82D5A33C: 57A0103A  slwi r0, r29, 2
	ctx.r[0].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82D5A340: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82D5A344: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82D5A348: 4E800420  bctr
	match ctx.r[29].u64 {
		0 => {
	pc = 0x82D5A35C; continue 'dispatch;
		},
		1 => {
	pc = 0x82D5A368; continue 'dispatch;
		},
		2 => {
	pc = 0x82D5A374; continue 'dispatch;
		},
		3 => {
	pc = 0x82D5A380; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82D5A34C: 82D5A35C  lwz r22, -0x5ca4(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-23716 as u32) ) } as u64;
	// 82D5A350: 82D5A368  lwz r22, -0x5c98(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-23704 as u32) ) } as u64;
	// 82D5A354: 82D5A374  lwz r22, -0x5c8c(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 82D5A358: 82D5A380  lwz r22, -0x5c80(r21)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[21].u32.wrapping_add(-23680 as u32) ) } as u64;
            }
            0x82D5A35C => {
    //   block [0x82D5A35C..0x82D5A368)
	// 82D5A35C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5A360: 388B4264  addi r4, r11, 0x4264
	ctx.r[4].s64 = ctx.r[11].s64 + 16996;
	// 82D5A364: 48000028  b 0x82d5a38c
	pc = 0x82D5A38C; continue 'dispatch;
            }
            0x82D5A368 => {
    //   block [0x82D5A368..0x82D5A374)
	// 82D5A368: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5A36C: 388B425C  addi r4, r11, 0x425c
	ctx.r[4].s64 = ctx.r[11].s64 + 16988;
	// 82D5A370: 4800001C  b 0x82d5a38c
	pc = 0x82D5A38C; continue 'dispatch;
            }
            0x82D5A374 => {
    //   block [0x82D5A374..0x82D5A380)
	// 82D5A374: 3D60820F  lis r11, -0x7df1
	ctx.r[11].s64 = -2112946176;
	// 82D5A378: 388B29C8  addi r4, r11, 0x29c8
	ctx.r[4].s64 = ctx.r[11].s64 + 10696;
	// 82D5A37C: 4800000C  b 0x82d5a388
	pc = 0x82D5A388; continue 'dispatch;
            }
            0x82D5A380 => {
    //   block [0x82D5A380..0x82D5A3C4)
	// 82D5A380: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82D5A384: 388B7D08  addi r4, r11, 0x7d08
	ctx.r[4].s64 = ctx.r[11].s64 + 32008;
	// 82D5A388: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82D5A38C: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82D5A390: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82D5A394: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82D5A398: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82D5A39C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5A3A0: 4BFFFD19  bl 0x82d5a0b8
	ctx.lr = 0x82D5A3A4;
	sub_82D5A0B8(ctx, base);
	// 82D5A3A4: 2F1D0002  cmpwi cr6, r29, 2
	ctx.cr[6].compare_i32(ctx.r[29].s32, 2, &mut ctx.xer);
	// 82D5A3A8: 419A0010  beq cr6, 0x82d5a3b8
	if ctx.cr[6].eq {
	pc = 0x82D5A3B8; continue 'dispatch;
	}
	// 82D5A3AC: 2F1D0003  cmpwi cr6, r29, 3
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3, &mut ctx.xer);
	// 82D5A3B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82D5A3B4: 409A0008  bne cr6, 0x82d5a3bc
	if !ctx.cr[6].eq {
	pc = 0x82D5A3BC; continue 'dispatch;
	}
	// 82D5A3B8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82D5A3BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82D5A3C0: 4BF4F090  b 0x82ca9450
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5A3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5A3C8 size=112
    let mut pc: u32 = 0x82D5A3C8;
    'dispatch: loop {
        match pc {
            0x82D5A3C8 => {
    //   block [0x82D5A3C8..0x82D5A438)
	// 82D5A3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5A3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5A3D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5A3D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5A3D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5A3DC: 3BE30014  addi r31, r3, 0x14
	ctx.r[31].s64 = ctx.r[3].s64 + 20;
	// 82D5A3E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5A3E4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82D5A3E8: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5A3EC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5A3F0: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5A3F4: 409A0010  bne cr6, 0x82d5a404
	if !ctx.cr[6].eq {
	pc = 0x82D5A404; continue 'dispatch;
	}
	// 82D5A3F8: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82D5A3FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5A400: 4BFFCB99  bl 0x82d56f98
	ctx.lr = 0x82D5A404;
	sub_82D56F98(ctx, base);
	// 82D5A404: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5A408: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A40C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5A410: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 82D5A414: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5A418: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5A41C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82D5A420: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5A424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5A428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5A42C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D5A430: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5A434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5A438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5A438 size=16
    let mut pc: u32 = 0x82D5A438;
    'dispatch: loop {
        match pc {
            0x82D5A438 => {
    //   block [0x82D5A438..0x82D5A448)
	// 82D5A438: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D5A43C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D5A440: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82D5A444: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5A448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5A448 size=160
    let mut pc: u32 = 0x82D5A448;
    'dispatch: loop {
        match pc {
            0x82D5A448 => {
    //   block [0x82D5A448..0x82D5A4E8)
	// 82D5A448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5A44C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5A450: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5A454: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5A458: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5A45C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5A460: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5A464: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82D5A468: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5A46C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5A470: 409A0020  bne cr6, 0x82d5a490
	if !ctx.cr[6].eq {
	pc = 0x82D5A490; continue 'dispatch;
	}
	// 82D5A474: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A478: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D5A47C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5A480: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82D5A484: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82D5A488: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5A48C: 4BFFAE3D  bl 0x82d552c8
	ctx.lr = 0x82D5A490;
	sub_82D552C8(ctx, base);
	// 82D5A490: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82D5A494: 4B50CA75  bl 0x82266f08
	ctx.lr = 0x82D5A498;
	sub_82266F08(ctx, base);
	// 82D5A498: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5A49C: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D5A4A0: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D5A4A4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D5A4A8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5A4AC: 419A0020  beq cr6, 0x82d5a4cc
	if ctx.cr[6].eq {
	pc = 0x82D5A4CC; continue 'dispatch;
	}
	// 82D5A4B0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A4B4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D5A4B8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82D5A4BC: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5A4C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5A4C4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D5A4C8: 4BFFAE01  bl 0x82d552c8
	ctx.lr = 0x82D5A4CC;
	sub_82D552C8(ctx, base);
	// 82D5A4CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5A4D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5A4D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5A4D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5A4DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D5A4E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5A4E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5A4E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5A4E8 size=1328
    let mut pc: u32 = 0x82D5A4E8;
    'dispatch: loop {
        match pc {
            0x82D5A4E8 => {
    //   block [0x82D5A4E8..0x82D5AA18)
	// 82D5A4E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5A4EC: 4BF4EF0D  bl 0x82ca93f8
	ctx.lr = 0x82D5A4F0;
	sub_82CA93D0(ctx, base);
	// 82D5A4F0: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5A4F4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5A4F8: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82D5A4FC: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 82D5A500: 396B4270  addi r11, r11, 0x4270
	ctx.r[11].s64 = ctx.r[11].s64 + 17008;
	// 82D5A504: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82D5A508: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82D5A50C: 3F408000  lis r26, -0x8000
	ctx.r[26].s64 = -2147483648;
	// 82D5A510: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5A514: B3380006  sth r25, 6(r24)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[24].u32.wrapping_add(6 as u32), ctx.r[25].u16 ) };
	// 82D5A518: 91780000  stw r11, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5A51C: 91580008  stw r10, 8(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82D5A520: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D5A524: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 82D5A528: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82D5A52C: 93410058  stw r26, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[26].u32 ) };
	// 82D5A530: 9B380010  stb r25, 0x10(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(16 as u32), ctx.r[25].u8 ) };
	// 82D5A534: 419A0058  beq cr6, 0x82d5a58c
	if ctx.cr[6].eq {
	pc = 0x82D5A58C; continue 'dispatch;
	}
	// 82D5A538: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5A53C: 4BFFE6ED  bl 0x82d58c28
	ctx.lr = 0x82D5A540;
	sub_82D58C28(ctx, base);
	// 82D5A540: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D5A544: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82D5A548: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5A54C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D5A550: 40980024  bge cr6, 0x82d5a574
	if !ctx.cr[6].lt {
	pc = 0x82D5A574; continue 'dispatch;
	}
	// 82D5A554: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5A558: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5A55C: 41980008  blt cr6, 0x82d5a564
	if ctx.cr[6].lt {
	pc = 0x82D5A564; continue 'dispatch;
	}
	// 82D5A560: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D5A564: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5A568: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D5A56C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5A570: 4BFFC9A1  bl 0x82d56f10
	ctx.lr = 0x82D5A574;
	sub_82D56F10(ctx, base);
	// 82D5A574: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5A578: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5A57C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D5A580: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D5A584: 4BFFE7AD  bl 0x82d58d30
	ctx.lr = 0x82D5A588;
	sub_82D58D30(ctx, base);
	// 82D5A588: 48000020  b 0x82d5a5a8
	pc = 0x82D5A5A8; continue 'dispatch;
	// 82D5A58C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5A590: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D5A594: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5A598: 4BFFC979  bl 0x82d56f10
	ctx.lr = 0x82D5A59C;
	sub_82D56F10(ctx, base);
	// 82D5A59C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5A5A0: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 82D5A5A4: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82D5A5A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D5A5AC: 93A10080  stw r29, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[29].u32 ) };
	// 82D5A5B0: 93A10084  stw r29, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[29].u32 ) };
	// 82D5A5B4: 3B8B0C00  addi r28, r11, 0xc00
	ctx.r[28].s64 = ctx.r[11].s64 + 3072;
	// 82D5A5B8: 93410088  stw r26, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[26].u32 ) };
	// 82D5A5BC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82D5A5C0: 4BFFE669  bl 0x82d58c28
	ctx.lr = 0x82D5A5C4;
	sub_82D58C28(ctx, base);
	// 82D5A5C4: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82D5A5C8: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82D5A5CC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5A5D0: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D5A5D4: 40980024  bge cr6, 0x82d5a5f8
	if !ctx.cr[6].lt {
	pc = 0x82D5A5F8; continue 'dispatch;
	}
	// 82D5A5D8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5A5DC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5A5E0: 41980008  blt cr6, 0x82d5a5e8
	if ctx.cr[6].lt {
	pc = 0x82D5A5E8; continue 'dispatch;
	}
	// 82D5A5E4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D5A5E8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5A5EC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D5A5F0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82D5A5F4: 4BFFC91D  bl 0x82d56f10
	ctx.lr = 0x82D5A5F8;
	sub_82D56F10(ctx, base);
	// 82D5A5F8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5A5FC: 80610080  lwz r3, 0x80(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D5A600: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82D5A604: 93E10084  stw r31, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[31].u32 ) };
	// 82D5A608: 4BFFE729  bl 0x82d58d30
	ctx.lr = 0x82D5A60C;
	sub_82D58D30(ctx, base);
	// 82D5A60C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D5A610: 93A10070  stw r29, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 82D5A614: 3BCB0C04  addi r30, r11, 0xc04
	ctx.r[30].s64 = ctx.r[11].s64 + 3076;
	// 82D5A618: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 82D5A61C: 93410078  stw r26, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[26].u32 ) };
	// 82D5A620: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5A624: 4BFFE605  bl 0x82d58c28
	ctx.lr = 0x82D5A628;
	sub_82D58C28(ctx, base);
	// 82D5A628: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82D5A62C: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82D5A630: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5A634: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D5A638: 40980024  bge cr6, 0x82d5a65c
	if !ctx.cr[6].lt {
	pc = 0x82D5A65C; continue 'dispatch;
	}
	// 82D5A63C: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5A640: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5A644: 41980008  blt cr6, 0x82d5a64c
	if ctx.cr[6].lt {
	pc = 0x82D5A64C; continue 'dispatch;
	}
	// 82D5A648: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D5A64C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5A650: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D5A654: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82D5A658: 4BFFC8B9  bl 0x82d56f10
	ctx.lr = 0x82D5A65C;
	sub_82D56F10(ctx, base);
	// 82D5A65C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5A660: 80610070  lwz r3, 0x70(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D5A664: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D5A668: 93E10074  stw r31, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 82D5A66C: 4BFFE6C5  bl 0x82d58d30
	ctx.lr = 0x82D5A670;
	sub_82D58D30(ctx, base);
	// 82D5A670: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82D5A674: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82D5A678: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82D5A67C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D5A680: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82D5A684: 4BFFEFF5  bl 0x82d59678
	ctx.lr = 0x82D5A688;
	sub_82D59678(ctx, base);
	// 82D5A688: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D5A68C: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D5A690: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5A694: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5A698: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D5A69C: 40980024  bge cr6, 0x82d5a6c0
	if !ctx.cr[6].lt {
	pc = 0x82D5A6C0; continue 'dispatch;
	}
	// 82D5A6A0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5A6A4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5A6A8: 41980008  blt cr6, 0x82d5a6b0
	if ctx.cr[6].lt {
	pc = 0x82D5A6B0; continue 'dispatch;
	}
	// 82D5A6AC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D5A6B0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5A6B4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D5A6B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5A6BC: 4BFFC855  bl 0x82d56f10
	ctx.lr = 0x82D5A6C0;
	sub_82D56F10(ctx, base);
	// 82D5A6C0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5A6C4: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D5A6C8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5A6CC: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A6D0: 4BFFE661  bl 0x82d58d30
	ctx.lr = 0x82D5A6D4;
	sub_82D58D30(ctx, base);
	// 82D5A6D4: 816100A8  lwz r11, 0xa8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 82D5A6D8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5A6DC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5A6E0: 409A0020  bne cr6, 0x82d5a700
	if !ctx.cr[6].eq {
	pc = 0x82D5A700; continue 'dispatch;
	}
	// 82D5A6E4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A6E8: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D5A6EC: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5A6F0: 808100A0  lwz r4, 0xa0(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) } as u64;
	// 82D5A6F4: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5A6F8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5A6FC: 4BFFABCD  bl 0x82d552c8
	ctx.lr = 0x82D5A700;
	sub_82D552C8(ctx, base);
	// 82D5A700: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82D5A704: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5A708: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5A70C: 409A0020  bne cr6, 0x82d5a72c
	if !ctx.cr[6].eq {
	pc = 0x82D5A72C; continue 'dispatch;
	}
	// 82D5A710: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A714: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D5A718: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5A71C: 80810070  lwz r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82D5A720: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5A724: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5A728: 4BFFABA1  bl 0x82d552c8
	ctx.lr = 0x82D5A72C;
	sub_82D552C8(ctx, base);
	// 82D5A72C: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82D5A730: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5A734: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5A738: 409A0020  bne cr6, 0x82d5a758
	if !ctx.cr[6].eq {
	pc = 0x82D5A758; continue 'dispatch;
	}
	// 82D5A73C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A740: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D5A744: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5A748: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82D5A74C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5A750: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5A754: 4BFFAB75  bl 0x82d552c8
	ctx.lr = 0x82D5A758;
	sub_82D552C8(ctx, base);
	// 82D5A758: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5A75C: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 82D5A760: 7D63D8AE  lbzx r11, r3, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82D5A764: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D5A768: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 82D5A76C: 419A000C  beq cr6, 0x82d5a778
	if ctx.cr[6].eq {
	pc = 0x82D5A778; continue 'dispatch;
	}
	// 82D5A770: 2F0B005C  cmpwi cr6, r11, 0x5c
	ctx.cr[6].compare_i32(ctx.r[11].s32, 92, &mut ctx.xer);
	// 82D5A774: 409A000C  bne cr6, 0x82d5a780
	if !ctx.cr[6].eq {
	pc = 0x82D5A780; continue 'dispatch;
	}
	// 82D5A778: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82D5A77C: 4BFFFFE4  b 0x82d5a760
	pc = 0x82D5A760; continue 'dispatch;
	// 82D5A780: 3880003A  li r4, 0x3a
	ctx.r[4].s64 = 58;
	// 82D5A784: 4BFFE50D  bl 0x82d58c90
	ctx.lr = 0x82D5A788;
	sub_82D58C90(ctx, base);
	// 82D5A788: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D5A78C: 409A0230  bne cr6, 0x82d5a9bc
	if !ctx.cr[6].eq {
	pc = 0x82D5A9BC; continue 'dispatch;
	}
	// 82D5A790: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5A794: 7D6AD8AE  lbzx r11, r10, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82D5A798: 2B0B005C  cmplwi cr6, r11, 0x5c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 92 as u32, &mut ctx.xer);
	// 82D5A79C: 409A0010  bne cr6, 0x82d5a7ac
	if !ctx.cr[6].eq {
	pc = 0x82D5A7AC; continue 'dispatch;
	}
	// 82D5A7A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82D5A7A4: 3BCB0CA0  addi r30, r11, 0xca0
	ctx.r[30].s64 = ctx.r[11].s64 + 3232;
	// 82D5A7A8: 48000008  b 0x82d5a7b0
	pc = 0x82D5A7B0; continue 'dispatch;
	// 82D5A7AC: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 82D5A7B0: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 82D5A7B4: 93A10060  stw r29, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 82D5A7B8: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82D5A7BC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D5A7C0: 93410068  stw r26, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[26].u32 ) };
	// 82D5A7C4: 419A0058  beq cr6, 0x82d5a81c
	if ctx.cr[6].eq {
	pc = 0x82D5A81C; continue 'dispatch;
	}
	// 82D5A7C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5A7CC: 4BFFE45D  bl 0x82d58c28
	ctx.lr = 0x82D5A7D0;
	sub_82D58C28(ctx, base);
	// 82D5A7D0: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82D5A7D4: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82D5A7D8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5A7DC: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D5A7E0: 40980024  bge cr6, 0x82d5a804
	if !ctx.cr[6].lt {
	pc = 0x82D5A804; continue 'dispatch;
	}
	// 82D5A7E4: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5A7E8: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5A7EC: 41980008  blt cr6, 0x82d5a7f4
	if ctx.cr[6].lt {
	pc = 0x82D5A7F4; continue 'dispatch;
	}
	// 82D5A7F0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D5A7F4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5A7F8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D5A7FC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5A800: 4BFFC711  bl 0x82d56f10
	ctx.lr = 0x82D5A804;
	sub_82D56F10(ctx, base);
	// 82D5A804: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5A808: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D5A80C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D5A810: 93E10064  stw r31, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[31].u32 ) };
	// 82D5A814: 4BFFE51D  bl 0x82d58d30
	ctx.lr = 0x82D5A818;
	sub_82D58D30(ctx, base);
	// 82D5A818: 48000020  b 0x82d5a838
	pc = 0x82D5A838; continue 'dispatch;
	// 82D5A81C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5A820: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D5A824: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82D5A828: 4BFFC6E9  bl 0x82d56f10
	ctx.lr = 0x82D5A82C;
	sub_82D56F10(ctx, base);
	// 82D5A82C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D5A830: 93210064  stw r25, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[25].u32 ) };
	// 82D5A834: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82D5A838: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82D5A83C: 93A10090  stw r29, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[29].u32 ) };
	// 82D5A840: 93A10094  stw r29, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[29].u32 ) };
	// 82D5A844: 3BCB6D64  addi r30, r11, 0x6d64
	ctx.r[30].s64 = ctx.r[11].s64 + 28004;
	// 82D5A848: 93410098  stw r26, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[26].u32 ) };
	// 82D5A84C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5A850: 4BFFE3D9  bl 0x82d58c28
	ctx.lr = 0x82D5A854;
	sub_82D58C28(ctx, base);
	// 82D5A854: 81610098  lwz r11, 0x98(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82D5A858: 3BE30001  addi r31, r3, 1
	ctx.r[31].s64 = ctx.r[3].s64 + 1;
	// 82D5A85C: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5A860: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D5A864: 40980024  bge cr6, 0x82d5a888
	if !ctx.cr[6].lt {
	pc = 0x82D5A888; continue 'dispatch;
	}
	// 82D5A868: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5A86C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5A870: 41980008  blt cr6, 0x82d5a878
	if ctx.cr[6].lt {
	pc = 0x82D5A878; continue 'dispatch;
	}
	// 82D5A874: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D5A878: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5A87C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D5A880: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82D5A884: 4BFFC68D  bl 0x82d56f10
	ctx.lr = 0x82D5A888;
	sub_82D56F10(ctx, base);
	// 82D5A888: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5A88C: 80610090  lwz r3, 0x90(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82D5A890: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D5A894: 93E10094  stw r31, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[31].u32 ) };
	// 82D5A898: 4BFFE499  bl 0x82d58d30
	ctx.lr = 0x82D5A89C;
	sub_82D58D30(ctx, base);
	// 82D5A89C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82D5A8A0: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82D5A8A4: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82D5A8A8: 7FFCDA14  add r31, r28, r27
	ctx.r[31].u64 = ctx.r[28].u64 + ctx.r[27].u64;
	// 82D5A8AC: 4BFFEA65  bl 0x82d59310
	ctx.lr = 0x82D5A8B0;
	sub_82D59310(ctx, base);
	// 82D5A8B0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82D5A8B4: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82D5A8B8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5A8BC: 4BFFEAD5  bl 0x82d59390
	ctx.lr = 0x82D5A8C0;
	sub_82D59390(ctx, base);
	// 82D5A8C0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D5A8C4: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D5A8C8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5A8CC: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5A8D0: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 82D5A8D4: 40980024  bge cr6, 0x82d5a8f8
	if !ctx.cr[6].lt {
	pc = 0x82D5A8F8; continue 'dispatch;
	}
	// 82D5A8D8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82D5A8DC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82D5A8E0: 41980008  blt cr6, 0x82d5a8e8
	if ctx.cr[6].lt {
	pc = 0x82D5A8E8; continue 'dispatch;
	}
	// 82D5A8E4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82D5A8E8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82D5A8EC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82D5A8F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5A8F4: 4BFFC61D  bl 0x82d56f10
	ctx.lr = 0x82D5A8F8;
	sub_82D56F10(ctx, base);
	// 82D5A8F8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82D5A8FC: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 82D5A900: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5A904: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A908: 4BFFE429  bl 0x82d58d30
	ctx.lr = 0x82D5A90C;
	sub_82D58D30(ctx, base);
	// 82D5A90C: 816100B8  lwz r11, 0xb8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 82D5A910: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5A914: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5A918: 409A0020  bne cr6, 0x82d5a938
	if !ctx.cr[6].eq {
	pc = 0x82D5A938; continue 'dispatch;
	}
	// 82D5A91C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A920: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D5A924: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5A928: 808100B0  lwz r4, 0xb0(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82D5A92C: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5A930: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5A934: 4BFFA995  bl 0x82d552c8
	ctx.lr = 0x82D5A938;
	sub_82D552C8(ctx, base);
	// 82D5A938: 816100C8  lwz r11, 0xc8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(200 as u32) ) } as u64;
	// 82D5A93C: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5A940: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5A944: 409A0020  bne cr6, 0x82d5a964
	if !ctx.cr[6].eq {
	pc = 0x82D5A964; continue 'dispatch;
	}
	// 82D5A948: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A94C: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D5A950: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5A954: 808100C0  lwz r4, 0xc0(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(192 as u32) ) } as u64;
	// 82D5A958: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5A95C: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5A960: 4BFFA969  bl 0x82d552c8
	ctx.lr = 0x82D5A964;
	sub_82D552C8(ctx, base);
	// 82D5A964: 81610098  lwz r11, 0x98(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 82D5A968: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5A96C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5A970: 409A0020  bne cr6, 0x82d5a990
	if !ctx.cr[6].eq {
	pc = 0x82D5A990; continue 'dispatch;
	}
	// 82D5A974: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A978: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D5A97C: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5A980: 80810090  lwz r4, 0x90(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82D5A984: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5A988: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5A98C: 4BFFA93D  bl 0x82d552c8
	ctx.lr = 0x82D5A990;
	sub_82D552C8(ctx, base);
	// 82D5A990: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82D5A994: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5A998: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5A99C: 409A0020  bne cr6, 0x82d5a9bc
	if !ctx.cr[6].eq {
	pc = 0x82D5A9BC; continue 'dispatch;
	}
	// 82D5A9A0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A9A4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D5A9A8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5A9AC: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82D5A9B0: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5A9B4: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5A9B8: 4BFFA911  bl 0x82d552c8
	ctx.lr = 0x82D5A9BC;
	sub_82D552C8(ctx, base);
	// 82D5A9BC: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 82D5A9C0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5A9C4: 388B6B38  addi r4, r11, 0x6b38
	ctx.r[4].s64 = ctx.r[11].s64 + 27448;
	// 82D5A9C8: 4BF500D9  bl 0x82caaaa0
	ctx.lr = 0x82D5A9CC;
	sub_82CAAAA0(ctx, base);
	// 82D5A9CC: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82D5A9D0: 9078000C  stw r3, 0xc(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82D5A9D4: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5A9D8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82D5A9DC: 7C6A0034  cntlzw r10, r3
	ctx.r[10].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82D5A9E0: 554ADFFE  rlwinm r10, r10, 0x1b, 0x1f, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82D5A9E4: 694A0001  xori r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 ^ 1;
	// 82D5A9E8: 99580010  stb r10, 0x10(r24)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[24].u32.wrapping_add(16 as u32), ctx.r[10].u8 ) };
	// 82D5A9EC: 409A0020  bne cr6, 0x82d5aa0c
	if !ctx.cr[6].eq {
	pc = 0x82D5AA0C; continue 'dispatch;
	}
	// 82D5A9F0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5A9F4: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 82D5A9F8: 38C00016  li r6, 0x16
	ctx.r[6].s64 = 22;
	// 82D5A9FC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82D5AA00: 556500BE  clrlwi r5, r11, 2
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82D5AA04: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82D5AA08: 4BFFA8C1  bl 0x82d552c8
	ctx.lr = 0x82D5AA0C;
	sub_82D552C8(ctx, base);
	// 82D5AA0C: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82D5AA10: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82D5AA14: 4BF4EA34  b 0x82ca9448
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5AA18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5AA18 size=72
    let mut pc: u32 = 0x82D5AA18;
    'dispatch: loop {
        match pc {
            0x82D5AA18 => {
    //   block [0x82D5AA18..0x82D5AA60)
	// 82D5AA18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5AA1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5AA20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5AA24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5AA28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5AA2C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82D5AA30: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D5AA34: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5AA38: 4BF57159  bl 0x82cb1b90
	ctx.lr = 0x82D5AA3C;
	sub_82CB1B90(ctx, base);
	// 82D5AA3C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82D5AA40: 4199000C  bgt cr6, 0x82d5aa4c
	if ctx.cr[6].gt {
	pc = 0x82D5AA4C; continue 'dispatch;
	}
	// 82D5AA44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5AA48: 997F0010  stb r11, 0x10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82D5AA4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5AA50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5AA54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5AA58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5AA5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5AA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5AA60 size=12
    let mut pc: u32 = 0x82D5AA60;
    'dispatch: loop {
        match pc {
            0x82D5AA60 => {
    //   block [0x82D5AA60..0x82D5AA6C)
	// 82D5AA60: 89640010  lbz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82D5AA64: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5AA68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5AA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5AA70 size=48
    let mut pc: u32 = 0x82D5AA70;
    'dispatch: loop {
        match pc {
            0x82D5AA70 => {
    //   block [0x82D5AA70..0x82D5AAA0)
	// 82D5AA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5AA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5AA78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5AA7C: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5AA80: 4BF50AC9  bl 0x82cab548
	ctx.lr = 0x82D5AA84;
	sub_82CAB548(ctx, base);
	// 82D5AA84: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82D5AA88: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82D5AA8C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82D5AA90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82D5AA94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5AA98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5AA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5AAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5AAA0 size=8
    let mut pc: u32 = 0x82D5AAA0;
    'dispatch: loop {
        match pc {
            0x82D5AAA0 => {
    //   block [0x82D5AAA0..0x82D5AAA8)
	// 82D5AAA0: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5AAA4: 4BF50DDC  b 0x82cab880
	sub_82CAB880(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5AAA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5AAA8 size=136
    let mut pc: u32 = 0x82D5AAA8;
    'dispatch: loop {
        match pc {
            0x82D5AAA8 => {
    //   block [0x82D5AAA8..0x82D5AB30)
	// 82D5AAA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5AAAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5AAB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5AAB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5AAB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5AABC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5AAC0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5AAC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5AAC8: 396B4270  addi r11, r11, 0x4270
	ctx.r[11].s64 = ctx.r[11].s64 + 17008;
	// 82D5AACC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82D5AAD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82D5AAD4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5AAD8: 419A0008  beq cr6, 0x82d5aae0
	if ctx.cr[6].eq {
	pc = 0x82D5AAE0; continue 'dispatch;
	}
	// 82D5AADC: 4BF5010D  bl 0x82caabe8
	ctx.lr = 0x82D5AAE0;
	sub_82CAABE8(ctx, base);
	// 82D5AAE0: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5AAE4: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82D5AAE8: 396B39E0  addi r11, r11, 0x39e0
	ctx.r[11].s64 = ctx.r[11].s64 + 14816;
	// 82D5AAEC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82D5AAF0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5AAF4: 419A0020  beq cr6, 0x82d5ab14
	if ctx.cr[6].eq {
	pc = 0x82D5AB14; continue 'dispatch;
	}
	// 82D5AAF8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5AAFC: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82D5AB00: 38C00019  li r6, 0x19
	ctx.r[6].s64 = 25;
	// 82D5AB04: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5AB08: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5AB0C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82D5AB10: 4BFFA7B9  bl 0x82d552c8
	ctx.lr = 0x82D5AB14;
	sub_82D552C8(ctx, base);
	// 82D5AB14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5AB18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82D5AB1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5AB20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5AB24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82D5AB28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5AB2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5AB30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5AB30 size=220
    let mut pc: u32 = 0x82D5AB30;
    'dispatch: loop {
        match pc {
            0x82D5AB30 => {
    //   block [0x82D5AB30..0x82D5AC0C)
	// 82D5AB30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5AB34: 4BF4E8D9  bl 0x82ca940c
	ctx.lr = 0x82D5AB38;
	sub_82CA93D0(ctx, base);
	// 82D5AB38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5AB3C: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5AB40: 3BC00004  li r30, 4
	ctx.r[30].s64 = 4;
	// 82D5AB44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D5AB48: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 82D5AB4C: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82D5AB50: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82D5AB54: 4BFFA6F5  bl 0x82d55248
	ctx.lr = 0x82D5AB58;
	sub_82D55248(ctx, base);
	// 82D5AB58: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82D5AB5C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5AB60: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82D5AB64: 4BFFF985  bl 0x82d5a4e8
	ctx.lr = 0x82D5AB68;
	sub_82D5A4E8(ctx, base);
	// 82D5AB68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5AB6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5AB70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5AB74: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5AB78: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82D5AB7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5AB80: 4E800421  bctrl
	ctx.lr = 0x82D5AB84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5AB84: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5AB88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5AB8C: 409A0074  bne cr6, 0x82d5ac00
	if !ctx.cr[6].eq {
	pc = 0x82D5AC00; continue 'dispatch;
	}
	// 82D5AB90: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 82D5AB94: 7C7EE82E  lwzx r3, r30, r29
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82D5AB98: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82D5AB9C: 4BFFA6AD  bl 0x82d55248
	ctx.lr = 0x82D5ABA0;
	sub_82D55248(ctx, base);
	// 82D5ABA0: 39600024  li r11, 0x24
	ctx.r[11].s64 = 36;
	// 82D5ABA4: 38A01000  li r5, 0x1000
	ctx.r[5].s64 = 4096;
	// 82D5ABA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5ABAC: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82D5ABB0: 480067B1  bl 0x82d61360
	ctx.lr = 0x82D5ABB4;
	sub_82D61360(ctx, base);
	// 82D5ABB4: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5ABB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D5ABBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5ABC0: 419A0034  beq cr6, 0x82d5abf4
	if ctx.cr[6].eq {
	pc = 0x82D5ABF4; continue 'dispatch;
	}
	// 82D5ABC4: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D5ABC8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D5ABCC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D5ABD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5ABD4: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D5ABD8: 409A001C  bne cr6, 0x82d5abf4
	if !ctx.cr[6].eq {
	pc = 0x82D5ABF4; continue 'dispatch;
	}
	// 82D5ABDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5ABE0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D5ABE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5ABE8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5ABEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5ABF0: 4E800421  bctrl
	ctx.lr = 0x82D5ABF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5ABF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5ABF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5ABFC: 4BF4E860  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
	// 82D5AC00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5AC04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5AC08: 4BF4E854  b 0x82ca945c
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5AC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5AC10 size=212
    let mut pc: u32 = 0x82D5AC10;
    'dispatch: loop {
        match pc {
            0x82D5AC10 => {
    //   block [0x82D5AC10..0x82D5ACE4)
	// 82D5AC10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5AC14: 4BF4E7F5  bl 0x82ca9408
	ctx.lr = 0x82D5AC18;
	sub_82CA93D0(ctx, base);
	// 82D5AC18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5AC1C: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5AC20: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 82D5AC24: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5AC28: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 82D5AC2C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82D5AC30: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D5AC34: 4BFFA615  bl 0x82d55248
	ctx.lr = 0x82D5AC38;
	sub_82D55248(ctx, base);
	// 82D5AC38: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 82D5AC3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5AC40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82D5AC44: 396B43FC  addi r11, r11, 0x43fc
	ctx.r[11].s64 = ctx.r[11].s64 + 17404;
	// 82D5AC48: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82D5AC4C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82D5AC50: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82D5AC54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82D5AC58: B13F0004  sth r9, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82D5AC5C: 995F000C  stb r10, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82D5AC60: 419A0014  beq cr6, 0x82d5ac74
	if ctx.cr[6].eq {
	pc = 0x82D5AC74; continue 'dispatch;
	}
	// 82D5AC64: 3D60820B  lis r11, -0x7df5
	ctx.r[11].s64 = -2113208320;
	// 82D5AC68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82D5AC6C: 38AB8EA4  addi r5, r11, -0x715c
	ctx.r[5].s64 = ctx.r[11].s64 + -29020;
	// 82D5AC70: 4B6F8569  bl 0x824531d8
	ctx.lr = 0x82D5AC74;
	sub_824531D8(ctx, base);
	// 82D5AC74: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 82D5AC78: 7C7DE02E  lwzx r3, r29, r28
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82D5AC7C: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82D5AC80: 4BFFA5C9  bl 0x82d55248
	ctx.lr = 0x82D5AC84;
	sub_82D55248(ctx, base);
	// 82D5AC84: 3960001C  li r11, 0x1c
	ctx.r[11].s64 = 28;
	// 82D5AC88: 38A01000  li r5, 0x1000
	ctx.r[5].s64 = 4096;
	// 82D5AC8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5AC90: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82D5AC94: 480051AD  bl 0x82d5fe40
	ctx.lr = 0x82D5AC98;
	sub_82D5FE40(ctx, base);
	// 82D5AC98: A17F0004  lhz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82D5AC9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D5ACA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82D5ACA4: 419A0034  beq cr6, 0x82d5acd8
	if ctx.cr[6].eq {
	pc = 0x82D5ACD8; continue 'dispatch;
	}
	// 82D5ACA8: A17F0006  lhz r11, 6(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 82D5ACAC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82D5ACB0: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 82D5ACB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82D5ACB8: B17F0006  sth r11, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82D5ACBC: 409A001C  bne cr6, 0x82d5acd8
	if !ctx.cr[6].eq {
	pc = 0x82D5ACD8; continue 'dispatch;
	}
	// 82D5ACC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5ACC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82D5ACC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82D5ACCC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82D5ACD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82D5ACD4: 4E800421  bctrl
	ctx.lr = 0x82D5ACD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82D5ACD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5ACDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82D5ACE0: 4BF4E778  b 0x82ca9458
	sub_82CA9420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5ACE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5ACE8 size=332
    let mut pc: u32 = 0x82D5ACE8;
    'dispatch: loop {
        match pc {
            0x82D5ACE8 => {
    //   block [0x82D5ACE8..0x82D5AE34)
	// 82D5ACE8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82D5ACEC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82D5ACF0: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 82D5ACF4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82D5ACF8: C00A0C14  lfs f0, 0xc14(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(3092 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5ACFC: 419A0074  beq cr6, 0x82d5ad70
	if ctx.cr[6].eq {
	pc = 0x82D5AD70; continue 'dispatch;
	}
	// 82D5AD00: 3CE08202  lis r7, -0x7dfe
	ctx.r[7].s64 = -2113798144;
	// 82D5AD04: D001FFD0  stfs f0, -0x30(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), tmp.u32 ) };
	// 82D5AD08: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82D5AD0C: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82D5AD10: 38C74C40  addi r6, r7, 0x4c40
	ctx.r[6].s64 = ctx.r[7].s64 + 19520;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5AE34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5AE34 size=84
    let mut pc: u32 = 0x82D5AE34;
    'dispatch: loop {
        match pc {
            0x82D5AE34 => {
    //   block [0x82D5AE34..0x82D5AE88)
	// 82D5AE34: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5AE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5AE88 size=100
    let mut pc: u32 = 0x82D5AE88;
    'dispatch: loop {
        match pc {
            0x82D5AE88 => {
    //   block [0x82D5AE88..0x82D5AEEC)
	// 82D5AE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5AE8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5AE90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5AE94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5AE98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5AE9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5AEA0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5AEA4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82D5AEA8: 4BFFBA61  bl 0x82d56908
	ctx.lr = 0x82D5AEAC;
	sub_82D56908(ctx, base);
	// 82D5AEAC: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82D5AEB0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82D5AEB4: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5AEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5AEF0 size=76
    let mut pc: u32 = 0x82D5AEF0;
    'dispatch: loop {
        match pc {
            0x82D5AEF0 => {
    //   block [0x82D5AEF0..0x82D5AF3C)
	// 82D5AEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5AEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5AEF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5AEFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5AF00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5AF04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5AF08: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5AF0C: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82D5AF10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82D5AF14: 4BFFC86D  bl 0x82d57780
	ctx.lr = 0x82D5AF18;
	sub_82D57780(ctx, base);
	// 82D5AF18: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5AF40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5AF40 size=80
    let mut pc: u32 = 0x82D5AF40;
    'dispatch: loop {
        match pc {
            0x82D5AF40 => {
    //   block [0x82D5AF40..0x82D5AF90)
	// 82D5AF40: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82D5AF44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5AF48: 3D207F80  lis r9, 0x7f80
	ctx.r[9].s64 = 2139095040;
	// 82D5AF4C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82D5AF50: 419A0024  beq cr6, 0x82d5af74
	if ctx.cr[6].eq {
	pc = 0x82D5AF74; continue 'dispatch;
	}
	// 82D5AF54: 2F0B000B  cmpwi cr6, r11, 0xb
	ctx.cr[6].compare_i32(ctx.r[11].s32, 11, &mut ctx.xer);
	// 82D5AF58: 419A001C  beq cr6, 0x82d5af74
	if ctx.cr[6].eq {
	pc = 0x82D5AF74; continue 'dispatch;
	}
	// 82D5AF5C: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5AF60: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 82D5AF64: 8101FFF0  lwz r8, -0x10(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82D5AF68: 55080050  rlwinm r8, r8, 0, 1, 8
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0xFFFFFFFFu64;
	// 82D5AF6C: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82D5AF70: 419A0020  beq cr6, 0x82d5af90
	if ctx.cr[6].eq {
		sub_82D5AF90(ctx, base);
		return;
	}
	// 82D5AF74: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82D5AF78: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82D5AF7C: 2F0B000C  cmpwi cr6, r11, 0xc
	ctx.cr[6].compare_i32(ctx.r[11].s32, 12, &mut ctx.xer);
	// 82D5AF80: 4198FFCC  blt cr6, 0x82d5af4c
	if ctx.cr[6].lt {
	pc = 0x82D5AF4C; continue 'dispatch;
	}
	// 82D5AF84: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82D5AF88: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5AF8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5AF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5AF90 size=12
    let mut pc: u32 = 0x82D5AF90;
    'dispatch: loop {
        match pc {
            0x82D5AF90 => {
    //   block [0x82D5AF90..0x82D5AF9C)
	// 82D5AF90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82D5AF94: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82D5AF98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5AFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5AFA0 size=104
    let mut pc: u32 = 0x82D5AFA0;
    'dispatch: loop {
        match pc {
            0x82D5AFA0 => {
    //   block [0x82D5AFA0..0x82D5B008)
	// 82D5AFA0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5B008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82D5B008 size=208
    let mut pc: u32 = 0x82D5B008;
    'dispatch: loop {
        match pc {
            0x82D5B008 => {
    //   block [0x82D5B008..0x82D5B0D8)
	// 82D5B008: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82D5B00C: D021FFE0  stfs f1, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5B0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82D5B0D8 size=208
    let mut pc: u32 = 0x82D5B0D8;
    'dispatch: loop {
        match pc {
            0x82D5B0D8 => {
    //   block [0x82D5B0D8..0x82D5B1A8)
	// 82D5B0D8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5B1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82D5B1A8 size=212
    let mut pc: u32 = 0x82D5B1A8;
    'dispatch: loop {
        match pc {
            0x82D5B1A8 => {
    //   block [0x82D5B1A8..0x82D5B27C)
	// 82D5B1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5B1AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5B1B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5B1B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5B1B8: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5B1BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5B1C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82D5B1C4: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82D5B1C8: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 82D5B1CC: 4BFFC5B5  bl 0x82d57780
	ctx.lr = 0x82D5B1D0;
	sub_82D57780(ctx, base);
	// 82D5B1D0: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 82D5B1D4: C01F0020  lfs f0, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82D5B1D8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82D5B1DC: 38810100  addi r4, r1, 0x100
	ctx.r[4].s64 = ctx.r[1].s64 + 256;
	// 82D5B1E0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5B280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5B280 size=132
    let mut pc: u32 = 0x82D5B280;
    'dispatch: loop {
        match pc {
            0x82D5B280 => {
    //   block [0x82D5B280..0x82D5B304)
	// 82D5B280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5B284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5B288: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82D5B28C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5B290: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5B294: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82D5B298: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82D5B29C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82D5B2A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82D5B2A4: 4800653D  bl 0x82d617e0
	ctx.lr = 0x82D5B2A8;
	sub_82D617E0(ctx, base);
	// 82D5B2A8: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82D5B2AC: 89610081  lbz r11, 0x81(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(129 as u32) ) } as u64;
	// 82D5B2B0: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82D5B2B4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82D5B2B8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82D5B2BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82D5B308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82D5B308 size=68
    let mut pc: u32 = 0x82D5B308;
    'dispatch: loop {
        match pc {
            0x82D5B308 => {
    //   block [0x82D5B308..0x82D5B34C)
	// 82D5B308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82D5B30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82D5B310: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82D5B314: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82D5B318: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82D5B31C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82D5B320: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82D5B324: 4BFFBDBD  bl 0x82d570e0
	ctx.lr = 0x82D5B328;
	sub_82D570E0(ctx, base);
	// 82D5B328: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82D5B32C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82D5B330: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82D5B334: 4BFFFF4D  bl 0x82d5b280
	ctx.lr = 0x82D5B338;
	sub_82D5B280(ctx, base);
	// 82D5B338: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82D5B33C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82D5B340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82D5B344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82D5B348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


