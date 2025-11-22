pub fn sub_82312DE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82312DE0 size=16
    let mut pc: u32 = 0x82312DE0;
    'dispatch: loop {
        match pc {
            0x82312DE0 => {
    //   block [0x82312DE0..0x82312DF0)
	// 82312DE0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82312DE4: C00B9A8C  lfs f0, -0x6574(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25972 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82312DE8: D00306C0  stfs f0, 0x6c0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1728 as u32), tmp.u32 ) };
	// 82312DEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82312DF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82312DF0 size=8
    let mut pc: u32 = 0x82312DF0;
    'dispatch: loop {
        match pc {
            0x82312DF0 => {
    //   block [0x82312DF0..0x82312DF8)
	// 82312DF0: 386306A0  addi r3, r3, 0x6a0
	ctx.r[3].s64 = ctx.r[3].s64 + 1696;
	// 82312DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82312DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82312DF8 size=8
    let mut pc: u32 = 0x82312DF8;
    'dispatch: loop {
        match pc {
            0x82312DF8 => {
    //   block [0x82312DF8..0x82312E00)
	// 82312DF8: C0230400  lfs f1, 0x400(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1024 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82312DFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82312E00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82312E00 size=8
    let mut pc: u32 = 0x82312E00;
    'dispatch: loop {
        match pc {
            0x82312E00 => {
    //   block [0x82312E00..0x82312E08)
	// 82312E00: C0230420  lfs f1, 0x420(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1056 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82312E04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82312E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82312E08 size=8
    let mut pc: u32 = 0x82312E08;
    'dispatch: loop {
        match pc {
            0x82312E08 => {
    //   block [0x82312E08..0x82312E10)
	// 82312E08: 806304C0  lwz r3, 0x4c0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1216 as u32) ) } as u64;
	// 82312E0C: 484A704C  b 0x827b9e58
	sub_827B9E58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82312E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82312E10 size=8
    let mut pc: u32 = 0x82312E10;
    'dispatch: loop {
        match pc {
            0x82312E10 => {
    //   block [0x82312E10..0x82312E18)
	// 82312E10: 806304C0  lwz r3, 0x4c0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1216 as u32) ) } as u64;
	// 82312E14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82312E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82312E18 size=68
    let mut pc: u32 = 0x82312E18;
    'dispatch: loop {
        match pc {
            0x82312E18 => {
    //   block [0x82312E18..0x82312E5C)
	// 82312E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82312E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82312E20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82312E24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82312E28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82312E2C: 807F04C0  lwz r3, 0x4c0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1216 as u32) ) } as u64;
	// 82312E30: 484A7029  bl 0x827b9e58
	ctx.lr = 0x82312E34;
	sub_827B9E58(ctx, base);
	// 82312E34: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82312E38: 4182000C  beq 0x82312e44
	if ctx.cr[0].eq {
	pc = 0x82312E44; continue 'dispatch;
	}
	// 82312E3C: 807F04C0  lwz r3, 0x4c0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1216 as u32) ) } as u64;
	// 82312E40: 48000008  b 0x82312e48
	pc = 0x82312E48; continue 'dispatch;
	// 82312E44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82312E48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82312E4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82312E50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82312E54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82312E58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82312E60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82312E60 size=36
    let mut pc: u32 = 0x82312E60;
    'dispatch: loop {
        match pc {
            0x82312E60 => {
    //   block [0x82312E60..0x82312E84)
	// 82312E60: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82312E64: C1A306C0  lfs f13, 0x6c0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1728 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82312E68: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82312E6C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82312E70: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82312E74: 40980008  bge cr6, 0x82312e7c
	if !ctx.cr[6].lt {
	pc = 0x82312E7C; continue 'dispatch;
	}
	// 82312E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82312E7C: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82312E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82312E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82312E88 size=212
    let mut pc: u32 = 0x82312E88;
    'dispatch: loop {
        match pc {
            0x82312E88 => {
    //   block [0x82312E88..0x82312F5C)
	// 82312E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82312E8C: 48E952D1  bl 0x831a815c
	ctx.lr = 0x82312E90;
	sub_831A8130(ctx, base);
	// 82312E90: 3981FFC0  addi r12, r1, -0x40
	ctx.r[12].s64 = ctx.r[1].s64 + -64;
	// 82312E94: 48E95BE5  bl 0x831a8a78
	ctx.lr = 0x82312E98;
	sub_831A8A40(ctx, base);
	// 82312E98: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82312E9C: 3FC08326  lis r30, -0x7cda
	ctx.r[30].s64 = -2094661632;
	// 82312EA0: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82312EA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82312EA8: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82312EAC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82312EB0: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 82312EB4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82312EB8: FF802090  fmr f28, f4
	ctx.f[28].f64 = ctx.f[4].f64;
	// 82312EBC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82312EC0: 809EB3C8  lwz r4, -0x4c38(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19512 as u32) ) } as u64;
	// 82312EC4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82312EC8: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82312ECC: 48AE0B3D  bl 0x82df3a08
	ctx.lr = 0x82312ED0;
	sub_82DF3A08(ctx, base);
	// 82312ED0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82312ED4: 3B210060  addi r25, r1, 0x60
	ctx.r[25].s64 = ctx.r[1].s64 + 96;
	// 82312ED8: 4BFF26C1  bl 0x82305598
	ctx.lr = 0x82312EDC;
	sub_82305598(ctx, base);
	// 82312EDC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82312EE0: 48AE0429  bl 0x82df3308
	ctx.lr = 0x82312EE4;
	sub_82DF3308(ctx, base);
	// 82312EE4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82312EE8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82312EEC: 48AE053D  bl 0x82df3428
	ctx.lr = 0x82312EF0;
	sub_82DF3428(ctx, base);
	// 82312EF0: 572B063F  clrlwi. r11, r25, 0x18
	ctx.r[11].u64 = ctx.r[25].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82312EF4: 40820058  bne 0x82312f4c
	if !ctx.cr[0].eq {
	pc = 0x82312F4C; continue 'dispatch;
	}
	// 82312EF8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82312EFC: 809EB3C8  lwz r4, -0x4c38(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19512 as u32) ) } as u64;
	// 82312F00: 48AE0B09  bl 0x82df3a08
	ctx.lr = 0x82312F04;
	sub_82DF3A08(ctx, base);
	// 82312F04: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82312F08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82312F0C: 4BFF2685  bl 0x82305590
	ctx.lr = 0x82312F10;
	sub_82305590(ctx, base);
	// 82312F10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82312F14: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82312F18: 48AE0511  bl 0x82df3428
	ctx.lr = 0x82312F1C;
	sub_82DF3428(ctx, base);
	// 82312F1C: 81610124  lwz r11, 0x124(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(292 as u32) ) } as u64;
	// 82312F20: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82312F24: FC80E090  fmr f4, f28
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[4].f64 = ctx.f[28].f64;
	// 82312F28: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82312F2C: FC60E890  fmr f3, f29
	ctx.f[3].f64 = ctx.f[29].f64;
	// 82312F30: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82312F34: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82312F38: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82312F3C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82312F40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82312F44: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82312F48: 480289E9  bl 0x8233b930
	ctx.lr = 0x82312F4C;
	sub_8233B930(ctx, base);
	// 82312F4C: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82312F50: 3981FFC0  addi r12, r1, -0x40
	ctx.r[12].s64 = ctx.r[1].s64 + -64;
	// 82312F54: 48E95B71  bl 0x831a8ac4
	ctx.lr = 0x82312F58;
	sub_831A8A8C(ctx, base);
	// 82312F58: 48E95254  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82312F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82312F60 size=8
    let mut pc: u32 = 0x82312F60;
    'dispatch: loop {
        match pc {
            0x82312F60 => {
    //   block [0x82312F60..0x82312F68)
	// 82312F60: C02306E8  lfs f1, 0x6e8(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1768 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82312F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82312F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82312F68 size=8
    let mut pc: u32 = 0x82312F68;
    'dispatch: loop {
        match pc {
            0x82312F68 => {
    //   block [0x82312F68..0x82312F70)
	// 82312F68: D02304BC  stfs f1, 0x4bc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1212 as u32), tmp.u32 ) };
	// 82312F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82312F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82312F70 size=8
    let mut pc: u32 = 0x82312F70;
    'dispatch: loop {
        match pc {
            0x82312F70 => {
    //   block [0x82312F70..0x82312F78)
	// 82312F70: C02304BC  lfs f1, 0x4bc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82312F74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82312F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82312F78 size=116
    let mut pc: u32 = 0x82312F78;
    'dispatch: loop {
        match pc {
            0x82312F78 => {
    //   block [0x82312F78..0x82312FEC)
	// 82312F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82312F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82312F80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82312F84: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82312F88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82312F8C: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82312F90: 41820010  beq 0x82312fa0
	if ctx.cr[0].eq {
	pc = 0x82312FA0; continue 'dispatch;
	}
	// 82312F94: 807F0100  lwz r3, 0x100(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 82312F98: 4BFF0849  bl 0x823037e0
	ctx.lr = 0x82312F9C;
	sub_823037E0(ctx, base);
	// 82312F9C: 4800003C  b 0x82312fd8
	pc = 0x82312FD8; continue 'dispatch;
	// 82312FA0: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82312FA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82312FA8: 808B4DF4  lwz r4, 0x4df4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19956 as u32) ) } as u64;
	// 82312FAC: 48AE0A5D  bl 0x82df3a08
	ctx.lr = 0x82312FB0;
	sub_82DF3A08(ctx, base);
	// 82312FB0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82312FB4: 807F0100  lwz r3, 0x100(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 82312FB8: 39000BBB  li r8, 0xbbb
	ctx.r[8].s64 = 3003;
	// 82312FBC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82312FC0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82312FC4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82312FC8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82312FCC: 4BFF0155  bl 0x82303120
	ctx.lr = 0x82312FD0;
	sub_82303120(ctx, base);
	// 82312FD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82312FD4: 48AE0455  bl 0x82df3428
	ctx.lr = 0x82312FD8;
	sub_82DF3428(ctx, base);
	// 82312FD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82312FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82312FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82312FE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82312FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82312FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82312FF0 size=8
    let mut pc: u32 = 0x82312FF0;
    'dispatch: loop {
        match pc {
            0x82312FF0 => {
    //   block [0x82312FF0..0x82312FF8)
	// 82312FF0: D02304E4  stfs f1, 0x4e4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1252 as u32), tmp.u32 ) };
	// 82312FF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82312FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82312FF8 size=8
    let mut pc: u32 = 0x82312FF8;
    'dispatch: loop {
        match pc {
            0x82312FF8 => {
    //   block [0x82312FF8..0x82313000)
	// 82312FF8: C02304E4  lfs f1, 0x4e4(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1252 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82312FFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313000 size=8
    let mut pc: u32 = 0x82313000;
    'dispatch: loop {
        match pc {
            0x82313000 => {
    //   block [0x82313000..0x82313008)
	// 82313000: C0230580  lfs f1, 0x580(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1408 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82313004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313008 size=8
    let mut pc: u32 = 0x82313008;
    'dispatch: loop {
        match pc {
            0x82313008 => {
    //   block [0x82313008..0x82313010)
	// 82313008: 88630508  lbz r3, 0x508(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1288 as u32) ) } as u64;
	// 8231300C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313010 size=12
    let mut pc: u32 = 0x82313010;
    'dispatch: loop {
        match pc {
            0x82313010 => {
    //   block [0x82313010..0x8231301C)
	// 82313010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82313014: 99630508  stb r11, 0x508(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(1288 as u32), ctx.r[11].u8 ) };
	// 82313018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82313020 size=100
    let mut pc: u32 = 0x82313020;
    'dispatch: loop {
        match pc {
            0x82313020 => {
    //   block [0x82313020..0x82313084)
	// 82313020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82313024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82313028: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8231302C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313030: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313034: 897F03C0  lbz r11, 0x3c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(960 as u32) ) } as u64;
	// 82313038: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8231303C: 40820030  bne 0x8231306c
	if !ctx.cr[0].eq {
	pc = 0x8231306C; continue 'dispatch;
	}
	// 82313040: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82313044: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82313048: 808BB3AC  lwz r4, -0x4c54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19540 as u32) ) } as u64;
	// 8231304C: 48AE09BD  bl 0x82df3a08
	ctx.lr = 0x82313050;
	sub_82DF3A08(ctx, base);
	// 82313050: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82313054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82313058: 4BFF2539  bl 0x82305590
	ctx.lr = 0x8231305C;
	sub_82305590(ctx, base);
	// 8231305C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82313060: 48AE03C9  bl 0x82df3428
	ctx.lr = 0x82313064;
	sub_82DF3428(ctx, base);
	// 82313064: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82313068: 48000008  b 0x82313070
	pc = 0x82313070; continue 'dispatch;
	// 8231306C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82313070: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82313074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8231307C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82313080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313088 size=8
    let mut pc: u32 = 0x82313088;
    'dispatch: loop {
        match pc {
            0x82313088 => {
    //   block [0x82313088..0x82313090)
	// 82313088: D0230584  stfs f1, 0x584(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1412 as u32), tmp.u32 ) };
	// 8231308C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313090 size=8
    let mut pc: u32 = 0x82313090;
    'dispatch: loop {
        match pc {
            0x82313090 => {
    //   block [0x82313090..0x82313098)
	// 82313090: C0230584  lfs f1, 0x584(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1412 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82313094: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313098 size=8
    let mut pc: u32 = 0x82313098;
    'dispatch: loop {
        match pc {
            0x82313098 => {
    //   block [0x82313098..0x823130A0)
	// 82313098: C023050C  lfs f1, 0x50c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1292 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8231309C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823130A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823130A0 size=8
    let mut pc: u32 = 0x823130A0;
    'dispatch: loop {
        match pc {
            0x823130A0 => {
    //   block [0x823130A0..0x823130A8)
	// 823130A0: C0230510  lfs f1, 0x510(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1296 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823130A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823130A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823130A8 size=8
    let mut pc: u32 = 0x823130A8;
    'dispatch: loop {
        match pc {
            0x823130A8 => {
    //   block [0x823130A8..0x823130B0)
	// 823130A8: D0230510  stfs f1, 0x510(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1296 as u32), tmp.u32 ) };
	// 823130AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823130B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823130B0 size=12
    let mut pc: u32 = 0x823130B0;
    'dispatch: loop {
        match pc {
            0x823130B0 => {
    //   block [0x823130B0..0x823130BC)
	// 823130B0: D02305A0  stfs f1, 0x5a0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1440 as u32), tmp.u32 ) };
	// 823130B4: 98A305A4  stb r5, 0x5a4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(1444 as u32), ctx.r[5].u8 ) };
	// 823130B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823130C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823130C0 size=8
    let mut pc: u32 = 0x823130C0;
    'dispatch: loop {
        match pc {
            0x823130C0 => {
    //   block [0x823130C0..0x823130C8)
	// 823130C0: 886305A4  lbz r3, 0x5a4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1444 as u32) ) } as u64;
	// 823130C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823130C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823130C8 size=140
    let mut pc: u32 = 0x823130C8;
    'dispatch: loop {
        match pc {
            0x823130C8 => {
    //   block [0x823130C8..0x82313154)
	// 823130C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823130CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823130D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823130D4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 823130D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823130DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823130E0: C3FF05A0  lfs f31, 0x5a0(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1440 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 823130E4: 4BFF2DC5  bl 0x82305ea8
	ctx.lr = 0x823130E8;
	sub_82305EA8(ctx, base);
	// 823130E8: EDA1F828  fsubs f13, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[1].f64 - ctx.f[31].f64) as f32) as f64);
	// 823130EC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823130F0: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823130F4: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 823130F8: 40980014  bge cr6, 0x8231310c
	if !ctx.cr[6].lt {
	pc = 0x8231310C; continue 'dispatch;
	}
	// 823130FC: 897F05A4  lbz r11, 0x5a4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1444 as u32) ) } as u64;
	// 82313100: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82313104: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82313108: 48000034  b 0x8231313c
	pc = 0x8231313C; continue 'dispatch;
	// 8231310C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82313110: 4BFF23A1  bl 0x823054b0
	ctx.lr = 0x82313114;
	sub_823054B0(ctx, base);
	// 82313114: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 82313118: C00B89AC  lfs f0, -0x7654(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-30292 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8231311C: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82313120: 40990018  ble cr6, 0x82313138
	if !ctx.cr[6].gt {
	pc = 0x82313138; continue 'dispatch;
	}
	// 82313124: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82313128: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8231312C: C00BD72C  lfs f0, -0x28d4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10452 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82313130: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 82313134: 41980008  blt cr6, 0x8231313c
	if ctx.cr[6].lt {
	pc = 0x8231313C; continue 'dispatch;
	}
	// 82313138: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8231313C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82313140: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313144: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82313148: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8231314C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82313150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313158 size=8
    let mut pc: u32 = 0x82313158;
    'dispatch: loop {
        match pc {
            0x82313158 => {
    //   block [0x82313158..0x82313160)
	// 82313158: 806306CC  lwz r3, 0x6cc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1740 as u32) ) } as u64;
	// 8231315C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82313160 size=112
    let mut pc: u32 = 0x82313160;
    'dispatch: loop {
        match pc {
            0x82313160 => {
    //   block [0x82313160..0x823131D0)
	// 82313160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82313164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82313168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8231316C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82313170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313174: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313178: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8231317C: 817F06CC  lwz r11, 0x6cc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1740 as u32) ) } as u64;
	// 82313180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82313184: 419A0034  beq cr6, 0x823131b8
	if ctx.cr[6].eq {
	pc = 0x823131B8; continue 'dispatch;
	}
	// 82313188: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8231318C: 40820018  bne 0x823131a4
	if !ctx.cr[0].eq {
	pc = 0x823131A4; continue 'dispatch;
	}
	// 82313190: 4BFF2271  bl 0x82305400
	ctx.lr = 0x82313194;
	sub_82305400(ctx, base);
	// 82313194: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82313198: 48AE0109  bl 0x82df32a0
	ctx.lr = 0x8231319C;
	sub_82DF32A0(ctx, base);
	// 8231319C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823131A0: 41820018  beq 0x823131b8
	if ctx.cr[0].eq {
	pc = 0x823131B8; continue 'dispatch;
	}
	// 823131A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823131A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823131AC: 4BFF472D  bl 0x823078d8
	ctx.lr = 0x823131B0;
	sub_823078D8(ctx, base);
	// 823131B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823131B4: 4BFF23FD  bl 0x823055b0
	ctx.lr = 0x823131B8;
	sub_823055B0(ctx, base);
	// 823131B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823131BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823131C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823131C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823131C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823131CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823131D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823131D0 size=20
    let mut pc: u32 = 0x823131D0;
    'dispatch: loop {
        match pc {
            0x823131D0 => {
    //   block [0x823131D0..0x823131E4)
	// 823131D0: 816306CC  lwz r11, 0x6cc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1740 as u32) ) } as u64;
	// 823131D4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 823131D8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 823131DC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 823131E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823131E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823131E8 size=88
    let mut pc: u32 = 0x823131E8;
    'dispatch: loop {
        match pc {
            0x823131E8 => {
    //   block [0x823131E8..0x82313240)
	// 823131E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823131EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823131F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823131F4: 89430D34  lbz r10, 0xd34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3380 as u32) ) } as u64;
	// 823131F8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 823131FC: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82313200: 4182000C  beq 0x8231320c
	if ctx.cr[0].eq {
	pc = 0x8231320C; continue 'dispatch;
	}
	// 82313204: 88630D35  lbz r3, 0xd35(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3381 as u32) ) } as u64;
	// 82313208: 48000028  b 0x82313230
	pc = 0x82313230; continue 'dispatch;
	// 8231320C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82313210: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82313214: 99410050  stb r10, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u8 ) };
	// 82313218: 419A0014  beq cr6, 0x8231322c
	if ctx.cr[6].eq {
	pc = 0x8231322C; continue 'dispatch;
	}
	// 8231321C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82313220: 38802005  li r4, 0x2005
	ctx.r[4].s64 = 8197;
	// 82313224: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82313228: 4817BE51  bl 0x8248f078
	ctx.lr = 0x8231322C;
	sub_8248F078(ctx, base);
	// 8231322C: 88610050  lbz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82313230: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82313234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8231323C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313240 size=4
    let mut pc: u32 = 0x82313240;
    'dispatch: loop {
        match pc {
            0x82313240 => {
    //   block [0x82313240..0x82313244)
	// 82313240: 4BFF5D50  b 0x82308f90
	sub_82308F90(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313248 size=8
    let mut pc: u32 = 0x82313248;
    'dispatch: loop {
        match pc {
            0x82313248 => {
    //   block [0x82313248..0x82313250)
	// 82313248: 98830534  stb r4, 0x534(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(1332 as u32), ctx.r[4].u8 ) };
	// 8231324C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313250 size=8
    let mut pc: u32 = 0x82313250;
    'dispatch: loop {
        match pc {
            0x82313250 => {
    //   block [0x82313250..0x82313258)
	// 82313250: 88630534  lbz r3, 0x534(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1332 as u32) ) } as u64;
	// 82313254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82313258 size=76
    let mut pc: u32 = 0x82313258;
    'dispatch: loop {
        match pc {
            0x82313258 => {
    //   block [0x82313258..0x823132A4)
	// 82313258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231325C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82313260: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82313264: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313268: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8231326C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313270: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82313274: 808BB554  lwz r4, -0x4aac(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19116 as u32) ) } as u64;
	// 82313278: 48AE0791  bl 0x82df3a08
	ctx.lr = 0x8231327C;
	sub_82DF3A08(ctx, base);
	// 8231327C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82313280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82313284: 4BFF22F5  bl 0x82305578
	ctx.lr = 0x82313288;
	sub_82305578(ctx, base);
	// 82313288: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8231328C: 48AE019D  bl 0x82df3428
	ctx.lr = 0x82313290;
	sub_82DF3428(ctx, base);
	// 82313290: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82313294: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313298: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8231329C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823132A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823132A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823132A8 size=16
    let mut pc: u32 = 0x823132A8;
    'dispatch: loop {
        match pc {
            0x823132A8 => {
    //   block [0x823132A8..0x823132B8)
	// 823132A8: 3964001C  addi r11, r4, 0x1c
	ctx.r[11].s64 = ctx.r[4].s64 + 28;
	// 823132AC: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823132B0: 7C6B1A14  add r3, r11, r3
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 823132B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823132B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823132B8 size=8
    let mut pc: u32 = 0x823132B8;
    'dispatch: loop {
        match pc {
            0x823132B8 => {
    //   block [0x823132B8..0x823132C0)
	// 823132B8: 80630C94  lwz r3, 0xc94(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3220 as u32) ) } as u64;
	// 823132BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823132C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823132C0 size=8
    let mut pc: u32 = 0x823132C0;
    'dispatch: loop {
        match pc {
            0x823132C0 => {
    //   block [0x823132C0..0x823132C8)
	// 823132C0: 38630CC0  addi r3, r3, 0xcc0
	ctx.r[3].s64 = ctx.r[3].s64 + 3264;
	// 823132C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823132C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823132C8 size=8
    let mut pc: u32 = 0x823132C8;
    'dispatch: loop {
        match pc {
            0x823132C8 => {
    //   block [0x823132C8..0x823132D0)
	// 823132C8: C023054C  lfs f1, 0x54c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1356 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823132CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823132D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823132D0 size=8
    let mut pc: u32 = 0x823132D0;
    'dispatch: loop {
        match pc {
            0x823132D0 => {
    //   block [0x823132D0..0x823132D8)
	// 823132D0: C0230550  lfs f1, 0x550(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1360 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823132D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823132D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823132D8 size=120
    let mut pc: u32 = 0x823132D8;
    'dispatch: loop {
        match pc {
            0x823132D8 => {
    //   block [0x823132D8..0x82313350)
	// 823132D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823132DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823132E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823132E4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823132E8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823132EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823132F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823132F4: 808BB55C  lwz r4, -0x4aa4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19108 as u32) ) } as u64;
	// 823132F8: 48AE0711  bl 0x82df3a08
	ctx.lr = 0x823132FC;
	sub_82DF3A08(ctx, base);
	// 823132FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82313300: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82313304: 4BFF2275  bl 0x82305578
	ctx.lr = 0x82313308;
	sub_82305578(ctx, base);
	// 82313308: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8231330C: 48AE011D  bl 0x82df3428
	ctx.lr = 0x82313310;
	sub_82DF3428(ctx, base);
	// 82313310: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82313314: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82313318: 808BB534  lwz r4, -0x4acc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19148 as u32) ) } as u64;
	// 8231331C: 48AE06ED  bl 0x82df3a08
	ctx.lr = 0x82313320;
	sub_82DF3A08(ctx, base);
	// 82313320: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82313324: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82313328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8231332C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82313330: 4BFF2239  bl 0x82305568
	ctx.lr = 0x82313334;
	sub_82305568(ctx, base);
	// 82313334: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82313338: 48AE00F1  bl 0x82df3428
	ctx.lr = 0x8231333C;
	sub_82DF3428(ctx, base);
	// 8231333C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82313340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82313348: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8231334C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82313350 size=76
    let mut pc: u32 = 0x82313350;
    'dispatch: loop {
        match pc {
            0x82313350 => {
    //   block [0x82313350..0x8231339C)
	// 82313350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82313354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82313358: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8231335C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313360: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82313364: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313368: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8231336C: 808BB564  lwz r4, -0x4a9c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19100 as u32) ) } as u64;
	// 82313370: 48AE0699  bl 0x82df3a08
	ctx.lr = 0x82313374;
	sub_82DF3A08(ctx, base);
	// 82313374: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82313378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8231337C: 4BFF21FD  bl 0x82305578
	ctx.lr = 0x82313380;
	sub_82305578(ctx, base);
	// 82313380: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82313384: 48AE00A5  bl 0x82df3428
	ctx.lr = 0x82313388;
	sub_82DF3428(ctx, base);
	// 82313388: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8231338C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82313394: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82313398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823133A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823133A0 size=36
    let mut pc: u32 = 0x823133A0;
    'dispatch: loop {
        match pc {
            0x823133A0 => {
    //   block [0x823133A0..0x823133C4)
	// 823133A0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823133A4: C1A30CCC  lfs f13, 0xccc(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3276 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823133A8: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823133AC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823133B0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 823133B4: 40980008  bge cr6, 0x823133bc
	if !ctx.cr[6].lt {
	pc = 0x823133BC; continue 'dispatch;
	}
	// 823133B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823133BC: 5563063E  clrlwi r3, r11, 0x18
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 823133C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823133C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823133C8 size=76
    let mut pc: u32 = 0x823133C8;
    'dispatch: loop {
        match pc {
            0x823133C8 => {
    //   block [0x823133C8..0x82313414)
	// 823133C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823133CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823133D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823133D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823133D8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823133DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823133E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823133E4: 808BB568  lwz r4, -0x4a98(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19096 as u32) ) } as u64;
	// 823133E8: 48AE0621  bl 0x82df3a08
	ctx.lr = 0x823133EC;
	sub_82DF3A08(ctx, base);
	// 823133EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823133F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823133F4: 4BFF2185  bl 0x82305578
	ctx.lr = 0x823133F8;
	sub_82305578(ctx, base);
	// 823133F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823133FC: 48AE002D  bl 0x82df3428
	ctx.lr = 0x82313400;
	sub_82DF3428(ctx, base);
	// 82313400: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82313404: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313408: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8231340C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82313410: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313418(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313418 size=16
    let mut pc: u32 = 0x82313418;
    'dispatch: loop {
        match pc {
            0x82313418 => {
    //   block [0x82313418..0x82313428)
	// 82313418: 816306F8  lwz r11, 0x6f8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1784 as u32) ) } as u64;
	// 8231341C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82313420: 916306F8  stw r11, 0x6f8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1784 as u32), ctx.r[11].u32 ) };
	// 82313424: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313428 size=16
    let mut pc: u32 = 0x82313428;
    'dispatch: loop {
        match pc {
            0x82313428 => {
    //   block [0x82313428..0x82313438)
	// 82313428: 816306F8  lwz r11, 0x6f8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1784 as u32) ) } as u64;
	// 8231342C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82313430: 916306F8  stw r11, 0x6f8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1784 as u32), ctx.r[11].u32 ) };
	// 82313434: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313438 size=12
    let mut pc: u32 = 0x82313438;
    'dispatch: loop {
        match pc {
            0x82313438 => {
    //   block [0x82313438..0x82313444)
	// 82313438: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8231343C: 916306F8  stw r11, 0x6f8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1784 as u32), ctx.r[11].u32 ) };
	// 82313440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313448 size=16
    let mut pc: u32 = 0x82313448;
    'dispatch: loop {
        match pc {
            0x82313448 => {
    //   block [0x82313448..0x82313458)
	// 82313448: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8231344C: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82313450: D0030D30  stfs f0, 0xd30(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(3376 as u32), tmp.u32 ) };
	// 82313454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313458 size=16
    let mut pc: u32 = 0x82313458;
    'dispatch: loop {
        match pc {
            0x82313458 => {
    //   block [0x82313458..0x82313468)
	// 82313458: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8231345C: 98830D35  stb r4, 0xd35(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3381 as u32), ctx.r[4].u8 ) };
	// 82313460: 99630D34  stb r11, 0xd34(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3380 as u32), ctx.r[11].u8 ) };
	// 82313464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313468 size=12
    let mut pc: u32 = 0x82313468;
    'dispatch: loop {
        match pc {
            0x82313468 => {
    //   block [0x82313468..0x82313474)
	// 82313468: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8231346C: 99630D34  stb r11, 0xd34(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3380 as u32), ctx.r[11].u8 ) };
	// 82313470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313478 size=16
    let mut pc: u32 = 0x82313478;
    'dispatch: loop {
        match pc {
            0x82313478 => {
    //   block [0x82313478..0x82313488)
	// 82313478: C0030D3C  lfs f0, 0xd3c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8231347C: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 82313480: D0030D3C  stfs f0, 0xd3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(3388 as u32), tmp.u32 ) };
	// 82313484: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313488 size=8
    let mut pc: u32 = 0x82313488;
    'dispatch: loop {
        match pc {
            0x82313488 => {
    //   block [0x82313488..0x82313490)
	// 82313488: D0230514  stfs f1, 0x514(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1300 as u32), tmp.u32 ) };
	// 8231348C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313490 size=8
    let mut pc: u32 = 0x82313490;
    'dispatch: loop {
        match pc {
            0x82313490 => {
    //   block [0x82313490..0x82313498)
	// 82313490: C0230514  lfs f1, 0x514(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1300 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82313494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82313498 size=152
    let mut pc: u32 = 0x82313498;
    'dispatch: loop {
        match pc {
            0x82313498 => {
    //   block [0x82313498..0x82313530)
	// 82313498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231349C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823134A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823134A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823134A8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 823134AC: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823134B0: 3FC08326  lis r30, -0x7cda
	ctx.r[30].s64 = -2094661632;
	// 823134B4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823134B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823134BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823134C0: 809EB4D4  lwz r4, -0x4b2c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19244 as u32) ) } as u64;
	// 823134C4: 48AE0545  bl 0x82df3a08
	ctx.lr = 0x823134C8;
	sub_82DF3A08(ctx, base);
	// 823134C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823134CC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823134D0: 4BFF2061  bl 0x82305530
	ctx.lr = 0x823134D4;
	sub_82305530(ctx, base);
	// 823134D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823134D8: 48ADFF51  bl 0x82df3428
	ctx.lr = 0x823134DC;
	sub_82DF3428(ctx, base);
	// 823134DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823134E0: 809EB4D4  lwz r4, -0x4b2c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19244 as u32) ) } as u64;
	// 823134E4: 48AE0525  bl 0x82df3a08
	ctx.lr = 0x823134E8;
	sub_82DF3A08(ctx, base);
	// 823134E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823134EC: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 823134F0: 487000A9  bl 0x82a13598
	ctx.lr = 0x823134F4;
	sub_82A13598(ctx, base);
	// 823134F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823134F8: 4BFEED39  bl 0x82302230
	ctx.lr = 0x823134FC;
	sub_82302230(ctx, base);
	// 823134FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313500: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82313504: 48ADFF25  bl 0x82df3428
	ctx.lr = 0x82313508;
	sub_82DF3428(ctx, base);
	// 82313508: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8231350C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82313510: 4803A669  bl 0x8234db78
	ctx.lr = 0x82313514;
	sub_8234DB78(ctx, base);
	// 82313514: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82313518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8231351C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82313520: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82313524: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82313528: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8231352C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313530 size=12
    let mut pc: u32 = 0x82313530;
    'dispatch: loop {
        match pc {
            0x82313530 => {
    //   block [0x82313530..0x8231353C)
	// 82313530: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82313534: 99630600  stb r11, 0x600(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(1536 as u32), ctx.r[11].u8 ) };
	// 82313538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313540 size=8
    let mut pc: u32 = 0x82313540;
    'dispatch: loop {
        match pc {
            0x82313540 => {
    //   block [0x82313540..0x82313548)
	// 82313540: C0230424  lfs f1, 0x424(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1060 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82313544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82313548 size=112
    let mut pc: u32 = 0x82313548;
    'dispatch: loop {
        match pc {
            0x82313548 => {
    //   block [0x82313548..0x823135B8)
	// 82313548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231354C: 48E94C1D  bl 0x831a8168
	ctx.lr = 0x82313550;
	sub_831A8130(ctx, base);
	// 82313550: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82313554: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313558: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8231355C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82313560: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313564: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82313568: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8231356C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82313570: 808BB428  lwz r4, -0x4bd8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19416 as u32) ) } as u64;
	// 82313574: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82313578: 48AE0491  bl 0x82df3a08
	ctx.lr = 0x8231357C;
	sub_82DF3A08(ctx, base);
	// 8231357C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82313580: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82313584: 4BFF200D  bl 0x82305590
	ctx.lr = 0x82313588;
	sub_82305590(ctx, base);
	// 82313588: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8231358C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82313590: 48ADFE99  bl 0x82df3428
	ctx.lr = 0x82313594;
	sub_82DF3428(ctx, base);
	// 82313594: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82313598: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8231359C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 823135A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823135A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823135A8: 48028DB1  bl 0x8233c358
	ctx.lr = 0x823135AC;
	sub_8233C358(ctx, base);
	// 823135AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823135B0: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 823135B4: 48E94C04  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823135B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823135B8 size=104
    let mut pc: u32 = 0x823135B8;
    'dispatch: loop {
        match pc {
            0x823135B8 => {
    //   block [0x823135B8..0x82313620)
	// 823135B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823135BC: 48E94BB1  bl 0x831a816c
	ctx.lr = 0x823135C0;
	sub_831A8130(ctx, base);
	// 823135C0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 823135C4: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823135C8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823135CC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823135D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823135D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823135D8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 823135DC: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 823135E0: 808BB428  lwz r4, -0x4bd8(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19416 as u32) ) } as u64;
	// 823135E4: 48AE0425  bl 0x82df3a08
	ctx.lr = 0x823135E8;
	sub_82DF3A08(ctx, base);
	// 823135E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823135EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823135F0: 4BFF1FA1  bl 0x82305590
	ctx.lr = 0x823135F4;
	sub_82305590(ctx, base);
	// 823135F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823135F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823135FC: 48ADFE2D  bl 0x82df3428
	ctx.lr = 0x82313600;
	sub_82DF3428(ctx, base);
	// 82313600: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82313604: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82313608: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8231360C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82313610: 48028C69  bl 0x8233c278
	ctx.lr = 0x82313614;
	sub_8233C278(ctx, base);
	// 82313614: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82313618: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8231361C: 48E94BA0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313620 size=16
    let mut pc: u32 = 0x82313620;
    'dispatch: loop {
        match pc {
            0x82313620 => {
    //   block [0x82313620..0x82313630)
	// 82313620: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82313624: D0230D20  stfs f1, 0xd20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(3360 as u32), tmp.u32 ) };
	// 82313628: 99630D1C  stb r11, 0xd1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3356 as u32), ctx.r[11].u8 ) };
	// 8231362C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313630 size=12
    let mut pc: u32 = 0x82313630;
    'dispatch: loop {
        match pc {
            0x82313630 => {
    //   block [0x82313630..0x8231363C)
	// 82313630: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82313634: 99630D1C  stb r11, 0xd1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3356 as u32), ctx.r[11].u8 ) };
	// 82313638: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313640 size=8
    let mut pc: u32 = 0x82313640;
    'dispatch: loop {
        match pc {
            0x82313640 => {
    //   block [0x82313640..0x82313648)
	// 82313640: 88630D1C  lbz r3, 0xd1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3356 as u32) ) } as u64;
	// 82313644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313648 size=8
    let mut pc: u32 = 0x82313648;
    'dispatch: loop {
        match pc {
            0x82313648 => {
    //   block [0x82313648..0x82313650)
	// 82313648: 80630538  lwz r3, 0x538(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1336 as u32) ) } as u64;
	// 8231364C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313650 size=8
    let mut pc: u32 = 0x82313650;
    'dispatch: loop {
        match pc {
            0x82313650 => {
    //   block [0x82313650..0x82313658)
	// 82313650: 90830538  stw r4, 0x538(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1336 as u32), ctx.r[4].u32 ) };
	// 82313654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313658 size=8
    let mut pc: u32 = 0x82313658;
    'dispatch: loop {
        match pc {
            0x82313658 => {
    //   block [0x82313658..0x82313660)
	// 82313658: C023053C  lfs f1, 0x53c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1340 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8231365C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313660 size=12
    let mut pc: u32 = 0x82313660;
    'dispatch: loop {
        match pc {
            0x82313660 => {
    //   block [0x82313660..0x8231366C)
	// 82313660: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82313664: 99630C98  stb r11, 0xc98(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3224 as u32), ctx.r[11].u8 ) };
	// 82313668: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313670 size=8
    let mut pc: u32 = 0x82313670;
    'dispatch: loop {
        match pc {
            0x82313670 => {
    //   block [0x82313670..0x82313678)
	// 82313670: 88630C98  lbz r3, 0xc98(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(3224 as u32) ) } as u64;
	// 82313674: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313678 size=28
    let mut pc: u32 = 0x82313678;
    'dispatch: loop {
        match pc {
            0x82313678 => {
    //   block [0x82313678..0x82313694)
	// 82313678: 896304B4  lbz r11, 0x4b4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1204 as u32) ) } as u64;
	// 8231367C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82313680: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82313684: 41820010  beq 0x82313694
	if ctx.cr[0].eq {
		sub_82313694(ctx, base);
		return;
	}
	// 82313688: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8231368C: 388B26E4  addi r4, r11, 0x26e4
	ctx.r[4].s64 = ctx.r[11].s64 + 9956;
	// 82313690: 4BFF5850  b 0x82308ee0
	sub_82308EE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313694(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313694 size=12
    let mut pc: u32 = 0x82313694;
    'dispatch: loop {
        match pc {
            0x82313694 => {
    //   block [0x82313694..0x823136A0)
	// 82313694: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82313698: 388B26DC  addi r4, r11, 0x26dc
	ctx.r[4].s64 = ctx.r[11].s64 + 9948;
	// 8231369C: 4BFF5844  b 0x82308ee0
	sub_82308EE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823136A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823136A0 size=28
    let mut pc: u32 = 0x823136A0;
    'dispatch: loop {
        match pc {
            0x823136A0 => {
    //   block [0x823136A0..0x823136BC)
	// 823136A0: 896304B4  lbz r11, 0x4b4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1204 as u32) ) } as u64;
	// 823136A4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 823136A8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823136AC: 41820010  beq 0x823136bc
	if ctx.cr[0].eq {
		sub_823136BC(ctx, base);
		return;
	}
	// 823136B0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 823136B4: 388B26F4  addi r4, r11, 0x26f4
	ctx.r[4].s64 = ctx.r[11].s64 + 9972;
	// 823136B8: 4BFF5828  b 0x82308ee0
	sub_82308EE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823136BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823136BC size=12
    let mut pc: u32 = 0x823136BC;
    'dispatch: loop {
        match pc {
            0x823136BC => {
    //   block [0x823136BC..0x823136C8)
	// 823136BC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 823136C0: 388B26EC  addi r4, r11, 0x26ec
	ctx.r[4].s64 = ctx.r[11].s64 + 9964;
	// 823136C4: 4BFF581C  b 0x82308ee0
	sub_82308EE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823136C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823136C8 size=28
    let mut pc: u32 = 0x823136C8;
    'dispatch: loop {
        match pc {
            0x823136C8 => {
    //   block [0x823136C8..0x823136E4)
	// 823136C8: 896304B4  lbz r11, 0x4b4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(1204 as u32) ) } as u64;
	// 823136CC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 823136D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823136D4: 41820010  beq 0x823136e4
	if ctx.cr[0].eq {
		sub_823136E4(ctx, base);
		return;
	}
	// 823136D8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 823136DC: 388B26CC  addi r4, r11, 0x26cc
	ctx.r[4].s64 = ctx.r[11].s64 + 9932;
	// 823136E0: 4BFF5800  b 0x82308ee0
	sub_82308EE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823136E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823136E4 size=12
    let mut pc: u32 = 0x823136E4;
    'dispatch: loop {
        match pc {
            0x823136E4 => {
    //   block [0x823136E4..0x823136F0)
	// 823136E4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 823136E8: 388B26C4  addi r4, r11, 0x26c4
	ctx.r[4].s64 = ctx.r[11].s64 + 9924;
	// 823136EC: 4BFF57F4  b 0x82308ee0
	sub_82308EE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823136F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823136F0 size=8
    let mut pc: u32 = 0x823136F0;
    'dispatch: loop {
        match pc {
            0x823136F0 => {
    //   block [0x823136F0..0x823136F8)
	// 823136F0: 90830D24  stw r4, 0xd24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(3364 as u32), ctx.r[4].u32 ) };
	// 823136F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823136F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823136F8 size=20
    let mut pc: u32 = 0x823136F8;
    'dispatch: loop {
        match pc {
            0x823136F8 => {
    //   block [0x823136F8..0x8231370C)
	// 823136F8: 81630D24  lwz r11, 0xd24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3364 as u32) ) } as u64;
	// 823136FC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82313700: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82313704: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82313708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82313710 size=64
    let mut pc: u32 = 0x82313710;
    'dispatch: loop {
        match pc {
            0x82313710 => {
    //   block [0x82313710..0x82313750)
	// 82313710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82313714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82313718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8231371C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82313720: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82313724: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82313728: 99610050  stb r11, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u8 ) };
	// 8231372C: 419A0010  beq cr6, 0x8231373c
	if ctx.cr[6].eq {
	pc = 0x8231373C; continue 'dispatch;
	}
	// 82313730: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82313734: 3880200B  li r4, 0x200b
	ctx.r[4].s64 = 8203;
	// 82313738: 4817B941  bl 0x8248f078
	ctx.lr = 0x8231373C;
	sub_8248F078(ctx, base);
	// 8231373C: 88610050  lbz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82313740: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82313744: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313748: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8231374C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313750 size=8
    let mut pc: u32 = 0x82313750;
    'dispatch: loop {
        match pc {
            0x82313750 => {
    //   block [0x82313750..0x82313758)
	// 82313750: 98830564  stb r4, 0x564(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(1380 as u32), ctx.r[4].u8 ) };
	// 82313754: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313758 size=8
    let mut pc: u32 = 0x82313758;
    'dispatch: loop {
        match pc {
            0x82313758 => {
    //   block [0x82313758..0x82313760)
	// 82313758: D023053C  stfs f1, 0x53c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1340 as u32), tmp.u32 ) };
	// 8231375C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82313760 size=92
    let mut pc: u32 = 0x82313760;
    'dispatch: loop {
        match pc {
            0x82313760 => {
    //   block [0x82313760..0x823137BC)
	// 82313760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82313764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82313768: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8231376C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82313770: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313774: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313778: 4BFF23E1  bl 0x82305b58
	ctx.lr = 0x8231377C;
	sub_82305B58(ctx, base);
	// 8231377C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82313780: 41820024  beq 0x823137a4
	if ctx.cr[0].eq {
	pc = 0x823137A4; continue 'dispatch;
	}
	// 82313784: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82313788: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8231378C: 4BFF232D  bl 0x82305ab8
	ctx.lr = 0x82313790;
	sub_82305AB8(ctx, base);
	// 82313790: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82313794: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82313798: 4BFF23C1  bl 0x82305b58
	ctx.lr = 0x8231379C;
	sub_82305B58(ctx, base);
	// 8231379C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823137A0: 487D13A1  bl 0x82ae4b40
	ctx.lr = 0x823137A4;
	sub_82AE4B40(ctx, base);
	// 823137A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823137A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823137AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823137B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823137B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823137B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823137C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823137C0 size=20
    let mut pc: u32 = 0x823137C0;
    'dispatch: loop {
        match pc {
            0x823137C0 => {
    //   block [0x823137C0..0x823137D4)
	// 823137C0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 823137C4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823137C8: 806A26B8  lwz r3, 0x26b8(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9912 as u32) ) } as u64;
	// 823137CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823137D0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823137D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823137D4 size=8
    let mut pc: u32 = 0x823137D4;
    'dispatch: loop {
        match pc {
            0x823137D4 => {
    //   block [0x823137D4..0x823137DC)
	// 823137D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823137D8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823137DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823137DC size=4
    let mut pc: u32 = 0x823137DC;
    'dispatch: loop {
        match pc {
            0x823137DC => {
    //   block [0x823137DC..0x823137E0)
	// 823137DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823137E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823137E0 size=136
    let mut pc: u32 = 0x823137E0;
    'dispatch: loop {
        match pc {
            0x823137E0 => {
    //   block [0x823137E0..0x82313868)
	// 823137E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823137E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823137E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823137EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823137F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823137F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823137F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823137FC: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82313800: 409A0020  bne cr6, 0x82313820
	if !ctx.cr[6].eq {
	pc = 0x82313820; continue 'dispatch;
	}
	// 82313804: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82313808: 419A0048  beq cr6, 0x82313850
	if ctx.cr[6].eq {
	pc = 0x82313850; continue 'dispatch;
	}
	// 8231380C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82313810: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82313814: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82313818: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8231381C: 48000034  b 0x82313850
	pc = 0x82313850; continue 'dispatch;
	// 82313820: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82313824: 419A002C  beq cr6, 0x82313850
	if ctx.cr[6].eq {
	pc = 0x82313850; continue 'dispatch;
	}
	// 82313828: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8231382C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82313830: 388B7E48  addi r4, r11, 0x7e48
	ctx.r[4].s64 = ctx.r[11].s64 + 32328;
	// 82313834: 48E948C5  bl 0x831a80f8
	ctx.lr = 0x82313838;
	sub_831A80F8(ctx, base);
	// 82313838: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8231383C: 4182000C  beq 0x82313848
	if ctx.cr[0].eq {
	pc = 0x82313848; continue 'dispatch;
	}
	// 82313840: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82313844: 4800000C  b 0x82313850
	pc = 0x82313850; continue 'dispatch;
	// 82313848: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8231384C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82313850: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82313854: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313858: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8231385C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82313860: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82313864: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82313868 size=84
    let mut pc: u32 = 0x82313868;
    'dispatch: loop {
        match pc {
            0x82313868 => {
    //   block [0x82313868..0x823138BC)
	// 82313868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231386C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82313870: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82313874: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313878: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8231387C: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82313880: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82313884: 419A0018  beq cr6, 0x8231389c
	if ctx.cr[6].eq {
	pc = 0x8231389C; continue 'dispatch;
	}
	// 82313888: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8231388C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82313890: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82313894: 4E800421  bctrl
	ctx.lr = 0x82313898;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82313898: 48000008  b 0x823138a0
	pc = 0x823138A0; continue 'dispatch;
	// 8231389C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823138A0: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 823138A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823138A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823138AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823138B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823138B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823138B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823138C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823138C0 size=80
    let mut pc: u32 = 0x823138C0;
    'dispatch: loop {
        match pc {
            0x823138C0 => {
    //   block [0x823138C0..0x82313910)
	// 823138C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823138C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823138C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823138CC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823138D0: C1AB08A8  lfs f13, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823138D4: FF016800  fcmpu cr6, f1, f13
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[13].f64);
	// 823138D8: 41990018  bgt cr6, 0x823138f0
	if ctx.cr[6].gt {
	pc = 0x823138F0; continue 'dispatch;
	}
	// 823138DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823138E0: FDA00890  fmr f13, f1
	ctx.f[13].f64 = ctx.f[1].f64;
	// 823138E4: C00B9534  lfs f0, -0x6acc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823138E8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 823138EC: 41980008  blt cr6, 0x823138f4
	if ctx.cr[6].lt {
	pc = 0x823138F4; continue 'dispatch;
	}
	// 823138F0: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 823138F4: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 823138F8: 48E979B1  bl 0x831ab2a8
	ctx.lr = 0x823138FC;
	sub_831AB2A8(ctx, base);
	// 823138FC: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82313900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82313904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8231390C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313910 size=104
    let mut pc: u32 = 0x82313910;
    'dispatch: loop {
        match pc {
            0x82313910 => {
    //   block [0x82313910..0x82313978)
	// 82313910: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82313914: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82313918: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8231391C: 390B6910  addi r8, r11, 0x6910
	ctx.r[8].s64 = ctx.r[11].s64 + 26896;
	// 82313920: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82313924: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82313928: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 8231392C: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82313930: C1A908A8  lfs f13, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82313934: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 82313938: D0030010  stfs f0, 0x10(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8231393C: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82313940: D1A30014  stfs f13, 0x14(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 82313944: 38C00050  li r6, 0x50
	ctx.r[6].s64 = 80;
	// 82313948: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8231394C: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 82313950: 13E040C7  vcmpequd (lvx128) v31, v0, v8
	tmp.u32 = ctx.r[8].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82313978 size=76
    let mut pc: u32 = 0x82313978;
    'dispatch: loop {
        match pc {
            0x82313978 => {
    //   block [0x82313978..0x823139C4)
	// 82313978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231397C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82313980: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82313984: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82313988: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8231398C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313990: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82313994: 48AFF3E5  bl 0x82e12d78
	ctx.lr = 0x82313998;
	sub_82E12D78(ctx, base);
	// 82313998: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8231399C: 4182000C  beq 0x823139a8
	if ctx.cr[0].eq {
	pc = 0x823139A8; continue 'dispatch;
	}
	// 823139A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823139A4: 48ADEA35  bl 0x82df23d8
	ctx.lr = 0x823139A8;
	sub_82DF23D8(ctx, base);
	// 823139A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823139AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823139B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823139B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823139B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823139BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823139C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823139C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823139C8 size=112
    let mut pc: u32 = 0x823139C8;
    'dispatch: loop {
        match pc {
            0x823139C8 => {
    //   block [0x823139C8..0x82313A38)
	// 823139C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823139CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823139D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823139D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823139D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823139DC: 894B0600  lbz r10, 0x600(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1536 as u32) ) } as u64;
	// 823139E0: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823139E4: 4182000C  beq 0x823139f0
	if ctx.cr[0].eq {
	pc = 0x823139F0; continue 'dispatch;
	}
	// 823139E8: 386B0610  addi r3, r11, 0x610
	ctx.r[3].s64 = ctx.r[11].s64 + 1552;
	// 823139EC: 48000038  b 0x82313a24
	pc = 0x82313A24; continue 'dispatch;
	// 823139F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823139F4: 808B0100  lwz r4, 0x100(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(256 as u32) ) } as u64;
	// 823139F8: 481FBB21  bl 0x8250f518
	ctx.lr = 0x823139FC;
	sub_8250F518(ctx, base);
	// 823139FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82313A00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82313A04: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 82313A08: 409A0008  bne cr6, 0x82313a10
	if !ctx.cr[6].eq {
	pc = 0x82313A10; continue 'dispatch;
	}
	// 82313A0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82313A10: 482145C1  bl 0x82527fd0
	ctx.lr = 0x82313A14;
	sub_82527FD0(ctx, base);
	// 82313A14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313A18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82313A1C: 48ADE275  bl 0x82df1c90
	ctx.lr = 0x82313A20;
	sub_82DF1C90(ctx, base);
	// 82313A20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82313A24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82313A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82313A30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82313A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313A38 size=12
    let mut pc: u32 = 0x82313A38;
    'dispatch: loop {
        match pc {
            0x82313A38 => {
    //   block [0x82313A38..0x82313A44)
	// 82313A38: 81630568  lwz r11, 0x568(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1384 as u32) ) } as u64;
	// 82313A3C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82313A40: 4BFD6F68  b 0x822ea9a8
	sub_822EA9A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313A48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313A48 size=12
    let mut pc: u32 = 0x82313A48;
    'dispatch: loop {
        match pc {
            0x82313A48 => {
    //   block [0x82313A48..0x82313A54)
	// 82313A48: 81630568  lwz r11, 0x568(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1384 as u32) ) } as u64;
	// 82313A4C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82313A50: 4BFD7B60  b 0x822eb5b0
	sub_822EB5B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313A58 size=12
    let mut pc: u32 = 0x82313A58;
    'dispatch: loop {
        match pc {
            0x82313A58 => {
    //   block [0x82313A58..0x82313A64)
	// 82313A58: 81630568  lwz r11, 0x568(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1384 as u32) ) } as u64;
	// 82313A5C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82313A60: 4BFD7750  b 0x822eb1b0
	sub_822EB1B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82313A68 size=92
    let mut pc: u32 = 0x82313A68;
    'dispatch: loop {
        match pc {
            0x82313A68 => {
    //   block [0x82313A68..0x82313AC4)
	// 82313A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82313A6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82313A70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82313A74: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82313A78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313A7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313A80: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 82313A84: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82313A88: 48147361  bl 0x8245ade8
	ctx.lr = 0x82313A8C;
	sub_8245ADE8(ctx, base);
	// 82313A8C: 38800085  li r4, 0x85
	ctx.r[4].s64 = 133;
	// 82313A90: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82313A94: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82313A98: 48147351  bl 0x8245ade8
	ctx.lr = 0x82313A9C;
	sub_8245ADE8(ctx, base);
	// 82313A9C: EDA10072  fmuls f13, f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 82313AA0: C01F0544  lfs f0, 0x544(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1348 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82313AA4: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82313AA8: EC2007F2  fmuls f1, f0, f31
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82313AAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82313AB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313AB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82313AB8: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82313ABC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82313AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82313AC8 size=92
    let mut pc: u32 = 0x82313AC8;
    'dispatch: loop {
        match pc {
            0x82313AC8 => {
    //   block [0x82313AC8..0x82313B24)
	// 82313AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82313ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82313AD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82313AD4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82313AD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313ADC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313AE0: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 82313AE4: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82313AE8: 48147301  bl 0x8245ade8
	ctx.lr = 0x82313AEC;
	sub_8245ADE8(ctx, base);
	// 82313AEC: 38800086  li r4, 0x86
	ctx.r[4].s64 = 134;
	// 82313AF0: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82313AF4: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82313AF8: 481472F1  bl 0x8245ade8
	ctx.lr = 0x82313AFC;
	sub_8245ADE8(ctx, base);
	// 82313AFC: EDA10072  fmuls f13, f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[1].f64 * ctx.f[1].f64) as f32) as f64);
	// 82313B00: C01F0548  lfs f0, 0x548(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82313B04: EC006824  fdivs f0, f0, f13
	ctx.f[0].f64 = ((ctx.f[0].f64 / ctx.f[13].f64) as f32) as f64;
	// 82313B08: EC2007F2  fmuls f1, f0, f31
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 82313B0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82313B10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313B14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82313B18: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82313B1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82313B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313B28 size=16
    let mut pc: u32 = 0x82313B28;
    'dispatch: loop {
        match pc {
            0x82313B28 => {
    //   block [0x82313B28..0x82313B38)
	// 82313B28: 39600210  li r11, 0x210
	ctx.r[11].s64 = 528;
	// 82313B2C: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313B38 size=32
    let mut pc: u32 = 0x82313B38;
    'dispatch: loop {
        match pc {
            0x82313B38 => {
    //   block [0x82313B38..0x82313B58)
	// 82313B38: 39600210  li r11, 0x210
	ctx.r[11].s64 = 528;
	// 82313B3C: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82313B40: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82313B44: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313B58 size=16
    let mut pc: u32 = 0x82313B58;
    'dispatch: loop {
        match pc {
            0x82313B58 => {
    //   block [0x82313B58..0x82313B68)
	// 82313B58: 396002F0  li r11, 0x2f0
	ctx.r[11].s64 = 752;
	// 82313B5C: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313B68 size=16
    let mut pc: u32 = 0x82313B68;
    'dispatch: loop {
        match pc {
            0x82313B68 => {
    //   block [0x82313B68..0x82313B78)
	// 82313B68: 396003D0  li r11, 0x3d0
	ctx.r[11].s64 = 976;
	// 82313B6C: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313B78 size=16
    let mut pc: u32 = 0x82313B78;
    'dispatch: loop {
        match pc {
            0x82313B78 => {
    //   block [0x82313B78..0x82313B88)
	// 82313B78: 396003D0  li r11, 0x3d0
	ctx.r[11].s64 = 976;
	// 82313B7C: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82313B88 size=120
    let mut pc: u32 = 0x82313B88;
    'dispatch: loop {
        match pc {
            0x82313B88 => {
    //   block [0x82313B88..0x82313C00)
	// 82313B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82313B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82313B90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82313B94: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82313B98: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82313B9C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313BA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313BA4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82313BA8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82313BAC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82313BB0: 387F0300  addi r3, r31, 0x300
	ctx.r[3].s64 = ctx.r[31].s64 + 768;
	// 82313BB4: 993F02E0  stb r9, 0x2e0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(736 as u32), ctx.r[9].u8 ) };
	// 82313BB8: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82313BBC: C3CA08A8  lfs f30, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 82313BC0: D3FF02F0  stfs f31, 0x2f0(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(752 as u32), tmp.u32 ) };
	// 82313BC4: D3DF02F4  stfs f30, 0x2f4(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(756 as u32), tmp.u32 ) };
	// 82313BC8: D3FF02F8  stfs f31, 0x2f8(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(760 as u32), tmp.u32 ) };
	// 82313BCC: D3DF02FC  stfs f30, 0x2fc(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(764 as u32), tmp.u32 ) };
	// 82313BD0: 48B697A1  bl 0x82e7d370
	ctx.lr = 0x82313BD4;
	sub_82E7D370(ctx, base);
	// 82313BD4: D3FF05F0  stfs f31, 0x5f0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1520 as u32), tmp.u32 ) };
	// 82313BD8: D3DF05F4  stfs f30, 0x5f4(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1524 as u32), tmp.u32 ) };
	// 82313BDC: D3FF05F8  stfs f31, 0x5f8(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1528 as u32), tmp.u32 ) };
	// 82313BE0: D3DF05FC  stfs f30, 0x5fc(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1532 as u32), tmp.u32 ) };
	// 82313BE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82313BE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313BEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82313BF0: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82313BF4: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82313BF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82313BFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82313C00 size=128
    let mut pc: u32 = 0x82313C00;
    'dispatch: loop {
        match pc {
            0x82313C00 => {
    //   block [0x82313C00..0x82313C80)
	// 82313C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82313C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82313C08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82313C0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82313C10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313C14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313C18: 4BFFFF71  bl 0x82313b88
	ctx.lr = 0x82313C1C;
	sub_82313B88(ctx, base);
	// 82313C1C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82313C20: 387F02D0  addi r3, r31, 0x2d0
	ctx.r[3].s64 = ctx.r[31].s64 + 720;
	// 82313C24: 9BDF0508  stb r30, 0x508(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1288 as u32), ctx.r[30].u8 ) };
	// 82313C28: 48B69749  bl 0x82e7d370
	ctx.lr = 0x82313C2C;
	sub_82E7D370(ctx, base);
	// 82313C2C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82313C30: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82313C34: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 82313C38: 38896880  addi r4, r9, 0x6880
	ctx.r[4].s64 = ctx.r[9].s64 + 26752;
	// 82313C3C: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82313C40: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82313C44: D01F03D0  stfs f0, 0x3d0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(976 as u32), tmp.u32 ) };
	// 82313C48: D1BF03D4  stfs f13, 0x3d4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(980 as u32), tmp.u32 ) };
	// 82313C4C: D01F03D8  stfs f0, 0x3d8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(984 as u32), tmp.u32 ) };
	// 82313C50: D1BF03DC  stfs f13, 0x3dc(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(988 as u32), tmp.u32 ) };
	// 82313C54: 9BDF03C0  stb r30, 0x3c0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(960 as u32), ctx.r[30].u8 ) };
	// 82313C58: D01F0580  stfs f0, 0x580(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), tmp.u32 ) };
	// 82313C5C: 807F0588  lwz r3, 0x588(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1416 as u32) ) } as u64;
	// 82313C60: D01F0584  stfs f0, 0x584(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1412 as u32), tmp.u32 ) };
	// 82313C64: 48AFF5FD  bl 0x82e13260
	ctx.lr = 0x82313C68;
	sub_82E13260(ctx, base);
	// 82313C68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82313C6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313C70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82313C74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82313C78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82313C7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313C80 size=16
    let mut pc: u32 = 0x82313C80;
    'dispatch: loop {
        match pc {
            0x82313C80 => {
    //   block [0x82313C80..0x82313C90)
	// 82313C80: 39600220  li r11, 0x220
	ctx.r[11].s64 = 544;
	// 82313C84: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313C90 size=16
    let mut pc: u32 = 0x82313C90;
    'dispatch: loop {
        match pc {
            0x82313C90 => {
    //   block [0x82313C90..0x82313CA0)
	// 82313C90: 396005F0  li r11, 0x5f0
	ctx.r[11].s64 = 1520;
	// 82313C94: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313CA0 size=24
    let mut pc: u32 = 0x82313CA0;
    'dispatch: loop {
        match pc {
            0x82313CA0 => {
    //   block [0x82313CA0..0x82313CB8)
	// 82313CA0: 816404A0  lwz r11, 0x4a0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(1184 as u32) ) } as u64;
	// 82313CA4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82313CA8: 816404A4  lwz r11, 0x4a4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(1188 as u32) ) } as u64;
	// 82313CAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82313CB0: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82313CB4: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313CB8 size=36
    let mut pc: u32 = 0x82313CB8;
    'dispatch: loop {
        match pc {
            0x82313CB8 => {
    //   block [0x82313CB8..0x82313CDC)
	// 82313CB8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82313CBC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82313CC0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82313CC4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82313CC8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82313CCC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82313CD0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82313CD4: 4082FFE8  bne 0x82313cbc
	if !ctx.cr[0].eq {
	pc = 0x82313CBC; continue 'dispatch;
	}
	// 82313CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313CE0 size=8
    let mut pc: u32 = 0x82313CE0;
    'dispatch: loop {
        match pc {
            0x82313CE0 => {
    //   block [0x82313CE0..0x82313CE8)
	// 82313CE0: 806304AC  lwz r3, 0x4ac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82313CE4: 4801DA4C  b 0x82331730
	sub_82331730(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313CE8 size=8
    let mut pc: u32 = 0x82313CE8;
    'dispatch: loop {
        match pc {
            0x82313CE8 => {
    //   block [0x82313CE8..0x82313CF0)
	// 82313CE8: 806304AC  lwz r3, 0x4ac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82313CEC: 4801DA5C  b 0x82331748
	sub_82331748(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313CF0 size=8
    let mut pc: u32 = 0x82313CF0;
    'dispatch: loop {
        match pc {
            0x82313CF0 => {
    //   block [0x82313CF0..0x82313CF8)
	// 82313CF0: 806304AC  lwz r3, 0x4ac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82313CF4: 4801DA64  b 0x82331758
	sub_82331758(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313CF8 size=8
    let mut pc: u32 = 0x82313CF8;
    'dispatch: loop {
        match pc {
            0x82313CF8 => {
    //   block [0x82313CF8..0x82313D00)
	// 82313CF8: 806304AC  lwz r3, 0x4ac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82313CFC: 4801DA6C  b 0x82331768
	sub_82331768(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313D00 size=8
    let mut pc: u32 = 0x82313D00;
    'dispatch: loop {
        match pc {
            0x82313D00 => {
    //   block [0x82313D00..0x82313D08)
	// 82313D00: 806304AC  lwz r3, 0x4ac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82313D04: 4801DA84  b 0x82331788
	sub_82331788(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313D08 size=44
    let mut pc: u32 = 0x82313D08;
    'dispatch: loop {
        match pc {
            0x82313D08 => {
    //   block [0x82313D08..0x82313D34)
	// 82313D08: 81630498  lwz r11, 0x498(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1176 as u32) ) } as u64;
	// 82313D0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82313D10: 419A0024  beq cr6, 0x82313d34
	if ctx.cr[6].eq {
		sub_82313D34(ctx, base);
		return;
	}
	// 82313D14: 81630490  lwz r11, 0x490(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1168 as u32) ) } as u64;
	// 82313D18: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82313D1C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82313D20: 409A0014  bne cr6, 0x82313d34
	if !ctx.cr[6].eq {
		sub_82313D34(ctx, base);
		return;
	}
	// 82313D24: 816304A8  lwz r11, 0x4a8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1192 as u32) ) } as u64;
	// 82313D28: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82313D2C: 7F0B2840  cmplw cr6, r11, r5
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82313D30: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313D34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313D34 size=8
    let mut pc: u32 = 0x82313D34;
    'dispatch: loop {
        match pc {
            0x82313D34 => {
    //   block [0x82313D34..0x82313D3C)
	// 82313D34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82313D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82313D40 size=80
    let mut pc: u32 = 0x82313D40;
    'dispatch: loop {
        match pc {
            0x82313D40 => {
    //   block [0x82313D40..0x82313D90)
	// 82313D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82313D44: 48E94429  bl 0x831a816c
	ctx.lr = 0x82313D48;
	sub_831A8130(ctx, base);
	// 82313D48: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313D4C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82313D50: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82313D54: 4BFFFC75  bl 0x823139c8
	ctx.lr = 0x82313D58;
	sub_823139C8(ctx, base);
	// 82313D58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82313D5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82313D60: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82313D64: 48B6846D  bl 0x82e7c1d0
	ctx.lr = 0x82313D68;
	sub_82E7C1D0(ctx, base);
	// 82313D68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82313D6C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82313D70: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82313D74: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82313D90 size=124
    let mut pc: u32 = 0x82313D90;
    'dispatch: loop {
        match pc {
            0x82313D90 => {
    //   block [0x82313D90..0x82313E0C)
	// 82313D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82313D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82313D98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82313D9C: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82313DA0: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82313DA4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313DA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313DAC: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82313DB0: 38800027  li r4, 0x27
	ctx.r[4].s64 = 39;
	// 82313DB4: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82313DB8: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82313DBC: 4801D975  bl 0x82331730
	ctx.lr = 0x82313DC0;
	sub_82331730(ctx, base);
	// 82313DC0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82313DC4: 4182002C  beq 0x82313df0
	if ctx.cr[0].eq {
	pc = 0x82313DF0; continue 'dispatch;
	}
	// 82313DC8: 38800026  li r4, 0x26
	ctx.r[4].s64 = 38;
	// 82313DCC: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82313DD0: 4801D979  bl 0x82331748
	ctx.lr = 0x82313DD4;
	sub_82331748(ctx, base);
	// 82313DD4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82313DD8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82313DDC: D3FF0D14  stfs f31, 0xd14(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3348 as u32), tmp.u32 ) };
	// 82313DE0: D3DF0D18  stfs f30, 0xd18(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3352 as u32), tmp.u32 ) };
	// 82313DE4: 995F0D10  stb r10, 0xd10(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3344 as u32), ctx.r[10].u8 ) };
	// 82313DE8: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82313DEC: D01F0CF0  stfs f0, 0xcf0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(3312 as u32), tmp.u32 ) };
	// 82313DF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82313DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82313DFC: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82313E00: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82313E04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82313E08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313E10 size=20
    let mut pc: u32 = 0x82313E10;
    'dispatch: loop {
        match pc {
            0x82313E10 => {
    //   block [0x82313E10..0x82313E24)
	// 82313E10: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82313E14: 806304AC  lwz r3, 0x4ac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82313E18: 38800027  li r4, 0x27
	ctx.r[4].s64 = 39;
	// 82313E1C: 41820008  beq 0x82313e24
	if ctx.cr[0].eq {
		sub_82313E24(ctx, base);
		return;
	}
	// 82313E20: 4801D928  b 0x82331748
	sub_82331748(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313E24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313E24 size=4
    let mut pc: u32 = 0x82313E24;
    'dispatch: loop {
        match pc {
            0x82313E24 => {
    //   block [0x82313E24..0x82313E28)
	// 82313E24: 4801D934  b 0x82331758
	sub_82331758(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82313E28 size=100
    let mut pc: u32 = 0x82313E28;
    'dispatch: loop {
        match pc {
            0x82313E28 => {
    //   block [0x82313E28..0x82313E8C)
	// 82313E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82313E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82313E30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82313E34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313E38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313E3C: 897F0508  lbz r11, 0x508(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1288 as u32) ) } as u64;
	// 82313E40: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82313E44: 41820030  beq 0x82313e74
	if ctx.cr[0].eq {
	pc = 0x82313E74; continue 'dispatch;
	}
	// 82313E48: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82313E4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82313E50: 808BB3E8  lwz r4, -0x4c18(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19480 as u32) ) } as u64;
	// 82313E54: 48ADFBB5  bl 0x82df3a08
	ctx.lr = 0x82313E58;
	sub_82DF3A08(ctx, base);
	// 82313E58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82313E5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82313E60: 4BFF1731  bl 0x82305590
	ctx.lr = 0x82313E64;
	sub_82305590(ctx, base);
	// 82313E64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82313E68: 48ADF5C1  bl 0x82df3428
	ctx.lr = 0x82313E6C;
	sub_82DF3428(ctx, base);
	// 82313E6C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82313E70: 48000008  b 0x82313e78
	pc = 0x82313E78; continue 'dispatch;
	// 82313E74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82313E78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82313E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82313E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82313E84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82313E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313E90 size=16
    let mut pc: u32 = 0x82313E90;
    'dispatch: loop {
        match pc {
            0x82313E90 => {
    //   block [0x82313E90..0x82313EA0)
	// 82313E90: 396003F0  li r11, 0x3f0
	ctx.r[11].s64 = 1008;
	// 82313E94: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82313EA0 size=12
    let mut pc: u32 = 0x82313EA0;
    'dispatch: loop {
        match pc {
            0x82313EA0 => {
    //   block [0x82313EA0..0x82313EAC)
	// 82313EA0: 806301FC  lwz r3, 0x1fc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(508 as u32) ) } as u64;
	// 82313EA4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82313EA8: 48146F40  b 0x8245ade8
	sub_8245ADE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313EB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82313EB0 size=176
    let mut pc: u32 = 0x82313EB0;
    'dispatch: loop {
        match pc {
            0x82313EB0 => {
    //   block [0x82313EB0..0x82313F60)
	// 82313EB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82313EB4: 48E942B9  bl 0x831a816c
	ctx.lr = 0x82313EB8;
	sub_831A8130(ctx, base);
	// 82313EB8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313EBC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82313EC0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82313EC4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82313EC8: 897F04B4  lbz r11, 0x4b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1204 as u32) ) } as u64;
	// 82313ECC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82313ED0: 4182007C  beq 0x82313f4c
	if ctx.cr[0].eq {
	pc = 0x82313F4C; continue 'dispatch;
	}
	// 82313ED4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82313ED8: 807F0498  lwz r3, 0x498(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1176 as u32) ) } as u64;
	// 82313EDC: 4BFBC1D5  bl 0x822d00b0
	ctx.lr = 0x82313EE0;
	sub_822D00B0(ctx, base);
	// 82313EE0: C1BE0004  lfs f13, 4(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82313EE4: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 82313EE8: 3D608209  lis r11, -0x7df7
	ctx.r[11].s64 = -2113339392;
	// 82313EEC: C00BA100  lfs f0, -0x5f00(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24320 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82313EF0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82313EF4: 40990010  ble cr6, 0x82313f04
	if !ctx.cr[6].gt {
	pc = 0x82313F04; continue 'dispatch;
	}
	// 82313EF8: 39600470  li r11, 0x470
	ctx.r[11].s64 = 1136;
	// 82313EFC: 13FF58C7  vcmpequd (lvx128) v31, v31, v11
	tmp.u32 = ctx.r[31].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82313F00: 48000008  b 0x82313f08
	pc = 0x82313F08; continue 'dispatch;
	// 82313F04: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82313F08: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82313F0C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82313F10: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82313F14: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313F60 size=16
    let mut pc: u32 = 0x82313F60;
    'dispatch: loop {
        match pc {
            0x82313F60 => {
    //   block [0x82313F60..0x82313F70)
	// 82313F60: 39600410  li r11, 0x410
	ctx.r[11].s64 = 1040;
	// 82313F64: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313F70 size=16
    let mut pc: u32 = 0x82313F70;
    'dispatch: loop {
        match pc {
            0x82313F70 => {
    //   block [0x82313F70..0x82313F80)
	// 82313F70: 39600300  li r11, 0x300
	ctx.r[11].s64 = 768;
	// 82313F74: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313F80 size=16
    let mut pc: u32 = 0x82313F80;
    'dispatch: loop {
        match pc {
            0x82313F80 => {
    //   block [0x82313F80..0x82313F90)
	// 82313F80: 39600310  li r11, 0x310
	ctx.r[11].s64 = 784;
	// 82313F84: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313F90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313F90 size=16
    let mut pc: u32 = 0x82313F90;
    'dispatch: loop {
        match pc {
            0x82313F90 => {
    //   block [0x82313F90..0x82313FA0)
	// 82313F90: 39600320  li r11, 0x320
	ctx.r[11].s64 = 800;
	// 82313F94: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82313FA0 size=16
    let mut pc: u32 = 0x82313FA0;
    'dispatch: loop {
        match pc {
            0x82313FA0 => {
    //   block [0x82313FA0..0x82313FB0)
	// 82313FA0: 39600330  li r11, 0x330
	ctx.r[11].s64 = 816;
	// 82313FA4: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82313FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82313FB0 size=108
    let mut pc: u32 = 0x82313FB0;
    'dispatch: loop {
        match pc {
            0x82313FB0 => {
    //   block [0x82313FB0..0x8231401C)
	// 82313FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82313FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82313FB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82313FBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82313FC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82313FC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82313FC8: 807F0100  lwz r3, 0x100(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 82313FCC: 4BFEC845  bl 0x82300810
	ctx.lr = 0x82313FD0;
	sub_82300810(ctx, base);
	// 82313FD0: 3880001B  li r4, 0x1b
	ctx.r[4].s64 = 27;
	// 82313FD4: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82313FD8: 4801D771  bl 0x82331748
	ctx.lr = 0x82313FDC;
	sub_82331748(ctx, base);
	// 82313FDC: 38800034  li r4, 0x34
	ctx.r[4].s64 = 52;
	// 82313FE0: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82313FE4: 4801D775  bl 0x82331758
	ctx.lr = 0x82313FE8;
	sub_82331758(ctx, base);
	// 82313FE8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82313FEC: 807F0588  lwz r3, 0x588(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1416 as u32) ) } as u64;
	// 82313FF0: 388B6880  addi r4, r11, 0x6880
	ctx.r[4].s64 = ctx.r[11].s64 + 26752;
	// 82313FF4: 48AFF26D  bl 0x82e13260
	ctx.lr = 0x82313FF8;
	sub_82E13260(ctx, base);
	// 82313FF8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82313FFC: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82314000: D01F054C  stfs f0, 0x54c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1356 as u32), tmp.u32 ) };
	// 82314004: D01F0550  stfs f0, 0x550(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1360 as u32), tmp.u32 ) };
	// 82314008: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8231400C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82314010: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82314014: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82314018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314020 size=112
    let mut pc: u32 = 0x82314020;
    'dispatch: loop {
        match pc {
            0x82314020 => {
    //   block [0x82314020..0x82314090)
	// 82314020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314028: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8231402C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314030: 896404B4  lbz r11, 0x4b4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(1204 as u32) ) } as u64;
	// 82314034: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82314038: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8231403C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82314040: 41820010  beq 0x82314050
	if ctx.cr[0].eq {
	pc = 0x82314050; continue 'dispatch;
	}
	// 82314044: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82314048: 806B26CC  lwz r3, 0x26cc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9932 as u32) ) } as u64;
	// 8231404C: 4800000C  b 0x82314058
	pc = 0x82314058; continue 'dispatch;
	// 82314050: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82314054: 806B26C4  lwz r3, 0x26c4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9924 as u32) ) } as u64;
	// 82314058: 48146D91  bl 0x8245ade8
	ctx.lr = 0x8231405C;
	sub_8245ADE8(ctx, base);
	// 8231405C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82314060: FC000850  fneg f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 82314064: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82314068: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8231406C: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82314070: D01F000C  stfs f0, 0xc(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82314074: D01F0008  stfs f0, 8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82314078: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8231407C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82314080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82314084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82314088: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8231408C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314090 size=88
    let mut pc: u32 = 0x82314090;
    'dispatch: loop {
        match pc {
            0x82314090 => {
    //   block [0x82314090..0x823140E8)
	// 82314090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314098: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8231409C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823140A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823140A4: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823140A8: 387F04D0  addi r3, r31, 0x4d0
	ctx.r[3].s64 = ctx.r[31].s64 + 1232;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823140E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823140E8 size=8
    let mut pc: u32 = 0x823140E8;
    'dispatch: loop {
        match pc {
            0x823140E8 => {
    //   block [0x823140E8..0x823140F0)
	// 823140E8: 806306E0  lwz r3, 0x6e0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1760 as u32) ) } as u64;
	// 823140EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823140F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823140F0 size=188
    let mut pc: u32 = 0x823140F0;
    'dispatch: loop {
        match pc {
            0x823140F0 => {
    //   block [0x823140F0..0x823141AC)
	// 823140F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823140F4: 48E9406D  bl 0x831a8160
	ctx.lr = 0x823140F8;
	sub_831A8130(ctx, base);
	// 823140F8: DBA1FFB0  stfd f29, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[29].u64 ) };
	// 823140FC: DBC1FFB8  stfd f30, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[30].u64 ) };
	// 82314100: DBE1FFC0  stfd f31, -0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 82314104: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314108: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8231410C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82314110: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82314114: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82314118: 396BA2F0  addi r11, r11, -0x5d10
	ctx.r[11].s64 = ctx.r[11].s64 + -23824;
	// 8231411C: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 82314120: 394003D0  li r10, 0x3d0
	ctx.r[10].s64 = 976;
	// 82314124: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82314128: 3D208326  lis r9, -0x7cda
	ctx.r[9].s64 = -2094661632;
	// 8231412C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82314130: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82314134: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823141B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823141B0 size=8
    let mut pc: u32 = 0x823141B0;
    'dispatch: loop {
        match pc {
            0x823141B0 => {
    //   block [0x823141B0..0x823141B8)
	// 823141B0: 806306EC  lwz r3, 0x6ec(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1772 as u32) ) } as u64;
	// 823141B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823141B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823141B8 size=228
    let mut pc: u32 = 0x823141B8;
    'dispatch: loop {
        match pc {
            0x823141B8 => {
    //   block [0x823141B8..0x8231429C)
	// 823141B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823141BC: 48E93FB1  bl 0x831a816c
	ctx.lr = 0x823141C0;
	sub_831A8130(ctx, base);
	// 823141C0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823141C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823141C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823141CC: 38800029  li r4, 0x29
	ctx.r[4].s64 = 41;
	// 823141D0: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823141D4: 4801D55D  bl 0x82331730
	ctx.lr = 0x823141D8;
	sub_82331730(ctx, base);
	// 823141D8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823141DC: 408200B8  bne 0x82314294
	if !ctx.cr[0].eq {
	pc = 0x82314294; continue 'dispatch;
	}
	// 823141E0: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 823141E4: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823141E8: 4801D549  bl 0x82331730
	ctx.lr = 0x823141EC;
	sub_82331730(ctx, base);
	// 823141EC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823141F0: 408200A4  bne 0x82314294
	if !ctx.cr[0].eq {
	pc = 0x82314294; continue 'dispatch;
	}
	// 823141F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823141F8: 889F04B4  lbz r4, 0x4b4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1204 as u32) ) } as u64;
	// 823141FC: 4BFFED7D  bl 0x82312f78
	ctx.lr = 0x82314200;
	sub_82312F78(ctx, base);
	// 82314200: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82314204: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314208: 808BB3AC  lwz r4, -0x4c54(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19540 as u32) ) } as u64;
	// 8231420C: 48ADF7FD  bl 0x82df3a08
	ctx.lr = 0x82314210;
	sub_82DF3A08(ctx, base);
	// 82314210: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82314214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82314218: 4BFF1379  bl 0x82305590
	ctx.lr = 0x8231421C;
	sub_82305590(ctx, base);
	// 8231421C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314220: 48ADF209  bl 0x82df3428
	ctx.lr = 0x82314224;
	sub_82DF3428(ctx, base);
	// 82314224: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82314228: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8231422C: 808BB4D8  lwz r4, -0x4b28(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19240 as u32) ) } as u64;
	// 82314230: 48ADF7D9  bl 0x82df3a08
	ctx.lr = 0x82314234;
	sub_82DF3A08(ctx, base);
	// 82314234: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82314238: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8231423C: 4BFF12ED  bl 0x82305528
	ctx.lr = 0x82314240;
	sub_82305528(ctx, base);
	// 82314240: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82314244: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314248: 48ADF1E1  bl 0x82df3428
	ctx.lr = 0x8231424C;
	sub_82DF3428(ctx, base);
	// 8231424C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82314250: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82314254: 4802C35D  bl 0x823405b0
	ctx.lr = 0x82314258;
	sub_823405B0(ctx, base);
	// 82314258: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8231425C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314260: 388BD84C  addi r4, r11, -0x27b4
	ctx.r[4].s64 = ctx.r[11].s64 + -10164;
	// 82314264: 48ADF7A5  bl 0x82df3a08
	ctx.lr = 0x82314268;
	sub_82DF3A08(ctx, base);
	// 82314268: 38C0000A  li r6, 0xa
	ctx.r[6].s64 = 10;
	// 8231426C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82314270: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82314274: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82314278: 4BFF4E21  bl 0x82309098
	ctx.lr = 0x8231427C;
	sub_82309098(ctx, base);
	// 8231427C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82314280: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82314284: 419A0008  beq cr6, 0x8231428c
	if ctx.cr[6].eq {
	pc = 0x8231428C; continue 'dispatch;
	}
	// 82314288: 4BFAC609  bl 0x822c0890
	ctx.lr = 0x8231428C;
	sub_822C0890(ctx, base);
	// 8231428C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314290: 48ADF199  bl 0x82df3428
	ctx.lr = 0x82314294;
	sub_82DF3428(ctx, base);
	// 82314294: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82314298: 48E93F24  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823142A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823142A0 size=132
    let mut pc: u32 = 0x823142A0;
    'dispatch: loop {
        match pc {
            0x823142A0 => {
    //   block [0x823142A0..0x82314324)
	// 823142A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823142A4: 48E93EC9  bl 0x831a816c
	ctx.lr = 0x823142A8;
	sub_831A8130(ctx, base);
	// 823142A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823142AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823142B0: 38800029  li r4, 0x29
	ctx.r[4].s64 = 41;
	// 823142B4: 807E04AC  lwz r3, 0x4ac(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823142B8: 4801D479  bl 0x82331730
	ctx.lr = 0x823142BC;
	sub_82331730(ctx, base);
	// 823142BC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823142C0: 4082005C  bne 0x8231431c
	if !ctx.cr[0].eq {
	pc = 0x8231431C; continue 'dispatch;
	}
	// 823142C4: 3FE08326  lis r31, -0x7cda
	ctx.r[31].s64 = -2094661632;
	// 823142C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823142CC: 809FB3D8  lwz r4, -0x4c28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-19496 as u32) ) } as u64;
	// 823142D0: 48ADF739  bl 0x82df3a08
	ctx.lr = 0x823142D4;
	sub_82DF3A08(ctx, base);
	// 823142D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823142D8: 3BA10050  addi r29, r1, 0x50
	ctx.r[29].s64 = ctx.r[1].s64 + 80;
	// 823142DC: 4BFF12BD  bl 0x82305598
	ctx.lr = 0x823142E0;
	sub_82305598(ctx, base);
	// 823142E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823142E4: 48ADEFBD  bl 0x82df32a0
	ctx.lr = 0x823142E8;
	sub_82DF32A0(ctx, base);
	// 823142E8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823142EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823142F0: 48ADF139  bl 0x82df3428
	ctx.lr = 0x823142F4;
	sub_82DF3428(ctx, base);
	// 823142F4: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823142F8: 41820024  beq 0x8231431c
	if ctx.cr[0].eq {
	pc = 0x8231431C; continue 'dispatch;
	}
	// 823142FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314300: 809FB3D8  lwz r4, -0x4c28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-19496 as u32) ) } as u64;
	// 82314304: 48ADF705  bl 0x82df3a08
	ctx.lr = 0x82314308;
	sub_82DF3A08(ctx, base);
	// 82314308: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8231430C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82314310: 4BFF1281  bl 0x82305590
	ctx.lr = 0x82314314;
	sub_82305590(ctx, base);
	// 82314314: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314318: 48ADF111  bl 0x82df3428
	ctx.lr = 0x8231431C;
	sub_82DF3428(ctx, base);
	// 8231431C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82314320: 48E93E9C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82314328 size=12
    let mut pc: u32 = 0x82314328;
    'dispatch: loop {
        match pc {
            0x82314328 => {
    //   block [0x82314328..0x82314334)
	// 82314328: 806304AC  lwz r3, 0x4ac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1196 as u32) ) } as u64;
	// 8231432C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82314330: 4801D400  b 0x82331730
	sub_82331730(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314338 size=56
    let mut pc: u32 = 0x82314338;
    'dispatch: loop {
        match pc {
            0x82314338 => {
    //   block [0x82314338..0x82314370)
	// 82314338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231433C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314340: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82314344: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314348: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8231434C: 38800054  li r4, 0x54
	ctx.r[4].s64 = 84;
	// 82314350: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82314354: 48146A95  bl 0x8245ade8
	ctx.lr = 0x82314358;
	sub_8245ADE8(ctx, base);
	// 82314358: D03F04E4  stfs f1, 0x4e4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1252 as u32), tmp.u32 ) };
	// 8231435C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82314360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82314364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82314368: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8231436C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82314370 size=80
    let mut pc: u32 = 0x82314370;
    'dispatch: loop {
        match pc {
            0x82314370 => {
    //   block [0x82314370..0x823143C0)
	// 82314370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8231437C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82314380: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314384: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82314388: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8231438C: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82314390: 48146A59  bl 0x8245ade8
	ctx.lr = 0x82314394;
	sub_8245ADE8(ctx, base);
	// 82314394: 38800031  li r4, 0x31
	ctx.r[4].s64 = 49;
	// 82314398: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 8231439C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823143A0: 48146A49  bl 0x8245ade8
	ctx.lr = 0x823143A4;
	sub_8245ADE8(ctx, base);
	// 823143A4: EC3F0824  fdivs f1, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ((ctx.f[31].f64 / ctx.f[1].f64) as f32) as f64;
	// 823143A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823143AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823143B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823143B4: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823143B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823143BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823143C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823143C0 size=48
    let mut pc: u32 = 0x823143C0;
    'dispatch: loop {
        match pc {
            0x823143C0 => {
    //   block [0x823143C0..0x823143F0)
	// 823143C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823143C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823143C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823143CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823143D0: C00BCEE4  lfs f0, -0x311c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12572 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823143D4: EC210032  fmuls f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 823143D8: 48E975C1  bl 0x831ab998
	ctx.lr = 0x823143DC;
	sub_831AB998(ctx, base);
	// 823143DC: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 823143E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823143E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823143E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823143EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823143F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823143F0 size=28
    let mut pc: u32 = 0x823143F0;
    'dispatch: loop {
        match pc {
            0x823143F0 => {
    //   block [0x823143F0..0x8231440C)
	// 823143F0: C0030584  lfs f0, 0x584(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1412 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823143F4: D023050C  stfs f1, 0x50c(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1292 as u32), tmp.u32 ) };
	// 823143F8: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 823143FC: 40980008  bge cr6, 0x82314404
	if !ctx.cr[6].lt {
	pc = 0x82314404; continue 'dispatch;
	}
	// 82314400: FC000890  fmr f0, f1
	ctx.f[0].f64 = ctx.f[1].f64;
	// 82314404: D0030584  stfs f0, 0x584(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1412 as u32), tmp.u32 ) };
	// 82314408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82314410 size=204
    let mut pc: u32 = 0x82314410;
    'dispatch: loop {
        match pc {
            0x82314410 => {
    //   block [0x82314410..0x823144DC)
	// 82314410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314418: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8231441C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314420: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82314424: 548B063F  clrlwi. r11, r4, 0x18
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82314428: 40820010  bne 0x82314438
	if !ctx.cr[0].eq {
	pc = 0x82314438; continue 'dispatch;
	}
	// 8231442C: 38800017  li r4, 0x17
	ctx.r[4].s64 = 23;
	// 82314430: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82314434: 4801D315  bl 0x82331748
	ctx.lr = 0x82314438;
	sub_82331748(ctx, base);
	// 82314438: 38800016  li r4, 0x16
	ctx.r[4].s64 = 22;
	// 8231443C: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82314440: 4801D2F1  bl 0x82331730
	ctx.lr = 0x82314444;
	sub_82331730(ctx, base);
	// 82314444: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82314448: 40820080  bne 0x823144c8
	if !ctx.cr[0].eq {
	pc = 0x823144C8; continue 'dispatch;
	}
	// 8231444C: 38800016  li r4, 0x16
	ctx.r[4].s64 = 22;
	// 82314450: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82314454: 4801D2F5  bl 0x82331748
	ctx.lr = 0x82314458;
	sub_82331748(ctx, base);
	// 82314458: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8231445C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314460: 808BB550  lwz r4, -0x4ab0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19120 as u32) ) } as u64;
	// 82314464: 48ADF5A5  bl 0x82df3a08
	ctx.lr = 0x82314468;
	sub_82DF3A08(ctx, base);
	// 82314468: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8231446C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82314470: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82314474: 4BFF3765  bl 0x82307bd8
	ctx.lr = 0x82314478;
	sub_82307BD8(ctx, base);
	// 82314478: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8231447C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82314480: 419A0008  beq cr6, 0x82314488
	if ctx.cr[6].eq {
	pc = 0x82314488; continue 'dispatch;
	}
	// 82314484: 4BFAC40D  bl 0x822c0890
	ctx.lr = 0x82314488;
	sub_822C0890(ctx, base);
	// 82314488: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8231448C: 48ADEF9D  bl 0x82df3428
	ctx.lr = 0x82314490;
	sub_82DF3428(ctx, base);
	// 82314490: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82314494: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82314498: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8231449C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823144A0: 4E800421  bctrl
	ctx.lr = 0x823144A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823144A4: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823144A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823144AC: 808BB4C8  lwz r4, -0x4b38(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19256 as u32) ) } as u64;
	// 823144B0: 48ADF559  bl 0x82df3a08
	ctx.lr = 0x823144B4;
	sub_82DF3A08(ctx, base);
	// 823144B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823144B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823144BC: 4BFF106D  bl 0x82305528
	ctx.lr = 0x823144C0;
	sub_82305528(ctx, base);
	// 823144C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823144C4: 48ADEF65  bl 0x82df3428
	ctx.lr = 0x823144C8;
	sub_82DF3428(ctx, base);
	// 823144C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823144CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823144D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823144D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823144D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823144E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823144E0 size=92
    let mut pc: u32 = 0x823144E0;
    'dispatch: loop {
        match pc {
            0x823144E0 => {
    //   block [0x823144E0..0x8231453C)
	// 823144E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823144E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823144E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823144EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823144F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823144F4: 38800016  li r4, 0x16
	ctx.r[4].s64 = 22;
	// 823144F8: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823144FC: 4801D25D  bl 0x82331758
	ctx.lr = 0x82314500;
	sub_82331758(ctx, base);
	// 82314500: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82314504: 806B26D4  lwz r3, 0x26d4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9940 as u32) ) } as u64;
	// 82314508: 48145DB1  bl 0x8245a2b8
	ctx.lr = 0x8231450C;
	sub_8245A2B8(ctx, base);
	// 8231450C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82314510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82314514: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82314518: 4BFF49C9  bl 0x82308ee0
	ctx.lr = 0x8231451C;
	sub_82308EE0(ctx, base);
	// 8231451C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82314520: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82314524: D01F0514  stfs f0, 0x514(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1300 as u32), tmp.u32 ) };
	// 82314528: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8231452C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82314530: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82314534: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82314538: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314540 size=280
    let mut pc: u32 = 0x82314540;
    'dispatch: loop {
        match pc {
            0x82314540 => {
    //   block [0x82314540..0x82314658)
	// 82314540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314548: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8231454C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82314550: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82314554: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314558: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8231455C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82314560: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82314564: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82314568: 4801D1C9  bl 0x82331730
	ctx.lr = 0x8231456C;
	sub_82331730(ctx, base);
	// 8231456C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82314570: 408200C8  bne 0x82314638
	if !ctx.cr[0].eq {
	pc = 0x82314638; continue 'dispatch;
	}
	// 82314574: 396003D0  li r11, 0x3d0
	ctx.r[11].s64 = 976;
	// 82314578: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8231457C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82314580: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82314584: 38800073  li r4, 0x73
	ctx.r[4].s64 = 115;
	// 82314588: 13DF58C7  vcmpequd (lvx128) v30, v31, v11
	tmp.u32 = ctx.r[31].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314658 size=296
    let mut pc: u32 = 0x82314658;
    'dispatch: loop {
        match pc {
            0x82314658 => {
    //   block [0x82314658..0x82314780)
	// 82314658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231465C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314660: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82314664: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82314668: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8231466C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82314670: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82314674: 897E0C90  lbz r11, 0xc90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(3216 as u32) ) } as u64;
	// 82314678: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8231467C: 408200EC  bne 0x82314768
	if !ctx.cr[0].eq {
	pc = 0x82314768; continue 'dispatch;
	}
	// 82314680: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82314684: 13E0F8C7  vcmpequd (lvx128) v31, v0, v31
	tmp.u32 = ctx.r[31].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82314688: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 8231468C: 396BA2F0  addi r11, r11, -0x5d10
	ctx.r[11].s64 = ctx.r[11].s64 + -23824;
	// 82314690: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82314694: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314780 size=220
    let mut pc: u32 = 0x82314780;
    'dispatch: loop {
        match pc {
            0x82314780 => {
    //   block [0x82314780..0x8231485C)
	// 82314780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314784: 48E939E5  bl 0x831a8168
	ctx.lr = 0x82314788;
	sub_831A8130(ctx, base);
	// 82314788: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8231478C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82314790: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82314794: 397C001C  addi r11, r28, 0x1c
	ctx.r[11].s64 = ctx.r[28].s64 + 28;
	// 82314798: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8231479C: 556B3032  slwi r11, r11, 6
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 823147A0: 7D6BF8AE  lbzx r11, r11, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 823147A4: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823147A8: 41820094  beq 0x8231483c
	if ctx.cr[0].eq {
	pc = 0x8231483C; continue 'dispatch;
	}
	// 823147AC: 395D001C  addi r10, r29, 0x1c
	ctx.r[10].s64 = ctx.r[29].s64 + 28;
	// 823147B0: 554A3032  slwi r10, r10, 6
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823147B4: 7D4AF8AE  lbzx r10, r10, r31
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 823147B8: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823147BC: 41820080  beq 0x8231483c
	if ctx.cr[0].eq {
	pc = 0x8231483C; continue 'dispatch;
	}
	// 823147C0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 823147C4: 3FC08335  lis r30, -0x7ccb
	ctx.r[30].s64 = -2093678592;
	// 823147C8: 816A2700  lwz r11, 0x2700(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(9984 as u32) ) } as u64;
	// 823147CC: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 823147D0: 40820024  bne 0x823147f4
	if !ctx.cr[0].eq {
	pc = 0x823147F4; continue 'dispatch;
	}
	// 823147D4: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 823147D8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 823147DC: 916A2700  stw r11, 0x2700(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(9984 as u32), ctx.r[11].u32 ) };
	// 823147E0: C829D858  lfd f1, -0x27a8(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(-10152 as u32) ) };
	// 823147E4: 48E946C5  bl 0x831a8ea8
	ctx.lr = 0x823147E8;
	sub_831A8EA8(ctx, base);
	// 823147E8: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 823147EC: D01E26FC  stfs f0, 0x26fc(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(9980 as u32), tmp.u32 ) };
	// 823147F0: 48000008  b 0x823147f8
	pc = 0x823147F8; continue 'dispatch;
	// 823147F4: C01E26FC  lfs f0, 0x26fc(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(9980 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823147F8: 57AA3032  slwi r10, r29, 6
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(6);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 823147FC: 578B3032  slwi r11, r28, 6
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(6);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82314800: 39200710  li r9, 0x710
	ctx.r[9].s64 = 1808;
	// 82314804: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 82314808: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8231480C: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82314810: 13EA48C7  vcmpequd (lvx128) v31, v10, v9
	tmp.u32 = ctx.r[10].u32 + ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82314814: 13CB48C7  vcmpequd (lvx128) v30, v11, v9
	tmp.u32 = ctx.r[11].u32 + ctx.r[9].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82314860 size=16
    let mut pc: u32 = 0x82314860;
    'dispatch: loop {
        match pc {
            0x82314860 => {
    //   block [0x82314860..0x82314870)
	// 82314860: 396006D0  li r11, 0x6d0
	ctx.r[11].s64 = 1744;
	// 82314864: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314870 size=176
    let mut pc: u32 = 0x82314870;
    'dispatch: loop {
        match pc {
            0x82314870 => {
    //   block [0x82314870..0x82314920)
	// 82314870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314874: 48E938F9  bl 0x831a816c
	ctx.lr = 0x82314878;
	sub_831A8130(ctx, base);
	// 82314878: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8231487C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82314880: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82314884: 38800009  li r4, 9
	ctx.r[4].s64 = 9;
	// 82314888: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8231488C: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82314890: 4801CEA1  bl 0x82331730
	ctx.lr = 0x82314894;
	sub_82331730(ctx, base);
	// 82314894: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82314898: 4182000C  beq 0x823148a4
	if ctx.cr[0].eq {
	pc = 0x823148A4; continue 'dispatch;
	}
	// 8231489C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 823148A0: 48000078  b 0x82314918
	pc = 0x82314918; continue 'dispatch;
	// 823148A4: 897F03C0  lbz r11, 0x3c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(960 as u32) ) } as u64;
	// 823148A8: 38800073  li r4, 0x73
	ctx.r[4].s64 = 115;
	// 823148AC: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 823148B0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823148B4: 41820024  beq 0x823148d8
	if ctx.cr[0].eq {
	pc = 0x823148D8; continue 'dispatch;
	}
	// 823148B8: 48146531  bl 0x8245ade8
	ctx.lr = 0x823148BC;
	sub_8245ADE8(ctx, base);
	// 823148BC: 3D608208  lis r11, -0x7df8
	ctx.r[11].s64 = -2113404928;
	// 823148C0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 823148C4: C00B614C  lfs f0, 0x614c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24908 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823148C8: EDA1002A  fadds f13, f1, f0
	ctx.f[13].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 823148CC: C00ACEE4  lfs f0, -0x311c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-12572 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823148D0: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 823148D4: 48000014  b 0x823148e8
	pc = 0x823148E8; continue 'dispatch;
	// 823148D8: 48146511  bl 0x8245ade8
	ctx.lr = 0x823148DC;
	sub_8245ADE8(ctx, base);
	// 823148DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823148E0: C00BCEE4  lfs f0, -0x311c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12572 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823148E4: EC210032  fmuls f1, f1, f0
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 823148E8: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823148EC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 823148F0: 13C0F0C7  vcmpequd (lvx128) v30, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314920 size=136
    let mut pc: u32 = 0x82314920;
    'dispatch: loop {
        match pc {
            0x82314920 => {
    //   block [0x82314920..0x823149A8)
	// 82314920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314924: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314928: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8231492C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82314930: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314934: 806301FC  lwz r3, 0x1fc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(508 as u32) ) } as u64;
	// 82314938: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 8231493C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82314940: 552B063F  clrlwi. r11, r9, 0x18
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82314944: 38800078  li r4, 0x78
	ctx.r[4].s64 = 120;
	// 82314948: 40820008  bne 0x82314950
	if !ctx.cr[0].eq {
	pc = 0x82314950; continue 'dispatch;
	}
	// 8231494C: 38800079  li r4, 0x79
	ctx.r[4].s64 = 121;
	// 82314950: 48146499  bl 0x8245ade8
	ctx.lr = 0x82314954;
	sub_8245ADE8(ctx, base);
	// 82314954: 13E0F8C7  vcmpequd (lvx128) v31, v0, v31
	tmp.u32 = ctx.r[31].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82314958: 13C0F0C7  vcmpequd (lvx128) v30, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8231495C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823149A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823149A8 size=12
    let mut pc: u32 = 0x823149A8;
    'dispatch: loop {
        match pc {
            0x823149A8 => {
    //   block [0x823149A8..0x823149B4)
	// 823149A8: 81630CC8  lwz r11, 0xcc8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3272 as u32) ) } as u64;
	// 823149AC: 386B0001  addi r3, r11, 1
	ctx.r[3].s64 = ctx.r[11].s64 + 1;
	// 823149B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823149B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823149B8 size=8
    let mut pc: u32 = 0x823149B8;
    'dispatch: loop {
        match pc {
            0x823149B8 => {
    //   block [0x823149B8..0x823149C0)
	// 823149B8: 806304B0  lwz r3, 0x4b0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1200 as u32) ) } as u64;
	// 823149BC: 4801CD74  b 0x82331730
	sub_82331730(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823149C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823149C0 size=8
    let mut pc: u32 = 0x823149C0;
    'dispatch: loop {
        match pc {
            0x823149C0 => {
    //   block [0x823149C0..0x823149C8)
	// 823149C0: 806304B0  lwz r3, 0x4b0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1200 as u32) ) } as u64;
	// 823149C4: 4801CD84  b 0x82331748
	sub_82331748(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823149C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823149C8 size=16
    let mut pc: u32 = 0x823149C8;
    'dispatch: loop {
        match pc {
            0x823149C8 => {
    //   block [0x823149C8..0x823149D8)
	// 823149C8: 39600120  li r11, 0x120
	ctx.r[11].s64 = 288;
	// 823149CC: 13E358C7  vcmpequd (lvx128) v31, v3, v11
	tmp.u32 = ctx.r[3].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823149D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823149D8 size=40
    let mut pc: u32 = 0x823149D8;
    'dispatch: loop {
        match pc {
            0x823149D8 => {
    //   block [0x823149D8..0x82314A00)
	// 823149D8: E9630CCA  lwa r11, 0xcc8(r3)
	ctx.r[11].s64 = (unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3272 as u32) ) } as i32) as i64;
	// 823149DC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823149E0: F961FFF0  std r11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[11].u64 ) };
	// 823149E4: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823149E8: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 823149EC: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823149F0: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 823149F4: EDAD08BC  fnmsubs f13, f13, f2, f1
	ctx.f[13].f64 = -(((ctx.f[13].f64 * ctx.f[2].f64 - ctx.f[1].f64) as f32) as f64);
	// 823149F8: FC2D036E  fsel f1, f13, f13, f0
	ctx.f[1].f64 = if ctx.f[13].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 823149FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82314A00 size=12
    let mut pc: u32 = 0x82314A00;
    'dispatch: loop {
        match pc {
            0x82314A00 => {
    //   block [0x82314A00..0x82314A0C)
	// 82314A00: 806304B0  lwz r3, 0x4b0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1200 as u32) ) } as u64;
	// 82314A04: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82314A08: 4801CD40  b 0x82331748
	sub_82331748(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82314A10 size=12
    let mut pc: u32 = 0x82314A10;
    'dispatch: loop {
        match pc {
            0x82314A10 => {
    //   block [0x82314A10..0x82314A1C)
	// 82314A10: 806304AC  lwz r3, 0x4ac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82314A14: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82314A18: 4801CD30  b 0x82331748
	sub_82331748(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82314A20 size=12
    let mut pc: u32 = 0x82314A20;
    'dispatch: loop {
        match pc {
            0x82314A20 => {
    //   block [0x82314A20..0x82314A2C)
	// 82314A20: 806304AC  lwz r3, 0x4ac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82314A24: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 82314A28: 4801CD20  b 0x82331748
	sub_82331748(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314A30 size=216
    let mut pc: u32 = 0x82314A30;
    'dispatch: loop {
        match pc {
            0x82314A30 => {
    //   block [0x82314A30..0x82314B08)
	// 82314A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314A34: 48E93739  bl 0x831a816c
	ctx.lr = 0x82314A38;
	sub_831A8130(ctx, base);
	// 82314A38: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82314A3C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314A40: 3FA08326  lis r29, -0x7cda
	ctx.r[29].s64 = -2094661632;
	// 82314A44: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82314A48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82314A4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314A50: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82314A54: 809DB55C  lwz r4, -0x4aa4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19108 as u32) ) } as u64;
	// 82314A58: 48ADEFB1  bl 0x82df3a08
	ctx.lr = 0x82314A5C;
	sub_82DF3A08(ctx, base);
	// 82314A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82314A60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82314A64: 4BFF0B15  bl 0x82305578
	ctx.lr = 0x82314A68;
	sub_82305578(ctx, base);
	// 82314A68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314A6C: 48ADE9BD  bl 0x82df3428
	ctx.lr = 0x82314A70;
	sub_82DF3428(ctx, base);
	// 82314A70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314A74: 809DB55C  lwz r4, -0x4aa4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19108 as u32) ) } as u64;
	// 82314A78: 48ADEF91  bl 0x82df3a08
	ctx.lr = 0x82314A7C;
	sub_82DF3A08(ctx, base);
	// 82314A7C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82314A80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82314A84: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82314A88: 4BFF3151  bl 0x82307bd8
	ctx.lr = 0x82314A8C;
	sub_82307BD8(ctx, base);
	// 82314A8C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82314A90: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82314A94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82314A98: 419A000C  beq cr6, 0x82314aa4
	if ctx.cr[6].eq {
	pc = 0x82314AA4; continue 'dispatch;
	}
	// 82314A9C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82314AA0: 4BFABDF1  bl 0x822c0890
	ctx.lr = 0x82314AA4;
	sub_822C0890(ctx, base);
	// 82314AA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314AA8: 48ADE981  bl 0x82df3428
	ctx.lr = 0x82314AAC;
	sub_82DF3428(ctx, base);
	// 82314AAC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82314AB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82314AB4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82314AB8: 48038FF1  bl 0x8234daa8
	ctx.lr = 0x82314ABC;
	sub_8234DAA8(ctx, base);
	// 82314ABC: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82314AC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314AC4: 808BB534  lwz r4, -0x4acc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19148 as u32) ) } as u64;
	// 82314AC8: 48ADEF41  bl 0x82df3a08
	ctx.lr = 0x82314ACC;
	sub_82DF3A08(ctx, base);
	// 82314ACC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82314AD0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82314AD4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82314AD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82314ADC: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82314AE0: 4BFF0A81  bl 0x82305560
	ctx.lr = 0x82314AE4;
	sub_82305560(ctx, base);
	// 82314AE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82314AE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314AEC: 48ADE93D  bl 0x82df3428
	ctx.lr = 0x82314AF0;
	sub_82DF3428(ctx, base);
	// 82314AF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82314AF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82314AF8: 480380D1  bl 0x8234cbc8
	ctx.lr = 0x82314AFC;
	sub_8234CBC8(ctx, base);
	// 82314AFC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82314B00: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82314B04: 48E936B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82314B08 size=92
    let mut pc: u32 = 0x82314B08;
    'dispatch: loop {
        match pc {
            0x82314B08 => {
    //   block [0x82314B08..0x82314B64)
	// 82314B08: 89440350  lbz r10, 0x350(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(848 as u32) ) } as u64;
	// 82314B0C: 39640350  addi r11, r4, 0x350
	ctx.r[11].s64 = ctx.r[4].s64 + 848;
	// 82314B10: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82314B14: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82314B18: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82314B1C: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82314B20: 99430000  stb r10, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82314B24: 39400050  li r10, 0x50
	ctx.r[10].s64 = 80;
	// 82314B28: 13EB30C7  vcmpequd (lvx128) v31, v11, v6
	tmp.u32 = ctx.r[11].u32 + ctx.r[6].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82314B68 size=92
    let mut pc: u32 = 0x82314B68;
    'dispatch: loop {
        match pc {
            0x82314B68 => {
    //   block [0x82314B68..0x82314BC4)
	// 82314B68: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82314B6C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82314B70: 39630350  addi r11, r3, 0x350
	ctx.r[11].s64 = ctx.r[3].s64 + 848;
	// 82314B74: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82314B78: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82314B7C: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 82314B80: 99430350  stb r10, 0x350(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(848 as u32), ctx.r[10].u8 ) };
	// 82314B84: 13E430C7  vcmpequd (lvx128) v31, v4, v6
	tmp.u32 = ctx.r[4].u32 + ctx.r[6].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82314BC8 size=16
    let mut pc: u32 = 0x82314BC8;
    'dispatch: loop {
        match pc {
            0x82314BC8 => {
    //   block [0x82314BC8..0x82314BD8)
	// 82314BC8: 39600360  li r11, 0x360
	ctx.r[11].s64 = 864;
	// 82314BCC: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82314BD8 size=12
    let mut pc: u32 = 0x82314BD8;
    'dispatch: loop {
        match pc {
            0x82314BD8 => {
    //   block [0x82314BD8..0x82314BE4)
	// 82314BD8: 806301FC  lwz r3, 0x1fc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(508 as u32) ) } as u64;
	// 82314BDC: 388000B7  li r4, 0xb7
	ctx.r[4].s64 = 183;
	// 82314BE0: 48146208  b 0x8245ade8
	sub_8245ADE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314BE8 size=52
    let mut pc: u32 = 0x82314BE8;
    'dispatch: loop {
        match pc {
            0x82314BE8 => {
    //   block [0x82314BE8..0x82314C1C)
	// 82314BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314BEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314BF0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314BF4: D021007C  stfs f1, 0x7c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82314BF8: 4BFF0A39  bl 0x82305630
	ctx.lr = 0x82314BFC;
	sub_82305630(ctx, base);
	// 82314BFC: 38A1007C  addi r5, r1, 0x7c
	ctx.r[5].s64 = ctx.r[1].s64 + 124;
	// 82314C00: 388000B7  li r4, 0xb7
	ctx.r[4].s64 = 183;
	// 82314C04: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82314C08: 48147141  bl 0x8245bd48
	ctx.lr = 0x82314C0C;
	sub_8245BD48(ctx, base);
	// 82314C0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82314C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82314C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82314C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82314C20 size=12
    let mut pc: u32 = 0x82314C20;
    'dispatch: loop {
        match pc {
            0x82314C20 => {
    //   block [0x82314C20..0x82314C2C)
	// 82314C20: 806301FC  lwz r3, 0x1fc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(508 as u32) ) } as u64;
	// 82314C24: 388000B6  li r4, 0xb6
	ctx.r[4].s64 = 182;
	// 82314C28: 481461C0  b 0x8245ade8
	sub_8245ADE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314C30 size=52
    let mut pc: u32 = 0x82314C30;
    'dispatch: loop {
        match pc {
            0x82314C30 => {
    //   block [0x82314C30..0x82314C64)
	// 82314C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314C38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314C3C: D021007C  stfs f1, 0x7c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82314C40: 4BFF09F1  bl 0x82305630
	ctx.lr = 0x82314C44;
	sub_82305630(ctx, base);
	// 82314C44: 38A1007C  addi r5, r1, 0x7c
	ctx.r[5].s64 = ctx.r[1].s64 + 124;
	// 82314C48: 388000B6  li r4, 0xb6
	ctx.r[4].s64 = 182;
	// 82314C4C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82314C50: 481470F9  bl 0x8245bd48
	ctx.lr = 0x82314C54;
	sub_8245BD48(ctx, base);
	// 82314C54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82314C58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82314C5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82314C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314C68 size=144
    let mut pc: u32 = 0x82314C68;
    'dispatch: loop {
        match pc {
            0x82314C68 => {
    //   block [0x82314C68..0x82314CF8)
	// 82314C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314C6C: 48E93501  bl 0x831a816c
	ctx.lr = 0x82314C70;
	sub_831A8130(ctx, base);
	// 82314C70: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314C74: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82314C78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82314C7C: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82314C80: 397F0650  addi r11, r31, 0x650
	ctx.r[11].s64 = ctx.r[31].s64 + 1616;
	// 82314C84: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82314C88: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82314C8C: 915F0650  stw r10, 0x650(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1616 as u32), ctx.r[10].u32 ) };
	// 82314C90: 4BFAF7D1  bl 0x822c4460
	ctx.lr = 0x82314C94;
	sub_822C4460(ctx, base);
	// 82314C94: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82314C98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82314C9C: 83BF0650  lwz r29, 0x650(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1616 as u32) ) } as u64;
	// 82314CA0: 4BFF0DD1  bl 0x82305a70
	ctx.lr = 0x82314CA4;
	sub_82305A70(ctx, base);
	// 82314CA4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82314CA8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82314CAC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82314CB0: C02B08A8  lfs f1, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82314CB4: 4BFBB815  bl 0x822d04c8
	ctx.lr = 0x82314CB8;
	sub_822D04C8(ctx, base);
	// 82314CB8: 807F0650  lwz r3, 0x650(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1616 as u32) ) } as u64;
	// 82314CBC: 4BFBB275  bl 0x822cff30
	ctx.lr = 0x82314CC0;
	sub_822CFF30(ctx, base);
	// 82314CC0: 9BDF0658  stb r30, 0x658(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(1624 as u32), ctx.r[30].u8 ) };
	// 82314CC4: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82314CC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314CCC: 808BB4D0  lwz r4, -0x4b30(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19248 as u32) ) } as u64;
	// 82314CD0: 48ADED39  bl 0x82df3a08
	ctx.lr = 0x82314CD4;
	sub_82DF3A08(ctx, base);
	// 82314CD4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82314CD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82314CDC: 4BFF084D  bl 0x82305528
	ctx.lr = 0x82314CE0;
	sub_82305528(ctx, base);
	// 82314CE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82314CE4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314CE8: 48ADE741  bl 0x82df3428
	ctx.lr = 0x82314CEC;
	sub_82DF3428(ctx, base);
	// 82314CEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82314CF0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82314CF4: 48E934C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314CF8 size=52
    let mut pc: u32 = 0x82314CF8;
    'dispatch: loop {
        match pc {
            0x82314CF8 => {
    //   block [0x82314CF8..0x82314D2C)
	// 82314CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314D00: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314D04: D021007C  stfs f1, 0x7c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82314D08: 4BFF0929  bl 0x82305630
	ctx.lr = 0x82314D0C;
	sub_82305630(ctx, base);
	// 82314D0C: 38A1007C  addi r5, r1, 0x7c
	ctx.r[5].s64 = ctx.r[1].s64 + 124;
	// 82314D10: 38800026  li r4, 0x26
	ctx.r[4].s64 = 38;
	// 82314D14: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82314D18: 48147031  bl 0x8245bd48
	ctx.lr = 0x82314D1C;
	sub_8245BD48(ctx, base);
	// 82314D1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82314D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82314D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82314D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82314D30 size=44
    let mut pc: u32 = 0x82314D30;
    'dispatch: loop {
        match pc {
            0x82314D30 => {
    //   block [0x82314D30..0x82314D5C)
	// 82314D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314D38: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314D3C: 4BFF08F5  bl 0x82305630
	ctx.lr = 0x82314D40;
	sub_82305630(ctx, base);
	// 82314D40: 38800026  li r4, 0x26
	ctx.r[4].s64 = 38;
	// 82314D44: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82314D48: 48146E39  bl 0x8245bb80
	ctx.lr = 0x82314D4C;
	sub_8245BB80(ctx, base);
	// 82314D4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82314D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82314D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82314D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82314D60 size=44
    let mut pc: u32 = 0x82314D60;
    'dispatch: loop {
        match pc {
            0x82314D60 => {
    //   block [0x82314D60..0x82314D8C)
	// 82314D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314D68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314D6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82314D70: C84BD860  lfd f2, -0x27a0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-10144 as u32) ) };
	// 82314D74: 48E96735  bl 0x831ab4a8
	ctx.lr = 0x82314D78;
	sub_831AB4A8(ctx, base);
	// 82314D78: FC200818  frsp f1, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (ctx.f[1].f64 as f32) as f64;
	// 82314D7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82314D80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82314D84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82314D88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82314D90 size=136
    let mut pc: u32 = 0x82314D90;
    'dispatch: loop {
        match pc {
            0x82314D90 => {
    //   block [0x82314D90..0x82314E18)
	// 82314D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314D94: 48E933D9  bl 0x831a816c
	ctx.lr = 0x82314D98;
	sub_831A8130(ctx, base);
	// 82314D98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314D9C: 3FA08326  lis r29, -0x7cda
	ctx.r[29].s64 = -2094661632;
	// 82314DA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82314DA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314DA8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82314DAC: 809DB564  lwz r4, -0x4a9c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19100 as u32) ) } as u64;
	// 82314DB0: 48ADEC59  bl 0x82df3a08
	ctx.lr = 0x82314DB4;
	sub_82DF3A08(ctx, base);
	// 82314DB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82314DB8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82314DBC: 4BFF07BD  bl 0x82305578
	ctx.lr = 0x82314DC0;
	sub_82305578(ctx, base);
	// 82314DC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314DC4: 48ADE665  bl 0x82df3428
	ctx.lr = 0x82314DC8;
	sub_82DF3428(ctx, base);
	// 82314DC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314DCC: 809DB564  lwz r4, -0x4a9c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19100 as u32) ) } as u64;
	// 82314DD0: 48ADEC39  bl 0x82df3a08
	ctx.lr = 0x82314DD4;
	sub_82DF3A08(ctx, base);
	// 82314DD4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82314DD8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82314DDC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82314DE0: 4BFF2DF9  bl 0x82307bd8
	ctx.lr = 0x82314DE4;
	sub_82307BD8(ctx, base);
	// 82314DE4: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82314DE8: 83C30000  lwz r30, 0(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82314DEC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82314DF0: 419A000C  beq cr6, 0x82314dfc
	if ctx.cr[6].eq {
	pc = 0x82314DFC; continue 'dispatch;
	}
	// 82314DF4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82314DF8: 4BFABA99  bl 0x822c0890
	ctx.lr = 0x82314DFC;
	sub_822C0890(ctx, base);
	// 82314DFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314E00: 48ADE629  bl 0x82df3428
	ctx.lr = 0x82314E04;
	sub_82DF3428(ctx, base);
	// 82314E04: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82314E08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82314E0C: 4803A01D  bl 0x8234ee28
	ctx.lr = 0x82314E10;
	sub_8234EE28(ctx, base);
	// 82314E10: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82314E14: 48E933A8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314E18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314E18 size=120
    let mut pc: u32 = 0x82314E18;
    'dispatch: loop {
        match pc {
            0x82314E18 => {
    //   block [0x82314E18..0x82314E90)
	// 82314E18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314E1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314E20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82314E24: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 82314E28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314E2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82314E30: 388000BD  li r4, 0xbd
	ctx.r[4].s64 = 189;
	// 82314E34: C3FF0CCC  lfs f31, 0xccc(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3276 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82314E38: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82314E3C: 48145FAD  bl 0x8245ade8
	ctx.lr = 0x82314E40;
	sub_8245ADE8(ctx, base);
	// 82314E40: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 82314E44: 41980030  blt cr6, 0x82314e74
	if ctx.cr[6].lt {
	pc = 0x82314E74; continue 'dispatch;
	}
	// 82314E48: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82314E4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314E50: 808BB400  lwz r4, -0x4c00(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19456 as u32) ) } as u64;
	// 82314E54: 48ADEBB5  bl 0x82df3a08
	ctx.lr = 0x82314E58;
	sub_82DF3A08(ctx, base);
	// 82314E58: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82314E5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82314E60: 4BFF0731  bl 0x82305590
	ctx.lr = 0x82314E64;
	sub_82305590(ctx, base);
	// 82314E64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314E68: 48ADE5C1  bl 0x82df3428
	ctx.lr = 0x82314E6C;
	sub_82DF3428(ctx, base);
	// 82314E6C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82314E70: 48000008  b 0x82314e78
	pc = 0x82314E78; continue 'dispatch;
	// 82314E74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82314E78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82314E7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82314E80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82314E84: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82314E88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82314E8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314E90 size=124
    let mut pc: u32 = 0x82314E90;
    'dispatch: loop {
        match pc {
            0x82314E90 => {
    //   block [0x82314E90..0x82314F0C)
	// 82314E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314E98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82314E9C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314EA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82314EA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314EA8: 4BFF31D1  bl 0x82308078
	ctx.lr = 0x82314EAC;
	sub_82308078(ctx, base);
	// 82314EAC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82314EB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82314EB4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82314EB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82314EBC: 4E800421  bctrl
	ctx.lr = 0x82314EC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82314EC0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82314EC4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82314EC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82314ECC: C00B0030  lfs f0, 0x30(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82314ED0: C1AB0034  lfs f13, 0x34(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82314ED4: C18B0038  lfs f12, 0x38(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82314ED8: C16B003C  lfs f11, 0x3c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82314EDC: D01F0000  stfs f0, 0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82314EE0: D1BF0004  stfs f13, 4(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82314EE4: D19F0008  stfs f12, 8(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82314EE8: D17F000C  stfs f11, 0xc(r31)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82314EEC: 419A0008  beq cr6, 0x82314ef4
	if ctx.cr[6].eq {
	pc = 0x82314EF4; continue 'dispatch;
	}
	// 82314EF0: 4BFAB9A1  bl 0x822c0890
	ctx.lr = 0x82314EF4;
	sub_822C0890(ctx, base);
	// 82314EF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82314EF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82314EFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82314F00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82314F04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82314F08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314F10 size=120
    let mut pc: u32 = 0x82314F10;
    'dispatch: loop {
        match pc {
            0x82314F10 => {
    //   block [0x82314F10..0x82314F88)
	// 82314F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314F18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82314F1C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314F20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82314F24: 2F040003  cmpwi cr6, r4, 3
	ctx.cr[6].compare_i32(ctx.r[4].s32, 3, &mut ctx.xer);
	// 82314F28: 419A0018  beq cr6, 0x82314f40
	if ctx.cr[6].eq {
	pc = 0x82314F40; continue 'dispatch;
	}
	// 82314F2C: 4BFF0705  bl 0x82305630
	ctx.lr = 0x82314F30;
	sub_82305630(ctx, base);
	// 82314F30: 3880006E  li r4, 0x6e
	ctx.r[4].s64 = 110;
	// 82314F34: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82314F38: 48146C49  bl 0x8245bb80
	ctx.lr = 0x82314F3C;
	sub_8245BB80(ctx, base);
	// 82314F3C: 48000038  b 0x82314f74
	pc = 0x82314F74; continue 'dispatch;
	// 82314F40: 38800082  li r4, 0x82
	ctx.r[4].s64 = 130;
	// 82314F44: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82314F48: 48145EA1  bl 0x8245ade8
	ctx.lr = 0x82314F4C;
	sub_8245ADE8(ctx, base);
	// 82314F4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82314F50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82314F54: C00B9524  lfs f0, -0x6adc(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27356 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82314F58: EC010032  fmuls f0, f1, f0
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64) as f32) as f64);
	// 82314F5C: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82314F60: 4BFF06D1  bl 0x82305630
	ctx.lr = 0x82314F64;
	sub_82305630(ctx, base);
	// 82314F64: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82314F68: 3880006E  li r4, 0x6e
	ctx.r[4].s64 = 110;
	// 82314F6C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82314F70: 48146DD9  bl 0x8245bd48
	ctx.lr = 0x82314F74;
	sub_8245BD48(ctx, base);
	// 82314F74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82314F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82314F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82314F80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82314F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82314F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82314F88 size=184
    let mut pc: u32 = 0x82314F88;
    'dispatch: loop {
        match pc {
            0x82314F88 => {
    //   block [0x82314F88..0x82315040)
	// 82314F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82314F8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82314F90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82314F94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82314F98: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82314F9C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82314FA0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82314FA4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82314FA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82314FAC: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82314FB0: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82314FB4: 419A0070  beq cr6, 0x82315024
	if ctx.cr[6].eq {
	pc = 0x82315024; continue 'dispatch;
	}
	// 82314FB8: 3FC08326  lis r30, -0x7cda
	ctx.r[30].s64 = -2094661632;
	// 82314FBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314FC0: 809EB568  lwz r4, -0x4a98(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19096 as u32) ) } as u64;
	// 82314FC4: 48ADEA45  bl 0x82df3a08
	ctx.lr = 0x82314FC8;
	sub_82DF3A08(ctx, base);
	// 82314FC8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82314FCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82314FD0: 4BFF05A9  bl 0x82305578
	ctx.lr = 0x82314FD4;
	sub_82305578(ctx, base);
	// 82314FD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314FD8: 48ADE451  bl 0x82df3428
	ctx.lr = 0x82314FDC;
	sub_82DF3428(ctx, base);
	// 82314FDC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82314FE0: 809EB568  lwz r4, -0x4a98(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-19096 as u32) ) } as u64;
	// 82314FE4: 48ADEA25  bl 0x82df3a08
	ctx.lr = 0x82314FE8;
	sub_82DF3A08(ctx, base);
	// 82314FE8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82314FEC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82314FF0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82314FF4: 4BFF2BE5  bl 0x82307bd8
	ctx.lr = 0x82314FF8;
	sub_82307BD8(ctx, base);
	// 82314FF8: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82314FFC: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315000: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82315004: 419A000C  beq cr6, 0x82315010
	if ctx.cr[6].eq {
	pc = 0x82315010; continue 'dispatch;
	}
	// 82315008: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 8231500C: 4BFAB885  bl 0x822c0890
	ctx.lr = 0x82315010;
	sub_822C0890(ctx, base);
	// 82315010: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315014: 48ADE415  bl 0x82df3428
	ctx.lr = 0x82315018;
	sub_82DF3428(ctx, base);
	// 82315018: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8231501C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82315020: 48038B59  bl 0x8234db78
	ctx.lr = 0x82315024;
	sub_8234DB78(ctx, base);
	// 82315024: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82315028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8231502C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82315030: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82315034: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82315038: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8231503C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82315040 size=160
    let mut pc: u32 = 0x82315040;
    'dispatch: loop {
        match pc {
            0x82315040 => {
    //   block [0x82315040..0x823150E0)
	// 82315040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82315044: 48E93129  bl 0x831a816c
	ctx.lr = 0x82315048;
	sub_831A8130(ctx, base);
	// 82315048: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 8231504C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82315050: 3FA08326  lis r29, -0x7cda
	ctx.r[29].s64 = -2094661632;
	// 82315054: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82315058: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8231505C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315060: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82315064: 809DB568  lwz r4, -0x4a98(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19096 as u32) ) } as u64;
	// 82315068: 48ADE9A1  bl 0x82df3a08
	ctx.lr = 0x8231506C;
	sub_82DF3A08(ctx, base);
	// 8231506C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82315070: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82315074: 4BFF0505  bl 0x82305578
	ctx.lr = 0x82315078;
	sub_82305578(ctx, base);
	// 82315078: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8231507C: 48ADE3AD  bl 0x82df3428
	ctx.lr = 0x82315080;
	sub_82DF3428(ctx, base);
	// 82315080: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315084: 809DB568  lwz r4, -0x4a98(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-19096 as u32) ) } as u64;
	// 82315088: 48ADE981  bl 0x82df3a08
	ctx.lr = 0x8231508C;
	sub_82DF3A08(ctx, base);
	// 8231508C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82315090: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82315094: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82315098: 4BFF2B41  bl 0x82307bd8
	ctx.lr = 0x8231509C;
	sub_82307BD8(ctx, base);
	// 8231509C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823150A0: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823150A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823150A8: 419A000C  beq cr6, 0x823150b4
	if ctx.cr[6].eq {
	pc = 0x823150B4; continue 'dispatch;
	}
	// 823150AC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823150B0: 4BFAB7E1  bl 0x822c0890
	ctx.lr = 0x823150B4;
	sub_822C0890(ctx, base);
	// 823150B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823150B8: 48ADE371  bl 0x82df3428
	ctx.lr = 0x823150BC;
	sub_82DF3428(ctx, base);
	// 823150BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823150C0: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 823150C4: 48038AB5  bl 0x8234db78
	ctx.lr = 0x823150C8;
	sub_8234DB78(ctx, base);
	// 823150C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823150CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823150D0: 48038B99  bl 0x8234dc68
	ctx.lr = 0x823150D4;
	sub_8234DC68(ctx, base);
	// 823150D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 823150D8: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 823150DC: 48E930E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823150E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823150E0 size=60
    let mut pc: u32 = 0x823150E0;
    'dispatch: loop {
        match pc {
            0x823150E0 => {
    //   block [0x823150E0..0x8231511C)
	// 823150E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823150E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823150E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823150EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823150F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823150F4: 389F05C0  addi r4, r31, 0x5c0
	ctx.r[4].s64 = ctx.r[31].s64 + 1472;
	// 823150F8: 4BFFEF99  bl 0x82314090
	ctx.lr = 0x823150FC;
	sub_82314090(ctx, base);
	// 823150FC: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 82315100: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82315104: 4801C655  bl 0x82331758
	ctx.lr = 0x82315108;
	sub_82331758(ctx, base);
	// 82315108: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8231510C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82315110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82315114: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82315118: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82315120 size=52
    let mut pc: u32 = 0x82315120;
    'dispatch: loop {
        match pc {
            0x82315120 => {
    //   block [0x82315120..0x82315154)
	// 82315120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82315124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82315128: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8231512C: D021007C  stfs f1, 0x7c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 82315130: 4BFF0501  bl 0x82305630
	ctx.lr = 0x82315134;
	sub_82305630(ctx, base);
	// 82315134: 38A1007C  addi r5, r1, 0x7c
	ctx.r[5].s64 = ctx.r[1].s64 + 124;
	// 82315138: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8231513C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315140: 48146C09  bl 0x8245bd48
	ctx.lr = 0x82315144;
	sub_8245BD48(ctx, base);
	// 82315144: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82315148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8231514C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82315150: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82315158 size=16
    let mut pc: u32 = 0x82315158;
    'dispatch: loop {
        match pc {
            0x82315158 => {
    //   block [0x82315158..0x82315168)
	// 82315158: 39600240  li r11, 0x240
	ctx.r[11].s64 = 576;
	// 8231515C: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315168(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82315168 size=156
    let mut pc: u32 = 0x82315168;
    'dispatch: loop {
        match pc {
            0x82315168 => {
    //   block [0x82315168..0x82315204)
	// 82315168: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231516C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82315170: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82315174: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82315178: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8231517C: 388000DF  li r4, 0xdf
	ctx.r[4].s64 = 223;
	// 82315180: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82315184: 48145C1D  bl 0x8245ada0
	ctx.lr = 0x82315188;
	sub_8245ADA0(ctx, base);
	// 82315188: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8231518C: 4182000C  beq 0x82315198
	if ctx.cr[0].eq {
	pc = 0x82315198; continue 'dispatch;
	}
	// 82315190: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82315194: 4800005C  b 0x823151f0
	pc = 0x823151F0; continue 'dispatch;
	// 82315198: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 8231519C: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823151A0: 4801C591  bl 0x82331730
	ctx.lr = 0x823151A4;
	sub_82331730(ctx, base);
	// 823151A4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823151A8: 4082FFE8  bne 0x82315190
	if !ctx.cr[0].eq {
	pc = 0x82315190; continue 'dispatch;
	}
	// 823151AC: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 823151B0: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823151B4: 4801C57D  bl 0x82331730
	ctx.lr = 0x823151B8;
	sub_82331730(ctx, base);
	// 823151B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823151BC: 4082FFD4  bne 0x82315190
	if !ctx.cr[0].eq {
	pc = 0x82315190; continue 'dispatch;
	}
	// 823151C0: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 823151C4: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823151C8: 4801C569  bl 0x82331730
	ctx.lr = 0x823151CC;
	sub_82331730(ctx, base);
	// 823151CC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823151D0: 4082FFC0  bne 0x82315190
	if !ctx.cr[0].eq {
	pc = 0x82315190; continue 'dispatch;
	}
	// 823151D4: 38800029  li r4, 0x29
	ctx.r[4].s64 = 41;
	// 823151D8: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823151DC: 4801C555  bl 0x82331730
	ctx.lr = 0x823151E0;
	sub_82331730(ctx, base);
	// 823151E0: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 823151E4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 823151E8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 823151EC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 823151F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823151F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823151F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823151FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82315200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82315208 size=348
    let mut pc: u32 = 0x82315208;
    'dispatch: loop {
        match pc {
            0x82315208 => {
    //   block [0x82315208..0x82315364)
	// 82315208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231520C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82315210: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82315214: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82315218: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 8231521C: 48E93859  bl 0x831a8a74
	ctx.lr = 0x82315220;
	sub_831A8A40(ctx, base);
	// 82315220: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82315224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82315228: FF600890  fmr f27, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[27].f64 = ctx.f[1].f64;
	// 8231522C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82315230: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82315234: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 82315238: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8231523C: C01F0D3C  lfs f0, 0xd3c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(3388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82315240: C38B08A8  lfs f28, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 82315244: C3AA9534  lfs f29, -0x6acc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 82315248: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8231524C: C3E908A4  lfs f31, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82315250: 40990058  ble cr6, 0x823152a8
	if !ctx.cr[6].gt {
	pc = 0x823152A8; continue 'dispatch;
	}
	// 82315254: FFC0E090  fmr f30, f28
	ctx.f[30].f64 = ctx.f[28].f64;
	// 82315258: FF1EF800  fcmpu cr6, f30, f31
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[31].f64);
	// 8231525C: 4099005C  ble cr6, 0x823152b8
	if !ctx.cr[6].gt {
	pc = 0x823152B8; continue 'dispatch;
	}
	// 82315260: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 82315264: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82315268: 48145B81  bl 0x8245ade8
	ctx.lr = 0x8231526C;
	sub_8245ADE8(ctx, base);
	// 8231526C: EC1E0828  fsubs f0, f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[30].f64 - ctx.f[1].f64) as f32) as f64);
	// 82315270: 388000C1  li r4, 0xc1
	ctx.r[4].s64 = 193;
	// 82315274: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82315278: FFC0F82E  fsel f30, f0, f0, f31
	ctx.f[30].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[31].f64 };
	// 8231527C: 48145B6D  bl 0x8245ade8
	ctx.lr = 0x82315280;
	sub_8245ADE8(ctx, base);
	// 82315280: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 82315284: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82315288: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 8231528C: 48145B5D  bl 0x8245ade8
	ctx.lr = 0x82315290;
	sub_8245ADE8(ctx, base);
	// 82315290: EC1D0828  fsubs f0, f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[29].f64 - ctx.f[1].f64) as f32) as f64);
	// 82315294: EC1E0024  fdivs f0, f30, f0
	ctx.f[0].f64 = ((ctx.f[30].f64 / ctx.f[0].f64) as f32) as f64;
	// 82315298: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 8231529C: 40990064  ble cr6, 0x82315300
	if !ctx.cr[6].gt {
	pc = 0x82315300; continue 'dispatch;
	}
	// 823152A0: FC00E090  fmr f0, f28
	ctx.f[0].f64 = ctx.f[28].f64;
	// 823152A4: 48000064  b 0x82315308
	pc = 0x82315308; continue 'dispatch;
	// 823152A8: FFC00090  fmr f30, f0
	ctx.f[30].f64 = ctx.f[0].f64;
	// 823152AC: FF00E800  fcmpu cr6, f0, f29
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[29].f64);
	// 823152B0: 4098FFA8  bge cr6, 0x82315258
	if !ctx.cr[6].lt {
	pc = 0x82315258; continue 'dispatch;
	}
	// 823152B4: FFC0E890  fmr f30, f29
	ctx.f[30].f64 = ctx.f[29].f64;
	// 823152B8: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 823152BC: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 823152C0: 48145B29  bl 0x8245ade8
	ctx.lr = 0x823152C4;
	sub_8245ADE8(ctx, base);
	// 823152C4: EC1E0F78  fmsubs f0, f30, f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[30].f64 * ctx.f[29].f64 - ctx.f[1].f64) as f32) as f64);
	// 823152C8: 388000C1  li r4, 0xc1
	ctx.r[4].s64 = 193;
	// 823152CC: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 823152D0: FFC0F82E  fsel f30, f0, f0, f31
	ctx.f[30].f64 = if ctx.f[0].f64 >= 0.0 { ctx.f[0].f64 } else { ctx.f[31].f64 };
	// 823152D4: 48145B15  bl 0x8245ade8
	ctx.lr = 0x823152D8;
	sub_8245ADE8(ctx, base);
	// 823152D8: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 823152DC: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 823152E0: FFA00890  fmr f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].f64 = ctx.f[1].f64;
	// 823152E4: 48145B05  bl 0x8245ade8
	ctx.lr = 0x823152E8;
	sub_8245ADE8(ctx, base);
	// 823152E8: EC1D0828  fsubs f0, f29, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[29].f64 - ctx.f[1].f64) as f32) as f64);
	// 823152EC: EC1E0024  fdivs f0, f30, f0
	ctx.f[0].f64 = ((ctx.f[30].f64 / ctx.f[0].f64) as f32) as f64;
	// 823152F0: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 823152F4: 40990020  ble cr6, 0x82315314
	if !ctx.cr[6].gt {
	pc = 0x82315314; continue 'dispatch;
	}
	// 823152F8: FC00E090  fmr f0, f28
	ctx.f[0].f64 = ctx.f[28].f64;
	// 823152FC: 48000020  b 0x8231531c
	pc = 0x8231531C; continue 'dispatch;
	// 82315300: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82315304: 41980008  blt cr6, 0x8231530c
	if ctx.cr[6].lt {
	pc = 0x8231530C; continue 'dispatch;
	}
	// 82315308: FFE00090  fmr f31, f0
	ctx.f[31].f64 = ctx.f[0].f64;
	// 8231530C: EFFFE02A  fadds f31, f31, f28
	ctx.f[31].f64 = ((ctx.f[31].f64 + ctx.f[28].f64) as f32) as f64;
	// 82315310: 48000014  b 0x82315324
	pc = 0x82315324; continue 'dispatch;
	// 82315314: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 82315318: 41980008  blt cr6, 0x82315320
	if ctx.cr[6].lt {
	pc = 0x82315320; continue 'dispatch;
	}
	// 8231531C: FFE00090  fmr f31, f0
	ctx.f[31].f64 = ctx.f[0].f64;
	// 82315320: EFFCF828  fsubs f31, f28, f31
	ctx.f[31].f64 = (((ctx.f[28].f64 - ctx.f[31].f64) as f32) as f64);
	// 82315324: 388000C3  li r4, 0xc3
	ctx.r[4].s64 = 195;
	// 82315328: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 8231532C: 48145ABD  bl 0x8245ade8
	ctx.lr = 0x82315330;
	sub_8245ADE8(ctx, base);
	// 82315330: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82315334: FC400890  fmr f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[1].f64;
	// 82315338: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8231533C: FC60D890  fmr f3, f27
	ctx.f[3].f64 = ctx.f[27].f64;
	// 82315340: 48182A49  bl 0x82497d88
	ctx.lr = 0x82315344;
	sub_82497D88(ctx, base);
	// 82315344: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82315348: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 8231534C: 48E93775  bl 0x831a8ac0
	ctx.lr = 0x82315350;
	sub_831A8A8C(ctx, base);
	// 82315350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82315354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82315358: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8231535C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82315360: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82315368 size=236
    let mut pc: u32 = 0x82315368;
    'dispatch: loop {
        match pc {
            0x82315368 => {
    //   block [0x82315368..0x82315454)
	// 82315368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231536C: 48E92E01  bl 0x831a816c
	ctx.lr = 0x82315370;
	sub_831A8130(ctx, base);
	// 82315370: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 82315374: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 82315378: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8231537C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82315380: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82315384: 38800089  li r4, 0x89
	ctx.r[4].s64 = 137;
	// 82315388: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8231538C: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82315390: 48145A59  bl 0x8245ade8
	ctx.lr = 0x82315394;
	sub_8245ADE8(ctx, base);
	// 82315394: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82315398: C3EBCEE4  lfs f31, -0x311c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-12572 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8231539C: EC2107F2  fmuls f1, f1, f31
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[31].f64) as f32) as f64);
	// 823153A0: 48E93B09  bl 0x831a8ea8
	ctx.lr = 0x823153A4;
	sub_831A8EA8(ctx, base);
	// 823153A4: 3880008B  li r4, 0x8b
	ctx.r[4].s64 = 139;
	// 823153A8: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 823153AC: FFC00818  frsp f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = (ctx.f[1].f64 as f32) as f64;
	// 823153B0: 48145A39  bl 0x8245ade8
	ctx.lr = 0x823153B4;
	sub_8245ADE8(ctx, base);
	// 823153B4: EC2107F2  fmuls f1, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = (((ctx.f[1].f64 * ctx.f[31].f64) as f32) as f64);
	// 823153B8: 48E93AF1  bl 0x831a8ea8
	ctx.lr = 0x823153BC;
	sub_831A8EA8(ctx, base);
	// 823153BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823153C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823153C4: FFE00818  frsp f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = (ctx.f[1].f64 as f32) as f64;
	// 823153C8: 48B68089  bl 0x82e7d450
	ctx.lr = 0x823153CC;
	sub_82E7D450(ctx, base);
	// 823153CC: 13C018C7  vcmpequd (lvx128) v30, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823153D0: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823153D4: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82315458 size=16
    let mut pc: u32 = 0x82315458;
    'dispatch: loop {
        match pc {
            0x82315458 => {
    //   block [0x82315458..0x82315468)
	// 82315458: 39600430  li r11, 0x430
	ctx.r[11].s64 = 1072;
	// 8231545C: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82315468 size=152
    let mut pc: u32 = 0x82315468;
    'dispatch: loop {
        match pc {
            0x82315468 => {
    //   block [0x82315468..0x82315500)
	// 82315468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231546C: 48E92D01  bl 0x831a816c
	ctx.lr = 0x82315470;
	sub_831A8130(ctx, base);
	// 82315470: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82315474: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82315478: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8231547C: 396BA2F0  addi r11, r11, -0x5d10
	ctx.r[11].s64 = ctx.r[11].s64 + -23824;
	// 82315480: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82315484: 3D208209  lis r9, -0x7df7
	ctx.r[9].s64 = -2113339392;
	// 82315488: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 8231548C: 13C0F0C7  vcmpequd (lvx128) v30, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82315490: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82315494: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82315498: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82315500 size=176
    let mut pc: u32 = 0x82315500;
    'dispatch: loop {
        match pc {
            0x82315500 => {
    //   block [0x82315500..0x823155B0)
	// 82315500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82315504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82315508: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8231550C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82315510: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82315514: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82315518: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 8231551C: 4801C215  bl 0x82331730
	ctx.lr = 0x82315520;
	sub_82331730(ctx, base);
	// 82315520: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82315524: 40820078  bne 0x8231559c
	if !ctx.cr[0].eq {
	pc = 0x8231559C; continue 'dispatch;
	}
	// 82315528: 897F04B4  lbz r11, 0x4b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1204 as u32) ) } as u64;
	// 8231552C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82315530: 41820010  beq 0x82315540
	if ctx.cr[0].eq {
	pc = 0x82315540; continue 'dispatch;
	}
	// 82315534: 807F0100  lwz r3, 0x100(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 82315538: 4BFEE2A9  bl 0x823037e0
	ctx.lr = 0x8231553C;
	sub_823037E0(ctx, base);
	// 8231553C: 4800003C  b 0x82315578
	pc = 0x82315578; continue 'dispatch;
	// 82315540: 3D608327  lis r11, -0x7cd9
	ctx.r[11].s64 = -2094596096;
	// 82315544: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315548: 808B4DF4  lwz r4, 0x4df4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(19956 as u32) ) } as u64;
	// 8231554C: 48ADE4BD  bl 0x82df3a08
	ctx.lr = 0x82315550;
	sub_82DF3A08(ctx, base);
	// 82315550: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82315554: 807F0100  lwz r3, 0x100(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(256 as u32) ) } as u64;
	// 82315558: 39000BBB  li r8, 0xbbb
	ctx.r[8].s64 = 3003;
	// 8231555C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82315560: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82315564: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82315568: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8231556C: 4BFEDBB5  bl 0x82303120
	ctx.lr = 0x82315570;
	sub_82303120(ctx, base);
	// 82315570: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315574: 48ADDEB5  bl 0x82df3428
	ctx.lr = 0x82315578;
	sub_82DF3428(ctx, base);
	// 82315578: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 8231557C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315580: 808BB42C  lwz r4, -0x4bd4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19412 as u32) ) } as u64;
	// 82315584: 48ADE485  bl 0x82df3a08
	ctx.lr = 0x82315588;
	sub_82DF3A08(ctx, base);
	// 82315588: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8231558C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82315590: 4BFF0001  bl 0x82305590
	ctx.lr = 0x82315594;
	sub_82305590(ctx, base);
	// 82315594: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315598: 48ADDE91  bl 0x82df3428
	ctx.lr = 0x8231559C;
	sub_82DF3428(ctx, base);
	// 8231559C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823155A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823155A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823155A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823155AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823155B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823155B0 size=8
    let mut pc: u32 = 0x823155B0;
    'dispatch: loop {
        match pc {
            0x823155B0 => {
    //   block [0x823155B0..0x823155B8)
	// 823155B0: 80630598  lwz r3, 0x598(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1432 as u32) ) } as u64;
	// 823155B4: 48AFDCAC  b 0x82e13260
	sub_82E13260(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823155B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823155B8 size=40
    let mut pc: u32 = 0x823155B8;
    'dispatch: loop {
        match pc {
            0x823155B8 => {
    //   block [0x823155B8..0x823155E0)
	// 823155B8: 81630538  lwz r11, 0x538(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1336 as u32) ) } as u64;
	// 823155BC: 7D645851  subf. r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823155C0: 91630538  stw r11, 0x538(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1336 as u32), ctx.r[11].u32 ) };
	// 823155C4: 40800008  bge 0x823155cc
	if !ctx.cr[0].lt {
	pc = 0x823155CC; continue 'dispatch;
	}
	// 823155C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823155CC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823155D0: 91630538  stw r11, 0x538(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1336 as u32), ctx.r[11].u32 ) };
	// 823155D4: 814A006C  lwz r10, 0x6c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(108 as u32) ) } as u64;
	// 823155D8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 823155DC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823155E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823155E0 size=36
    let mut pc: u32 = 0x823155E0;
    'dispatch: loop {
        match pc {
            0x823155E0 => {
    //   block [0x823155E0..0x82315604)
	// 823155E0: 39600CA0  li r11, 0xca0
	ctx.r[11].s64 = 3232;
	// 823155E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823155E8: 39200CB0  li r9, 0xcb0
	ctx.r[9].s64 = 3248;
	// 823155EC: 99430C98  stb r10, 0xc98(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(3224 as u32), ctx.r[10].u8 ) };
	// 823155F0: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82315608 size=16
    let mut pc: u32 = 0x82315608;
    'dispatch: loop {
        match pc {
            0x82315608 => {
    //   block [0x82315608..0x82315618)
	// 82315608: 39600CA0  li r11, 0xca0
	ctx.r[11].s64 = 3232;
	// 8231560C: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82315618 size=16
    let mut pc: u32 = 0x82315618;
    'dispatch: loop {
        match pc {
            0x82315618 => {
    //   block [0x82315618..0x82315628)
	// 82315618: 39600CB0  li r11, 0xcb0
	ctx.r[11].s64 = 3248;
	// 8231561C: 13E458C7  vcmpequd (lvx128) v31, v4, v11
	tmp.u32 = ctx.r[4].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82315628 size=104
    let mut pc: u32 = 0x82315628;
    'dispatch: loop {
        match pc {
            0x82315628 => {
    //   block [0x82315628..0x82315690)
	// 82315628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231562C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82315630: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82315634: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82315638: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8231563C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82315640: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82315644: 38800016  li r4, 0x16
	ctx.r[4].s64 = 22;
	// 82315648: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 8231564C: 4801C0E5  bl 0x82331730
	ctx.lr = 0x82315650;
	sub_82331730(ctx, base);
	// 82315650: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82315654: 41820018  beq 0x8231566c
	if ctx.cr[0].eq {
	pc = 0x8231566C; continue 'dispatch;
	}
	// 82315658: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 8231565C: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82315660: 4801C0D1  bl 0x82331730
	ctx.lr = 0x82315664;
	sub_82331730(ctx, base);
	// 82315664: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82315668: 40820010  bne 0x82315678
	if !ctx.cr[0].eq {
	pc = 0x82315678; continue 'dispatch;
	}
	// 8231566C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82315670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82315674: 4BFF397D  bl 0x82308ff0
	ctx.lr = 0x82315678;
	sub_82308FF0(ctx, base);
	// 82315678: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8231567C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82315680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82315684: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82315688: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8231568C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82315690 size=264
    let mut pc: u32 = 0x82315690;
    'dispatch: loop {
        match pc {
            0x82315690 => {
    //   block [0x82315690..0x82315798)
	// 82315690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82315694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82315698: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8231569C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 823156A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823156A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823156A8: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 823156AC: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823156B0: 4801C081  bl 0x82331730
	ctx.lr = 0x823156B4;
	sub_82331730(ctx, base);
	// 823156B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823156B8: 408200C8  bne 0x82315780
	if !ctx.cr[0].eq {
	pc = 0x82315780; continue 'dispatch;
	}
	// 823156BC: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 823156C0: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823156C4: 4801C085  bl 0x82331748
	ctx.lr = 0x823156C8;
	sub_82331748(ctx, base);
	// 823156C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823156CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823156D0: C3EBD86C  lfs f31, -0x2794(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10132 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 823156D4: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 823156D8: 4BFEFF59  bl 0x82305630
	ctx.lr = 0x823156DC;
	sub_82305630(ctx, base);
	// 823156DC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823156E0: 38800058  li r4, 0x58
	ctx.r[4].s64 = 88;
	// 823156E4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823156E8: 48146661  bl 0x8245bd48
	ctx.lr = 0x823156EC;
	sub_8245BD48(ctx, base);
	// 823156EC: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 823156F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823156F4: 4BFEFF3D  bl 0x82305630
	ctx.lr = 0x823156F8;
	sub_82305630(ctx, base);
	// 823156F8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823156FC: 38800056  li r4, 0x56
	ctx.r[4].s64 = 86;
	// 82315700: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315704: 48146645  bl 0x8245bd48
	ctx.lr = 0x82315708;
	sub_8245BD48(ctx, base);
	// 82315708: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8231570C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82315710: 4BFEFF21  bl 0x82305630
	ctx.lr = 0x82315714;
	sub_82305630(ctx, base);
	// 82315714: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82315718: 38800057  li r4, 0x57
	ctx.r[4].s64 = 87;
	// 8231571C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315720: 48146629  bl 0x8245bd48
	ctx.lr = 0x82315724;
	sub_8245BD48(ctx, base);
	// 82315724: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82315728: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8231572C: 4BFEFF05  bl 0x82305630
	ctx.lr = 0x82315730;
	sub_82305630(ctx, base);
	// 82315730: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82315734: 38800055  li r4, 0x55
	ctx.r[4].s64 = 85;
	// 82315738: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8231573C: 4814660D  bl 0x8245bd48
	ctx.lr = 0x82315740;
	sub_8245BD48(ctx, base);
	// 82315740: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82315744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82315748: C3EBD868  lfs f31, -0x2798(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-10136 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8231574C: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82315750: 4BFEFEE1  bl 0x82305630
	ctx.lr = 0x82315754;
	sub_82305630(ctx, base);
	// 82315754: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82315758: 3880005C  li r4, 0x5c
	ctx.r[4].s64 = 92;
	// 8231575C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315760: 481465E9  bl 0x8245bd48
	ctx.lr = 0x82315764;
	sub_8245BD48(ctx, base);
	// 82315764: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82315768: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8231576C: 4BFEFEC5  bl 0x82305630
	ctx.lr = 0x82315770;
	sub_82305630(ctx, base);
	// 82315770: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82315774: 3880005B  li r4, 0x5b
	ctx.r[4].s64 = 91;
	// 82315778: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8231577C: 481465CD  bl 0x8245bd48
	ctx.lr = 0x82315780;
	sub_8245BD48(ctx, base);
	// 82315780: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82315784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82315788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8231578C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82315790: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82315794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82315798 size=192
    let mut pc: u32 = 0x82315798;
    'dispatch: loop {
        match pc {
            0x82315798 => {
    //   block [0x82315798..0x82315858)
	// 82315798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231579C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823157A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823157A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823157A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823157AC: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 823157B0: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823157B4: 4801BF7D  bl 0x82331730
	ctx.lr = 0x823157B8;
	sub_82331730(ctx, base);
	// 823157B8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823157BC: 41820088  beq 0x82315844
	if ctx.cr[0].eq {
	pc = 0x82315844; continue 'dispatch;
	}
	// 823157C0: 3880002F  li r4, 0x2f
	ctx.r[4].s64 = 47;
	// 823157C4: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823157C8: 4801BF91  bl 0x82331758
	ctx.lr = 0x823157CC;
	sub_82331758(ctx, base);
	// 823157CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823157D0: 4BFEFE61  bl 0x82305630
	ctx.lr = 0x823157D4;
	sub_82305630(ctx, base);
	// 823157D4: 38800058  li r4, 0x58
	ctx.r[4].s64 = 88;
	// 823157D8: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823157DC: 481463A5  bl 0x8245bb80
	ctx.lr = 0x823157E0;
	sub_8245BB80(ctx, base);
	// 823157E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823157E4: 4BFEFE4D  bl 0x82305630
	ctx.lr = 0x823157E8;
	sub_82305630(ctx, base);
	// 823157E8: 38800056  li r4, 0x56
	ctx.r[4].s64 = 86;
	// 823157EC: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823157F0: 48146391  bl 0x8245bb80
	ctx.lr = 0x823157F4;
	sub_8245BB80(ctx, base);
	// 823157F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823157F8: 4BFEFE39  bl 0x82305630
	ctx.lr = 0x823157FC;
	sub_82305630(ctx, base);
	// 823157FC: 38800057  li r4, 0x57
	ctx.r[4].s64 = 87;
	// 82315800: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315804: 4814637D  bl 0x8245bb80
	ctx.lr = 0x82315808;
	sub_8245BB80(ctx, base);
	// 82315808: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8231580C: 4BFEFE25  bl 0x82305630
	ctx.lr = 0x82315810;
	sub_82305630(ctx, base);
	// 82315810: 38800055  li r4, 0x55
	ctx.r[4].s64 = 85;
	// 82315814: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315818: 48146369  bl 0x8245bb80
	ctx.lr = 0x8231581C;
	sub_8245BB80(ctx, base);
	// 8231581C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82315820: 4BFEFE11  bl 0x82305630
	ctx.lr = 0x82315824;
	sub_82305630(ctx, base);
	// 82315824: 3880005C  li r4, 0x5c
	ctx.r[4].s64 = 92;
	// 82315828: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8231582C: 48146355  bl 0x8245bb80
	ctx.lr = 0x82315830;
	sub_8245BB80(ctx, base);
	// 82315830: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82315834: 4BFEFDFD  bl 0x82305630
	ctx.lr = 0x82315838;
	sub_82305630(ctx, base);
	// 82315838: 3880005B  li r4, 0x5b
	ctx.r[4].s64 = 91;
	// 8231583C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315840: 48146341  bl 0x8245bb80
	ctx.lr = 0x82315844;
	sub_8245BB80(ctx, base);
	// 82315844: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82315848: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8231584C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82315850: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82315854: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82315858 size=176
    let mut pc: u32 = 0x82315858;
    'dispatch: loop {
        match pc {
            0x82315858 => {
    //   block [0x82315858..0x82315908)
	// 82315858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231585C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82315860: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82315864: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82315868: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8231586C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82315870: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82315874: 897F03C0  lbz r11, 0x3c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(960 as u32) ) } as u64;
	// 82315878: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8231587C: 40820060  bne 0x823158dc
	if !ctx.cr[0].eq {
	pc = 0x823158DC; continue 'dispatch;
	}
	// 82315880: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315884: 4BFF24ED  bl 0x82307d70
	ctx.lr = 0x82315888;
	sub_82307D70(ctx, base);
	// 82315888: C0010050  lfs f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8231588C: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 82315890: C1A10058  lfs f13, 0x58(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82315894: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82315898: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8231589C: EDAD037A  fmadds f13, f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 823158A0: C0010054  lfs f0, 0x54(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823158A4: EC000032  fmuls f0, f0, f0
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[0].f64) as f32) as f64);
	// 823158A8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 823158AC: 4099000C  ble cr6, 0x823158b8
	if !ctx.cr[6].gt {
	pc = 0x823158B8; continue 'dispatch;
	}
	// 823158B0: 4BFF2411  bl 0x82307cc0
	ctx.lr = 0x823158B4;
	sub_82307CC0(ctx, base);
	// 823158B4: 48000008  b 0x823158bc
	pc = 0x823158BC; continue 'dispatch;
	// 823158B8: 4BFF24B9  bl 0x82307d70
	ctx.lr = 0x823158BC;
	sub_82307D70(ctx, base);
	// 823158BC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823158C0: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82315908 size=152
    let mut pc: u32 = 0x82315908;
    'dispatch: loop {
        match pc {
            0x82315908 => {
    //   block [0x82315908..0x823159A0)
	// 82315908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231590C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82315910: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82315914: DBC1FFE0  stfd f30, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 82315918: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 8231591C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82315920: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82315924: 388000D4  li r4, 0xd4
	ctx.r[4].s64 = 212;
	// 82315928: 817F05D4  lwz r11, 0x5d4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1492 as u32) ) } as u64;
	// 8231592C: 815F05D0  lwz r10, 0x5d0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1488 as u32) ) } as u64;
	// 82315930: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82315934: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82315938: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8231593C: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82315940: 7D4A07B4  extsw r10, r10
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82315944: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82315948: C8010050  lfd f0, 0x50(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8231594C: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82315950: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82315954: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 82315958: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 8231595C: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 82315960: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82315964: EFED0024  fdivs f31, f13, f0
	ctx.f[31].f64 = ((ctx.f[13].f64 / ctx.f[0].f64) as f32) as f64;
	// 82315968: 48145481  bl 0x8245ade8
	ctx.lr = 0x8231596C;
	sub_8245ADE8(ctx, base);
	// 8231596C: 388000D3  li r4, 0xd3
	ctx.r[4].s64 = 211;
	// 82315970: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 82315974: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 82315978: 48145471  bl 0x8245ade8
	ctx.lr = 0x8231597C;
	sub_8245ADE8(ctx, base);
	// 8231597C: EC1E0828  fsubs f0, f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[30].f64 - ctx.f[1].f64) as f32) as f64);
	// 82315980: EC200FFA  fmadds f1, f0, f31, f1
	ctx.f[1].f64 = (((ctx.f[0].f64 * ctx.f[31].f64 + ctx.f[1].f64) as f32) as f64);
	// 82315984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82315988: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8231598C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82315990: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82315994: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82315998: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8231599C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823159A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823159A0 size=12
    let mut pc: u32 = 0x823159A0;
    'dispatch: loop {
        match pc {
            0x823159A0 => {
    //   block [0x823159A0..0x823159AC)
	// 823159A0: 806304AC  lwz r3, 0x4ac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823159A4: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 823159A8: 4801BD88  b 0x82331730
	sub_82331730(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823159B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823159B0 size=8
    let mut pc: u32 = 0x823159B0;
    'dispatch: loop {
        match pc {
            0x823159B0 => {
    //   block [0x823159B0..0x823159B8)
	// 823159B0: 80630558  lwz r3, 0x558(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1368 as u32) ) } as u64;
	// 823159B4: 488A0AAC  b 0x82bb6460
	sub_82BB6460(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823159B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823159B8 size=8
    let mut pc: u32 = 0x823159B8;
    'dispatch: loop {
        match pc {
            0x823159B8 => {
    //   block [0x823159B8..0x823159C0)
	// 823159B8: 80630558  lwz r3, 0x558(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1368 as u32) ) } as u64;
	// 823159BC: 4803AC34  b 0x823505f0
	sub_823505F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823159C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823159C0 size=8
    let mut pc: u32 = 0x823159C0;
    'dispatch: loop {
        match pc {
            0x823159C0 => {
    //   block [0x823159C0..0x823159C8)
	// 823159C0: 80630558  lwz r3, 0x558(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1368 as u32) ) } as u64;
	// 823159C4: 4803ACEC  b 0x823506b0
	sub_823506B0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823159C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823159C8 size=8
    let mut pc: u32 = 0x823159C8;
    'dispatch: loop {
        match pc {
            0x823159C8 => {
    //   block [0x823159C8..0x823159D0)
	// 823159C8: 80630558  lwz r3, 0x558(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1368 as u32) ) } as u64;
	// 823159CC: 4831AE34  b 0x82630800
	sub_82630800(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823159D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823159D0 size=12
    let mut pc: u32 = 0x823159D0;
    'dispatch: loop {
        match pc {
            0x823159D0 => {
    //   block [0x823159D0..0x823159DC)
	// 823159D0: 806304AC  lwz r3, 0x4ac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823159D4: 38800029  li r4, 0x29
	ctx.r[4].s64 = 41;
	// 823159D8: 4801BD70  b 0x82331748
	sub_82331748(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823159E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823159E0 size=12
    let mut pc: u32 = 0x823159E0;
    'dispatch: loop {
        match pc {
            0x823159E0 => {
    //   block [0x823159E0..0x823159EC)
	// 823159E0: 806304AC  lwz r3, 0x4ac(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823159E4: 38800029  li r4, 0x29
	ctx.r[4].s64 = 41;
	// 823159E8: 4801BD70  b 0x82331758
	sub_82331758(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823159F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823159F0 size=112
    let mut pc: u32 = 0x823159F0;
    'dispatch: loop {
        match pc {
            0x823159F0 => {
    //   block [0x823159F0..0x82315A60)
	// 823159F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823159F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823159F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823159FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82315A00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82315A04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82315A08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315A0C: 481D507D  bl 0x824eaa88
	ctx.lr = 0x82315A10;
	sub_824EAA88(ctx, base);
	// 82315A10: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315A14: 481D3235  bl 0x824e8c48
	ctx.lr = 0x82315A18;
	sub_824E8C48(ctx, base);
	// 82315A18: 481D2C81  bl 0x824e8698
	ctx.lr = 0x82315A1C;
	sub_824E8698(ctx, base);
	// 82315A1C: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 82315A20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315A24: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82315A28: 557EDFFE  rlwinm r30, r11, 0x1b, 0x1f, 0x1f
	ctx.r[30].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82315A2C: 48ADC265  bl 0x82df1c90
	ctx.lr = 0x82315A30;
	sub_82DF1C90(ctx, base);
	// 82315A30: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82315A34: 41820014  beq 0x82315a48
	if ctx.cr[0].eq {
	pc = 0x82315A48; continue 'dispatch;
	}
	// 82315A38: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82315A3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82315A40: 997F0D50  stb r11, 0xd50(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(3408 as u32), ctx.r[11].u8 ) };
	// 82315A44: 4BFEFD2D  bl 0x82305770
	ctx.lr = 0x82315A48;
	sub_82305770(ctx, base);
	// 82315A48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82315A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82315A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82315A54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82315A58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82315A5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82315A60 size=20
    let mut pc: u32 = 0x82315A60;
    'dispatch: loop {
        match pc {
            0x82315A60 => {
    //   block [0x82315A60..0x82315A74)
	// 82315A60: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82315A64: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82315A68: 894B0011  lbz r10, 0x11(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 82315A6C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82315A70: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315A74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82315A74 size=40
    let mut pc: u32 = 0x82315A74;
    'dispatch: loop {
        match pc {
            0x82315A74 => {
    //   block [0x82315A74..0x82315A9C)
	// 82315A74: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315A78: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82315A7C: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82315A80: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82315A84: 41980008  blt cr6, 0x82315a8c
	if ctx.cr[6].lt {
	pc = 0x82315A8C; continue 'dispatch;
	}
	// 82315A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82315A8C: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82315A90: 4182000C  beq 0x82315a9c
	if ctx.cr[0].eq {
		sub_82315A9C(ctx, base);
		return;
	}
	// 82315A94: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82315A98: 4800000C  b 0x82315aa4
	sub_82315A9C(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315A9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82315A9C size=24
    let mut pc: u32 = 0x82315A9C;
    'dispatch: loop {
        match pc {
            0x82315A9C => {
    //   block [0x82315A9C..0x82315AB4)
	// 82315A9C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82315AA0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315AA4: 894B0011  lbz r10, 0x11(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(17 as u32) ) } as u64;
	// 82315AA8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82315AAC: 419AFFCC  beq cr6, 0x82315a78
	if ctx.cr[6].eq {
		sub_82315A74(ctx, base);
		return;
	}
	// 82315AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82315AB8 size=20
    let mut pc: u32 = 0x82315AB8;
    'dispatch: loop {
        match pc {
            0x82315AB8 => {
    //   block [0x82315AB8..0x82315ACC)
	// 82315AB8: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82315ABC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82315AC0: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82315AC4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82315AC8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315ACC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82315ACC size=40
    let mut pc: u32 = 0x82315ACC;
    'dispatch: loop {
        match pc {
            0x82315ACC => {
    //   block [0x82315ACC..0x82315AF4)
	// 82315ACC: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82315AD0: C1AB000C  lfs f13, 0xc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82315AD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82315AD8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82315ADC: 41980008  blt cr6, 0x82315ae4
	if ctx.cr[6].lt {
	pc = 0x82315AE4; continue 'dispatch;
	}
	// 82315AE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82315AE4: 554A063F  clrlwi. r10, r10, 0x18
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82315AE8: 4182000C  beq 0x82315af4
	if ctx.cr[0].eq {
		sub_82315AF4(ctx, base);
		return;
	}
	// 82315AEC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82315AF0: 4800000C  b 0x82315afc
	sub_82315AF4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315AF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82315AF4 size=24
    let mut pc: u32 = 0x82315AF4;
    'dispatch: loop {
        match pc {
            0x82315AF4 => {
    //   block [0x82315AF4..0x82315B0C)
	// 82315AF4: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82315AF8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315AFC: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82315B00: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82315B04: 419AFFCC  beq cr6, 0x82315ad0
	if ctx.cr[6].eq {
		sub_82315ACC(ctx, base);
		return;
	}
	// 82315B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82315B10 size=80
    let mut pc: u32 = 0x82315B10;
    'dispatch: loop {
        match pc {
            0x82315B10 => {
    //   block [0x82315B10..0x82315B60)
	// 82315B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82315B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82315B18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82315B1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82315B20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82315B24: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82315B28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82315B2C: 419A0018  beq cr6, 0x82315b44
	if ctx.cr[6].eq {
	pc = 0x82315B44; continue 'dispatch;
	}
	// 82315B30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315B34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82315B38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315B3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82315B40: 4E800421  bctrl
	ctx.lr = 0x82315B44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82315B44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82315B48: 48ADD8E1  bl 0x82df3428
	ctx.lr = 0x82315B4C;
	sub_82DF3428(ctx, base);
	// 82315B4C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82315B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82315B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82315B58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82315B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82315B60 size=196
    let mut pc: u32 = 0x82315B60;
    'dispatch: loop {
        match pc {
            0x82315B60 => {
    //   block [0x82315B60..0x82315C24)
	// 82315B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82315B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82315B68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82315B6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82315B70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82315B74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82315B78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82315B7C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82315B80: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82315B84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82315B88: 4BFAADB1  bl 0x822c0938
	ctx.lr = 0x82315B8C;
	sub_822C0938(ctx, base);
	// 82315B8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82315B90: 41820028  beq 0x82315bb8
	if ctx.cr[0].eq {
	pc = 0x82315BB8; continue 'dispatch;
	}
	// 82315B94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82315B98: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82315B9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82315BA0: 392BD73C  addi r9, r11, -0x28c4
	ctx.r[9].s64 = ctx.r[11].s64 + -10436;
	// 82315BA4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82315BA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82315BAC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82315BB0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82315BB4: 48000008  b 0x82315bbc
	pc = 0x82315BBC; continue 'dispatch;
	// 82315BB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82315BBC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82315BC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82315BC4: 409A0044  bne cr6, 0x82315c08
	if !ctx.cr[6].eq {
	pc = 0x82315C08; continue 'dispatch;
	}
	// 82315BC8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82315BCC: 419A001C  beq cr6, 0x82315be8
	if ctx.cr[6].eq {
	pc = 0x82315BE8; continue 'dispatch;
	}
	// 82315BD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315BD4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82315BD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82315BDC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315BE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82315BE4: 4E800421  bctrl
	ctx.lr = 0x82315BE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82315BE8: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82315BEC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82315BF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315BF4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82315BF8: 816B7850  lwz r11, 0x7850(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30800 as u32) ) } as u64;
	// 82315BFC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82315C00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82315C04: 4BFAA3FD  bl 0x822c0000
	ctx.lr = 0x82315C08;
	sub_822C0000(ctx, base);
	// 82315C08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82315C0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82315C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82315C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82315C18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82315C1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82315C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82315C28 size=196
    let mut pc: u32 = 0x82315C28;
    'dispatch: loop {
        match pc {
            0x82315C28 => {
    //   block [0x82315C28..0x82315CEC)
	// 82315C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82315C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82315C30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82315C34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82315C38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82315C3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82315C40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82315C44: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82315C48: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82315C4C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82315C50: 4BFAACE9  bl 0x822c0938
	ctx.lr = 0x82315C54;
	sub_822C0938(ctx, base);
	// 82315C54: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82315C58: 41820028  beq 0x82315c80
	if ctx.cr[0].eq {
	pc = 0x82315C80; continue 'dispatch;
	}
	// 82315C5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82315C60: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82315C64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82315C68: 392BD750  addi r9, r11, -0x28b0
	ctx.r[9].s64 = ctx.r[11].s64 + -10416;
	// 82315C6C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82315C70: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82315C74: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82315C78: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82315C7C: 48000008  b 0x82315c84
	pc = 0x82315C84; continue 'dispatch;
	// 82315C80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82315C84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82315C88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82315C8C: 409A0044  bne cr6, 0x82315cd0
	if !ctx.cr[6].eq {
	pc = 0x82315CD0; continue 'dispatch;
	}
	// 82315C90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82315C94: 419A001C  beq cr6, 0x82315cb0
	if ctx.cr[6].eq {
	pc = 0x82315CB0; continue 'dispatch;
	}
	// 82315C98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315C9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82315CA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82315CA4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315CA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82315CAC: 4E800421  bctrl
	ctx.lr = 0x82315CB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82315CB0: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82315CB4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82315CB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315CBC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82315CC0: 816B7850  lwz r11, 0x7850(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30800 as u32) ) } as u64;
	// 82315CC4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82315CC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82315CCC: 4BFAA335  bl 0x822c0000
	ctx.lr = 0x82315CD0;
	sub_822C0000(ctx, base);
	// 82315CD0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82315CD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82315CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82315CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82315CE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82315CE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82315CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82315CF0 size=196
    let mut pc: u32 = 0x82315CF0;
    'dispatch: loop {
        match pc {
            0x82315CF0 => {
    //   block [0x82315CF0..0x82315DB4)
	// 82315CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82315CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82315CF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82315CFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82315D00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82315D04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82315D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82315D0C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82315D10: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82315D14: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82315D18: 4BFAAC21  bl 0x822c0938
	ctx.lr = 0x82315D1C;
	sub_822C0938(ctx, base);
	// 82315D1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82315D20: 41820028  beq 0x82315d48
	if ctx.cr[0].eq {
	pc = 0x82315D48; continue 'dispatch;
	}
	// 82315D24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82315D28: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82315D2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82315D30: 392BD764  addi r9, r11, -0x289c
	ctx.r[9].s64 = ctx.r[11].s64 + -10396;
	// 82315D34: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82315D38: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82315D3C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82315D40: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82315D44: 48000008  b 0x82315d4c
	pc = 0x82315D4C; continue 'dispatch;
	// 82315D48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82315D4C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82315D50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82315D54: 409A0044  bne cr6, 0x82315d98
	if !ctx.cr[6].eq {
	pc = 0x82315D98; continue 'dispatch;
	}
	// 82315D58: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82315D5C: 419A001C  beq cr6, 0x82315d78
	if ctx.cr[6].eq {
	pc = 0x82315D78; continue 'dispatch;
	}
	// 82315D60: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315D64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82315D68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82315D6C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315D70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82315D74: 4E800421  bctrl
	ctx.lr = 0x82315D78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82315D78: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82315D7C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82315D80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315D84: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82315D88: 816B7850  lwz r11, 0x7850(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30800 as u32) ) } as u64;
	// 82315D8C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82315D90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82315D94: 4BFAA26D  bl 0x822c0000
	ctx.lr = 0x82315D98;
	sub_822C0000(ctx, base);
	// 82315D98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82315D9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82315DA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82315DA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82315DA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82315DAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82315DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82315DB8 size=196
    let mut pc: u32 = 0x82315DB8;
    'dispatch: loop {
        match pc {
            0x82315DB8 => {
    //   block [0x82315DB8..0x82315E7C)
	// 82315DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82315DBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82315DC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82315DC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82315DC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82315DCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82315DD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82315DD4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82315DD8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82315DDC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82315DE0: 4BFAAB59  bl 0x822c0938
	ctx.lr = 0x82315DE4;
	sub_822C0938(ctx, base);
	// 82315DE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82315DE8: 41820028  beq 0x82315e10
	if ctx.cr[0].eq {
	pc = 0x82315E10; continue 'dispatch;
	}
	// 82315DEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82315DF0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82315DF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82315DF8: 392BD778  addi r9, r11, -0x2888
	ctx.r[9].s64 = ctx.r[11].s64 + -10376;
	// 82315DFC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82315E00: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82315E04: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82315E08: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82315E0C: 48000008  b 0x82315e14
	pc = 0x82315E14; continue 'dispatch;
	// 82315E10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82315E14: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82315E18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82315E1C: 409A0044  bne cr6, 0x82315e60
	if !ctx.cr[6].eq {
	pc = 0x82315E60; continue 'dispatch;
	}
	// 82315E20: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82315E24: 419A001C  beq cr6, 0x82315e40
	if ctx.cr[6].eq {
	pc = 0x82315E40; continue 'dispatch;
	}
	// 82315E28: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315E2C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82315E30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82315E34: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315E38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82315E3C: 4E800421  bctrl
	ctx.lr = 0x82315E40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82315E40: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82315E44: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82315E48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315E4C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82315E50: 816B7850  lwz r11, 0x7850(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30800 as u32) ) } as u64;
	// 82315E54: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82315E58: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82315E5C: 4BFAA1A5  bl 0x822c0000
	ctx.lr = 0x82315E60;
	sub_822C0000(ctx, base);
	// 82315E60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82315E64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82315E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82315E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82315E70: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82315E74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82315E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82315E80 size=196
    let mut pc: u32 = 0x82315E80;
    'dispatch: loop {
        match pc {
            0x82315E80 => {
    //   block [0x82315E80..0x82315F44)
	// 82315E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82315E84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82315E88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82315E8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82315E90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82315E94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82315E98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82315E9C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82315EA0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82315EA4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82315EA8: 4BFAAA91  bl 0x822c0938
	ctx.lr = 0x82315EAC;
	sub_822C0938(ctx, base);
	// 82315EAC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82315EB0: 41820028  beq 0x82315ed8
	if ctx.cr[0].eq {
	pc = 0x82315ED8; continue 'dispatch;
	}
	// 82315EB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82315EB8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82315EBC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82315EC0: 392BD78C  addi r9, r11, -0x2874
	ctx.r[9].s64 = ctx.r[11].s64 + -10356;
	// 82315EC4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82315EC8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82315ECC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82315ED0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82315ED4: 48000008  b 0x82315edc
	pc = 0x82315EDC; continue 'dispatch;
	// 82315ED8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82315EDC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82315EE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82315EE4: 409A0044  bne cr6, 0x82315f28
	if !ctx.cr[6].eq {
	pc = 0x82315F28; continue 'dispatch;
	}
	// 82315EE8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82315EEC: 419A001C  beq cr6, 0x82315f08
	if ctx.cr[6].eq {
	pc = 0x82315F08; continue 'dispatch;
	}
	// 82315EF0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315EF4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82315EF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82315EFC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315F00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82315F04: 4E800421  bctrl
	ctx.lr = 0x82315F08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82315F08: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82315F0C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82315F10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315F14: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82315F18: 816B7850  lwz r11, 0x7850(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30800 as u32) ) } as u64;
	// 82315F1C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82315F20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82315F24: 4BFAA0DD  bl 0x822c0000
	ctx.lr = 0x82315F28;
	sub_822C0000(ctx, base);
	// 82315F28: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82315F2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82315F30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82315F34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82315F38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82315F3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82315F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82315F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82315F48 size=196
    let mut pc: u32 = 0x82315F48;
    'dispatch: loop {
        match pc {
            0x82315F48 => {
    //   block [0x82315F48..0x8231600C)
	// 82315F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82315F4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82315F50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82315F54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82315F58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82315F5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82315F60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82315F64: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82315F68: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82315F6C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82315F70: 4BFAA9C9  bl 0x822c0938
	ctx.lr = 0x82315F74;
	sub_822C0938(ctx, base);
	// 82315F74: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82315F78: 41820028  beq 0x82315fa0
	if ctx.cr[0].eq {
	pc = 0x82315FA0; continue 'dispatch;
	}
	// 82315F7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82315F80: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82315F84: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82315F88: 392BD7A0  addi r9, r11, -0x2860
	ctx.r[9].s64 = ctx.r[11].s64 + -10336;
	// 82315F8C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82315F90: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82315F94: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82315F98: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82315F9C: 48000008  b 0x82315fa4
	pc = 0x82315FA4; continue 'dispatch;
	// 82315FA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82315FA4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82315FA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82315FAC: 409A0044  bne cr6, 0x82315ff0
	if !ctx.cr[6].eq {
	pc = 0x82315FF0; continue 'dispatch;
	}
	// 82315FB0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82315FB4: 419A001C  beq cr6, 0x82315fd0
	if ctx.cr[6].eq {
	pc = 0x82315FD0; continue 'dispatch;
	}
	// 82315FB8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315FBC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82315FC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82315FC4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82315FC8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82315FCC: 4E800421  bctrl
	ctx.lr = 0x82315FD0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82315FD0: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82315FD4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82315FD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82315FDC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82315FE0: 816B7850  lwz r11, 0x7850(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30800 as u32) ) } as u64;
	// 82315FE4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82315FE8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82315FEC: 4BFAA015  bl 0x822c0000
	ctx.lr = 0x82315FF0;
	sub_822C0000(ctx, base);
	// 82315FF0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82315FF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82315FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82315FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82316000: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82316004: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82316008: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82316010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82316010 size=196
    let mut pc: u32 = 0x82316010;
    'dispatch: loop {
        match pc {
            0x82316010 => {
    //   block [0x82316010..0x823160D4)
	// 82316010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82316014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82316018: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8231601C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82316020: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82316024: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82316028: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8231602C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82316030: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82316034: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82316038: 4BFAA901  bl 0x822c0938
	ctx.lr = 0x8231603C;
	sub_822C0938(ctx, base);
	// 8231603C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82316040: 41820028  beq 0x82316068
	if ctx.cr[0].eq {
	pc = 0x82316068; continue 'dispatch;
	}
	// 82316044: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82316048: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8231604C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82316050: 392BD7B4  addi r9, r11, -0x284c
	ctx.r[9].s64 = ctx.r[11].s64 + -10316;
	// 82316054: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82316058: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8231605C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82316060: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82316064: 48000008  b 0x8231606c
	pc = 0x8231606C; continue 'dispatch;
	// 82316068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8231606C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82316070: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82316074: 409A0044  bne cr6, 0x823160b8
	if !ctx.cr[6].eq {
	pc = 0x823160B8; continue 'dispatch;
	}
	// 82316078: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8231607C: 419A001C  beq cr6, 0x82316098
	if ctx.cr[6].eq {
	pc = 0x82316098; continue 'dispatch;
	}
	// 82316080: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82316084: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82316088: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8231608C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82316090: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82316094: 4E800421  bctrl
	ctx.lr = 0x82316098;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82316098: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8231609C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823160A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823160A4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823160A8: 816B7850  lwz r11, 0x7850(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30800 as u32) ) } as u64;
	// 823160AC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823160B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823160B4: 4BFA9F4D  bl 0x822c0000
	ctx.lr = 0x823160B8;
	sub_822C0000(ctx, base);
	// 823160B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823160BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823160C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823160C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823160C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823160CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823160D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823160D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823160D8 size=196
    let mut pc: u32 = 0x823160D8;
    'dispatch: loop {
        match pc {
            0x823160D8 => {
    //   block [0x823160D8..0x8231619C)
	// 823160D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823160DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823160E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823160E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823160E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823160EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823160F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823160F4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823160F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823160FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82316100: 4BFAA839  bl 0x822c0938
	ctx.lr = 0x82316104;
	sub_822C0938(ctx, base);
	// 82316104: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82316108: 41820028  beq 0x82316130
	if ctx.cr[0].eq {
	pc = 0x82316130; continue 'dispatch;
	}
	// 8231610C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82316110: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82316114: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82316118: 392BD7C8  addi r9, r11, -0x2838
	ctx.r[9].s64 = ctx.r[11].s64 + -10296;
	// 8231611C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82316120: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82316124: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82316128: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8231612C: 48000008  b 0x82316134
	pc = 0x82316134; continue 'dispatch;
	// 82316130: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82316134: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82316138: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8231613C: 409A0044  bne cr6, 0x82316180
	if !ctx.cr[6].eq {
	pc = 0x82316180; continue 'dispatch;
	}
	// 82316140: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82316144: 419A001C  beq cr6, 0x82316160
	if ctx.cr[6].eq {
	pc = 0x82316160; continue 'dispatch;
	}
	// 82316148: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8231614C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82316150: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82316154: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82316158: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8231615C: 4E800421  bctrl
	ctx.lr = 0x82316160;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82316160: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82316164: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82316168: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8231616C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82316170: 816B7850  lwz r11, 0x7850(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30800 as u32) ) } as u64;
	// 82316174: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82316178: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8231617C: 4BFA9E85  bl 0x822c0000
	ctx.lr = 0x82316180;
	sub_822C0000(ctx, base);
	// 82316180: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82316184: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82316188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8231618C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82316190: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82316194: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82316198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823161A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823161A0 size=196
    let mut pc: u32 = 0x823161A0;
    'dispatch: loop {
        match pc {
            0x823161A0 => {
    //   block [0x823161A0..0x82316264)
	// 823161A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823161A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823161A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823161AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823161B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823161B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823161B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823161BC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823161C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823161C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823161C8: 4BFAA771  bl 0x822c0938
	ctx.lr = 0x823161CC;
	sub_822C0938(ctx, base);
	// 823161CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823161D0: 41820028  beq 0x823161f8
	if ctx.cr[0].eq {
	pc = 0x823161F8; continue 'dispatch;
	}
	// 823161D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823161D8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823161DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823161E0: 392BD7DC  addi r9, r11, -0x2824
	ctx.r[9].s64 = ctx.r[11].s64 + -10276;
	// 823161E4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823161E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823161EC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823161F0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823161F4: 48000008  b 0x823161fc
	pc = 0x823161FC; continue 'dispatch;
	// 823161F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823161FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82316200: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82316204: 409A0044  bne cr6, 0x82316248
	if !ctx.cr[6].eq {
	pc = 0x82316248; continue 'dispatch;
	}
	// 82316208: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8231620C: 419A001C  beq cr6, 0x82316228
	if ctx.cr[6].eq {
	pc = 0x82316228; continue 'dispatch;
	}
	// 82316210: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82316214: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82316218: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8231621C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82316220: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82316224: 4E800421  bctrl
	ctx.lr = 0x82316228;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82316228: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 8231622C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82316230: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82316234: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82316238: 816B7850  lwz r11, 0x7850(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30800 as u32) ) } as u64;
	// 8231623C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82316240: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82316244: 4BFA9DBD  bl 0x822c0000
	ctx.lr = 0x82316248;
	sub_822C0000(ctx, base);
	// 82316248: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8231624C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82316250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82316254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82316258: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8231625C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82316260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82316268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82316268 size=196
    let mut pc: u32 = 0x82316268;
    'dispatch: loop {
        match pc {
            0x82316268 => {
    //   block [0x82316268..0x8231632C)
	// 82316268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231626C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82316270: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82316274: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82316278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8231627C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82316280: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82316284: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82316288: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8231628C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82316290: 4BFAA6A9  bl 0x822c0938
	ctx.lr = 0x82316294;
	sub_822C0938(ctx, base);
	// 82316294: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82316298: 41820028  beq 0x823162c0
	if ctx.cr[0].eq {
	pc = 0x823162C0; continue 'dispatch;
	}
	// 8231629C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823162A0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823162A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823162A8: 392BD7F0  addi r9, r11, -0x2810
	ctx.r[9].s64 = ctx.r[11].s64 + -10256;
	// 823162AC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823162B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823162B4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823162B8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823162BC: 48000008  b 0x823162c4
	pc = 0x823162C4; continue 'dispatch;
	// 823162C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823162C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823162C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823162CC: 409A0044  bne cr6, 0x82316310
	if !ctx.cr[6].eq {
	pc = 0x82316310; continue 'dispatch;
	}
	// 823162D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823162D4: 419A001C  beq cr6, 0x823162f0
	if ctx.cr[6].eq {
	pc = 0x823162F0; continue 'dispatch;
	}
	// 823162D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823162DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823162E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823162E4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823162E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823162EC: 4E800421  bctrl
	ctx.lr = 0x823162F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823162F0: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 823162F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823162F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823162FC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82316300: 816B7850  lwz r11, 0x7850(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30800 as u32) ) } as u64;
	// 82316304: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82316308: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8231630C: 4BFA9CF5  bl 0x822c0000
	ctx.lr = 0x82316310;
	sub_822C0000(ctx, base);
	// 82316310: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82316314: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82316318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8231631C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82316320: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82316324: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82316328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82316330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82316330 size=196
    let mut pc: u32 = 0x82316330;
    'dispatch: loop {
        match pc {
            0x82316330 => {
    //   block [0x82316330..0x823163F4)
	// 82316330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82316334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82316338: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8231633C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82316340: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82316344: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82316348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8231634C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82316350: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82316354: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82316358: 4BFAA5E1  bl 0x822c0938
	ctx.lr = 0x8231635C;
	sub_822C0938(ctx, base);
	// 8231635C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82316360: 41820028  beq 0x82316388
	if ctx.cr[0].eq {
	pc = 0x82316388; continue 'dispatch;
	}
	// 82316364: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82316368: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8231636C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82316370: 392BD804  addi r9, r11, -0x27fc
	ctx.r[9].s64 = ctx.r[11].s64 + -10236;
	// 82316374: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82316378: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8231637C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82316380: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82316384: 48000008  b 0x8231638c
	pc = 0x8231638C; continue 'dispatch;
	// 82316388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8231638C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82316390: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82316394: 409A0044  bne cr6, 0x823163d8
	if !ctx.cr[6].eq {
	pc = 0x823163D8; continue 'dispatch;
	}
	// 82316398: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8231639C: 419A001C  beq cr6, 0x823163b8
	if ctx.cr[6].eq {
	pc = 0x823163B8; continue 'dispatch;
	}
	// 823163A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823163A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823163A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823163AC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823163B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823163B4: 4E800421  bctrl
	ctx.lr = 0x823163B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823163B8: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 823163BC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823163C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823163C4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823163C8: 816B7850  lwz r11, 0x7850(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30800 as u32) ) } as u64;
	// 823163CC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823163D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823163D4: 4BFA9C2D  bl 0x822c0000
	ctx.lr = 0x823163D8;
	sub_822C0000(ctx, base);
	// 823163D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823163DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823163E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823163E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823163E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823163EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823163F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823163F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823163F8 size=196
    let mut pc: u32 = 0x823163F8;
    'dispatch: loop {
        match pc {
            0x823163F8 => {
    //   block [0x823163F8..0x823164BC)
	// 823163F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823163FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82316400: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82316404: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82316408: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8231640C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82316410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82316414: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82316418: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8231641C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82316420: 4BFAA519  bl 0x822c0938
	ctx.lr = 0x82316424;
	sub_822C0938(ctx, base);
	// 82316424: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82316428: 41820028  beq 0x82316450
	if ctx.cr[0].eq {
	pc = 0x82316450; continue 'dispatch;
	}
	// 8231642C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82316430: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82316434: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82316438: 392BD818  addi r9, r11, -0x27e8
	ctx.r[9].s64 = ctx.r[11].s64 + -10216;
	// 8231643C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82316440: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82316444: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82316448: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8231644C: 48000008  b 0x82316454
	pc = 0x82316454; continue 'dispatch;
	// 82316450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82316454: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82316458: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8231645C: 409A0044  bne cr6, 0x823164a0
	if !ctx.cr[6].eq {
	pc = 0x823164A0; continue 'dispatch;
	}
	// 82316460: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82316464: 419A001C  beq cr6, 0x82316480
	if ctx.cr[6].eq {
	pc = 0x82316480; continue 'dispatch;
	}
	// 82316468: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8231646C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82316470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82316474: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82316478: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8231647C: 4E800421  bctrl
	ctx.lr = 0x82316480;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82316480: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 82316484: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82316488: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8231648C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82316490: 816B7850  lwz r11, 0x7850(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(30800 as u32) ) } as u64;
	// 82316494: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82316498: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8231649C: 4BFA9B65  bl 0x822c0000
	ctx.lr = 0x823164A0;
	sub_822C0000(ctx, base);
	// 823164A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823164A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823164A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823164AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823164B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823164B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823164B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823164C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823164C0 size=12
    let mut pc: u32 = 0x823164C0;
    'dispatch: loop {
        match pc {
            0x823164C0 => {
    //   block [0x823164C0..0x823164CC)
	// 823164C0: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 823164C4: 386B7FC4  addi r3, r11, 0x7fc4
	ctx.r[3].s64 = ctx.r[11].s64 + 32708;
	// 823164C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823164D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823164D0 size=84
    let mut pc: u32 = 0x823164D0;
    'dispatch: loop {
        match pc {
            0x823164D0 => {
    //   block [0x823164D0..0x82316524)
	// 823164D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823164D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823164D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823164DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823164E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823164E4: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 823164E8: 4BFAA451  bl 0x822c0938
	ctx.lr = 0x823164EC;
	sub_822C0938(ctx, base);
	// 823164EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823164F0: 4182001C  beq 0x8231650c
	if ctx.cr[0].eq {
	pc = 0x8231650C; continue 'dispatch;
	}
	// 823164F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823164F8: 396BD87C  addi r11, r11, -0x2784
	ctx.r[11].s64 = ctx.r[11].s64 + -10116;
	// 823164FC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82316500: 897F0004  lbz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82316504: 99630004  stb r11, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82316508: 48000008  b 0x82316510
	pc = 0x82316510; continue 'dispatch;
	// 8231650C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82316510: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82316514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82316518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8231651C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82316520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82316528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82316528 size=200
    let mut pc: u32 = 0x82316528;
    'dispatch: loop {
        match pc {
            0x82316528 => {
    //   block [0x82316528..0x823165F0)
	// 82316528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231652C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82316530: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82316534: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82316538: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8231653C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82316540: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82316544: 419A005C  beq cr6, 0x823165a0
	if ctx.cr[6].eq {
	pc = 0x823165A0; continue 'dispatch;
	}
	// 82316548: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8231654C: 3BFE00A0  addi r31, r30, 0xa0
	ctx.r[31].s64 = ctx.r[30].s64 + 160;
	// 82316550: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82316554: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82316558: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8231655C: 4E800421  bctrl
	ctx.lr = 0x82316560;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82316560: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82316564: 389E0060  addi r4, r30, 0x60
	ctx.r[4].s64 = ctx.r[30].s64 + 96;
	// 82316568: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8231656C: 4BFAE395  bl 0x822c4900
	ctx.lr = 0x82316570;
	sub_822C4900(ctx, base);
	// 82316570: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82316574: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82316578: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 8231657C: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82316580: 13C71C07  vcmpneb. (lvlx128) v30, v7, v3
	tmp.u32 = ctx.r[7].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82316584: 13A81C07  vcmpneb. (lvlx128) v29, v8, v3
	tmp.u32 = ctx.r[8].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82316588: 13891C07  vcmpneb. (lvlx128) v28, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823165F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823165F0 size=232
    let mut pc: u32 = 0x823165F0;
    'dispatch: loop {
        match pc {
            0x823165F0 => {
    //   block [0x823165F0..0x823166D8)
	// 823165F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823165F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823165F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823165FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82316600: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82316604: 807F0200  lwz r3, 0x200(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(512 as u32) ) } as u64;
	// 82316608: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8231660C: 419A0008  beq cr6, 0x82316614
	if ctx.cr[6].eq {
	pc = 0x82316614; continue 'dispatch;
	}
	// 82316610: 4BFAA281  bl 0x822c0890
	ctx.lr = 0x82316614;
	sub_822C0890(ctx, base);
	// 82316614: 807F01F8  lwz r3, 0x1f8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(504 as u32) ) } as u64;
	// 82316618: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8231661C: 419A0008  beq cr6, 0x82316624
	if ctx.cr[6].eq {
	pc = 0x82316624; continue 'dispatch;
	}
	// 82316620: 4BFAA271  bl 0x822c0890
	ctx.lr = 0x82316624;
	sub_822C0890(ctx, base);
	// 82316624: 807F0168  lwz r3, 0x168(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) } as u64;
	// 82316628: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8231662C: 419A0008  beq cr6, 0x82316634
	if ctx.cr[6].eq {
	pc = 0x82316634; continue 'dispatch;
	}
	// 82316630: 4BFAA261  bl 0x822c0890
	ctx.lr = 0x82316634;
	sub_822C0890(ctx, base);
	// 82316634: 807F0160  lwz r3, 0x160(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) } as u64;
	// 82316638: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8231663C: 419A0008  beq cr6, 0x82316644
	if ctx.cr[6].eq {
	pc = 0x82316644; continue 'dispatch;
	}
	// 82316640: 4BFAA251  bl 0x822c0890
	ctx.lr = 0x82316644;
	sub_822C0890(ctx, base);
	// 82316644: 807F0144  lwz r3, 0x144(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(324 as u32) ) } as u64;
	// 82316648: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8231664C: 419A0008  beq cr6, 0x82316654
	if ctx.cr[6].eq {
	pc = 0x82316654; continue 'dispatch;
	}
	// 82316650: 4BFAA241  bl 0x822c0890
	ctx.lr = 0x82316654;
	sub_822C0890(ctx, base);
	// 82316654: 807F0110  lwz r3, 0x110(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(272 as u32) ) } as u64;
	// 82316658: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8231665C: 419A0008  beq cr6, 0x82316664
	if ctx.cr[6].eq {
	pc = 0x82316664; continue 'dispatch;
	}
	// 82316660: 4BFAA231  bl 0x822c0890
	ctx.lr = 0x82316664;
	sub_822C0890(ctx, base);
	// 82316664: 807F0108  lwz r3, 0x108(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(264 as u32) ) } as u64;
	// 82316668: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8231666C: 419A0008  beq cr6, 0x82316674
	if ctx.cr[6].eq {
	pc = 0x82316674; continue 'dispatch;
	}
	// 82316670: 4BFAA221  bl 0x822c0890
	ctx.lr = 0x82316674;
	sub_822C0890(ctx, base);
	// 82316674: 807F0034  lwz r3, 0x34(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 82316678: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8231667C: 419A0008  beq cr6, 0x82316684
	if ctx.cr[6].eq {
	pc = 0x82316684; continue 'dispatch;
	}
	// 82316680: 4BFAA211  bl 0x822c0890
	ctx.lr = 0x82316684;
	sub_822C0890(ctx, base);
	// 82316684: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82316688: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8231668C: 419A0008  beq cr6, 0x82316694
	if ctx.cr[6].eq {
	pc = 0x82316694; continue 'dispatch;
	}
	// 82316690: 4BFAA201  bl 0x822c0890
	ctx.lr = 0x82316694;
	sub_822C0890(ctx, base);
	// 82316694: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82316698: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8231669C: 419A0008  beq cr6, 0x823166a4
	if ctx.cr[6].eq {
	pc = 0x823166A4; continue 'dispatch;
	}
	// 823166A0: 4BFAA1F1  bl 0x822c0890
	ctx.lr = 0x823166A4;
	sub_822C0890(ctx, base);
	// 823166A4: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 823166A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823166AC: 419A0008  beq cr6, 0x823166b4
	if ctx.cr[6].eq {
	pc = 0x823166B4; continue 'dispatch;
	}
	// 823166B0: 4BFAA1E1  bl 0x822c0890
	ctx.lr = 0x823166B4;
	sub_822C0890(ctx, base);
	// 823166B4: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 823166B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823166BC: 419A0008  beq cr6, 0x823166c4
	if ctx.cr[6].eq {
	pc = 0x823166C4; continue 'dispatch;
	}
	// 823166C0: 4BFAA1D1  bl 0x822c0890
	ctx.lr = 0x823166C4;
	sub_822C0890(ctx, base);
	// 823166C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823166C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823166CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823166D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823166D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823166D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823166D8 size=100
    let mut pc: u32 = 0x823166D8;
    'dispatch: loop {
        match pc {
            0x823166D8 => {
    //   block [0x823166D8..0x8231673C)
	// 823166D8: 89440000  lbz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 823166DC: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 823166E0: 396302E0  addi r11, r3, 0x2e0
	ctx.r[11].s64 = ctx.r[3].s64 + 736;
	// 823166E4: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 823166E8: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 823166EC: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	// 823166F0: 994302E0  stb r10, 0x2e0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(736 as u32), ctx.r[10].u8 ) };
	// 823166F4: 13E430C7  vcmpequd (lvx128) v31, v4, v6
	tmp.u32 = ctx.r[4].u32 + ctx.r[6].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8231673C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8231673C size=20
    let mut pc: u32 = 0x8231673C;
    'dispatch: loop {
        match pc {
            0x8231673C => {
    //   block [0x8231673C..0x82316750)
	// 8231673C: 396002F0  li r11, 0x2f0
	ctx.r[11].s64 = 752;
	// 82316740: 39400430  li r10, 0x430
	ctx.r[10].s64 = 1072;
	// 82316744: 13E358C7  vcmpequd (lvx128) v31, v3, v11
	tmp.u32 = ctx.r[3].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82316750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82316750 size=1044
    let mut pc: u32 = 0x82316750;
    'dispatch: loop {
        match pc {
            0x82316750 => {
    //   block [0x82316750..0x82316B64)
	// 82316750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82316754: 48E91A05  bl 0x831a8158
	ctx.lr = 0x82316758;
	sub_831A8130(ctx, base);
	// 82316758: 3981FFB8  addi r12, r1, -0x48
	ctx.r[12].s64 = ctx.r[1].s64 + -72;
	// 8231675C: 48E9230D  bl 0x831a8a68
	ctx.lr = 0x82316760;
	sub_831A8A40(ctx, base);
	// 82316760: 3980FF60  li r12, -0xa0
	ctx.r[12].s64 = -160;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82316B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82316B68 size=112
    let mut pc: u32 = 0x82316B68;
    'dispatch: loop {
        match pc {
            0x82316B68 => {
    //   block [0x82316B68..0x82316BD8)
	// 82316B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82316B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82316B70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82316B74: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82316B78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82316B7C: 80640588  lwz r3, 0x588(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(1416 as u32) ) } as u64;
	// 82316B80: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82316B84: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82316B88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82316B8C: 4E800421  bctrl
	ctx.lr = 0x82316B90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82316B90: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82316B94: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82316B98: 13EB1C07  vcmpneb. (lvlx128) v31, v11, v3
	tmp.u32 = ctx.r[11].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82316B9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82316BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82316BD8 size=136
    let mut pc: u32 = 0x82316BD8;
    'dispatch: loop {
        match pc {
            0x82316BD8 => {
    //   block [0x82316BD8..0x82316C60)
	// 82316BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82316BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82316BE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82316BE4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82316BE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82316BEC: 80640588  lwz r3, 0x588(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(1416 as u32) ) } as u64;
	// 82316BF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82316BF4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82316BF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82316BFC: 4E800421  bctrl
	ctx.lr = 0x82316C00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82316C00: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 82316C04: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82316C08: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82316C0C: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82316C10: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82316C14: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82316C18: 13CB1C07  vcmpneb. (lvlx128) v30, v11, v3
	tmp.u32 = ctx.r[11].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82316C1C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82316C20: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82316C24: 13AA1C07  vcmpneb. (lvlx128) v29, v10, v3
	tmp.u32 = ctx.r[10].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82316C28: 13891C07  vcmpneb. (lvlx128) v28, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82316C2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82316C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82316C60 size=556
    let mut pc: u32 = 0x82316C60;
    'dispatch: loop {
        match pc {
            0x82316C60 => {
    //   block [0x82316C60..0x82316E8C)
	// 82316C60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82316C64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82316C68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82316C6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82316C70: 3981FFE8  addi r12, r1, -0x18
	ctx.r[12].s64 = ctx.r[1].s64 + -24;
	// 82316C74: 48E91E01  bl 0x831a8a74
	ctx.lr = 0x82316C78;
	sub_831A8A40(ctx, base);
	// 82316C78: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82316C7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82316C80: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 82316C84: 3BDF0250  addi r30, r31, 0x250
	ctx.r[30].s64 = ctx.r[31].s64 + 592;
	// 82316C88: 38BF0210  addi r5, r31, 0x210
	ctx.r[5].s64 = ctx.r[31].s64 + 528;
	// 82316C8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82316C90: 48B650B9  bl 0x82e7bd48
	ctx.lr = 0x82316C94;
	sub_82E7BD48(ctx, base);
	// 82316C94: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82316C98: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82316E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82316E90 size=140
    let mut pc: u32 = 0x82316E90;
    'dispatch: loop {
        match pc {
            0x82316E90 => {
    //   block [0x82316E90..0x82316F1C)
	// 82316E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82316E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82316E98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82316F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82316F20 size=180
    let mut pc: u32 = 0x82316F20;
    'dispatch: loop {
        match pc {
            0x82316F20 => {
    //   block [0x82316F20..0x82316FD4)
	// 82316F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82316F24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82316F28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82316F2C: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82316F30: 39630460  addi r11, r3, 0x460
	ctx.r[11].s64 = ctx.r[3].s64 + 1120;
	// 82316F34: D0030460  stfs f0, 0x460(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1120 as u32), tmp.u32 ) };
	// 82316F38: 54AA063F  clrlwi. r10, r5, 0x18
	ctx.r[10].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82316F3C: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82316F40: D0030464  stfs f0, 0x464(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1124 as u32), tmp.u32 ) };
	// 82316F44: C0040008  lfs f0, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82316F48: D0030468  stfs f0, 0x468(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1128 as u32), tmp.u32 ) };
	// 82316F4C: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82316F50: D003046C  stfs f0, 0x46c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1132 as u32), tmp.u32 ) };
	// 82316F54: 41820070  beq 0x82316fc4
	if ctx.cr[0].eq {
	pc = 0x82316FC4; continue 'dispatch;
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82316FD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82316FD8 size=4
    let mut pc: u32 = 0x82316FD8;
    'dispatch: loop {
        match pc {
            0x82316FD8 => {
    //   block [0x82316FD8..0x82316FDC)
	// 82316FD8: 4BFFFF48  b 0x82316f20
	sub_82316F20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82316FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82316FE0 size=128
    let mut pc: u32 = 0x82316FE0;
    'dispatch: loop {
        match pc {
            0x82316FE0 => {
    //   block [0x82316FE0..0x82317060)
	// 82316FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82316FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82316FE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82316FEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82316FF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82316FF4: 897F0565  lbz r11, 0x565(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1381 as u32) ) } as u64;
	// 82316FF8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82316FFC: 41820008  beq 0x82317004
	if ctx.cr[0].eq {
	pc = 0x82317004; continue 'dispatch;
	}
	// 82317000: 4BFFFC61  bl 0x82316c60
	ctx.lr = 0x82317004;
	sub_82316C60(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82317060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82317060 size=88
    let mut pc: u32 = 0x82317060;
    'dispatch: loop {
        match pc {
            0x82317060 => {
    //   block [0x82317060..0x823170B8)
	// 82317060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82317064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82317068: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8231706C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82317070: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82317074: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82317078: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8231707C: 897F0565  lbz r11, 0x565(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1381 as u32) ) } as u64;
	// 82317080: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82317084: 4182000C  beq 0x82317090
	if ctx.cr[0].eq {
	pc = 0x82317090; continue 'dispatch;
	}
	// 82317088: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8231708C: 4BFFFBD5  bl 0x82316c60
	ctx.lr = 0x82317090;
	sub_82316C60(ctx, base);
	// 82317090: 389F0220  addi r4, r31, 0x220
	ctx.r[4].s64 = ctx.r[31].s64 + 544;
	// 82317094: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82317098: 48B663B9  bl 0x82e7d450
	ctx.lr = 0x8231709C;
	sub_82E7D450(ctx, base);
	// 8231709C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823170A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823170A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823170A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823170AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823170B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823170B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823170B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823170B8 size=88
    let mut pc: u32 = 0x823170B8;
    'dispatch: loop {
        match pc {
            0x823170B8 => {
    //   block [0x823170B8..0x82317110)
	// 823170B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823170BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823170C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823170C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823170C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823170CC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823170D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823170D4: 897F0565  lbz r11, 0x565(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1381 as u32) ) } as u64;
	// 823170D8: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823170DC: 4182000C  beq 0x823170e8
	if ctx.cr[0].eq {
	pc = 0x823170E8; continue 'dispatch;
	}
	// 823170E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823170E4: 4BFFFB7D  bl 0x82316c60
	ctx.lr = 0x823170E8;
	sub_82316C60(ctx, base);
	// 823170E8: 39600230  li r11, 0x230
	ctx.r[11].s64 = 560;
	// 823170EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823170F0: 13FF58C7  vcmpequd (lvx128) v31, v31, v11
	tmp.u32 = ctx.r[31].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82317110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82317110 size=128
    let mut pc: u32 = 0x82317110;
    'dispatch: loop {
        match pc {
            0x82317110 => {
    //   block [0x82317110..0x82317190)
	// 82317110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82317114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82317118: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8231711C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82317120: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82317124: 897F0565  lbz r11, 0x565(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1381 as u32) ) } as u64;
	// 82317128: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8231712C: 41820008  beq 0x82317134
	if ctx.cr[0].eq {
	pc = 0x82317134; continue 'dispatch;
	}
	// 82317130: 4BFFFB31  bl 0x82316c60
	ctx.lr = 0x82317134;
	sub_82316C60(ctx, base);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82317190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82317190 size=88
    let mut pc: u32 = 0x82317190;
    'dispatch: loop {
        match pc {
            0x82317190 => {
    //   block [0x82317190..0x823171E8)
	// 82317190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82317194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82317198: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8231719C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823171A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823171A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823171A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823171AC: 897F0565  lbz r11, 0x565(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1381 as u32) ) } as u64;
	// 823171B0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823171B4: 4182000C  beq 0x823171c0
	if ctx.cr[0].eq {
	pc = 0x823171C0; continue 'dispatch;
	}
	// 823171B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823171BC: 4BFFFAA5  bl 0x82316c60
	ctx.lr = 0x823171C0;
	sub_82316C60(ctx, base);
	// 823171C0: 389F0230  addi r4, r31, 0x230
	ctx.r[4].s64 = ctx.r[31].s64 + 560;
	// 823171C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823171C8: 48B66289  bl 0x82e7d450
	ctx.lr = 0x823171CC;
	sub_82E7D450(ctx, base);
	// 823171CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823171D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823171D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823171D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823171DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823171E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823171E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823171E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823171E8 size=24
    let mut pc: u32 = 0x823171E8;
    'dispatch: loop {
        match pc {
            0x823171E8 => {
    //   block [0x823171E8..0x82317200)
	// 823171E8: 81640498  lwz r11, 0x498(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(1176 as u32) ) } as u64;
	// 823171EC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823171F0: 8164049C  lwz r11, 0x49c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(1180 as u32) ) } as u64;
	// 823171F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823171F8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823171FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82317200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82317200 size=36
    let mut pc: u32 = 0x82317200;
    'dispatch: loop {
        match pc {
            0x82317200 => {
    //   block [0x82317200..0x82317224)
	// 82317200: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82317204: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82317208: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8231720C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82317210: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82317214: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82317218: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 8231721C: 4082FFE8  bne 0x82317204
	if !ctx.cr[0].eq {
	pc = 0x82317204; continue 'dispatch;
	}
	// 82317220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82317228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82317228 size=128
    let mut pc: u32 = 0x82317228;
    'dispatch: loop {
        match pc {
            0x82317228 => {
    //   block [0x82317228..0x823172A8)
	// 82317228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231722C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82317230: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82317234: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82317238: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8231723C: 3880001F  li r4, 0x1f
	ctx.r[4].s64 = 31;
	// 82317240: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82317244: 4801A4ED  bl 0x82331730
	ctx.lr = 0x82317248;
	sub_82331730(ctx, base);
	// 82317248: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8231724C: 40820044  bne 0x82317290
	if !ctx.cr[0].eq {
	pc = 0x82317290; continue 'dispatch;
	}
	// 82317250: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82317254: 807F04B0  lwz r3, 0x4b0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1200 as u32) ) } as u64;
	// 82317258: 4801A4D9  bl 0x82331730
	ctx.lr = 0x8231725C;
	sub_82331730(ctx, base);
	// 8231725C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82317260: 41820030  beq 0x82317290
	if ctx.cr[0].eq {
	pc = 0x82317290; continue 'dispatch;
	}
	// 82317264: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 82317268: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8231726C: 808BB3A0  lwz r4, -0x4c60(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-19552 as u32) ) } as u64;
	// 82317270: 48ADC799  bl 0x82df3a08
	ctx.lr = 0x82317274;
	sub_82DF3A08(ctx, base);
	// 82317274: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82317278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8231727C: 4BFEE315  bl 0x82305590
	ctx.lr = 0x82317280;
	sub_82305590(ctx, base);
	// 82317280: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82317284: 48ADC1A5  bl 0x82df3428
	ctx.lr = 0x82317288;
	sub_82DF3428(ctx, base);
	// 82317288: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8231728C: 48000008  b 0x82317294
	pc = 0x82317294; continue 'dispatch;
	// 82317290: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82317294: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82317298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8231729C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823172A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823172A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823172A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823172A8 size=384
    let mut pc: u32 = 0x823172A8;
    'dispatch: loop {
        match pc {
            0x823172A8 => {
    //   block [0x823172A8..0x82317428)
	// 823172A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823172AC: 48E90EC1  bl 0x831a816c
	ctx.lr = 0x823172B0;
	sub_831A8130(ctx, base);
	// 823172B0: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 823172B4: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823172B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823172BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823172C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823172C4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 823172C8: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823172CC: 4801A465  bl 0x82331730
	ctx.lr = 0x823172D0;
	sub_82331730(ctx, base);
	// 823172D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823172D4: 4182004C  beq 0x82317320
	if ctx.cr[0].eq {
	pc = 0x82317320; continue 'dispatch;
	}
	// 823172D8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 823172DC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 823172E0: 396B6880  addi r11, r11, 0x6880
	ctx.r[11].s64 = ctx.r[11].s64 + 26752;
	// 823172E4: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 823172E8: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 823172EC: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 823172F0: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 823172F4: 38A100A0  addi r5, r1, 0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + 160;
	// 823172F8: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823172FC: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82317300: 13CA5C07  vcmpneb. (lvlx128) v30, v10, v11
	tmp.u32 = ctx.r[10].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82317304: 13A95C07  vcmpneb. (lvlx128) v29, v9, v11
	tmp.u32 = ctx.r[9].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82317308: 13885C07  vcmpneb. (lvlx128) v28, v8, v11
	tmp.u32 = ctx.r[8].u32 + ctx.r[11].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82317428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82317428 size=164
    let mut pc: u32 = 0x82317428;
    'dispatch: loop {
        match pc {
            0x82317428 => {
    //   block [0x82317428..0x823174CC)
	// 82317428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231742C: 48E90D41  bl 0x831a816c
	ctx.lr = 0x82317430;
	sub_831A8130(ctx, base);
	// 82317430: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82317434: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82317438: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8231743C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82317440: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82317444: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 82317448: 13C0E8C7  vcmpequd (lvx128) v30, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823174D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823174D0 size=200
    let mut pc: u32 = 0x823174D0;
    'dispatch: loop {
        match pc {
            0x823174D0 => {
    //   block [0x823174D0..0x82317598)
	// 823174D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823174D4: 48E90C99  bl 0x831a816c
	ctx.lr = 0x823174D8;
	sub_831A8130(ctx, base);
	// 823174D8: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 823174DC: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823174E0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823174E4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 823174E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823174EC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 823174F0: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823174F4: 13C0E8C7  vcmpequd (lvx128) v30, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82317598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82317598 size=36
    let mut pc: u32 = 0x82317598;
    'dispatch: loop {
        match pc {
            0x82317598 => {
    //   block [0x82317598..0x823175BC)
	// 82317598: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8231759C: C1A306C0  lfs f13, 0x6c0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1728 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823175A0: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823175A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823175A8: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 823175AC: 40980008  bge cr6, 0x823175b4
	if !ctx.cr[6].lt {
	pc = 0x823175B4; continue 'dispatch;
	}
	// 823175B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823175B4: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823175B8: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823175BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823175BC size=24
    let mut pc: u32 = 0x823175BC;
    'dispatch: loop {
        match pc {
            0x823175BC => {
    //   block [0x823175BC..0x823175D4)
	// 823175BC: 396306A0  addi r11, r3, 0x6a0
	ctx.r[11].s64 = ctx.r[3].s64 + 1696;
	// 823175C0: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823175C4: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823175D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823175D8 size=188
    let mut pc: u32 = 0x823175D8;
    'dispatch: loop {
        match pc {
            0x823175D8 => {
    //   block [0x823175D8..0x82317694)
	// 823175D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823175DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823175E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823175E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823175E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823175EC: 388406B0  addi r4, r4, 0x6b0
	ctx.r[4].s64 = ctx.r[4].s64 + 1712;
	// 823175F0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 823175F4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 823175F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823175FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82317600: 13E020C7  vcmpequd (lvx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82317698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82317698 size=540
    let mut pc: u32 = 0x82317698;
    'dispatch: loop {
        match pc {
            0x82317698 => {
    //   block [0x82317698..0x823178B4)
	// 82317698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8231769C: 48E90AD1  bl 0x831a816c
	ctx.lr = 0x823176A0;
	sub_831A8130(ctx, base);
	// 823176A0: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 823176A4: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 823176A8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823176AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823176B0: 4BFFF931  bl 0x82316fe0
	ctx.lr = 0x823176B4;
	sub_82316FE0(ctx, base);
	// 823176B4: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 823176B8: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 823176BC: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823176C0: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 823176C4: 4801A06D  bl 0x82331730
	ctx.lr = 0x823176C8;
	sub_82331730(ctx, base);
	// 823176C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823176CC: 546A063F  clrlwi. r10, r3, 0x18
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823176D0: C3CB08A4  lfs f30, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 823176D4: 40820018  bne 0x823176ec
	if !ctx.cr[0].eq {
	pc = 0x823176EC; continue 'dispatch;
	}
	// 823176D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823176DC: C00B9C28  lfs f0, -0x63d8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-25560 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823176E0: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 823176E4: 40980008  bge cr6, 0x823176ec
	if !ctx.cr[6].lt {
	pc = 0x823176EC; continue 'dispatch;
	}
	// 823176E8: D3C10050  stfs f30, 0x50(r1)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 823176EC: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 823176F0: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 823176F4: 4801A03D  bl 0x82331730
	ctx.lr = 0x823176F8;
	sub_82331730(ctx, base);
	// 823176F8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823176FC: 3BA003D0  li r29, 0x3d0
	ctx.r[29].s64 = 976;
	// 82317700: 41820014  beq 0x82317714
	if ctx.cr[0].eq {
	pc = 0x82317714; continue 'dispatch;
	}
	// 82317704: 3BDF02D0  addi r30, r31, 0x2d0
	ctx.r[30].s64 = ctx.r[31].s64 + 720;
	// 82317708: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8231770C: 48B65C65  bl 0x82e7d370
	ctx.lr = 0x82317710;
	sub_82E7D370(ctx, base);
	// 82317710: 48000118  b 0x82317828
	pc = 0x82317828; continue 'dispatch;
	// 82317714: 897F03C0  lbz r11, 0x3c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(960 as u32) ) } as u64;
	// 82317718: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 8231771C: 40820094  bne 0x823177b0
	if !ctx.cr[0].eq {
	pc = 0x823177B0; continue 'dispatch;
	}
	// 82317720: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 82317724: 807F04AC  lwz r3, 0x4ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1196 as u32) ) } as u64;
	// 82317728: 4801A009  bl 0x82331730
	ctx.lr = 0x8231772C;
	sub_82331730(ctx, base);
	// 8231772C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82317730: 40820080  bne 0x823177b0
	if !ctx.cr[0].eq {
	pc = 0x823177B0; continue 'dispatch;
	}
	// 82317734: 3BDF02D0  addi r30, r31, 0x2d0
	ctx.r[30].s64 = ctx.r[31].s64 + 720;
	// 82317738: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 8231773C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82317740: 13E0F0C7  vcmpequd (lvx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823178B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823178B8 size=24
    let mut pc: u32 = 0x823178B8;
    'dispatch: loop {
        match pc {
            0x823178B8 => {
    //   block [0x823178B8..0x823178D0)
	// 823178B8: 816406C4  lwz r11, 0x6c4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(1732 as u32) ) } as u64;
	// 823178BC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823178C0: 816406C8  lwz r11, 0x6c8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(1736 as u32) ) } as u64;
	// 823178C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823178C8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 823178CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823178D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823178D0 size=36
    let mut pc: u32 = 0x823178D0;
    'dispatch: loop {
        match pc {
            0x823178D0 => {
    //   block [0x823178D0..0x823178F4)
	// 823178D0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 823178D4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823178D8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823178DC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823178E0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 823178E4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823178E8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823178EC: 4082FFE8  bne 0x823178d4
	if !ctx.cr[0].eq {
	pc = 0x823178D4; continue 'dispatch;
	}
	// 823178F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823178F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823178F8 size=116
    let mut pc: u32 = 0x823178F8;
    'dispatch: loop {
        match pc {
            0x823178F8 => {
    //   block [0x823178F8..0x8231796C)
	// 823178F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823178FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82317900: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82317904: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82317908: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8231790C: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82317910: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82317914: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82317918: 397F06C4  addi r11, r31, 0x6c4
	ctx.r[11].s64 = ctx.r[31].s64 + 1732;
	// 8231791C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82317920: 915F06C4  stw r10, 0x6c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1732 as u32), ctx.r[10].u32 ) };
	// 82317924: 4BFACB3D  bl 0x822c4460
	ctx.lr = 0x82317928;
	sub_822C4460(ctx, base);
	// 82317928: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8231792C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82317930: 83DF06C4  lwz r30, 0x6c4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1732 as u32) ) } as u64;
	// 82317934: 4BFF1465  bl 0x82308d98
	ctx.lr = 0x82317938;
	sub_82308D98(ctx, base);
	// 82317938: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8231793C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82317940: 48AFB699  bl 0x82e12fd8
	ctx.lr = 0x82317944;
	sub_82E12FD8(ctx, base);
	// 82317944: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82317948: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8231794C: 419A0008  beq cr6, 0x82317954
	if ctx.cr[6].eq {
	pc = 0x82317954; continue 'dispatch;
	}
	// 82317950: 4BFA8F41  bl 0x822c0890
	ctx.lr = 0x82317954;
	sub_822C0890(ctx, base);
	// 82317954: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82317958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8231795C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82317960: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82317964: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82317968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82317970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82317970 size=220
    let mut pc: u32 = 0x82317970;
    'dispatch: loop {
        match pc {
            0x82317970 => {
    //   block [0x82317970..0x82317A4C)
	// 82317970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82317974: 48E907F5  bl 0x831a8168
	ctx.lr = 0x82317978;
	sub_831A8130(ctx, base);
	// 82317978: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8231797C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82317980: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82317984: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82317988: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8231798C: 817F06CC  lwz r11, 0x6cc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1740 as u32) ) } as u64;
	// 82317990: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82317994: 409A00B0  bne cr6, 0x82317a44
	if !ctx.cr[6].eq {
	pc = 0x82317A44; continue 'dispatch;
	}
	// 82317998: 4BFFFF61  bl 0x823178f8
	ctx.lr = 0x8231799C;
	sub_823178F8(ctx, base);
	// 8231799C: 38800070  li r4, 0x70
	ctx.r[4].s64 = 112;
	// 823179A0: 807F01FC  lwz r3, 0x1fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(508 as u32) ) } as u64;
	// 823179A4: 48143445  bl 0x8245ade8
	ctx.lr = 0x823179A8;
	sub_8245ADE8(ctx, base);
	// 823179A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823179AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823179B0: D03F06D4  stfs f1, 0x6d4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1748 as u32), tmp.u32 ) };
	// 823179B4: 3D208326  lis r9, -0x7cda
	ctx.r[9].s64 = -2094661632;
	// 823179B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823179BC: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823179C0: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823179C4: D01F06D0  stfs f0, 0x6d0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1744 as u32), tmp.u32 ) };
	// 823179C8: D01F06D8  stfs f0, 0x6d8(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1752 as u32), tmp.u32 ) };
	// 823179CC: D1BF06DC  stfs f13, 0x6dc(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1756 as u32), tmp.u32 ) };
	// 823179D0: 93DF06CC  stw r30, 0x6cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1740 as u32), ctx.r[30].u32 ) };
	// 823179D4: 8089B3C0  lwz r4, -0x4c40(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-19520 as u32) ) } as u64;
	// 823179D8: 48ADC031  bl 0x82df3a08
	ctx.lr = 0x823179DC;
	sub_82DF3A08(ctx, base);
	// 823179DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823179E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823179E4: 4BFEDBAD  bl 0x82305590
	ctx.lr = 0x823179E8;
	sub_82305590(ctx, base);
	// 823179E8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823179EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823179F0: 48ADBA39  bl 0x82df3428
	ctx.lr = 0x823179F4;
	sub_82DF3428(ctx, base);
	// 823179F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 823179F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823179FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82317A00: 4801DDE9  bl 0x823357e8
	ctx.lr = 0x82317A04;
	sub_823357E8(ctx, base);
	// 82317A04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82317A08: 4BFFC1F9  bl 0x82313c00
	ctx.lr = 0x82317A0C;
	sub_82313C00(ctx, base);
	// 82317A0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82317A10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82317A14: 388BD890  addi r4, r11, -0x2770
	ctx.r[4].s64 = ctx.r[11].s64 + -10096;
	// 82317A18: 48ADBFF1  bl 0x82df3a08
	ctx.lr = 0x82317A1C;
	sub_82DF3A08(ctx, base);
	// 82317A1C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82317A20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82317A24: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82317A28: 4BFF1311  bl 0x82308d38
	ctx.lr = 0x82317A2C;
	sub_82308D38(ctx, base);
	// 82317A2C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82317A30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82317A34: 419A0008  beq cr6, 0x82317a3c
	if ctx.cr[6].eq {
	pc = 0x82317A3C; continue 'dispatch;
	}
	// 82317A38: 4BFA8E59  bl 0x822c0890
	ctx.lr = 0x82317A3C;
	sub_822C0890(ctx, base);
	// 82317A3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82317A40: 48ADB9E9  bl 0x82df3428
	ctx.lr = 0x82317A44;
	sub_82DF3428(ctx, base);
	// 82317A44: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82317A48: 48E90770  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82317A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82317A50 size=100
    let mut pc: u32 = 0x82317A50;
    'dispatch: loop {
        match pc {
            0x82317A50 => {
    //   block [0x82317A50..0x82317AB4)
	// 82317A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82317A54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82317A58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82317A5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82317A60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82317A64: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82317A68: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 82317A6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82317A70: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 82317A74: 48143375  bl 0x8245ade8
	ctx.lr = 0x82317A78;
	sub_8245ADE8(ctx, base);
	// 82317A78: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82317A7C: D0210050  stfs f1, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 82317A80: 394004D0  li r10, 0x4d0
	ctx.r[10].s64 = 1232;
	// 82317A84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82317A88: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82317AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82317AB8 size=76
    let mut pc: u32 = 0x82317AB8;
    'dispatch: loop {
        match pc {
            0x82317AB8 => {
    //   block [0x82317AB8..0x82317B04)
	// 82317AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82317ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82317AC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82317AC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82317AC8: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82317ACC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82317AD0: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 82317AD4: 397F06E0  addi r11, r31, 0x6e0
	ctx.r[11].s64 = ctx.r[31].s64 + 1760;
	// 82317AD8: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82317ADC: 915F06E0  stw r10, 0x6e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1760 as u32), ctx.r[10].u32 ) };
	// 82317AE0: 4BFAC981  bl 0x822c4460
	ctx.lr = 0x82317AE4;
	sub_822C4460(ctx, base);
	// 82317AE4: 389F010C  addi r4, r31, 0x10c
	ctx.r[4].s64 = ctx.r[31].s64 + 268;
	// 82317AE8: 807F06E0  lwz r3, 0x6e0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1760 as u32) ) } as u64;
	// 82317AEC: 4BFF5BCD  bl 0x8230d6b8
	ctx.lr = 0x82317AF0;
	sub_8230D6B8(ctx, base);
	// 82317AF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82317AF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82317AF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82317AFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82317B00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


