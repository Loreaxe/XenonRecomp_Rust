pub fn sub_82405770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82405770 size=28
    let mut pc: u32 = 0x82405770;
    'dispatch: loop {
        match pc {
            0x82405770 => {
    //   block [0x82405770..0x8240578C)
	// 82405770: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82405774: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82405778: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8240577C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82405780: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 82405784: 806B3828  lwz r3, 0x3828(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14376 as u32) ) } as u64;
	// 82405788: 48003A20  b 0x824091a8
	sub_824091A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82405790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82405790 size=16
    let mut pc: u32 = 0x82405790;
    'dispatch: loop {
        match pc {
            0x82405790 => {
    //   block [0x82405790..0x824057A0)
	// 82405790: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82405794: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82405798: 806B3828  lwz r3, 0x3828(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14376 as u32) ) } as u64;
	// 8240579C: 4800267C  b 0x82407e18
	sub_82407E18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824057A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824057A0 size=16
    let mut pc: u32 = 0x824057A0;
    'dispatch: loop {
        match pc {
            0x824057A0 => {
    //   block [0x824057A0..0x824057B0)
	// 824057A0: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 824057A4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824057A8: 806B3828  lwz r3, 0x3828(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14376 as u32) ) } as u64;
	// 824057AC: 480026DC  b 0x82407e88
	sub_82407E88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824057B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824057B0 size=20
    let mut pc: u32 = 0x824057B0;
    'dispatch: loop {
        match pc {
            0x824057B0 => {
    //   block [0x824057B0..0x824057C4)
	// 824057B0: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 824057B4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824057B8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824057BC: 806B3828  lwz r3, 0x3828(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14376 as u32) ) } as u64;
	// 824057C0: 48002A80  b 0x82408240
	sub_82408240(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824057C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824057C8 size=20
    let mut pc: u32 = 0x824057C8;
    'dispatch: loop {
        match pc {
            0x824057C8 => {
    //   block [0x824057C8..0x824057DC)
	// 824057C8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 824057CC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824057D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824057D4: 806B3828  lwz r3, 0x3828(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14376 as u32) ) } as u64;
	// 824057D8: 48002AE8  b 0x824082c0
	sub_824082C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824057E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824057E0 size=24
    let mut pc: u32 = 0x824057E0;
    'dispatch: loop {
        match pc {
            0x824057E0 => {
    //   block [0x824057E0..0x824057F8)
	// 824057E0: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 824057E4: 1D430164  mulli r10, r3, 0x164
	ctx.r[10].s64 = ctx.r[3].s64 * 356;
	// 824057E8: 816B3828  lwz r11, 0x3828(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14376 as u32) ) } as u64;
	// 824057EC: 816B1C3C  lwz r11, 0x1c3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7228 as u32) ) } as u64;
	// 824057F0: 7C6B502E  lwzx r3, r11, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824057F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824057F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824057F8 size=12
    let mut pc: u32 = 0x824057F8;
    'dispatch: loop {
        match pc {
            0x824057F8 => {
    //   block [0x824057F8..0x82405804)
	// 824057F8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 824057FC: 806B3824  lwz r3, 0x3824(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14372 as u32) ) } as u64;
	// 82405800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82405808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82405808 size=12
    let mut pc: u32 = 0x82405808;
    'dispatch: loop {
        match pc {
            0x82405808 => {
    //   block [0x82405808..0x82405814)
	// 82405808: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8240580C: 806B3828  lwz r3, 0x3828(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14376 as u32) ) } as u64;
	// 82405810: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82405818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82405818 size=8
    let mut pc: u32 = 0x82405818;
    'dispatch: loop {
        match pc {
            0x82405818 => {
    //   block [0x82405818..0x82405820)
	// 82405818: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240581C: 4BFFFA04  b 0x82405220
	sub_82405220(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82405820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82405820 size=188
    let mut pc: u32 = 0x82405820;
    'dispatch: loop {
        match pc {
            0x82405820 => {
    //   block [0x82405820..0x824058DC)
	// 82405820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82405824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82405828: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240582C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82405830: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82405834: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82405838: 3BEB3824  addi r31, r11, 0x3824
	ctx.r[31].s64 = ctx.r[11].s64 + 14372;
	// 8240583C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82405840: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82405844: 409A001C  bne cr6, 0x82405860
	if !ctx.cr[6].eq {
	pc = 0x82405860; continue 'dispatch;
	}
	// 82405848: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240584C: 386BBEC4  addi r3, r11, -0x413c
	ctx.r[3].s64 = ctx.r[11].s64 + -16700;
	// 82405850: 4BEAD731  bl 0x822b2f80
	ctx.lr = 0x82405854;
	sub_822B2F80(ctx, base);
	// 82405854: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82405858: 60630029  ori r3, r3, 0x29
	ctx.r[3].u64 = ctx.r[3].u64 | 41;
	// 8240585C: 48000068  b 0x824058c4
	pc = 0x824058C4; continue 'dispatch;
	// 82405860: 4BFFFE79  bl 0x824056d8
	ctx.lr = 0x82405864;
	sub_824056D8(ctx, base);
	// 82405864: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82405868: 3FC08273  lis r30, -0x7d8d
	ctx.r[30].s64 = -2106392576;
	// 8240586C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82405870: 419A0028  beq cr6, 0x82405898
	if ctx.cr[6].eq {
	pc = 0x82405898; continue 'dispatch;
	}
	// 82405874: 480024ED  bl 0x82407d60
	ctx.lr = 0x82405878;
	sub_82407D60(ctx, base);
	// 82405878: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240587C: 48002F8D  bl 0x82408808
	ctx.lr = 0x82405880;
	sub_82408808(ctx, base);
	// 82405880: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82405884: 817E3804  lwz r11, 0x3804(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14340 as u32) ) } as u64;
	// 82405888: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8240588C: 4E800421  bctrl
	ctx.lr = 0x82405890;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82405890: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82405894: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82405898: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240589C: 4800227D  bl 0x82407b18
	ctx.lr = 0x824058A0;
	sub_82407B18(ctx, base);
	// 824058A0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824058A4: 48001F5D  bl 0x82407800
	ctx.lr = 0x824058A8;
	sub_82407800(ctx, base);
	// 824058A8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824058AC: 817E3804  lwz r11, 0x3804(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14340 as u32) ) } as u64;
	// 824058B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824058B4: 4E800421  bctrl
	ctx.lr = 0x824058B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824058B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824058BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824058C0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824058C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824058C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824058CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824058D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824058D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824058D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824058E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824058E0 size=556
    let mut pc: u32 = 0x824058E0;
    'dispatch: loop {
        match pc {
            0x824058E0 => {
    //   block [0x824058E0..0x82405B0C)
	// 824058E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824058E4: 4812F7C9  bl 0x825350ac
	ctx.lr = 0x824058E8;
	sub_82535080(ctx, base);
	// 824058E8: DBC1FFB0  stfd f30, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[30].u64 ) };
	// 824058EC: DBE1FFB8  stfd f31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 824058F0: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824058F4: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 824058F8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 824058FC: 3BCB3824  addi r30, r11, 0x3824
	ctx.r[30].s64 = ctx.r[11].s64 + 14372;
	// 82405900: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82405904: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82405908: 409A0010  bne cr6, 0x82405918
	if !ctx.cr[6].eq {
	pc = 0x82405918; continue 'dispatch;
	}
	// 8240590C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82405910: 60630029  ori r3, r3, 0x29
	ctx.r[3].u64 = ctx.r[3].u64 | 41;
	// 82405914: 480001E8  b 0x82405afc
	pc = 0x82405AFC; continue 'dispatch;
	// 82405918: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8240591C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82405920: 48004141  bl 0x82409a60
	ctx.lr = 0x82405924;
	sub_82409A60(ctx, base);
	// 82405924: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82405928: 48004059  bl 0x82409980
	ctx.lr = 0x8240592C;
	sub_82409980(ctx, base);
	// 8240592C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82405930: 4081002C  ble 0x8240595c
	if !ctx.cr[0].gt {
	pc = 0x8240595C; continue 'dispatch;
	}
	// 82405934: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82405938: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8240593C: 40990020  ble cr6, 0x8240595c
	if !ctx.cr[6].gt {
	pc = 0x8240595C; continue 'dispatch;
	}
	// 82405940: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82405944: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82405948: 480024B1  bl 0x82407df8
	ctx.lr = 0x8240594C;
	sub_82407DF8(ctx, base);
	// 8240594C: 4BFFFADD  bl 0x82405428
	ctx.lr = 0x82405950;
	sub_82405428(ctx, base);
	// 82405950: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82405954: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82405958: 4198FFE8  blt cr6, 0x82405940
	if ctx.cr[6].lt {
	pc = 0x82405940; continue 'dispatch;
	}
	// 8240595C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82405960: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82405964: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82405968: 48006F89  bl 0x8240c8f0
	ctx.lr = 0x8240596C;
	sub_8240C8F0(ctx, base);
	// 8240596C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82405970: 834B1C3C  lwz r26, 0x1c3c(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82405974: 480014DD  bl 0x82406e50
	ctx.lr = 0x82405978;
	sub_82406E50(ctx, base);
	// 82405978: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240597C: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82405980: 4081003C  ble 0x824059bc
	if !ctx.cr[0].gt {
	pc = 0x824059BC; continue 'dispatch;
	}
	// 82405984: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82405988: 48001881  bl 0x82407208
	ctx.lr = 0x8240598C;
	sub_82407208(ctx, base);
	// 8240598C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82405990: 409A0020  bne cr6, 0x824059b0
	if !ctx.cr[6].eq {
	pc = 0x824059B0; continue 'dispatch;
	}
	// 82405994: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82405998: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240599C: 48002B5D  bl 0x824084f8
	ctx.lr = 0x824059A0;
	sub_824084F8(ctx, base);
	// 824059A0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 824059A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824059A8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824059AC: 48001EE5  bl 0x82407890
	ctx.lr = 0x824059B0;
	sub_82407890(ctx, base);
	// 824059B0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824059B4: 7F1FE800  cmpw cr6, r31, r29
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824059B8: 4198FFCC  blt cr6, 0x82405984
	if ctx.cr[6].lt {
	pc = 0x82405984; continue 'dispatch;
	}
	// 824059BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824059C0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 824059C4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824059C8: 3F6B0002  addis r27, r11, 2
	ctx.r[27].s64 = ctx.r[11].s64 + 131072;
	// 824059CC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824059D0: 3B7B0010  addi r27, r27, 0x10
	ctx.r[27].s64 = ctx.r[27].s64 + 16;
	// 824059D4: C3CA1850  lfs f30, 0x1850(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 824059D8: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824059DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824059E0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824059E4: 480054CD  bl 0x8240aeb0
	ctx.lr = 0x824059E8;
	sub_8240AEB0(ctx, base);
	// 824059E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824059EC: 48004975  bl 0x8240a360
	ctx.lr = 0x824059F0;
	sub_8240A360(ctx, base);
	// 824059F0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824059F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824059F8: 48004501  bl 0x82409ef8
	ctx.lr = 0x824059FC;
	sub_82409EF8(ctx, base);
	// 824059FC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82405A00: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82405A04: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82405A08: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82405A0C: 480056E5  bl 0x8240b0f0
	ctx.lr = 0x82405A10;
	sub_8240B0F0(ctx, base);
	// 82405A10: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82405A14: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82405A18: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82405A1C: 48003995  bl 0x824093b0
	ctx.lr = 0x82405A20;
	sub_824093B0(ctx, base);
	// 82405A20: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82405A24: 408200B0  bne 0x82405ad4
	if !ctx.cr[0].eq {
	pc = 0x82405AD4; continue 'dispatch;
	}
	// 82405A28: 81610104  lwz r11, 0x104(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82405A2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82405A30: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82405A34: 409A0024  bne cr6, 0x82405a58
	if !ctx.cr[6].eq {
	pc = 0x82405A58; continue 'dispatch;
	}
	// 82405A38: FC40F890  fmr f2, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[31].f64;
	// 82405A3C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82405A40: 48004D99  bl 0x8240a7d8
	ctx.lr = 0x82405A44;
	sub_8240A7D8(ctx, base);
	// 82405A44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82405A48: 480045B1  bl 0x82409ff8
	ctx.lr = 0x82405A4C;
	sub_82409FF8(ctx, base);
	// 82405A4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82405A50: 480044C1  bl 0x82409f10
	ctx.lr = 0x82405A54;
	sub_82409F10(ctx, base);
	// 82405A54: 48000080  b 0x82405ad4
	pc = 0x82405AD4; continue 'dispatch;
	// 82405A58: C0210060  lfs f1, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82405A5C: 4800466D  bl 0x8240a0c8
	ctx.lr = 0x82405A60;
	sub_8240A0C8(ctx, base);
	// 82405A60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82405A64: C0210068  lfs f1, 0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82405A68: 48004709  bl 0x8240a170
	ctx.lr = 0x82405A6C;
	sub_8240A170(ctx, base);
	// 82405A6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82405A70: C0410100  lfs f2, 0x100(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(256 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82405A74: C02100FC  lfs f1, 0xfc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(252 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82405A78: 48004D61  bl 0x8240a7d8
	ctx.lr = 0x82405A7C;
	sub_8240A7D8(ctx, base);
	// 82405A7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82405A80: C0210108  lfs f1, 0x108(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(264 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82405A84: 4BEEB94D  bl 0x822f13d0
	ctx.lr = 0x82405A88;
	sub_822F13D0(ctx, base);
	// 82405A88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82405A8C: C021010C  lfs f1, 0x10c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(268 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82405A90: 4BEEB941  bl 0x822f13d0
	ctx.lr = 0x82405A94;
	sub_822F13D0(ctx, base);
	// 82405A94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82405A98: 38A10110  addi r5, r1, 0x110
	ctx.r[5].s64 = ctx.r[1].s64 + 272;
	// 82405A9C: D3C1025C  stfs f30, 0x25c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(604 as u32), tmp.u32 ) };
	// 82405AA0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82405AA4: D3E10260  stfs f31, 0x260(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(608 as u32), tmp.u32 ) };
	// 82405AA8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82405AAC: D3E10268  stfs f31, 0x268(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(616 as u32), tmp.u32 ) };
	// 82405AB0: D3C1026C  stfs f30, 0x26c(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(620 as u32), tmp.u32 ) };
	// 82405AB4: D3C10270  stfs f30, 0x270(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(624 as u32), tmp.u32 ) };
	// 82405AB8: 91610264  stw r11, 0x264(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(612 as u32), ctx.r[11].u32 ) };
	// 82405ABC: 480058A5  bl 0x8240b360
	ctx.lr = 0x82405AC0;
	sub_8240B360(ctx, base);
	// 82405AC0: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82405AC4: 3881006C  addi r4, r1, 0x6c
	ctx.r[4].s64 = ctx.r[1].s64 + 108;
	// 82405AC8: 80C10148  lwz r6, 0x148(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(328 as u32) ) } as u64;
	// 82405ACC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82405AD0: 48004719  bl 0x8240a1e8
	ctx.lr = 0x82405AD4;
	sub_8240A1E8(ctx, base);
	// 82405AD4: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82405AD8: 2F1D00C0  cmpwi cr6, r29, 0xc0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 192, &mut ctx.xer);
	// 82405ADC: 4198FF00  blt cr6, 0x824059dc
	if ctx.cr[6].lt {
	pc = 0x824059DC; continue 'dispatch;
	}
	// 82405AE0: 7B2B0020  clrldi r11, r25, 0x20
	ctx.r[11].u64 = ctx.r[25].u64 & 0x00000000FFFFFFFFu64;
	// 82405AE4: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82405AE8: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82405AEC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82405AF0: FC200018  frsp f1, f0
	ctx.f[1].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82405AF4: 48000A8D  bl 0x82406580
	ctx.lr = 0x82405AF8;
	sub_82406580(ctx, base);
	// 82405AF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82405AFC: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82405B00: CBC1FFB0  lfd f30, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 82405B04: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82405B08: 4812F5F4  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82405B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82405B10 size=132
    let mut pc: u32 = 0x82405B10;
    'dispatch: loop {
        match pc {
            0x82405B10 => {
    //   block [0x82405B10..0x82405B94)
	// 82405B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82405B14: 4812F5A5  bl 0x825350b8
	ctx.lr = 0x82405B18;
	sub_82535080(ctx, base);
	// 82405B18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82405B1C: 3FE08288  lis r31, -0x7d78
	ctx.r[31].s64 = -2105016320;
	// 82405B20: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82405B24: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82405B28: 817F3828  lwz r11, 0x3828(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14376 as u32) ) } as u64;
	// 82405B2C: 838B1C3C  lwz r28, 0x1c3c(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82405B30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82405B34: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82405B38: 480056A1  bl 0x8240b1d8
	ctx.lr = 0x82405B3C;
	sub_8240B1D8(ctx, base);
	// 82405B3C: 7F03E800  cmpw cr6, r3, r29
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82405B40: 409A0024  bne cr6, 0x82405b64
	if !ctx.cr[6].eq {
	pc = 0x82405B64; continue 'dispatch;
	}
	// 82405B44: 817F3828  lwz r11, 0x3828(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14376 as u32) ) } as u64;
	// 82405B48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82405B4C: 806B1C3C  lwz r3, 0x1c3c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82405B50: 48005501  bl 0x8240b050
	ctx.lr = 0x82405B54;
	sub_8240B050(ctx, base);
	// 82405B54: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82405B58: 4082000C  bne 0x82405b64
	if !ctx.cr[0].eq {
	pc = 0x82405B64; continue 'dispatch;
	}
	// 82405B5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82405B60: 4BFFFA61  bl 0x824055c0
	ctx.lr = 0x82405B64;
	sub_824055C0(ctx, base);
	// 82405B64: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82405B68: 2F1E00C0  cmpwi cr6, r30, 0xc0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 192, &mut ctx.xer);
	// 82405B6C: 4198FFC4  blt cr6, 0x82405b30
	if ctx.cr[6].lt {
	pc = 0x82405B30; continue 'dispatch;
	}
	// 82405B70: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82405B74: 807F3828  lwz r3, 0x3828(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14376 as u32) ) } as u64;
	// 82405B78: 48002BE9  bl 0x82408760
	ctx.lr = 0x82405B7C;
	sub_82408760(ctx, base);
	// 82405B7C: 817F3828  lwz r11, 0x3828(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14376 as u32) ) } as u64;
	// 82405B80: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82405B84: 806B1C38  lwz r3, 0x1c38(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82405B88: 480066B1  bl 0x8240c238
	ctx.lr = 0x82405B8C;
	sub_8240C238(ctx, base);
	// 82405B8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82405B90: 4812F578  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82405B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82405B98 size=168
    let mut pc: u32 = 0x82405B98;
    'dispatch: loop {
        match pc {
            0x82405B98 => {
    //   block [0x82405B98..0x82405C40)
	// 82405B98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82405B9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82405BA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82405BA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82405BA8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82405BAC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82405BB0: 814100E4  lwz r10, 0xe4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(228 as u32) ) } as u64;
	// 82405BB4: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82405BB8: 3FE08288  lis r31, -0x7d78
	ctx.r[31].s64 = -2105016320;
	// 82405BBC: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82405BC0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82405BC4: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82405BC8: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 82405BCC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82405BD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82405BD4: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 82405BD8: 807F3828  lwz r3, 0x3828(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14376 as u32) ) } as u64;
	// 82405BDC: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82405BE0: 48003F61  bl 0x82409b40
	ctx.lr = 0x82405BE4;
	sub_82409B40(ctx, base);
	// 82405BE4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82405BE8: 40820040  bne 0x82405c28
	if !ctx.cr[0].eq {
	pc = 0x82405C28; continue 'dispatch;
	}
	// 82405BEC: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82405BF0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82405BF4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82405BF8: 4099002C  ble cr6, 0x82405c24
	if !ctx.cr[6].gt {
	pc = 0x82405C24; continue 'dispatch;
	}
	// 82405BFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82405C00: 807F3828  lwz r3, 0x3828(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14376 as u32) ) } as u64;
	// 82405C04: 480021F5  bl 0x82407df8
	ctx.lr = 0x82405C08;
	sub_82407DF8(ctx, base);
	// 82405C08: 4BFFF821  bl 0x82405428
	ctx.lr = 0x82405C0C;
	sub_82405428(ctx, base);
	// 82405C0C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82405C10: 40820018  bne 0x82405c28
	if !ctx.cr[0].eq {
	pc = 0x82405C28; continue 'dispatch;
	}
	// 82405C14: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 82405C18: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82405C1C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82405C20: 4198FFDC  blt cr6, 0x82405bfc
	if ctx.cr[6].lt {
	pc = 0x82405BFC; continue 'dispatch;
	}
	// 82405C24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82405C28: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82405C2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82405C30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82405C34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82405C38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82405C3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82405C40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82405C40 size=52
    let mut pc: u32 = 0x82405C40;
    'dispatch: loop {
        match pc {
            0x82405C40 => {
    //   block [0x82405C40..0x82405C74)
	// 82405C40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82405C44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82405C48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82405C4C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82405C50: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82405C54: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82405C58: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82405C5C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82405C60: 4BFFFF39  bl 0x82405b98
	ctx.lr = 0x82405C64;
	sub_82405B98(ctx, base);
	// 82405C64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82405C68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82405C6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82405C70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82405C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82405C78 size=44
    let mut pc: u32 = 0x82405C78;
    'dispatch: loop {
        match pc {
            0x82405C78 => {
    //   block [0x82405C78..0x82405CA4)
	// 82405C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82405C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82405C80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82405C84: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82405C88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82405C8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82405C90: 4BFFFF09  bl 0x82405b98
	ctx.lr = 0x82405C94;
	sub_82405B98(ctx, base);
	// 82405C94: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82405C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82405C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82405CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82405CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82405CA8 size=240
    let mut pc: u32 = 0x82405CA8;
    'dispatch: loop {
        match pc {
            0x82405CA8 => {
    //   block [0x82405CA8..0x82405D98)
	// 82405CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82405CAC: 4812F401  bl 0x825350ac
	ctx.lr = 0x82405CB0;
	sub_82535080(ctx, base);
	// 82405CB0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82405CB4: 3F808288  lis r28, -0x7d78
	ctx.r[28].s64 = -2105016320;
	// 82405CB8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82405CBC: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82405CC0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82405CC4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82405CC8: 807C3828  lwz r3, 0x3828(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(14376 as u32) ) } as u64;
	// 82405CCC: 81631C3C  lwz r11, 0x1c3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82405CD0: 409A0020  bne cr6, 0x82405cf0
	if !ctx.cr[6].eq {
	pc = 0x82405CF0; continue 'dispatch;
	}
	// 82405CD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82405CD8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82405CDC: 386BBF34  addi r3, r11, -0x40cc
	ctx.r[3].s64 = ctx.r[11].s64 + -16588;
	// 82405CE0: 4BEAD2A1  bl 0x822b2f80
	ctx.lr = 0x82405CE4;
	sub_822B2F80(ctx, base);
	// 82405CE4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82405CE8: 60630012  ori r3, r3, 0x12
	ctx.r[3].u64 = ctx.r[3].u64 | 18;
	// 82405CEC: 480000A4  b 0x82405d90
	pc = 0x82405D90; continue 'dispatch;
	// 82405CF0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82405CF4: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 82405CF8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82405CFC: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 82405D00: 409A005C  bne cr6, 0x82405d5c
	if !ctx.cr[6].eq {
	pc = 0x82405D5C; continue 'dispatch;
	}
	// 82405D04: 2B190001  cmplwi cr6, r25, 1
	ctx.cr[6].compare_u32(ctx.r[25].u32, 1 as u32, &mut ctx.xer);
	// 82405D08: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82405D0C: 409A0030  bne cr6, 0x82405d3c
	if !ctx.cr[6].eq {
	pc = 0x82405D3C; continue 'dispatch;
	}
	// 82405D10: 83C31C3C  lwz r30, 0x1c3c(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82405D14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82405D18: 48005901  bl 0x8240b618
	ctx.lr = 0x82405D1C;
	sub_8240B618(ctx, base);
	// 82405D1C: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82405D20: 409A000C  bne cr6, 0x82405d2c
	if !ctx.cr[6].eq {
	pc = 0x82405D2C; continue 'dispatch;
	}
	// 82405D24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82405D28: 4BFFF899  bl 0x824055c0
	ctx.lr = 0x82405D2C;
	sub_824055C0(ctx, base);
	// 82405D2C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82405D30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82405D34: 480052A5  bl 0x8240afd8
	ctx.lr = 0x82405D38;
	sub_8240AFD8(ctx, base);
	// 82405D38: 4800001C  b 0x82405d54
	pc = 0x82405D54; continue 'dispatch;
	// 82405D3C: 80631C3C  lwz r3, 0x1c3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82405D40: 48005311  bl 0x8240b050
	ctx.lr = 0x82405D44;
	sub_8240B050(ctx, base);
	// 82405D44: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82405D48: 4082000C  bne 0x82405d54
	if !ctx.cr[0].eq {
	pc = 0x82405D54; continue 'dispatch;
	}
	// 82405D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82405D50: 4BFFF871  bl 0x824055c0
	ctx.lr = 0x82405D54;
	sub_824055C0(ctx, base);
	// 82405D54: 807C3828  lwz r3, 0x3828(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(14376 as u32) ) } as u64;
	// 82405D58: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82405D5C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82405D60: 3B5A0164  addi r26, r26, 0x164
	ctx.r[26].s64 = ctx.r[26].s64 + 356;
	// 82405D64: 2F1F00C0  cmpwi cr6, r31, 0xc0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 192, &mut ctx.xer);
	// 82405D68: 4198FF90  blt cr6, 0x82405cf8
	if ctx.cr[6].lt {
	pc = 0x82405CF8; continue 'dispatch;
	}
	// 82405D6C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82405D70: 480029E1  bl 0x82408750
	ctx.lr = 0x82405D74;
	sub_82408750(ctx, base);
	// 82405D74: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82405D78: 409A0014  bne cr6, 0x82405d8c
	if !ctx.cr[6].eq {
	pc = 0x82405D8C; continue 'dispatch;
	}
	// 82405D7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82405D80: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82405D84: 386BBEF8  addi r3, r11, -0x4108
	ctx.r[3].s64 = ctx.r[11].s64 + -16648;
	// 82405D88: 4BFFFF58  b 0x82405ce0
	pc = 0x82405CE0; continue 'dispatch;
	// 82405D8C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82405D90: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82405D94: 4812F368  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82405D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82405D98 size=8
    let mut pc: u32 = 0x82405D98;
    'dispatch: loop {
        match pc {
            0x82405D98 => {
    //   block [0x82405D98..0x82405DA0)
	// 82405D98: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82405D9C: 4BFFFF0C  b 0x82405ca8
	sub_82405CA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82405DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82405DA0 size=52
    let mut pc: u32 = 0x82405DA0;
    'dispatch: loop {
        match pc {
            0x82405DA0 => {
    //   block [0x82405DA0..0x82405DD4)
	// 82405DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82405DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82405DA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82405DAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82405DB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82405DB4: 4BFFFA45  bl 0x824057f8
	ctx.lr = 0x82405DB8;
	sub_824057F8(ctx, base);
	// 82405DB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82405DBC: 48001B65  bl 0x82407920
	ctx.lr = 0x82405DC0;
	sub_82407920(ctx, base);
	// 82405DC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82405DC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82405DC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82405DCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82405DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82405DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82405DD8 size=244
    let mut pc: u32 = 0x82405DD8;
    'dispatch: loop {
        match pc {
            0x82405DD8 => {
    //   block [0x82405DD8..0x82405ECC)
	// 82405DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82405DDC: 4812F2D9  bl 0x825350b4
	ctx.lr = 0x82405DE0;
	sub_82535080(ctx, base);
	// 82405DE0: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 82405DE4: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82405DE8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82405DEC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82405DF0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82405DF4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82405DF8: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82405DFC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82405E00: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82405E04: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 82405E08: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82405E0C: 409A001C  bne cr6, 0x82405e28
	if !ctx.cr[6].eq {
	pc = 0x82405E28; continue 'dispatch;
	}
	// 82405E10: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82405E14: 386BBFBC  addi r3, r11, -0x4044
	ctx.r[3].s64 = ctx.r[11].s64 + -16452;
	// 82405E18: 4BEAD169  bl 0x822b2f80
	ctx.lr = 0x82405E1C;
	sub_822B2F80(ctx, base);
	// 82405E1C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82405E20: 60630031  ori r3, r3, 0x31
	ctx.r[3].u64 = ctx.r[3].u64 | 49;
	// 82405E24: 48000098  b 0x82405ebc
	pc = 0x82405EBC; continue 'dispatch;
	// 82405E28: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82405E2C: 409A001C  bne cr6, 0x82405e48
	if !ctx.cr[6].eq {
	pc = 0x82405E48; continue 'dispatch;
	}
	// 82405E30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82405E34: 386BBF94  addi r3, r11, -0x406c
	ctx.r[3].s64 = ctx.r[11].s64 + -16492;
	// 82405E38: 4BEAD149  bl 0x822b2f80
	ctx.lr = 0x82405E3C;
	sub_822B2F80(ctx, base);
	// 82405E3C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82405E40: 60630032  ori r3, r3, 0x32
	ctx.r[3].u64 = ctx.r[3].u64 | 50;
	// 82405E44: 48000078  b 0x82405ebc
	pc = 0x82405EBC; continue 'dispatch;
	// 82405E48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82405E4C: 409A001C  bne cr6, 0x82405e68
	if !ctx.cr[6].eq {
	pc = 0x82405E68; continue 'dispatch;
	}
	// 82405E50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82405E54: 386BBF6C  addi r3, r11, -0x4094
	ctx.r[3].s64 = ctx.r[11].s64 + -16532;
	// 82405E58: 4BEAD129  bl 0x822b2f80
	ctx.lr = 0x82405E5C;
	sub_822B2F80(ctx, base);
	// 82405E5C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82405E60: 60630033  ori r3, r3, 0x33
	ctx.r[3].u64 = ctx.r[3].u64 | 51;
	// 82405E64: 48000058  b 0x82405ebc
	pc = 0x82405EBC; continue 'dispatch;
	// 82405E68: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82405E6C: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 82405E70: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82405E74: 4BFBAD8D  bl 0x823c0c00
	ctx.lr = 0x82405E78;
	sub_823C0C00(ctx, base);
	// 82405E78: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82405E7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82405E80: 3861006C  addi r3, r1, 0x6c
	ctx.r[3].s64 = ctx.r[1].s64 + 108;
	// 82405E84: 4BFBAD7D  bl 0x823c0c00
	ctx.lr = 0x82405E88;
	sub_823C0C00(ctx, base);
	// 82405E88: 81610114  lwz r11, 0x114(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(276 as u32) ) } as u64;
	// 82405E8C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82405E90: D3C10078  stfs f30, 0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 82405E94: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 82405E98: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82405E9C: 7F68DB78  mr r8, r27
	ctx.r[8].u64 = ctx.r[27].u64;
	// 82405EA0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82405EA4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82405EA8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82405EAC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82405EB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82405EB4: C04B1FF8  lfs f2, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82405EB8: 4BFFFCE1  bl 0x82405b98
	ctx.lr = 0x82405EBC;
	sub_82405B98(ctx, base);
	// 82405EBC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82405EC0: CBC1FFC0  lfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82405EC4: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82405EC8: 4812F23C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82405ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82405ED0 size=280
    let mut pc: u32 = 0x82405ED0;
    'dispatch: loop {
        match pc {
            0x82405ED0 => {
    //   block [0x82405ED0..0x82405FE8)
	// 82405ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82405ED4: 4812F1E5  bl 0x825350b8
	ctx.lr = 0x82405ED8;
	sub_82535080(ctx, base);
	// 82405ED8: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 82405EDC: 4813010D  bl 0x82535fe8
	ctx.lr = 0x82405EE0;
	sub_82535FB0(ctx, base);
	// 82405EE0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82405EE4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82405EE8: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82405EEC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82405EF0: FFA01090  fmr f29, f2
	ctx.f[29].f64 = ctx.f[2].f64;
	// 82405EF4: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82405EF8: FF801890  fmr f28, f3
	ctx.f[28].f64 = ctx.f[3].f64;
	// 82405EFC: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 82405F00: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82405F04: 409A001C  bne cr6, 0x82405f20
	if !ctx.cr[6].eq {
	pc = 0x82405F20; continue 'dispatch;
	}
	// 82405F08: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82405F0C: 386BC090  addi r3, r11, -0x3f70
	ctx.r[3].s64 = ctx.r[11].s64 + -16240;
	// 82405F10: 4BEAD071  bl 0x822b2f80
	ctx.lr = 0x82405F14;
	sub_822B2F80(ctx, base);
	// 82405F14: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82405F18: 60630031  ori r3, r3, 0x31
	ctx.r[3].u64 = ctx.r[3].u64 | 49;
	// 82405F1C: 480000BC  b 0x82405fd8
	pc = 0x82405FD8; continue 'dispatch;
	// 82405F20: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82405F24: 409A001C  bne cr6, 0x82405f40
	if !ctx.cr[6].eq {
	pc = 0x82405F40; continue 'dispatch;
	}
	// 82405F28: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82405F2C: 386BC058  addi r3, r11, -0x3fa8
	ctx.r[3].s64 = ctx.r[11].s64 + -16296;
	// 82405F30: 4BEAD051  bl 0x822b2f80
	ctx.lr = 0x82405F34;
	sub_822B2F80(ctx, base);
	// 82405F34: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82405F38: 60630032  ori r3, r3, 0x32
	ctx.r[3].u64 = ctx.r[3].u64 | 50;
	// 82405F3C: 4800009C  b 0x82405fd8
	pc = 0x82405FD8; continue 'dispatch;
	// 82405F40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82405F44: 409A001C  bne cr6, 0x82405f60
	if !ctx.cr[6].eq {
	pc = 0x82405F60; continue 'dispatch;
	}
	// 82405F48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82405F4C: 386BC01C  addi r3, r11, -0x3fe4
	ctx.r[3].s64 = ctx.r[11].s64 + -16356;
	// 82405F50: 4BEAD031  bl 0x822b2f80
	ctx.lr = 0x82405F54;
	sub_822B2F80(ctx, base);
	// 82405F54: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82405F58: 60630033  ori r3, r3, 0x33
	ctx.r[3].u64 = ctx.r[3].u64 | 51;
	// 82405F5C: 4800007C  b 0x82405fd8
	pc = 0x82405FD8; continue 'dispatch;
	// 82405F60: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82405F64: C3CB1FF8  lfs f30, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82405F68: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 82405F6C: 40980028  bge cr6, 0x82405f94
	if !ctx.cr[6].lt {
	pc = 0x82405F94; continue 'dispatch;
	}
	// 82405F70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82405F74: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82405F78: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 82405F7C: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 82405F80: 386BBFE4  addi r3, r11, -0x401c
	ctx.r[3].s64 = ctx.r[11].s64 + -16412;
	// 82405F84: 4BEACFFD  bl 0x822b2f80
	ctx.lr = 0x82405F88;
	sub_822B2F80(ctx, base);
	// 82405F88: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82405F8C: 60630018  ori r3, r3, 0x18
	ctx.r[3].u64 = ctx.r[3].u64 | 24;
	// 82405F90: 48000048  b 0x82405fd8
	pc = 0x82405FD8; continue 'dispatch;
	// 82405F94: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82405F98: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 82405F9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82405FA0: 4BFBAC61  bl 0x823c0c00
	ctx.lr = 0x82405FA4;
	sub_823C0C00(ctx, base);
	// 82405FA4: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 82405FA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82405FAC: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82405FB0: 4BFBAC51  bl 0x823c0c00
	ctx.lr = 0x82405FB4;
	sub_823C0C00(ctx, base);
	// 82405FB4: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82405FB8: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82405FBC: D3810068  stfs f28, 0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 82405FC0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82405FC4: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 82405FC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82405FCC: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82405FD0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82405FD4: 4BFFF79D  bl 0x82405770
	ctx.lr = 0x82405FD8;
	sub_82405770(ctx, base);
	// 82405FD8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82405FDC: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 82405FE0: 48130055  bl 0x82536034
	ctx.lr = 0x82405FE4;
	sub_82535FFC(ctx, base);
	// 82405FE4: 4812F124  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82405FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82405FE8 size=4
    let mut pc: u32 = 0x82405FE8;
    'dispatch: loop {
        match pc {
            0x82405FE8 => {
    //   block [0x82405FE8..0x82405FEC)
	// 82405FE8: 4BFFFDB0  b 0x82405d98
	sub_82405D98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82405FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82405FF0 size=160
    let mut pc: u32 = 0x82405FF0;
    'dispatch: loop {
        match pc {
            0x82405FF0 => {
    //   block [0x82405FF0..0x82406090)
	// 82405FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82405FF4: 4812F0BD  bl 0x825350b0
	ctx.lr = 0x82405FF8;
	sub_82535080(ctx, base);
	// 82405FF8: DBC1FFB8  stfd f30, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[30].u64 ) };
	// 82405FFC: DBE1FFC0  stfd f31, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82406000: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406004: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82406008: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240600C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82406010: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82406014: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82406018: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8240601C: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 82406020: 7D254B78  mr r5, r9
	ctx.r[5].u64 = ctx.r[9].u64;
	// 82406024: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82406028: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240602C: 7D5A5378  mr r26, r10
	ctx.r[26].u64 = ctx.r[10].u64;
	// 82406030: 480004E1  bl 0x82406510
	ctx.lr = 0x82406034;
	sub_82406510(ctx, base);
	// 82406034: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82406038: 4182001C  beq 0x82406054
	if ctx.cr[0].eq {
	pc = 0x82406054; continue 'dispatch;
	}
	// 8240603C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406040: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82406044: 386BC0CC  addi r3, r11, -0x3f34
	ctx.r[3].s64 = ctx.r[11].s64 + -16180;
	// 82406048: 4BEACF39  bl 0x822b2f80
	ctx.lr = 0x8240604C;
	sub_822B2F80(ctx, base);
	// 8240604C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82406050: 48000030  b 0x82406080
	pc = 0x82406080; continue 'dispatch;
	// 82406054: 8161011C  lwz r11, 0x11c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(284 as u32) ) } as u64;
	// 82406058: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 8240605C: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82406060: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82406064: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82406068: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240606C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82406070: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82406074: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82406078: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8240607C: 4BFFFD5D  bl 0x82405dd8
	ctx.lr = 0x82406080;
	sub_82405DD8(ctx, base);
	// 82406080: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82406084: CBC1FFB8  lfd f30, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82406088: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 8240608C: 4812F074  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82406090 size=68
    let mut pc: u32 = 0x82406090;
    'dispatch: loop {
        match pc {
            0x82406090 => {
    //   block [0x82406090..0x824060D4)
	// 82406090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406098: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240609C: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 824060A0: FC400890  fmr f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[1].f64;
	// 824060A4: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 824060A8: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 824060AC: 7CA72B78  mr r7, r5
	ctx.r[7].u64 = ctx.r[5].u64;
	// 824060B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824060B4: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824060B8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824060BC: C02B1850  lfs f1, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824060C0: 4BFFFD19  bl 0x82405dd8
	ctx.lr = 0x824060C4;
	sub_82405DD8(ctx, base);
	// 824060C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824060C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824060CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824060D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824060D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824060D8 size=124
    let mut pc: u32 = 0x824060D8;
    'dispatch: loop {
        match pc {
            0x824060D8 => {
    //   block [0x824060D8..0x82406154)
	// 824060D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824060DC: 4812EFDD  bl 0x825350b8
	ctx.lr = 0x824060E0;
	sub_82535080(ctx, base);
	// 824060E0: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 824060E4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824060E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824060EC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824060F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824060F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824060F8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824060FC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82406100: 4BFFF6B1  bl 0x824057b0
	ctx.lr = 0x82406104;
	sub_824057B0(ctx, base);
	// 82406104: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82406108: 40820040  bne 0x82406148
	if !ctx.cr[0].eq {
	pc = 0x82406148; continue 'dispatch;
	}
	// 8240610C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82406110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82406114: 4BFFF6B5  bl 0x824057c8
	ctx.lr = 0x82406118;
	sub_824057C8(ctx, base);
	// 82406118: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240611C: 4082002C  bne 0x82406148
	if !ctx.cr[0].eq {
	pc = 0x82406148; continue 'dispatch;
	}
	// 82406120: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82406124: C0210050  lfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82406128: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 8240612C: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82406130: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 82406134: FC60F890  fmr f3, f31
	ctx.f[3].f64 = ctx.f[31].f64;
	// 82406138: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8240613C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82406140: C04B1FF8  lfs f2, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82406144: 4BFFFD8D  bl 0x82405ed0
	ctx.lr = 0x82406148;
	sub_82405ED0(ctx, base);
	// 82406148: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8240614C: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82406150: 4812EFB8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406158 size=48
    let mut pc: u32 = 0x82406158;
    'dispatch: loop {
        match pc {
            0x82406158 => {
    //   block [0x82406158..0x82406188)
	// 82406158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240615C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406164: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82406168: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8240616C: 386B3838  addi r3, r11, 0x3838
	ctx.r[3].s64 = ctx.r[11].s64 + 14392;
	// 82406170: 48006D19  bl 0x8240ce88
	ctx.lr = 0x82406174;
	sub_8240CE88(ctx, base);
	// 82406174: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240617C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82406188 size=220
    let mut pc: u32 = 0x82406188;
    'dispatch: loop {
        match pc {
            0x82406188 => {
    //   block [0x82406188..0x82406264)
	// 82406188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240618C: 4812EF29  bl 0x825350b4
	ctx.lr = 0x82406190;
	sub_82535080(ctx, base);
	// 82406190: DBA1FFB8  stfd f29, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[29].u64 ) };
	// 82406194: DBC1FFC0  stfd f30, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 82406198: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 8240619C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824061A0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824061A4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824061A8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824061AC: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 824061B0: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 824061B4: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 824061B8: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 824061BC: 3BE00006  li r31, 6
	ctx.r[31].s64 = 6;
	// 824061C0: C00B2F80  lfs f0, 0x2f80(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12160 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824061C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824061C8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824061CC: C00B76F4  lfs f0, 0x76f4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30452 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824061D0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824061D4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 824061D8: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824061DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824061E0: D0010058  stfs f0, 0x58(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 824061E4: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 824061E8: C1ABC10C  lfs f13, -0x3ef4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16116 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824061EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824061F0: D1A10060  stfs f13, 0x60(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 824061F4: C1AB236C  lfs f13, 0x236c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9068 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824061F8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 824061FC: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82406200: 3B6B3838  addi r27, r11, 0x3838
	ctx.r[27].s64 = ctx.r[11].s64 + 14392;
	// 82406204: C01D0000  lfs f0, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82406208: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8240620C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82406210: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 82406214: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82406218: EC20F82A  fadds f1, f0, f31
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 8240621C: 48006C6D  bl 0x8240ce88
	ctx.lr = 0x82406220;
	sub_8240CE88(ctx, base);
	// 82406220: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82406224: 3BDE0018  addi r30, r30, 0x18
	ctx.r[30].s64 = ctx.r[30].s64 + 24;
	// 82406228: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 8240622C: 4082FFD8  bne 0x82406204
	if !ctx.cr[0].eq {
	pc = 0x82406204; continue 'dispatch;
	}
	// 82406230: 397C0048  addi r11, r28, 0x48
	ctx.r[11].s64 = ctx.r[28].s64 + 72;
	// 82406234: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82406238: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 8240623C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82406240: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82406244: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82406248: 4200FFF8  bdnz 0x82406240
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82406240; continue 'dispatch;
	}
	// 8240624C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406250: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82406254: CBA1FFB8  lfd f29, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82406258: CBC1FFC0  lfd f30, -0x40(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 8240625C: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82406260: 4812EEA4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82406268 size=188
    let mut pc: u32 = 0x82406268;
    'dispatch: loop {
        match pc {
            0x82406268 => {
    //   block [0x82406268..0x82406324)
	// 82406268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240626C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82406274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82406278: DBA1FFD0  stfd f29, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[29].u64 ) };
	// 8240627C: DBC1FFD8  stfd f30, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82406280: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82406284: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406288: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240628C: ED81102A  fadds f12, f1, f2
	ctx.f[12].f64 = ((ctx.f[1].f64 + ctx.f[2].f64) as f32) as f64;
	// 82406290: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 82406294: FFE01890  fmr f31, f3
	ctx.f[31].f64 = ctx.f[3].f64;
	// 82406298: FFC02090  fmr f30, f4
	ctx.f[30].f64 = ctx.f[4].f64;
	// 8240629C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824062A0: 39400024  li r10, 0x24
	ctx.r[10].s64 = 36;
	// 824062A4: C00B76F4  lfs f0, 0x76f4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30452 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824062A8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 824062AC: EDA10028  fsubs f13, f1, f0
	ctx.f[13].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 824062B0: EFAC002A  fadds f29, f12, f0
	ctx.f[29].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 824062B4: EC2D1028  fsubs f1, f13, f2
	ctx.f[1].f64 = (((ctx.f[13].f64 - ctx.f[2].f64) as f32) as f64);
	// 824062B8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 824062BC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824062C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824062C4: 4200FFF8  bdnz 0x824062bc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824062BC; continue 'dispatch;
	}
	// 824062C8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 824062CC: FC60F090  fmr f3, f30
	ctx.f[3].f64 = ctx.f[30].f64;
	// 824062D0: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 824062D4: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 824062D8: 3BCB3838  addi r30, r11, 0x3838
	ctx.r[30].s64 = ctx.r[11].s64 + 14392;
	// 824062DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824062E0: 48006BA9  bl 0x8240ce88
	ctx.lr = 0x824062E4;
	sub_8240CE88(ctx, base);
	// 824062E4: 38FF0018  addi r7, r31, 0x18
	ctx.r[7].s64 = ctx.r[31].s64 + 24;
	// 824062E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824062EC: FC60F090  fmr f3, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[30].f64;
	// 824062F0: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 824062F4: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 824062F8: 48006B91  bl 0x8240ce88
	ctx.lr = 0x824062FC;
	sub_8240CE88(ctx, base);
	// 824062FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406300: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82406304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240630C: CBA1FFD0  lfd f29, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82406310: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82406314: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82406318: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240631C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82406320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82406328 size=248
    let mut pc: u32 = 0x82406328;
    'dispatch: loop {
        match pc {
            0x82406328 => {
    //   block [0x82406328..0x82406420)
	// 82406328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240632C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406330: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82406334: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82406338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240633C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82406340: FC001890  fmr f0, f3
	ctx.f[0].f64 = ctx.f[3].f64;
	// 82406344: FFE02890  fmr f31, f5
	ctx.f[31].f64 = ctx.f[5].f64;
	// 82406348: 7D3F4B78  mr r31, r9
	ctx.r[31].u64 = ctx.r[9].u64;
	// 8240634C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82406350: 41990018  bgt cr6, 0x82406368
	if ctx.cr[6].gt {
	pc = 0x82406368; continue 'dispatch;
	}
	// 82406354: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406358: 386BC110  addi r3, r11, -0x3ef0
	ctx.r[3].s64 = ctx.r[11].s64 + -16112;
	// 8240635C: 4BEACC25  bl 0x822b2f80
	ctx.lr = 0x82406360;
	sub_822B2F80(ctx, base);
	// 82406360: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82406364: 480000A4  b 0x82406408
	pc = 0x82406408; continue 'dispatch;
	// 82406368: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 8240636C: 409A0058  bne cr6, 0x824063c4
	if !ctx.cr[6].eq {
	pc = 0x824063C4; continue 'dispatch;
	}
	// 82406370: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82406374: FC602090  fmr f3, f4
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[4].f64;
	// 82406378: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8240637C: FC400090  fmr f2, f0
	ctx.f[2].f64 = ctx.f[0].f64;
	// 82406380: 386B3838  addi r3, r11, 0x3838
	ctx.r[3].s64 = ctx.r[11].s64 + 14392;
	// 82406384: 48006B05  bl 0x8240ce88
	ctx.lr = 0x82406388;
	sub_8240CE88(ctx, base);
	// 82406388: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8240638C: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82406390: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82406394: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 82406398: 39000006  li r8, 6
	ctx.r[8].s64 = 6;
	// 8240639C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 824063A0: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824063A4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 824063A8: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 824063AC: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 824063B0: 4200FFF0  bdnz 0x824063a0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824063A0; continue 'dispatch;
	}
	// 824063B4: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 824063B8: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 824063BC: 4082FFD4  bne 0x82406390
	if !ctx.cr[0].eq {
	pc = 0x82406390; continue 'dispatch;
	}
	// 824063C0: 4800002C  b 0x824063ec
	pc = 0x824063EC; continue 'dispatch;
	// 824063C4: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 824063C8: 409A0014  bne cr6, 0x824063dc
	if !ctx.cr[6].eq {
	pc = 0x824063DC; continue 'dispatch;
	}
	// 824063CC: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 824063D0: FC600090  fmr f3, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[0].f64;
	// 824063D4: 4BFFFE95  bl 0x82406268
	ctx.lr = 0x824063D8;
	sub_82406268(ctx, base);
	// 824063D8: 48000014  b 0x824063ec
	pc = 0x824063EC; continue 'dispatch;
	// 824063DC: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 824063E0: FC602090  fmr f3, f4
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[3].f64 = ctx.f[4].f64;
	// 824063E4: FC400090  fmr f2, f0
	ctx.f[2].f64 = ctx.f[0].f64;
	// 824063E8: 4BFFFDA1  bl 0x82406188
	ctx.lr = 0x824063EC;
	sub_82406188(ctx, base);
	// 824063EC: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 824063F0: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 824063F4: D3EA0000  stfs f31, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824063F8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824063FC: 394A0018  addi r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 + 24;
	// 82406400: 4082FFF4  bne 0x824063f4
	if !ctx.cr[0].eq {
	pc = 0x824063F4; continue 'dispatch;
	}
	// 82406404: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240640C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406414: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82406418: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240641C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406420 size=164
    let mut pc: u32 = 0x82406420;
    'dispatch: loop {
        match pc {
            0x82406420 => {
    //   block [0x82406420..0x824064C4)
	// 82406420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406428: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240642C: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82406430: FC001090  fmr f0, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[2].f64;
	// 82406434: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82406438: 806B3830  lwz r3, 0x3830(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14384 as u32) ) } as u64;
	// 8240643C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82406440: 409A0010  bne cr6, 0x82406450
	if !ctx.cr[6].eq {
	pc = 0x82406450; continue 'dispatch;
	}
	// 82406444: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82406448: 6063002E  ori r3, r3, 0x2e
	ctx.r[3].u64 = ctx.r[3].u64 | 46;
	// 8240644C: 48000068  b 0x824064b4
	pc = 0x824064B4; continue 'dispatch;
	// 82406450: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 82406454: 419A0050  beq cr6, 0x824064a4
	if ctx.cr[6].eq {
	pc = 0x824064A4; continue 'dispatch;
	}
	// 82406458: 2F090002  cmpwi cr6, r9, 2
	ctx.cr[6].compare_i32(ctx.r[9].s32, 2, &mut ctx.xer);
	// 8240645C: 419A003C  beq cr6, 0x82406498
	if ctx.cr[6].eq {
	pc = 0x82406498; continue 'dispatch;
	}
	// 82406460: 2F090004  cmpwi cr6, r9, 4
	ctx.cr[6].compare_i32(ctx.r[9].s32, 4, &mut ctx.xer);
	// 82406464: 419A0020  beq cr6, 0x82406484
	if ctx.cr[6].eq {
	pc = 0x82406484; continue 'dispatch;
	}
	// 82406468: 2F090006  cmpwi cr6, r9, 6
	ctx.cr[6].compare_i32(ctx.r[9].s32, 6, &mut ctx.xer);
	// 8240646C: 419A0018  beq cr6, 0x82406484
	if ctx.cr[6].eq {
	pc = 0x82406484; continue 'dispatch;
	}
	// 82406470: 2F090008  cmpwi cr6, r9, 8
	ctx.cr[6].compare_i32(ctx.r[9].s32, 8, &mut ctx.xer);
	// 82406474: 419A0010  beq cr6, 0x82406484
	if ctx.cr[6].eq {
	pc = 0x82406484; continue 'dispatch;
	}
	// 82406478: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240647C: 6063001A  ori r3, r3, 0x1a
	ctx.r[3].u64 = ctx.r[3].u64 | 26;
	// 82406480: 48000034  b 0x824064b4
	pc = 0x824064B4; continue 'dispatch;
	// 82406484: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 82406488: FC401890  fmr f2, f3
	ctx.f[2].f64 = ctx.f[3].f64;
	// 8240648C: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 82406490: 48008009  bl 0x8240e498
	ctx.lr = 0x82406494;
	sub_8240E498(ctx, base);
	// 82406494: 48000020  b 0x824064b4
	pc = 0x824064B4; continue 'dispatch;
	// 82406498: FC400090  fmr f2, f0
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[0].f64;
	// 8240649C: 48007FCD  bl 0x8240e468
	ctx.lr = 0x824064A0;
	sub_8240E468(ctx, base);
	// 824064A0: 48000014  b 0x824064b4
	pc = 0x824064B4; continue 'dispatch;
	// 824064A4: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 824064A8: FC401890  fmr f2, f3
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[3].f64;
	// 824064AC: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 824064B0: 48007F81  bl 0x8240e430
	ctx.lr = 0x824064B4;
	sub_8240E430(ctx, base);
	// 824064B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824064B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824064BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824064C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824064C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824064C8 size=32
    let mut pc: u32 = 0x824064C8;
    'dispatch: loop {
        match pc {
            0x824064C8 => {
    //   block [0x824064C8..0x824064E8)
	// 824064C8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 824064CC: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 824064D0: 816B3830  lwz r11, 0x3830(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14384 as u32) ) } as u64;
	// 824064D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824064D8: 409A0010  bne cr6, 0x824064e8
	if !ctx.cr[6].eq {
		sub_824064E8(ctx, base);
		return;
	}
	// 824064DC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 824064E0: 6063002E  ori r3, r3, 0x2e
	ctx.r[3].u64 = ctx.r[3].u64 | 46;
	// 824064E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824064E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824064E8 size=20
    let mut pc: u32 = 0x824064E8;
    'dispatch: loop {
        match pc {
            0x824064E8 => {
    //   block [0x824064E8..0x824064FC)
	// 824064E8: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 824064EC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824064F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 824064F4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824064F8: 480073F0  b 0x8240d8e8
	sub_8240D8E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82406500 size=16
    let mut pc: u32 = 0x82406500;
    'dispatch: loop {
        match pc {
            0x82406500 => {
    //   block [0x82406500..0x82406510)
	// 82406500: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82406504: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82406508: 386B3838  addi r3, r11, 0x3838
	ctx.r[3].s64 = ctx.r[11].s64 + 14392;
	// 8240650C: 480063F4  b 0x8240c900
	sub_8240C900(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82406510 size=32
    let mut pc: u32 = 0x82406510;
    'dispatch: loop {
        match pc {
            0x82406510 => {
    //   block [0x82406510..0x82406530)
	// 82406510: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82406514: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82406518: 816B3830  lwz r11, 0x3830(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14384 as u32) ) } as u64;
	// 8240651C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82406520: 409A0010  bne cr6, 0x82406530
	if !ctx.cr[6].eq {
		sub_82406530(ctx, base);
		return;
	}
	// 82406524: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82406528: 6063002E  ori r3, r3, 0x2e
	ctx.r[3].u64 = ctx.r[3].u64 | 46;
	// 8240652C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82406530 size=20
    let mut pc: u32 = 0x82406530;
    'dispatch: loop {
        match pc {
            0x82406530 => {
    //   block [0x82406530..0x82406544)
	// 82406530: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82406534: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82406538: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8240653C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82406540: 48007468  b 0x8240d9a8
	sub_8240D9A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82406548 size=16
    let mut pc: u32 = 0x82406548;
    'dispatch: loop {
        match pc {
            0x82406548 => {
    //   block [0x82406548..0x82406558)
	// 82406548: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8240654C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82406550: 386B3838  addi r3, r11, 0x3838
	ctx.r[3].s64 = ctx.r[11].s64 + 14392;
	// 82406554: 480068F4  b 0x8240ce48
	sub_8240CE48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82406558 size=40
    let mut pc: u32 = 0x82406558;
    'dispatch: loop {
        match pc {
            0x82406558 => {
    //   block [0x82406558..0x82406580)
	// 82406558: 7D094378  mr r9, r8
	ctx.r[9].u64 = ctx.r[8].u64;
	// 8240655C: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82406560: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82406564: 3D408288  lis r10, -0x7d78
	ctx.r[10].s64 = -2105016320;
	// 82406568: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8240656C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82406570: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82406574: 386A3838  addi r3, r10, 0x3838
	ctx.r[3].s64 = ctx.r[10].s64 + 14392;
	// 82406578: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8240657C: 4800646C  b 0x8240c9e8
	sub_8240C9E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82406580 size=28
    let mut pc: u32 = 0x82406580;
    'dispatch: loop {
        match pc {
            0x82406580 => {
    //   block [0x82406580..0x8240659C)
	// 82406580: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82406584: 806B3830  lwz r3, 0x3830(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14384 as u32) ) } as u64;
	// 82406588: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8240658C: 409A0010  bne cr6, 0x8240659c
	if !ctx.cr[6].eq {
		sub_8240659C(ctx, base);
		return;
	}
	// 82406590: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82406594: 6063002E  ori r3, r3, 0x2e
	ctx.r[3].u64 = ctx.r[3].u64 | 46;
	// 82406598: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240659C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240659C size=4
    let mut pc: u32 = 0x8240659C;
    'dispatch: loop {
        match pc {
            0x8240659C => {
    //   block [0x8240659C..0x824065A0)
	// 8240659C: 48007544  b 0x8240dae0
	sub_8240DAE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824065A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824065A0 size=220
    let mut pc: u32 = 0x824065A0;
    'dispatch: loop {
        match pc {
            0x824065A0 => {
    //   block [0x824065A0..0x8240667C)
	// 824065A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824065A4: 4812EB19  bl 0x825350bc
	ctx.lr = 0x824065A8;
	sub_82535080(ctx, base);
	// 824065A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824065AC: 3FA08288  lis r29, -0x7d78
	ctx.r[29].s64 = -2105016320;
	// 824065B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824065B4: 807D3830  lwz r3, 0x3830(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(14384 as u32) ) } as u64;
	// 824065B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824065BC: 409A0010  bne cr6, 0x824065cc
	if !ctx.cr[6].eq {
	pc = 0x824065CC; continue 'dispatch;
	}
	// 824065C0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 824065C4: 6063002E  ori r3, r3, 0x2e
	ctx.r[3].u64 = ctx.r[3].u64 | 46;
	// 824065C8: 480000AC  b 0x82406674
	pc = 0x82406674; continue 'dispatch;
	// 824065CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824065D0: 409A0010  bne cr6, 0x824065e0
	if !ctx.cr[6].eq {
	pc = 0x824065E0; continue 'dispatch;
	}
	// 824065D4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 824065D8: 60630037  ori r3, r3, 0x37
	ctx.r[3].u64 = ctx.r[3].u64 | 55;
	// 824065DC: 48000098  b 0x82406674
	pc = 0x82406674; continue 'dispatch;
	// 824065E0: 889F0005  lbz r4, 5(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(5 as u32) ) } as u64;
	// 824065E4: 2B040002  cmplwi cr6, r4, 2
	ctx.cr[6].compare_u32(ctx.r[4].u32, 2 as u32, &mut ctx.xer);
	// 824065E8: 419A0020  beq cr6, 0x82406608
	if ctx.cr[6].eq {
	pc = 0x82406608; continue 'dispatch;
	}
	// 824065EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824065F0: 88BF0006  lbz r5, 6(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(6 as u32) ) } as u64;
	// 824065F4: 386BC150  addi r3, r11, -0x3eb0
	ctx.r[3].s64 = ctx.r[11].s64 + -16048;
	// 824065F8: 4BEAC989  bl 0x822b2f80
	ctx.lr = 0x824065FC;
	sub_822B2F80(ctx, base);
	// 824065FC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82406600: 60630036  ori r3, r3, 0x36
	ctx.r[3].u64 = ctx.r[3].u64 | 54;
	// 82406604: 48000070  b 0x82406674
	pc = 0x82406674; continue 'dispatch;
	// 82406608: C03F0010  lfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240660C: 4800715D  bl 0x8240d768
	ctx.lr = 0x82406610;
	sub_8240D768(ctx, base);
	// 82406610: 807D3830  lwz r3, 0x3830(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(14384 as u32) ) } as u64;
	// 82406614: C03F0014  lfs f1, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82406618: 480071A9  bl 0x8240d7c0
	ctx.lr = 0x8240661C;
	sub_8240D7C0(ctx, base);
	// 8240661C: 807D3830  lwz r3, 0x3830(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(14384 as u32) ) } as u64;
	// 82406620: C03F0018  lfs f1, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82406624: 480071F5  bl 0x8240d818
	ctx.lr = 0x82406628;
	sub_8240D818(ctx, base);
	// 82406628: 807D3830  lwz r3, 0x3830(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(14384 as u32) ) } as u64;
	// 8240662C: C03F001C  lfs f1, 0x1c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82406630: 48007251  bl 0x8240d880
	ctx.lr = 0x82406634;
	sub_8240D880(ctx, base);
	// 82406634: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82406638: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8240663C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82406640: 40990030  ble cr6, 0x82406670
	if !ctx.cr[6].gt {
	pc = 0x82406670; continue 'dispatch;
	}
	// 82406644: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82406648: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240664C: 807D3830  lwz r3, 0x3830(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(14384 as u32) ) } as u64;
	// 82406650: 7D7E59D6  mullw r11, r30, r11
	ctx.r[11].s64 = (ctx.r[30].s32 as i64) * (ctx.r[11].s32 as i64);
	// 82406654: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82406658: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 8240665C: 480074AD  bl 0x8240db08
	ctx.lr = 0x82406660;
	sub_8240DB08(ctx, base);
	// 82406660: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82406664: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82406668: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240666C: 4198FFD8  blt cr6, 0x82406644
	if ctx.cr[6].lt {
	pc = 0x82406644; continue 'dispatch;
	}
	// 82406670: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406674: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82406678: 4812EA94  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406680 size=132
    let mut pc: u32 = 0x82406680;
    'dispatch: loop {
        match pc {
            0x82406680 => {
    //   block [0x82406680..0x82406704)
	// 82406680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406688: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240668C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82406690: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406694: 3FE08288  lis r31, -0x7d78
	ctx.r[31].s64 = -2105016320;
	// 82406698: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240669C: 817F3830  lwz r11, 0x3830(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(14384 as u32) ) } as u64;
	// 824066A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824066A4: 409A0040  bne cr6, 0x824066e4
	if !ctx.cr[6].eq {
	pc = 0x824066E4; continue 'dispatch;
	}
	// 824066A8: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 824066AC: 38602618  li r3, 0x2618
	ctx.r[3].s64 = 9752;
	// 824066B0: 816B3800  lwz r11, 0x3800(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(14336 as u32) ) } as u64;
	// 824066B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824066B8: 4E800421  bctrl
	ctx.lr = 0x824066BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824066BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824066C0: 41820010  beq 0x824066d0
	if ctx.cr[0].eq {
	pc = 0x824066D0; continue 'dispatch;
	}
	// 824066C4: 48006DAD  bl 0x8240d470
	ctx.lr = 0x824066C8;
	sub_8240D470(ctx, base);
	// 824066C8: 907F3830  stw r3, 0x3830(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14384 as u32), ctx.r[3].u32 ) };
	// 824066CC: 4800000C  b 0x824066d8
	pc = 0x824066D8; continue 'dispatch;
	// 824066D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824066D4: 917F3830  stw r11, 0x3830(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(14384 as u32), ctx.r[11].u32 ) };
	// 824066D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824066DC: 4BFFFEC5  bl 0x824065a0
	ctx.lr = 0x824066E0;
	sub_824065A0(ctx, base);
	// 824066E0: 4800000C  b 0x824066ec
	pc = 0x824066EC; continue 'dispatch;
	// 824066E4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 824066E8: 60630030  ori r3, r3, 0x30
	ctx.r[3].u64 = ctx.r[3].u64 | 48;
	// 824066EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824066F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824066F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824066F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824066FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82406700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406708 size=112
    let mut pc: u32 = 0x82406708;
    'dispatch: loop {
        match pc {
            0x82406708 => {
    //   block [0x82406708..0x82406778)
	// 82406708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240670C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406710: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82406714: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82406718: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240671C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82406720: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82406724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82406728: 4BFFF0B9  bl 0x824057e0
	ctx.lr = 0x8240672C;
	sub_824057E0(ctx, base);
	// 8240672C: 7F03F040  cmplw cr6, r3, r30
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82406730: 419A0040  beq cr6, 0x82406770
	if ctx.cr[6].eq {
	pc = 0x82406770; continue 'dispatch;
	}
	// 82406734: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82406738: 2F1F00C0  cmpwi cr6, r31, 0xc0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 192, &mut ctx.xer);
	// 8240673C: 4198FFE8  blt cr6, 0x82406724
	if ctx.cr[6].lt {
	pc = 0x82406724; continue 'dispatch;
	}
	// 82406740: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406744: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82406748: 386BC188  addi r3, r11, -0x3e78
	ctx.r[3].s64 = ctx.r[11].s64 + -15992;
	// 8240674C: 4BEAC835  bl 0x822b2f80
	ctx.lr = 0x82406750;
	sub_822B2F80(ctx, base);
	// 82406750: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82406754: 60630012  ori r3, r3, 0x12
	ctx.r[3].u64 = ctx.r[3].u64 | 18;
	// 82406758: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240675C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406760: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406764: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82406768: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240676C: 4E800020  blr
	return;
	// 82406770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82406774: 4BFFFFE4  b 0x82406758
	pc = 0x82406758; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82406778 size=172
    let mut pc: u32 = 0x82406778;
    'dispatch: loop {
        match pc {
            0x82406778 => {
    //   block [0x82406778..0x82406824)
	// 82406778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240677C: 4812E93D  bl 0x825350b8
	ctx.lr = 0x82406780;
	sub_82535080(ctx, base);
	// 82406780: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406784: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82406788: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240678C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82406790: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82406794: 4099007C  ble cr6, 0x82406810
	if !ctx.cr[6].gt {
	pc = 0x82406810; continue 'dispatch;
	}
	// 82406798: 4BFFFF71  bl 0x82406708
	ctx.lr = 0x8240679C;
	sub_82406708(ctx, base);
	// 8240679C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824067A0: 4080000C  bge 0x824067ac
	if !ctx.cr[0].lt {
	pc = 0x824067AC; continue 'dispatch;
	}
	// 824067A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824067A8: 48000074  b 0x8240681c
	pc = 0x8240681C; continue 'dispatch;
	// 824067AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824067B0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 824067B4: 4BFFEFFD  bl 0x824057b0
	ctx.lr = 0x824067B8;
	sub_824057B0(ctx, base);
	// 824067B8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824067BC: 40820060  bne 0x8240681c
	if !ctx.cr[0].eq {
	pc = 0x8240681C; continue 'dispatch;
	}
	// 824067C0: 7FAA07B4  extsw r10, r29
	ctx.r[10].s64 = ctx.r[29].s32 as i64;
	// 824067C4: C1A10050  lfs f13, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824067C8: 57EB2036  slwi r11, r31, 4
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824067CC: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 824067D0: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 824067D4: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824067D8: C8010058  lfd f0, 0x58(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 824067DC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 824067E0: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 824067E4: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 824067E8: C1AA1FF8  lfs f13, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824067EC: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 824067F0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824067F4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 824067F8: 40980018  bge cr6, 0x82406810
	if !ctx.cr[6].lt {
	pc = 0x82406810; continue 'dispatch;
	}
	// 824067FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82406800: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82406804: 938B000C  stw r28, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82406808: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8240680C: 4800000C  b 0x82406818
	pc = 0x82406818; continue 'dispatch;
	// 82406810: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82406814: 4BFFF585  bl 0x82405d98
	ctx.lr = 0x82406818;
	sub_82405D98(ctx, base);
	// 82406818: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240681C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82406820: 4812E8E8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82406828 size=220
    let mut pc: u32 = 0x82406828;
    'dispatch: loop {
        match pc {
            0x82406828 => {
    //   block [0x82406828..0x82406904)
	// 82406828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240682C: 4812E88D  bl 0x825350b8
	ctx.lr = 0x82406830;
	sub_82535080(ctx, base);
	// 82406830: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82406834: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406838: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240683C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82406840: 3BE3000C  addi r31, r3, 0xc
	ctx.r[31].s64 = ctx.r[3].s64 + 12;
	// 82406844: 3B8000C0  li r28, 0xc0
	ctx.r[28].s64 = 192;
	// 82406848: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8240684C: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82406850: 817FFFF8  lwz r11, -8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406854: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82406858: 40990084  ble cr6, 0x824068dc
	if !ctx.cr[6].gt {
	pc = 0x824068DC; continue 'dispatch;
	}
	// 8240685C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82406860: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82406864: 41820078  beq 0x824068dc
	if ctx.cr[0].eq {
	pc = 0x824068DC; continue 'dispatch;
	}
	// 82406868: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240686C: 4BFFEF45  bl 0x824057b0
	ctx.lr = 0x82406870;
	sub_824057B0(ctx, base);
	// 82406870: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82406874: 4182000C  beq 0x82406880
	if ctx.cr[0].eq {
	pc = 0x82406880; continue 'dispatch;
	}
	// 82406878: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8240687C: 4800006C  b 0x824068e8
	pc = 0x824068E8; continue 'dispatch;
	// 82406880: 7BAB0020  clrldi r11, r29, 0x20
	ctx.r[11].u64 = ctx.r[29].u64 & 0x00000000FFFFFFFFu64;
	// 82406884: C01FFFF4  lfs f0, -0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82406888: C1810050  lfs f12, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240688C: F9610058  std r11, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u64 ) };
	// 82406890: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 82406894: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82406898: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8240689C: EC20637A  fmadds f1, f0, f13, f12
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[13].f64 + ctx.f[12].f64) as f32) as f64);
	// 824068A0: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 824068A4: 40980008  bge cr6, 0x824068ac
	if !ctx.cr[6].lt {
	pc = 0x824068AC; continue 'dispatch;
	}
	// 824068A8: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824068AC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824068B0: 4BFFEE89  bl 0x82405738
	ctx.lr = 0x824068B4;
	sub_82405738(ctx, base);
	// 824068B4: 817FFFF8  lwz r11, -8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824068B8: 7D7D5851  subf. r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824068BC: 917FFFF8  stw r11, -8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), ctx.r[11].u32 ) };
	// 824068C0: 4181001C  bgt 0x824068dc
	if ctx.cr[0].gt {
	pc = 0x824068DC; continue 'dispatch;
	}
	// 824068C4: 817FFFFC  lwz r11, -4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824068C8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824068CC: 419A0010  beq cr6, 0x824068dc
	if ctx.cr[6].eq {
	pc = 0x824068DC; continue 'dispatch;
	}
	// 824068D0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824068D4: 4BFFF4C5  bl 0x82405d98
	ctx.lr = 0x824068D8;
	sub_82405D98(ctx, base);
	// 824068D8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 824068DC: 817FFFF8  lwz r11, -8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824068E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824068E4: 41990008  bgt cr6, 0x824068ec
	if ctx.cr[6].gt {
	pc = 0x824068EC; continue 'dispatch;
	}
	// 824068E8: 93DFFFF8  stw r30, -8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-8 as u32), ctx.r[30].u32 ) };
	// 824068EC: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824068F0: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 824068F4: 4082FF5C  bne 0x82406850
	if !ctx.cr[0].eq {
	pc = 0x82406850; continue 'dispatch;
	}
	// 824068F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824068FC: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82406900: 4812E808  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82406908 size=24
    let mut pc: u32 = 0x82406908;
    'dispatch: loop {
        match pc {
            0x82406908 => {
    //   block [0x82406908..0x82406920)
	// 82406908: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240690C: 3D408288  lis r10, -0x7d78
	ctx.r[10].s64 = -2105016320;
	// 82406910: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82406914: 386A3A68  addi r3, r10, 0x3a68
	ctx.r[3].s64 = ctx.r[10].s64 + 14952;
	// 82406918: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8240691C: 4BFFFE5C  b 0x82406778
	sub_82406778(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82406920 size=16
    let mut pc: u32 = 0x82406920;
    'dispatch: loop {
        match pc {
            0x82406920 => {
    //   block [0x82406920..0x82406930)
	// 82406920: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82406924: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82406928: 386B3A68  addi r3, r11, 0x3a68
	ctx.r[3].s64 = ctx.r[11].s64 + 14952;
	// 8240692C: 4BFFFEFC  b 0x82406828
	sub_82406828(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406930 size=60
    let mut pc: u32 = 0x82406930;
    'dispatch: loop {
        match pc {
            0x82406930 => {
    //   block [0x82406930..0x8240696C)
	// 82406930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406938: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240693C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82406940: 419A000C  beq cr6, 0x8240694c
	if ctx.cr[6].eq {
	pc = 0x8240694C; continue 'dispatch;
	}
	// 82406944: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82406948: 48000014  b 0x8240695c
	pc = 0x8240695C; continue 'dispatch;
	// 8240694C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406950: 386BC1D0  addi r3, r11, -0x3e30
	ctx.r[3].s64 = ctx.r[11].s64 + -15920;
	// 82406954: 4BEAC62D  bl 0x822b2f80
	ctx.lr = 0x82406958;
	sub_822B2F80(ctx, base);
	// 82406958: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240695C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82406960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406970 size=60
    let mut pc: u32 = 0x82406970;
    'dispatch: loop {
        match pc {
            0x82406970 => {
    //   block [0x82406970..0x824069AC)
	// 82406970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406978: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240697C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82406980: 419A000C  beq cr6, 0x8240698c
	if ctx.cr[6].eq {
	pc = 0x8240698C; continue 'dispatch;
	}
	// 82406984: 88640000  lbz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82406988: 48000014  b 0x8240699c
	pc = 0x8240699C; continue 'dispatch;
	// 8240698C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406990: 386BC228  addi r3, r11, -0x3dd8
	ctx.r[3].s64 = ctx.r[11].s64 + -15832;
	// 82406994: 4BEAC5ED  bl 0x822b2f80
	ctx.lr = 0x82406998;
	sub_822B2F80(ctx, base);
	// 82406998: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240699C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824069A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824069A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824069A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824069B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824069B0 size=60
    let mut pc: u32 = 0x824069B0;
    'dispatch: loop {
        match pc {
            0x824069B0 => {
    //   block [0x824069B0..0x824069EC)
	// 824069B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824069B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824069B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824069BC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824069C0: 419A000C  beq cr6, 0x824069cc
	if ctx.cr[6].eq {
	pc = 0x824069CC; continue 'dispatch;
	}
	// 824069C4: 88640001  lbz r3, 1(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1 as u32) ) } as u64;
	// 824069C8: 48000014  b 0x824069dc
	pc = 0x824069DC; continue 'dispatch;
	// 824069CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824069D0: 386BC278  addi r3, r11, -0x3d88
	ctx.r[3].s64 = ctx.r[11].s64 + -15752;
	// 824069D4: 4BEAC5AD  bl 0x822b2f80
	ctx.lr = 0x824069D8;
	sub_822B2F80(ctx, base);
	// 824069D8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824069DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824069E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824069E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824069E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824069F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824069F0 size=60
    let mut pc: u32 = 0x824069F0;
    'dispatch: loop {
        match pc {
            0x824069F0 => {
    //   block [0x824069F0..0x82406A2C)
	// 824069F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824069F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824069F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824069FC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82406A00: 419A000C  beq cr6, 0x82406a0c
	if ctx.cr[6].eq {
	pc = 0x82406A0C; continue 'dispatch;
	}
	// 82406A04: 88640002  lbz r3, 2(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(2 as u32) ) } as u64;
	// 82406A08: 48000014  b 0x82406a1c
	pc = 0x82406A1C; continue 'dispatch;
	// 82406A0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406A10: 386BC2D0  addi r3, r11, -0x3d30
	ctx.r[3].s64 = ctx.r[11].s64 + -15664;
	// 82406A14: 4BEAC56D  bl 0x822b2f80
	ctx.lr = 0x82406A18;
	sub_822B2F80(ctx, base);
	// 82406A18: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82406A1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82406A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406A28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406A30 size=60
    let mut pc: u32 = 0x82406A30;
    'dispatch: loop {
        match pc {
            0x82406A30 => {
    //   block [0x82406A30..0x82406A6C)
	// 82406A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406A34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406A38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406A3C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82406A40: 419A000C  beq cr6, 0x82406a4c
	if ctx.cr[6].eq {
	pc = 0x82406A4C; continue 'dispatch;
	}
	// 82406A44: 80640008  lwz r3, 8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82406A48: 48000014  b 0x82406a5c
	pc = 0x82406A5C; continue 'dispatch;
	// 82406A4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406A50: 386BC328  addi r3, r11, -0x3cd8
	ctx.r[3].s64 = ctx.r[11].s64 + -15576;
	// 82406A54: 4BEAC52D  bl 0x822b2f80
	ctx.lr = 0x82406A58;
	sub_822B2F80(ctx, base);
	// 82406A58: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82406A5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82406A60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406A64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406A68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406A70 size=60
    let mut pc: u32 = 0x82406A70;
    'dispatch: loop {
        match pc {
            0x82406A70 => {
    //   block [0x82406A70..0x82406AAC)
	// 82406A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406A74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406A78: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406A7C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82406A80: 419A000C  beq cr6, 0x82406a8c
	if ctx.cr[6].eq {
	pc = 0x82406A8C; continue 'dispatch;
	}
	// 82406A84: 8064000C  lwz r3, 0xc(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82406A88: 48000014  b 0x82406a9c
	pc = 0x82406A9C; continue 'dispatch;
	// 82406A8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406A90: 386BC378  addi r3, r11, -0x3c88
	ctx.r[3].s64 = ctx.r[11].s64 + -15496;
	// 82406A94: 4BEAC4ED  bl 0x822b2f80
	ctx.lr = 0x82406A98;
	sub_822B2F80(ctx, base);
	// 82406A98: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82406A9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82406AA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406AA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406AA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406AB0 size=60
    let mut pc: u32 = 0x82406AB0;
    'dispatch: loop {
        match pc {
            0x82406AB0 => {
    //   block [0x82406AB0..0x82406AEC)
	// 82406AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406AB8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406ABC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82406AC0: 419A000C  beq cr6, 0x82406acc
	if ctx.cr[6].eq {
	pc = 0x82406ACC; continue 'dispatch;
	}
	// 82406AC4: 80640010  lwz r3, 0x10(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82406AC8: 48000014  b 0x82406adc
	pc = 0x82406ADC; continue 'dispatch;
	// 82406ACC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406AD0: 386BC3C8  addi r3, r11, -0x3c38
	ctx.r[3].s64 = ctx.r[11].s64 + -15416;
	// 82406AD4: 4BEAC4AD  bl 0x822b2f80
	ctx.lr = 0x82406AD8;
	sub_822B2F80(ctx, base);
	// 82406AD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406ADC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82406AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406AE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406AF0 size=60
    let mut pc: u32 = 0x82406AF0;
    'dispatch: loop {
        match pc {
            0x82406AF0 => {
    //   block [0x82406AF0..0x82406B2C)
	// 82406AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406AF8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406AFC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82406B00: 419A000C  beq cr6, 0x82406b0c
	if ctx.cr[6].eq {
	pc = 0x82406B0C; continue 'dispatch;
	}
	// 82406B04: 80640014  lwz r3, 0x14(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82406B08: 48000014  b 0x82406b1c
	pc = 0x82406B1C; continue 'dispatch;
	// 82406B0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406B10: 386BC420  addi r3, r11, -0x3be0
	ctx.r[3].s64 = ctx.r[11].s64 + -15328;
	// 82406B14: 4BEAC46D  bl 0x822b2f80
	ctx.lr = 0x82406B18;
	sub_822B2F80(ctx, base);
	// 82406B18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406B1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82406B20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406B24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406B28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406B30 size=108
    let mut pc: u32 = 0x82406B30;
    'dispatch: loop {
        match pc {
            0x82406B30 => {
    //   block [0x82406B30..0x82406B9C)
	// 82406B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406B34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406B38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406B3C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82406B40: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82406B44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82406B48: 409A0014  bne cr6, 0x82406b5c
	if !ctx.cr[6].eq {
	pc = 0x82406B5C; continue 'dispatch;
	}
	// 82406B4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406B50: 386BC4D0  addi r3, r11, -0x3b30
	ctx.r[3].s64 = ctx.r[11].s64 + -15152;
	// 82406B54: 4BEAC42D  bl 0x822b2f80
	ctx.lr = 0x82406B58;
	sub_822B2F80(ctx, base);
	// 82406B58: 48000030  b 0x82406b88
	pc = 0x82406B88; continue 'dispatch;
	// 82406B5C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82406B60: 41980018  blt cr6, 0x82406b78
	if ctx.cr[6].lt {
	pc = 0x82406B78; continue 'dispatch;
	}
	// 82406B64: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82406B68: 7F045000  cmpw cr6, r4, r10
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82406B6C: 4098000C  bge cr6, 0x82406b78
	if !ctx.cr[6].lt {
	pc = 0x82406B78; continue 'dispatch;
	}
	// 82406B70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406B74: 48000018  b 0x82406b8c
	pc = 0x82406B8C; continue 'dispatch;
	// 82406B78: 80AB000C  lwz r5, 0xc(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82406B7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406B80: 386BC478  addi r3, r11, -0x3b88
	ctx.r[3].s64 = ctx.r[11].s64 + -15240;
	// 82406B84: 4BEAC3FD  bl 0x822b2f80
	ctx.lr = 0x82406B88;
	sub_822B2F80(ctx, base);
	// 82406B88: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82406B8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82406B90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406B94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406BA0 size=104
    let mut pc: u32 = 0x82406BA0;
    'dispatch: loop {
        match pc {
            0x82406BA0 => {
    //   block [0x82406BA0..0x82406C08)
	// 82406BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406BA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82406BAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82406BB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406BB4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82406BB8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82406BBC: 4BFFFF75  bl 0x82406b30
	ctx.lr = 0x82406BC0;
	sub_82406B30(ctx, base);
	// 82406BC0: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82406BC4: 40800018  bge 0x82406bdc
	if !ctx.cr[0].lt {
	pc = 0x82406BDC; continue 'dispatch;
	}
	// 82406BC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406BCC: 386BC520  addi r3, r11, -0x3ae0
	ctx.r[3].s64 = ctx.r[11].s64 + -15072;
	// 82406BD0: 4BEAC3B1  bl 0x822b2f80
	ctx.lr = 0x82406BD4;
	sub_822B2F80(ctx, base);
	// 82406BD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406BD8: 48000018  b 0x82406bf0
	pc = 0x82406BF0; continue 'dispatch;
	// 82406BDC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82406BE0: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82406BE4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82406BE8: 7D6BF82E  lwzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 82406BEC: 7C6BFA14  add r3, r11, r31
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82406BF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82406BF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406BF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406BFC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82406C00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82406C04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406C08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406C08 size=64
    let mut pc: u32 = 0x82406C08;
    'dispatch: loop {
        match pc {
            0x82406C08 => {
    //   block [0x82406C08..0x82406C48)
	// 82406C08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406C0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406C10: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406C14: 4BFFFF8D  bl 0x82406ba0
	ctx.lr = 0x82406C18;
	sub_82406BA0(ctx, base);
	// 82406C18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82406C1C: 40820018  bne 0x82406c34
	if !ctx.cr[0].eq {
	pc = 0x82406C34; continue 'dispatch;
	}
	// 82406C20: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406C24: 386BC55C  addi r3, r11, -0x3aa4
	ctx.r[3].s64 = ctx.r[11].s64 + -15012;
	// 82406C28: 4BEAC359  bl 0x822b2f80
	ctx.lr = 0x82406C2C;
	sub_822B2F80(ctx, base);
	// 82406C2C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406C30: 48000008  b 0x82406c38
	pc = 0x82406C38; continue 'dispatch;
	// 82406C34: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82406C38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82406C3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406C40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406C44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406C48 size=56
    let mut pc: u32 = 0x82406C48;
    'dispatch: loop {
        match pc {
            0x82406C48 => {
    //   block [0x82406C48..0x82406C80)
	// 82406C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406C4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406C50: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406C54: 4BFFFF4D  bl 0x82406ba0
	ctx.lr = 0x82406C58;
	sub_82406BA0(ctx, base);
	// 82406C58: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82406C5C: 40820014  bne 0x82406c70
	if !ctx.cr[0].eq {
	pc = 0x82406C70; continue 'dispatch;
	}
	// 82406C60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406C64: 386BC59C  addi r3, r11, -0x3a64
	ctx.r[3].s64 = ctx.r[11].s64 + -14948;
	// 82406C68: 4BEAC319  bl 0x822b2f80
	ctx.lr = 0x82406C6C;
	sub_822B2F80(ctx, base);
	// 82406C6C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82406C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406C80 size=76
    let mut pc: u32 = 0x82406C80;
    'dispatch: loop {
        match pc {
            0x82406C80 => {
    //   block [0x82406C80..0x82406CCC)
	// 82406C80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406C84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406C88: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406C8C: 4BFFFF15  bl 0x82406ba0
	ctx.lr = 0x82406C90;
	sub_82406BA0(ctx, base);
	// 82406C90: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82406C94: 40820014  bne 0x82406ca8
	if !ctx.cr[0].eq {
	pc = 0x82406CA8; continue 'dispatch;
	}
	// 82406C98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406C9C: 386BC5D8  addi r3, r11, -0x3a28
	ctx.r[3].s64 = ctx.r[11].s64 + -14888;
	// 82406CA0: 4BEAC2E1  bl 0x822b2f80
	ctx.lr = 0x82406CA4;
	sub_822B2F80(ctx, base);
	// 82406CA4: 48000014  b 0x82406cb8
	pc = 0x82406CB8; continue 'dispatch;
	// 82406CA8: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82406CAC: 386300BC  addi r3, r3, 0xbc
	ctx.r[3].s64 = ctx.r[3].s64 + 188;
	// 82406CB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82406CB4: 41990008  bgt cr6, 0x82406cbc
	if ctx.cr[6].gt {
	pc = 0x82406CBC; continue 'dispatch;
	}
	// 82406CB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406CBC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82406CC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406CC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406CD0 size=124
    let mut pc: u32 = 0x82406CD0;
    'dispatch: loop {
        match pc {
            0x82406CD0 => {
    //   block [0x82406CD0..0x82406D4C)
	// 82406CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406CD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82406CDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406CE0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82406CE4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82406CE8: 409A0018  bne cr6, 0x82406d00
	if !ctx.cr[6].eq {
	pc = 0x82406D00; continue 'dispatch;
	}
	// 82406CEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406CF0: 386BC648  addi r3, r11, -0x39b8
	ctx.r[3].s64 = ctx.r[11].s64 + -14776;
	// 82406CF4: 4BEAC28D  bl 0x822b2f80
	ctx.lr = 0x82406CF8;
	sub_822B2F80(ctx, base);
	// 82406CF8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82406CFC: 4800003C  b 0x82406d38
	pc = 0x82406D38; continue 'dispatch;
	// 82406D00: 4BFFFEA1  bl 0x82406ba0
	ctx.lr = 0x82406D04;
	sub_82406BA0(ctx, base);
	// 82406D04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82406D08: 40820024  bne 0x82406d2c
	if !ctx.cr[0].eq {
	pc = 0x82406D2C; continue 'dispatch;
	}
	// 82406D0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406D10: 386BC59C  addi r3, r11, -0x3a64
	ctx.r[3].s64 = ctx.r[11].s64 + -14948;
	// 82406D14: 4BEAC26D  bl 0x822b2f80
	ctx.lr = 0x82406D18;
	sub_822B2F80(ctx, base);
	// 82406D18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82406D1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406D20: 386BC610  addi r3, r11, -0x39f0
	ctx.r[3].s64 = ctx.r[11].s64 + -14832;
	// 82406D24: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82406D28: 4BFFFFCC  b 0x82406cf4
	pc = 0x82406CF4; continue 'dispatch;
	// 82406D2C: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82406D30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406D34: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82406D38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82406D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406D44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82406D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406D50 size=124
    let mut pc: u32 = 0x82406D50;
    'dispatch: loop {
        match pc {
            0x82406D50 => {
    //   block [0x82406D50..0x82406DCC)
	// 82406D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406D58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82406D5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406D60: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82406D64: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82406D68: 409A0018  bne cr6, 0x82406d80
	if !ctx.cr[6].eq {
	pc = 0x82406D80; continue 'dispatch;
	}
	// 82406D6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406D70: 386BC6C0  addi r3, r11, -0x3940
	ctx.r[3].s64 = ctx.r[11].s64 + -14656;
	// 82406D74: 4BEAC20D  bl 0x822b2f80
	ctx.lr = 0x82406D78;
	sub_822B2F80(ctx, base);
	// 82406D78: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82406D7C: 4800003C  b 0x82406db8
	pc = 0x82406DB8; continue 'dispatch;
	// 82406D80: 4BFFFE21  bl 0x82406ba0
	ctx.lr = 0x82406D84;
	sub_82406BA0(ctx, base);
	// 82406D84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82406D88: 40820024  bne 0x82406dac
	if !ctx.cr[0].eq {
	pc = 0x82406DAC; continue 'dispatch;
	}
	// 82406D8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406D90: 386BC59C  addi r3, r11, -0x3a64
	ctx.r[3].s64 = ctx.r[11].s64 + -14948;
	// 82406D94: 4BEAC1ED  bl 0x822b2f80
	ctx.lr = 0x82406D98;
	sub_822B2F80(ctx, base);
	// 82406D98: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82406D9C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406DA0: 386BC68C  addi r3, r11, -0x3974
	ctx.r[3].s64 = ctx.r[11].s64 + -14708;
	// 82406DA4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82406DA8: 4BFFFFCC  b 0x82406d74
	pc = 0x82406D74; continue 'dispatch;
	// 82406DAC: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82406DB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406DB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82406DB8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82406DBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406DC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406DC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82406DC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82406DD0 size=124
    let mut pc: u32 = 0x82406DD0;
    'dispatch: loop {
        match pc {
            0x82406DD0 => {
    //   block [0x82406DD0..0x82406E4C)
	// 82406DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82406DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82406DD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82406DDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82406DE0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82406DE4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82406DE8: 409A0018  bne cr6, 0x82406e00
	if !ctx.cr[6].eq {
	pc = 0x82406E00; continue 'dispatch;
	}
	// 82406DEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406DF0: 386BC740  addi r3, r11, -0x38c0
	ctx.r[3].s64 = ctx.r[11].s64 + -14528;
	// 82406DF4: 4BEAC18D  bl 0x822b2f80
	ctx.lr = 0x82406DF8;
	sub_822B2F80(ctx, base);
	// 82406DF8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82406DFC: 4800003C  b 0x82406e38
	pc = 0x82406E38; continue 'dispatch;
	// 82406E00: 4BFFFDA1  bl 0x82406ba0
	ctx.lr = 0x82406E04;
	sub_82406BA0(ctx, base);
	// 82406E04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82406E08: 40820024  bne 0x82406e2c
	if !ctx.cr[0].eq {
	pc = 0x82406E2C; continue 'dispatch;
	}
	// 82406E0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406E10: 386BC59C  addi r3, r11, -0x3a64
	ctx.r[3].s64 = ctx.r[11].s64 + -14948;
	// 82406E14: 4BEAC16D  bl 0x822b2f80
	ctx.lr = 0x82406E18;
	sub_822B2F80(ctx, base);
	// 82406E18: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82406E1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82406E20: 386BC704  addi r3, r11, -0x38fc
	ctx.r[3].s64 = ctx.r[11].s64 + -14588;
	// 82406E24: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82406E28: 4BFFFFCC  b 0x82406df4
	pc = 0x82406DF4; continue 'dispatch;
	// 82406E2C: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82406E30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406E34: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82406E38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82406E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82406E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82406E44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82406E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82406E50 size=12
    let mut pc: u32 = 0x82406E50;
    'dispatch: loop {
        match pc {
            0x82406E50 => {
    //   block [0x82406E50..0x82406E5C)
	// 82406E50: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82406E54: 806B3254  lwz r3, 0x3254(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12884 as u32) ) } as u64;
	// 82406E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82406E60 size=72
    let mut pc: u32 = 0x82406E60;
    'dispatch: loop {
        match pc {
            0x82406E60 => {
    //   block [0x82406E60..0x82406EA8)
	// 82406E60: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82406E64: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82406E68: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82406E6C: 390B4768  addi r8, r11, 0x4768
	ctx.r[8].s64 = ctx.r[11].s64 + 18280;
	// 82406E70: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82406E74: 39680018  addi r11, r8, 0x18
	ctx.r[11].s64 = ctx.r[8].s64 + 24;
	// 82406E78: C1A92268  lfs f13, 0x2268(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8808 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82406E7C: C00A1FF8  lfs f0, 0x1ff8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82406E80: 894BFFFC  lbz r10, -4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82406E84: 2B0A0002  cmplwi cr6, r10, 2
	ctx.cr[6].compare_u32(ctx.r[10].u32, 2 as u32, &mut ctx.xer);
	// 82406E88: 409A0044  bne cr6, 0x82406ecc
	if !ctx.cr[6].eq {
		sub_82406EA8(ctx, base);
		return;
	}
	// 82406E8C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82406E90: 7D435051  subf. r10, r3, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82406E94: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82406E98: 41810010  bgt 0x82406ea8
	if ctx.cr[0].gt {
		sub_82406EA8(ctx, base);
		return;
	}
	// 82406E9C: C18B0004  lfs f12, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82406EA0: 98EBFFFC  stb r7, -4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[7].u8 ) };
	// 82406EA4: 48000024  b 0x82406ec8
	sub_82406EA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82406EA8 size=88
    let mut pc: u32 = 0x82406EA8;
    'dispatch: loop {
        match pc {
            0x82406EA8 => {
    //   block [0x82406EA8..0x82406F00)
	// 82406EA8: 786A0020  clrldi r10, r3, 0x20
	ctx.r[10].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82406EAC: C18BFFEC  lfs f12, -0x14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82406EB0: C16B0008  lfs f11, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82406EB4: F941FFF0  std r10, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u64 ) };
	// 82406EB8: C941FFF0  lfd f10, -0x10(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82406EBC: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 82406EC0: FD405018  frsp f10, f10
	ctx.f[10].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82406EC4: ED8B62BA  fmadds f12, f11, f10, f12
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[10].f64 + ctx.f[12].f64) as f32) as f64);
	// 82406EC8: D18BFFEC  stfs f12, -0x14(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-20 as u32), tmp.u32 ) };
	// 82406ECC: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82406ED0: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82406ED4: 4081005C  ble 0x82406f30
	if !ctx.cr[0].gt {
		sub_82406F0C(ctx, base);
		return;
	}
	// 82406ED8: 7D435051  subf. r10, r3, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82406EDC: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82406EE0: 4181002C  bgt 0x82406f0c
	if ctx.cr[0].gt {
		sub_82406F0C(ctx, base);
		return;
	}
	// 82406EE4: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82406EE8: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82406EEC: 409A0014  bne cr6, 0x82406f00
	if !ctx.cr[6].eq {
		sub_82406F00(ctx, base);
		return;
	}
	// 82406EF0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82406EF4: D00B0018  stfs f0, 0x18(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82406EF8: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82406EFC: 48000034  b 0x82406f30
	sub_82406F0C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82406F00 size=12
    let mut pc: u32 = 0x82406F00;
    'dispatch: loop {
        match pc {
            0x82406F00 => {
    //   block [0x82406F00..0x82406F0C)
	// 82406F00: D1AB0018  stfs f13, 0x18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82406F04: 90EB0024  stw r7, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[7].u32 ) };
	// 82406F08: 48000028  b 0x82406f30
	sub_82406F0C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406F0C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82406F0C size=60
    let mut pc: u32 = 0x82406F0C;
    'dispatch: loop {
        match pc {
            0x82406F0C => {
    //   block [0x82406F0C..0x82406F48)
	// 82406F0C: 786A0020  clrldi r10, r3, 0x20
	ctx.r[10].u64 = ctx.r[3].u64 & 0x00000000FFFFFFFFu64;
	// 82406F10: C18B0018  lfs f12, 0x18(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82406F14: C16B001C  lfs f11, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82406F18: F941FFF8  std r10, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[10].u64 ) };
	// 82406F1C: C941FFF8  lfd f10, -8(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82406F20: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 82406F24: FD405018  frsp f10, f10
	ctx.f[10].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82406F28: ED8B62BA  fmadds f12, f11, f10, f12
	ctx.f[12].f64 = (((ctx.f[11].f64 * ctx.f[10].f64 + ctx.f[12].f64) as f32) as f64);
	// 82406F2C: D18B0018  stfs f12, 0x18(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82406F30: 396B0040  addi r11, r11, 0x40
	ctx.r[11].s64 = ctx.r[11].s64 + 64;
	// 82406F34: 39481118  addi r10, r8, 0x1118
	ctx.r[10].s64 = ctx.r[8].s64 + 4376;
	// 82406F38: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82406F3C: 4198FF44  blt cr6, 0x82406e80
	if ctx.cr[6].lt {
		sub_82406E60(ctx, base);
		return;
	}
	// 82406F40: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406F44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82406F48 size=24
    let mut pc: u32 = 0x82406F48;
    'dispatch: loop {
        match pc {
            0x82406F48 => {
    //   block [0x82406F48..0x82406F60)
	// 82406F48: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82406F4C: 546A1838  slwi r10, r3, 3
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82406F50: 396B4668  addi r11, r11, 0x4668
	ctx.r[11].s64 = ctx.r[11].s64 + 18024;
	// 82406F54: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82406F58: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82406F5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82406F60 size=68
    let mut pc: u32 = 0x82406F60;
    'dispatch: loop {
        match pc {
            0x82406F60 => {
    //   block [0x82406F60..0x82406FA4)
	// 82406F60: 39430040  addi r10, r3, 0x40
	ctx.r[10].s64 = ctx.r[3].s64 + 64;
	// 82406F64: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82406F68: 4198003C  blt cr6, 0x82406fa4
	if ctx.cr[6].lt {
		sub_82406FA4(ctx, base);
		return;
	}
	// 82406F6C: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82406F70: 816B3254  lwz r11, 0x3254(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12884 as u32) ) } as u64;
	// 82406F74: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82406F78: 4098002C  bge cr6, 0x82406fa4
	if !ctx.cr[6].lt {
		sub_82406FA4(ctx, base);
		return;
	}
	// 82406F7C: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82406F80: 55493032  slwi r9, r10, 6
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82406F84: 396B4768  addi r11, r11, 0x4768
	ctx.r[11].s64 = ctx.r[11].s64 + 18280;
	// 82406F88: 396B0014  addi r11, r11, 0x14
	ctx.r[11].s64 = ctx.r[11].s64 + 20;
	// 82406F8C: 7D6958AE  lbzx r11, r9, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82406F90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82406F94: 419A0010  beq cr6, 0x82406fa4
	if ctx.cr[6].eq {
		sub_82406FA4(ctx, base);
		return;
	}
	// 82406F98: 2B0A0043  cmplwi cr6, r10, 0x43
	ctx.cr[6].compare_u32(ctx.r[10].u32, 67 as u32, &mut ctx.xer);
	// 82406F9C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82406FA0: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406FA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82406FA4 size=8
    let mut pc: u32 = 0x82406FA4;
    'dispatch: loop {
        match pc {
            0x82406FA4 => {
    //   block [0x82406FA4..0x82406FAC)
	// 82406FA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82406FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82406FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82406FB0 size=88
    let mut pc: u32 = 0x82406FB0;
    'dispatch: loop {
        match pc {
            0x82406FB0 => {
    //   block [0x82406FB0..0x82407008)
	// 82406FB0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82406FB4: 4198004C  blt cr6, 0x82407000
	if ctx.cr[6].lt {
	pc = 0x82407000; continue 'dispatch;
	}
	// 82406FB8: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82406FBC: 394B4768  addi r10, r11, 0x4768
	ctx.r[10].s64 = ctx.r[11].s64 + 18280;
	// 82406FC0: 546B3032  slwi r11, r3, 6
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82406FC4: 392A0014  addi r9, r10, 0x14
	ctx.r[9].s64 = ctx.r[10].s64 + 20;
	// 82406FC8: 7D2B48AE  lbzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82406FCC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82406FD0: 419A0030  beq cr6, 0x82407000
	if ctx.cr[6].eq {
	pc = 0x82407000; continue 'dispatch;
	}
	// 82406FD4: 2B030043  cmplwi cr6, r3, 0x43
	ctx.cr[6].compare_u32(ctx.r[3].u32, 67 as u32, &mut ctx.xer);
	// 82406FD8: 41990028  bgt cr6, 0x82407000
	if ctx.cr[6].gt {
	pc = 0x82407000; continue 'dispatch;
	}
	// 82406FDC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82406FE0: 812B003C  lwz r9, 0x3c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 82406FE4: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 82406FE8: 419A0020  beq cr6, 0x82407008
	if ctx.cr[6].eq {
		sub_82407008(ctx, base);
		return;
	}
	// 82406FEC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82406FF0: 2F030040  cmpwi cr6, r3, 0x40
	ctx.cr[6].compare_i32(ctx.r[3].s32, 64, &mut ctx.xer);
	// 82406FF4: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82406FF8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82406FFC: 4098FFC4  bge cr6, 0x82406fc0
	if !ctx.cr[6].lt {
	pc = 0x82406FC0; continue 'dispatch;
	}
	// 82407000: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82407004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82407008 size=8
    let mut pc: u32 = 0x82407008;
    'dispatch: loop {
        match pc {
            0x82407008 => {
    //   block [0x82407008..0x82407010)
	// 82407008: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8240700C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82407010 size=52
    let mut pc: u32 = 0x82407010;
    'dispatch: loop {
        match pc {
            0x82407010 => {
    //   block [0x82407010..0x82407044)
	// 82407010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82407014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82407018: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240701C: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82407020: 4BFFFF41  bl 0x82406f60
	ctx.lr = 0x82407024;
	sub_82406F60(ctx, base);
	// 82407024: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82407028: 4182000C  beq 0x82407034
	if ctx.cr[0].eq {
	pc = 0x82407034; continue 'dispatch;
	}
	// 8240702C: 38680040  addi r3, r8, 0x40
	ctx.r[3].s64 = ctx.r[8].s64 + 64;
	// 82407030: 4BFFFF81  bl 0x82406fb0
	ctx.lr = 0x82407034;
	sub_82406FB0(ctx, base);
	// 82407034: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82407038: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240703C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82407040: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407048(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82407048 size=48
    let mut pc: u32 = 0x82407048;
    'dispatch: loop {
        match pc {
            0x82407048 => {
    //   block [0x82407048..0x82407078)
	// 82407048: 3D408288  lis r10, -0x7d78
	ctx.r[10].s64 = -2105016320;
	// 8240704C: 546B3032  slwi r11, r3, 6
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82407050: 394A4768  addi r10, r10, 0x4768
	ctx.r[10].s64 = ctx.r[10].s64 + 18280;
	// 82407054: 392A0014  addi r9, r10, 0x14
	ctx.r[9].s64 = ctx.r[10].s64 + 20;
	// 82407058: 7D2B48AE  lbzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8240705C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82407060: 419A000C  beq cr6, 0x8240706c
	if ctx.cr[6].eq {
	pc = 0x8240706C; continue 'dispatch;
	}
	// 82407064: 2B030043  cmplwi cr6, r3, 0x43
	ctx.cr[6].compare_u32(ctx.r[3].u32, 67 as u32, &mut ctx.xer);
	// 82407068: 40990010  ble cr6, 0x82407078
	if !ctx.cr[6].gt {
		sub_82407078(ctx, base);
		return;
	}
	// 8240706C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82407070: 60630001  ori r3, r3, 1
	ctx.r[3].u64 = ctx.r[3].u64 | 1;
	// 82407074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82407078 size=16
    let mut pc: u32 = 0x82407078;
    'dispatch: loop {
        match pc {
            0x82407078 => {
    //   block [0x82407078..0x82407088)
	// 82407078: 394A0028  addi r10, r10, 0x28
	ctx.r[10].s64 = ctx.r[10].s64 + 40;
	// 8240707C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82407080: 7C2B552E  stfsx f1, r11, r10
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	// 82407084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82407088 size=48
    let mut pc: u32 = 0x82407088;
    'dispatch: loop {
        match pc {
            0x82407088 => {
    //   block [0x82407088..0x824070B8)
	// 82407088: 3D408288  lis r10, -0x7d78
	ctx.r[10].s64 = -2105016320;
	// 8240708C: 546B3032  slwi r11, r3, 6
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82407090: 394A4768  addi r10, r10, 0x4768
	ctx.r[10].s64 = ctx.r[10].s64 + 18280;
	// 82407094: 392A0014  addi r9, r10, 0x14
	ctx.r[9].s64 = ctx.r[10].s64 + 20;
	// 82407098: 7D2B48AE  lbzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8240709C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824070A0: 419A000C  beq cr6, 0x824070ac
	if ctx.cr[6].eq {
	pc = 0x824070AC; continue 'dispatch;
	}
	// 824070A4: 2B030043  cmplwi cr6, r3, 0x43
	ctx.cr[6].compare_u32(ctx.r[3].u32, 67 as u32, &mut ctx.xer);
	// 824070A8: 40990010  ble cr6, 0x824070b8
	if !ctx.cr[6].gt {
		sub_824070B8(ctx, base);
		return;
	}
	// 824070AC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 824070B0: 60630001  ori r3, r3, 1
	ctx.r[3].u64 = ctx.r[3].u64 | 1;
	// 824070B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824070B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824070B8 size=96
    let mut pc: u32 = 0x824070B8;
    'dispatch: loop {
        match pc {
            0x824070B8 => {
    //   block [0x824070B8..0x82407118)
	// 824070B8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824070BC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 824070C0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 824070C4: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 824070C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824070CC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824070D0: C0091FF8  lfs f0, 0x1ff8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824070D4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824070D8: C1AA0004  lfs f13, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824070DC: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824070E0: C1AA0008  lfs f13, 8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824070E4: D1AB0008  stfs f13, 8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824070E8: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824070EC: D00B0028  stfs f0, 0x28(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824070F0: 990B0014  stb r8, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[8].u8 ) };
	// 824070F4: D00B0030  stfs f0, 0x30(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 824070F8: 912B002C  stw r9, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[9].u32 ) };
	// 824070FC: D00B0034  stfs f0, 0x34(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 82407100: 912B0038  stw r9, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[9].u32 ) };
	// 82407104: 912B003C  stw r9, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[9].u32 ) };
	// 82407108: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8240710C: 912B0024  stw r9, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 82407110: 90EB0010  stw r7, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 82407114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82407118 size=156
    let mut pc: u32 = 0x82407118;
    'dispatch: loop {
        match pc {
            0x82407118 => {
    //   block [0x82407118..0x824071B4)
	// 82407118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240711C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82407120: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82407124: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82407128: 390B4668  addi r8, r11, 0x4668
	ctx.r[8].s64 = ctx.r[11].s64 + 18024;
	// 8240712C: 546B3032  slwi r11, r3, 6
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82407130: 39480100  addi r10, r8, 0x100
	ctx.r[10].s64 = ctx.r[8].s64 + 256;
	// 82407134: 394A1000  addi r10, r10, 0x1000
	ctx.r[10].s64 = ctx.r[10].s64 + 4096;
	// 82407138: 7CEB5214  add r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8240713C: 4BFFFE25  bl 0x82406f60
	ctx.lr = 0x82407140;
	sub_82406F60(ctx, base);
	// 82407140: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82407144: 40820010  bne 0x82407154
	if !ctx.cr[0].eq {
	pc = 0x82407154; continue 'dispatch;
	}
	// 82407148: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240714C: 60630006  ori r3, r3, 6
	ctx.r[3].u64 = ctx.r[3].u64 | 6;
	// 82407150: 48000054  b 0x824071a4
	pc = 0x824071A4; continue 'dispatch;
	// 82407154: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82407158: 41980038  blt cr6, 0x82407190
	if ctx.cr[6].lt {
	pc = 0x82407190; continue 'dispatch;
	}
	// 8240715C: 2F040020  cmpwi cr6, r4, 0x20
	ctx.cr[6].compare_i32(ctx.r[4].s32, 32, &mut ctx.xer);
	// 82407160: 40980014  bge cr6, 0x82407174
	if !ctx.cr[6].lt {
	pc = 0x82407174; continue 'dispatch;
	}
	// 82407164: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82407168: 7D6B402E  lwzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8240716C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82407170: 409A0010  bne cr6, 0x82407180
	if !ctx.cr[6].eq {
	pc = 0x82407180; continue 'dispatch;
	}
	// 82407174: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82407178: 60630007  ori r3, r3, 7
	ctx.r[3].u64 = ctx.r[3].u64 | 7;
	// 8240717C: 48000028  b 0x824071a4
	pc = 0x824071A4; continue 'dispatch;
	// 82407180: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82407184: 90870010  stw r4, 0x10(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82407188: 91670024  stw r11, 0x24(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8240718C: 48000014  b 0x824071a0
	pc = 0x824071A0; continue 'dispatch;
	// 82407190: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82407194: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82407198: 91670010  stw r11, 0x10(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8240719C: 91470024  stw r10, 0x24(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 824071A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824071A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824071A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824071AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824071B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824071B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824071B8 size=80
    let mut pc: u32 = 0x824071B8;
    'dispatch: loop {
        match pc {
            0x824071B8 => {
    //   block [0x824071B8..0x82407208)
	// 824071B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824071BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824071C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824071C4: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 824071C8: 4BFFFD99  bl 0x82406f60
	ctx.lr = 0x824071CC;
	sub_82406F60(ctx, base);
	// 824071CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824071D0: 40820010  bne 0x824071e0
	if !ctx.cr[0].eq {
	pc = 0x824071E0; continue 'dispatch;
	}
	// 824071D4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 824071D8: 60630006  ori r3, r3, 6
	ctx.r[3].u64 = ctx.r[3].u64 | 6;
	// 824071DC: 4800001C  b 0x824071f8
	pc = 0x824071F8; continue 'dispatch;
	// 824071E0: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 824071E4: 550A3032  slwi r10, r8, 6
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824071E8: 396B4768  addi r11, r11, 0x4768
	ctx.r[11].s64 = ctx.r[11].s64 + 18280;
	// 824071EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824071F0: 396B1004  addi r11, r11, 0x1004
	ctx.r[11].s64 = ctx.r[11].s64 + 4100;
	// 824071F4: 7C2A5D2E  stfsx f1, r10, r11
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), tmp.u32) };
	// 824071F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824071FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82407200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82407204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82407208 size=76
    let mut pc: u32 = 0x82407208;
    'dispatch: loop {
        match pc {
            0x82407208 => {
    //   block [0x82407208..0x82407254)
	// 82407208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240720C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82407210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82407214: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82407218: 4BFFFD49  bl 0x82406f60
	ctx.lr = 0x8240721C;
	sub_82406F60(ctx, base);
	// 8240721C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82407220: 41820024  beq 0x82407244
	if ctx.cr[0].eq {
	pc = 0x82407244; continue 'dispatch;
	}
	// 82407224: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 82407228: 550A3032  slwi r10, r8, 6
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8240722C: 396B4768  addi r11, r11, 0x4768
	ctx.r[11].s64 = ctx.r[11].s64 + 18280;
	// 82407230: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82407234: 396B1000  addi r11, r11, 0x1000
	ctx.r[11].s64 = ctx.r[11].s64 + 4096;
	// 82407238: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8240723C: 806B0024  lwz r3, 0x24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82407240: 912B0024  stw r9, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 82407244: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82407248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240724C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82407250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82407258 size=52
    let mut pc: u32 = 0x82407258;
    'dispatch: loop {
        match pc {
            0x82407258 => {
    //   block [0x82407258..0x8240728C)
	// 82407258: 3D408288  lis r10, -0x7d78
	ctx.r[10].s64 = -2105016320;
	// 8240725C: 546B3032  slwi r11, r3, 6
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82407260: 394A4768  addi r10, r10, 0x4768
	ctx.r[10].s64 = ctx.r[10].s64 + 18280;
	// 82407264: 392A0014  addi r9, r10, 0x14
	ctx.r[9].s64 = ctx.r[10].s64 + 20;
	// 82407268: 7D2B48AE  lbzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8240726C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82407270: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82407274: 419A000C  beq cr6, 0x82407280
	if ctx.cr[6].eq {
	pc = 0x82407280; continue 'dispatch;
	}
	// 82407278: 2B030043  cmplwi cr6, r3, 0x43
	ctx.cr[6].compare_u32(ctx.r[3].u32, 67 as u32, &mut ctx.xer);
	// 8240727C: 40990010  ble cr6, 0x8240728c
	if !ctx.cr[6].gt {
		sub_8240728C(ctx, base);
		return;
	}
	// 82407280: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82407284: 60630001  ori r3, r3, 1
	ctx.r[3].u64 = ctx.r[3].u64 | 1;
	// 82407288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240728C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240728C size=12
    let mut pc: u32 = 0x8240728C;
    'dispatch: loop {
        match pc {
            0x8240728C => {
    //   block [0x8240728C..0x82407298)
	// 8240728C: D02B0004  stfs f1, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82407290: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82407294: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407298(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82407298 size=196
    let mut pc: u32 = 0x82407298;
    'dispatch: loop {
        match pc {
            0x82407298 => {
    //   block [0x82407298..0x8240735C)
	// 82407298: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240729C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824072A0: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824072A4: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 824072A8: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 824072AC: 9104000C  stw r8, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 824072B0: 38EB4768  addi r7, r11, 0x4768
	ctx.r[7].s64 = ctx.r[11].s64 + 18280;
	// 824072B4: 546B3032  slwi r11, r3, 6
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824072B8: 39470014  addi r10, r7, 0x14
	ctx.r[10].s64 = ctx.r[7].s64 + 20;
	// 824072BC: 7D4B50AE  lbzx r10, r11, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824072C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824072C4: 419A0098  beq cr6, 0x8240735c
	if ctx.cr[6].eq {
		sub_8240735C(ctx, base);
		return;
	}
	// 824072C8: 2B030043  cmplwi cr6, r3, 0x43
	ctx.cr[6].compare_u32(ctx.r[3].u32, 67 as u32, &mut ctx.xer);
	// 824072CC: 41990090  bgt cr6, 0x8240735c
	if ctx.cr[6].gt {
		sub_8240735C(ctx, base);
		return;
	}
	// 824072D0: 7D6B3A14  add r11, r11, r7
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 824072D4: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 824072D8: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824072DC: 41800030  blt 0x8240730c
	if ctx.cr[0].lt {
	pc = 0x8240730C; continue 'dispatch;
	}
	// 824072E0: 8124000C  lwz r9, 0xc(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 824072E4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 824072E8: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824072EC: 7D49212E  stwx r10, r9, r4
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32), ctx.r[10].u32) };
	// 824072F0: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 824072F4: 394A0024  addi r10, r10, 0x24
	ctx.r[10].s64 = ctx.r[10].s64 + 36;
	// 824072F8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824072FC: 7C6A212E  stwx r3, r10, r4
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[4].u32), ctx.r[3].u32) };
	// 82407300: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82407304: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82407308: 9144000C  stw r10, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8240730C: C16B0028  lfs f11, 0x28(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82407310: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82407314: C18B0030  lfs f12, 0x30(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82407318: 2F030040  cmpwi cr6, r3, 0x40
	ctx.cr[6].compare_i32(ctx.r[3].s32, 64, &mut ctx.xer);
	// 8240731C: ED8C582A  fadds f12, f12, f11
	ctx.f[12].f64 = ((ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64;
	// 82407320: C16B0004  lfs f11, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82407324: C14B0008  lfs f10, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82407328: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240732C: 7D0A4214  add r8, r10, r8
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82407330: EDAA682A  fadds f13, f10, f13
	ctx.f[13].f64 = ((ctx.f[10].f64 + ctx.f[13].f64) as f32) as f64;
	// 82407334: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82407338: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8240733C: ED8C582A  fadds f12, f12, f11
	ctx.f[12].f64 = ((ctx.f[12].f64 + ctx.f[11].f64) as f32) as f64;
	// 82407340: EC0C002A  fadds f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 82407344: 4098FF70  bge cr6, 0x824072b4
	if !ctx.cr[6].lt {
	pc = 0x824072B4; continue 'dispatch;
	}
	// 82407348: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240734C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82407350: D1A40004  stfs f13, 4(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82407354: 91040008  stw r8, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82407358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240735C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240735C size=12
    let mut pc: u32 = 0x8240735C;
    'dispatch: loop {
        match pc {
            0x8240735C => {
    //   block [0x8240735C..0x82407368)
	// 8240735C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82407360: 60630001  ori r3, r3, 1
	ctx.r[3].u64 = ctx.r[3].u64 | 1;
	// 82407364: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82407368 size=48
    let mut pc: u32 = 0x82407368;
    'dispatch: loop {
        match pc {
            0x82407368 => {
    //   block [0x82407368..0x82407398)
	// 82407368: 3D408288  lis r10, -0x7d78
	ctx.r[10].s64 = -2105016320;
	// 8240736C: 546B3032  slwi r11, r3, 6
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82407370: 394A4768  addi r10, r10, 0x4768
	ctx.r[10].s64 = ctx.r[10].s64 + 18280;
	// 82407374: 392A0014  addi r9, r10, 0x14
	ctx.r[9].s64 = ctx.r[10].s64 + 20;
	// 82407378: 7D2B48AE  lbzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8240737C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82407380: 419A000C  beq cr6, 0x8240738c
	if ctx.cr[6].eq {
	pc = 0x8240738C; continue 'dispatch;
	}
	// 82407384: 2B030043  cmplwi cr6, r3, 0x43
	ctx.cr[6].compare_u32(ctx.r[3].u32, 67 as u32, &mut ctx.xer);
	// 82407388: 40990010  ble cr6, 0x82407398
	if !ctx.cr[6].gt {
		sub_82407398(ctx, base);
		return;
	}
	// 8240738C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82407390: 60630001  ori r3, r3, 1
	ctx.r[3].u64 = ctx.r[3].u64 | 1;
	// 82407394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82407398 size=12
    let mut pc: u32 = 0x82407398;
    'dispatch: loop {
        match pc {
            0x82407398 => {
    //   block [0x82407398..0x824073A4)
	// 82407398: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8240739C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824073A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824073A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824073A8 size=48
    let mut pc: u32 = 0x824073A8;
    'dispatch: loop {
        match pc {
            0x824073A8 => {
    //   block [0x824073A8..0x824073D8)
	// 824073A8: 3D408288  lis r10, -0x7d78
	ctx.r[10].s64 = -2105016320;
	// 824073AC: 546B3032  slwi r11, r3, 6
	ctx.r[11].u32 = ctx.r[3].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824073B0: 394A4768  addi r10, r10, 0x4768
	ctx.r[10].s64 = ctx.r[10].s64 + 18280;
	// 824073B4: 392A0014  addi r9, r10, 0x14
	ctx.r[9].s64 = ctx.r[10].s64 + 20;
	// 824073B8: 7D2B48AE  lbzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 824073BC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824073C0: 419A000C  beq cr6, 0x824073cc
	if ctx.cr[6].eq {
	pc = 0x824073CC; continue 'dispatch;
	}
	// 824073C4: 2B030043  cmplwi cr6, r3, 0x43
	ctx.cr[6].compare_u32(ctx.r[3].u32, 67 as u32, &mut ctx.xer);
	// 824073C8: 40990010  ble cr6, 0x824073d8
	if !ctx.cr[6].gt {
		sub_824073D8(ctx, base);
		return;
	}
	// 824073CC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824073D0: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824073D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824073D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824073D8 size=28
    let mut pc: u32 = 0x824073D8;
    'dispatch: loop {
        match pc {
            0x824073D8 => {
    //   block [0x824073D8..0x824073F4)
	// 824073D8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824073DC: C1AB0028  lfs f13, 0x28(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824073E0: C00B0030  lfs f0, 0x30(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824073E4: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 824073E8: C1AB0004  lfs f13, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824073EC: EC20682A  fadds f1, f0, f13
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 824073F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824073F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824073F8 size=64
    let mut pc: u32 = 0x824073F8;
    'dispatch: loop {
        match pc {
            0x824073F8 => {
    //   block [0x824073F8..0x82407438)
	// 824073F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824073FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82407400: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82407404: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82407408: 38630040  addi r3, r3, 0x40
	ctx.r[3].s64 = ctx.r[3].s64 + 64;
	// 8240740C: 4BFFFE8D  bl 0x82407298
	ctx.lr = 0x82407410;
	sub_82407298(ctx, base);
	// 82407410: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82407414: 41820010  beq 0x82407424
	if ctx.cr[0].eq {
	pc = 0x82407424; continue 'dispatch;
	}
	// 82407418: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240741C: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82407420: 48000008  b 0x82407428
	pc = 0x82407428; continue 'dispatch;
	// 82407424: C0210050  lfs f1, 0x50(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82407428: 38210170  addi r1, r1, 0x170
	ctx.r[1].s64 = ctx.r[1].s64 + 368;
	// 8240742C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82407430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82407434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82407438 size=352
    let mut pc: u32 = 0x82407438;
    'dispatch: loop {
        match pc {
            0x82407438 => {
    //   block [0x82407438..0x82407598)
	// 82407438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240743C: 4812DC6D  bl 0x825350a8
	ctx.lr = 0x82407440;
	sub_82535080(ctx, base);
	// 82407440: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82407444: 4BFFE3C5  bl 0x82405808
	ctx.lr = 0x82407448;
	sub_82405808(ctx, base);
	// 82407448: 3D608288  lis r11, -0x7d78
	ctx.r[11].s64 = -2105016320;
	// 8240744C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82407450: 388B4668  addi r4, r11, 0x4668
	ctx.r[4].s64 = ctx.r[11].s64 + 18024;
	// 82407454: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82407458: 39640100  addi r11, r4, 0x100
	ctx.r[11].s64 = ctx.r[4].s64 + 256;
	// 8240745C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82407460: 38CB0008  addi r6, r11, 8
	ctx.r[6].s64 = ctx.r[11].s64 + 8;
	// 82407464: 81631C4C  lwz r11, 0x1c4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7244 as u32) ) } as u64;
	// 82407468: 392B043C  addi r9, r11, 0x43c
	ctx.r[9].s64 = ctx.r[11].s64 + 1084;
	// 8240746C: 3BEB003C  addi r31, r11, 0x3c
	ctx.r[31].s64 = ctx.r[11].s64 + 60;
	// 82407470: 3B4B047C  addi r26, r11, 0x47c
	ctx.r[26].s64 = ctx.r[11].s64 + 1148;
	// 82407474: 832B0000  lwz r25, 0(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82407478: 3BC9FC00  addi r30, r9, -0x400
	ctx.r[30].s64 = ctx.r[9].s64 + -1024;
	// 8240747C: 836B0004  lwz r27, 4(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82407480: 830B0008  lwz r24, 8(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82407484: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82407488: C18B1FF8  lfs f12, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240748C: 2F050040  cmpwi cr6, r5, 0x40
	ctx.cr[6].compare_i32(ctx.r[5].s32, 64, &mut ctx.xer);
	// 82407490: 40980020  bge cr6, 0x824074b0
	if !ctx.cr[6].lt {
	pc = 0x824074B0; continue 'dispatch;
	}
	// 82407494: 7F05C800  cmpw cr6, r5, r25
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[25].s32, &mut ctx.xer);
	// 82407498: 40980088  bge cr6, 0x82407520
	if !ctx.cr[6].lt {
	pc = 0x82407520; continue 'dispatch;
	}
	// 8240749C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 824074A0: 9BA6000C  stb r29, 0xc(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[29].u8 ) };
	// 824074A4: 93E6FFF8  stw r31, -8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-8 as u32), ctx.r[31].u32 ) };
	// 824074A8: 4BFFFBE1  bl 0x82407088
	ctx.lr = 0x824074AC;
	sub_82407088(ctx, base);
	// 824074AC: 4800007C  b 0x82407528
	pc = 0x82407528; continue 'dispatch;
	// 824074B0: 3865FFC0  addi r3, r5, -0x40
	ctx.r[3].s64 = ctx.r[5].s64 + -64;
	// 824074B4: 7F03D800  cmpw cr6, r3, r27
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[27].s32, &mut ctx.xer);
	// 824074B8: 40980068  bge cr6, 0x82407520
	if !ctx.cr[6].lt {
	pc = 0x82407520; continue 'dispatch;
	}
	// 824074BC: 9BA6000C  stb r29, 0xc(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[29].u8 ) };
	// 824074C0: 93C6FFF8  stw r30, -8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-8 as u32), ctx.r[30].u32 ) };
	// 824074C4: 4BFFFA9D  bl 0x82406f60
	ctx.lr = 0x824074C8;
	sub_82406F60(ctx, base);
	// 824074C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824074CC: 4182005C  beq 0x82407528
	if ctx.cr[0].eq {
	pc = 0x82407528; continue 'dispatch;
	}
	// 824074D0: 57CB003E  slwi r11, r30, 0
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824074D4: 81460008  lwz r10, 8(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 824074D8: C00B0004  lfs f0, 4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824074DC: 93860004  stw r28, 4(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 824074E0: D006FFFC  stfs f0, -4(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 824074E4: 9BA6000C  stb r29, 0xc(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[29].u8 ) };
	// 824074E8: D1860000  stfs f12, 0(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824074EC: 93860024  stw r28, 0x24(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(36 as u32), ctx.r[28].u32 ) };
	// 824074F0: D1860020  stfs f12, 0x20(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 824074F4: 93860030  stw r28, 0x30(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(48 as u32), ctx.r[28].u32 ) };
	// 824074F8: D1860028  stfs f12, 0x28(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 824074FC: 93860034  stw r28, 0x34(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(52 as u32), ctx.r[28].u32 ) };
	// 82407500: D186002C  stfs f12, 0x2c(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82407504: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82407508: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 8240750C: 419A001C  beq cr6, 0x82407528
	if ctx.cr[6].eq {
	pc = 0x82407528; continue 'dispatch;
	}
	// 82407510: 93A6001C  stw r29, 0x1c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82407514: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82407518: 91660008  stw r11, 8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240751C: 4800000C  b 0x82407528
	pc = 0x82407528; continue 'dispatch;
	// 82407520: 9386FFF8  stw r28, -8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(-8 as u32), ctx.r[28].u32 ) };
	// 82407524: 9B86000C  stb r28, 0xc(r6)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[6].u32.wrapping_add(12 as u32), ctx.r[28].u8 ) };
	// 82407528: 39640100  addi r11, r4, 0x100
	ctx.r[11].s64 = ctx.r[4].s64 + 256;
	// 8240752C: 93A6001C  stw r29, 0x1c(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82407530: 38C60040  addi r6, r6, 0x40
	ctx.r[6].s64 = ctx.r[6].s64 + 64;
	// 82407534: 396B1108  addi r11, r11, 0x1108
	ctx.r[11].s64 = ctx.r[11].s64 + 4360;
	// 82407538: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 8240753C: 3BFF0010  addi r31, r31, 0x10
	ctx.r[31].s64 = ctx.r[31].s64 + 16;
	// 82407540: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82407544: 7F065800  cmpw cr6, r6, r11
	ctx.cr[6].compare_i32(ctx.r[6].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82407548: 4198FF44  blt cr6, 0x8240748c
	if ctx.cr[6].lt {
	pc = 0x8240748C; continue 'dispatch;
	}
	// 8240754C: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82407550: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82407554: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82407558: 7F09C000  cmpw cr6, r9, r24
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[24].s32, &mut ctx.xer);
	// 8240755C: 40980010  bge cr6, 0x8240756c
	if !ctx.cr[6].lt {
	pc = 0x8240756C; continue 'dispatch;
	}
	// 82407560: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82407564: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82407568: 4800000C  b 0x82407574
	pc = 0x82407574; continue 'dispatch;
	// 8240756C: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82407570: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82407574: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82407578: 39040100  addi r8, r4, 0x100
	ctx.r[8].s64 = ctx.r[4].s64 + 256;
	// 8240757C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 82407580: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82407584: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82407588: 4198FFD0  blt cr6, 0x82407558
	if ctx.cr[6].lt {
	pc = 0x82407558; continue 'dispatch;
	}
	// 8240758C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82407590: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82407594: 4812DB64  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82407598 size=4
    let mut pc: u32 = 0x82407598;
    'dispatch: loop {
        match pc {
            0x82407598 => {
    //   block [0x82407598..0x8240759C)
	// 82407598: 4BFFFEA0  b 0x82407438
	sub_82407438(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824075A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824075A0 size=412
    let mut pc: u32 = 0x824075A0;
    'dispatch: loop {
        match pc {
            0x824075A0 => {
    //   block [0x824075A0..0x8240773C)
	// 824075A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824075A4: 4812DB09  bl 0x825350ac
	ctx.lr = 0x824075A8;
	sub_82535080(ctx, base);
	// 824075A8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824075AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824075B0: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 824075B4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 824075B8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 824075BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824075C0: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 824075C4: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 824075C8: 4198000C  blt cr6, 0x824075d4
	if ctx.cr[6].lt {
	pc = 0x824075D4; continue 'dispatch;
	}
	// 824075CC: 9B7F0211  stb r27, 0x211(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(529 as u32), ctx.r[27].u8 ) };
	// 824075D0: 48000008  b 0x824075d8
	pc = 0x824075D8; continue 'dispatch;
	// 824075D4: 9BBF0211  stb r29, 0x211(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(529 as u32), ctx.r[29].u8 ) };
	// 824075D8: 80BF0084  lwz r5, 0x84(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 824075DC: 90FF0214  stw r7, 0x214(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(532 as u32), ctx.r[7].u32 ) };
	// 824075E0: 2C050000  cmpwi r5, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824075E4: 41820040  beq 0x82407624
	if ctx.cr[0].eq {
	pc = 0x82407624; continue 'dispatch;
	}
	// 824075E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824075EC: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 824075F0: 386BC7D4  addi r3, r11, -0x382c
	ctx.r[3].s64 = ctx.r[11].s64 + -14380;
	// 824075F4: 4BEAB98D  bl 0x822b2f80
	ctx.lr = 0x824075F8;
	sub_822B2F80(ctx, base);
	// 824075F8: 3BDF1310  addi r30, r31, 0x1310
	ctx.r[30].s64 = ctx.r[31].s64 + 4880;
	// 824075FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82407600: 48305C5D  bl 0x8270d25c
	ctx.lr = 0x82407604;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82407604: 39600009  li r11, 9
	ctx.r[11].s64 = 9;
	// 82407608: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240760C: 939F00F0  stw r28, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[28].u32 ) };
	// 82407610: 93BF00EC  stw r29, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[29].u32 ) };
	// 82407614: 937F00F4  stw r27, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[27].u32 ) };
	// 82407618: 917F0134  stw r11, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 8240761C: 48305C51  bl 0x8270d26c
	ctx.lr = 0x82407620;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82407620: 48000034  b 0x82407654
	pc = 0x82407654; continue 'dispatch;
	// 82407624: 3BDF1310  addi r30, r31, 0x1310
	ctx.r[30].s64 = ctx.r[31].s64 + 4880;
	// 82407628: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240762C: 48305C31  bl 0x8270d25c
	ctx.lr = 0x82407630;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 82407630: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82407634: 939F00F0  stw r28, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[28].u32 ) };
	// 82407638: 937F00F4  stw r27, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[27].u32 ) };
	// 8240763C: 937F0134  stw r27, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[27].u32 ) };
	// 82407640: 48305C2D  bl 0x8270d26c
	ctx.lr = 0x82407644;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 82407644: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82407648: 809F0080  lwz r4, 0x80(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8240764C: 386BC7A4  addi r3, r11, -0x385c
	ctx.r[3].s64 = ctx.r[11].s64 + -14428;
	// 82407650: 4BEAB931  bl 0x822b2f80
	ctx.lr = 0x82407654;
	sub_822B2F80(ctx, base);
	// 82407654: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82407658: 3B9F01FC  addi r28, r31, 0x1fc
	ctx.r[28].s64 = ctx.r[31].s64 + 508;
	// 8240765C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82407660: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82407664: C04B1FF8  lfs f2, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82407668: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240766C: C02B1850  lfs f1, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82407670: 48000101  bl 0x82407770
	ctx.lr = 0x82407674;
	sub_82407770(ctx, base);
	// 82407674: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82407678: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8240767C: 48000105  bl 0x82407780
	ctx.lr = 0x82407680;
	sub_82407780(ctx, base);
	// 82407680: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82407684: 935F00E8  stw r26, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[26].u32 ) };
	// 82407688: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240768C: 93BF00EC  stw r29, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[29].u32 ) };
	// 82407690: 93BF01F0  stw r29, 0x1f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(496 as u32), ctx.r[29].u32 ) };
	// 82407694: 9BBF00FA  stb r29, 0xfa(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(250 as u32), ctx.r[29].u8 ) };
	// 82407698: 9BBF0210  stb r29, 0x210(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(528 as u32), ctx.r[29].u8 ) };
	// 8240769C: 419A004C  beq cr6, 0x824076e8
	if ctx.cr[6].eq {
	pc = 0x824076E8; continue 'dispatch;
	}
	// 824076A0: 7F2ACB78  mr r10, r25
	ctx.r[10].u64 = ctx.r[25].u64;
	// 824076A4: 397F0090  addi r11, r31, 0x90
	ctx.r[11].s64 = ctx.r[31].s64 + 144;
	// 824076A8: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824076AC: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824076B0: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824076B4: 7D084850  subf r8, r8, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 824076B8: 41820014  beq 0x824076cc
	if ctx.cr[0].eq {
	pc = 0x824076CC; continue 'dispatch;
	}
	// 824076BC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824076C0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824076C4: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824076C8: 419AFFE0  beq cr6, 0x824076a8
	if ctx.cr[6].eq {
	pc = 0x824076A8; continue 'dispatch;
	}
	// 824076CC: 2C080000  cmpwi r8, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824076D0: 41820018  beq 0x824076e8
	if ctx.cr[0].eq {
	pc = 0x824076E8; continue 'dispatch;
	}
	// 824076D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824076D8: 48305B85  bl 0x8270d25c
	ctx.lr = 0x824076DC;
	// extern call 0x8270D25C → crate::xboxkrnl::RtlEnterCriticalSection
	crate::xboxkrnl::RtlEnterCriticalSection(ctx, base);
	// 824076DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824076E0: 9B7F00F8  stb r27, 0xf8(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[27].u8 ) };
	// 824076E4: 48305B89  bl 0x8270d26c
	ctx.lr = 0x824076E8;
	// extern call 0x8270D26C → crate::xboxkrnl::RtlLeaveCriticalSection
	crate::xboxkrnl::RtlLeaveCriticalSection(ctx, base);
	// 824076E8: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 824076EC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824076F0: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824076F4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824076F8: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824076FC: 409AFFF4  bne cr6, 0x824076f0
	if !ctx.cr[6].eq {
	pc = 0x824076F0; continue 'dispatch;
	}
	// 82407700: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82407704: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82407708: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240770C: 38AB0001  addi r5, r11, 1
	ctx.r[5].s64 = ctx.r[11].s64 + 1;
	// 82407710: 2F050050  cmpwi cr6, r5, 0x50
	ctx.cr[6].compare_i32(ctx.r[5].s32, 80, &mut ctx.xer);
	// 82407714: 40980018  bge cr6, 0x8240772c
	if !ctx.cr[6].lt {
	pc = 0x8240772C; continue 'dispatch;
	}
	// 82407718: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8240771C: 387F0090  addi r3, r31, 0x90
	ctx.r[3].s64 = ctx.r[31].s64 + 144;
	// 82407720: 4BFB94E1  bl 0x823c0c00
	ctx.lr = 0x82407724;
	sub_823C0C00(ctx, base);
	// 82407724: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82407728: 4800000C  b 0x82407734
	pc = 0x82407734; continue 'dispatch;
	// 8240772C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82407730: 6063000C  ori r3, r3, 0xc
	ctx.r[3].u64 = ctx.r[3].u64 | 12;
	// 82407734: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82407738: 4812D9C4  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82407740 size=28
    let mut pc: u32 = 0x82407740;
    'dispatch: loop {
        match pc {
            0x82407740 => {
    //   block [0x82407740..0x8240775C)
	// 82407740: 81630084  lwz r11, 0x84(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(132 as u32) ) } as u64;
	// 82407744: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 82407748: 409A0014  bne cr6, 0x8240775c
	if !ctx.cr[6].eq {
		sub_8240775C(ctx, base);
		return;
	}
	// 8240774C: C003020C  lfs f0, 0x20c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(524 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82407750: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82407754: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82407758: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240775C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240775C size=8
    let mut pc: u32 = 0x8240775C;
    'dispatch: loop {
        match pc {
            0x8240775C => {
    //   block [0x8240775C..0x82407764)
	// 8240775C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82407760: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82407768 size=8
    let mut pc: u32 = 0x82407768;
    'dispatch: loop {
        match pc {
            0x82407768 => {
    //   block [0x82407768..0x82407770)
	// 82407768: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 8240776C: 4BFFFE34  b 0x824075a0
	sub_824075A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82407770 size=16
    let mut pc: u32 = 0x82407770;
    'dispatch: loop {
        match pc {
            0x82407770 => {
    //   block [0x82407770..0x82407780)
	// 82407770: D0230000  stfs f1, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82407774: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82407778: D0430004  stfs f2, 4(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240777C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82407780 size=20
    let mut pc: u32 = 0x82407780;
    'dispatch: loop {
        match pc {
            0x82407780 => {
    //   block [0x82407780..0x82407794)
	// 82407780: 7C8B0034  cntlzw r11, r4
	ctx.r[11].u64 = if ctx.r[4].u32 == 0 { 32 } else { ctx.r[4].u32.leading_zeros() as u64 };
	// 82407784: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82407788: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 8240778C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82407790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82407798 size=40
    let mut pc: u32 = 0x82407798;
    'dispatch: loop {
        match pc {
            0x82407798 => {
    //   block [0x82407798..0x824077C0)
	// 82407798: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240779C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824077A0: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824077A4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824077A8: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824077AC: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 824077B0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 824077B4: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824077B8: D1A30004  stfs f13, 4(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824077BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824077C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824077C0 size=60
    let mut pc: u32 = 0x824077C0;
    'dispatch: loop {
        match pc {
            0x824077C0 => {
    //   block [0x824077C0..0x824077FC)
	// 824077C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824077C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824077C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824077CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824077D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824077D4: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 824077D8: 48005001  bl 0x8240c7d8
	ctx.lr = 0x824077DC;
	sub_8240C7D8(ctx, base);
	// 824077DC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 824077E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824077E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824077E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824077EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824077F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824077F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824077F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82407800 size=8
    let mut pc: u32 = 0x82407800;
    'dispatch: loop {
        match pc {
            0x82407800 => {
    //   block [0x82407800..0x82407808)
	// 82407800: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 82407804: 48005014  b 0x8240c818
	sub_8240C818(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82407808 size=136
    let mut pc: u32 = 0x82407808;
    'dispatch: loop {
        match pc {
            0x82407808 => {
    //   block [0x82407808..0x82407890)
	// 82407808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240780C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82407810: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82407814: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82407818: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240781C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82407820: 38A0000B  li r5, 0xb
	ctx.r[5].s64 = 11;
	// 82407824: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82407828: 38610059  addi r3, r1, 0x59
	ctx.r[3].s64 = ctx.r[1].s64 + 89;
	// 8240782C: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 82407830: 4812D9A1  bl 0x825351d0
	ctx.lr = 0x82407834;
	sub_825351D0(ctx, base);
	// 82407834: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82407838: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8240783C: 396B6490  addi r11, r11, 0x6490
	ctx.r[11].s64 = ctx.r[11].s64 + 25744;
	// 82407840: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82407844: 99410059  stb r10, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[10].u8 ) };
	// 82407848: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8240784C: 4817CA75  bl 0x825842c0
	ctx.lr = 0x82407850;
	sub_825842C0(ctx, base);
	// 82407850: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82407854: 40800018  bge 0x8240786c
	if !ctx.cr[0].lt {
	pc = 0x8240786C; continue 'dispatch;
	}
	// 82407858: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240785C: 386BC80C  addi r3, r11, -0x37f4
	ctx.r[3].s64 = ctx.r[11].s64 + -14324;
	// 82407860: 4BEAB721  bl 0x822b2f80
	ctx.lr = 0x82407864;
	sub_822B2F80(ctx, base);
	// 82407864: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82407868: 48000014  b 0x8240787c
	pc = 0x8240787C; continue 'dispatch;
	// 8240786C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82407870: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 82407874: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82407878: 48004FE1  bl 0x8240c858
	ctx.lr = 0x8240787C;
	sub_8240C858(ctx, base);
	// 8240787C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82407880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82407884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82407888: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240788C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82407890 size=144
    let mut pc: u32 = 0x82407890;
    'dispatch: loop {
        match pc {
            0x82407890 => {
    //   block [0x82407890..0x82407920)
	// 82407890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82407894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82407898: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240789C: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 824078A0: 41990060  bgt cr6, 0x82407900
	if ctx.cr[6].gt {
	pc = 0x82407900; continue 'dispatch;
	}
	// 824078A4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 824078A8: 40980008  bge cr6, 0x824078b0
	if !ctx.cr[6].lt {
	pc = 0x824078B0; continue 'dispatch;
	}
	// 824078AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824078B0: 3D240001  addis r9, r4, 1
	ctx.r[9].s64 = ctx.r[4].s64 + 65536;
	// 824078B4: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 824078B8: 39298A85  addi r9, r9, -0x757b
	ctx.r[9].s64 = ctx.r[9].s64 + -30075;
	// 824078BC: 396B3258  addi r11, r11, 0x3258
	ctx.r[11].s64 = ctx.r[11].s64 + 12888;
	// 824078C0: 1D450030  mulli r10, r5, 0x30
	ctx.r[10].s64 = ctx.r[5].s64 * 48;
	// 824078C4: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824078C8: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 824078CC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824078D0: 7C69182E  lwzx r3, r9, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 824078D4: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 824078D8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 824078DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824078E0: 41820018  beq 0x824078f8
	if ctx.cr[0].eq {
	pc = 0x824078F8; continue 'dispatch;
	}
	// 824078E4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 824078E8: 38C00003  li r6, 3
	ctx.r[6].s64 = 3;
	// 824078EC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824078F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824078F4: 4817C31D  bl 0x82583c10
	ctx.lr = 0x824078F8;
	sub_82583C10(ctx, base);
	// 824078F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824078FC: 48000014  b 0x82407910
	pc = 0x82407910; continue 'dispatch;
	// 82407900: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82407904: 386BC83C  addi r3, r11, -0x37c4
	ctx.r[3].s64 = ctx.r[11].s64 + -14276;
	// 82407908: 4BEAB679  bl 0x822b2f80
	ctx.lr = 0x8240790C;
	sub_822B2F80(ctx, base);
	// 8240790C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82407910: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82407914: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82407918: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240791C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82407920 size=72
    let mut pc: u32 = 0x82407920;
    'dispatch: loop {
        match pc {
            0x82407920 => {
    //   block [0x82407920..0x82407968)
	// 82407920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82407924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82407928: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240792C: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 82407930: 41990018  bgt cr6, 0x82407948
	if ctx.cr[6].gt {
	pc = 0x82407948; continue 'dispatch;
	}
	// 82407934: 3D640001  addis r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 65536;
	// 82407938: 396B8A85  addi r11, r11, -0x757b
	ctx.r[11].s64 = ctx.r[11].s64 + -30075;
	// 8240793C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82407940: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82407944: 48000014  b 0x82407958
	pc = 0x82407958; continue 'dispatch;
	// 82407948: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240794C: 386BC878  addi r3, r11, -0x3788
	ctx.r[3].s64 = ctx.r[11].s64 + -14216;
	// 82407950: 4BEAB631  bl 0x822b2f80
	ctx.lr = 0x82407954;
	sub_822B2F80(ctx, base);
	// 82407954: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82407958: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240795C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82407960: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82407964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82407968 size=336
    let mut pc: u32 = 0x82407968;
    'dispatch: loop {
        match pc {
            0x82407968 => {
    //   block [0x82407968..0x82407AB8)
	// 82407968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240796C: 4812D749  bl 0x825350b4
	ctx.lr = 0x82407970;
	sub_82535080(ctx, base);
	// 82407970: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82407974: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82407978: 39410058  addi r10, r1, 0x58
	ctx.r[10].s64 = ctx.r[1].s64 + 88;
	// 8240797C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82407980: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82407984: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82407988: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240798C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82407990: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82407994: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82407998: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8240799C: D00100A4  stfs f0, 0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 824079A0: 3B800006  li r28, 6
	ctx.r[28].s64 = 6;
	// 824079A4: D00100AC  stfs f0, 0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 824079A8: 38A0001B  li r5, 0x1b
	ctx.r[5].s64 = 27;
	// 824079AC: D00100B4  stfs f0, 0xb4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 824079B0: 99610058  stb r11, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u8 ) };
	// 824079B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824079B8: 996100B9  stb r11, 0xb9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(185 as u32), ctx.r[11].u8 ) };
	// 824079BC: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 824079C0: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 824079C4: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 824079C8: 38610081  addi r3, r1, 0x81
	ctx.r[3].s64 = ctx.r[1].s64 + 129;
	// 824079CC: D00100BC  stfs f0, 0xbc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 824079D0: D00100C4  stfs f0, 0xc4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 824079D4: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 824079D8: D00100CC  stfs f0, 0xcc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 824079DC: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 824079E0: 996100C1  stb r11, 0xc1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(193 as u32), ctx.r[11].u8 ) };
	// 824079E4: 39600005  li r11, 5
	ctx.r[11].s64 = 5;
	// 824079E8: 9BC10070  stb r30, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[30].u8 ) };
	// 824079EC: 9BE100A0  stb r31, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[31].u8 ) };
	// 824079F0: 9BE100A1  stb r31, 0xa1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(161 as u32), ctx.r[31].u8 ) };
	// 824079F4: 9BE100A8  stb r31, 0xa8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[31].u8 ) };
	// 824079F8: 996100C9  stb r11, 0xc9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(201 as u32), ctx.r[11].u8 ) };
	// 824079FC: 396100A0  addi r11, r1, 0xa0
	ctx.r[11].s64 = ctx.r[1].s64 + 160;
	// 82407A00: 9BC100A9  stb r30, 0xa9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(169 as u32), ctx.r[30].u8 ) };
	// 82407A04: 9BE100B0  stb r31, 0xb0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u8 ) };
	// 82407A08: 994100B1  stb r10, 0xb1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(177 as u32), ctx.r[10].u8 ) };
	// 82407A0C: 9BE100B8  stb r31, 0xb8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[31].u8 ) };
	// 82407A10: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82407A14: 39610068  addi r11, r1, 0x68
	ctx.r[11].s64 = ctx.r[1].s64 + 104;
	// 82407A18: 9BE100C0  stb r31, 0xc0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[31].u8 ) };
	// 82407A1C: 9BE100C8  stb r31, 0xc8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[31].u8 ) };
	// 82407A20: 9B810060  stb r28, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[28].u8 ) };
	// 82407A24: 9BC10078  stb r30, 0x78(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[30].u8 ) };
	// 82407A28: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82407A2C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82407A30: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82407A34: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82407A38: 4812D799  bl 0x825351d0
	ctx.lr = 0x82407A3C;
	sub_825351D0(ctx, base);
	// 82407A3C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82407A40: 9B810089  stb r28, 0x89(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(137 as u32), ctx.r[28].u8 ) };
	// 82407A44: 9BE10080  stb r31, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u8 ) };
	// 82407A48: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82407A4C: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82407A50: 9161008C  stw r11, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82407A54: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 82407A58: 616BBB80  ori r11, r11, 0xbb80
	ctx.r[11].u64 = ctx.r[11].u64 | 48000;
	// 82407A5C: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 82407A60: 39610078  addi r11, r1, 0x78
	ctx.r[11].s64 = ctx.r[1].s64 + 120;
	// 82407A64: 91610090  stw r11, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 82407A68: 40990044  ble cr6, 0x82407aac
	if !ctx.cr[6].gt {
	pc = 0x82407AAC; continue 'dispatch;
	}
	// 82407A6C: 3FDB0002  addis r30, r27, 2
	ctx.r[30].s64 = ctx.r[27].s64 + 131072;
	// 82407A70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82407A74: 3BDE2A14  addi r30, r30, 0x2a14
	ctx.r[30].s64 = ctx.r[30].s64 + 10772;
	// 82407A78: 3BABC8B4  addi r29, r11, -0x374c
	ctx.r[29].s64 = ctx.r[11].s64 + -14156;
	// 82407A7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82407A80: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82407A84: 4817C8A5  bl 0x82584328
	ctx.lr = 0x82407A88;
	sub_82584328(ctx, base);
	// 82407A88: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82407A8C: 4080000C  bge 0x82407a98
	if !ctx.cr[0].lt {
	pc = 0x82407A98; continue 'dispatch;
	}
	// 82407A90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82407A94: 4BEAB4ED  bl 0x822b2f80
	ctx.lr = 0x82407A98;
	sub_822B2F80(ctx, base);
	// 82407A98: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82407A9C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82407AA0: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82407AA4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82407AA8: 4198FFD4  blt cr6, 0x82407a7c
	if ctx.cr[6].lt {
	pc = 0x82407A7C; continue 'dispatch;
	}
	// 82407AAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82407AB0: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82407AB4: 4812D650  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82407AB8 size=92
    let mut pc: u32 = 0x82407AB8;
    'dispatch: loop {
        match pc {
            0x82407AB8 => {
    //   block [0x82407AB8..0x82407B14)
	// 82407AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82407ABC: 4812D601  bl 0x825350bc
	ctx.lr = 0x82407AC0;
	sub_82535080(ctx, base);
	// 82407AC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82407AC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82407AC8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82407ACC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82407AD0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82407AD4: 40990038  ble cr6, 0x82407b0c
	if !ctx.cr[6].gt {
	pc = 0x82407B0C; continue 'dispatch;
	}
	// 82407AD8: 3FFE0002  addis r31, r30, 2
	ctx.r[31].s64 = ctx.r[30].s64 + 131072;
	// 82407ADC: 3BFF2A14  addi r31, r31, 0x2a14
	ctx.r[31].s64 = ctx.r[31].s64 + 10772;
	// 82407AE0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82407AE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82407AE8: 41820010  beq 0x82407af8
	if ctx.cr[0].eq {
	pc = 0x82407AF8; continue 'dispatch;
	}
	// 82407AEC: 4817C0BD  bl 0x82583ba8
	ctx.lr = 0x82407AF0;
	sub_82583BA8(ctx, base);
	// 82407AF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82407AF4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82407AF8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82407AFC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82407B00: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82407B04: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82407B08: 4198FFD8  blt cr6, 0x82407ae0
	if ctx.cr[6].lt {
	pc = 0x82407AE0; continue 'dispatch;
	}
	// 82407B0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82407B10: 4812D5FC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407B18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82407B18 size=60
    let mut pc: u32 = 0x82407B18;
    'dispatch: loop {
        match pc {
            0x82407B18 => {
    //   block [0x82407B18..0x82407B54)
	// 82407B18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82407B1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82407B20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82407B24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82407B28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82407B2C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 82407B30: 48004D81  bl 0x8240c8b0
	ctx.lr = 0x82407B34;
	sub_8240C8B0(ctx, base);
	// 82407B34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82407B38: 4BFFFF81  bl 0x82407ab8
	ctx.lr = 0x82407B3C;
	sub_82407AB8(ctx, base);
	// 82407B3C: 4817C68D  bl 0x825841c8
	ctx.lr = 0x82407B40;
	sub_825841C8(ctx, base);
	// 82407B40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82407B44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82407B48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82407B4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82407B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82407B58 size=516
    let mut pc: u32 = 0x82407B58;
    'dispatch: loop {
        match pc {
            0x82407B58 => {
    //   block [0x82407B58..0x82407D5C)
	// 82407B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82407B5C: 4812D561  bl 0x825350bc
	ctx.lr = 0x82407B60;
	sub_82535080(ctx, base);
	// 82407B60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82407B64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82407B68: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82407B6C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82407B70: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82407B74: 3FC08273  lis r30, -0x7d8d
	ctx.r[30].s64 = -2106392576;
	// 82407B78: 3C600008  lis r3, 8
	ctx.r[3].s64 = 524288;
	// 82407B7C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82407B80: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82407B84: 60630310  ori r3, r3, 0x310
	ctx.r[3].u64 = ctx.r[3].u64 | 784;
	// 82407B88: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82407B8C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82407B90: 93BF0010  stw r29, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[29].u32 ) };
	// 82407B94: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 82407B98: 817E3800  lwz r11, 0x3800(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14336 as u32) ) } as u64;
	// 82407B9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82407BA0: 4E800421  bctrl
	ctx.lr = 0x82407BA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82407BA4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82407BA8: 41820010  beq 0x82407bb8
	if ctx.cr[0].eq {
	pc = 0x82407BB8; continue 'dispatch;
	}
	// 82407BAC: 4800428D  bl 0x8240be38
	ctx.lr = 0x82407BB0;
	sub_8240BE38(ctx, base);
	// 82407BB0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82407BB4: 48000008  b 0x82407bbc
	pc = 0x82407BBC; continue 'dispatch;
	// 82407BB8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82407BBC: 3C600001  lis r3, 1
	ctx.r[3].s64 = 65536;
	// 82407BC0: 917F1C38  stw r11, 0x1c38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7224 as u32), ctx.r[11].u32 ) };
	// 82407BC4: 817E3800  lwz r11, 0x3800(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14336 as u32) ) } as u64;
	// 82407BC8: 60630D08  ori r3, r3, 0xd08
	ctx.r[3].u64 = ctx.r[3].u64 | 3336;
	// 82407BCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82407BD0: 4E800421  bctrl
	ctx.lr = 0x82407BD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82407BD4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82407BD8: 41820010  beq 0x82407be8
	if ctx.cr[0].eq {
	pc = 0x82407BE8; continue 'dispatch;
	}
	// 82407BDC: 4800408D  bl 0x8240bc68
	ctx.lr = 0x82407BE0;
	sub_8240BC68(ctx, base);
	// 82407BE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82407BE4: 48000008  b 0x82407bec
	pc = 0x82407BEC; continue 'dispatch;
	// 82407BE8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82407BEC: 917F1C3C  stw r11, 0x1c3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7228 as u32), ctx.r[11].u32 ) };
	// 82407BF0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82407BF4: 817E3800  lwz r11, 0x3800(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14336 as u32) ) } as u64;
	// 82407BF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82407BFC: 4E800421  bctrl
	ctx.lr = 0x82407C00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82407C00: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82407C04: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 82407C08: 917F1C40  stw r11, 0x1c40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7232 as u32), ctx.r[11].u32 ) };
	// 82407C0C: 817E3800  lwz r11, 0x3800(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14336 as u32) ) } as u64;
	// 82407C10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82407C14: 4E800421  bctrl
	ctx.lr = 0x82407C18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82407C18: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82407C1C: 41820010  beq 0x82407c2c
	if ctx.cr[0].eq {
	pc = 0x82407C2C; continue 'dispatch;
	}
	// 82407C20: 48007751  bl 0x8240f370
	ctx.lr = 0x82407C24;
	sub_8240F370(ctx, base);
	// 82407C24: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82407C28: 48000008  b 0x82407c30
	pc = 0x82407C30; continue 'dispatch;
	// 82407C2C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82407C30: 917F1C44  stw r11, 0x1c44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7236 as u32), ctx.r[11].u32 ) };
	// 82407C34: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82407C38: 817E3800  lwz r11, 0x3800(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14336 as u32) ) } as u64;
	// 82407C3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82407C40: 4E800421  bctrl
	ctx.lr = 0x82407C44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82407C44: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82407C48: 38600DBC  li r3, 0xdbc
	ctx.r[3].s64 = 3516;
	// 82407C4C: 917F1C48  stw r11, 0x1c48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7240 as u32), ctx.r[11].u32 ) };
	// 82407C50: 817E3800  lwz r11, 0x3800(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14336 as u32) ) } as u64;
	// 82407C54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82407C58: 4E800421  bctrl
	ctx.lr = 0x82407C5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82407C5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82407C60: 41820010  beq 0x82407c70
	if ctx.cr[0].eq {
	pc = 0x82407C70; continue 'dispatch;
	}
	// 82407C64: 4800717D  bl 0x8240ede0
	ctx.lr = 0x82407C68;
	sub_8240EDE0(ctx, base);
	// 82407C68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82407C6C: 48000008  b 0x82407c74
	pc = 0x82407C74; continue 'dispatch;
	// 82407C70: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82407C74: 917F1C4C  stw r11, 0x1c4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7244 as u32), ctx.r[11].u32 ) };
	// 82407C78: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82407C7C: 817E3800  lwz r11, 0x3800(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14336 as u32) ) } as u64;
	// 82407C80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82407C84: 4E800421  bctrl
	ctx.lr = 0x82407C88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82407C88: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82407C8C: 41820010  beq 0x82407c9c
	if ctx.cr[0].eq {
	pc = 0x82407C9C; continue 'dispatch;
	}
	// 82407C90: 4BEAB2F1  bl 0x822b2f80
	ctx.lr = 0x82407C94;
	sub_822B2F80(ctx, base);
	// 82407C94: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82407C98: 48000008  b 0x82407ca0
	pc = 0x82407CA0; continue 'dispatch;
	// 82407C9C: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82407CA0: 917F1C54  stw r11, 0x1c54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7252 as u32), ctx.r[11].u32 ) };
	// 82407CA4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82407CA8: 817E3800  lwz r11, 0x3800(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14336 as u32) ) } as u64;
	// 82407CAC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82407CB0: 4E800421  bctrl
	ctx.lr = 0x82407CB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82407CB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82407CB8: 41820010  beq 0x82407cc8
	if ctx.cr[0].eq {
	pc = 0x82407CC8; continue 'dispatch;
	}
	// 82407CBC: 4BEAB2C5  bl 0x822b2f80
	ctx.lr = 0x82407CC0;
	sub_822B2F80(ctx, base);
	// 82407CC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82407CC4: 48000008  b 0x82407ccc
	pc = 0x82407CCC; continue 'dispatch;
	// 82407CC8: 7FABEB78  mr r11, r29
	ctx.r[11].u64 = ctx.r[29].u64;
	// 82407CCC: 917F1C58  stw r11, 0x1c58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7256 as u32), ctx.r[11].u32 ) };
	// 82407CD0: 38602500  li r3, 0x2500
	ctx.r[3].s64 = 9472;
	// 82407CD4: 817E3800  lwz r11, 0x3800(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14336 as u32) ) } as u64;
	// 82407CD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82407CDC: 4E800421  bctrl
	ctx.lr = 0x82407CE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82407CE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82407CE4: 4182000C  beq 0x82407cf0
	if ctx.cr[0].eq {
	pc = 0x82407CF0; continue 'dispatch;
	}
	// 82407CE8: 48006F89  bl 0x8240ec70
	ctx.lr = 0x82407CEC;
	sub_8240EC70(ctx, base);
	// 82407CEC: 48000008  b 0x82407cf4
	pc = 0x82407CF4; continue 'dispatch;
	// 82407CF0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82407CF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82407CF8: 907F1C50  stw r3, 0x1c50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7248 as u32), ctx.r[3].u32 ) };
	// 82407CFC: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 82407D00: C1AB293C  lfs f13, 0x293c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10556 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82407D04: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82407D08: D1BF1C18  stfs f13, 0x1c18(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7192 as u32), tmp.u32 ) };
	// 82407D0C: C00B2268  lfs f0, 0x2268(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8808 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82407D10: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82407D14: D01F1C1C  stfs f0, 0x1c1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7196 as u32), tmp.u32 ) };
	// 82407D18: C1AB2424  lfs f13, 0x2424(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9252 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82407D1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82407D20: D1BF1C20  stfs f13, 0x1c20(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7200 as u32), tmp.u32 ) };
	// 82407D24: C18BC8EC  lfs f12, -0x3714(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14100 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82407D28: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82407D2C: D19F1C24  stfs f12, 0x1c24(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7204 as u32), tmp.u32 ) };
	// 82407D30: C1AB8E30  lfs f13, -0x71d0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29136 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82407D34: 397F0118  addi r11, r31, 0x118
	ctx.r[11].s64 = ctx.r[31].s64 + 280;
	// 82407D38: D1BF1C28  stfs f13, 0x1c28(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7208 as u32), tmp.u32 ) };
	// 82407D3C: D00BFF00  stfs f0, -0x100(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-256 as u32), tmp.u32 ) };
	// 82407D40: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82407D44: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82407D48: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82407D4C: 4082FFF0  bne 0x82407d3c
	if !ctx.cr[0].eq {
	pc = 0x82407D3C; continue 'dispatch;
	}
	// 82407D50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82407D54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82407D58: 4812D3B4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82407D60 size=56
    let mut pc: u32 = 0x82407D60;
    'dispatch: loop {
        match pc {
            0x82407D60 => {
    //   block [0x82407D60..0x82407D98)
	// 82407D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82407D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82407D68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82407D6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82407D70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82407D74: 807F1C50  lwz r3, 0x1c50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82407D78: 4BEE9659  bl 0x822f13d0
	ctx.lr = 0x82407D7C;
	sub_822F13D0(ctx, base);
	// 82407D7C: 807F1C38  lwz r3, 0x1c38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82407D80: 480048F9  bl 0x8240c678
	ctx.lr = 0x82407D84;
	sub_8240C678(ctx, base);
	// 82407D84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82407D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82407D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82407D90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82407D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82407D98 size=96
    let mut pc: u32 = 0x82407D98;
    'dispatch: loop {
        match pc {
            0x82407D98 => {
    //   block [0x82407D98..0x82407DF8)
	// 82407D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82407D9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82407DA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82407DA4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82407DA8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82407DAC: 814B1C4C  lwz r10, 0x1c4c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(7244 as u32) ) } as u64;
	// 82407DB0: 80AA0010  lwz r5, 0x10(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 82407DB4: 41980020  blt cr6, 0x82407dd4
	if ctx.cr[6].lt {
	pc = 0x82407DD4; continue 'dispatch;
	}
	// 82407DB8: 7F042800  cmpw cr6, r4, r5
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82407DBC: 40980018  bge cr6, 0x82407dd4
	if !ctx.cr[6].lt {
	pc = 0x82407DD4; continue 'dispatch;
	}
	// 82407DC0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82407DC4: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82407DC8: 4BFFE781  bl 0x82406548
	ctx.lr = 0x82407DCC;
	sub_82406548(ctx, base);
	// 82407DCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82407DD0: 48000018  b 0x82407de8
	pc = 0x82407DE8; continue 'dispatch;
	// 82407DD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82407DD8: 386BC8F0  addi r3, r11, -0x3710
	ctx.r[3].s64 = ctx.r[11].s64 + -14096;
	// 82407DDC: 4BEAB1A5  bl 0x822b2f80
	ctx.lr = 0x82407DE0;
	sub_822B2F80(ctx, base);
	// 82407DE0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82407DE4: 6063002B  ori r3, r3, 0x2b
	ctx.r[3].u64 = ctx.r[3].u64 | 43;
	// 82407DE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82407DEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82407DF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82407DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82407DF8 size=24
    let mut pc: u32 = 0x82407DF8;
    'dispatch: loop {
        match pc {
            0x82407DF8 => {
    //   block [0x82407DF8..0x82407E10)
	// 82407DF8: 2B04001F  cmplwi cr6, r4, 0x1f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 31 as u32, &mut ctx.xer);
	// 82407DFC: 41990014  bgt cr6, 0x82407e10
	if ctx.cr[6].gt {
		sub_82407E10(ctx, base);
		return;
	}
	// 82407E00: 1D6400D0  mulli r11, r4, 0xd0
	ctx.r[11].s64 = ctx.r[4].s64 * 208;
	// 82407E04: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82407E08: 386B0218  addi r3, r11, 0x218
	ctx.r[3].s64 = ctx.r[11].s64 + 536;
	// 82407E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82407E10 size=8
    let mut pc: u32 = 0x82407E10;
    'dispatch: loop {
        match pc {
            0x82407E10 => {
    //   block [0x82407E10..0x82407E18)
	// 82407E10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82407E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82407E18 size=108
    let mut pc: u32 = 0x82407E18;
    'dispatch: loop {
        match pc {
            0x82407E18 => {
    //   block [0x82407E18..0x82407E84)
	// 82407E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82407E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82407E20: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82407E24: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82407E28: 419A0048  beq cr6, 0x82407e70
	if ctx.cr[6].eq {
	pc = 0x82407E70; continue 'dispatch;
	}
	// 82407E2C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82407E30: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82407E34: 4199003C  bgt cr6, 0x82407e70
	if ctx.cr[6].gt {
	pc = 0x82407E70; continue 'dispatch;
	}
	// 82407E38: 81631C3C  lwz r11, 0x1c3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82407E3C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82407E40: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82407E44: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82407E48: 419A0028  beq cr6, 0x82407e70
	if ctx.cr[6].eq {
	pc = 0x82407E70; continue 'dispatch;
	}
	// 82407E4C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82407E50: 396B0164  addi r11, r11, 0x164
	ctx.r[11].s64 = ctx.r[11].s64 + 356;
	// 82407E54: 2F0A00C0  cmpwi cr6, r10, 0xc0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 192, &mut ctx.xer);
	// 82407E58: 4198FFE8  blt cr6, 0x82407e40
	if ctx.cr[6].lt {
	pc = 0x82407E40; continue 'dispatch;
	}
	// 82407E5C: 80631C50  lwz r3, 0x1c50(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82407E60: 48006B19  bl 0x8240e978
	ctx.lr = 0x82407E64;
	sub_8240E978(ctx, base);
	// 82407E64: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82407E68: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82407E6C: 40810008  ble 0x82407e74
	if !ctx.cr[0].gt {
	pc = 0x82407E74; continue 'dispatch;
	}
	// 82407E70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82407E74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82407E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82407E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82407E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82407E88 size=116
    let mut pc: u32 = 0x82407E88;
    'dispatch: loop {
        match pc {
            0x82407E88 => {
    //   block [0x82407E88..0x82407EFC)
	// 82407E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82407E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82407E90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82407E94: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82407E98: 419A0048  beq cr6, 0x82407ee0
	if ctx.cr[6].eq {
	pc = 0x82407EE0; continue 'dispatch;
	}
	// 82407E9C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82407EA0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82407EA4: 4199003C  bgt cr6, 0x82407ee0
	if ctx.cr[6].gt {
	pc = 0x82407EE0; continue 'dispatch;
	}
	// 82407EA8: 81631C3C  lwz r11, 0x1c3c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82407EAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82407EB0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82407EB4: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82407EB8: 419A003C  beq cr6, 0x82407ef4
	if ctx.cr[6].eq {
	pc = 0x82407EF4; continue 'dispatch;
	}
	// 82407EBC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82407EC0: 396B0164  addi r11, r11, 0x164
	ctx.r[11].s64 = ctx.r[11].s64 + 356;
	// 82407EC4: 2F0A00C0  cmpwi cr6, r10, 0xc0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 192, &mut ctx.xer);
	// 82407EC8: 4198FFE8  blt cr6, 0x82407eb0
	if ctx.cr[6].lt {
	pc = 0x82407EB0; continue 'dispatch;
	}
	// 82407ECC: 80631C50  lwz r3, 0x1c50(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82407ED0: 48006AA9  bl 0x8240e978
	ctx.lr = 0x82407ED4;
	sub_8240E978(ctx, base);
	// 82407ED4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82407ED8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82407EDC: 41810008  bgt 0x82407ee4
	if ctx.cr[0].gt {
	pc = 0x82407EE4; continue 'dispatch;
	}
	// 82407EE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82407EE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82407EE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82407EEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82407EF0: 4E800020  blr
	return;
	// 82407EF4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82407EF8: 4BFFFFEC  b 0x82407ee4
	pc = 0x82407EE4; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82407F00 size=228
    let mut pc: u32 = 0x82407F00;
    'dispatch: loop {
        match pc {
            0x82407F00 => {
    //   block [0x82407F00..0x82407FE4)
	// 82407F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82407F04: 4812D1B1  bl 0x825350b4
	ctx.lr = 0x82407F08;
	sub_82535080(ctx, base);
	// 82407F08: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82407F0C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82407F10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82407F14: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82407F18: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82407F1C: 419A0030  beq cr6, 0x82407f4c
	if ctx.cr[6].eq {
	pc = 0x82407F4C; continue 'dispatch;
	}
	// 82407F20: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82407F24: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82407F28: 41990024  bgt cr6, 0x82407f4c
	if ctx.cr[6].gt {
	pc = 0x82407F4C; continue 'dispatch;
	}
	// 82407F2C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82407F30: 807F1C38  lwz r3, 0x1c38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82407F34: 4800454D  bl 0x8240c480
	ctx.lr = 0x82407F38;
	sub_8240C480(ctx, base);
	// 82407F38: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82407F3C: 4182001C  beq 0x82407f58
	if ctx.cr[0].eq {
	pc = 0x82407F58; continue 'dispatch;
	}
	// 82407F40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82407F44: 386BC938  addi r3, r11, -0x36c8
	ctx.r[3].s64 = ctx.r[11].s64 + -14024;
	// 82407F48: 4BEAB039  bl 0x822b2f80
	ctx.lr = 0x82407F4C;
	sub_822B2F80(ctx, base);
	// 82407F4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82407F50: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82407F54: 4812D1B0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
	// 82407F58: 817F1C38  lwz r11, 0x1c38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82407F5C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82407F60: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82407F64: 3C6B0008  addis r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 524288;
	// 82407F68: 3863030C  addi r3, r3, 0x30c
	ctx.r[3].s64 = ctx.r[3].s64 + 780;
	// 82407F6C: 4BFFEC9D  bl 0x82406c08
	ctx.lr = 0x82407F70;
	sub_82406C08(ctx, base);
	// 82407F70: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82407F74: 4182FFD8  beq 0x82407f4c
	if ctx.cr[0].eq {
	pc = 0x82407F4C; continue 'dispatch;
	}
	// 82407F78: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82407F7C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82407F80: 807F1C3C  lwz r3, 0x1c3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82407F84: 7D7E182E  lwzx r11, r30, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82407F88: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82407F8C: 409A0014  bne cr6, 0x82407fa0
	if !ctx.cr[6].eq {
	pc = 0x82407FA0; continue 'dispatch;
	}
	// 82407F90: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82407F94: 4800332D  bl 0x8240b2c0
	ctx.lr = 0x82407F98;
	sub_8240B2C0(ctx, base);
	// 82407F98: 7F1C1840  cmplw cr6, r28, r3
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[3].u32, &mut ctx.xer);
	// 82407F9C: 419A0024  beq cr6, 0x82407fc0
	if ctx.cr[6].eq {
	pc = 0x82407FC0; continue 'dispatch;
	}
	// 82407FA0: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82407FA4: 3BDE0164  addi r30, r30, 0x164
	ctx.r[30].s64 = ctx.r[30].s64 + 356;
	// 82407FA8: 616B0B00  ori r11, r11, 0xb00
	ctx.r[11].u64 = ctx.r[11].u64 | 2816;
	// 82407FAC: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82407FB0: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82407FB4: 4198FFCC  blt cr6, 0x82407f80
	if ctx.cr[6].lt {
	pc = 0x82407F80; continue 'dispatch;
	}
	// 82407FB8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82407FBC: 4BFFFF94  b 0x82407f50
	pc = 0x82407F50; continue 'dispatch;
	// 82407FC0: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82407FC4: 807F1C3C  lwz r3, 0x1c3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82407FC8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82407FCC: 48003255  bl 0x8240b220
	ctx.lr = 0x82407FD0;
	sub_8240B220(ctx, base);
	// 82407FD0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82407FD4: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82407FD8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82407FDC: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82407FE0: 4BFFFF70  b 0x82407f50
	pc = 0x82407F50; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82407FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82407FE8 size=236
    let mut pc: u32 = 0x82407FE8;
    'dispatch: loop {
        match pc {
            0x82407FE8 => {
    //   block [0x82407FE8..0x824080D4)
	// 82407FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82407FEC: 4812D0C9  bl 0x825350b4
	ctx.lr = 0x82407FF0;
	sub_82535080(ctx, base);
	// 82407FF0: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 82407FF4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82407FF8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82407FFC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82408000: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82408004: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82408008: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240800C: 409A001C  bne cr6, 0x82408028
	if !ctx.cr[6].eq {
	pc = 0x82408028; continue 'dispatch;
	}
	// 82408010: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82408014: 386BC9F8  addi r3, r11, -0x3608
	ctx.r[3].s64 = ctx.r[11].s64 + -13832;
	// 82408018: 4BEAAF69  bl 0x822b2f80
	ctx.lr = 0x8240801C;
	sub_822B2F80(ctx, base);
	// 8240801C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82408020: 60630012  ori r3, r3, 0x12
	ctx.r[3].u64 = ctx.r[3].u64 | 18;
	// 82408024: 480000A4  b 0x824080c8
	pc = 0x824080C8; continue 'dispatch;
	// 82408028: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240802C: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82408030: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82408034: 40980028  bge cr6, 0x8240805c
	if !ctx.cr[6].lt {
	pc = 0x8240805C; continue 'dispatch;
	}
	// 82408038: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240803C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82408040: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 82408044: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 82408048: 386BC9C4  addi r3, r11, -0x363c
	ctx.r[3].s64 = ctx.r[11].s64 + -13884;
	// 8240804C: 4BEAAF35  bl 0x822b2f80
	ctx.lr = 0x82408050;
	sub_822B2F80(ctx, base);
	// 82408050: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82408054: 60630018  ori r3, r3, 0x18
	ctx.r[3].u64 = ctx.r[3].u64 | 24;
	// 82408058: 48000070  b 0x824080c8
	pc = 0x824080C8; continue 'dispatch;
	// 8240805C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82408060: 807C1C50  lwz r3, 0x1c50(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82408064: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82408068: 480067B9  bl 0x8240e820
	ctx.lr = 0x8240806C;
	sub_8240E820(ctx, base);
	// 8240806C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82408070: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82408074: 807C1C3C  lwz r3, 0x1c3c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82408078: 7D7E182E  lwzx r11, r30, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8240807C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 82408080: 409A0014  bne cr6, 0x82408094
	if !ctx.cr[6].eq {
	pc = 0x82408094; continue 'dispatch;
	}
	// 82408084: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82408088: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240808C: 4800346D  bl 0x8240b4f8
	ctx.lr = 0x82408090;
	sub_8240B4F8(ctx, base);
	// 82408090: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82408094: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82408098: 3BDE0164  addi r30, r30, 0x164
	ctx.r[30].s64 = ctx.r[30].s64 + 356;
	// 8240809C: 616B0B00  ori r11, r11, 0xb00
	ctx.r[11].u64 = ctx.r[11].u64 | 2816;
	// 824080A0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 824080A4: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824080A8: 4198FFCC  blt cr6, 0x82408074
	if ctx.cr[6].lt {
	pc = 0x82408074; continue 'dispatch;
	}
	// 824080AC: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 824080B0: 409A0014  bne cr6, 0x824080c4
	if !ctx.cr[6].eq {
	pc = 0x824080C4; continue 'dispatch;
	}
	// 824080B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824080B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824080BC: 386BC984  addi r3, r11, -0x367c
	ctx.r[3].s64 = ctx.r[11].s64 + -13948;
	// 824080C0: 4BFFFF58  b 0x82408018
	pc = 0x82408018; continue 'dispatch;
	// 824080C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824080C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824080CC: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 824080D0: 4812D034  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824080D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824080D8 size=184
    let mut pc: u32 = 0x824080D8;
    'dispatch: loop {
        match pc {
            0x824080D8 => {
    //   block [0x824080D8..0x82408190)
	// 824080D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824080DC: 4812CFD9  bl 0x825350b4
	ctx.lr = 0x824080E0;
	sub_82535080(ctx, base);
	// 824080E0: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 824080E4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824080E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824080EC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824080F0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824080F4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 824080F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824080FC: 409A001C  bne cr6, 0x82408118
	if !ctx.cr[6].eq {
	pc = 0x82408118; continue 'dispatch;
	}
	// 82408100: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82408104: 386BCA70  addi r3, r11, -0x3590
	ctx.r[3].s64 = ctx.r[11].s64 + -13712;
	// 82408108: 4BEAAE79  bl 0x822b2f80
	ctx.lr = 0x8240810C;
	sub_822B2F80(ctx, base);
	// 8240810C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82408110: 60630012  ori r3, r3, 0x12
	ctx.r[3].u64 = ctx.r[3].u64 | 18;
	// 82408114: 48000070  b 0x82408184
	pc = 0x82408184; continue 'dispatch;
	// 82408118: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240811C: 807C1C50  lwz r3, 0x1c50(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82408120: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82408124: 4800672D  bl 0x8240e850
	ctx.lr = 0x82408128;
	sub_8240E850(ctx, base);
	// 82408128: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8240812C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82408130: 807C1C3C  lwz r3, 0x1c3c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82408134: 7D7E182E  lwzx r11, r30, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 82408138: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 8240813C: 409A0014  bne cr6, 0x82408150
	if !ctx.cr[6].eq {
	pc = 0x82408150; continue 'dispatch;
	}
	// 82408140: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82408144: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82408148: 48003481  bl 0x8240b5c8
	ctx.lr = 0x8240814C;
	sub_8240B5C8(ctx, base);
	// 8240814C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82408150: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82408154: 3BDE0164  addi r30, r30, 0x164
	ctx.r[30].s64 = ctx.r[30].s64 + 356;
	// 82408158: 616B0B00  ori r11, r11, 0xb00
	ctx.r[11].u64 = ctx.r[11].u64 | 2816;
	// 8240815C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82408160: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82408164: 4198FFCC  blt cr6, 0x82408130
	if ctx.cr[6].lt {
	pc = 0x82408130; continue 'dispatch;
	}
	// 82408168: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8240816C: 409A0014  bne cr6, 0x82408180
	if !ctx.cr[6].eq {
	pc = 0x82408180; continue 'dispatch;
	}
	// 82408170: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82408174: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82408178: 386BCA34  addi r3, r11, -0x35cc
	ctx.r[3].s64 = ctx.r[11].s64 + -13772;
	// 8240817C: 4BFFFF8C  b 0x82408108
	pc = 0x82408108; continue 'dispatch;
	// 82408180: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82408184: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82408188: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 8240818C: 4812CF78  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82408190 size=176
    let mut pc: u32 = 0x82408190;
    'dispatch: loop {
        match pc {
            0x82408190 => {
    //   block [0x82408190..0x82408240)
	// 82408190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82408194: 4812CF1D  bl 0x825350b0
	ctx.lr = 0x82408198;
	sub_82535080(ctx, base);
	// 82408198: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240819C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824081A0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824081A4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 824081A8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 824081AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824081B0: 409A001C  bne cr6, 0x824081cc
	if !ctx.cr[6].eq {
	pc = 0x824081CC; continue 'dispatch;
	}
	// 824081B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824081B8: 386BCAE4  addi r3, r11, -0x351c
	ctx.r[3].s64 = ctx.r[11].s64 + -13596;
	// 824081BC: 4BEAADC5  bl 0x822b2f80
	ctx.lr = 0x824081C0;
	sub_822B2F80(ctx, base);
	// 824081C0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 824081C4: 60630012  ori r3, r3, 0x12
	ctx.r[3].u64 = ctx.r[3].u64 | 18;
	// 824081C8: 48000070  b 0x82408238
	pc = 0x82408238; continue 'dispatch;
	// 824081CC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 824081D0: 807C1C50  lwz r3, 0x1c50(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(7248 as u32) ) } as u64;
	// 824081D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824081D8: 480066A9  bl 0x8240e880
	ctx.lr = 0x824081DC;
	sub_8240E880(ctx, base);
	// 824081DC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824081E0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824081E4: 807C1C3C  lwz r3, 0x1c3c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(7228 as u32) ) } as u64;
	// 824081E8: 7D7E182E  lwzx r11, r30, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 824081EC: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 824081F0: 409A0014  bne cr6, 0x82408204
	if !ctx.cr[6].eq {
	pc = 0x82408204; continue 'dispatch;
	}
	// 824081F4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 824081F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824081FC: 4800337D  bl 0x8240b578
	ctx.lr = 0x82408200;
	sub_8240B578(ctx, base);
	// 82408200: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 82408204: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82408208: 3BDE0164  addi r30, r30, 0x164
	ctx.r[30].s64 = ctx.r[30].s64 + 356;
	// 8240820C: 616B0B00  ori r11, r11, 0xb00
	ctx.r[11].u64 = ctx.r[11].u64 | 2816;
	// 82408210: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82408214: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82408218: 4198FFCC  blt cr6, 0x824081e4
	if ctx.cr[6].lt {
	pc = 0x824081E4; continue 'dispatch;
	}
	// 8240821C: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 82408220: 409A0014  bne cr6, 0x82408234
	if !ctx.cr[6].eq {
	pc = 0x82408234; continue 'dispatch;
	}
	// 82408224: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82408228: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240822C: 386BCAA8  addi r3, r11, -0x3558
	ctx.r[3].s64 = ctx.r[11].s64 + -13656;
	// 82408230: 4BFFFF8C  b 0x824081bc
	pc = 0x824081BC; continue 'dispatch;
	// 82408234: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82408238: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8240823C: 4812CEC4  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82408240 size=124
    let mut pc: u32 = 0x82408240;
    'dispatch: loop {
        match pc {
            0x82408240 => {
    //   block [0x82408240..0x824082BC)
	// 82408240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82408244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82408248: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240824C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82408250: 409A001C  bne cr6, 0x8240826c
	if !ctx.cr[6].eq {
	pc = 0x8240826C; continue 'dispatch;
	}
	// 82408254: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82408258: 386BCB20  addi r3, r11, -0x34e0
	ctx.r[3].s64 = ctx.r[11].s64 + -13536;
	// 8240825C: 4BEAAD25  bl 0x822b2f80
	ctx.lr = 0x82408260;
	sub_822B2F80(ctx, base);
	// 82408260: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82408264: 60630012  ori r3, r3, 0x12
	ctx.r[3].u64 = ctx.r[3].u64 | 18;
	// 82408268: 48000044  b 0x824082ac
	pc = 0x824082AC; continue 'dispatch;
	// 8240826C: 80631C3C  lwz r3, 0x1c3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82408270: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82408274: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82408278: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240827C: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82408280: 419A0018  beq cr6, 0x82408298
	if ctx.cr[6].eq {
	pc = 0x82408298; continue 'dispatch;
	}
	// 82408284: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82408288: 394A0164  addi r10, r10, 0x164
	ctx.r[10].s64 = ctx.r[10].s64 + 356;
	// 8240828C: 2F0B00C0  cmpwi cr6, r11, 0xc0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 192, &mut ctx.xer);
	// 82408290: 4198FFE8  blt cr6, 0x82408278
	if ctx.cr[6].lt {
	pc = 0x82408278; continue 'dispatch;
	}
	// 82408294: 4BFFFFC0  b 0x82408254
	pc = 0x82408254; continue 'dispatch;
	// 82408298: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240829C: 4198FFB8  blt cr6, 0x82408254
	if ctx.cr[6].lt {
	pc = 0x82408254; continue 'dispatch;
	}
	// 824082A0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824082A4: 48003115  bl 0x8240b3b8
	ctx.lr = 0x824082A8;
	sub_8240B3B8(ctx, base);
	// 824082A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824082AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824082B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824082B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824082B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824082C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824082C0 size=136
    let mut pc: u32 = 0x824082C0;
    'dispatch: loop {
        match pc {
            0x824082C0 => {
    //   block [0x824082C0..0x82408348)
	// 824082C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824082C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824082C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824082CC: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824082D0: 409A001C  bne cr6, 0x824082ec
	if !ctx.cr[6].eq {
	pc = 0x824082EC; continue 'dispatch;
	}
	// 824082D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824082D8: 386BCB60  addi r3, r11, -0x34a0
	ctx.r[3].s64 = ctx.r[11].s64 + -13472;
	// 824082DC: 4BEAACA5  bl 0x822b2f80
	ctx.lr = 0x824082E0;
	sub_822B2F80(ctx, base);
	// 824082E0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 824082E4: 60630012  ori r3, r3, 0x12
	ctx.r[3].u64 = ctx.r[3].u64 | 18;
	// 824082E8: 48000050  b 0x82408338
	pc = 0x82408338; continue 'dispatch;
	// 824082EC: 80631C3C  lwz r3, 0x1c3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7228 as u32) ) } as u64;
	// 824082F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824082F4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824082F8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824082FC: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82408300: 419A0018  beq cr6, 0x82408318
	if ctx.cr[6].eq {
	pc = 0x82408318; continue 'dispatch;
	}
	// 82408304: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82408308: 394A0164  addi r10, r10, 0x164
	ctx.r[10].s64 = ctx.r[10].s64 + 356;
	// 8240830C: 2F0B00C0  cmpwi cr6, r11, 0xc0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 192, &mut ctx.xer);
	// 82408310: 4198FFE8  blt cr6, 0x824082f8
	if ctx.cr[6].lt {
	pc = 0x824082F8; continue 'dispatch;
	}
	// 82408314: 4800000C  b 0x82408320
	pc = 0x82408320; continue 'dispatch;
	// 82408318: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240831C: 40980010  bge cr6, 0x8240832c
	if !ctx.cr[6].lt {
	pc = 0x8240832C; continue 'dispatch;
	}
	// 82408320: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82408324: 386BCB9C  addi r3, r11, -0x3464
	ctx.r[3].s64 = ctx.r[11].s64 + -13412;
	// 82408328: 4BFFFFB4  b 0x824082dc
	pc = 0x824082DC; continue 'dispatch;
	// 8240832C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82408330: 480030D9  bl 0x8240b408
	ctx.lr = 0x82408334;
	sub_8240B408(ctx, base);
	// 82408334: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82408338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240833C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82408340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82408344: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82408348 size=124
    let mut pc: u32 = 0x82408348;
    'dispatch: loop {
        match pc {
            0x82408348 => {
    //   block [0x82408348..0x824083C4)
	// 82408348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240834C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82408350: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82408354: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82408358: 409A001C  bne cr6, 0x82408374
	if !ctx.cr[6].eq {
	pc = 0x82408374; continue 'dispatch;
	}
	// 8240835C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82408360: 386BCBE0  addi r3, r11, -0x3420
	ctx.r[3].s64 = ctx.r[11].s64 + -13344;
	// 82408364: 4BEAAC1D  bl 0x822b2f80
	ctx.lr = 0x82408368;
	sub_822B2F80(ctx, base);
	// 82408368: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240836C: 60630012  ori r3, r3, 0x12
	ctx.r[3].u64 = ctx.r[3].u64 | 18;
	// 82408370: 48000044  b 0x824083b4
	pc = 0x824083B4; continue 'dispatch;
	// 82408374: 80631C3C  lwz r3, 0x1c3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82408378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240837C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82408380: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82408384: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82408388: 419A0018  beq cr6, 0x824083a0
	if ctx.cr[6].eq {
	pc = 0x824083A0; continue 'dispatch;
	}
	// 8240838C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82408390: 394A0164  addi r10, r10, 0x164
	ctx.r[10].s64 = ctx.r[10].s64 + 356;
	// 82408394: 2F0B00C0  cmpwi cr6, r11, 0xc0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 192, &mut ctx.xer);
	// 82408398: 4198FFE8  blt cr6, 0x82408380
	if ctx.cr[6].lt {
	pc = 0x82408380; continue 'dispatch;
	}
	// 8240839C: 4BFFFFC0  b 0x8240835c
	pc = 0x8240835C; continue 'dispatch;
	// 824083A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824083A4: 4198FFB8  blt cr6, 0x8240835c
	if ctx.cr[6].lt {
	pc = 0x8240835C; continue 'dispatch;
	}
	// 824083A8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 824083AC: 480030AD  bl 0x8240b458
	ctx.lr = 0x824083B0;
	sub_8240B458(ctx, base);
	// 824083B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824083B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824083B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824083BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824083C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824083C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824083C8 size=124
    let mut pc: u32 = 0x824083C8;
    'dispatch: loop {
        match pc {
            0x824083C8 => {
    //   block [0x824083C8..0x82408444)
	// 824083C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824083CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824083D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824083D4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824083D8: 409A001C  bne cr6, 0x824083f4
	if !ctx.cr[6].eq {
	pc = 0x824083F4; continue 'dispatch;
	}
	// 824083DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824083E0: 386BCC28  addi r3, r11, -0x33d8
	ctx.r[3].s64 = ctx.r[11].s64 + -13272;
	// 824083E4: 4BEAAB9D  bl 0x822b2f80
	ctx.lr = 0x824083E8;
	sub_822B2F80(ctx, base);
	// 824083E8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 824083EC: 60630012  ori r3, r3, 0x12
	ctx.r[3].u64 = ctx.r[3].u64 | 18;
	// 824083F0: 48000044  b 0x82408434
	pc = 0x82408434; continue 'dispatch;
	// 824083F4: 80631C3C  lwz r3, 0x1c3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7228 as u32) ) } as u64;
	// 824083F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824083FC: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82408400: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82408404: 7F092040  cmplw cr6, r9, r4
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82408408: 419A0018  beq cr6, 0x82408420
	if ctx.cr[6].eq {
	pc = 0x82408420; continue 'dispatch;
	}
	// 8240840C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82408410: 394A0164  addi r10, r10, 0x164
	ctx.r[10].s64 = ctx.r[10].s64 + 356;
	// 82408414: 2F0B00C0  cmpwi cr6, r11, 0xc0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 192, &mut ctx.xer);
	// 82408418: 4198FFE8  blt cr6, 0x82408400
	if ctx.cr[6].lt {
	pc = 0x82408400; continue 'dispatch;
	}
	// 8240841C: 4BFFFFC0  b 0x824083dc
	pc = 0x824083DC; continue 'dispatch;
	// 82408420: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82408424: 4198FFB8  blt cr6, 0x824083dc
	if ctx.cr[6].lt {
	pc = 0x824083DC; continue 'dispatch;
	}
	// 82408428: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8240842C: 4800307D  bl 0x8240b4a8
	ctx.lr = 0x82408430;
	sub_8240B4A8(ctx, base);
	// 82408430: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82408434: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82408438: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240843C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82408440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82408448 size=24
    let mut pc: u32 = 0x82408448;
    'dispatch: loop {
        match pc {
            0x82408448 => {
    //   block [0x82408448..0x82408460)
	// 82408448: C0031C1C  lfs f0, 0x1c1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7196 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240844C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82408450: 41990010  bgt cr6, 0x82408460
	if ctx.cr[6].gt {
		sub_82408460(ctx, base);
		return;
	}
	// 82408454: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82408458: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240845C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82408460 size=40
    let mut pc: u32 = 0x82408460;
    'dispatch: loop {
        match pc {
            0x82408460 => {
    //   block [0x82408460..0x82408488)
	// 82408460: C1A31C24  lfs f13, 0x1c24(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7204 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82408464: C1831C28  lfs f12, 0x1c28(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7208 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82408468: FF016800  fcmpu cr6, f1, f13
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 8240846C: 4199001C  bgt cr6, 0x82408488
	if ctx.cr[6].gt {
		sub_82408488(ctx, base);
		return;
	}
	// 82408470: C0031C1C  lfs f0, 0x1c1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7196 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82408474: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82408478: EC010028  fsubs f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240847C: EDAC6824  fdivs f13, f12, f13
	ctx.f[13].f64 = ((ctx.f[12].f64 / ctx.f[13].f64) as f32) as f64;
	// 82408480: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82408484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82408488 size=24
    let mut pc: u32 = 0x82408488;
    'dispatch: loop {
        match pc {
            0x82408488 => {
    //   block [0x82408488..0x824084A0)
	// 82408488: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240848C: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82408490: ED8C0028  fsubs f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 82408494: EDAC6824  fdivs f13, f12, f13
	ctx.f[13].f64 = ((ctx.f[12].f64 / ctx.f[13].f64) as f32) as f64;
	// 82408498: EC2D007A  fmadds f1, f13, f1, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8240849C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824084A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824084A0 size=24
    let mut pc: u32 = 0x824084A0;
    'dispatch: loop {
        match pc {
            0x824084A0 => {
    //   block [0x824084A0..0x824084B8)
	// 824084A0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824084A4: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824084A8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 824084AC: 4199000C  bgt cr6, 0x824084b8
	if ctx.cr[6].gt {
		sub_824084B8(ctx, base);
		return;
	}
	// 824084B0: C0231C1C  lfs f1, 0x1c1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7196 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824084B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824084B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824084B8 size=36
    let mut pc: u32 = 0x824084B8;
    'dispatch: loop {
        match pc {
            0x824084B8 => {
    //   block [0x824084B8..0x824084DC)
	// 824084B8: C1A31C28  lfs f13, 0x1c28(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7208 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824084BC: C1831C24  lfs f12, 0x1c24(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7204 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824084C0: FF016800  fcmpu cr6, f1, f13
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 824084C4: 41990018  bgt cr6, 0x824084dc
	if ctx.cr[6].gt {
		sub_824084DC(ctx, base);
		return;
	}
	// 824084C8: C0031C1C  lfs f0, 0x1c1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7196 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824084CC: ED8C0028  fsubs f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 824084D0: EDAC6824  fdivs f13, f12, f13
	ctx.f[13].f64 = ((ctx.f[12].f64 / ctx.f[13].f64) as f32) as f64;
	// 824084D4: EC2D007A  fmadds f1, f13, f1, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 824084D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824084DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824084DC size=28
    let mut pc: u32 = 0x824084DC;
    'dispatch: loop {
        match pc {
            0x824084DC => {
    //   block [0x824084DC..0x824084F8)
	// 824084DC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824084E0: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824084E4: EDAD0028  fsubs f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 824084E8: EC010028  fsubs f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 824084EC: EDAC6824  fdivs f13, f12, f13
	ctx.f[13].f64 = ((ctx.f[12].f64 / ctx.f[13].f64) as f32) as f64;
	// 824084F0: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 824084F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824084F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824084F8 size=80
    let mut pc: u32 = 0x824084F8;
    'dispatch: loop {
        match pc {
            0x824084F8 => {
    //   block [0x824084F8..0x82408548)
	// 824084F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824084FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82408500: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82408504: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82408508: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240850C: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 82408510: 4BFFED89  bl 0x82407298
	ctx.lr = 0x82408514;
	sub_82407298(ctx, base);
	// 82408514: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82408518: 4082001C  bne 0x82408534
	if !ctx.cr[0].eq {
	pc = 0x82408534; continue 'dispatch;
	}
	// 8240851C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82408520: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82408524: 40990010  ble cr6, 0x82408534
	if !ctx.cr[6].gt {
	pc = 0x82408534; continue 'dispatch;
	}
	// 82408528: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 8240852C: 4BFFEA1D  bl 0x82406f48
	ctx.lr = 0x82408530;
	sub_82406F48(ctx, base);
	// 82408530: 48000008  b 0x82408538
	pc = 0x82408538; continue 'dispatch;
	// 82408534: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82408538: 38210170  addi r1, r1, 0x170
	ctx.r[1].s64 = ctx.r[1].s64 + 368;
	// 8240853C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82408540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82408544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82408548 size=328
    let mut pc: u32 = 0x82408548;
    'dispatch: loop {
        match pc {
            0x82408548 => {
    //   block [0x82408548..0x82408690)
	// 82408548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240854C: 4812CB5D  bl 0x825350a8
	ctx.lr = 0x82408550;
	sub_82535080(ctx, base);
	// 82408550: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82408554: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82408558: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240855C: 81781C4C  lwz r11, 0x1c4c(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(7244 as u32) ) } as u64;
	// 82408560: 3B2B0A7C  addi r25, r11, 0xa7c
	ctx.r[25].s64 = ctx.r[11].s64 + 2684;
	// 82408564: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82408568: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240856C: 409900E4  ble cr6, 0x82408650
	if !ctx.cr[6].gt {
	pc = 0x82408650; continue 'dispatch;
	}
	// 82408570: 7F2BCB78  mr r11, r25
	ctx.r[11].u64 = ctx.r[25].u64;
	// 82408574: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82408578: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8240857C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82408580: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82408584: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82408588: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8240858C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82408590: 4200FFF0  bdnz 0x82408580
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82408580; continue 'dispatch;
	}
	// 82408594: 83C10094  lwz r30, 0x94(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82408598: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240859C: 83A10090  lwz r29, 0x90(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 824085A0: 83810088  lwz r28, 0x88(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 824085A4: 7FCB07B4  extsw r11, r30
	ctx.r[11].s64 = ctx.r[30].s32 as i64;
	// 824085A8: 83610084  lwz r27, 0x84(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 824085AC: 7FAA07B4  extsw r10, r29
	ctx.r[10].s64 = ctx.r[29].s32 as i64;
	// 824085B0: 83410080  lwz r26, 0x80(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824085B4: 7F8907B4  extsw r9, r28
	ctx.r[9].s64 = ctx.r[28].s32 as i64;
	// 824085B8: 7F6807B4  extsw r8, r27
	ctx.r[8].s64 = ctx.r[27].s32 as i64;
	// 824085BC: 7F4707B4  extsw r7, r26
	ctx.r[7].s64 = ctx.r[26].s32 as i64;
	// 824085C0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 824085C4: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 824085C8: F9210060  std r9, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u64 ) };
	// 824085CC: F9010068  std r8, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[8].u64 ) };
	// 824085D0: F8E10070  std r7, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u64 ) };
	// 824085D4: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824085D8: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 824085DC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 824085E0: C9810060  lfd f12, 0x60(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 824085E4: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 824085E8: C9610068  lfd f11, 0x68(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 824085EC: FD80669C  fcfid f12, f12
	ctx.f[12].f64 = (ctx.f[12].s64 as f64);
	// 824085F0: C9410070  lfd f10, 0x70(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 824085F4: FD605E9C  fcfid f11, f11
	ctx.f[11].f64 = (ctx.f[11].s64 as f64);
	// 824085F8: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 824085FC: FCA00018  frsp f5, f0
	ctx.f[5].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82408600: FC806818  frsp f4, f13
	ctx.f[4].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82408604: FC606018  frsp f3, f12
	ctx.f[3].f64 = (ctx.f[12].f64 as f32) as f64;
	// 82408608: FC405818  frsp f2, f11
	ctx.f[2].f64 = (ctx.f[11].f64 as f32) as f64;
	// 8240860C: FC205018  frsp f1, f10
	ctx.f[1].f64 = (ctx.f[10].f64 as f32) as f64;
	// 82408610: 4BFFDEF1  bl 0x82406500
	ctx.lr = 0x82408614;
	sub_82406500(ctx, base);
	// 82408614: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82408618: 40820070  bne 0x82408688
	if !ctx.cr[0].eq {
	pc = 0x82408688; continue 'dispatch;
	}
	// 8240861C: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 82408620: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82408624: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82408628: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8240862C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82408630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82408634: 4BFFDF25  bl 0x82406558
	ctx.lr = 0x82408638;
	sub_82406558(ctx, base);
	// 82408638: 81781C4C  lwz r11, 0x1c4c(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(7244 as u32) ) } as u64;
	// 8240863C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82408640: 3B390020  addi r25, r25, 0x20
	ctx.r[25].s64 = ctx.r[25].s64 + 32;
	// 82408644: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82408648: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240864C: 4198FF24  blt cr6, 0x82408570
	if ctx.cr[6].lt {
	pc = 0x82408570; continue 'dispatch;
	}
	// 82408650: 81781C4C  lwz r11, 0x1c4c(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(7244 as u32) ) } as u64;
	// 82408654: 80AB0010  lwz r5, 0x10(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82408658: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8240865C: 41990018  bgt cr6, 0x82408674
	if ctx.cr[6].gt {
	pc = 0x82408674; continue 'dispatch;
	}
	// 82408660: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82408664: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82408668: 386BC8F0  addi r3, r11, -0x3710
	ctx.r[3].s64 = ctx.r[11].s64 + -14096;
	// 8240866C: 4BEAA915  bl 0x822b2f80
	ctx.lr = 0x82408670;
	sub_822B2F80(ctx, base);
	// 82408670: 48000014  b 0x82408684
	pc = 0x82408684; continue 'dispatch;
	// 82408674: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82408678: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8240867C: 91780008  stw r11, 8(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82408680: 4BFFDEC9  bl 0x82406548
	ctx.lr = 0x82408684;
	sub_82406548(ctx, base);
	// 82408684: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82408688: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8240868C: 4812CA6C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82408690 size=188
    let mut pc: u32 = 0x82408690;
    'dispatch: loop {
        match pc {
            0x82408690 => {
    //   block [0x82408690..0x8240874C)
	// 82408690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82408694: 4812CA25  bl 0x825350b8
	ctx.lr = 0x82408698;
	sub_82535080(ctx, base);
	// 82408698: DBA1FFC0  stfd f29, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[29].u64 ) };
	// 8240869C: DBC1FFC8  stfd f30, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 824086A0: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 824086A4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824086A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824086AC: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 824086B0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824086B4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 824086B8: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 824086BC: C3FE1C1C  lfs f31, 0x1c1c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7196 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824086C0: C3CB1FF8  lfs f30, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 824086C4: 41980070  blt cr6, 0x82408734
	if ctx.cr[6].lt {
	pc = 0x82408734; continue 'dispatch;
	}
	// 824086C8: EC5EE82A  fadds f2, f30, f29
	ctx.f[2].f64 = ((ctx.f[30].f64 + ctx.f[29].f64) as f32) as f64;
	// 824086CC: FF02F800  fcmpu cr6, f2, f31
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[31].f64);
	// 824086D0: 40980008  bge cr6, 0x824086d8
	if !ctx.cr[6].lt {
	pc = 0x824086D8; continue 'dispatch;
	}
	// 824086D4: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 824086D8: 397F0046  addi r11, r31, 0x46
	ctx.r[11].s64 = ctx.r[31].s64 + 70;
	// 824086DC: 557C103A  slwi r28, r11, 2
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 824086E0: 7D7CF02E  lwzx r11, r28, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 824086E4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824086E8: 397F0006  addi r11, r31, 6
	ctx.r[11].s64 = ctx.r[31].s64 + 6;
	// 824086EC: 409A0010  bne cr6, 0x824086fc
	if !ctx.cr[6].eq {
	pc = 0x824086FC; continue 'dispatch;
	}
	// 824086F0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824086F4: 7C4BF52E  stfsx f2, r11, r30
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), tmp.u32) };
	// 824086F8: 48000014  b 0x8240870c
	pc = 0x8240870C; continue 'dispatch;
	// 824086FC: 557D103A  slwi r29, r11, 2
	ctx.r[29].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[29].u64 = ctx.r[29].u32 as u64;
	// 82408700: 7C3DF42E  lfsx f1, r29, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82408704: 48005EAD  bl 0x8240e5b0
	ctx.lr = 0x82408708;
	sub_8240E5B0(ctx, base);
	// 82408708: 7C3DF52E  stfsx f1, r29, r30
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32), tmp.u32) };
	// 8240870C: 7D7CF02E  lwzx r11, r28, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82408710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82408714: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82408718: 7D7CF12E  stwx r11, r28, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32), ctx.r[11].u32) };
	// 8240871C: 4BFFEC8D  bl 0x824073a8
	ctx.lr = 0x82408720;
	sub_824073A8(ctx, base);
	// 82408720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82408724: EFC1F02A  fadds f30, f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ((ctx.f[1].f64 + ctx.f[30].f64) as f32) as f64;
	// 82408728: 4BFFEC41  bl 0x82407368
	ctx.lr = 0x8240872C;
	sub_82407368(ctx, base);
	// 8240872C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82408730: 4080FF98  bge 0x824086c8
	if !ctx.cr[0].lt {
	pc = 0x824086C8; continue 'dispatch;
	}
	// 82408734: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82408738: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8240873C: CBA1FFC0  lfd f29, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82408740: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82408744: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82408748: 4812C9C0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82408750 size=8
    let mut pc: u32 = 0x82408750;
    'dispatch: loop {
        match pc {
            0x82408750 => {
    //   block [0x82408750..0x82408758)
	// 82408750: 80631C50  lwz r3, 0x1c50(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82408754: 480063BC  b 0x8240eb10
	sub_8240EB10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82408758 size=8
    let mut pc: u32 = 0x82408758;
    'dispatch: loop {
        match pc {
            0x82408758 => {
    //   block [0x82408758..0x82408760)
	// 82408758: 80631C50  lwz r3, 0x1c50(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7248 as u32) ) } as u64;
	// 8240875C: 48006424  b 0x8240eb80
	sub_8240EB80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82408760 size=8
    let mut pc: u32 = 0x82408760;
    'dispatch: loop {
        match pc {
            0x82408760 => {
    //   block [0x82408760..0x82408768)
	// 82408760: 80631C50  lwz r3, 0x1c50(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82408764: 4800646C  b 0x8240ebd0
	sub_8240EBD0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82408768 size=8
    let mut pc: u32 = 0x82408768;
    'dispatch: loop {
        match pc {
            0x82408768 => {
    //   block [0x82408768..0x82408770)
	// 82408768: 80631C50  lwz r3, 0x1c50(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7248 as u32) ) } as u64;
	// 8240876C: 480065A4  b 0x8240ed10
	sub_8240ED10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82408770 size=152
    let mut pc: u32 = 0x82408770;
    'dispatch: loop {
        match pc {
            0x82408770 => {
    //   block [0x82408770..0x82408808)
	// 82408770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82408774: 4812C945  bl 0x825350b8
	ctx.lr = 0x82408778;
	sub_82535080(ctx, base);
	// 82408778: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240877C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82408780: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82408784: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82408788: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 8240878C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82408790: 807F1C50  lwz r3, 0x1c50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82408794: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82408798: 48005F91  bl 0x8240e728
	ctx.lr = 0x8240879C;
	sub_8240E728(ctx, base);
	// 8240879C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824087A0: 80C10050  lwz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824087A4: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 824087A8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824087AC: 419A002C  beq cr6, 0x824087d8
	if ctx.cr[6].eq {
	pc = 0x824087D8; continue 'dispatch;
	}
	// 824087B0: 80A10058  lwz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 824087B4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 824087B8: 41980020  blt cr6, 0x824087d8
	if ctx.cr[6].lt {
	pc = 0x824087D8; continue 'dispatch;
	}
	// 824087BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824087C0: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824087C4: 4BFFF73D  bl 0x82407f00
	ctx.lr = 0x824087C8;
	sub_82407F00(ctx, base);
	// 824087C8: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 824087CC: 409A000C  bne cr6, 0x824087d8
	if !ctx.cr[6].eq {
	pc = 0x824087D8; continue 'dispatch;
	}
	// 824087D0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824087D4: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824087D8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 824087DC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 824087E0: 2F1E0040  cmpwi cr6, r30, 0x40
	ctx.cr[6].compare_i32(ctx.r[30].s32, 64, &mut ctx.xer);
	// 824087E4: 4198FFA8  blt cr6, 0x8240878c
	if ctx.cr[6].lt {
	pc = 0x8240878C; continue 'dispatch;
	}
	// 824087E8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 824087EC: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 824087F0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824087F4: 807F1C50  lwz r3, 0x1c50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7248 as u32) ) } as u64;
	// 824087F8: 48005EA1  bl 0x8240e698
	ctx.lr = 0x824087FC;
	sub_8240E698(ctx, base);
	// 824087FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82408800: 38210190  addi r1, r1, 0x190
	ctx.r[1].s64 = ctx.r[1].s64 + 400;
	// 82408804: 4812C904  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82408808 size=308
    let mut pc: u32 = 0x82408808;
    'dispatch: loop {
        match pc {
            0x82408808 => {
    //   block [0x82408808..0x8240893C)
	// 82408808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240880C: 4812C8B1  bl 0x825350bc
	ctx.lr = 0x82408810;
	sub_82535080(ctx, base);
	// 82408810: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82408814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82408818: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8240881C: 3FC08273  lis r30, -0x7d8d
	ctx.r[30].s64 = -2106392576;
	// 82408820: 807F1C50  lwz r3, 0x1c50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82408824: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82408828: 4182001C  beq 0x82408844
	if ctx.cr[0].eq {
	pc = 0x82408844; continue 'dispatch;
	}
	// 8240882C: 4BEAA755  bl 0x822b2f80
	ctx.lr = 0x82408830;
	sub_822B2F80(ctx, base);
	// 82408830: 807F1C50  lwz r3, 0x1c50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82408834: 817E3804  lwz r11, 0x3804(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14340 as u32) ) } as u64;
	// 82408838: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8240883C: 4E800421  bctrl
	ctx.lr = 0x82408840;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82408840: 93BF1C50  stw r29, 0x1c50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7248 as u32), ctx.r[29].u32 ) };
	// 82408844: 807F1C58  lwz r3, 0x1c58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7256 as u32) ) } as u64;
	// 82408848: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240884C: 4182001C  beq 0x82408868
	if ctx.cr[0].eq {
	pc = 0x82408868; continue 'dispatch;
	}
	// 82408850: 4BEAA731  bl 0x822b2f80
	ctx.lr = 0x82408854;
	sub_822B2F80(ctx, base);
	// 82408854: 807F1C58  lwz r3, 0x1c58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7256 as u32) ) } as u64;
	// 82408858: 817E3804  lwz r11, 0x3804(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14340 as u32) ) } as u64;
	// 8240885C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82408860: 4E800421  bctrl
	ctx.lr = 0x82408864;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82408864: 93BF1C58  stw r29, 0x1c58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7256 as u32), ctx.r[29].u32 ) };
	// 82408868: 807F1C54  lwz r3, 0x1c54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7252 as u32) ) } as u64;
	// 8240886C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82408870: 4182001C  beq 0x8240888c
	if ctx.cr[0].eq {
	pc = 0x8240888C; continue 'dispatch;
	}
	// 82408874: 4BEAA70D  bl 0x822b2f80
	ctx.lr = 0x82408878;
	sub_822B2F80(ctx, base);
	// 82408878: 807F1C54  lwz r3, 0x1c54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7252 as u32) ) } as u64;
	// 8240887C: 817E3804  lwz r11, 0x3804(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14340 as u32) ) } as u64;
	// 82408880: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82408884: 4E800421  bctrl
	ctx.lr = 0x82408888;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82408888: 93BF1C54  stw r29, 0x1c54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7252 as u32), ctx.r[29].u32 ) };
	// 8240888C: 807F1C4C  lwz r3, 0x1c4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7244 as u32) ) } as u64;
	// 82408890: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82408894: 41820014  beq 0x824088a8
	if ctx.cr[0].eq {
	pc = 0x824088A8; continue 'dispatch;
	}
	// 82408898: 817E3804  lwz r11, 0x3804(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14340 as u32) ) } as u64;
	// 8240889C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824088A0: 4E800421  bctrl
	ctx.lr = 0x824088A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824088A4: 93BF1C4C  stw r29, 0x1c4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7244 as u32), ctx.r[29].u32 ) };
	// 824088A8: 807F1C48  lwz r3, 0x1c48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7240 as u32) ) } as u64;
	// 824088AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824088B0: 41820014  beq 0x824088c4
	if ctx.cr[0].eq {
	pc = 0x824088C4; continue 'dispatch;
	}
	// 824088B4: 817E3804  lwz r11, 0x3804(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14340 as u32) ) } as u64;
	// 824088B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824088BC: 4E800421  bctrl
	ctx.lr = 0x824088C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824088C0: 93BF1C48  stw r29, 0x1c48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7240 as u32), ctx.r[29].u32 ) };
	// 824088C4: 807F1C44  lwz r3, 0x1c44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7236 as u32) ) } as u64;
	// 824088C8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824088CC: 41820014  beq 0x824088e0
	if ctx.cr[0].eq {
	pc = 0x824088E0; continue 'dispatch;
	}
	// 824088D0: 817E3804  lwz r11, 0x3804(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14340 as u32) ) } as u64;
	// 824088D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824088D8: 4E800421  bctrl
	ctx.lr = 0x824088DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824088DC: 93BF1C44  stw r29, 0x1c44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7236 as u32), ctx.r[29].u32 ) };
	// 824088E0: 807F1C40  lwz r3, 0x1c40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7232 as u32) ) } as u64;
	// 824088E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 824088E8: 41820014  beq 0x824088fc
	if ctx.cr[0].eq {
	pc = 0x824088FC; continue 'dispatch;
	}
	// 824088EC: 817E3804  lwz r11, 0x3804(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14340 as u32) ) } as u64;
	// 824088F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824088F4: 4E800421  bctrl
	ctx.lr = 0x824088F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824088F8: 93BF1C40  stw r29, 0x1c40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7232 as u32), ctx.r[29].u32 ) };
	// 824088FC: 807F1C3C  lwz r3, 0x1c3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82408900: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82408904: 41820014  beq 0x82408918
	if ctx.cr[0].eq {
	pc = 0x82408918; continue 'dispatch;
	}
	// 82408908: 817E3804  lwz r11, 0x3804(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14340 as u32) ) } as u64;
	// 8240890C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82408910: 4E800421  bctrl
	ctx.lr = 0x82408914;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82408914: 93BF1C3C  stw r29, 0x1c3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7228 as u32), ctx.r[29].u32 ) };
	// 82408918: 807F1C38  lwz r3, 0x1c38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 8240891C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82408920: 41820014  beq 0x82408934
	if ctx.cr[0].eq {
	pc = 0x82408934; continue 'dispatch;
	}
	// 82408924: 817E3804  lwz r11, 0x3804(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(14340 as u32) ) } as u64;
	// 82408928: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8240892C: 4E800421  bctrl
	ctx.lr = 0x82408930;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82408930: 93BF1C38  stw r29, 0x1c38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(7224 as u32), ctx.r[29].u32 ) };
	// 82408934: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82408938: 4812C7D4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82408940 size=228
    let mut pc: u32 = 0x82408940;
    'dispatch: loop {
        match pc {
            0x82408940 => {
    //   block [0x82408940..0x82408A24)
	// 82408940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82408944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82408948: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240894C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82408950: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82408954: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82408958: 4BFFEC41  bl 0x82407598
	ctx.lr = 0x8240895C;
	sub_82407598(ctx, base);
	// 8240895C: 817F1C4C  lwz r11, 0x1c4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7244 as u32) ) } as u64;
	// 82408960: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82408964: 387F1C18  addi r3, r31, 0x1c18
	ctx.r[3].s64 = ctx.r[31].s64 + 7192;
	// 82408968: 388B001C  addi r4, r11, 0x1c
	ctx.r[4].s64 = ctx.r[11].s64 + 28;
	// 8240896C: 4BFB8295  bl 0x823c0c00
	ctx.lr = 0x82408970;
	sub_823C0C00(ctx, base);
	// 82408970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82408974: 4BFFFBD5  bl 0x82408548
	ctx.lr = 0x82408978;
	sub_82408548(ctx, base);
	// 82408978: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8240897C: 4182001C  beq 0x82408998
	if ctx.cr[0].eq {
	pc = 0x82408998; continue 'dispatch;
	}
	// 82408980: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82408984: 386BCCB0  addi r3, r11, -0x3350
	ctx.r[3].s64 = ctx.r[11].s64 + -13136;
	// 82408988: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240898C: 4BEAA5F5  bl 0x822b2f80
	ctx.lr = 0x82408990;
	sub_822B2F80(ctx, base);
	// 82408990: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82408994: 48000078  b 0x82408a0c
	pc = 0x82408A0C; continue 'dispatch;
	// 82408998: 813F1C4C  lwz r9, 0x1c4c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7244 as u32) ) } as u64;
	// 8240899C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824089A0: 3949067C  addi r10, r9, 0x67c
	ctx.r[10].s64 = ctx.r[9].s64 + 1660;
	// 824089A4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824089A8: 38E10150  addi r7, r1, 0x150
	ctx.r[7].s64 = ctx.r[1].s64 + 336;
	// 824089AC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824089B0: 7D0B392E  stwx r8, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[8].u32) };
	// 824089B4: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824089B8: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 824089BC: 7D0B312E  stwx r8, r11, r6
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[6].u32), ctx.r[8].u32) };
	// 824089C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824089C4: 2F0B0100  cmpwi cr6, r11, 0x100
	ctx.cr[6].compare_i32(ctx.r[11].s32, 256, &mut ctx.xer);
	// 824089C8: 4198FFDC  blt cr6, 0x824089a4
	if ctx.cr[6].lt {
	pc = 0x824089A4; continue 'dispatch;
	}
	// 824089CC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824089D0: 80C9000C  lwz r6, 0xc(r9)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 824089D4: 38810150  addi r4, r1, 0x150
	ctx.r[4].s64 = ctx.r[1].s64 + 336;
	// 824089D8: 807F1C3C  lwz r3, 0x1c3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7228 as u32) ) } as u64;
	// 824089DC: 48002C8D  bl 0x8240b668
	ctx.lr = 0x824089E0;
	sub_8240B668(ctx, base);
	// 824089E0: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824089E4: 41820010  beq 0x824089f4
	if ctx.cr[0].eq {
	pc = 0x824089F4; continue 'dispatch;
	}
	// 824089E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824089EC: 386BCC70  addi r3, r11, -0x3390
	ctx.r[3].s64 = ctx.r[11].s64 + -13200;
	// 824089F0: 4BFFFF98  b 0x82408988
	pc = 0x82408988; continue 'dispatch;
	// 824089F4: 817F1C4C  lwz r11, 0x1c4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7244 as u32) ) } as u64;
	// 824089F8: 807F1C44  lwz r3, 0x1c44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7236 as u32) ) } as u64;
	// 824089FC: 388B0BBC  addi r4, r11, 0xbbc
	ctx.r[4].s64 = ctx.r[11].s64 + 3004;
	// 82408A00: 80AB0014  lwz r5, 0x14(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82408A04: 4BEE89CD  bl 0x822f13d0
	ctx.lr = 0x82408A08;
	sub_822F13D0(ctx, base);
	// 82408A08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82408A0C: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82408A10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82408A14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82408A18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82408A1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82408A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82408A28 size=92
    let mut pc: u32 = 0x82408A28;
    'dispatch: loop {
        match pc {
            0x82408A28 => {
    //   block [0x82408A28..0x82408A84)
	// 82408A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82408A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82408A30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82408A34: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82408A38: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82408A3C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82408A40: 40980024  bge cr6, 0x82408a64
	if !ctx.cr[6].lt {
	pc = 0x82408A64; continue 'dispatch;
	}
	// 82408A44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82408A48: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 82408A4C: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 82408A50: 386BCCF4  addi r3, r11, -0x330c
	ctx.r[3].s64 = ctx.r[11].s64 + -13068;
	// 82408A54: 4BEAA52D  bl 0x822b2f80
	ctx.lr = 0x82408A58;
	sub_822B2F80(ctx, base);
	// 82408A58: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82408A5C: 6063002A  ori r3, r3, 0x2a
	ctx.r[3].u64 = ctx.r[3].u64 | 42;
	// 82408A60: 48000014  b 0x82408a74
	pc = 0x82408A74; continue 'dispatch;
	// 82408A64: 4BFFFA3D  bl 0x824084a0
	ctx.lr = 0x82408A68;
	sub_824084A0(ctx, base);
	// 82408A68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82408A6C: 4BFFE7ED  bl 0x82407258
	ctx.lr = 0x82408A70;
	sub_82407258(ctx, base);
	// 82408A70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82408A74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82408A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82408A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82408A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82408A88 size=344
    let mut pc: u32 = 0x82408A88;
    'dispatch: loop {
        match pc {
            0x82408A88 => {
    //   block [0x82408A88..0x82408BE0)
	// 82408A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82408A8C: 4812C61D  bl 0x825350a8
	ctx.lr = 0x82408A90;
	sub_82535080(ctx, base);
	// 82408A90: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 82408A94: 4812D555  bl 0x82535fe8
	ctx.lr = 0x82408A98;
	sub_82535FB0(ctx, base);
	// 82408A98: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82408A9C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82408AA0: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82408AA4: 3D400008  lis r10, 8
	ctx.r[10].s64 = 524288;
	// 82408AA8: FF801090  fmr f28, f2
	ctx.f[28].f64 = ctx.f[2].f64;
	// 82408AAC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82408AB0: 615E030C  ori r30, r10, 0x30c
	ctx.r[30].u64 = ctx.r[10].u64 | 780;
	// 82408AB4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82408AB8: 817C1C38  lwz r11, 0x1c38(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82408ABC: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82408AC0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82408AC4: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82408AC8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82408ACC: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82408AD0: 4BFFE139  bl 0x82406c08
	ctx.lr = 0x82408AD4;
	sub_82406C08(ctx, base);
	// 82408AD4: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82408AD8: 4082001C  bne 0x82408af4
	if !ctx.cr[0].eq {
	pc = 0x82408AF4; continue 'dispatch;
	}
	// 82408ADC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82408AE0: 386BCD30  addi r3, r11, -0x32d0
	ctx.r[3].s64 = ctx.r[11].s64 + -13008;
	// 82408AE4: 4BEAA49D  bl 0x822b2f80
	ctx.lr = 0x82408AE8;
	sub_822B2F80(ctx, base);
	// 82408AE8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82408AEC: 6063000F  ori r3, r3, 0xf
	ctx.r[3].u64 = ctx.r[3].u64 | 15;
	// 82408AF0: 480000E0  b 0x82408bd0
	pc = 0x82408BD0; continue 'dispatch;
	// 82408AF4: 817D001C  lwz r11, 0x1c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82408AF8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82408AFC: 41990014  bgt cr6, 0x82408b10
	if ctx.cr[6].gt {
	pc = 0x82408B10; continue 'dispatch;
	}
	// 82408B00: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82408B04: 4098000C  bge cr6, 0x82408b10
	if !ctx.cr[6].lt {
	pc = 0x82408B10; continue 'dispatch;
	}
	// 82408B08: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82408B0C: 480000C4  b 0x82408bd0
	pc = 0x82408BD0; continue 'dispatch;
	// 82408B10: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82408B14: 815C1C38  lwz r10, 0x1c38(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82408B18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82408B1C: 93410054  stw r26, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u32 ) };
	// 82408B20: 7C6AF214  add r3, r10, r30
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82408B24: C3EB1850  lfs f31, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82408B28: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82408B2C: D3E100C0  stfs f31, 0xc0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 82408B30: D3E100D0  stfs f31, 0xd0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 82408B34: D3E100D4  stfs f31, 0xd4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 82408B38: C3CB1FF8  lfs f30, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82408B3C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82408B40: D3C100C4  stfs f30, 0xc4(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82408B44: D3C100CC  stfs f30, 0xcc(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 82408B48: 916100C8  stw r11, 0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82408B4C: 8161019C  lwz r11, 0x19c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(412 as u32) ) } as u64;
	// 82408B50: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82408B54: 4BFFDEDD  bl 0x82406a30
	ctx.lr = 0x82408B58;
	sub_82406A30(ctx, base);
	// 82408B58: 816101A4  lwz r11, 0x1a4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(420 as u32) ) } as u64;
	// 82408B5C: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82408B60: 9361005C  stw r27, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 82408B64: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82408B68: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 82408B6C: 93A10064  stw r29, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[29].u32 ) };
	// 82408B70: 93210068  stw r25, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[25].u32 ) };
	// 82408B74: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82408B78: 409A0024  bne cr6, 0x82408b9c
	if !ctx.cr[6].eq {
	pc = 0x82408B9C; continue 'dispatch;
	}
	// 82408B7C: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 82408B80: 808101AC  lwz r4, 0x1ac(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(428 as u32) ) } as u64;
	// 82408B84: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82408B88: 4BFB8079  bl 0x823c0c00
	ctx.lr = 0x82408B8C;
	sub_823C0C00(ctx, base);
	// 82408B8C: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 82408B90: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82408B94: 808101B4  lwz r4, 0x1b4(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82408B98: 4BFB8069  bl 0x823c0c00
	ctx.lr = 0x82408B9C;
	sub_823C0C00(ctx, base);
	// 82408B9C: 81610194  lwz r11, 0x194(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(404 as u32) ) } as u64;
	// 82408BA0: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 82408BA4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82408BA8: D3A100C0  stfs f29, 0xc0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), tmp.u32 ) };
	// 82408BAC: D38100C4  stfs f28, 0xc4(r1)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 82408BB0: 807C1C50  lwz r3, 0x1c50(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82408BB4: D3C100CC  stfs f30, 0xcc(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 82408BB8: D3E100D4  stfs f31, 0xd4(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 82408BBC: D3E100D0  stfs f31, 0xd0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 82408BC0: 916100C8  stw r11, 0xc8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82408BC4: 80BD001C  lwz r5, 0x1c(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) } as u64;
	// 82408BC8: 48005BC9  bl 0x8240e790
	ctx.lr = 0x82408BCC;
	sub_8240E790(ctx, base);
	// 82408BCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82408BD0: 38210140  addi r1, r1, 0x140
	ctx.r[1].s64 = ctx.r[1].s64 + 320;
	// 82408BD4: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 82408BD8: 4812D45D  bl 0x82536034
	ctx.lr = 0x82408BDC;
	sub_82535FFC(ctx, base);
	// 82408BDC: 4812C51C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82408BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82408BE0 size=1476
    let mut pc: u32 = 0x82408BE0;
    'dispatch: loop {
        match pc {
            0x82408BE0 => {
    //   block [0x82408BE0..0x824091A4)
	// 82408BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82408BE4: 4812C4A1  bl 0x82535084
	ctx.lr = 0x82408BE8;
	sub_82535080(ctx, base);
	// 82408BE8: 3981FF70  addi r12, r1, -0x90
	ctx.r[12].s64 = ctx.r[1].s64 + -144;
	// 82408BEC: 4812D3E1  bl 0x82535fcc
	ctx.lr = 0x82408BF0;
	sub_82535FB0(ctx, base);
	// 82408BF0: 9421FAF0  stwu r1, -0x510(r1)
	ea = ctx.r[1].u32.wrapping_add(-1296 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82408BF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82408BF8: FEA00890  fmr f21, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[21].f64 = ctx.f[1].f64;
	// 82408BFC: 7D4F5378  mr r15, r10
	ctx.r[15].u64 = ctx.r[10].u64;
	// 82408C00: FEE01090  fmr f23, f2
	ctx.f[23].f64 = ctx.f[2].f64;
	// 82408C04: 3D400008  lis r10, 8
	ctx.r[10].s64 = 524288;
	// 82408C08: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82408C0C: 615E030C  ori r30, r10, 0x30c
	ctx.r[30].u64 = ctx.r[10].u64 | 780;
	// 82408C10: 817F1C38  lwz r11, 0x1c38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82408C14: 7CB02B78  mr r16, r5
	ctx.r[16].u64 = ctx.r[5].u64;
	// 82408C18: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82408C1C: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82408C20: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82408C24: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82408C28: 4BFFDFE1  bl 0x82406c08
	ctx.lr = 0x82408C2C;
	sub_82406C08(ctx, base);
	// 82408C2C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82408C30: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82408C34: 40820020  bne 0x82408c54
	if !ctx.cr[0].eq {
	pc = 0x82408C54; continue 'dispatch;
	}
	// 82408C38: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82408C3C: 7E048378  mr r4, r16
	ctx.r[4].u64 = ctx.r[16].u64;
	// 82408C40: 386BCDB8  addi r3, r11, -0x3248
	ctx.r[3].s64 = ctx.r[11].s64 + -12872;
	// 82408C44: 4BEAA33D  bl 0x822b2f80
	ctx.lr = 0x82408C48;
	sub_822B2F80(ctx, base);
	// 82408C48: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82408C4C: 6063000F  ori r3, r3, 0xf
	ctx.r[3].u64 = ctx.r[3].u64 | 15;
	// 82408C50: 48000544  b 0x82409194
	pc = 0x82409194; continue 'dispatch;
	// 82408C54: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82408C58: 815F1C38  lwz r10, 0x1c38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82408C5C: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 82408C60: 7C6AF214  add r3, r10, r30
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 82408C64: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82408C68: C2CB1850  lfs f22, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[22].f64 = (tmp.f32 as f64);
	// 82408C6C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82408C70: D2C101AC  stfs f22, 0x1ac(r1)
	tmp.f32 = (ctx.f[22].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(428 as u32), tmp.u32 ) };
	// 82408C74: 926101B4  stw r19, 0x1b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(436 as u32), ctx.r[19].u32 ) };
	// 82408C78: D2C101BC  stfs f22, 0x1bc(r1)
	tmp.f32 = (ctx.f[22].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(444 as u32), tmp.u32 ) };
	// 82408C7C: D2C101C0  stfs f22, 0x1c0(r1)
	tmp.f32 = (ctx.f[22].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(448 as u32), tmp.u32 ) };
	// 82408C80: C38B1FF8  lfs f28, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82408C84: D38101B0  stfs f28, 0x1b0(r1)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(432 as u32), tmp.u32 ) };
	// 82408C88: D38101B8  stfs f28, 0x1b8(r1)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(440 as u32), tmp.u32 ) };
	// 82408C8C: 4BFFDFBD  bl 0x82406c48
	ctx.lr = 0x82408C90;
	sub_82406C48(ctx, base);
	// 82408C90: 817F1C38  lwz r11, 0x1c38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82408C94: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82408C98: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82408C9C: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82408CA0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82408CA4: 4BFFDEFD  bl 0x82406ba0
	ctx.lr = 0x82408CA8;
	sub_82406BA0(ctx, base);
	// 82408CA8: 817F1C38  lwz r11, 0x1c38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82408CAC: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82408CB0: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82408CB4: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82408CB8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82408CBC: 4BFFDFC5  bl 0x82406c80
	ctx.lr = 0x82408CC0;
	sub_82406C80(ctx, base);
	// 82408CC0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82408CC4: 807B000C  lwz r3, 0xc(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82408CC8: 48006D09  bl 0x8240f9d0
	ctx.lr = 0x82408CCC;
	sub_8240F9D0(ctx, base);
	// 82408CCC: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82408CD0: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82408CD4: 807F1C40  lwz r3, 0x1c40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7232 as u32) ) } as u64;
	// 82408CD8: 48006B89  bl 0x8240f860
	ctx.lr = 0x82408CDC;
	sub_8240F860(ctx, base);
	// 82408CDC: 82E1056C  lwz r23, 0x56c(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1388 as u32) ) } as u64;
	// 82408CE0: 8221057C  lwz r17, 0x57c(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1404 as u32) ) } as u64;
	// 82408CE4: FF400890  fmr f26, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[26].f64 = ctx.f[1].f64;
	// 82408CE8: 82410574  lwz r18, 0x574(r1)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1396 as u32) ) } as u64;
	// 82408CEC: 2B170001  cmplwi cr6, r23, 1
	ctx.cr[6].compare_u32(ctx.r[23].u32, 1 as u32, &mut ctx.xer);
	// 82408CF0: 409A006C  bne cr6, 0x82408d5c
	if !ctx.cr[6].eq {
	pc = 0x82408D5C; continue 'dispatch;
	}
	// 82408CF4: 394101D0  addi r10, r1, 0x1d0
	ctx.r[10].s64 = ctx.r[1].s64 + 464;
	// 82408CF8: C07C0020  lfs f3, 0x20(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82408CFC: 7E258B78  mr r5, r17
	ctx.r[5].u64 = ctx.r[17].u64;
	// 82408D00: C03C0014  lfs f1, 0x14(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82408D04: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 82408D08: 80DC0024  lwz r6, 0x24(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(36 as u32) ) } as u64;
	// 82408D0C: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82408D10: FC40D090  fmr f2, f26
	ctx.f[2].f64 = ctx.f[26].f64;
	// 82408D14: 4BFFD70D  bl 0x82406420
	ctx.lr = 0x82408D18;
	sub_82406420(ctx, base);
	// 82408D18: 7C651B79  or. r5, r3, r3
	ctx.r[5].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82408D1C: 41820020  beq 0x82408d3c
	if ctx.cr[0].eq {
	pc = 0x82408D3C; continue 'dispatch;
	}
	// 82408D20: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82408D24: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82408D28: 386BCD70  addi r3, r11, -0x3290
	ctx.r[3].s64 = ctx.r[11].s64 + -12944;
	// 82408D2C: 4BEAA255  bl 0x822b2f80
	ctx.lr = 0x82408D30;
	sub_822B2F80(ctx, base);
	// 82408D30: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82408D34: 6063002D  ori r3, r3, 0x2d
	ctx.r[3].u64 = ctx.r[3].u64 | 45;
	// 82408D38: 4800045C  b 0x82409194
	pc = 0x82409194; continue 'dispatch;
	// 82408D3C: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82408D40: 807F1C3C  lwz r3, 0x1c3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82408D44: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82408D48: C021026C  lfs f1, 0x26c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(620 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82408D4C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82408D50: 80810564  lwz r4, 0x564(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1380 as u32) ) } as u64;
	// 82408D54: 48002FDD  bl 0x8240bd30
	ctx.lr = 0x82408D58;
	sub_8240BD30(ctx, base);
	// 82408D58: 4800001C  b 0x82408d74
	pc = 0x82408D74; continue 'dispatch;
	// 82408D5C: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82408D60: 807F1C3C  lwz r3, 0x1c3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82408D64: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82408D68: 80810564  lwz r4, 0x564(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1380 as u32) ) } as u64;
	// 82408D6C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82408D70: 48002FB1  bl 0x8240bd20
	ctx.lr = 0x82408D74;
	sub_8240BD20(ctx, base);
	// 82408D74: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82408D78: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82408D7C: 40980008  bge cr6, 0x82408d84
	if !ctx.cr[6].lt {
	pc = 0x82408D84; continue 'dispatch;
	}
	// 82408D80: 48000414  b 0x82409194
	pc = 0x82409194; continue 'dispatch;
	// 82408D84: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82408D88: 807F1C3C  lwz r3, 0x1c3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82408D8C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82408D90: 480025D1  bl 0x8240b360
	ctx.lr = 0x82408D94;
	sub_8240B360(ctx, base);
	// 82408D94: 38810280  addi r4, r1, 0x280
	ctx.r[4].s64 = ctx.r[1].s64 + 640;
	// 82408D98: 82BB000C  lwz r21, 0xc(r27)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82408D9C: 829B0020  lwz r20, 0x20(r27)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82408DA0: 837B0024  lwz r27, 0x24(r27)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82408DA4: 807C0030  lwz r3, 0x30(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(48 as u32) ) } as u64;
	// 82408DA8: 4BFFE4F1  bl 0x82407298
	ctx.lr = 0x82408DAC;
	sub_82407298(ctx, base);
	// 82408DAC: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82408DB0: 408203E4  bne 0x82409194
	if !ctx.cr[0].eq {
	pc = 0x82409194; continue 'dispatch;
	}
	// 82408DB4: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82408DB8: 807F1C40  lwz r3, 0x1c40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7232 as u32) ) } as u64;
	// 82408DBC: 4800695D  bl 0x8240f718
	ctx.lr = 0x82408DC0;
	sub_8240F718(ctx, base);
	// 82408DC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82408DC4: FF600890  fmr f27, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[27].f64 = ctx.f[1].f64;
	// 82408DC8: 4BFFE5E1  bl 0x824073a8
	ctx.lr = 0x82408DCC;
	sub_824073A8(ctx, base);
	// 82408DCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82408DD0: 4BFFF679  bl 0x82408448
	ctx.lr = 0x82408DD4;
	sub_82408448(ctx, base);
	// 82408DD4: FF01E000  fcmpu cr6, f1, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[28].f64);
	// 82408DD8: 409900FC  ble cr6, 0x82408ed4
	if !ctx.cr[6].gt {
	pc = 0x82408ED4; continue 'dispatch;
	}
	// 82408DDC: C01F1C1C  lfs f0, 0x1c1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7196 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82408DE0: C1A10280  lfs f13, 0x280(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(640 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82408DE4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82408DE8: 409900EC  ble cr6, 0x82408ed4
	if !ctx.cr[6].gt {
	pc = 0x82408ED4; continue 'dispatch;
	}
	// 82408DEC: FF15E000  fcmpu cr6, f21, f28
	ctx.cr[6].compare_f64(ctx.f[21].f64, ctx.f[28].f64);
	// 82408DF0: 409900E4  ble cr6, 0x82408ed4
	if !ctx.cr[6].gt {
	pc = 0x82408ED4; continue 'dispatch;
	}
	// 82408DF4: 807C0030  lwz r3, 0x30(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(48 as u32) ) } as u64;
	// 82408DF8: 4BFFE1B9  bl 0x82406fb0
	ctx.lr = 0x82408DFC;
	sub_82406FB0(ctx, base);
	// 82408DFC: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82408E00: 419A00D4  beq cr6, 0x82408ed4
	if ctx.cr[6].eq {
	pc = 0x82408ED4; continue 'dispatch;
	}
	// 82408E04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82408E08: FC20A890  fmr f1, f21
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[21].f64;
	// 82408E0C: 4BFFF695  bl 0x824084a0
	ctx.lr = 0x82408E10;
	sub_824084A0(ctx, base);
	// 82408E10: FC400890  fmr f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[1].f64;
	// 82408E14: C0BF1C24  lfs f5, 0x1c24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7204 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 82408E18: 807F1C40  lwz r3, 0x1c40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7232 as u32) ) } as u64;
	// 82408E1C: C09C0064  lfs f4, 0x64(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(100 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82408E20: FC20D890  fmr f1, f27
	ctx.f[1].f64 = ctx.f[27].f64;
	// 82408E24: C0610280  lfs f3, 0x280(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(640 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82408E28: 480065F1  bl 0x8240f418
	ctx.lr = 0x82408E2C;
	sub_8240F418(ctx, base);
	// 82408E2C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82408E30: 2B170001  cmplwi cr6, r23, 1
	ctx.cr[6].compare_u32(ctx.r[23].u32, 1 as u32, &mut ctx.xer);
	// 82408E34: 409A006C  bne cr6, 0x82408ea0
	if !ctx.cr[6].eq {
	pc = 0x82408EA0; continue 'dispatch;
	}
	// 82408E38: 817F1C4C  lwz r11, 0x1c4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7244 as u32) ) } as u64;
	// 82408E3C: C0210274  lfs f1, 0x274(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(628 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82408E40: 83D20030  lwz r30, 0x30(r18)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(48 as u32) ) } as u64;
	// 82408E44: 3BAB0CBC  addi r29, r11, 0xcbc
	ctx.r[29].s64 = ctx.r[11].s64 + 3260;
	// 82408E48: 480056C9  bl 0x8240e510
	ctx.lr = 0x82408E4C;
	sub_8240E510(ctx, base);
	// 82408E4C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82408E50: EFE1F82A  fadds f31, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ((ctx.f[1].f64 + ctx.f[31].f64) as f32) as f64;
	// 82408E54: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82408E58: 409A0048  bne cr6, 0x82408ea0
	if !ctx.cr[6].eq {
	pc = 0x82408EA0; continue 'dispatch;
	}
	// 82408E5C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82408E60: 41980040  blt cr6, 0x82408ea0
	if ctx.cr[6].lt {
	pc = 0x82408EA0; continue 'dispatch;
	}
	// 82408E64: 817F1C4C  lwz r11, 0x1c4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7244 as u32) ) } as u64;
	// 82408E68: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82408E6C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82408E70: 40980030  bge cr6, 0x82408ea0
	if !ctx.cr[6].lt {
	pc = 0x82408EA0; continue 'dispatch;
	}
	// 82408E74: 57CB2834  slwi r11, r30, 5
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82408E78: 807F1C48  lwz r3, 0x1c48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7240 as u32) ) } as u64;
	// 82408E7C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82408E80: C0210278  lfs f1, 0x278(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(632 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82408E84: 7CABEA14  add r5, r11, r29
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 82408E88: C045001C  lfs f2, 0x1c(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82408E8C: 48006A45  bl 0x8240f8d0
	ctx.lr = 0x82408E90;
	sub_8240F8D0(ctx, base);
	// 82408E90: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82408E94: 4082000C  bne 0x82408ea0
	if !ctx.cr[0].eq {
	pc = 0x82408EA0; continue 'dispatch;
	}
	// 82408E98: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82408E9C: EFE0F82A  fadds f31, f0, f31
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 82408EA0: C01F1C18  lfs f0, 0x1c18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7192 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82408EA4: EFDF0028  fsubs f30, f31, f0
	ctx.f[30].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 82408EA8: C1A10280  lfs f13, 0x280(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(640 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82408EAC: EFBF6828  fsubs f29, f31, f13
	ctx.f[29].f64 = (((ctx.f[31].f64 - ctx.f[13].f64) as f32) as f64);
	// 82408EB0: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82408EB4: 480056A5  bl 0x8240e558
	ctx.lr = 0x82408EB8;
	sub_8240E558(ctx, base);
	// 82408EB8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82408EBC: 809C0050  lwz r4, 0x50(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(80 as u32) ) } as u64;
	// 82408EC0: 807F1C40  lwz r3, 0x1c40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7232 as u32) ) } as u64;
	// 82408EC4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82408EC8: 480065F9  bl 0x8240f4c0
	ctx.lr = 0x82408ECC;
	sub_8240F4C0(ctx, base);
	// 82408ECC: EF2107F2  fmuls f25, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[25].f64 = (((ctx.f[1].f64 * ctx.f[31].f64) as f32) as f64);
	// 82408ED0: 48000010  b 0x82408ee0
	pc = 0x82408EE0; continue 'dispatch;
	// 82408ED4: C3DF1C1C  lfs f30, 0x1c1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7196 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82408ED8: FF20E090  fmr f25, f28
	ctx.f[25].f64 = ctx.f[28].f64;
	// 82408EDC: FFA0F090  fmr f29, f30
	ctx.f[29].f64 = ctx.f[30].f64;
	// 82408EE0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82408EE4: 807F1C40  lwz r3, 0x1c40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7232 as u32) ) } as u64;
	// 82408EE8: D361007C  stfs f27, 0x7c(r1)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82408EEC: D3A100A0  stfs f29, 0xa0(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82408EF0: 48006879  bl 0x8240f768
	ctx.lr = 0x82408EF4;
	sub_8240F768(ctx, base);
	// 82408EF4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82408EF8: 2B170001  cmplwi cr6, r23, 1
	ctx.cr[6].compare_u32(ctx.r[23].u32, 1 as u32, &mut ctx.xer);
	// 82408EFC: FC40E090  fmr f2, f28
	ctx.f[2].f64 = ctx.f[28].f64;
	// 82408F00: 409A0010  bne cr6, 0x82408f10
	if !ctx.cr[6].eq {
	pc = 0x82408F10; continue 'dispatch;
	}
	// 82408F04: C0210270  lfs f1, 0x270(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(624 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82408F08: 48005759  bl 0x8240e660
	ctx.lr = 0x82408F0C;
	sub_8240E660(ctx, base);
	// 82408F0C: FC400890  fmr f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[1].f64;
	// 82408F10: 7DE57B78  mr r5, r15
	ctx.r[5].u64 = ctx.r[15].u64;
	// 82408F14: 80C10288  lwz r6, 0x288(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(648 as u32) ) } as u64;
	// 82408F18: C07C0078  lfs f3, 0x78(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(120 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82408F1C: 807F1C40  lwz r3, 0x1c40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7232 as u32) ) } as u64;
	// 82408F20: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82408F24: 4800650D  bl 0x8240f430
	ctx.lr = 0x82408F28;
	sub_8240F430(ctx, base);
	// 82408F28: D3E10084  stfs f31, 0x84(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 82408F2C: FF000890  fmr f24, f1
	ctx.f[24].f64 = ctx.f[1].f64;
	// 82408F30: 2B170001  cmplwi cr6, r23, 1
	ctx.cr[6].compare_u32(ctx.r[23].u32, 1 as u32, &mut ctx.xer);
	// 82408F34: 409A0060  bne cr6, 0x82408f94
	if !ctx.cr[6].eq {
	pc = 0x82408F94; continue 'dispatch;
	}
	// 82408F38: 3BC101DC  addi r30, r1, 0x1dc
	ctx.r[30].s64 = ctx.r[1].s64 + 476;
	// 82408F3C: 3BA00006  li r29, 6
	ctx.r[29].s64 = 6;
	// 82408F40: C03E0000  lfs f1, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82408F44: 480055CD  bl 0x8240e510
	ctx.lr = 0x82408F48;
	sub_8240E510(ctx, base);
	// 82408F48: C01C0008  lfs f0, 8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82408F4C: EC00F02A  fadds f0, f0, f30
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[30].f64) as f32) as f64;
	// 82408F50: EC20082A  fadds f1, f0, f1
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 82408F54: 48005605  bl 0x8240e558
	ctx.lr = 0x82408F58;
	sub_8240E558(ctx, base);
	// 82408F58: D03E0000  stfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82408F5C: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82408F60: 3BDE0018  addi r30, r30, 0x18
	ctx.r[30].s64 = ctx.r[30].s64 + 24;
	// 82408F64: 4082FFDC  bne 0x82408f40
	if !ctx.cr[0].eq {
	pc = 0x82408F40; continue 'dispatch;
	}
	// 82408F68: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 82408F6C: 388101D0  addi r4, r1, 0x1d0
	ctx.r[4].s64 = ctx.r[1].s64 + 464;
	// 82408F70: 38610390  addi r3, r1, 0x390
	ctx.r[3].s64 = ctx.r[1].s64 + 912;
	// 82408F74: 4812C065  bl 0x82534fd8
	ctx.lr = 0x82408F78;
	sub_82534FD8(ctx, base);
	// 82408F78: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82408F7C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82408F80: 388101D0  addi r4, r1, 0x1d0
	ctx.r[4].s64 = ctx.r[1].s64 + 464;
	// 82408F84: 38A000AC  li r5, 0xac
	ctx.r[5].s64 = 172;
	// 82408F88: 91610094  stw r11, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82408F8C: 4812BBC5  bl 0x82534b50
	ctx.lr = 0x82408F90;
	sub_82534B50(ctx, base);
	// 82408F90: 48000068  b 0x82408ff8
	pc = 0x82408FF8; continue 'dispatch;
	// 82408F94: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82408F98: 807F1C40  lwz r3, 0x1c40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7232 as u32) ) } as u64;
	// 82408F9C: 48006875  bl 0x8240f810
	ctx.lr = 0x82408FA0;
	sub_8240F810(ctx, base);
	// 82408FA0: 807F1C40  lwz r3, 0x1c40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7232 as u32) ) } as u64;
	// 82408FA4: C0610284  lfs f3, 0x284(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(644 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82408FA8: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82408FAC: FC40B890  fmr f2, f23
	ctx.f[2].f64 = ctx.f[23].f64;
	// 82408FB0: 48006501  bl 0x8240f4b0
	ctx.lr = 0x82408FB4;
	sub_8240F4B0(ctx, base);
	// 82408FB4: C01C0008  lfs f0, 8(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82408FB8: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82408FBC: EC20F02A  fadds f1, f0, f30
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[30].f64) as f32) as f64;
	// 82408FC0: 48005599  bl 0x8240e558
	ctx.lr = 0x82408FC4;
	sub_8240E558(ctx, base);
	// 82408FC4: 39210390  addi r9, r1, 0x390
	ctx.r[9].s64 = ctx.r[1].s64 + 912;
	// 82408FC8: FCA00890  fmr f5, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[5].f64 = ctx.f[1].f64;
	// 82408FCC: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 82408FD0: C09C0020  lfs f4, 0x20(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82408FD4: C05C0014  lfs f2, 0x14(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82408FD8: FC60D090  fmr f3, f26
	ctx.f[3].f64 = ctx.f[26].f64;
	// 82408FDC: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82408FE0: 4BFFD349  bl 0x82406328
	ctx.lr = 0x82408FE4;
	sub_82406328(ctx, base);
	// 82408FE4: D3E10080  stfs f31, 0x80(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), tmp.u32 ) };
	// 82408FE8: 92610094  stw r19, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[19].u32 ) };
	// 82408FEC: D3410088  stfs f26, 0x88(r1)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), tmp.u32 ) };
	// 82408FF0: C01C0020  lfs f0, 0x20(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82408FF4: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82408FF8: 807C0038  lwz r3, 0x38(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 82408FFC: 4BFFE3FD  bl 0x824073f8
	ctx.lr = 0x82409000;
	sub_824073F8(ctx, base);
	// 82409000: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82409004: C01F1C1C  lfs f0, 0x1c1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7196 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409008: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8240900C: 40990064  ble cr6, 0x82409070
	if !ctx.cr[6].gt {
	pc = 0x82409070; continue 'dispatch;
	}
	// 82409010: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 82409014: 4099005C  ble cr6, 0x82409070
	if !ctx.cr[6].gt {
	pc = 0x82409070; continue 'dispatch;
	}
	// 82409018: 807C0038  lwz r3, 0x38(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 8240901C: 4BFFDFF5  bl 0x82407010
	ctx.lr = 0x82409020;
	sub_82407010(ctx, base);
	// 82409020: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82409024: 419A004C  beq cr6, 0x82409070
	if ctx.cr[6].eq {
	pc = 0x82409070; continue 'dispatch;
	}
	// 82409028: C01C003C  lfs f0, 0x3c(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240902C: 2B170001  cmplwi cr6, r23, 1
	ctx.cr[6].compare_u32(ctx.r[23].u32, 1 as u32, &mut ctx.xer);
	// 82409030: EFE0F82A  fadds f31, f0, f31
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 82409034: 409A000C  bne cr6, 0x82409040
	if !ctx.cr[6].eq {
	pc = 0x82409040; continue 'dispatch;
	}
	// 82409038: C0010260  lfs f0, 0x260(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(608 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240903C: EFE0F82A  fadds f31, f0, f31
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 82409040: FF1FE000  fcmpu cr6, f31, f28
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[28].f64);
	// 82409044: 40990008  ble cr6, 0x8240904c
	if !ctx.cr[6].gt {
	pc = 0x8240904C; continue 'dispatch;
	}
	// 82409048: FFE0E090  fmr f31, f28
	ctx.f[31].f64 = ctx.f[28].f64;
	// 8240904C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82409050: 48005509  bl 0x8240e558
	ctx.lr = 0x82409054;
	sub_8240E558(ctx, base);
	// 82409054: FF600890  fmr f27, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[27].f64 = ctx.f[1].f64;
	// 82409058: FF1EE000  fcmpu cr6, f30, f28
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[28].f64);
	// 8240905C: 4098000C  bge cr6, 0x82409068
	if !ctx.cr[6].lt {
	pc = 0x82409068; continue 'dispatch;
	}
	// 82409060: EC3FF02A  fadds f1, f31, f30
	ctx.f[1].f64 = ((ctx.f[31].f64 + ctx.f[30].f64) as f32) as f64;
	// 82409064: 48000014  b 0x82409078
	pc = 0x82409078; continue 'dispatch;
	// 82409068: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240906C: 4800000C  b 0x82409078
	pc = 0x82409078; continue 'dispatch;
	// 82409070: FF60E090  fmr f27, f28
	ctx.f[27].f64 = ctx.f[28].f64;
	// 82409074: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 82409078: 480054E1  bl 0x8240e558
	ctx.lr = 0x8240907C;
	sub_8240E558(ctx, base);
	// 8240907C: 817C004C  lwz r11, 0x4c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(76 as u32) ) } as u64;
	// 82409080: 83DC0038  lwz r30, 0x38(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 82409084: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 82409088: D2A101AC  stfs f21, 0x1ac(r1)
	tmp.f32 = (ctx.f[21].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(428 as u32), tmp.u32 ) };
	// 8240908C: 2B170001  cmplwi cr6, r23, 1
	ctx.cr[6].compare_u32(ctx.r[23].u32, 1 as u32, &mut ctx.xer);
	// 82409090: D2E101B0  stfs f23, 0x1b0(r1)
	tmp.f32 = (ctx.f[23].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(432 as u32), tmp.u32 ) };
	// 82409094: 92010064  stw r16, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[16].u32 ) };
	// 82409098: D38101B8  stfs f28, 0x1b8(r1)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(440 as u32), tmp.u32 ) };
	// 8240909C: 557DFFFE  rlwinm r29, r11, 0x1f, 0x1f, 0x1f
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	// 824090A0: D2C101C0  stfs f22, 0x1c0(r1)
	tmp.f32 = (ctx.f[22].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(448 as u32), tmp.u32 ) };
	// 824090A4: 91E101B4  stw r15, 0x1b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(436 as u32), ctx.r[15].u32 ) };
	// 824090A8: D2C101BC  stfs f22, 0x1bc(r1)
	tmp.f32 = (ctx.f[22].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(444 as u32), tmp.u32 ) };
	// 824090AC: 409A0024  bne cr6, 0x824090d0
	if !ctx.cr[6].eq {
	pc = 0x824090D0; continue 'dispatch;
	}
	// 824090B0: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 824090B4: 7E449378  mr r4, r18
	ctx.r[4].u64 = ctx.r[18].u64;
	// 824090B8: 3861015C  addi r3, r1, 0x15c
	ctx.r[3].s64 = ctx.r[1].s64 + 348;
	// 824090BC: 4BFB7B45  bl 0x823c0c00
	ctx.lr = 0x824090C0;
	sub_823C0C00(ctx, base);
	// 824090C0: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 824090C4: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 824090C8: 38610190  addi r3, r1, 0x190
	ctx.r[3].s64 = ctx.r[1].s64 + 400;
	// 824090CC: 4BFB7B35  bl 0x823c0c00
	ctx.lr = 0x824090D0;
	sub_823C0C00(ctx, base);
	// 824090D0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824090D4: 807F1C3C  lwz r3, 0x1c3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7228 as u32) ) } as u64;
	// 824090D8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 824090DC: D321009C  stfs f25, 0x9c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[25].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 824090E0: D30100A4  stfs f24, 0xa4(r1)
	tmp.f32 = (ctx.f[24].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 824090E4: 92C10098  stw r22, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[22].u32 ) };
	// 824090E8: 48002221  bl 0x8240b308
	ctx.lr = 0x824090EC;
	sub_8240B308(ctx, base);
	// 824090EC: C3DC0028  lfs f30, 0x28(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 824090F0: C3FC002C  lfs f31, 0x2c(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(44 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824090F4: 2B170001  cmplwi cr6, r23, 1
	ctx.cr[6].compare_u32(ctx.r[23].u32, 1 as u32, &mut ctx.xer);
	// 824090F8: 409A0024  bne cr6, 0x8240911c
	if !ctx.cr[6].eq {
	pc = 0x8240911C; continue 'dispatch;
	}
	// 824090FC: C0010264  lfs f0, 0x264(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(612 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409100: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 82409104: 40990008  ble cr6, 0x8240910c
	if !ctx.cr[6].gt {
	pc = 0x8240910C; continue 'dispatch;
	}
	// 82409108: FFC00090  fmr f30, f0
	ctx.f[30].f64 = ctx.f[0].f64;
	// 8240910C: C0010268  lfs f0, 0x268(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(616 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409110: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82409114: 40990008  ble cr6, 0x8240911c
	if !ctx.cr[6].gt {
	pc = 0x8240911C; continue 'dispatch;
	}
	// 82409118: FFE00090  fmr f31, f0
	ctx.f[31].f64 = ctx.f[0].f64;
	// 8240911C: FF1EB000  fcmpu cr6, f30, f22
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[22].f64);
	// 82409120: 40990008  ble cr6, 0x82409128
	if !ctx.cr[6].gt {
	pc = 0x82409128; continue 'dispatch;
	}
	// 82409124: FFC0B090  fmr f30, f22
	ctx.f[30].f64 = ctx.f[22].f64;
	// 82409128: FF1FB000  fcmpu cr6, f31, f22
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[22].f64);
	// 8240912C: 40990008  ble cr6, 0x82409134
	if !ctx.cr[6].gt {
	pc = 0x82409134; continue 'dispatch;
	}
	// 82409130: FFE0B090  fmr f31, f22
	ctx.f[31].f64 = ctx.f[22].f64;
	// 82409134: 83E10584  lwz r31, 0x584(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(1412 as u32) ) } as u64;
	// 82409138: FC20C890  fmr f1, f25
	ctx.f[1].f64 = ctx.f[25].f64;
	// 8240913C: D33F0000  stfs f25, 0(r31)
	tmp.f32 = (ctx.f[25].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82409140: 480053D1  bl 0x8240e510
	ctx.lr = 0x82409144;
	sub_8240E510(ctx, base);
	// 82409144: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82409148: D03F0004  stfs f1, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240914C: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 82409150: D31F0008  stfs f24, 8(r31)
	tmp.f32 = (ctx.f[24].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82409154: 38810390  addi r4, r1, 0x390
	ctx.r[4].s64 = ctx.r[1].s64 + 912;
	// 82409158: D37F009C  stfs f27, 0x9c(r31)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8240915C: D3BF00A0  stfs f29, 0xa0(r31)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82409160: 927F00A4  stw r19, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[19].u32 ) };
	// 82409164: D3DF00A8  stfs f30, 0xa8(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 82409168: D3FF00AC  stfs f31, 0xac(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 8240916C: 4812BE6D  bl 0x82534fd8
	ctx.lr = 0x82409170;
	sub_82534FD8(ctx, base);
	// 82409170: 92BF00B0  stw r21, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[21].u32 ) };
	// 82409174: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82409178: 93BF00B4  stw r29, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[29].u32 ) };
	// 8240917C: 929F00B8  stw r20, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[20].u32 ) };
	// 82409180: 937F00BC  stw r27, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[27].u32 ) };
	// 82409184: 93DF00C0  stw r30, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 82409188: 921F00C4  stw r16, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[16].u32 ) };
	// 8240918C: 935F00C8  stw r26, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[26].u32 ) };
	// 82409190: 933F00CC  stw r25, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[25].u32 ) };
	// 82409194: 38210510  addi r1, r1, 0x510
	ctx.r[1].s64 = ctx.r[1].s64 + 1296;
	// 82409198: 3981FF70  addi r12, r1, -0x90
	ctx.r[12].s64 = ctx.r[1].s64 + -144;
	// 8240919C: 4812CE7D  bl 0x82536018
	ctx.lr = 0x824091A0;
	sub_82535FFC(ctx, base);
	// 824091A0: 4812BF34  b 0x825350d4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824091A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824091A8 size=520
    let mut pc: u32 = 0x824091A8;
    'dispatch: loop {
        match pc {
            0x824091A8 => {
    //   block [0x824091A8..0x824093B0)
	// 824091A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824091AC: 4812BEFD  bl 0x825350a8
	ctx.lr = 0x824091B0;
	sub_82535080(ctx, base);
	// 824091B0: DBC1FFA8  stfd f30, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[30].u64 ) };
	// 824091B4: DBE1FFB0  stfd f31, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 824091B8: 9421FD10  stwu r1, -0x2f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-752 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824091BC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824091C0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 824091C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824091C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824091CC: 7D3B4B78  mr r27, r9
	ctx.r[27].u64 = ctx.r[9].u64;
	// 824091D0: C3EB1850  lfs f31, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824091D4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824091D8: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 824091DC: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 824091E0: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 824091E4: 7F38CB78  mr r24, r25
	ctx.r[24].u64 = ctx.r[25].u64;
	// 824091E8: D3E10064  stfs f31, 0x64(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 824091EC: 93210058  stw r25, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[25].u32 ) };
	// 824091F0: C3CB1FF8  lfs f30, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 824091F4: D3C10054  stfs f30, 0x54(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 824091F8: FF01F000  fcmpu cr6, f1, f30
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[30].f64);
	// 824091FC: D3C1005C  stfs f30, 0x5c(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82409200: 40980024  bge cr6, 0x82409224
	if !ctx.cr[6].lt {
	pc = 0x82409224; continue 'dispatch;
	}
	// 82409204: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409208: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8240920C: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 82409210: 386BCE90  addi r3, r11, -0x3170
	ctx.r[3].s64 = ctx.r[11].s64 + -12656;
	// 82409214: 4BEA9D6D  bl 0x822b2f80
	ctx.lr = 0x82409218;
	sub_822B2F80(ctx, base);
	// 82409218: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240921C: 60630018  ori r3, r3, 0x18
	ctx.r[3].u64 = ctx.r[3].u64 | 24;
	// 82409220: 48000160  b 0x82409380
	pc = 0x82409380; continue 'dispatch;
	// 82409224: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82409228: 409A0020  bne cr6, 0x82409248
	if !ctx.cr[6].eq {
	pc = 0x82409248; continue 'dispatch;
	}
	// 8240922C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409230: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82409234: 386BCE50  addi r3, r11, -0x31b0
	ctx.r[3].s64 = ctx.r[11].s64 + -12720;
	// 82409238: 4BEA9D49  bl 0x822b2f80
	ctx.lr = 0x8240923C;
	sub_822B2F80(ctx, base);
	// 8240923C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82409240: 60630012  ori r3, r3, 0x12
	ctx.r[3].u64 = ctx.r[3].u64 | 18;
	// 82409244: 4800013C  b 0x82409380
	pc = 0x82409380; continue 'dispatch;
	// 82409248: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 8240924C: D0210050  stfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82409250: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82409254: D0410054  stfs f2, 0x54(r1)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82409258: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240925C: D061005C  stfs f3, 0x5c(r1)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82409260: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82409264: 4BFFF0E5  bl 0x82408348
	ctx.lr = 0x82409268;
	sub_82408348(ctx, base);
	// 82409268: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 8240926C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82409270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82409274: 4BFFF155  bl 0x824083c8
	ctx.lr = 0x82409278;
	sub_824083C8(ctx, base);
	// 82409278: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8240927C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82409280: 807F1C50  lwz r3, 0x1c50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82409284: 4800562D  bl 0x8240e8b0
	ctx.lr = 0x82409288;
	sub_8240E8B0(ctx, base);
	// 82409288: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8240928C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82409290: 807F1C50  lwz r3, 0x1c50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82409294: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82409298: 48005669  bl 0x8240e900
	ctx.lr = 0x8240929C;
	sub_8240E900(ctx, base);
	// 8240929C: 7F3DCB78  mr r29, r25
	ctx.r[29].u64 = ctx.r[25].u64;
	// 824092A0: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	// 824092A4: 807F1C3C  lwz r3, 0x1c3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7228 as u32) ) } as u64;
	// 824092A8: 7D7A182E  lwzx r11, r26, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 824092AC: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 824092B0: 409A00AC  bne cr6, 0x8240935c
	if !ctx.cr[6].eq {
	pc = 0x8240935C; continue 'dispatch;
	}
	// 824092B4: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 824092B8: D3E101BC  stfs f31, 0x1bc(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(444 as u32), tmp.u32 ) };
	// 824092BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824092C0: D3C101C0  stfs f30, 0x1c0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(448 as u32), tmp.u32 ) };
	// 824092C4: D3C101C8  stfs f30, 0x1c8(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(456 as u32), tmp.u32 ) };
	// 824092C8: 932101C4  stw r25, 0x1c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(452 as u32), ctx.r[25].u32 ) };
	// 824092CC: D3E101CC  stfs f31, 0x1cc(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(460 as u32), tmp.u32 ) };
	// 824092D0: D3E101D0  stfs f31, 0x1d0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(464 as u32), tmp.u32 ) };
	// 824092D4: 4800208D  bl 0x8240b360
	ctx.lr = 0x824092D8;
	sub_8240B360(ctx, base);
	// 824092D8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 824092DC: 419A005C  beq cr6, 0x82409338
	if ctx.cr[6].eq {
	pc = 0x82409338; continue 'dispatch;
	}
	// 824092E0: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 824092E4: 419A0054  beq cr6, 0x82409338
	if ctx.cr[6].eq {
	pc = 0x82409338; continue 'dispatch;
	}
	// 824092E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824092EC: 807F1C3C  lwz r3, 0x1c3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7228 as u32) ) } as u64;
	// 824092F0: 48001FD1  bl 0x8240b2c0
	ctx.lr = 0x824092F4;
	sub_8240B2C0(ctx, base);
	// 824092F4: 7C6B1B79  or. r11, r3, r3
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824092F8: 41820098  beq 0x82409390
	if ctx.cr[0].eq {
	pc = 0x82409390; continue 'dispatch;
	}
	// 824092FC: 394101E0  addi r10, r1, 0x1e0
	ctx.r[10].s64 = ctx.r[1].s64 + 480;
	// 82409300: C06B0020  lfs f3, 0x20(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82409304: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82409308: C02B0014  lfs f1, 0x14(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240930C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82409310: 806100A8  lwz r3, 0xa8(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 82409314: 80CB0024  lwz r6, 0x24(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82409318: C0410098  lfs f2, 0x98(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8240931C: 4BFFD105  bl 0x82406420
	ctx.lr = 0x82409320;
	sub_82406420(ctx, base);
	// 82409320: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82409324: 40820074  bne 0x82409398
	if !ctx.cr[0].eq {
	pc = 0x82409398; continue 'dispatch;
	}
	// 82409328: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 8240932C: 388101E0  addi r4, r1, 0x1e0
	ctx.r[4].s64 = ctx.r[1].s64 + 480;
	// 82409330: 38A000AC  li r5, 0xac
	ctx.r[5].s64 = 172;
	// 82409334: 4812B81D  bl 0x82534b50
	ctx.lr = 0x82409338;
	sub_82534B50(ctx, base);
	// 82409338: 386101BC  addi r3, r1, 0x1bc
	ctx.r[3].s64 = ctx.r[1].s64 + 444;
	// 8240933C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82409340: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82409344: 4812B80D  bl 0x82534b50
	ctx.lr = 0x82409348;
	sub_82534B50(ctx, base);
	// 82409348: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8240934C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82409350: 807F1C3C  lwz r3, 0x1c3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82409354: 48001FB5  bl 0x8240b308
	ctx.lr = 0x82409358;
	sub_8240B308(ctx, base);
	// 82409358: 3B180001  addi r24, r24, 1
	ctx.r[24].s64 = ctx.r[24].s64 + 1;
	// 8240935C: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 82409360: 3B5A0164  addi r26, r26, 0x164
	ctx.r[26].s64 = ctx.r[26].s64 + 356;
	// 82409364: 616B0B00  ori r11, r11, 0xb00
	ctx.r[11].u64 = ctx.r[11].u64 | 2816;
	// 82409368: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8240936C: 7F1A5800  cmpw cr6, r26, r11
	ctx.cr[6].compare_i32(ctx.r[26].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82409370: 4198FF34  blt cr6, 0x824092a4
	if ctx.cr[6].lt {
	pc = 0x824092A4; continue 'dispatch;
	}
	// 82409374: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82409378: 419AFEC4  beq cr6, 0x8240923c
	if ctx.cr[6].eq {
	pc = 0x8240923C; continue 'dispatch;
	}
	// 8240937C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82409380: 382102F0  addi r1, r1, 0x2f0
	ctx.r[1].s64 = ctx.r[1].s64 + 752;
	// 82409384: CBC1FFA8  lfd f30, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 82409388: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8240938C: 4812BD6C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
	// 82409390: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82409394: 4BFFFFEC  b 0x82409380
	pc = 0x82409380; continue 'dispatch;
	// 82409398: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240939C: 386BCE10  addi r3, r11, -0x31f0
	ctx.r[3].s64 = ctx.r[11].s64 + -12784;
	// 824093A0: 4BEA9BE1  bl 0x822b2f80
	ctx.lr = 0x824093A4;
	sub_822B2F80(ctx, base);
	// 824093A4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 824093A8: 6063002D  ori r3, r3, 0x2d
	ctx.r[3].u64 = ctx.r[3].u64 | 45;
	// 824093AC: 4BFFFFD4  b 0x82409380
	pc = 0x82409380; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824093B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824093B0 size=1108
    let mut pc: u32 = 0x824093B0;
    'dispatch: loop {
        match pc {
            0x824093B0 => {
    //   block [0x824093B0..0x82409804)
	// 824093B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824093B4: 4812BCF9  bl 0x825350ac
	ctx.lr = 0x824093B8;
	sub_82535080(ctx, base);
	// 824093B8: 3981FFC0  addi r12, r1, -0x40
	ctx.r[12].s64 = ctx.r[1].s64 + -64;
	// 824093BC: 4812CC29  bl 0x82535fe4
	ctx.lr = 0x824093C0;
	sub_82535FB0(ctx, base);
	// 824093C0: 9421FC20  stwu r1, -0x3e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-992 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824093C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824093C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824093CC: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 824093D0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 824093D4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824093D8: C36B1850  lfs f27, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 824093DC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824093E0: 807E1C3C  lwz r3, 0x1c3c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7228 as u32) ) } as u64;
	// 824093E4: D36101AC  stfs f27, 0x1ac(r1)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(428 as u32), tmp.u32 ) };
	// 824093E8: D36101BC  stfs f27, 0x1bc(r1)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(444 as u32), tmp.u32 ) };
	// 824093EC: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 824093F0: D36101C0  stfs f27, 0x1c0(r1)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(448 as u32), tmp.u32 ) };
	// 824093F4: 934101B4  stw r26, 0x1b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(436 as u32), ctx.r[26].u32 ) };
	// 824093F8: C3CB1FF8  lfs f30, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 824093FC: D3C101B0  stfs f30, 0x1b0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(432 as u32), tmp.u32 ) };
	// 82409400: D3C101B8  stfs f30, 0x1b8(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(440 as u32), tmp.u32 ) };
	// 82409404: 48001F5D  bl 0x8240b360
	ctx.lr = 0x82409408;
	sub_8240B360(ctx, base);
	// 82409408: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240940C: 408203E8  bne 0x824097f4
	if !ctx.cr[0].eq {
	pc = 0x824097F4; continue 'dispatch;
	}
	// 82409410: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82409414: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82409418: 409A000C  bne cr6, 0x82409424
	if !ctx.cr[6].eq {
	pc = 0x82409424; continue 'dispatch;
	}
	// 8240941C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82409420: 480003D4  b 0x824097f4
	pc = 0x824097F4; continue 'dispatch;
	// 82409424: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82409428: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240942C: 419AFFF0  beq cr6, 0x8240941c
	if ctx.cr[6].eq {
	pc = 0x8240941C; continue 'dispatch;
	}
	// 82409430: 388101D0  addi r4, r1, 0x1d0
	ctx.r[4].s64 = ctx.r[1].s64 + 464;
	// 82409434: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82409438: 3BAB0030  addi r29, r11, 0x30
	ctx.r[29].s64 = ctx.r[11].s64 + 48;
	// 8240943C: 7D7B5B78  mr r27, r11
	ctx.r[27].u64 = ctx.r[11].u64;
	// 82409440: 4BFFDE59  bl 0x82407298
	ctx.lr = 0x82409444;
	sub_82407298(ctx, base);
	// 82409444: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82409448: 408203AC  bne 0x824097f4
	if !ctx.cr[0].eq {
	pc = 0x824097F4; continue 'dispatch;
	}
	// 8240944C: 4BFFDF5D  bl 0x824073a8
	ctx.lr = 0x82409450;
	sub_824073A8(ctx, base);
	// 82409450: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82409454: 4BFFEFF5  bl 0x82408448
	ctx.lr = 0x82409458;
	sub_82408448(ctx, base);
	// 82409458: FF01F000  fcmpu cr6, f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[30].f64);
	// 8240945C: 409900E8  ble cr6, 0x82409544
	if !ctx.cr[6].gt {
	pc = 0x82409544; continue 'dispatch;
	}
	// 82409460: C01E1C1C  lfs f0, 0x1c1c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7196 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409464: C1A101D0  lfs f13, 0x1d0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(464 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82409468: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240946C: 409900D8  ble cr6, 0x82409544
	if !ctx.cr[6].gt {
	pc = 0x82409544; continue 'dispatch;
	}
	// 82409470: C00101AC  lfs f0, 0x1ac(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(428 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409474: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 82409478: 409900CC  ble cr6, 0x82409544
	if !ctx.cr[6].gt {
	pc = 0x82409544; continue 'dispatch;
	}
	// 8240947C: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82409480: 4BFFDB31  bl 0x82406fb0
	ctx.lr = 0x82409484;
	sub_82406FB0(ctx, base);
	// 82409484: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 82409488: 419A00BC  beq cr6, 0x82409544
	if ctx.cr[6].eq {
	pc = 0x82409544; continue 'dispatch;
	}
	// 8240948C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82409490: C02101AC  lfs f1, 0x1ac(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(428 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82409494: 4BFFF00D  bl 0x824084a0
	ctx.lr = 0x82409498;
	sub_824084A0(ctx, base);
	// 82409498: FC400890  fmr f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[1].f64;
	// 8240949C: C0BE1C24  lfs f5, 0x1c24(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7204 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 824094A0: 807E1C40  lwz r3, 0x1c40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7232 as u32) ) } as u64;
	// 824094A4: C09B0064  lfs f4, 0x64(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(100 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 824094A8: C06101D0  lfs f3, 0x1d0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(464 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 824094AC: C021007C  lfs f1, 0x7c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824094B0: 48005F69  bl 0x8240f418
	ctx.lr = 0x824094B4;
	sub_8240F418(ctx, base);
	// 824094B4: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 824094B8: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 824094BC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 824094C0: 409A0064  bne cr6, 0x82409524
	if !ctx.cr[6].eq {
	pc = 0x82409524; continue 'dispatch;
	}
	// 824094C4: 817E1C4C  lwz r11, 0x1c4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7244 as u32) ) } as u64;
	// 824094C8: C0210154  lfs f1, 0x154(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(340 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824094CC: 83A1018C  lwz r29, 0x18c(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(396 as u32) ) } as u64;
	// 824094D0: 3B8B0CBC  addi r28, r11, 0xcbc
	ctx.r[28].s64 = ctx.r[11].s64 + 3260;
	// 824094D4: 4800503D  bl 0x8240e510
	ctx.lr = 0x824094D8;
	sub_8240E510(ctx, base);
	// 824094D8: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 824094DC: EFE1F82A  fadds f31, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ((ctx.f[1].f64 + ctx.f[31].f64) as f32) as f64;
	// 824094E0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 824094E4: 409A0040  bne cr6, 0x82409524
	if !ctx.cr[6].eq {
	pc = 0x82409524; continue 'dispatch;
	}
	// 824094E8: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824094EC: 41980038  blt cr6, 0x82409524
	if ctx.cr[6].lt {
	pc = 0x82409524; continue 'dispatch;
	}
	// 824094F0: 817E1C4C  lwz r11, 0x1c4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7244 as u32) ) } as u64;
	// 824094F4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 824094F8: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824094FC: 40980028  bge cr6, 0x82409524
	if !ctx.cr[6].lt {
	pc = 0x82409524; continue 'dispatch;
	}
	// 82409500: 57AB2834  slwi r11, r29, 5
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82409504: 807E1C48  lwz r3, 0x1c48(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7240 as u32) ) } as u64;
	// 82409508: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8240950C: C0210158  lfs f1, 0x158(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(344 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82409510: 7CABE214  add r5, r11, r28
	ctx.r[5].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 82409514: C045001C  lfs f2, 0x1c(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82409518: 480063B9  bl 0x8240f8d0
	ctx.lr = 0x8240951C;
	sub_8240F8D0(ctx, base);
	// 8240951C: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409520: EFE0F82A  fadds f31, f0, f31
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 82409524: C01E1C18  lfs f0, 0x1c18(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7192 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409528: EFBF0028  fsubs f29, f31, f0
	ctx.f[29].f64 = (((ctx.f[31].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240952C: C1A101D0  lfs f13, 0x1d0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(464 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82409530: EF9F6828  fsubs f28, f31, f13
	ctx.f[28].f64 = (((ctx.f[31].f64 - ctx.f[13].f64) as f32) as f64);
	// 82409534: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 82409538: 48005021  bl 0x8240e558
	ctx.lr = 0x8240953C;
	sub_8240E558(ctx, base);
	// 8240953C: D03F0000  stfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82409540: 48000014  b 0x82409554
	pc = 0x82409554; continue 'dispatch;
	// 82409544: C01E1C1C  lfs f0, 0x1c1c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7196 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409548: D3DF0000  stfs f30, 0(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240954C: FFA00090  fmr f29, f0
	ctx.f[29].f64 = ctx.f[0].f64;
	// 82409550: FF800090  fmr f28, f0
	ctx.f[28].f64 = ctx.f[0].f64;
	// 82409554: 807B0038  lwz r3, 0x38(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(56 as u32) ) } as u64;
	// 82409558: 4BFFDEA1  bl 0x824073f8
	ctx.lr = 0x8240955C;
	sub_824073F8(ctx, base);
	// 8240955C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82409560: C01E1C1C  lfs f0, 0x1c1c(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7196 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409564: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82409568: 4099007C  ble cr6, 0x824095e4
	if !ctx.cr[6].gt {
	pc = 0x824095E4; continue 'dispatch;
	}
	// 8240956C: C1A101B8  lfs f13, 0x1b8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(440 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82409570: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82409574: 40990070  ble cr6, 0x824095e4
	if !ctx.cr[6].gt {
	pc = 0x824095E4; continue 'dispatch;
	}
	// 82409578: FF1D0000  fcmpu cr6, f29, f0
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[0].f64);
	// 8240957C: 40990068  ble cr6, 0x824095e4
	if !ctx.cr[6].gt {
	pc = 0x824095E4; continue 'dispatch;
	}
	// 82409580: 807B0038  lwz r3, 0x38(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(56 as u32) ) } as u64;
	// 82409584: 4BFFDA8D  bl 0x82407010
	ctx.lr = 0x82409588;
	sub_82407010(ctx, base);
	// 82409588: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 8240958C: 419A0058  beq cr6, 0x824095e4
	if ctx.cr[6].eq {
	pc = 0x824095E4; continue 'dispatch;
	}
	// 82409590: C01B003C  lfs f0, 0x3c(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(60 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409594: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82409598: C1A101B8  lfs f13, 0x1b8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(440 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240959C: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 824095A0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 824095A4: EFE0F82A  fadds f31, f0, f31
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 824095A8: 409A000C  bne cr6, 0x824095b4
	if !ctx.cr[6].eq {
	pc = 0x824095B4; continue 'dispatch;
	}
	// 824095AC: C0010140  lfs f0, 0x140(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(320 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824095B0: EFE0F82A  fadds f31, f0, f31
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 824095B4: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 824095B8: 40990008  ble cr6, 0x824095c0
	if !ctx.cr[6].gt {
	pc = 0x824095C0; continue 'dispatch;
	}
	// 824095BC: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	// 824095C0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824095C4: 48004F95  bl 0x8240e558
	ctx.lr = 0x824095C8;
	sub_8240E558(ctx, base);
	// 824095C8: D03F009C  stfs f1, 0x9c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 824095CC: FF1DF000  fcmpu cr6, f29, f30
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[30].f64);
	// 824095D0: 4098000C  bge cr6, 0x824095dc
	if !ctx.cr[6].lt {
	pc = 0x824095DC; continue 'dispatch;
	}
	// 824095D4: EC3FE82A  fadds f1, f31, f29
	ctx.f[1].f64 = ((ctx.f[31].f64 + ctx.f[29].f64) as f32) as f64;
	// 824095D8: 48000014  b 0x824095ec
	pc = 0x824095EC; continue 'dispatch;
	// 824095DC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 824095E0: 4800000C  b 0x824095ec
	pc = 0x824095EC; continue 'dispatch;
	// 824095E4: D3DF009C  stfs f30, 0x9c(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 824095E8: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 824095EC: 48004F6D  bl 0x8240e558
	ctx.lr = 0x824095F0;
	sub_8240E558(ctx, base);
	// 824095F0: D03F00A0  stfs f1, 0xa0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 824095F4: 80A100A8  lwz r5, 0xa8(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 824095F8: 809B0050  lwz r4, 0x50(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(80 as u32) ) } as u64;
	// 824095FC: 807E1C40  lwz r3, 0x1c40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7232 as u32) ) } as u64;
	// 82409600: 48005EC1  bl 0x8240f4c0
	ctx.lr = 0x82409604;
	sub_8240F4C0(ctx, base);
	// 82409604: 80A100AC  lwz r5, 0xac(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82409608: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240960C: 809B0060  lwz r4, 0x60(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(96 as u32) ) } as u64;
	// 82409610: 807E1C40  lwz r3, 0x1c40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7232 as u32) ) } as u64;
	// 82409614: 48005F3D  bl 0x8240f550
	ctx.lr = 0x82409618;
	sub_8240F550(ctx, base);
	// 82409618: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240961C: 816100AC  lwz r11, 0xac(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 82409620: C1BF00A0  lfs f13, 0xa0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82409624: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82409628: EDAD07F2  fmuls f13, f13, f31
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[31].f64) as f32) as f64);
	// 8240962C: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82409630: D1BF00A0  stfs f13, 0xa0(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82409634: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82409638: 935F00A4  stw r26, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[26].u32 ) };
	// 8240963C: 40990028  ble cr6, 0x82409664
	if !ctx.cr[6].gt {
	pc = 0x82409664; continue 'dispatch;
	}
	// 82409640: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 82409644: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82409648: EDAD0072  fmuls f13, f13, f1
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[1].f64) as f32) as f64);
	// 8240964C: D1BF00A0  stfs f13, 0xa0(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 82409650: FF01F000  fcmpu cr6, f1, f30
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[30].f64);
	// 82409654: 41990010  bgt cr6, 0x82409664
	if ctx.cr[6].gt {
	pc = 0x82409664; continue 'dispatch;
	}
	// 82409658: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8240965C: D3DF0000  stfs f30, 0(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82409660: 917F00A4  stw r11, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 82409664: C03F0000  lfs f1, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82409668: 48004EA9  bl 0x8240e510
	ctx.lr = 0x8240966C;
	sub_8240E510(ctx, base);
	// 8240966C: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 82409670: D03F0004  stfs f1, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82409674: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82409678: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240967C: 409A0010  bne cr6, 0x8240968c
	if !ctx.cr[6].eq {
	pc = 0x8240968C; continue 'dispatch;
	}
	// 82409680: C0210150  lfs f1, 0x150(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(336 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82409684: 48004FDD  bl 0x8240e660
	ctx.lr = 0x82409688;
	sub_8240E660(ctx, base);
	// 82409688: FC400890  fmr f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[1].f64;
	// 8240968C: 80C101D8  lwz r6, 0x1d8(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(472 as u32) ) } as u64;
	// 82409690: C07B0078  lfs f3, 0x78(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(120 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 82409694: 80A101B4  lwz r5, 0x1b4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(436 as u32) ) } as u64;
	// 82409698: C0210084  lfs f1, 0x84(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240969C: 807E1C40  lwz r3, 0x1c40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7232 as u32) ) } as u64;
	// 824096A0: 48005D91  bl 0x8240f430
	ctx.lr = 0x824096A4;
	sub_8240F430(ctx, base);
	// 824096A4: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 824096A8: D03F0008  stfs f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824096AC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 824096B0: 409A003C  bne cr6, 0x824096ec
	if !ctx.cr[6].eq {
	pc = 0x824096EC; continue 'dispatch;
	}
	// 824096B4: 3BA100BC  addi r29, r1, 0xbc
	ctx.r[29].s64 = ctx.r[1].s64 + 188;
	// 824096B8: 3B800006  li r28, 6
	ctx.r[28].s64 = 6;
	// 824096BC: C03D0000  lfs f1, 0(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824096C0: 48004E51  bl 0x8240e510
	ctx.lr = 0x824096C4;
	sub_8240E510(ctx, base);
	// 824096C4: C01B0008  lfs f0, 8(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824096C8: EC1D002A  fadds f0, f29, f0
	ctx.f[0].f64 = ((ctx.f[29].f64 + ctx.f[0].f64) as f32) as f64;
	// 824096CC: EC20082A  fadds f1, f0, f1
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 824096D0: 48004E89  bl 0x8240e558
	ctx.lr = 0x824096D4;
	sub_8240E558(ctx, base);
	// 824096D4: D03D0000  stfs f1, 0(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824096D8: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824096DC: 3BBD0018  addi r29, r29, 0x18
	ctx.r[29].s64 = ctx.r[29].s64 + 24;
	// 824096E0: 4082FFDC  bne 0x824096bc
	if !ctx.cr[0].eq {
	pc = 0x824096BC; continue 'dispatch;
	}
	// 824096E4: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 824096E8: 4800004C  b 0x82409734
	pc = 0x82409734; continue 'dispatch;
	// 824096EC: 807E1C40  lwz r3, 0x1c40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7232 as u32) ) } as u64;
	// 824096F0: C06101D4  lfs f3, 0x1d4(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(468 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 824096F4: C04101B0  lfs f2, 0x1b0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(432 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 824096F8: C0210080  lfs f1, 0x80(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824096FC: 48005DB5  bl 0x8240f4b0
	ctx.lr = 0x82409700;
	sub_8240F4B0(ctx, base);
	// 82409700: C01B0008  lfs f0, 8(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409704: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82409708: EC20E82A  fadds f1, f0, f29
	ctx.f[1].f64 = ((ctx.f[0].f64 + ctx.f[29].f64) as f32) as f64;
	// 8240970C: 48004E4D  bl 0x8240e558
	ctx.lr = 0x82409710;
	sub_8240E558(ctx, base);
	// 82409710: 392102E0  addi r9, r1, 0x2e0
	ctx.r[9].s64 = ctx.r[1].s64 + 736;
	// 82409714: FCA00890  fmr f5, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[5].f64 = ctx.f[1].f64;
	// 82409718: 80610098  lwz r3, 0x98(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 8240971C: C05B0014  lfs f2, 0x14(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82409720: C081008C  lfs f4, 0x8c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 82409724: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82409728: C0610088  lfs f3, 0x88(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8240972C: 4BFFCBFD  bl 0x82406328
	ctx.lr = 0x82409730;
	sub_82406328(ctx, base);
	// 82409730: 388102E0  addi r4, r1, 0x2e0
	ctx.r[4].s64 = ctx.r[1].s64 + 736;
	// 82409734: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 82409738: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 8240973C: 4812B89D  bl 0x82534fd8
	ctx.lr = 0x82409740;
	sub_82534FD8(ctx, base);
	// 82409740: C01B0028  lfs f0, 0x28(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409744: C1A101C0  lfs f13, 0x1c0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(448 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82409748: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 8240974C: 4099000C  ble cr6, 0x82409758
	if !ctx.cr[6].gt {
	pc = 0x82409758; continue 'dispatch;
	}
	// 82409750: D1BF00A8  stfs f13, 0xa8(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 82409754: 48000008  b 0x8240975c
	pc = 0x8240975C; continue 'dispatch;
	// 82409758: D01F00A8  stfs f0, 0xa8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 8240975C: C01B002C  lfs f0, 0x2c(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409760: C1A101BC  lfs f13, 0x1bc(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(444 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82409764: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82409768: 4099000C  ble cr6, 0x82409774
	if !ctx.cr[6].gt {
	pc = 0x82409774; continue 'dispatch;
	}
	// 8240976C: D1BF00AC  stfs f13, 0xac(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82409770: 48000008  b 0x82409778
	pc = 0x82409778; continue 'dispatch;
	// 82409774: D01F00AC  stfs f0, 0xac(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 82409778: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 8240977C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82409780: 409A002C  bne cr6, 0x824097ac
	if !ctx.cr[6].eq {
	pc = 0x824097AC; continue 'dispatch;
	}
	// 82409784: C1BF00A8  lfs f13, 0xa8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82409788: C0010144  lfs f0, 0x144(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(324 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240978C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82409790: 40990008  ble cr6, 0x82409798
	if !ctx.cr[6].gt {
	pc = 0x82409798; continue 'dispatch;
	}
	// 82409794: D01F00A8  stfs f0, 0xa8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 82409798: C1BF00AC  lfs f13, 0xac(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240979C: C0010148  lfs f0, 0x148(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(328 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824097A0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824097A4: 40990008  ble cr6, 0x824097ac
	if !ctx.cr[6].gt {
	pc = 0x824097AC; continue 'dispatch;
	}
	// 824097A8: D01F00AC  stfs f0, 0xac(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 824097AC: C01F00A8  lfs f0, 0xa8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824097B0: FF00D800  fcmpu cr6, f0, f27
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[27].f64);
	// 824097B4: 40990008  ble cr6, 0x824097bc
	if !ctx.cr[6].gt {
	pc = 0x824097BC; continue 'dispatch;
	}
	// 824097B8: D37F00A8  stfs f27, 0xa8(r31)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), tmp.u32 ) };
	// 824097BC: C01F00AC  lfs f0, 0xac(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824097C0: FF00D800  fcmpu cr6, f0, f27
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[27].f64);
	// 824097C4: 40990008  ble cr6, 0x824097cc
	if !ctx.cr[6].gt {
	pc = 0x824097CC; continue 'dispatch;
	}
	// 824097C8: D37F00AC  stfs f27, 0xac(r31)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 824097CC: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824097D0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824097D4: D001009C  stfs f0, 0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 824097D8: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 824097DC: C01F0008  lfs f0, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824097E0: 807E1C3C  lwz r3, 0x1c3c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7228 as u32) ) } as u64;
	// 824097E4: D00100A4  stfs f0, 0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 824097E8: D38100A0  stfs f28, 0xa0(r1)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), tmp.u32 ) };
	// 824097EC: 48001B1D  bl 0x8240b308
	ctx.lr = 0x824097F0;
	sub_8240B308(ctx, base);
	// 824097F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824097F4: 382103E0  addi r1, r1, 0x3e0
	ctx.r[1].s64 = ctx.r[1].s64 + 992;
	// 824097F8: 3981FFC0  addi r12, r1, -0x40
	ctx.r[12].s64 = ctx.r[1].s64 + -64;
	// 824097FC: 4812C835  bl 0x82536030
	ctx.lr = 0x82409800;
	sub_82535FFC(ctx, base);
	// 82409800: 4812B8FC  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82409808 size=372
    let mut pc: u32 = 0x82409808;
    'dispatch: loop {
        match pc {
            0x82409808 => {
    //   block [0x82409808..0x8240997C)
	// 82409808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240980C: 4812B8A5  bl 0x825350b0
	ctx.lr = 0x82409810;
	sub_82535080(ctx, base);
	// 82409810: DBC1FFB8  stfd f30, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[30].u64 ) };
	// 82409814: DBE1FFC0  stfd f31, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82409818: 9421FDF0  stwu r1, -0x210(r1)
	ea = ctx.r[1].u32.wrapping_add(-528 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240981C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82409820: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82409824: 4BFFB9ED  bl 0x82405210
	ctx.lr = 0x82409828;
	sub_82405210(ctx, base);
	// 82409828: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8240982C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82409830: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 82409834: 397F0118  addi r11, r31, 0x118
	ctx.r[11].s64 = ctx.r[31].s64 + 280;
	// 82409838: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 8240983C: C01F1C1C  lfs f0, 0x1c1c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7196 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409840: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 82409844: D00BFF00  stfs f0, -0x100(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-256 as u32), tmp.u32 ) };
	// 82409848: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240984C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82409850: 4082FFEC  bne 0x8240983c
	if !ctx.cr[0].eq {
	pc = 0x8240983C; continue 'dispatch;
	}
	// 82409854: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 82409858: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8240985C: 40990040  ble cr6, 0x8240989c
	if !ctx.cr[6].gt {
	pc = 0x8240989C; continue 'dispatch;
	}
	// 82409860: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82409864: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82409868: 4BFFB971  bl 0x824051d8
	ctx.lr = 0x8240986C;
	sub_824051D8(ctx, base);
	// 8240986C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82409870: 40820020  bne 0x82409890
	if !ctx.cr[0].eq {
	pc = 0x82409890; continue 'dispatch;
	}
	// 82409874: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82409878: 4BFFB921  bl 0x82405198
	ctx.lr = 0x8240987C;
	sub_82405198(ctx, base);
	// 8240987C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82409880: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82409884: C0210050  lfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82409888: 4BFFEE09  bl 0x82408690
	ctx.lr = 0x8240988C;
	sub_82408690(ctx, base);
	// 8240988C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82409890: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82409894: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82409898: 4198FFC8  blt cr6, 0x82409860
	if ctx.cr[6].lt {
	pc = 0x82409860; continue 'dispatch;
	}
	// 8240989C: 807F1C3C  lwz r3, 0x1c3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7228 as u32) ) } as u64;
	// 824098A0: 48001FC9  bl 0x8240b868
	ctx.lr = 0x824098A4;
	sub_8240B868(ctx, base);
	// 824098A4: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 824098A8: 409A0024  bne cr6, 0x824098cc
	if !ctx.cr[6].eq {
	pc = 0x824098CC; continue 'dispatch;
	}
	// 824098AC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 824098B0: 409A001C  bne cr6, 0x824098cc
	if !ctx.cr[6].eq {
	pc = 0x824098CC; continue 'dispatch;
	}
	// 824098B4: 817F1C4C  lwz r11, 0x1c4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7244 as u32) ) } as u64;
	// 824098B8: 807F1C44  lwz r3, 0x1c44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7236 as u32) ) } as u64;
	// 824098BC: 388B0BBC  addi r4, r11, 0xbbc
	ctx.r[4].s64 = ctx.r[11].s64 + 3004;
	// 824098C0: 80AB0014  lwz r5, 0x14(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 824098C4: 48005ADD  bl 0x8240f3a0
	ctx.lr = 0x824098C8;
	sub_8240F3A0(ctx, base);
	// 824098C8: 480000A0  b 0x82409968
	pc = 0x82409968; continue 'dispatch;
	// 824098CC: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824098D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 824098D4: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 824098D8: C3CA1FF8  lfs f30, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 824098DC: C3EB1850  lfs f31, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 824098E0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 824098E4: 807F1C3C  lwz r3, 0x1c3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7228 as u32) ) } as u64;
	// 824098E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824098EC: D3E101AC  stfs f31, 0x1ac(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(428 as u32), tmp.u32 ) };
	// 824098F0: D3C101B0  stfs f30, 0x1b0(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(432 as u32), tmp.u32 ) };
	// 824098F4: 936101B4  stw r27, 0x1b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(436 as u32), ctx.r[27].u32 ) };
	// 824098F8: D3C101B8  stfs f30, 0x1b8(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(440 as u32), tmp.u32 ) };
	// 824098FC: D3E101BC  stfs f31, 0x1bc(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(444 as u32), tmp.u32 ) };
	// 82409900: D3E101C0  stfs f31, 0x1c0(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(448 as u32), tmp.u32 ) };
	// 82409904: 48001A5D  bl 0x8240b360
	ctx.lr = 0x82409908;
	sub_8240B360(ctx, base);
	// 82409908: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8240990C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82409910: 419A0030  beq cr6, 0x82409940
	if ctx.cr[6].eq {
	pc = 0x82409940; continue 'dispatch;
	}
	// 82409914: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82409918: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8240991C: 807F1C38  lwz r3, 0x1c38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82409920: 48002DD9  bl 0x8240c6f8
	ctx.lr = 0x82409924;
	sub_8240C6F8(ctx, base);
	// 82409924: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82409928: 40820018  bne 0x82409940
	if !ctx.cr[0].eq {
	pc = 0x82409940; continue 'dispatch;
	}
	// 8240992C: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82409930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82409934: C02100A0  lfs f1, 0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82409938: 80AB0030  lwz r5, 0x30(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8240993C: 4BFFED55  bl 0x82408690
	ctx.lr = 0x82409940;
	sub_82408690(ctx, base);
	// 82409940: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82409944: 2F1E00C0  cmpwi cr6, r30, 0xc0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 192, &mut ctx.xer);
	// 82409948: 4198FF98  blt cr6, 0x824098e0
	if ctx.cr[6].lt {
	pc = 0x824098E0; continue 'dispatch;
	}
	// 8240994C: 817F1C4C  lwz r11, 0x1c4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7244 as u32) ) } as u64;
	// 82409950: 38BF0018  addi r5, r31, 0x18
	ctx.r[5].s64 = ctx.r[31].s64 + 24;
	// 82409954: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82409958: 807F1C44  lwz r3, 0x1c44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7236 as u32) ) } as u64;
	// 8240995C: 38CB0BBC  addi r6, r11, 0xbbc
	ctx.r[6].s64 = ctx.r[11].s64 + 3004;
	// 82409960: 80EB0014  lwz r7, 0x14(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82409964: 48005625  bl 0x8240ef88
	ctx.lr = 0x82409968;
	sub_8240EF88(ctx, base);
	// 82409968: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240996C: 38210210  addi r1, r1, 0x210
	ctx.r[1].s64 = ctx.r[1].s64 + 528;
	// 82409970: CBC1FFB8  lfd f30, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 82409974: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 82409978: 4812B788  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82409980 size=220
    let mut pc: u32 = 0x82409980;
    'dispatch: loop {
        match pc {
            0x82409980 => {
    //   block [0x82409980..0x82409A5C)
	// 82409980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82409984: 4812B739  bl 0x825350bc
	ctx.lr = 0x82409988;
	sub_82535080(ctx, base);
	// 82409988: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240998C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82409990: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82409994: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82409998: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 8240999C: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824099A0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824099A4: 807F1C50  lwz r3, 0x1c50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7248 as u32) ) } as u64;
	// 824099A8: D00100F0  stfs f0, 0xf0(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 824099AC: D0010100  stfs f0, 0x100(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), tmp.u32 ) };
	// 824099B0: 93C100F8  stw r30, 0xf8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), ctx.r[30].u32 ) };
	// 824099B4: D0010104  stfs f0, 0x104(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), tmp.u32 ) };
	// 824099B8: C1AB1FF8  lfs f13, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824099BC: D1A100F4  stfs f13, 0xf4(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 824099C0: D1A100FC  stfs f13, 0xfc(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 824099C4: 4800509D  bl 0x8240ea60
	ctx.lr = 0x824099C8;
	sub_8240EA60(ctx, base);
	// 824099C8: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 824099CC: 4082007C  bne 0x82409a48
	if !ctx.cr[0].eq {
	pc = 0x82409A48; continue 'dispatch;
	}
	// 824099D0: 3BBF0218  addi r29, r31, 0x218
	ctx.r[29].s64 = ctx.r[31].s64 + 536;
	// 824099D4: 8141009C  lwz r10, 0x9c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(156 as u32) ) } as u64;
	// 824099D8: 396100D4  addi r11, r1, 0xd4
	ctx.r[11].s64 = ctx.r[1].s64 + 212;
	// 824099DC: 81010080  lwz r8, 0x80(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 824099E0: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 824099E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824099E8: 80E10098  lwz r7, 0x98(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) } as u64;
	// 824099EC: C04100F4  lfs f2, 0xf4(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(244 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 824099F0: 80C1008C  lwz r6, 0x8c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 824099F4: C02100F0  lfs f1, 0xf0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(240 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824099F8: 80A10084  lwz r5, 0x84(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 824099FC: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82409A00: 814100F8  lwz r10, 0xf8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(248 as u32) ) } as u64;
	// 82409A04: 80810090  lwz r4, 0x90(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 82409A08: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 82409A0C: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82409A10: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82409A14: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82409A18: 4BFFF1C9  bl 0x82408be0
	ctx.lr = 0x82409A1C;
	sub_82408BE0(ctx, base);
	// 82409A1C: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82409A20: 40820014  bne 0x82409a34
	if !ctx.cr[0].eq {
	pc = 0x82409A34; continue 'dispatch;
	}
	// 82409A24: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82409A28: 3BBD00D0  addi r29, r29, 0xd0
	ctx.r[29].s64 = ctx.r[29].s64 + 208;
	// 82409A2C: 2F1E0020  cmpwi cr6, r30, 0x20
	ctx.cr[6].compare_i32(ctx.r[30].s32, 32, &mut ctx.xer);
	// 82409A30: 40980024  bge cr6, 0x82409a54
	if !ctx.cr[6].lt {
	pc = 0x82409A54; continue 'dispatch;
	}
	// 82409A34: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 82409A38: 807F1C50  lwz r3, 0x1c50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82409A3C: 48005025  bl 0x8240ea60
	ctx.lr = 0x82409A40;
	sub_8240EA60(ctx, base);
	// 82409A40: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82409A44: 4182FF90  beq 0x824099d4
	if ctx.cr[0].eq {
	pc = 0x824099D4; continue 'dispatch;
	}
	// 82409A48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82409A4C: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 82409A50: 4812B6BC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
	// 82409A54: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82409A58: 4BFFFFF4  b 0x82409a4c
	pc = 0x82409A4C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82409A60 size=124
    let mut pc: u32 = 0x82409A60;
    'dispatch: loop {
        match pc {
            0x82409A60 => {
    //   block [0x82409A60..0x82409ADC)
	// 82409A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82409A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82409A68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82409A6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82409A70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82409A74: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82409A78: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82409A7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82409A80: 4BFFCEA1  bl 0x82406920
	ctx.lr = 0x82409A84;
	sub_82406920(ctx, base);
	// 82409A84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82409A88: 4BFFD3D9  bl 0x82406e60
	ctx.lr = 0x82409A8C;
	sub_82406E60(ctx, base);
	// 82409A8C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82409A90: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82409A94: 409A0010  bne cr6, 0x82409aa4
	if !ctx.cr[6].eq {
	pc = 0x82409AA4; continue 'dispatch;
	}
	// 82409A98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82409A9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82409AA0: 4BFFFD69  bl 0x82409808
	ctx.lr = 0x82409AA4;
	sub_82409808(ctx, base);
	// 82409AA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82409AA8: 807E1C3C  lwz r3, 0x1c3c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7228 as u32) ) } as u64;
	// 82409AAC: 480016E5  bl 0x8240b190
	ctx.lr = 0x82409AB0;
	sub_8240B190(ctx, base);
	// 82409AB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82409AB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82409AB8: 4BFFECB9  bl 0x82408770
	ctx.lr = 0x82409ABC;
	sub_82408770(ctx, base);
	// 82409ABC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82409AC0: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82409AC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82409AC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82409ACC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82409AD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82409AD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82409AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82409AE0 size=96
    let mut pc: u32 = 0x82409AE0;
    'dispatch: loop {
        match pc {
            0x82409AE0 => {
    //   block [0x82409AE0..0x82409B40)
	// 82409AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82409AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82409AE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82409AEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82409AF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82409AF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82409AF8: 807E1C4C  lwz r3, 0x1c4c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(7244 as u32) ) } as u64;
	// 82409AFC: 48005315  bl 0x8240ee10
	ctx.lr = 0x82409B00;
	sub_8240EE10(ctx, base);
	// 82409B00: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82409B04: 4182001C  beq 0x82409b20
	if ctx.cr[0].eq {
	pc = 0x82409B20; continue 'dispatch;
	}
	// 82409B08: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409B0C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82409B10: 386BCECC  addi r3, r11, -0x3134
	ctx.r[3].s64 = ctx.r[11].s64 + -12596;
	// 82409B14: 4BEA946D  bl 0x822b2f80
	ctx.lr = 0x82409B18;
	sub_822B2F80(ctx, base);
	// 82409B18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82409B1C: 4800000C  b 0x82409b28
	pc = 0x82409B28; continue 'dispatch;
	// 82409B20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82409B24: 4BFFEE1D  bl 0x82408940
	ctx.lr = 0x82409B28;
	sub_82408940(ctx, base);
	// 82409B28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82409B2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82409B30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82409B34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82409B38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82409B3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82409B40 size=704
    let mut pc: u32 = 0x82409B40;
    'dispatch: loop {
        match pc {
            0x82409B40 => {
    //   block [0x82409B40..0x82409E00)
	// 82409B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82409B44: 4812B549  bl 0x8253508c
	ctx.lr = 0x82409B48;
	sub_82535080(ctx, base);
	// 82409B48: DBC1FF70  stfd f30, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[30].u64 ) };
	// 82409B4C: DBE1FF78  stfd f31, -0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 82409B50: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82409B54: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82409B58: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82409B5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82409B60: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82409B64: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82409B68: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82409B6C: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82409B70: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409B74: 7D344B78  mr r20, r9
	ctx.r[20].u64 = ctx.r[9].u64;
	// 82409B78: 7D565378  mr r22, r10
	ctx.r[22].u64 = ctx.r[10].u64;
	// 82409B7C: 3A200000  li r17, 0
	ctx.r[17].s64 = 0;
	// 82409B80: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82409B84: 40980024  bge cr6, 0x82409ba8
	if !ctx.cr[6].lt {
	pc = 0x82409BA8; continue 'dispatch;
	}
	// 82409B88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409B8C: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 82409B90: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 82409B94: 386BD028  addi r3, r11, -0x2fd8
	ctx.r[3].s64 = ctx.r[11].s64 + -12248;
	// 82409B98: 4BEA93E9  bl 0x822b2f80
	ctx.lr = 0x82409B9C;
	sub_822B2F80(ctx, base);
	// 82409B9C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82409BA0: 60630018  ori r3, r3, 0x18
	ctx.r[3].u64 = ctx.r[3].u64 | 24;
	// 82409BA4: 4800024C  b 0x82409df0
	pc = 0x82409DF0; continue 'dispatch;
	// 82409BA8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82409BAC: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 82409BB0: 356B0001  addic. r11, r11, 1
	ctx.xer.ca = (ctx.r[11].u32 > (!(1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82409BB4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82409BB8: 40820008  bne 0x82409bc0
	if !ctx.cr[0].eq {
	pc = 0x82409BC0; continue 'dispatch;
	}
	// 82409BBC: 935F0004  stw r26, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u32 ) };
	// 82409BC0: 8161017C  lwz r11, 0x17c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(380 as u32) ) } as u64;
	// 82409BC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82409BC8: 419A0214  beq cr6, 0x82409ddc
	if ctx.cr[6].eq {
	pc = 0x82409DDC; continue 'dispatch;
	}
	// 82409BCC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82409BD0: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82409BD4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82409BD8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82409BDC: 807F1C38  lwz r3, 0x1c38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82409BE0: 480028A1  bl 0x8240c480
	ctx.lr = 0x82409BE4;
	sub_8240C480(ctx, base);
	// 82409BE4: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82409BE8: 4182001C  beq 0x82409c04
	if ctx.cr[0].eq {
	pc = 0x82409C04; continue 'dispatch;
	}
	// 82409BEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409BF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82409BF4: 386BCFE0  addi r3, r11, -0x3020
	ctx.r[3].s64 = ctx.r[11].s64 + -12320;
	// 82409BF8: 4BEA9389  bl 0x822b2f80
	ctx.lr = 0x82409BFC;
	sub_822B2F80(ctx, base);
	// 82409BFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82409C00: 480001F0  b 0x82409df0
	pc = 0x82409DF0; continue 'dispatch;
	// 82409C04: 83010174  lwz r24, 0x174(r1)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(372 as u32) ) } as u64;
	// 82409C08: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82409C0C: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 82409C10: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82409C14: 419A000C  beq cr6, 0x82409c20
	if ctx.cr[6].eq {
	pc = 0x82409C20; continue 'dispatch;
	}
	// 82409C18: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82409C1C: 409A0008  bne cr6, 0x82409c24
	if !ctx.cr[6].eq {
	pc = 0x82409C24; continue 'dispatch;
	}
	// 82409C20: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82409C24: 3D400008  lis r10, 8
	ctx.r[10].s64 = 524288;
	// 82409C28: 817F1C38  lwz r11, 0x1c38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82409C2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82409C30: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82409C34: 6157030C  ori r23, r10, 0x30c
	ctx.r[23].u64 = ctx.r[10].u64 | 780;
	// 82409C38: 7C6BBA14  add r3, r11, r23
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 82409C3C: 4BFFD00D  bl 0x82406c48
	ctx.lr = 0x82409C40;
	sub_82406C48(ctx, base);
	// 82409C40: 7C731B79  or. r19, r3, r3
	ctx.r[19].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[19].s32, 0, &mut ctx.xer);
	// 82409C44: 40820024  bne 0x82409c68
	if !ctx.cr[0].eq {
	pc = 0x82409C68; continue 'dispatch;
	}
	// 82409C48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409C4C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82409C50: 386BCF90  addi r3, r11, -0x3070
	ctx.r[3].s64 = ctx.r[11].s64 + -12400;
	// 82409C54: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82409C58: 4BEA9329  bl 0x822b2f80
	ctx.lr = 0x82409C5C;
	sub_822B2F80(ctx, base);
	// 82409C5C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82409C60: 6063000F  ori r3, r3, 0xf
	ctx.r[3].u64 = ctx.r[3].u64 | 15;
	// 82409C64: 4800018C  b 0x82409df0
	pc = 0x82409DF0; continue 'dispatch;
	// 82409C68: 8173002C  lwz r11, 0x2c(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(44 as u32) ) } as u64;
	// 82409C6C: 3A400000  li r18, 0
	ctx.r[18].s64 = 0;
	// 82409C70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82409C74: 4198007C  blt cr6, 0x82409cf0
	if ctx.cr[6].lt {
	pc = 0x82409CF0; continue 'dispatch;
	}
	// 82409C78: 3B3F0218  addi r25, r31, 0x218
	ctx.r[25].s64 = ctx.r[31].s64 + 536;
	// 82409C7C: 817F1C38  lwz r11, 0x1c38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82409C80: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82409C84: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82409C88: 7C6BBA14  add r3, r11, r23
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[23].u64;
	// 82409C8C: 4BFFCF7D  bl 0x82406c08
	ctx.lr = 0x82409C90;
	sub_82406C08(ctx, base);
	// 82409C90: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82409C94: 41820120  beq 0x82409db4
	if ctx.cr[0].eq {
	pc = 0x82409DB4; continue 'dispatch;
	}
	// 82409C98: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82409C9C: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82409CA0: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 82409CA4: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82409CA8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82409CAC: 93010074  stw r24, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[24].u32 ) };
	// 82409CB0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82409CB4: 92C1006C  stw r22, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[22].u32 ) };
	// 82409CB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82409CBC: 93410064  stw r26, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 82409CC0: 92810054  stw r20, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[20].u32 ) };
	// 82409CC4: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82409CC8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82409CCC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82409CD0: 4BFFEDB9  bl 0x82408a88
	ctx.lr = 0x82409CD4;
	sub_82408A88(ctx, base);
	// 82409CD4: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82409CD8: 40820028  bne 0x82409d00
	if !ctx.cr[0].eq {
	pc = 0x82409D00; continue 'dispatch;
	}
	// 82409CDC: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82409CE0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82409CE4: 419A000C  beq cr6, 0x82409cf0
	if ctx.cr[6].eq {
	pc = 0x82409CF0; continue 'dispatch;
	}
	// 82409CE8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82409CEC: 409A008C  bne cr6, 0x82409d78
	if !ctx.cr[6].eq {
	pc = 0x82409D78; continue 'dispatch;
	}
	// 82409CF0: 81610184  lwz r11, 0x184(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(388 as u32) ) } as u64;
	// 82409CF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82409CF8: 922B0000  stw r17, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[17].u32 ) };
	// 82409CFC: 480000F4  b 0x82409df0
	pc = 0x82409DF0; continue 'dispatch;
	// 82409D00: 2F110020  cmpwi cr6, r17, 0x20
	ctx.cr[6].compare_i32(ctx.r[17].s32, 32, &mut ctx.xer);
	// 82409D04: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82409D08: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82409D0C: 409800B8  bge cr6, 0x82409dc4
	if !ctx.cr[6].lt {
	pc = 0x82409DC4; continue 'dispatch;
	}
	// 82409D10: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82409D14: 7E8AA378  mr r10, r20
	ctx.r[10].u64 = ctx.r[20].u64;
	// 82409D18: 7EA7AB78  mr r7, r21
	ctx.r[7].u64 = ctx.r[21].u64;
	// 82409D1C: 80810080  lwz r4, 0x80(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82409D20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82409D24: 93210074  stw r25, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[25].u32 ) };
	// 82409D28: 9301006C  stw r24, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[24].u32 ) };
	// 82409D2C: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82409D30: 92C10064  stw r22, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[22].u32 ) };
	// 82409D34: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82409D38: 9341005C  stw r26, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 82409D3C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82409D40: 4BFFEEA1  bl 0x82408be0
	ctx.lr = 0x82409D44;
	sub_82408BE0(ctx, base);
	// 82409D44: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82409D48: 4082000C  bne 0x82409d54
	if !ctx.cr[0].eq {
	pc = 0x82409D54; continue 'dispatch;
	}
	// 82409D4C: 3A310001  addi r17, r17, 1
	ctx.r[17].s64 = ctx.r[17].s64 + 1;
	// 82409D50: 3B3900D0  addi r25, r25, 0xd0
	ctx.r[25].s64 = ctx.r[25].s64 + 208;
	// 82409D54: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 82409D58: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82409D5C: 409A000C  bne cr6, 0x82409d68
	if !ctx.cr[6].eq {
	pc = 0x82409D68; continue 'dispatch;
	}
	// 82409D60: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82409D64: 419AFF8C  beq cr6, 0x82409cf0
	if ctx.cr[6].eq {
	pc = 0x82409CF0; continue 'dispatch;
	}
	// 82409D68: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82409D6C: 409A000C  bne cr6, 0x82409d78
	if !ctx.cr[6].eq {
	pc = 0x82409D78; continue 'dispatch;
	}
	// 82409D70: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82409D74: 409AFF7C  bne cr6, 0x82409cf0
	if !ctx.cr[6].eq {
	pc = 0x82409CF0; continue 'dispatch;
	}
	// 82409D78: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82409D7C: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 82409D80: 419A0008  beq cr6, 0x82409d88
	if ctx.cr[6].eq {
	pc = 0x82409D88; continue 'dispatch;
	}
	// 82409D84: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 82409D88: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82409D8C: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 82409D90: 4180000C  blt 0x82409d9c
	if ctx.cr[0].lt {
	pc = 0x82409D9C; continue 'dispatch;
	}
	// 82409D94: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 82409D98: 48000008  b 0x82409da0
	pc = 0x82409DA0; continue 'dispatch;
	// 82409D9C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82409DA0: 8173002C  lwz r11, 0x2c(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(44 as u32) ) } as u64;
	// 82409DA4: 3A520001  addi r18, r18, 1
	ctx.r[18].s64 = ctx.r[18].s64 + 1;
	// 82409DA8: 7F125800  cmpw cr6, r18, r11
	ctx.cr[6].compare_i32(ctx.r[18].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82409DAC: 4099FED0  ble cr6, 0x82409c7c
	if !ctx.cr[6].gt {
	pc = 0x82409C7C; continue 'dispatch;
	}
	// 82409DB0: 4BFFFF40  b 0x82409cf0
	pc = 0x82409CF0; continue 'dispatch;
	// 82409DB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409DB8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82409DBC: 386BCDB8  addi r3, r11, -0x3248
	ctx.r[3].s64 = ctx.r[11].s64 + -12872;
	// 82409DC0: 4BFFFE94  b 0x82409c54
	pc = 0x82409C54; continue 'dispatch;
	// 82409DC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409DC8: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82409DCC: 386BCF40  addi r3, r11, -0x30c0
	ctx.r[3].s64 = ctx.r[11].s64 + -12480;
	// 82409DD0: 4BEA91B1  bl 0x822b2f80
	ctx.lr = 0x82409DD4;
	sub_822B2F80(ctx, base);
	// 82409DD4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82409DD8: 48000018  b 0x82409df0
	pc = 0x82409DF0; continue 'dispatch;
	// 82409DDC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409DE0: 386BCF08  addi r3, r11, -0x30f8
	ctx.r[3].s64 = ctx.r[11].s64 + -12536;
	// 82409DE4: 4BEA919D  bl 0x822b2f80
	ctx.lr = 0x82409DE8;
	sub_822B2F80(ctx, base);
	// 82409DE8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 82409DEC: 60630015  ori r3, r3, 0x15
	ctx.r[3].u64 = ctx.r[3].u64 | 21;
	// 82409DF0: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 82409DF4: CBC1FF70  lfd f30, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-144 as u32) ) };
	// 82409DF8: CBE1FF78  lfd f31, -0x88(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-136 as u32) ) };
	// 82409DFC: 4812B2E0  b 0x825350dc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82409E00 size=104
    let mut pc: u32 = 0x82409E00;
    'dispatch: loop {
        match pc {
            0x82409E00 => {
    //   block [0x82409E00..0x82409E68)
	// 82409E00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82409E04: 4812B2B9  bl 0x825350bc
	ctx.lr = 0x82409E08;
	sub_82535080(ctx, base);
	// 82409E08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82409E0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82409E10: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82409E14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82409E18: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82409E1C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82409E20: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82409E24: 4BFFFCBD  bl 0x82409ae0
	ctx.lr = 0x82409E28;
	sub_82409AE0(ctx, base);
	// 82409E28: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82409E2C: 4182001C  beq 0x82409e48
	if ctx.cr[0].eq {
	pc = 0x82409E48; continue 'dispatch;
	}
	// 82409E30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409E34: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82409E38: 386BD058  addi r3, r11, -0x2fa8
	ctx.r[3].s64 = ctx.r[11].s64 + -12200;
	// 82409E3C: 4BEA9145  bl 0x822b2f80
	ctx.lr = 0x82409E40;
	sub_822B2F80(ctx, base);
	// 82409E40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82409E44: 4800001C  b 0x82409e60
	pc = 0x82409E60; continue 'dispatch;
	// 82409E48: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82409E4C: 807F1C38  lwz r3, 0x1c38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7224 as u32) ) } as u64;
	// 82409E50: 48002059  bl 0x8240bea8
	ctx.lr = 0x82409E54;
	sub_8240BEA8(ctx, base);
	// 82409E54: 807F1C50  lwz r3, 0x1c50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(7248 as u32) ) } as u64;
	// 82409E58: 48004E91  bl 0x8240ece8
	ctx.lr = 0x82409E5C;
	sub_8240ECE8(ctx, base);
	// 82409E5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82409E60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82409E64: 4812B2A8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82409E68 size=72
    let mut pc: u32 = 0x82409E68;
    'dispatch: loop {
        match pc {
            0x82409E68 => {
    //   block [0x82409E68..0x82409EB0)
	// 82409E68: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 82409E6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82409E70: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82409E74: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82409E78: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 82409E7C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82409E80: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 82409E84: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82409E88: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82409E8C: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82409E90: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82409E94: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82409E98: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82409E9C: 91430034  stw r10, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 82409EA0: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 82409EA4: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82409EA8: 9963000C  stb r11, 0xc(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u8 ) };
	// 82409EAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82409EB0 size=68
    let mut pc: u32 = 0x82409EB0;
    'dispatch: loop {
        match pc {
            0x82409EB0 => {
    //   block [0x82409EB0..0x82409EF4)
	// 82409EB0: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82409EB4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82409EB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82409EBC: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82409EC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82409EC4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82409EC8: 914B0028  stw r10, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 82409ECC: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82409ED0: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 82409ED4: 914B0024  stw r10, 0x24(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 82409ED8: 914B001C  stw r10, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82409EDC: 910B0014  stw r8, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82409EE0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82409EE4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82409EE8: 994B000C  stb r10, 0xc(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u8 ) };
	// 82409EEC: 90AB0010  stw r5, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82409EF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82409EF8 size=20
    let mut pc: u32 = 0x82409EF8;
    'dispatch: loop {
        match pc {
            0x82409EF8 => {
    //   block [0x82409EF8..0x82409F0C)
	// 82409EF8: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82409EFC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82409F00: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82409F04: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82409F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82409F10 size=104
    let mut pc: u32 = 0x82409F10;
    'dispatch: loop {
        match pc {
            0x82409F10 => {
    //   block [0x82409F10..0x82409F78)
	// 82409F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82409F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82409F18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82409F1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82409F20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82409F24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82409F28: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82409F2C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82409F30: 409A0030  bne cr6, 0x82409f60
	if !ctx.cr[6].eq {
	pc = 0x82409F60; continue 'dispatch;
	}
	// 82409F34: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82409F38: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82409F3C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82409F40: 4182000C  beq 0x82409f4c
	if ctx.cr[0].eq {
	pc = 0x82409F4C; continue 'dispatch;
	}
	// 82409F44: 48179C65  bl 0x82583ba8
	ctx.lr = 0x82409F48;
	sub_82583BA8(ctx, base);
	// 82409F48: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 82409F4C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82409F50: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82409F54: 9BDF000C  stb r30, 0xc(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u8 ) };
	// 82409F58: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82409F5C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82409F60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82409F64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82409F68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82409F6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82409F70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82409F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82409F78 size=100
    let mut pc: u32 = 0x82409F78;
    'dispatch: loop {
        match pc {
            0x82409F78 => {
    //   block [0x82409F78..0x82409FDC)
	// 82409F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82409F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82409F80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82409F84: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82409F88: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82409F8C: 419A000C  beq cr6, 0x82409f98
	if ctx.cr[6].eq {
	pc = 0x82409F98; continue 'dispatch;
	}
	// 82409F90: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 82409F94: 48000038  b 0x82409fcc
	pc = 0x82409FCC; continue 'dispatch;
	// 82409F98: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82409F9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82409FA0: 41820028  beq 0x82409fc8
	if ctx.cr[0].eq {
	pc = 0x82409FC8; continue 'dispatch;
	}
	// 82409FA4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82409FA8: 48179EF9  bl 0x82583ea0
	ctx.lr = 0x82409FAC;
	sub_82583EA0(ctx, base);
	// 82409FAC: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82409FB0: 40800018  bge 0x82409fc8
	if !ctx.cr[0].lt {
	pc = 0x82409FC8; continue 'dispatch;
	}
	// 82409FB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82409FB8: 386BD090  addi r3, r11, -0x2f70
	ctx.r[3].s64 = ctx.r[11].s64 + -12144;
	// 82409FBC: 4BEA8FC5  bl 0x822b2f80
	ctx.lr = 0x82409FC0;
	sub_822B2F80(ctx, base);
	// 82409FC0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82409FC4: 48000008  b 0x82409fcc
	pc = 0x82409FCC; continue 'dispatch;
	// 82409FC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82409FCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82409FD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82409FD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82409FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82409FE0 size=16
    let mut pc: u32 = 0x82409FE0;
    'dispatch: loop {
        match pc {
            0x82409FE0 => {
    //   block [0x82409FE0..0x82409FF0)
	// 82409FE0: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82409FE4: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 82409FE8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82409FEC: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82409FF0 size=8
    let mut pc: u32 = 0x82409FF0;
    'dispatch: loop {
        match pc {
            0x82409FF0 => {
    //   block [0x82409FF0..0x82409FF8)
	// 82409FF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82409FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82409FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82409FF8 size=92
    let mut pc: u32 = 0x82409FF8;
    'dispatch: loop {
        match pc {
            0x82409FF8 => {
    //   block [0x82409FF8..0x8240A054)
	// 82409FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82409FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240A000: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240A004: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A008: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240A00C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240A010: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240A014: 419A000C  beq cr6, 0x8240a020
	if ctx.cr[6].eq {
	pc = 0x8240A020; continue 'dispatch;
	}
	// 8240A018: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 8240A01C: 48000024  b 0x8240a040
	pc = 0x8240A040; continue 'dispatch;
	// 8240A020: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A024: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240A028: 4182000C  beq 0x8240a034
	if ctx.cr[0].eq {
	pc = 0x8240A034; continue 'dispatch;
	}
	// 8240A02C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8240A030: 48179EB9  bl 0x82583ee8
	ctx.lr = 0x8240A034;
	sub_82583EE8(ctx, base);
	// 8240A034: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240A038: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240A03C: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8240A040: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240A044: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A048: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240A04C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240A050: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240A058 size=112
    let mut pc: u32 = 0x8240A058;
    'dispatch: loop {
        match pc {
            0x8240A058 => {
    //   block [0x8240A058..0x8240A0C8)
	// 8240A058: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A05C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240A060: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240A064: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A068: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240A06C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240A070: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240A074: 419A000C  beq cr6, 0x8240a080
	if ctx.cr[6].eq {
	pc = 0x8240A080; continue 'dispatch;
	}
	// 8240A078: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 8240A07C: 48000038  b 0x8240a0b4
	pc = 0x8240A0B4; continue 'dispatch;
	// 8240A080: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8240A084: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240A088: 409A000C  bne cr6, 0x8240a094
	if !ctx.cr[6].eq {
	pc = 0x8240A094; continue 'dispatch;
	}
	// 8240A08C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240A090: 48000024  b 0x8240a0b4
	pc = 0x8240A0B4; continue 'dispatch;
	// 8240A094: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A098: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240A09C: 4182000C  beq 0x8240a0a8
	if ctx.cr[0].eq {
	pc = 0x8240A0A8; continue 'dispatch;
	}
	// 8240A0A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8240A0A4: 48179E45  bl 0x82583ee8
	ctx.lr = 0x8240A0A8;
	sub_82583EE8(ctx, base);
	// 8240A0A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8240A0AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240A0B0: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 8240A0B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240A0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240A0C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240A0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240A0C8 size=168
    let mut pc: u32 = 0x8240A0C8;
    'dispatch: loop {
        match pc {
            0x8240A0C8 => {
    //   block [0x8240A0C8..0x8240A170)
	// 8240A0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240A0D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240A0D4: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 8240A0D8: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8240A0DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A0E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240A0E4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240A0E8: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240A0EC: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240A0F0: 419A000C  beq cr6, 0x8240a0fc
	if ctx.cr[6].eq {
	pc = 0x8240A0FC; continue 'dispatch;
	}
	// 8240A0F4: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 8240A0F8: 4800005C  b 0x8240a154
	pc = 0x8240A154; continue 'dispatch;
	// 8240A0FC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A100: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240A104: 419A004C  beq cr6, 0x8240a150
	if ctx.cr[6].eq {
	pc = 0x8240A150; continue 'dispatch;
	}
	// 8240A108: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240A10C: C3CB1FF8  lfs f30, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240A110: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 8240A114: 4198002C  blt cr6, 0x8240a140
	if ctx.cr[6].lt {
	pc = 0x8240A140; continue 'dispatch;
	}
	// 8240A118: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240A11C: 4812A13D  bl 0x82534258
	ctx.lr = 0x8240A120;
	sub_82534258(ctx, base);
	// 8240A120: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240A124: 4082001C  bne 0x8240a140
	if !ctx.cr[0].eq {
	pc = 0x8240A140; continue 'dispatch;
	}
	// 8240A128: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240A12C: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240A130: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8240A134: 40990010  ble cr6, 0x8240a144
	if !ctx.cr[6].gt {
	pc = 0x8240A144; continue 'dispatch;
	}
	// 8240A138: FFE00090  fmr f31, f0
	ctx.f[31].f64 = ctx.f[0].f64;
	// 8240A13C: 48000008  b 0x8240a144
	pc = 0x8240A144; continue 'dispatch;
	// 8240A140: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	// 8240A144: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A148: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240A14C: 48179EB5  bl 0x82584000
	ctx.lr = 0x8240A150;
	sub_82584000(ctx, base);
	// 8240A150: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240A154: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240A158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A15C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240A160: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8240A164: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240A168: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240A16C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240A170 size=116
    let mut pc: u32 = 0x8240A170;
    'dispatch: loop {
        match pc {
            0x8240A170 => {
    //   block [0x8240A170..0x8240A1E4)
	// 8240A170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240A178: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A17C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240A180: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240A184: 419A000C  beq cr6, 0x8240a190
	if ctx.cr[6].eq {
	pc = 0x8240A190; continue 'dispatch;
	}
	// 8240A188: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 8240A18C: 48000048  b 0x8240a1d4
	pc = 0x8240A1D4; continue 'dispatch;
	// 8240A190: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240A194: C00B2698  lfs f0, 0x2698(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9880 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240A198: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240A19C: EC210032  fmuls f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240A1A0: C00B2068  lfs f0, 0x2068(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8296 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240A1A4: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240A1A8: 41990014  bgt cr6, 0x8240a1bc
	if ctx.cr[6].gt {
	pc = 0x8240A1BC; continue 'dispatch;
	}
	// 8240A1AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A1B0: C00BC808  lfs f0, -0x37f8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14328 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240A1B4: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240A1B8: 40980008  bge cr6, 0x8240a1c0
	if !ctx.cr[6].lt {
	pc = 0x8240A1C0; continue 'dispatch;
	}
	// 8240A1BC: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 8240A1C0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A1C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240A1C8: 41820008  beq 0x8240a1d0
	if ctx.cr[0].eq {
	pc = 0x8240A1D0; continue 'dispatch;
	}
	// 8240A1CC: 48179EAD  bl 0x82584078
	ctx.lr = 0x8240A1D0;
	sub_82584078(ctx, base);
	// 8240A1D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240A1D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240A1D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A1DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240A1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240A1E8 size=376
    let mut pc: u32 = 0x8240A1E8;
    'dispatch: loop {
        match pc {
            0x8240A1E8 => {
    //   block [0x8240A1E8..0x8240A360)
	// 8240A1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A1EC: 4812AEB9  bl 0x825350a4
	ctx.lr = 0x8240A1F0;
	sub_82535080(ctx, base);
	// 8240A1F0: DBA1FF98  stfd f29, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[29].u64 ) };
	// 8240A1F4: DBC1FFA0  stfd f30, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[30].u64 ) };
	// 8240A1F8: DBE1FFA8  stfd f31, -0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 8240A1FC: 9421FDC0  stwu r1, -0x240(r1)
	ea = ctx.r[1].u32.wrapping_add(-576 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A200: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8240A204: 81770028  lwz r11, 0x28(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240A208: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240A20C: 419A000C  beq cr6, 0x8240a218
	if ctx.cr[6].eq {
	pc = 0x8240A218; continue 'dispatch;
	}
	// 8240A210: 3860FFFE  li r3, -2
	ctx.r[3].s64 = -2;
	// 8240A214: 48000138  b 0x8240a34c
	pc = 0x8240A34C; continue 'dispatch;
	// 8240A218: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8240A21C: 2F060006  cmpwi cr6, r6, 6
	ctx.cr[6].compare_i32(ctx.r[6].s32, 6, &mut ctx.xer);
	// 8240A220: 40990008  ble cr6, 0x8240a228
	if !ctx.cr[6].gt {
	pc = 0x8240A228; continue 'dispatch;
	}
	// 8240A224: 3B200006  li r25, 6
	ctx.r[25].s64 = 6;
	// 8240A228: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8240A22C: 4099008C  ble cr6, 0x8240a2b8
	if !ctx.cr[6].gt {
	pc = 0x8240A2B8; continue 'dispatch;
	}
	// 8240A230: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8240A234: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240A238: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8240A23C: 3BE100B4  addi r31, r1, 0xb4
	ctx.r[31].s64 = ctx.r[1].s64 + 180;
	// 8240A240: 54B8103A  slwi r24, r5, 2
	ctx.r[24].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 8240A244: C3AA1850  lfs f29, 0x1850(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8240A248: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8240A24C: C3CB1FF8  lfs f30, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240A250: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	// 8240A254: 7FBBEB78  mr r27, r29
	ctx.r[27].u64 = ctx.r[29].u64;
	// 8240A258: 3B800006  li r28, 6
	ctx.r[28].s64 = 6;
	// 8240A25C: C3FB0000  lfs f31, 0(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8240A260: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240A264: 48129FF5  bl 0x82534258
	ctx.lr = 0x8240A268;
	sub_82534258(ctx, base);
	// 8240A268: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240A26C: 4082000C  bne 0x8240a278
	if !ctx.cr[0].eq {
	pc = 0x8240A278; continue 'dispatch;
	}
	// 8240A270: FF1FF000  fcmpu cr6, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 8240A274: 4098000C  bge cr6, 0x8240a280
	if !ctx.cr[6].lt {
	pc = 0x8240A280; continue 'dispatch;
	}
	// 8240A278: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	// 8240A27C: 48000010  b 0x8240a28c
	pc = 0x8240A28C; continue 'dispatch;
	// 8240A280: FF1FE800  fcmpu cr6, f31, f29
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[29].f64);
	// 8240A284: 40990008  ble cr6, 0x8240a28c
	if !ctx.cr[6].gt {
	pc = 0x8240A28C; continue 'dispatch;
	}
	// 8240A288: FFE0E890  fmr f31, f29
	ctx.f[31].f64 = ctx.f[29].f64;
	// 8240A28C: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8240A290: D3FF0000  stfs f31, 0(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240A294: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8240A298: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 8240A29C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8240A2A0: 997FFFFC  stb r11, -4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(-4 as u32), ctx.r[11].u8 ) };
	// 8240A2A4: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 8240A2A8: 4082FFB4  bne 0x8240a25c
	if !ctx.cr[0].eq {
	pc = 0x8240A25C; continue 'dispatch;
	}
	// 8240A2AC: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8240A2B0: 7FB8EA14  add r29, r24, r29
	ctx.r[29].u64 = ctx.r[24].u64 + ctx.r[29].u64;
	// 8240A2B4: 4082FFA0  bne 0x8240a254
	if !ctx.cr[0].eq {
	pc = 0x8240A254; continue 'dispatch;
	}
	// 8240A2B8: 1D790006  mulli r11, r25, 6
	ctx.r[11].s64 = ctx.r[25].s64 * 6;
	// 8240A2BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240A2C0: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 8240A2C4: 396100B0  addi r11, r1, 0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + 176;
	// 8240A2C8: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 8240A2CC: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8240A2D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240A2D4: 99610070  stb r11, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u8 ) };
	// 8240A2D8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8240A2DC: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 8240A2E0: 40990028  ble cr6, 0x8240a308
	if !ctx.cr[6].gt {
	pc = 0x8240A308; continue 'dispatch;
	}
	// 8240A2E4: C017002C  lfs f0, 0x2c(r23)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(44 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240A2E8: 39610084  addi r11, r1, 0x84
	ctx.r[11].s64 = ctx.r[1].s64 + 132;
	// 8240A2EC: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 8240A2F0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240A2F4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8240A2F8: 7F0AC800  cmpw cr6, r10, r25
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8240A2FC: 992BFFFC  stb r9, -4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[9].u8 ) };
	// 8240A300: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8240A304: 4198FFE8  blt cr6, 0x8240a2ec
	if ctx.cr[6].lt {
	pc = 0x8240A2EC; continue 'dispatch;
	}
	// 8240A308: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 8240A30C: 80770004  lwz r3, 4(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A310: 9B210058  stb r25, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[25].u8 ) };
	// 8240A314: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240A318: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8240A31C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8240A320: 99610078  stb r11, 0x78(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u8 ) };
	// 8240A324: 39610058  addi r11, r1, 0x58
	ctx.r[11].s64 = ctx.r[1].s64 + 88;
	// 8240A328: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8240A32C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8240A330: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8240A334: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 8240A338: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8240A33C: 4182000C  beq 0x8240a348
	if ctx.cr[0].eq {
	pc = 0x8240A348; continue 'dispatch;
	}
	// 8240A340: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240A344: 48179995  bl 0x82583cd8
	ctx.lr = 0x8240A348;
	sub_82583CD8(ctx, base);
	// 8240A348: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240A34C: 38210240  addi r1, r1, 0x240
	ctx.r[1].s64 = ctx.r[1].s64 + 576;
	// 8240A350: CBA1FF98  lfd f29, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-104 as u32) ) };
	// 8240A354: CBC1FFA0  lfd f30, -0x60(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-96 as u32) ) };
	// 8240A358: CBE1FFA8  lfd f31, -0x58(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 8240A35C: 4812AD98  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240A360 size=24
    let mut pc: u32 = 0x8240A360;
    'dispatch: loop {
        match pc {
            0x8240A360 => {
    //   block [0x8240A360..0x8240A378)
	// 8240A360: 8963000C  lbz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8240A364: 556B07BE  clrlwi r11, r11, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x00000003u64;
	// 8240A368: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8240A36C: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8240A370: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 8240A374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240A378 size=252
    let mut pc: u32 = 0x8240A378;
    'dispatch: loop {
        match pc {
            0x8240A378 => {
    //   block [0x8240A378..0x8240A474)
	// 8240A378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A37C: 4812AD3D  bl 0x825350b8
	ctx.lr = 0x8240A380;
	sub_82535080(ctx, base);
	// 8240A380: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A384: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240A388: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8240A38C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240A390: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8240A394: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 8240A398: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240A39C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8240A3A0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8240A3A4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8240A3A8: 4812AE29  bl 0x825351d0
	ctx.lr = 0x8240A3AC;
	sub_825351D0(ctx, base);
	// 8240A3AC: 57EB057F  clrlwi. r11, r31, 0x15
	ctx.r[11].u64 = ctx.r[31].u32 as u64 & 0x000007FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240A3B0: 4182001C  beq 0x8240a3cc
	if ctx.cr[0].eq {
	pc = 0x8240A3CC; continue 'dispatch;
	}
	// 8240A3B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A3B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240A3BC: 386BD118  addi r3, r11, -0x2ee8
	ctx.r[3].s64 = ctx.r[11].s64 + -12008;
	// 8240A3C0: 4BEA8BC1  bl 0x822b2f80
	ctx.lr = 0x8240A3C4;
	sub_822B2F80(ctx, base);
	// 8240A3C4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240A3C8: 480000A4  b 0x8240a46c
	pc = 0x8240A46C; continue 'dispatch;
	// 8240A3CC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8240A3D0: 409A0018  bne cr6, 0x8240a3e8
	if !ctx.cr[6].eq {
	pc = 0x8240A3E8; continue 'dispatch;
	}
	// 8240A3D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A3D8: 386BD0E0  addi r3, r11, -0x2f20
	ctx.r[3].s64 = ctx.r[11].s64 + -12064;
	// 8240A3DC: 4BEA8BA5  bl 0x822b2f80
	ctx.lr = 0x8240A3E0;
	sub_822B2F80(ctx, base);
	// 8240A3E0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240A3E4: 48000088  b 0x8240a46c
	pc = 0x8240A46C; continue 'dispatch;
	// 8240A3E8: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8240A3EC: A13E0008  lhz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240A3F0: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8240A3F4: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 8240A3F8: 2C090000  cmpwi r9, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240A3FC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8240A400: 40810040  ble 0x8240a440
	if !ctx.cr[0].gt {
	pc = 0x8240A440; continue 'dispatch;
	}
	// 8240A404: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8240A408: 395E001C  addi r10, r30, 0x1c
	ctx.r[10].s64 = ctx.r[30].s64 + 28;
	// 8240A40C: 80EAFFF8  lwz r7, -8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A410: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8240A414: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240A418: 80CAFFFC  lwz r6, -4(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) } as u64;
	// 8240A41C: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 8240A420: 90EBFFFC  stw r7, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[7].u32 ) };
	// 8240A424: 5507E13E  srwi r7, r8, 4
	ctx.r[7].u32 = ctx.r[8].u32.wrapping_shr(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 8240A428: 5508073E  clrlwi r8, r8, 0x1c
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x0000000Fu64;
	// 8240A42C: 90CB0000  stw r6, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 8240A430: 98EB0004  stb r7, 4(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u8 ) };
	// 8240A434: 990B0005  stb r8, 5(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(5 as u32), ctx.r[8].u8 ) };
	// 8240A438: 396B000C  addi r11, r11, 0xc
	ctx.r[11].s64 = ctx.r[11].s64 + 12;
	// 8240A43C: 4082FFD0  bne 0x8240a40c
	if !ctx.cr[0].eq {
	pc = 0x8240A40C; continue 'dispatch;
	}
	// 8240A440: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8240A444: 807C0004  lwz r3, 4(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A448: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240A44C: 4817993D  bl 0x82583d88
	ctx.lr = 0x8240A450;
	sub_82583D88(ctx, base);
	// 8240A450: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240A454: 40800014  bge 0x8240a468
	if !ctx.cr[0].lt {
	pc = 0x8240A468; continue 'dispatch;
	}
	// 8240A458: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A45C: 386BD0B4  addi r3, r11, -0x2f4c
	ctx.r[3].s64 = ctx.r[11].s64 + -12108;
	// 8240A460: 4BEA8B21  bl 0x822b2f80
	ctx.lr = 0x8240A464;
	sub_822B2F80(ctx, base);
	// 8240A464: 4BFFFF7C  b 0x8240a3e0
	pc = 0x8240A3E0; continue 'dispatch;
	// 8240A468: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240A46C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 8240A470: 4812AC98  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240A478 size=196
    let mut pc: u32 = 0x8240A478;
    'dispatch: loop {
        match pc {
            0x8240A478 => {
    //   block [0x8240A478..0x8240A53C)
	// 8240A478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A47C: 4812AC3D  bl 0x825350b8
	ctx.lr = 0x8240A480;
	sub_82535080(ctx, base);
	// 8240A480: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A484: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240A488: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240A48C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8240A490: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8240A494: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 8240A498: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240A49C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8240A4A0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8240A4A4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8240A4A8: 4812AD29  bl 0x825351d0
	ctx.lr = 0x8240A4AC;
	sub_825351D0(ctx, base);
	// 8240A4AC: 57CB057F  clrlwi. r11, r30, 0x15
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000007FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240A4B0: 4182001C  beq 0x8240a4cc
	if ctx.cr[0].eq {
	pc = 0x8240A4CC; continue 'dispatch;
	}
	// 8240A4B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A4B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240A4BC: 386BD1D8  addi r3, r11, -0x2e28
	ctx.r[3].s64 = ctx.r[11].s64 + -11816;
	// 8240A4C0: 4BEA8AC1  bl 0x822b2f80
	ctx.lr = 0x8240A4C4;
	sub_822B2F80(ctx, base);
	// 8240A4C4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240A4C8: 4800006C  b 0x8240a534
	pc = 0x8240A534; continue 'dispatch;
	// 8240A4CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240A4D0: 409A0018  bne cr6, 0x8240a4e8
	if !ctx.cr[6].eq {
	pc = 0x8240A4E8; continue 'dispatch;
	}
	// 8240A4D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A4D8: 386BD198  addi r3, r11, -0x2e68
	ctx.r[3].s64 = ctx.r[11].s64 + -11880;
	// 8240A4DC: 4BEA8AA5  bl 0x822b2f80
	ctx.lr = 0x8240A4E0;
	sub_822B2F80(ctx, base);
	// 8240A4E0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240A4E4: 48000050  b 0x8240a534
	pc = 0x8240A534; continue 'dispatch;
	// 8240A4E8: 897F0003  lbz r11, 3(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(3 as u32) ) } as u64;
	// 8240A4EC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8240A4F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240A4F4: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A4F8: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 8240A4FC: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 8240A500: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8240A504: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A508: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8240A50C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240A510: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8240A514: 4817992D  bl 0x82583e40
	ctx.lr = 0x8240A518;
	sub_82583E40(ctx, base);
	// 8240A518: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240A51C: 40800014  bge 0x8240a530
	if !ctx.cr[0].lt {
	pc = 0x8240A530; continue 'dispatch;
	}
	// 8240A520: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A524: 386BD168  addi r3, r11, -0x2e98
	ctx.r[3].s64 = ctx.r[11].s64 + -11928;
	// 8240A528: 4BEA8A59  bl 0x822b2f80
	ctx.lr = 0x8240A52C;
	sub_822B2F80(ctx, base);
	// 8240A52C: 4BFFFFB4  b 0x8240a4e0
	pc = 0x8240A4E0; continue 'dispatch;
	// 8240A530: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240A534: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8240A538: 4812ABD0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240A540 size=664
    let mut pc: u32 = 0x8240A540;
    'dispatch: loop {
        match pc {
            0x8240A540 => {
    //   block [0x8240A540..0x8240A7D8)
	// 8240A540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240A548: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A54C: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8240A550: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240A554: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8240A558: 38E00003  li r7, 3
	ctx.r[7].s64 = 3;
	// 8240A55C: 39000004  li r8, 4
	ctx.r[8].s64 = 4;
	// 8240A560: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240A564: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8240A568: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 8240A56C: D0010084  stfs f0, 0x84(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), tmp.u32 ) };
	// 8240A570: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8240A574: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8240A578: D0010094  stfs f0, 0x94(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), tmp.u32 ) };
	// 8240A57C: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 8240A580: D001009C  stfs f0, 0x9c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), tmp.u32 ) };
	// 8240A584: 99610081  stb r11, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[11].u8 ) };
	// 8240A588: D00100A4  stfs f0, 0xa4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), tmp.u32 ) };
	// 8240A58C: 99610088  stb r11, 0x88(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u8 ) };
	// 8240A590: D00100AC  stfs f0, 0xac(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), tmp.u32 ) };
	// 8240A594: 98C10089  stb r6, 0x89(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(137 as u32), ctx.r[6].u8 ) };
	// 8240A598: D00100B4  stfs f0, 0xb4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), tmp.u32 ) };
	// 8240A59C: 99610090  stb r11, 0x90(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[11].u8 ) };
	// 8240A5A0: D00100BC  stfs f0, 0xbc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 8240A5A4: 99410091  stb r10, 0x91(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(145 as u32), ctx.r[10].u8 ) };
	// 8240A5A8: D00100C4  stfs f0, 0xc4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), tmp.u32 ) };
	// 8240A5AC: 99610098  stb r11, 0x98(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[11].u8 ) };
	// 8240A5B0: D00100CC  stfs f0, 0xcc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(204 as u32), tmp.u32 ) };
	// 8240A5B4: 98E10099  stb r7, 0x99(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(153 as u32), ctx.r[7].u8 ) };
	// 8240A5B8: D00100D4  stfs f0, 0xd4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 8240A5BC: 996100A0  stb r11, 0xa0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[11].u8 ) };
	// 8240A5C0: D00100DC  stfs f0, 0xdc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(220 as u32), tmp.u32 ) };
	// 8240A5C4: 990100A1  stb r8, 0xa1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(161 as u32), ctx.r[8].u8 ) };
	// 8240A5C8: D00100E4  stfs f0, 0xe4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 8240A5CC: 996100A8  stb r11, 0xa8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[11].u8 ) };
	// 8240A5D0: D00100EC  stfs f0, 0xec(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 8240A5D4: 992100A9  stb r9, 0xa9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(169 as u32), ctx.r[9].u8 ) };
	// 8240A5D8: D00100F4  stfs f0, 0xf4(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8240A5DC: 98C100B0  stb r6, 0xb0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[6].u8 ) };
	// 8240A5E0: D00100FC  stfs f0, 0xfc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(252 as u32), tmp.u32 ) };
	// 8240A5E4: 996100B1  stb r11, 0xb1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(177 as u32), ctx.r[11].u8 ) };
	// 8240A5E8: D0010104  stfs f0, 0x104(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), tmp.u32 ) };
	// 8240A5EC: 98C100B8  stb r6, 0xb8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[6].u8 ) };
	// 8240A5F0: D001010C  stfs f0, 0x10c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 8240A5F4: 98C100B9  stb r6, 0xb9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(185 as u32), ctx.r[6].u8 ) };
	// 8240A5F8: D0010114  stfs f0, 0x114(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(276 as u32), tmp.u32 ) };
	// 8240A5FC: 98C100C0  stb r6, 0xc0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[6].u8 ) };
	// 8240A600: D001011C  stfs f0, 0x11c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(284 as u32), tmp.u32 ) };
	// 8240A604: 994100C1  stb r10, 0xc1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(193 as u32), ctx.r[10].u8 ) };
	// 8240A608: D0010124  stfs f0, 0x124(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(292 as u32), tmp.u32 ) };
	// 8240A60C: 98C100C8  stb r6, 0xc8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(200 as u32), ctx.r[6].u8 ) };
	// 8240A610: D001012C  stfs f0, 0x12c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), tmp.u32 ) };
	// 8240A614: 98E100C9  stb r7, 0xc9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(201 as u32), ctx.r[7].u8 ) };
	// 8240A618: D0010134  stfs f0, 0x134(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), tmp.u32 ) };
	// 8240A61C: 98C100D0  stb r6, 0xd0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(208 as u32), ctx.r[6].u8 ) };
	// 8240A620: D001013C  stfs f0, 0x13c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(316 as u32), tmp.u32 ) };
	// 8240A624: 990100D1  stb r8, 0xd1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(209 as u32), ctx.r[8].u8 ) };
	// 8240A628: D0010144  stfs f0, 0x144(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(324 as u32), tmp.u32 ) };
	// 8240A62C: 98C100D8  stb r6, 0xd8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(216 as u32), ctx.r[6].u8 ) };
	// 8240A630: D001014C  stfs f0, 0x14c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(332 as u32), tmp.u32 ) };
	// 8240A634: 992100D9  stb r9, 0xd9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(217 as u32), ctx.r[9].u8 ) };
	// 8240A638: D0010154  stfs f0, 0x154(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(340 as u32), tmp.u32 ) };
	// 8240A63C: 994100E0  stb r10, 0xe0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(224 as u32), ctx.r[10].u8 ) };
	// 8240A640: 996100E1  stb r11, 0xe1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(225 as u32), ctx.r[11].u8 ) };
	// 8240A644: 994100E8  stb r10, 0xe8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[10].u8 ) };
	// 8240A648: 98C100E9  stb r6, 0xe9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(233 as u32), ctx.r[6].u8 ) };
	// 8240A64C: 994100F0  stb r10, 0xf0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[10].u8 ) };
	// 8240A650: 994100F1  stb r10, 0xf1(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(241 as u32), ctx.r[10].u8 ) };
	// 8240A654: 994100F8  stb r10, 0xf8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(248 as u32), ctx.r[10].u8 ) };
	// 8240A658: 98E100F9  stb r7, 0xf9(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(249 as u32), ctx.r[7].u8 ) };
	// 8240A65C: 99410100  stb r10, 0x100(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(256 as u32), ctx.r[10].u8 ) };
	// 8240A660: 99010101  stb r8, 0x101(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(257 as u32), ctx.r[8].u8 ) };
	// 8240A664: 99410108  stb r10, 0x108(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(264 as u32), ctx.r[10].u8 ) };
	// 8240A668: 99210109  stb r9, 0x109(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(265 as u32), ctx.r[9].u8 ) };
	// 8240A66C: 98E10110  stb r7, 0x110(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(272 as u32), ctx.r[7].u8 ) };
	// 8240A670: 99610111  stb r11, 0x111(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(273 as u32), ctx.r[11].u8 ) };
	// 8240A674: 98E10118  stb r7, 0x118(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(280 as u32), ctx.r[7].u8 ) };
	// 8240A678: 98C10119  stb r6, 0x119(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(281 as u32), ctx.r[6].u8 ) };
	// 8240A67C: 98E10120  stb r7, 0x120(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(288 as u32), ctx.r[7].u8 ) };
	// 8240A680: 99410121  stb r10, 0x121(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(289 as u32), ctx.r[10].u8 ) };
	// 8240A684: 98E10128  stb r7, 0x128(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(296 as u32), ctx.r[7].u8 ) };
	// 8240A688: 98E10129  stb r7, 0x129(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(297 as u32), ctx.r[7].u8 ) };
	// 8240A68C: 98E10130  stb r7, 0x130(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(304 as u32), ctx.r[7].u8 ) };
	// 8240A690: 99210139  stb r9, 0x139(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(313 as u32), ctx.r[9].u8 ) };
	// 8240A694: D001015C  stfs f0, 0x15c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(348 as u32), tmp.u32 ) };
	// 8240A698: 99210169  stb r9, 0x169(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(361 as u32), ctx.r[9].u8 ) };
	// 8240A69C: D0010164  stfs f0, 0x164(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(356 as u32), tmp.u32 ) };
	// 8240A6A0: 99210170  stb r9, 0x170(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(368 as u32), ctx.r[9].u8 ) };
	// 8240A6A4: D001016C  stfs f0, 0x16c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(364 as u32), tmp.u32 ) };
	// 8240A6A8: 99210178  stb r9, 0x178(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(376 as u32), ctx.r[9].u8 ) };
	// 8240A6AC: D0010174  stfs f0, 0x174(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(372 as u32), tmp.u32 ) };
	// 8240A6B0: 99210180  stb r9, 0x180(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(384 as u32), ctx.r[9].u8 ) };
	// 8240A6B4: D001017C  stfs f0, 0x17c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(380 as u32), tmp.u32 ) };
	// 8240A6B8: 99210188  stb r9, 0x188(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(392 as u32), ctx.r[9].u8 ) };
	// 8240A6BC: D0010184  stfs f0, 0x184(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(388 as u32), tmp.u32 ) };
	// 8240A6C0: 99210190  stb r9, 0x190(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(400 as u32), ctx.r[9].u8 ) };
	// 8240A6C4: D001018C  stfs f0, 0x18c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(396 as u32), tmp.u32 ) };
	// 8240A6C8: 99210198  stb r9, 0x198(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(408 as u32), ctx.r[9].u8 ) };
	// 8240A6CC: D0010194  stfs f0, 0x194(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(404 as u32), tmp.u32 ) };
	// 8240A6D0: 99210199  stb r9, 0x199(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(409 as u32), ctx.r[9].u8 ) };
	// 8240A6D4: D001019C  stfs f0, 0x19c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(412 as u32), tmp.u32 ) };
	// 8240A6D8: 99210078  stb r9, 0x78(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[9].u8 ) };
	// 8240A6DC: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 8240A6E0: 99610141  stb r11, 0x141(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(321 as u32), ctx.r[11].u8 ) };
	// 8240A6E4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8240A6E8: 99610171  stb r11, 0x171(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(369 as u32), ctx.r[11].u8 ) };
	// 8240A6EC: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8240A6F0: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8240A6F4: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8240A6F8: 99610051  stb r11, 0x51(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(81 as u32), ctx.r[11].u8 ) };
	// 8240A6FC: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8240A700: 992101A8  stb r9, 0x1a8(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(424 as u32), ctx.r[9].u8 ) };
	// 8240A704: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8240A708: 99610059  stb r11, 0x59(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(89 as u32), ctx.r[11].u8 ) };
	// 8240A70C: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8240A710: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 8240A714: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8240A718: 99610069  stb r11, 0x69(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(105 as u32), ctx.r[11].u8 ) };
	// 8240A71C: 99610071  stb r11, 0x71(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(113 as u32), ctx.r[11].u8 ) };
	// 8240A720: 99610079  stb r11, 0x79(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(121 as u32), ctx.r[11].u8 ) };
	// 8240A724: 912101AC  stw r9, 0x1ac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(428 as u32), ctx.r[9].u32 ) };
	// 8240A728: 39200024  li r9, 0x24
	ctx.r[9].s64 = 36;
	// 8240A72C: 916101C0  stw r11, 0x1c0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(448 as u32), ctx.r[11].u32 ) };
	// 8240A730: 396101B0  addi r11, r1, 0x1b0
	ctx.r[11].s64 = ctx.r[1].s64 + 432;
	// 8240A734: 80650004  lwz r3, 4(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A738: 99410151  stb r10, 0x151(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(337 as u32), ctx.r[10].u8 ) };
	// 8240A73C: 99410181  stb r10, 0x181(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(385 as u32), ctx.r[10].u8 ) };
	// 8240A740: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240A744: 99410060  stb r10, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u8 ) };
	// 8240A748: 992101B0  stb r9, 0x1b0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(432 as u32), ctx.r[9].u8 ) };
	// 8240A74C: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 8240A750: 994101A0  stb r10, 0x1a0(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(416 as u32), ctx.r[10].u8 ) };
	// 8240A754: 394101C0  addi r10, r1, 0x1c0
	ctx.r[10].s64 = ctx.r[1].s64 + 448;
	// 8240A758: 916101C4  stw r11, 0x1c4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(452 as u32), ctx.r[11].u32 ) };
	// 8240A75C: 396101A8  addi r11, r1, 0x1a8
	ctx.r[11].s64 = ctx.r[1].s64 + 424;
	// 8240A760: 80A50008  lwz r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240A764: 99010131  stb r8, 0x131(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(305 as u32), ctx.r[8].u8 ) };
	// 8240A768: 98E10138  stb r7, 0x138(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(312 as u32), ctx.r[7].u8 ) };
	// 8240A76C: 99010140  stb r8, 0x140(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[8].u8 ) };
	// 8240A770: 99010148  stb r8, 0x148(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(328 as u32), ctx.r[8].u8 ) };
	// 8240A774: 98C10149  stb r6, 0x149(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(329 as u32), ctx.r[6].u8 ) };
	// 8240A778: 99010150  stb r8, 0x150(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(336 as u32), ctx.r[8].u8 ) };
	// 8240A77C: 99010158  stb r8, 0x158(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(344 as u32), ctx.r[8].u8 ) };
	// 8240A780: 90A101C8  stw r5, 0x1c8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(456 as u32), ctx.r[5].u32 ) };
	// 8240A784: 98E10159  stb r7, 0x159(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(345 as u32), ctx.r[7].u8 ) };
	// 8240A788: 99010160  stb r8, 0x160(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(352 as u32), ctx.r[8].u8 ) };
	// 8240A78C: 99010161  stb r8, 0x161(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(353 as u32), ctx.r[8].u8 ) };
	// 8240A790: 99010168  stb r8, 0x168(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(360 as u32), ctx.r[8].u8 ) };
	// 8240A794: 98C10179  stb r6, 0x179(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(377 as u32), ctx.r[6].u8 ) };
	// 8240A798: 98E10189  stb r7, 0x189(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(393 as u32), ctx.r[7].u8 ) };
	// 8240A79C: 99010191  stb r8, 0x191(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(401 as u32), ctx.r[8].u8 ) };
	// 8240A7A0: 98C10058  stb r6, 0x58(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u8 ) };
	// 8240A7A4: 98E10068  stb r7, 0x68(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u8 ) };
	// 8240A7A8: 99010070  stb r8, 0x70(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u8 ) };
	// 8240A7AC: 912101B4  stw r9, 0x1b4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(436 as u32), ctx.r[9].u32 ) };
	// 8240A7B0: 914101A4  stw r10, 0x1a4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(420 as u32), ctx.r[10].u32 ) };
	// 8240A7B4: 916101CC  stw r11, 0x1cc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(460 as u32), ctx.r[11].u32 ) };
	// 8240A7B8: 4182000C  beq 0x8240a7c4
	if ctx.cr[0].eq {
	pc = 0x8240A7C4; continue 'dispatch;
	}
	// 8240A7BC: 388101A0  addi r4, r1, 0x1a0
	ctx.r[4].s64 = ctx.r[1].s64 + 416;
	// 8240A7C0: 481794C1  bl 0x82583c80
	ctx.lr = 0x8240A7C4;
	sub_82583C80(ctx, base);
	// 8240A7C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240A7C8: 382101E0  addi r1, r1, 0x1e0
	ctx.r[1].s64 = ctx.r[1].s64 + 480;
	// 8240A7CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A7D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240A7D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240A7D8 size=40
    let mut pc: u32 = 0x8240A7D8;
    'dispatch: loop {
        match pc {
            0x8240A7D8 => {
    //   block [0x8240A7D8..0x8240A800)
	// 8240A7D8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240A7DC: D023002C  stfs f1, 0x2c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8240A7E0: D0430030  stfs f2, 0x30(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8240A7E4: C00B1FF8  lfs f0, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240A7E8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240A7EC: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240A7F0: C1AB1850  lfs f13, 0x1850(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240A7F4: 4098000C  bge cr6, 0x8240a800
	if !ctx.cr[6].lt {
		sub_8240A800(ctx, base);
		return;
	}
	// 8240A7F8: D003002C  stfs f0, 0x2c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8240A7FC: 48000010  b 0x8240a80c
	sub_8240A800(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240A800 size=28
    let mut pc: u32 = 0x8240A800;
    'dispatch: loop {
        match pc {
            0x8240A800 => {
    //   block [0x8240A800..0x8240A81C)
	// 8240A800: FF016800  fcmpu cr6, f1, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 8240A804: 40990008  ble cr6, 0x8240a80c
	if !ctx.cr[6].gt {
	pc = 0x8240A80C; continue 'dispatch;
	}
	// 8240A808: D1A3002C  stfs f13, 0x2c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8240A80C: FF020000  fcmpu cr6, f2, f0
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[0].f64);
	// 8240A810: 4098000C  bge cr6, 0x8240a81c
	if !ctx.cr[6].lt {
		sub_8240A81C(ctx, base);
		return;
	}
	// 8240A814: D0030030  stfs f0, 0x30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8240A818: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A81C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240A81C size=8
    let mut pc: u32 = 0x8240A81C;
    'dispatch: loop {
        match pc {
            0x8240A81C => {
    //   block [0x8240A81C..0x8240A824)
	// 8240A81C: FF026800  fcmpu cr6, f2, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[2].f64, ctx.f[13].f64);
	// 8240A820: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A824(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240A824 size=8
    let mut pc: u32 = 0x8240A824;
    'dispatch: loop {
        match pc {
            0x8240A824 => {
    //   block [0x8240A824..0x8240A82C)
	// 8240A824: D1A30030  stfs f13, 0x30(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8240A828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240A830 size=88
    let mut pc: u32 = 0x8240A830;
    'dispatch: loop {
        match pc {
            0x8240A830 => {
    //   block [0x8240A830..0x8240A888)
	// 8240A830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A834: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240A838: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240A83C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A840: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240A844: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240A848: 40980010  bge cr6, 0x8240a858
	if !ctx.cr[6].lt {
	pc = 0x8240A858; continue 'dispatch;
	}
	// 8240A84C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240A850: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240A854: 4800001C  b 0x8240a870
	pc = 0x8240A870; continue 'dispatch;
	// 8240A858: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240A85C: 909F0034  stw r4, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[4].u32 ) };
	// 8240A860: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240A864: 4BFFD0BD  bl 0x82407920
	ctx.lr = 0x8240A868;
	sub_82407920(ctx, base);
	// 8240A868: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240A86C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240A870: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240A874: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240A878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240A880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240A884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240A888 size=4
    let mut pc: u32 = 0x8240A888;
    'dispatch: loop {
        match pc {
            0x8240A888 => {
    //   block [0x8240A888..0x8240A88C)
	// 8240A888: 4BFFF688  b 0x82409f10
	sub_82409F10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240A890 size=84
    let mut pc: u32 = 0x8240A890;
    'dispatch: loop {
        match pc {
            0x8240A890 => {
    //   block [0x8240A890..0x8240A8E4)
	// 8240A890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A894: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240A898: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240A89C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A8A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240A8A4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8240A8A8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8240A8AC: 409A0024  bne cr6, 0x8240a8d0
	if !ctx.cr[6].eq {
	pc = 0x8240A8D0; continue 'dispatch;
	}
	// 8240A8B0: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 8240A8B4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A8B8: 48179479  bl 0x82583d30
	ctx.lr = 0x8240A8BC;
	sub_82583D30(ctx, base);
	// 8240A8BC: 897F000C  lbz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8240A8C0: 556B06B5  rlwinm. r11, r11, 0, 0x1a, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240A8C4: 4182000C  beq 0x8240a8d0
	if ctx.cr[0].eq {
	pc = 0x8240A8D0; continue 'dispatch;
	}
	// 8240A8C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240A8CC: 4BFFF645  bl 0x82409f10
	ctx.lr = 0x8240A8D0;
	sub_82409F10(ctx, base);
	// 8240A8D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240A8D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240A8D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240A8DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240A8E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240A8E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240A8E8 size=352
    let mut pc: u32 = 0x8240A8E8;
    'dispatch: loop {
        match pc {
            0x8240A8E8 => {
    //   block [0x8240A8E8..0x8240AA48)
	// 8240A8E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240A8EC: 4812A7B5  bl 0x825350a0
	ctx.lr = 0x8240A8F0;
	sub_82535080(ctx, base);
	// 8240A8F0: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240A8F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240A8F8: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 8240A8FC: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 8240A900: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 8240A904: 2B040FFF  cmplwi cr6, r4, 0xfff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 4095 as u32, &mut ctx.xer);
	// 8240A908: 41990128  bgt cr6, 0x8240aa30
	if ctx.cr[6].gt {
	pc = 0x8240AA30; continue 'dispatch;
	}
	// 8240A90C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240A910: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8240A914: 4800527D  bl 0x8240fb90
	ctx.lr = 0x8240A918;
	sub_8240FB90(ctx, base);
	// 8240A918: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240A91C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240A920: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 8240A924: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240A928: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8240A92C: 838B0010  lwz r28, 0x10(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240A930: 836B000C  lwz r27, 0xc(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8240A934: 83EB0014  lwz r31, 0x14(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240A938: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 8240A93C: 4812A895  bl 0x825351d0
	ctx.lr = 0x8240A940;
	sub_825351D0(ctx, base);
	// 8240A940: A17F000E  lhz r11, 0xe(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(14 as u32) ) } as u64;
	// 8240A944: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 8240A948: 3B400002  li r26, 2
	ctx.r[26].s64 = 2;
	// 8240A94C: 2B0B0010  cmplwi cr6, r11, 0x10
	ctx.cr[6].compare_u32(ctx.r[11].u32, 16 as u32, &mut ctx.xer);
	// 8240A950: 409A000C  bne cr6, 0x8240a95c
	if !ctx.cr[6].eq {
	pc = 0x8240A95C; continue 'dispatch;
	}
	// 8240A954: 9B210050  stb r25, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[25].u8 ) };
	// 8240A958: 48000020  b 0x8240a978
	pc = 0x8240A978; continue 'dispatch;
	// 8240A95C: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 8240A960: 409A000C  bne cr6, 0x8240a96c
	if !ctx.cr[6].eq {
	pc = 0x8240A96C; continue 'dispatch;
	}
	// 8240A964: 9B410050  stb r26, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u8 ) };
	// 8240A968: 48000010  b 0x8240a978
	pc = 0x8240A978; continue 'dispatch;
	// 8240A96C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A970: 386BD2B0  addi r3, r11, -0x2d50
	ctx.r[3].s64 = ctx.r[11].s64 + -11600;
	// 8240A974: 4BEA860D  bl 0x822b2f80
	ctx.lr = 0x8240A978;
	sub_822B2F80(ctx, base);
	// 8240A978: A17F0002  lhz r11, 2(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 8240A97C: 3957FFFF  addi r10, r23, -1
	ctx.r[10].s64 = ctx.r[23].s64 + -1;
	// 8240A980: 813F0004  lwz r9, 4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240A984: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 8240A988: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 8240A98C: 9B410089  stb r26, 0x89(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(137 as u32), ctx.r[26].u8 ) };
	// 8240A990: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240A994: 9B21008B  stb r25, 0x8b(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(139 as u32), ctx.r[25].u8 ) };
	// 8240A998: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240A99C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 8240A9A0: 554BDFFE  rlwinm r11, r10, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8240A9A4: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 8240A9A8: 39200024  li r9, 0x24
	ctx.r[9].s64 = 36;
	// 8240A9AC: 99610088  stb r11, 0x88(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u8 ) };
	// 8240A9B0: 9921008A  stb r9, 0x8a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(138 as u32), ctx.r[9].u8 ) };
	// 8240A9B4: 48179915  bl 0x825842c8
	ctx.lr = 0x8240A9B8;
	sub_825842C8(ctx, base);
	// 8240A9B8: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240A9BC: 40800010  bge 0x8240a9cc
	if !ctx.cr[0].lt {
	pc = 0x8240A9CC; continue 'dispatch;
	}
	// 8240A9C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240A9C4: 386BD278  addi r3, r11, -0x2d88
	ctx.r[3].s64 = ctx.r[11].s64 + -11656;
	// 8240A9C8: 4BEA85B9  bl 0x822b2f80
	ctx.lr = 0x8240A9CC;
	sub_822B2F80(ctx, base);
	// 8240A9CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240A9D0: A09F0002  lhz r4, 2(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(2 as u32) ) } as u64;
	// 8240A9D4: 4BFFFB6D  bl 0x8240a540
	ctx.lr = 0x8240A9D8;
	sub_8240A540(ctx, base);
	// 8240A9D8: 38A00054  li r5, 0x54
	ctx.r[5].s64 = 84;
	// 8240A9DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240A9E0: 386100B4  addi r3, r1, 0xb4
	ctx.r[3].s64 = ctx.r[1].s64 + 180;
	// 8240A9E4: 4812A7ED  bl 0x825351d0
	ctx.lr = 0x8240A9E8;
	sub_825351D0(ctx, base);
	// 8240A9E8: 21780000  subfic r11, r24, 0
	ctx.xer.ca = ctx.r[24].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[24].s64;
	// 8240A9EC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8240A9F0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240A9F4: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 8240A9F8: 938100B0  stw r28, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[28].u32 ) };
	// 8240A9FC: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 8240AA00: 936100B4  stw r27, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[27].u32 ) };
	// 8240AA04: 92C100BC  stw r22, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[22].u32 ) };
	// 8240AA08: 930100C0  stw r24, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[24].u32 ) };
	// 8240AA0C: 916100B8  stw r11, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 8240AA10: 48179379  bl 0x82583d88
	ctx.lr = 0x8240AA14;
	sub_82583D88(ctx, base);
	// 8240AA14: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240AA18: 40800010  bge 0x8240aa28
	if !ctx.cr[0].lt {
	pc = 0x8240AA28; continue 'dispatch;
	}
	// 8240AA1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240AA20: 386BD0B4  addi r3, r11, -0x2f4c
	ctx.r[3].s64 = ctx.r[11].s64 + -12108;
	// 8240AA24: 4BEA855D  bl 0x822b2f80
	ctx.lr = 0x8240AA28;
	sub_822B2F80(ctx, base);
	// 8240AA28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240AA2C: 48000014  b 0x8240aa40
	pc = 0x8240AA40; continue 'dispatch;
	// 8240AA30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240AA34: 386BD230  addi r3, r11, -0x2dd0
	ctx.r[3].s64 = ctx.r[11].s64 + -11728;
	// 8240AA38: 4BEA8549  bl 0x822b2f80
	ctx.lr = 0x8240AA3C;
	sub_822B2F80(ctx, base);
	// 8240AA3C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240AA40: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 8240AA44: 4812A6AC  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AA48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240AA48 size=300
    let mut pc: u32 = 0x8240AA48;
    'dispatch: loop {
        match pc {
            0x8240AA48 => {
    //   block [0x8240AA48..0x8240AB74)
	// 8240AA48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240AA4C: 4812A66D  bl 0x825350b8
	ctx.lr = 0x8240AA50;
	sub_82535080(ctx, base);
	// 8240AA50: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AA54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240AA58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240AA5C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240AA60: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8240AA64: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 8240AA68: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240AA6C: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8240AA70: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 8240AA74: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8240AA78: 4812A759  bl 0x825351d0
	ctx.lr = 0x8240AA7C;
	sub_825351D0(ctx, base);
	// 8240AA7C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240AA80: 409A0018  bne cr6, 0x8240aa98
	if !ctx.cr[6].eq {
	pc = 0x8240AA98; continue 'dispatch;
	}
	// 8240AA84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240AA88: 386BD2D0  addi r3, r11, -0x2d30
	ctx.r[3].s64 = ctx.r[11].s64 + -11568;
	// 8240AA8C: 4BEA84F5  bl 0x822b2f80
	ctx.lr = 0x8240AA90;
	sub_822B2F80(ctx, base);
	// 8240AA90: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240AA94: 480000D8  b 0x8240ab6c
	pc = 0x8240AB6C; continue 'dispatch;
	// 8240AA98: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8240AA9C: 409A000C  bne cr6, 0x8240aaa8
	if !ctx.cr[6].eq {
	pc = 0x8240AAA8; continue 'dispatch;
	}
	// 8240AAA0: A17F0008  lhz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240AAA4: 48000008  b 0x8240aaac
	pc = 0x8240AAAC; continue 'dispatch;
	// 8240AAA8: 897F0001  lbz r11, 1(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1 as u32) ) } as u64;
	// 8240AAAC: 5569043F  clrlwi. r9, r11, 0x10
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x0000FFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8240AAB0: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 8240AAB4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 8240AAB8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8240AABC: 99610089  stb r11, 0x89(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(137 as u32), ctx.r[11].u8 ) };
	// 8240AAC0: 39600024  li r11, 0x24
	ctx.r[11].s64 = 36;
	// 8240AAC4: C00A2068  lfs f0, 0x2068(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8296 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240AAC8: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 8240AACC: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 8240AAD0: 9961008A  stb r11, 0x8a(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(138 as u32), ctx.r[11].u8 ) };
	// 8240AAD4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8240AAD8: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 8240AADC: 9961008B  stb r11, 0x8b(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(139 as u32), ctx.r[11].u8 ) };
	// 8240AAE0: 40810054  ble 0x8240ab34
	if !ctx.cr[0].gt {
	pc = 0x8240AB34; continue 'dispatch;
	}
	// 8240AAE4: 3961005C  addi r11, r1, 0x5c
	ctx.r[11].s64 = ctx.r[1].s64 + 92;
	// 8240AAE8: 38FF0028  addi r7, r31, 0x28
	ctx.r[7].s64 = ctx.r[31].s64 + 40;
	// 8240AAEC: 395F001D  addi r10, r31, 0x1d
	ctx.r[10].s64 = ctx.r[31].s64 + 29;
	// 8240AAF0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8240AAF4: 409A000C  bne cr6, 0x8240ab00
	if !ctx.cr[6].eq {
	pc = 0x8240AB00; continue 'dispatch;
	}
	// 8240AAF8: 810AFFF3  lwz r8, -0xd(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-13 as u32) ) } as u64;
	// 8240AAFC: 48000008  b 0x8240ab04
	pc = 0x8240AB04; continue 'dispatch;
	// 8240AB00: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8240AB04: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8240AB08: 910BFFFC  stw r8, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[8].u32 ) };
	// 8240AB0C: 409A000C  bne cr6, 0x8240ab18
	if !ctx.cr[6].eq {
	pc = 0x8240AB18; continue 'dispatch;
	}
	// 8240AB10: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240AB14: 48000008  b 0x8240ab1c
	pc = 0x8240AB1C; continue 'dispatch;
	// 8240AB18: 89070000  lbz r8, 0(r7)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240AB1C: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 8240AB20: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8240AB24: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 8240AB28: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 8240AB2C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8240AB30: 4082FFC0  bne 0x8240aaf0
	if !ctx.cr[0].eq {
	pc = 0x8240AAF0; continue 'dispatch;
	}
	// 8240AB34: 397CFFFF  addi r11, r28, -1
	ctx.r[11].s64 = ctx.r[28].s64 + -1;
	// 8240AB38: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 8240AB3C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8240AB40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8240AB44: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8240AB48: 99610088  stb r11, 0x88(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u8 ) };
	// 8240AB4C: 4817977D  bl 0x825842c8
	ctx.lr = 0x8240AB50;
	sub_825842C8(ctx, base);
	// 8240AB50: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240AB54: 40800014  bge 0x8240ab68
	if !ctx.cr[0].lt {
	pc = 0x8240AB68; continue 'dispatch;
	}
	// 8240AB58: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240AB5C: 386BD278  addi r3, r11, -0x2d88
	ctx.r[3].s64 = ctx.r[11].s64 + -11656;
	// 8240AB60: 4BEA8421  bl 0x822b2f80
	ctx.lr = 0x8240AB64;
	sub_822B2F80(ctx, base);
	// 8240AB64: 4BFFFF2C  b 0x8240aa90
	pc = 0x8240AA90; continue 'dispatch;
	// 8240AB68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240AB6C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 8240AB70: 4812A598  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AB78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240AB78 size=132
    let mut pc: u32 = 0x8240AB78;
    'dispatch: loop {
        match pc {
            0x8240AB78 => {
    //   block [0x8240AB78..0x8240ABFC)
	// 8240AB78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240AB7C: 4812A541  bl 0x825350bc
	ctx.lr = 0x8240AB80;
	sub_82535080(ctx, base);
	// 8240AB80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AB84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240AB88: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8240AB8C: 2B040FFF  cmplwi cr6, r4, 0xfff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 4095 as u32, &mut ctx.xer);
	// 8240AB90: 41990054  bgt cr6, 0x8240abe4
	if ctx.cr[6].gt {
	pc = 0x8240ABE4; continue 'dispatch;
	}
	// 8240AB94: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240AB98: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8240AB9C: 48004FF5  bl 0x8240fb90
	ctx.lr = 0x8240ABA0;
	sub_8240FB90(ctx, base);
	// 8240ABA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240ABA4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240ABA8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8240ABAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240ABB0: 83BF0014  lwz r29, 0x14(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240ABB4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240ABB8: 4BFFFE91  bl 0x8240aa48
	ctx.lr = 0x8240ABBC;
	sub_8240AA48(ctx, base);
	// 8240ABBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240ABC0: A09D0008  lhz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240ABC4: 4BFFF97D  bl 0x8240a540
	ctx.lr = 0x8240ABC8;
	sub_8240A540(ctx, base);
	// 8240ABC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240ABCC: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240ABD0: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8240ABD4: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240ABD8: 4BFFF7A1  bl 0x8240a378
	ctx.lr = 0x8240ABDC;
	sub_8240A378(ctx, base);
	// 8240ABDC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240ABE0: 48000014  b 0x8240abf4
	pc = 0x8240ABF4; continue 'dispatch;
	// 8240ABE4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240ABE8: 386BD318  addi r3, r11, -0x2ce8
	ctx.r[3].s64 = ctx.r[11].s64 + -11496;
	// 8240ABEC: 4BEA8395  bl 0x822b2f80
	ctx.lr = 0x8240ABF0;
	sub_822B2F80(ctx, base);
	// 8240ABF0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240ABF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240ABF8: 4812A514  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240AC00 size=132
    let mut pc: u32 = 0x8240AC00;
    'dispatch: loop {
        match pc {
            0x8240AC00 => {
    //   block [0x8240AC00..0x8240AC84)
	// 8240AC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240AC04: 4812A4B9  bl 0x825350bc
	ctx.lr = 0x8240AC08;
	sub_82535080(ctx, base);
	// 8240AC08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AC0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240AC10: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8240AC14: 2B040FFF  cmplwi cr6, r4, 0xfff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 4095 as u32, &mut ctx.xer);
	// 8240AC18: 41990054  bgt cr6, 0x8240ac6c
	if ctx.cr[6].gt {
	pc = 0x8240AC6C; continue 'dispatch;
	}
	// 8240AC1C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240AC20: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8240AC24: 48004F6D  bl 0x8240fb90
	ctx.lr = 0x8240AC28;
	sub_8240FB90(ctx, base);
	// 8240AC28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240AC2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240AC30: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8240AC34: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240AC38: 83BF0014  lwz r29, 0x14(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240AC3C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240AC40: 4BFFFE09  bl 0x8240aa48
	ctx.lr = 0x8240AC44;
	sub_8240AA48(ctx, base);
	// 8240AC44: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240AC48: 889D0001  lbz r4, 1(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(1 as u32) ) } as u64;
	// 8240AC4C: 4BFFF8F5  bl 0x8240a540
	ctx.lr = 0x8240AC50;
	sub_8240A540(ctx, base);
	// 8240AC50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240AC54: 80DF0014  lwz r6, 0x14(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240AC58: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8240AC5C: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8240AC60: 4BFFF819  bl 0x8240a478
	ctx.lr = 0x8240AC64;
	sub_8240A478(ctx, base);
	// 8240AC64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240AC68: 48000014  b 0x8240ac7c
	pc = 0x8240AC7C; continue 'dispatch;
	// 8240AC6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240AC70: 386BD318  addi r3, r11, -0x2ce8
	ctx.r[3].s64 = ctx.r[11].s64 + -11496;
	// 8240AC74: 4BEA830D  bl 0x822b2f80
	ctx.lr = 0x8240AC78;
	sub_822B2F80(ctx, base);
	// 8240AC78: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240AC7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240AC80: 4812A48C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AC88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240AC88 size=296
    let mut pc: u32 = 0x8240AC88;
    'dispatch: loop {
        match pc {
            0x8240AC88 => {
    //   block [0x8240AC88..0x8240ADB0)
	// 8240AC88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240AC8C: 4812A421  bl 0x825350ac
	ctx.lr = 0x8240AC90;
	sub_82535080(ctx, base);
	// 8240AC90: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AC94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240AC98: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8240AC9C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8240ACA0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8240ACA4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8240ACA8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240ACAC: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 8240ACB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240ACB4: 419A001C  beq cr6, 0x8240acd0
	if ctx.cr[6].eq {
	pc = 0x8240ACD0; continue 'dispatch;
	}
	// 8240ACB8: 4BFFF341  bl 0x82409ff8
	ctx.lr = 0x8240ACBC;
	sub_82409FF8(ctx, base);
	// 8240ACBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240ACC0: 4BFFF251  bl 0x82409f10
	ctx.lr = 0x8240ACC4;
	sub_82409F10(ctx, base);
	// 8240ACC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240ACC8: 809F0034  lwz r4, 0x34(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8240ACCC: 4BFFFB65  bl 0x8240a830
	ctx.lr = 0x8240ACD0;
	sub_8240A830(ctx, base);
	// 8240ACD0: 2B1E0FFF  cmplwi cr6, r30, 0xfff
	ctx.cr[6].compare_u32(ctx.r[30].u32, 4095 as u32, &mut ctx.xer);
	// 8240ACD4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240ACD8: 419900C0  bgt cr6, 0x8240ad98
	if ctx.cr[6].gt {
	pc = 0x8240AD98; continue 'dispatch;
	}
	// 8240ACDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240ACE0: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8240ACE4: 48004EAD  bl 0x8240fb90
	ctx.lr = 0x8240ACE8;
	sub_8240FB90(ctx, base);
	// 8240ACE8: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8240ACEC: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240ACF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240ACF4: 409A004C  bne cr6, 0x8240ad40
	if !ctx.cr[6].eq {
	pc = 0x8240AD40; continue 'dispatch;
	}
	// 8240ACF8: 2B190001  cmplwi cr6, r25, 1
	ctx.cr[6].compare_u32(ctx.r[25].u32, 1 as u32, &mut ctx.xer);
	// 8240ACFC: 41980088  blt cr6, 0x8240ad84
	if ctx.cr[6].lt {
	pc = 0x8240AD84; continue 'dispatch;
	}
	// 8240AD00: 419A0068  beq cr6, 0x8240ad68
	if ctx.cr[6].eq {
	pc = 0x8240AD68; continue 'dispatch;
	}
	// 8240AD04: 2B190004  cmplwi cr6, r25, 4
	ctx.cr[6].compare_u32(ctx.r[25].u32, 4 as u32, &mut ctx.xer);
	// 8240AD08: 419A0020  beq cr6, 0x8240ad28
	if ctx.cr[6].eq {
	pc = 0x8240AD28; continue 'dispatch;
	}
	// 8240AD0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240AD10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8240AD14: 386BD3A8  addi r3, r11, -0x2c58
	ctx.r[3].s64 = ctx.r[11].s64 + -11352;
	// 8240AD18: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8240AD1C: 4BEA8265  bl 0x822b2f80
	ctx.lr = 0x8240AD20;
	sub_822B2F80(ctx, base);
	// 8240AD20: 3860FFFD  li r3, -3
	ctx.r[3].s64 = -3;
	// 8240AD24: 48000084  b 0x8240ada8
	pc = 0x8240ADA8; continue 'dispatch;
	// 8240AD28: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240AD2C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240AD30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240AD34: 4BFFFECD  bl 0x8240ac00
	ctx.lr = 0x8240AD38;
	sub_8240AC00(ctx, base);
	// 8240AD38: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240AD3C: 41800068  blt 0x8240ada4
	if ctx.cr[0].lt {
	pc = 0x8240ADA4; continue 'dispatch;
	}
	// 8240AD40: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8240AD44: 933F0014  stw r25, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[25].u32 ) };
	// 8240AD48: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8240AD4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240AD50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240AD54: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8240AD58: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240AD5C: 915F0024  stw r10, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 8240AD60: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8240AD64: 48000044  b 0x8240ada8
	pc = 0x8240ADA8; continue 'dispatch;
	// 8240AD68: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8240AD6C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8240AD70: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240AD74: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240AD78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240AD7C: 4BFFFB6D  bl 0x8240a8e8
	ctx.lr = 0x8240AD80;
	sub_8240A8E8(ctx, base);
	// 8240AD80: 4BFFFFB8  b 0x8240ad38
	pc = 0x8240AD38; continue 'dispatch;
	// 8240AD84: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240AD88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240AD8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240AD90: 4BFFFDE9  bl 0x8240ab78
	ctx.lr = 0x8240AD94;
	sub_8240AB78(ctx, base);
	// 8240AD94: 4BFFFFA4  b 0x8240ad38
	pc = 0x8240AD38; continue 'dispatch;
	// 8240AD98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240AD9C: 386BD360  addi r3, r11, -0x2ca0
	ctx.r[3].s64 = ctx.r[11].s64 + -11424;
	// 8240ADA0: 4BEA81E1  bl 0x822b2f80
	ctx.lr = 0x8240ADA4;
	sub_822B2F80(ctx, base);
	// 8240ADA4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240ADA8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8240ADAC: 4812A350  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240ADB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240ADB0 size=72
    let mut pc: u32 = 0x8240ADB0;
    'dispatch: loop {
        match pc {
            0x8240ADB0 => {
    //   block [0x8240ADB0..0x8240ADF8)
	// 8240ADB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240ADB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240ADB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240ADBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240ADC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240ADC4: 3BE32A04  addi r31, r3, 0x2a04
	ctx.r[31].s64 = ctx.r[3].s64 + 10756;
	// 8240ADC8: 3BC000BF  li r30, 0xbf
	ctx.r[30].s64 = 191;
	// 8240ADCC: 3BFFFFC8  addi r31, r31, -0x38
	ctx.r[31].s64 = ctx.r[31].s64 + -56;
	// 8240ADD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240ADD4: 4BFFFAB5  bl 0x8240a888
	ctx.lr = 0x8240ADD8;
	sub_8240A888(ctx, base);
	// 8240ADD8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8240ADDC: 4080FFF0  bge 0x8240adcc
	if !ctx.cr[0].lt {
	pc = 0x8240ADCC; continue 'dispatch;
	}
	// 8240ADE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240ADE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240ADE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240ADEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240ADF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240ADF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240ADF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240ADF8 size=120
    let mut pc: u32 = 0x8240ADF8;
    'dispatch: loop {
        match pc {
            0x8240ADF8 => {
    //   block [0x8240ADF8..0x8240AE70)
	// 8240ADF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240ADFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240AE00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240AE04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240AE08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AE0C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240AE10: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240AE14: 3BC30004  addi r30, r3, 4
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	// 8240AE18: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240AE1C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240AE20: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8240AE24: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8240AE28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240AE2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240AE30: 4BFFF081  bl 0x82409eb0
	ctx.lr = 0x8240AE34;
	sub_82409EB0(ctx, base);
	// 8240AE34: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 8240AE38: 419A0030  beq cr6, 0x8240ae68
	if ctx.cr[6].eq {
	pc = 0x8240AE68; continue 'dispatch;
	}
	// 8240AE3C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240AE40: 3BDE0038  addi r30, r30, 0x38
	ctx.r[30].s64 = ctx.r[30].s64 + 56;
	// 8240AE44: 2F1F00C0  cmpwi cr6, r31, 0xc0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 192, &mut ctx.xer);
	// 8240AE48: 4198FFDC  blt cr6, 0x8240ae24
	if ctx.cr[6].lt {
	pc = 0x8240AE24; continue 'dispatch;
	}
	// 8240AE4C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240AE50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240AE54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240AE58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240AE5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240AE60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240AE64: 4E800020  blr
	return;
	// 8240AE68: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240AE6C: 4BFFFFE4  b 0x8240ae50
	pc = 0x8240AE50; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240AE70 size=60
    let mut pc: u32 = 0x8240AE70;
    'dispatch: loop {
        match pc {
            0x8240AE70 => {
    //   block [0x8240AE70..0x8240AEAC)
	// 8240AE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240AE74: 4812A249  bl 0x825350bc
	ctx.lr = 0x8240AE78;
	sub_82535080(ctx, base);
	// 8240AE78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AE7C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8240AE80: 3BC30004  addi r30, r3, 4
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	// 8240AE84: 3BE000C0  li r31, 0xc0
	ctx.r[31].s64 = 192;
	// 8240AE88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240AE8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240AE90: 4BFFFA01  bl 0x8240a890
	ctx.lr = 0x8240AE94;
	sub_8240A890(ctx, base);
	// 8240AE94: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240AE98: 3BDE0038  addi r30, r30, 0x38
	ctx.r[30].s64 = ctx.r[30].s64 + 56;
	// 8240AE9C: 4082FFEC  bne 0x8240ae88
	if !ctx.cr[0].eq {
	pc = 0x8240AE88; continue 'dispatch;
	}
	// 8240AEA0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8240AEA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240AEA8: 4812A264  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AEB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240AEB0 size=16
    let mut pc: u32 = 0x8240AEB0;
    'dispatch: loop {
        match pc {
            0x8240AEB0 => {
    //   block [0x8240AEB0..0x8240AEC0)
	// 8240AEB0: 2B0400C0  cmplwi cr6, r4, 0xc0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 192 as u32, &mut ctx.xer);
	// 8240AEB4: 4198000C  blt cr6, 0x8240aec0
	if ctx.cr[6].lt {
		sub_8240AEC0(ctx, base);
		return;
	}
	// 8240AEB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240AEBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240AEC0 size=16
    let mut pc: u32 = 0x8240AEC0;
    'dispatch: loop {
        match pc {
            0x8240AEC0 => {
    //   block [0x8240AEC0..0x8240AED0)
	// 8240AEC0: 1D640038  mulli r11, r4, 0x38
	ctx.r[11].s64 = ctx.r[4].s64 * 56;
	// 8240AEC4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240AEC8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8240AECC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240AED0 size=56
    let mut pc: u32 = 0x8240AED0;
    'dispatch: loop {
        match pc {
            0x8240AED0 => {
    //   block [0x8240AED0..0x8240AF08)
	// 8240AED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240AED4: 4812A1E9  bl 0x825350bc
	ctx.lr = 0x8240AED8;
	sub_82535080(ctx, base);
	// 8240AED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AEDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240AEE0: 3BE000BF  li r31, 0xbf
	ctx.r[31].s64 = 191;
	// 8240AEE4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 8240AEE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240AEEC: 4BFFEF7D  bl 0x82409e68
	ctx.lr = 0x8240AEF0;
	sub_82409E68(ctx, base);
	// 8240AEF0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240AEF4: 3BDE0038  addi r30, r30, 0x38
	ctx.r[30].s64 = ctx.r[30].s64 + 56;
	// 8240AEF8: 4080FFF0  bge 0x8240aee8
	if !ctx.cr[0].lt {
	pc = 0x8240AEE8; continue 'dispatch;
	}
	// 8240AEFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240AF00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240AF04: 4812A208  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240AF08 size=204
    let mut pc: u32 = 0x8240AF08;
    'dispatch: loop {
        match pc {
            0x8240AF08 => {
    //   block [0x8240AF08..0x8240AFD4)
	// 8240AF08: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8240AF0C: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240AF10: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240AF14: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8240AF18: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240AF1C: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 8240AF20: 3CE03F80  lis r7, 0x3f80
	ctx.r[7].s64 = 1065353216;
	// 8240AF24: 39200024  li r9, 0x24
	ctx.r[9].s64 = 36;
	// 8240AF28: 390B0050  addi r8, r11, 0x50
	ctx.r[8].s64 = ctx.r[11].s64 + 80;
	// 8240AF2C: C1AA1850  lfs f13, 0x1850(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(6224 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240AF30: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240AF34: D00B001C  stfs f0, 0x1c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8240AF38: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 8240AF3C: D00B0020  stfs f0, 0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8240AF40: D00B0024  stfs f0, 0x24(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8240AF44: D1AB0028  stfs f13, 0x28(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8240AF48: D00B002C  stfs f0, 0x2c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8240AF4C: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8240AF50: D00B003C  stfs f0, 0x3c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8240AF54: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8240AF58: D00B0040  stfs f0, 0x40(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 8240AF5C: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 8240AF60: D00B0044  stfs f0, 0x44(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 8240AF64: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8240AF68: 914B0014  stw r10, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 8240AF6C: 914B0018  stw r10, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 8240AF70: 914B0030  stw r10, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8240AF74: 914B0038  stw r10, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8240AF78: 914B0034  stw r10, 0x34(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 8240AF7C: 914B0048  stw r10, 0x48(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 8240AF80: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 8240AF84: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8240AF88: 90E80000  stw r7, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8240AF8C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8240AF90: 4200FFF8  bdnz 0x8240af88
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8240AF88; continue 'dispatch;
	}
	// 8240AF94: D00B00E0  stfs f0, 0xe0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), tmp.u32 ) };
	// 8240AF98: 90CB012C  stw r6, 0x12c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(300 as u32), ctx.r[6].u32 ) };
	// 8240AF9C: D1AB00E4  stfs f13, 0xe4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(228 as u32), tmp.u32 ) };
	// 8240AFA0: 914B0154  stw r10, 0x154(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(340 as u32), ctx.r[10].u32 ) };
	// 8240AFA4: D1AB00E8  stfs f13, 0xe8(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(232 as u32), tmp.u32 ) };
	// 8240AFA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240AFAC: D1AB00EC  stfs f13, 0xec(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 8240AFB0: D1AB00F0  stfs f13, 0xf0(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(240 as u32), tmp.u32 ) };
	// 8240AFB4: D1AB00F4  stfs f13, 0xf4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(244 as u32), tmp.u32 ) };
	// 8240AFB8: D00B0148  stfs f0, 0x148(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(328 as u32), tmp.u32 ) };
	// 8240AFBC: D1AB014C  stfs f13, 0x14c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(332 as u32), tmp.u32 ) };
	// 8240AFC0: D00B0150  stfs f0, 0x150(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(336 as u32), tmp.u32 ) };
	// 8240AFC4: D00B0158  stfs f0, 0x158(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(344 as u32), tmp.u32 ) };
	// 8240AFC8: D1AB015C  stfs f13, 0x15c(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(348 as u32), tmp.u32 ) };
	// 8240AFCC: D1AB0160  stfs f13, 0x160(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(352 as u32), tmp.u32 ) };
	// 8240AFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240AFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240AFD8 size=116
    let mut pc: u32 = 0x8240AFD8;
    'dispatch: loop {
        match pc {
            0x8240AFD8 => {
    //   block [0x8240AFD8..0x8240B04C)
	// 8240AFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240AFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240AFE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240AFE4: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240AFE8: 41990040  bgt cr6, 0x8240b028
	if ctx.cr[6].gt {
	pc = 0x8240B028; continue 'dispatch;
	}
	// 8240AFEC: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240AFF0: 7D4B1A14  add r10, r11, r3
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240AFF4: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240AFF8: 2C0B0000  cmpwi r11, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240AFFC: 41820020  beq 0x8240b01c
	if ctx.cr[0].eq {
	pc = 0x8240B01C; continue 'dispatch;
	}
	// 8240B000: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 8240B004: 419A0018  beq cr6, 0x8240b01c
	if ctx.cr[6].eq {
	pc = 0x8240B01C; continue 'dispatch;
	}
	// 8240B008: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 8240B00C: 419A0010  beq cr6, 0x8240b01c
	if ctx.cr[6].eq {
	pc = 0x8240B01C; continue 'dispatch;
	}
	// 8240B010: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 8240B014: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240B018: 48000008  b 0x8240b020
	pc = 0x8240B020; continue 'dispatch;
	// 8240B01C: 4BFFFEED  bl 0x8240af08
	ctx.lr = 0x8240B020;
	sub_8240AF08(ctx, base);
	// 8240B020: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B024: 48000018  b 0x8240b03c
	pc = 0x8240B03C; continue 'dispatch;
	// 8240B028: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B02C: 386BD3F4  addi r3, r11, -0x2c0c
	ctx.r[3].s64 = ctx.r[11].s64 + -11276;
	// 8240B030: 4BEA7F51  bl 0x822b2f80
	ctx.lr = 0x8240B034;
	sub_822B2F80(ctx, base);
	// 8240B034: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B038: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 8240B03C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B040: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B044: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B050 size=68
    let mut pc: u32 = 0x8240B050;
    'dispatch: loop {
        match pc {
            0x8240B050 => {
    //   block [0x8240B050..0x8240B094)
	// 8240B050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B054: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B058: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B05C: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B060: 41990010  bgt cr6, 0x8240b070
	if ctx.cr[6].gt {
	pc = 0x8240B070; continue 'dispatch;
	}
	// 8240B064: 4BFFFEA5  bl 0x8240af08
	ctx.lr = 0x8240B068;
	sub_8240AF08(ctx, base);
	// 8240B068: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B06C: 48000018  b 0x8240b084
	pc = 0x8240B084; continue 'dispatch;
	// 8240B070: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B074: 386BD438  addi r3, r11, -0x2bc8
	ctx.r[3].s64 = ctx.r[11].s64 + -11208;
	// 8240B078: 4BEA7F09  bl 0x822b2f80
	ctx.lr = 0x8240B07C;
	sub_822B2F80(ctx, base);
	// 8240B07C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B080: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 8240B084: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B08C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B098 size=84
    let mut pc: u32 = 0x8240B098;
    'dispatch: loop {
        match pc {
            0x8240B098 => {
    //   block [0x8240B098..0x8240B0EC)
	// 8240B098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B09C: 4812A021  bl 0x825350bc
	ctx.lr = 0x8240B0A0;
	sub_82535080(ctx, base);
	// 8240B0A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B0A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B0A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240B0AC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240B0B0: 3BCBD438  addi r30, r11, -0x2bc8
	ctx.r[30].s64 = ctx.r[11].s64 + -11208;
	// 8240B0B4: 2B1F00BF  cmplwi cr6, r31, 0xbf
	ctx.cr[6].compare_u32(ctx.r[31].u32, 191 as u32, &mut ctx.xer);
	// 8240B0B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240B0BC: 41990010  bgt cr6, 0x8240b0cc
	if ctx.cr[6].gt {
	pc = 0x8240B0CC; continue 'dispatch;
	}
	// 8240B0C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240B0C4: 4BFFFE45  bl 0x8240af08
	ctx.lr = 0x8240B0C8;
	sub_8240AF08(ctx, base);
	// 8240B0C8: 4800000C  b 0x8240b0d4
	pc = 0x8240B0D4; continue 'dispatch;
	// 8240B0CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240B0D0: 4BEA7EB1  bl 0x822b2f80
	ctx.lr = 0x8240B0D4;
	sub_822B2F80(ctx, base);
	// 8240B0D4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240B0D8: 2F1F00C0  cmpwi cr6, r31, 0xc0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 192, &mut ctx.xer);
	// 8240B0DC: 4198FFD8  blt cr6, 0x8240b0b4
	if ctx.cr[6].lt {
	pc = 0x8240B0B4; continue 'dispatch;
	}
	// 8240B0E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B0E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240B0E8: 4812A024  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B0F0 size=156
    let mut pc: u32 = 0x8240B0F0;
    'dispatch: loop {
        match pc {
            0x8240B0F0 => {
    //   block [0x8240B0F0..0x8240B18C)
	// 8240B0F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B0F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B0F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B0FC: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B100: 41990068  bgt cr6, 0x8240b168
	if ctx.cr[6].gt {
	pc = 0x8240B168; continue 'dispatch;
	}
	// 8240B104: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B108: 7D4B1A14  add r10, r11, r3
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B10C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240B110: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 8240B114: 419A004C  beq cr6, 0x8240b160
	if ctx.cr[6].eq {
	pc = 0x8240B160; continue 'dispatch;
	}
	// 8240B118: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8240B11C: 419A0030  beq cr6, 0x8240b14c
	if ctx.cr[6].eq {
	pc = 0x8240B14C; continue 'dispatch;
	}
	// 8240B120: 2B050001  cmplwi cr6, r5, 1
	ctx.cr[6].compare_u32(ctx.r[5].u32, 1 as u32, &mut ctx.xer);
	// 8240B124: 409A0018  bne cr6, 0x8240b13c
	if !ctx.cr[6].eq {
	pc = 0x8240B13C; continue 'dispatch;
	}
	// 8240B128: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8240B12C: 409A0034  bne cr6, 0x8240b160
	if !ctx.cr[6].eq {
	pc = 0x8240B160; continue 'dispatch;
	}
	// 8240B130: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8240B134: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240B138: 48000028  b 0x8240b160
	pc = 0x8240B160; continue 'dispatch;
	// 8240B13C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8240B140: 419A0020  beq cr6, 0x8240b160
	if ctx.cr[6].eq {
	pc = 0x8240B160; continue 'dispatch;
	}
	// 8240B144: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 8240B148: 419A0018  beq cr6, 0x8240b160
	if ctx.cr[6].eq {
	pc = 0x8240B160; continue 'dispatch;
	}
	// 8240B14C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240B150: 419A0010  beq cr6, 0x8240b160
	if ctx.cr[6].eq {
	pc = 0x8240B160; continue 'dispatch;
	}
	// 8240B154: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240B158: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8240B15C: 4BFFFDAD  bl 0x8240af08
	ctx.lr = 0x8240B160;
	sub_8240AF08(ctx, base);
	// 8240B160: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B164: 48000018  b 0x8240b17c
	pc = 0x8240B17C; continue 'dispatch;
	// 8240B168: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B16C: 386BD480  addi r3, r11, -0x2b80
	ctx.r[3].s64 = ctx.r[11].s64 + -11136;
	// 8240B170: 4BEA7E11  bl 0x822b2f80
	ctx.lr = 0x8240B174;
	sub_822B2F80(ctx, base);
	// 8240B174: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B178: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 8240B17C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240B190 size=72
    let mut pc: u32 = 0x8240B190;
    'dispatch: loop {
        match pc {
            0x8240B190 => {
    //   block [0x8240B190..0x8240B1D8)
	// 8240B190: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 8240B194: 390000C0  li r8, 0xc0
	ctx.r[8].s64 = 192;
	// 8240B198: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240B19C: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 8240B1A0: 409A0010  bne cr6, 0x8240b1b0
	if !ctx.cr[6].eq {
	pc = 0x8240B1B0; continue 'dispatch;
	}
	// 8240B1A4: 814B0040  lwz r10, 0x40(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 8240B1A8: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 8240B1AC: 914B0040  stw r10, 0x40(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 8240B1B0: 2F090004  cmpwi cr6, r9, 4
	ctx.cr[6].compare_i32(ctx.r[9].s32, 4, &mut ctx.xer);
	// 8240B1B4: 409A0010  bne cr6, 0x8240b1c4
	if !ctx.cr[6].eq {
	pc = 0x8240B1C4; continue 'dispatch;
	}
	// 8240B1B8: 814B0044  lwz r10, 0x44(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(68 as u32) ) } as u64;
	// 8240B1BC: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 8240B1C0: 914B0044  stw r10, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 8240B1C4: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8240B1C8: 396B0164  addi r11, r11, 0x164
	ctx.r[11].s64 = ctx.r[11].s64 + 356;
	// 8240B1CC: 4082FFCC  bne 0x8240b198
	if !ctx.cr[0].eq {
	pc = 0x8240B198; continue 'dispatch;
	}
	// 8240B1D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B1D8 size=68
    let mut pc: u32 = 0x8240B1D8;
    'dispatch: loop {
        match pc {
            0x8240B1D8 => {
    //   block [0x8240B1D8..0x8240B21C)
	// 8240B1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B1E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B1E4: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B1E8: 41990014  bgt cr6, 0x8240b1fc
	if ctx.cr[6].gt {
	pc = 0x8240B1FC; continue 'dispatch;
	}
	// 8240B1EC: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B1F0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B1F4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240B1F8: 48000014  b 0x8240b20c
	pc = 0x8240B20C; continue 'dispatch;
	// 8240B1FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B200: 386BD4C4  addi r3, r11, -0x2b3c
	ctx.r[3].s64 = ctx.r[11].s64 + -11068;
	// 8240B204: 4BEA7D7D  bl 0x822b2f80
	ctx.lr = 0x8240B208;
	sub_822B2F80(ctx, base);
	// 8240B208: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8240B20C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B220 size=80
    let mut pc: u32 = 0x8240B220;
    'dispatch: loop {
        match pc {
            0x8240B220 => {
    //   block [0x8240B220..0x8240B270)
	// 8240B220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B228: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B22C: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B230: 4199001C  bgt cr6, 0x8240b24c
	if ctx.cr[6].gt {
	pc = 0x8240B24C; continue 'dispatch;
	}
	// 8240B234: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B238: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B23C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B240: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240B244: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240B248: 48000018  b 0x8240b260
	pc = 0x8240B260; continue 'dispatch;
	// 8240B24C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B250: 386BD500  addi r3, r11, -0x2b00
	ctx.r[3].s64 = ctx.r[11].s64 + -11008;
	// 8240B254: 4BEA7D2D  bl 0x822b2f80
	ctx.lr = 0x8240B258;
	sub_822B2F80(ctx, base);
	// 8240B258: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B25C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 8240B260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B26C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B270 size=80
    let mut pc: u32 = 0x8240B270;
    'dispatch: loop {
        match pc {
            0x8240B270 => {
    //   block [0x8240B270..0x8240B2C0)
	// 8240B270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B278: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B27C: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B280: 4199001C  bgt cr6, 0x8240b29c
	if ctx.cr[6].gt {
	pc = 0x8240B29C; continue 'dispatch;
	}
	// 8240B284: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B288: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B28C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B290: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8240B294: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240B298: 48000018  b 0x8240b2b0
	pc = 0x8240B2B0; continue 'dispatch;
	// 8240B29C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B2A0: 386BD540  addi r3, r11, -0x2ac0
	ctx.r[3].s64 = ctx.r[11].s64 + -10944;
	// 8240B2A4: 4BEA7CDD  bl 0x822b2f80
	ctx.lr = 0x8240B2A8;
	sub_822B2F80(ctx, base);
	// 8240B2A8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B2AC: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 8240B2B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B2B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B2B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B2BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B2C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B2C0 size=68
    let mut pc: u32 = 0x8240B2C0;
    'dispatch: loop {
        match pc {
            0x8240B2C0 => {
    //   block [0x8240B2C0..0x8240B304)
	// 8240B2C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B2C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B2C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B2CC: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B2D0: 41990014  bgt cr6, 0x8240b2e4
	if ctx.cr[6].gt {
	pc = 0x8240B2E4; continue 'dispatch;
	}
	// 8240B2D4: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B2D8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B2DC: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240B2E0: 48000014  b 0x8240b2f4
	pc = 0x8240B2F4; continue 'dispatch;
	// 8240B2E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B2E8: 386BD580  addi r3, r11, -0x2a80
	ctx.r[3].s64 = ctx.r[11].s64 + -10880;
	// 8240B2EC: 4BEA7C95  bl 0x822b2f80
	ctx.lr = 0x8240B2F0;
	sub_822B2F80(ctx, base);
	// 8240B2F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B2F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B2F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B2FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B308 size=88
    let mut pc: u32 = 0x8240B308;
    'dispatch: loop {
        match pc {
            0x8240B308 => {
    //   block [0x8240B308..0x8240B360)
	// 8240B308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B310: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B314: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 8240B318: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B31C: 41990020  bgt cr6, 0x8240b33c
	if ctx.cr[6].gt {
	pc = 0x8240B33C; continue 'dispatch;
	}
	// 8240B320: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B324: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B328: 38A00164  li r5, 0x164
	ctx.r[5].s64 = 356;
	// 8240B32C: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 8240B330: 4BFB58D1  bl 0x823c0c00
	ctx.lr = 0x8240B334;
	sub_823C0C00(ctx, base);
	// 8240B334: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B338: 48000018  b 0x8240b350
	pc = 0x8240B350; continue 'dispatch;
	// 8240B33C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B340: 386BD5C8  addi r3, r11, -0x2a38
	ctx.r[3].s64 = ctx.r[11].s64 + -10808;
	// 8240B344: 4BEA7C3D  bl 0x822b2f80
	ctx.lr = 0x8240B348;
	sub_822B2F80(ctx, base);
	// 8240B348: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B34C: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 8240B350: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B354: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B358: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B35C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B360 size=88
    let mut pc: u32 = 0x8240B360;
    'dispatch: loop {
        match pc {
            0x8240B360 => {
    //   block [0x8240B360..0x8240B3B8)
	// 8240B360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B368: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B36C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8240B370: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8240B374: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B378: 4199001C  bgt cr6, 0x8240b394
	if ctx.cr[6].gt {
	pc = 0x8240B394; continue 'dispatch;
	}
	// 8240B37C: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B380: 7C8B5214  add r4, r11, r10
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8240B384: 38A00164  li r5, 0x164
	ctx.r[5].s64 = 356;
	// 8240B388: 4BFB5879  bl 0x823c0c00
	ctx.lr = 0x8240B38C;
	sub_823C0C00(ctx, base);
	// 8240B38C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B390: 48000018  b 0x8240b3a8
	pc = 0x8240B3A8; continue 'dispatch;
	// 8240B394: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B398: 386BD608  addi r3, r11, -0x29f8
	ctx.r[3].s64 = ctx.r[11].s64 + -10744;
	// 8240B39C: 4BEA7BE5  bl 0x822b2f80
	ctx.lr = 0x8240B3A0;
	sub_822B2F80(ctx, base);
	// 8240B3A0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B3A4: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 8240B3A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B3AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B3B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B3B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B3B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240B3B8 size=80
    let mut pc: u32 = 0x8240B3B8;
    'dispatch: loop {
        match pc {
            0x8240B3B8 => {
    //   block [0x8240B3B8..0x8240B408)
	// 8240B3B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B3BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B3C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B3C4: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B3C8: 4199001C  bgt cr6, 0x8240b3e4
	if ctx.cr[6].gt {
	pc = 0x8240B3E4; continue 'dispatch;
	}
	// 8240B3CC: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B3D0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B3D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B3D8: C00B014C  lfs f0, 0x14c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(332 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240B3DC: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240B3E0: 48000018  b 0x8240b3f8
	pc = 0x8240B3F8; continue 'dispatch;
	// 8240B3E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B3E8: 386BD648  addi r3, r11, -0x29b8
	ctx.r[3].s64 = ctx.r[11].s64 + -10680;
	// 8240B3EC: 4BEA7B95  bl 0x822b2f80
	ctx.lr = 0x8240B3F0;
	sub_822B2F80(ctx, base);
	// 8240B3F0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B3F4: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 8240B3F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B408 size=80
    let mut pc: u32 = 0x8240B408;
    'dispatch: loop {
        match pc {
            0x8240B408 => {
    //   block [0x8240B408..0x8240B458)
	// 8240B408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B410: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B414: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B418: 4199001C  bgt cr6, 0x8240b434
	if ctx.cr[6].gt {
	pc = 0x8240B434; continue 'dispatch;
	}
	// 8240B41C: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B420: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B424: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B428: 816B0154  lwz r11, 0x154(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(340 as u32) ) } as u64;
	// 8240B42C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240B430: 48000018  b 0x8240b448
	pc = 0x8240B448; continue 'dispatch;
	// 8240B434: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B438: 386BD688  addi r3, r11, -0x2978
	ctx.r[3].s64 = ctx.r[11].s64 + -10616;
	// 8240B43C: 4BEA7B45  bl 0x822b2f80
	ctx.lr = 0x8240B440;
	sub_822B2F80(ctx, base);
	// 8240B440: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B444: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 8240B448: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B44C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B450: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240B458 size=80
    let mut pc: u32 = 0x8240B458;
    'dispatch: loop {
        match pc {
            0x8240B458 => {
    //   block [0x8240B458..0x8240B4A8)
	// 8240B458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B45C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B460: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B464: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B468: 4199001C  bgt cr6, 0x8240b484
	if ctx.cr[6].gt {
	pc = 0x8240B484; continue 'dispatch;
	}
	// 8240B46C: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B470: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B474: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B478: C00B0160  lfs f0, 0x160(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240B47C: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240B480: 48000018  b 0x8240b498
	pc = 0x8240B498; continue 'dispatch;
	// 8240B484: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B488: 386BD6D0  addi r3, r11, -0x2930
	ctx.r[3].s64 = ctx.r[11].s64 + -10544;
	// 8240B48C: 4BEA7AF5  bl 0x822b2f80
	ctx.lr = 0x8240B490;
	sub_822B2F80(ctx, base);
	// 8240B490: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B494: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 8240B498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B49C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B4A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B4A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240B4A8 size=80
    let mut pc: u32 = 0x8240B4A8;
    'dispatch: loop {
        match pc {
            0x8240B4A8 => {
    //   block [0x8240B4A8..0x8240B4F8)
	// 8240B4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B4B0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B4B4: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B4B8: 4199001C  bgt cr6, 0x8240b4d4
	if ctx.cr[6].gt {
	pc = 0x8240B4D4; continue 'dispatch;
	}
	// 8240B4BC: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B4C0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B4C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B4C8: C00B015C  lfs f0, 0x15c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(348 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240B4CC: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240B4D0: 48000018  b 0x8240b4e8
	pc = 0x8240B4E8; continue 'dispatch;
	// 8240B4D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B4D8: 386BD720  addi r3, r11, -0x28e0
	ctx.r[3].s64 = ctx.r[11].s64 + -10464;
	// 8240B4DC: 4BEA7AA5  bl 0x822b2f80
	ctx.lr = 0x8240B4E0;
	sub_822B2F80(ctx, base);
	// 8240B4E0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B4E4: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 8240B4E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B4EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B4F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B4F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B4F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240B4F8 size=124
    let mut pc: u32 = 0x8240B4F8;
    'dispatch: loop {
        match pc {
            0x8240B4F8 => {
    //   block [0x8240B4F8..0x8240B574)
	// 8240B4F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B4FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B500: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B504: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B508: 41990048  bgt cr6, 0x8240b550
	if ctx.cr[6].gt {
	pc = 0x8240B550; continue 'dispatch;
	}
	// 8240B50C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240B510: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240B514: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240B518: 40980024  bge cr6, 0x8240b53c
	if !ctx.cr[6].lt {
	pc = 0x8240B53C; continue 'dispatch;
	}
	// 8240B51C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B520: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8240B524: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 8240B528: 386BD7B0  addi r3, r11, -0x2850
	ctx.r[3].s64 = ctx.r[11].s64 + -10320;
	// 8240B52C: 4BEA7A55  bl 0x822b2f80
	ctx.lr = 0x8240B530;
	sub_822B2F80(ctx, base);
	// 8240B530: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B534: 60630018  ori r3, r3, 0x18
	ctx.r[3].u64 = ctx.r[3].u64 | 24;
	// 8240B538: 4800002C  b 0x8240b564
	pc = 0x8240B564; continue 'dispatch;
	// 8240B53C: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B540: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B544: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B548: D02B014C  stfs f1, 0x14c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(332 as u32), tmp.u32 ) };
	// 8240B54C: 48000018  b 0x8240b564
	pc = 0x8240B564; continue 'dispatch;
	// 8240B550: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B554: 386BD76C  addi r3, r11, -0x2894
	ctx.r[3].s64 = ctx.r[11].s64 + -10388;
	// 8240B558: 4BEA7A29  bl 0x822b2f80
	ctx.lr = 0x8240B55C;
	sub_822B2F80(ctx, base);
	// 8240B55C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B560: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 8240B564: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B568: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B56C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B570: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B578 size=76
    let mut pc: u32 = 0x8240B578;
    'dispatch: loop {
        match pc {
            0x8240B578 => {
    //   block [0x8240B578..0x8240B5C4)
	// 8240B578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B580: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B584: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B588: 41990018  bgt cr6, 0x8240b5a0
	if ctx.cr[6].gt {
	pc = 0x8240B5A0; continue 'dispatch;
	}
	// 8240B58C: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B590: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B594: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B598: 90AB0154  stw r5, 0x154(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(340 as u32), ctx.r[5].u32 ) };
	// 8240B59C: 48000018  b 0x8240b5b4
	pc = 0x8240B5B4; continue 'dispatch;
	// 8240B5A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B5A4: 386BD7F0  addi r3, r11, -0x2810
	ctx.r[3].s64 = ctx.r[11].s64 + -10256;
	// 8240B5A8: 4BEA79D9  bl 0x822b2f80
	ctx.lr = 0x8240B5AC;
	sub_822B2F80(ctx, base);
	// 8240B5AC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B5B0: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 8240B5B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B5B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B5BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B5C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240B5C8 size=76
    let mut pc: u32 = 0x8240B5C8;
    'dispatch: loop {
        match pc {
            0x8240B5C8 => {
    //   block [0x8240B5C8..0x8240B614)
	// 8240B5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B5D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B5D4: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B5D8: 41990018  bgt cr6, 0x8240b5f0
	if ctx.cr[6].gt {
	pc = 0x8240B5F0; continue 'dispatch;
	}
	// 8240B5DC: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B5E0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B5E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B5E8: D02B0150  stfs f1, 0x150(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(336 as u32), tmp.u32 ) };
	// 8240B5EC: 48000018  b 0x8240b604
	pc = 0x8240B604; continue 'dispatch;
	// 8240B5F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B5F4: 386BD834  addi r3, r11, -0x27cc
	ctx.r[3].s64 = ctx.r[11].s64 + -10188;
	// 8240B5F8: 4BEA7989  bl 0x822b2f80
	ctx.lr = 0x8240B5FC;
	sub_822B2F80(ctx, base);
	// 8240B5FC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B600: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 8240B604: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B60C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B618 size=80
    let mut pc: u32 = 0x8240B618;
    'dispatch: loop {
        match pc {
            0x8240B618 => {
    //   block [0x8240B618..0x8240B668)
	// 8240B618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B61C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B620: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B624: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B628: 41990020  bgt cr6, 0x8240b648
	if ctx.cr[6].gt {
	pc = 0x8240B648; continue 'dispatch;
	}
	// 8240B62C: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B630: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B634: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240B638: 396BFFFB  addi r11, r11, -5
	ctx.r[11].s64 = ctx.r[11].s64 + -5;
	// 8240B63C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8240B640: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8240B644: 48000014  b 0x8240b658
	pc = 0x8240B658; continue 'dispatch;
	// 8240B648: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B64C: 386BD874  addi r3, r11, -0x278c
	ctx.r[3].s64 = ctx.r[11].s64 + -10124;
	// 8240B650: 4BEA7931  bl 0x822b2f80
	ctx.lr = 0x8240B654;
	sub_822B2F80(ctx, base);
	// 8240B654: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B658: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B65C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B660: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B668 size=168
    let mut pc: u32 = 0x8240B668;
    'dispatch: loop {
        match pc {
            0x8240B668 => {
    //   block [0x8240B668..0x8240B710)
	// 8240B668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B670: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B674: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 8240B678: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8240B67C: 2B060040  cmplwi cr6, r6, 0x40
	ctx.cr[6].compare_u32(ctx.r[6].u32, 64 as u32, &mut ctx.xer);
	// 8240B680: 41990068  bgt cr6, 0x8240b6e8
	if ctx.cr[6].gt {
	pc = 0x8240B6E8; continue 'dispatch;
	}
	// 8240B684: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8240B688: 419A0058  beq cr6, 0x8240b6e0
	if ctx.cr[6].eq {
	pc = 0x8240B6E0; continue 'dispatch;
	}
	// 8240B68C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240B690: 419A0050  beq cr6, 0x8240b6e0
	if ctx.cr[6].eq {
	pc = 0x8240B6E0; continue 'dispatch;
	}
	// 8240B694: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8240B698: 40990034  ble cr6, 0x8240b6cc
	if !ctx.cr[6].gt {
	pc = 0x8240B6CC; continue 'dispatch;
	}
	// 8240B69C: 3D470001  addis r10, r7, 1
	ctx.r[10].s64 = ctx.r[7].s64 + 65536;
	// 8240B6A0: 7D0B2050  subf r8, r11, r4
	ctx.r[8].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 8240B6A4: 394A0C04  addi r10, r10, 0xc04
	ctx.r[10].s64 = ctx.r[10].s64 + 3076;
	// 8240B6A8: 7CC93378  mr r9, r6
	ctx.r[9].u64 = ctx.r[6].u64;
	// 8240B6AC: 7CA8582E  lwzx r5, r8, r11
	ctx.r[5].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8240B6B0: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8240B6B4: 90AAFF00  stw r5, -0x100(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-256 as u32), ctx.r[5].u32 ) };
	// 8240B6B8: 80AB0000  lwz r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240B6BC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240B6C0: 90AA0000  stw r5, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8240B6C4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8240B6C8: 4082FFE4  bne 0x8240b6ac
	if !ctx.cr[0].eq {
	pc = 0x8240B6AC; continue 'dispatch;
	}
	// 8240B6CC: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 8240B6D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B6D4: 616B0D04  ori r11, r11, 0xd04
	ctx.r[11].u64 = ctx.r[11].u64 | 3332;
	// 8240B6D8: 7CC7592E  stwx r6, r7, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[11].u32), ctx.r[6].u32) };
	// 8240B6DC: 48000024  b 0x8240b700
	pc = 0x8240B700; continue 'dispatch;
	// 8240B6E0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B6E4: 4800001C  b 0x8240b700
	pc = 0x8240B700; continue 'dispatch;
	// 8240B6E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B6EC: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8240B6F0: 386BD8B8  addi r3, r11, -0x2748
	ctx.r[3].s64 = ctx.r[11].s64 + -10056;
	// 8240B6F4: 4BEA788D  bl 0x822b2f80
	ctx.lr = 0x8240B6F8;
	sub_822B2F80(ctx, base);
	// 8240B6F8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B6FC: 60630014  ori r3, r3, 0x14
	ctx.r[3].u64 = ctx.r[3].u64 | 20;
	// 8240B700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B70C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B710 size=120
    let mut pc: u32 = 0x8240B710;
    'dispatch: loop {
        match pc {
            0x8240B710 => {
    //   block [0x8240B710..0x8240B788)
	// 8240B710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B71C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240B720: 41980044  blt cr6, 0x8240b764
	if ctx.cr[6].lt {
	pc = 0x8240B764; continue 'dispatch;
	}
	// 8240B724: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 8240B728: 616B0D04  ori r11, r11, 0xd04
	ctx.r[11].u64 = ctx.r[11].u64 | 3332;
	// 8240B72C: 7D63582E  lwzx r11, r3, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8240B730: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240B734: 40980030  bge cr6, 0x8240b764
	if !ctx.cr[6].lt {
	pc = 0x8240B764; continue 'dispatch;
	}
	// 8240B738: 396442C1  addi r11, r4, 0x42c1
	ctx.r[11].s64 = ctx.r[4].s64 + 17089;
	// 8240B73C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240B740: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8240B744: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240B748: 41810030  bgt 0x8240b778
	if ctx.cr[0].gt {
	pc = 0x8240B778; continue 'dispatch;
	}
	// 8240B74C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B750: 386BD948  addi r3, r11, -0x26b8
	ctx.r[3].s64 = ctx.r[11].s64 + -9912;
	// 8240B754: 4BEA782D  bl 0x822b2f80
	ctx.lr = 0x8240B758;
	sub_822B2F80(ctx, base);
	// 8240B758: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B75C: 60630011  ori r3, r3, 0x11
	ctx.r[3].u64 = ctx.r[3].u64 | 17;
	// 8240B760: 48000018  b 0x8240b778
	pc = 0x8240B778; continue 'dispatch;
	// 8240B764: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B768: 386BD900  addi r3, r11, -0x2700
	ctx.r[3].s64 = ctx.r[11].s64 + -9984;
	// 8240B76C: 4BEA7815  bl 0x822b2f80
	ctx.lr = 0x8240B770;
	sub_822B2F80(ctx, base);
	// 8240B770: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B774: 60630014  ori r3, r3, 0x14
	ctx.r[3].u64 = ctx.r[3].u64 | 20;
	// 8240B778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B788 size=92
    let mut pc: u32 = 0x8240B788;
    'dispatch: loop {
        match pc {
            0x8240B788 => {
    //   block [0x8240B788..0x8240B7E4)
	// 8240B788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B78C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B790: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B794: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240B798: 41980028  blt cr6, 0x8240b7c0
	if ctx.cr[6].lt {
	pc = 0x8240B7C0; continue 'dispatch;
	}
	// 8240B79C: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 8240B7A0: 616B0D04  ori r11, r11, 0xd04
	ctx.r[11].u64 = ctx.r[11].u64 | 3332;
	// 8240B7A4: 7D63582E  lwzx r11, r3, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8240B7A8: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240B7AC: 40980014  bge cr6, 0x8240b7c0
	if !ctx.cr[6].lt {
	pc = 0x8240B7C0; continue 'dispatch;
	}
	// 8240B7B0: 39644301  addi r11, r4, 0x4301
	ctx.r[11].s64 = ctx.r[4].s64 + 17153;
	// 8240B7B4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240B7B8: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8240B7BC: 48000018  b 0x8240b7d4
	pc = 0x8240B7D4; continue 'dispatch;
	// 8240B7C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B7C4: 386BD9B0  addi r3, r11, -0x2650
	ctx.r[3].s64 = ctx.r[11].s64 + -9808;
	// 8240B7C8: 4BEA77B9  bl 0x822b2f80
	ctx.lr = 0x8240B7CC;
	sub_822B2F80(ctx, base);
	// 8240B7CC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B7D0: 60630014  ori r3, r3, 0x14
	ctx.r[3].u64 = ctx.r[3].u64 | 20;
	// 8240B7D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B7D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B7DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B7E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B7E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240B7E8 size=128
    let mut pc: u32 = 0x8240B7E8;
    'dispatch: loop {
        match pc {
            0x8240B7E8 => {
    //   block [0x8240B7E8..0x8240B868)
	// 8240B7E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B7EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240B7F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B7F4: 2B0400BF  cmplwi cr6, r4, 0xbf
	ctx.cr[6].compare_u32(ctx.r[4].u32, 191 as u32, &mut ctx.xer);
	// 8240B7F8: 4199004C  bgt cr6, 0x8240b844
	if ctx.cr[6].gt {
	pc = 0x8240B844; continue 'dispatch;
	}
	// 8240B7FC: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240B800: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240B804: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8240B808: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240B80C: 41820030  beq 0x8240b83c
	if ctx.cr[0].eq {
	pc = 0x8240B83C; continue 'dispatch;
	}
	// 8240B810: 812B0008  lwz r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8240B814: 2F090006  cmpwi cr6, r9, 6
	ctx.cr[6].compare_i32(ctx.r[9].s32, 6, &mut ctx.xer);
	// 8240B818: 419A0024  beq cr6, 0x8240b83c
	if ctx.cr[6].eq {
	pc = 0x8240B83C; continue 'dispatch;
	}
	// 8240B81C: 814A004C  lwz r10, 0x4c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(76 as u32) ) } as u64;
	// 8240B820: 554A07FF  clrlwi. r10, r10, 0x1f
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240B824: 41820018  beq 0x8240b83c
	if ctx.cr[0].eq {
	pc = 0x8240B83C; continue 'dispatch;
	}
	// 8240B828: 39400005  li r10, 5
	ctx.r[10].s64 = 5;
	// 8240B82C: 912B000C  stw r9, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 8240B830: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B834: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8240B838: 48000020  b 0x8240b858
	pc = 0x8240B858; continue 'dispatch;
	// 8240B83C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B840: 48000018  b 0x8240b858
	pc = 0x8240B858; continue 'dispatch;
	// 8240B844: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240B848: 386BD9FC  addi r3, r11, -0x2604
	ctx.r[3].s64 = ctx.r[11].s64 + -9732;
	// 8240B84C: 4BEA7735  bl 0x822b2f80
	ctx.lr = 0x8240B850;
	sub_822B2F80(ctx, base);
	// 8240B850: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B854: 60630016  ori r3, r3, 0x16
	ctx.r[3].u64 = ctx.r[3].u64 | 22;
	// 8240B858: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240B85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240B860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240B864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240B868 size=60
    let mut pc: u32 = 0x8240B868;
    'dispatch: loop {
        match pc {
            0x8240B868 => {
    //   block [0x8240B868..0x8240B8A4)
	// 8240B868: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8240B86C: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 8240B870: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240B874: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240B878: 41820014  beq 0x8240b88c
	if ctx.cr[0].eq {
	pc = 0x8240B88C; continue 'dispatch;
	}
	// 8240B87C: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 8240B880: 419A000C  beq cr6, 0x8240b88c
	if ctx.cr[6].eq {
	pc = 0x8240B88C; continue 'dispatch;
	}
	// 8240B884: 2F0A0006  cmpwi cr6, r10, 6
	ctx.cr[6].compare_i32(ctx.r[10].s32, 6, &mut ctx.xer);
	// 8240B888: 409A001C  bne cr6, 0x8240b8a4
	if !ctx.cr[6].eq {
		sub_8240B8A4(ctx, base);
		return;
	}
	// 8240B88C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 8240B890: 396B0164  addi r11, r11, 0x164
	ctx.r[11].s64 = ctx.r[11].s64 + 356;
	// 8240B894: 2F0900C0  cmpwi cr6, r9, 0xc0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 192, &mut ctx.xer);
	// 8240B898: 4198FFD8  blt cr6, 0x8240b870
	if ctx.cr[6].lt {
	pc = 0x8240B870; continue 'dispatch;
	}
	// 8240B89C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8240B8A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B8A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240B8A4 size=8
    let mut pc: u32 = 0x8240B8A4;
    'dispatch: loop {
        match pc {
            0x8240B8A4 => {
    //   block [0x8240B8A4..0x8240B8AC)
	// 8240B8A4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240B8A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240B8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240B8B0 size=704
    let mut pc: u32 = 0x8240B8B0;
    'dispatch: loop {
        match pc {
            0x8240B8B0 => {
    //   block [0x8240B8B0..0x8240BB70)
	// 8240B8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240B8B4: 481297F1  bl 0x825350a4
	ctx.lr = 0x8240B8B8;
	sub_82535080(ctx, base);
	// 8240B8B8: 9421F350  stwu r1, -0xcb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-3248 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240B8BC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8240B8C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240B8C4: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 8240B8C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240B8CC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8240B8D0: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8240B8D4: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 8240B8D8: 7F1DC378  mr r29, r24
	ctx.r[29].u64 = ctx.r[24].u64;
	// 8240B8DC: 4BFFFE35  bl 0x8240b710
	ctx.lr = 0x8240B8E0;
	sub_8240B710(ctx, base);
	// 8240B8E0: 7C7C1B79  or. r28, r3, r3
	ctx.r[28].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8240B8E4: 4080000C  bge 0x8240b8f0
	if !ctx.cr[0].lt {
	pc = 0x8240B8F0; continue 'dispatch;
	}
	// 8240B8E8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8240B8EC: 4800027C  b 0x8240bb68
	pc = 0x8240BB68; continue 'dispatch;
	// 8240B8F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240B8F4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8240B8F8: 4BFFFE91  bl 0x8240b788
	ctx.lr = 0x8240B8FC;
	sub_8240B788(ctx, base);
	// 8240B8FC: 393B0030  addi r9, r27, 0x30
	ctx.r[9].s64 = ctx.r[27].s64 + 48;
	// 8240B900: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 8240B904: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8240B908: 39010660  addi r8, r1, 0x660
	ctx.r[8].s64 = ctx.r[1].s64 + 1632;
	// 8240B90C: 8149FFD8  lwz r10, -0x28(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-40 as u32) ) } as u64;
	// 8240B910: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240B914: 7F0B412E  stwx r24, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[24].u32) };
	// 8240B918: 40810038  ble 0x8240b950
	if !ctx.cr[0].gt {
	pc = 0x8240B950; continue 'dispatch;
	}
	// 8240B91C: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 8240B920: 40990010  ble cr6, 0x8240b930
	if !ctx.cr[6].gt {
	pc = 0x8240B930; continue 'dispatch;
	}
	// 8240B924: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 8240B928: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 8240B92C: 41990024  bgt cr6, 0x8240b950
	if ctx.cr[6].gt {
	pc = 0x8240B950; continue 'dispatch;
	}
	// 8240B930: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8240B934: 80E90000  lwz r7, 0(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240B938: 7F1E3800  cmpw cr6, r30, r7
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[7].s32, &mut ctx.xer);
	// 8240B93C: 7F4B512E  stwx r26, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u32) };
	// 8240B940: 409A0018  bne cr6, 0x8240b958
	if !ctx.cr[6].eq {
	pc = 0x8240B958; continue 'dispatch;
	}
	// 8240B944: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8240B948: 7F4B412E  stwx r26, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[26].u32) };
	// 8240B94C: 4800000C  b 0x8240b958
	pc = 0x8240B958; continue 'dispatch;
	// 8240B950: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8240B954: 7F0B512E  stwx r24, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[24].u32) };
	// 8240B958: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240B95C: 39290164  addi r9, r9, 0x164
	ctx.r[9].s64 = ctx.r[9].s64 + 356;
	// 8240B960: 2F0B0300  cmpwi cr6, r11, 0x300
	ctx.cr[6].compare_i32(ctx.r[11].s32, 768, &mut ctx.xer);
	// 8240B964: 4198FFA4  blt cr6, 0x8240b908
	if ctx.cr[6].lt {
	pc = 0x8240B908; continue 'dispatch;
	}
	// 8240B968: 7F1DE000  cmpw cr6, r29, r28
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8240B96C: 409A0020  bne cr6, 0x8240b98c
	if !ctx.cr[6].eq {
	pc = 0x8240B98C; continue 'dispatch;
	}
	// 8240B970: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8240B974: 419A0010  beq cr6, 0x8240b984
	if ctx.cr[6].eq {
	pc = 0x8240B984; continue 'dispatch;
	}
	// 8240B978: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240B97C: 60630043  ori r3, r3, 0x43
	ctx.r[3].u64 = ctx.r[3].u64 | 67;
	// 8240B980: 480001E8  b 0x8240bb68
	pc = 0x8240BB68; continue 'dispatch;
	// 8240B984: 38810660  addi r4, r1, 0x660
	ctx.r[4].s64 = ctx.r[1].s64 + 1632;
	// 8240B988: 48000048  b 0x8240b9d0
	pc = 0x8240B9D0; continue 'dispatch;
	// 8240B98C: 409801C0  bge cr6, 0x8240bb4c
	if !ctx.cr[6].lt {
	pc = 0x8240BB4C; continue 'dispatch;
	}
	// 8240B990: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240B994: 7F1FC800  cmpw cr6, r31, r25
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8240B998: 41990034  bgt cr6, 0x8240b9cc
	if ctx.cr[6].gt {
	pc = 0x8240B9CC; continue 'dispatch;
	}
	// 8240B99C: 1D7F0164  mulli r11, r31, 0x164
	ctx.r[11].s64 = ctx.r[31].s64 * 356;
	// 8240B9A0: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8240B9A4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8240B9A8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240B9AC: 2C0A0000  cmpwi r10, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240B9B0: 418201B8  beq 0x8240bb68
	if ctx.cr[0].eq {
	pc = 0x8240BB68; continue 'dispatch;
	}
	// 8240B9B4: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 8240B9B8: 419A01B0  beq cr6, 0x8240bb68
	if ctx.cr[6].eq {
	pc = 0x8240BB68; continue 'dispatch;
	}
	// 8240B9BC: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8240B9C0: 396B0164  addi r11, r11, 0x164
	ctx.r[11].s64 = ctx.r[11].s64 + 356;
	// 8240B9C4: 7F03C800  cmpw cr6, r3, r25
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8240B9C8: 4099FFE0  ble cr6, 0x8240b9a8
	if !ctx.cr[6].gt {
	pc = 0x8240B9A8; continue 'dispatch;
	}
	// 8240B9CC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8240B9D0: 38A00300  li r5, 0x300
	ctx.r[5].s64 = 768;
	// 8240B9D4: 38610360  addi r3, r1, 0x360
	ctx.r[3].s64 = ctx.r[1].s64 + 864;
	// 8240B9D8: 4BFB5229  bl 0x823c0c00
	ctx.lr = 0x8240B9DC;
	sub_823C0C00(ctx, base);
	// 8240B9DC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8240B9E0: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8240B9E4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8240B9E8: 7F1FC800  cmpw cr6, r31, r25
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8240B9EC: 419900BC  bgt cr6, 0x8240baa8
	if ctx.cr[6].gt {
	pc = 0x8240BAA8; continue 'dispatch;
	}
	// 8240B9F0: 1D7F0164  mulli r11, r31, 0x164
	ctx.r[11].s64 = ctx.r[31].s64 * 356;
	// 8240B9F4: 7D4BDA14  add r10, r11, r27
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8240B9F8: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240B9FC: 38CA0034  addi r6, r10, 0x34
	ctx.r[6].s64 = ctx.r[10].s64 + 52;
	// 8240BA00: 39210360  addi r9, r1, 0x360
	ctx.r[9].s64 = ctx.r[1].s64 + 864;
	// 8240BA04: 8146FFE0  lwz r10, -0x20(r6)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-32 as u32) ) } as u64;
	// 8240BA08: 38E10960  addi r7, r1, 0x960
	ctx.r[7].s64 = ctx.r[1].s64 + 2400;
	// 8240BA0C: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8240BA10: 7F0B392E  stwx r24, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[24].u32) };
	// 8240BA14: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 8240BA18: 409A007C  bne cr6, 0x8240ba94
	if !ctx.cr[6].eq {
	pc = 0x8240BA94; continue 'dispatch;
	}
	// 8240BA1C: 81260000  lwz r9, 0(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240BA20: 2B090001  cmplwi cr6, r9, 1
	ctx.cr[6].compare_u32(ctx.r[9].u32, 1 as u32, &mut ctx.xer);
	// 8240BA24: 409A0040  bne cr6, 0x8240ba64
	if !ctx.cr[6].eq {
	pc = 0x8240BA64; continue 'dispatch;
	}
	// 8240BA28: 812A0040  lwz r9, 0x40(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 8240BA2C: 2F0903E8  cmpwi cr6, r9, 0x3e8
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1000, &mut ctx.xer);
	// 8240BA30: 40980034  bge cr6, 0x8240ba64
	if !ctx.cr[6].lt {
	pc = 0x8240BA64; continue 'dispatch;
	}
	// 8240BA34: 7D2A07B4  extsw r10, r9
	ctx.r[10].s64 = ctx.r[9].s32 as i64;
	// 8240BA38: C00600B8  lfs f0, 0xb8(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240BA3C: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8240BA40: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 8240BA44: C9A10058  lfd f13, 0x58(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8240BA48: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8240BA4C: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 8240BA50: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8240BA54: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8240BA58: 7C004FAE  stfiwx f0, 0, r9
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32, tmp.u32) };
	// 8240BA5C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240BA60: 4800000C  b 0x8240ba6c
	pc = 0x8240BA6C; continue 'dispatch;
	// 8240BA64: 814A0040  lwz r10, 0x40(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(64 as u32) ) } as u64;
	// 8240BA68: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8240BA6C: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8240BA70: 41980018  blt cr6, 0x8240ba88
	if ctx.cr[6].lt {
	pc = 0x8240BA88; continue 'dispatch;
	}
	// 8240BA74: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240BA78: 41980010  blt cr6, 0x8240ba88
	if ctx.cr[6].lt {
	pc = 0x8240BA88; continue 'dispatch;
	}
	// 8240BA7C: 7F0A2000  cmpw cr6, r10, r4
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8240BA80: 409A0014  bne cr6, 0x8240ba94
	if !ctx.cr[6].eq {
	pc = 0x8240BA94; continue 'dispatch;
	}
	// 8240BA84: 4800000C  b 0x8240ba90
	pc = 0x8240BA90; continue 'dispatch;
	// 8240BA88: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 8240BA8C: 7CA82B78  mr r8, r5
	ctx.r[8].u64 = ctx.r[5].u64;
	// 8240BA90: 7F4B392E  stwx r26, r11, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[7].u32), ctx.r[26].u32) };
	// 8240BA94: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 8240BA98: 38C60164  addi r6, r6, 0x164
	ctx.r[6].s64 = ctx.r[6].s64 + 356;
	// 8240BA9C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240BAA0: 7F05C800  cmpw cr6, r5, r25
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8240BAA4: 4099FF5C  ble cr6, 0x8240ba00
	if !ctx.cr[6].gt {
	pc = 0x8240BA00; continue 'dispatch;
	}
	// 8240BAA8: 7F172000  cmpw cr6, r23, r4
	ctx.cr[6].compare_i32(ctx.r[23].s32, ctx.r[4].s32, &mut ctx.xer);
	// 8240BAAC: 40980020  bge cr6, 0x8240bacc
	if !ctx.cr[6].lt {
	pc = 0x8240BACC; continue 'dispatch;
	}
	// 8240BAB0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BAB4: 7EE4BB78  mr r4, r23
	ctx.r[4].u64 = ctx.r[23].u64;
	// 8240BAB8: 386BDAE8  addi r3, r11, -0x2518
	ctx.r[3].s64 = ctx.r[11].s64 + -9496;
	// 8240BABC: 4BEA74C5  bl 0x822b2f80
	ctx.lr = 0x8240BAC0;
	sub_822B2F80(ctx, base);
	// 8240BAC0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240BAC4: 60630010  ori r3, r3, 0x10
	ctx.r[3].u64 = ctx.r[3].u64 | 16;
	// 8240BAC8: 480000A0  b 0x8240bb68
	pc = 0x8240BB68; continue 'dispatch;
	// 8240BACC: 3880FFFF  li r4, -1
	ctx.r[4].s64 = -1;
	// 8240BAD0: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 8240BAD4: 7F08C800  cmpw cr6, r8, r25
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8240BAD8: 4199005C  bgt cr6, 0x8240bb34
	if ctx.cr[6].gt {
	pc = 0x8240BB34; continue 'dispatch;
	}
	// 8240BADC: 1D680164  mulli r11, r8, 0x164
	ctx.r[11].s64 = ctx.r[8].s64 * 356;
	// 8240BAE0: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8240BAE4: 39210960  addi r9, r1, 0x960
	ctx.r[9].s64 = ctx.r[1].s64 + 2400;
	// 8240BAE8: 7D6BDA14  add r11, r11, r27
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 8240BAEC: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8240BAF0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240BAF4: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 8240BAF8: 409A0020  bne cr6, 0x8240bb18
	if !ctx.cr[6].eq {
	pc = 0x8240BB18; continue 'dispatch;
	}
	// 8240BAFC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240BB00: 41980010  blt cr6, 0x8240bb10
	if ctx.cr[6].lt {
	pc = 0x8240BB10; continue 'dispatch;
	}
	// 8240BB04: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240BB08: 7F074840  cmplw cr6, r7, r9
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8240BB0C: 4099000C  ble cr6, 0x8240bb18
	if !ctx.cr[6].gt {
	pc = 0x8240BB18; continue 'dispatch;
	}
	// 8240BB10: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240BB14: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 8240BB18: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8240BB1C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8240BB20: 396B0164  addi r11, r11, 0x164
	ctx.r[11].s64 = ctx.r[11].s64 + 356;
	// 8240BB24: 7F08C800  cmpw cr6, r8, r25
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[25].s32, &mut ctx.xer);
	// 8240BB28: 4099FFC8  ble cr6, 0x8240baf0
	if !ctx.cr[6].gt {
	pc = 0x8240BAF0; continue 'dispatch;
	}
	// 8240BB2C: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240BB30: 40980014  bge cr6, 0x8240bb44
	if !ctx.cr[6].lt {
	pc = 0x8240BB44; continue 'dispatch;
	}
	// 8240BB34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BB38: 386BDAA0  addi r3, r11, -0x2560
	ctx.r[3].s64 = ctx.r[11].s64 + -9568;
	// 8240BB3C: 4BEA7445  bl 0x822b2f80
	ctx.lr = 0x8240BB40;
	sub_822B2F80(ctx, base);
	// 8240BB40: 48000024  b 0x8240bb64
	pc = 0x8240BB64; continue 'dispatch;
	// 8240BB44: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8240BB48: 48000020  b 0x8240bb68
	pc = 0x8240BB68; continue 'dispatch;
	// 8240BB4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BB50: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8240BB54: 386BDA40  addi r3, r11, -0x25c0
	ctx.r[3].s64 = ctx.r[11].s64 + -9664;
	// 8240BB58: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8240BB5C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240BB60: 4BEA7421  bl 0x822b2f80
	ctx.lr = 0x8240BB64;
	sub_822B2F80(ctx, base);
	// 8240BB64: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240BB68: 38210CB0  addi r1, r1, 0xcb0
	ctx.r[1].s64 = ctx.r[1].s64 + 3248;
	// 8240BB6C: 48129588  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240BB70 size=244
    let mut pc: u32 = 0x8240BB70;
    'dispatch: loop {
        match pc {
            0x8240BB70 => {
    //   block [0x8240BB70..0x8240BC64)
	// 8240BB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240BB74: 4812953D  bl 0x825350b0
	ctx.lr = 0x8240BB78;
	sub_82535080(ctx, base);
	// 8240BB78: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240BB7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240BB80: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8240BB84: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8240BB88: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8240BB8C: 2F07FFFF  cmpwi cr6, r7, -1
	ctx.cr[6].compare_i32(ctx.r[7].s32, -1, &mut ctx.xer);
	// 8240BB90: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 8240BB94: 409A0008  bne cr6, 0x8240bb9c
	if !ctx.cr[6].eq {
	pc = 0x8240BB9C; continue 'dispatch;
	}
	// 8240BB98: 83BE0034  lwz r29, 0x34(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 8240BB9C: 817E0040  lwz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 8240BBA0: 2F0B03E8  cmpwi cr6, r11, 0x3e8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1000, &mut ctx.xer);
	// 8240BBA4: 40980030  bge cr6, 0x8240bbd4
	if !ctx.cr[6].lt {
	pc = 0x8240BBD4; continue 'dispatch;
	}
	// 8240BBA8: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8240BBAC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8240BBB0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8240BBB4: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8240BBB8: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 8240BBBC: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8240BBC0: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8240BBC4: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8240BBC8: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8240BBCC: 83810050  lwz r28, 0x50(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240BBD0: 48000008  b 0x8240bbd8
	pc = 0x8240BBD8; continue 'dispatch;
	// 8240BBD4: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 8240BBD8: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 8240BBDC: 2F0B0064  cmpwi cr6, r11, 0x64
	ctx.cr[6].compare_i32(ctx.r[11].s32, 100, &mut ctx.xer);
	// 8240BBE0: 419A0030  beq cr6, 0x8240bc10
	if ctx.cr[6].eq {
	pc = 0x8240BC10; continue 'dispatch;
	}
	// 8240BBE4: 480028FD  bl 0x8240e4e0
	ctx.lr = 0x8240BBE8;
	sub_8240E4E0(ctx, base);
	// 8240BBE8: 39400064  li r10, 0x64
	ctx.r[10].s64 = 100;
	// 8240BBEC: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 8240BBF0: 7D435396  divwu r10, r3, r10
	ctx.r[10].u32 = ctx.r[3].u32 / ctx.r[10].u32;
	// 8240BBF4: 1D4A0064  mulli r10, r10, 0x64
	ctx.r[10].s64 = ctx.r[10].s64 * 100;
	// 8240BBF8: 7D4A1850  subf r10, r10, r3
	ctx.r[10].s64 = ctx.r[3].s64 - ctx.r[10].s64;
	// 8240BBFC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8240BC00: 41990010  bgt cr6, 0x8240bc10
	if ctx.cr[6].gt {
	pc = 0x8240BC10; continue 'dispatch;
	}
	// 8240BC04: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240BC08: 60630013  ori r3, r3, 0x13
	ctx.r[3].u64 = ctx.r[3].u64 | 19;
	// 8240BC0C: 48000050  b 0x8240bc5c
	pc = 0x8240BC5C; continue 'dispatch;
	// 8240BC10: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 8240BC14: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8240BC18: 38A000BF  li r5, 0xbf
	ctx.r[5].s64 = 191;
	// 8240BC1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240BC20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240BC24: 4BFFFC8D  bl 0x8240b8b0
	ctx.lr = 0x8240BC28;
	sub_8240B8B0(ctx, base);
	// 8240BC28: 7C641B79  or. r4, r3, r3
	ctx.r[4].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 8240BC2C: 4180002C  blt 0x8240bc58
	if ctx.cr[0].lt {
	pc = 0x8240BC58; continue 'dispatch;
	}
	// 8240BC30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240BC34: 4BFFF2D5  bl 0x8240af08
	ctx.lr = 0x8240BC38;
	sub_8240AF08(ctx, base);
	// 8240BC38: 1D640164  mulli r11, r4, 0x164
	ctx.r[11].s64 = ctx.r[4].s64 * 356;
	// 8240BC3C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240BC40: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 8240BC44: 936B0000  stw r27, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8240BC48: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8240BC4C: 93CB0014  stw r30, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 8240BC50: 934B0018  stw r26, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[26].u32 ) };
	// 8240BC54: 93AB0030  stw r29, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[29].u32 ) };
	// 8240BC58: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8240BC5C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8240BC60: 481294A0  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240BC68 size=180
    let mut pc: u32 = 0x8240BC68;
    'dispatch: loop {
        match pc {
            0x8240BC68 => {
    //   block [0x8240BC68..0x8240BD1C)
	// 8240BC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240BC6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240BC70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240BC74: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 8240BC78: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 8240BC7C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8240BC80: 394000BF  li r10, 0xbf
	ctx.r[10].s64 = 191;
	// 8240BC84: 39650154  addi r11, r5, 0x154
	ctx.r[11].s64 = ctx.r[5].s64 + 340;
	// 8240BC88: C1A71FF8  lfs f13, 0x1ff8(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240BC8C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8240BC90: C0081850  lfs f0, 0x1850(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240BC94: D00BFFF8  stfs f0, -8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 8240BC98: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8240BC9C: D1ABFFFC  stfs f13, -4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8240BCA0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240BCA4: D1AB0004  stfs f13, 4(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240BCA8: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240BCAC: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240BCB0: 396B0164  addi r11, r11, 0x164
	ctx.r[11].s64 = ctx.r[11].s64 + 356;
	// 8240BCB4: 4080FFE0  bge 0x8240bc94
	if !ctx.cr[0].lt {
	pc = 0x8240BC94; continue 'dispatch;
	}
	// 8240BCB8: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 8240BCBC: 3D400001  lis r10, 1
	ctx.r[10].s64 = 65536;
	// 8240BCC0: 616B0B00  ori r11, r11, 0xb00
	ctx.r[11].u64 = ctx.r[11].u64 | 2816;
	// 8240BCC4: 61480D04  ori r8, r10, 0xd04
	ctx.r[8].u64 = ctx.r[10].u64 | 3332;
	// 8240BCC8: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 8240BCCC: 7D25592E  stwx r9, r5, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 8240BCD0: 3D650001  addis r11, r5, 1
	ctx.r[11].s64 = ctx.r[5].s64 + 65536;
	// 8240BCD4: 396B0C04  addi r11, r11, 0xc04
	ctx.r[11].s64 = ctx.r[11].s64 + 3076;
	// 8240BCD8: 7D45412E  stwx r10, r5, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[5].u32.wrapping_add(ctx.r[8].u32), ctx.r[10].u32) };
	// 8240BCDC: 912BFF00  stw r9, -0x100(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-256 as u32), ctx.r[9].u32 ) };
	// 8240BCE0: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240BCE4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8240BCE8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240BCEC: 4082FFF0  bne 0x8240bcdc
	if !ctx.cr[0].eq {
	pc = 0x8240BCDC; continue 'dispatch;
	}
	// 8240BCF0: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 8240BCF4: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8240BCF8: 4BFFF211  bl 0x8240af08
	ctx.lr = 0x8240BCFC;
	sub_8240AF08(ctx, base);
	// 8240BCFC: 38840001  addi r4, r4, 1
	ctx.r[4].s64 = ctx.r[4].s64 + 1;
	// 8240BD00: 2F0400C0  cmpwi cr6, r4, 0xc0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 192, &mut ctx.xer);
	// 8240BD04: 4198FFF0  blt cr6, 0x8240bcf4
	if ctx.cr[6].lt {
	pc = 0x8240BCF4; continue 'dispatch;
	}
	// 8240BD08: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8240BD0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240BD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240BD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240BD18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240BD20 size=12
    let mut pc: u32 = 0x8240BD20;
    'dispatch: loop {
        match pc {
            0x8240BD20 => {
    //   block [0x8240BD20..0x8240BD2C)
	// 8240BD20: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240BD24: C02B1850  lfs f1, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240BD28: 4BFFFE48  b 0x8240bb70
	sub_8240BB70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240BD30 size=4
    let mut pc: u32 = 0x8240BD30;
    'dispatch: loop {
        match pc {
            0x8240BD30 => {
    //   block [0x8240BD30..0x8240BD34)
	// 8240BD30: 4BFFFE40  b 0x8240bb70
	sub_8240BB70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240BD38 size=172
    let mut pc: u32 = 0x8240BD38;
    'dispatch: loop {
        match pc {
            0x8240BD38 => {
    //   block [0x8240BD38..0x8240BDE4)
	// 8240BD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240BD3C: 4812937D  bl 0x825350b8
	ctx.lr = 0x8240BD40;
	sub_82535080(ctx, base);
	// 8240BD40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240BD44: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240BD48: 2B04003F  cmplwi cr6, r4, 0x3f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 63 as u32, &mut ctx.xer);
	// 8240BD4C: 4199007C  bgt cr6, 0x8240bdc8
	if ctx.cr[6].gt {
	pc = 0x8240BDC8; continue 'dispatch;
	}
	// 8240BD50: 1D642008  mulli r11, r4, 0x2008
	ctx.r[11].s64 = ctx.r[4].s64 * 8200;
	// 8240BD54: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 8240BD58: 3BCB0108  addi r30, r11, 0x108
	ctx.r[30].s64 = ctx.r[11].s64 + 264;
	// 8240BD5C: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240BD60: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240BD64: 41820050  beq 0x8240bdb4
	if ctx.cr[0].eq {
	pc = 0x8240BDB4; continue 'dispatch;
	}
	// 8240BD68: 3C7D0008  addis r3, r29, 8
	ctx.r[3].s64 = ctx.r[29].s64 + 524288;
	// 8240BD6C: 3863030C  addi r3, r3, 0x30c
	ctx.r[3].s64 = ctx.r[3].s64 + 780;
	// 8240BD70: 4BFFAD01  bl 0x82406a70
	ctx.lr = 0x8240BD74;
	sub_82406A70(ctx, base);
	// 8240BD74: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240BD78: 4081003C  ble 0x8240bdb4
	if !ctx.cr[0].gt {
	pc = 0x8240BDB4; continue 'dispatch;
	}
	// 8240BD7C: 3BFE0008  addi r31, r30, 8
	ctx.r[31].s64 = ctx.r[30].s64 + 8;
	// 8240BD80: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8240BD84: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240BD88: 2C040000  cmpwi r4, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240BD8C: 4180001C  blt 0x8240bda8
	if ctx.cr[0].lt {
	pc = 0x8240BDA8; continue 'dispatch;
	}
	// 8240BD90: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8240BD94: 616B0308  ori r11, r11, 0x308
	ctx.r[11].u64 = ctx.r[11].u64 | 776;
	// 8240BD98: 7C7D582E  lwzx r3, r29, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8240BD9C: 48003DE5  bl 0x8240fb80
	ctx.lr = 0x8240BDA0;
	sub_8240FB80(ctx, base);
	// 8240BDA0: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8240BDA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240BDA8: 379CFFFF  addic. r28, r28, -1
	ctx.xer.ca = (ctx.r[28].u32 > (!(-1 as u32)));
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 8240BDAC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8240BDB0: 4082FFD4  bne 0x8240bd84
	if !ctx.cr[0].eq {
	pc = 0x8240BD84; continue 'dispatch;
	}
	// 8240BDB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240BDB8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240BDBC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240BDC0: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8240BDC4: 48000018  b 0x8240bddc
	pc = 0x8240BDDC; continue 'dispatch;
	// 8240BDC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BDCC: 386BDB38  addi r3, r11, -0x24c8
	ctx.r[3].s64 = ctx.r[11].s64 + -9416;
	// 8240BDD0: 4BEA71B1  bl 0x822b2f80
	ctx.lr = 0x8240BDD4;
	sub_822B2F80(ctx, base);
	// 8240BDD4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240BDD8: 6063001E  ori r3, r3, 0x1e
	ctx.r[3].u64 = ctx.r[3].u64 | 30;
	// 8240BDDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240BDE0: 48129328  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BDE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240BDE8 size=76
    let mut pc: u32 = 0x8240BDE8;
    'dispatch: loop {
        match pc {
            0x8240BDE8 => {
    //   block [0x8240BDE8..0x8240BE34)
	// 8240BDE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240BDEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240BDF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240BDF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240BDF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240BDFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240BE00: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240BE04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240BE08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240BE0C: 4BFFFF2D  bl 0x8240bd38
	ctx.lr = 0x8240BE10;
	sub_8240BD38(ctx, base);
	// 8240BE10: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240BE14: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240BE18: 4198FFEC  blt cr6, 0x8240be04
	if ctx.cr[6].lt {
	pc = 0x8240BE04; continue 'dispatch;
	}
	// 8240BE1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240BE20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240BE24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240BE28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240BE2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240BE30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BE38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240BE38 size=108
    let mut pc: u32 = 0x8240BE38;
    'dispatch: loop {
        match pc {
            0x8240BE38 => {
    //   block [0x8240BE38..0x8240BEA4)
	// 8240BE38: 39630108  addi r11, r3, 0x108
	ctx.r[11].s64 = ctx.r[3].s64 + 264;
	// 8240BE3C: 38C0003F  li r6, 0x3f
	ctx.r[6].s64 = 63;
	// 8240BE40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240BE44: 392B0008  addi r9, r11, 8
	ctx.r[9].s64 = ctx.r[11].s64 + 8;
	// 8240BE48: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8240BE4C: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 8240BE50: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8240BE54: 39000800  li r8, 0x800
	ctx.r[8].s64 = 2048;
	// 8240BE58: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8240BE5C: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8240BE60: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8240BE64: 4200FFF8  bdnz 0x8240be5c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8240BE5C; continue 'dispatch;
	}
	// 8240BE68: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8240BE6C: 396B2008  addi r11, r11, 0x2008
	ctx.r[11].s64 = ctx.r[11].s64 + 8200;
	// 8240BE70: 4080FFD4  bge 0x8240be44
	if !ctx.cr[0].lt {
	pc = 0x8240BE44; continue 'dispatch;
	}
	// 8240BE74: 39630008  addi r11, r3, 8
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	// 8240BE78: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8240BE7C: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 8240BE80: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8240BE84: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8240BE88: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8240BE8C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240BE90: 4200FFF8  bdnz 0x8240be88
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8240BE88; continue 'dispatch;
	}
	// 8240BE94: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8240BE98: 616B0308  ori r11, r11, 0x308
	ctx.r[11].u64 = ctx.r[11].u64 | 776;
	// 8240BE9C: 7D43592E  stwx r10, r3, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[3].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8240BEA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BEA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240BEA8 size=100
    let mut pc: u32 = 0x8240BEA8;
    'dispatch: loop {
        match pc {
            0x8240BEA8 => {
    //   block [0x8240BEA8..0x8240BF0C)
	// 8240BEA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240BEAC: 48129211  bl 0x825350bc
	ctx.lr = 0x8240BEB0;
	sub_82535080(ctx, base);
	// 8240BEB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240BEB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240BEB8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8240BEBC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8240BEC0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8240BEC4: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8240BEC8: 4BFFFF21  bl 0x8240bde8
	ctx.lr = 0x8240BECC;
	sub_8240BDE8(ctx, base);
	// 8240BECC: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 8240BED0: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 8240BED4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8240BED8: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8240BEDC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240BEE0: 4200FFF8  bdnz 0x8240bed8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8240BED8; continue 'dispatch;
	}
	// 8240BEE4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8240BEE8: 419A0018  beq cr6, 0x8240bf00
	if ctx.cr[6].eq {
	pc = 0x8240BF00; continue 'dispatch;
	}
	// 8240BEEC: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8240BEF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240BEF4: 616B0308  ori r11, r11, 0x308
	ctx.r[11].u64 = ctx.r[11].u64 | 776;
	// 8240BEF8: 7FBF592E  stwx r29, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 8240BEFC: 48000008  b 0x8240bf04
	pc = 0x8240BF04; continue 'dispatch;
	// 8240BF00: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240BF04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240BF08: 48129204  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240BF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240BF10 size=808
    let mut pc: u32 = 0x8240BF10;
    'dispatch: loop {
        match pc {
            0x8240BF10 => {
    //   block [0x8240BF10..0x8240C238)
	// 8240BF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240BF14: 48129181  bl 0x82535094
	ctx.lr = 0x8240BF18;
	sub_82535080(ctx, base);
	// 8240BF18: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240BF1C: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8240BF20: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8240BF24: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8240BF28: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 8240BF2C: 2B18003F  cmplwi cr6, r24, 0x3f
	ctx.cr[6].compare_u32(ctx.r[24].u32, 63 as u32, &mut ctx.xer);
	// 8240BF30: 40990030  ble cr6, 0x8240bf60
	if !ctx.cr[6].gt {
	pc = 0x8240BF60; continue 'dispatch;
	}
	// 8240BF34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BF38: 386BDB88  addi r3, r11, -0x2478
	ctx.r[3].s64 = ctx.r[11].s64 + -9336;
	// 8240BF3C: 4BEA7045  bl 0x822b2f80
	ctx.lr = 0x8240BF40;
	sub_822B2F80(ctx, base);
	// 8240BF40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BF44: 3C808000  lis r4, -0x8000
	ctx.r[4].s64 = -2147483648;
	// 8240BF48: 386BDE28  addi r3, r11, -0x21d8
	ctx.r[3].s64 = ctx.r[11].s64 + -8664;
	// 8240BF4C: 6084001E  ori r4, r4, 0x1e
	ctx.r[4].u64 = ctx.r[4].u64 | 30;
	// 8240BF50: 4BEA7031  bl 0x822b2f80
	ctx.lr = 0x8240BF54;
	sub_822B2F80(ctx, base);
	// 8240BF54: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240BF58: 6063001E  ori r3, r3, 0x1e
	ctx.r[3].u64 = ctx.r[3].u64 | 30;
	// 8240BF5C: 480002D4  b 0x8240c230
	pc = 0x8240C230; continue 'dispatch;
	// 8240BF60: 1D782008  mulli r11, r24, 0x2008
	ctx.r[11].s64 = ctx.r[24].s64 * 8200;
	// 8240BF64: 7E6BE214  add r19, r11, r28
	ctx.r[19].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8240BF68: 81730108  lwz r11, 0x108(r19)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[19].u32.wrapping_add(264 as u32) ) } as u64;
	// 8240BF6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240BF70: 419A0020  beq cr6, 0x8240bf90
	if ctx.cr[6].eq {
	pc = 0x8240BF90; continue 'dispatch;
	}
	// 8240BF74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BF78: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8240BF7C: 386BDDD8  addi r3, r11, -0x2228
	ctx.r[3].s64 = ctx.r[11].s64 + -8744;
	// 8240BF80: 4BEA7001  bl 0x822b2f80
	ctx.lr = 0x8240BF84;
	sub_822B2F80(ctx, base);
	// 8240BF84: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240BF88: 60630021  ori r3, r3, 0x21
	ctx.r[3].u64 = ctx.r[3].u64 | 33;
	// 8240BF8C: 480002A4  b 0x8240c230
	pc = 0x8240C230; continue 'dispatch;
	// 8240BF90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240BF94: 409A001C  bne cr6, 0x8240bfb0
	if !ctx.cr[6].eq {
	pc = 0x8240BFB0; continue 'dispatch;
	}
	// 8240BF98: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240BF9C: 386BDD90  addi r3, r11, -0x2270
	ctx.r[3].s64 = ctx.r[11].s64 + -8816;
	// 8240BFA0: 4BEA6FE1  bl 0x822b2f80
	ctx.lr = 0x8240BFA4;
	sub_822B2F80(ctx, base);
	// 8240BFA4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240BFA8: 60630024  ori r3, r3, 0x24
	ctx.r[3].u64 = ctx.r[3].u64 | 36;
	// 8240BFAC: 48000284  b 0x8240c230
	pc = 0x8240C230; continue 'dispatch;
	// 8240BFB0: 3FDC0008  addis r30, r28, 8
	ctx.r[30].s64 = ctx.r[28].s64 + 524288;
	// 8240BFB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240BFB8: 3BDE030C  addi r30, r30, 0x30c
	ctx.r[30].s64 = ctx.r[30].s64 + 780;
	// 8240BFBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240BFC0: 4BFFAA31  bl 0x824069f0
	ctx.lr = 0x8240BFC4;
	sub_824069F0(ctx, base);
	// 8240BFC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240BFC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240BFCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240BFD0: 4BFFA9E1  bl 0x824069b0
	ctx.lr = 0x8240BFD4;
	sub_824069B0(ctx, base);
	// 8240BFD4: 7C640774  extsb r4, r3
	ctx.r[4].s64 = ctx.r[3].s8 as i64;
	// 8240BFD8: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 8240BFDC: 409A023C  bne cr6, 0x8240c218
	if !ctx.cr[6].eq {
	pc = 0x8240C218; continue 'dispatch;
	}
	// 8240BFE0: 7FAB0774  extsb r11, r29
	ctx.r[11].s64 = ctx.r[29].s8 as i64;
	// 8240BFE4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8240BFE8: 409A0230  bne cr6, 0x8240c218
	if !ctx.cr[6].eq {
	pc = 0x8240C218; continue 'dispatch;
	}
	// 8240BFEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240BFF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240BFF4: 4BFFA97D  bl 0x82406970
	ctx.lr = 0x8240BFF8;
	sub_82406970(ctx, base);
	// 8240BFF8: 7C6B0775  extsb. r11, r3
	ctx.r[11].s64 = ctx.r[3].s8 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240BFFC: 4182001C  beq 0x8240c018
	if ctx.cr[0].eq {
	pc = 0x8240C018; continue 'dispatch;
	}
	// 8240C000: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C004: 386BDD48  addi r3, r11, -0x22b8
	ctx.r[3].s64 = ctx.r[11].s64 + -8888;
	// 8240C008: 4BEA6F79  bl 0x822b2f80
	ctx.lr = 0x8240C00C;
	sub_822B2F80(ctx, base);
	// 8240C00C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C010: 60630042  ori r3, r3, 0x42
	ctx.r[3].u64 = ctx.r[3].u64 | 66;
	// 8240C014: 4800021C  b 0x8240c230
	pc = 0x8240C230; continue 'dispatch;
	// 8240C018: 2B150000  cmplwi cr6, r21, 0
	ctx.cr[6].compare_u32(ctx.r[21].u32, 0 as u32, &mut ctx.xer);
	// 8240C01C: 409A0014  bne cr6, 0x8240c030
	if !ctx.cr[6].eq {
	pc = 0x8240C030; continue 'dispatch;
	}
	// 8240C020: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C024: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C028: 4BFFAA89  bl 0x82406ab0
	ctx.lr = 0x8240C02C;
	sub_82406AB0(ctx, base);
	// 8240C02C: 7EA3FA14  add r21, r3, r31
	ctx.r[21].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 8240C030: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C034: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C038: 4BFFAA39  bl 0x82406a70
	ctx.lr = 0x8240C03C;
	sub_82406A70(ctx, base);
	// 8240C03C: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 8240C040: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C044: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C048: 4BFFAAA9  bl 0x82406af0
	ctx.lr = 0x8240C04C;
	sub_82406AF0(ctx, base);
	// 8240C04C: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8240C050: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8240C054: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8240C058: 61760308  ori r22, r11, 0x308
	ctx.r[22].u64 = ctx.r[11].u64 | 776;
	// 8240C05C: 2F140000  cmpwi cr6, r20, 0
	ctx.cr[6].compare_i32(ctx.r[20].s32, 0, &mut ctx.xer);
	// 8240C060: 409900D8  ble cr6, 0x8240c138
	if !ctx.cr[6].gt {
	pc = 0x8240C138; continue 'dispatch;
	}
	// 8240C064: 3B530110  addi r26, r19, 0x110
	ctx.r[26].s64 = ctx.r[19].s64 + 272;
	// 8240C068: 7EFCB214  add r23, r28, r22
	ctx.r[23].u64 = ctx.r[28].u64 + ctx.r[22].u64;
	// 8240C06C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8240C070: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240C074: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C078: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C07C: 4BFFAC55  bl 0x82406cd0
	ctx.lr = 0x8240C080;
	sub_82406CD0(ctx, base);
	// 8240C080: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8240C084: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240C088: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C08C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C090: 4BFFACC1  bl 0x82406d50
	ctx.lr = 0x8240C094;
	sub_82406D50(ctx, base);
	// 8240C094: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 8240C098: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240C09C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C0A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C0A4: 4BFFAD2D  bl 0x82406dd0
	ctx.lr = 0x8240C0A8;
	sub_82406DD0(ctx, base);
	// 8240C0A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8240C0AC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240C0B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C0B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C0B8: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8240C0BC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240C0C0: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8240C0C4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8240C0C8: 7D6BAA14  add r11, r11, r21
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[21].u64;
	// 8240C0CC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 8240C0D0: 4BFFABB1  bl 0x82406c80
	ctx.lr = 0x8240C0D4;
	sub_82406C80(ctx, base);
	// 8240C0D4: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 8240C0D8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240C0DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C0E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C0E4: 4BFFAB65  bl 0x82406c48
	ctx.lr = 0x8240C0E8;
	sub_82406C48(ctx, base);
	// 8240C0E8: 90610078  stw r3, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[3].u32 ) };
	// 8240C0EC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8240C0F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C0F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C0F8: 4BFFAB11  bl 0x82406c08
	ctx.lr = 0x8240C0FC;
	sub_82406C08(ctx, base);
	// 8240C0FC: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8240C100: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8240C104: 9061007C  stw r3, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 8240C108: 80770000  lwz r3, 0(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C10C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8240C110: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 8240C114: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8240C118: 480039F1  bl 0x8240fb08
	ctx.lr = 0x8240C11C;
	sub_8240FB08(ctx, base);
	// 8240C11C: 7C7B1B79  or. r27, r3, r3
	ctx.r[27].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8240C120: 937A0000  stw r27, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 8240C124: 41800060  blt 0x8240c184
	if ctx.cr[0].lt {
	pc = 0x8240C184; continue 'dispatch;
	}
	// 8240C128: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8240C12C: 3B5A0004  addi r26, r26, 4
	ctx.r[26].s64 = ctx.r[26].s64 + 4;
	// 8240C130: 7F1DA000  cmpw cr6, r29, r20
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[20].s32, &mut ctx.xer);
	// 8240C134: 4198FF38  blt cr6, 0x8240c06c
	if ctx.cr[6].lt {
	pc = 0x8240C06C; continue 'dispatch;
	}
	// 8240C138: 7FBCB214  add r29, r28, r22
	ctx.r[29].u64 = ctx.r[28].u64 + ctx.r[22].u64;
	// 8240C13C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 8240C140: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C144: 48003A5D  bl 0x8240fba0
	ctx.lr = 0x8240C148;
	sub_8240FBA0(ctx, base);
	// 8240C148: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240C14C: 40800060  bge 0x8240c1ac
	if !ctx.cr[0].lt {
	pc = 0x8240C1AC; continue 'dispatch;
	}
	// 8240C150: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C154: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C158: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 8240C15C: 386BDCF0  addi r3, r11, -0x2310
	ctx.r[3].s64 = ctx.r[11].s64 + -8976;
	// 8240C160: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8240C164: 7C8BCA14  add r4, r11, r25
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[25].u64;
	// 8240C168: 4BEA6E19  bl 0x822b2f80
	ctx.lr = 0x8240C16C;
	sub_822B2F80(ctx, base);
	// 8240C16C: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8240C170: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8240C174: 4BFFFBC5  bl 0x8240bd38
	ctx.lr = 0x8240C178;
	sub_8240BD38(ctx, base);
	// 8240C178: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C17C: 60630027  ori r3, r3, 0x27
	ctx.r[3].u64 = ctx.r[3].u64 | 39;
	// 8240C180: 480000B0  b 0x8240c230
	pc = 0x8240C230; continue 'dispatch;
	// 8240C184: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8240C188: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8240C18C: 4BFFFBAD  bl 0x8240bd38
	ctx.lr = 0x8240C190;
	sub_8240BD38(ctx, base);
	// 8240C190: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C194: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8240C198: 386BDC98  addi r3, r11, -0x2368
	ctx.r[3].s64 = ctx.r[11].s64 + -9064;
	// 8240C19C: 4BEA6DE5  bl 0x822b2f80
	ctx.lr = 0x8240C1A0;
	sub_822B2F80(ctx, base);
	// 8240C1A0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C1A4: 60630026  ori r3, r3, 0x26
	ctx.r[3].u64 = ctx.r[3].u64 | 38;
	// 8240C1A8: 48000088  b 0x8240c230
	pc = 0x8240C230; continue 'dispatch;
	// 8240C1AC: 39780002  addi r11, r24, 2
	ctx.r[11].s64 = ctx.r[24].s64 + 2;
	// 8240C1B0: 93F30108  stw r31, 0x108(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(264 as u32), ctx.r[31].u32 ) };
	// 8240C1B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8240C1B8: 92B3010C  stw r21, 0x10c(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(268 as u32), ctx.r[21].u32 ) };
	// 8240C1BC: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240C1C0: 7C695670  srawi r9, r3, 0xa
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[3].s32 >> 10) as i64;
	// 8240C1C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C1C8: 7FA90194  addze r29, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[29].s64 = tmp.s64;
	// 8240C1CC: 7F295670  srawi r9, r25, 0xa
	ctx.xer.ca = (ctx.r[25].s32 < 0) && ((ctx.r[25].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[25].s32 >> 10) as i64;
	// 8240C1D0: 7D4BE12E  stwx r10, r11, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[28].u32), ctx.r[10].u32) };
	// 8240C1D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8240C1D8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C1DC: 7FC90194  addze r30, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[30].s64 = tmp.s64;
	// 8240C1E0: 3BEB0001  addi r31, r11, 1
	ctx.r[31].s64 = ctx.r[11].s64 + 1;
	// 8240C1E4: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8240C1E8: 4BFFA849  bl 0x82406a30
	ctx.lr = 0x8240C1EC;
	sub_82406A30(ctx, base);
	// 8240C1EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C1F0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8240C1F4: 396BDC30  addi r11, r11, -0x23d0
	ctx.r[11].s64 = ctx.r[11].s64 + -9168;
	// 8240C1F8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 8240C1FC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8240C200: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 8240C204: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 8240C208: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 8240C20C: 4BEA6D75  bl 0x822b2f80
	ctx.lr = 0x8240C210;
	sub_822B2F80(ctx, base);
	// 8240C210: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240C214: 4800001C  b 0x8240c230
	pc = 0x8240C230; continue 'dispatch;
	// 8240C218: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C21C: 7FA50774  extsb r5, r29
	ctx.r[5].s64 = ctx.r[29].s8 as i64;
	// 8240C220: 386BDBD8  addi r3, r11, -0x2428
	ctx.r[3].s64 = ctx.r[11].s64 + -9256;
	// 8240C224: 4BEA6D5D  bl 0x822b2f80
	ctx.lr = 0x8240C228;
	sub_822B2F80(ctx, base);
	// 8240C228: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C22C: 6063001D  ori r3, r3, 0x1d
	ctx.r[3].u64 = ctx.r[3].u64 | 29;
	// 8240C230: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8240C234: 48128EB0  b 0x825350e4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240C238 size=264
    let mut pc: u32 = 0x8240C238;
    'dispatch: loop {
        match pc {
            0x8240C238 => {
    //   block [0x8240C238..0x8240C340)
	// 8240C238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C23C: 48128E7D  bl 0x825350b8
	ctx.lr = 0x8240C240;
	sub_82535080(ctx, base);
	// 8240C240: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C244: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8240C248: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240C24C: 2B1E003F  cmplwi cr6, r30, 0x3f
	ctx.cr[6].compare_u32(ctx.r[30].u32, 63 as u32, &mut ctx.xer);
	// 8240C250: 40990030  ble cr6, 0x8240c280
	if !ctx.cr[6].gt {
	pc = 0x8240C280; continue 'dispatch;
	}
	// 8240C254: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C258: 386BDB88  addi r3, r11, -0x2478
	ctx.r[3].s64 = ctx.r[11].s64 + -9336;
	// 8240C25C: 4BEA6D25  bl 0x822b2f80
	ctx.lr = 0x8240C260;
	sub_822B2F80(ctx, base);
	// 8240C260: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C264: 3C808000  lis r4, -0x8000
	ctx.r[4].s64 = -2147483648;
	// 8240C268: 386BDF18  addi r3, r11, -0x20e8
	ctx.r[3].s64 = ctx.r[11].s64 + -8424;
	// 8240C26C: 6084001E  ori r4, r4, 0x1e
	ctx.r[4].u64 = ctx.r[4].u64 | 30;
	// 8240C270: 4BEA6D11  bl 0x822b2f80
	ctx.lr = 0x8240C274;
	sub_822B2F80(ctx, base);
	// 8240C274: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C278: 6063001E  ori r3, r3, 0x1e
	ctx.r[3].u64 = ctx.r[3].u64 | 30;
	// 8240C27C: 480000BC  b 0x8240c338
	pc = 0x8240C338; continue 'dispatch;
	// 8240C280: 1D7E2008  mulli r11, r30, 0x2008
	ctx.r[11].s64 = ctx.r[30].s64 * 8200;
	// 8240C284: 7F8BFA14  add r28, r11, r31
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240C288: 817C0108  lwz r11, 0x108(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(264 as u32) ) } as u64;
	// 8240C28C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8240C290: 409A0020  bne cr6, 0x8240c2b0
	if !ctx.cr[6].eq {
	pc = 0x8240C2B0; continue 'dispatch;
	}
	// 8240C294: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C298: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240C29C: 386BDEC8  addi r3, r11, -0x2138
	ctx.r[3].s64 = ctx.r[11].s64 + -8504;
	// 8240C2A0: 4BEA6CE1  bl 0x822b2f80
	ctx.lr = 0x8240C2A4;
	sub_822B2F80(ctx, base);
	// 8240C2A4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C2A8: 60630020  ori r3, r3, 0x20
	ctx.r[3].u64 = ctx.r[3].u64 | 32;
	// 8240C2AC: 4800008C  b 0x8240c338
	pc = 0x8240C338; continue 'dispatch;
	// 8240C2B0: 3FBF0008  addis r29, r31, 8
	ctx.r[29].s64 = ctx.r[31].s64 + 524288;
	// 8240C2B4: 809C0108  lwz r4, 0x108(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(264 as u32) ) } as u64;
	// 8240C2B8: 3BBD030C  addi r29, r29, 0x30c
	ctx.r[29].s64 = ctx.r[29].s64 + 780;
	// 8240C2BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240C2C0: 4BFFA7B1  bl 0x82406a70
	ctx.lr = 0x8240C2C4;
	sub_82406A70(ctx, base);
	// 8240C2C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240C2C8: 809C0108  lwz r4, 0x108(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(264 as u32) ) } as u64;
	// 8240C2CC: 4BFFA825  bl 0x82406af0
	ctx.lr = 0x8240C2D0;
	sub_82406AF0(ctx, base);
	// 8240C2D0: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8240C2D4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240C2D8: 616B0308  ori r11, r11, 0x308
	ctx.r[11].u64 = ctx.r[11].u64 | 776;
	// 8240C2DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8240C2E0: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8240C2E4: 480038ED  bl 0x8240fbd0
	ctx.lr = 0x8240C2E8;
	sub_8240FBD0(ctx, base);
	// 8240C2E8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8240C2EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240C2F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240C2F4: 4BFFFA45  bl 0x8240bd38
	ctx.lr = 0x8240C2F8;
	sub_8240BD38(ctx, base);
	// 8240C2F8: 397E0002  addi r11, r30, 2
	ctx.r[11].s64 = ctx.r[30].s64 + 2;
	// 8240C2FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240C300: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8240C304: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C308: 7F885670  srawi r8, r28, 0xa
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[28].s32 >> 10) as i64;
	// 8240C30C: 386BDE70  addi r3, r11, -0x2190
	ctx.r[3].s64 = ctx.r[11].s64 + -8592;
	// 8240C310: 7CE80194  addze r7, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[7].s64 = tmp.s64;
	// 8240C314: 7D49F92E  stwx r10, r9, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 8240C318: 7FA85670  srawi r8, r29, 0xa
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 10) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[29].s32 >> 10) as i64;
	// 8240C31C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C320: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240C324: 7CC80194  addze r6, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[6].s64 = tmp.s64;
	// 8240C328: 38ABFFFF  addi r5, r11, -1
	ctx.r[5].s64 = ctx.r[11].s64 + -1;
	// 8240C32C: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 8240C330: 4BEA6C51  bl 0x822b2f80
	ctx.lr = 0x8240C334;
	sub_822B2F80(ctx, base);
	// 8240C334: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240C338: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240C33C: 48128DCC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C340 size=152
    let mut pc: u32 = 0x8240C340;
    'dispatch: loop {
        match pc {
            0x8240C340 => {
    //   block [0x8240C340..0x8240C3D8)
	// 8240C340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C344: 48128D75  bl 0x825350b8
	ctx.lr = 0x8240C348;
	sub_82535080(ctx, base);
	// 8240C348: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C34C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240C350: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8240C354: 3BDF0108  addi r30, r31, 0x108
	ctx.r[30].s64 = ctx.r[31].s64 + 264;
	// 8240C358: 3BA00040  li r29, 0x40
	ctx.r[29].s64 = 64;
	// 8240C35C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8240C360: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 8240C364: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C368: 28040000  cmplwi r4, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240C36C: 41820024  beq 0x8240c390
	if ctx.cr[0].eq {
	pc = 0x8240C390; continue 'dispatch;
	}
	// 8240C370: 3C7F0008  addis r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 524288;
	// 8240C374: 3863030C  addi r3, r3, 0x30c
	ctx.r[3].s64 = ctx.r[3].s64 + 780;
	// 8240C378: 4BFFA779  bl 0x82406af0
	ctx.lr = 0x8240C37C;
	sub_82406AF0(ctx, base);
	// 8240C37C: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8240C380: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8240C384: 616B0308  ori r11, r11, 0x308
	ctx.r[11].u64 = ctx.r[11].u64 | 776;
	// 8240C388: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8240C38C: 48003845  bl 0x8240fbd0
	ctx.lr = 0x8240C390;
	sub_8240FBD0(ctx, base);
	// 8240C390: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8240C394: 3BDE2008  addi r30, r30, 0x2008
	ctx.r[30].s64 = ctx.r[30].s64 + 8200;
	// 8240C398: 4082FFCC  bne 0x8240c364
	if !ctx.cr[0].eq {
	pc = 0x8240C364; continue 'dispatch;
	}
	// 8240C39C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240C3A0: 4BFFFA49  bl 0x8240bde8
	ctx.lr = 0x8240C3A4;
	sub_8240BDE8(ctx, base);
	// 8240C3A4: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 8240C3A8: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 8240C3AC: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 8240C3B0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8240C3B4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8240C3B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8240C3BC: 4200FFF8  bdnz 0x8240c3b4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8240C3B4; continue 'dispatch;
	}
	// 8240C3C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C3C4: 386BDF5C  addi r3, r11, -0x20a4
	ctx.r[3].s64 = ctx.r[11].s64 + -8356;
	// 8240C3C8: 4BEA6BB9  bl 0x822b2f80
	ctx.lr = 0x8240C3CC;
	sub_822B2F80(ctx, base);
	// 8240C3CC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240C3D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240C3D4: 48128D34  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C3D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C3D8 size=168
    let mut pc: u32 = 0x8240C3D8;
    'dispatch: loop {
        match pc {
            0x8240C3D8 => {
    //   block [0x8240C3D8..0x8240C480)
	// 8240C3D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C3DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C3E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C3E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C3E8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8240C3EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8240C3F0: 409A001C  bne cr6, 0x8240c40c
	if !ctx.cr[6].eq {
	pc = 0x8240C40C; continue 'dispatch;
	}
	// 8240C3F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C3F8: 386BDFD0  addi r3, r11, -0x2030
	ctx.r[3].s64 = ctx.r[11].s64 + -8240;
	// 8240C3FC: 4BEA6B85  bl 0x822b2f80
	ctx.lr = 0x8240C400;
	sub_822B2F80(ctx, base);
	// 8240C400: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C404: 60630022  ori r3, r3, 0x22
	ctx.r[3].u64 = ctx.r[3].u64 | 34;
	// 8240C408: 48000064  b 0x8240c46c
	pc = 0x8240C46C; continue 'dispatch;
	// 8240C40C: 2B04003F  cmplwi cr6, r4, 0x3f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 63 as u32, &mut ctx.xer);
	// 8240C410: 40990034  ble cr6, 0x8240c444
	if !ctx.cr[6].gt {
	pc = 0x8240C444; continue 'dispatch;
	}
	// 8240C414: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C418: 386BDB88  addi r3, r11, -0x2478
	ctx.r[3].s64 = ctx.r[11].s64 + -9336;
	// 8240C41C: 4BEA6B65  bl 0x822b2f80
	ctx.lr = 0x8240C420;
	sub_822B2F80(ctx, base);
	// 8240C420: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C424: 3C808000  lis r4, -0x8000
	ctx.r[4].s64 = -2147483648;
	// 8240C428: 386BDF88  addi r3, r11, -0x2078
	ctx.r[3].s64 = ctx.r[11].s64 + -8312;
	// 8240C42C: 6084001E  ori r4, r4, 0x1e
	ctx.r[4].u64 = ctx.r[4].u64 | 30;
	// 8240C430: 4BEA6B51  bl 0x822b2f80
	ctx.lr = 0x8240C434;
	sub_822B2F80(ctx, base);
	// 8240C434: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C438: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8240C43C: 6063001E  ori r3, r3, 0x1e
	ctx.r[3].u64 = ctx.r[3].u64 | 30;
	// 8240C440: 48000028  b 0x8240c468
	pc = 0x8240C468; continue 'dispatch;
	// 8240C444: 39640002  addi r11, r4, 2
	ctx.r[11].s64 = ctx.r[4].s64 + 2;
	// 8240C448: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240C44C: 7D4B182E  lwzx r10, r11, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8240C450: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 8240C454: 409A000C  bne cr6, 0x8240c460
	if !ctx.cr[6].eq {
	pc = 0x8240C460; continue 'dispatch;
	}
	// 8240C458: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 8240C45C: 7D4B192E  stwx r10, r11, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), ctx.r[10].u32) };
	// 8240C460: 7D6B182E  lwzx r11, r11, r3
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8240C464: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240C468: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240C46C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240C470: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C474: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C478: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C480 size=180
    let mut pc: u32 = 0x8240C480;
    'dispatch: loop {
        match pc {
            0x8240C480 => {
    //   block [0x8240C480..0x8240C534)
	// 8240C480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C488: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C48C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C490: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8240C494: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8240C498: 409A001C  bne cr6, 0x8240c4b4
	if !ctx.cr[6].eq {
	pc = 0x8240C4B4; continue 'dispatch;
	}
	// 8240C49C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C4A0: 386BE0C0  addi r3, r11, -0x1f40
	ctx.r[3].s64 = ctx.r[11].s64 + -8000;
	// 8240C4A4: 4BEA6ADD  bl 0x822b2f80
	ctx.lr = 0x8240C4A8;
	sub_822B2F80(ctx, base);
	// 8240C4A8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C4AC: 60630024  ori r3, r3, 0x24
	ctx.r[3].u64 = ctx.r[3].u64 | 36;
	// 8240C4B0: 48000070  b 0x8240c520
	pc = 0x8240C520; continue 'dispatch;
	// 8240C4B4: 2B1F003F  cmplwi cr6, r31, 0x3f
	ctx.cr[6].compare_u32(ctx.r[31].u32, 63 as u32, &mut ctx.xer);
	// 8240C4B8: 40990030  ble cr6, 0x8240c4e8
	if !ctx.cr[6].gt {
	pc = 0x8240C4E8; continue 'dispatch;
	}
	// 8240C4BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C4C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C4C4: 386BDB88  addi r3, r11, -0x2478
	ctx.r[3].s64 = ctx.r[11].s64 + -9336;
	// 8240C4C8: 4BEA6AB9  bl 0x822b2f80
	ctx.lr = 0x8240C4CC;
	sub_822B2F80(ctx, base);
	// 8240C4CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C4D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C4D4: 386BE070  addi r3, r11, -0x1f90
	ctx.r[3].s64 = ctx.r[11].s64 + -8080;
	// 8240C4D8: 4BEA6AA9  bl 0x822b2f80
	ctx.lr = 0x8240C4DC;
	sub_822B2F80(ctx, base);
	// 8240C4DC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C4E0: 6063001E  ori r3, r3, 0x1e
	ctx.r[3].u64 = ctx.r[3].u64 | 30;
	// 8240C4E4: 4800003C  b 0x8240c520
	pc = 0x8240C520; continue 'dispatch;
	// 8240C4E8: 1D7F2008  mulli r11, r31, 0x2008
	ctx.r[11].s64 = ctx.r[31].s64 * 8200;
	// 8240C4EC: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240C4F0: 816B0108  lwz r11, 0x108(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(264 as u32) ) } as u64;
	// 8240C4F4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8240C4F8: 40820020  bne 0x8240c518
	if !ctx.cr[0].eq {
	pc = 0x8240C518; continue 'dispatch;
	}
	// 8240C4FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C500: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C504: 386BE018  addi r3, r11, -0x1fe8
	ctx.r[3].s64 = ctx.r[11].s64 + -8168;
	// 8240C508: 4BEA6A79  bl 0x822b2f80
	ctx.lr = 0x8240C50C;
	sub_822B2F80(ctx, base);
	// 8240C50C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C510: 60630020  ori r3, r3, 0x20
	ctx.r[3].u64 = ctx.r[3].u64 | 32;
	// 8240C514: 4800000C  b 0x8240c520
	pc = 0x8240C520; continue 'dispatch;
	// 8240C518: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240C51C: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240C520: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240C524: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C528: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C52C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C530: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C538 size=136
    let mut pc: u32 = 0x8240C538;
    'dispatch: loop {
        match pc {
            0x8240C538 => {
    //   block [0x8240C538..0x8240C5C0)
	// 8240C538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C53C: 48128B81  bl 0x825350bc
	ctx.lr = 0x8240C540;
	sub_82535080(ctx, base);
	// 8240C540: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C544: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8240C548: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8240C54C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240C550: 4BFFFF31  bl 0x8240c480
	ctx.lr = 0x8240C554;
	sub_8240C480(ctx, base);
	// 8240C554: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240C558: 4182001C  beq 0x8240c574
	if ctx.cr[0].eq {
	pc = 0x8240C574; continue 'dispatch;
	}
	// 8240C55C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C560: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C564: 386BE168  addi r3, r11, -0x1e98
	ctx.r[3].s64 = ctx.r[11].s64 + -7832;
	// 8240C568: 4BEA6A19  bl 0x822b2f80
	ctx.lr = 0x8240C56C;
	sub_822B2F80(ctx, base);
	// 8240C56C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240C570: 48000048  b 0x8240c5b8
	pc = 0x8240C5B8; continue 'dispatch;
	// 8240C574: 3C7D0008  addis r3, r29, 8
	ctx.r[3].s64 = ctx.r[29].s64 + 524288;
	// 8240C578: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240C57C: 3863030C  addi r3, r3, 0x30c
	ctx.r[3].s64 = ctx.r[3].s64 + 780;
	// 8240C580: 4BFFA4F1  bl 0x82406a70
	ctx.lr = 0x8240C584;
	sub_82406A70(ctx, base);
	// 8240C584: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 8240C588: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 8240C58C: 41980014  blt cr6, 0x8240c5a0
	if ctx.cr[6].lt {
	pc = 0x8240C5A0; continue 'dispatch;
	}
	// 8240C590: 7F1E2800  cmpw cr6, r30, r5
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[5].s32, &mut ctx.xer);
	// 8240C594: 4098000C  bge cr6, 0x8240c5a0
	if !ctx.cr[6].lt {
	pc = 0x8240C5A0; continue 'dispatch;
	}
	// 8240C598: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240C59C: 4800001C  b 0x8240c5b8
	pc = 0x8240C5B8; continue 'dispatch;
	// 8240C5A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C5A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8240C5A8: 386BE110  addi r3, r11, -0x1ef0
	ctx.r[3].s64 = ctx.r[11].s64 + -7920;
	// 8240C5AC: 4BEA69D5  bl 0x822b2f80
	ctx.lr = 0x8240C5B0;
	sub_822B2F80(ctx, base);
	// 8240C5B0: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C5B4: 6063000F  ori r3, r3, 0xf
	ctx.r[3].u64 = ctx.r[3].u64 | 15;
	// 8240C5B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240C5BC: 48128B50  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C5C0 size=180
    let mut pc: u32 = 0x8240C5C0;
    'dispatch: loop {
        match pc {
            0x8240C5C0 => {
    //   block [0x8240C5C0..0x8240C674)
	// 8240C5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C5C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240C5CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C5D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C5D4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8240C5D8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8240C5DC: 39630108  addi r11, r3, 0x108
	ctx.r[11].s64 = ctx.r[3].s64 + 264;
	// 8240C5E0: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C5E4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8240C5E8: 419A0018  beq cr6, 0x8240c600
	if ctx.cr[6].eq {
	pc = 0x8240C600; continue 'dispatch;
	}
	// 8240C5EC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8240C5F0: 396B2008  addi r11, r11, 0x2008
	ctx.r[11].s64 = ctx.r[11].s64 + 8200;
	// 8240C5F4: 2F1F0040  cmpwi cr6, r31, 0x40
	ctx.cr[6].compare_i32(ctx.r[31].s32, 64, &mut ctx.xer);
	// 8240C5F8: 4198FFE8  blt cr6, 0x8240c5e0
	if ctx.cr[6].lt {
	pc = 0x8240C5E0; continue 'dispatch;
	}
	// 8240C5FC: 4800000C  b 0x8240c608
	pc = 0x8240C608; continue 'dispatch;
	// 8240C600: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240C604: 40980020  bge cr6, 0x8240c624
	if !ctx.cr[6].lt {
	pc = 0x8240C624; continue 'dispatch;
	}
	// 8240C608: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C60C: 38800040  li r4, 0x40
	ctx.r[4].s64 = 64;
	// 8240C610: 386BE1E8  addi r3, r11, -0x1e18
	ctx.r[3].s64 = ctx.r[11].s64 + -7704;
	// 8240C614: 4BEA696D  bl 0x822b2f80
	ctx.lr = 0x8240C618;
	sub_822B2F80(ctx, base);
	// 8240C618: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C61C: 6063001B  ori r3, r3, 0x1b
	ctx.r[3].u64 = ctx.r[3].u64 | 27;
	// 8240C620: 4800003C  b 0x8240c65c
	pc = 0x8240C65C; continue 'dispatch;
	// 8240C624: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 8240C628: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8240C62C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C630: 4BFFF8E1  bl 0x8240bf10
	ctx.lr = 0x8240C634;
	sub_8240BF10(ctx, base);
	// 8240C634: 2C030000  cmpwi r3, 0
	ctx.cr[0].compare_i32(ctx.r[0].s32, 0, &mut ctx.xer);
	// 8240C638: 40820024  bne 0x8240c65c
	if !ctx.cr[0].eq {
	pc = 0x8240C65C; continue 'dispatch;
	}
	// 8240C63C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8240C640: 419A0008  beq cr6, 0x8240c648
	if ctx.cr[6].eq {
	pc = 0x8240C648; continue 'dispatch;
	}
	// 8240C644: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 8240C648: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C64C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C650: 386BE1B0  addi r3, r11, -0x1e50
	ctx.r[3].s64 = ctx.r[11].s64 + -7760;
	// 8240C654: 4BEA692D  bl 0x822b2f80
	ctx.lr = 0x8240C658;
	sub_822B2F80(ctx, base);
	// 8240C658: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240C65C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240C660: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C664: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C668: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240C66C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C670: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C678 size=60
    let mut pc: u32 = 0x8240C678;
    'dispatch: loop {
        match pc {
            0x8240C678 => {
    //   block [0x8240C678..0x8240C6B4)
	// 8240C678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C67C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C680: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C684: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C688: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240C68C: 4BFFFCB5  bl 0x8240c340
	ctx.lr = 0x8240C690;
	sub_8240C340(ctx, base);
	// 8240C690: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8240C694: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240C698: 616B0308  ori r11, r11, 0x308
	ctx.r[11].u64 = ctx.r[11].u64 | 776;
	// 8240C69C: 7D5F592E  stwx r10, r31, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32), ctx.r[10].u32) };
	// 8240C6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240C6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C6AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C6B8 size=64
    let mut pc: u32 = 0x8240C6B8;
    'dispatch: loop {
        match pc {
            0x8240C6B8 => {
    //   block [0x8240C6B8..0x8240C6F8)
	// 8240C6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C6BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C6C0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C6C4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8240C6C8: 409A001C  bne cr6, 0x8240c6e4
	if !ctx.cr[6].eq {
	pc = 0x8240C6E4; continue 'dispatch;
	}
	// 8240C6CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C6D0: 386BE238  addi r3, r11, -0x1dc8
	ctx.r[3].s64 = ctx.r[11].s64 + -7624;
	// 8240C6D4: 4BEA68AD  bl 0x822b2f80
	ctx.lr = 0x8240C6D8;
	sub_822B2F80(ctx, base);
	// 8240C6D8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C6DC: 60630025  ori r3, r3, 0x25
	ctx.r[3].u64 = ctx.r[3].u64 | 37;
	// 8240C6E0: 48000008  b 0x8240c6e8
	pc = 0x8240C6E8; continue 'dispatch;
	// 8240C6E4: 4BFFFEDD  bl 0x8240c5c0
	ctx.lr = 0x8240C6E8;
	sub_8240C5C0(ctx, base);
	// 8240C6E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240C6EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C6F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C6F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C6F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C6F8 size=136
    let mut pc: u32 = 0x8240C6F8;
    'dispatch: loop {
        match pc {
            0x8240C6F8 => {
    //   block [0x8240C6F8..0x8240C780)
	// 8240C6F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C6FC: 481289C1  bl 0x825350bc
	ctx.lr = 0x8240C700;
	sub_82535080(ctx, base);
	// 8240C700: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C704: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8240C708: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240C70C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8240C710: 409A001C  bne cr6, 0x8240c72c
	if !ctx.cr[6].eq {
	pc = 0x8240C72C; continue 'dispatch;
	}
	// 8240C714: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C718: 386BE2C8  addi r3, r11, -0x1d38
	ctx.r[3].s64 = ctx.r[11].s64 + -7480;
	// 8240C71C: 4BEA6865  bl 0x822b2f80
	ctx.lr = 0x8240C720;
	sub_822B2F80(ctx, base);
	// 8240C720: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C724: 60630023  ori r3, r3, 0x23
	ctx.r[3].u64 = ctx.r[3].u64 | 35;
	// 8240C728: 48000050  b 0x8240c778
	pc = 0x8240C778; continue 'dispatch;
	// 8240C72C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8240C730: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8240C734: 4BFFFD4D  bl 0x8240c480
	ctx.lr = 0x8240C738;
	sub_8240C480(ctx, base);
	// 8240C738: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240C73C: 41820020  beq 0x8240c75c
	if ctx.cr[0].eq {
	pc = 0x8240C75C; continue 'dispatch;
	}
	// 8240C740: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C744: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C748: 386BE280  addi r3, r11, -0x1d80
	ctx.r[3].s64 = ctx.r[11].s64 + -7552;
	// 8240C74C: 4BEA6835  bl 0x822b2f80
	ctx.lr = 0x8240C750;
	sub_822B2F80(ctx, base);
	// 8240C750: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 8240C754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240C758: 4800001C  b 0x8240c774
	pc = 0x8240C774; continue 'dispatch;
	// 8240C75C: 3C7D0008  addis r3, r29, 8
	ctx.r[3].s64 = ctx.r[29].s64 + 524288;
	// 8240C760: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8240C764: 3863030C  addi r3, r3, 0x30c
	ctx.r[3].s64 = ctx.r[3].s64 + 780;
	// 8240C768: 4BFFA2C9  bl 0x82406a30
	ctx.lr = 0x8240C76C;
	sub_82406A30(ctx, base);
	// 8240C76C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8240C770: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240C774: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240C778: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240C77C: 48128990  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C780 size=88
    let mut pc: u32 = 0x8240C780;
    'dispatch: loop {
        match pc {
            0x8240C780 => {
    //   block [0x8240C780..0x8240C7D8)
	// 8240C780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C784: 48128935  bl 0x825350b8
	ctx.lr = 0x8240C788;
	sub_82535080(ctx, base);
	// 8240C788: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C78C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240C790: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8240C794: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8240C798: 4BFFFDA1  bl 0x8240c538
	ctx.lr = 0x8240C79C;
	sub_8240C538(ctx, base);
	// 8240C79C: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8240C7A0: 4182001C  beq 0x8240c7bc
	if ctx.cr[0].eq {
	pc = 0x8240C7BC; continue 'dispatch;
	}
	// 8240C7A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240C7A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8240C7AC: 386BE310  addi r3, r11, -0x1cf0
	ctx.r[3].s64 = ctx.r[11].s64 + -7408;
	// 8240C7B0: 4BEA67D1  bl 0x822b2f80
	ctx.lr = 0x8240C7B4;
	sub_822B2F80(ctx, base);
	// 8240C7B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240C7B8: 48000018  b 0x8240c7d0
	pc = 0x8240C7D0; continue 'dispatch;
	// 8240C7BC: 1D7D0802  mulli r11, r29, 0x802
	ctx.r[11].s64 = ctx.r[29].s64 * 2050;
	// 8240C7C0: 7D6BE214  add r11, r11, r28
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[28].u64;
	// 8240C7C4: 396B0044  addi r11, r11, 0x44
	ctx.r[11].s64 = ctx.r[11].s64 + 68;
	// 8240C7C8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240C7CC: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8240C7D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240C7D4: 48128934  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C7D8 size=64
    let mut pc: u32 = 0x8240C7D8;
    'dispatch: loop {
        match pc {
            0x8240C7D8 => {
    //   block [0x8240C7D8..0x8240C818)
	// 8240C7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C7DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C7E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C7E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C7E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240C7EC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8240C7F0: 48003411  bl 0x8240fc00
	ctx.lr = 0x8240C7F4;
	sub_8240FC00(ctx, base);
	// 8240C7F4: 3C7F0002  addis r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 131072;
	// 8240C7F8: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 8240C7FC: 4BFFE6D5  bl 0x8240aed0
	ctx.lr = 0x8240C800;
	sub_8240AED0(ctx, base);
	// 8240C800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240C804: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240C808: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C80C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C810: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C818 size=60
    let mut pc: u32 = 0x8240C818;
    'dispatch: loop {
        match pc {
            0x8240C818 => {
    //   block [0x8240C818..0x8240C854)
	// 8240C818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C820: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C824: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C828: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240C82C: 3C7F0002  addis r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 131072;
	// 8240C830: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 8240C834: 4BFFE57D  bl 0x8240adb0
	ctx.lr = 0x8240C838;
	sub_8240ADB0(ctx, base);
	// 8240C838: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8240C83C: 4800321D  bl 0x8240fa58
	ctx.lr = 0x8240C840;
	sub_8240FA58(ctx, base);
	// 8240C840: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240C844: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C848: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C84C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C850: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C858 size=88
    let mut pc: u32 = 0x8240C858;
    'dispatch: loop {
        match pc {
            0x8240C858 => {
    //   block [0x8240C858..0x8240C8B0)
	// 8240C858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C85C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C860: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C864: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C868: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8240C86C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240C870: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8240C874: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8240C878: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 8240C87C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8240C880: 48003229  bl 0x8240faa8
	ctx.lr = 0x8240C884;
	sub_8240FAA8(ctx, base);
	// 8240C884: 3C7F0002  addis r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 131072;
	// 8240C888: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 8240C88C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8240C890: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 8240C894: 4BFFE565  bl 0x8240adf8
	ctx.lr = 0x8240C898;
	sub_8240ADF8(ctx, base);
	// 8240C898: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8240C89C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240C8A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C8A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C8A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C8AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C8B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240C8B0 size=60
    let mut pc: u32 = 0x8240C8B0;
    'dispatch: loop {
        match pc {
            0x8240C8B0 => {
    //   block [0x8240C8B0..0x8240C8EC)
	// 8240C8B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C8B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240C8B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240C8BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C8C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240C8C4: 3C7F0002  addis r3, r31, 2
	ctx.r[3].s64 = ctx.r[31].s64 + 131072;
	// 8240C8C8: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 8240C8CC: 4BEA66B5  bl 0x822b2f80
	ctx.lr = 0x8240C8D0;
	sub_822B2F80(ctx, base);
	// 8240C8D0: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8240C8D4: 4BEA66AD  bl 0x822b2f80
	ctx.lr = 0x8240C8D8;
	sub_822B2F80(ctx, base);
	// 8240C8D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240C8DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240C8E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240C8E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240C8E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240C8F0 size=12
    let mut pc: u32 = 0x8240C8F0;
    'dispatch: loop {
        match pc {
            0x8240C8F0 => {
    //   block [0x8240C8F0..0x8240C8FC)
	// 8240C8F0: 3C630002  addis r3, r3, 2
	ctx.r[3].s64 = ctx.r[3].s64 + 131072;
	// 8240C8F4: 3863000C  addi r3, r3, 0xc
	ctx.r[3].s64 = ctx.r[3].s64 + 12;
	// 8240C8F8: 4BFFE578  b 0x8240ae70
	sub_8240AE70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240C900 size=228
    let mut pc: u32 = 0x8240C900;
    'dispatch: loop {
        match pc {
            0x8240C900 => {
    //   block [0x8240C900..0x8240C9E4)
	// 8240C900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240C904: 481287B5  bl 0x825350b8
	ctx.lr = 0x8240C908;
	sub_82535080(ctx, base);
	// 8240C908: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 8240C90C: 481296DD  bl 0x82535fe8
	ctx.lr = 0x8240C910;
	sub_82535FB0(ctx, base);
	// 8240C910: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240C914: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8240C918: FFE01090  fmr f31, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[2].f64;
	// 8240C91C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8240C920: FFC01890  fmr f30, f3
	ctx.f[30].f64 = ctx.f[3].f64;
	// 8240C924: FFA02090  fmr f29, f4
	ctx.f[29].f64 = ctx.f[4].f64;
	// 8240C928: 2B1D0009  cmplwi cr6, r29, 9
	ctx.cr[6].compare_u32(ctx.r[29].u32, 9 as u32, &mut ctx.xer);
	// 8240C92C: FF802890  fmr f28, f5
	ctx.f[28].f64 = ctx.f[5].f64;
	// 8240C930: 4199009C  bgt cr6, 0x8240c9cc
	if ctx.cr[6].gt {
	pc = 0x8240C9CC; continue 'dispatch;
	}
	// 8240C934: 397D0003  addi r11, r29, 3
	ctx.r[11].s64 = ctx.r[29].s64 + 3;
	// 8240C938: 1F8B0018  mulli r28, r11, 0x18
	ctx.r[28].s64 = ctx.r[11].s64 * 24;
	// 8240C93C: 480034E5  bl 0x8240fe20
	ctx.lr = 0x8240C940;
	sub_8240FE20(ctx, base);
	// 8240C940: 7C3CF52E  stfsx f1, r28, r30
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32), tmp.u32) };
	// 8240C944: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240C948: 1D7D0018  mulli r11, r29, 0x18
	ctx.r[11].s64 = ctx.r[29].s64 * 24;
	// 8240C94C: 7FEBF214  add r31, r11, r30
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8240C950: 480034D1  bl 0x8240fe20
	ctx.lr = 0x8240C954;
	sub_8240FE20(ctx, base);
	// 8240C954: D03F004C  stfs f1, 0x4c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 8240C958: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240C95C: 480034C5  bl 0x8240fe20
	ctx.lr = 0x8240C960;
	sub_8240FE20(ctx, base);
	// 8240C960: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240C964: D03F0050  stfs f1, 0x50(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8240C968: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240C96C: 480034B5  bl 0x8240fe20
	ctx.lr = 0x8240C970;
	sub_8240FE20(ctx, base);
	// 8240C970: D03F0054  stfs f1, 0x54(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8240C974: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240C978: 480034A9  bl 0x8240fe20
	ctx.lr = 0x8240C97C;
	sub_8240FE20(ctx, base);
	// 8240C97C: D03F0058  stfs f1, 0x58(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8240C980: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 8240C984: 4800349D  bl 0x8240fe20
	ctx.lr = 0x8240C988;
	sub_8240FE20(ctx, base);
	// 8240C988: 397D000D  addi r11, r29, 0xd
	ctx.r[11].s64 = ctx.r[29].s64 + 13;
	// 8240C98C: 815F0148  lwz r10, 0x148(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(328 as u32) ) } as u64;
	// 8240C990: D03F005C  stfs f1, 0x5c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8240C994: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 8240C998: 7D6BF02E  lwzx r11, r11, r30
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8240C99C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8240C9A0: 409A000C  bne cr6, 0x8240c9ac
	if !ctx.cr[6].eq {
	pc = 0x8240C9AC; continue 'dispatch;
	}
	// 8240C9A4: 7C1CF42E  lfsx f0, r28, r30
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[30].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240C9A8: D01F0058  stfs f0, 0x58(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8240C9AC: 817F013C  lwz r11, 0x13c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 8240C9B0: 815F014C  lwz r10, 0x14c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(332 as u32) ) } as u64;
	// 8240C9B4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 8240C9B8: 409A000C  bne cr6, 0x8240c9c4
	if !ctx.cr[6].eq {
	pc = 0x8240C9C4; continue 'dispatch;
	}
	// 8240C9BC: C01F004C  lfs f0, 0x4c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240C9C0: D01F005C  stfs f0, 0x5c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8240C9C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240C9C8: 4800000C  b 0x8240c9d4
	pc = 0x8240C9D4; continue 'dispatch;
	// 8240C9CC: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240C9D0: 6063002B  ori r3, r3, 0x2b
	ctx.r[3].u64 = ctx.r[3].u64 | 43;
	// 8240C9D4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8240C9D8: 3981FFD8  addi r12, r1, -0x28
	ctx.r[12].s64 = ctx.r[1].s64 + -40;
	// 8240C9DC: 48129659  bl 0x82536034
	ctx.lr = 0x8240C9E0;
	sub_82535FFC(ctx, base);
	// 8240C9E0: 48128728  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240C9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240C9E8 size=60
    let mut pc: u32 = 0x8240C9E8;
    'dispatch: loop {
        match pc {
            0x8240C9E8 => {
    //   block [0x8240C9E8..0x8240CA24)
	// 8240C9E8: 2B040009  cmplwi cr6, r4, 9
	ctx.cr[6].compare_u32(ctx.r[4].u32, 9 as u32, &mut ctx.xer);
	// 8240C9EC: 41990038  bgt cr6, 0x8240ca24
	if ctx.cr[6].gt {
		sub_8240CA24(ctx, base);
		return;
	}
	// 8240C9F0: 3944000D  addi r10, r4, 0xd
	ctx.r[10].s64 = ctx.r[4].s64 + 13;
	// 8240C9F4: 1D640018  mulli r11, r4, 0x18
	ctx.r[11].s64 = ctx.r[4].s64 * 24;
	// 8240C9F8: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 8240C9FC: 7CAA192E  stwx r5, r10, r3
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[5].u32) };
	// 8240CA00: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240CA04: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8240CA08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240CA0C: 90CB013C  stw r6, 0x13c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(316 as u32), ctx.r[6].u32 ) };
	// 8240CA10: 90EB0140  stw r7, 0x140(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(320 as u32), ctx.r[7].u32 ) };
	// 8240CA14: 914B0144  stw r10, 0x144(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(324 as u32), ctx.r[10].u32 ) };
	// 8240CA18: 910B0148  stw r8, 0x148(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(328 as u32), ctx.r[8].u32 ) };
	// 8240CA1C: 912B014C  stw r9, 0x14c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(332 as u32), ctx.r[9].u32 ) };
	// 8240CA20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CA24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240CA24 size=12
    let mut pc: u32 = 0x8240CA24;
    'dispatch: loop {
        match pc {
            0x8240CA24 => {
    //   block [0x8240CA24..0x8240CA30)
	// 8240CA24: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240CA28: 6063002B  ori r3, r3, 0x2b
	ctx.r[3].u64 = ctx.r[3].u64 | 43;
	// 8240CA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240CA30 size=232
    let mut pc: u32 = 0x8240CA30;
    'dispatch: loop {
        match pc {
            0x8240CA30 => {
    //   block [0x8240CA30..0x8240CB18)
	// 8240CA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240CA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240CA38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240CA3C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8240CA40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240CA44: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CA48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240CA4C: C3EBBFFC  lfs f31, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8240CA50: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CA54: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CA58: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CA5C: C19F0014  lfs f12, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240CA60: D1BF0040  stfs f13, 0x40(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 8240CA64: C14B210C  lfs f10, 0x210c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8460 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8240CA68: ED2D502A  fadds f9, f13, f10
	ctx.f[9].f64 = ((ctx.f[13].f64 + ctx.f[10].f64) as f32) as f64;
	// 8240CA6C: C17F0010  lfs f11, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240CA70: ED4A0028  fsubs f10, f10, f0
	ctx.f[10].f64 = (((ctx.f[10].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240CA74: D15F0038  stfs f10, 0x38(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), tmp.u32 ) };
	// 8240CA78: ED4C6828  fsubs f10, f12, f13
	ctx.f[10].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 8240CA7C: D15F0020  stfs f10, 0x20(r31)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8240CA80: ED8B6028  fsubs f12, f11, f12
	ctx.f[12].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240CA84: D19F0028  stfs f12, 0x28(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8240CA88: ED805828  fsubs f12, f0, f11
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[11].f64) as f32) as f64);
	// 8240CA8C: D19F0030  stfs f12, 0x30(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8240CA90: EDA90028  fsubs f13, f9, f0
	ctx.f[13].f64 = (((ctx.f[9].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240CA94: D1BF0018  stfs f13, 0x18(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8240CA98: EC2D07FA  fmadds f1, f13, f31, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[31].f64 + ctx.f[0].f64) as f32) as f64);
	// 8240CA9C: 48003385  bl 0x8240fe20
	ctx.lr = 0x8240CAA0;
	sub_8240FE20(ctx, base);
	// 8240CAA0: C01F0020  lfs f0, 0x20(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CAA4: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CAA8: D03F001C  stfs f1, 0x1c(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8240CAAC: EC206FFA  fmadds f1, f0, f31, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64 + ctx.f[13].f64) as f32) as f64);
	// 8240CAB0: 48003371  bl 0x8240fe20
	ctx.lr = 0x8240CAB4;
	sub_8240FE20(ctx, base);
	// 8240CAB4: C01F0028  lfs f0, 0x28(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CAB8: C1BF0014  lfs f13, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CABC: D03F0024  stfs f1, 0x24(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8240CAC0: EC206FFA  fmadds f1, f0, f31, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64 + ctx.f[13].f64) as f32) as f64);
	// 8240CAC4: 4800335D  bl 0x8240fe20
	ctx.lr = 0x8240CAC8;
	sub_8240FE20(ctx, base);
	// 8240CAC8: C01F0030  lfs f0, 0x30(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CACC: C1BF0010  lfs f13, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CAD0: D03F002C  stfs f1, 0x2c(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 8240CAD4: EC206FFA  fmadds f1, f0, f31, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64 + ctx.f[13].f64) as f32) as f64);
	// 8240CAD8: 48003349  bl 0x8240fe20
	ctx.lr = 0x8240CADC;
	sub_8240FE20(ctx, base);
	// 8240CADC: C01F0038  lfs f0, 0x38(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CAE0: C1BF0000  lfs f13, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CAE4: D03F0034  stfs f1, 0x34(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 8240CAE8: EC206FFA  fmadds f1, f0, f31, f13
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64 + ctx.f[13].f64) as f32) as f64);
	// 8240CAEC: 48003335  bl 0x8240fe20
	ctx.lr = 0x8240CAF0;
	sub_8240FE20(ctx, base);
	// 8240CAF0: C01F0040  lfs f0, 0x40(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CAF4: EC0007F2  fmuls f0, f0, f31
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 8240CAF8: D03F003C  stfs f1, 0x3c(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8240CAFC: D01F0044  stfs f0, 0x44(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 8240CB00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240CB04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240CB08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240CB0C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240CB10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240CB14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CB18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240CB18 size=116
    let mut pc: u32 = 0x8240CB18;
    'dispatch: loop {
        match pc {
            0x8240CB18 => {
    //   block [0x8240CB18..0x8240CB8C)
	// 8240CB18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240CB1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240CB20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240CB24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240CB28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240CB2C: 480032F5  bl 0x8240fe20
	ctx.lr = 0x8240CB30;
	sub_8240FE20(ctx, base);
	// 8240CB30: C01F0004  lfs f0, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CB34: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 8240CB38: 40980024  bge cr6, 0x8240cb5c
	if !ctx.cr[6].lt {
	pc = 0x8240CB5C; continue 'dispatch;
	}
	// 8240CB3C: C01F0014  lfs f0, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CB40: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 8240CB44: 4099000C  ble cr6, 0x8240cb50
	if !ctx.cr[6].gt {
	pc = 0x8240CB50; continue 'dispatch;
	}
	// 8240CB48: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8240CB4C: 4800002C  b 0x8240cb78
	pc = 0x8240CB78; continue 'dispatch;
	// 8240CB50: C01F0000  lfs f0, 0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CB54: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 8240CB58: 4199000C  bgt cr6, 0x8240cb64
	if ctx.cr[6].gt {
	pc = 0x8240CB64; continue 'dispatch;
	}
	// 8240CB5C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240CB60: 48000018  b 0x8240cb78
	pc = 0x8240CB78; continue 'dispatch;
	// 8240CB64: C01F0010  lfs f0, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CB68: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 8240CB6C: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 8240CB70: 41980008  blt cr6, 0x8240cb78
	if ctx.cr[6].lt {
	pc = 0x8240CB78; continue 'dispatch;
	}
	// 8240CB74: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8240CB78: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240CB7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240CB80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240CB84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240CB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240CB90 size=212
    let mut pc: u32 = 0x8240CB90;
    'dispatch: loop {
        match pc {
            0x8240CB90 => {
    //   block [0x8240CB90..0x8240CC64)
	// 8240CB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240CB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240CB98: DBC1FFE8  stfd f30, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[30].u64 ) };
	// 8240CB9C: DBE1FFF0  stfd f31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 8240CBA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240CBA4: FC000890  fmr f0, f1
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8240CBA8: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 8240CBAC: 41980050  blt cr6, 0x8240cbfc
	if ctx.cr[6].lt {
	pc = 0x8240CBFC; continue 'dispatch;
	}
	// 8240CBB0: 419A003C  beq cr6, 0x8240cbec
	if ctx.cr[6].eq {
	pc = 0x8240CBEC; continue 'dispatch;
	}
	// 8240CBB4: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 8240CBB8: 41980024  blt cr6, 0x8240cbdc
	if ctx.cr[6].lt {
	pc = 0x8240CBDC; continue 'dispatch;
	}
	// 8240CBBC: 419A0010  beq cr6, 0x8240cbcc
	if ctx.cr[6].eq {
	pc = 0x8240CBCC; continue 'dispatch;
	}
	// 8240CBC0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CBC4: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240CBC8: 48000084  b 0x8240cc4c
	pc = 0x8240CC4C; continue 'dispatch;
	// 8240CBCC: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CBD0: 2F050004  cmpwi cr6, r5, 4
	ctx.cr[6].compare_i32(ctx.r[5].s32, 4, &mut ctx.xer);
	// 8240CBD4: C1830010  lfs f12, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240CBD8: 48000030  b 0x8240cc08
	pc = 0x8240CC08; continue 'dispatch;
	// 8240CBDC: C1A30010  lfs f13, 0x10(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CBE0: 2F050005  cmpwi cr6, r5, 5
	ctx.cr[6].compare_i32(ctx.r[5].s32, 5, &mut ctx.xer);
	// 8240CBE4: C1830014  lfs f12, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240CBE8: 48000020  b 0x8240cc08
	pc = 0x8240CC08; continue 'dispatch;
	// 8240CBEC: C1A30014  lfs f13, 0x14(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CBF0: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8240CBF4: C1830004  lfs f12, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240CBF8: 48000010  b 0x8240cc08
	pc = 0x8240CC08; continue 'dispatch;
	// 8240CBFC: C1A30004  lfs f13, 4(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CC00: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8240CC04: C1830000  lfs f12, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240CC08: EC2D6028  fsubs f1, f13, f12
	ctx.f[1].f64 = (((ctx.f[13].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240CC0C: 409A000C  bne cr6, 0x8240cc18
	if !ctx.cr[6].eq {
	pc = 0x8240CC18; continue 'dispatch;
	}
	// 8240CC10: EFE06028  fsubs f31, f0, f12
	ctx.f[31].f64 = (((ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240CC14: 48000008  b 0x8240cc1c
	pc = 0x8240CC1C; continue 'dispatch;
	// 8240CC18: EFED0028  fsubs f31, f13, f0
	ctx.f[31].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240CC1C: 48003205  bl 0x8240fe20
	ctx.lr = 0x8240CC20;
	sub_8240FE20(ctx, base);
	// 8240CC20: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8240CC24: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240CC28: 480031F9  bl 0x8240fe20
	ctx.lr = 0x8240CC2C;
	sub_8240FE20(ctx, base);
	// 8240CC2C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CC30: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CC34: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 8240CC38: 409A000C  bne cr6, 0x8240cc44
	if !ctx.cr[6].eq {
	pc = 0x8240CC44; continue 'dispatch;
	}
	// 8240CC3C: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 8240CC40: 4800000C  b 0x8240cc4c
	pc = 0x8240CC4C; continue 'dispatch;
	// 8240CC44: EC1E0828  fsubs f0, f30, f1
	ctx.f[0].f64 = (((ctx.f[30].f64 - ctx.f[1].f64) as f32) as f64);
	// 8240CC48: EC20F024  fdivs f1, f0, f30
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[30].f64) as f32) as f64;
	// 8240CC4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240CC50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240CC54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240CC58: CBC1FFE8  lfd f30, -0x18(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240CC5C: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240CC60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CC68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240CC68 size=320
    let mut pc: u32 = 0x8240CC68;
    'dispatch: loop {
        match pc {
            0x8240CC68 => {
    //   block [0x8240CC68..0x8240CDA8)
	// 8240CC68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240CC6C: 48128451  bl 0x825350bc
	ctx.lr = 0x8240CC70;
	sub_82535080(ctx, base);
	// 8240CC70: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 8240CC74: 48129371  bl 0x82535fe4
	ctx.lr = 0x8240CC78;
	sub_82535FB0(ctx, base);
	// 8240CC78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240CC7C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CC80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240CC84: C3CB2F80  lfs f30, 0x2f80(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12160 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240CC88: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240CC8C: 48003195  bl 0x8240fe20
	ctx.lr = 0x8240CC90;
	sub_8240FE20(ctx, base);
	// 8240CC90: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240CC94: D03F0000  stfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240CC98: C3AB76F4  lfs f29, 0x76f4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30452 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8240CC9C: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240CCA0: 48003181  bl 0x8240fe20
	ctx.lr = 0x8240CCA4;
	sub_8240FE20(ctx, base);
	// 8240CCA4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CCA8: D03F0004  stfs f1, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240CCAC: C3EB1FF8  lfs f31, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8240CCB0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240CCB4: 4800316D  bl 0x8240fe20
	ctx.lr = 0x8240CCB8;
	sub_8240FE20(ctx, base);
	// 8240CCB8: D03F0008  stfs f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240CCBC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240CCC0: 48003161  bl 0x8240fe20
	ctx.lr = 0x8240CCC4;
	sub_8240FE20(ctx, base);
	// 8240CCC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240CCC8: D03F000C  stfs f1, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240CCCC: C38BC10C  lfs f28, -0x3ef4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16116 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 8240CCD0: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 8240CCD4: 4800314D  bl 0x8240fe20
	ctx.lr = 0x8240CCD8;
	sub_8240FE20(ctx, base);
	// 8240CCD8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CCDC: D03F0010  stfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240CCE0: C36B236C  lfs f27, 0x236c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9068 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 8240CCE4: FC20D890  fmr f1, f27
	ctx.f[1].f64 = ctx.f[27].f64;
	// 8240CCE8: 48003139  bl 0x8240fe20
	ctx.lr = 0x8240CCEC;
	sub_8240FE20(ctx, base);
	// 8240CCEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240CCF0: D03F0014  stfs f1, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240CCF4: 4BFFFD3D  bl 0x8240ca30
	ctx.lr = 0x8240CCF8;
	sub_8240CA30(ctx, base);
	// 8240CCF8: 3BDF004C  addi r30, r31, 0x4c
	ctx.r[30].s64 = ctx.r[31].s64 + 76;
	// 8240CCFC: 3BA0000A  li r29, 0xa
	ctx.r[29].s64 = 10;
	// 8240CD00: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240CD04: 4800311D  bl 0x8240fe20
	ctx.lr = 0x8240CD08;
	sub_8240FE20(ctx, base);
	// 8240CD08: D03EFFFC  stfs f1, -4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8240CD0C: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240CD10: 48003111  bl 0x8240fe20
	ctx.lr = 0x8240CD14;
	sub_8240FE20(ctx, base);
	// 8240CD14: D03E0000  stfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240CD18: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240CD1C: 48003105  bl 0x8240fe20
	ctx.lr = 0x8240CD20;
	sub_8240FE20(ctx, base);
	// 8240CD20: D03E0004  stfs f1, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240CD24: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240CD28: 480030F9  bl 0x8240fe20
	ctx.lr = 0x8240CD2C;
	sub_8240FE20(ctx, base);
	// 8240CD2C: D03E0008  stfs f1, 8(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240CD30: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 8240CD34: 480030ED  bl 0x8240fe20
	ctx.lr = 0x8240CD38;
	sub_8240FE20(ctx, base);
	// 8240CD38: D03E000C  stfs f1, 0xc(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240CD3C: FC20D890  fmr f1, f27
	ctx.f[1].f64 = ctx.f[27].f64;
	// 8240CD40: 480030E1  bl 0x8240fe20
	ctx.lr = 0x8240CD44;
	sub_8240FE20(ctx, base);
	// 8240CD44: D03E0010  stfs f1, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240CD48: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8240CD4C: 3BDE0018  addi r30, r30, 0x18
	ctx.r[30].s64 = ctx.r[30].s64 + 24;
	// 8240CD50: 4082FFB0  bne 0x8240cd00
	if !ctx.cr[0].eq {
	pc = 0x8240CD00; continue 'dispatch;
	}
	// 8240CD54: 397F013C  addi r11, r31, 0x13c
	ctx.r[11].s64 = ctx.r[31].s64 + 316;
	// 8240CD58: 3940000A  li r10, 0xa
	ctx.r[10].s64 = 10;
	// 8240CD5C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8240CD60: 3900FFE2  li r8, -0x1e
	ctx.r[8].s64 = -30;
	// 8240CD64: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8240CD68: 38E0001E  li r7, 0x1e
	ctx.r[7].s64 = 30;
	// 8240CD6C: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 8240CD70: 38C0FF92  li r6, -0x6e
	ctx.r[6].s64 = -110;
	// 8240CD74: 38A0006E  li r5, 0x6e
	ctx.r[5].s64 = 110;
	// 8240CD78: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240CD7C: 910BFFFC  stw r8, -4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-4 as u32), ctx.r[8].u32 ) };
	// 8240CD80: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8240CD84: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 8240CD88: 90AB0010  stw r5, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 8240CD8C: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 8240CD90: 4082FFD0  bne 0x8240cd60
	if !ctx.cr[0].eq {
	pc = 0x8240CD60; continue 'dispatch;
	}
	// 8240CD94: 913F0228  stw r9, 0x228(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(552 as u32), ctx.r[9].u32 ) };
	// 8240CD98: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8240CD9C: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 8240CDA0: 48129291  bl 0x82536030
	ctx.lr = 0x8240CDA4;
	sub_82535FFC(ctx, base);
	// 8240CDA4: 48128368  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240CDA8 size=156
    let mut pc: u32 = 0x8240CDA8;
    'dispatch: loop {
        match pc {
            0x8240CDA8 => {
    //   block [0x8240CDA8..0x8240CE44)
	// 8240CDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240CDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240CDB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240CDB4: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 8240CDB8: 48129231  bl 0x82535fe8
	ctx.lr = 0x8240CDBC;
	sub_82535FB0(ctx, base);
	// 8240CDBC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240CDC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240CDC4: FFE01090  fmr f31, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[2].f64;
	// 8240CDC8: FFC01890  fmr f30, f3
	ctx.f[30].f64 = ctx.f[3].f64;
	// 8240CDCC: FFA02090  fmr f29, f4
	ctx.f[29].f64 = ctx.f[4].f64;
	// 8240CDD0: FF802890  fmr f28, f5
	ctx.f[28].f64 = ctx.f[5].f64;
	// 8240CDD4: 4800304D  bl 0x8240fe20
	ctx.lr = 0x8240CDD8;
	sub_8240FE20(ctx, base);
	// 8240CDD8: D03F0000  stfs f1, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240CDDC: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240CDE0: 48003041  bl 0x8240fe20
	ctx.lr = 0x8240CDE4;
	sub_8240FE20(ctx, base);
	// 8240CDE4: D03F0004  stfs f1, 4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240CDE8: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240CDEC: 48003035  bl 0x8240fe20
	ctx.lr = 0x8240CDF0;
	sub_8240FE20(ctx, base);
	// 8240CDF0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CDF4: D03F0008  stfs f1, 8(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240CDF8: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240CDFC: 48003025  bl 0x8240fe20
	ctx.lr = 0x8240CE00;
	sub_8240FE20(ctx, base);
	// 8240CE00: D03F000C  stfs f1, 0xc(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240CE04: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240CE08: 48003019  bl 0x8240fe20
	ctx.lr = 0x8240CE0C;
	sub_8240FE20(ctx, base);
	// 8240CE0C: D03F0010  stfs f1, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240CE10: FC20E090  fmr f1, f28
	ctx.f[1].f64 = ctx.f[28].f64;
	// 8240CE14: 4800300D  bl 0x8240fe20
	ctx.lr = 0x8240CE18;
	sub_8240FE20(ctx, base);
	// 8240CE18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240CE1C: D03F0014  stfs f1, 0x14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240CE20: 4BFFFC11  bl 0x8240ca30
	ctx.lr = 0x8240CE24;
	sub_8240CA30(ctx, base);
	// 8240CE24: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240CE28: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8240CE2C: 3981FFF0  addi r12, r1, -0x10
	ctx.r[12].s64 = ctx.r[1].s64 + -16;
	// 8240CE30: 48129205  bl 0x82536034
	ctx.lr = 0x8240CE34;
	sub_82535FFC(ctx, base);
	// 8240CE34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240CE38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240CE3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240CE40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CE48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8240CE48 size=48
    let mut pc: u32 = 0x8240CE48;
    'dispatch: loop {
        match pc {
            0x8240CE48 => {
    //   block [0x8240CE48..0x8240CE78)
	// 8240CE48: 2B040009  cmplwi cr6, r4, 9
	ctx.cr[6].compare_u32(ctx.r[4].u32, 9 as u32, &mut ctx.xer);
	// 8240CE4C: 4199002C  bgt cr6, 0x8240ce78
	if ctx.cr[6].gt {
		sub_8240CE78(ctx, base);
		return;
	}
	// 8240CE50: 39640003  addi r11, r4, 3
	ctx.r[11].s64 = ctx.r[4].s64 + 3;
	// 8240CE54: 90830228  stw r4, 0x228(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(552 as u32), ctx.r[4].u32 ) };
	// 8240CE58: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 8240CE5C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240CE60: C0AB0014  lfs f5, 0x14(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 8240CE64: C08B0010  lfs f4, 0x10(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 8240CE68: C06B0008  lfs f3, 8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8240CE6C: C04B0004  lfs f2, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 8240CE70: C02B0000  lfs f1, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240CE74: 4BFFFF34  b 0x8240cda8
	sub_8240CDA8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8240CE78 size=12
    let mut pc: u32 = 0x8240CE78;
    'dispatch: loop {
        match pc {
            0x8240CE78 => {
    //   block [0x8240CE78..0x8240CE84)
	// 8240CE78: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240CE7C: 6063002B  ori r3, r3, 0x2b
	ctx.r[3].u64 = ctx.r[3].u64 | 43;
	// 8240CE80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240CE88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240CE88 size=1460
    let mut pc: u32 = 0x8240CE88;
    'dispatch: loop {
        match pc {
            0x8240CE88 => {
    //   block [0x8240CE88..0x8240D43C)
	// 8240CE88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240CE8C: 48128231  bl 0x825350bc
	ctx.lr = 0x8240CE90;
	sub_82535080(ctx, base);
	// 8240CE90: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 8240CE94: 48129145  bl 0x82535fd8
	ctx.lr = 0x8240CE98;
	sub_82535FB0(ctx, base);
	// 8240CE98: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240CE9C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CEA0: FFC01090  fmr f30, f2
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[2].f64;
	// 8240CEA4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8240CEA8: FF001890  fmr f24, f3
	ctx.f[24].f64 = ctx.f[3].f64;
	// 8240CEAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240CEB0: C34B1FF8  lfs f26, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 8240CEB4: D35E000C  stfs f26, 0xc(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240CEB8: 817F0228  lwz r11, 0x228(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(552 as u32) ) } as u64;
	// 8240CEBC: 394B000D  addi r10, r11, 0xd
	ctx.r[10].s64 = ctx.r[11].s64 + 13;
	// 8240CEC0: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 8240CEC4: 7D4AF82E  lwzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8240CEC8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8240CECC: 409A0038  bne cr6, 0x8240cf04
	if !ctx.cr[6].eq {
	pc = 0x8240CF04; continue 'dispatch;
	}
	// 8240CED0: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 8240CED4: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240CED8: 816B013C  lwz r11, 0x13c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(316 as u32) ) } as u64;
	// 8240CEDC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240CEE0: 409A0024  bne cr6, 0x8240cf04
	if !ctx.cr[6].eq {
	pc = 0x8240CF04; continue 'dispatch;
	}
	// 8240CEE4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CEE8: D35E0010  stfs f26, 0x10(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240CEEC: D35E0014  stfs f26, 0x14(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240CEF0: D35E0008  stfs f26, 8(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240CEF4: C00B2AD4  lfs f0, 0x2ad4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(10964 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CEF8: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240CEFC: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240CF00: 4800052C  b 0x8240d42c
	pc = 0x8240D42C; continue 'dispatch;
	// 8240CF04: 48002F1D  bl 0x8240fe20
	ctx.lr = 0x8240CF08;
	sub_8240FE20(ctx, base);
	// 8240CF08: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CF0C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240CF10: FF1ED000  fcmpu cr6, f30, f26
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[26].f64);
	// 8240CF14: C3AB213C  lfs f29, 0x213c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8508 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8240CF18: 40980024  bge cr6, 0x8240cf3c
	if !ctx.cr[6].lt {
	pc = 0x8240CF3C; continue 'dispatch;
	}
	// 8240CF1C: FFC0F050  fneg f30, f30
	ctx.f[30].u64 = ctx.f[30].u64 ^ 0x8000_0000_0000_0000u64;
	// 8240CF20: FF1FE800  fcmpu cr6, f31, f29
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[29].f64);
	// 8240CF24: 40990014  ble cr6, 0x8240cf38
	if !ctx.cr[6].gt {
	pc = 0x8240CF38; continue 'dispatch;
	}
	// 8240CF28: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CF2C: C00B2C68  lfs f0, 0x2c68(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(11368 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CF30: EFE0F828  fsubs f31, f0, f31
	ctx.f[31].f64 = (((ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 8240CF34: 48000008  b 0x8240cf3c
	pc = 0x8240CF3C; continue 'dispatch;
	// 8240CF38: EFFDF828  fsubs f31, f29, f31
	ctx.f[31].f64 = (((ctx.f[29].f64 - ctx.f[31].f64) as f32) as f64);
	// 8240CF3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240CF40: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8240CF44: 4BFFFBD5  bl 0x8240cb18
	ctx.lr = 0x8240CF48;
	sub_8240CB18(ctx, base);
	// 8240CF48: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8240CF4C: FF1FE800  fcmpu cr6, f31, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[29].f64);
	// 8240CF50: 57AB1838  slwi r11, r29, 3
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8240CF54: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240CF58: C00B001C  lfs f0, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CF5C: 40990018  ble cr6, 0x8240cf74
	if !ctx.cr[6].gt {
	pc = 0x8240CF74; continue 'dispatch;
	}
	// 8240CF60: FF00E800  fcmpu cr6, f0, f29
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[29].f64);
	// 8240CF64: 40980010  bge cr6, 0x8240cf74
	if !ctx.cr[6].lt {
	pc = 0x8240CF74; continue 'dispatch;
	}
	// 8240CF68: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CF6C: C1AB210C  lfs f13, 0x210c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8460 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240CF70: EC00682A  fadds f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64;
	// 8240CF74: EC20F828  fsubs f1, f0, f31
	ctx.f[1].f64 = (((ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 8240CF78: 48002F01  bl 0x8240fe78
	ctx.lr = 0x8240CF7C;
	sub_8240FE78(ctx, base);
	// 8240CF7C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240CF80: C3AB1850  lfs f29, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8240CF84: EC1DF028  fsubs f0, f29, f30
	ctx.f[0].f64 = (((ctx.f[29].f64 - ctx.f[30].f64) as f32) as f64);
	// 8240CF88: EC20F87A  fmadds f1, f0, f1, f31
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[31].f64) as f32) as f64);
	// 8240CF8C: 48002E95  bl 0x8240fe20
	ctx.lr = 0x8240CF90;
	sub_8240FE20(ctx, base);
	// 8240CF90: EDBEE82A  fadds f13, f30, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = ((ctx.f[30].f64 + ctx.f[29].f64) as f32) as f64;
	// 8240CF94: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240CF98: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8240CF9C: C00BBFFC  lfs f0, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240CFA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240CFA4: 396BE358  addi r11, r11, -0x1ca8
	ctx.r[11].s64 = ctx.r[11].s64 + -7336;
	// 8240CFA8: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 8240CFAC: C32B0000  lfs f25, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 8240CFB0: FF00E800  fcmpu cr6, f0, f29
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[29].f64);
	// 8240CFB4: 4198000C  blt cr6, 0x8240cfc0
	if ctx.cr[6].lt {
	pc = 0x8240CFC0; continue 'dispatch;
	}
	// 8240CFB8: FF80E890  fmr f28, f29
	ctx.f[28].f64 = ctx.f[29].f64;
	// 8240CFBC: 48000028  b 0x8240cfe4
	pc = 0x8240CFE4; continue 'dispatch;
	// 8240CFC0: FF00D000  fcmpu cr6, f0, f26
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[26].f64);
	// 8240CFC4: 4199000C  bgt cr6, 0x8240cfd0
	if ctx.cr[6].gt {
	pc = 0x8240CFD0; continue 'dispatch;
	}
	// 8240CFC8: FF80D090  fmr f28, f26
	ctx.f[28].f64 = ctx.f[26].f64;
	// 8240CFCC: 48000028  b 0x8240cff4
	pc = 0x8240CFF4; continue 'dispatch;
	// 8240CFD0: EC200672  fmuls f1, f0, f25
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240CFD4: 48002C6D  bl 0x8240fc40
	ctx.lr = 0x8240CFD8;
	sub_8240FC40(ctx, base);
	// 8240CFD8: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 8240CFDC: FF1CE800  fcmpu cr6, f28, f29
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[29].f64);
	// 8240CFE0: 4198000C  blt cr6, 0x8240cfec
	if ctx.cr[6].lt {
	pc = 0x8240CFEC; continue 'dispatch;
	}
	// 8240CFE4: FF60D090  fmr f27, f26
	ctx.f[27].f64 = ctx.f[26].f64;
	// 8240CFE8: 48000020  b 0x8240d008
	pc = 0x8240D008; continue 'dispatch;
	// 8240CFEC: FF1CD000  fcmpu cr6, f28, f26
	ctx.cr[6].compare_f64(ctx.f[28].f64, ctx.f[26].f64);
	// 8240CFF0: 4199000C  bgt cr6, 0x8240cffc
	if ctx.cr[6].gt {
	pc = 0x8240CFFC; continue 'dispatch;
	}
	// 8240CFF4: FF60E890  fmr f27, f29
	ctx.f[27].f64 = ctx.f[29].f64;
	// 8240CFF8: 48000010  b 0x8240d008
	pc = 0x8240D008; continue 'dispatch;
	// 8240CFFC: EC3CEF3C  fnmsubs f1, f28, f28, f29
	ctx.f[1].f64 = -(((ctx.f[28].f64 * ctx.f[28].f64 - ctx.f[29].f64) as f32) as f64);
	// 8240D000: 48002EB9  bl 0x8240feb8
	ctx.lr = 0x8240D004;
	sub_8240FEB8(ctx, base);
	// 8240D004: FF600890  fmr f27, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[27].f64 = ctx.f[1].f64;
	// 8240D008: 2B1D0001  cmplwi cr6, r29, 1
	ctx.cr[6].compare_u32(ctx.r[29].u32, 1 as u32, &mut ctx.xer);
	// 8240D00C: 419801EC  blt cr6, 0x8240d1f8
	if ctx.cr[6].lt {
	pc = 0x8240D1F8; continue 'dispatch;
	}
	// 8240D010: 419A0148  beq cr6, 0x8240d158
	if ctx.cr[6].eq {
	pc = 0x8240D158; continue 'dispatch;
	}
	// 8240D014: 2B1D0003  cmplwi cr6, r29, 3
	ctx.cr[6].compare_u32(ctx.r[29].u32, 3 as u32, &mut ctx.xer);
	// 8240D018: 419800A0  blt cr6, 0x8240d0b8
	if ctx.cr[6].lt {
	pc = 0x8240D0B8; continue 'dispatch;
	}
	// 8240D01C: 409A0278  bne cr6, 0x8240d294
	if !ctx.cr[6].eq {
	pc = 0x8240D294; continue 'dispatch;
	}
	// 8240D020: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8240D024: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D028: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8240D02C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D030: 4BFFFB61  bl 0x8240cb90
	ctx.lr = 0x8240D034;
	sub_8240CB90(ctx, base);
	// 8240D034: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D038: 4198000C  blt cr6, 0x8240d044
	if ctx.cr[6].lt {
	pc = 0x8240D044; continue 'dispatch;
	}
	// 8240D03C: FFE0E890  fmr f31, f29
	ctx.f[31].f64 = ctx.f[29].f64;
	// 8240D040: 48000020  b 0x8240d060
	pc = 0x8240D060; continue 'dispatch;
	// 8240D044: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D048: 4199000C  bgt cr6, 0x8240d054
	if ctx.cr[6].gt {
	pc = 0x8240D054; continue 'dispatch;
	}
	// 8240D04C: FFE0D090  fmr f31, f26
	ctx.f[31].f64 = ctx.f[26].f64;
	// 8240D050: 48000010  b 0x8240d060
	pc = 0x8240D060; continue 'dispatch;
	// 8240D054: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D058: 48002BE9  bl 0x8240fc40
	ctx.lr = 0x8240D05C;
	sub_8240FC40(ctx, base);
	// 8240D05C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240D060: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8240D064: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D068: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 8240D06C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D070: 4BFFFB21  bl 0x8240cb90
	ctx.lr = 0x8240D074;
	sub_8240CB90(ctx, base);
	// 8240D074: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D078: 4198000C  blt cr6, 0x8240d084
	if ctx.cr[6].lt {
	pc = 0x8240D084; continue 'dispatch;
	}
	// 8240D07C: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240D080: 4800001C  b 0x8240d09c
	pc = 0x8240D09C; continue 'dispatch;
	// 8240D084: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D088: 4199000C  bgt cr6, 0x8240d094
	if ctx.cr[6].gt {
	pc = 0x8240D094; continue 'dispatch;
	}
	// 8240D08C: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 8240D090: 4800000C  b 0x8240d09c
	pc = 0x8240D09C; continue 'dispatch;
	// 8240D094: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D098: 48002BA9  bl 0x8240fc40
	ctx.lr = 0x8240D09C;
	sub_8240FC40(ctx, base);
	// 8240D09C: EC1F0732  fmuls f0, f31, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D0A0: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D0A4: EC010732  fmuls f0, f1, f28
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D0A8: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D0AC: EC1F06F2  fmuls f0, f31, f27
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[27].f64) as f32) as f64);
	// 8240D0B0: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D0B4: 480001D8  b 0x8240d28c
	pc = 0x8240D28C; continue 'dispatch;
	// 8240D0B8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8240D0BC: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D0C0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8240D0C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D0C8: 4BFFFAC9  bl 0x8240cb90
	ctx.lr = 0x8240D0CC;
	sub_8240CB90(ctx, base);
	// 8240D0CC: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D0D0: 4198000C  blt cr6, 0x8240d0dc
	if ctx.cr[6].lt {
	pc = 0x8240D0DC; continue 'dispatch;
	}
	// 8240D0D4: FFE0E890  fmr f31, f29
	ctx.f[31].f64 = ctx.f[29].f64;
	// 8240D0D8: 48000020  b 0x8240d0f8
	pc = 0x8240D0F8; continue 'dispatch;
	// 8240D0DC: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D0E0: 4199000C  bgt cr6, 0x8240d0ec
	if ctx.cr[6].gt {
	pc = 0x8240D0EC; continue 'dispatch;
	}
	// 8240D0E4: FFE0D090  fmr f31, f26
	ctx.f[31].f64 = ctx.f[26].f64;
	// 8240D0E8: 48000010  b 0x8240d0f8
	pc = 0x8240D0F8; continue 'dispatch;
	// 8240D0EC: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D0F0: 48002B51  bl 0x8240fc40
	ctx.lr = 0x8240D0F4;
	sub_8240FC40(ctx, base);
	// 8240D0F4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240D0F8: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8240D0FC: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D100: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8240D104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D108: 4BFFFA89  bl 0x8240cb90
	ctx.lr = 0x8240D10C;
	sub_8240CB90(ctx, base);
	// 8240D10C: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D110: 4198000C  blt cr6, 0x8240d11c
	if ctx.cr[6].lt {
	pc = 0x8240D11C; continue 'dispatch;
	}
	// 8240D114: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240D118: 4800001C  b 0x8240d134
	pc = 0x8240D134; continue 'dispatch;
	// 8240D11C: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D120: 4199000C  bgt cr6, 0x8240d12c
	if ctx.cr[6].gt {
	pc = 0x8240D12C; continue 'dispatch;
	}
	// 8240D124: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 8240D128: 4800000C  b 0x8240d134
	pc = 0x8240D134; continue 'dispatch;
	// 8240D12C: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D130: 48002B11  bl 0x8240fc40
	ctx.lr = 0x8240D134;
	sub_8240FC40(ctx, base);
	// 8240D134: EC1F0732  fmuls f0, f31, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D138: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D13C: EC010732  fmuls f0, f1, f28
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D140: D01E0014  stfs f0, 0x14(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240D144: EC1F06F2  fmuls f0, f31, f27
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[27].f64) as f32) as f64);
	// 8240D148: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D14C: EC0106F2  fmuls f0, f1, f27
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[27].f64) as f32) as f64);
	// 8240D150: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D154: 48000140  b 0x8240d294
	pc = 0x8240D294; continue 'dispatch;
	// 8240D158: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8240D15C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D160: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8240D164: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D168: 4BFFFA29  bl 0x8240cb90
	ctx.lr = 0x8240D16C;
	sub_8240CB90(ctx, base);
	// 8240D16C: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D170: 4198000C  blt cr6, 0x8240d17c
	if ctx.cr[6].lt {
	pc = 0x8240D17C; continue 'dispatch;
	}
	// 8240D174: FFE0E890  fmr f31, f29
	ctx.f[31].f64 = ctx.f[29].f64;
	// 8240D178: 48000020  b 0x8240d198
	pc = 0x8240D198; continue 'dispatch;
	// 8240D17C: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D180: 4199000C  bgt cr6, 0x8240d18c
	if ctx.cr[6].gt {
	pc = 0x8240D18C; continue 'dispatch;
	}
	// 8240D184: FFE0D090  fmr f31, f26
	ctx.f[31].f64 = ctx.f[26].f64;
	// 8240D188: 48000010  b 0x8240d198
	pc = 0x8240D198; continue 'dispatch;
	// 8240D18C: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D190: 48002AB1  bl 0x8240fc40
	ctx.lr = 0x8240D194;
	sub_8240FC40(ctx, base);
	// 8240D194: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240D198: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8240D19C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D1A0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8240D1A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D1A8: 4BFFF9E9  bl 0x8240cb90
	ctx.lr = 0x8240D1AC;
	sub_8240CB90(ctx, base);
	// 8240D1AC: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D1B0: 4198000C  blt cr6, 0x8240d1bc
	if ctx.cr[6].lt {
	pc = 0x8240D1BC; continue 'dispatch;
	}
	// 8240D1B4: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240D1B8: 4800001C  b 0x8240d1d4
	pc = 0x8240D1D4; continue 'dispatch;
	// 8240D1BC: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D1C0: 4199000C  bgt cr6, 0x8240d1cc
	if ctx.cr[6].gt {
	pc = 0x8240D1CC; continue 'dispatch;
	}
	// 8240D1C4: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 8240D1C8: 4800000C  b 0x8240d1d4
	pc = 0x8240D1D4; continue 'dispatch;
	// 8240D1CC: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D1D0: 48002A71  bl 0x8240fc40
	ctx.lr = 0x8240D1D4;
	sub_8240FC40(ctx, base);
	// 8240D1D4: EC1F0732  fmuls f0, f31, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D1D8: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D1DC: EC010732  fmuls f0, f1, f28
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D1E0: D01E0014  stfs f0, 0x14(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240D1E4: EC1F06F2  fmuls f0, f31, f27
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[27].f64) as f32) as f64);
	// 8240D1E8: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D1EC: EC0106F2  fmuls f0, f1, f27
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[27].f64) as f32) as f64);
	// 8240D1F0: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D1F4: 480000A0  b 0x8240d294
	pc = 0x8240D294; continue 'dispatch;
	// 8240D1F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8240D1FC: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D200: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240D204: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D208: 4BFFF989  bl 0x8240cb90
	ctx.lr = 0x8240D20C;
	sub_8240CB90(ctx, base);
	// 8240D20C: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D210: 4198000C  blt cr6, 0x8240d21c
	if ctx.cr[6].lt {
	pc = 0x8240D21C; continue 'dispatch;
	}
	// 8240D214: FFE0E890  fmr f31, f29
	ctx.f[31].f64 = ctx.f[29].f64;
	// 8240D218: 48000020  b 0x8240d238
	pc = 0x8240D238; continue 'dispatch;
	// 8240D21C: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D220: 4199000C  bgt cr6, 0x8240d22c
	if ctx.cr[6].gt {
	pc = 0x8240D22C; continue 'dispatch;
	}
	// 8240D224: FFE0D090  fmr f31, f26
	ctx.f[31].f64 = ctx.f[26].f64;
	// 8240D228: 48000010  b 0x8240d238
	pc = 0x8240D238; continue 'dispatch;
	// 8240D22C: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D230: 48002A11  bl 0x8240fc40
	ctx.lr = 0x8240D234;
	sub_8240FC40(ctx, base);
	// 8240D234: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240D238: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8240D23C: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8240D240: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8240D244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D248: 4BFFF949  bl 0x8240cb90
	ctx.lr = 0x8240D24C;
	sub_8240CB90(ctx, base);
	// 8240D24C: FF01E800  fcmpu cr6, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[29].f64);
	// 8240D250: 4198000C  blt cr6, 0x8240d25c
	if ctx.cr[6].lt {
	pc = 0x8240D25C; continue 'dispatch;
	}
	// 8240D254: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240D258: 4800001C  b 0x8240d274
	pc = 0x8240D274; continue 'dispatch;
	// 8240D25C: FF01D000  fcmpu cr6, f1, f26
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[26].f64);
	// 8240D260: 4199000C  bgt cr6, 0x8240d26c
	if ctx.cr[6].gt {
	pc = 0x8240D26C; continue 'dispatch;
	}
	// 8240D264: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 8240D268: 4800000C  b 0x8240d274
	pc = 0x8240D274; continue 'dispatch;
	// 8240D26C: EC210672  fmuls f1, f1, f25
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D270: 480029D1  bl 0x8240fc40
	ctx.lr = 0x8240D274;
	sub_8240FC40(ctx, base);
	// 8240D274: EC1F0732  fmuls f0, f31, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D278: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D27C: EC010732  fmuls f0, f1, f28
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[28].f64) as f32) as f64);
	// 8240D280: D01E0004  stfs f0, 4(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D284: EC1F06F2  fmuls f0, f31, f27
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[27].f64) as f32) as f64);
	// 8240D288: D01E0010  stfs f0, 0x10(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D28C: EC0106F2  fmuls f0, f1, f27
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[27].f64) as f32) as f64);
	// 8240D290: D01E0014  stfs f0, 0x14(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240D294: 4BFF80FD  bl 0x82405390
	ctx.lr = 0x8240D298;
	sub_82405390(ctx, base);
	// 8240D298: 817F0228  lwz r11, 0x228(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(552 as u32) ) } as u64;
	// 8240D29C: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8240D2A0: 394B000D  addi r10, r11, 0xd
	ctx.r[10].s64 = ctx.r[11].s64 + 13;
	// 8240D2A4: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 8240D2A8: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 8240D2AC: 7D4AF82E  lwzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8240D2B0: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240D2B4: 816B0148  lwz r11, 0x148(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 8240D2B8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240D2BC: 409A0044  bne cr6, 0x8240d300
	if !ctx.cr[6].eq {
	pc = 0x8240D300; continue 'dispatch;
	}
	// 8240D2C0: C03E0010  lfs f1, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240D2C4: 4800124D  bl 0x8240e510
	ctx.lr = 0x8240D2C8;
	sub_8240E510(ctx, base);
	// 8240D2C8: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240D2CC: 4BFF80ED  bl 0x824053b8
	ctx.lr = 0x8240D2D0;
	sub_824053B8(ctx, base);
	// 8240D2D0: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8240D2D4: C03E0000  lfs f1, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240D2D8: EFFF002A  fadds f31, f31, f0
	ctx.f[31].f64 = ((ctx.f[31].f64 + ctx.f[0].f64) as f32) as f64;
	// 8240D2DC: 48001235  bl 0x8240e510
	ctx.lr = 0x8240D2E0;
	sub_8240E510(ctx, base);
	// 8240D2E0: FF1FF000  fcmpu cr6, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 8240D2E4: 40980008  bge cr6, 0x8240d2ec
	if !ctx.cr[6].lt {
	pc = 0x8240D2EC; continue 'dispatch;
	}
	// 8240D2E8: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	// 8240D2EC: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 8240D2F0: 480012C1  bl 0x8240e5b0
	ctx.lr = 0x8240D2F4;
	sub_8240E5B0(ctx, base);
	// 8240D2F4: 48001265  bl 0x8240e558
	ctx.lr = 0x8240D2F8;
	sub_8240E558(ctx, base);
	// 8240D2F8: D03E0000  stfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D2FC: D35E0010  stfs f26, 0x10(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D300: 817F0228  lwz r11, 0x228(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(552 as u32) ) } as u64;
	// 8240D304: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 8240D308: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240D30C: 814B013C  lwz r10, 0x13c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(316 as u32) ) } as u64;
	// 8240D310: 816B014C  lwz r11, 0x14c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(332 as u32) ) } as u64;
	// 8240D314: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240D318: 409A0044  bne cr6, 0x8240d35c
	if !ctx.cr[6].eq {
	pc = 0x8240D35C; continue 'dispatch;
	}
	// 8240D31C: C03E0014  lfs f1, 0x14(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240D320: 480011F1  bl 0x8240e510
	ctx.lr = 0x8240D324;
	sub_8240E510(ctx, base);
	// 8240D324: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240D328: 4BFF8091  bl 0x824053b8
	ctx.lr = 0x8240D32C;
	sub_824053B8(ctx, base);
	// 8240D32C: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 8240D330: C03E0004  lfs f1, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240D334: EFFF002A  fadds f31, f31, f0
	ctx.f[31].f64 = ((ctx.f[31].f64 + ctx.f[0].f64) as f32) as f64;
	// 8240D338: 480011D9  bl 0x8240e510
	ctx.lr = 0x8240D33C;
	sub_8240E510(ctx, base);
	// 8240D33C: FF1FF000  fcmpu cr6, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 8240D340: 40980008  bge cr6, 0x8240d348
	if !ctx.cr[6].lt {
	pc = 0x8240D348; continue 'dispatch;
	}
	// 8240D344: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	// 8240D348: FC40F890  fmr f2, f31
	ctx.f[2].f64 = ctx.f[31].f64;
	// 8240D34C: 48001265  bl 0x8240e5b0
	ctx.lr = 0x8240D350;
	sub_8240E5B0(ctx, base);
	// 8240D350: 48001209  bl 0x8240e558
	ctx.lr = 0x8240D354;
	sub_8240E558(ctx, base);
	// 8240D354: D03E0004  stfs f1, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D358: D35E0014  stfs f26, 0x14(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240D35C: 817F0228  lwz r11, 0x228(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(552 as u32) ) } as u64;
	// 8240D360: 394B000D  addi r10, r11, 0xd
	ctx.r[10].s64 = ctx.r[11].s64 + 13;
	// 8240D364: 1D6B0018  mulli r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 * 24;
	// 8240D368: 1D4A0018  mulli r10, r10, 0x18
	ctx.r[10].s64 = ctx.r[10].s64 * 24;
	// 8240D36C: 7D4AF82E  lwzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 8240D370: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8240D374: 816B0148  lwz r11, 0x148(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(328 as u32) ) } as u64;
	// 8240D378: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240D37C: 419A008C  beq cr6, 0x8240d408
	if ctx.cr[6].eq {
	pc = 0x8240D408; continue 'dispatch;
	}
	// 8240D380: FF18D000  fcmpu cr6, f24, f26
	ctx.cr[6].compare_f64(ctx.f[24].f64, ctx.f[26].f64);
	// 8240D384: 40990084  ble cr6, 0x8240d408
	if !ctx.cr[6].gt {
	pc = 0x8240D408; continue 'dispatch;
	}
	// 8240D388: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8240D38C: 409A007C  bne cr6, 0x8240d408
	if !ctx.cr[6].eq {
	pc = 0x8240D408; continue 'dispatch;
	}
	// 8240D390: C1BE0000  lfs f13, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240D394: FF18E800  fcmpu cr6, f24, f29
	ctx.cr[6].compare_f64(ctx.f[24].f64, ctx.f[29].f64);
	// 8240D398: EDAD0372  fmuls f13, f13, f13
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64) as f32) as f64);
	// 8240D39C: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D3A0: EFC0683A  fmadds f30, f0, f0, f13
	ctx.f[30].f64 = (((ctx.f[0].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 8240D3A4: 4198000C  blt cr6, 0x8240d3b0
	if ctx.cr[6].lt {
	pc = 0x8240D3B0; continue 'dispatch;
	}
	// 8240D3A8: FFE0E890  fmr f31, f29
	ctx.f[31].f64 = ctx.f[29].f64;
	// 8240D3AC: 48000018  b 0x8240d3c4
	pc = 0x8240D3C4; continue 'dispatch;
	// 8240D3B0: EC380672  fmuls f1, f24, f25
	ctx.f[1].f64 = (((ctx.f[24].f64 * ctx.f[25].f64) as f32) as f64);
	// 8240D3B4: 4800288D  bl 0x8240fc40
	ctx.lr = 0x8240D3B8;
	sub_8240FC40(ctx, base);
	// 8240D3B8: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240D3BC: FF1FE800  fcmpu cr6, f31, f29
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[29].f64);
	// 8240D3C0: 4198000C  blt cr6, 0x8240d3cc
	if ctx.cr[6].lt {
	pc = 0x8240D3CC; continue 'dispatch;
	}
	// 8240D3C4: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 8240D3C8: 4800001C  b 0x8240d3e4
	pc = 0x8240D3E4; continue 'dispatch;
	// 8240D3CC: FF1FD000  fcmpu cr6, f31, f26
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[26].f64);
	// 8240D3D0: 4199000C  bgt cr6, 0x8240d3dc
	if ctx.cr[6].gt {
	pc = 0x8240D3DC; continue 'dispatch;
	}
	// 8240D3D4: FC20E890  fmr f1, f29
	ctx.f[1].f64 = ctx.f[29].f64;
	// 8240D3D8: 4800000C  b 0x8240d3e4
	pc = 0x8240D3E4; continue 'dispatch;
	// 8240D3DC: EC3FEFFC  fnmsubs f1, f31, f31, f29
	ctx.f[1].f64 = -(((ctx.f[31].f64 * ctx.f[31].f64 - ctx.f[29].f64) as f32) as f64);
	// 8240D3E0: 48002AD9  bl 0x8240feb8
	ctx.lr = 0x8240D3E4;
	sub_8240FEB8(ctx, base);
	// 8240D3E4: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D3E8: EDBE07F2  fmuls f13, f30, f31
	ctx.f[13].f64 = (((ctx.f[30].f64 * ctx.f[31].f64) as f32) as f64);
	// 8240D3EC: C19E0004  lfs f12, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240D3F0: EC000072  fmuls f0, f0, f1
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8240D3F4: ED8C0072  fmuls f12, f12, f1
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[1].f64) as f32) as f64);
	// 8240D3F8: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D3FC: D19E0004  stfs f12, 4(r30)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D400: D1BE0008  stfs f13, 8(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240D404: 48000008  b 0x8240d40c
	pc = 0x8240D40C; continue 'dispatch;
	// 8240D408: D35E0008  stfs f26, 8(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240D40C: 39600006  li r11, 6
	ctx.r[11].s64 = 6;
	// 8240D410: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D414: FF00D000  fcmpu cr6, f0, f26
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[26].f64);
	// 8240D418: 40980008  bge cr6, 0x8240d420
	if !ctx.cr[6].lt {
	pc = 0x8240D420; continue 'dispatch;
	}
	// 8240D41C: D35E0000  stfs f26, 0(r30)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D420: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8240D424: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8240D428: 4082FFE8  bne 0x8240d410
	if !ctx.cr[0].eq {
	pc = 0x8240D410; continue 'dispatch;
	}
	// 8240D42C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8240D430: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 8240D434: 48128BF1  bl 0x82536024
	ctx.lr = 0x8240D438;
	sub_82535FFC(ctx, base);
	// 8240D438: 48127CD4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8240D440 size=48
    let mut pc: u32 = 0x8240D440;
    'dispatch: loop {
        match pc {
            0x8240D440 => {
    //   block [0x8240D440..0x8240D470)
	// 8240D440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D448: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240D44C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D450: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240D454: 4BFFF815  bl 0x8240cc68
	ctx.lr = 0x8240D458;
	sub_8240CC68(ctx, base);
	// 8240D458: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D45C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240D460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D468: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240D46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D470 size=304
    let mut pc: u32 = 0x8240D470;
    'dispatch: loop {
        match pc {
            0x8240D470 => {
    //   block [0x8240D470..0x8240D5A0)
	// 8240D470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D474: 48127C3D  bl 0x825350b0
	ctx.lr = 0x8240D478;
	sub_82535080(ctx, base);
	// 8240D478: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 8240D47C: 48128B61  bl 0x82535fdc
	ctx.lr = 0x8240D480;
	sub_82535FB0(ctx, base);
	// 8240D480: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D484: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240D488: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8240D48C: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 8240D490: 3CA0820D  lis r5, -0x7df3
	ctx.r[5].s64 = -2113077248;
	// 8240D494: 3CC0820D  lis r6, -0x7df3
	ctx.r[6].s64 = -2113077248;
	// 8240D498: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D49C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D4A0: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 8240D4A4: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D4A8: 3D008200  lis r8, -0x7e00
	ctx.r[8].s64 = -2113929216;
	// 8240D4AC: C344C808  lfs f26, -0x37f8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(-14328 as u32) ) };
	ctx.f[26].f64 = (tmp.f32 as f64);
	// 8240D4B0: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8240D4B4: C3651FF8  lfs f27, 0x1ff8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8184 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 8240D4B8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 8240D4BC: C3862074  lfs f28, 0x2074(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8308 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 8240D4C0: C1AB2068  lfs f13, 0x2068(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8296 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240D4C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D4C8: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D4CC: 3B9F0130  addi r28, r31, 0x130
	ctx.r[28].s64 = ctx.r[31].s64 + 304;
	// 8240D4D0: C3C876F4  lfs f30, 0x76f4(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(30452 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 8240D4D4: 3B400020  li r26, 0x20
	ctx.r[26].s64 = 32;
	// 8240D4D8: C3E9294C  lfs f31, 0x294c(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(10572 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8240D4DC: C32A2140  lfs f25, 0x2140(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8512 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 8240D4E0: C18BE35C  lfs f12, -0x1ca4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-7332 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240D4E4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240D4E8: C3A7E360  lfs f29, -0x1ca0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(-7328 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 8240D4EC: D19F0008  stfs f12, 8(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240D4F0: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240D4F4: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D4F8: C1AB184C  lfs f13, 0x184c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6220 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240D4FC: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 8240D500: D1BF0014  stfs f13, 0x14(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240D504: 3B6B3808  addi r27, r11, 0x3808
	ctx.r[27].s64 = ctx.r[11].s64 + 14344;
	// 8240D508: 3BBCFEEC  addi r29, r28, -0x114
	ctx.r[29].s64 = ctx.r[28].s64 + -276;
	// 8240D50C: 3BDB0004  addi r30, r27, 4
	ctx.r[30].s64 = ctx.r[27].s64 + 4;
	// 8240D510: C01EFFFC  lfs f0, -4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D514: D01DFFFC  stfs f0, -4(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8240D518: C03E0000  lfs f1, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240D51C: 4800103D  bl 0x8240e558
	ctx.lr = 0x8240D520;
	sub_8240E558(ctx, base);
	// 8240D520: D03D0000  stfs f1, 0(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D524: C01E0004  lfs f0, 4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D528: EC00C82A  fadds f0, f0, f25
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[25].f64) as f32) as f64;
	// 8240D52C: D01D0004  stfs f0, 4(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D530: C03E0008  lfs f1, 8(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240D534: 48001025  bl 0x8240e558
	ctx.lr = 0x8240D538;
	sub_8240E558(ctx, base);
	// 8240D538: D03D0008  stfs f1, 8(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240D53C: 397B011C  addi r11, r27, 0x11c
	ctx.r[11].s64 = ctx.r[27].s64 + 284;
	// 8240D540: C01E000C  lfs f0, 0xc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D544: D01D000C  stfs f0, 0xc(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240D548: C01E0010  lfs f0, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D54C: D01D0010  stfs f0, 0x10(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D550: C01E0014  lfs f0, 0x14(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D554: 3BDE001C  addi r30, r30, 0x1c
	ctx.r[30].s64 = ctx.r[30].s64 + 28;
	// 8240D558: D01D0014  stfs f0, 0x14(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240D55C: 3BBD001C  addi r29, r29, 0x1c
	ctx.r[29].s64 = ctx.r[29].s64 + 28;
	// 8240D560: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8240D564: 4198FFAC  blt cr6, 0x8240d510
	if ctx.cr[6].lt {
	pc = 0x8240D510; continue 'dispatch;
	}
	// 8240D568: D3FC0000  stfs f31, 0(r28)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D56C: 375AFFFF  addic. r26, r26, -1
	ctx.xer.ca = (ctx.r[26].u32 > (!(-1 as u32)));
	ctx.r[26].s64 = ctx.r[26].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 8240D570: D3DC0004  stfs f30, 4(r28)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D574: D3BC0008  stfs f29, 8(r28)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240D578: D39C000C  stfs f28, 0xc(r28)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240D57C: D37C0010  stfs f27, 0x10(r28)
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8240D580: D35C0014  stfs f26, 0x14(r28)
	tmp.f32 = (ctx.f[26].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 8240D584: 3B9C0130  addi r28, r28, 0x130
	ctx.r[28].s64 = ctx.r[28].s64 + 304;
	// 8240D588: 4082FF80  bne 0x8240d508
	if !ctx.cr[0].eq {
	pc = 0x8240D508; continue 'dispatch;
	}
	// 8240D58C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8240D590: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8240D594: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 8240D598: 48128A91  bl 0x82536028
	ctx.lr = 0x8240D59C;
	sub_82535FFC(ctx, base);
	// 8240D59C: 48127B64  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D5A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D5A0 size=236
    let mut pc: u32 = 0x8240D5A0;
    'dispatch: loop {
        match pc {
            0x8240D5A0 => {
    //   block [0x8240D5A0..0x8240D68C)
	// 8240D5A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D5A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D5A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D5AC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D5B0: 2B04001F  cmplwi cr6, r4, 0x1f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 31 as u32, &mut ctx.xer);
	// 8240D5B4: C16B1FF8  lfs f11, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240D5B8: D1670000  stfs f11, 0(r7)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D5BC: 419900AC  bgt cr6, 0x8240d668
	if ctx.cr[6].gt {
	pc = 0x8240D668; continue 'dispatch;
	}
	// 8240D5C0: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8240D5C4: 4098001C  bge cr6, 0x8240d5e0
	if !ctx.cr[6].lt {
	pc = 0x8240D5E0; continue 'dispatch;
	}
	// 8240D5C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D5CC: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 8240D5D0: 386BE3B8  addi r3, r11, -0x1c48
	ctx.r[3].s64 = ctx.r[11].s64 + -7240;
	// 8240D5D4: 4BEA59AD  bl 0x822b2f80
	ctx.lr = 0x8240D5D8;
	sub_822B2F80(ctx, base);
	// 8240D5D8: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D5DC: 480000A0  b 0x8240d67c
	pc = 0x8240D67C; continue 'dispatch;
	// 8240D5E0: 1D640130  mulli r11, r4, 0x130
	ctx.r[11].s64 = ctx.r[4].s64 * 304;
	// 8240D5E4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240D5E8: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 8240D5EC: 396B0018  addi r11, r11, 0x18
	ctx.r[11].s64 = ctx.r[11].s64 + 24;
	// 8240D5F0: 419A0054  beq cr6, 0x8240d644
	if ctx.cr[6].eq {
	pc = 0x8240D644; continue 'dispatch;
	}
	// 8240D5F4: 2F06000A  cmpwi cr6, r6, 0xa
	ctx.cr[6].compare_i32(ctx.r[6].s32, 10, &mut ctx.xer);
	// 8240D5F8: 419A0048  beq cr6, 0x8240d640
	if ctx.cr[6].eq {
	pc = 0x8240D640; continue 'dispatch;
	}
	// 8240D5FC: 1D46001C  mulli r10, r6, 0x1c
	ctx.r[10].s64 = ctx.r[6].s64 * 28;
	// 8240D600: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8240D604: C1AB0000  lfs f13, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240D608: C00BFFE4  lfs f0, -0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D60C: C18BFFE8  lfs f12, -0x18(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8240D610: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240D614: 409A000C  bne cr6, 0x8240d620
	if !ctx.cr[6].eq {
	pc = 0x8240D620; continue 'dispatch;
	}
	// 8240D618: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 8240D61C: 48000034  b 0x8240d650
	pc = 0x8240D650; continue 'dispatch;
	// 8240D620: C14B0004  lfs f10, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8240D624: ED210028  fsubs f9, f1, f0
	ctx.f[9].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240D628: ED4A6028  fsubs f10, f10, f12
	ctx.f[10].f64 = (((ctx.f[10].f64 - ctx.f[12].f64) as f32) as f64);
	// 8240D62C: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240D630: EDAA0272  fmuls f13, f10, f9
	ctx.f[13].f64 = (((ctx.f[10].f64 * ctx.f[9].f64) as f32) as f64);
	// 8240D634: EC0D0024  fdivs f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240D638: EC00602A  fadds f0, f0, f12
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[12].f64) as f32) as f64;
	// 8240D63C: 48000014  b 0x8240d650
	pc = 0x8240D650; continue 'dispatch;
	// 8240D640: 38C00009  li r6, 9
	ctx.r[6].s64 = 9;
	// 8240D644: 1D46001C  mulli r10, r6, 0x1c
	ctx.r[10].s64 = ctx.r[6].s64 * 28;
	// 8240D648: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8240D64C: C00B0004  lfs f0, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D650: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 8240D654: 40980008  bge cr6, 0x8240d65c
	if !ctx.cr[6].lt {
	pc = 0x8240D65C; continue 'dispatch;
	}
	// 8240D658: FC005890  fmr f0, f11
	ctx.f[0].f64 = ctx.f[11].f64;
	// 8240D65C: D0070000  stfs f0, 0(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D660: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240D664: 48000018  b 0x8240d67c
	pc = 0x8240D67C; continue 'dispatch;
	// 8240D668: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D66C: 386BE368  addi r3, r11, -0x1c98
	ctx.r[3].s64 = ctx.r[11].s64 + -7320;
	// 8240D670: 4BEA5911  bl 0x822b2f80
	ctx.lr = 0x8240D674;
	sub_822B2F80(ctx, base);
	// 8240D674: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D678: 60630034  ori r3, r3, 0x34
	ctx.r[3].u64 = ctx.r[3].u64 | 52;
	// 8240D67C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240D680: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D684: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D688: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D690 size=216
    let mut pc: u32 = 0x8240D690;
    'dispatch: loop {
        match pc {
            0x8240D690 => {
    //   block [0x8240D690..0x8240D768)
	// 8240D690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D698: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8240D69C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8240D6A0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8240D6A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D6A8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8240D6AC: 2B04001F  cmplwi cr6, r4, 0x1f
	ctx.cr[6].compare_u32(ctx.r[4].u32, 31 as u32, &mut ctx.xer);
	// 8240D6B0: 41990088  bgt cr6, 0x8240d738
	if ctx.cr[6].gt {
	pc = 0x8240D738; continue 'dispatch;
	}
	// 8240D6B4: 1D640130  mulli r11, r4, 0x130
	ctx.r[11].s64 = ctx.r[4].s64 * 304;
	// 8240D6B8: 7FEB1A14  add r31, r11, r3
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 8240D6BC: 480027BD  bl 0x8240fe78
	ctx.lr = 0x8240D6C0;
	sub_8240FE78(ctx, base);
	// 8240D6C0: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8240D6C4: C03F0140  lfs f1, 0x140(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(320 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8240D6C8: 480027B1  bl 0x8240fe78
	ctx.lr = 0x8240D6CC;
	sub_8240FE78(ctx, base);
	// 8240D6CC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D6D0: C1AB1FF8  lfs f13, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240D6D4: FF1F6800  fcmpu cr6, f31, f13
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[13].f64);
	// 8240D6D8: 40980008  bge cr6, 0x8240d6e0
	if !ctx.cr[6].lt {
	pc = 0x8240D6E0; continue 'dispatch;
	}
	// 8240D6DC: FFE0F850  fneg f31, f31
	ctx.f[31].u64 = ctx.f[31].u64 ^ 0x8000_0000_0000_0000u64;
	// 8240D6E0: FF016800  fcmpu cr6, f1, f13
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 8240D6E4: 40980008  bge cr6, 0x8240d6ec
	if !ctx.cr[6].lt {
	pc = 0x8240D6EC; continue 'dispatch;
	}
	// 8240D6E8: FC200850  fneg f1, f1
	ctx.f[1].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 8240D6EC: ED81F82A  fadds f12, f1, f31
	ctx.f[12].f64 = ((ctx.f[1].f64 + ctx.f[31].f64) as f32) as f64;
	// 8240D6F0: C01F0134  lfs f0, 0x134(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D6F4: FF0C0000  fcmpu cr6, f12, f0
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[0].f64);
	// 8240D6F8: 40990034  ble cr6, 0x8240d72c
	if !ctx.cr[6].gt {
	pc = 0x8240D72C; continue 'dispatch;
	}
	// 8240D6FC: C1BF0138  lfs f13, 0x138(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(312 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240D700: FF0C6800  fcmpu cr6, f12, f13
	ctx.cr[6].compare_f64(ctx.f[12].f64, ctx.f[13].f64);
	// 8240D704: 40980024  bge cr6, 0x8240d728
	if !ctx.cr[6].lt {
	pc = 0x8240D728; continue 'dispatch;
	}
	// 8240D708: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8240D70C: 419A0020  beq cr6, 0x8240d72c
	if ctx.cr[6].eq {
	pc = 0x8240D72C; continue 'dispatch;
	}
	// 8240D710: ED8C0028  fsubs f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240D714: C17F013C  lfs f11, 0x13c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8240D718: EC0D0028  fsubs f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8240D71C: EDAC02F2  fmuls f13, f12, f11
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[11].f64) as f32) as f64);
	// 8240D720: EDAD0024  fdivs f13, f13, f0
	ctx.f[13].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 8240D724: 48000008  b 0x8240d72c
	pc = 0x8240D72C; continue 'dispatch;
	// 8240D728: C1BF013C  lfs f13, 0x13c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8240D72C: D1BE0000  stfs f13, 0(r30)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8240D730: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240D734: 48000018  b 0x8240d74c
	pc = 0x8240D74C; continue 'dispatch;
	// 8240D738: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D73C: 386BE408  addi r3, r11, -0x1bf8
	ctx.r[3].s64 = ctx.r[11].s64 + -7160;
	// 8240D740: 4BEA5841  bl 0x822b2f80
	ctx.lr = 0x8240D744;
	sub_822B2F80(ctx, base);
	// 8240D744: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D748: 60630034  ori r3, r3, 0x34
	ctx.r[3].u64 = ctx.r[3].u64 | 52;
	// 8240D74C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8240D750: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D754: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D758: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8240D75C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8240D760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8240D764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D768 size=88
    let mut pc: u32 = 0x8240D768;
    'dispatch: loop {
        match pc {
            0x8240D768 => {
    //   block [0x8240D768..0x8240D7C0)
	// 8240D768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D76C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D770: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D774: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D778: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8240D77C: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D780: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240D784: 41990024  bgt cr6, 0x8240d7a8
	if ctx.cr[6].gt {
	pc = 0x8240D7A8; continue 'dispatch;
	}
	// 8240D788: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D78C: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8240D790: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 8240D794: 386BE458  addi r3, r11, -0x1ba8
	ctx.r[3].s64 = ctx.r[11].s64 + -7080;
	// 8240D798: 4BEA57E9  bl 0x822b2f80
	ctx.lr = 0x8240D79C;
	sub_822B2F80(ctx, base);
	// 8240D79C: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D7A0: 6063003C  ori r3, r3, 0x3c
	ctx.r[3].u64 = ctx.r[3].u64 | 60;
	// 8240D7A4: 4800000C  b 0x8240d7b0
	pc = 0x8240D7B0; continue 'dispatch;
	// 8240D7A8: D02A000C  stfs f1, 0xc(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 8240D7AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240D7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240D7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D7BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D7C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D7C0 size=88
    let mut pc: u32 = 0x8240D7C0;
    'dispatch: loop {
        match pc {
            0x8240D7C0 => {
    //   block [0x8240D7C0..0x8240D818)
	// 8240D7C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D7C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D7C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D7CC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D7D0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8240D7D4: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D7D8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240D7DC: 41990024  bgt cr6, 0x8240d800
	if ctx.cr[6].gt {
	pc = 0x8240D800; continue 'dispatch;
	}
	// 8240D7E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D7E4: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8240D7E8: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 8240D7EC: 386BE4A0  addi r3, r11, -0x1b60
	ctx.r[3].s64 = ctx.r[11].s64 + -7008;
	// 8240D7F0: 4BEA5791  bl 0x822b2f80
	ctx.lr = 0x8240D7F4;
	sub_822B2F80(ctx, base);
	// 8240D7F4: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D7F8: 6063003D  ori r3, r3, 0x3d
	ctx.r[3].u64 = ctx.r[3].u64 | 61;
	// 8240D7FC: 4800000C  b 0x8240d808
	pc = 0x8240D808; continue 'dispatch;
	// 8240D800: D02A0008  stfs f1, 8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 8240D804: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240D808: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240D80C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D810: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8240D818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8240D818 size=100
    let mut pc: u32 = 0x8240D818;
    'dispatch: loop {
        match pc {
            0x8240D818 => {
    //   block [0x8240D818..0x8240D87C)
	// 8240D818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8240D81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8240D820: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8240D824: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8240D828: C00B1850  lfs f0, 0x1850(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(6224 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D82C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240D830: 41980020  blt cr6, 0x8240d850
	if ctx.cr[6].lt {
	pc = 0x8240D850; continue 'dispatch;
	}
	// 8240D834: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8240D838: C00B3034  lfs f0, 0x3034(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8240D83C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 8240D840: 41990010  bgt cr6, 0x8240d850
	if ctx.cr[6].gt {
	pc = 0x8240D850; continue 'dispatch;
	}
	// 8240D844: D0230004  stfs f1, 4(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8240D848: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8240D84C: 48000020  b 0x8240d86c
	pc = 0x8240D86C; continue 'dispatch;
	// 8240D850: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8240D854: D8210018  stfd f1, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.f[1].u64 ) };
	// 8240D858: E8810018  ld r4, 0x18(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(24 as u32) ) };
	// 8240D85C: 386BE4E8  addi r3, r11, -0x1b18
	ctx.r[3].s64 = ctx.r[11].s64 + -6936;
	// 8240D860: 4BEA5721  bl 0x822b2f80
	ctx.lr = 0x8240D864;
	sub_822B2F80(ctx, base);
	// 8240D864: 3C608000  lis r3, -0x8000
	ctx.r[3].s64 = -2147483648;
	// 8240D868: 6063003E  ori r3, r3, 0x3e
	ctx.r[3].u64 = ctx.r[3].u64 | 62;
	// 8240D86C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8240D870: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8240D874: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8240D878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


