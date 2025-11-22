pub fn sub_82E2A5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E2A5C8 size=24
    let mut pc: u32 = 0x82E2A5C8;
    'dispatch: loop {
        match pc {
            0x82E2A5C8 => {
    //   block [0x82E2A5C8..0x82E2A5E0)
	// 82E2A5C8: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E2A5CC: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2A5D0: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82E2A5D4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82E2A5D8: 7C2B552E  stfsx f1, r11, r10
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	// 82E2A5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2A5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82E2A5E0 size=48
    let mut pc: u32 = 0x82E2A5E0;
    'dispatch: loop {
        match pc {
            0x82E2A5E0 => {
    //   block [0x82E2A5E0..0x82E2A610)
	// 82E2A5E0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2A5E4: 548A2036  slwi r10, r4, 4
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82E2A5E8: C0050000  lfs f0, 0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E2A5EC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82E2A5F0: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E2A5F4: C0050004  lfs f0, 4(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E2A5F8: D00B0004  stfs f0, 4(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82E2A5FC: C0050008  lfs f0, 8(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E2A600: D00B0008  stfs f0, 8(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 82E2A604: C005000C  lfs f0, 0xc(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E2A608: D00B000C  stfs f0, 0xc(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 82E2A60C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2A610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E2A610 size=172
    let mut pc: u32 = 0x82E2A610;
    'dispatch: loop {
        match pc {
            0x82E2A610 => {
    //   block [0x82E2A610..0x82E2A6BC)
	// 82E2A610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2A614: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2A618: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2A61C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2A620: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2A624: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2A628: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2A62C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2A630: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2A634: 4BFCEEFD  bl 0x82df9530
	ctx.lr = 0x82E2A638;
	sub_82DF9530(ctx, base);
	// 82E2A638: 3D603FFF  lis r11, 0x3fff
	ctx.r[11].s64 = 1073676288;
	// 82E2A63C: 616AFFFF  ori r10, r11, 0xffff
	ctx.r[10].u64 = ctx.r[11].u64 | 65535;
	// 82E2A640: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E2A644: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2A648: 893E0002  lbz r9, 2(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82E2A64C: 552B103E  rotlwi r11, r9, 2
	ctx.r[11].u64 = ((ctx.r[9].u32).rotate_left(2)) as u64;
	// 82E2A650: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E2A654: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E2A658: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E2A65C: 40990008  ble cr6, 0x82e2a664
	if !ctx.cr[6].gt {
	pc = 0x82E2A664; continue 'dispatch;
	}
	// 82E2A660: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82E2A664: 4BFC5F1D  bl 0x82df0580
	ctx.lr = 0x82E2A668;
	sub_82DF0580(ctx, base);
	// 82E2A668: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82E2A66C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2A670: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E2A674: 4BFEC5ED  bl 0x82e16c60
	ctx.lr = 0x82E2A678;
	sub_82E16C60(ctx, base);
	// 82E2A678: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2A67C: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2A680: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2A684: 556B103B  rlwinm. r11, r11, 2, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2A688: 4182001C  beq 0x82e2a6a4
	if ctx.cr[0].eq {
	pc = 0x82E2A6A4; continue 'dispatch;
	}
	// 82E2A68C: C0090000  lfs f0, 0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82E2A690: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2A694: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 82E2A698: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E2A69C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82E2A6A0: 4082FFEC  bne 0x82e2a68c
	if !ctx.cr[0].eq {
	pc = 0x82E2A68C; continue 'dispatch;
	}
	// 82E2A6A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2A6A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2A6AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2A6B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2A6B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2A6B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2A6C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2A6C0 size=172
    let mut pc: u32 = 0x82E2A6C0;
    'dispatch: loop {
        match pc {
            0x82E2A6C0 => {
    //   block [0x82E2A6C0..0x82E2A76C)
	// 82E2A6C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2A6C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2A6C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2A6CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2A6D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2A6D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2A6D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2A6DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2A6E0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2A6E4: 4BFCEE4D  bl 0x82df9530
	ctx.lr = 0x82E2A6E8;
	sub_82DF9530(ctx, base);
	// 82E2A6E8: 3D603FFF  lis r11, 0x3fff
	ctx.r[11].s64 = 1073676288;
	// 82E2A6EC: 616AFFFF  ori r10, r11, 0xffff
	ctx.r[10].u64 = ctx.r[11].u64 | 65535;
	// 82E2A6F0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E2A6F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2A6F8: 893E0002  lbz r9, 2(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82E2A6FC: 552B103E  rotlwi r11, r9, 2
	ctx.r[11].u64 = ((ctx.r[9].u32).rotate_left(2)) as u64;
	// 82E2A700: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E2A704: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E2A708: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E2A70C: 40990008  ble cr6, 0x82e2a714
	if !ctx.cr[6].gt {
	pc = 0x82E2A714; continue 'dispatch;
	}
	// 82E2A710: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82E2A714: 4BFC5E6D  bl 0x82df0580
	ctx.lr = 0x82E2A718;
	sub_82DF0580(ctx, base);
	// 82E2A718: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82E2A71C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2A720: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E2A724: 4BFEC4D5  bl 0x82e16bf8
	ctx.lr = 0x82E2A728;
	sub_82E16BF8(ctx, base);
	// 82E2A728: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2A72C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2A730: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2A734: 556B103B  rlwinm. r11, r11, 2, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2A738: 4182001C  beq 0x82e2a754
	if ctx.cr[0].eq {
	pc = 0x82E2A754; continue 'dispatch;
	}
	// 82E2A73C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2A740: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2A744: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E2A748: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E2A74C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82E2A750: 4082FFEC  bne 0x82e2a73c
	if !ctx.cr[0].eq {
	pc = 0x82E2A73C; continue 'dispatch;
	}
	// 82E2A754: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2A758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2A75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2A760: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2A764: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2A768: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2A770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2A770 size=168
    let mut pc: u32 = 0x82E2A770;
    'dispatch: loop {
        match pc {
            0x82E2A770 => {
    //   block [0x82E2A770..0x82E2A818)
	// 82E2A770: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2A774: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2A778: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2A77C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2A780: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2A784: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2A788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2A78C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2A790: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2A794: 4BFCED9D  bl 0x82df9530
	ctx.lr = 0x82E2A798;
	sub_82DF9530(ctx, base);
	// 82E2A798: 3D603FFF  lis r11, 0x3fff
	ctx.r[11].s64 = 1073676288;
	// 82E2A79C: 616AFFFF  ori r10, r11, 0xffff
	ctx.r[10].u64 = ctx.r[11].u64 | 65535;
	// 82E2A7A0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E2A7A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2A7A8: 897E0002  lbz r11, 2(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(2 as u32) ) } as u64;
	// 82E2A7AC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E2A7B0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2A7B4: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E2A7B8: 40990008  ble cr6, 0x82e2a7c0
	if !ctx.cr[6].gt {
	pc = 0x82E2A7C0; continue 'dispatch;
	}
	// 82E2A7BC: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82E2A7C0: 4BFC5DC1  bl 0x82df0580
	ctx.lr = 0x82E2A7C4;
	sub_82DF0580(ctx, base);
	// 82E2A7C4: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 82E2A7C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2A7CC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E2A7D0: 4BFEC429  bl 0x82e16bf8
	ctx.lr = 0x82E2A7D4;
	sub_82E16BF8(ctx, base);
	// 82E2A7D4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2A7D8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2A7DC: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2A7E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2A7E4: 419A001C  beq cr6, 0x82e2a800
	if ctx.cr[6].eq {
	pc = 0x82E2A800; continue 'dispatch;
	}
	// 82E2A7E8: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2A7EC: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2A7F0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82E2A7F4: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E2A7F8: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82E2A7FC: 4082FFEC  bne 0x82e2a7e8
	if !ctx.cr[0].eq {
	pc = 0x82E2A7E8; continue 'dispatch;
	}
	// 82E2A800: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2A804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2A808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2A80C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2A810: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2A814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2A818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2A818 size=96
    let mut pc: u32 = 0x82E2A818;
    'dispatch: loop {
        match pc {
            0x82E2A818 => {
    //   block [0x82E2A818..0x82E2A878)
	// 82E2A818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2A81C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2A820: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2A824: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2A828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2A82C: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82E2A830: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E2A834: 4BDAF83D  bl 0x82bda070
	ctx.lr = 0x82E2A838;
	sub_82BDA070(ctx, base);
	// 82E2A838: 7C7E1B79  or. r30, r3, r3
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E2A83C: 40820008  bne 0x82e2a844
	if !ctx.cr[0].eq {
	pc = 0x82E2A844; continue 'dispatch;
	}
	// 82E2A840: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E2A844: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E2A848: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2A84C: 419A0008  beq cr6, 0x82e2a854
	if ctx.cr[6].eq {
	pc = 0x82E2A854; continue 'dispatch;
	}
	// 82E2A850: 4BDB4789  bl 0x82bdefd8
	ctx.lr = 0x82E2A854;
	sub_82BDEFD8(ctx, base);
	// 82E2A854: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82E2A858: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2A85C: 4BFD3FFD  bl 0x82dfe858
	ctx.lr = 0x82E2A860;
	sub_82DFE858(ctx, base);
	// 82E2A860: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2A864: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2A868: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2A86C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2A870: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2A874: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2A878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2A878 size=152
    let mut pc: u32 = 0x82E2A878;
    'dispatch: loop {
        match pc {
            0x82E2A878 => {
    //   block [0x82E2A878..0x82E2A910)
	// 82E2A878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2A87C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2A880: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2A884: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2A888: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2A88C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2A890: 4BFD3F51  bl 0x82dfe7e0
	ctx.lr = 0x82E2A894;
	sub_82DFE7E0(ctx, base);
	// 82E2A894: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2A898: 4182005C  beq 0x82e2a8f4
	if ctx.cr[0].eq {
	pc = 0x82E2A8F4; continue 'dispatch;
	}
	// 82E2A89C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E2A8A0: 38C0002D  li r6, 0x2d
	ctx.r[6].s64 = 45;
	// 82E2A8A4: 38ABBFF0  addi r5, r11, -0x4010
	ctx.r[5].s64 = ctx.r[11].s64 + -16400;
	// 82E2A8A8: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 82E2A8AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2A8B0: 4BFCEF89  bl 0x82df9838
	ctx.lr = 0x82E2A8B4;
	sub_82DF9838(ctx, base);
	// 82E2A8B4: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E2A8B8: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82E2A8BC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 82E2A8C0: 419A0024  beq cr6, 0x82e2a8e4
	if ctx.cr[6].eq {
	pc = 0x82E2A8E4; continue 'dispatch;
	}
	// 82E2A8C4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E2A8C8: 80DF001C  lwz r6, 0x1c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E2A8CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2A8D0: 4BFFFF49  bl 0x82e2a818
	ctx.lr = 0x82E2A8D4;
	sub_82E2A818(ctx, base);
	// 82E2A8D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2A8D8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2A8DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2A8E0: 4837E329  bl 0x831a8c08
	ctx.lr = 0x82E2A8E4;
	sub_831A8C08(ctx, base);
	// 82E2A8E4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2A8E8: 4B4A4D19  bl 0x822cf600
	ctx.lr = 0x82E2A8EC;
	sub_822CF600(ctx, base);
	// 82E2A8EC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E2A8F0: 48000008  b 0x82e2a8f8
	pc = 0x82E2A8F8; continue 'dispatch;
	// 82E2A8F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E2A8F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2A8FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2A900: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2A904: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2A908: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2A90C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2A910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2A910 size=140
    let mut pc: u32 = 0x82E2A910;
    'dispatch: loop {
        match pc {
            0x82E2A910 => {
    //   block [0x82E2A910..0x82E2A99C)
	// 82E2A910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2A914: 4837D855  bl 0x831a8168
	ctx.lr = 0x82E2A918;
	sub_831A8130(ctx, base);
	// 82E2A918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2A91C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2A920: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2A924: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E2A928: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2A92C: 4BFD3F3D  bl 0x82dfe868
	ctx.lr = 0x82E2A930;
	sub_82DFE868(ctx, base);
	// 82E2A930: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2A934: 40820060  bne 0x82e2a994
	if !ctx.cr[0].eq {
	pc = 0x82E2A994; continue 'dispatch;
	}
	// 82E2A938: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E2A93C: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82E2A940: 4BFC5C41  bl 0x82df0580
	ctx.lr = 0x82E2A944;
	sub_82DF0580(ctx, base);
	// 82E2A944: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E2A948: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82E2A94C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E2A950: 4837E2B9  bl 0x831a8c08
	ctx.lr = 0x82E2A954;
	sub_831A8C08(ctx, base);
	// 82E2A954: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2A958: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2A95C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E2A960: 4837DBB1  bl 0x831a8510
	ctx.lr = 0x82E2A964;
	sub_831A8510(ctx, base);
	// 82E2A964: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E2A968: 806B16D4  lwz r3, 0x16d4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5844 as u32) ) } as u64;
	// 82E2A96C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2A970: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2A974: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E2A978: 4E800421  bctrl
	ctx.lr = 0x82E2A97C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E2A97C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2A980: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E2A984: 4B6C1825  bl 0x824ec1a8
	ctx.lr = 0x82E2A988;
	sub_824EC1A8(ctx, base);
	// 82E2A988: 939F001C  stw r28, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[28].u32 ) };
	// 82E2A98C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2A990: 4BFD3EC9  bl 0x82dfe858
	ctx.lr = 0x82E2A994;
	sub_82DFE858(ctx, base);
	// 82E2A994: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2A998: 4837D820  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2A9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2A9A0 size=72
    let mut pc: u32 = 0x82E2A9A0;
    'dispatch: loop {
        match pc {
            0x82E2A9A0 => {
    //   block [0x82E2A9A0..0x82E2A9E8)
	// 82E2A9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2A9A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2A9A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2A9AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2A9B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2A9B4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E2A9B8: 48052E29  bl 0x82e7d7e0
	ctx.lr = 0x82E2A9BC;
	sub_82E7D7E0(ctx, base);
	// 82E2A9BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2A9C0: 4BFD3DD9  bl 0x82dfe798
	ctx.lr = 0x82E2A9C4;
	sub_82DFE798(ctx, base);
	// 82E2A9C4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E2A9C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2A9CC: 396BC03C  addi r11, r11, -0x3fc4
	ctx.r[11].s64 = ctx.r[11].s64 + -16324;
	// 82E2A9D0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2A9D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2A9D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2A9DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2A9E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2A9E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2A9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2A9E8 size=72
    let mut pc: u32 = 0x82E2A9E8;
    'dispatch: loop {
        match pc {
            0x82E2A9E8 => {
    //   block [0x82E2A9E8..0x82E2AA30)
	// 82E2A9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2A9EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2A9F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2A9F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2A9F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2A9FC: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E2AA00: 48052E09  bl 0x82e7d808
	ctx.lr = 0x82E2AA04;
	sub_82E7D808(ctx, base);
	// 82E2AA04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2AA08: 4BFD3D91  bl 0x82dfe798
	ctx.lr = 0x82E2AA0C;
	sub_82DFE798(ctx, base);
	// 82E2AA0C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E2AA10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2AA14: 396BC03C  addi r11, r11, -0x3fc4
	ctx.r[11].s64 = ctx.r[11].s64 + -16324;
	// 82E2AA18: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2AA1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2AA20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2AA24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2AA28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2AA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2AA30 size=92
    let mut pc: u32 = 0x82E2AA30;
    'dispatch: loop {
        match pc {
            0x82E2AA30 => {
    //   block [0x82E2AA30..0x82E2AA8C)
	// 82E2AA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2AA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2AA38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2AA3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2AA40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2AA44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2AA48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2AA4C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E2AA50: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82E2AA54: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2AA58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2AA5C: 4BFC89CD  bl 0x82df3428
	ctx.lr = 0x82E2AA60;
	sub_82DF3428(ctx, base);
	// 82E2AA60: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2AA64: 4182000C  beq 0x82e2aa70
	if ctx.cr[0].eq {
	pc = 0x82E2AA70; continue 'dispatch;
	}
	// 82E2AA68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2AA6C: 4BFC796D  bl 0x82df23d8
	ctx.lr = 0x82E2AA70;
	sub_82DF23D8(ctx, base);
	// 82E2AA70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2AA74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2AA78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2AA7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2AA80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2AA84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2AA88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E2AA90 size=20
    let mut pc: u32 = 0x82E2AA90;
    'dispatch: loop {
        match pc {
            0x82E2AA90 => {
    //   block [0x82E2AA90..0x82E2AAA4)
	// 82E2AA90: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E2AA94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2AA98: 409A000C  bne cr6, 0x82e2aaa4
	if !ctx.cr[6].eq {
		sub_82E2AAA4(ctx, base);
		return;
	}
	// 82E2AA9C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82E2AAA0: 48000010  b 0x82e2aab0
	sub_82E2AAA4(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AAA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E2AAA4 size=44
    let mut pc: u32 = 0x82E2AAA4;
    'dispatch: loop {
        match pc {
            0x82E2AAA4 => {
    //   block [0x82E2AAA4..0x82E2AAD0)
	// 82E2AAA4: 8143002C  lwz r10, 0x2c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E2AAA8: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 82E2AAAC: 7D691E70  srawi r9, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82E2AAB0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2AAB4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E2AAB8: 419A0028  beq cr6, 0x82e2aae0
	if ctx.cr[6].eq {
		sub_82E2AAD0(ctx, base);
		return;
	}
	// 82E2AABC: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E2AAC0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2AAC4: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2AAC8: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 82E2AACC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AAD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E2AAD0 size=24
    let mut pc: u32 = 0x82E2AAD0;
    'dispatch: loop {
        match pc {
            0x82E2AAD0 => {
    //   block [0x82E2AAD0..0x82E2AAE8)
	// 82E2AAD0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2AAD4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82E2AAD8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E2AADC: 4198FFE4  blt cr6, 0x82e2aac0
	if ctx.cr[6].lt {
		sub_82E2AAA4(ctx, base);
		return;
	}
	// 82E2AAE0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E2AAE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AAE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E2AAE8 size=20
    let mut pc: u32 = 0x82E2AAE8;
    'dispatch: loop {
        match pc {
            0x82E2AAE8 => {
    //   block [0x82E2AAE8..0x82E2AAFC)
	// 82E2AAE8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E2AAEC: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82E2AAF0: 396BB37C  addi r11, r11, -0x4c84
	ctx.r[11].s64 = ctx.r[11].s64 + -19588;
	// 82E2AAF4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2AAF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E2AB00 size=16
    let mut pc: u32 = 0x82E2AB00;
    'dispatch: loop {
        match pc {
            0x82E2AB00 => {
    //   block [0x82E2AB00..0x82E2AB10)
	// 82E2AB00: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E2AB04: 396BB37C  addi r11, r11, -0x4c84
	ctx.r[11].s64 = ctx.r[11].s64 + -19588;
	// 82E2AB08: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2AB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2AB10 size=156
    let mut pc: u32 = 0x82E2AB10;
    'dispatch: loop {
        match pc {
            0x82E2AB10 => {
    //   block [0x82E2AB10..0x82E2ABAC)
	// 82E2AB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2AB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2AB18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2AB1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2AB20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2AB24: 54AB063F  clrlwi. r11, r5, 0x18
	ctx.r[11].u64 = ctx.r[5].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2AB28: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2AB2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2AB30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2AB34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2AB38: 388BA7AC  addi r4, r11, -0x5854
	ctx.r[4].s64 = ctx.r[11].s64 + -22612;
	// 82E2AB3C: 41820028  beq 0x82e2ab64
	if ctx.cr[0].eq {
	pc = 0x82E2AB64; continue 'dispatch;
	}
	// 82E2AB40: 4BFC8EC9  bl 0x82df3a08
	ctx.lr = 0x82E2AB44;
	sub_82DF3A08(ctx, base);
	// 82E2AB44: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2AB48: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2AB4C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2AB50: 48004821  bl 0x82e2f370
	ctx.lr = 0x82E2AB54;
	sub_82E2F370(ctx, base);
	// 82E2AB54: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2AB58: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2AB5C: 4BFD6685  bl 0x82e011e0
	ctx.lr = 0x82E2AB60;
	sub_82E011E0(ctx, base);
	// 82E2AB60: 48000024  b 0x82e2ab84
	pc = 0x82E2AB84; continue 'dispatch;
	// 82E2AB64: 4BFC8EA5  bl 0x82df3a08
	ctx.lr = 0x82E2AB68;
	sub_82DF3A08(ctx, base);
	// 82E2AB68: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2AB6C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2AB70: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2AB74: 480047FD  bl 0x82e2f370
	ctx.lr = 0x82E2AB78;
	sub_82E2F370(ctx, base);
	// 82E2AB78: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2AB7C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2AB80: 4BFD69F9  bl 0x82e01578
	ctx.lr = 0x82E2AB84;
	sub_82E01578(ctx, base);
	// 82E2AB84: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2AB88: 4BFC88A1  bl 0x82df3428
	ctx.lr = 0x82E2AB8C;
	sub_82DF3428(ctx, base);
	// 82E2AB8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2AB90: 4BFC8899  bl 0x82df3428
	ctx.lr = 0x82E2AB94;
	sub_82DF3428(ctx, base);
	// 82E2AB94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2AB98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2AB9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2ABA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2ABA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2ABA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2ABB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E2ABB0 size=64
    let mut pc: u32 = 0x82E2ABB0;
    'dispatch: loop {
        match pc {
            0x82E2ABB0 => {
    //   block [0x82E2ABB0..0x82E2ABF0)
	// 82E2ABB0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2ABB4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2ABB8: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2ABBC: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2ABC0: 892A0021  lbz r9, 0x21(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(33 as u32) ) } as u64;
	// 82E2ABC4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E2ABC8: 409A0008  bne cr6, 0x82e2abd0
	if !ctx.cr[6].eq {
	pc = 0x82E2ABD0; continue 'dispatch;
	}
	// 82E2ABCC: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82E2ABD0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2ABD4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E2ABD8: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2ABDC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2ABE0: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E2ABE4: 409A000C  bne cr6, 0x82e2abf0
	if !ctx.cr[6].eq {
		sub_82E2ABF0(ctx, base);
		return;
	}
	// 82E2ABE8: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2ABEC: 48000020  b 0x82e2ac0c
	sub_82E2AC08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2ABF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E2ABF0 size=24
    let mut pc: u32 = 0x82E2ABF0;
    'dispatch: loop {
        match pc {
            0x82E2ABF0 => {
    //   block [0x82E2ABF0..0x82E2AC08)
	// 82E2ABF0: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2ABF4: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2ABF8: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E2ABFC: 409A000C  bne cr6, 0x82e2ac08
	if !ctx.cr[6].eq {
		sub_82E2AC08(ctx, base);
		return;
	}
	// 82E2AC00: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2AC04: 48000008  b 0x82e2ac0c
	sub_82E2AC08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E2AC08 size=16
    let mut pc: u32 = 0x82E2AC08;
    'dispatch: loop {
        match pc {
            0x82E2AC08 => {
    //   block [0x82E2AC08..0x82E2AC18)
	// 82E2AC08: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2AC0C: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82E2AC10: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2AC14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AC18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2AC18 size=116
    let mut pc: u32 = 0x82E2AC18;
    'dispatch: loop {
        match pc {
            0x82E2AC18 => {
    //   block [0x82E2AC18..0x82E2AC8C)
	// 82E2AC18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2AC1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2AC20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2AC24: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2AC28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2AC2C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E2AC30: 419A0040  beq cr6, 0x82e2ac70
	if ctx.cr[6].eq {
	pc = 0x82E2AC70; continue 'dispatch;
	}
	// 82E2AC34: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E2AC38: 0CDF0000  twi 6, r31, 0
	// 82E2AC3C: 7D6BFB96  divwu r11, r11, r31
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[31].u32;
	// 82E2AC40: 2B0B0008  cmplwi cr6, r11, 8
	ctx.cr[6].compare_u32(ctx.r[11].u32, 8 as u32, &mut ctx.xer);
	// 82E2AC44: 4098002C  bge cr6, 0x82e2ac70
	if !ctx.cr[6].lt {
	pc = 0x82E2AC70; continue 'dispatch;
	}
	// 82E2AC48: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E2AC4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2AC50: 396B0828  addi r11, r11, 0x828
	ctx.r[11].s64 = ctx.r[11].s64 + 2088;
	// 82E2AC54: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2AC58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2AC5C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2AC60: 4B499651  bl 0x822c42b0
	ctx.lr = 0x82E2AC64;
	sub_822C42B0(ctx, base);
	// 82E2AC64: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82E2AC68: 396B0818  addi r11, r11, 0x818
	ctx.r[11].s64 = ctx.r[11].s64 + 2072;
	// 82E2AC6C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2AC70: 57E31838  slwi r3, r31, 3
	ctx.r[3].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E2AC74: 4B495CC5  bl 0x822c0938
	ctx.lr = 0x82E2AC78;
	sub_822C0938(ctx, base);
	// 82E2AC78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2AC7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2AC80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2AC84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2AC88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2AC90 size=72
    let mut pc: u32 = 0x82E2AC90;
    'dispatch: loop {
        match pc {
            0x82E2AC90 => {
    //   block [0x82E2AC90..0x82E2ACD8)
	// 82E2AC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2AC94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2AC98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2AC9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2ACA0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2ACA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2ACA8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E2ACAC: 388BA630  addi r4, r11, -0x59d0
	ctx.r[4].s64 = ctx.r[11].s64 + -22992;
	// 82E2ACB0: 4837D449  bl 0x831a80f8
	ctx.lr = 0x82E2ACB4;
	sub_831A80F8(ctx, base);
	// 82E2ACB4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2ACB8: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E2ACBC: 40820008  bne 0x82e2acc4
	if !ctx.cr[0].eq {
	pc = 0x82E2ACC4; continue 'dispatch;
	}
	// 82E2ACC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E2ACC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2ACC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2ACCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2ACD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2ACD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2ACD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2ACD8 size=72
    let mut pc: u32 = 0x82E2ACD8;
    'dispatch: loop {
        match pc {
            0x82E2ACD8 => {
    //   block [0x82E2ACD8..0x82E2AD20)
	// 82E2ACD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2ACDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2ACE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2ACE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2ACE8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2ACEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2ACF0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E2ACF4: 388BA688  addi r4, r11, -0x5978
	ctx.r[4].s64 = ctx.r[11].s64 + -22904;
	// 82E2ACF8: 4837D401  bl 0x831a80f8
	ctx.lr = 0x82E2ACFC;
	sub_831A80F8(ctx, base);
	// 82E2ACFC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2AD00: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E2AD04: 40820008  bne 0x82e2ad0c
	if !ctx.cr[0].eq {
	pc = 0x82E2AD0C; continue 'dispatch;
	}
	// 82E2AD08: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E2AD0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2AD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2AD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2AD18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2AD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2AD20 size=72
    let mut pc: u32 = 0x82E2AD20;
    'dispatch: loop {
        match pc {
            0x82E2AD20 => {
    //   block [0x82E2AD20..0x82E2AD68)
	// 82E2AD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2AD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2AD28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2AD2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2AD30: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2AD34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2AD38: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E2AD3C: 388BA6E0  addi r4, r11, -0x5920
	ctx.r[4].s64 = ctx.r[11].s64 + -22816;
	// 82E2AD40: 4837D3B9  bl 0x831a80f8
	ctx.lr = 0x82E2AD44;
	sub_831A80F8(ctx, base);
	// 82E2AD44: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2AD48: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E2AD4C: 40820008  bne 0x82e2ad54
	if !ctx.cr[0].eq {
	pc = 0x82E2AD54; continue 'dispatch;
	}
	// 82E2AD50: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E2AD54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2AD58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2AD5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2AD60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2AD64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AD68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2AD68 size=72
    let mut pc: u32 = 0x82E2AD68;
    'dispatch: loop {
        match pc {
            0x82E2AD68 => {
    //   block [0x82E2AD68..0x82E2ADB0)
	// 82E2AD68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2AD6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2AD70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2AD74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2AD78: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2AD7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2AD80: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E2AD84: 388BA740  addi r4, r11, -0x58c0
	ctx.r[4].s64 = ctx.r[11].s64 + -22720;
	// 82E2AD88: 4837D371  bl 0x831a80f8
	ctx.lr = 0x82E2AD8C;
	sub_831A80F8(ctx, base);
	// 82E2AD8C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2AD90: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E2AD94: 40820008  bne 0x82e2ad9c
	if !ctx.cr[0].eq {
	pc = 0x82E2AD9C; continue 'dispatch;
	}
	// 82E2AD98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E2AD9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2ADA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2ADA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2ADA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2ADAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2ADB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2ADB0 size=112
    let mut pc: u32 = 0x82E2ADB0;
    'dispatch: loop {
        match pc {
            0x82E2ADB0 => {
    //   block [0x82E2ADB0..0x82E2AE20)
	// 82E2ADB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2ADB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2ADB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2ADBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2ADC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2ADC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2ADC8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2ADCC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2ADD0: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2ADD4: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2ADD8: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 82E2ADDC: 4BFC760D  bl 0x82df23e8
	ctx.lr = 0x82E2ADE0;
	sub_82DF23E8(ctx, base);
	// 82E2ADE0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2ADE4: 41820010  beq 0x82e2adf4
	if ctx.cr[0].eq {
	pc = 0x82E2ADF4; continue 'dispatch;
	}
	// 82E2ADE8: 4BFF8379  bl 0x82e23160
	ctx.lr = 0x82E2ADEC;
	sub_82E23160(ctx, base);
	// 82E2ADEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2ADF0: 48000008  b 0x82e2adf8
	pc = 0x82E2ADF8; continue 'dispatch;
	// 82E2ADF4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2ADF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2ADFC: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2AE00: 4BFD3A79  bl 0x82dfe878
	ctx.lr = 0x82E2AE04;
	sub_82DFE878(ctx, base);
	// 82E2AE04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2AE08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2AE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2AE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2AE14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2AE18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2AE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2AE20 size=112
    let mut pc: u32 = 0x82E2AE20;
    'dispatch: loop {
        match pc {
            0x82E2AE20 => {
    //   block [0x82E2AE20..0x82E2AE90)
	// 82E2AE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2AE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2AE28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2AE2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2AE30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2AE34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2AE38: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2AE3C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2AE40: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2AE44: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2AE48: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 82E2AE4C: 4BFC759D  bl 0x82df23e8
	ctx.lr = 0x82E2AE50;
	sub_82DF23E8(ctx, base);
	// 82E2AE50: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2AE54: 41820010  beq 0x82e2ae64
	if ctx.cr[0].eq {
	pc = 0x82E2AE64; continue 'dispatch;
	}
	// 82E2AE58: 4BFEEF99  bl 0x82e19df0
	ctx.lr = 0x82E2AE5C;
	sub_82E19DF0(ctx, base);
	// 82E2AE5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2AE60: 48000008  b 0x82e2ae68
	pc = 0x82E2AE68; continue 'dispatch;
	// 82E2AE64: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2AE68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2AE6C: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2AE70: 4BFD3A09  bl 0x82dfe878
	ctx.lr = 0x82E2AE74;
	sub_82DFE878(ctx, base);
	// 82E2AE74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2AE78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2AE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2AE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2AE84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2AE88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2AE8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2AE90 size=112
    let mut pc: u32 = 0x82E2AE90;
    'dispatch: loop {
        match pc {
            0x82E2AE90 => {
    //   block [0x82E2AE90..0x82E2AF00)
	// 82E2AE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2AE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2AE98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2AE9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2AEA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2AEA4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2AEA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2AEAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2AEB0: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2AEB4: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2AEB8: 38600150  li r3, 0x150
	ctx.r[3].s64 = 336;
	// 82E2AEBC: 4BFC752D  bl 0x82df23e8
	ctx.lr = 0x82E2AEC0;
	sub_82DF23E8(ctx, base);
	// 82E2AEC0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2AEC4: 41820010  beq 0x82e2aed4
	if ctx.cr[0].eq {
	pc = 0x82E2AED4; continue 'dispatch;
	}
	// 82E2AEC8: 4BFF1051  bl 0x82e1bf18
	ctx.lr = 0x82E2AECC;
	sub_82E1BF18(ctx, base);
	// 82E2AECC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2AED0: 48000008  b 0x82e2aed8
	pc = 0x82E2AED8; continue 'dispatch;
	// 82E2AED4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2AED8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2AEDC: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2AEE0: 4BFD3999  bl 0x82dfe878
	ctx.lr = 0x82E2AEE4;
	sub_82DFE878(ctx, base);
	// 82E2AEE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2AEE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2AEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2AEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2AEF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2AEF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2AEFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2AF00 size=112
    let mut pc: u32 = 0x82E2AF00;
    'dispatch: loop {
        match pc {
            0x82E2AF00 => {
    //   block [0x82E2AF00..0x82E2AF70)
	// 82E2AF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2AF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2AF08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2AF0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2AF10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2AF14: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2AF18: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2AF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2AF20: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2AF24: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2AF28: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 82E2AF2C: 4BFC74BD  bl 0x82df23e8
	ctx.lr = 0x82E2AF30;
	sub_82DF23E8(ctx, base);
	// 82E2AF30: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2AF34: 41820010  beq 0x82e2af44
	if ctx.cr[0].eq {
	pc = 0x82E2AF44; continue 'dispatch;
	}
	// 82E2AF38: 4BFF10D1  bl 0x82e1c008
	ctx.lr = 0x82E2AF3C;
	sub_82E1C008(ctx, base);
	// 82E2AF3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2AF40: 48000008  b 0x82e2af48
	pc = 0x82E2AF48; continue 'dispatch;
	// 82E2AF44: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2AF48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2AF4C: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2AF50: 4BFD3929  bl 0x82dfe878
	ctx.lr = 0x82E2AF54;
	sub_82DFE878(ctx, base);
	// 82E2AF54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2AF58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2AF5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2AF60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2AF64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2AF68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2AF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2AF70 size=112
    let mut pc: u32 = 0x82E2AF70;
    'dispatch: loop {
        match pc {
            0x82E2AF70 => {
    //   block [0x82E2AF70..0x82E2AFE0)
	// 82E2AF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2AF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2AF78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2AF7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2AF80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2AF84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2AF88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2AF8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2AF90: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2AF94: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2AF98: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82E2AF9C: 4BFC744D  bl 0x82df23e8
	ctx.lr = 0x82E2AFA0;
	sub_82DF23E8(ctx, base);
	// 82E2AFA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2AFA4: 41820010  beq 0x82e2afb4
	if ctx.cr[0].eq {
	pc = 0x82E2AFB4; continue 'dispatch;
	}
	// 82E2AFA8: 4BFFE271  bl 0x82e29218
	ctx.lr = 0x82E2AFAC;
	sub_82E29218(ctx, base);
	// 82E2AFAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2AFB0: 48000008  b 0x82e2afb8
	pc = 0x82E2AFB8; continue 'dispatch;
	// 82E2AFB4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2AFB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2AFBC: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2AFC0: 4BFD38B9  bl 0x82dfe878
	ctx.lr = 0x82E2AFC4;
	sub_82DFE878(ctx, base);
	// 82E2AFC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2AFC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2AFCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2AFD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2AFD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2AFD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2AFDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2AFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2AFE0 size=112
    let mut pc: u32 = 0x82E2AFE0;
    'dispatch: loop {
        match pc {
            0x82E2AFE0 => {
    //   block [0x82E2AFE0..0x82E2B050)
	// 82E2AFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2AFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2AFE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2AFEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2AFF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2AFF4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2AFF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2AFFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2B000: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2B004: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2B008: 3860008C  li r3, 0x8c
	ctx.r[3].s64 = 140;
	// 82E2B00C: 4BFC73DD  bl 0x82df23e8
	ctx.lr = 0x82E2B010;
	sub_82DF23E8(ctx, base);
	// 82E2B010: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2B014: 41820010  beq 0x82e2b024
	if ctx.cr[0].eq {
	pc = 0x82E2B024; continue 'dispatch;
	}
	// 82E2B018: 483D0851  bl 0x831fb868
	ctx.lr = 0x82E2B01C;
	sub_831FB868(ctx, base);
	// 82E2B01C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2B020: 48000008  b 0x82e2b028
	pc = 0x82E2B028; continue 'dispatch;
	// 82E2B024: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2B028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B02C: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2B030: 4BFD3849  bl 0x82dfe878
	ctx.lr = 0x82E2B034;
	sub_82DFE878(ctx, base);
	// 82E2B034: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B038: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2B03C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2B040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2B044: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2B048: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2B04C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E2B050 size=64
    let mut pc: u32 = 0x82E2B050;
    'dispatch: loop {
        match pc {
            0x82E2B050 => {
    //   block [0x82E2B050..0x82E2B090)
	// 82E2B050: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2B054: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2B058: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E2B05C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2B060: 892A0021  lbz r9, 0x21(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(33 as u32) ) } as u64;
	// 82E2B064: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82E2B068: 409A0008  bne cr6, 0x82e2b070
	if !ctx.cr[6].eq {
	pc = 0x82E2B070; continue 'dispatch;
	}
	// 82E2B06C: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82E2B070: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2B074: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E2B078: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2B07C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2B080: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E2B084: 409A000C  bne cr6, 0x82e2b090
	if !ctx.cr[6].eq {
		sub_82E2B090(ctx, base);
		return;
	}
	// 82E2B088: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2B08C: 48000020  b 0x82e2b0ac
	sub_82E2B0A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E2B090 size=24
    let mut pc: u32 = 0x82E2B090;
    'dispatch: loop {
        match pc {
            0x82E2B090 => {
    //   block [0x82E2B090..0x82E2B0A8)
	// 82E2B090: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2B094: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2B098: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E2B09C: 409A000C  bne cr6, 0x82e2b0a8
	if !ctx.cr[6].eq {
		sub_82E2B0A8(ctx, base);
		return;
	}
	// 82E2B0A0: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B0A4: 48000008  b 0x82e2b0ac
	sub_82E2B0A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B0A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E2B0A8 size=16
    let mut pc: u32 = 0x82E2B0A8;
    'dispatch: loop {
        match pc {
            0x82E2B0A8 => {
    //   block [0x82E2B0A8..0x82E2B0B8)
	// 82E2B0A8: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2B0AC: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82E2B0B0: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2B0B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B0B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B0B8 size=124
    let mut pc: u32 = 0x82E2B0B8;
    'dispatch: loop {
        match pc {
            0x82E2B0B8 => {
    //   block [0x82E2B0B8..0x82E2B134)
	// 82E2B0B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B0BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2B0C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2B0C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2B0C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B0CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2B0D0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2B0D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2B0D8: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2B0DC: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2B0E0: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E2B0E4: 4BFC7305  bl 0x82df23e8
	ctx.lr = 0x82E2B0E8;
	sub_82DF23E8(ctx, base);
	// 82E2B0E8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82E2B0EC: 4182001C  beq 0x82e2b108
	if ctx.cr[0].eq {
	pc = 0x82E2B108; continue 'dispatch;
	}
	// 82E2B0F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B0F4: 4BFD36A5  bl 0x82dfe798
	ctx.lr = 0x82E2B0F8;
	sub_82DFE798(ctx, base);
	// 82E2B0F8: 3D608203  lis r11, -0x7dfd
	ctx.r[11].s64 = -2113732608;
	// 82E2B0FC: 396B2954  addi r11, r11, 0x2954
	ctx.r[11].s64 = ctx.r[11].s64 + 10580;
	// 82E2B100: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B104: 48000008  b 0x82e2b10c
	pc = 0x82E2B10C; continue 'dispatch;
	// 82E2B108: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2B10C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B110: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2B114: 4BFD3765  bl 0x82dfe878
	ctx.lr = 0x82E2B118;
	sub_82DFE878(ctx, base);
	// 82E2B118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B11C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2B120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2B124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2B128: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2B12C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2B130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B138 size=92
    let mut pc: u32 = 0x82E2B138;
    'dispatch: loop {
        match pc {
            0x82E2B138 => {
    //   block [0x82E2B138..0x82E2B194)
	// 82E2B138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2B140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B144: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E2B148: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E2B14C: 4BFF1E95  bl 0x82e1cfe0
	ctx.lr = 0x82E2B150;
	sub_82E1CFE0(ctx, base);
	// 82E2B150: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2B154: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2B158: 41820008  beq 0x82e2b160
	if ctx.cr[0].eq {
	pc = 0x82E2B160; continue 'dispatch;
	}
	// 82E2B15C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2B160: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2B164: 41820008  beq 0x82e2b16c
	if ctx.cr[0].eq {
	pc = 0x82E2B16C; continue 'dispatch;
	}
	// 82E2B168: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2B16C: 35630008  addic. r11, r3, 8
	ctx.xer.ca = (ctx.r[3].u32 > (!(8 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 8;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2B170: 41820008  beq 0x82e2b178
	if ctx.cr[0].eq {
	pc = 0x82E2B178; continue 'dispatch;
	}
	// 82E2B174: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2B178: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E2B17C: 99430015  stb r10, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[10].u8 ) };
	// 82E2B180: 99630014  stb r11, 0x14(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82E2B184: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2B188: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2B18C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2B190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B198 size=92
    let mut pc: u32 = 0x82E2B198;
    'dispatch: loop {
        match pc {
            0x82E2B198 => {
    //   block [0x82E2B198..0x82E2B1F4)
	// 82E2B198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2B1A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2B1A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2B1A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B1AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2B1B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E2B1B4: 4800001C  b 0x82e2b1d0
	pc = 0x82E2B1D0; continue 'dispatch;
	// 82E2B1B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E2B1BC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2B1C0: 4BFFFFD9  bl 0x82e2b198
	ctx.lr = 0x82E2B1C4;
	sub_82E2B198(ctx, base);
	// 82E2B1C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B1C8: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2B1CC: 4B49509D  bl 0x822c0268
	ctx.lr = 0x82E2B1D0;
	sub_822C0268(ctx, base);
	// 82E2B1D0: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E2B1D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2B1D8: 419AFFE0  beq cr6, 0x82e2b1b8
	if ctx.cr[6].eq {
	pc = 0x82E2B1B8; continue 'dispatch;
	}
	// 82E2B1DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2B1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2B1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2B1E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2B1EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2B1F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B1F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B1F8 size=172
    let mut pc: u32 = 0x82E2B1F8;
    'dispatch: loop {
        match pc {
            0x82E2B1F8 => {
    //   block [0x82E2B1F8..0x82E2B2A4)
	// 82E2B1F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B1FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2B200: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2B204: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2B208: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B20C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2B210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2B214: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82E2B218: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E2B21C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B220: 4B495719  bl 0x822c0938
	ctx.lr = 0x82E2B224;
	sub_822C0938(ctx, base);
	// 82E2B224: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2B228: 41820028  beq 0x82e2b250
	if ctx.cr[0].eq {
	pc = 0x82E2B250; continue 'dispatch;
	}
	// 82E2B22C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E2B230: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E2B234: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E2B238: 392BC044  addi r9, r11, -0x3fbc
	ctx.r[9].s64 = ctx.r[11].s64 + -16316;
	// 82E2B23C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E2B240: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E2B244: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E2B248: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E2B24C: 48000008  b 0x82e2b254
	pc = 0x82E2B254; continue 'dispatch;
	// 82E2B250: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2B254: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B258: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2B25C: 409A002C  bne cr6, 0x82e2b288
	if !ctx.cr[6].eq {
	pc = 0x82E2B288; continue 'dispatch;
	}
	// 82E2B260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B264: 4BFC7175  bl 0x82df23d8
	ctx.lr = 0x82E2B268;
	sub_82DF23D8(ctx, base);
	// 82E2B268: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2B26C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E2B270: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2B274: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E2B278: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E2B27C: 816BA62C  lwz r11, -0x59d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22996 as u32) ) } as u64;
	// 82E2B280: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E2B284: 4B494D7D  bl 0x822c0000
	ctx.lr = 0x82E2B288;
	sub_822C0000(ctx, base);
	// 82E2B288: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E2B28C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2B290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2B294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2B298: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2B29C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2B2A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B2A8 size=172
    let mut pc: u32 = 0x82E2B2A8;
    'dispatch: loop {
        match pc {
            0x82E2B2A8 => {
    //   block [0x82E2B2A8..0x82E2B354)
	// 82E2B2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2B2B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2B2B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2B2B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B2BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2B2C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2B2C4: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82E2B2C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E2B2CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B2D0: 4B495669  bl 0x822c0938
	ctx.lr = 0x82E2B2D4;
	sub_822C0938(ctx, base);
	// 82E2B2D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2B2D8: 41820028  beq 0x82e2b300
	if ctx.cr[0].eq {
	pc = 0x82E2B300; continue 'dispatch;
	}
	// 82E2B2DC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E2B2E0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E2B2E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E2B2E8: 392BC054  addi r9, r11, -0x3fac
	ctx.r[9].s64 = ctx.r[11].s64 + -16300;
	// 82E2B2EC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E2B2F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E2B2F4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E2B2F8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E2B2FC: 48000008  b 0x82e2b304
	pc = 0x82E2B304; continue 'dispatch;
	// 82E2B300: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2B304: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B308: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2B30C: 409A002C  bne cr6, 0x82e2b338
	if !ctx.cr[6].eq {
	pc = 0x82E2B338; continue 'dispatch;
	}
	// 82E2B310: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B314: 4BFC70C5  bl 0x82df23d8
	ctx.lr = 0x82E2B318;
	sub_82DF23D8(ctx, base);
	// 82E2B318: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2B31C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E2B320: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2B324: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E2B328: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E2B32C: 816BA62C  lwz r11, -0x59d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22996 as u32) ) } as u64;
	// 82E2B330: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E2B334: 4B494CCD  bl 0x822c0000
	ctx.lr = 0x82E2B338;
	sub_822C0000(ctx, base);
	// 82E2B338: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E2B33C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2B340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2B344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2B348: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2B34C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2B350: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B358 size=172
    let mut pc: u32 = 0x82E2B358;
    'dispatch: loop {
        match pc {
            0x82E2B358 => {
    //   block [0x82E2B358..0x82E2B404)
	// 82E2B358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B35C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2B360: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2B364: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2B368: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B36C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2B370: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2B374: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82E2B378: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E2B37C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B380: 4B4955B9  bl 0x822c0938
	ctx.lr = 0x82E2B384;
	sub_822C0938(ctx, base);
	// 82E2B384: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2B388: 41820028  beq 0x82e2b3b0
	if ctx.cr[0].eq {
	pc = 0x82E2B3B0; continue 'dispatch;
	}
	// 82E2B38C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E2B390: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E2B394: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E2B398: 392BC074  addi r9, r11, -0x3f8c
	ctx.r[9].s64 = ctx.r[11].s64 + -16268;
	// 82E2B39C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E2B3A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E2B3A4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E2B3A8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E2B3AC: 48000008  b 0x82e2b3b4
	pc = 0x82E2B3B4; continue 'dispatch;
	// 82E2B3B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2B3B4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B3B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2B3BC: 409A002C  bne cr6, 0x82e2b3e8
	if !ctx.cr[6].eq {
	pc = 0x82E2B3E8; continue 'dispatch;
	}
	// 82E2B3C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B3C4: 4BFC7015  bl 0x82df23d8
	ctx.lr = 0x82E2B3C8;
	sub_82DF23D8(ctx, base);
	// 82E2B3C8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2B3CC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E2B3D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2B3D4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E2B3D8: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E2B3DC: 816BA62C  lwz r11, -0x59d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22996 as u32) ) } as u64;
	// 82E2B3E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E2B3E4: 4B494C1D  bl 0x822c0000
	ctx.lr = 0x82E2B3E8;
	sub_822C0000(ctx, base);
	// 82E2B3E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E2B3EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2B3F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2B3F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2B3F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2B3FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2B400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B408 size=164
    let mut pc: u32 = 0x82E2B408;
    'dispatch: loop {
        match pc {
            0x82E2B408 => {
    //   block [0x82E2B408..0x82E2B4AC)
	// 82E2B408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B40C: 4837CD61  bl 0x831a816c
	ctx.lr = 0x82E2B410;
	sub_831A8130(ctx, base);
	// 82E2B410: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B414: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2B418: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2B41C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2B420: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2B424: 396BB0B8  addi r11, r11, -0x4f48
	ctx.r[11].s64 = ctx.r[11].s64 + -20296;
	// 82E2B428: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2B42C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2B430: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2B434: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2B438: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2B43C: 4BFD34BD  bl 0x82dfe8f8
	ctx.lr = 0x82E2B440;
	sub_82DFE8F8(ctx, base);
	// 82E2B440: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2B444: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2B448: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2B44C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2B450: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2B454: 4BFD6445  bl 0x82e01898
	ctx.lr = 0x82E2B458;
	sub_82E01898(ctx, base);
	// 82E2B458: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2B45C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B460: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2B464: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2B468: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2B46C: 419A0024  beq cr6, 0x82e2b490
	if ctx.cr[6].eq {
	pc = 0x82E2B490; continue 'dispatch;
	}
	// 82E2B470: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2B474: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2B478: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2B47C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2B480: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2B484: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2B488: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2B48C: 4082FFE8  bne 0x82e2b474
	if !ctx.cr[0].eq {
	pc = 0x82E2B474; continue 'dispatch;
	}
	// 82E2B490: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2B494: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2B498: 419A0008  beq cr6, 0x82e2b4a0
	if ctx.cr[6].eq {
	pc = 0x82E2B4A0; continue 'dispatch;
	}
	// 82E2B49C: 4B4953F5  bl 0x822c0890
	ctx.lr = 0x82E2B4A0;
	sub_822C0890(ctx, base);
	// 82E2B4A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B4A4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2B4A8: 4837CD14  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B4B0 size=164
    let mut pc: u32 = 0x82E2B4B0;
    'dispatch: loop {
        match pc {
            0x82E2B4B0 => {
    //   block [0x82E2B4B0..0x82E2B554)
	// 82E2B4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B4B4: 4837CCB9  bl 0x831a816c
	ctx.lr = 0x82E2B4B8;
	sub_831A8130(ctx, base);
	// 82E2B4B8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B4BC: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2B4C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2B4C4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2B4C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2B4CC: 396BADB0  addi r11, r11, -0x5250
	ctx.r[11].s64 = ctx.r[11].s64 + -21072;
	// 82E2B4D0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2B4D4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2B4D8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2B4DC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2B4E0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2B4E4: 4BFD3415  bl 0x82dfe8f8
	ctx.lr = 0x82E2B4E8;
	sub_82DFE8F8(ctx, base);
	// 82E2B4E8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2B4EC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2B4F0: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2B4F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2B4F8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2B4FC: 4BFD639D  bl 0x82e01898
	ctx.lr = 0x82E2B500;
	sub_82E01898(ctx, base);
	// 82E2B500: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2B504: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B508: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2B50C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2B510: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2B514: 419A0024  beq cr6, 0x82e2b538
	if ctx.cr[6].eq {
	pc = 0x82E2B538; continue 'dispatch;
	}
	// 82E2B518: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2B51C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2B520: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2B524: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2B528: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2B52C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2B530: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2B534: 4082FFE8  bne 0x82e2b51c
	if !ctx.cr[0].eq {
	pc = 0x82E2B51C; continue 'dispatch;
	}
	// 82E2B538: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2B53C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2B540: 419A0008  beq cr6, 0x82e2b548
	if ctx.cr[6].eq {
	pc = 0x82E2B548; continue 'dispatch;
	}
	// 82E2B544: 4B49534D  bl 0x822c0890
	ctx.lr = 0x82E2B548;
	sub_822C0890(ctx, base);
	// 82E2B548: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B54C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2B550: 4837CC6C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B558 size=164
    let mut pc: u32 = 0x82E2B558;
    'dispatch: loop {
        match pc {
            0x82E2B558 => {
    //   block [0x82E2B558..0x82E2B5FC)
	// 82E2B558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B55C: 4837CC11  bl 0x831a816c
	ctx.lr = 0x82E2B560;
	sub_831A8130(ctx, base);
	// 82E2B560: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B564: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2B568: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2B56C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2B570: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2B574: 396BAE20  addi r11, r11, -0x51e0
	ctx.r[11].s64 = ctx.r[11].s64 + -20960;
	// 82E2B578: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2B57C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2B580: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2B584: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2B588: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2B58C: 4BFD336D  bl 0x82dfe8f8
	ctx.lr = 0x82E2B590;
	sub_82DFE8F8(ctx, base);
	// 82E2B590: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2B594: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2B598: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2B59C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2B5A0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2B5A4: 4BFD62F5  bl 0x82e01898
	ctx.lr = 0x82E2B5A8;
	sub_82E01898(ctx, base);
	// 82E2B5A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2B5AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B5B0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2B5B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2B5B8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2B5BC: 419A0024  beq cr6, 0x82e2b5e0
	if ctx.cr[6].eq {
	pc = 0x82E2B5E0; continue 'dispatch;
	}
	// 82E2B5C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2B5C4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2B5C8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2B5CC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2B5D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2B5D4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2B5D8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2B5DC: 4082FFE8  bne 0x82e2b5c4
	if !ctx.cr[0].eq {
	pc = 0x82E2B5C4; continue 'dispatch;
	}
	// 82E2B5E0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2B5E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2B5E8: 419A0008  beq cr6, 0x82e2b5f0
	if ctx.cr[6].eq {
	pc = 0x82E2B5F0; continue 'dispatch;
	}
	// 82E2B5EC: 4B4952A5  bl 0x822c0890
	ctx.lr = 0x82E2B5F0;
	sub_822C0890(ctx, base);
	// 82E2B5F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B5F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2B5F8: 4837CBC4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B600 size=164
    let mut pc: u32 = 0x82E2B600;
    'dispatch: loop {
        match pc {
            0x82E2B600 => {
    //   block [0x82E2B600..0x82E2B6A4)
	// 82E2B600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B604: 4837CB69  bl 0x831a816c
	ctx.lr = 0x82E2B608;
	sub_831A8130(ctx, base);
	// 82E2B608: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B60C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2B610: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2B614: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2B618: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2B61C: 396BAE90  addi r11, r11, -0x5170
	ctx.r[11].s64 = ctx.r[11].s64 + -20848;
	// 82E2B620: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2B624: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2B628: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2B62C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2B630: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2B634: 4BFD32C5  bl 0x82dfe8f8
	ctx.lr = 0x82E2B638;
	sub_82DFE8F8(ctx, base);
	// 82E2B638: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2B63C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2B640: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2B644: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2B648: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2B64C: 4BFD624D  bl 0x82e01898
	ctx.lr = 0x82E2B650;
	sub_82E01898(ctx, base);
	// 82E2B650: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2B654: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B658: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2B65C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2B660: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2B664: 419A0024  beq cr6, 0x82e2b688
	if ctx.cr[6].eq {
	pc = 0x82E2B688; continue 'dispatch;
	}
	// 82E2B668: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2B66C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2B670: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2B674: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2B678: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2B67C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2B680: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2B684: 4082FFE8  bne 0x82e2b66c
	if !ctx.cr[0].eq {
	pc = 0x82E2B66C; continue 'dispatch;
	}
	// 82E2B688: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2B68C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2B690: 419A0008  beq cr6, 0x82e2b698
	if ctx.cr[6].eq {
	pc = 0x82E2B698; continue 'dispatch;
	}
	// 82E2B694: 4B4951FD  bl 0x822c0890
	ctx.lr = 0x82E2B698;
	sub_822C0890(ctx, base);
	// 82E2B698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B69C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2B6A0: 4837CB1C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B6A8 size=164
    let mut pc: u32 = 0x82E2B6A8;
    'dispatch: loop {
        match pc {
            0x82E2B6A8 => {
    //   block [0x82E2B6A8..0x82E2B74C)
	// 82E2B6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B6AC: 4837CAC1  bl 0x831a816c
	ctx.lr = 0x82E2B6B0;
	sub_831A8130(ctx, base);
	// 82E2B6B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B6B4: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2B6B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2B6BC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2B6C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2B6C4: 396BAF00  addi r11, r11, -0x5100
	ctx.r[11].s64 = ctx.r[11].s64 + -20736;
	// 82E2B6C8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2B6CC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2B6D0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2B6D4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2B6D8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2B6DC: 4BFD321D  bl 0x82dfe8f8
	ctx.lr = 0x82E2B6E0;
	sub_82DFE8F8(ctx, base);
	// 82E2B6E0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2B6E4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2B6E8: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2B6EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2B6F0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2B6F4: 4BFD61A5  bl 0x82e01898
	ctx.lr = 0x82E2B6F8;
	sub_82E01898(ctx, base);
	// 82E2B6F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2B6FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B700: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2B704: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2B708: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2B70C: 419A0024  beq cr6, 0x82e2b730
	if ctx.cr[6].eq {
	pc = 0x82E2B730; continue 'dispatch;
	}
	// 82E2B710: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2B714: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2B718: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2B71C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2B720: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2B724: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2B728: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2B72C: 4082FFE8  bne 0x82e2b714
	if !ctx.cr[0].eq {
	pc = 0x82E2B714; continue 'dispatch;
	}
	// 82E2B730: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2B734: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2B738: 419A0008  beq cr6, 0x82e2b740
	if ctx.cr[6].eq {
	pc = 0x82E2B740; continue 'dispatch;
	}
	// 82E2B73C: 4B495155  bl 0x822c0890
	ctx.lr = 0x82E2B740;
	sub_822C0890(ctx, base);
	// 82E2B740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B744: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2B748: 4837CA74  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B750 size=164
    let mut pc: u32 = 0x82E2B750;
    'dispatch: loop {
        match pc {
            0x82E2B750 => {
    //   block [0x82E2B750..0x82E2B7F4)
	// 82E2B750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B754: 4837CA19  bl 0x831a816c
	ctx.lr = 0x82E2B758;
	sub_831A8130(ctx, base);
	// 82E2B758: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B75C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2B760: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2B764: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2B768: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2B76C: 396BAF70  addi r11, r11, -0x5090
	ctx.r[11].s64 = ctx.r[11].s64 + -20624;
	// 82E2B770: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2B774: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2B778: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2B77C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2B780: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2B784: 4BFD3175  bl 0x82dfe8f8
	ctx.lr = 0x82E2B788;
	sub_82DFE8F8(ctx, base);
	// 82E2B788: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2B78C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2B790: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2B794: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2B798: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2B79C: 4BFD60FD  bl 0x82e01898
	ctx.lr = 0x82E2B7A0;
	sub_82E01898(ctx, base);
	// 82E2B7A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2B7A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B7A8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2B7AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2B7B0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2B7B4: 419A0024  beq cr6, 0x82e2b7d8
	if ctx.cr[6].eq {
	pc = 0x82E2B7D8; continue 'dispatch;
	}
	// 82E2B7B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2B7BC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2B7C0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2B7C4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2B7C8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2B7CC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2B7D0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2B7D4: 4082FFE8  bne 0x82e2b7bc
	if !ctx.cr[0].eq {
	pc = 0x82E2B7BC; continue 'dispatch;
	}
	// 82E2B7D8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2B7DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2B7E0: 419A0008  beq cr6, 0x82e2b7e8
	if ctx.cr[6].eq {
	pc = 0x82E2B7E8; continue 'dispatch;
	}
	// 82E2B7E4: 4B4950AD  bl 0x822c0890
	ctx.lr = 0x82E2B7E8;
	sub_822C0890(ctx, base);
	// 82E2B7E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B7EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2B7F0: 4837C9CC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B7F8 size=164
    let mut pc: u32 = 0x82E2B7F8;
    'dispatch: loop {
        match pc {
            0x82E2B7F8 => {
    //   block [0x82E2B7F8..0x82E2B89C)
	// 82E2B7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B7FC: 4837C971  bl 0x831a816c
	ctx.lr = 0x82E2B800;
	sub_831A8130(ctx, base);
	// 82E2B800: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B804: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2B808: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2B80C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2B810: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2B814: 396BAFE0  addi r11, r11, -0x5020
	ctx.r[11].s64 = ctx.r[11].s64 + -20512;
	// 82E2B818: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2B81C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2B820: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2B824: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2B828: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2B82C: 4BFD30CD  bl 0x82dfe8f8
	ctx.lr = 0x82E2B830;
	sub_82DFE8F8(ctx, base);
	// 82E2B830: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2B834: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2B838: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2B83C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2B840: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2B844: 4BFD6055  bl 0x82e01898
	ctx.lr = 0x82E2B848;
	sub_82E01898(ctx, base);
	// 82E2B848: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2B84C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B850: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2B854: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2B858: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2B85C: 419A0024  beq cr6, 0x82e2b880
	if ctx.cr[6].eq {
	pc = 0x82E2B880; continue 'dispatch;
	}
	// 82E2B860: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2B864: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2B868: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2B86C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2B870: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2B874: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2B878: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2B87C: 4082FFE8  bne 0x82e2b864
	if !ctx.cr[0].eq {
	pc = 0x82E2B864; continue 'dispatch;
	}
	// 82E2B880: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2B884: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2B888: 419A0008  beq cr6, 0x82e2b890
	if ctx.cr[6].eq {
	pc = 0x82E2B890; continue 'dispatch;
	}
	// 82E2B88C: 4B495005  bl 0x822c0890
	ctx.lr = 0x82E2B890;
	sub_822C0890(ctx, base);
	// 82E2B890: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B894: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2B898: 4837C924  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B8A0 size=100
    let mut pc: u32 = 0x82E2B8A0;
    'dispatch: loop {
        match pc {
            0x82E2B8A0 => {
    //   block [0x82E2B8A0..0x82E2B904)
	// 82E2B8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B8A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2B8A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2B8AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B8B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2B8B4: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E2B8B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2B8BC: 419A0008  beq cr6, 0x82e2b8c4
	if ctx.cr[6].eq {
	pc = 0x82E2B8C4; continue 'dispatch;
	}
	// 82E2B8C0: 4B494FD1  bl 0x822c0890
	ctx.lr = 0x82E2B8C4;
	sub_822C0890(ctx, base);
	// 82E2B8C4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E2B8C8: 4837D341  bl 0x831a8c08
	ctx.lr = 0x82E2B8CC;
	sub_831A8C08(ctx, base);
	// 82E2B8CC: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E2B8D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2B8D4: 419A0008  beq cr6, 0x82e2b8dc
	if ctx.cr[6].eq {
	pc = 0x82E2B8DC; continue 'dispatch;
	}
	// 82E2B8D8: 4BDB3701  bl 0x82bdefd8
	ctx.lr = 0x82E2B8DC;
	sub_82BDEFD8(ctx, base);
	// 82E2B8DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2B8E0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E2B8E4: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82E2B8E8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B8EC: 4BFC7B3D  bl 0x82df3428
	ctx.lr = 0x82E2B8F0;
	sub_82DF3428(ctx, base);
	// 82E2B8F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2B8F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2B8F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2B8FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2B900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B908 size=80
    let mut pc: u32 = 0x82E2B908;
    'dispatch: loop {
        match pc {
            0x82E2B908 => {
    //   block [0x82E2B908..0x82E2B958)
	// 82E2B908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B90C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2B910: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2B914: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B918: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2B91C: 4BFD2E7D  bl 0x82dfe798
	ctx.lr = 0x82E2B920;
	sub_82DFE798(ctx, base);
	// 82E2B920: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E2B924: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2B928: 394AC084  addi r10, r10, -0x3f7c
	ctx.r[10].s64 = ctx.r[10].s64 + -16252;
	// 82E2B92C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B930: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2B934: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E2B938: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E2B93C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E2B940: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E2B944: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2B948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2B94C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2B950: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2B954: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B958 size=76
    let mut pc: u32 = 0x82E2B958;
    'dispatch: loop {
        match pc {
            0x82E2B958 => {
    //   block [0x82E2B958..0x82E2B9A4)
	// 82E2B958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B95C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2B960: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2B964: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2B968: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B96C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2B970: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2B974: 4BFFFF2D  bl 0x82e2b8a0
	ctx.lr = 0x82E2B978;
	sub_82E2B8A0(ctx, base);
	// 82E2B978: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2B97C: 4182000C  beq 0x82e2b988
	if ctx.cr[0].eq {
	pc = 0x82E2B988; continue 'dispatch;
	}
	// 82E2B980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B984: 4BFC6A55  bl 0x82df23d8
	ctx.lr = 0x82E2B988;
	sub_82DF23D8(ctx, base);
	// 82E2B988: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2B98C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2B990: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2B994: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2B998: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2B99C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2B9A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2B9A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2B9A8 size=88
    let mut pc: u32 = 0x82E2B9A8;
    'dispatch: loop {
        match pc {
            0x82E2B9A8 => {
    //   block [0x82E2B9A8..0x82E2BA00)
	// 82E2B9A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2B9AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2B9B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2B9B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2B9B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2B9BC: 4BFFF77D  bl 0x82e2b138
	ctx.lr = 0x82E2B9C0;
	sub_82E2B138(ctx, base);
	// 82E2B9C0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E2B9C4: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82E2B9C8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2B9CC: 99630015  stb r11, 0x15(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(21 as u32), ctx.r[11].u8 ) };
	// 82E2B9D0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2B9D4: 916B0004  stw r11, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2B9D8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2B9DC: 916B0000  stw r11, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2B9E0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2B9E4: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2B9E8: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E2B9EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2B9F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2B9F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2B9F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2B9FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2BA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2BA00 size=84
    let mut pc: u32 = 0x82E2BA00;
    'dispatch: loop {
        match pc {
            0x82E2BA00 => {
    //   block [0x82E2BA00..0x82E2BA54)
	// 82E2BA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2BA04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2BA08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2BA0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2BA10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2BA14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2BA18: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2BA1C: 4BFFF77D  bl 0x82e2b198
	ctx.lr = 0x82E2BA20;
	sub_82E2B198(ctx, base);
	// 82E2BA20: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2BA24: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2BA28: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E2BA2C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2BA30: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2BA34: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2BA38: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2BA3C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2BA40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2BA44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2BA48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2BA4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2BA50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2BA58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2BA58 size=164
    let mut pc: u32 = 0x82E2BA58;
    'dispatch: loop {
        match pc {
            0x82E2BA58 => {
    //   block [0x82E2BA58..0x82E2BAFC)
	// 82E2BA58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2BA5C: 4837C709  bl 0x831a8164
	ctx.lr = 0x82E2BA60;
	sub_831A8130(ctx, base);
	// 82E2BA60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2BA64: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E2BA68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2BA6C: 576B07BD  rlwinm. r11, r27, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2BA70: 41820058  beq 0x82e2bac8
	if ctx.cr[0].eq {
	pc = 0x82E2BAC8; continue 'dispatch;
	}
	// 82E2BA74: 815FFFF0  lwz r10, -0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82E2BA78: 3B9FFFF0  addi r28, r31, -0x10
	ctx.r[28].s64 = ctx.r[31].s64 + -16;
	// 82E2BA7C: 1D6A0050  mulli r11, r10, 0x50
	ctx.r[11].s64 = ctx.r[10].s64 * 80;
	// 82E2BA80: 37CAFFFF  addic. r30, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E2BA84: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E2BA88: 41800028  blt 0x82e2bab0
	if ctx.cr[0].lt {
	pc = 0x82E2BAB0; continue 'dispatch;
	}
	// 82E2BA8C: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 82E2BA90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2BA94: 3BAB9B84  addi r29, r11, -0x647c
	ctx.r[29].s64 = ctx.r[11].s64 + -25724;
	// 82E2BA98: 3BFFFFB0  addi r31, r31, -0x50
	ctx.r[31].s64 = ctx.r[31].s64 + -80;
	// 82E2BA9C: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E2BAA0: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E2BAA4: 4BFC7985  bl 0x82df3428
	ctx.lr = 0x82E2BAA8;
	sub_82DF3428(ctx, base);
	// 82E2BAA8: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E2BAAC: 4080FFEC  bge 0x82e2ba98
	if !ctx.cr[0].lt {
	pc = 0x82E2BA98; continue 'dispatch;
	}
	// 82E2BAB0: 576B07FF  clrlwi. r11, r27, 0x1f
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2BAB4: 4182000C  beq 0x82e2bac0
	if ctx.cr[0].eq {
	pc = 0x82E2BAC0; continue 'dispatch;
	}
	// 82E2BAB8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E2BABC: 4BFC691D  bl 0x82df23d8
	ctx.lr = 0x82E2BAC0;
	sub_82DF23D8(ctx, base);
	// 82E2BAC0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82E2BAC4: 48000030  b 0x82e2baf4
	pc = 0x82E2BAF4; continue 'dispatch;
	// 82E2BAC8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82E2BACC: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82E2BAD0: 394A9B84  addi r10, r10, -0x647c
	ctx.r[10].s64 = ctx.r[10].s64 + -25724;
	// 82E2BAD4: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82E2BAD8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82E2BADC: 4BFC794D  bl 0x82df3428
	ctx.lr = 0x82E2BAE0;
	sub_82DF3428(ctx, base);
	// 82E2BAE0: 576B07FF  clrlwi. r11, r27, 0x1f
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2BAE4: 4182000C  beq 0x82e2baf0
	if ctx.cr[0].eq {
	pc = 0x82E2BAF0; continue 'dispatch;
	}
	// 82E2BAE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BAEC: 4BFC68ED  bl 0x82df23d8
	ctx.lr = 0x82E2BAF0;
	sub_82DF23D8(ctx, base);
	// 82E2BAF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BAF4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2BAF8: 4837C6BC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2BB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2BB00 size=108
    let mut pc: u32 = 0x82E2BB00;
    'dispatch: loop {
        match pc {
            0x82E2BB00 => {
    //   block [0x82E2BB00..0x82E2BB6C)
	// 82E2BB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2BB04: 4837C665  bl 0x831a8168
	ctx.lr = 0x82E2BB08;
	sub_831A8130(ctx, base);
	// 82E2BB08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2BB0C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2BB10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2BB14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2BB18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BB1C: 388BA794  addi r4, r11, -0x586c
	ctx.r[4].s64 = ctx.r[11].s64 + -22636;
	// 82E2BB20: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2BB24: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2BB28: 4BFC7EE1  bl 0x82df3a08
	ctx.lr = 0x82E2BB2C;
	sub_82DF3A08(ctx, base);
	// 82E2BB2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2BB30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2BB34: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BB38: 48003839  bl 0x82e2f370
	ctx.lr = 0x82E2BB3C;
	sub_82E2F370(ctx, base);
	// 82E2BB3C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2BB40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BB44: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2BB48: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2BB4C: 4BFFF8BD  bl 0x82e2b408
	ctx.lr = 0x82E2BB50;
	sub_82E2B408(ctx, base);
	// 82E2BB50: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BB54: 4BFC78D5  bl 0x82df3428
	ctx.lr = 0x82E2BB58;
	sub_82DF3428(ctx, base);
	// 82E2BB58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BB5C: 4BFC78CD  bl 0x82df3428
	ctx.lr = 0x82E2BB60;
	sub_82DF3428(ctx, base);
	// 82E2BB60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BB64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2BB68: 4837C650  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2BB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2BB70 size=108
    let mut pc: u32 = 0x82E2BB70;
    'dispatch: loop {
        match pc {
            0x82E2BB70 => {
    //   block [0x82E2BB70..0x82E2BBDC)
	// 82E2BB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2BB74: 4837C5F5  bl 0x831a8168
	ctx.lr = 0x82E2BB78;
	sub_831A8130(ctx, base);
	// 82E2BB78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2BB7C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2BB80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2BB84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2BB88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BB8C: 388BA798  addi r4, r11, -0x5868
	ctx.r[4].s64 = ctx.r[11].s64 + -22632;
	// 82E2BB90: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2BB94: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2BB98: 4BFC7E71  bl 0x82df3a08
	ctx.lr = 0x82E2BB9C;
	sub_82DF3A08(ctx, base);
	// 82E2BB9C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2BBA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2BBA4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BBA8: 480037C9  bl 0x82e2f370
	ctx.lr = 0x82E2BBAC;
	sub_82E2F370(ctx, base);
	// 82E2BBAC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2BBB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BBB4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2BBB8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2BBBC: 4BFFF8F5  bl 0x82e2b4b0
	ctx.lr = 0x82E2BBC0;
	sub_82E2B4B0(ctx, base);
	// 82E2BBC0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BBC4: 4BFC7865  bl 0x82df3428
	ctx.lr = 0x82E2BBC8;
	sub_82DF3428(ctx, base);
	// 82E2BBC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BBCC: 4BFC785D  bl 0x82df3428
	ctx.lr = 0x82E2BBD0;
	sub_82DF3428(ctx, base);
	// 82E2BBD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BBD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2BBD8: 4837C5E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2BBE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2BBE0 size=108
    let mut pc: u32 = 0x82E2BBE0;
    'dispatch: loop {
        match pc {
            0x82E2BBE0 => {
    //   block [0x82E2BBE0..0x82E2BC4C)
	// 82E2BBE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2BBE4: 4837C585  bl 0x831a8168
	ctx.lr = 0x82E2BBE8;
	sub_831A8130(ctx, base);
	// 82E2BBE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2BBEC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2BBF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2BBF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2BBF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BBFC: 388BA7AC  addi r4, r11, -0x5854
	ctx.r[4].s64 = ctx.r[11].s64 + -22612;
	// 82E2BC00: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2BC04: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2BC08: 4BFC7E01  bl 0x82df3a08
	ctx.lr = 0x82E2BC0C;
	sub_82DF3A08(ctx, base);
	// 82E2BC0C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2BC10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2BC14: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BC18: 48003759  bl 0x82e2f370
	ctx.lr = 0x82E2BC1C;
	sub_82E2F370(ctx, base);
	// 82E2BC1C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2BC20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BC24: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2BC28: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2BC2C: 4B7B35F5  bl 0x825df220
	ctx.lr = 0x82E2BC30;
	sub_825DF220(ctx, base);
	// 82E2BC30: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BC34: 4BFC77F5  bl 0x82df3428
	ctx.lr = 0x82E2BC38;
	sub_82DF3428(ctx, base);
	// 82E2BC38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BC3C: 4BFC77ED  bl 0x82df3428
	ctx.lr = 0x82E2BC40;
	sub_82DF3428(ctx, base);
	// 82E2BC40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BC44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2BC48: 4837C570  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2BC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2BC50 size=108
    let mut pc: u32 = 0x82E2BC50;
    'dispatch: loop {
        match pc {
            0x82E2BC50 => {
    //   block [0x82E2BC50..0x82E2BCBC)
	// 82E2BC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2BC54: 4837C515  bl 0x831a8168
	ctx.lr = 0x82E2BC58;
	sub_831A8130(ctx, base);
	// 82E2BC58: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2BC5C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2BC60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2BC64: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2BC68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BC6C: 388BA7C4  addi r4, r11, -0x583c
	ctx.r[4].s64 = ctx.r[11].s64 + -22588;
	// 82E2BC70: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2BC74: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2BC78: 4BFC7D91  bl 0x82df3a08
	ctx.lr = 0x82E2BC7C;
	sub_82DF3A08(ctx, base);
	// 82E2BC7C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2BC80: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2BC84: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BC88: 480036E9  bl 0x82e2f370
	ctx.lr = 0x82E2BC8C;
	sub_82E2F370(ctx, base);
	// 82E2BC8C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2BC90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BC94: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2BC98: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2BC9C: 4BFFF8BD  bl 0x82e2b558
	ctx.lr = 0x82E2BCA0;
	sub_82E2B558(ctx, base);
	// 82E2BCA0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BCA4: 4BFC7785  bl 0x82df3428
	ctx.lr = 0x82E2BCA8;
	sub_82DF3428(ctx, base);
	// 82E2BCA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BCAC: 4BFC777D  bl 0x82df3428
	ctx.lr = 0x82E2BCB0;
	sub_82DF3428(ctx, base);
	// 82E2BCB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BCB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2BCB8: 4837C500  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2BCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2BCC0 size=108
    let mut pc: u32 = 0x82E2BCC0;
    'dispatch: loop {
        match pc {
            0x82E2BCC0 => {
    //   block [0x82E2BCC0..0x82E2BD2C)
	// 82E2BCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2BCC4: 4837C4A5  bl 0x831a8168
	ctx.lr = 0x82E2BCC8;
	sub_831A8130(ctx, base);
	// 82E2BCC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2BCCC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2BCD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2BCD4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2BCD8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BCDC: 388BA7D0  addi r4, r11, -0x5830
	ctx.r[4].s64 = ctx.r[11].s64 + -22576;
	// 82E2BCE0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2BCE4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2BCE8: 4BFC7D21  bl 0x82df3a08
	ctx.lr = 0x82E2BCEC;
	sub_82DF3A08(ctx, base);
	// 82E2BCEC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2BCF0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2BCF4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BCF8: 48003679  bl 0x82e2f370
	ctx.lr = 0x82E2BCFC;
	sub_82E2F370(ctx, base);
	// 82E2BCFC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2BD00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BD04: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2BD08: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2BD0C: 4BFFF8F5  bl 0x82e2b600
	ctx.lr = 0x82E2BD10;
	sub_82E2B600(ctx, base);
	// 82E2BD10: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BD14: 4BFC7715  bl 0x82df3428
	ctx.lr = 0x82E2BD18;
	sub_82DF3428(ctx, base);
	// 82E2BD18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BD1C: 4BFC770D  bl 0x82df3428
	ctx.lr = 0x82E2BD20;
	sub_82DF3428(ctx, base);
	// 82E2BD20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BD24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2BD28: 4837C490  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2BD30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2BD30 size=108
    let mut pc: u32 = 0x82E2BD30;
    'dispatch: loop {
        match pc {
            0x82E2BD30 => {
    //   block [0x82E2BD30..0x82E2BD9C)
	// 82E2BD30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2BD34: 4837C435  bl 0x831a8168
	ctx.lr = 0x82E2BD38;
	sub_831A8130(ctx, base);
	// 82E2BD38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2BD3C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2BD40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2BD44: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2BD48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BD4C: 388BA7D4  addi r4, r11, -0x582c
	ctx.r[4].s64 = ctx.r[11].s64 + -22572;
	// 82E2BD50: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2BD54: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2BD58: 4BFC7CB1  bl 0x82df3a08
	ctx.lr = 0x82E2BD5C;
	sub_82DF3A08(ctx, base);
	// 82E2BD5C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2BD60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2BD64: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BD68: 48003609  bl 0x82e2f370
	ctx.lr = 0x82E2BD6C;
	sub_82E2F370(ctx, base);
	// 82E2BD6C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2BD70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BD74: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2BD78: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2BD7C: 4BFFF92D  bl 0x82e2b6a8
	ctx.lr = 0x82E2BD80;
	sub_82E2B6A8(ctx, base);
	// 82E2BD80: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BD84: 4BFC76A5  bl 0x82df3428
	ctx.lr = 0x82E2BD88;
	sub_82DF3428(ctx, base);
	// 82E2BD88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BD8C: 4BFC769D  bl 0x82df3428
	ctx.lr = 0x82E2BD90;
	sub_82DF3428(ctx, base);
	// 82E2BD90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BD94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2BD98: 4837C420  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2BDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2BDA0 size=108
    let mut pc: u32 = 0x82E2BDA0;
    'dispatch: loop {
        match pc {
            0x82E2BDA0 => {
    //   block [0x82E2BDA0..0x82E2BE0C)
	// 82E2BDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2BDA4: 4837C3C5  bl 0x831a8168
	ctx.lr = 0x82E2BDA8;
	sub_831A8130(ctx, base);
	// 82E2BDA8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2BDAC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2BDB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2BDB4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2BDB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BDBC: 388BA7DC  addi r4, r11, -0x5824
	ctx.r[4].s64 = ctx.r[11].s64 + -22564;
	// 82E2BDC0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2BDC4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2BDC8: 4BFC7C41  bl 0x82df3a08
	ctx.lr = 0x82E2BDCC;
	sub_82DF3A08(ctx, base);
	// 82E2BDCC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2BDD0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2BDD4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BDD8: 48003599  bl 0x82e2f370
	ctx.lr = 0x82E2BDDC;
	sub_82E2F370(ctx, base);
	// 82E2BDDC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2BDE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BDE4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2BDE8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2BDEC: 4BFFF965  bl 0x82e2b750
	ctx.lr = 0x82E2BDF0;
	sub_82E2B750(ctx, base);
	// 82E2BDF0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BDF4: 4BFC7635  bl 0x82df3428
	ctx.lr = 0x82E2BDF8;
	sub_82DF3428(ctx, base);
	// 82E2BDF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BDFC: 4BFC762D  bl 0x82df3428
	ctx.lr = 0x82E2BE00;
	sub_82DF3428(ctx, base);
	// 82E2BE00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BE04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2BE08: 4837C3B0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2BE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2BE10 size=108
    let mut pc: u32 = 0x82E2BE10;
    'dispatch: loop {
        match pc {
            0x82E2BE10 => {
    //   block [0x82E2BE10..0x82E2BE7C)
	// 82E2BE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2BE14: 4837C355  bl 0x831a8168
	ctx.lr = 0x82E2BE18;
	sub_831A8130(ctx, base);
	// 82E2BE18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2BE1C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2BE20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2BE24: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2BE28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BE2C: 388BA804  addi r4, r11, -0x57fc
	ctx.r[4].s64 = ctx.r[11].s64 + -22524;
	// 82E2BE30: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2BE34: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2BE38: 4BFC7BD1  bl 0x82df3a08
	ctx.lr = 0x82E2BE3C;
	sub_82DF3A08(ctx, base);
	// 82E2BE3C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2BE40: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2BE44: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BE48: 48003529  bl 0x82e2f370
	ctx.lr = 0x82E2BE4C;
	sub_82E2F370(ctx, base);
	// 82E2BE4C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2BE50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BE54: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2BE58: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2BE5C: 4BFFF99D  bl 0x82e2b7f8
	ctx.lr = 0x82E2BE60;
	sub_82E2B7F8(ctx, base);
	// 82E2BE60: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2BE64: 4BFC75C5  bl 0x82df3428
	ctx.lr = 0x82E2BE68;
	sub_82DF3428(ctx, base);
	// 82E2BE68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2BE6C: 4BFC75BD  bl 0x82df3428
	ctx.lr = 0x82E2BE70;
	sub_82DF3428(ctx, base);
	// 82E2BE70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BE74: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2BE78: 4837C340  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2BE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2BE80 size=112
    let mut pc: u32 = 0x82E2BE80;
    'dispatch: loop {
        match pc {
            0x82E2BE80 => {
    //   block [0x82E2BE80..0x82E2BEF0)
	// 82E2BE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2BE84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2BE88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2BE8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2BE90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2BE94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2BE98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2BE9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2BEA0: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2BEA4: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2BEA8: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82E2BEAC: 4BFC653D  bl 0x82df23e8
	ctx.lr = 0x82E2BEB0;
	sub_82DF23E8(ctx, base);
	// 82E2BEB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2BEB4: 41820010  beq 0x82e2bec4
	if ctx.cr[0].eq {
	pc = 0x82E2BEC4; continue 'dispatch;
	}
	// 82E2BEB8: 4B4B9691  bl 0x822e5548
	ctx.lr = 0x82E2BEBC;
	sub_822E5548(ctx, base);
	// 82E2BEBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2BEC0: 48000008  b 0x82e2bec8
	pc = 0x82E2BEC8; continue 'dispatch;
	// 82E2BEC4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2BEC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BECC: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2BED0: 4BFD29A9  bl 0x82dfe878
	ctx.lr = 0x82E2BED4;
	sub_82DFE878(ctx, base);
	// 82E2BED4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BED8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2BEDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2BEE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2BEE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2BEE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2BEEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2BEF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2BEF0 size=112
    let mut pc: u32 = 0x82E2BEF0;
    'dispatch: loop {
        match pc {
            0x82E2BEF0 => {
    //   block [0x82E2BEF0..0x82E2BF60)
	// 82E2BEF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2BEF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2BEF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2BEFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2BF00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2BF04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2BF08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2BF0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2BF10: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2BF14: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2BF18: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82E2BF1C: 4BFC64CD  bl 0x82df23e8
	ctx.lr = 0x82E2BF20;
	sub_82DF23E8(ctx, base);
	// 82E2BF20: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2BF24: 41820010  beq 0x82e2bf34
	if ctx.cr[0].eq {
	pc = 0x82E2BF34; continue 'dispatch;
	}
	// 82E2BF28: 4BFFF9E1  bl 0x82e2b908
	ctx.lr = 0x82E2BF2C;
	sub_82E2B908(ctx, base);
	// 82E2BF2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2BF30: 48000008  b 0x82e2bf38
	pc = 0x82E2BF38; continue 'dispatch;
	// 82E2BF34: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2BF38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BF3C: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2BF40: 4BFD2939  bl 0x82dfe878
	ctx.lr = 0x82E2BF44;
	sub_82DFE878(ctx, base);
	// 82E2BF44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BF48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2BF4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2BF50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2BF54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2BF58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2BF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2BF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2BF60 size=176
    let mut pc: u32 = 0x82E2BF60;
    'dispatch: loop {
        match pc {
            0x82E2BF60 => {
    //   block [0x82E2BF60..0x82E2C010)
	// 82E2BF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2BF64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2BF68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2BF6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2BF70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2BF74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2BF78: 4BFD2821  bl 0x82dfe798
	ctx.lr = 0x82E2BF7C;
	sub_82DFE798(ctx, base);
	// 82E2BF7C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E2BF80: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E2BF84: 396BC08C  addi r11, r11, -0x3f74
	ctx.r[11].s64 = ctx.r[11].s64 + -16244;
	// 82E2BF88: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2BF8C: 4BFFEA15  bl 0x82e2a9a0
	ctx.lr = 0x82E2BF90;
	sub_82E2A9A0(ctx, base);
	// 82E2BF90: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82E2BF94: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E2BF98: 397F0034  addi r11, r31, 0x34
	ctx.r[11].s64 = ctx.r[31].s64 + 52;
	// 82E2BF9C: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 82E2BFA0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E2BFA4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E2BFA8: 9BCA0000  stb r30, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82E2BFAC: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E2BFB0: 4BFFF249  bl 0x82e2b1f8
	ctx.lr = 0x82E2BFB4;
	sub_82E2B1F8(ctx, base);
	// 82E2BFB4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82E2BFB8: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 82E2BFBC: 397F0040  addi r11, r31, 0x40
	ctx.r[11].s64 = ctx.r[31].s64 + 64;
	// 82E2BFC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E2BFC4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E2BFC8: 9BCA0000  stb r30, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82E2BFCC: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E2BFD0: 4BFFF2D9  bl 0x82e2b2a8
	ctx.lr = 0x82E2BFD4;
	sub_82E2B2A8(ctx, base);
	// 82E2BFD4: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82E2BFD8: 93DF004C  stw r30, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[30].u32 ) };
	// 82E2BFDC: 397F004C  addi r11, r31, 0x4c
	ctx.r[11].s64 = ctx.r[31].s64 + 76;
	// 82E2BFE0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E2BFE4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E2BFE8: 9BCA0000  stb r30, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82E2BFEC: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E2BFF0: 4BFEAB59  bl 0x82e16b48
	ctx.lr = 0x82E2BFF4;
	sub_82E16B48(ctx, base);
	// 82E2BFF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2BFF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2BFFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2C000: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2C004: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2C008: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2C00C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C010 size=132
    let mut pc: u32 = 0x82E2C010;
    'dispatch: loop {
        match pc {
            0x82E2C010 => {
    //   block [0x82E2C010..0x82E2C094)
	// 82E2C010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2C018: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2C01C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2C020: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C024: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C028: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E2C02C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2C030: 419A0008  beq cr6, 0x82e2c038
	if ctx.cr[6].eq {
	pc = 0x82E2C038; continue 'dispatch;
	}
	// 82E2C034: 4B49485D  bl 0x822c0890
	ctx.lr = 0x82E2C038;
	sub_822C0890(ctx, base);
	// 82E2C038: 807F0044  lwz r3, 0x44(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82E2C03C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2C040: 419A0008  beq cr6, 0x82e2c048
	if ctx.cr[6].eq {
	pc = 0x82E2C048; continue 'dispatch;
	}
	// 82E2C044: 4B49484D  bl 0x822c0890
	ctx.lr = 0x82E2C048;
	sub_822C0890(ctx, base);
	// 82E2C048: 807F0038  lwz r3, 0x38(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E2C04C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2C050: 419A0008  beq cr6, 0x82e2c058
	if ctx.cr[6].eq {
	pc = 0x82E2C058; continue 'dispatch;
	}
	// 82E2C054: 4B49483D  bl 0x822c0890
	ctx.lr = 0x82E2C058;
	sub_822C0890(ctx, base);
	// 82E2C058: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82E2C05C: 397F000C  addi r11, r31, 0xc
	ctx.r[11].s64 = ctx.r[31].s64 + 12;
	// 82E2C060: 3BCA9B84  addi r30, r10, -0x647c
	ctx.r[30].s64 = ctx.r[10].s64 + -25724;
	// 82E2C064: 386B0008  addi r3, r11, 8
	ctx.r[3].s64 = ctx.r[11].s64 + 8;
	// 82E2C068: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82E2C06C: 4BFC73BD  bl 0x82df3428
	ctx.lr = 0x82E2C070;
	sub_82DF3428(ctx, base);
	// 82E2C070: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E2C074: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E2C078: 4BFC73B1  bl 0x82df3428
	ctx.lr = 0x82E2C07C;
	sub_82DF3428(ctx, base);
	// 82E2C07C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2C080: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2C084: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2C088: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2C08C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2C090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C098 size=76
    let mut pc: u32 = 0x82E2C098;
    'dispatch: loop {
        match pc {
            0x82E2C098 => {
    //   block [0x82E2C098..0x82E2C0E4)
	// 82E2C098: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C09C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2C0A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2C0A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2C0A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C0AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C0B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2C0B4: 4BFFFF5D  bl 0x82e2c010
	ctx.lr = 0x82E2C0B8;
	sub_82E2C010(ctx, base);
	// 82E2C0B8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2C0BC: 4182000C  beq 0x82e2c0c8
	if ctx.cr[0].eq {
	pc = 0x82E2C0C8; continue 'dispatch;
	}
	// 82E2C0C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C0C4: 4BFC6315  bl 0x82df23d8
	ctx.lr = 0x82E2C0C8;
	sub_82DF23D8(ctx, base);
	// 82E2C0C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C0CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2C0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2C0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2C0D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2C0DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2C0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C0E8 size=152
    let mut pc: u32 = 0x82E2C0E8;
    'dispatch: loop {
        match pc {
            0x82E2C0E8 => {
    //   block [0x82E2C0E8..0x82E2C180)
	// 82E2C0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2C0F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2C0F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2C0F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C0FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C100: 4BFD2699  bl 0x82dfe798
	ctx.lr = 0x82E2C104;
	sub_82DFE798(ctx, base);
	// 82E2C104: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E2C108: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82E2C10C: 396BC094  addi r11, r11, -0x3f6c
	ctx.r[11].s64 = ctx.r[11].s64 + -16236;
	// 82E2C110: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E2C114: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2C118: 397F0018  addi r11, r31, 0x18
	ctx.r[11].s64 = ctx.r[31].s64 + 24;
	// 82E2C11C: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82E2C120: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E2C124: 9BCA0000  stb r30, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82E2C128: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82E2C12C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E2C130: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 82E2C134: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E2C138: 4BFFF221  bl 0x82e2b358
	ctx.lr = 0x82E2C13C;
	sub_82E2B358(ctx, base);
	// 82E2C13C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E2C140: 9BCB0000  stb r30, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82E2C144: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 82E2C148: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82E2C14C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E2C150: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E2C154: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E2C158: 4B5EFCF1  bl 0x8241be48
	ctx.lr = 0x82E2C15C;
	sub_8241BE48(ctx, base);
	// 82E2C15C: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 82E2C160: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82E2C164: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C168: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2C16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2C170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2C174: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2C178: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2C17C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C180 size=124
    let mut pc: u32 = 0x82E2C180;
    'dispatch: loop {
        match pc {
            0x82E2C180 => {
    //   block [0x82E2C180..0x82E2C1FC)
	// 82E2C180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2C188: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2C18C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C190: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C194: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82E2C198: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2C19C: 419A0008  beq cr6, 0x82e2c1a4
	if ctx.cr[6].eq {
	pc = 0x82E2C1A4; continue 'dispatch;
	}
	// 82E2C1A0: 4B4946F1  bl 0x822c0890
	ctx.lr = 0x82E2C1A4;
	sub_822C0890(ctx, base);
	// 82E2C1A4: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E2C1A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2C1AC: 419A0008  beq cr6, 0x82e2c1b4
	if ctx.cr[6].eq {
	pc = 0x82E2C1B4; continue 'dispatch;
	}
	// 82E2C1B0: 4B4946E1  bl 0x822c0890
	ctx.lr = 0x82E2C1B4;
	sub_822C0890(ctx, base);
	// 82E2C1B4: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E2C1B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2C1BC: 419A0008  beq cr6, 0x82e2c1c4
	if ctx.cr[6].eq {
	pc = 0x82E2C1C4; continue 'dispatch;
	}
	// 82E2C1C0: 4B4946D1  bl 0x822c0890
	ctx.lr = 0x82E2C1C4;
	sub_822C0890(ctx, base);
	// 82E2C1C4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E2C1C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2C1CC: 419A0008  beq cr6, 0x82e2c1d4
	if ctx.cr[6].eq {
	pc = 0x82E2C1D4; continue 'dispatch;
	}
	// 82E2C1D0: 4B4946C1  bl 0x822c0890
	ctx.lr = 0x82E2C1D4;
	sub_822C0890(ctx, base);
	// 82E2C1D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2C1D8: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E2C1DC: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82E2C1E0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2C1E4: 4BFC7245  bl 0x82df3428
	ctx.lr = 0x82E2C1E8;
	sub_82DF3428(ctx, base);
	// 82E2C1E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2C1EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2C1F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2C1F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2C1F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C200 size=76
    let mut pc: u32 = 0x82E2C200;
    'dispatch: loop {
        match pc {
            0x82E2C200 => {
    //   block [0x82E2C200..0x82E2C24C)
	// 82E2C200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2C208: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2C20C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2C210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C214: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C218: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2C21C: 4BFFFF65  bl 0x82e2c180
	ctx.lr = 0x82E2C220;
	sub_82E2C180(ctx, base);
	// 82E2C220: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2C224: 4182000C  beq 0x82e2c230
	if ctx.cr[0].eq {
	pc = 0x82E2C230; continue 'dispatch;
	}
	// 82E2C228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C22C: 4BFC61AD  bl 0x82df23d8
	ctx.lr = 0x82E2C230;
	sub_82DF23D8(ctx, base);
	// 82E2C230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C234: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2C238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2C23C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2C240: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2C244: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2C248: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C250 size=164
    let mut pc: u32 = 0x82E2C250;
    'dispatch: loop {
        match pc {
            0x82E2C250 => {
    //   block [0x82E2C250..0x82E2C2F4)
	// 82E2C250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C254: 4837BF19  bl 0x831a816c
	ctx.lr = 0x82E2C258;
	sub_831A8130(ctx, base);
	// 82E2C258: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C25C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2C260: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C264: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2C268: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2C26C: 396BBE80  addi r11, r11, -0x4180
	ctx.r[11].s64 = ctx.r[11].s64 + -16768;
	// 82E2C270: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2C274: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2C278: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2C27C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2C280: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2C284: 4BFD2675  bl 0x82dfe8f8
	ctx.lr = 0x82E2C288;
	sub_82DFE8F8(ctx, base);
	// 82E2C288: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2C28C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2C290: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2C294: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2C298: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2C29C: 4BFD55FD  bl 0x82e01898
	ctx.lr = 0x82E2C2A0;
	sub_82E01898(ctx, base);
	// 82E2C2A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2C2A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2C2A8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2C2AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2C2B0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2C2B4: 419A0024  beq cr6, 0x82e2c2d8
	if ctx.cr[6].eq {
	pc = 0x82E2C2D8; continue 'dispatch;
	}
	// 82E2C2B8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2C2BC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2C2C0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2C2C4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2C2C8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2C2CC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2C2D0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2C2D4: 4082FFE8  bne 0x82e2c2bc
	if !ctx.cr[0].eq {
	pc = 0x82E2C2BC; continue 'dispatch;
	}
	// 82E2C2D8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2C2DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2C2E0: 419A0008  beq cr6, 0x82e2c2e8
	if ctx.cr[6].eq {
	pc = 0x82E2C2E8; continue 'dispatch;
	}
	// 82E2C2E4: 4B4945AD  bl 0x822c0890
	ctx.lr = 0x82E2C2E8;
	sub_822C0890(ctx, base);
	// 82E2C2E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C2EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2C2F0: 4837BECC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C2F8 size=164
    let mut pc: u32 = 0x82E2C2F8;
    'dispatch: loop {
        match pc {
            0x82E2C2F8 => {
    //   block [0x82E2C2F8..0x82E2C39C)
	// 82E2C2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C2FC: 4837BE71  bl 0x831a816c
	ctx.lr = 0x82E2C300;
	sub_831A8130(ctx, base);
	// 82E2C300: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C304: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2C308: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C30C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2C310: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2C314: 396BBEF0  addi r11, r11, -0x4110
	ctx.r[11].s64 = ctx.r[11].s64 + -16656;
	// 82E2C318: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2C31C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2C320: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2C324: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2C328: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2C32C: 4BFD25CD  bl 0x82dfe8f8
	ctx.lr = 0x82E2C330;
	sub_82DFE8F8(ctx, base);
	// 82E2C330: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2C334: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2C338: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2C33C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2C340: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2C344: 4BFD5555  bl 0x82e01898
	ctx.lr = 0x82E2C348;
	sub_82E01898(ctx, base);
	// 82E2C348: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2C34C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2C350: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2C354: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2C358: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2C35C: 419A0024  beq cr6, 0x82e2c380
	if ctx.cr[6].eq {
	pc = 0x82E2C380; continue 'dispatch;
	}
	// 82E2C360: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2C364: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2C368: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2C36C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2C370: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2C374: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2C378: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2C37C: 4082FFE8  bne 0x82e2c364
	if !ctx.cr[0].eq {
	pc = 0x82E2C364; continue 'dispatch;
	}
	// 82E2C380: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2C384: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2C388: 419A0008  beq cr6, 0x82e2c390
	if ctx.cr[6].eq {
	pc = 0x82E2C390; continue 'dispatch;
	}
	// 82E2C38C: 4B494505  bl 0x822c0890
	ctx.lr = 0x82E2C390;
	sub_822C0890(ctx, base);
	// 82E2C390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C394: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2C398: 4837BE24  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C3A0 size=112
    let mut pc: u32 = 0x82E2C3A0;
    'dispatch: loop {
        match pc {
            0x82E2C3A0 => {
    //   block [0x82E2C3A0..0x82E2C410)
	// 82E2C3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2C3A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2C3AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2C3B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C3B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2C3B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2C3BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2C3C0: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2C3C4: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2C3C8: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 82E2C3CC: 4BFC601D  bl 0x82df23e8
	ctx.lr = 0x82E2C3D0;
	sub_82DF23E8(ctx, base);
	// 82E2C3D0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2C3D4: 41820010  beq 0x82e2c3e4
	if ctx.cr[0].eq {
	pc = 0x82E2C3E4; continue 'dispatch;
	}
	// 82E2C3D8: 4BFFFB89  bl 0x82e2bf60
	ctx.lr = 0x82E2C3DC;
	sub_82E2BF60(ctx, base);
	// 82E2C3DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C3E0: 48000008  b 0x82e2c3e8
	pc = 0x82E2C3E8; continue 'dispatch;
	// 82E2C3E4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2C3E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C3EC: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2C3F0: 4BFD2489  bl 0x82dfe878
	ctx.lr = 0x82E2C3F4;
	sub_82DFE878(ctx, base);
	// 82E2C3F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C3F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2C3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2C400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2C404: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2C408: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2C40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C410 size=112
    let mut pc: u32 = 0x82E2C410;
    'dispatch: loop {
        match pc {
            0x82E2C410 => {
    //   block [0x82E2C410..0x82E2C480)
	// 82E2C410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2C418: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2C41C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2C420: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C424: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2C428: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2C42C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2C430: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2C434: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2C438: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82E2C43C: 4BFC5FAD  bl 0x82df23e8
	ctx.lr = 0x82E2C440;
	sub_82DF23E8(ctx, base);
	// 82E2C440: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2C444: 41820010  beq 0x82e2c454
	if ctx.cr[0].eq {
	pc = 0x82E2C454; continue 'dispatch;
	}
	// 82E2C448: 4BFFFCA1  bl 0x82e2c0e8
	ctx.lr = 0x82E2C44C;
	sub_82E2C0E8(ctx, base);
	// 82E2C44C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C450: 48000008  b 0x82e2c458
	pc = 0x82E2C458; continue 'dispatch;
	// 82E2C454: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2C458: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C45C: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2C460: 4BFD2419  bl 0x82dfe878
	ctx.lr = 0x82E2C464;
	sub_82DFE878(ctx, base);
	// 82E2C464: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C468: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2C46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2C470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2C474: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2C478: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2C47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E2C480 size=12
    let mut pc: u32 = 0x82E2C480;
    'dispatch: loop {
        match pc {
            0x82E2C480 => {
    //   block [0x82E2C480..0x82E2C48C)
	// 82E2C480: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E2C484: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2C488: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C48C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E2C48C size=8
    let mut pc: u32 = 0x82E2C48C;
    'dispatch: loop {
        match pc {
            0x82E2C48C => {
    //   block [0x82E2C48C..0x82E2C494)
	// 82E2C48C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E2C490: 4BFFF5C8  b 0x82e2ba58
	sub_82E2BA58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C494(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E2C494 size=4
    let mut pc: u32 = 0x82E2C494;
    'dispatch: loop {
        match pc {
            0x82E2C494 => {
    //   block [0x82E2C494..0x82E2C498)
	// 82E2C494: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C498 size=108
    let mut pc: u32 = 0x82E2C498;
    'dispatch: loop {
        match pc {
            0x82E2C498 => {
    //   block [0x82E2C498..0x82E2C504)
	// 82E2C498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C49C: 4837BCCD  bl 0x831a8168
	ctx.lr = 0x82E2C4A0;
	sub_831A8130(ctx, base);
	// 82E2C4A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C4A4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2C4A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C4AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2C4B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2C4B4: 388BA7B4  addi r4, r11, -0x584c
	ctx.r[4].s64 = ctx.r[11].s64 + -22604;
	// 82E2C4B8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2C4BC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2C4C0: 4BFC7549  bl 0x82df3a08
	ctx.lr = 0x82E2C4C4;
	sub_82DF3A08(ctx, base);
	// 82E2C4C4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2C4C8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2C4CC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2C4D0: 48002EA1  bl 0x82e2f370
	ctx.lr = 0x82E2C4D4;
	sub_82E2F370(ctx, base);
	// 82E2C4D4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2C4D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C4DC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2C4E0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2C4E4: 4BFFFE15  bl 0x82e2c2f8
	ctx.lr = 0x82E2C4E8;
	sub_82E2C2F8(ctx, base);
	// 82E2C4E8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2C4EC: 4BFC6F3D  bl 0x82df3428
	ctx.lr = 0x82E2C4F0;
	sub_82DF3428(ctx, base);
	// 82E2C4F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2C4F4: 4BFC6F35  bl 0x82df3428
	ctx.lr = 0x82E2C4F8;
	sub_82DF3428(ctx, base);
	// 82E2C4F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C4FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2C500: 4837BCB8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C508 size=108
    let mut pc: u32 = 0x82E2C508;
    'dispatch: loop {
        match pc {
            0x82E2C508 => {
    //   block [0x82E2C508..0x82E2C574)
	// 82E2C508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C50C: 4837BC5D  bl 0x831a8168
	ctx.lr = 0x82E2C510;
	sub_831A8130(ctx, base);
	// 82E2C510: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C514: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2C518: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C51C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2C520: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2C524: 388BA7E0  addi r4, r11, -0x5820
	ctx.r[4].s64 = ctx.r[11].s64 + -22560;
	// 82E2C528: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2C52C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2C530: 4BFC74D9  bl 0x82df3a08
	ctx.lr = 0x82E2C534;
	sub_82DF3A08(ctx, base);
	// 82E2C534: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2C538: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2C53C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2C540: 48002E31  bl 0x82e2f370
	ctx.lr = 0x82E2C544;
	sub_82E2F370(ctx, base);
	// 82E2C544: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2C548: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C54C: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2C550: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2C554: 4BFFFCFD  bl 0x82e2c250
	ctx.lr = 0x82E2C558;
	sub_82E2C250(ctx, base);
	// 82E2C558: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2C55C: 4BFC6ECD  bl 0x82df3428
	ctx.lr = 0x82E2C560;
	sub_82DF3428(ctx, base);
	// 82E2C560: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2C564: 4BFC6EC5  bl 0x82df3428
	ctx.lr = 0x82E2C568;
	sub_82DF3428(ctx, base);
	// 82E2C568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C56C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2C570: 4837BC48  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C578 size=108
    let mut pc: u32 = 0x82E2C578;
    'dispatch: loop {
        match pc {
            0x82E2C578 => {
    //   block [0x82E2C578..0x82E2C5E4)
	// 82E2C578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C57C: 4837BBED  bl 0x831a8168
	ctx.lr = 0x82E2C580;
	sub_831A8130(ctx, base);
	// 82E2C580: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C584: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2C588: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C58C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2C590: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2C594: 388BA7EC  addi r4, r11, -0x5814
	ctx.r[4].s64 = ctx.r[11].s64 + -22548;
	// 82E2C598: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2C59C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2C5A0: 4BFC7469  bl 0x82df3a08
	ctx.lr = 0x82E2C5A4;
	sub_82DF3A08(ctx, base);
	// 82E2C5A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2C5A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2C5AC: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2C5B0: 48002DC1  bl 0x82e2f370
	ctx.lr = 0x82E2C5B4;
	sub_82E2F370(ctx, base);
	// 82E2C5B4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2C5B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C5BC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2C5C0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2C5C4: 4BFFFD35  bl 0x82e2c2f8
	ctx.lr = 0x82E2C5C8;
	sub_82E2C2F8(ctx, base);
	// 82E2C5C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2C5CC: 4BFC6E5D  bl 0x82df3428
	ctx.lr = 0x82E2C5D0;
	sub_82DF3428(ctx, base);
	// 82E2C5D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2C5D4: 4BFC6E55  bl 0x82df3428
	ctx.lr = 0x82E2C5D8;
	sub_82DF3428(ctx, base);
	// 82E2C5D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C5DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2C5E0: 4837BBD8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C5E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C5E8 size=164
    let mut pc: u32 = 0x82E2C5E8;
    'dispatch: loop {
        match pc {
            0x82E2C5E8 => {
    //   block [0x82E2C5E8..0x82E2C68C)
	// 82E2C5E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C5EC: 4837BB81  bl 0x831a816c
	ctx.lr = 0x82E2C5F0;
	sub_831A8130(ctx, base);
	// 82E2C5F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C5F4: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2C5F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C5FC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2C600: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2C604: 396BC3A0  addi r11, r11, -0x3c60
	ctx.r[11].s64 = ctx.r[11].s64 + -15456;
	// 82E2C608: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2C60C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2C610: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2C614: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2C618: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2C61C: 4BFD22DD  bl 0x82dfe8f8
	ctx.lr = 0x82E2C620;
	sub_82DFE8F8(ctx, base);
	// 82E2C620: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2C624: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2C628: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2C62C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2C630: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2C634: 4BFD5265  bl 0x82e01898
	ctx.lr = 0x82E2C638;
	sub_82E01898(ctx, base);
	// 82E2C638: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2C63C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2C640: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2C644: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2C648: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2C64C: 419A0024  beq cr6, 0x82e2c670
	if ctx.cr[6].eq {
	pc = 0x82E2C670; continue 'dispatch;
	}
	// 82E2C650: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2C654: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2C658: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2C65C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2C660: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2C664: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2C668: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2C66C: 4082FFE8  bne 0x82e2c654
	if !ctx.cr[0].eq {
	pc = 0x82E2C654; continue 'dispatch;
	}
	// 82E2C670: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2C674: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2C678: 419A0008  beq cr6, 0x82e2c680
	if ctx.cr[6].eq {
	pc = 0x82E2C680; continue 'dispatch;
	}
	// 82E2C67C: 4B494215  bl 0x822c0890
	ctx.lr = 0x82E2C680;
	sub_822C0890(ctx, base);
	// 82E2C680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C684: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2C688: 4837BB34  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C690 size=164
    let mut pc: u32 = 0x82E2C690;
    'dispatch: loop {
        match pc {
            0x82E2C690 => {
    //   block [0x82E2C690..0x82E2C734)
	// 82E2C690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C694: 4837BAD9  bl 0x831a816c
	ctx.lr = 0x82E2C698;
	sub_831A8130(ctx, base);
	// 82E2C698: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C69C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2C6A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C6A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2C6A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2C6AC: 396BC410  addi r11, r11, -0x3bf0
	ctx.r[11].s64 = ctx.r[11].s64 + -15344;
	// 82E2C6B0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2C6B4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2C6B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2C6BC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2C6C0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2C6C4: 4BFD2235  bl 0x82dfe8f8
	ctx.lr = 0x82E2C6C8;
	sub_82DFE8F8(ctx, base);
	// 82E2C6C8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2C6CC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2C6D0: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2C6D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2C6D8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2C6DC: 4BFD51BD  bl 0x82e01898
	ctx.lr = 0x82E2C6E0;
	sub_82E01898(ctx, base);
	// 82E2C6E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2C6E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2C6E8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2C6EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2C6F0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2C6F4: 419A0024  beq cr6, 0x82e2c718
	if ctx.cr[6].eq {
	pc = 0x82E2C718; continue 'dispatch;
	}
	// 82E2C6F8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2C6FC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2C700: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2C704: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2C708: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2C70C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2C710: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2C714: 4082FFE8  bne 0x82e2c6fc
	if !ctx.cr[0].eq {
	pc = 0x82E2C6FC; continue 'dispatch;
	}
	// 82E2C718: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2C71C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2C720: 419A0008  beq cr6, 0x82e2c728
	if ctx.cr[6].eq {
	pc = 0x82E2C728; continue 'dispatch;
	}
	// 82E2C724: 4B49416D  bl 0x822c0890
	ctx.lr = 0x82E2C728;
	sub_822C0890(ctx, base);
	// 82E2C728: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C72C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2C730: 4837BA8C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C738 size=184
    let mut pc: u32 = 0x82E2C738;
    'dispatch: loop {
        match pc {
            0x82E2C738 => {
    //   block [0x82E2C738..0x82E2C7F0)
	// 82E2C738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2C740: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2C744: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2C748: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C74C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2C750: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2C754: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82E2C758: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E2C75C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2C760: 4B4941D9  bl 0x822c0938
	ctx.lr = 0x82E2C764;
	sub_822C0938(ctx, base);
	// 82E2C764: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2C768: 41820028  beq 0x82e2c790
	if ctx.cr[0].eq {
	pc = 0x82E2C790; continue 'dispatch;
	}
	// 82E2C76C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E2C770: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E2C774: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E2C778: 392BC064  addi r9, r11, -0x3f9c
	ctx.r[9].s64 = ctx.r[11].s64 + -16284;
	// 82E2C77C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E2C780: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E2C784: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E2C788: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E2C78C: 48000008  b 0x82e2c794
	pc = 0x82E2C794; continue 'dispatch;
	// 82E2C790: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2C794: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2C798: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2C79C: 409A0038  bne cr6, 0x82e2c7d4
	if !ctx.cr[6].eq {
	pc = 0x82E2C7D4; continue 'dispatch;
	}
	// 82E2C7A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E2C7A4: 419A0010  beq cr6, 0x82e2c7b4
	if ctx.cr[6].eq {
	pc = 0x82E2C7B4; continue 'dispatch;
	}
	// 82E2C7A8: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82E2C7AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C7B0: 4BFFF2A9  bl 0x82e2ba58
	ctx.lr = 0x82E2C7B4;
	sub_82E2BA58(ctx, base);
	// 82E2C7B4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2C7B8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E2C7BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2C7C0: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E2C7C4: 816BA62C  lwz r11, -0x59d4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22996 as u32) ) } as u64;
	// 82E2C7C8: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E2C7CC: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E2C7D0: 4B493831  bl 0x822c0000
	ctx.lr = 0x82E2C7D4;
	sub_822C0000(ctx, base);
	// 82E2C7D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E2C7D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2C7DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2C7E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2C7E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2C7E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2C7EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C7F0 size=108
    let mut pc: u32 = 0x82E2C7F0;
    'dispatch: loop {
        match pc {
            0x82E2C7F0 => {
    //   block [0x82E2C7F0..0x82E2C85C)
	// 82E2C7F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C7F4: 4837B975  bl 0x831a8168
	ctx.lr = 0x82E2C7F8;
	sub_831A8130(ctx, base);
	// 82E2C7F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C7FC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2C800: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C804: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2C808: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2C80C: 388BA7A0  addi r4, r11, -0x5860
	ctx.r[4].s64 = ctx.r[11].s64 + -22624;
	// 82E2C810: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2C814: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2C818: 4BFC71F1  bl 0x82df3a08
	ctx.lr = 0x82E2C81C;
	sub_82DF3A08(ctx, base);
	// 82E2C81C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2C820: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2C824: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2C828: 48002B49  bl 0x82e2f370
	ctx.lr = 0x82E2C82C;
	sub_82E2F370(ctx, base);
	// 82E2C82C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2C830: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C834: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2C838: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2C83C: 4BFFFDAD  bl 0x82e2c5e8
	ctx.lr = 0x82E2C840;
	sub_82E2C5E8(ctx, base);
	// 82E2C840: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2C844: 4BFC6BE5  bl 0x82df3428
	ctx.lr = 0x82E2C848;
	sub_82DF3428(ctx, base);
	// 82E2C848: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2C84C: 4BFC6BDD  bl 0x82df3428
	ctx.lr = 0x82E2C850;
	sub_82DF3428(ctx, base);
	// 82E2C850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C854: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2C858: 4837B960  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C860 size=108
    let mut pc: u32 = 0x82E2C860;
    'dispatch: loop {
        match pc {
            0x82E2C860 => {
    //   block [0x82E2C860..0x82E2C8CC)
	// 82E2C860: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C864: 4837B905  bl 0x831a8168
	ctx.lr = 0x82E2C868;
	sub_831A8130(ctx, base);
	// 82E2C868: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C86C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2C870: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2C874: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2C878: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2C87C: 388BA800  addi r4, r11, -0x5800
	ctx.r[4].s64 = ctx.r[11].s64 + -22528;
	// 82E2C880: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2C884: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2C888: 4BFC7181  bl 0x82df3a08
	ctx.lr = 0x82E2C88C;
	sub_82DF3A08(ctx, base);
	// 82E2C88C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2C890: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2C894: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2C898: 48002AD9  bl 0x82e2f370
	ctx.lr = 0x82E2C89C;
	sub_82E2F370(ctx, base);
	// 82E2C89C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2C8A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C8A4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2C8A8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2C8AC: 4BFFFDE5  bl 0x82e2c690
	ctx.lr = 0x82E2C8B0;
	sub_82E2C690(ctx, base);
	// 82E2C8B0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2C8B4: 4BFC6B75  bl 0x82df3428
	ctx.lr = 0x82E2C8B8;
	sub_82DF3428(ctx, base);
	// 82E2C8B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2C8BC: 4BFC6B6D  bl 0x82df3428
	ctx.lr = 0x82E2C8C0;
	sub_82DF3428(ctx, base);
	// 82E2C8C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2C8C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2C8C8: 4837B8F0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2C8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2C8D0 size=1008
    let mut pc: u32 = 0x82E2C8D0;
    'dispatch: loop {
        match pc {
            0x82E2C8D0 => {
    //   block [0x82E2C8D0..0x82E2CCC0)
	// 82E2C8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2C8D4: 4837B885  bl 0x831a8158
	ctx.lr = 0x82E2C8D8;
	sub_831A8130(ctx, base);
	// 82E2C8D8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2C8DC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E2C8E0: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E2C8E4: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E2C8E8: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 82E2C8EC: 897F0015  lbz r11, 0x15(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E2C8F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2C8F4: 419A0048  beq cr6, 0x82e2c93c
	if ctx.cr[6].eq {
	pc = 0x82E2C93C; continue 'dispatch;
	}
	// 82E2C8F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2C8FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2C900: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 82E2C904: 4B498FC5  bl 0x822c58c8
	ctx.lr = 0x82E2C908;
	sub_822C58C8(ctx, base);
	// 82E2C908: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2C90C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E2C910: 4B49D5A1  bl 0x822c9eb0
	ctx.lr = 0x82E2C914;
	sub_822C9EB0(ctx, base);
	// 82E2C914: 4B49799D  bl 0x822c42b0
	ctx.lr = 0x82E2C918;
	sub_822C42B0(ctx, base);
	// 82E2C918: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2C91C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E2C920: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 82E2C924: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E2C928: 4B498B49  bl 0x822c5470
	ctx.lr = 0x82E2C92C;
	sub_822C5470(ctx, base);
	// 82E2C92C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E2C930: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E2C934: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2C938: 4B4983A9  bl 0x822c4ce0
	ctx.lr = 0x82E2C93C;
	sub_822C4CE0(ctx, base);
	// 82E2C93C: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 82E2C940: 7FFAFB78  mr r26, r31
	ctx.r[26].u64 = ctx.r[31].u64;
	// 82E2C944: 4BFCF315  bl 0x82dfbc58
	ctx.lr = 0x82E2C948;
	sub_82DFBC58(ctx, base);
	// 82E2C948: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2C94C: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E2C950: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E2C954: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82E2C958: 419A000C  beq cr6, 0x82e2c964
	if ctx.cr[6].eq {
	pc = 0x82E2C964; continue 'dispatch;
	}
	// 82E2C95C: 839A0008  lwz r28, 8(r26)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2C960: 48000028  b 0x82e2c988
	pc = 0x82E2C988; continue 'dispatch;
	// 82E2C964: 815A0008  lwz r10, 8(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2C968: 894A0015  lbz r10, 0x15(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E2C96C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E2C970: 419A000C  beq cr6, 0x82e2c97c
	if ctx.cr[6].eq {
	pc = 0x82E2C97C; continue 'dispatch;
	}
	// 82E2C974: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82E2C978: 48000010  b 0x82e2c988
	pc = 0x82E2C988; continue 'dispatch;
	// 82E2C97C: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2C980: 7F19D040  cmplw cr6, r25, r26
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E2C984: 409A00DC  bne cr6, 0x82e2ca60
	if !ctx.cr[6].eq {
	pc = 0x82E2CA60; continue 'dispatch;
	}
	// 82E2C988: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E2C98C: 83FA0004  lwz r31, 4(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2C990: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2C994: 409A0008  bne cr6, 0x82e2c99c
	if !ctx.cr[6].eq {
	pc = 0x82E2C99C; continue 'dispatch;
	}
	// 82E2C998: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E2C99C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2C9A0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2C9A4: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E2C9A8: 409A000C  bne cr6, 0x82e2c9b4
	if !ctx.cr[6].eq {
	pc = 0x82E2C9B4; continue 'dispatch;
	}
	// 82E2C9AC: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E2C9B0: 4800001C  b 0x82e2c9cc
	pc = 0x82E2C9CC; continue 'dispatch;
	// 82E2C9B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2C9B8: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E2C9BC: 409A000C  bne cr6, 0x82e2c9c8
	if !ctx.cr[6].eq {
	pc = 0x82E2C9C8; continue 'dispatch;
	}
	// 82E2C9C0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E2C9C4: 48000008  b 0x82e2c9cc
	pc = 0x82E2C9CC; continue 'dispatch;
	// 82E2C9C8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E2C9CC: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2C9D0: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2C9D4: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E2C9D8: 409A003C  bne cr6, 0x82e2ca14
	if !ctx.cr[6].eq {
	pc = 0x82E2CA14; continue 'dispatch;
	}
	// 82E2C9DC: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E2C9E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2C9E4: 419A000C  beq cr6, 0x82e2c9f0
	if ctx.cr[6].eq {
	pc = 0x82E2C9F0; continue 'dispatch;
	}
	// 82E2C9E8: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E2C9EC: 48000024  b 0x82e2ca10
	pc = 0x82E2CA10; continue 'dispatch;
	// 82E2C9F0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2C9F4: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82E2C9F8: 4800000C  b 0x82e2ca04
	pc = 0x82E2CA04; continue 'dispatch;
	// 82E2C9FC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E2CA00: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2CA04: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E2CA08: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E2CA0C: 419AFFF0  beq cr6, 0x82e2c9fc
	if ctx.cr[6].eq {
	pc = 0x82E2C9FC; continue 'dispatch;
	}
	// 82E2CA10: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2CA14: 813B0004  lwz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CA18: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CA1C: 7F0BD040  cmplw cr6, r11, r26
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E2CA20: 409A00D4  bne cr6, 0x82e2caf4
	if !ctx.cr[6].eq {
	pc = 0x82E2CAF4; continue 'dispatch;
	}
	// 82E2CA24: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E2CA28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2CA2C: 419A000C  beq cr6, 0x82e2ca38
	if ctx.cr[6].eq {
	pc = 0x82E2CA38; continue 'dispatch;
	}
	// 82E2CA30: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E2CA34: 48000024  b 0x82e2ca58
	pc = 0x82E2CA58; continue 'dispatch;
	// 82E2CA38: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CA3C: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82E2CA40: 4800000C  b 0x82e2ca4c
	pc = 0x82E2CA4C; continue 'dispatch;
	// 82E2CA44: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E2CA48: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CA4C: 890B0015  lbz r8, 0x15(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E2CA50: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E2CA54: 419AFFF0  beq cr6, 0x82e2ca44
	if ctx.cr[6].eq {
	pc = 0x82E2CA44; continue 'dispatch;
	}
	// 82E2CA58: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E2CA5C: 48000098  b 0x82e2caf4
	pc = 0x82E2CAF4; continue 'dispatch;
	// 82E2CA60: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E2CA64: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2CA68: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2CA6C: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CA70: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E2CA74: 409A000C  bne cr6, 0x82e2ca80
	if !ctx.cr[6].eq {
	pc = 0x82E2CA80; continue 'dispatch;
	}
	// 82E2CA78: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E2CA7C: 4800002C  b 0x82e2caa8
	pc = 0x82E2CAA8; continue 'dispatch;
	// 82E2CA80: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E2CA84: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CA88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2CA8C: 409A0008  bne cr6, 0x82e2ca94
	if !ctx.cr[6].eq {
	pc = 0x82E2CA94; continue 'dispatch;
	}
	// 82E2CA90: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E2CA94: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E2CA98: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CA9C: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2CAA0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CAA4: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E2CAA8: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CAAC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CAB0: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E2CAB4: 409A000C  bne cr6, 0x82e2cac0
	if !ctx.cr[6].eq {
	pc = 0x82E2CAC0; continue 'dispatch;
	}
	// 82E2CAB8: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E2CABC: 48000020  b 0x82e2cadc
	pc = 0x82E2CADC; continue 'dispatch;
	// 82E2CAC0: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CAC4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2CAC8: 7F0AD040  cmplw cr6, r10, r26
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82E2CACC: 409A000C  bne cr6, 0x82e2cad8
	if !ctx.cr[6].eq {
	pc = 0x82E2CAD8; continue 'dispatch;
	}
	// 82E2CAD0: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82E2CAD4: 48000008  b 0x82e2cadc
	pc = 0x82E2CADC; continue 'dispatch;
	// 82E2CAD8: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82E2CADC: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CAE0: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2CAE4: 897A0014  lbz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2CAE8: 89590014  lbz r10, 0x14(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2CAEC: 99790014  stb r11, 0x14(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(20 as u32), ctx.r[11].u8 ) };
	// 82E2CAF0: 995A0014  stb r10, 0x14(r26)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[26].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 82E2CAF4: 897A0014  lbz r11, 0x14(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2CAF8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E2CAFC: 409A0198  bne cr6, 0x82e2cc94
	if !ctx.cr[6].eq {
	pc = 0x82E2CC94; continue 'dispatch;
	}
	// 82E2CB00: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CB04: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82E2CB08: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CB0C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E2CB10: 419A0180  beq cr6, 0x82e2cc90
	if ctx.cr[6].eq {
	pc = 0x82E2CC90; continue 'dispatch;
	}
	// 82E2CB14: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E2CB18: 897C0014  lbz r11, 0x14(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2CB1C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E2CB20: 409A0170  bne cr6, 0x82e2cc90
	if !ctx.cr[6].eq {
	pc = 0x82E2CC90; continue 'dispatch;
	}
	// 82E2CB24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2CB28: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E2CB2C: 409A00A8  bne cr6, 0x82e2cbd4
	if !ctx.cr[6].eq {
	pc = 0x82E2CBD4; continue 'dispatch;
	}
	// 82E2CB30: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CB34: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2CB38: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E2CB3C: 409A001C  bne cr6, 0x82e2cb58
	if !ctx.cr[6].eq {
	pc = 0x82E2CB58; continue 'dispatch;
	}
	// 82E2CB40: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82E2CB44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E2CB48: 9BBF0014  stb r29, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82E2CB4C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E2CB50: 4BFC8E61  bl 0x82df59b0
	ctx.lr = 0x82E2CB54;
	sub_82DF59B0(ctx, base);
	// 82E2CB54: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CB58: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E2CB5C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E2CB60: 409A00C8  bne cr6, 0x82e2cc28
	if !ctx.cr[6].eq {
	pc = 0x82E2CC28; continue 'dispatch;
	}
	// 82E2CB64: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2CB68: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2CB6C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E2CB70: 409A0014  bne cr6, 0x82e2cb84
	if !ctx.cr[6].eq {
	pc = 0x82E2CB84; continue 'dispatch;
	}
	// 82E2CB74: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CB78: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2CB7C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E2CB80: 419A00A4  beq cr6, 0x82e2cc24
	if ctx.cr[6].eq {
	pc = 0x82E2CC24; continue 'dispatch;
	}
	// 82E2CB84: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CB88: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2CB8C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E2CB90: 409A0020  bne cr6, 0x82e2cbb0
	if !ctx.cr[6].eq {
	pc = 0x82E2CBB0; continue 'dispatch;
	}
	// 82E2CB94: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2CB98: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E2CB9C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E2CBA0: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82E2CBA4: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82E2CBA8: 48033ED9  bl 0x82e60a80
	ctx.lr = 0x82E2CBAC;
	sub_82E60A80(ctx, base);
	// 82E2CBAC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CBB0: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2CBB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E2CBB8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E2CBBC: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 82E2CBC0: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82E2CBC4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CBC8: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82E2CBCC: 4BFC8DE5  bl 0x82df59b0
	ctx.lr = 0x82E2CBD0;
	sub_82DF59B0(ctx, base);
	// 82E2CBD0: 480000C0  b 0x82e2cc90
	pc = 0x82E2CC90; continue 'dispatch;
	// 82E2CBD4: 894B0014  lbz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2CBD8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E2CBDC: 409A001C  bne cr6, 0x82e2cbf8
	if !ctx.cr[6].eq {
	pc = 0x82E2CBF8; continue 'dispatch;
	}
	// 82E2CBE0: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82E2CBE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E2CBE8: 9BBF0014  stb r29, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82E2CBEC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E2CBF0: 48033E91  bl 0x82e60a80
	ctx.lr = 0x82E2CBF4;
	sub_82E60A80(ctx, base);
	// 82E2CBF4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2CBF8: 894B0015  lbz r10, 0x15(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(21 as u32) ) } as u64;
	// 82E2CBFC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E2CC00: 409A0028  bne cr6, 0x82e2cc28
	if !ctx.cr[6].eq {
	pc = 0x82E2CC28; continue 'dispatch;
	}
	// 82E2CC04: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CC08: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2CC0C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E2CC10: 409A0034  bne cr6, 0x82e2cc44
	if !ctx.cr[6].eq {
	pc = 0x82E2CC44; continue 'dispatch;
	}
	// 82E2CC14: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2CC18: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2CC1C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E2CC20: 409A0024  bne cr6, 0x82e2cc44
	if !ctx.cr[6].eq {
	pc = 0x82E2CC44; continue 'dispatch;
	}
	// 82E2CC24: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82E2CC28: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CC2C: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82E2CC30: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CC34: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CC38: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E2CC3C: 409AFEDC  bne cr6, 0x82e2cb18
	if !ctx.cr[6].eq {
	pc = 0x82E2CB18; continue 'dispatch;
	}
	// 82E2CC40: 48000050  b 0x82e2cc90
	pc = 0x82E2CC90; continue 'dispatch;
	// 82E2CC44: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2CC48: 894A0014  lbz r10, 0x14(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2CC4C: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E2CC50: 409A0020  bne cr6, 0x82e2cc70
	if !ctx.cr[6].eq {
	pc = 0x82E2CC70; continue 'dispatch;
	}
	// 82E2CC54: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CC58: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E2CC5C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E2CC60: 9BCA0014  stb r30, 0x14(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82E2CC64: 9BAB0014  stb r29, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[29].u8 ) };
	// 82E2CC68: 4BFC8D49  bl 0x82df59b0
	ctx.lr = 0x82E2CC6C;
	sub_82DF59B0(ctx, base);
	// 82E2CC6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2CC70: 895F0014  lbz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2CC74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E2CC78: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82E2CC7C: 994B0014  stb r10, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[10].u8 ) };
	// 82E2CC80: 9BDF0014  stb r30, 0x14(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82E2CC84: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2CC88: 9BCB0014  stb r30, 0x14(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82E2CC8C: 48033DF5  bl 0x82e60a80
	ctx.lr = 0x82E2CC90;
	sub_82E60A80(ctx, base);
	// 82E2CC90: 9BDC0014  stb r30, 0x14(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[30].u8 ) };
	// 82E2CC94: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E2CC98: 4B4935D1  bl 0x822c0268
	ctx.lr = 0x82E2CC9C;
	sub_822C0268(ctx, base);
	// 82E2CC9C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CCA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2CCA4: 419A000C  beq cr6, 0x82e2ccb0
	if ctx.cr[6].eq {
	pc = 0x82E2CCB0; continue 'dispatch;
	}
	// 82E2CCA8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E2CCAC: 917B0008  stw r11, 8(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2CCB0: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82E2CCB4: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E2CCB8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E2CCBC: 4837B4EC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2CCC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2CCC0 size=96
    let mut pc: u32 = 0x82E2CCC0;
    'dispatch: loop {
        match pc {
            0x82E2CCC0 => {
    //   block [0x82E2CCC0..0x82E2CD20)
	// 82E2CCC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2CCC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2CCC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2CCCC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2CCD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2CCD4: 4BFD1AC5  bl 0x82dfe798
	ctx.lr = 0x82E2CCD8;
	sub_82DFE798(ctx, base);
	// 82E2CCD8: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82E2CCDC: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E2CCE0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2CCE4: 390BC09C  addi r8, r11, -0x3f64
	ctx.r[8].s64 = ctx.r[11].s64 + -16228;
	// 82E2CCE8: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82E2CCEC: 99490000  stb r10, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82E2CCF0: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E2CCF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E2CCF8: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82E2CCFC: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E2CD00: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E2CD04: 4BFFFA35  bl 0x82e2c738
	ctx.lr = 0x82E2CD08;
	sub_82E2C738(ctx, base);
	// 82E2CD08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2CD0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2CD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2CD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2CD18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2CD1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2CD20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2CD20 size=76
    let mut pc: u32 = 0x82E2CD20;
    'dispatch: loop {
        match pc {
            0x82E2CD20 => {
    //   block [0x82E2CD20..0x82E2CD6C)
	// 82E2CD20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2CD24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2CD28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2CD2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2CD30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2CD34: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2CD38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2CD3C: 419A0008  beq cr6, 0x82e2cd44
	if ctx.cr[6].eq {
	pc = 0x82E2CD44; continue 'dispatch;
	}
	// 82E2CD40: 4B493B51  bl 0x822c0890
	ctx.lr = 0x82E2CD44;
	sub_822C0890(ctx, base);
	// 82E2CD44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2CD48: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E2CD4C: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82E2CD50: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2CD54: 4BFC66D5  bl 0x82df3428
	ctx.lr = 0x82E2CD58;
	sub_82DF3428(ctx, base);
	// 82E2CD58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2CD5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2CD60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2CD64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2CD68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2CD70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2CD70 size=76
    let mut pc: u32 = 0x82E2CD70;
    'dispatch: loop {
        match pc {
            0x82E2CD70 => {
    //   block [0x82E2CD70..0x82E2CDBC)
	// 82E2CD70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2CD74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2CD78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2CD7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2CD80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2CD84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2CD88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2CD8C: 4BFFFF95  bl 0x82e2cd20
	ctx.lr = 0x82E2CD90;
	sub_82E2CD20(ctx, base);
	// 82E2CD90: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2CD94: 4182000C  beq 0x82e2cda0
	if ctx.cr[0].eq {
	pc = 0x82E2CDA0; continue 'dispatch;
	}
	// 82E2CD98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2CD9C: 4BFC563D  bl 0x82df23d8
	ctx.lr = 0x82E2CDA0;
	sub_82DF23D8(ctx, base);
	// 82E2CDA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2CDA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2CDA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2CDAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2CDB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2CDB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2CDB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2CDC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2CDC0 size=88
    let mut pc: u32 = 0x82E2CDC0;
    'dispatch: loop {
        match pc {
            0x82E2CDC0 => {
    //   block [0x82E2CDC0..0x82E2CE18)
	// 82E2CDC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2CDC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2CDC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2CDCC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2CDD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2CDD4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CDD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2CDDC: 419A0018  beq cr6, 0x82e2cdf4
	if ctx.cr[6].eq {
	pc = 0x82E2CDF4; continue 'dispatch;
	}
	// 82E2CDE0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E2CDE4: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2CDE8: 48022209  bl 0x82e4eff0
	ctx.lr = 0x82E2CDEC;
	sub_82E4EFF0(ctx, base);
	// 82E2CDEC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CDF0: 4B493479  bl 0x822c0268
	ctx.lr = 0x82E2CDF4;
	sub_822C0268(ctx, base);
	// 82E2CDF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2CDF8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2CDFC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2CE00: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E2CE04: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2CE08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2CE0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2CE10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2CE14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2CE18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2CE18 size=132
    let mut pc: u32 = 0x82E2CE18;
    'dispatch: loop {
        match pc {
            0x82E2CE18 => {
    //   block [0x82E2CE18..0x82E2CE9C)
	// 82E2CE18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2CE1C: 4837B34D  bl 0x831a8168
	ctx.lr = 0x82E2CE20;
	sub_831A8130(ctx, base);
	// 82E2CE20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2CE24: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E2CE28: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82E2CE2C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E2CE30: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2CE34: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CE38: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2CE3C: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E2CE40: 409A0044  bne cr6, 0x82e2ce84
	if !ctx.cr[6].eq {
	pc = 0x82E2CE84; continue 'dispatch;
	}
	// 82E2CE44: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E2CE48: 409A003C  bne cr6, 0x82e2ce84
	if !ctx.cr[6].eq {
	pc = 0x82E2CE84; continue 'dispatch;
	}
	// 82E2CE4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2CE50: 4BFFEBB1  bl 0x82e2ba00
	ctx.lr = 0x82E2CE54;
	sub_82E2BA00(ctx, base);
	// 82E2CE54: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CE58: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2CE5C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2CE60: 48000030  b 0x82e2ce90
	pc = 0x82E2CE90; continue 'dispatch;
	// 82E2CE64: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82E2CE68: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E2CE6C: 4BFCEDED  bl 0x82dfbc58
	ctx.lr = 0x82E2CE70;
	sub_82DFBC58(ctx, base);
	// 82E2CE70: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E2CE74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E2CE78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2CE7C: 4BFFFA55  bl 0x82e2c8d0
	ctx.lr = 0x82E2CE80;
	sub_82E2C8D0(ctx, base);
	// 82E2CE80: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82E2CE84: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E2CE88: 409AFFDC  bne cr6, 0x82e2ce64
	if !ctx.cr[6].eq {
	pc = 0x82E2CE64; continue 'dispatch;
	}
	// 82E2CE8C: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82E2CE90: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E2CE94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2CE98: 4837B320  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2CEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2CEA0 size=112
    let mut pc: u32 = 0x82E2CEA0;
    'dispatch: loop {
        match pc {
            0x82E2CEA0 => {
    //   block [0x82E2CEA0..0x82E2CF10)
	// 82E2CEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2CEA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2CEA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2CEAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2CEB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2CEB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2CEB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2CEBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2CEC0: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2CEC4: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2CEC8: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82E2CECC: 4BFC551D  bl 0x82df23e8
	ctx.lr = 0x82E2CED0;
	sub_82DF23E8(ctx, base);
	// 82E2CED0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2CED4: 41820010  beq 0x82e2cee4
	if ctx.cr[0].eq {
	pc = 0x82E2CEE4; continue 'dispatch;
	}
	// 82E2CED8: 4BFFFDE9  bl 0x82e2ccc0
	ctx.lr = 0x82E2CEDC;
	sub_82E2CCC0(ctx, base);
	// 82E2CEDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2CEE0: 48000008  b 0x82e2cee8
	pc = 0x82E2CEE8; continue 'dispatch;
	// 82E2CEE4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2CEE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2CEEC: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2CEF0: 4BFD1989  bl 0x82dfe878
	ctx.lr = 0x82E2CEF4;
	sub_82DFE878(ctx, base);
	// 82E2CEF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2CEF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2CEFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2CF00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2CF04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2CF08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2CF0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2CF10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2CF10 size=80
    let mut pc: u32 = 0x82E2CF10;
    'dispatch: loop {
        match pc {
            0x82E2CF10 => {
    //   block [0x82E2CF10..0x82E2CF60)
	// 82E2CF10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2CF14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2CF18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2CF1C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2CF20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2CF24: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2CF28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E2CF2C: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CF30: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2CF34: 4BFFFEE5  bl 0x82e2ce18
	ctx.lr = 0x82E2CF38;
	sub_82E2CE18(ctx, base);
	// 82E2CF38: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CF3C: 4B49332D  bl 0x822c0268
	ctx.lr = 0x82E2CF40;
	sub_822C0268(ctx, base);
	// 82E2CF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2CF44: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2CF48: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2CF4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2CF50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2CF54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2CF58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2CF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2CF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2CF60 size=164
    let mut pc: u32 = 0x82E2CF60;
    'dispatch: loop {
        match pc {
            0x82E2CF60 => {
    //   block [0x82E2CF60..0x82E2D004)
	// 82E2CF60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2CF64: 4837B209  bl 0x831a816c
	ctx.lr = 0x82E2CF68;
	sub_831A8130(ctx, base);
	// 82E2CF68: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2CF6C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2CF70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2CF74: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2CF78: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2CF7C: 396BCEA0  addi r11, r11, -0x3160
	ctx.r[11].s64 = ctx.r[11].s64 + -12640;
	// 82E2CF80: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2CF84: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2CF88: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2CF8C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2CF90: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2CF94: 4BFD1965  bl 0x82dfe8f8
	ctx.lr = 0x82E2CF98;
	sub_82DFE8F8(ctx, base);
	// 82E2CF98: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2CF9C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2CFA0: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2CFA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2CFA8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2CFAC: 4BFD48ED  bl 0x82e01898
	ctx.lr = 0x82E2CFB0;
	sub_82E01898(ctx, base);
	// 82E2CFB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2CFB4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2CFB8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2CFBC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2CFC0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2CFC4: 419A0024  beq cr6, 0x82e2cfe8
	if ctx.cr[6].eq {
	pc = 0x82E2CFE8; continue 'dispatch;
	}
	// 82E2CFC8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2CFCC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2CFD0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2CFD4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2CFD8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2CFDC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2CFE0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2CFE4: 4082FFE8  bne 0x82e2cfcc
	if !ctx.cr[0].eq {
	pc = 0x82E2CFCC; continue 'dispatch;
	}
	// 82E2CFE8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2CFEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2CFF0: 419A0008  beq cr6, 0x82e2cff8
	if ctx.cr[6].eq {
	pc = 0x82E2CFF8; continue 'dispatch;
	}
	// 82E2CFF4: 4B49389D  bl 0x822c0890
	ctx.lr = 0x82E2CFF8;
	sub_822C0890(ctx, base);
	// 82E2CFF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2CFFC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2D000: 4837B1BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2D008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2D008 size=124
    let mut pc: u32 = 0x82E2D008;
    'dispatch: loop {
        match pc {
            0x82E2D008 => {
    //   block [0x82E2D008..0x82E2D084)
	// 82E2D008: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2D00C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2D010: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2D014: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2D018: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2D01C: 4BFD177D  bl 0x82dfe798
	ctx.lr = 0x82E2D020;
	sub_82DFE798(ctx, base);
	// 82E2D020: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E2D024: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2D028: 394AC0A4  addi r10, r10, -0x3f5c
	ctx.r[10].s64 = ctx.r[10].s64 + -16220;
	// 82E2D02C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2D030: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2D034: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E2D038: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E2D03C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E2D040: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E2D044: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E2D048: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E2D04C: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82E2D050: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E2D054: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E2D058: 917F0040  stw r11, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82E2D05C: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82E2D060: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82E2D064: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2D068: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E2D06C: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82E2D070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2D074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2D078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2D07C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2D080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2D088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2D088 size=84
    let mut pc: u32 = 0x82E2D088;
    'dispatch: loop {
        match pc {
            0x82E2D088 => {
    //   block [0x82E2D088..0x82E2D0DC)
	// 82E2D088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2D08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2D090: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2D094: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2D098: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2D09C: 4BFD16FD  bl 0x82dfe798
	ctx.lr = 0x82E2D0A0;
	sub_82DFE798(ctx, base);
	// 82E2D0A0: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E2D0A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2D0A8: 394AC0AC  addi r10, r10, -0x3f54
	ctx.r[10].s64 = ctx.r[10].s64 + -16212;
	// 82E2D0AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2D0B0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2D0B4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E2D0B8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E2D0BC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82E2D0C0: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E2D0C4: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E2D0C8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2D0CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2D0D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2D0D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2D0D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2D0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2D0E0 size=76
    let mut pc: u32 = 0x82E2D0E0;
    'dispatch: loop {
        match pc {
            0x82E2D0E0 => {
    //   block [0x82E2D0E0..0x82E2D12C)
	// 82E2D0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2D0E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2D0E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2D0EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2D0F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2D0F4: 4BFD16A5  bl 0x82dfe798
	ctx.lr = 0x82E2D0F8;
	sub_82DFE798(ctx, base);
	// 82E2D0F8: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E2D0FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2D100: 394AC0B4  addi r10, r10, -0x3f4c
	ctx.r[10].s64 = ctx.r[10].s64 + -16204;
	// 82E2D104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2D108: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2D10C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E2D110: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E2D114: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E2D118: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2D11C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2D120: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2D124: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2D128: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2D130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2D130 size=76
    let mut pc: u32 = 0x82E2D130;
    'dispatch: loop {
        match pc {
            0x82E2D130 => {
    //   block [0x82E2D130..0x82E2D17C)
	// 82E2D130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2D134: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2D138: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2D13C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2D140: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2D144: 4BFD1655  bl 0x82dfe798
	ctx.lr = 0x82E2D148;
	sub_82DFE798(ctx, base);
	// 82E2D148: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E2D14C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2D150: 394AC0BC  addi r10, r10, -0x3f44
	ctx.r[10].s64 = ctx.r[10].s64 + -16196;
	// 82E2D154: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2D158: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2D15C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E2D160: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E2D164: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E2D168: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2D16C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2D170: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2D174: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2D178: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2D180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2D180 size=84
    let mut pc: u32 = 0x82E2D180;
    'dispatch: loop {
        match pc {
            0x82E2D180 => {
    //   block [0x82E2D180..0x82E2D1D4)
	// 82E2D180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2D184: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2D188: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2D18C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2D190: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2D194: 4BFD1605  bl 0x82dfe798
	ctx.lr = 0x82E2D198;
	sub_82DFE798(ctx, base);
	// 82E2D198: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E2D19C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2D1A0: 394AC0C4  addi r10, r10, -0x3f3c
	ctx.r[10].s64 = ctx.r[10].s64 + -16188;
	// 82E2D1A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2D1A8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2D1AC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E2D1B0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E2D1B4: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82E2D1B8: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82E2D1BC: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E2D1C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2D1C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2D1C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2D1CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2D1D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2D1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2D1D8 size=156
    let mut pc: u32 = 0x82E2D1D8;
    'dispatch: loop {
        match pc {
            0x82E2D1D8 => {
    //   block [0x82E2D1D8..0x82E2D274)
	// 82E2D1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2D1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2D1E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2D1E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2D1E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2D1EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2D1F0: 4BFD15A9  bl 0x82dfe798
	ctx.lr = 0x82E2D1F4;
	sub_82DFE798(ctx, base);
	// 82E2D1F4: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E2D1F8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82E2D1FC: 396BC0CC  addi r11, r11, -0x3f34
	ctx.r[11].s64 = ctx.r[11].s64 + -16180;
	// 82E2D200: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E2D204: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2D208: 397F0024  addi r11, r31, 0x24
	ctx.r[11].s64 = ctx.r[31].s64 + 36;
	// 82E2D20C: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 82E2D210: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E2D214: 9BCA0000  stb r30, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82E2D218: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82E2D21C: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E2D220: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 82E2D224: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E2D228: 4BFFE131  bl 0x82e2b358
	ctx.lr = 0x82E2D22C;
	sub_82E2B358(ctx, base);
	// 82E2D22C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E2D230: 9BCB0000  stb r30, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82E2D234: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 82E2D238: 397F002C  addi r11, r31, 0x2c
	ctx.r[11].s64 = ctx.r[31].s64 + 44;
	// 82E2D23C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E2D240: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E2D244: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E2D248: 4B5EEC01  bl 0x8241be48
	ctx.lr = 0x82E2D24C;
	sub_8241BE48(ctx, base);
	// 82E2D24C: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 82E2D250: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 82E2D254: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2D258: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 82E2D25C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2D260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2D264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2D268: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2D26C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2D270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2D278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2D278 size=152
    let mut pc: u32 = 0x82E2D278;
    'dispatch: loop {
        match pc {
            0x82E2D278 => {
    //   block [0x82E2D278..0x82E2D310)
	// 82E2D278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2D27C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2D280: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2D284: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2D288: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2D28C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2D290: 3BFE0034  addi r31, r30, 0x34
	ctx.r[31].s64 = ctx.r[30].s64 + 52;
	// 82E2D294: 807E0038  lwz r3, 0x38(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82E2D298: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2D29C: 419A0008  beq cr6, 0x82e2d2a4
	if ctx.cr[6].eq {
	pc = 0x82E2D2A4; continue 'dispatch;
	}
	// 82E2D2A0: 4B492FC9  bl 0x822c0268
	ctx.lr = 0x82E2D2A4;
	sub_822C0268(ctx, base);
	// 82E2D2A4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2D2A8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2D2AC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2D2B0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E2D2B4: 807E0030  lwz r3, 0x30(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82E2D2B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2D2BC: 419A0008  beq cr6, 0x82e2d2c4
	if ctx.cr[6].eq {
	pc = 0x82E2D2C4; continue 'dispatch;
	}
	// 82E2D2C0: 4B4935D1  bl 0x822c0890
	ctx.lr = 0x82E2D2C4;
	sub_822C0890(ctx, base);
	// 82E2D2C4: 807E0028  lwz r3, 0x28(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82E2D2C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2D2CC: 419A0008  beq cr6, 0x82e2d2d4
	if ctx.cr[6].eq {
	pc = 0x82E2D2D4; continue 'dispatch;
	}
	// 82E2D2D0: 4B4935C1  bl 0x822c0890
	ctx.lr = 0x82E2D2D4;
	sub_822C0890(ctx, base);
	// 82E2D2D4: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E2D2D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2D2DC: 419A0008  beq cr6, 0x82e2d2e4
	if ctx.cr[6].eq {
	pc = 0x82E2D2E4; continue 'dispatch;
	}
	// 82E2D2E0: 4B4935B1  bl 0x822c0890
	ctx.lr = 0x82E2D2E4;
	sub_822C0890(ctx, base);
	// 82E2D2E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2D2E8: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 82E2D2EC: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82E2D2F0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2D2F4: 4BFC6135  bl 0x82df3428
	ctx.lr = 0x82E2D2F8;
	sub_82DF3428(ctx, base);
	// 82E2D2F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2D2FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2D300: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2D304: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2D308: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2D30C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2D310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2D310 size=76
    let mut pc: u32 = 0x82E2D310;
    'dispatch: loop {
        match pc {
            0x82E2D310 => {
    //   block [0x82E2D310..0x82E2D35C)
	// 82E2D310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2D314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2D318: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2D31C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2D320: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2D324: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2D328: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2D32C: 4BFFFF4D  bl 0x82e2d278
	ctx.lr = 0x82E2D330;
	sub_82E2D278(ctx, base);
	// 82E2D330: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2D334: 4182000C  beq 0x82e2d340
	if ctx.cr[0].eq {
	pc = 0x82E2D340; continue 'dispatch;
	}
	// 82E2D338: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2D33C: 4BFC509D  bl 0x82df23d8
	ctx.lr = 0x82E2D340;
	sub_82DF23D8(ctx, base);
	// 82E2D340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2D344: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2D348: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2D34C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2D350: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2D354: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2D358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2D360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2D360 size=96
    let mut pc: u32 = 0x82E2D360;
    'dispatch: loop {
        match pc {
            0x82E2D360 => {
    //   block [0x82E2D360..0x82E2D3C0)
	// 82E2D360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2D364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2D368: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2D36C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2D370: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2D374: 4BFD1425  bl 0x82dfe798
	ctx.lr = 0x82E2D378;
	sub_82DFE798(ctx, base);
	// 82E2D378: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E2D37C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2D380: 394AC0D4  addi r10, r10, -0x3f2c
	ctx.r[10].s64 = ctx.r[10].s64 + -16172;
	// 82E2D384: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2D388: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2D38C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E2D390: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82E2D394: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E2D398: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E2D39C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82E2D3A0: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E2D3A4: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E2D3A8: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E2D3AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2D3B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2D3B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2D3B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2D3BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2D3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2D3C0 size=128
    let mut pc: u32 = 0x82E2D3C0;
    'dispatch: loop {
        match pc {
            0x82E2D3C0 => {
    //   block [0x82E2D3C0..0x82E2D440)
	// 82E2D3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2D3C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2D3C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2D3CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2D3D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2D3D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2D3D8: 3BFE0030  addi r31, r30, 0x30
	ctx.r[31].s64 = ctx.r[30].s64 + 48;
	// 82E2D3DC: 807E0034  lwz r3, 0x34(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 82E2D3E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2D3E4: 419A0008  beq cr6, 0x82e2d3ec
	if ctx.cr[6].eq {
	pc = 0x82E2D3EC; continue 'dispatch;
	}
	// 82E2D3E8: 4B492E81  bl 0x822c0268
	ctx.lr = 0x82E2D3EC;
	sub_822C0268(ctx, base);
	// 82E2D3EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2D3F0: 387E0020  addi r3, r30, 0x20
	ctx.r[3].s64 = ctx.r[30].s64 + 32;
	// 82E2D3F4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2D3F8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2D3FC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E2D400: 4BFEE989  bl 0x82e1bd88
	ctx.lr = 0x82E2D404;
	sub_82E1BD88(ctx, base);
	// 82E2D404: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E2D408: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2D40C: 419A0008  beq cr6, 0x82e2d414
	if ctx.cr[6].eq {
	pc = 0x82E2D414; continue 'dispatch;
	}
	// 82E2D410: 4B493481  bl 0x822c0890
	ctx.lr = 0x82E2D414;
	sub_822C0890(ctx, base);
	// 82E2D414: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2D418: 387E0008  addi r3, r30, 8
	ctx.r[3].s64 = ctx.r[30].s64 + 8;
	// 82E2D41C: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82E2D420: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2D424: 4BFC6005  bl 0x82df3428
	ctx.lr = 0x82E2D428;
	sub_82DF3428(ctx, base);
	// 82E2D428: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2D42C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2D430: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2D434: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2D438: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2D43C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2D440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2D440 size=76
    let mut pc: u32 = 0x82E2D440;
    'dispatch: loop {
        match pc {
            0x82E2D440 => {
    //   block [0x82E2D440..0x82E2D48C)
	// 82E2D440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2D444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2D448: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2D44C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2D450: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2D454: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2D458: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2D45C: 4BFFFF65  bl 0x82e2d3c0
	ctx.lr = 0x82E2D460;
	sub_82E2D3C0(ctx, base);
	// 82E2D460: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2D464: 4182000C  beq 0x82e2d470
	if ctx.cr[0].eq {
	pc = 0x82E2D470; continue 'dispatch;
	}
	// 82E2D468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2D46C: 4BFC4F6D  bl 0x82df23d8
	ctx.lr = 0x82E2D470;
	sub_82DF23D8(ctx, base);
	// 82E2D470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2D474: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2D478: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2D47C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2D480: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2D484: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2D488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2D490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2D490 size=136
    let mut pc: u32 = 0x82E2D490;
    'dispatch: loop {
        match pc {
            0x82E2D490 => {
    //   block [0x82E2D490..0x82E2D518)
	// 82E2D490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2D494: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2D498: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2D49C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2D4A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2D4A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2D4A8: 3BDF004C  addi r30, r31, 0x4c
	ctx.r[30].s64 = ctx.r[31].s64 + 76;
	// 82E2D4AC: 807F0050  lwz r3, 0x50(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E2D4B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2D4B4: 419A0008  beq cr6, 0x82e2d4bc
	if ctx.cr[6].eq {
	pc = 0x82E2D4BC; continue 'dispatch;
	}
	// 82E2D4B8: 4B492DB1  bl 0x822c0268
	ctx.lr = 0x82E2D4BC;
	sub_822C0268(ctx, base);
	// 82E2D4BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2D4C0: 387F003C  addi r3, r31, 0x3c
	ctx.r[3].s64 = ctx.r[31].s64 + 60;
	// 82E2D4C4: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2D4C8: 917E0008  stw r11, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2D4CC: 917E000C  stw r11, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E2D4D0: 4BFFF8F1  bl 0x82e2cdc0
	ctx.lr = 0x82E2D4D4;
	sub_82E2CDC0(ctx, base);
	// 82E2D4D4: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 82E2D4D8: 4BFFF8E9  bl 0x82e2cdc0
	ctx.lr = 0x82E2D4DC;
	sub_82E2CDC0(ctx, base);
	// 82E2D4DC: 387F001C  addi r3, r31, 0x1c
	ctx.r[3].s64 = ctx.r[31].s64 + 28;
	// 82E2D4E0: 4BFEE8A9  bl 0x82e1bd88
	ctx.lr = 0x82E2D4E4;
	sub_82E1BD88(ctx, base);
	// 82E2D4E4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E2D4E8: 4BFEE8A1  bl 0x82e1bd88
	ctx.lr = 0x82E2D4EC;
	sub_82E1BD88(ctx, base);
	// 82E2D4EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2D4F0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E2D4F4: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82E2D4F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2D4FC: 4BFC5F2D  bl 0x82df3428
	ctx.lr = 0x82E2D500;
	sub_82DF3428(ctx, base);
	// 82E2D500: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2D504: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2D508: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2D50C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2D510: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2D514: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2D518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2D518 size=84
    let mut pc: u32 = 0x82E2D518;
    'dispatch: loop {
        match pc {
            0x82E2D518 => {
    //   block [0x82E2D518..0x82E2D56C)
	// 82E2D518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2D51C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2D520: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2D524: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2D528: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2D52C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82E2D530: 4BFFF891  bl 0x82e2cdc0
	ctx.lr = 0x82E2D534;
	sub_82E2CDC0(ctx, base);
	// 82E2D534: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E2D538: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2D53C: 419A0008  beq cr6, 0x82e2d544
	if ctx.cr[6].eq {
	pc = 0x82E2D544; continue 'dispatch;
	}
	// 82E2D540: 4B493351  bl 0x822c0890
	ctx.lr = 0x82E2D544;
	sub_822C0890(ctx, base);
	// 82E2D544: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2D548: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E2D54C: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82E2D550: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2D554: 4BFC5ED5  bl 0x82df3428
	ctx.lr = 0x82E2D558;
	sub_82DF3428(ctx, base);
	// 82E2D558: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2D55C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2D560: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2D564: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2D568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2D570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2D570 size=1036
    let mut pc: u32 = 0x82E2D570;
    'dispatch: loop {
        match pc {
            0x82E2D570 => {
    //   block [0x82E2D570..0x82E2D97C)
	// 82E2D570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2D574: 4837ABE5  bl 0x831a8158
	ctx.lr = 0x82E2D578;
	sub_831A8130(ctx, base);
	// 82E2D578: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2D57C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82E2D580: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82E2D584: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82E2D588: 93E10104  stw r31, 0x104(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(260 as u32), ctx.r[31].u32 ) };
	// 82E2D58C: 897F0021  lbz r11, 0x21(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(33 as u32) ) } as u64;
	// 82E2D590: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2D594: 419A0048  beq cr6, 0x82e2d5dc
	if ctx.cr[6].eq {
	pc = 0x82E2D5DC; continue 'dispatch;
	}
	// 82E2D598: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2D59C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2D5A0: 388B9620  addi r4, r11, -0x69e0
	ctx.r[4].s64 = ctx.r[11].s64 + -27104;
	// 82E2D5A4: 4B498325  bl 0x822c58c8
	ctx.lr = 0x82E2D5A8;
	sub_822C58C8(ctx, base);
	// 82E2D5A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2D5AC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E2D5B0: 4B49C901  bl 0x822c9eb0
	ctx.lr = 0x82E2D5B4;
	sub_822C9EB0(ctx, base);
	// 82E2D5B4: 4B496CFD  bl 0x822c42b0
	ctx.lr = 0x82E2D5B8;
	sub_822C42B0(ctx, base);
	// 82E2D5B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2D5BC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E2D5C0: 396B9600  addi r11, r11, -0x6a00
	ctx.r[11].s64 = ctx.r[11].s64 + -27136;
	// 82E2D5C4: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E2D5C8: 4B497EA9  bl 0x822c5470
	ctx.lr = 0x82E2D5CC;
	sub_822C5470(ctx, base);
	// 82E2D5CC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E2D5D0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E2D5D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2D5D8: 4B497709  bl 0x822c4ce0
	ctx.lr = 0x82E2D5DC;
	sub_822C4CE0(ctx, base);
	// 82E2D5DC: 38610104  addi r3, r1, 0x104
	ctx.r[3].s64 = ctx.r[1].s64 + 260;
	// 82E2D5E0: 7FFBFB78  mr r27, r31
	ctx.r[27].u64 = ctx.r[31].u64;
	// 82E2D5E4: 4BFD13ED  bl 0x82dfe9d0
	ctx.lr = 0x82E2D5E8;
	sub_82DFE9D0(ctx, base);
	// 82E2D5E8: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D5EC: 894B0021  lbz r10, 0x21(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 82E2D5F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E2D5F4: 83210104  lwz r25, 0x104(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(260 as u32) ) } as u64;
	// 82E2D5F8: 419A000C  beq cr6, 0x82e2d604
	if ctx.cr[6].eq {
	pc = 0x82E2D604; continue 'dispatch;
	}
	// 82E2D5FC: 839B0008  lwz r28, 8(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D600: 48000028  b 0x82e2d628
	pc = 0x82E2D628; continue 'dispatch;
	// 82E2D604: 815B0008  lwz r10, 8(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D608: 894A0021  lbz r10, 0x21(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(33 as u32) ) } as u64;
	// 82E2D60C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E2D610: 419A000C  beq cr6, 0x82e2d61c
	if ctx.cr[6].eq {
	pc = 0x82E2D61C; continue 'dispatch;
	}
	// 82E2D614: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 82E2D618: 48000010  b 0x82e2d628
	pc = 0x82E2D628; continue 'dispatch;
	// 82E2D61C: 83990008  lwz r28, 8(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D620: 7F19D840  cmplw cr6, r25, r27
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E2D624: 409A00DC  bne cr6, 0x82e2d700
	if !ctx.cr[6].eq {
	pc = 0x82E2D700; continue 'dispatch;
	}
	// 82E2D628: 897C0021  lbz r11, 0x21(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(33 as u32) ) } as u64;
	// 82E2D62C: 83FB0004  lwz r31, 4(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2D630: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2D634: 409A0008  bne cr6, 0x82e2d63c
	if !ctx.cr[6].eq {
	pc = 0x82E2D63C; continue 'dispatch;
	}
	// 82E2D638: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E2D63C: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2D640: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2D644: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E2D648: 409A000C  bne cr6, 0x82e2d654
	if !ctx.cr[6].eq {
	pc = 0x82E2D654; continue 'dispatch;
	}
	// 82E2D64C: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E2D650: 4800001C  b 0x82e2d66c
	pc = 0x82E2D66C; continue 'dispatch;
	// 82E2D654: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D658: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E2D65C: 409A000C  bne cr6, 0x82e2d668
	if !ctx.cr[6].eq {
	pc = 0x82E2D668; continue 'dispatch;
	}
	// 82E2D660: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E2D664: 48000008  b 0x82e2d66c
	pc = 0x82E2D66C; continue 'dispatch;
	// 82E2D668: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E2D66C: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2D670: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D674: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E2D678: 409A003C  bne cr6, 0x82e2d6b4
	if !ctx.cr[6].eq {
	pc = 0x82E2D6B4; continue 'dispatch;
	}
	// 82E2D67C: 897C0021  lbz r11, 0x21(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(33 as u32) ) } as u64;
	// 82E2D680: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2D684: 419A000C  beq cr6, 0x82e2d690
	if ctx.cr[6].eq {
	pc = 0x82E2D690; continue 'dispatch;
	}
	// 82E2D688: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E2D68C: 48000024  b 0x82e2d6b0
	pc = 0x82E2D6B0; continue 'dispatch;
	// 82E2D690: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D694: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82E2D698: 4800000C  b 0x82e2d6a4
	pc = 0x82E2D6A4; continue 'dispatch;
	// 82E2D69C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E2D6A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D6A4: 890B0021  lbz r8, 0x21(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 82E2D6A8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E2D6AC: 419AFFF0  beq cr6, 0x82e2d69c
	if ctx.cr[6].eq {
	pc = 0x82E2D69C; continue 'dispatch;
	}
	// 82E2D6B0: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2D6B4: 813A0004  lwz r9, 4(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2D6B8: 81690008  lwz r11, 8(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D6BC: 7F0BD840  cmplw cr6, r11, r27
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E2D6C0: 409A00D4  bne cr6, 0x82e2d794
	if !ctx.cr[6].eq {
	pc = 0x82E2D794; continue 'dispatch;
	}
	// 82E2D6C4: 897C0021  lbz r11, 0x21(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(33 as u32) ) } as u64;
	// 82E2D6C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2D6CC: 419A000C  beq cr6, 0x82e2d6d8
	if ctx.cr[6].eq {
	pc = 0x82E2D6D8; continue 'dispatch;
	}
	// 82E2D6D0: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82E2D6D4: 48000024  b 0x82e2d6f8
	pc = 0x82E2D6F8; continue 'dispatch;
	// 82E2D6D8: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D6DC: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 82E2D6E0: 4800000C  b 0x82e2d6ec
	pc = 0x82E2D6EC; continue 'dispatch;
	// 82E2D6E4: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82E2D6E8: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D6EC: 890B0021  lbz r8, 0x21(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 82E2D6F0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82E2D6F4: 419AFFF0  beq cr6, 0x82e2d6e4
	if ctx.cr[6].eq {
	pc = 0x82E2D6E4; continue 'dispatch;
	}
	// 82E2D6F8: 91490008  stw r10, 8(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E2D6FC: 48000098  b 0x82e2d794
	pc = 0x82E2D794; continue 'dispatch;
	// 82E2D700: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E2D704: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D708: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2D70C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D710: 7F195840  cmplw cr6, r25, r11
	ctx.cr[6].compare_u32(ctx.r[25].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E2D714: 409A000C  bne cr6, 0x82e2d720
	if !ctx.cr[6].eq {
	pc = 0x82E2D720; continue 'dispatch;
	}
	// 82E2D718: 7F3FCB78  mr r31, r25
	ctx.r[31].u64 = ctx.r[25].u64;
	// 82E2D71C: 4800002C  b 0x82e2d748
	pc = 0x82E2D748; continue 'dispatch;
	// 82E2D720: 897C0021  lbz r11, 0x21(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(33 as u32) ) } as u64;
	// 82E2D724: 83F90004  lwz r31, 4(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2D728: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2D72C: 409A0008  bne cr6, 0x82e2d734
	if !ctx.cr[6].eq {
	pc = 0x82E2D734; continue 'dispatch;
	}
	// 82E2D730: 93FC0004  stw r31, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[31].u32 ) };
	// 82E2D734: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E2D738: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D73C: 91790008  stw r11, 8(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2D740: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D744: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E2D748: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2D74C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2D750: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E2D754: 409A000C  bne cr6, 0x82e2d760
	if !ctx.cr[6].eq {
	pc = 0x82E2D760; continue 'dispatch;
	}
	// 82E2D758: 932B0004  stw r25, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 82E2D75C: 48000020  b 0x82e2d77c
	pc = 0x82E2D77C; continue 'dispatch;
	// 82E2D760: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2D764: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D768: 7F0AD840  cmplw cr6, r10, r27
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[27].u32, &mut ctx.xer);
	// 82E2D76C: 409A000C  bne cr6, 0x82e2d778
	if !ctx.cr[6].eq {
	pc = 0x82E2D778; continue 'dispatch;
	}
	// 82E2D770: 932B0000  stw r25, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82E2D774: 48000008  b 0x82e2d77c
	pc = 0x82E2D77C; continue 'dispatch;
	// 82E2D778: 932B0008  stw r25, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82E2D77C: 817B0004  lwz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2D780: 91790004  stw r11, 4(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2D784: 897B0020  lbz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E2D788: 89590020  lbz r10, 0x20(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E2D78C: 99790020  stb r11, 0x20(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 82E2D790: 995B0020  stb r10, 0x20(r27)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[27].u32.wrapping_add(32 as u32), ctx.r[10].u8 ) };
	// 82E2D794: 897B0020  lbz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E2D798: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E2D79C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E2D7A0: 409A0188  bne cr6, 0x82e2d928
	if !ctx.cr[6].eq {
	pc = 0x82E2D928; continue 'dispatch;
	}
	// 82E2D7A4: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82E2D7A8: 4800011C  b 0x82e2d8c4
	pc = 0x82E2D8C4; continue 'dispatch;
	// 82E2D7AC: 897C0020  lbz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E2D7B0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E2D7B4: 409A0170  bne cr6, 0x82e2d924
	if !ctx.cr[6].eq {
	pc = 0x82E2D924; continue 'dispatch;
	}
	// 82E2D7B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D7BC: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E2D7C0: 409A00A8  bne cr6, 0x82e2d868
	if !ctx.cr[6].eq {
	pc = 0x82E2D868; continue 'dispatch;
	}
	// 82E2D7C4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D7C8: 894B0020  lbz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E2D7CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E2D7D0: 409A001C  bne cr6, 0x82e2d7ec
	if !ctx.cr[6].eq {
	pc = 0x82E2D7EC; continue 'dispatch;
	}
	// 82E2D7D4: 9BCB0020  stb r30, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 82E2D7D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E2D7DC: 9BBF0020  stb r29, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 82E2D7E0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E2D7E4: 4BFFD86D  bl 0x82e2b050
	ctx.lr = 0x82E2D7E8;
	sub_82E2B050(ctx, base);
	// 82E2D7E8: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D7EC: 894B0021  lbz r10, 0x21(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 82E2D7F0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E2D7F4: 409A00C8  bne cr6, 0x82e2d8bc
	if !ctx.cr[6].eq {
	pc = 0x82E2D8BC; continue 'dispatch;
	}
	// 82E2D7F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D7FC: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E2D800: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E2D804: 409A0014  bne cr6, 0x82e2d818
	if !ctx.cr[6].eq {
	pc = 0x82E2D818; continue 'dispatch;
	}
	// 82E2D808: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D80C: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E2D810: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E2D814: 419A00A4  beq cr6, 0x82e2d8b8
	if ctx.cr[6].eq {
	pc = 0x82E2D8B8; continue 'dispatch;
	}
	// 82E2D818: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D81C: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E2D820: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E2D824: 409A0020  bne cr6, 0x82e2d844
	if !ctx.cr[6].eq {
	pc = 0x82E2D844; continue 'dispatch;
	}
	// 82E2D828: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D82C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E2D830: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E2D834: 9BCA0020  stb r30, 0x20(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 82E2D838: 9BAB0020  stb r29, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 82E2D83C: 4BFFD375  bl 0x82e2abb0
	ctx.lr = 0x82E2D840;
	sub_82E2ABB0(ctx, base);
	// 82E2D840: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D844: 895F0020  lbz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E2D848: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E2D84C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E2D850: 994B0020  stb r10, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u8 ) };
	// 82E2D854: 9BDF0020  stb r30, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 82E2D858: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D85C: 9BCB0020  stb r30, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 82E2D860: 4BFFD7F1  bl 0x82e2b050
	ctx.lr = 0x82E2D864;
	sub_82E2B050(ctx, base);
	// 82E2D864: 480000C0  b 0x82e2d924
	pc = 0x82E2D924; continue 'dispatch;
	// 82E2D868: 894B0020  lbz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E2D86C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E2D870: 409A001C  bne cr6, 0x82e2d88c
	if !ctx.cr[6].eq {
	pc = 0x82E2D88C; continue 'dispatch;
	}
	// 82E2D874: 9BCB0020  stb r30, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 82E2D878: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E2D87C: 9BBF0020  stb r29, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 82E2D880: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E2D884: 4BFFD32D  bl 0x82e2abb0
	ctx.lr = 0x82E2D888;
	sub_82E2ABB0(ctx, base);
	// 82E2D888: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D88C: 894B0021  lbz r10, 0x21(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(33 as u32) ) } as u64;
	// 82E2D890: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82E2D894: 409A0028  bne cr6, 0x82e2d8bc
	if !ctx.cr[6].eq {
	pc = 0x82E2D8BC; continue 'dispatch;
	}
	// 82E2D898: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D89C: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E2D8A0: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E2D8A4: 409A0034  bne cr6, 0x82e2d8d8
	if !ctx.cr[6].eq {
	pc = 0x82E2D8D8; continue 'dispatch;
	}
	// 82E2D8A8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D8AC: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E2D8B0: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E2D8B4: 409A0024  bne cr6, 0x82e2d8d8
	if !ctx.cr[6].eq {
	pc = 0x82E2D8D8; continue 'dispatch;
	}
	// 82E2D8B8: 9BAB0020  stb r29, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 82E2D8BC: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 82E2D8C0: 83FF0004  lwz r31, 4(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2D8C4: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2D8C8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2D8CC: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E2D8D0: 409AFEDC  bne cr6, 0x82e2d7ac
	if !ctx.cr[6].eq {
	pc = 0x82E2D7AC; continue 'dispatch;
	}
	// 82E2D8D4: 48000050  b 0x82e2d924
	pc = 0x82E2D924; continue 'dispatch;
	// 82E2D8D8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D8DC: 894A0020  lbz r10, 0x20(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E2D8E0: 2B0A0001  cmplwi cr6, r10, 1
	ctx.cr[6].compare_u32(ctx.r[10].u32, 1 as u32, &mut ctx.xer);
	// 82E2D8E4: 409A0020  bne cr6, 0x82e2d904
	if !ctx.cr[6].eq {
	pc = 0x82E2D904; continue 'dispatch;
	}
	// 82E2D8E8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D8EC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82E2D8F0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E2D8F4: 9BCA0020  stb r30, 0x20(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 82E2D8F8: 9BAB0020  stb r29, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[29].u8 ) };
	// 82E2D8FC: 4BFFD755  bl 0x82e2b050
	ctx.lr = 0x82E2D900;
	sub_82E2B050(ctx, base);
	// 82E2D900: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D904: 895F0020  lbz r10, 0x20(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82E2D908: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E2D90C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E2D910: 994B0020  stb r10, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u8 ) };
	// 82E2D914: 9BDF0020  stb r30, 0x20(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 82E2D918: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D91C: 9BCB0020  stb r30, 0x20(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 82E2D920: 4BFFD291  bl 0x82e2abb0
	ctx.lr = 0x82E2D924;
	sub_82E2ABB0(ctx, base);
	// 82E2D924: 9BDC0020  stb r30, 0x20(r28)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[28].u32.wrapping_add(32 as u32), ctx.r[30].u8 ) };
	// 82E2D928: 807B0014  lwz r3, 0x14(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2D92C: 3BFB0010  addi r31, r27, 0x10
	ctx.r[31].s64 = ctx.r[27].s64 + 16;
	// 82E2D930: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2D934: 419A0008  beq cr6, 0x82e2d93c
	if ctx.cr[6].eq {
	pc = 0x82E2D93C; continue 'dispatch;
	}
	// 82E2D938: 4B492931  bl 0x822c0268
	ctx.lr = 0x82E2D93C;
	sub_822C0268(ctx, base);
	// 82E2D93C: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 82E2D940: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E2D944: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82E2D948: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82E2D94C: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 82E2D950: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E2D954: 4BFC4835  bl 0x82df2188
	ctx.lr = 0x82E2D958;
	sub_82DF2188(ctx, base);
	// 82E2D958: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D95C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2D960: 419A000C  beq cr6, 0x82e2d96c
	if ctx.cr[6].eq {
	pc = 0x82E2D96C; continue 'dispatch;
	}
	// 82E2D964: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82E2D968: 917A0008  stw r11, 8(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2D96C: 93380000  stw r25, 0(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 82E2D970: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 82E2D974: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E2D978: 4837A830  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2D980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2D980 size=128
    let mut pc: u32 = 0x82E2D980;
    'dispatch: loop {
        match pc {
            0x82E2D980 => {
    //   block [0x82E2D980..0x82E2DA00)
	// 82E2D980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2D984: 4837A7DD  bl 0x831a8160
	ctx.lr = 0x82E2D988;
	sub_831A8130(ctx, base);
	// 82E2D988: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2D98C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2D990: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82E2D994: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 82E2D998: 897D0021  lbz r11, 0x21(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(33 as u32) ) } as u64;
	// 82E2D99C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2D9A0: 409A0058  bne cr6, 0x82e2d9f8
	if !ctx.cr[6].eq {
	pc = 0x82E2D9F8; continue 'dispatch;
	}
	// 82E2D9A4: 3F608335  lis r27, -0x7ccb
	ctx.r[27].s64 = -2093678592;
	// 82E2D9A8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E2D9AC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82E2D9B0: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E2D9B4: 4BFFFFCD  bl 0x82e2d980
	ctx.lr = 0x82E2D9B8;
	sub_82E2D980(ctx, base);
	// 82E2D9B8: 807D0014  lwz r3, 0x14(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E2D9BC: 3BFD0010  addi r31, r29, 0x10
	ctx.r[31].s64 = ctx.r[29].s64 + 16;
	// 82E2D9C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2D9C4: 83DE0000  lwz r30, 0(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2D9C8: 419A0008  beq cr6, 0x82e2d9d0
	if ctx.cr[6].eq {
	pc = 0x82E2D9D0; continue 'dispatch;
	}
	// 82E2D9CC: 4B49289D  bl 0x822c0268
	ctx.lr = 0x82E2D9D0;
	sub_822C0268(ctx, base);
	// 82E2D9D0: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82E2D9D4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2D9D8: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82E2D9DC: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 82E2D9E0: 807B110C  lwz r3, 0x110c(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E2D9E4: 4BFC47A5  bl 0x82df2188
	ctx.lr = 0x82E2D9E8;
	sub_82DF2188(ctx, base);
	// 82E2D9E8: 897E0021  lbz r11, 0x21(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(33 as u32) ) } as u64;
	// 82E2D9EC: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82E2D9F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2D9F4: 419AFFB8  beq cr6, 0x82e2d9ac
	if ctx.cr[6].eq {
	pc = 0x82E2D9AC; continue 'dispatch;
	}
	// 82E2D9F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2D9FC: 4837A7B4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2DA00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2DA00 size=108
    let mut pc: u32 = 0x82E2DA00;
    'dispatch: loop {
        match pc {
            0x82E2DA00 => {
    //   block [0x82E2DA00..0x82E2DA6C)
	// 82E2DA00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2DA04: 4837A765  bl 0x831a8168
	ctx.lr = 0x82E2DA08;
	sub_831A8130(ctx, base);
	// 82E2DA08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2DA0C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2DA10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2DA14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2DA18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2DA1C: 388BA7CC  addi r4, r11, -0x5834
	ctx.r[4].s64 = ctx.r[11].s64 + -22580;
	// 82E2DA20: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2DA24: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2DA28: 4BFC5FE1  bl 0x82df3a08
	ctx.lr = 0x82E2DA2C;
	sub_82DF3A08(ctx, base);
	// 82E2DA2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2DA30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2DA34: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2DA38: 48001939  bl 0x82e2f370
	ctx.lr = 0x82E2DA3C;
	sub_82E2F370(ctx, base);
	// 82E2DA3C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2DA40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DA44: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2DA48: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2DA4C: 4BFFF515  bl 0x82e2cf60
	ctx.lr = 0x82E2DA50;
	sub_82E2CF60(ctx, base);
	// 82E2DA50: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2DA54: 4BFC59D5  bl 0x82df3428
	ctx.lr = 0x82E2DA58;
	sub_82DF3428(ctx, base);
	// 82E2DA58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2DA5C: 4BFC59CD  bl 0x82df3428
	ctx.lr = 0x82E2DA60;
	sub_82DF3428(ctx, base);
	// 82E2DA60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DA64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2DA68: 4837A750  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2DA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2DA70 size=112
    let mut pc: u32 = 0x82E2DA70;
    'dispatch: loop {
        match pc {
            0x82E2DA70 => {
    //   block [0x82E2DA70..0x82E2DAE0)
	// 82E2DA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2DA74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2DA78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2DA7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2DA80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2DA84: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2DA88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2DA8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2DA90: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2DA94: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2DA98: 3860005C  li r3, 0x5c
	ctx.r[3].s64 = 92;
	// 82E2DA9C: 4BFC494D  bl 0x82df23e8
	ctx.lr = 0x82E2DAA0;
	sub_82DF23E8(ctx, base);
	// 82E2DAA0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2DAA4: 41820010  beq 0x82e2dab4
	if ctx.cr[0].eq {
	pc = 0x82E2DAB4; continue 'dispatch;
	}
	// 82E2DAA8: 4BFFF561  bl 0x82e2d008
	ctx.lr = 0x82E2DAAC;
	sub_82E2D008(ctx, base);
	// 82E2DAAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2DAB0: 48000008  b 0x82e2dab8
	pc = 0x82E2DAB8; continue 'dispatch;
	// 82E2DAB4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2DAB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DABC: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2DAC0: 4BFD0DB9  bl 0x82dfe878
	ctx.lr = 0x82E2DAC4;
	sub_82DFE878(ctx, base);
	// 82E2DAC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DAC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2DACC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2DAD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2DAD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2DAD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2DADC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2DAE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2DAE0 size=112
    let mut pc: u32 = 0x82E2DAE0;
    'dispatch: loop {
        match pc {
            0x82E2DAE0 => {
    //   block [0x82E2DAE0..0x82E2DB50)
	// 82E2DAE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2DAE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2DAE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2DAEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2DAF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2DAF4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2DAF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2DAFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2DB00: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2DB04: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2DB08: 38600050  li r3, 0x50
	ctx.r[3].s64 = 80;
	// 82E2DB0C: 4BFC48DD  bl 0x82df23e8
	ctx.lr = 0x82E2DB10;
	sub_82DF23E8(ctx, base);
	// 82E2DB10: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2DB14: 41820010  beq 0x82e2db24
	if ctx.cr[0].eq {
	pc = 0x82E2DB24; continue 'dispatch;
	}
	// 82E2DB18: 4B7CF459  bl 0x825fcf70
	ctx.lr = 0x82E2DB1C;
	sub_825FCF70(ctx, base);
	// 82E2DB1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2DB20: 48000008  b 0x82e2db28
	pc = 0x82E2DB28; continue 'dispatch;
	// 82E2DB24: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2DB28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DB2C: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2DB30: 4BFD0D49  bl 0x82dfe878
	ctx.lr = 0x82E2DB34;
	sub_82DFE878(ctx, base);
	// 82E2DB34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DB38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2DB3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2DB40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2DB44: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2DB48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2DB4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2DB50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2DB50 size=112
    let mut pc: u32 = 0x82E2DB50;
    'dispatch: loop {
        match pc {
            0x82E2DB50 => {
    //   block [0x82E2DB50..0x82E2DBC0)
	// 82E2DB50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2DB54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2DB58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2DB5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2DB60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2DB64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2DB68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2DB6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2DB70: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2DB74: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2DB78: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82E2DB7C: 4BFC486D  bl 0x82df23e8
	ctx.lr = 0x82E2DB80;
	sub_82DF23E8(ctx, base);
	// 82E2DB80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2DB84: 41820010  beq 0x82e2db94
	if ctx.cr[0].eq {
	pc = 0x82E2DB94; continue 'dispatch;
	}
	// 82E2DB88: 4BFFF501  bl 0x82e2d088
	ctx.lr = 0x82E2DB8C;
	sub_82E2D088(ctx, base);
	// 82E2DB8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2DB90: 48000008  b 0x82e2db98
	pc = 0x82E2DB98; continue 'dispatch;
	// 82E2DB94: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2DB98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DB9C: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2DBA0: 4BFD0CD9  bl 0x82dfe878
	ctx.lr = 0x82E2DBA4;
	sub_82DFE878(ctx, base);
	// 82E2DBA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DBA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2DBAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2DBB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2DBB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2DBB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2DBBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2DBC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2DBC0 size=112
    let mut pc: u32 = 0x82E2DBC0;
    'dispatch: loop {
        match pc {
            0x82E2DBC0 => {
    //   block [0x82E2DBC0..0x82E2DC30)
	// 82E2DBC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2DBC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2DBC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2DBCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2DBD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2DBD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2DBD8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2DBDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2DBE0: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2DBE4: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2DBE8: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82E2DBEC: 4BFC47FD  bl 0x82df23e8
	ctx.lr = 0x82E2DBF0;
	sub_82DF23E8(ctx, base);
	// 82E2DBF0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2DBF4: 41820010  beq 0x82e2dc04
	if ctx.cr[0].eq {
	pc = 0x82E2DC04; continue 'dispatch;
	}
	// 82E2DBF8: 4BFFF4E9  bl 0x82e2d0e0
	ctx.lr = 0x82E2DBFC;
	sub_82E2D0E0(ctx, base);
	// 82E2DBFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2DC00: 48000008  b 0x82e2dc08
	pc = 0x82E2DC08; continue 'dispatch;
	// 82E2DC04: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2DC08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DC0C: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2DC10: 4BFD0C69  bl 0x82dfe878
	ctx.lr = 0x82E2DC14;
	sub_82DFE878(ctx, base);
	// 82E2DC14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DC18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2DC1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2DC20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2DC24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2DC28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2DC2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2DC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2DC30 size=112
    let mut pc: u32 = 0x82E2DC30;
    'dispatch: loop {
        match pc {
            0x82E2DC30 => {
    //   block [0x82E2DC30..0x82E2DCA0)
	// 82E2DC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2DC34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2DC38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2DC3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2DC40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2DC44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2DC48: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2DC4C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2DC50: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2DC54: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2DC58: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82E2DC5C: 4BFC478D  bl 0x82df23e8
	ctx.lr = 0x82E2DC60;
	sub_82DF23E8(ctx, base);
	// 82E2DC60: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2DC64: 41820010  beq 0x82e2dc74
	if ctx.cr[0].eq {
	pc = 0x82E2DC74; continue 'dispatch;
	}
	// 82E2DC68: 4BFFF4C9  bl 0x82e2d130
	ctx.lr = 0x82E2DC6C;
	sub_82E2D130(ctx, base);
	// 82E2DC6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2DC70: 48000008  b 0x82e2dc78
	pc = 0x82E2DC78; continue 'dispatch;
	// 82E2DC74: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2DC78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DC7C: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2DC80: 4BFD0BF9  bl 0x82dfe878
	ctx.lr = 0x82E2DC84;
	sub_82DFE878(ctx, base);
	// 82E2DC84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DC88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2DC8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2DC90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2DC94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2DC98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2DC9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2DCA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2DCA0 size=112
    let mut pc: u32 = 0x82E2DCA0;
    'dispatch: loop {
        match pc {
            0x82E2DCA0 => {
    //   block [0x82E2DCA0..0x82E2DD10)
	// 82E2DCA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2DCA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2DCA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2DCAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2DCB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2DCB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2DCB8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2DCBC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2DCC0: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2DCC4: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2DCC8: 38600028  li r3, 0x28
	ctx.r[3].s64 = 40;
	// 82E2DCCC: 4BFC471D  bl 0x82df23e8
	ctx.lr = 0x82E2DCD0;
	sub_82DF23E8(ctx, base);
	// 82E2DCD0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2DCD4: 41820010  beq 0x82e2dce4
	if ctx.cr[0].eq {
	pc = 0x82E2DCE4; continue 'dispatch;
	}
	// 82E2DCD8: 4BFFF4A9  bl 0x82e2d180
	ctx.lr = 0x82E2DCDC;
	sub_82E2D180(ctx, base);
	// 82E2DCDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2DCE0: 48000008  b 0x82e2dce8
	pc = 0x82E2DCE8; continue 'dispatch;
	// 82E2DCE4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2DCE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DCEC: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2DCF0: 4BFD0B89  bl 0x82dfe878
	ctx.lr = 0x82E2DCF4;
	sub_82DFE878(ctx, base);
	// 82E2DCF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DCF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2DCFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2DD00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2DD04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2DD08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2DD0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2DD10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2DD10 size=112
    let mut pc: u32 = 0x82E2DD10;
    'dispatch: loop {
        match pc {
            0x82E2DD10 => {
    //   block [0x82E2DD10..0x82E2DD80)
	// 82E2DD10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2DD14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2DD18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2DD1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2DD20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2DD24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2DD28: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2DD2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2DD30: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2DD34: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2DD38: 38600044  li r3, 0x44
	ctx.r[3].s64 = 68;
	// 82E2DD3C: 4BFC46AD  bl 0x82df23e8
	ctx.lr = 0x82E2DD40;
	sub_82DF23E8(ctx, base);
	// 82E2DD40: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2DD44: 41820010  beq 0x82e2dd54
	if ctx.cr[0].eq {
	pc = 0x82E2DD54; continue 'dispatch;
	}
	// 82E2DD48: 4BFFF491  bl 0x82e2d1d8
	ctx.lr = 0x82E2DD4C;
	sub_82E2D1D8(ctx, base);
	// 82E2DD4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2DD50: 48000008  b 0x82e2dd58
	pc = 0x82E2DD58; continue 'dispatch;
	// 82E2DD54: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2DD58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DD5C: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2DD60: 4BFD0B19  bl 0x82dfe878
	ctx.lr = 0x82E2DD64;
	sub_82DFE878(ctx, base);
	// 82E2DD64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DD68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2DD6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2DD70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2DD74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2DD78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2DD7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2DD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2DD80 size=112
    let mut pc: u32 = 0x82E2DD80;
    'dispatch: loop {
        match pc {
            0x82E2DD80 => {
    //   block [0x82E2DD80..0x82E2DDF0)
	// 82E2DD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2DD84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2DD88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2DD8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2DD90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2DD94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2DD98: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2DD9C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2DDA0: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2DDA4: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2DDA8: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 82E2DDAC: 4BFC463D  bl 0x82df23e8
	ctx.lr = 0x82E2DDB0;
	sub_82DF23E8(ctx, base);
	// 82E2DDB0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2DDB4: 41820010  beq 0x82e2ddc4
	if ctx.cr[0].eq {
	pc = 0x82E2DDC4; continue 'dispatch;
	}
	// 82E2DDB8: 4BFFF5A9  bl 0x82e2d360
	ctx.lr = 0x82E2DDBC;
	sub_82E2D360(ctx, base);
	// 82E2DDBC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2DDC0: 48000008  b 0x82e2ddc8
	pc = 0x82E2DDC8; continue 'dispatch;
	// 82E2DDC4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2DDC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DDCC: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2DDD0: 4BFD0AA9  bl 0x82dfe878
	ctx.lr = 0x82E2DDD4;
	sub_82DFE878(ctx, base);
	// 82E2DDD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DDD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2DDDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2DDE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2DDE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2DDE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2DDEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2DDF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2DDF0 size=120
    let mut pc: u32 = 0x82E2DDF0;
    'dispatch: loop {
        match pc {
            0x82E2DDF0 => {
    //   block [0x82E2DDF0..0x82E2DE68)
	// 82E2DDF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2DDF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2DDF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2DDFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2DE00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2DE04: 4BFD0995  bl 0x82dfe798
	ctx.lr = 0x82E2DE08;
	sub_82DFE798(ctx, base);
	// 82E2DE08: 3D408212  lis r10, -0x7dee
	ctx.r[10].s64 = -2112749568;
	// 82E2DE0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2DE10: 394AC0DC  addi r10, r10, -0x3f24
	ctx.r[10].s64 = ctx.r[10].s64 + -16164;
	// 82E2DE14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DE18: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2DE1C: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82E2DE20: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E2DE24: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82E2DE28: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E2DE2C: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82E2DE30: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82E2DE34: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82E2DE38: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 82E2DE3C: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82E2DE40: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82E2DE44: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82E2DE48: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82E2DE4C: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2DE50: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E2DE54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2DE58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2DE5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2DE60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2DE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2DE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2DE68 size=108
    let mut pc: u32 = 0x82E2DE68;
    'dispatch: loop {
        match pc {
            0x82E2DE68 => {
    //   block [0x82E2DE68..0x82E2DED4)
	// 82E2DE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2DE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2DE70: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2DE74: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2DE78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2DE7C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E2DE80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2DE84: 419A0008  beq cr6, 0x82e2de8c
	if ctx.cr[6].eq {
	pc = 0x82E2DE8C; continue 'dispatch;
	}
	// 82E2DE88: 4B492A09  bl 0x822c0890
	ctx.lr = 0x82E2DE8C;
	sub_822C0890(ctx, base);
	// 82E2DE8C: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 82E2DE90: 4B683411  bl 0x824b12a0
	ctx.lr = 0x82E2DE94;
	sub_824B12A0(ctx, base);
	// 82E2DE94: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82E2DE98: 4B683409  bl 0x824b12a0
	ctx.lr = 0x82E2DE9C;
	sub_824B12A0(ctx, base);
	// 82E2DE9C: 387F0020  addi r3, r31, 0x20
	ctx.r[3].s64 = ctx.r[31].s64 + 32;
	// 82E2DEA0: 4B683401  bl 0x824b12a0
	ctx.lr = 0x82E2DEA4;
	sub_824B12A0(ctx, base);
	// 82E2DEA4: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E2DEA8: 4B6833F9  bl 0x824b12a0
	ctx.lr = 0x82E2DEAC;
	sub_824B12A0(ctx, base);
	// 82E2DEAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2DEB0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E2DEB4: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82E2DEB8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2DEBC: 4BFC556D  bl 0x82df3428
	ctx.lr = 0x82E2DEC0;
	sub_82DF3428(ctx, base);
	// 82E2DEC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2DEC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2DEC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2DECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2DED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2DED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2DED8 size=76
    let mut pc: u32 = 0x82E2DED8;
    'dispatch: loop {
        match pc {
            0x82E2DED8 => {
    //   block [0x82E2DED8..0x82E2DF24)
	// 82E2DED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2DEDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2DEE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2DEE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2DEE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2DEEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2DEF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2DEF4: 4BFFF59D  bl 0x82e2d490
	ctx.lr = 0x82E2DEF8;
	sub_82E2D490(ctx, base);
	// 82E2DEF8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2DEFC: 4182000C  beq 0x82e2df08
	if ctx.cr[0].eq {
	pc = 0x82E2DF08; continue 'dispatch;
	}
	// 82E2DF00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DF04: 4BFC44D5  bl 0x82df23d8
	ctx.lr = 0x82E2DF08;
	sub_82DF23D8(ctx, base);
	// 82E2DF08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DF0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2DF10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2DF14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2DF18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2DF1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2DF20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2DF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2DF28 size=100
    let mut pc: u32 = 0x82E2DF28;
    'dispatch: loop {
        match pc {
            0x82E2DF28 => {
    //   block [0x82E2DF28..0x82E2DF8C)
	// 82E2DF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2DF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2DF30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2DF34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2DF38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2DF3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2DF40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2DF44: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E2DF48: 4BFFEE79  bl 0x82e2cdc0
	ctx.lr = 0x82E2DF4C;
	sub_82E2CDC0(ctx, base);
	// 82E2DF4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2DF50: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E2DF54: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82E2DF58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2DF5C: 4BFC54CD  bl 0x82df3428
	ctx.lr = 0x82E2DF60;
	sub_82DF3428(ctx, base);
	// 82E2DF60: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2DF64: 4182000C  beq 0x82e2df70
	if ctx.cr[0].eq {
	pc = 0x82E2DF70; continue 'dispatch;
	}
	// 82E2DF68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DF6C: 4BFC446D  bl 0x82df23d8
	ctx.lr = 0x82E2DF70;
	sub_82DF23D8(ctx, base);
	// 82E2DF70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DF74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2DF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2DF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2DF80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2DF84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2DF88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2DF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2DF90 size=76
    let mut pc: u32 = 0x82E2DF90;
    'dispatch: loop {
        match pc {
            0x82E2DF90 => {
    //   block [0x82E2DF90..0x82E2DFDC)
	// 82E2DF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2DF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2DF98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2DF9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2DFA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2DFA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2DFA8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2DFAC: 4BFFFEBD  bl 0x82e2de68
	ctx.lr = 0x82E2DFB0;
	sub_82E2DE68(ctx, base);
	// 82E2DFB0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2DFB4: 4182000C  beq 0x82e2dfc0
	if ctx.cr[0].eq {
	pc = 0x82E2DFC0; continue 'dispatch;
	}
	// 82E2DFB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DFBC: 4BFC441D  bl 0x82df23d8
	ctx.lr = 0x82E2DFC0;
	sub_82DF23D8(ctx, base);
	// 82E2DFC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2DFC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2DFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2DFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2DFD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2DFD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2DFD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2DFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2DFE0 size=76
    let mut pc: u32 = 0x82E2DFE0;
    'dispatch: loop {
        match pc {
            0x82E2DFE0 => {
    //   block [0x82E2DFE0..0x82E2E02C)
	// 82E2DFE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2DFE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2DFE8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2DFEC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2DFF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2DFF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2DFF8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2DFFC: 4BFFF51D  bl 0x82e2d518
	ctx.lr = 0x82E2E000;
	sub_82E2D518(ctx, base);
	// 82E2E000: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2E004: 4182000C  beq 0x82e2e010
	if ctx.cr[0].eq {
	pc = 0x82E2E010; continue 'dispatch;
	}
	// 82E2E008: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E00C: 4BFC43CD  bl 0x82df23d8
	ctx.lr = 0x82E2E010;
	sub_82DF23D8(ctx, base);
	// 82E2E010: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E014: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2E018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2E01C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2E020: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2E024: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2E028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E030 size=84
    let mut pc: u32 = 0x82E2E030;
    'dispatch: loop {
        match pc {
            0x82E2E030 => {
    //   block [0x82E2E030..0x82E2E084)
	// 82E2E030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2E038: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2E03C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E040: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E044: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E048: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E04C: 4BFFF935  bl 0x82e2d980
	ctx.lr = 0x82E2E050;
	sub_82E2D980(ctx, base);
	// 82E2E050: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E054: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2E058: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E2E05C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2E060: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E064: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E2E068: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E06C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2E070: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2E074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2E078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2E07C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2E080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E088 size=164
    let mut pc: u32 = 0x82E2E088;
    'dispatch: loop {
        match pc {
            0x82E2E088 => {
    //   block [0x82E2E088..0x82E2E12C)
	// 82E2E088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E08C: 4837A0E1  bl 0x831a816c
	ctx.lr = 0x82E2E090;
	sub_831A8130(ctx, base);
	// 82E2E090: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E094: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2E098: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E09C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2E0A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2E0A4: 396BDA70  addi r11, r11, -0x2590
	ctx.r[11].s64 = ctx.r[11].s64 + -9616;
	// 82E2E0A8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2E0AC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2E0B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2E0B4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2E0B8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2E0BC: 4BFD083D  bl 0x82dfe8f8
	ctx.lr = 0x82E2E0C0;
	sub_82DFE8F8(ctx, base);
	// 82E2E0C0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2E0C4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2E0C8: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2E0CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2E0D0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2E0D4: 4BFD37C5  bl 0x82e01898
	ctx.lr = 0x82E2E0D8;
	sub_82E01898(ctx, base);
	// 82E2E0D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2E0DC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2E0E0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E0E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2E0E8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2E0EC: 419A0024  beq cr6, 0x82e2e110
	if ctx.cr[6].eq {
	pc = 0x82E2E110; continue 'dispatch;
	}
	// 82E2E0F0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2E0F4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2E0F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E0FC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2E100: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2E104: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2E108: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E10C: 4082FFE8  bne 0x82e2e0f4
	if !ctx.cr[0].eq {
	pc = 0x82E2E0F4; continue 'dispatch;
	}
	// 82E2E110: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2E114: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2E118: 419A0008  beq cr6, 0x82e2e120
	if ctx.cr[6].eq {
	pc = 0x82E2E120; continue 'dispatch;
	}
	// 82E2E11C: 4B492775  bl 0x822c0890
	ctx.lr = 0x82E2E120;
	sub_822C0890(ctx, base);
	// 82E2E120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E124: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2E128: 4837A094  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E130 size=164
    let mut pc: u32 = 0x82E2E130;
    'dispatch: loop {
        match pc {
            0x82E2E130 => {
    //   block [0x82E2E130..0x82E2E1D4)
	// 82E2E130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E134: 4837A039  bl 0x831a816c
	ctx.lr = 0x82E2E138;
	sub_831A8130(ctx, base);
	// 82E2E138: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E13C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2E140: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E144: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2E148: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2E14C: 396BDAE0  addi r11, r11, -0x2520
	ctx.r[11].s64 = ctx.r[11].s64 + -9504;
	// 82E2E150: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2E154: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2E158: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2E15C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2E160: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2E164: 4BFD0795  bl 0x82dfe8f8
	ctx.lr = 0x82E2E168;
	sub_82DFE8F8(ctx, base);
	// 82E2E168: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2E16C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2E170: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2E174: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2E178: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2E17C: 4BFD371D  bl 0x82e01898
	ctx.lr = 0x82E2E180;
	sub_82E01898(ctx, base);
	// 82E2E180: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2E184: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2E188: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E18C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2E190: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2E194: 419A0024  beq cr6, 0x82e2e1b8
	if ctx.cr[6].eq {
	pc = 0x82E2E1B8; continue 'dispatch;
	}
	// 82E2E198: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2E19C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2E1A0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E1A4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2E1A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2E1AC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2E1B0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E1B4: 4082FFE8  bne 0x82e2e19c
	if !ctx.cr[0].eq {
	pc = 0x82E2E19C; continue 'dispatch;
	}
	// 82E2E1B8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2E1BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2E1C0: 419A0008  beq cr6, 0x82e2e1c8
	if ctx.cr[6].eq {
	pc = 0x82E2E1C8; continue 'dispatch;
	}
	// 82E2E1C4: 4B4926CD  bl 0x822c0890
	ctx.lr = 0x82E2E1C8;
	sub_822C0890(ctx, base);
	// 82E2E1C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E1CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2E1D0: 48379FEC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E1D8 size=164
    let mut pc: u32 = 0x82E2E1D8;
    'dispatch: loop {
        match pc {
            0x82E2E1D8 => {
    //   block [0x82E2E1D8..0x82E2E27C)
	// 82E2E1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E1DC: 48379F91  bl 0x831a816c
	ctx.lr = 0x82E2E1E0;
	sub_831A8130(ctx, base);
	// 82E2E1E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E1E4: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2E1E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E1EC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2E1F0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2E1F4: 396BDB50  addi r11, r11, -0x24b0
	ctx.r[11].s64 = ctx.r[11].s64 + -9392;
	// 82E2E1F8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2E1FC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2E200: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2E204: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2E208: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2E20C: 4BFD06ED  bl 0x82dfe8f8
	ctx.lr = 0x82E2E210;
	sub_82DFE8F8(ctx, base);
	// 82E2E210: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2E214: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2E218: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2E21C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2E220: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2E224: 4BFD3675  bl 0x82e01898
	ctx.lr = 0x82E2E228;
	sub_82E01898(ctx, base);
	// 82E2E228: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2E22C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2E230: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E234: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2E238: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2E23C: 419A0024  beq cr6, 0x82e2e260
	if ctx.cr[6].eq {
	pc = 0x82E2E260; continue 'dispatch;
	}
	// 82E2E240: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2E244: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2E248: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E24C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2E250: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2E254: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2E258: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E25C: 4082FFE8  bne 0x82e2e244
	if !ctx.cr[0].eq {
	pc = 0x82E2E244; continue 'dispatch;
	}
	// 82E2E260: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2E264: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2E268: 419A0008  beq cr6, 0x82e2e270
	if ctx.cr[6].eq {
	pc = 0x82E2E270; continue 'dispatch;
	}
	// 82E2E26C: 4B492625  bl 0x822c0890
	ctx.lr = 0x82E2E270;
	sub_822C0890(ctx, base);
	// 82E2E270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E274: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2E278: 48379F44  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E280 size=164
    let mut pc: u32 = 0x82E2E280;
    'dispatch: loop {
        match pc {
            0x82E2E280 => {
    //   block [0x82E2E280..0x82E2E324)
	// 82E2E280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E284: 48379EE9  bl 0x831a816c
	ctx.lr = 0x82E2E288;
	sub_831A8130(ctx, base);
	// 82E2E288: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E28C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2E290: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E294: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2E298: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2E29C: 396BDBC0  addi r11, r11, -0x2440
	ctx.r[11].s64 = ctx.r[11].s64 + -9280;
	// 82E2E2A0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2E2A4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2E2A8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2E2AC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2E2B0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2E2B4: 4BFD0645  bl 0x82dfe8f8
	ctx.lr = 0x82E2E2B8;
	sub_82DFE8F8(ctx, base);
	// 82E2E2B8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2E2BC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2E2C0: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2E2C4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2E2C8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2E2CC: 4BFD35CD  bl 0x82e01898
	ctx.lr = 0x82E2E2D0;
	sub_82E01898(ctx, base);
	// 82E2E2D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2E2D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2E2D8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E2DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2E2E0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2E2E4: 419A0024  beq cr6, 0x82e2e308
	if ctx.cr[6].eq {
	pc = 0x82E2E308; continue 'dispatch;
	}
	// 82E2E2E8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2E2EC: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2E2F0: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E2F4: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2E2F8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2E2FC: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2E300: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E304: 4082FFE8  bne 0x82e2e2ec
	if !ctx.cr[0].eq {
	pc = 0x82E2E2EC; continue 'dispatch;
	}
	// 82E2E308: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2E30C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2E310: 419A0008  beq cr6, 0x82e2e318
	if ctx.cr[6].eq {
	pc = 0x82E2E318; continue 'dispatch;
	}
	// 82E2E314: 4B49257D  bl 0x822c0890
	ctx.lr = 0x82E2E318;
	sub_822C0890(ctx, base);
	// 82E2E318: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E31C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2E320: 48379E9C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E328 size=164
    let mut pc: u32 = 0x82E2E328;
    'dispatch: loop {
        match pc {
            0x82E2E328 => {
    //   block [0x82E2E328..0x82E2E3CC)
	// 82E2E328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E32C: 48379E41  bl 0x831a816c
	ctx.lr = 0x82E2E330;
	sub_831A8130(ctx, base);
	// 82E2E330: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E334: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2E338: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E33C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2E340: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2E344: 396BDC30  addi r11, r11, -0x23d0
	ctx.r[11].s64 = ctx.r[11].s64 + -9168;
	// 82E2E348: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2E34C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2E350: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2E354: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2E358: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2E35C: 4BFD059D  bl 0x82dfe8f8
	ctx.lr = 0x82E2E360;
	sub_82DFE8F8(ctx, base);
	// 82E2E360: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2E364: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2E368: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2E36C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2E370: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2E374: 4BFD3525  bl 0x82e01898
	ctx.lr = 0x82E2E378;
	sub_82E01898(ctx, base);
	// 82E2E378: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2E37C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2E380: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E384: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2E388: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2E38C: 419A0024  beq cr6, 0x82e2e3b0
	if ctx.cr[6].eq {
	pc = 0x82E2E3B0; continue 'dispatch;
	}
	// 82E2E390: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2E394: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2E398: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E39C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2E3A0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2E3A4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2E3A8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E3AC: 4082FFE8  bne 0x82e2e394
	if !ctx.cr[0].eq {
	pc = 0x82E2E394; continue 'dispatch;
	}
	// 82E2E3B0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2E3B4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2E3B8: 419A0008  beq cr6, 0x82e2e3c0
	if ctx.cr[6].eq {
	pc = 0x82E2E3C0; continue 'dispatch;
	}
	// 82E2E3BC: 4B4924D5  bl 0x822c0890
	ctx.lr = 0x82E2E3C0;
	sub_822C0890(ctx, base);
	// 82E2E3C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E3C4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2E3C8: 48379DF4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E3D0 size=164
    let mut pc: u32 = 0x82E2E3D0;
    'dispatch: loop {
        match pc {
            0x82E2E3D0 => {
    //   block [0x82E2E3D0..0x82E2E474)
	// 82E2E3D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E3D4: 48379D99  bl 0x831a816c
	ctx.lr = 0x82E2E3D8;
	sub_831A8130(ctx, base);
	// 82E2E3D8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E3DC: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2E3E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E3E4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2E3E8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2E3EC: 396BDCA0  addi r11, r11, -0x2360
	ctx.r[11].s64 = ctx.r[11].s64 + -9056;
	// 82E2E3F0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2E3F4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2E3F8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2E3FC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2E400: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2E404: 4BFD04F5  bl 0x82dfe8f8
	ctx.lr = 0x82E2E408;
	sub_82DFE8F8(ctx, base);
	// 82E2E408: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2E40C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2E410: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2E414: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2E418: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2E41C: 4BFD347D  bl 0x82e01898
	ctx.lr = 0x82E2E420;
	sub_82E01898(ctx, base);
	// 82E2E420: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2E424: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2E428: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E42C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2E430: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2E434: 419A0024  beq cr6, 0x82e2e458
	if ctx.cr[6].eq {
	pc = 0x82E2E458; continue 'dispatch;
	}
	// 82E2E438: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2E43C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2E440: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E444: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2E448: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2E44C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2E450: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E454: 4082FFE8  bne 0x82e2e43c
	if !ctx.cr[0].eq {
	pc = 0x82E2E43C; continue 'dispatch;
	}
	// 82E2E458: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2E45C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2E460: 419A0008  beq cr6, 0x82e2e468
	if ctx.cr[6].eq {
	pc = 0x82E2E468; continue 'dispatch;
	}
	// 82E2E464: 4B49242D  bl 0x822c0890
	ctx.lr = 0x82E2E468;
	sub_822C0890(ctx, base);
	// 82E2E468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E46C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2E470: 48379D4C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E478 size=164
    let mut pc: u32 = 0x82E2E478;
    'dispatch: loop {
        match pc {
            0x82E2E478 => {
    //   block [0x82E2E478..0x82E2E51C)
	// 82E2E478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E47C: 48379CF1  bl 0x831a816c
	ctx.lr = 0x82E2E480;
	sub_831A8130(ctx, base);
	// 82E2E480: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E484: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2E488: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E48C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2E490: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2E494: 396BDD10  addi r11, r11, -0x22f0
	ctx.r[11].s64 = ctx.r[11].s64 + -8944;
	// 82E2E498: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2E49C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2E4A0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2E4A4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2E4A8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2E4AC: 4BFD044D  bl 0x82dfe8f8
	ctx.lr = 0x82E2E4B0;
	sub_82DFE8F8(ctx, base);
	// 82E2E4B0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2E4B4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2E4B8: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2E4BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2E4C0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2E4C4: 4BFD33D5  bl 0x82e01898
	ctx.lr = 0x82E2E4C8;
	sub_82E01898(ctx, base);
	// 82E2E4C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2E4CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2E4D0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E4D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2E4D8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2E4DC: 419A0024  beq cr6, 0x82e2e500
	if ctx.cr[6].eq {
	pc = 0x82E2E500; continue 'dispatch;
	}
	// 82E2E4E0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2E4E4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2E4E8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E4EC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2E4F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2E4F4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2E4F8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E4FC: 4082FFE8  bne 0x82e2e4e4
	if !ctx.cr[0].eq {
	pc = 0x82E2E4E4; continue 'dispatch;
	}
	// 82E2E500: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2E504: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2E508: 419A0008  beq cr6, 0x82e2e510
	if ctx.cr[6].eq {
	pc = 0x82E2E510; continue 'dispatch;
	}
	// 82E2E50C: 4B492385  bl 0x822c0890
	ctx.lr = 0x82E2E510;
	sub_822C0890(ctx, base);
	// 82E2E510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E514: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2E518: 48379CA4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E520 size=164
    let mut pc: u32 = 0x82E2E520;
    'dispatch: loop {
        match pc {
            0x82E2E520 => {
    //   block [0x82E2E520..0x82E2E5C4)
	// 82E2E520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E524: 48379C49  bl 0x831a816c
	ctx.lr = 0x82E2E528;
	sub_831A8130(ctx, base);
	// 82E2E528: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E52C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2E530: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E534: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2E538: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2E53C: 396BDD80  addi r11, r11, -0x2280
	ctx.r[11].s64 = ctx.r[11].s64 + -8832;
	// 82E2E540: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2E544: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2E548: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2E54C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2E550: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2E554: 4BFD03A5  bl 0x82dfe8f8
	ctx.lr = 0x82E2E558;
	sub_82DFE8F8(ctx, base);
	// 82E2E558: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2E55C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2E560: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2E564: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2E568: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2E56C: 4BFD332D  bl 0x82e01898
	ctx.lr = 0x82E2E570;
	sub_82E01898(ctx, base);
	// 82E2E570: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2E574: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2E578: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E57C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2E580: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2E584: 419A0024  beq cr6, 0x82e2e5a8
	if ctx.cr[6].eq {
	pc = 0x82E2E5A8; continue 'dispatch;
	}
	// 82E2E588: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2E58C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2E590: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E594: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2E598: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2E59C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2E5A0: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2E5A4: 4082FFE8  bne 0x82e2e58c
	if !ctx.cr[0].eq {
	pc = 0x82E2E58C; continue 'dispatch;
	}
	// 82E2E5A8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2E5AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2E5B0: 419A0008  beq cr6, 0x82e2e5b8
	if ctx.cr[6].eq {
	pc = 0x82E2E5B8; continue 'dispatch;
	}
	// 82E2E5B4: 4B4922DD  bl 0x822c0890
	ctx.lr = 0x82E2E5B8;
	sub_822C0890(ctx, base);
	// 82E2E5B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E5BC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2E5C0: 48379BFC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E5C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E5C8 size=112
    let mut pc: u32 = 0x82E2E5C8;
    'dispatch: loop {
        match pc {
            0x82E2E5C8 => {
    //   block [0x82E2E5C8..0x82E2E638)
	// 82E2E5C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E5CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2E5D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2E5D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2E5D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E5DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2E5E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2E5E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2E5E8: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2E5EC: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2E5F0: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 82E2E5F4: 4BFC3DF5  bl 0x82df23e8
	ctx.lr = 0x82E2E5F8;
	sub_82DF23E8(ctx, base);
	// 82E2E5F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2E5FC: 41820010  beq 0x82e2e60c
	if ctx.cr[0].eq {
	pc = 0x82E2E60C; continue 'dispatch;
	}
	// 82E2E600: 4BFE5A89  bl 0x82e14088
	ctx.lr = 0x82E2E604;
	sub_82E14088(ctx, base);
	// 82E2E604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E608: 48000008  b 0x82e2e610
	pc = 0x82E2E610; continue 'dispatch;
	// 82E2E60C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2E610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E614: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2E618: 4BFD0261  bl 0x82dfe878
	ctx.lr = 0x82E2E61C;
	sub_82DFE878(ctx, base);
	// 82E2E61C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E620: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2E624: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2E628: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2E62C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2E630: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2E634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E638 size=112
    let mut pc: u32 = 0x82E2E638;
    'dispatch: loop {
        match pc {
            0x82E2E638 => {
    //   block [0x82E2E638..0x82E2E6A8)
	// 82E2E638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2E640: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2E644: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2E648: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E64C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2E650: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2E654: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2E658: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2E65C: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2E660: 38600058  li r3, 0x58
	ctx.r[3].s64 = 88;
	// 82E2E664: 4BFC3D85  bl 0x82df23e8
	ctx.lr = 0x82E2E668;
	sub_82DF23E8(ctx, base);
	// 82E2E668: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2E66C: 41820010  beq 0x82e2e67c
	if ctx.cr[0].eq {
	pc = 0x82E2E67C; continue 'dispatch;
	}
	// 82E2E670: 4BFFF781  bl 0x82e2ddf0
	ctx.lr = 0x82E2E674;
	sub_82E2DDF0(ctx, base);
	// 82E2E674: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E678: 48000008  b 0x82e2e680
	pc = 0x82E2E680; continue 'dispatch;
	// 82E2E67C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2E680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E684: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2E688: 4BFD01F1  bl 0x82dfe878
	ctx.lr = 0x82E2E68C;
	sub_82DFE878(ctx, base);
	// 82E2E68C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E690: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2E694: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2E698: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2E69C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2E6A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2E6A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E6A8 size=132
    let mut pc: u32 = 0x82E2E6A8;
    'dispatch: loop {
        match pc {
            0x82E2E6A8 => {
    //   block [0x82E2E6A8..0x82E2E72C)
	// 82E2E6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E6AC: 48379ABD  bl 0x831a8168
	ctx.lr = 0x82E2E6B0;
	sub_831A8130(ctx, base);
	// 82E2E6B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E6B4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E2E6B8: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82E2E6BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E2E6C0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2E6C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E6C8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2E6CC: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E2E6D0: 409A0044  bne cr6, 0x82e2e714
	if !ctx.cr[6].eq {
	pc = 0x82E2E714; continue 'dispatch;
	}
	// 82E2E6D4: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E2E6D8: 409A003C  bne cr6, 0x82e2e714
	if !ctx.cr[6].eq {
	pc = 0x82E2E714; continue 'dispatch;
	}
	// 82E2E6DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E6E0: 4BFFF951  bl 0x82e2e030
	ctx.lr = 0x82E2E6E4;
	sub_82E2E030(ctx, base);
	// 82E2E6E4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E6E8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2E6EC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2E6F0: 48000030  b 0x82e2e720
	pc = 0x82E2E720; continue 'dispatch;
	// 82E2E6F4: 386100A4  addi r3, r1, 0xa4
	ctx.r[3].s64 = ctx.r[1].s64 + 164;
	// 82E2E6F8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E2E6FC: 4BFD02D5  bl 0x82dfe9d0
	ctx.lr = 0x82E2E700;
	sub_82DFE9D0(ctx, base);
	// 82E2E700: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E2E704: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E2E708: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2E70C: 4BFFEE65  bl 0x82e2d570
	ctx.lr = 0x82E2E710;
	sub_82E2D570(ctx, base);
	// 82E2E710: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 82E2E714: 7F05F040  cmplw cr6, r5, r30
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[30].u32, &mut ctx.xer);
	// 82E2E718: 409AFFDC  bne cr6, 0x82e2e6f4
	if !ctx.cr[6].eq {
	pc = 0x82E2E6F4; continue 'dispatch;
	}
	// 82E2E71C: 90BD0000  stw r5, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82E2E720: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E2E724: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2E728: 48379A90  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E730 size=108
    let mut pc: u32 = 0x82E2E730;
    'dispatch: loop {
        match pc {
            0x82E2E730 => {
    //   block [0x82E2E730..0x82E2E79C)
	// 82E2E730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E734: 48379A35  bl 0x831a8168
	ctx.lr = 0x82E2E738;
	sub_831A8130(ctx, base);
	// 82E2E738: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E73C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2E740: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E744: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2E748: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2E74C: 388BA790  addi r4, r11, -0x5870
	ctx.r[4].s64 = ctx.r[11].s64 + -22640;
	// 82E2E750: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2E754: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2E758: 4BFC52B1  bl 0x82df3a08
	ctx.lr = 0x82E2E75C;
	sub_82DF3A08(ctx, base);
	// 82E2E75C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2E760: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2E764: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2E768: 48000C09  bl 0x82e2f370
	ctx.lr = 0x82E2E76C;
	sub_82E2F370(ctx, base);
	// 82E2E76C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2E770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E774: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E778: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2E77C: 4BFFF90D  bl 0x82e2e088
	ctx.lr = 0x82E2E780;
	sub_82E2E088(ctx, base);
	// 82E2E780: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2E784: 4BFC4CA5  bl 0x82df3428
	ctx.lr = 0x82E2E788;
	sub_82DF3428(ctx, base);
	// 82E2E788: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2E78C: 4BFC4C9D  bl 0x82df3428
	ctx.lr = 0x82E2E790;
	sub_82DF3428(ctx, base);
	// 82E2E790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E794: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2E798: 48379A20  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E7A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E7A0 size=108
    let mut pc: u32 = 0x82E2E7A0;
    'dispatch: loop {
        match pc {
            0x82E2E7A0 => {
    //   block [0x82E2E7A0..0x82E2E80C)
	// 82E2E7A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E7A4: 483799C5  bl 0x831a8168
	ctx.lr = 0x82E2E7A8;
	sub_831A8130(ctx, base);
	// 82E2E7A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E7AC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2E7B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E7B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2E7B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2E7BC: 388BA7A8  addi r4, r11, -0x5858
	ctx.r[4].s64 = ctx.r[11].s64 + -22616;
	// 82E2E7C0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2E7C4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2E7C8: 4BFC5241  bl 0x82df3a08
	ctx.lr = 0x82E2E7CC;
	sub_82DF3A08(ctx, base);
	// 82E2E7CC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2E7D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2E7D4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2E7D8: 48000B99  bl 0x82e2f370
	ctx.lr = 0x82E2E7DC;
	sub_82E2F370(ctx, base);
	// 82E2E7DC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2E7E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E7E4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E7E8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2E7EC: 4BFFF945  bl 0x82e2e130
	ctx.lr = 0x82E2E7F0;
	sub_82E2E130(ctx, base);
	// 82E2E7F0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2E7F4: 4BFC4C35  bl 0x82df3428
	ctx.lr = 0x82E2E7F8;
	sub_82DF3428(ctx, base);
	// 82E2E7F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2E7FC: 4BFC4C2D  bl 0x82df3428
	ctx.lr = 0x82E2E800;
	sub_82DF3428(ctx, base);
	// 82E2E800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E804: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2E808: 483799B0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E810 size=108
    let mut pc: u32 = 0x82E2E810;
    'dispatch: loop {
        match pc {
            0x82E2E810 => {
    //   block [0x82E2E810..0x82E2E87C)
	// 82E2E810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E814: 48379955  bl 0x831a8168
	ctx.lr = 0x82E2E818;
	sub_831A8130(ctx, base);
	// 82E2E818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E81C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2E820: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E824: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2E828: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2E82C: 388BA7B0  addi r4, r11, -0x5850
	ctx.r[4].s64 = ctx.r[11].s64 + -22608;
	// 82E2E830: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2E834: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2E838: 4BFC51D1  bl 0x82df3a08
	ctx.lr = 0x82E2E83C;
	sub_82DF3A08(ctx, base);
	// 82E2E83C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2E840: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2E844: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2E848: 48000B29  bl 0x82e2f370
	ctx.lr = 0x82E2E84C;
	sub_82E2F370(ctx, base);
	// 82E2E84C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2E850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E854: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E858: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2E85C: 4BFFF97D  bl 0x82e2e1d8
	ctx.lr = 0x82E2E860;
	sub_82E2E1D8(ctx, base);
	// 82E2E860: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2E864: 4BFC4BC5  bl 0x82df3428
	ctx.lr = 0x82E2E868;
	sub_82DF3428(ctx, base);
	// 82E2E868: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2E86C: 4BFC4BBD  bl 0x82df3428
	ctx.lr = 0x82E2E870;
	sub_82DF3428(ctx, base);
	// 82E2E870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E874: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2E878: 48379940  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E880 size=108
    let mut pc: u32 = 0x82E2E880;
    'dispatch: loop {
        match pc {
            0x82E2E880 => {
    //   block [0x82E2E880..0x82E2E8EC)
	// 82E2E880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E884: 483798E5  bl 0x831a8168
	ctx.lr = 0x82E2E888;
	sub_831A8130(ctx, base);
	// 82E2E888: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E88C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2E890: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E894: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2E898: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2E89C: 388BA7C0  addi r4, r11, -0x5840
	ctx.r[4].s64 = ctx.r[11].s64 + -22592;
	// 82E2E8A0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2E8A4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2E8A8: 4BFC5161  bl 0x82df3a08
	ctx.lr = 0x82E2E8AC;
	sub_82DF3A08(ctx, base);
	// 82E2E8AC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2E8B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2E8B4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2E8B8: 48000AB9  bl 0x82e2f370
	ctx.lr = 0x82E2E8BC;
	sub_82E2F370(ctx, base);
	// 82E2E8BC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2E8C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E8C4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E8C8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2E8CC: 4BFFF9B5  bl 0x82e2e280
	ctx.lr = 0x82E2E8D0;
	sub_82E2E280(ctx, base);
	// 82E2E8D0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2E8D4: 4BFC4B55  bl 0x82df3428
	ctx.lr = 0x82E2E8D8;
	sub_82DF3428(ctx, base);
	// 82E2E8D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2E8DC: 4BFC4B4D  bl 0x82df3428
	ctx.lr = 0x82E2E8E0;
	sub_82DF3428(ctx, base);
	// 82E2E8E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E8E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2E8E8: 483798D0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E8F0 size=108
    let mut pc: u32 = 0x82E2E8F0;
    'dispatch: loop {
        match pc {
            0x82E2E8F0 => {
    //   block [0x82E2E8F0..0x82E2E95C)
	// 82E2E8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E8F4: 48379875  bl 0x831a8168
	ctx.lr = 0x82E2E8F8;
	sub_831A8130(ctx, base);
	// 82E2E8F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E8FC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2E900: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E904: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2E908: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2E90C: 388BA7C8  addi r4, r11, -0x5838
	ctx.r[4].s64 = ctx.r[11].s64 + -22584;
	// 82E2E910: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2E914: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2E918: 4BFC50F1  bl 0x82df3a08
	ctx.lr = 0x82E2E91C;
	sub_82DF3A08(ctx, base);
	// 82E2E91C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2E920: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2E924: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2E928: 48000A49  bl 0x82e2f370
	ctx.lr = 0x82E2E92C;
	sub_82E2F370(ctx, base);
	// 82E2E92C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2E930: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E934: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E938: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2E93C: 4BFFF9ED  bl 0x82e2e328
	ctx.lr = 0x82E2E940;
	sub_82E2E328(ctx, base);
	// 82E2E940: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2E944: 4BFC4AE5  bl 0x82df3428
	ctx.lr = 0x82E2E948;
	sub_82DF3428(ctx, base);
	// 82E2E948: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2E94C: 4BFC4ADD  bl 0x82df3428
	ctx.lr = 0x82E2E950;
	sub_82DF3428(ctx, base);
	// 82E2E950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E954: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2E958: 48379860  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E960 size=108
    let mut pc: u32 = 0x82E2E960;
    'dispatch: loop {
        match pc {
            0x82E2E960 => {
    //   block [0x82E2E960..0x82E2E9CC)
	// 82E2E960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E964: 48379805  bl 0x831a8168
	ctx.lr = 0x82E2E968;
	sub_831A8130(ctx, base);
	// 82E2E968: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E96C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2E970: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E974: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2E978: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2E97C: 388BA7E8  addi r4, r11, -0x5818
	ctx.r[4].s64 = ctx.r[11].s64 + -22552;
	// 82E2E980: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2E984: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2E988: 4BFC5081  bl 0x82df3a08
	ctx.lr = 0x82E2E98C;
	sub_82DF3A08(ctx, base);
	// 82E2E98C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2E990: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2E994: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2E998: 480009D9  bl 0x82e2f370
	ctx.lr = 0x82E2E99C;
	sub_82E2F370(ctx, base);
	// 82E2E99C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2E9A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E9A4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2E9A8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2E9AC: 4BFFFA25  bl 0x82e2e3d0
	ctx.lr = 0x82E2E9B0;
	sub_82E2E3D0(ctx, base);
	// 82E2E9B0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2E9B4: 4BFC4A75  bl 0x82df3428
	ctx.lr = 0x82E2E9B8;
	sub_82DF3428(ctx, base);
	// 82E2E9B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2E9BC: 4BFC4A6D  bl 0x82df3428
	ctx.lr = 0x82E2E9C0;
	sub_82DF3428(ctx, base);
	// 82E2E9C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2E9C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2E9C8: 483797F0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2E9D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2E9D0 size=108
    let mut pc: u32 = 0x82E2E9D0;
    'dispatch: loop {
        match pc {
            0x82E2E9D0 => {
    //   block [0x82E2E9D0..0x82E2EA3C)
	// 82E2E9D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2E9D4: 48379795  bl 0x831a8168
	ctx.lr = 0x82E2E9D8;
	sub_831A8130(ctx, base);
	// 82E2E9D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2E9DC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2E9E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2E9E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2E9E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2E9EC: 388BA7F8  addi r4, r11, -0x5808
	ctx.r[4].s64 = ctx.r[11].s64 + -22536;
	// 82E2E9F0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2E9F4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2E9F8: 4BFC5011  bl 0x82df3a08
	ctx.lr = 0x82E2E9FC;
	sub_82DF3A08(ctx, base);
	// 82E2E9FC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2EA00: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2EA04: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2EA08: 48000969  bl 0x82e2f370
	ctx.lr = 0x82E2EA0C;
	sub_82E2F370(ctx, base);
	// 82E2EA0C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2EA10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2EA14: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2EA18: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2EA1C: 4BFFFA5D  bl 0x82e2e478
	ctx.lr = 0x82E2EA20;
	sub_82E2E478(ctx, base);
	// 82E2EA20: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2EA24: 4BFC4A05  bl 0x82df3428
	ctx.lr = 0x82E2EA28;
	sub_82DF3428(ctx, base);
	// 82E2EA28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2EA2C: 4BFC49FD  bl 0x82df3428
	ctx.lr = 0x82E2EA30;
	sub_82DF3428(ctx, base);
	// 82E2EA30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2EA34: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2EA38: 48379780  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2EA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2EA40 size=108
    let mut pc: u32 = 0x82E2EA40;
    'dispatch: loop {
        match pc {
            0x82E2EA40 => {
    //   block [0x82E2EA40..0x82E2EAAC)
	// 82E2EA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2EA44: 48379725  bl 0x831a8168
	ctx.lr = 0x82E2EA48;
	sub_831A8130(ctx, base);
	// 82E2EA48: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2EA4C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2EA50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2EA54: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2EA58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2EA5C: 388BA7FC  addi r4, r11, -0x5804
	ctx.r[4].s64 = ctx.r[11].s64 + -22532;
	// 82E2EA60: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2EA64: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2EA68: 4BFC4FA1  bl 0x82df3a08
	ctx.lr = 0x82E2EA6C;
	sub_82DF3A08(ctx, base);
	// 82E2EA6C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2EA70: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2EA74: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2EA78: 480008F9  bl 0x82e2f370
	ctx.lr = 0x82E2EA7C;
	sub_82E2F370(ctx, base);
	// 82E2EA7C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2EA80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2EA84: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2EA88: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2EA8C: 4BFFFA95  bl 0x82e2e520
	ctx.lr = 0x82E2EA90;
	sub_82E2E520(ctx, base);
	// 82E2EA90: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2EA94: 4BFC4995  bl 0x82df3428
	ctx.lr = 0x82E2EA98;
	sub_82DF3428(ctx, base);
	// 82E2EA98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2EA9C: 4BFC498D  bl 0x82df3428
	ctx.lr = 0x82E2EAA0;
	sub_82DF3428(ctx, base);
	// 82E2EAA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2EAA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2EAA8: 48379710  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2EAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2EAB0 size=164
    let mut pc: u32 = 0x82E2EAB0;
    'dispatch: loop {
        match pc {
            0x82E2EAB0 => {
    //   block [0x82E2EAB0..0x82E2EB54)
	// 82E2EAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2EAB4: 483796B9  bl 0x831a816c
	ctx.lr = 0x82E2EAB8;
	sub_831A8130(ctx, base);
	// 82E2EAB8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2EABC: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2EAC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2EAC4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2EAC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2EACC: 396BE5C8  addi r11, r11, -0x1a38
	ctx.r[11].s64 = ctx.r[11].s64 + -6712;
	// 82E2EAD0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2EAD4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2EAD8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2EADC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2EAE0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2EAE4: 4BFCFE15  bl 0x82dfe8f8
	ctx.lr = 0x82E2EAE8;
	sub_82DFE8F8(ctx, base);
	// 82E2EAE8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2EAEC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2EAF0: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2EAF4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2EAF8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2EAFC: 4BFD2D9D  bl 0x82e01898
	ctx.lr = 0x82E2EB00;
	sub_82E01898(ctx, base);
	// 82E2EB00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2EB04: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2EB08: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2EB0C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2EB10: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2EB14: 419A0024  beq cr6, 0x82e2eb38
	if ctx.cr[6].eq {
	pc = 0x82E2EB38; continue 'dispatch;
	}
	// 82E2EB18: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2EB1C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2EB20: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2EB24: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2EB28: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2EB2C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2EB30: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2EB34: 4082FFE8  bne 0x82e2eb1c
	if !ctx.cr[0].eq {
	pc = 0x82E2EB1C; continue 'dispatch;
	}
	// 82E2EB38: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2EB3C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2EB40: 419A0008  beq cr6, 0x82e2eb48
	if ctx.cr[6].eq {
	pc = 0x82E2EB48; continue 'dispatch;
	}
	// 82E2EB44: 4B491D4D  bl 0x822c0890
	ctx.lr = 0x82E2EB48;
	sub_822C0890(ctx, base);
	// 82E2EB48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2EB4C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2EB50: 4837966C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2EB58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2EB58 size=164
    let mut pc: u32 = 0x82E2EB58;
    'dispatch: loop {
        match pc {
            0x82E2EB58 => {
    //   block [0x82E2EB58..0x82E2EBFC)
	// 82E2EB58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2EB5C: 48379611  bl 0x831a816c
	ctx.lr = 0x82E2EB60;
	sub_831A8130(ctx, base);
	// 82E2EB60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2EB64: 3D60827C  lis r11, -0x7d84
	ctx.r[11].s64 = -2105802752;
	// 82E2EB68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2EB6C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2EB70: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2EB74: 396BE788  addi r11, r11, -0x1878
	ctx.r[11].s64 = ctx.r[11].s64 + -6264;
	// 82E2EB78: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2EB7C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2EB80: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2EB84: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2EB88: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2EB8C: 4BFCFD6D  bl 0x82dfe8f8
	ctx.lr = 0x82E2EB90;
	sub_82DFE8F8(ctx, base);
	// 82E2EB90: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2EB94: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2EB98: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2EB9C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2EBA0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2EBA4: 4BFD2CF5  bl 0x82e01898
	ctx.lr = 0x82E2EBA8;
	sub_82E01898(ctx, base);
	// 82E2EBA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2EBAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2EBB0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2EBB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2EBB8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2EBBC: 419A0024  beq cr6, 0x82e2ebe0
	if ctx.cr[6].eq {
	pc = 0x82E2EBE0; continue 'dispatch;
	}
	// 82E2EBC0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2EBC4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2EBC8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2EBCC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2EBD0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2EBD4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2EBD8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2EBDC: 4082FFE8  bne 0x82e2ebc4
	if !ctx.cr[0].eq {
	pc = 0x82E2EBC4; continue 'dispatch;
	}
	// 82E2EBE0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2EBE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2EBE8: 419A0008  beq cr6, 0x82e2ebf0
	if ctx.cr[6].eq {
	pc = 0x82E2EBF0; continue 'dispatch;
	}
	// 82E2EBEC: 4B491CA5  bl 0x822c0890
	ctx.lr = 0x82E2EBF0;
	sub_822C0890(ctx, base);
	// 82E2EBF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2EBF4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2EBF8: 483795C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2EC00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2EC00 size=164
    let mut pc: u32 = 0x82E2EC00;
    'dispatch: loop {
        match pc {
            0x82E2EC00 => {
    //   block [0x82E2EC00..0x82E2ECA4)
	// 82E2EC00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2EC04: 48379569  bl 0x831a816c
	ctx.lr = 0x82E2EC08;
	sub_831A8130(ctx, base);
	// 82E2EC08: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2EC0C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2EC10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2EC14: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2EC18: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2EC1C: 396BE638  addi r11, r11, -0x19c8
	ctx.r[11].s64 = ctx.r[11].s64 + -6600;
	// 82E2EC20: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2EC24: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2EC28: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2EC2C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2EC30: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2EC34: 4BFCFCC5  bl 0x82dfe8f8
	ctx.lr = 0x82E2EC38;
	sub_82DFE8F8(ctx, base);
	// 82E2EC38: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2EC3C: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2EC40: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2EC44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2EC48: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2EC4C: 4BFD2C4D  bl 0x82e01898
	ctx.lr = 0x82E2EC50;
	sub_82E01898(ctx, base);
	// 82E2EC50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2EC54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2EC58: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2EC5C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2EC60: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2EC64: 419A0024  beq cr6, 0x82e2ec88
	if ctx.cr[6].eq {
	pc = 0x82E2EC88; continue 'dispatch;
	}
	// 82E2EC68: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2EC6C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2EC70: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2EC74: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2EC78: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2EC7C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2EC80: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2EC84: 4082FFE8  bne 0x82e2ec6c
	if !ctx.cr[0].eq {
	pc = 0x82E2EC6C; continue 'dispatch;
	}
	// 82E2EC88: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2EC8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2EC90: 419A0008  beq cr6, 0x82e2ec98
	if ctx.cr[6].eq {
	pc = 0x82E2EC98; continue 'dispatch;
	}
	// 82E2EC94: 4B491BFD  bl 0x822c0890
	ctx.lr = 0x82E2EC98;
	sub_822C0890(ctx, base);
	// 82E2EC98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2EC9C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2ECA0: 4837951C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2ECA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2ECA8 size=84
    let mut pc: u32 = 0x82E2ECA8;
    'dispatch: loop {
        match pc {
            0x82E2ECA8 => {
    //   block [0x82E2ECA8..0x82E2ECFC)
	// 82E2ECA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2ECAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2ECB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2ECB4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2ECB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2ECBC: 4BFCFADD  bl 0x82dfe798
	ctx.lr = 0x82E2ECC0;
	sub_82DFE798(ctx, base);
	// 82E2ECC0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E2ECC4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E2ECC8: 396BC0E4  addi r11, r11, -0x3f1c
	ctx.r[11].s64 = ctx.r[11].s64 + -16156;
	// 82E2ECCC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2ECD0: 4BFFCCD9  bl 0x82e2b9a8
	ctx.lr = 0x82E2ECD4;
	sub_82E2B9A8(ctx, base);
	// 82E2ECD4: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82E2ECD8: 4BFFCCD1  bl 0x82e2b9a8
	ctx.lr = 0x82E2ECDC;
	sub_82E2B9A8(ctx, base);
	// 82E2ECDC: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82E2ECE0: 4BFFCCC9  bl 0x82e2b9a8
	ctx.lr = 0x82E2ECE4;
	sub_82E2B9A8(ctx, base);
	// 82E2ECE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2ECE8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2ECEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2ECF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2ECF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2ECF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2ED00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2ED00 size=84
    let mut pc: u32 = 0x82E2ED00;
    'dispatch: loop {
        match pc {
            0x82E2ED00 => {
    //   block [0x82E2ED00..0x82E2ED54)
	// 82E2ED00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2ED04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2ED08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2ED0C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2ED10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2ED14: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82E2ED18: 4BFFE1F9  bl 0x82e2cf10
	ctx.lr = 0x82E2ED1C;
	sub_82E2CF10(ctx, base);
	// 82E2ED1C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82E2ED20: 4BFFE1F1  bl 0x82e2cf10
	ctx.lr = 0x82E2ED24;
	sub_82E2CF10(ctx, base);
	// 82E2ED24: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E2ED28: 4BFFE1E9  bl 0x82e2cf10
	ctx.lr = 0x82E2ED2C;
	sub_82E2CF10(ctx, base);
	// 82E2ED2C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2ED30: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E2ED34: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82E2ED38: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2ED3C: 4BFC46ED  bl 0x82df3428
	ctx.lr = 0x82E2ED40;
	sub_82DF3428(ctx, base);
	// 82E2ED40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2ED44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2ED48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2ED4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2ED50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2ED58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2ED58 size=76
    let mut pc: u32 = 0x82E2ED58;
    'dispatch: loop {
        match pc {
            0x82E2ED58 => {
    //   block [0x82E2ED58..0x82E2EDA4)
	// 82E2ED58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2ED5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2ED60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2ED64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2ED68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2ED6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2ED70: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2ED74: 4BFFFF8D  bl 0x82e2ed00
	ctx.lr = 0x82E2ED78;
	sub_82E2ED00(ctx, base);
	// 82E2ED78: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2ED7C: 4182000C  beq 0x82e2ed88
	if ctx.cr[0].eq {
	pc = 0x82E2ED88; continue 'dispatch;
	}
	// 82E2ED80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2ED84: 4BFC3655  bl 0x82df23d8
	ctx.lr = 0x82E2ED88;
	sub_82DF23D8(ctx, base);
	// 82E2ED88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2ED8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2ED90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2ED94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2ED98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2ED9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2EDA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2EDA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2EDA8 size=88
    let mut pc: u32 = 0x82E2EDA8;
    'dispatch: loop {
        match pc {
            0x82E2EDA8 => {
    //   block [0x82E2EDA8..0x82E2EE00)
	// 82E2EDA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2EDAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2EDB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2EDB4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2EDB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2EDBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2EDC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E2EDC4: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2EDC8: 80A60000  lwz r5, 0(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2EDCC: 4BFFF8DD  bl 0x82e2e6a8
	ctx.lr = 0x82E2EDD0;
	sub_82E2E6A8(ctx, base);
	// 82E2EDD0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 82E2EDD4: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2EDD8: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 82E2EDDC: 4BFC33AD  bl 0x82df2188
	ctx.lr = 0x82E2EDE0;
	sub_82DF2188(ctx, base);
	// 82E2EDE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2EDE4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2EDE8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E2EDEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2EDF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2EDF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2EDF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2EDFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2EE00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2EE00 size=108
    let mut pc: u32 = 0x82E2EE00;
    'dispatch: loop {
        match pc {
            0x82E2EE00 => {
    //   block [0x82E2EE00..0x82E2EE6C)
	// 82E2EE00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2EE04: 48379365  bl 0x831a8168
	ctx.lr = 0x82E2EE08;
	sub_831A8130(ctx, base);
	// 82E2EE08: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2EE0C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2EE10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2EE14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2EE18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2EE1C: 388BA79C  addi r4, r11, -0x5864
	ctx.r[4].s64 = ctx.r[11].s64 + -22628;
	// 82E2EE20: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2EE24: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2EE28: 4BFC4BE1  bl 0x82df3a08
	ctx.lr = 0x82E2EE2C;
	sub_82DF3A08(ctx, base);
	// 82E2EE2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2EE30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2EE34: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2EE38: 48000539  bl 0x82e2f370
	ctx.lr = 0x82E2EE3C;
	sub_82E2F370(ctx, base);
	// 82E2EE3C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2EE40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2EE44: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2EE48: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2EE4C: 4BFFFC65  bl 0x82e2eab0
	ctx.lr = 0x82E2EE50;
	sub_82E2EAB0(ctx, base);
	// 82E2EE50: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2EE54: 4BFC45D5  bl 0x82df3428
	ctx.lr = 0x82E2EE58;
	sub_82DF3428(ctx, base);
	// 82E2EE58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2EE5C: 4BFC45CD  bl 0x82df3428
	ctx.lr = 0x82E2EE60;
	sub_82DF3428(ctx, base);
	// 82E2EE60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2EE64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2EE68: 48379350  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2EE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2EE70 size=108
    let mut pc: u32 = 0x82E2EE70;
    'dispatch: loop {
        match pc {
            0x82E2EE70 => {
    //   block [0x82E2EE70..0x82E2EEDC)
	// 82E2EE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2EE74: 483792F5  bl 0x831a8168
	ctx.lr = 0x82E2EE78;
	sub_831A8130(ctx, base);
	// 82E2EE78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2EE7C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2EE80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2EE84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2EE88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2EE8C: 388BA7A4  addi r4, r11, -0x585c
	ctx.r[4].s64 = ctx.r[11].s64 + -22620;
	// 82E2EE90: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2EE94: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2EE98: 4BFC4B71  bl 0x82df3a08
	ctx.lr = 0x82E2EE9C;
	sub_82DF3A08(ctx, base);
	// 82E2EE9C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2EEA0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2EEA4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2EEA8: 480004C9  bl 0x82e2f370
	ctx.lr = 0x82E2EEAC;
	sub_82E2F370(ctx, base);
	// 82E2EEAC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2EEB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2EEB4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2EEB8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2EEBC: 4BFFFC9D  bl 0x82e2eb58
	ctx.lr = 0x82E2EEC0;
	sub_82E2EB58(ctx, base);
	// 82E2EEC0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2EEC4: 4BFC4565  bl 0x82df3428
	ctx.lr = 0x82E2EEC8;
	sub_82DF3428(ctx, base);
	// 82E2EEC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2EECC: 4BFC455D  bl 0x82df3428
	ctx.lr = 0x82E2EED0;
	sub_82DF3428(ctx, base);
	// 82E2EED0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2EED4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2EED8: 483792E0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2EEE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2EEE0 size=108
    let mut pc: u32 = 0x82E2EEE0;
    'dispatch: loop {
        match pc {
            0x82E2EEE0 => {
    //   block [0x82E2EEE0..0x82E2EF4C)
	// 82E2EEE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2EEE4: 48379285  bl 0x831a8168
	ctx.lr = 0x82E2EEE8;
	sub_831A8130(ctx, base);
	// 82E2EEE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2EEEC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2EEF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2EEF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2EEF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2EEFC: 388BA7D8  addi r4, r11, -0x5828
	ctx.r[4].s64 = ctx.r[11].s64 + -22568;
	// 82E2EF00: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2EF04: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2EF08: 4BFC4B01  bl 0x82df3a08
	ctx.lr = 0x82E2EF0C;
	sub_82DF3A08(ctx, base);
	// 82E2EF0C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2EF10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2EF14: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2EF18: 48000459  bl 0x82e2f370
	ctx.lr = 0x82E2EF1C;
	sub_82E2F370(ctx, base);
	// 82E2EF1C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2EF20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2EF24: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2EF28: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2EF2C: 4BFFFCD5  bl 0x82e2ec00
	ctx.lr = 0x82E2EF30;
	sub_82E2EC00(ctx, base);
	// 82E2EF30: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2EF34: 4BFC44F5  bl 0x82df3428
	ctx.lr = 0x82E2EF38;
	sub_82DF3428(ctx, base);
	// 82E2EF38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2EF3C: 4BFC44ED  bl 0x82df3428
	ctx.lr = 0x82E2EF40;
	sub_82DF3428(ctx, base);
	// 82E2EF40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2EF44: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2EF48: 48379270  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2EF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2EF50 size=112
    let mut pc: u32 = 0x82E2EF50;
    'dispatch: loop {
        match pc {
            0x82E2EF50 => {
    //   block [0x82E2EF50..0x82E2EFC0)
	// 82E2EF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2EF54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2EF58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2EF5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2EF60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2EF64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2EF68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2EF6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2EF70: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2EF74: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2EF78: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 82E2EF7C: 4BFC346D  bl 0x82df23e8
	ctx.lr = 0x82E2EF80;
	sub_82DF23E8(ctx, base);
	// 82E2EF80: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2EF84: 41820010  beq 0x82e2ef94
	if ctx.cr[0].eq {
	pc = 0x82E2EF94; continue 'dispatch;
	}
	// 82E2EF88: 4BFFFD21  bl 0x82e2eca8
	ctx.lr = 0x82E2EF8C;
	sub_82E2ECA8(ctx, base);
	// 82E2EF8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2EF90: 48000008  b 0x82e2ef98
	pc = 0x82E2EF98; continue 'dispatch;
	// 82E2EF94: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2EF98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2EF9C: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2EFA0: 4BFCF8D9  bl 0x82dfe878
	ctx.lr = 0x82E2EFA4;
	sub_82DFE878(ctx, base);
	// 82E2EFA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2EFA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2EFAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2EFB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2EFB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2EFB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2EFBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2EFC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2EFC0 size=164
    let mut pc: u32 = 0x82E2EFC0;
    'dispatch: loop {
        match pc {
            0x82E2EFC0 => {
    //   block [0x82E2EFC0..0x82E2F064)
	// 82E2EFC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2EFC4: 483791A9  bl 0x831a816c
	ctx.lr = 0x82E2EFC8;
	sub_831A8130(ctx, base);
	// 82E2EFC8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2EFCC: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2EFD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2EFD4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2EFD8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2EFDC: 396BEF50  addi r11, r11, -0x10b0
	ctx.r[11].s64 = ctx.r[11].s64 + -4272;
	// 82E2EFE0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2EFE4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2EFE8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2EFEC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2EFF0: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2EFF4: 4BFCF905  bl 0x82dfe8f8
	ctx.lr = 0x82E2EFF8;
	sub_82DFE8F8(ctx, base);
	// 82E2EFF8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2EFFC: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2F000: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2F004: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2F008: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2F00C: 4BFD288D  bl 0x82e01898
	ctx.lr = 0x82E2F010;
	sub_82E01898(ctx, base);
	// 82E2F010: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2F014: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2F018: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2F01C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2F020: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2F024: 419A0024  beq cr6, 0x82e2f048
	if ctx.cr[6].eq {
	pc = 0x82E2F048; continue 'dispatch;
	}
	// 82E2F028: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2F02C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2F030: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2F034: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2F038: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2F03C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2F040: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2F044: 4082FFE8  bne 0x82e2f02c
	if !ctx.cr[0].eq {
	pc = 0x82E2F02C; continue 'dispatch;
	}
	// 82E2F048: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2F04C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2F050: 419A0008  beq cr6, 0x82e2f058
	if ctx.cr[6].eq {
	pc = 0x82E2F058; continue 'dispatch;
	}
	// 82E2F054: 4B49183D  bl 0x822c0890
	ctx.lr = 0x82E2F058;
	sub_822C0890(ctx, base);
	// 82E2F058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F05C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2F060: 4837915C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F068 size=108
    let mut pc: u32 = 0x82E2F068;
    'dispatch: loop {
        match pc {
            0x82E2F068 => {
    //   block [0x82E2F068..0x82E2F0D4)
	// 82E2F068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F06C: 483790FD  bl 0x831a8168
	ctx.lr = 0x82E2F070;
	sub_831A8130(ctx, base);
	// 82E2F070: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F074: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F078: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F07C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F080: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F084: 388BA7F0  addi r4, r11, -0x5810
	ctx.r[4].s64 = ctx.r[11].s64 + -22544;
	// 82E2F088: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2F08C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2F090: 4BFC4979  bl 0x82df3a08
	ctx.lr = 0x82E2F094;
	sub_82DF3A08(ctx, base);
	// 82E2F094: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2F098: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F09C: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F0A0: 480002D1  bl 0x82e2f370
	ctx.lr = 0x82E2F0A4;
	sub_82E2F370(ctx, base);
	// 82E2F0A4: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2F0A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F0AC: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2F0B0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2F0B4: 4BFFFF0D  bl 0x82e2efc0
	ctx.lr = 0x82E2F0B8;
	sub_82E2EFC0(ctx, base);
	// 82E2F0B8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F0BC: 4BFC436D  bl 0x82df3428
	ctx.lr = 0x82E2F0C0;
	sub_82DF3428(ctx, base);
	// 82E2F0C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F0C4: 4BFC4365  bl 0x82df3428
	ctx.lr = 0x82E2F0C8;
	sub_82DF3428(ctx, base);
	// 82E2F0C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F0CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2F0D0: 483790E8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F0D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F0D8 size=92
    let mut pc: u32 = 0x82E2F0D8;
    'dispatch: loop {
        match pc {
            0x82E2F0D8 => {
    //   block [0x82E2F0D8..0x82E2F134)
	// 82E2F0D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F0DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F0E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F0E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F0E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F0EC: 4BFCF6AD  bl 0x82dfe798
	ctx.lr = 0x82E2F0F0;
	sub_82DFE798(ctx, base);
	// 82E2F0F0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E2F0F4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E2F0F8: 396BC0EC  addi r11, r11, -0x3f14
	ctx.r[11].s64 = ctx.r[11].s64 + -16148;
	// 82E2F0FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2F100: 4BFFC8A9  bl 0x82e2b9a8
	ctx.lr = 0x82E2F104;
	sub_82E2B9A8(ctx, base);
	// 82E2F104: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82E2F108: 4BFFC8A1  bl 0x82e2b9a8
	ctx.lr = 0x82E2F10C;
	sub_82E2B9A8(ctx, base);
	// 82E2F10C: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82E2F110: 4BFFC899  bl 0x82e2b9a8
	ctx.lr = 0x82E2F114;
	sub_82E2B9A8(ctx, base);
	// 82E2F114: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82E2F118: 4BFD41B1  bl 0x82e032c8
	ctx.lr = 0x82E2F11C;
	sub_82E032C8(ctx, base);
	// 82E2F11C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2F124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F12C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F138 size=92
    let mut pc: u32 = 0x82E2F138;
    'dispatch: loop {
        match pc {
            0x82E2F138 => {
    //   block [0x82E2F138..0x82E2F194)
	// 82E2F138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F13C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F140: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F144: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F148: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F14C: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82E2F150: 4BFFFC59  bl 0x82e2eda8
	ctx.lr = 0x82E2F154;
	sub_82E2EDA8(ctx, base);
	// 82E2F154: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 82E2F158: 4BFFDDB9  bl 0x82e2cf10
	ctx.lr = 0x82E2F15C;
	sub_82E2CF10(ctx, base);
	// 82E2F15C: 387F0018  addi r3, r31, 0x18
	ctx.r[3].s64 = ctx.r[31].s64 + 24;
	// 82E2F160: 4BFFDDB1  bl 0x82e2cf10
	ctx.lr = 0x82E2F164;
	sub_82E2CF10(ctx, base);
	// 82E2F164: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E2F168: 4BFFDDA9  bl 0x82e2cf10
	ctx.lr = 0x82E2F16C;
	sub_82E2CF10(ctx, base);
	// 82E2F16C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2F170: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E2F174: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82E2F178: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2F17C: 4BFC42AD  bl 0x82df3428
	ctx.lr = 0x82E2F180;
	sub_82DF3428(ctx, base);
	// 82E2F180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2F184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F18C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F198 size=76
    let mut pc: u32 = 0x82E2F198;
    'dispatch: loop {
        match pc {
            0x82E2F198 => {
    //   block [0x82E2F198..0x82E2F1E4)
	// 82E2F198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F1A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F1A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F1A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F1AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F1B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F1B4: 4BFFFF85  bl 0x82e2f138
	ctx.lr = 0x82E2F1B8;
	sub_82E2F138(ctx, base);
	// 82E2F1B8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2F1BC: 4182000C  beq 0x82e2f1c8
	if ctx.cr[0].eq {
	pc = 0x82E2F1C8; continue 'dispatch;
	}
	// 82E2F1C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F1C4: 4BFC3215  bl 0x82df23d8
	ctx.lr = 0x82E2F1C8;
	sub_82DF23D8(ctx, base);
	// 82E2F1C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F1CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F1D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F1D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F1D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F1DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F1E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F1E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F1E8 size=112
    let mut pc: u32 = 0x82E2F1E8;
    'dispatch: loop {
        match pc {
            0x82E2F1E8 => {
    //   block [0x82E2F1E8..0x82E2F258)
	// 82E2F1E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F1EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F1F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F1F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F1F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F1FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E2F200: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2F204: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E2F208: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 82E2F20C: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 82E2F210: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 82E2F214: 4BFC31D5  bl 0x82df23e8
	ctx.lr = 0x82E2F218;
	sub_82DF23E8(ctx, base);
	// 82E2F218: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E2F21C: 41820010  beq 0x82e2f22c
	if ctx.cr[0].eq {
	pc = 0x82E2F22C; continue 'dispatch;
	}
	// 82E2F220: 4BFFFEB9  bl 0x82e2f0d8
	ctx.lr = 0x82E2F224;
	sub_82E2F0D8(ctx, base);
	// 82E2F224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F228: 48000008  b 0x82e2f230
	pc = 0x82E2F230; continue 'dispatch;
	// 82E2F22C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E2F230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F234: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E2F238: 4BFCF641  bl 0x82dfe878
	ctx.lr = 0x82E2F23C;
	sub_82DFE878(ctx, base);
	// 82E2F23C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F240: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F24C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F250: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F258 size=164
    let mut pc: u32 = 0x82E2F258;
    'dispatch: loop {
        match pc {
            0x82E2F258 => {
    //   block [0x82E2F258..0x82E2F2FC)
	// 82E2F258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F25C: 48378F11  bl 0x831a816c
	ctx.lr = 0x82E2F260;
	sub_831A8130(ctx, base);
	// 82E2F260: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F264: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E2F268: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F26C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E2F270: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E2F274: 396BF1E8  addi r11, r11, -0xe18
	ctx.r[11].s64 = ctx.r[11].s64 + -3608;
	// 82E2F278: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E2F27C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82E2F280: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E2F284: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E2F288: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82E2F28C: 4BFCF66D  bl 0x82dfe8f8
	ctx.lr = 0x82E2F290;
	sub_82DFE8F8(ctx, base);
	// 82E2F290: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 82E2F294: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 82E2F298: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82E2F29C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E2F2A0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E2F2A4: 4BFD25F5  bl 0x82e01898
	ctx.lr = 0x82E2F2A8;
	sub_82E01898(ctx, base);
	// 82E2F2A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2F2AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2F2B0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2F2B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E2F2B8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2F2BC: 419A0024  beq cr6, 0x82e2f2e0
	if ctx.cr[6].eq {
	pc = 0x82E2F2E0; continue 'dispatch;
	}
	// 82E2F2C0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E2F2C4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 82E2F2C8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2F2CC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 82E2F2D0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E2F2D4: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 82E2F2D8: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 82E2F2DC: 4082FFE8  bne 0x82e2f2c4
	if !ctx.cr[0].eq {
	pc = 0x82E2F2C4; continue 'dispatch;
	}
	// 82E2F2E0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E2F2E4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E2F2E8: 419A0008  beq cr6, 0x82e2f2f0
	if ctx.cr[6].eq {
	pc = 0x82E2F2F0; continue 'dispatch;
	}
	// 82E2F2EC: 4B4915A5  bl 0x822c0890
	ctx.lr = 0x82E2F2F0;
	sub_822C0890(ctx, base);
	// 82E2F2F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F2F4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E2F2F8: 48378EC4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F300 size=108
    let mut pc: u32 = 0x82E2F300;
    'dispatch: loop {
        match pc {
            0x82E2F300 => {
    //   block [0x82E2F300..0x82E2F36C)
	// 82E2F300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F304: 48378E65  bl 0x831a8168
	ctx.lr = 0x82E2F308;
	sub_831A8130(ctx, base);
	// 82E2F308: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F30C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F310: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F314: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F318: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F31C: 388BA7B8  addi r4, r11, -0x5848
	ctx.r[4].s64 = ctx.r[11].s64 + -22600;
	// 82E2F320: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E2F324: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E2F328: 4BFC46E1  bl 0x82df3a08
	ctx.lr = 0x82E2F32C;
	sub_82DF3A08(ctx, base);
	// 82E2F32C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E2F330: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F334: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F338: 48000039  bl 0x82e2f370
	ctx.lr = 0x82E2F33C;
	sub_82E2F370(ctx, base);
	// 82E2F33C: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82E2F340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F344: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2F348: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E2F34C: 4BFFFF0D  bl 0x82e2f258
	ctx.lr = 0x82E2F350;
	sub_82E2F258(ctx, base);
	// 82E2F350: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F354: 4BFC40D5  bl 0x82df3428
	ctx.lr = 0x82E2F358;
	sub_82DF3428(ctx, base);
	// 82E2F358: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F35C: 4BFC40CD  bl 0x82df3428
	ctx.lr = 0x82E2F360;
	sub_82DF3428(ctx, base);
	// 82E2F360: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F364: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E2F368: 48378E50  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F370 size=48
    let mut pc: u32 = 0x82E2F370;
    'dispatch: loop {
        match pc {
            0x82E2F370 => {
    //   block [0x82E2F370..0x82E2F3A0)
	// 82E2F370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F378: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F37C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F380: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F384: 4BFC48BD  bl 0x82df3c40
	ctx.lr = 0x82E2F388;
	sub_82DF3C40(ctx, base);
	// 82E2F388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F38C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E2F390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F398: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F39C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F3A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F3A0 size=112
    let mut pc: u32 = 0x82E2F3A0;
    'dispatch: loop {
        match pc {
            0x82E2F3A0 => {
    //   block [0x82E2F3A0..0x82E2F410)
	// 82E2F3A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F3A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F3A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F3AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F3B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F3B4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F3B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F3BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F3C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F3C4: 388BA790  addi r4, r11, -0x5870
	ctx.r[4].s64 = ctx.r[11].s64 + -22640;
	// 82E2F3C8: 4BFC4641  bl 0x82df3a08
	ctx.lr = 0x82E2F3CC;
	sub_82DF3A08(ctx, base);
	// 82E2F3CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2F3D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F3D4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F3D8: 4BFFFF99  bl 0x82e2f370
	ctx.lr = 0x82E2F3DC;
	sub_82E2F370(ctx, base);
	// 82E2F3DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2F3E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F3E4: 4BFD3055  bl 0x82e02438
	ctx.lr = 0x82E2F3E8;
	sub_82E02438(ctx, base);
	// 82E2F3E8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F3EC: 4BFC403D  bl 0x82df3428
	ctx.lr = 0x82E2F3F0;
	sub_82DF3428(ctx, base);
	// 82E2F3F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F3F4: 4BFC4035  bl 0x82df3428
	ctx.lr = 0x82E2F3F8;
	sub_82DF3428(ctx, base);
	// 82E2F3F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F3FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F400: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F404: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F408: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F40C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F410 size=112
    let mut pc: u32 = 0x82E2F410;
    'dispatch: loop {
        match pc {
            0x82E2F410 => {
    //   block [0x82E2F410..0x82E2F480)
	// 82E2F410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F418: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F41C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F420: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F424: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F428: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F42C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F430: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F434: 388BA794  addi r4, r11, -0x586c
	ctx.r[4].s64 = ctx.r[11].s64 + -22636;
	// 82E2F438: 4BFC45D1  bl 0x82df3a08
	ctx.lr = 0x82E2F43C;
	sub_82DF3A08(ctx, base);
	// 82E2F43C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2F440: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F444: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F448: 4BFFFF29  bl 0x82e2f370
	ctx.lr = 0x82E2F44C;
	sub_82E2F370(ctx, base);
	// 82E2F44C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2F450: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F454: 4BFD2FE5  bl 0x82e02438
	ctx.lr = 0x82E2F458;
	sub_82E02438(ctx, base);
	// 82E2F458: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F45C: 4BFC3FCD  bl 0x82df3428
	ctx.lr = 0x82E2F460;
	sub_82DF3428(ctx, base);
	// 82E2F460: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F464: 4BFC3FC5  bl 0x82df3428
	ctx.lr = 0x82E2F468;
	sub_82DF3428(ctx, base);
	// 82E2F468: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F46C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F470: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F474: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F478: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F47C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F480 size=112
    let mut pc: u32 = 0x82E2F480;
    'dispatch: loop {
        match pc {
            0x82E2F480 => {
    //   block [0x82E2F480..0x82E2F4F0)
	// 82E2F480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F488: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F48C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F490: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F494: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F498: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F49C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F4A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F4A4: 388BA7A4  addi r4, r11, -0x585c
	ctx.r[4].s64 = ctx.r[11].s64 + -22620;
	// 82E2F4A8: 4BFC4561  bl 0x82df3a08
	ctx.lr = 0x82E2F4AC;
	sub_82DF3A08(ctx, base);
	// 82E2F4AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2F4B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F4B4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F4B8: 4BFFFEB9  bl 0x82e2f370
	ctx.lr = 0x82E2F4BC;
	sub_82E2F370(ctx, base);
	// 82E2F4BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2F4C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F4C4: 4BFD2F75  bl 0x82e02438
	ctx.lr = 0x82E2F4C8;
	sub_82E02438(ctx, base);
	// 82E2F4C8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F4CC: 4BFC3F5D  bl 0x82df3428
	ctx.lr = 0x82E2F4D0;
	sub_82DF3428(ctx, base);
	// 82E2F4D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F4D4: 4BFC3F55  bl 0x82df3428
	ctx.lr = 0x82E2F4D8;
	sub_82DF3428(ctx, base);
	// 82E2F4D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F4DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F4E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F4E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F4E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F4EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F4F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F4F0 size=112
    let mut pc: u32 = 0x82E2F4F0;
    'dispatch: loop {
        match pc {
            0x82E2F4F0 => {
    //   block [0x82E2F4F0..0x82E2F560)
	// 82E2F4F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F4F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F4F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F4FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F504: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F508: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F50C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F510: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F514: 388BA7A8  addi r4, r11, -0x5858
	ctx.r[4].s64 = ctx.r[11].s64 + -22616;
	// 82E2F518: 4BFC44F1  bl 0x82df3a08
	ctx.lr = 0x82E2F51C;
	sub_82DF3A08(ctx, base);
	// 82E2F51C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2F520: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F524: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F528: 4BFFFE49  bl 0x82e2f370
	ctx.lr = 0x82E2F52C;
	sub_82E2F370(ctx, base);
	// 82E2F52C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2F530: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F534: 4BFD2F05  bl 0x82e02438
	ctx.lr = 0x82E2F538;
	sub_82E02438(ctx, base);
	// 82E2F538: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F53C: 4BFC3EED  bl 0x82df3428
	ctx.lr = 0x82E2F540;
	sub_82DF3428(ctx, base);
	// 82E2F540: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F544: 4BFC3EE5  bl 0x82df3428
	ctx.lr = 0x82E2F548;
	sub_82DF3428(ctx, base);
	// 82E2F548: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F54C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F554: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F558: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F55C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F560(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F560 size=112
    let mut pc: u32 = 0x82E2F560;
    'dispatch: loop {
        match pc {
            0x82E2F560 => {
    //   block [0x82E2F560..0x82E2F5D0)
	// 82E2F560: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F564: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F568: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F56C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F570: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F574: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F578: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F57C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F580: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F584: 388BA7E8  addi r4, r11, -0x5818
	ctx.r[4].s64 = ctx.r[11].s64 + -22552;
	// 82E2F588: 4BFC4481  bl 0x82df3a08
	ctx.lr = 0x82E2F58C;
	sub_82DF3A08(ctx, base);
	// 82E2F58C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2F590: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F594: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F598: 4BFFFDD9  bl 0x82e2f370
	ctx.lr = 0x82E2F59C;
	sub_82E2F370(ctx, base);
	// 82E2F59C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2F5A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F5A4: 4BFD2E95  bl 0x82e02438
	ctx.lr = 0x82E2F5A8;
	sub_82E02438(ctx, base);
	// 82E2F5A8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F5AC: 4BFC3E7D  bl 0x82df3428
	ctx.lr = 0x82E2F5B0;
	sub_82DF3428(ctx, base);
	// 82E2F5B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F5B4: 4BFC3E75  bl 0x82df3428
	ctx.lr = 0x82E2F5B8;
	sub_82DF3428(ctx, base);
	// 82E2F5B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F5BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F5C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F5C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F5C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F5CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F5D0 size=112
    let mut pc: u32 = 0x82E2F5D0;
    'dispatch: loop {
        match pc {
            0x82E2F5D0 => {
    //   block [0x82E2F5D0..0x82E2F640)
	// 82E2F5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F5D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F5D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F5DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F5E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F5E4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F5E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F5EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F5F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F5F4: 388BA7EC  addi r4, r11, -0x5814
	ctx.r[4].s64 = ctx.r[11].s64 + -22548;
	// 82E2F5F8: 4BFC4411  bl 0x82df3a08
	ctx.lr = 0x82E2F5FC;
	sub_82DF3A08(ctx, base);
	// 82E2F5FC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2F600: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F604: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F608: 4BFFFD69  bl 0x82e2f370
	ctx.lr = 0x82E2F60C;
	sub_82E2F370(ctx, base);
	// 82E2F60C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2F610: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F614: 4BFD2E25  bl 0x82e02438
	ctx.lr = 0x82E2F618;
	sub_82E02438(ctx, base);
	// 82E2F618: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F61C: 4BFC3E0D  bl 0x82df3428
	ctx.lr = 0x82E2F620;
	sub_82DF3428(ctx, base);
	// 82E2F620: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F624: 4BFC3E05  bl 0x82df3428
	ctx.lr = 0x82E2F628;
	sub_82DF3428(ctx, base);
	// 82E2F628: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F62C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F630: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F634: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F638: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F63C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F640 size=112
    let mut pc: u32 = 0x82E2F640;
    'dispatch: loop {
        match pc {
            0x82E2F640 => {
    //   block [0x82E2F640..0x82E2F6B0)
	// 82E2F640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F644: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F648: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F64C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F650: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F654: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F658: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F65C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F660: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F664: 388BA7F0  addi r4, r11, -0x5810
	ctx.r[4].s64 = ctx.r[11].s64 + -22544;
	// 82E2F668: 4BFC43A1  bl 0x82df3a08
	ctx.lr = 0x82E2F66C;
	sub_82DF3A08(ctx, base);
	// 82E2F66C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2F670: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F674: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F678: 4BFFFCF9  bl 0x82e2f370
	ctx.lr = 0x82E2F67C;
	sub_82E2F370(ctx, base);
	// 82E2F67C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2F680: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F684: 4BFD2DB5  bl 0x82e02438
	ctx.lr = 0x82E2F688;
	sub_82E02438(ctx, base);
	// 82E2F688: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F68C: 4BFC3D9D  bl 0x82df3428
	ctx.lr = 0x82E2F690;
	sub_82DF3428(ctx, base);
	// 82E2F690: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F694: 4BFC3D95  bl 0x82df3428
	ctx.lr = 0x82E2F698;
	sub_82DF3428(ctx, base);
	// 82E2F698: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F69C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F6A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F6A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F6A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F6AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F6B0 size=112
    let mut pc: u32 = 0x82E2F6B0;
    'dispatch: loop {
        match pc {
            0x82E2F6B0 => {
    //   block [0x82E2F6B0..0x82E2F720)
	// 82E2F6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F6B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F6B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F6BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F6C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F6C4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F6C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F6CC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F6D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F6D4: 388BA7B0  addi r4, r11, -0x5850
	ctx.r[4].s64 = ctx.r[11].s64 + -22608;
	// 82E2F6D8: 4BFC4331  bl 0x82df3a08
	ctx.lr = 0x82E2F6DC;
	sub_82DF3A08(ctx, base);
	// 82E2F6DC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2F6E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F6E4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F6E8: 4BFFFC89  bl 0x82e2f370
	ctx.lr = 0x82E2F6EC;
	sub_82E2F370(ctx, base);
	// 82E2F6EC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2F6F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F6F4: 4BFD2D45  bl 0x82e02438
	ctx.lr = 0x82E2F6F8;
	sub_82E02438(ctx, base);
	// 82E2F6F8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F6FC: 4BFC3D2D  bl 0x82df3428
	ctx.lr = 0x82E2F700;
	sub_82DF3428(ctx, base);
	// 82E2F700: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F704: 4BFC3D25  bl 0x82df3428
	ctx.lr = 0x82E2F708;
	sub_82DF3428(ctx, base);
	// 82E2F708: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F714: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F718: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F71C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F720 size=112
    let mut pc: u32 = 0x82E2F720;
    'dispatch: loop {
        match pc {
            0x82E2F720 => {
    //   block [0x82E2F720..0x82E2F790)
	// 82E2F720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F728: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F72C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F730: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F734: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F738: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F73C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F740: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F744: 388BA7B4  addi r4, r11, -0x584c
	ctx.r[4].s64 = ctx.r[11].s64 + -22604;
	// 82E2F748: 4BFC42C1  bl 0x82df3a08
	ctx.lr = 0x82E2F74C;
	sub_82DF3A08(ctx, base);
	// 82E2F74C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2F750: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F754: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F758: 4BFFFC19  bl 0x82e2f370
	ctx.lr = 0x82E2F75C;
	sub_82E2F370(ctx, base);
	// 82E2F75C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2F760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F764: 4BFD2CD5  bl 0x82e02438
	ctx.lr = 0x82E2F768;
	sub_82E02438(ctx, base);
	// 82E2F768: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F76C: 4BFC3CBD  bl 0x82df3428
	ctx.lr = 0x82E2F770;
	sub_82DF3428(ctx, base);
	// 82E2F770: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F774: 4BFC3CB5  bl 0x82df3428
	ctx.lr = 0x82E2F778;
	sub_82DF3428(ctx, base);
	// 82E2F778: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F784: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F788: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F78C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F790 size=112
    let mut pc: u32 = 0x82E2F790;
    'dispatch: loop {
        match pc {
            0x82E2F790 => {
    //   block [0x82E2F790..0x82E2F800)
	// 82E2F790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F794: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F798: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F79C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F7A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F7A4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F7A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F7AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F7B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F7B4: 388BA7B8  addi r4, r11, -0x5848
	ctx.r[4].s64 = ctx.r[11].s64 + -22600;
	// 82E2F7B8: 4BFC4251  bl 0x82df3a08
	ctx.lr = 0x82E2F7BC;
	sub_82DF3A08(ctx, base);
	// 82E2F7BC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2F7C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F7C4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F7C8: 4BFFFBA9  bl 0x82e2f370
	ctx.lr = 0x82E2F7CC;
	sub_82E2F370(ctx, base);
	// 82E2F7CC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2F7D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F7D4: 4BFD2C65  bl 0x82e02438
	ctx.lr = 0x82E2F7D8;
	sub_82E02438(ctx, base);
	// 82E2F7D8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F7DC: 4BFC3C4D  bl 0x82df3428
	ctx.lr = 0x82E2F7E0;
	sub_82DF3428(ctx, base);
	// 82E2F7E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F7E4: 4BFC3C45  bl 0x82df3428
	ctx.lr = 0x82E2F7E8;
	sub_82DF3428(ctx, base);
	// 82E2F7E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F7EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F7F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F7F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F7F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F7FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F800 size=112
    let mut pc: u32 = 0x82E2F800;
    'dispatch: loop {
        match pc {
            0x82E2F800 => {
    //   block [0x82E2F800..0x82E2F870)
	// 82E2F800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F808: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F80C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F810: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F814: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F818: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F81C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F820: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F824: 388BA7C0  addi r4, r11, -0x5840
	ctx.r[4].s64 = ctx.r[11].s64 + -22592;
	// 82E2F828: 4BFC41E1  bl 0x82df3a08
	ctx.lr = 0x82E2F82C;
	sub_82DF3A08(ctx, base);
	// 82E2F82C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2F830: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F834: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F838: 4BFFFB39  bl 0x82e2f370
	ctx.lr = 0x82E2F83C;
	sub_82E2F370(ctx, base);
	// 82E2F83C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2F840: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F844: 4BFD2BF5  bl 0x82e02438
	ctx.lr = 0x82E2F848;
	sub_82E02438(ctx, base);
	// 82E2F848: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F84C: 4BFC3BDD  bl 0x82df3428
	ctx.lr = 0x82E2F850;
	sub_82DF3428(ctx, base);
	// 82E2F850: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F854: 4BFC3BD5  bl 0x82df3428
	ctx.lr = 0x82E2F858;
	sub_82DF3428(ctx, base);
	// 82E2F858: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F85C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F860: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F864: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F868: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F86C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F870 size=112
    let mut pc: u32 = 0x82E2F870;
    'dispatch: loop {
        match pc {
            0x82E2F870 => {
    //   block [0x82E2F870..0x82E2F8E0)
	// 82E2F870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F878: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F87C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F880: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F884: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F888: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F88C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F890: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F894: 388BA7C4  addi r4, r11, -0x583c
	ctx.r[4].s64 = ctx.r[11].s64 + -22588;
	// 82E2F898: 4BFC4171  bl 0x82df3a08
	ctx.lr = 0x82E2F89C;
	sub_82DF3A08(ctx, base);
	// 82E2F89C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2F8A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F8A4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F8A8: 4BFFFAC9  bl 0x82e2f370
	ctx.lr = 0x82E2F8AC;
	sub_82E2F370(ctx, base);
	// 82E2F8AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2F8B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F8B4: 4BFD2B85  bl 0x82e02438
	ctx.lr = 0x82E2F8B8;
	sub_82E02438(ctx, base);
	// 82E2F8B8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F8BC: 4BFC3B6D  bl 0x82df3428
	ctx.lr = 0x82E2F8C0;
	sub_82DF3428(ctx, base);
	// 82E2F8C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F8C4: 4BFC3B65  bl 0x82df3428
	ctx.lr = 0x82E2F8C8;
	sub_82DF3428(ctx, base);
	// 82E2F8C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F8CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F8D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F8D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F8D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F8DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F8E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F8E0 size=112
    let mut pc: u32 = 0x82E2F8E0;
    'dispatch: loop {
        match pc {
            0x82E2F8E0 => {
    //   block [0x82E2F8E0..0x82E2F950)
	// 82E2F8E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F8E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F8E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F8EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F8F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F8F4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F8F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F8FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F900: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F904: 388BA7DC  addi r4, r11, -0x5824
	ctx.r[4].s64 = ctx.r[11].s64 + -22564;
	// 82E2F908: 4BFC4101  bl 0x82df3a08
	ctx.lr = 0x82E2F90C;
	sub_82DF3A08(ctx, base);
	// 82E2F90C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2F910: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F914: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F918: 4BFFFA59  bl 0x82e2f370
	ctx.lr = 0x82E2F91C;
	sub_82E2F370(ctx, base);
	// 82E2F91C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2F920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F924: 4BFD2B15  bl 0x82e02438
	ctx.lr = 0x82E2F928;
	sub_82E02438(ctx, base);
	// 82E2F928: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F92C: 4BFC3AFD  bl 0x82df3428
	ctx.lr = 0x82E2F930;
	sub_82DF3428(ctx, base);
	// 82E2F930: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F934: 4BFC3AF5  bl 0x82df3428
	ctx.lr = 0x82E2F938;
	sub_82DF3428(ctx, base);
	// 82E2F938: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F944: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F948: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F94C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F950 size=112
    let mut pc: u32 = 0x82E2F950;
    'dispatch: loop {
        match pc {
            0x82E2F950 => {
    //   block [0x82E2F950..0x82E2F9C0)
	// 82E2F950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F958: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F95C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F960: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F964: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F968: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F96C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F970: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F974: 388BA7E0  addi r4, r11, -0x5820
	ctx.r[4].s64 = ctx.r[11].s64 + -22560;
	// 82E2F978: 4BFC4091  bl 0x82df3a08
	ctx.lr = 0x82E2F97C;
	sub_82DF3A08(ctx, base);
	// 82E2F97C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2F980: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F984: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F988: 4BFFF9E9  bl 0x82e2f370
	ctx.lr = 0x82E2F98C;
	sub_82E2F370(ctx, base);
	// 82E2F98C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2F990: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2F994: 4BFD2AA5  bl 0x82e02438
	ctx.lr = 0x82E2F998;
	sub_82E02438(ctx, base);
	// 82E2F998: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F99C: 4BFC3A8D  bl 0x82df3428
	ctx.lr = 0x82E2F9A0;
	sub_82DF3428(ctx, base);
	// 82E2F9A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F9A4: 4BFC3A85  bl 0x82df3428
	ctx.lr = 0x82E2F9A8;
	sub_82DF3428(ctx, base);
	// 82E2F9A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2F9AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2F9B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2F9B4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2F9B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2F9BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2F9C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2F9C0 size=112
    let mut pc: u32 = 0x82E2F9C0;
    'dispatch: loop {
        match pc {
            0x82E2F9C0 => {
    //   block [0x82E2F9C0..0x82E2FA30)
	// 82E2F9C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2F9C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2F9C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2F9CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2F9D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2F9D4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2F9D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2F9DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2F9E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2F9E4: 388BA7AC  addi r4, r11, -0x5854
	ctx.r[4].s64 = ctx.r[11].s64 + -22612;
	// 82E2F9E8: 4BFC4021  bl 0x82df3a08
	ctx.lr = 0x82E2F9EC;
	sub_82DF3A08(ctx, base);
	// 82E2F9EC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2F9F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2F9F4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2F9F8: 4BFFF979  bl 0x82e2f370
	ctx.lr = 0x82E2F9FC;
	sub_82E2F370(ctx, base);
	// 82E2F9FC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2FA00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2FA04: 4BFD2A35  bl 0x82e02438
	ctx.lr = 0x82E2FA08;
	sub_82E02438(ctx, base);
	// 82E2FA08: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FA0C: 4BFC3A1D  bl 0x82df3428
	ctx.lr = 0x82E2FA10;
	sub_82DF3428(ctx, base);
	// 82E2FA10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FA14: 4BFC3A15  bl 0x82df3428
	ctx.lr = 0x82E2FA18;
	sub_82DF3428(ctx, base);
	// 82E2FA18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2FA1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2FA20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2FA24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2FA28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2FA2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2FA30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2FA30 size=112
    let mut pc: u32 = 0x82E2FA30;
    'dispatch: loop {
        match pc {
            0x82E2FA30 => {
    //   block [0x82E2FA30..0x82E2FAA0)
	// 82E2FA30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2FA34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2FA38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2FA3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2FA40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2FA44: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2FA48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2FA4C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2FA50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FA54: 388BA798  addi r4, r11, -0x5868
	ctx.r[4].s64 = ctx.r[11].s64 + -22632;
	// 82E2FA58: 4BFC3FB1  bl 0x82df3a08
	ctx.lr = 0x82E2FA5C;
	sub_82DF3A08(ctx, base);
	// 82E2FA5C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2FA60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2FA64: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FA68: 4BFFF909  bl 0x82e2f370
	ctx.lr = 0x82E2FA6C;
	sub_82E2F370(ctx, base);
	// 82E2FA6C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2FA70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2FA74: 4BFD29C5  bl 0x82e02438
	ctx.lr = 0x82E2FA78;
	sub_82E02438(ctx, base);
	// 82E2FA78: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FA7C: 4BFC39AD  bl 0x82df3428
	ctx.lr = 0x82E2FA80;
	sub_82DF3428(ctx, base);
	// 82E2FA80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FA84: 4BFC39A5  bl 0x82df3428
	ctx.lr = 0x82E2FA88;
	sub_82DF3428(ctx, base);
	// 82E2FA88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2FA8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2FA90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2FA94: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2FA98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2FA9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2FAA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2FAA0 size=112
    let mut pc: u32 = 0x82E2FAA0;
    'dispatch: loop {
        match pc {
            0x82E2FAA0 => {
    //   block [0x82E2FAA0..0x82E2FB10)
	// 82E2FAA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2FAA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2FAA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2FAAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2FAB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2FAB4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2FAB8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2FABC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2FAC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FAC4: 388BA79C  addi r4, r11, -0x5864
	ctx.r[4].s64 = ctx.r[11].s64 + -22628;
	// 82E2FAC8: 4BFC3F41  bl 0x82df3a08
	ctx.lr = 0x82E2FACC;
	sub_82DF3A08(ctx, base);
	// 82E2FACC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2FAD0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2FAD4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FAD8: 4BFFF899  bl 0x82e2f370
	ctx.lr = 0x82E2FADC;
	sub_82E2F370(ctx, base);
	// 82E2FADC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2FAE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2FAE4: 4BFD2955  bl 0x82e02438
	ctx.lr = 0x82E2FAE8;
	sub_82E02438(ctx, base);
	// 82E2FAE8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FAEC: 4BFC393D  bl 0x82df3428
	ctx.lr = 0x82E2FAF0;
	sub_82DF3428(ctx, base);
	// 82E2FAF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FAF4: 4BFC3935  bl 0x82df3428
	ctx.lr = 0x82E2FAF8;
	sub_82DF3428(ctx, base);
	// 82E2FAF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2FAFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2FB00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2FB04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2FB08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2FB0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2FB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2FB10 size=112
    let mut pc: u32 = 0x82E2FB10;
    'dispatch: loop {
        match pc {
            0x82E2FB10 => {
    //   block [0x82E2FB10..0x82E2FB80)
	// 82E2FB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2FB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2FB18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2FB1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2FB20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2FB24: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2FB28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2FB2C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2FB30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FB34: 388BA7A0  addi r4, r11, -0x5860
	ctx.r[4].s64 = ctx.r[11].s64 + -22624;
	// 82E2FB38: 4BFC3ED1  bl 0x82df3a08
	ctx.lr = 0x82E2FB3C;
	sub_82DF3A08(ctx, base);
	// 82E2FB3C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2FB40: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2FB44: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FB48: 4BFFF829  bl 0x82e2f370
	ctx.lr = 0x82E2FB4C;
	sub_82E2F370(ctx, base);
	// 82E2FB4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2FB50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2FB54: 4BFD28E5  bl 0x82e02438
	ctx.lr = 0x82E2FB58;
	sub_82E02438(ctx, base);
	// 82E2FB58: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FB5C: 4BFC38CD  bl 0x82df3428
	ctx.lr = 0x82E2FB60;
	sub_82DF3428(ctx, base);
	// 82E2FB60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FB64: 4BFC38C5  bl 0x82df3428
	ctx.lr = 0x82E2FB68;
	sub_82DF3428(ctx, base);
	// 82E2FB68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2FB6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2FB70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2FB74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2FB78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2FB7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2FB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2FB80 size=112
    let mut pc: u32 = 0x82E2FB80;
    'dispatch: loop {
        match pc {
            0x82E2FB80 => {
    //   block [0x82E2FB80..0x82E2FBF0)
	// 82E2FB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2FB84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2FB88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2FB8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2FB90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2FB94: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2FB98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2FB9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2FBA0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FBA4: 388BA7C8  addi r4, r11, -0x5838
	ctx.r[4].s64 = ctx.r[11].s64 + -22584;
	// 82E2FBA8: 4BFC3E61  bl 0x82df3a08
	ctx.lr = 0x82E2FBAC;
	sub_82DF3A08(ctx, base);
	// 82E2FBAC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2FBB0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2FBB4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FBB8: 4BFFF7B9  bl 0x82e2f370
	ctx.lr = 0x82E2FBBC;
	sub_82E2F370(ctx, base);
	// 82E2FBBC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2FBC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2FBC4: 4BFD2875  bl 0x82e02438
	ctx.lr = 0x82E2FBC8;
	sub_82E02438(ctx, base);
	// 82E2FBC8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FBCC: 4BFC385D  bl 0x82df3428
	ctx.lr = 0x82E2FBD0;
	sub_82DF3428(ctx, base);
	// 82E2FBD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FBD4: 4BFC3855  bl 0x82df3428
	ctx.lr = 0x82E2FBD8;
	sub_82DF3428(ctx, base);
	// 82E2FBD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2FBDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2FBE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2FBE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2FBE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2FBEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2FBF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2FBF0 size=112
    let mut pc: u32 = 0x82E2FBF0;
    'dispatch: loop {
        match pc {
            0x82E2FBF0 => {
    //   block [0x82E2FBF0..0x82E2FC60)
	// 82E2FBF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2FBF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2FBF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2FBFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2FC00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2FC04: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2FC08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2FC0C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2FC10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FC14: 388BA7CC  addi r4, r11, -0x5834
	ctx.r[4].s64 = ctx.r[11].s64 + -22580;
	// 82E2FC18: 4BFC3DF1  bl 0x82df3a08
	ctx.lr = 0x82E2FC1C;
	sub_82DF3A08(ctx, base);
	// 82E2FC1C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2FC20: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2FC24: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FC28: 4BFFF749  bl 0x82e2f370
	ctx.lr = 0x82E2FC2C;
	sub_82E2F370(ctx, base);
	// 82E2FC2C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2FC30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2FC34: 4BFD2805  bl 0x82e02438
	ctx.lr = 0x82E2FC38;
	sub_82E02438(ctx, base);
	// 82E2FC38: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FC3C: 4BFC37ED  bl 0x82df3428
	ctx.lr = 0x82E2FC40;
	sub_82DF3428(ctx, base);
	// 82E2FC40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FC44: 4BFC37E5  bl 0x82df3428
	ctx.lr = 0x82E2FC48;
	sub_82DF3428(ctx, base);
	// 82E2FC48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2FC4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2FC50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2FC54: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2FC58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2FC5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2FC60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2FC60 size=112
    let mut pc: u32 = 0x82E2FC60;
    'dispatch: loop {
        match pc {
            0x82E2FC60 => {
    //   block [0x82E2FC60..0x82E2FCD0)
	// 82E2FC60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2FC64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2FC68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2FC6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2FC70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2FC74: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2FC78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2FC7C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2FC80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FC84: 388BA7D0  addi r4, r11, -0x5830
	ctx.r[4].s64 = ctx.r[11].s64 + -22576;
	// 82E2FC88: 4BFC3D81  bl 0x82df3a08
	ctx.lr = 0x82E2FC8C;
	sub_82DF3A08(ctx, base);
	// 82E2FC8C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2FC90: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2FC94: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FC98: 4BFFF6D9  bl 0x82e2f370
	ctx.lr = 0x82E2FC9C;
	sub_82E2F370(ctx, base);
	// 82E2FC9C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2FCA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2FCA4: 4BFD2795  bl 0x82e02438
	ctx.lr = 0x82E2FCA8;
	sub_82E02438(ctx, base);
	// 82E2FCA8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FCAC: 4BFC377D  bl 0x82df3428
	ctx.lr = 0x82E2FCB0;
	sub_82DF3428(ctx, base);
	// 82E2FCB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FCB4: 4BFC3775  bl 0x82df3428
	ctx.lr = 0x82E2FCB8;
	sub_82DF3428(ctx, base);
	// 82E2FCB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2FCBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2FCC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2FCC4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2FCC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2FCCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2FCD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2FCD0 size=112
    let mut pc: u32 = 0x82E2FCD0;
    'dispatch: loop {
        match pc {
            0x82E2FCD0 => {
    //   block [0x82E2FCD0..0x82E2FD40)
	// 82E2FCD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2FCD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2FCD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2FCDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2FCE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2FCE4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2FCE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2FCEC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2FCF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FCF4: 388BA7D4  addi r4, r11, -0x582c
	ctx.r[4].s64 = ctx.r[11].s64 + -22572;
	// 82E2FCF8: 4BFC3D11  bl 0x82df3a08
	ctx.lr = 0x82E2FCFC;
	sub_82DF3A08(ctx, base);
	// 82E2FCFC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2FD00: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2FD04: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FD08: 4BFFF669  bl 0x82e2f370
	ctx.lr = 0x82E2FD0C;
	sub_82E2F370(ctx, base);
	// 82E2FD0C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2FD10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2FD14: 4BFD2725  bl 0x82e02438
	ctx.lr = 0x82E2FD18;
	sub_82E02438(ctx, base);
	// 82E2FD18: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FD1C: 4BFC370D  bl 0x82df3428
	ctx.lr = 0x82E2FD20;
	sub_82DF3428(ctx, base);
	// 82E2FD20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FD24: 4BFC3705  bl 0x82df3428
	ctx.lr = 0x82E2FD28;
	sub_82DF3428(ctx, base);
	// 82E2FD28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2FD2C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2FD30: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2FD34: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2FD38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2FD3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2FD40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2FD40 size=112
    let mut pc: u32 = 0x82E2FD40;
    'dispatch: loop {
        match pc {
            0x82E2FD40 => {
    //   block [0x82E2FD40..0x82E2FDB0)
	// 82E2FD40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2FD44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2FD48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2FD4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2FD50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2FD54: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2FD58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2FD5C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2FD60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FD64: 388BA7D8  addi r4, r11, -0x5828
	ctx.r[4].s64 = ctx.r[11].s64 + -22568;
	// 82E2FD68: 4BFC3CA1  bl 0x82df3a08
	ctx.lr = 0x82E2FD6C;
	sub_82DF3A08(ctx, base);
	// 82E2FD6C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2FD70: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2FD74: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FD78: 4BFFF5F9  bl 0x82e2f370
	ctx.lr = 0x82E2FD7C;
	sub_82E2F370(ctx, base);
	// 82E2FD7C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2FD80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2FD84: 4BFD26B5  bl 0x82e02438
	ctx.lr = 0x82E2FD88;
	sub_82E02438(ctx, base);
	// 82E2FD88: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FD8C: 4BFC369D  bl 0x82df3428
	ctx.lr = 0x82E2FD90;
	sub_82DF3428(ctx, base);
	// 82E2FD90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FD94: 4BFC3695  bl 0x82df3428
	ctx.lr = 0x82E2FD98;
	sub_82DF3428(ctx, base);
	// 82E2FD98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2FD9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2FDA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2FDA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2FDA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2FDAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2FDB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2FDB0 size=112
    let mut pc: u32 = 0x82E2FDB0;
    'dispatch: loop {
        match pc {
            0x82E2FDB0 => {
    //   block [0x82E2FDB0..0x82E2FE20)
	// 82E2FDB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2FDB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2FDB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2FDBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2FDC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2FDC4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2FDC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2FDCC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2FDD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FDD4: 388BA7F8  addi r4, r11, -0x5808
	ctx.r[4].s64 = ctx.r[11].s64 + -22536;
	// 82E2FDD8: 4BFC3C31  bl 0x82df3a08
	ctx.lr = 0x82E2FDDC;
	sub_82DF3A08(ctx, base);
	// 82E2FDDC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2FDE0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2FDE4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FDE8: 4BFFF589  bl 0x82e2f370
	ctx.lr = 0x82E2FDEC;
	sub_82E2F370(ctx, base);
	// 82E2FDEC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2FDF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2FDF4: 4BFD2645  bl 0x82e02438
	ctx.lr = 0x82E2FDF8;
	sub_82E02438(ctx, base);
	// 82E2FDF8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FDFC: 4BFC362D  bl 0x82df3428
	ctx.lr = 0x82E2FE00;
	sub_82DF3428(ctx, base);
	// 82E2FE00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FE04: 4BFC3625  bl 0x82df3428
	ctx.lr = 0x82E2FE08;
	sub_82DF3428(ctx, base);
	// 82E2FE08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2FE0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2FE10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2FE14: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2FE18: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2FE1C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2FE20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2FE20 size=112
    let mut pc: u32 = 0x82E2FE20;
    'dispatch: loop {
        match pc {
            0x82E2FE20 => {
    //   block [0x82E2FE20..0x82E2FE90)
	// 82E2FE20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2FE24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2FE28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2FE2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2FE30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2FE34: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2FE38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2FE3C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2FE40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FE44: 388BA7FC  addi r4, r11, -0x5804
	ctx.r[4].s64 = ctx.r[11].s64 + -22532;
	// 82E2FE48: 4BFC3BC1  bl 0x82df3a08
	ctx.lr = 0x82E2FE4C;
	sub_82DF3A08(ctx, base);
	// 82E2FE4C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2FE50: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2FE54: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FE58: 4BFFF519  bl 0x82e2f370
	ctx.lr = 0x82E2FE5C;
	sub_82E2F370(ctx, base);
	// 82E2FE5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2FE60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2FE64: 4BFD25D5  bl 0x82e02438
	ctx.lr = 0x82E2FE68;
	sub_82E02438(ctx, base);
	// 82E2FE68: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FE6C: 4BFC35BD  bl 0x82df3428
	ctx.lr = 0x82E2FE70;
	sub_82DF3428(ctx, base);
	// 82E2FE70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FE74: 4BFC35B5  bl 0x82df3428
	ctx.lr = 0x82E2FE78;
	sub_82DF3428(ctx, base);
	// 82E2FE78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2FE7C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2FE80: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2FE84: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2FE88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2FE8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2FE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2FE90 size=112
    let mut pc: u32 = 0x82E2FE90;
    'dispatch: loop {
        match pc {
            0x82E2FE90 => {
    //   block [0x82E2FE90..0x82E2FF00)
	// 82E2FE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2FE94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2FE98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2FE9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2FEA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2FEA4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2FEA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2FEAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2FEB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FEB4: 388BA800  addi r4, r11, -0x5800
	ctx.r[4].s64 = ctx.r[11].s64 + -22528;
	// 82E2FEB8: 4BFC3B51  bl 0x82df3a08
	ctx.lr = 0x82E2FEBC;
	sub_82DF3A08(ctx, base);
	// 82E2FEBC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2FEC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2FEC4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FEC8: 4BFFF4A9  bl 0x82e2f370
	ctx.lr = 0x82E2FECC;
	sub_82E2F370(ctx, base);
	// 82E2FECC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2FED0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2FED4: 4BFD2565  bl 0x82e02438
	ctx.lr = 0x82E2FED8;
	sub_82E02438(ctx, base);
	// 82E2FED8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FEDC: 4BFC354D  bl 0x82df3428
	ctx.lr = 0x82E2FEE0;
	sub_82DF3428(ctx, base);
	// 82E2FEE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FEE4: 4BFC3545  bl 0x82df3428
	ctx.lr = 0x82E2FEE8;
	sub_82DF3428(ctx, base);
	// 82E2FEE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2FEEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2FEF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2FEF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2FEF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2FEFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2FF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2FF00 size=112
    let mut pc: u32 = 0x82E2FF00;
    'dispatch: loop {
        match pc {
            0x82E2FF00 => {
    //   block [0x82E2FF00..0x82E2FF70)
	// 82E2FF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2FF04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2FF08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2FF0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2FF10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2FF14: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2FF18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E2FF1C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E2FF20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FF24: 388BA804  addi r4, r11, -0x57fc
	ctx.r[4].s64 = ctx.r[11].s64 + -22524;
	// 82E2FF28: 4BFC3AE1  bl 0x82df3a08
	ctx.lr = 0x82E2FF2C;
	sub_82DF3A08(ctx, base);
	// 82E2FF2C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E2FF30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E2FF34: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FF38: 4BFFF439  bl 0x82e2f370
	ctx.lr = 0x82E2FF3C;
	sub_82E2F370(ctx, base);
	// 82E2FF3C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E2FF40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E2FF44: 4BFD24F5  bl 0x82e02438
	ctx.lr = 0x82E2FF48;
	sub_82E02438(ctx, base);
	// 82E2FF48: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E2FF4C: 4BFC34DD  bl 0x82df3428
	ctx.lr = 0x82E2FF50;
	sub_82DF3428(ctx, base);
	// 82E2FF50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E2FF54: 4BFC34D5  bl 0x82df3428
	ctx.lr = 0x82E2FF58;
	sub_82DF3428(ctx, base);
	// 82E2FF58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2FF5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2FF60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2FF64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2FF68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2FF6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2FF70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2FF70 size=136
    let mut pc: u32 = 0x82E2FF70;
    'dispatch: loop {
        match pc {
            0x82E2FF70 => {
    //   block [0x82E2FF70..0x82E2FFF8)
	// 82E2FF70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2FF74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E2FF78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E2FF7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E2FF80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E2FF84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E2FF88: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E2FF8C: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E2FF90: 409A0020  bne cr6, 0x82e2ffb0
	if !ctx.cr[6].eq {
	pc = 0x82E2FFB0; continue 'dispatch;
	}
	// 82E2FF94: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E2FF98: 419A0048  beq cr6, 0x82e2ffe0
	if ctx.cr[6].eq {
	pc = 0x82E2FFE0; continue 'dispatch;
	}
	// 82E2FF9C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2FFA0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2FFA4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E2FFA8: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E2FFAC: 48000034  b 0x82e2ffe0
	pc = 0x82E2FFE0; continue 'dispatch;
	// 82E2FFB0: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82E2FFB4: 419A002C  beq cr6, 0x82e2ffe0
	if ctx.cr[6].eq {
	pc = 0x82E2FFE0; continue 'dispatch;
	}
	// 82E2FFB8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E2FFBC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E2FFC0: 388BA910  addi r4, r11, -0x56f0
	ctx.r[4].s64 = ctx.r[11].s64 + -22256;
	// 82E2FFC4: 48378135  bl 0x831a80f8
	ctx.lr = 0x82E2FFC8;
	sub_831A80F8(ctx, base);
	// 82E2FFC8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E2FFCC: 4182000C  beq 0x82e2ffd8
	if ctx.cr[0].eq {
	pc = 0x82E2FFD8; continue 'dispatch;
	}
	// 82E2FFD0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E2FFD4: 4800000C  b 0x82e2ffe0
	pc = 0x82E2FFE0; continue 'dispatch;
	// 82E2FFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E2FFDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E2FFE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E2FFE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E2FFE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E2FFEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E2FFF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E2FFF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E2FFF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E2FFF8 size=144
    let mut pc: u32 = 0x82E2FFF8;
    'dispatch: loop {
        match pc {
            0x82E2FFF8 => {
    //   block [0x82E2FFF8..0x82E30088)
	// 82E2FFF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E2FFFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E30000: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E30004: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E30008: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3000C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E30010: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30014: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E30018: 409A0028  bne cr6, 0x82e30040
	if !ctx.cr[6].eq {
	pc = 0x82E30040; continue 'dispatch;
	}
	// 82E3001C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E30020: 419A0050  beq cr6, 0x82e30070
	if ctx.cr[6].eq {
	pc = 0x82E30070; continue 'dispatch;
	}
	// 82E30024: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E30028: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E3002C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E30030: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E30034: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E30038: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E3003C: 48000034  b 0x82e30070
	pc = 0x82E30070; continue 'dispatch;
	// 82E30040: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82E30044: 419A002C  beq cr6, 0x82e30070
	if ctx.cr[6].eq {
	pc = 0x82E30070; continue 'dispatch;
	}
	// 82E30048: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E3004C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E30050: 388BAA30  addi r4, r11, -0x55d0
	ctx.r[4].s64 = ctx.r[11].s64 + -21968;
	// 82E30054: 483780A5  bl 0x831a80f8
	ctx.lr = 0x82E30058;
	sub_831A80F8(ctx, base);
	// 82E30058: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3005C: 4182000C  beq 0x82e30068
	if ctx.cr[0].eq {
	pc = 0x82E30068; continue 'dispatch;
	}
	// 82E30060: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E30064: 4800000C  b 0x82e30070
	pc = 0x82E30070; continue 'dispatch;
	// 82E30068: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E3006C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E30070: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E30074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E30078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3007C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E30080: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E30084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30088 size=152
    let mut pc: u32 = 0x82E30088;
    'dispatch: loop {
        match pc {
            0x82E30088 => {
    //   block [0x82E30088..0x82E30120)
	// 82E30088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3008C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E30090: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E30094: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E30098: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3009C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E300A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E300A4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 82E300A8: 409A0030  bne cr6, 0x82e300d8
	if !ctx.cr[6].eq {
	pc = 0x82E300D8; continue 'dispatch;
	}
	// 82E300AC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E300B0: 419A0058  beq cr6, 0x82e30108
	if ctx.cr[6].eq {
	pc = 0x82E30108; continue 'dispatch;
	}
	// 82E300B4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E300B8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E300BC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E300C0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82E300C4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E300C8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82E300CC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E300D0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E300D4: 48000034  b 0x82e30108
	pc = 0x82E30108; continue 'dispatch;
	// 82E300D8: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 82E300DC: 419A002C  beq cr6, 0x82e30108
	if ctx.cr[6].eq {
	pc = 0x82E30108; continue 'dispatch;
	}
	// 82E300E0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E300E4: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E300E8: 388BAB60  addi r4, r11, -0x54a0
	ctx.r[4].s64 = ctx.r[11].s64 + -21664;
	// 82E300EC: 4837800D  bl 0x831a80f8
	ctx.lr = 0x82E300F0;
	sub_831A80F8(ctx, base);
	// 82E300F0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E300F4: 4182000C  beq 0x82e30100
	if ctx.cr[0].eq {
	pc = 0x82E30100; continue 'dispatch;
	}
	// 82E300F8: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82E300FC: 4800000C  b 0x82e30108
	pc = 0x82E30108; continue 'dispatch;
	// 82E30100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E30104: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E30108: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E3010C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E30110: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E30114: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E30118: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E3011C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30120 size=108
    let mut pc: u32 = 0x82E30120;
    'dispatch: loop {
        match pc {
            0x82E30120 => {
    //   block [0x82E30120..0x82E3018C)
	// 82E30120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E30128: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3012C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E30130: F8810080  std r4, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u64 ) };
	// 82E30134: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E30138: F8A10088  std r5, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[5].u64 ) };
	// 82E3013C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E30140: 4B99F099  bl 0x827cf1d8
	ctx.lr = 0x82E30144;
	sub_827CF1D8(ctx, base);
	// 82E30144: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E30148: 4082002C  bne 0x82e30174
	if !ctx.cr[0].eq {
	pc = 0x82E30174; continue 'dispatch;
	}
	// 82E3014C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E30150: 419A001C  beq cr6, 0x82e3016c
	if ctx.cr[6].eq {
	pc = 0x82E3016C; continue 'dispatch;
	}
	// 82E30154: 81610080  lwz r11, 0x80(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 82E30158: 81410084  lwz r10, 0x84(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 82E3015C: 81210088  lwz r9, 0x88(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 82E30160: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E30164: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E30168: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82E3016C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E30170: 48000008  b 0x82e30178
	pc = 0x82E30178; continue 'dispatch;
	// 82E30174: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E30178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E3017C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E30180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E30184: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E30188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30190 size=120
    let mut pc: u32 = 0x82E30190;
    'dispatch: loop {
        match pc {
            0x82E30190 => {
    //   block [0x82E30190..0x82E30208)
	// 82E30190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E30198: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E3019C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E301A0: F8810080  std r4, 0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u64 ) };
	// 82E301A4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82E301A8: F8A10088  std r5, 0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[5].u64 ) };
	// 82E301AC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82E301B0: 4B99F029  bl 0x827cf1d8
	ctx.lr = 0x82E301B4;
	sub_827CF1D8(ctx, base);
	// 82E301B4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E301B8: 40820038  bne 0x82e301f0
	if !ctx.cr[0].eq {
	pc = 0x82E301F0; continue 'dispatch;
	}
	// 82E301BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E301C0: 419A0028  beq cr6, 0x82e301e8
	if ctx.cr[6].eq {
	pc = 0x82E301E8; continue 'dispatch;
	}
	// 82E301C4: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82E301C8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E301CC: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E301D0: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E301D4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E301D8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82E301DC: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E301E0: 911F0008  stw r8, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82E301E4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E301E8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82E301EC: 48000008  b 0x82e301f4
	pc = 0x82E301F4; continue 'dispatch;
	// 82E301F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E301F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E301F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E301FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E30200: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E30204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30208 size=328
    let mut pc: u32 = 0x82E30208;
    'dispatch: loop {
        match pc {
            0x82E30208 => {
    //   block [0x82E30208..0x82E30350)
	// 82E30208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3020C: 48377F61  bl 0x831a816c
	ctx.lr = 0x82E30210;
	sub_831A8130(ctx, base);
	// 82E30210: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E30214: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E30218: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3021C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30220: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E30224: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30228: 4BFFA8C1  bl 0x82e2aae8
	ctx.lr = 0x82E3022C;
	sub_82E2AAE8(ctx, base);
	// 82E3022C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30230: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E30234: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30238: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E3023C: 4BFFEC35  bl 0x82e2ee70
	ctx.lr = 0x82E30240;
	sub_82E2EE70(ctx, base);
	// 82E30240: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E30244: 419A00EC  beq cr6, 0x82e30330
	if ctx.cr[6].eq {
	pc = 0x82E30330; continue 'dispatch;
	}
	// 82E30248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3024C: 4BFDE25D  bl 0x82e0e4a8
	ctx.lr = 0x82E30250;
	sub_82E0E4A8(ctx, base);
	// 82E30250: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E30254: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E30258: 419A00C0  beq cr6, 0x82e30318
	if ctx.cr[6].eq {
	pc = 0x82E30318; continue 'dispatch;
	}
	// 82E3025C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82E30260: 419A009C  beq cr6, 0x82e302fc
	if ctx.cr[6].eq {
	pc = 0x82E302FC; continue 'dispatch;
	}
	// 82E30264: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82E30268: 419A0078  beq cr6, 0x82e302e0
	if ctx.cr[6].eq {
	pc = 0x82E302E0; continue 'dispatch;
	}
	// 82E3026C: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82E30270: 419A0054  beq cr6, 0x82e302c4
	if ctx.cr[6].eq {
	pc = 0x82E302C4; continue 'dispatch;
	}
	// 82E30274: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 82E30278: 419A0030  beq cr6, 0x82e302a8
	if ctx.cr[6].eq {
	pc = 0x82E302A8; continue 'dispatch;
	}
	// 82E3027C: 2B0B03E9  cmplwi cr6, r11, 0x3e9
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1001 as u32, &mut ctx.xer);
	// 82E30280: 4099000C  ble cr6, 0x82e3028c
	if !ctx.cr[6].gt {
	pc = 0x82E3028C; continue 'dispatch;
	}
	// 82E30284: 2B0B03ED  cmplwi cr6, r11, 0x3ed
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1005 as u32, &mut ctx.xer);
	// 82E30288: 409900A8  ble cr6, 0x82e30330
	if !ctx.cr[6].gt {
	pc = 0x82E30330; continue 'dispatch;
	}
	// 82E3028C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30290: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30294: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E30298: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3029C: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E302A0: 48006E51  bl 0x82e370f0
	ctx.lr = 0x82E302A4;
	sub_82E370F0(ctx, base);
	// 82E302A4: 4800008C  b 0x82e30330
	pc = 0x82E30330; continue 'dispatch;
	// 82E302A8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E302AC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E302B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E302B4: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E302B8: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E302BC: 480084ED  bl 0x82e387a8
	ctx.lr = 0x82E302C0;
	sub_82E387A8(ctx, base);
	// 82E302C0: 48000070  b 0x82e30330
	pc = 0x82E30330; continue 'dispatch;
	// 82E302C4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E302C8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E302CC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E302D0: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E302D4: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E302D8: 48007ED1  bl 0x82e381a8
	ctx.lr = 0x82E302DC;
	sub_82E381A8(ctx, base);
	// 82E302DC: 48000054  b 0x82e30330
	pc = 0x82E30330; continue 'dispatch;
	// 82E302E0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E302E4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E302E8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E302EC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E302F0: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E302F4: 480078B5  bl 0x82e37ba8
	ctx.lr = 0x82E302F8;
	sub_82E37BA8(ctx, base);
	// 82E302F8: 48000038  b 0x82e30330
	pc = 0x82E30330; continue 'dispatch;
	// 82E302FC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30300: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30304: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E30308: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3030C: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E30310: 48007339  bl 0x82e37648
	ctx.lr = 0x82E30314;
	sub_82E37648(ctx, base);
	// 82E30314: 4800001C  b 0x82e30330
	pc = 0x82E30330; continue 'dispatch;
	// 82E30318: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3031C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30320: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E30324: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E30328: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E3032C: 48006DC5  bl 0x82e370f0
	ctx.lr = 0x82E30330;
	sub_82E370F0(ctx, base);
	// 82E30330: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E30334: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30338: 419A0008  beq cr6, 0x82e30340
	if ctx.cr[6].eq {
	pc = 0x82E30340; continue 'dispatch;
	}
	// 82E3033C: 4B490555  bl 0x822c0890
	ctx.lr = 0x82E30340;
	sub_822C0890(ctx, base);
	// 82E30340: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30344: 4BFFA7BD  bl 0x82e2ab00
	ctx.lr = 0x82E30348;
	sub_82E2AB00(ctx, base);
	// 82E30348: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E3034C: 48377E70  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30350 size=172
    let mut pc: u32 = 0x82E30350;
    'dispatch: loop {
        match pc {
            0x82E30350 => {
    //   block [0x82E30350..0x82E303FC)
	// 82E30350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30354: 48377E19  bl 0x831a816c
	ctx.lr = 0x82E30358;
	sub_831A8130(ctx, base);
	// 82E30358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3035C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E30360: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30364: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30368: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3036C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30370: 4BFFA779  bl 0x82e2aae8
	ctx.lr = 0x82E30374;
	sub_82E2AAE8(ctx, base);
	// 82E30374: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30378: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E3037C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30380: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30384: 4BFFE5DD  bl 0x82e2e960
	ctx.lr = 0x82E30388;
	sub_82E2E960(ctx, base);
	// 82E30388: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E3038C: 419A0050  beq cr6, 0x82e303dc
	if ctx.cr[6].eq {
	pc = 0x82E303DC; continue 'dispatch;
	}
	// 82E30390: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E30394: 4BFDE115  bl 0x82e0e4a8
	ctx.lr = 0x82E30398;
	sub_82E0E4A8(ctx, base);
	// 82E30398: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3039C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E303A0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E303A4: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82E303A8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E303AC: 409A0014  bne cr6, 0x82e303c0
	if !ctx.cr[6].eq {
	pc = 0x82E303C0; continue 'dispatch;
	}
	// 82E303B0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E303B4: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E303B8: 480089F9  bl 0x82e38db0
	ctx.lr = 0x82E303BC;
	sub_82E38DB0(ctx, base);
	// 82E303BC: 48000020  b 0x82e303dc
	pc = 0x82E303DC; continue 'dispatch;
	// 82E303C0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E303C4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E303C8: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E303CC: 409A000C  bne cr6, 0x82e303d8
	if !ctx.cr[6].eq {
	pc = 0x82E303D8; continue 'dispatch;
	}
	// 82E303D0: 480088D1  bl 0x82e38ca0
	ctx.lr = 0x82E303D4;
	sub_82E38CA0(ctx, base);
	// 82E303D4: 48000008  b 0x82e303dc
	pc = 0x82E303DC; continue 'dispatch;
	// 82E303D8: 48008949  bl 0x82e38d20
	ctx.lr = 0x82E303DC;
	sub_82E38D20(ctx, base);
	// 82E303DC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E303E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E303E4: 419A0008  beq cr6, 0x82e303ec
	if ctx.cr[6].eq {
	pc = 0x82E303EC; continue 'dispatch;
	}
	// 82E303E8: 4B4904A9  bl 0x822c0890
	ctx.lr = 0x82E303EC;
	sub_822C0890(ctx, base);
	// 82E303EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E303F0: 4BFFA711  bl 0x82e2ab00
	ctx.lr = 0x82E303F4;
	sub_82E2AB00(ctx, base);
	// 82E303F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E303F8: 48377DC4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30400 size=172
    let mut pc: u32 = 0x82E30400;
    'dispatch: loop {
        match pc {
            0x82E30400 => {
    //   block [0x82E30400..0x82E304AC)
	// 82E30400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30404: 48377D69  bl 0x831a816c
	ctx.lr = 0x82E30408;
	sub_831A8130(ctx, base);
	// 82E30408: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3040C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E30410: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30414: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30418: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3041C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30420: 4BFFA6C9  bl 0x82e2aae8
	ctx.lr = 0x82E30424;
	sub_82E2AAE8(ctx, base);
	// 82E30424: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30428: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E3042C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30430: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30434: 4BFFE3DD  bl 0x82e2e810
	ctx.lr = 0x82E30438;
	sub_82E2E810(ctx, base);
	// 82E30438: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E3043C: 419A0050  beq cr6, 0x82e3048c
	if ctx.cr[6].eq {
	pc = 0x82E3048C; continue 'dispatch;
	}
	// 82E30440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E30444: 4BFDE065  bl 0x82e0e4a8
	ctx.lr = 0x82E30448;
	sub_82E0E4A8(ctx, base);
	// 82E30448: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3044C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E30450: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30454: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E30458: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E3045C: 409A0014  bne cr6, 0x82e30470
	if !ctx.cr[6].eq {
	pc = 0x82E30470; continue 'dispatch;
	}
	// 82E30460: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30464: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E30468: 48008A41  bl 0x82e38ea8
	ctx.lr = 0x82E3046C;
	sub_82E38EA8(ctx, base);
	// 82E3046C: 48000020  b 0x82e3048c
	pc = 0x82E3048C; continue 'dispatch;
	// 82E30470: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82E30474: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30478: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E3047C: 409A000C  bne cr6, 0x82e30488
	if !ctx.cr[6].eq {
	pc = 0x82E30488; continue 'dispatch;
	}
	// 82E30480: 48008EE9  bl 0x82e39368
	ctx.lr = 0x82E30484;
	sub_82E39368(ctx, base);
	// 82E30484: 48000008  b 0x82e3048c
	pc = 0x82E3048C; continue 'dispatch;
	// 82E30488: 48008A21  bl 0x82e38ea8
	ctx.lr = 0x82E3048C;
	sub_82E38EA8(ctx, base);
	// 82E3048C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E30490: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30494: 419A0008  beq cr6, 0x82e3049c
	if ctx.cr[6].eq {
	pc = 0x82E3049C; continue 'dispatch;
	}
	// 82E30498: 4B4903F9  bl 0x822c0890
	ctx.lr = 0x82E3049C;
	sub_822C0890(ctx, base);
	// 82E3049C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E304A0: 4BFFA661  bl 0x82e2ab00
	ctx.lr = 0x82E304A4;
	sub_82E2AB00(ctx, base);
	// 82E304A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E304A8: 48377D14  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E304B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E304B0 size=168
    let mut pc: u32 = 0x82E304B0;
    'dispatch: loop {
        match pc {
            0x82E304B0 => {
    //   block [0x82E304B0..0x82E30558)
	// 82E304B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E304B4: 48377CAD  bl 0x831a8160
	ctx.lr = 0x82E304B8;
	sub_831A8130(ctx, base);
	// 82E304B8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E304BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E304C0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E304C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E304C8: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E304CC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E304D0: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82E304D4: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82E304D8: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82E304DC: 4BFFA60D  bl 0x82e2aae8
	ctx.lr = 0x82E304E0;
	sub_82E2AAE8(ctx, base);
	// 82E304E0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E304E4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E304E8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E304EC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E304F0: 4BFFB6F1  bl 0x82e2bbe0
	ctx.lr = 0x82E304F4;
	sub_82E2BBE0(ctx, base);
	// 82E304F4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E304F8: 419A0040  beq cr6, 0x82e30538
	if ctx.cr[6].eq {
	pc = 0x82E30538; continue 'dispatch;
	}
	// 82E304FC: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82E30500: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E30504: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82E30508: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E3050C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E30510: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E30514: 48008F5D  bl 0x82e39470
	ctx.lr = 0x82E30518;
	sub_82E39470(ctx, base);
	// 82E30518: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3051C: 41820014  beq 0x82e30530
	if ctx.cr[0].eq {
	pc = 0x82E30530; continue 'dispatch;
	}
	// 82E30520: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30524: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E30528: 4BFDECF1  bl 0x82e0f218
	ctx.lr = 0x82E3052C;
	sub_82E0F218(ctx, base);
	// 82E3052C: 4800000C  b 0x82e30538
	pc = 0x82E30538; continue 'dispatch;
	// 82E30530: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E30534: 4BFCE325  bl 0x82dfe858
	ctx.lr = 0x82E30538;
	sub_82DFE858(ctx, base);
	// 82E30538: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E3053C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30540: 419A0008  beq cr6, 0x82e30548
	if ctx.cr[6].eq {
	pc = 0x82E30548; continue 'dispatch;
	}
	// 82E30544: 4B49034D  bl 0x822c0890
	ctx.lr = 0x82E30548;
	sub_822C0890(ctx, base);
	// 82E30548: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E3054C: 4BFFA5B5  bl 0x82e2ab00
	ctx.lr = 0x82E30550;
	sub_82E2AB00(ctx, base);
	// 82E30550: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E30554: 48377C5C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30558 size=176
    let mut pc: u32 = 0x82E30558;
    'dispatch: loop {
        match pc {
            0x82E30558 => {
    //   block [0x82E30558..0x82E30608)
	// 82E30558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3055C: 48377C11  bl 0x831a816c
	ctx.lr = 0x82E30560;
	sub_831A8130(ctx, base);
	// 82E30560: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E30564: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E30568: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3056C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30570: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E30574: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30578: 4BFFA571  bl 0x82e2aae8
	ctx.lr = 0x82E3057C;
	sub_82E2AAE8(ctx, base);
	// 82E3057C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30580: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E30584: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30588: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E3058C: 4BFFB5E5  bl 0x82e2bb70
	ctx.lr = 0x82E30590;
	sub_82E2BB70(ctx, base);
	// 82E30590: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E30594: 419A0054  beq cr6, 0x82e305e8
	if ctx.cr[6].eq {
	pc = 0x82E305E8; continue 'dispatch;
	}
	// 82E30598: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3059C: 4BFDDF0D  bl 0x82e0e4a8
	ctx.lr = 0x82E305A0;
	sub_82E0E4A8(ctx, base);
	// 82E305A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E305A4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E305A8: 409A0020  bne cr6, 0x82e305c8
	if !ctx.cr[6].eq {
	pc = 0x82E305C8; continue 'dispatch;
	}
	// 82E305AC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E305B0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E305B4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E305B8: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E305BC: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E305C0: 480090D9  bl 0x82e39698
	ctx.lr = 0x82E305C4;
	sub_82E39698(ctx, base);
	// 82E305C4: 48000024  b 0x82e305e8
	pc = 0x82E305E8; continue 'dispatch;
	// 82E305C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E305CC: 409A001C  bne cr6, 0x82e305e8
	if !ctx.cr[6].eq {
	pc = 0x82E305E8; continue 'dispatch;
	}
	// 82E305D0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E305D4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E305D8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E305DC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E305E0: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E305E4: 4800901D  bl 0x82e39600
	ctx.lr = 0x82E305E8;
	sub_82E39600(ctx, base);
	// 82E305E8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E305EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E305F0: 419A0008  beq cr6, 0x82e305f8
	if ctx.cr[6].eq {
	pc = 0x82E305F8; continue 'dispatch;
	}
	// 82E305F4: 4B49029D  bl 0x822c0890
	ctx.lr = 0x82E305F8;
	sub_822C0890(ctx, base);
	// 82E305F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E305FC: 4BFFA505  bl 0x82e2ab00
	ctx.lr = 0x82E30600;
	sub_82E2AB00(ctx, base);
	// 82E30600: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E30604: 48377BB8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30608 size=72
    let mut pc: u32 = 0x82E30608;
    'dispatch: loop {
        match pc {
            0x82E30608 => {
    //   block [0x82E30608..0x82E30650)
	// 82E30608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3060C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E30610: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E30614: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82E30618: 419A001C  beq cr6, 0x82e30634
	if ctx.cr[6].eq {
	pc = 0x82E30634; continue 'dispatch;
	}
	// 82E3061C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E30620: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E30624: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82E30628: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3062C: 4BFFF945  bl 0x82e2ff70
	ctx.lr = 0x82E30630;
	sub_82E2FF70(ctx, base);
	// 82E30630: 48000010  b 0x82e30640
	pc = 0x82E30640; continue 'dispatch;
	// 82E30634: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E30638: 396BA910  addi r11, r11, -0x56f0
	ctx.r[11].s64 = ctx.r[11].s64 + -22256;
	// 82E3063C: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E30640: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E30644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E30648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E3064C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30650 size=72
    let mut pc: u32 = 0x82E30650;
    'dispatch: loop {
        match pc {
            0x82E30650 => {
    //   block [0x82E30650..0x82E30698)
	// 82E30650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E30658: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3065C: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82E30660: 419A001C  beq cr6, 0x82e3067c
	if ctx.cr[6].eq {
	pc = 0x82E3067C; continue 'dispatch;
	}
	// 82E30664: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E30668: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E3066C: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82E30670: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E30674: 4BFFF985  bl 0x82e2fff8
	ctx.lr = 0x82E30678;
	sub_82E2FFF8(ctx, base);
	// 82E30678: 48000010  b 0x82e30688
	pc = 0x82E30688; continue 'dispatch;
	// 82E3067C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E30680: 396BAA30  addi r11, r11, -0x55d0
	ctx.r[11].s64 = ctx.r[11].s64 + -21968;
	// 82E30684: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E30688: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E3068C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E30690: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E30694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30698 size=72
    let mut pc: u32 = 0x82E30698;
    'dispatch: loop {
        match pc {
            0x82E30698 => {
    //   block [0x82E30698..0x82E306E0)
	// 82E30698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3069C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E306A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E306A4: 2F050003  cmpwi cr6, r5, 3
	ctx.cr[6].compare_i32(ctx.r[5].s32, 3, &mut ctx.xer);
	// 82E306A8: 419A001C  beq cr6, 0x82e306c4
	if ctx.cr[6].eq {
	pc = 0x82E306C4; continue 'dispatch;
	}
	// 82E306AC: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E306B0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E306B4: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82E306B8: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E306BC: 4BFFF9CD  bl 0x82e30088
	ctx.lr = 0x82E306C0;
	sub_82E30088(ctx, base);
	// 82E306C0: 48000010  b 0x82e306d0
	pc = 0x82E306D0; continue 'dispatch;
	// 82E306C4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E306C8: 396BAB60  addi r11, r11, -0x54a0
	ctx.r[11].s64 = ctx.r[11].s64 + -21664;
	// 82E306CC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E306D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E306D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E306D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E306DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E306E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E306E0 size=172
    let mut pc: u32 = 0x82E306E0;
    'dispatch: loop {
        match pc {
            0x82E306E0 => {
    //   block [0x82E306E0..0x82E3078C)
	// 82E306E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E306E4: 48377A89  bl 0x831a816c
	ctx.lr = 0x82E306E8;
	sub_831A8130(ctx, base);
	// 82E306E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E306EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E306F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E306F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E306F8: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E306FC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30700: 4BFFA3E9  bl 0x82e2aae8
	ctx.lr = 0x82E30704;
	sub_82E2AAE8(ctx, base);
	// 82E30704: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30708: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E3070C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30710: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30714: 4BFFE01D  bl 0x82e2e730
	ctx.lr = 0x82E30718;
	sub_82E2E730(ctx, base);
	// 82E30718: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E3071C: 419A0050  beq cr6, 0x82e3076c
	if ctx.cr[6].eq {
	pc = 0x82E3076C; continue 'dispatch;
	}
	// 82E30720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E30724: 4BFDDD85  bl 0x82e0e4a8
	ctx.lr = 0x82E30728;
	sub_82E0E4A8(ctx, base);
	// 82E30728: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3072C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E30730: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30734: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82E30738: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E3073C: 409A0014  bne cr6, 0x82e30750
	if !ctx.cr[6].eq {
	pc = 0x82E30750; continue 'dispatch;
	}
	// 82E30740: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30744: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E30748: 48009A39  bl 0x82e3a180
	ctx.lr = 0x82E3074C;
	sub_82E3A180(ctx, base);
	// 82E3074C: 48000020  b 0x82e3076c
	pc = 0x82E3076C; continue 'dispatch;
	// 82E30750: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E30754: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30758: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E3075C: 409A000C  bne cr6, 0x82e30768
	if !ctx.cr[6].eq {
	pc = 0x82E30768; continue 'dispatch;
	}
	// 82E30760: 48009819  bl 0x82e39f78
	ctx.lr = 0x82E30764;
	sub_82E39F78(ctx, base);
	// 82E30764: 48000008  b 0x82e3076c
	pc = 0x82E3076C; continue 'dispatch;
	// 82E30768: 480095D9  bl 0x82e39d40
	ctx.lr = 0x82E3076C;
	sub_82E39D40(ctx, base);
	// 82E3076C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E30770: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30774: 419A0008  beq cr6, 0x82e3077c
	if ctx.cr[6].eq {
	pc = 0x82E3077C; continue 'dispatch;
	}
	// 82E30778: 4B490119  bl 0x822c0890
	ctx.lr = 0x82E3077C;
	sub_822C0890(ctx, base);
	// 82E3077C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30780: 4BFFA381  bl 0x82e2ab00
	ctx.lr = 0x82E30784;
	sub_82E2AB00(ctx, base);
	// 82E30784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E30788: 48377A34  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30790 size=128
    let mut pc: u32 = 0x82E30790;
    'dispatch: loop {
        match pc {
            0x82E30790 => {
    //   block [0x82E30790..0x82E30810)
	// 82E30790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30794: 483779D9  bl 0x831a816c
	ctx.lr = 0x82E30798;
	sub_831A8130(ctx, base);
	// 82E30798: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3079C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E307A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E307A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E307A8: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E307AC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E307B0: 4BFFA339  bl 0x82e2aae8
	ctx.lr = 0x82E307B4;
	sub_82E2AAE8(ctx, base);
	// 82E307B4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E307B8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E307BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E307C0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E307C4: 4BFFB33D  bl 0x82e2bb00
	ctx.lr = 0x82E307C8;
	sub_82E2BB00(ctx, base);
	// 82E307C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E307CC: 419A0024  beq cr6, 0x82e307f0
	if ctx.cr[6].eq {
	pc = 0x82E307F0; continue 'dispatch;
	}
	// 82E307D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E307D4: 4BFDDCD5  bl 0x82e0e4a8
	ctx.lr = 0x82E307D8;
	sub_82E0E4A8(ctx, base);
	// 82E307D8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E307DC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E307E0: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E307E4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E307E8: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E307EC: 480090A5  bl 0x82e39890
	ctx.lr = 0x82E307F0;
	sub_82E39890(ctx, base);
	// 82E307F0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E307F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E307F8: 419A0008  beq cr6, 0x82e30800
	if ctx.cr[6].eq {
	pc = 0x82E30800; continue 'dispatch;
	}
	// 82E307FC: 4B490095  bl 0x822c0890
	ctx.lr = 0x82E30800;
	sub_822C0890(ctx, base);
	// 82E30800: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30804: 4BFFA2FD  bl 0x82e2ab00
	ctx.lr = 0x82E30808;
	sub_82E2AB00(ctx, base);
	// 82E30808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E3080C: 483779B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30810 size=176
    let mut pc: u32 = 0x82E30810;
    'dispatch: loop {
        match pc {
            0x82E30810 => {
    //   block [0x82E30810..0x82E308C0)
	// 82E30810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30814: 48377959  bl 0x831a816c
	ctx.lr = 0x82E30818;
	sub_831A8130(ctx, base);
	// 82E30818: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3081C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E30820: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30824: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30828: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3082C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30830: 4BFFA2B9  bl 0x82e2aae8
	ctx.lr = 0x82E30834;
	sub_82E2AAE8(ctx, base);
	// 82E30834: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30838: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E3083C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30840: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30844: 4BFFDF5D  bl 0x82e2e7a0
	ctx.lr = 0x82E30848;
	sub_82E2E7A0(ctx, base);
	// 82E30848: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E3084C: 419A0054  beq cr6, 0x82e308a0
	if ctx.cr[6].eq {
	pc = 0x82E308A0; continue 'dispatch;
	}
	// 82E30850: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E30854: 4BFDDC55  bl 0x82e0e4a8
	ctx.lr = 0x82E30858;
	sub_82E0E4A8(ctx, base);
	// 82E30858: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3085C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E30860: 419A0028  beq cr6, 0x82e30888
	if ctx.cr[6].eq {
	pc = 0x82E30888; continue 'dispatch;
	}
	// 82E30864: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 82E30868: 419A0020  beq cr6, 0x82e30888
	if ctx.cr[6].eq {
	pc = 0x82E30888; continue 'dispatch;
	}
	// 82E3086C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30870: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30874: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E30878: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3087C: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E30880: 48009F11  bl 0x82e3a790
	ctx.lr = 0x82E30884;
	sub_82E3A790(ctx, base);
	// 82E30884: 4800001C  b 0x82e308a0
	pc = 0x82E308A0; continue 'dispatch;
	// 82E30888: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3088C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30890: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E30894: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E30898: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E3089C: 4800A13D  bl 0x82e3a9d8
	ctx.lr = 0x82E308A0;
	sub_82E3A9D8(ctx, base);
	// 82E308A0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E308A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E308A8: 419A0008  beq cr6, 0x82e308b0
	if ctx.cr[6].eq {
	pc = 0x82E308B0; continue 'dispatch;
	}
	// 82E308AC: 4B48FFE5  bl 0x822c0890
	ctx.lr = 0x82E308B0;
	sub_822C0890(ctx, base);
	// 82E308B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E308B4: 4BFFA24D  bl 0x82e2ab00
	ctx.lr = 0x82E308B8;
	sub_82E2AB00(ctx, base);
	// 82E308B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E308BC: 48377900  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E308C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E308C0 size=128
    let mut pc: u32 = 0x82E308C0;
    'dispatch: loop {
        match pc {
            0x82E308C0 => {
    //   block [0x82E308C0..0x82E30940)
	// 82E308C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E308C4: 483778A9  bl 0x831a816c
	ctx.lr = 0x82E308C8;
	sub_831A8130(ctx, base);
	// 82E308C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E308CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E308D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E308D4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E308D8: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E308DC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E308E0: 4BFFA209  bl 0x82e2aae8
	ctx.lr = 0x82E308E4;
	sub_82E2AAE8(ctx, base);
	// 82E308E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E308E8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E308EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E308F0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E308F4: 4BFFE775  bl 0x82e2f068
	ctx.lr = 0x82E308F8;
	sub_82E2F068(ctx, base);
	// 82E308F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E308FC: 419A0024  beq cr6, 0x82e30920
	if ctx.cr[6].eq {
	pc = 0x82E30920; continue 'dispatch;
	}
	// 82E30900: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E30904: 4BFDDBA5  bl 0x82e0e4a8
	ctx.lr = 0x82E30908;
	sub_82E0E4A8(ctx, base);
	// 82E30908: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3090C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30910: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E30914: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E30918: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E3091C: 4800A415  bl 0x82e3ad30
	ctx.lr = 0x82E30920;
	sub_82E3AD30(ctx, base);
	// 82E30920: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E30924: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30928: 419A0008  beq cr6, 0x82e30930
	if ctx.cr[6].eq {
	pc = 0x82E30930; continue 'dispatch;
	}
	// 82E3092C: 4B48FF65  bl 0x822c0890
	ctx.lr = 0x82E30930;
	sub_822C0890(ctx, base);
	// 82E30930: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30934: 4BFFA1CD  bl 0x82e2ab00
	ctx.lr = 0x82E30938;
	sub_82E2AB00(ctx, base);
	// 82E30938: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E3093C: 48377880  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30940 size=148
    let mut pc: u32 = 0x82E30940;
    'dispatch: loop {
        match pc {
            0x82E30940 => {
    //   block [0x82E30940..0x82E309D4)
	// 82E30940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30944: 48377821  bl 0x831a8164
	ctx.lr = 0x82E30948;
	sub_831A8130(ctx, base);
	// 82E30948: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3094C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E30950: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30954: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30958: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3095C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E30960: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30964: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 82E30968: 4BFFA181  bl 0x82e2aae8
	ctx.lr = 0x82E3096C;
	sub_82E2AAE8(ctx, base);
	// 82E3096C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30970: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E30974: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E30978: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3097C: 4BFFBBFD  bl 0x82e2c578
	ctx.lr = 0x82E30980;
	sub_82E2C578(ctx, base);
	// 82E30980: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E30984: 419A0030  beq cr6, 0x82e309b4
	if ctx.cr[6].eq {
	pc = 0x82E309B4; continue 'dispatch;
	}
	// 82E30988: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3098C: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E30990: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30994: 41820014  beq 0x82e309a8
	if ctx.cr[0].eq {
	pc = 0x82E309A8; continue 'dispatch;
	}
	// 82E30998: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E3099C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E309A0: 4BFF9F71  bl 0x82e2a910
	ctx.lr = 0x82E309A4;
	sub_82E2A910(ctx, base);
	// 82E309A4: 48000010  b 0x82e309b4
	pc = 0x82E309B4; continue 'dispatch;
	// 82E309A8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E309AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E309B0: 4BFF9E69  bl 0x82e2a818
	ctx.lr = 0x82E309B4;
	sub_82E2A818(ctx, base);
	// 82E309B4: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E309B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E309BC: 419A0008  beq cr6, 0x82e309c4
	if ctx.cr[6].eq {
	pc = 0x82E309C4; continue 'dispatch;
	}
	// 82E309C0: 4B48FED1  bl 0x822c0890
	ctx.lr = 0x82E309C4;
	sub_822C0890(ctx, base);
	// 82E309C4: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E309C8: 4BFFA139  bl 0x82e2ab00
	ctx.lr = 0x82E309CC;
	sub_82E2AB00(ctx, base);
	// 82E309CC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E309D0: 483777E4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E309D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E309D8 size=148
    let mut pc: u32 = 0x82E309D8;
    'dispatch: loop {
        match pc {
            0x82E309D8 => {
    //   block [0x82E309D8..0x82E30A6C)
	// 82E309D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E309DC: 48377789  bl 0x831a8164
	ctx.lr = 0x82E309E0;
	sub_831A8130(ctx, base);
	// 82E309E0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E309E4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82E309E8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E309EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E309F0: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E309F4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E309F8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E309FC: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 82E30A00: 4BFFA0E9  bl 0x82e2aae8
	ctx.lr = 0x82E30A04;
	sub_82E2AAE8(ctx, base);
	// 82E30A04: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30A08: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82E30A0C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E30A10: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30A14: 4BFFBA85  bl 0x82e2c498
	ctx.lr = 0x82E30A18;
	sub_82E2C498(ctx, base);
	// 82E30A18: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E30A1C: 419A0030  beq cr6, 0x82e30a4c
	if ctx.cr[6].eq {
	pc = 0x82E30A4C; continue 'dispatch;
	}
	// 82E30A20: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E30A24: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E30A28: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30A2C: 41820014  beq 0x82e30a40
	if ctx.cr[0].eq {
	pc = 0x82E30A40; continue 'dispatch;
	}
	// 82E30A30: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E30A34: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82E30A38: 4BFF9ED9  bl 0x82e2a910
	ctx.lr = 0x82E30A3C;
	sub_82E2A910(ctx, base);
	// 82E30A3C: 48000010  b 0x82e30a4c
	pc = 0x82E30A4C; continue 'dispatch;
	// 82E30A40: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E30A44: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E30A48: 4BFF92A1  bl 0x82e29ce8
	ctx.lr = 0x82E30A4C;
	sub_82E29CE8(ctx, base);
	// 82E30A4C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E30A50: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30A54: 419A0008  beq cr6, 0x82e30a5c
	if ctx.cr[6].eq {
	pc = 0x82E30A5C; continue 'dispatch;
	}
	// 82E30A58: 4B48FE39  bl 0x822c0890
	ctx.lr = 0x82E30A5C;
	sub_822C0890(ctx, base);
	// 82E30A5C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30A60: 4BFFA0A1  bl 0x82e2ab00
	ctx.lr = 0x82E30A64;
	sub_82E2AB00(ctx, base);
	// 82E30A64: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82E30A68: 4837774C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30A70 size=128
    let mut pc: u32 = 0x82E30A70;
    'dispatch: loop {
        match pc {
            0x82E30A70 => {
    //   block [0x82E30A70..0x82E30AF0)
	// 82E30A70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30A74: 483776F9  bl 0x831a816c
	ctx.lr = 0x82E30A78;
	sub_831A8130(ctx, base);
	// 82E30A78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E30A7C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E30A80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30A84: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30A88: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E30A8C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30A90: 4BFFA059  bl 0x82e2aae8
	ctx.lr = 0x82E30A94;
	sub_82E2AAE8(ctx, base);
	// 82E30A94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30A98: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E30A9C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30AA0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30AA4: 4BFFE85D  bl 0x82e2f300
	ctx.lr = 0x82E30AA8;
	sub_82E2F300(ctx, base);
	// 82E30AA8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E30AAC: 419A0024  beq cr6, 0x82e30ad0
	if ctx.cr[6].eq {
	pc = 0x82E30AD0; continue 'dispatch;
	}
	// 82E30AB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E30AB4: 4BFDD9F5  bl 0x82e0e4a8
	ctx.lr = 0x82E30AB8;
	sub_82E0E4A8(ctx, base);
	// 82E30AB8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30ABC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30AC0: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E30AC4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E30AC8: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E30ACC: 4800A2DD  bl 0x82e3ada8
	ctx.lr = 0x82E30AD0;
	sub_82E3ADA8(ctx, base);
	// 82E30AD0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E30AD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30AD8: 419A0008  beq cr6, 0x82e30ae0
	if ctx.cr[6].eq {
	pc = 0x82E30AE0; continue 'dispatch;
	}
	// 82E30ADC: 4B48FDB5  bl 0x822c0890
	ctx.lr = 0x82E30AE0;
	sub_822C0890(ctx, base);
	// 82E30AE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30AE4: 4BFFA01D  bl 0x82e2ab00
	ctx.lr = 0x82E30AE8;
	sub_82E2AB00(ctx, base);
	// 82E30AE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E30AEC: 483776D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30AF0 size=128
    let mut pc: u32 = 0x82E30AF0;
    'dispatch: loop {
        match pc {
            0x82E30AF0 => {
    //   block [0x82E30AF0..0x82E30B70)
	// 82E30AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30AF4: 48377679  bl 0x831a816c
	ctx.lr = 0x82E30AF8;
	sub_831A8130(ctx, base);
	// 82E30AF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E30AFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E30B00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30B04: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30B08: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E30B0C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30B10: 4BFF9FD9  bl 0x82e2aae8
	ctx.lr = 0x82E30B14;
	sub_82E2AAE8(ctx, base);
	// 82E30B14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30B18: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E30B1C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30B20: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30B24: 4BFFDD5D  bl 0x82e2e880
	ctx.lr = 0x82E30B28;
	sub_82E2E880(ctx, base);
	// 82E30B28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E30B2C: 419A0024  beq cr6, 0x82e30b50
	if ctx.cr[6].eq {
	pc = 0x82E30B50; continue 'dispatch;
	}
	// 82E30B30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E30B34: 4BFDD975  bl 0x82e0e4a8
	ctx.lr = 0x82E30B38;
	sub_82E0E4A8(ctx, base);
	// 82E30B38: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30B3C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30B40: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E30B44: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E30B48: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E30B4C: 4800A6CD  bl 0x82e3b218
	ctx.lr = 0x82E30B50;
	sub_82E3B218(ctx, base);
	// 82E30B50: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E30B54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30B58: 419A0008  beq cr6, 0x82e30b60
	if ctx.cr[6].eq {
	pc = 0x82E30B60; continue 'dispatch;
	}
	// 82E30B5C: 4B48FD35  bl 0x822c0890
	ctx.lr = 0x82E30B60;
	sub_822C0890(ctx, base);
	// 82E30B60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30B64: 4BFF9F9D  bl 0x82e2ab00
	ctx.lr = 0x82E30B68;
	sub_82E2AB00(ctx, base);
	// 82E30B68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E30B6C: 48377650  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30B70 size=128
    let mut pc: u32 = 0x82E30B70;
    'dispatch: loop {
        match pc {
            0x82E30B70 => {
    //   block [0x82E30B70..0x82E30BF0)
	// 82E30B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30B74: 483775F9  bl 0x831a816c
	ctx.lr = 0x82E30B78;
	sub_831A8130(ctx, base);
	// 82E30B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E30B7C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E30B80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30B84: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30B88: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E30B8C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30B90: 4BFF9F59  bl 0x82e2aae8
	ctx.lr = 0x82E30B94;
	sub_82E2AAE8(ctx, base);
	// 82E30B94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30B98: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E30B9C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30BA0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30BA4: 4BFFB0AD  bl 0x82e2bc50
	ctx.lr = 0x82E30BA8;
	sub_82E2BC50(ctx, base);
	// 82E30BA8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E30BAC: 419A0024  beq cr6, 0x82e30bd0
	if ctx.cr[6].eq {
	pc = 0x82E30BD0; continue 'dispatch;
	}
	// 82E30BB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E30BB4: 4BFDD8F5  bl 0x82e0e4a8
	ctx.lr = 0x82E30BB8;
	sub_82E0E4A8(ctx, base);
	// 82E30BB8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30BBC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30BC0: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E30BC4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E30BC8: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E30BCC: 4800A73D  bl 0x82e3b308
	ctx.lr = 0x82E30BD0;
	sub_82E3B308(ctx, base);
	// 82E30BD0: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E30BD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30BD8: 419A0008  beq cr6, 0x82e30be0
	if ctx.cr[6].eq {
	pc = 0x82E30BE0; continue 'dispatch;
	}
	// 82E30BDC: 4B48FCB5  bl 0x822c0890
	ctx.lr = 0x82E30BE0;
	sub_822C0890(ctx, base);
	// 82E30BE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30BE4: 4BFF9F1D  bl 0x82e2ab00
	ctx.lr = 0x82E30BE8;
	sub_82E2AB00(ctx, base);
	// 82E30BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E30BEC: 483775D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30BF0 size=128
    let mut pc: u32 = 0x82E30BF0;
    'dispatch: loop {
        match pc {
            0x82E30BF0 => {
    //   block [0x82E30BF0..0x82E30C70)
	// 82E30BF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30BF4: 48377579  bl 0x831a816c
	ctx.lr = 0x82E30BF8;
	sub_831A8130(ctx, base);
	// 82E30BF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E30BFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E30C00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30C04: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30C08: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E30C0C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30C10: 4BFF9ED9  bl 0x82e2aae8
	ctx.lr = 0x82E30C14;
	sub_82E2AAE8(ctx, base);
	// 82E30C14: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30C18: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E30C1C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30C20: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30C24: 4BFFB17D  bl 0x82e2bda0
	ctx.lr = 0x82E30C28;
	sub_82E2BDA0(ctx, base);
	// 82E30C28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E30C2C: 419A0024  beq cr6, 0x82e30c50
	if ctx.cr[6].eq {
	pc = 0x82E30C50; continue 'dispatch;
	}
	// 82E30C30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E30C34: 4BFDD875  bl 0x82e0e4a8
	ctx.lr = 0x82E30C38;
	sub_82E0E4A8(ctx, base);
	// 82E30C38: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30C3C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30C40: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E30C44: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E30C48: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E30C4C: 4800A8F5  bl 0x82e3b540
	ctx.lr = 0x82E30C50;
	sub_82E3B540(ctx, base);
	// 82E30C50: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E30C54: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30C58: 419A0008  beq cr6, 0x82e30c60
	if ctx.cr[6].eq {
	pc = 0x82E30C60; continue 'dispatch;
	}
	// 82E30C5C: 4B48FC35  bl 0x822c0890
	ctx.lr = 0x82E30C60;
	sub_822C0890(ctx, base);
	// 82E30C60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30C64: 4BFF9E9D  bl 0x82e2ab00
	ctx.lr = 0x82E30C68;
	sub_82E2AB00(ctx, base);
	// 82E30C68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E30C6C: 48377550  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30C70 size=148
    let mut pc: u32 = 0x82E30C70;
    'dispatch: loop {
        match pc {
            0x82E30C70 => {
    //   block [0x82E30C70..0x82E30D04)
	// 82E30C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30C74: 483774F9  bl 0x831a816c
	ctx.lr = 0x82E30C78;
	sub_831A8130(ctx, base);
	// 82E30C78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E30C7C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E30C80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30C84: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30C88: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E30C8C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30C90: 4BFF9E59  bl 0x82e2aae8
	ctx.lr = 0x82E30C94;
	sub_82E2AAE8(ctx, base);
	// 82E30C94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30C98: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E30C9C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30CA0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30CA4: 4BFFB865  bl 0x82e2c508
	ctx.lr = 0x82E30CA8;
	sub_82E2C508(ctx, base);
	// 82E30CA8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E30CAC: 419A0038  beq cr6, 0x82e30ce4
	if ctx.cr[6].eq {
	pc = 0x82E30CE4; continue 'dispatch;
	}
	// 82E30CB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E30CB4: 4BFDD7F5  bl 0x82e0e4a8
	ctx.lr = 0x82E30CB8;
	sub_82E0E4A8(ctx, base);
	// 82E30CB8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E30CBC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E30CC0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30CC4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E30CC8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E30CCC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30CD0: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E30CD4: 409A000C  bne cr6, 0x82e30ce0
	if !ctx.cr[6].eq {
	pc = 0x82E30CE0; continue 'dispatch;
	}
	// 82E30CD8: 4800A951  bl 0x82e3b628
	ctx.lr = 0x82E30CDC;
	sub_82E3B628(ctx, base);
	// 82E30CDC: 48000008  b 0x82e30ce4
	pc = 0x82E30CE4; continue 'dispatch;
	// 82E30CE0: 4800AA59  bl 0x82e3b738
	ctx.lr = 0x82E30CE4;
	sub_82E3B738(ctx, base);
	// 82E30CE4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E30CE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30CEC: 419A0008  beq cr6, 0x82e30cf4
	if ctx.cr[6].eq {
	pc = 0x82E30CF4; continue 'dispatch;
	}
	// 82E30CF0: 4B48FBA1  bl 0x822c0890
	ctx.lr = 0x82E30CF4;
	sub_822C0890(ctx, base);
	// 82E30CF4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30CF8: 4BFF9E09  bl 0x82e2ab00
	ctx.lr = 0x82E30CFC;
	sub_82E2AB00(ctx, base);
	// 82E30CFC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E30D00: 483774BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30D08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30D08 size=176
    let mut pc: u32 = 0x82E30D08;
    'dispatch: loop {
        match pc {
            0x82E30D08 => {
    //   block [0x82E30D08..0x82E30DB8)
	// 82E30D08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30D0C: 48377455  bl 0x831a8160
	ctx.lr = 0x82E30D10;
	sub_831A8130(ctx, base);
	// 82E30D10: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E30D14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E30D18: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30D1C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E30D20: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E30D24: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E30D28: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82E30D2C: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82E30D30: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82E30D34: 4BFF9DB5  bl 0x82e2aae8
	ctx.lr = 0x82E30D38;
	sub_82E2AAE8(ctx, base);
	// 82E30D38: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30D3C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E30D40: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E30D44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30D48: 4BFFAE99  bl 0x82e2bbe0
	ctx.lr = 0x82E30D4C;
	sub_82E2BBE0(ctx, base);
	// 82E30D4C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E30D50: 419A0048  beq cr6, 0x82e30d98
	if ctx.cr[6].eq {
	pc = 0x82E30D98; continue 'dispatch;
	}
	// 82E30D54: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82E30D58: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E30D5C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 82E30D60: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82E30D64: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82E30D68: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E30D6C: 4800AACD  bl 0x82e3b838
	ctx.lr = 0x82E30D70;
	sub_82E3B838(ctx, base);
	// 82E30D70: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E30D74: 41820014  beq 0x82e30d88
	if ctx.cr[0].eq {
	pc = 0x82E30D88; continue 'dispatch;
	}
	// 82E30D78: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30D7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E30D80: 4BFDE499  bl 0x82e0f218
	ctx.lr = 0x82E30D84;
	sub_82E0F218(ctx, base);
	// 82E30D84: 48000014  b 0x82e30d98
	pc = 0x82E30D98; continue 'dispatch;
	// 82E30D88: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E30D8C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30D90: 419A0008  beq cr6, 0x82e30d98
	if ctx.cr[6].eq {
	pc = 0x82E30D98; continue 'dispatch;
	}
	// 82E30D94: 4BFCDAC5  bl 0x82dfe858
	ctx.lr = 0x82E30D98;
	sub_82DFE858(ctx, base);
	// 82E30D98: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E30D9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30DA0: 419A0008  beq cr6, 0x82e30da8
	if ctx.cr[6].eq {
	pc = 0x82E30DA8; continue 'dispatch;
	}
	// 82E30DA4: 4B48FAED  bl 0x822c0890
	ctx.lr = 0x82E30DA8;
	sub_822C0890(ctx, base);
	// 82E30DA8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30DAC: 4BFF9D55  bl 0x82e2ab00
	ctx.lr = 0x82E30DB0;
	sub_82E2AB00(ctx, base);
	// 82E30DB0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E30DB4: 483773FC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30DB8 size=140
    let mut pc: u32 = 0x82E30DB8;
    'dispatch: loop {
        match pc {
            0x82E30DB8 => {
    //   block [0x82E30DB8..0x82E30E44)
	// 82E30DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30DBC: 483773B1  bl 0x831a816c
	ctx.lr = 0x82E30DC0;
	sub_831A8130(ctx, base);
	// 82E30DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E30DC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E30DC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30DCC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30DD0: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E30DD4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30DD8: 4BFF9D11  bl 0x82e2aae8
	ctx.lr = 0x82E30DDC;
	sub_82E2AAE8(ctx, base);
	// 82E30DDC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30DE0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E30DE4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30DE8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30DEC: 4BFFE015  bl 0x82e2ee00
	ctx.lr = 0x82E30DF0;
	sub_82E2EE00(ctx, base);
	// 82E30DF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E30DF4: 419A0030  beq cr6, 0x82e30e24
	if ctx.cr[6].eq {
	pc = 0x82E30E24; continue 'dispatch;
	}
	// 82E30DF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E30DFC: 4BFDD6AD  bl 0x82e0e4a8
	ctx.lr = 0x82E30E00;
	sub_82E0E4A8(ctx, base);
	// 82E30E00: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E30E04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E30E08: 409A001C  bne cr6, 0x82e30e24
	if !ctx.cr[6].eq {
	pc = 0x82E30E24; continue 'dispatch;
	}
	// 82E30E0C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30E10: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30E14: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E30E18: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E30E1C: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E30E20: 480089C9  bl 0x82e397e8
	ctx.lr = 0x82E30E24;
	sub_82E397E8(ctx, base);
	// 82E30E24: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E30E28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30E2C: 419A0008  beq cr6, 0x82e30e34
	if ctx.cr[6].eq {
	pc = 0x82E30E34; continue 'dispatch;
	}
	// 82E30E30: 4B48FA61  bl 0x822c0890
	ctx.lr = 0x82E30E34;
	sub_822C0890(ctx, base);
	// 82E30E34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30E38: 4BFF9CC9  bl 0x82e2ab00
	ctx.lr = 0x82E30E3C;
	sub_82E2AB00(ctx, base);
	// 82E30E3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E30E40: 4837737C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30E48 size=176
    let mut pc: u32 = 0x82E30E48;
    'dispatch: loop {
        match pc {
            0x82E30E48 => {
    //   block [0x82E30E48..0x82E30EF8)
	// 82E30E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30E4C: 48377321  bl 0x831a816c
	ctx.lr = 0x82E30E50;
	sub_831A8130(ctx, base);
	// 82E30E50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E30E54: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E30E58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30E5C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30E60: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E30E64: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30E68: 4BFF9C81  bl 0x82e2aae8
	ctx.lr = 0x82E30E6C;
	sub_82E2AAE8(ctx, base);
	// 82E30E6C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30E70: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E30E74: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30E78: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30E7C: 4BFFB975  bl 0x82e2c7f0
	ctx.lr = 0x82E30E80;
	sub_82E2C7F0(ctx, base);
	// 82E30E80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E30E84: 419A0054  beq cr6, 0x82e30ed8
	if ctx.cr[6].eq {
	pc = 0x82E30ED8; continue 'dispatch;
	}
	// 82E30E88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E30E8C: 4BFDD61D  bl 0x82e0e4a8
	ctx.lr = 0x82E30E90;
	sub_82E0E4A8(ctx, base);
	// 82E30E90: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E30E94: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E30E98: 409A0020  bne cr6, 0x82e30eb8
	if !ctx.cr[6].eq {
	pc = 0x82E30EB8; continue 'dispatch;
	}
	// 82E30E9C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30EA0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30EA4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E30EA8: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E30EAC: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E30EB0: 4800AE71  bl 0x82e3bd20
	ctx.lr = 0x82E30EB4;
	sub_82E3BD20(ctx, base);
	// 82E30EB4: 48000024  b 0x82e30ed8
	pc = 0x82E30ED8; continue 'dispatch;
	// 82E30EB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E30EBC: 409A001C  bne cr6, 0x82e30ed8
	if !ctx.cr[6].eq {
	pc = 0x82E30ED8; continue 'dispatch;
	}
	// 82E30EC0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30EC4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30EC8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E30ECC: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E30ED0: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E30ED4: 4800AC55  bl 0x82e3bb28
	ctx.lr = 0x82E30ED8;
	sub_82E3BB28(ctx, base);
	// 82E30ED8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E30EDC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30EE0: 419A0008  beq cr6, 0x82e30ee8
	if ctx.cr[6].eq {
	pc = 0x82E30EE8; continue 'dispatch;
	}
	// 82E30EE4: 4B48F9AD  bl 0x822c0890
	ctx.lr = 0x82E30EE8;
	sub_822C0890(ctx, base);
	// 82E30EE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30EEC: 4BFF9C15  bl 0x82e2ab00
	ctx.lr = 0x82E30EF0;
	sub_82E2AB00(ctx, base);
	// 82E30EF0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E30EF4: 483772C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30EF8 size=196
    let mut pc: u32 = 0x82E30EF8;
    'dispatch: loop {
        match pc {
            0x82E30EF8 => {
    //   block [0x82E30EF8..0x82E30FBC)
	// 82E30EF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30EFC: 48377271  bl 0x831a816c
	ctx.lr = 0x82E30F00;
	sub_831A8130(ctx, base);
	// 82E30F00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E30F04: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E30F08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30F0C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30F10: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E30F14: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30F18: 4BFF9BD1  bl 0x82e2aae8
	ctx.lr = 0x82E30F1C;
	sub_82E2AAE8(ctx, base);
	// 82E30F1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30F20: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E30F24: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30F28: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30F2C: 4BFFD9C5  bl 0x82e2e8f0
	ctx.lr = 0x82E30F30;
	sub_82E2E8F0(ctx, base);
	// 82E30F30: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E30F34: 419A0068  beq cr6, 0x82e30f9c
	if ctx.cr[6].eq {
	pc = 0x82E30F9C; continue 'dispatch;
	}
	// 82E30F38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E30F3C: 4BFDD56D  bl 0x82e0e4a8
	ctx.lr = 0x82E30F40;
	sub_82E0E4A8(ctx, base);
	// 82E30F40: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E30F44: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E30F48: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E30F4C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82E30F50: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E30F54: 409A0014  bne cr6, 0x82e30f68
	if !ctx.cr[6].eq {
	pc = 0x82E30F68; continue 'dispatch;
	}
	// 82E30F58: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30F5C: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E30F60: 48004F89  bl 0x82e35ee8
	ctx.lr = 0x82E30F64;
	sub_82E35EE8(ctx, base);
	// 82E30F64: 48000038  b 0x82e30f9c
	pc = 0x82E30F9C; continue 'dispatch;
	// 82E30F68: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82E30F6C: 409A0014  bne cr6, 0x82e30f80
	if !ctx.cr[6].eq {
	pc = 0x82E30F80; continue 'dispatch;
	}
	// 82E30F70: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30F74: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E30F78: 48004E91  bl 0x82e35e08
	ctx.lr = 0x82E30F7C;
	sub_82E35E08(ctx, base);
	// 82E30F7C: 48000020  b 0x82e30f9c
	pc = 0x82E30F9C; continue 'dispatch;
	// 82E30F80: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E30F84: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E30F88: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E30F8C: 409A000C  bne cr6, 0x82e30f98
	if !ctx.cr[6].eq {
	pc = 0x82E30F98; continue 'dispatch;
	}
	// 82E30F90: 48004D99  bl 0x82e35d28
	ctx.lr = 0x82E30F94;
	sub_82E35D28(ctx, base);
	// 82E30F94: 48000008  b 0x82e30f9c
	pc = 0x82E30F9C; continue 'dispatch;
	// 82E30F98: 48004CB1  bl 0x82e35c48
	ctx.lr = 0x82E30F9C;
	sub_82E35C48(ctx, base);
	// 82E30F9C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E30FA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E30FA4: 419A0008  beq cr6, 0x82e30fac
	if ctx.cr[6].eq {
	pc = 0x82E30FAC; continue 'dispatch;
	}
	// 82E30FA8: 4B48F8E9  bl 0x822c0890
	ctx.lr = 0x82E30FAC;
	sub_822C0890(ctx, base);
	// 82E30FAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30FB0: 4BFF9B51  bl 0x82e2ab00
	ctx.lr = 0x82E30FB4;
	sub_82E2AB00(ctx, base);
	// 82E30FB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E30FB8: 48377204  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E30FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E30FC0 size=128
    let mut pc: u32 = 0x82E30FC0;
    'dispatch: loop {
        match pc {
            0x82E30FC0 => {
    //   block [0x82E30FC0..0x82E31040)
	// 82E30FC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E30FC4: 483771A9  bl 0x831a816c
	ctx.lr = 0x82E30FC8;
	sub_831A8130(ctx, base);
	// 82E30FC8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E30FCC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E30FD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E30FD4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E30FD8: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E30FDC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E30FE0: 4BFF9B09  bl 0x82e2aae8
	ctx.lr = 0x82E30FE4;
	sub_82E2AAE8(ctx, base);
	// 82E30FE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E30FE8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E30FEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E30FF0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E30FF4: 4BFFCA0D  bl 0x82e2da00
	ctx.lr = 0x82E30FF8;
	sub_82E2DA00(ctx, base);
	// 82E30FF8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E30FFC: 419A0024  beq cr6, 0x82e31020
	if ctx.cr[6].eq {
	pc = 0x82E31020; continue 'dispatch;
	}
	// 82E31000: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E31004: 4BFDD4A5  bl 0x82e0e4a8
	ctx.lr = 0x82E31008;
	sub_82E0E4A8(ctx, base);
	// 82E31008: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3100C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E31010: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E31014: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E31018: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E3101C: 480026E5  bl 0x82e33700
	ctx.lr = 0x82E31020;
	sub_82E33700(ctx, base);
	// 82E31020: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E31024: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E31028: 419A0008  beq cr6, 0x82e31030
	if ctx.cr[6].eq {
	pc = 0x82E31030; continue 'dispatch;
	}
	// 82E3102C: 4B48F865  bl 0x822c0890
	ctx.lr = 0x82E31030;
	sub_822C0890(ctx, base);
	// 82E31030: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31034: 4BFF9ACD  bl 0x82e2ab00
	ctx.lr = 0x82E31038;
	sub_82E2AB00(ctx, base);
	// 82E31038: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E3103C: 48377180  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E31040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E31040 size=148
    let mut pc: u32 = 0x82E31040;
    'dispatch: loop {
        match pc {
            0x82E31040 => {
    //   block [0x82E31040..0x82E310D4)
	// 82E31040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E31044: 48377129  bl 0x831a816c
	ctx.lr = 0x82E31048;
	sub_831A8130(ctx, base);
	// 82E31048: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3104C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E31050: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31054: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E31058: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3105C: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E31060: 4BFF9A89  bl 0x82e2aae8
	ctx.lr = 0x82E31064;
	sub_82E2AAE8(ctx, base);
	// 82E31064: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E31068: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E3106C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31070: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E31074: 4BFFAC4D  bl 0x82e2bcc0
	ctx.lr = 0x82E31078;
	sub_82E2BCC0(ctx, base);
	// 82E31078: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E3107C: 419A0038  beq cr6, 0x82e310b4
	if ctx.cr[6].eq {
	pc = 0x82E310B4; continue 'dispatch;
	}
	// 82E31080: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E31084: 4BFDD425  bl 0x82e0e4a8
	ctx.lr = 0x82E31088;
	sub_82E0E4A8(ctx, base);
	// 82E31088: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3108C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E31090: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E31094: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E31098: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E3109C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E310A0: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E310A4: 409A000C  bne cr6, 0x82e310b0
	if !ctx.cr[6].eq {
	pc = 0x82E310B0; continue 'dispatch;
	}
	// 82E310A8: 48004961  bl 0x82e35a08
	ctx.lr = 0x82E310AC;
	sub_82E35A08(ctx, base);
	// 82E310AC: 48000008  b 0x82e310b4
	pc = 0x82E310B4; continue 'dispatch;
	// 82E310B0: 48004611  bl 0x82e356c0
	ctx.lr = 0x82E310B4;
	sub_82E356C0(ctx, base);
	// 82E310B4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E310B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E310BC: 419A0008  beq cr6, 0x82e310c4
	if ctx.cr[6].eq {
	pc = 0x82E310C4; continue 'dispatch;
	}
	// 82E310C0: 4B48F7D1  bl 0x822c0890
	ctx.lr = 0x82E310C4;
	sub_822C0890(ctx, base);
	// 82E310C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E310C8: 4BFF9A39  bl 0x82e2ab00
	ctx.lr = 0x82E310CC;
	sub_82E2AB00(ctx, base);
	// 82E310CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E310D0: 483770EC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E310D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E310D8 size=244
    let mut pc: u32 = 0x82E310D8;
    'dispatch: loop {
        match pc {
            0x82E310D8 => {
    //   block [0x82E310D8..0x82E311CC)
	// 82E310D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E310DC: 48377091  bl 0x831a816c
	ctx.lr = 0x82E310E0;
	sub_831A8130(ctx, base);
	// 82E310E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E310E4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E310E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E310EC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E310F0: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E310F4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E310F8: 4BFF99F1  bl 0x82e2aae8
	ctx.lr = 0x82E310FC;
	sub_82E2AAE8(ctx, base);
	// 82E310FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E31100: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E31104: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31108: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E3110C: 4BFFAC25  bl 0x82e2bd30
	ctx.lr = 0x82E31110;
	sub_82E2BD30(ctx, base);
	// 82E31110: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E31114: 419A0098  beq cr6, 0x82e311ac
	if ctx.cr[6].eq {
	pc = 0x82E311AC; continue 'dispatch;
	}
	// 82E31118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3111C: 4BFDD38D  bl 0x82e0e4a8
	ctx.lr = 0x82E31120;
	sub_82E0E4A8(ctx, base);
	// 82E31120: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E31124: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E31128: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E3112C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 82E31130: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E31134: 419A006C  beq cr6, 0x82e311a0
	if ctx.cr[6].eq {
	pc = 0x82E311A0; continue 'dispatch;
	}
	// 82E31138: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 82E3113C: 419A0054  beq cr6, 0x82e31190
	if ctx.cr[6].eq {
	pc = 0x82E31190; continue 'dispatch;
	}
	// 82E31140: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 82E31144: 419A003C  beq cr6, 0x82e31180
	if ctx.cr[6].eq {
	pc = 0x82E31180; continue 'dispatch;
	}
	// 82E31148: 2B0B03EA  cmplwi cr6, r11, 0x3ea
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1002 as u32, &mut ctx.xer);
	// 82E3114C: 419A0024  beq cr6, 0x82e31170
	if ctx.cr[6].eq {
	pc = 0x82E31170; continue 'dispatch;
	}
	// 82E31150: 2B0B03ED  cmplwi cr6, r11, 0x3ed
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1005 as u32, &mut ctx.xer);
	// 82E31154: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E31158: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E3115C: 419A000C  beq cr6, 0x82e31168
	if ctx.cr[6].eq {
	pc = 0x82E31168; continue 'dispatch;
	}
	// 82E31160: 48001821  bl 0x82e32980
	ctx.lr = 0x82E31164;
	sub_82E32980(ctx, base);
	// 82E31164: 48000048  b 0x82e311ac
	pc = 0x82E311AC; continue 'dispatch;
	// 82E31168: 48005129  bl 0x82e36290
	ctx.lr = 0x82E3116C;
	sub_82E36290(ctx, base);
	// 82E3116C: 48000040  b 0x82e311ac
	pc = 0x82E311AC; continue 'dispatch;
	// 82E31170: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E31174: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E31178: 48001491  bl 0x82e32608
	ctx.lr = 0x82E3117C;
	sub_82E32608(ctx, base);
	// 82E3117C: 48000030  b 0x82e311ac
	pc = 0x82E311AC; continue 'dispatch;
	// 82E31180: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E31184: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E31188: 48004F11  bl 0x82e36098
	ctx.lr = 0x82E3118C;
	sub_82E36098(ctx, base);
	// 82E3118C: 48000020  b 0x82e311ac
	pc = 0x82E311AC; continue 'dispatch;
	// 82E31190: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E31194: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E31198: 48003B89  bl 0x82e34d20
	ctx.lr = 0x82E3119C;
	sub_82E34D20(ctx, base);
	// 82E3119C: 48000010  b 0x82e311ac
	pc = 0x82E311AC; continue 'dispatch;
	// 82E311A0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E311A4: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E311A8: 48003739  bl 0x82e348e0
	ctx.lr = 0x82E311AC;
	sub_82E348E0(ctx, base);
	// 82E311AC: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E311B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E311B4: 419A0008  beq cr6, 0x82e311bc
	if ctx.cr[6].eq {
	pc = 0x82E311BC; continue 'dispatch;
	}
	// 82E311B8: 4B48F6D9  bl 0x822c0890
	ctx.lr = 0x82E311BC;
	sub_822C0890(ctx, base);
	// 82E311BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E311C0: 4BFF9941  bl 0x82e2ab00
	ctx.lr = 0x82E311C4;
	sub_82E2AB00(ctx, base);
	// 82E311C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E311C8: 48376FF4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E311D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E311D0 size=148
    let mut pc: u32 = 0x82E311D0;
    'dispatch: loop {
        match pc {
            0x82E311D0 => {
    //   block [0x82E311D0..0x82E31264)
	// 82E311D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E311D4: 48376F99  bl 0x831a816c
	ctx.lr = 0x82E311D8;
	sub_831A8130(ctx, base);
	// 82E311D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E311DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E311E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E311E4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E311E8: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E311EC: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E311F0: 4BFF98F9  bl 0x82e2aae8
	ctx.lr = 0x82E311F4;
	sub_82E2AAE8(ctx, base);
	// 82E311F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E311F8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E311FC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31200: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E31204: 4BFFDCDD  bl 0x82e2eee0
	ctx.lr = 0x82E31208;
	sub_82E2EEE0(ctx, base);
	// 82E31208: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E3120C: 419A0038  beq cr6, 0x82e31244
	if ctx.cr[6].eq {
	pc = 0x82E31244; continue 'dispatch;
	}
	// 82E31210: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E31214: 4BFDD295  bl 0x82e0e4a8
	ctx.lr = 0x82E31218;
	sub_82E0E4A8(ctx, base);
	// 82E31218: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3121C: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E31220: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E31224: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 82E31228: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E3122C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E31230: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E31234: 409A000C  bne cr6, 0x82e31240
	if !ctx.cr[6].eq {
	pc = 0x82E31240; continue 'dispatch;
	}
	// 82E31238: 48005399  bl 0x82e365d0
	ctx.lr = 0x82E3123C;
	sub_82E365D0(ctx, base);
	// 82E3123C: 48000008  b 0x82e31244
	pc = 0x82E31244; continue 'dispatch;
	// 82E31240: 480031E9  bl 0x82e34428
	ctx.lr = 0x82E31244;
	sub_82E34428(ctx, base);
	// 82E31244: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E31248: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E3124C: 419A0008  beq cr6, 0x82e31254
	if ctx.cr[6].eq {
	pc = 0x82E31254; continue 'dispatch;
	}
	// 82E31250: 4B48F641  bl 0x822c0890
	ctx.lr = 0x82E31254;
	sub_822C0890(ctx, base);
	// 82E31254: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31258: 4BFF98A9  bl 0x82e2ab00
	ctx.lr = 0x82E3125C;
	sub_82E2AB00(ctx, base);
	// 82E3125C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E31260: 48376F5C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E31268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E31268 size=128
    let mut pc: u32 = 0x82E31268;
    'dispatch: loop {
        match pc {
            0x82E31268 => {
    //   block [0x82E31268..0x82E312E8)
	// 82E31268: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3126C: 48376F01  bl 0x831a816c
	ctx.lr = 0x82E31270;
	sub_831A8130(ctx, base);
	// 82E31270: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E31274: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E31278: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3127C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E31280: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E31284: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E31288: 4BFF9861  bl 0x82e2aae8
	ctx.lr = 0x82E3128C;
	sub_82E2AAE8(ctx, base);
	// 82E3128C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E31290: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E31294: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31298: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E3129C: 4BFFD735  bl 0x82e2e9d0
	ctx.lr = 0x82E312A0;
	sub_82E2E9D0(ctx, base);
	// 82E312A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E312A4: 419A0024  beq cr6, 0x82e312c8
	if ctx.cr[6].eq {
	pc = 0x82E312C8; continue 'dispatch;
	}
	// 82E312A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E312AC: 4BFDD1FD  bl 0x82e0e4a8
	ctx.lr = 0x82E312B0;
	sub_82E0E4A8(ctx, base);
	// 82E312B0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E312B4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E312B8: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E312BC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E312C0: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E312C4: 48002E0D  bl 0x82e340d0
	ctx.lr = 0x82E312C8;
	sub_82E340D0(ctx, base);
	// 82E312C8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E312CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E312D0: 419A0008  beq cr6, 0x82e312d8
	if ctx.cr[6].eq {
	pc = 0x82E312D8; continue 'dispatch;
	}
	// 82E312D4: 4B48F5BD  bl 0x822c0890
	ctx.lr = 0x82E312D8;
	sub_822C0890(ctx, base);
	// 82E312D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E312DC: 4BFF9825  bl 0x82e2ab00
	ctx.lr = 0x82E312E0;
	sub_82E2AB00(ctx, base);
	// 82E312E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E312E4: 48376ED8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E312E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E312E8 size=128
    let mut pc: u32 = 0x82E312E8;
    'dispatch: loop {
        match pc {
            0x82E312E8 => {
    //   block [0x82E312E8..0x82E31368)
	// 82E312E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E312EC: 48376E81  bl 0x831a816c
	ctx.lr = 0x82E312F0;
	sub_831A8130(ctx, base);
	// 82E312F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E312F4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E312F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E312FC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E31300: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E31304: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E31308: 4BFF97E1  bl 0x82e2aae8
	ctx.lr = 0x82E3130C;
	sub_82E2AAE8(ctx, base);
	// 82E3130C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E31310: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E31314: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31318: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E3131C: 4BFFD725  bl 0x82e2ea40
	ctx.lr = 0x82E31320;
	sub_82E2EA40(ctx, base);
	// 82E31320: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E31324: 419A0024  beq cr6, 0x82e31348
	if ctx.cr[6].eq {
	pc = 0x82E31348; continue 'dispatch;
	}
	// 82E31328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3132C: 4BFDD17D  bl 0x82e0e4a8
	ctx.lr = 0x82E31330;
	sub_82E0E4A8(ctx, base);
	// 82E31330: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E31334: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E31338: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E3133C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E31340: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E31344: 48002F5D  bl 0x82e342a0
	ctx.lr = 0x82E31348;
	sub_82E342A0(ctx, base);
	// 82E31348: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E3134C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E31350: 419A0008  beq cr6, 0x82e31358
	if ctx.cr[6].eq {
	pc = 0x82E31358; continue 'dispatch;
	}
	// 82E31354: 4B48F53D  bl 0x822c0890
	ctx.lr = 0x82E31358;
	sub_822C0890(ctx, base);
	// 82E31358: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3135C: 4BFF97A5  bl 0x82e2ab00
	ctx.lr = 0x82E31360;
	sub_82E2AB00(ctx, base);
	// 82E31360: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E31364: 48376E58  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E31368(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E31368 size=128
    let mut pc: u32 = 0x82E31368;
    'dispatch: loop {
        match pc {
            0x82E31368 => {
    //   block [0x82E31368..0x82E313E8)
	// 82E31368: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3136C: 48376E01  bl 0x831a816c
	ctx.lr = 0x82E31370;
	sub_831A8130(ctx, base);
	// 82E31370: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E31374: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E31378: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3137C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E31380: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E31384: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82E31388: 4BFF9761  bl 0x82e2aae8
	ctx.lr = 0x82E3138C;
	sub_82E2AAE8(ctx, base);
	// 82E3138C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E31390: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E31394: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31398: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E3139C: 4BFFB4C5  bl 0x82e2c860
	ctx.lr = 0x82E313A0;
	sub_82E2C860(ctx, base);
	// 82E313A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E313A4: 419A0024  beq cr6, 0x82e313c8
	if ctx.cr[6].eq {
	pc = 0x82E313C8; continue 'dispatch;
	}
	// 82E313A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E313AC: 4BFDD0FD  bl 0x82e0e4a8
	ctx.lr = 0x82E313B0;
	sub_82E0E4A8(ctx, base);
	// 82E313B0: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E313B4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82E313B8: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E313BC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E313C0: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E313C4: 4800212D  bl 0x82e334f0
	ctx.lr = 0x82E313C8;
	sub_82E334F0(ctx, base);
	// 82E313C8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E313CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E313D0: 419A0008  beq cr6, 0x82e313d8
	if ctx.cr[6].eq {
	pc = 0x82E313D8; continue 'dispatch;
	}
	// 82E313D4: 4B48F4BD  bl 0x822c0890
	ctx.lr = 0x82E313D8;
	sub_822C0890(ctx, base);
	// 82E313D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E313DC: 4BFF9725  bl 0x82e2ab00
	ctx.lr = 0x82E313E0;
	sub_82E2AB00(ctx, base);
	// 82E313E0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E313E4: 48376DD8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E313E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E313E8 size=276
    let mut pc: u32 = 0x82E313E8;
    'dispatch: loop {
        match pc {
            0x82E313E8 => {
    //   block [0x82E313E8..0x82E314FC)
	// 82E313E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E313EC: 48376D7D  bl 0x831a8168
	ctx.lr = 0x82E313F0;
	sub_831A8130(ctx, base);
	// 82E313F0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E313F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E313F8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E313FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E31400: 80860000  lwz r4, 0(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E31404: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E31408: 4BFF96E1  bl 0x82e2aae8
	ctx.lr = 0x82E3140C;
	sub_82E2AAE8(ctx, base);
	// 82E3140C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E31410: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82E31414: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E31418: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E3141C: 4BFFA9F5  bl 0x82e2be10
	ctx.lr = 0x82E31420;
	sub_82E2BE10(ctx, base);
	// 82E31420: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82E31424: 419A00B8  beq cr6, 0x82e314dc
	if ctx.cr[6].eq {
	pc = 0x82E314DC; continue 'dispatch;
	}
	// 82E31428: 83E10060  lwz r31, 0x60(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82E3142C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E31430: 4BFCD439  bl 0x82dfe868
	ctx.lr = 0x82E31434;
	sub_82DFE868(ctx, base);
	// 82E31434: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E31438: 408200A4  bne 0x82e314dc
	if !ctx.cr[0].eq {
	pc = 0x82E314DC; continue 'dispatch;
	}
	// 82E3143C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E31440: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E31444: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E31448: 483CA2F9  bl 0x831fb740
	ctx.lr = 0x82E3144C;
	sub_831FB740(ctx, base);
	// 82E3144C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E31450: 4BFCD409  bl 0x82dfe858
	ctx.lr = 0x82E31454;
	sub_82DFE858(ctx, base);
	// 82E31454: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E31458: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82E3145C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E31460: 4099007C  ble cr6, 0x82e314dc
	if !ctx.cr[6].gt {
	pc = 0x82E314DC; continue 'dispatch;
	}
	// 82E31464: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 82E31468: 3BBF0078  addi r29, r31, 0x78
	ctx.r[29].s64 = ctx.r[31].s64 + 120;
	// 82E3146C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31470: 4BFC1D41  bl 0x82df31b0
	ctx.lr = 0x82E31474;
	sub_82DF31B0(ctx, base);
	// 82E31474: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E31478: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3147C: 4BFC258D  bl 0x82df3a08
	ctx.lr = 0x82E31480;
	sub_82DF3A08(ctx, base);
	// 82E31480: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E31484: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E31488: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82E3148C: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 82E31490: 4BFFD9E1  bl 0x82e2ee70
	ctx.lr = 0x82E31494;
	sub_82E2EE70(ctx, base);
	// 82E31494: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E31498: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 82E3149C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82E314A0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E314A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E314A8: 4B492FB9  bl 0x822c4460
	ctx.lr = 0x82E314AC;
	sub_822C4460(ctx, base);
	// 82E314AC: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82E314B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E314B4: 419A0008  beq cr6, 0x82e314bc
	if ctx.cr[6].eq {
	pc = 0x82E314BC; continue 'dispatch;
	}
	// 82E314B8: 4B48F3D9  bl 0x822c0890
	ctx.lr = 0x82E314BC;
	sub_822C0890(ctx, base);
	// 82E314BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E314C0: 4BFC1F69  bl 0x82df3428
	ctx.lr = 0x82E314C4;
	sub_82DF3428(ctx, base);
	// 82E314C4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E314C8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82E314CC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82E314D0: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82E314D4: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82E314D8: 4198FF94  blt cr6, 0x82e3146c
	if ctx.cr[6].lt {
	pc = 0x82E3146C; continue 'dispatch;
	}
	// 82E314DC: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E314E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E314E4: 419A0008  beq cr6, 0x82e314ec
	if ctx.cr[6].eq {
	pc = 0x82E314EC; continue 'dispatch;
	}
	// 82E314E8: 4B48F3A9  bl 0x822c0890
	ctx.lr = 0x82E314EC;
	sub_822C0890(ctx, base);
	// 82E314EC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E314F0: 4BFF9611  bl 0x82e2ab00
	ctx.lr = 0x82E314F4;
	sub_82E2AB00(ctx, base);
	// 82E314F4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E314F8: 48376CC0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E31500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E31500 size=36
    let mut pc: u32 = 0x82E31500;
    'dispatch: loop {
        match pc {
            0x82E31500 => {
    //   block [0x82E31500..0x82E31524)
	// 82E31500: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E31504: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E31508: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E3150C: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82E31510: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82E31514: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E31518: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3151C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E31520: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E31528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E31528 size=40
    let mut pc: u32 = 0x82E31528;
    'dispatch: loop {
        match pc {
            0x82E31528 => {
    //   block [0x82E31528..0x82E31550)
	// 82E31528: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E3152C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E31530: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E31534: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82E31538: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82E3153C: 890B0008  lbz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E31540: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E31544: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E31548: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E3154C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E31550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E31550 size=44
    let mut pc: u32 = 0x82E31550;
    'dispatch: loop {
        match pc {
            0x82E31550 => {
    //   block [0x82E31550..0x82E3157C)
	// 82E31550: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E31554: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E31558: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82E3155C: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82E31560: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82E31564: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E31568: 890B0008  lbz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E3156C: 80EB0004  lwz r7, 4(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E31570: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E31574: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E31578: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E31580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E31580 size=128
    let mut pc: u32 = 0x82E31580;
    'dispatch: loop {
        match pc {
            0x82E31580 => {
    //   block [0x82E31580..0x82E31600)
	// 82E31580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E31584: 48376BE9  bl 0x831a816c
	ctx.lr = 0x82E31588;
	sub_831A8130(ctx, base);
	// 82E31588: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3158C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82E31590: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E31594: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E31598: 3BEB660C  addi r31, r11, 0x660c
	ctx.r[31].s64 = ctx.r[11].s64 + 26124;
	// 82E3159C: 816A6614  lwz r11, 0x6614(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26132 as u32) ) } as u64;
	// 82E315A0: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E315A4: 40820024  bne 0x82e315c8
	if !ctx.cr[0].eq {
	pc = 0x82E315C8; continue 'dispatch;
	}
	// 82E315A8: 3D2082E3  lis r9, -0x7d1d
	ctx.r[9].s64 = -2099052544;
	// 82E315AC: 3D0082E3  lis r8, -0x7d1d
	ctx.r[8].s64 = -2099052544;
	// 82E315B0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82E315B4: 39291500  addi r9, r9, 0x1500
	ctx.r[9].s64 = ctx.r[9].s64 + 5376;
	// 82E315B8: 39080608  addi r8, r8, 0x608
	ctx.r[8].s64 = ctx.r[8].s64 + 1544;
	// 82E315BC: 916A6614  stw r11, 0x6614(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(26132 as u32), ctx.r[11].u32 ) };
	// 82E315C0: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E315C4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E315C8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E315CC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E315D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E315D4: 38BE0008  addi r5, r30, 8
	ctx.r[5].s64 = ctx.r[30].s64 + 8;
	// 82E315D8: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82E315DC: 88C10050  lbz r6, 0x50(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E315E0: 4BFDA111  bl 0x82e0b6f0
	ctx.lr = 0x82E315E4;
	sub_82E0B6F0(ctx, base);
	// 82E315E4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E315E8: 4182000C  beq 0x82e315f4
	if ctx.cr[0].eq {
	pc = 0x82E315F4; continue 'dispatch;
	}
	// 82E315EC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E315F0: 48000008  b 0x82e315f8
	pc = 0x82E315F8; continue 'dispatch;
	// 82E315F4: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E315F8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E315FC: 48376BC0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E31600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E31600 size=140
    let mut pc: u32 = 0x82E31600;
    'dispatch: loop {
        match pc {
            0x82E31600 => {
    //   block [0x82E31600..0x82E3168C)
	// 82E31600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E31604: 48376B69  bl 0x831a816c
	ctx.lr = 0x82E31608;
	sub_831A8130(ctx, base);
	// 82E31608: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3160C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82E31610: F8A100A8  std r5, 0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[5].u64 ) };
	// 82E31614: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E31618: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E3161C: 3BEB6618  addi r31, r11, 0x6618
	ctx.r[31].s64 = ctx.r[11].s64 + 26136;
	// 82E31620: 816A6620  lwz r11, 0x6620(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26144 as u32) ) } as u64;
	// 82E31624: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E31628: 40820024  bne 0x82e3164c
	if !ctx.cr[0].eq {
	pc = 0x82E3164C; continue 'dispatch;
	}
	// 82E3162C: 3D2082E3  lis r9, -0x7d1d
	ctx.r[9].s64 = -2099052544;
	// 82E31630: 3D0082E3  lis r8, -0x7d1d
	ctx.r[8].s64 = -2099052544;
	// 82E31634: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82E31638: 39291528  addi r9, r9, 0x1528
	ctx.r[9].s64 = ctx.r[9].s64 + 5416;
	// 82E3163C: 39080650  addi r8, r8, 0x650
	ctx.r[8].s64 = ctx.r[8].s64 + 1616;
	// 82E31640: 916A6620  stw r11, 0x6620(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(26144 as u32), ctx.r[11].u32 ) };
	// 82E31644: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E31648: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E3164C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E31650: 814100A8  lwz r10, 0xa8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 82E31654: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E31658: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3165C: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 82E31660: 794507E6  rldicr r5, r10, 0x20, 0x3f
	ctx.r[5].u64 = (ctx.r[10].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 82E31664: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82E31668: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E3166C: 4BFFEAB5  bl 0x82e30120
	ctx.lr = 0x82E31670;
	sub_82E30120(ctx, base);
	// 82E31670: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E31674: 4182000C  beq 0x82e31680
	if ctx.cr[0].eq {
	pc = 0x82E31680; continue 'dispatch;
	}
	// 82E31678: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E3167C: 48000008  b 0x82e31684
	pc = 0x82E31684; continue 'dispatch;
	// 82E31680: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E31684: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E31688: 48376B34  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E31690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E31690 size=128
    let mut pc: u32 = 0x82E31690;
    'dispatch: loop {
        match pc {
            0x82E31690 => {
    //   block [0x82E31690..0x82E31710)
	// 82E31690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E31694: 48376AD9  bl 0x831a816c
	ctx.lr = 0x82E31698;
	sub_831A8130(ctx, base);
	// 82E31698: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3169C: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 82E316A0: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 82E316A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E316A8: 3BEB6624  addi r31, r11, 0x6624
	ctx.r[31].s64 = ctx.r[11].s64 + 26148;
	// 82E316AC: 816A662C  lwz r11, 0x662c(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26156 as u32) ) } as u64;
	// 82E316B0: 556907FF  clrlwi. r9, r11, 0x1f
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82E316B4: 40820024  bne 0x82e316d8
	if !ctx.cr[0].eq {
	pc = 0x82E316D8; continue 'dispatch;
	}
	// 82E316B8: 3D2082E3  lis r9, -0x7d1d
	ctx.r[9].s64 = -2099052544;
	// 82E316BC: 3D0082E3  lis r8, -0x7d1d
	ctx.r[8].s64 = -2099052544;
	// 82E316C0: 616B0001  ori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 | 1;
	// 82E316C4: 39291550  addi r9, r9, 0x1550
	ctx.r[9].s64 = ctx.r[9].s64 + 5456;
	// 82E316C8: 39080698  addi r8, r8, 0x698
	ctx.r[8].s64 = ctx.r[8].s64 + 1688;
	// 82E316CC: 916A662C  stw r11, 0x662c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(26156 as u32), ctx.r[11].u32 ) };
	// 82E316D0: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E316D4: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82E316D8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E316DC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82E316E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E316E4: 38DE0008  addi r6, r30, 8
	ctx.r[6].s64 = ctx.r[30].s64 + 8;
	// 82E316E8: 9BAB0000  stb r29, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u8 ) };
	// 82E316EC: 88E10050  lbz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E316F0: 4BFFEAA1  bl 0x82e30190
	ctx.lr = 0x82E316F4;
	sub_82E30190(ctx, base);
	// 82E316F4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E316F8: 4182000C  beq 0x82e31704
	if ctx.cr[0].eq {
	pc = 0x82E31704; continue 'dispatch;
	}
	// 82E316FC: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82E31700: 48000008  b 0x82e31708
	pc = 0x82E31708; continue 'dispatch;
	// 82E31704: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82E31708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E3170C: 48376AB0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E31710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E31710 size=3548
    let mut pc: u32 = 0x82E31710;
    'dispatch: loop {
        match pc {
            0x82E31710 => {
    //   block [0x82E31710..0x82E324EC)
	// 82E31710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E31714: 48376A51  bl 0x831a8164
	ctx.lr = 0x82E31718;
	sub_831A8130(ctx, base);
	// 82E31718: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3171C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E31720: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E31724: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E31728: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3172C: 388BA810  addi r4, r11, -0x57f0
	ctx.r[4].s64 = ctx.r[11].s64 + -22512;
	// 82E31730: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82E31734: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82E31738: 4BFC22D1  bl 0x82df3a08
	ctx.lr = 0x82E3173C;
	sub_82DF3A08(ctx, base);
	// 82E3173C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82E31740: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31744: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82E31748: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E3174C: 388BF3A0  addi r4, r11, -0xc60
	ctx.r[4].s64 = ctx.r[11].s64 + -3168;
	// 82E31750: 4B4A09D1  bl 0x822d2120
	ctx.lr = 0x82E31754;
	sub_822D2120(ctx, base);
	// 82E31754: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31758: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E3175C: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E31760: 396B06E0  addi r11, r11, 0x6e0
	ctx.r[11].s64 = ctx.r[11].s64 + 1760;
	// 82E31764: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31768: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E3176C: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E31770: 4BFFFE11  bl 0x82e31580
	ctx.lr = 0x82E31774;
	sub_82E31580(ctx, base);
	// 82E31774: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31778: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E3177C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82E31780: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E31784: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31788: 4BFD1A81  bl 0x82e03208
	ctx.lr = 0x82E3178C;
	sub_82E03208(ctx, base);
	// 82E3178C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31790: 4BFC1C99  bl 0x82df3428
	ctx.lr = 0x82E31794;
	sub_82DF3428(ctx, base);
	// 82E31794: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E31798: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3179C: 388BA828  addi r4, r11, -0x57d8
	ctx.r[4].s64 = ctx.r[11].s64 + -22488;
	// 82E317A0: 4BFC2269  bl 0x82df3a08
	ctx.lr = 0x82E317A4;
	sub_82DF3A08(ctx, base);
	// 82E317A4: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E317A8: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E317AC: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E317B0: 388BF410  addi r4, r11, -0xbf0
	ctx.r[4].s64 = ctx.r[11].s64 + -3056;
	// 82E317B4: 4B4A096D  bl 0x822d2120
	ctx.lr = 0x82E317B8;
	sub_822D2120(ctx, base);
	// 82E317B8: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E317BC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E317C0: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E317C4: 396B0790  addi r11, r11, 0x790
	ctx.r[11].s64 = ctx.r[11].s64 + 1936;
	// 82E317C8: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82E317CC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E317D0: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E317D4: 4BFFFDAD  bl 0x82e31580
	ctx.lr = 0x82E317D8;
	sub_82E31580(ctx, base);
	// 82E317D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E317DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E317E0: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82E317E4: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E317E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E317EC: 4BFD1A1D  bl 0x82e03208
	ctx.lr = 0x82E317F0;
	sub_82E03208(ctx, base);
	// 82E317F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E317F4: 4BFC1C35  bl 0x82df3428
	ctx.lr = 0x82E317F8;
	sub_82DF3428(ctx, base);
	// 82E317F8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E317FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31800: 388BA82C  addi r4, r11, -0x57d4
	ctx.r[4].s64 = ctx.r[11].s64 + -22484;
	// 82E31804: 4BFC2205  bl 0x82df3a08
	ctx.lr = 0x82E31808;
	sub_82DF3A08(ctx, base);
	// 82E31808: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E3180C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31810: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31814: 388BF480  addi r4, r11, -0xb80
	ctx.r[4].s64 = ctx.r[11].s64 + -2944;
	// 82E31818: 4B4A0909  bl 0x822d2120
	ctx.lr = 0x82E3181C;
	sub_822D2120(ctx, base);
	// 82E3181C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31820: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E31824: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E31828: 396B0208  addi r11, r11, 0x208
	ctx.r[11].s64 = ctx.r[11].s64 + 520;
	// 82E3182C: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82E31830: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E31834: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E31838: 4BFFFD49  bl 0x82e31580
	ctx.lr = 0x82E3183C;
	sub_82E31580(ctx, base);
	// 82E3183C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E31840: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82E31844: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E31848: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3184C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31850: 4BFD19B9  bl 0x82e03208
	ctx.lr = 0x82E31854;
	sub_82E03208(ctx, base);
	// 82E31854: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31858: 4BFC1BD1  bl 0x82df3428
	ctx.lr = 0x82E3185C;
	sub_82DF3428(ctx, base);
	// 82E3185C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E31860: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31864: 388BA834  addi r4, r11, -0x57cc
	ctx.r[4].s64 = ctx.r[11].s64 + -22476;
	// 82E31868: 4BFC21A1  bl 0x82df3a08
	ctx.lr = 0x82E3186C;
	sub_82DF3A08(ctx, base);
	// 82E3186C: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31870: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31874: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31878: 388BF4F0  addi r4, r11, -0xb10
	ctx.r[4].s64 = ctx.r[11].s64 + -2832;
	// 82E3187C: 4B4A08A5  bl 0x822d2120
	ctx.lr = 0x82E31880;
	sub_822D2120(ctx, base);
	// 82E31880: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31884: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E31888: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E3188C: 396B0810  addi r11, r11, 0x810
	ctx.r[11].s64 = ctx.r[11].s64 + 2064;
	// 82E31890: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82E31894: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E31898: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E3189C: 4BFFFCE5  bl 0x82e31580
	ctx.lr = 0x82E318A0;
	sub_82E31580(ctx, base);
	// 82E318A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E318A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E318A8: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82E318AC: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E318B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E318B4: 4BFD1955  bl 0x82e03208
	ctx.lr = 0x82E318B8;
	sub_82E03208(ctx, base);
	// 82E318B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E318BC: 4BFC1B6D  bl 0x82df3428
	ctx.lr = 0x82E318C0;
	sub_82DF3428(ctx, base);
	// 82E318C0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E318C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E318C8: 388BA840  addi r4, r11, -0x57c0
	ctx.r[4].s64 = ctx.r[11].s64 + -22464;
	// 82E318CC: 4BFC213D  bl 0x82df3a08
	ctx.lr = 0x82E318D0;
	sub_82DF3A08(ctx, base);
	// 82E318D0: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E318D4: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E318D8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E318DC: 388BF560  addi r4, r11, -0xaa0
	ctx.r[4].s64 = ctx.r[11].s64 + -2720;
	// 82E318E0: 4B4A0841  bl 0x822d2120
	ctx.lr = 0x82E318E4;
	sub_822D2120(ctx, base);
	// 82E318E4: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E318E8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E318EC: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E318F0: 396B0350  addi r11, r11, 0x350
	ctx.r[11].s64 = ctx.r[11].s64 + 848;
	// 82E318F4: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82E318F8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E318FC: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E31900: 4BFFFC81  bl 0x82e31580
	ctx.lr = 0x82E31904;
	sub_82E31580(ctx, base);
	// 82E31904: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31908: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E3190C: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82E31910: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E31914: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31918: 4BFD18F1  bl 0x82e03208
	ctx.lr = 0x82E3191C;
	sub_82E03208(ctx, base);
	// 82E3191C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31920: 4BFC1B09  bl 0x82df3428
	ctx.lr = 0x82E31924;
	sub_82DF3428(ctx, base);
	// 82E31924: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E31928: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3192C: 388BA850  addi r4, r11, -0x57b0
	ctx.r[4].s64 = ctx.r[11].s64 + -22448;
	// 82E31930: 4BFC20D9  bl 0x82df3a08
	ctx.lr = 0x82E31934;
	sub_82DF3A08(ctx, base);
	// 82E31934: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31938: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E3193C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31940: 388BF5D0  addi r4, r11, -0xa30
	ctx.r[4].s64 = ctx.r[11].s64 + -2608;
	// 82E31944: 4B4A07DD  bl 0x822d2120
	ctx.lr = 0x82E31948;
	sub_822D2120(ctx, base);
	// 82E31948: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82E3194C: 9B810064  stb r28, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u8 ) };
	// 82E31950: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31954: E9410060  ld r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E31958: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E3195C: F9410064  std r10, 0x64(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u64 ) };
	// 82E31960: 396B0940  addi r11, r11, 0x940
	ctx.r[11].s64 = ctx.r[11].s64 + 2368;
	// 82E31964: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82E31968: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E3196C: 794507E6  rldicr r5, r10, 0x20, 0x3f
	ctx.r[5].u64 = (ctx.r[10].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 82E31970: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E31974: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82E31978: 4BFFFC89  bl 0x82e31600
	ctx.lr = 0x82E3197C;
	sub_82E31600(ctx, base);
	// 82E3197C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E31980: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82E31984: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31988: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E3198C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31990: 4BFD1879  bl 0x82e03208
	ctx.lr = 0x82E31994;
	sub_82E03208(ctx, base);
	// 82E31994: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31998: 4BFC1A91  bl 0x82df3428
	ctx.lr = 0x82E3199C;
	sub_82DF3428(ctx, base);
	// 82E3199C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E319A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E319A4: 388BA854  addi r4, r11, -0x57ac
	ctx.r[4].s64 = ctx.r[11].s64 + -22444;
	// 82E319A8: 4BFC2061  bl 0x82df3a08
	ctx.lr = 0x82E319AC;
	sub_82DF3A08(ctx, base);
	// 82E319AC: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E319B0: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E319B4: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E319B8: 388BF640  addi r4, r11, -0x9c0
	ctx.r[4].s64 = ctx.r[11].s64 + -2496;
	// 82E319BC: 4B4A0765  bl 0x822d2120
	ctx.lr = 0x82E319C0;
	sub_822D2120(ctx, base);
	// 82E319C0: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E319C4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E319C8: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E319CC: 396B08C0  addi r11, r11, 0x8c0
	ctx.r[11].s64 = ctx.r[11].s64 + 2240;
	// 82E319D0: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82E319D4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E319D8: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E319DC: 4BFFFBA5  bl 0x82e31580
	ctx.lr = 0x82E319E0;
	sub_82E31580(ctx, base);
	// 82E319E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E319E4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E319E8: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82E319EC: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E319F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E319F4: 4BFD1815  bl 0x82e03208
	ctx.lr = 0x82E319F8;
	sub_82E03208(ctx, base);
	// 82E319F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E319FC: 4BFC1A2D  bl 0x82df3428
	ctx.lr = 0x82E31A00;
	sub_82DF3428(ctx, base);
	// 82E31A00: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E31A04: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31A08: 388BA85C  addi r4, r11, -0x57a4
	ctx.r[4].s64 = ctx.r[11].s64 + -22436;
	// 82E31A0C: 4BFC1FFD  bl 0x82df3a08
	ctx.lr = 0x82E31A10;
	sub_82DF3A08(ctx, base);
	// 82E31A10: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31A14: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31A18: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31A1C: 388BF6B0  addi r4, r11, -0x950
	ctx.r[4].s64 = ctx.r[11].s64 + -2384;
	// 82E31A20: 4B4A0701  bl 0x822d2120
	ctx.lr = 0x82E31A24;
	sub_822D2120(ctx, base);
	// 82E31A24: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31A28: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E31A2C: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E31A30: 396B0400  addi r11, r11, 0x400
	ctx.r[11].s64 = ctx.r[11].s64 + 1024;
	// 82E31A34: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 82E31A38: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E31A3C: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E31A40: 4BFFFB41  bl 0x82e31580
	ctx.lr = 0x82E31A44;
	sub_82E31580(ctx, base);
	// 82E31A44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31A48: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E31A4C: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82E31A50: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82E31A54: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31A58: 4BFD17B1  bl 0x82e03208
	ctx.lr = 0x82E31A5C;
	sub_82E03208(ctx, base);
	// 82E31A5C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31A60: 4BFC19C9  bl 0x82df3428
	ctx.lr = 0x82E31A64;
	sub_82DF3428(ctx, base);
	// 82E31A64: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E31A68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31A6C: 388BA868  addi r4, r11, -0x5798
	ctx.r[4].s64 = ctx.r[11].s64 + -22424;
	// 82E31A70: 4BFC1F99  bl 0x82df3a08
	ctx.lr = 0x82E31A74;
	sub_82DF3A08(ctx, base);
	// 82E31A74: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31A78: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31A7C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31A80: 388BF720  addi r4, r11, -0x8e0
	ctx.r[4].s64 = ctx.r[11].s64 + -2272;
	// 82E31A84: 4B4A069D  bl 0x822d2120
	ctx.lr = 0x82E31A88;
	sub_822D2120(ctx, base);
	// 82E31A88: 9B810064  stb r28, 0x64(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u8 ) };
	// 82E31A8C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31A90: 93C10060  stw r30, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 82E31A94: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E31A98: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E31A9C: 396B09D8  addi r11, r11, 0x9d8
	ctx.r[11].s64 = ctx.r[11].s64 + 2520;
	// 82E31AA0: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82E31AA4: E9610060  ld r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E31AA8: F9610074  std r11, 0x74(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u64 ) };
	// 82E31AAC: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 82E31AB0: E8810070  ld r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82E31AB4: 796507E6  rldicr r5, r11, 0x20, 0x3f
	ctx.r[5].u64 = (ctx.r[11].u64).rotate_left(32) & 0xFFFFFFFFFFFFFFFF;
	// 82E31AB8: 4BFFFB49  bl 0x82e31600
	ctx.lr = 0x82E31ABC;
	sub_82E31600(ctx, base);
	// 82E31ABC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31AC0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E31AC4: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82E31AC8: 38A100B0  addi r5, r1, 0xb0
	ctx.r[5].s64 = ctx.r[1].s64 + 176;
	// 82E31ACC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31AD0: 4BFD1739  bl 0x82e03208
	ctx.lr = 0x82E31AD4;
	sub_82E03208(ctx, base);
	// 82E31AD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31AD8: 4BFC1951  bl 0x82df3428
	ctx.lr = 0x82E31ADC;
	sub_82DF3428(ctx, base);
	// 82E31ADC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E31AE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31AE4: 388BA86C  addi r4, r11, -0x5794
	ctx.r[4].s64 = ctx.r[11].s64 + -22420;
	// 82E31AE8: 4BFC1F21  bl 0x82df3a08
	ctx.lr = 0x82E31AEC;
	sub_82DF3A08(ctx, base);
	// 82E31AEC: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E31AF0: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31AF4: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E31AF8: 388BF790  addi r4, r11, -0x870
	ctx.r[4].s64 = ctx.r[11].s64 + -2160;
	// 82E31AFC: 4B4A0625  bl 0x822d2120
	ctx.lr = 0x82E31B00;
	sub_822D2120(ctx, base);
	// 82E31B00: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31B04: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31B08: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E31B0C: 396B0A70  addi r11, r11, 0xa70
	ctx.r[11].s64 = ctx.r[11].s64 + 2672;
	// 82E31B10: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31B14: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E31B18: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E31B1C: 4BFFFA65  bl 0x82e31580
	ctx.lr = 0x82E31B20;
	sub_82E31580(ctx, base);
	// 82E31B20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31B24: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E31B28: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E31B2C: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E31B30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31B34: 4BFD16D5  bl 0x82e03208
	ctx.lr = 0x82E31B38;
	sub_82E03208(ctx, base);
	// 82E31B38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31B3C: 4BFC18ED  bl 0x82df3428
	ctx.lr = 0x82E31B40;
	sub_82DF3428(ctx, base);
	// 82E31B40: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E31B44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31B48: 388BA874  addi r4, r11, -0x578c
	ctx.r[4].s64 = ctx.r[11].s64 + -22412;
	// 82E31B4C: 4BFC1EBD  bl 0x82df3a08
	ctx.lr = 0x82E31B50;
	sub_82DF3A08(ctx, base);
	// 82E31B50: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E31B54: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31B58: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E31B5C: 388BF800  addi r4, r11, -0x800
	ctx.r[4].s64 = ctx.r[11].s64 + -2048;
	// 82E31B60: 4B4A05C1  bl 0x822d2120
	ctx.lr = 0x82E31B64;
	sub_822D2120(ctx, base);
	// 82E31B64: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31B68: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31B6C: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E31B70: 396B0AF0  addi r11, r11, 0xaf0
	ctx.r[11].s64 = ctx.r[11].s64 + 2800;
	// 82E31B74: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31B78: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E31B7C: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E31B80: 4BFFFA01  bl 0x82e31580
	ctx.lr = 0x82E31B84;
	sub_82E31580(ctx, base);
	// 82E31B84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31B88: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E31B8C: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E31B90: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E31B94: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31B98: 4BFD1671  bl 0x82e03208
	ctx.lr = 0x82E31B9C;
	sub_82E03208(ctx, base);
	// 82E31B9C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31BA0: 4BFC1889  bl 0x82df3428
	ctx.lr = 0x82E31BA4;
	sub_82DF3428(ctx, base);
	// 82E31BA4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E31BA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31BAC: 388BA87C  addi r4, r11, -0x5784
	ctx.r[4].s64 = ctx.r[11].s64 + -22404;
	// 82E31BB0: 4BFC1E59  bl 0x82df3a08
	ctx.lr = 0x82E31BB4;
	sub_82DF3A08(ctx, base);
	// 82E31BB4: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E31BB8: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31BBC: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E31BC0: 388BF870  addi r4, r11, -0x790
	ctx.r[4].s64 = ctx.r[11].s64 + -1936;
	// 82E31BC4: 4B4A055D  bl 0x822d2120
	ctx.lr = 0x82E31BC8;
	sub_822D2120(ctx, base);
	// 82E31BC8: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E31BCC: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31BD0: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31BD4: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31BD8: 396B0B70  addi r11, r11, 0xb70
	ctx.r[11].s64 = ctx.r[11].s64 + 2928;
	// 82E31BDC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E31BE0: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E31BE4: 4BFFF99D  bl 0x82e31580
	ctx.lr = 0x82E31BE8;
	sub_82E31580(ctx, base);
	// 82E31BE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31BEC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E31BF0: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E31BF4: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E31BF8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31BFC: 4BFD160D  bl 0x82e03208
	ctx.lr = 0x82E31C00;
	sub_82E03208(ctx, base);
	// 82E31C00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31C04: 4BFC1825  bl 0x82df3428
	ctx.lr = 0x82E31C08;
	sub_82DF3428(ctx, base);
	// 82E31C08: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E31C0C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31C10: 388BA888  addi r4, r11, -0x5778
	ctx.r[4].s64 = ctx.r[11].s64 + -22392;
	// 82E31C14: 4BFC1DF5  bl 0x82df3a08
	ctx.lr = 0x82E31C18;
	sub_82DF3A08(ctx, base);
	// 82E31C18: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E31C1C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31C20: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E31C24: 388BF8E0  addi r4, r11, -0x720
	ctx.r[4].s64 = ctx.r[11].s64 + -1824;
	// 82E31C28: 4B4A04F9  bl 0x822d2120
	ctx.lr = 0x82E31C2C;
	sub_822D2120(ctx, base);
	// 82E31C2C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31C30: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E31C34: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31C38: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31C3C: 396B0BF0  addi r11, r11, 0xbf0
	ctx.r[11].s64 = ctx.r[11].s64 + 3056;
	// 82E31C40: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E31C44: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E31C48: 4BFFF939  bl 0x82e31580
	ctx.lr = 0x82E31C4C;
	sub_82E31580(ctx, base);
	// 82E31C4C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31C50: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E31C54: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E31C58: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E31C5C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31C60: 4BFD15A9  bl 0x82e03208
	ctx.lr = 0x82E31C64;
	sub_82E03208(ctx, base);
	// 82E31C64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31C68: 4BFC17C1  bl 0x82df3428
	ctx.lr = 0x82E31C6C;
	sub_82DF3428(ctx, base);
	// 82E31C6C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E31C70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31C74: 388BA890  addi r4, r11, -0x5770
	ctx.r[4].s64 = ctx.r[11].s64 + -22384;
	// 82E31C78: 4BFC1D91  bl 0x82df3a08
	ctx.lr = 0x82E31C7C;
	sub_82DF3A08(ctx, base);
	// 82E31C7C: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E31C80: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31C84: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E31C88: 388BF950  addi r4, r11, -0x6b0
	ctx.r[4].s64 = ctx.r[11].s64 + -1712;
	// 82E31C8C: 4B4A0495  bl 0x822d2120
	ctx.lr = 0x82E31C90;
	sub_822D2120(ctx, base);
	// 82E31C90: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31C94: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31C98: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E31C9C: 396B0C70  addi r11, r11, 0xc70
	ctx.r[11].s64 = ctx.r[11].s64 + 3184;
	// 82E31CA0: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31CA4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E31CA8: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E31CAC: 4BFFF8D5  bl 0x82e31580
	ctx.lr = 0x82E31CB0;
	sub_82E31580(ctx, base);
	// 82E31CB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31CB4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E31CB8: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E31CBC: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E31CC0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31CC4: 4BFD1545  bl 0x82e03208
	ctx.lr = 0x82E31CC8;
	sub_82E03208(ctx, base);
	// 82E31CC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31CCC: 4BFC175D  bl 0x82df3428
	ctx.lr = 0x82E31CD0;
	sub_82DF3428(ctx, base);
	// 82E31CD0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E31CD4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31CD8: 388BC10C  addi r4, r11, -0x3ef4
	ctx.r[4].s64 = ctx.r[11].s64 + -16116;
	// 82E31CDC: 4BFC1D2D  bl 0x82df3a08
	ctx.lr = 0x82E31CE0;
	sub_82DF3A08(ctx, base);
	// 82E31CE0: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E31CE4: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31CE8: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E31CEC: 388BF9C0  addi r4, r11, -0x640
	ctx.r[4].s64 = ctx.r[11].s64 + -1600;
	// 82E31CF0: 4B4A0431  bl 0x822d2120
	ctx.lr = 0x82E31CF4;
	sub_822D2120(ctx, base);
	// 82E31CF4: 3D4082E3  lis r10, -0x7d1d
	ctx.r[10].s64 = -2099052544;
	// 82E31CF8: 9B610074  stb r27, 0x74(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[27].u8 ) };
	// 82E31CFC: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E31D00: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E31D04: 394A0D08  addi r10, r10, 0xd08
	ctx.r[10].s64 = ctx.r[10].s64 + 3336;
	// 82E31D08: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82E31D0C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31D10: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82E31D14: 91210078  stw r9, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 82E31D18: 93C10074  stw r30, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82E31D1C: E8810070  ld r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82E31D20: E8A10078  ld r5, 0x78(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 82E31D24: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31D28: 4BFFF969  bl 0x82e31690
	ctx.lr = 0x82E31D2C;
	sub_82E31690(ctx, base);
	// 82E31D2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31D30: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E31D34: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E31D38: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E31D3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31D40: 4BFD14C9  bl 0x82e03208
	ctx.lr = 0x82E31D44;
	sub_82E03208(ctx, base);
	// 82E31D44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31D48: 4BFC16E1  bl 0x82df3428
	ctx.lr = 0x82E31D4C;
	sub_82DF3428(ctx, base);
	// 82E31D4C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E31D50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31D54: 388BC108  addi r4, r11, -0x3ef8
	ctx.r[4].s64 = ctx.r[11].s64 + -16120;
	// 82E31D58: 4BFC1CB1  bl 0x82df3a08
	ctx.lr = 0x82E31D5C;
	sub_82DF3A08(ctx, base);
	// 82E31D5C: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E31D60: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31D64: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E31D68: 388BF9C0  addi r4, r11, -0x640
	ctx.r[4].s64 = ctx.r[11].s64 + -1600;
	// 82E31D6C: 4B4A03B5  bl 0x822d2120
	ctx.lr = 0x82E31D70;
	sub_822D2120(ctx, base);
	// 82E31D70: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E31D74: 9B610074  stb r27, 0x74(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[27].u8 ) };
	// 82E31D78: 81410074  lwz r10, 0x74(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E31D7C: 3D2082E3  lis r9, -0x7d1d
	ctx.r[9].s64 = -2099052544;
	// 82E31D80: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82E31D84: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31D88: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82E31D8C: 39490D08  addi r10, r9, 0xd08
	ctx.r[10].s64 = ctx.r[9].s64 + 3336;
	// 82E31D90: 93C10074  stw r30, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82E31D94: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82E31D98: E8810070  ld r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82E31D9C: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31DA0: E8A10078  ld r5, 0x78(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 82E31DA4: 4BFFF8ED  bl 0x82e31690
	ctx.lr = 0x82E31DA8;
	sub_82E31690(ctx, base);
	// 82E31DA8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31DAC: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E31DB0: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E31DB4: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E31DB8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31DBC: 4BFD144D  bl 0x82e03208
	ctx.lr = 0x82E31DC0;
	sub_82E03208(ctx, base);
	// 82E31DC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31DC4: 4BFC1665  bl 0x82df3428
	ctx.lr = 0x82E31DC8;
	sub_82DF3428(ctx, base);
	// 82E31DC8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E31DCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31DD0: 388BC104  addi r4, r11, -0x3efc
	ctx.r[4].s64 = ctx.r[11].s64 + -16124;
	// 82E31DD4: 4BFC1C35  bl 0x82df3a08
	ctx.lr = 0x82E31DD8;
	sub_82DF3A08(ctx, base);
	// 82E31DD8: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E31DDC: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31DE0: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E31DE4: 388BF9C0  addi r4, r11, -0x640
	ctx.r[4].s64 = ctx.r[11].s64 + -1600;
	// 82E31DE8: 4B4A0339  bl 0x822d2120
	ctx.lr = 0x82E31DEC;
	sub_822D2120(ctx, base);
	// 82E31DEC: 9B610074  stb r27, 0x74(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[27].u8 ) };
	// 82E31DF0: 3D4082E3  lis r10, -0x7d1d
	ctx.r[10].s64 = -2099052544;
	// 82E31DF4: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31DF8: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E31DFC: 394A0D08  addi r10, r10, 0xd08
	ctx.r[10].s64 = ctx.r[10].s64 + 3336;
	// 82E31E00: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31E04: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82E31E08: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82E31E0C: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E31E10: 93C10074  stw r30, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82E31E14: 91210078  stw r9, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 82E31E18: E8A10078  ld r5, 0x78(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 82E31E1C: E8810070  ld r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82E31E20: 4BFFF871  bl 0x82e31690
	ctx.lr = 0x82E31E24;
	sub_82E31690(ctx, base);
	// 82E31E24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31E28: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E31E2C: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E31E30: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E31E34: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31E38: 4BFD13D1  bl 0x82e03208
	ctx.lr = 0x82E31E3C;
	sub_82E03208(ctx, base);
	// 82E31E3C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31E40: 4BFC15E9  bl 0x82df3428
	ctx.lr = 0x82E31E44;
	sub_82DF3428(ctx, base);
	// 82E31E44: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E31E48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31E4C: 388BC100  addi r4, r11, -0x3f00
	ctx.r[4].s64 = ctx.r[11].s64 + -16128;
	// 82E31E50: 4BFC1BB9  bl 0x82df3a08
	ctx.lr = 0x82E31E54;
	sub_82DF3A08(ctx, base);
	// 82E31E54: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E31E58: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31E5C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E31E60: 388BF9C0  addi r4, r11, -0x640
	ctx.r[4].s64 = ctx.r[11].s64 + -1600;
	// 82E31E64: 4B4A02BD  bl 0x822d2120
	ctx.lr = 0x82E31E68;
	sub_822D2120(ctx, base);
	// 82E31E68: 3D4082E3  lis r10, -0x7d1d
	ctx.r[10].s64 = -2099052544;
	// 82E31E6C: 9B610074  stb r27, 0x74(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[27].u8 ) };
	// 82E31E70: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E31E74: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E31E78: 394A0D08  addi r10, r10, 0xd08
	ctx.r[10].s64 = ctx.r[10].s64 + 3336;
	// 82E31E7C: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82E31E80: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31E84: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82E31E88: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82E31E8C: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E31E90: E8A10068  ld r5, 0x68(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 82E31E94: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31E98: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E31E9C: 4BFFF7F5  bl 0x82e31690
	ctx.lr = 0x82E31EA0;
	sub_82E31690(ctx, base);
	// 82E31EA0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31EA4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E31EA8: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E31EAC: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E31EB0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31EB4: 4BFD1355  bl 0x82e03208
	ctx.lr = 0x82E31EB8;
	sub_82E03208(ctx, base);
	// 82E31EB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31EBC: 4BFC156D  bl 0x82df3428
	ctx.lr = 0x82E31EC0;
	sub_82DF3428(ctx, base);
	// 82E31EC0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E31EC4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31EC8: 388BC0FC  addi r4, r11, -0x3f04
	ctx.r[4].s64 = ctx.r[11].s64 + -16132;
	// 82E31ECC: 4BFC1B3D  bl 0x82df3a08
	ctx.lr = 0x82E31ED0;
	sub_82DF3A08(ctx, base);
	// 82E31ED0: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E31ED4: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31ED8: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E31EDC: 388BF9C0  addi r4, r11, -0x640
	ctx.r[4].s64 = ctx.r[11].s64 + -1600;
	// 82E31EE0: 4B4A0241  bl 0x822d2120
	ctx.lr = 0x82E31EE4;
	sub_822D2120(ctx, base);
	// 82E31EE4: 9B610074  stb r27, 0x74(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[27].u8 ) };
	// 82E31EE8: 3D4082E3  lis r10, -0x7d1d
	ctx.r[10].s64 = -2099052544;
	// 82E31EEC: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31EF0: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E31EF4: 394A0D08  addi r10, r10, 0xd08
	ctx.r[10].s64 = ctx.r[10].s64 + 3336;
	// 82E31EF8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31EFC: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82E31F00: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82E31F04: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E31F08: 93C10074  stw r30, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82E31F0C: 91210078  stw r9, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 82E31F10: E8810070  ld r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82E31F14: E8A10078  ld r5, 0x78(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 82E31F18: 4BFFF779  bl 0x82e31690
	ctx.lr = 0x82E31F1C;
	sub_82E31690(ctx, base);
	// 82E31F1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31F20: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E31F24: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E31F28: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E31F2C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31F30: 4BFD12D9  bl 0x82e03208
	ctx.lr = 0x82E31F34;
	sub_82E03208(ctx, base);
	// 82E31F34: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31F38: 4BFC14F1  bl 0x82df3428
	ctx.lr = 0x82E31F3C;
	sub_82DF3428(ctx, base);
	// 82E31F3C: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E31F40: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31F44: 388BC0F8  addi r4, r11, -0x3f08
	ctx.r[4].s64 = ctx.r[11].s64 + -16136;
	// 82E31F48: 4BFC1AC1  bl 0x82df3a08
	ctx.lr = 0x82E31F4C;
	sub_82DF3A08(ctx, base);
	// 82E31F4C: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E31F50: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31F54: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E31F58: 388BF9C0  addi r4, r11, -0x640
	ctx.r[4].s64 = ctx.r[11].s64 + -1600;
	// 82E31F5C: 4B4A01C5  bl 0x822d2120
	ctx.lr = 0x82E31F60;
	sub_822D2120(ctx, base);
	// 82E31F60: 3D4082E3  lis r10, -0x7d1d
	ctx.r[10].s64 = -2099052544;
	// 82E31F64: 9B610074  stb r27, 0x74(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[27].u8 ) };
	// 82E31F68: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E31F6C: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E31F70: 394A0D08  addi r10, r10, 0xd08
	ctx.r[10].s64 = ctx.r[10].s64 + 3336;
	// 82E31F74: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82E31F78: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31F7C: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82E31F80: 91210078  stw r9, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 82E31F84: 93C10074  stw r30, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82E31F88: E8810070  ld r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82E31F8C: E8A10078  ld r5, 0x78(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 82E31F90: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E31F94: 4BFFF6FD  bl 0x82e31690
	ctx.lr = 0x82E31F98;
	sub_82E31690(ctx, base);
	// 82E31F98: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E31F9C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E31FA0: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E31FA4: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E31FA8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E31FAC: 4BFD125D  bl 0x82e03208
	ctx.lr = 0x82E31FB0;
	sub_82E03208(ctx, base);
	// 82E31FB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31FB4: 4BFC1475  bl 0x82df3428
	ctx.lr = 0x82E31FB8;
	sub_82DF3428(ctx, base);
	// 82E31FB8: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E31FBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E31FC0: 388BC0F4  addi r4, r11, -0x3f0c
	ctx.r[4].s64 = ctx.r[11].s64 + -16140;
	// 82E31FC4: 4BFC1A45  bl 0x82df3a08
	ctx.lr = 0x82E31FC8;
	sub_82DF3A08(ctx, base);
	// 82E31FC8: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E31FCC: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E31FD0: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E31FD4: 388BF9C0  addi r4, r11, -0x640
	ctx.r[4].s64 = ctx.r[11].s64 + -1600;
	// 82E31FD8: 4B4A0149  bl 0x822d2120
	ctx.lr = 0x82E31FDC;
	sub_822D2120(ctx, base);
	// 82E31FDC: 3D4082E3  lis r10, -0x7d1d
	ctx.r[10].s64 = -2099052544;
	// 82E31FE0: 9B610074  stb r27, 0x74(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[27].u8 ) };
	// 82E31FE4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 82E31FE8: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82E31FEC: 394A04B0  addi r10, r10, 0x4b0
	ctx.r[10].s64 = ctx.r[10].s64 + 1200;
	// 82E31FF0: 93C10074  stw r30, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 82E31FF4: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E31FF8: 91410070  stw r10, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[10].u32 ) };
	// 82E31FFC: 91210078  stw r9, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[9].u32 ) };
	// 82E32000: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82E32004: E8A10078  ld r5, 0x78(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 82E32008: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E3200C: E8810070  ld r4, 0x70(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 82E32010: 4BFFF681  bl 0x82e31690
	ctx.lr = 0x82E32014;
	sub_82E31690(ctx, base);
	// 82E32014: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E32018: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E3201C: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E32020: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E32024: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E32028: 4BFD11E1  bl 0x82e03208
	ctx.lr = 0x82E3202C;
	sub_82E03208(ctx, base);
	// 82E3202C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32030: 4BFC13F9  bl 0x82df3428
	ctx.lr = 0x82E32034;
	sub_82DF3428(ctx, base);
	// 82E32034: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E32038: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3203C: 388BA898  addi r4, r11, -0x5768
	ctx.r[4].s64 = ctx.r[11].s64 + -22376;
	// 82E32040: 4BFC19C9  bl 0x82df3a08
	ctx.lr = 0x82E32044;
	sub_82DF3A08(ctx, base);
	// 82E32044: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E32048: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E3204C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E32050: 388BFA30  addi r4, r11, -0x5d0
	ctx.r[4].s64 = ctx.r[11].s64 + -1488;
	// 82E32054: 4B4A00CD  bl 0x822d2120
	ctx.lr = 0x82E32058;
	sub_822D2120(ctx, base);
	// 82E32058: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E3205C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E32060: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E32064: 396B0558  addi r11, r11, 0x558
	ctx.r[11].s64 = ctx.r[11].s64 + 1368;
	// 82E32068: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E3206C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E32070: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E32074: 4BFFF50D  bl 0x82e31580
	ctx.lr = 0x82E32078;
	sub_82E31580(ctx, base);
	// 82E32078: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E3207C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E32080: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E32084: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E32088: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3208C: 4BFD117D  bl 0x82e03208
	ctx.lr = 0x82E32090;
	sub_82E03208(ctx, base);
	// 82E32090: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32094: 4BFC1395  bl 0x82df3428
	ctx.lr = 0x82E32098;
	sub_82DF3428(ctx, base);
	// 82E32098: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E3209C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E320A0: 388BA8A0  addi r4, r11, -0x5760
	ctx.r[4].s64 = ctx.r[11].s64 + -22368;
	// 82E320A4: 4BFC1965  bl 0x82df3a08
	ctx.lr = 0x82E320A8;
	sub_82DF3A08(ctx, base);
	// 82E320A8: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E320AC: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E320B0: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E320B4: 388BFAA0  addi r4, r11, -0x560
	ctx.r[4].s64 = ctx.r[11].s64 + -1376;
	// 82E320B8: 4B4A0069  bl 0x822d2120
	ctx.lr = 0x82E320BC;
	sub_822D2120(ctx, base);
	// 82E320BC: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E320C0: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E320C4: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E320C8: 396B0DB8  addi r11, r11, 0xdb8
	ctx.r[11].s64 = ctx.r[11].s64 + 3512;
	// 82E320CC: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E320D0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E320D4: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E320D8: 4BFFF4A9  bl 0x82e31580
	ctx.lr = 0x82E320DC;
	sub_82E31580(ctx, base);
	// 82E320DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E320E0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E320E4: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E320E8: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E320EC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E320F0: 4BFD1119  bl 0x82e03208
	ctx.lr = 0x82E320F4;
	sub_82E03208(ctx, base);
	// 82E320F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E320F8: 4BFC1331  bl 0x82df3428
	ctx.lr = 0x82E320FC;
	sub_82DF3428(ctx, base);
	// 82E320FC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E32100: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32104: 388BA8AC  addi r4, r11, -0x5754
	ctx.r[4].s64 = ctx.r[11].s64 + -22356;
	// 82E32108: 4BFC1901  bl 0x82df3a08
	ctx.lr = 0x82E3210C;
	sub_82DF3A08(ctx, base);
	// 82E3210C: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E32110: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E32114: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E32118: 388BFB10  addi r4, r11, -0x4f0
	ctx.r[4].s64 = ctx.r[11].s64 + -1264;
	// 82E3211C: 4B4A0005  bl 0x822d2120
	ctx.lr = 0x82E32120;
	sub_822D2120(ctx, base);
	// 82E32120: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E32124: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E32128: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E3212C: 396B0E48  addi r11, r11, 0xe48
	ctx.r[11].s64 = ctx.r[11].s64 + 3656;
	// 82E32130: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E32134: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E32138: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E3213C: 4BFFF445  bl 0x82e31580
	ctx.lr = 0x82E32140;
	sub_82E31580(ctx, base);
	// 82E32140: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E32144: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E32148: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E3214C: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E32150: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E32154: 4BFD10B5  bl 0x82e03208
	ctx.lr = 0x82E32158;
	sub_82E03208(ctx, base);
	// 82E32158: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3215C: 4BFC12CD  bl 0x82df3428
	ctx.lr = 0x82E32160;
	sub_82DF3428(ctx, base);
	// 82E32160: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E32164: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32168: 388BA8B0  addi r4, r11, -0x5750
	ctx.r[4].s64 = ctx.r[11].s64 + -22352;
	// 82E3216C: 4BFC189D  bl 0x82df3a08
	ctx.lr = 0x82E32170;
	sub_82DF3A08(ctx, base);
	// 82E32170: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E32174: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E32178: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E3217C: 388BFB80  addi r4, r11, -0x480
	ctx.r[4].s64 = ctx.r[11].s64 + -1152;
	// 82E32180: 4B49FFA1  bl 0x822d2120
	ctx.lr = 0x82E32184;
	sub_822D2120(ctx, base);
	// 82E32184: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E32188: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E3218C: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E32190: 396B0EF8  addi r11, r11, 0xef8
	ctx.r[11].s64 = ctx.r[11].s64 + 3832;
	// 82E32194: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E32198: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E3219C: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E321A0: 4BFFF3E1  bl 0x82e31580
	ctx.lr = 0x82E321A4;
	sub_82E31580(ctx, base);
	// 82E321A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E321A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E321AC: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E321B0: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E321B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E321B8: 4BFD1051  bl 0x82e03208
	ctx.lr = 0x82E321BC;
	sub_82E03208(ctx, base);
	// 82E321BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E321C0: 4BFC1269  bl 0x82df3428
	ctx.lr = 0x82E321C4;
	sub_82DF3428(ctx, base);
	// 82E321C4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E321C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E321CC: 388BA8B8  addi r4, r11, -0x5748
	ctx.r[4].s64 = ctx.r[11].s64 + -22344;
	// 82E321D0: 4BFC1839  bl 0x82df3a08
	ctx.lr = 0x82E321D4;
	sub_82DF3A08(ctx, base);
	// 82E321D4: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E321D8: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E321DC: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E321E0: 388BFBF0  addi r4, r11, -0x410
	ctx.r[4].s64 = ctx.r[11].s64 + -1040;
	// 82E321E4: 4B49FF3D  bl 0x822d2120
	ctx.lr = 0x82E321E8;
	sub_822D2120(ctx, base);
	// 82E321E8: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E321EC: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E321F0: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E321F4: 396B0FC0  addi r11, r11, 0xfc0
	ctx.r[11].s64 = ctx.r[11].s64 + 4032;
	// 82E321F8: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E321FC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E32200: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E32204: 4BFFF37D  bl 0x82e31580
	ctx.lr = 0x82E32208;
	sub_82E31580(ctx, base);
	// 82E32208: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E3220C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E32210: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E32214: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E32218: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E3221C: 4BFD0FED  bl 0x82e03208
	ctx.lr = 0x82E32220;
	sub_82E03208(ctx, base);
	// 82E32220: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32224: 4BFC1205  bl 0x82df3428
	ctx.lr = 0x82E32228;
	sub_82DF3428(ctx, base);
	// 82E32228: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E3222C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32230: 388BA8C0  addi r4, r11, -0x5740
	ctx.r[4].s64 = ctx.r[11].s64 + -22336;
	// 82E32234: 4BFC17D5  bl 0x82df3a08
	ctx.lr = 0x82E32238;
	sub_82DF3A08(ctx, base);
	// 82E32238: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E3223C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E32240: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E32244: 388BFC60  addi r4, r11, -0x3a0
	ctx.r[4].s64 = ctx.r[11].s64 + -928;
	// 82E32248: 4B49FED9  bl 0x822d2120
	ctx.lr = 0x82E3224C;
	sub_822D2120(ctx, base);
	// 82E3224C: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E32250: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E32254: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E32258: 396B1040  addi r11, r11, 0x1040
	ctx.r[11].s64 = ctx.r[11].s64 + 4160;
	// 82E3225C: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E32260: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E32264: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E32268: 4BFFF319  bl 0x82e31580
	ctx.lr = 0x82E3226C;
	sub_82E31580(ctx, base);
	// 82E3226C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E32270: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E32274: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E32278: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E3227C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E32280: 4BFD0F89  bl 0x82e03208
	ctx.lr = 0x82E32284;
	sub_82E03208(ctx, base);
	// 82E32284: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32288: 4BFC11A1  bl 0x82df3428
	ctx.lr = 0x82E3228C;
	sub_82DF3428(ctx, base);
	// 82E3228C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E32290: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32294: 388BA8D0  addi r4, r11, -0x5730
	ctx.r[4].s64 = ctx.r[11].s64 + -22320;
	// 82E32298: 4BFC1771  bl 0x82df3a08
	ctx.lr = 0x82E3229C;
	sub_82DF3A08(ctx, base);
	// 82E3229C: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E322A0: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E322A4: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E322A8: 388BFCD0  addi r4, r11, -0x330
	ctx.r[4].s64 = ctx.r[11].s64 + -816;
	// 82E322AC: 4B49FE75  bl 0x822d2120
	ctx.lr = 0x82E322B0;
	sub_822D2120(ctx, base);
	// 82E322B0: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E322B4: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E322B8: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E322BC: 396B10D8  addi r11, r11, 0x10d8
	ctx.r[11].s64 = ctx.r[11].s64 + 4312;
	// 82E322C0: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E322C4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E322C8: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E322CC: 4BFFF2B5  bl 0x82e31580
	ctx.lr = 0x82E322D0;
	sub_82E31580(ctx, base);
	// 82E322D0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E322D4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E322D8: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E322DC: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E322E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E322E4: 4BFD0F25  bl 0x82e03208
	ctx.lr = 0x82E322E8;
	sub_82E03208(ctx, base);
	// 82E322E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E322EC: 4BFC113D  bl 0x82df3428
	ctx.lr = 0x82E322F0;
	sub_82DF3428(ctx, base);
	// 82E322F0: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E322F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E322F8: 388BA8E8  addi r4, r11, -0x5718
	ctx.r[4].s64 = ctx.r[11].s64 + -22296;
	// 82E322FC: 4BFC170D  bl 0x82df3a08
	ctx.lr = 0x82E32300;
	sub_82DF3A08(ctx, base);
	// 82E32300: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E32304: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E32308: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E3230C: 388BFD40  addi r4, r11, -0x2c0
	ctx.r[4].s64 = ctx.r[11].s64 + -704;
	// 82E32310: 4B49FE11  bl 0x822d2120
	ctx.lr = 0x82E32314;
	sub_822D2120(ctx, base);
	// 82E32314: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E32318: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E3231C: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E32320: 396B11D0  addi r11, r11, 0x11d0
	ctx.r[11].s64 = ctx.r[11].s64 + 4560;
	// 82E32324: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E32328: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E3232C: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E32330: 4BFFF251  bl 0x82e31580
	ctx.lr = 0x82E32334;
	sub_82E31580(ctx, base);
	// 82E32334: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E32338: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82E3233C: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E32340: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E32344: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E32348: 4BFD0EC1  bl 0x82e03208
	ctx.lr = 0x82E3234C;
	sub_82E03208(ctx, base);
	// 82E3234C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32350: 4BFC10D9  bl 0x82df3428
	ctx.lr = 0x82E32354;
	sub_82DF3428(ctx, base);
	// 82E32354: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E32358: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3235C: 388BA8F8  addi r4, r11, -0x5708
	ctx.r[4].s64 = ctx.r[11].s64 + -22280;
	// 82E32360: 4BFC16A9  bl 0x82df3a08
	ctx.lr = 0x82E32364;
	sub_82DF3A08(ctx, base);
	// 82E32364: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E32368: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E3236C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E32370: 388BFDB0  addi r4, r11, -0x250
	ctx.r[4].s64 = ctx.r[11].s64 + -592;
	// 82E32374: 4B49FDAD  bl 0x822d2120
	ctx.lr = 0x82E32378;
	sub_822D2120(ctx, base);
	// 82E32378: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E3237C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E32380: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E32384: 396B1268  addi r11, r11, 0x1268
	ctx.r[11].s64 = ctx.r[11].s64 + 4712;
	// 82E32388: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E3238C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E32390: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E32394: 4BFFF1ED  bl 0x82e31580
	ctx.lr = 0x82E32398;
	sub_82E31580(ctx, base);
	// 82E32398: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E3239C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E323A0: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E323A4: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E323A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E323AC: 4BFD0E5D  bl 0x82e03208
	ctx.lr = 0x82E323B0;
	sub_82E03208(ctx, base);
	// 82E323B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E323B4: 4BFC1075  bl 0x82df3428
	ctx.lr = 0x82E323B8;
	sub_82DF3428(ctx, base);
	// 82E323B8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E323BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E323C0: 388BA8FC  addi r4, r11, -0x5704
	ctx.r[4].s64 = ctx.r[11].s64 + -22276;
	// 82E323C4: 4BFC1645  bl 0x82df3a08
	ctx.lr = 0x82E323C8;
	sub_82DF3A08(ctx, base);
	// 82E323C8: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E323CC: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E323D0: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E323D4: 388BFE20  addi r4, r11, -0x1e0
	ctx.r[4].s64 = ctx.r[11].s64 + -480;
	// 82E323D8: 4B49FD49  bl 0x822d2120
	ctx.lr = 0x82E323DC;
	sub_822D2120(ctx, base);
	// 82E323DC: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E323E0: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E323E4: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E323E8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E323EC: 396B12E8  addi r11, r11, 0x12e8
	ctx.r[11].s64 = ctx.r[11].s64 + 4840;
	// 82E323F0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E323F4: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E323F8: 4BFFF189  bl 0x82e31580
	ctx.lr = 0x82E323FC;
	sub_82E31580(ctx, base);
	// 82E323FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E32400: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E32404: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E32408: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E3240C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E32410: 4BFD0DF9  bl 0x82e03208
	ctx.lr = 0x82E32414;
	sub_82E03208(ctx, base);
	// 82E32414: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32418: 4BFC1011  bl 0x82df3428
	ctx.lr = 0x82E3241C;
	sub_82DF3428(ctx, base);
	// 82E3241C: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E32420: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32424: 388BA900  addi r4, r11, -0x5700
	ctx.r[4].s64 = ctx.r[11].s64 + -22272;
	// 82E32428: 4BFC15E1  bl 0x82df3a08
	ctx.lr = 0x82E3242C;
	sub_82DF3A08(ctx, base);
	// 82E3242C: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E32430: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E32434: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E32438: 388BFE90  addi r4, r11, -0x170
	ctx.r[4].s64 = ctx.r[11].s64 + -368;
	// 82E3243C: 4B49FCE5  bl 0x822d2120
	ctx.lr = 0x82E32440;
	sub_822D2120(ctx, base);
	// 82E32440: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E32444: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E32448: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E3244C: 396B1368  addi r11, r11, 0x1368
	ctx.r[11].s64 = ctx.r[11].s64 + 4968;
	// 82E32450: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E32454: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E32458: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E3245C: 4BFFF125  bl 0x82e31580
	ctx.lr = 0x82E32460;
	sub_82E31580(ctx, base);
	// 82E32460: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E32464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E32468: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E3246C: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E32470: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E32474: 4BFD0D95  bl 0x82e03208
	ctx.lr = 0x82E32478;
	sub_82E03208(ctx, base);
	// 82E32478: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3247C: 4BFC0FAD  bl 0x82df3428
	ctx.lr = 0x82E32480;
	sub_82DF3428(ctx, base);
	// 82E32480: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E32484: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32488: 388BA904  addi r4, r11, -0x56fc
	ctx.r[4].s64 = ctx.r[11].s64 + -22268;
	// 82E3248C: 4BFC157D  bl 0x82df3a08
	ctx.lr = 0x82E32490;
	sub_82DF3A08(ctx, base);
	// 82E32490: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82E32494: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E32498: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82E3249C: 388BFF00  addi r4, r11, -0x100
	ctx.r[4].s64 = ctx.r[11].s64 + -256;
	// 82E324A0: 4B49FC81  bl 0x822d2120
	ctx.lr = 0x82E324A4;
	sub_822D2120(ctx, base);
	// 82E324A4: 3D6082E3  lis r11, -0x7d1d
	ctx.r[11].s64 = -2099052544;
	// 82E324A8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 82E324AC: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82E324B0: 396B13E8  addi r11, r11, 0x13e8
	ctx.r[11].s64 = ctx.r[11].s64 + 5096;
	// 82E324B4: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 82E324B8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82E324BC: E8810060  ld r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 82E324C0: 4BFFF0C1  bl 0x82e31580
	ctx.lr = 0x82E324C4;
	sub_82E31580(ctx, base);
	// 82E324C4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82E324C8: 38C100B0  addi r6, r1, 0xb0
	ctx.r[6].s64 = ctx.r[1].s64 + 176;
	// 82E324CC: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82E324D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82E324D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E324D8: 4BFD0D31  bl 0x82e03208
	ctx.lr = 0x82E324DC;
	sub_82E03208(ctx, base);
	// 82E324DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E324E0: 4BFC0F49  bl 0x82df3428
	ctx.lr = 0x82E324E4;
	sub_82DF3428(ctx, base);
	// 82E324E4: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82E324E8: 48375CCC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E324F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E324F0 size=72
    let mut pc: u32 = 0x82E324F0;
    'dispatch: loop {
        match pc {
            0x82E324F0 => {
    //   block [0x82E324F0..0x82E32538)
	// 82E324F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E324F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E324F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E324FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E32500: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E32504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E32508: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82E3250C: 388BACE0  addi r4, r11, -0x5320
	ctx.r[4].s64 = ctx.r[11].s64 + -21280;
	// 82E32510: 48375BE9  bl 0x831a80f8
	ctx.lr = 0x82E32514;
	sub_831A80F8(ctx, base);
	// 82E32514: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E32518: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82E3251C: 40820008  bne 0x82e32524
	if !ctx.cr[0].eq {
	pc = 0x82E32524; continue 'dispatch;
	}
	// 82E32520: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82E32524: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E32528: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3252C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E32530: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E32534: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E32538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82E32538 size=8
    let mut pc: u32 = 0x82E32538;
    'dispatch: loop {
        match pc {
            0x82E32538 => {
    //   block [0x82E32538..0x82E32540)
	// 82E32538: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E3253C: 483766CC  b 0x831a8c08
	sub_831A8C08(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E32540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E32540 size=196
    let mut pc: u32 = 0x82E32540;
    'dispatch: loop {
        match pc {
            0x82E32540 => {
    //   block [0x82E32540..0x82E32604)
	// 82E32540: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E32544: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E32548: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E3254C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E32550: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E32554: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E32558: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E3255C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82E32560: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E32564: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E32568: 4B48E3D1  bl 0x822c0938
	ctx.lr = 0x82E3256C;
	sub_822C0938(ctx, base);
	// 82E3256C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E32570: 41820028  beq 0x82e32598
	if ctx.cr[0].eq {
	pc = 0x82E32598; continue 'dispatch;
	}
	// 82E32574: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E32578: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E3257C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E32580: 392BB790  addi r9, r11, -0x4870
	ctx.r[9].s64 = ctx.r[11].s64 + -18544;
	// 82E32584: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E32588: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E3258C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E32590: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E32594: 48000008  b 0x82e3259c
	pc = 0x82E3259C; continue 'dispatch;
	// 82E32598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E3259C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E325A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E325A4: 409A0044  bne cr6, 0x82e325e8
	if !ctx.cr[6].eq {
	pc = 0x82E325E8; continue 'dispatch;
	}
	// 82E325A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82E325AC: 419A001C  beq cr6, 0x82e325c8
	if ctx.cr[6].eq {
	pc = 0x82E325C8; continue 'dispatch;
	}
	// 82E325B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E325B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82E325B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E325BC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E325C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82E325C4: 4E800421  bctrl
	ctx.lr = 0x82E325C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82E325C8: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E325CC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E325D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E325D4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E325D8: 816BACDC  lwz r11, -0x5324(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21284 as u32) ) } as u64;
	// 82E325DC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E325E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E325E4: 4B48DA1D  bl 0x822c0000
	ctx.lr = 0x82E325E8;
	sub_822C0000(ctx, base);
	// 82E325E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E325EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E325F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E325F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E325F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E325FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E32600: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E32608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E32608 size=304
    let mut pc: u32 = 0x82E32608;
    'dispatch: loop {
        match pc {
            0x82E32608 => {
    //   block [0x82E32608..0x82E32738)
	// 82E32608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3260C: 48375B5D  bl 0x831a8168
	ctx.lr = 0x82E32610;
	sub_831A8130(ctx, base);
	// 82E32610: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E32614: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E32618: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E3261C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E32620: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E32624: 4BFCC245  bl 0x82dfe868
	ctx.lr = 0x82E32628;
	sub_82DFE868(ctx, base);
	// 82E32628: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3262C: 40820104  bne 0x82e32730
	if !ctx.cr[0].eq {
	pc = 0x82E32730; continue 'dispatch;
	}
	// 82E32630: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32634: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E32638: 4BFC13D1  bl 0x82df3a08
	ctx.lr = 0x82E3263C;
	sub_82DF3A08(ctx, base);
	// 82E3263C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E32640: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E32644: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E32648: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E3264C: 4BFFC895  bl 0x82e2eee0
	ctx.lr = 0x82E32650;
	sub_82E2EEE0(ctx, base);
	// 82E32650: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E32654: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 82E32658: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82E3265C: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82E32660: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E32664: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E32668: 4B491DF9  bl 0x822c4460
	ctx.lr = 0x82E3266C;
	sub_822C4460(ctx, base);
	// 82E3266C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E32670: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E32674: 419A0008  beq cr6, 0x82e3267c
	if ctx.cr[6].eq {
	pc = 0x82E3267C; continue 'dispatch;
	}
	// 82E32678: 4B48E219  bl 0x822c0890
	ctx.lr = 0x82E3267C;
	sub_822C0890(ctx, base);
	// 82E3267C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32680: 4BFC0DA9  bl 0x82df3428
	ctx.lr = 0x82E32684;
	sub_82DF3428(ctx, base);
	// 82E32684: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E32688: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E3268C: 4BFC11ED  bl 0x82df3878
	ctx.lr = 0x82E32690;
	sub_82DF3878(ctx, base);
	// 82E32690: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E32694: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82E32698: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E3269C: 48375E75  bl 0x831a8510
	ctx.lr = 0x82E326A0;
	sub_831A8510(ctx, base);
	// 82E326A0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E326A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E326A8: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E326AC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E326B0: 480098C9  bl 0x82e3bf78
	ctx.lr = 0x82E326B4;
	sub_82E3BF78(ctx, base);
	// 82E326B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E326B8: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E326BC: 4BFC134D  bl 0x82df3a08
	ctx.lr = 0x82E326C0;
	sub_82DF3A08(ctx, base);
	// 82E326C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E326C4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E326C8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E326CC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E326D0: 83BF0024  lwz r29, 0x24(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E326D4: 4BFF950D  bl 0x82e2bbe0
	ctx.lr = 0x82E326D8;
	sub_82E2BBE0(ctx, base);
	// 82E326D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E326DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E326E0: 4B6C3D59  bl 0x824f6438
	ctx.lr = 0x82E326E4;
	sub_824F6438(ctx, base);
	// 82E326E4: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E326E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E326EC: 419A0008  beq cr6, 0x82e326f4
	if ctx.cr[6].eq {
	pc = 0x82E326F4; continue 'dispatch;
	}
	// 82E326F0: 4B48E1A1  bl 0x822c0890
	ctx.lr = 0x82E326F4;
	sub_822C0890(ctx, base);
	// 82E326F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E326F8: 4BFC0D31  bl 0x82df3428
	ctx.lr = 0x82E326FC;
	sub_82DF3428(ctx, base);
	// 82E326FC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32700: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E32704: 4BFC1305  bl 0x82df3a08
	ctx.lr = 0x82E32708;
	sub_82DF3A08(ctx, base);
	// 82E32708: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E3270C: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 82E32710: 4BFC14C1  bl 0x82df3bd0
	ctx.lr = 0x82E32714;
	sub_82DF3BD0(ctx, base);
	// 82E32714: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32718: 4BFC0D11  bl 0x82df3428
	ctx.lr = 0x82E3271C;
	sub_82DF3428(ctx, base);
	// 82E3271C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82E32720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E32724: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E32728: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82E3272C: 4BFCC12D  bl 0x82dfe858
	ctx.lr = 0x82E32730;
	sub_82DFE858(ctx, base);
	// 82E32730: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E32734: 48375A84  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E32738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82E32738 size=248
    let mut pc: u32 = 0x82E32738;
    'dispatch: loop {
        match pc {
            0x82E32738 => {
    //   block [0x82E32738..0x82E32830)
	// 82E32738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E3273C: 48375A2D  bl 0x831a8168
	ctx.lr = 0x82E32740;
	sub_831A8130(ctx, base);
	// 82E32740: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E32744: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E32748: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E3274C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E32750: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E32754: 4BFCC115  bl 0x82dfe868
	ctx.lr = 0x82E32758;
	sub_82DFE868(ctx, base);
	// 82E32758: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E3275C: 408200CC  bne 0x82e32828
	if !ctx.cr[0].eq {
	pc = 0x82E32828; continue 'dispatch;
	}
	// 82E32760: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E32764: 4BFF823D  bl 0x82e2a9a0
	ctx.lr = 0x82E32768;
	sub_82E2A9A0(ctx, base);
	// 82E32768: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E3276C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E32770: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E32774: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E32778: 48009839  bl 0x82e3bfb0
	ctx.lr = 0x82E3277C;
	sub_82E3BFB0(ctx, base);
	// 82E3277C: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82E32780: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32784: 4804B155  bl 0x82e7d8d8
	ctx.lr = 0x82E32788;
	sub_82E7D8D8(ctx, base);
	// 82E32788: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E3278C: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82E32790: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 82E32794: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E32798: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E32830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E32830 size=156
    let mut pc: u32 = 0x82E32830;
    'dispatch: loop {
        match pc {
            0x82E32830 => {
    //   block [0x82E32830..0x82E328CC)
	// 82E32830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E32834: 48375931  bl 0x831a8164
	ctx.lr = 0x82E32838;
	sub_831A8130(ctx, base);
	// 82E32838: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3283C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82E32840: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E32844: 576B07BD  rlwinm. r11, r27, 0, 0x1e, 0x1e
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E32848: 41820054  beq 0x82e3289c
	if ctx.cr[0].eq {
	pc = 0x82E3289C; continue 'dispatch;
	}
	// 82E3284C: 815FFFF0  lwz r10, -0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-16 as u32) ) } as u64;
	// 82E32850: 3BBFFFF0  addi r29, r31, -0x10
	ctx.r[29].s64 = ctx.r[31].s64 + -16;
	// 82E32854: 1D6A0030  mulli r11, r10, 0x30
	ctx.r[11].s64 = ctx.r[10].s64 * 48;
	// 82E32858: 37CAFFFF  addic. r30, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E3285C: 7FEBFA14  add r31, r11, r31
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82E32860: 41800024  blt 0x82e32884
	if ctx.cr[0].lt {
	pc = 0x82E32884; continue 'dispatch;
	}
	// 82E32864: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E32868: 3B8B9B84  addi r28, r11, -0x647c
	ctx.r[28].s64 = ctx.r[11].s64 + -25724;
	// 82E3286C: 3BFFFFD0  addi r31, r31, -0x30
	ctx.r[31].s64 = ctx.r[31].s64 + -48;
	// 82E32870: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E32874: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82E32878: 4BFC0BB1  bl 0x82df3428
	ctx.lr = 0x82E3287C;
	sub_82DF3428(ctx, base);
	// 82E3287C: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82E32880: 4080FFEC  bge 0x82e3286c
	if !ctx.cr[0].lt {
	pc = 0x82E3286C; continue 'dispatch;
	}
	// 82E32884: 576B07FF  clrlwi. r11, r27, 0x1f
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E32888: 4182000C  beq 0x82e32894
	if ctx.cr[0].eq {
	pc = 0x82E32894; continue 'dispatch;
	}
	// 82E3288C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E32890: 4BFBFB49  bl 0x82df23d8
	ctx.lr = 0x82E32894;
	sub_82DF23D8(ctx, base);
	// 82E32894: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E32898: 4800002C  b 0x82e328c4
	pc = 0x82E328C4; continue 'dispatch;
	// 82E3289C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E328A0: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E328A4: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82E328A8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E328AC: 4BFC0B7D  bl 0x82df3428
	ctx.lr = 0x82E328B0;
	sub_82DF3428(ctx, base);
	// 82E328B0: 576B07FF  clrlwi. r11, r27, 0x1f
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E328B4: 4182000C  beq 0x82e328c0
	if ctx.cr[0].eq {
	pc = 0x82E328C0; continue 'dispatch;
	}
	// 82E328B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E328BC: 4BFBFB1D  bl 0x82df23d8
	ctx.lr = 0x82E328C0;
	sub_82DF23D8(ctx, base);
	// 82E328C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E328C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82E328C8: 483758EC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E328D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E328D0 size=172
    let mut pc: u32 = 0x82E328D0;
    'dispatch: loop {
        match pc {
            0x82E328D0 => {
    //   block [0x82E328D0..0x82E3297C)
	// 82E328D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E328D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E328D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E328DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E328E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E328E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E328E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E328EC: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 82E328F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82E328F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E328F8: 4B48E041  bl 0x822c0938
	ctx.lr = 0x82E328FC;
	sub_822C0938(ctx, base);
	// 82E328FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 82E32900: 41820028  beq 0x82e32928
	if ctx.cr[0].eq {
	pc = 0x82E32928; continue 'dispatch;
	}
	// 82E32904: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E32908: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 82E3290C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82E32910: 392BC110  addi r9, r11, -0x3ef0
	ctx.r[9].s64 = ctx.r[11].s64 + -16112;
	// 82E32914: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E32918: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E3291C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E32920: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82E32924: 48000008  b 0x82e3292c
	pc = 0x82E3292C; continue 'dispatch;
	// 82E32928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E3292C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E32930: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E32934: 409A002C  bne cr6, 0x82e32960
	if !ctx.cr[6].eq {
	pc = 0x82E32960; continue 'dispatch;
	}
	// 82E32938: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E3293C: 483762CD  bl 0x831a8c08
	ctx.lr = 0x82E32940;
	sub_831A8C08(ctx, base);
	// 82E32940: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 82E32944: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 82E32948: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E3294C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 82E32950: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82E32954: 816BACDC  lwz r11, -0x5324(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-21284 as u32) ) } as u64;
	// 82E32958: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82E3295C: 4B48D6A5  bl 0x822c0000
	ctx.lr = 0x82E32960;
	sub_822C0000(ctx, base);
	// 82E32960: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E32964: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E32968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E3296C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E32970: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E32974: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E32978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E32980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E32980 size=292
    let mut pc: u32 = 0x82E32980;
    'dispatch: loop {
        match pc {
            0x82E32980 => {
    //   block [0x82E32980..0x82E32AA4)
	// 82E32980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E32984: 483757E5  bl 0x831a8168
	ctx.lr = 0x82E32988;
	sub_831A8130(ctx, base);
	// 82E32988: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E3298C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E32990: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E32994: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82E32998: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E3299C: 4BFCBECD  bl 0x82dfe868
	ctx.lr = 0x82E329A0;
	sub_82DFE868(ctx, base);
	// 82E329A0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E329A4: 408200F8  bne 0x82e32a9c
	if !ctx.cr[0].eq {
	pc = 0x82E32A9C; continue 'dispatch;
	}
	// 82E329A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E329AC: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E329B0: 4BFC1059  bl 0x82df3a08
	ctx.lr = 0x82E329B4;
	sub_82DF3A08(ctx, base);
	// 82E329B4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E329B8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E329BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E329C0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E329C4: 4BFFC51D  bl 0x82e2eee0
	ctx.lr = 0x82E329C8;
	sub_82E2EEE0(ctx, base);
	// 82E329C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82E329CC: 395F000C  addi r10, r31, 0xc
	ctx.r[10].s64 = ctx.r[31].s64 + 12;
	// 82E329D0: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 82E329D4: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 82E329D8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E329DC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E329E0: 4B491A81  bl 0x822c4460
	ctx.lr = 0x82E329E4;
	sub_822C4460(ctx, base);
	// 82E329E4: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E329E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E329EC: 419A0008  beq cr6, 0x82e329f4
	if ctx.cr[6].eq {
	pc = 0x82E329F4; continue 'dispatch;
	}
	// 82E329F0: 4B48DEA1  bl 0x822c0890
	ctx.lr = 0x82E329F4;
	sub_822C0890(ctx, base);
	// 82E329F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E329F8: 4BFC0A31  bl 0x82df3428
	ctx.lr = 0x82E329FC;
	sub_82DF3428(ctx, base);
	// 82E329FC: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 82E32A00: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E32A04: 4BFC0E75  bl 0x82df3878
	ctx.lr = 0x82E32A08;
	sub_82DF3878(ctx, base);
	// 82E32A08: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82E32A0C: 38A00040  li r5, 0x40
	ctx.r[5].s64 = 64;
	// 82E32A10: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E32A14: 48375AFD  bl 0x831a8510
	ctx.lr = 0x82E32A18;
	sub_831A8510(ctx, base);
	// 82E32A18: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E32A1C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82E32A20: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E32A24: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82E32A28: 48009551  bl 0x82e3bf78
	ctx.lr = 0x82E32A2C;
	sub_82E3BF78(ctx, base);
	// 82E32A2C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32A30: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E32A34: 4BFC0FD5  bl 0x82df3a08
	ctx.lr = 0x82E32A38;
	sub_82DF3A08(ctx, base);
	// 82E32A38: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82E32A3C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E32A40: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82E32A44: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82E32A48: 83BF0024  lwz r29, 0x24(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E32A4C: 4BFF9195  bl 0x82e2bbe0
	ctx.lr = 0x82E32A50;
	sub_82E2BBE0(ctx, base);
	// 82E32A50: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E32A54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E32A58: 4B6C39E1  bl 0x824f6438
	ctx.lr = 0x82E32A5C;
	sub_824F6438(ctx, base);
	// 82E32A5C: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82E32A60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E32A64: 419A0008  beq cr6, 0x82e32a6c
	if ctx.cr[6].eq {
	pc = 0x82E32A6C; continue 'dispatch;
	}
	// 82E32A68: 4B48DE29  bl 0x822c0890
	ctx.lr = 0x82E32A6C;
	sub_822C0890(ctx, base);
	// 82E32A6C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32A70: 4BFC09B9  bl 0x82df3428
	ctx.lr = 0x82E32A74;
	sub_82DF3428(ctx, base);
	// 82E32A74: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32A78: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E32A7C: 4BFC0F8D  bl 0x82df3a08
	ctx.lr = 0x82E32A80;
	sub_82DF3A08(ctx, base);
	// 82E32A80: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E32A84: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 82E32A88: 4BFC1149  bl 0x82df3bd0
	ctx.lr = 0x82E32A8C;
	sub_82DF3BD0(ctx, base);
	// 82E32A8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32A90: 4BFC0999  bl 0x82df3428
	ctx.lr = 0x82E32A94;
	sub_82DF3428(ctx, base);
	// 82E32A94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E32A98: 4BFCBDC1  bl 0x82dfe858
	ctx.lr = 0x82E32A9C;
	sub_82DFE858(ctx, base);
	// 82E32A9C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82E32AA0: 48375718  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E32AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E32AA8 size=100
    let mut pc: u32 = 0x82E32AA8;
    'dispatch: loop {
        match pc {
            0x82E32AA8 => {
    //   block [0x82E32AA8..0x82E32B0C)
	// 82E32AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E32AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E32AB0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E32AB4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E32AB8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82E32ABC: 90810058  stw r4, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 82E32AC0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E32AC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E32AC8: 3861005C  addi r3, r1, 0x5c
	ctx.r[3].s64 = ctx.r[1].s64 + 92;
	// 82E32ACC: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82E32AD0: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E32AD4: 4BFFFDFD  bl 0x82e328d0
	ctx.lr = 0x82E32AD8;
	sub_82E328D0(ctx, base);
	// 82E32AD8: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82E32ADC: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82E32AE0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E32AE4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E32AE8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E32AEC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E32AF0: 419A0008  beq cr6, 0x82e32af8
	if ctx.cr[6].eq {
	pc = 0x82E32AF8; continue 'dispatch;
	}
	// 82E32AF4: 4B48DD9D  bl 0x822c0890
	ctx.lr = 0x82E32AF8;
	sub_822C0890(ctx, base);
	// 82E32AF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E32AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E32B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E32B04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E32B08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E32B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E32B10 size=112
    let mut pc: u32 = 0x82E32B10;
    'dispatch: loop {
        match pc {
            0x82E32B10 => {
    //   block [0x82E32B10..0x82E32B80)
	// 82E32B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E32B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E32B18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E32B1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E32B20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E32B24: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E32B28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E32B2C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82E32B30: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E32B34: 4BFFFA0D  bl 0x82e32540
	ctx.lr = 0x82E32B38;
	sub_82E32540(ctx, base);
	// 82E32B38: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E32B3C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82E32B40: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 82E32B44: 4B48D4BD  bl 0x822c0000
	ctx.lr = 0x82E32B48;
	sub_822C0000(ctx, base);
	// 82E32B48: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E32B4C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82E32B50: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E32B54: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E32B58: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E32B5C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82E32B60: 419A0008  beq cr6, 0x82e32b68
	if ctx.cr[6].eq {
	pc = 0x82E32B68; continue 'dispatch;
	}
	// 82E32B64: 4B48DD2D  bl 0x822c0890
	ctx.lr = 0x82E32B68;
	sub_822C0890(ctx, base);
	// 82E32B68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E32B6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E32B70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E32B74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E32B78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E32B7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E32B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E32B80 size=212
    let mut pc: u32 = 0x82E32B80;
    'dispatch: loop {
        match pc {
            0x82E32B80 => {
    //   block [0x82E32B80..0x82E32C54)
	// 82E32B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E32B84: 483755E9  bl 0x831a816c
	ctx.lr = 0x82E32B88;
	sub_831A8130(ctx, base);
	// 82E32B88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E32B8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E32B90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E32B94: 4BFCBCD5  bl 0x82dfe868
	ctx.lr = 0x82E32B98;
	sub_82DFE868(ctx, base);
	// 82E32B98: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E32B9C: 408200B0  bne 0x82e32c4c
	if !ctx.cr[0].eq {
	pc = 0x82E32C4C; continue 'dispatch;
	}
	// 82E32BA0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82E32BA4: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82E32BA8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E32BAC: 4BFBD9D5  bl 0x82df0580
	ctx.lr = 0x82E32BB0;
	sub_82DF0580(ctx, base);
	// 82E32BB0: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82E32BB4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E32BB8: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82E32BBC: 4B7FC4E5  bl 0x8262f0a0
	ctx.lr = 0x82E32BC0;
	sub_8262F0A0(ctx, base);
	// 82E32BC0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E32BC4: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82E32BC8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E32BCC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E32BD0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E32BD4: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E32BD8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E32BDC: 5563083C  slwi r3, r11, 1
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E32BE0: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E32BE4: 4BDAC605  bl 0x82bdf1e8
	ctx.lr = 0x82E32BE8;
	sub_82BDF1E8(ctx, base);
	// 82E32BE8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E32BEC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E32BF0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E32BF4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82E32BF8: 4BDAC6A1  bl 0x82bdf298
	ctx.lr = 0x82E32BFC;
	sub_82BDF298(ctx, base);
	// 82E32BFC: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E32C00: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E32C04: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82E32C08: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E32C0C: 419A001C  beq cr6, 0x82e32c28
	if ctx.cr[6].eq {
	pc = 0x82E32C28; continue 'dispatch;
	}
	// 82E32C10: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E32C14: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E32C18: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82E32C1C: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82E32C20: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82E32C24: 4082FFEC  bne 0x82e32c10
	if !ctx.cr[0].eq {
	pc = 0x82E32C10; continue 'dispatch;
	}
	// 82E32C28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82E32C2C: 4BDAC6B5  bl 0x82bdf2e0
	ctx.lr = 0x82E32C30;
	sub_82BDF2E0(ctx, base);
	// 82E32C30: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E32C34: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E32C38: 419A0008  beq cr6, 0x82e32c40
	if ctx.cr[6].eq {
	pc = 0x82E32C40; continue 'dispatch;
	}
	// 82E32C3C: 4BDAC39D  bl 0x82bdefd8
	ctx.lr = 0x82E32C40;
	sub_82BDEFD8(ctx, base);
	// 82E32C40: 93BF001C  stw r29, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 82E32C44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E32C48: 4BFCBC11  bl 0x82dfe858
	ctx.lr = 0x82E32C4C;
	sub_82DFE858(ctx, base);
	// 82E32C4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E32C50: 4837556C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E32C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E32C58 size=108
    let mut pc: u32 = 0x82E32C58;
    'dispatch: loop {
        match pc {
            0x82E32C58 => {
    //   block [0x82E32C58..0x82E32CC4)
	// 82E32C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E32C5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E32C60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E32C64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E32C68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E32C6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E32C70: 4BFCBB29  bl 0x82dfe798
	ctx.lr = 0x82E32C74;
	sub_82DFE798(ctx, base);
	// 82E32C74: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82E32C78: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82E32C7C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82E32C80: 392BC130  addi r9, r11, -0x3ed0
	ctx.r[9].s64 = ctx.r[11].s64 + -16080;
	// 82E32C84: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 82E32C88: 9BCA0000  stb r30, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[30].u8 ) };
	// 82E32C8C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82E32C90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E32C94: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82E32C98: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E32C9C: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E32CA0: 4B7FBEF9  bl 0x8262eb98
	ctx.lr = 0x82E32CA4;
	sub_8262EB98(ctx, base);
	// 82E32CA4: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82E32CA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E32CAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E32CB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E32CB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E32CB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E32CBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E32CC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E32CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E32CC8 size=92
    let mut pc: u32 = 0x82E32CC8;
    'dispatch: loop {
        match pc {
            0x82E32CC8 => {
    //   block [0x82E32CC8..0x82E32D24)
	// 82E32CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E32CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E32CD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E32CD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E32CD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E32CDC: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E32CE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E32CE4: 419A0008  beq cr6, 0x82e32cec
	if ctx.cr[6].eq {
	pc = 0x82E32CEC; continue 'dispatch;
	}
	// 82E32CE8: 4BDAC2F1  bl 0x82bdefd8
	ctx.lr = 0x82E32CEC;
	sub_82BDEFD8(ctx, base);
	// 82E32CEC: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82E32CF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E32CF4: 419A0008  beq cr6, 0x82e32cfc
	if ctx.cr[6].eq {
	pc = 0x82E32CFC; continue 'dispatch;
	}
	// 82E32CF8: 4B48DB99  bl 0x822c0890
	ctx.lr = 0x82E32CFC;
	sub_822C0890(ctx, base);
	// 82E32CFC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E32D00: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 82E32D04: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82E32D08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82E32D0C: 4BFC071D  bl 0x82df3428
	ctx.lr = 0x82E32D10;
	sub_82DF3428(ctx, base);
	// 82E32D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82E32D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E32D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E32D1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E32D20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E32D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E32D28 size=76
    let mut pc: u32 = 0x82E32D28;
    'dispatch: loop {
        match pc {
            0x82E32D28 => {
    //   block [0x82E32D28..0x82E32D74)
	// 82E32D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E32D2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E32D30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82E32D34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E32D38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E32D3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E32D40: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82E32D44: 4BFFFF85  bl 0x82e32cc8
	ctx.lr = 0x82E32D48;
	sub_82E32CC8(ctx, base);
	// 82E32D48: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E32D4C: 4182000C  beq 0x82e32d58
	if ctx.cr[0].eq {
	pc = 0x82E32D58; continue 'dispatch;
	}
	// 82E32D50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E32D54: 4BFBF685  bl 0x82df23d8
	ctx.lr = 0x82E32D58;
	sub_82DF23D8(ctx, base);
	// 82E32D58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E32D5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E32D60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E32D64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E32D68: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82E32D6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E32D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E32D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E32D78 size=276
    let mut pc: u32 = 0x82E32D78;
    'dispatch: loop {
        match pc {
            0x82E32D78 => {
    //   block [0x82E32D78..0x82E32E8C)
	// 82E32D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E32D7C: 483753F1  bl 0x831a816c
	ctx.lr = 0x82E32D80;
	sub_831A8130(ctx, base);
	// 82E32D80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E32D84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E32D88: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E32D8C: 4BFCBADD  bl 0x82dfe868
	ctx.lr = 0x82E32D90;
	sub_82DFE868(ctx, base);
	// 82E32D90: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E32D94: 408200F0  bne 0x82e32e84
	if !ctx.cr[0].eq {
	pc = 0x82E32E84; continue 'dispatch;
	}
	// 82E32D98: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E32D9C: 3D403FFF  lis r10, 0x3fff
	ctx.r[10].s64 = 1073676288;
	// 82E32DA0: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82E32DA4: 5563103A  slwi r3, r11, 2
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E32DA8: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E32DAC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82E32DB0: 40990008  ble cr6, 0x82e32db8
	if !ctx.cr[6].gt {
	pc = 0x82E32DB8; continue 'dispatch;
	}
	// 82E32DB4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82E32DB8: 4BFBD7C9  bl 0x82df0580
	ctx.lr = 0x82E32DBC;
	sub_82DF0580(ctx, base);
	// 82E32DBC: 3BDF0010  addi r30, r31, 0x10
	ctx.r[30].s64 = ctx.r[31].s64 + 16;
	// 82E32DC0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E32DC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E32DC8: 4B7FC2D9  bl 0x8262f0a0
	ctx.lr = 0x82E32DCC;
	sub_8262F0A0(ctx, base);
	// 82E32DCC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E32DD0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E32DD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82E32DD8: 4099002C  ble cr6, 0x82e32e04
	if !ctx.cr[6].gt {
	pc = 0x82E32E04; continue 'dispatch;
	}
	// 82E32DDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E32DE0: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E32DE4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E32DE8: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E32DEC: 7D2B482E  lwzx r9, r11, r9
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82E32DF0: 7D2B412E  stwx r9, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 82E32DF4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82E32DF8: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E32DFC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82E32E00: 4198FFE0  blt cr6, 0x82e32de0
	if ctx.cr[6].lt {
	pc = 0x82E32DE0; continue 'dispatch;
	}
	// 82E32E04: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E32E08: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82E32E0C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82E32E10: 5563083C  slwi r3, r11, 1
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E32E14: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E32E18: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82E32E1C: 4BDAC3CD  bl 0x82bdf1e8
	ctx.lr = 0x82E32E20;
	sub_82BDF1E8(ctx, base);
	// 82E32E20: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82E32E24: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82E32E28: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E32E2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82E32E30: 4BDAC469  bl 0x82bdf298
	ctx.lr = 0x82E32E34;
	sub_82BDF298(ctx, base);
	// 82E32E34: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82E32E38: 815D000C  lwz r10, 0xc(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E32E3C: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82E32E40: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E32E44: 419A001C  beq cr6, 0x82e32e60
	if ctx.cr[6].eq {
	pc = 0x82E32E60; continue 'dispatch;
	}
	// 82E32E48: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E32E4C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E32E50: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 82E32E54: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82E32E58: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82E32E5C: 4082FFEC  bne 0x82e32e48
	if !ctx.cr[0].eq {
	pc = 0x82E32E48; continue 'dispatch;
	}
	// 82E32E60: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E32E64: 4BDAC47D  bl 0x82bdf2e0
	ctx.lr = 0x82E32E68;
	sub_82BDF2E0(ctx, base);
	// 82E32E68: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82E32E6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82E32E70: 419A0008  beq cr6, 0x82e32e78
	if ctx.cr[6].eq {
	pc = 0x82E32E78; continue 'dispatch;
	}
	// 82E32E74: 4BDAC165  bl 0x82bdefd8
	ctx.lr = 0x82E32E78;
	sub_82BDEFD8(ctx, base);
	// 82E32E78: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 82E32E7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E32E80: 4BFCB9D9  bl 0x82dfe858
	ctx.lr = 0x82E32E84;
	sub_82DFE858(ctx, base);
	// 82E32E84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E32E88: 48375334  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E32E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E32E90 size=260
    let mut pc: u32 = 0x82E32E90;
    'dispatch: loop {
        match pc {
            0x82E32E90 => {
    //   block [0x82E32E90..0x82E32F94)
	// 82E32E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E32E94: 483752D5  bl 0x831a8168
	ctx.lr = 0x82E32E98;
	sub_831A8130(ctx, base);
	// 82E32E98: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E32E9C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E32EA0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82E32EA4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82E32EA8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82E32EAC: 4BFCB9BD  bl 0x82dfe868
	ctx.lr = 0x82E32EB0;
	sub_82DFE868(ctx, base);
	// 82E32EB0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E32EB4: 408200D8  bne 0x82e32f8c
	if !ctx.cr[0].eq {
	pc = 0x82E32F8C; continue 'dispatch;
	}
	// 82E32EB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32EBC: 4BFF7AE5  bl 0x82e2a9a0
	ctx.lr = 0x82E32EC0;
	sub_82E2A9A0(ctx, base);
	// 82E32EC0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82E32EC4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82E32EC8: 809D0000  lwz r4, 0(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E32ECC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82E32ED0: 480090E1  bl 0x82e3bfb0
	ctx.lr = 0x82E32ED4;
	sub_82E3BFB0(ctx, base);
	// 82E32ED4: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82E32ED8: 3881005C  addi r4, r1, 0x5c
	ctx.r[4].s64 = ctx.r[1].s64 + 92;
	// 82E32EDC: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 82E32EE0: 48375631  bl 0x831a8510
	ctx.lr = 0x82E32EE4;
	sub_831A8510(ctx, base);
	// 82E32EE4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E32EE8: 3D401FFF  lis r10, 0x1fff
	ctx.r[10].s64 = 536805376;
	// 82E32EEC: 55631838  slwi r3, r11, 3
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82E32EF0: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82E32EF4: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82E32EF8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82E32EFC: 40990008  ble cr6, 0x82e32f04
	if !ctx.cr[6].gt {
	pc = 0x82E32F04; continue 'dispatch;
	}
	// 82E32F00: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82E32F04: 4BFBD67D  bl 0x82df0580
	ctx.lr = 0x82E32F08;
	sub_82DF0580(ctx, base);
	// 82E32F08: 3BDF0028  addi r30, r31, 0x28
	ctx.r[30].s64 = ctx.r[31].s64 + 40;
	// 82E32F0C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82E32F10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82E32F14: 4BFFFB95  bl 0x82e32aa8
	ctx.lr = 0x82E32F18;
	sub_82E32AA8(ctx, base);
	// 82E32F18: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E32F1C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E32F20: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82E32F24: 40990044  ble cr6, 0x82e32f68
	if !ctx.cr[6].gt {
	pc = 0x82E32F68; continue 'dispatch;
	}
	// 82E32F28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82E32F2C: 813D0008  lwz r9, 8(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E32F30: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82E32F34: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E32F38: 7D29582E  lwzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82E32F3C: 7D2B412E  stwx r9, r11, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32), ctx.r[9].u32) };
	// 82E32F40: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82E32F44: 813D0008  lwz r9, 8(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82E32F48: 7D295A14  add r9, r9, r11
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82E32F4C: 7D0B4214  add r8, r11, r8
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82E32F50: 81290004  lwz r9, 4(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82E32F54: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82E32F58: 91280004  stw r9, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82E32F5C: 813F0024  lwz r9, 0x24(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82E32F60: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82E32F64: 4198FFC8  blt cr6, 0x82e32f2c
	if ctx.cr[6].lt {
	pc = 0x82E32F2C; continue 'dispatch;
	}
	// 82E32F68: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82E32F6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E32F70: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82E32F74: 4BFCB8E5  bl 0x82dfe858
	ctx.lr = 0x82E32F78;
	sub_82DFE858(ctx, base);
	// 82E32F78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82E32F7C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82E32F80: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 82E32F84: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82E32F88: 4BFC04A1  bl 0x82df3428
	ctx.lr = 0x82E32F8C;
	sub_82DF3428(ctx, base);
	// 82E32F8C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82E32F90: 48375228  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82E32F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82E32F98 size=84
    let mut pc: u32 = 0x82E32F98;
    'dispatch: loop {
        match pc {
            0x82E32F98 => {
    //   block [0x82E32F98..0x82E32FEC)
	// 82E32F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82E32F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82E32FA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82E32FA4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82E32FA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82E32FAC: 4804A835  bl 0x82e7d7e0
	ctx.lr = 0x82E32FB0;
	sub_82E7D7E0(ctx, base);
	// 82E32FB0: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 82E32FB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82E32FB8: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 82E32FBC: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82E32FC0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82E32FC4: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 82E32FC8: 99490000  stb r10, 0(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82E32FCC: 88A10050  lbz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82E32FD0: 4BFFF901  bl 0x82e328d0
	ctx.lr = 0x82E32FD4;
	sub_82E328D0(ctx, base);
	// 82E32FD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82E32FD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82E32FDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82E32FE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82E32FE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82E32FE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


