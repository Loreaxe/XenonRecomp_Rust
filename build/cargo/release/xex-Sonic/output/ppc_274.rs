pub fn sub_8314E190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314E190 size=8
    let mut pc: u32 = 0x8314E190;
    'dispatch: loop {
        match pc {
            0x8314E190 => {
    //   block [0x8314E190..0x8314E198)
	// 8314E190: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8314E194: 48000010  b 0x8314e1a4
	sub_8314E1A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314E198 size=8
    let mut pc: u32 = 0x8314E198;
    'dispatch: loop {
        match pc {
            0x8314E198 => {
    //   block [0x8314E198..0x8314E1A0)
	// 8314E198: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8314E19C: 48000008  b 0x8314e1a4
	sub_8314E1A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E1A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8314E1A0 size=56
    let mut pc: u32 = 0x8314E1A0;
    'dispatch: loop {
        match pc {
            0x8314E1A0 => {
    //   block [0x8314E1A0..0x8314E1D8)
	// 8314E1A0: 396B0003  addi r11, r11, 3
	ctx.r[11].s64 = ctx.r[11].s64 + 3;
	// 8314E1A4: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314E1A8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8314E1AC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8314E1B0: C00BFFF8  lfs f0, -8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E1B4: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E1B8: ED810028  fsubs f12, f1, f0
	ctx.f[12].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 8314E1BC: ED6D0028  fsubs f11, f13, f0
	ctx.f[11].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8314E1C0: C14BFFFC  lfs f10, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 8314E1C4: C12B0004  lfs f9, 4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8314E1C8: ED095028  fsubs f8, f9, f10
	ctx.f[8].f64 = (((ctx.f[9].f64 - ctx.f[10].f64) as f32) as f64);
	// 8314E1CC: ECEC5824  fdivs f7, f12, f11
	ctx.f[7].f64 = ((ctx.f[12].f64 / ctx.f[11].f64) as f32) as f64;
	// 8314E1D0: EC27523A  fmadds f1, f7, f8, f10
	ctx.f[1].f64 = (((ctx.f[7].f64 * ctx.f[8].f64 + ctx.f[10].f64) as f32) as f64);
	// 8314E1D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E1D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314E1D8 size=80
    let mut pc: u32 = 0x8314E1D8;
    'dispatch: loop {
        match pc {
            0x8314E1D8 => {
    //   block [0x8314E1D8..0x8314E228)
	// 8314E1D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314E1DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8314E1E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8314E1E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314E1E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314E1EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E1F0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314E1F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314E1F8: 4E800421  bctrl
	ctx.lr = 0x8314E1FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314E1FC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E200: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8314E204: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314E208: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314E20C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8314E210: 4E800421  bctrl
	ctx.lr = 0x8314E214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314E214: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8314E218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314E21C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314E220: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314E224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8314E228 size=32
    let mut pc: u32 = 0x8314E228;
    'dispatch: loop {
        match pc {
            0x8314E228 => {
    //   block [0x8314E228..0x8314E248)
	// 8314E228: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314E22C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314E230: 409A0018  bne cr6, 0x8314e248
	if !ctx.cr[6].eq {
		sub_8314E248(ctx, base);
		return;
	}
	// 8314E234: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8314E238: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E23C: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8314E240: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8314E244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8314E248 size=60
    let mut pc: u32 = 0x8314E248;
    'dispatch: loop {
        match pc {
            0x8314E248 => {
    //   block [0x8314E248..0x8314E284)
	// 8314E248: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8314E24C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8314E250: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E254: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314E258: C0090000  lfs f0, 0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E25C: D0050000  stfs f0, 0(r5)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8314E260: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314E264: 80EA0008  lwz r7, 8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314E268: 54EA1838  slwi r10, r7, 3
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8314E26C: 7CCA5A14  add r6, r10, r11
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8314E270: C1A6FFF8  lfs f13, -8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E274: D1A40000  stfs f13, 0(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8314E278: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314E27C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8314E280: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E284(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8314E284 size=92
    let mut pc: u32 = 0x8314E284;
    'dispatch: loop {
        match pc {
            0x8314E284 => {
    //   block [0x8314E284..0x8314E2E0)
	// 8314E284: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 8314E288: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8314E28C: C1840000  lfs f12, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8314E290: 7D4B482E  lwzx r10, r11, r9
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8314E294: 80EA0008  lwz r7, 8(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314E298: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314E29C: 54EA1838  slwi r10, r7, 3
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8314E2A0: 7CCA5A14  add r6, r10, r11
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8314E2A4: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E2A8: C006FFF8  lfs f0, -8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E2AC: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8314E2B0: 40990008  ble cr6, 0x8314e2b8
	if !ctx.cr[6].gt {
	pc = 0x8314E2B8; continue 'dispatch;
	}
	// 8314E2B4: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8314E2B8: C0050000  lfs f0, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E2BC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8314E2C0: 40980008  bge cr6, 0x8314e2c8
	if !ctx.cr[6].lt {
	pc = 0x8314E2C8; continue 'dispatch;
	}
	// 8314E2C4: D1A50000  stfs f13, 0(r5)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 8314E2C8: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314E2CC: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8314E2D0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8314E2D4: 7F085840  cmplw cr6, r8, r11
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8314E2D8: 4198FFB0  blt cr6, 0x8314e288
	if ctx.cr[6].lt {
	pc = 0x8314E288; continue 'dispatch;
	}
	// 8314E2DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314E2E0 size=200
    let mut pc: u32 = 0x8314E2E0;
    'dispatch: loop {
        match pc {
            0x8314E2E0 => {
    //   block [0x8314E2E0..0x8314E3A8)
	// 8314E2E0: FBC1FFF0  std r30, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[30].u64 ) };
	// 8314E2E4: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 8314E2E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8314E2EC: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E2F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8314E2F4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8314E2F8: 409AFFF4  bne cr6, 0x8314e2ec
	if !ctx.cr[6].eq {
	pc = 0x8314E2EC; continue 'dispatch;
	}
	// 8314E2FC: 7D435850  subf r10, r3, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 8314E300: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8314E304: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8314E308: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8314E30C: 5549003E  slwi r9, r10, 0
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 8314E310: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 8314E314: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8314E318: 419A0030  beq cr6, 0x8314e348
	if ctx.cr[6].eq {
	pc = 0x8314E348; continue 'dispatch;
	}
	// 8314E31C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8314E320: 7CC32050  subf r6, r3, r4
	ctx.r[6].s64 = ctx.r[4].s64 - ctx.r[3].s64;
	// 8314E324: 88EA0000  lbz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E328: 2B07003A  cmplwi cr6, r7, 0x3a
	ctx.cr[6].compare_u32(ctx.r[7].u32, 58 as u32, &mut ctx.xer);
	// 8314E32C: 419A001C  beq cr6, 0x8314e348
	if ctx.cr[6].eq {
	pc = 0x8314E348; continue 'dispatch;
	}
	// 8314E330: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8314E334: 7CE651AE  stbx r7, r6, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[6].u32.wrapping_add(ctx.r[10].u32), ctx.r[7].u8) };
	// 8314E338: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8314E33C: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 8314E340: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8314E344: 4198FFE0  blt cr6, 0x8314e324
	if ctx.cr[6].lt {
	pc = 0x8314E324; continue 'dispatch;
	}
	// 8314E348: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8314E34C: 7FE821AE  stbx r31, r8, r4
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[4].u32), ctx.r[31].u8) };
	// 8314E350: 4098001C  bge cr6, 0x8314e36c
	if !ctx.cr[6].lt {
	pc = 0x8314E36C; continue 'dispatch;
	}
	// 8314E354: 7D4B18AE  lbzx r10, r11, r3
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8314E358: 2B0A003A  cmplwi cr6, r10, 0x3a
	ctx.cr[6].compare_u32(ctx.r[10].u32, 58 as u32, &mut ctx.xer);
	// 8314E35C: 409A0010  bne cr6, 0x8314e36c
	if !ctx.cr[6].eq {
	pc = 0x8314E36C; continue 'dispatch;
	}
	// 8314E360: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8314E364: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8314E368: 4198FFEC  blt cr6, 0x8314e354
	if ctx.cr[6].lt {
	pc = 0x8314E354; continue 'dispatch;
	}
	// 8314E36C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 8314E370: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8314E374: 40980024  bge cr6, 0x8314e398
	if !ctx.cr[6].lt {
	pc = 0x8314E398; continue 'dispatch;
	}
	// 8314E378: 7D0B18AE  lbzx r8, r11, r3
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8314E37C: 2B08003A  cmplwi cr6, r8, 0x3a
	ctx.cr[6].compare_u32(ctx.r[8].u32, 58 as u32, &mut ctx.xer);
	// 8314E380: 419A0018  beq cr6, 0x8314e398
	if ctx.cr[6].eq {
	pc = 0x8314E398; continue 'dispatch;
	}
	// 8314E384: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8314E388: 7D0A29AE  stbx r8, r10, r5
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[5].u32), ctx.r[8].u8) };
	// 8314E38C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8314E390: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8314E394: 4198FFE4  blt cr6, 0x8314e378
	if ctx.cr[6].lt {
	pc = 0x8314E378; continue 'dispatch;
	}
	// 8314E398: 7FEA29AE  stbx r31, r10, r5
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[5].u32), ctx.r[31].u8) };
	// 8314E39C: EBC1FFF0  ld r30, -0x10(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314E3A0: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 8314E3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314E3A8 size=116
    let mut pc: u32 = 0x8314E3A8;
    'dispatch: loop {
        match pc {
            0x8314E3A8 => {
    //   block [0x8314E3A8..0x8314E41C)
	// 8314E3A8: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8314E3AC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8314E3B0: 419A0074  beq cr6, 0x8314e424
	if ctx.cr[6].eq {
		sub_8314E424(ctx, base);
		return;
	}
	// 8314E3B4: 89690000  lbz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E3B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314E3BC: 419A0068  beq cr6, 0x8314e424
	if ctx.cr[6].eq {
		sub_8314E424(ctx, base);
		return;
	}
	// 8314E3C0: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8314E3C4: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E3C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8314E3CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8314E3D0: 409AFFF4  bne cr6, 0x8314e3c4
	if !ctx.cr[6].eq {
	pc = 0x8314E3C4; continue 'dispatch;
	}
	// 8314E3D4: 7D495850  subf r10, r9, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 8314E3D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8314E3DC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8314E3E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314E3E4: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8314E3E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8314E3EC: 419A0020  beq cr6, 0x8314e40c
	if ctx.cr[6].eq {
	pc = 0x8314E40C; continue 'dispatch;
	}
	// 8314E3F0: 7D0B48AE  lbzx r8, r11, r9
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 8314E3F4: 2B08000A  cmplwi cr6, r8, 0xa
	ctx.cr[6].compare_u32(ctx.r[8].u32, 10 as u32, &mut ctx.xer);
	// 8314E3F8: 409A0008  bne cr6, 0x8314e400
	if !ctx.cr[6].eq {
	pc = 0x8314E400; continue 'dispatch;
	}
	// 8314E3FC: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8314E400: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8314E404: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8314E408: 4198FFE8  blt cr6, 0x8314e3f0
	if ctx.cr[6].lt {
	pc = 0x8314E3F0; continue 'dispatch;
	}
	// 8314E40C: 7D6A4A14  add r11, r10, r9
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8314E410: 894BFFFF  lbz r10, -1(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 8314E414: 2B0A000A  cmplwi cr6, r10, 0xa
	ctx.cr[6].compare_u32(ctx.r[10].u32, 10 as u32, &mut ctx.xer);
	// 8314E418: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E41C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314E41C size=8
    let mut pc: u32 = 0x8314E41C;
    'dispatch: loop {
        match pc {
            0x8314E41C => {
    //   block [0x8314E41C..0x8314E424)
	// 8314E41C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8314E420: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E424(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314E424 size=8
    let mut pc: u32 = 0x8314E424;
    'dispatch: loop {
        match pc {
            0x8314E424 => {
    //   block [0x8314E424..0x8314E42C)
	// 8314E424: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314E428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314E430 size=140
    let mut pc: u32 = 0x8314E430;
    'dispatch: loop {
        match pc {
            0x8314E430 => {
    //   block [0x8314E430..0x8314E4BC)
	// 8314E430: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8314E434: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E438: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8314E43C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8314E440: 409AFFF4  bne cr6, 0x8314e434
	if !ctx.cr[6].eq {
	pc = 0x8314E434; continue 'dispatch;
	}
	// 8314E444: 7D435850  subf r10, r3, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 8314E448: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8314E44C: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8314E450: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 8314E454: 5548003E  slwi r8, r10, 0
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 8314E458: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8314E45C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8314E460: 419A0028  beq cr6, 0x8314e488
	if ctx.cr[6].eq {
	pc = 0x8314E488; continue 'dispatch;
	}
	// 8314E464: 7F0A2040  cmplw cr6, r10, r4
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[4].u32, &mut ctx.xer);
	// 8314E468: 419A0020  beq cr6, 0x8314e488
	if ctx.cr[6].eq {
	pc = 0x8314E488; continue 'dispatch;
	}
	// 8314E46C: 7D2B18AE  lbzx r9, r11, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8314E470: 2B09000A  cmplwi cr6, r9, 0xa
	ctx.cr[6].compare_u32(ctx.r[9].u32, 10 as u32, &mut ctx.xer);
	// 8314E474: 409A0008  bne cr6, 0x8314e47c
	if !ctx.cr[6].eq {
	pc = 0x8314E47C; continue 'dispatch;
	}
	// 8314E478: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8314E47C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8314E480: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8314E484: 4198FFE0  blt cr6, 0x8314e464
	if ctx.cr[6].lt {
	pc = 0x8314E464; continue 'dispatch;
	}
	// 8314E488: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8314E48C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8314E490: 40980024  bge cr6, 0x8314e4b4
	if !ctx.cr[6].lt {
	pc = 0x8314E4B4; continue 'dispatch;
	}
	// 8314E494: 7D2B18AE  lbzx r9, r11, r3
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8314E498: 2B09000A  cmplwi cr6, r9, 0xa
	ctx.cr[6].compare_u32(ctx.r[9].u32, 10 as u32, &mut ctx.xer);
	// 8314E49C: 419A0018  beq cr6, 0x8314e4b4
	if ctx.cr[6].eq {
	pc = 0x8314E4B4; continue 'dispatch;
	}
	// 8314E4A0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8314E4A4: 7D2A29AE  stbx r9, r10, r5
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[5].u32), ctx.r[9].u8) };
	// 8314E4A8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8314E4AC: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8314E4B0: 4198FFE4  blt cr6, 0x8314e494
	if ctx.cr[6].lt {
	pc = 0x8314E494; continue 'dispatch;
	}
	// 8314E4B4: 7CEA29AE  stbx r7, r10, r5
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[5].u32), ctx.r[7].u8) };
	// 8314E4B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8314E4C0 size=784
    let mut pc: u32 = 0x8314E4C0;
    'dispatch: loop {
        match pc {
            0x8314E4C0 => {
    //   block [0x8314E4C0..0x8314E5B8)
	// 8314E4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314E4C4: 48059CA5  bl 0x831a8168
	ctx.lr = 0x8314E4C8;
	sub_831A8130(ctx, base);
	// 8314E4C8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314E4CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8314E4D0: FC800890  fmr f4, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[4].f64 = ctx.f[1].f64;
	// 8314E4D4: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 8314E4D8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8314E4DC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8314E4E0: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 8314E4E4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314E4E8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8314E4EC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8314E4F0: 4200FFF8  bdnz 0x8314e4e8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8314E4E8; continue 'dispatch;
	}
	// 8314E4F4: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314E4F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8314E4FC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8314E500: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314E504: 409902C4  ble cr6, 0x8314e7c8
	if !ctx.cr[6].gt {
	pc = 0x8314E7C8; continue 'dispatch;
	}
	// 8314E508: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8314E50C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8314E510: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8314E514: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8314E518: C0CB08A8  lfs f6, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 8314E51C: C0AA08A4  lfs f5, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 8314E520: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 8314E524: FC202090  fmr f1, f4
	ctx.f[1].f64 = ctx.f[4].f64;
	// 8314E528: 7F9F582E  lwzx r28, r31, r11
	ctx.r[28].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314E52C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8314E530: 4BFFFBA1  bl 0x8314e0d0
	ctx.lr = 0x8314E534;
	sub_8314E0D0(ctx, base);
	// 8314E534: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314E538: 2B0B0017  cmplwi cr6, r11, 0x17
	ctx.cr[6].compare_u32(ctx.r[11].u32, 23 as u32, &mut ctx.xer);
	// 8314E53C: 41990244  bgt cr6, 0x8314e780
	if ctx.cr[6].gt {
	pc = 0x8314E780; continue 'dispatch;
	}
	// 8314E540: 3D808315  lis r12, -0x7ceb
	ctx.r[12].s64 = -2095775744;
	// 8314E544: 398CE558  addi r12, r12, -0x1aa8
	ctx.r[12].s64 = ctx.r[12].s64 + -6824;
	// 8314E548: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8314E54C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8314E550: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8314E554: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x8314E5B8; continue 'dispatch;
		},
		1 => {
	pc = 0x8314E5D4; continue 'dispatch;
		},
		2 => {
	pc = 0x8314E5E8; continue 'dispatch;
		},
		3 => {
	pc = 0x8314E61C; continue 'dispatch;
		},
		4 => {
	pc = 0x8314E5FC; continue 'dispatch;
		},
		5 => {
	pc = 0x8314E630; continue 'dispatch;
		},
		6 => {
	pc = 0x8314E63C; continue 'dispatch;
		},
		7 => {
	pc = 0x8314E648; continue 'dispatch;
		},
		8 => {
	pc = 0x8314E654; continue 'dispatch;
		},
		9 => {
	pc = 0x8314E660; continue 'dispatch;
		},
		10 => {
	pc = 0x8314E66C; continue 'dispatch;
		},
		11 => {
	pc = 0x8314E678; continue 'dispatch;
		},
		12 => {
	pc = 0x8314E684; continue 'dispatch;
		},
		13 => {
	pc = 0x8314E690; continue 'dispatch;
		},
		14 => {
	pc = 0x8314E6A4; continue 'dispatch;
		},
		15 => {
	pc = 0x8314E6B8; continue 'dispatch;
		},
		16 => {
	pc = 0x8314E6CC; continue 'dispatch;
		},
		17 => {
	pc = 0x8314E6E0; continue 'dispatch;
		},
		18 => {
	pc = 0x8314E6F4; continue 'dispatch;
		},
		19 => {
	pc = 0x8314E708; continue 'dispatch;
		},
		20 => {
	pc = 0x8314E71C; continue 'dispatch;
		},
		21 => {
	pc = 0x8314E730; continue 'dispatch;
		},
		22 => {
	pc = 0x8314E744; continue 'dispatch;
		},
		23 => {
	pc = 0x8314E758; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8314E558: 8314E5B8  lwz r24, -0x1a48(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6728 as u32) ) } as u64;
	// 8314E55C: 8314E5D4  lwz r24, -0x1a2c(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6700 as u32) ) } as u64;
	// 8314E560: 8314E5E8  lwz r24, -0x1a18(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6680 as u32) ) } as u64;
	// 8314E564: 8314E61C  lwz r24, -0x19e4(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6628 as u32) ) } as u64;
	// 8314E568: 8314E5FC  lwz r24, -0x1a04(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6660 as u32) ) } as u64;
	// 8314E56C: 8314E630  lwz r24, -0x19d0(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6608 as u32) ) } as u64;
	// 8314E570: 8314E63C  lwz r24, -0x19c4(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6596 as u32) ) } as u64;
	// 8314E574: 8314E648  lwz r24, -0x19b8(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6584 as u32) ) } as u64;
	// 8314E578: 8314E654  lwz r24, -0x19ac(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6572 as u32) ) } as u64;
	// 8314E57C: 8314E660  lwz r24, -0x19a0(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6560 as u32) ) } as u64;
	// 8314E580: 8314E66C  lwz r24, -0x1994(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6548 as u32) ) } as u64;
	// 8314E584: 8314E678  lwz r24, -0x1988(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6536 as u32) ) } as u64;
	// 8314E588: 8314E684  lwz r24, -0x197c(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6524 as u32) ) } as u64;
	// 8314E58C: 8314E690  lwz r24, -0x1970(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6512 as u32) ) } as u64;
	// 8314E590: 8314E6A4  lwz r24, -0x195c(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6492 as u32) ) } as u64;
	// 8314E594: 8314E6B8  lwz r24, -0x1948(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6472 as u32) ) } as u64;
	// 8314E598: 8314E6CC  lwz r24, -0x1934(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6452 as u32) ) } as u64;
	// 8314E59C: 8314E6E0  lwz r24, -0x1920(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6432 as u32) ) } as u64;
	// 8314E5A0: 8314E6F4  lwz r24, -0x190c(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6412 as u32) ) } as u64;
	// 8314E5A4: 8314E708  lwz r24, -0x18f8(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6392 as u32) ) } as u64;
	// 8314E5A8: 8314E71C  lwz r24, -0x18e4(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6372 as u32) ) } as u64;
	// 8314E5AC: 8314E730  lwz r24, -0x18d0(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6352 as u32) ) } as u64;
	// 8314E5B0: 8314E744  lwz r24, -0x18bc(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6332 as u32) ) } as u64;
	// 8314E5B4: 8314E758  lwz r24, -0x18a8(r20)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(-6312 as u32) ) } as u64;
            }
            0x8314E5B8 => {
    //   block [0x8314E5B8..0x8314E5D4)
	// 8314E5B8: C0060024  lfs f0, 0x24(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E5BC: C1A60020  lfs f13, 0x20(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E5C0: ED800072  fmuls f12, f0, f1
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 8314E5C4: ED60697A  fmadds f11, f0, f5, f13
	ctx.f[11].f64 = (((ctx.f[0].f64 * ctx.f[5].f64 + ctx.f[13].f64) as f32) as f64);
	// 8314E5C8: D1660020  stfs f11, 0x20(r6)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8314E5CC: D1860024  stfs f12, 0x24(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8314E5D0: 480001B0  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E5D4 => {
    //   block [0x8314E5D4..0x8314E5E8)
	// 8314E5D4: C0060028  lfs f0, 0x28(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E5D8: C1A6002C  lfs f13, 0x2c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E5DC: ED8D007A  fmadds f12, f13, f1, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8314E5E0: D1860028  stfs f12, 0x28(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 8314E5E4: 4800019C  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E5E8 => {
    //   block [0x8314E5E8..0x8314E5FC)
	// 8314E5E8: C0060030  lfs f0, 0x30(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(48 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E5EC: C1A60034  lfs f13, 0x34(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(52 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E5F0: ED8D007A  fmadds f12, f13, f1, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8314E5F4: D1860030  stfs f12, 0x30(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8314E5F8: 48000188  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E5FC => {
    //   block [0x8314E5FC..0x8314E61C)
	// 8314E5FC: C006007C  lfs f0, 0x7c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E600: EDA60828  fsubs f13, f6, f1
	ctx.f[13].f64 = (((ctx.f[6].f64 - ctx.f[1].f64) as f32) as f64);
	// 8314E604: ED860028  fsubs f12, f6, f0
	ctx.f[12].f64 = (((ctx.f[6].f64 - ctx.f[0].f64) as f32) as f64);
	// 8314E608: C1660080  lfs f11, 0x80(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(128 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 8314E60C: ED4D62FA  fmadds f10, f13, f11, f12
	ctx.f[10].f64 = (((ctx.f[13].f64 * ctx.f[11].f64 + ctx.f[12].f64) as f32) as f64);
	// 8314E610: ED265028  fsubs f9, f6, f10
	ctx.f[9].f64 = (((ctx.f[6].f64 - ctx.f[10].f64) as f32) as f64);
	// 8314E614: D126007C  stfs f9, 0x7c(r6)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 8314E618: 48000168  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E61C => {
    //   block [0x8314E61C..0x8314E630)
	// 8314E61C: C0060074  lfs f0, 0x74(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(116 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E620: C1A60078  lfs f13, 0x78(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(120 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E624: ED8D007A  fmadds f12, f13, f1, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8314E628: D1860074  stfs f12, 0x74(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8314E62C: 48000154  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E630 => {
    //   block [0x8314E630..0x8314E63C)
	// 8314E630: D0210050  stfs f1, 0x50(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 8314E634: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8314E638: 48000148  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E63C => {
    //   block [0x8314E63C..0x8314E648)
	// 8314E63C: D0210054  stfs f1, 0x54(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8314E640: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8314E644: 4800013C  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E648 => {
    //   block [0x8314E648..0x8314E654)
	// 8314E648: D0210058  stfs f1, 0x58(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 8314E64C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8314E650: 48000130  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E654 => {
    //   block [0x8314E654..0x8314E660)
	// 8314E654: D021005C  stfs f1, 0x5c(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8314E658: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8314E65C: 48000124  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E660 => {
    //   block [0x8314E660..0x8314E66C)
	// 8314E660: D0210060  stfs f1, 0x60(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 8314E664: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8314E668: 48000118  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E66C => {
    //   block [0x8314E66C..0x8314E678)
	// 8314E66C: D0210064  stfs f1, 0x64(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 8314E670: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8314E674: 4800010C  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E678 => {
    //   block [0x8314E678..0x8314E684)
	// 8314E678: D0210068  stfs f1, 0x68(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 8314E67C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8314E680: 48000100  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E684 => {
    //   block [0x8314E684..0x8314E690)
	// 8314E684: D021006C  stfs f1, 0x6c(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8314E688: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8314E68C: 480000F4  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E690 => {
    //   block [0x8314E690..0x8314E6A4)
	// 8314E690: C00602F4  lfs f0, 0x2f4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(756 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E694: C1A60314  lfs f13, 0x314(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(788 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E698: ED8D007A  fmadds f12, f13, f1, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8314E69C: D18602F4  stfs f12, 0x2f4(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(756 as u32), tmp.u32 ) };
	// 8314E6A0: 480000E0  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E6A4 => {
    //   block [0x8314E6A4..0x8314E6B8)
	// 8314E6A4: C00602F8  lfs f0, 0x2f8(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(760 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E6A8: C1A60318  lfs f13, 0x318(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(792 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E6AC: ED8D007A  fmadds f12, f13, f1, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8314E6B0: D18602F8  stfs f12, 0x2f8(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(760 as u32), tmp.u32 ) };
	// 8314E6B4: 480000CC  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E6B8 => {
    //   block [0x8314E6B8..0x8314E6CC)
	// 8314E6B8: C00602FC  lfs f0, 0x2fc(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(764 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E6BC: C1A6031C  lfs f13, 0x31c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(796 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E6C0: ED8D007A  fmadds f12, f13, f1, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8314E6C4: D18602FC  stfs f12, 0x2fc(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(764 as u32), tmp.u32 ) };
	// 8314E6C8: 480000B8  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E6CC => {
    //   block [0x8314E6CC..0x8314E6E0)
	// 8314E6CC: C0060300  lfs f0, 0x300(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(768 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E6D0: C1A60320  lfs f13, 0x320(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(800 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E6D4: ED8D007A  fmadds f12, f13, f1, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8314E6D8: D1860300  stfs f12, 0x300(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(768 as u32), tmp.u32 ) };
	// 8314E6DC: 480000A4  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E6E0 => {
    //   block [0x8314E6E0..0x8314E6F4)
	// 8314E6E0: C0060304  lfs f0, 0x304(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(772 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E6E4: C1A60324  lfs f13, 0x324(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(804 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E6E8: ED8D007A  fmadds f12, f13, f1, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8314E6EC: D1860304  stfs f12, 0x304(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(772 as u32), tmp.u32 ) };
	// 8314E6F0: 48000090  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E6F4 => {
    //   block [0x8314E6F4..0x8314E708)
	// 8314E6F4: C0060308  lfs f0, 0x308(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(776 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E6F8: C1A60328  lfs f13, 0x328(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(808 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E6FC: ED8D007A  fmadds f12, f13, f1, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8314E700: D1860308  stfs f12, 0x308(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(776 as u32), tmp.u32 ) };
	// 8314E704: 4800007C  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E708 => {
    //   block [0x8314E708..0x8314E71C)
	// 8314E708: C006030C  lfs f0, 0x30c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(780 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E70C: C1A6032C  lfs f13, 0x32c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(812 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E710: ED8D007A  fmadds f12, f13, f1, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8314E714: D186030C  stfs f12, 0x30c(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(780 as u32), tmp.u32 ) };
	// 8314E718: 48000068  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E71C => {
    //   block [0x8314E71C..0x8314E730)
	// 8314E71C: C0060310  lfs f0, 0x310(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(784 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E720: C1A60330  lfs f13, 0x330(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(816 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E724: ED8D007A  fmadds f12, f13, f1, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8314E728: D1860310  stfs f12, 0x310(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(784 as u32), tmp.u32 ) };
	// 8314E72C: 48000054  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E730 => {
    //   block [0x8314E730..0x8314E744)
	// 8314E730: C0060360  lfs f0, 0x360(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(864 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E734: C1A60364  lfs f13, 0x364(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(868 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E738: ED8D007A  fmadds f12, f13, f1, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8314E73C: D1860360  stfs f12, 0x360(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(864 as u32), tmp.u32 ) };
	// 8314E740: 48000040  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E744 => {
    //   block [0x8314E744..0x8314E758)
	// 8314E744: C0060368  lfs f0, 0x368(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(872 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E748: C1A6036C  lfs f13, 0x36c(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(876 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314E74C: ED8D007A  fmadds f12, f13, f1, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64);
	// 8314E750: D1860368  stfs f12, 0x368(r6)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(872 as u32), tmp.u32 ) };
	// 8314E754: 4800002C  b 0x8314e780
	pc = 0x8314E780; continue 'dispatch;
            }
            0x8314E758 => {
    //   block [0x8314E758..0x8314E7D0)
	// 8314E758: 81660390  lwz r11, 0x390(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(912 as u32) ) } as u64;
	// 8314E75C: 556A0084  rlwinm r10, r11, 0, 2, 2
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8314E760: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8314E764: 409A0010  bne cr6, 0x8314e774
	if !ctx.cr[6].eq {
	pc = 0x8314E774; continue 'dispatch;
	}
	// 8314E768: 656B2000  oris r11, r11, 0x2000
	ctx.r[11].u64 = ctx.r[11].u64 | 536870912;
	// 8314E76C: 908600A8  stw r4, 0xa8(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(168 as u32), ctx.r[4].u32 ) };
	// 8314E770: 91660390  stw r11, 0x390(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(912 as u32), ctx.r[11].u32 ) };
	// 8314E774: C0060374  lfs f0, 0x374(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(884 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E778: EDA0082A  fadds f13, f0, f1
	ctx.f[13].f64 = ((ctx.f[0].f64 + ctx.f[1].f64) as f32) as f64;
	// 8314E77C: D1A60374  stfs f13, 0x374(r6)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(884 as u32), tmp.u32 ) };
	// 8314E780: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314E784: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8314E788: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8314E78C: 7F1D5840  cmplw cr6, r29, r11
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8314E790: 4198FD90  blt cr6, 0x8314e520
	if ctx.cr[6].lt {
	pc = 0x8314E520; continue 'dispatch;
	}
	// 8314E794: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 8314E798: 409A0030  bne cr6, 0x8314e7c8
	if !ctx.cr[6].eq {
	pc = 0x8314E7C8; continue 'dispatch;
	}
	// 8314E79C: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 8314E7A0: 3D403F80  lis r10, 0x3f80
	ctx.r[10].s64 = 1065353216;
	// 8314E7A4: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8314E7A8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8314E7AC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8314E7B0: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8314E7B4: 4200FFF8  bdnz 0x8314e7ac
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8314E7AC; continue 'dispatch;
	}
	// 8314E7B8: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8314E7BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8314E7C0: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 8314E7C4: 4BFFF7AD  bl 0x8314df70
	ctx.lr = 0x8314E7C8;
	sub_8314DF70(ctx, base);
	// 8314E7C8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8314E7CC: 480599EC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E7D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8314E7D0 size=88
    let mut pc: u32 = 0x8314E7D0;
    'dispatch: loop {
        match pc {
            0x8314E7D0 => {
    //   block [0x8314E7D0..0x8314E828)
	// 8314E7D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8314E7D4: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 8314E7D8: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E7DC: 409A000C  bne cr6, 0x8314e7e8
	if !ctx.cr[6].eq {
	pc = 0x8314E7E8; continue 'dispatch;
	}
	// 8314E7E0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8314E7E4: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314E7E8: 81230014  lwz r9, 0x14(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314E7EC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8314E7F0: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8314E7F4: 419A002C  beq cr6, 0x8314e820
	if ctx.cr[6].eq {
	pc = 0x8314E820; continue 'dispatch;
	}
	// 8314E7F8: 81030018  lwz r8, 0x18(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 8314E7FC: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 8314E800: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E804: 80C70004  lwz r6, 4(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314E808: 7F053000  cmpw cr6, r5, r6
	ctx.cr[6].compare_i32(ctx.r[5].s32, ctx.r[6].s32, &mut ctx.xer);
	// 8314E80C: 419A001C  beq cr6, 0x8314e828
	if ctx.cr[6].eq {
		sub_8314E828(ctx, base);
		return;
	}
	// 8314E810: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8314E814: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8314E818: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8314E81C: 4198FFE4  blt cr6, 0x8314e800
	if ctx.cr[6].lt {
	pc = 0x8314E800; continue 'dispatch;
	}
	// 8314E820: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 8314E824: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314E828 size=12
    let mut pc: u32 = 0x8314E828;
    'dispatch: loop {
        match pc {
            0x8314E828 => {
    //   block [0x8314E828..0x8314E834)
	// 8314E828: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8314E82C: 7C6B402E  lwzx r3, r11, r8
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8314E830: 4BFFF8A0  b 0x8314e0d0
	sub_8314E0D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314E838 size=80
    let mut pc: u32 = 0x8314E838;
    'dispatch: loop {
        match pc {
            0x8314E838 => {
    //   block [0x8314E838..0x8314E888)
	// 8314E838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314E83C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8314E840: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8314E844: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314E848: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314E84C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E850: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8314E854: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314E858: 4E800421  bctrl
	ctx.lr = 0x8314E85C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314E85C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E860: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8314E864: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314E868: 81090014  lwz r8, 0x14(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314E86C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8314E870: 4E800421  bctrl
	ctx.lr = 0x8314E874;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314E874: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8314E878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314E87C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314E880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314E884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314E888 size=20
    let mut pc: u32 = 0x8314E888;
    'dispatch: loop {
        match pc {
            0x8314E888 => {
    //   block [0x8314E888..0x8314E89C)
	// 8314E888: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314E88C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E890: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314E894: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314E898: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314E8A0 size=20
    let mut pc: u32 = 0x8314E8A0;
    'dispatch: loop {
        match pc {
            0x8314E8A0 => {
    //   block [0x8314E8A0..0x8314E8B4)
	// 8314E8A0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314E8A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E8A8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314E8AC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314E8B0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314E8B8 size=12
    let mut pc: u32 = 0x8314E8B8;
    'dispatch: loop {
        match pc {
            0x8314E8B8 => {
    //   block [0x8314E8B8..0x8314E8C4)
	// 8314E8B8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314E8BC: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314E8C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E8C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8314E8C8 size=12
    let mut pc: u32 = 0x8314E8C8;
    'dispatch: loop {
        match pc {
            0x8314E8C8 => {
    //   block [0x8314E8C8..0x8314E8D4)
	// 8314E8C8: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314E8CC: C02B0010  lfs f1, 0x10(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8314E8D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E8D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314E8D8 size=80
    let mut pc: u32 = 0x8314E8D8;
    'dispatch: loop {
        match pc {
            0x8314E8D8 => {
    //   block [0x8314E8D8..0x8314E928)
	// 8314E8D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314E8DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8314E8E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8314E8E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314E8E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314E8EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E8F0: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 8314E8F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314E8F8: 4E800421  bctrl
	ctx.lr = 0x8314E8FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314E8FC: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E900: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8314E904: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314E908: 81090028  lwz r8, 0x28(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 8314E90C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8314E910: 4E800421  bctrl
	ctx.lr = 0x8314E914;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314E914: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8314E918: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314E91C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314E920: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314E924: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314E928 size=204
    let mut pc: u32 = 0x8314E928;
    'dispatch: loop {
        match pc {
            0x8314E928 => {
    //   block [0x8314E928..0x8314E9F4)
	// 8314E928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314E92C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8314E930: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8314E934: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314E938: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314E93C: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 8314E940: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8314E944: 38AA4EE0  addi r5, r10, 0x4ee0
	ctx.r[5].s64 = ctx.r[10].s64 + 20192;
	// 8314E948: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8314E94C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314E950: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314E954: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8314E958: 48010DA9  bl 0x8315f700
	ctx.lr = 0x8314E95C;
	sub_8315F700(ctx, base);
	// 8314E95C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8314E960: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8314E964: 409A0044  bne cr6, 0x8314e9a8
	if !ctx.cr[6].eq {
	pc = 0x8314E9A8; continue 'dispatch;
	}
	// 8314E968: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314E96C: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8314E970: 388B4ED4  addi r4, r11, 0x4ed4
	ctx.r[4].s64 = ctx.r[11].s64 + 20180;
	// 8314E974: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314E978: 480111C9  bl 0x8315fb40
	ctx.lr = 0x8314E97C;
	sub_8315FB40(ctx, base);
	// 8314E97C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314E980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314E984: 812A0030  lwz r9, 0x30(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(48 as u32) ) } as u64;
	// 8314E988: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8314E98C: 4E800421  bctrl
	ctx.lr = 0x8314E990;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314E990: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314E994: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8314E998: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314E99C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314E9A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314E9A4: 4E800020  blr
	return;
	// 8314E9A8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314E9AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8314E9B0: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 8314E9B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314E9B8: 40990024  ble cr6, 0x8314e9dc
	if !ctx.cr[6].gt {
	pc = 0x8314E9DC; continue 'dispatch;
	}
	// 8314E9BC: 7D2B4B78  mr r11, r9
	ctx.r[11].u64 = ctx.r[9].u64;
	// 8314E9C0: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314E9C4: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8314E9C8: 7D28592E  stwx r9, r8, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 8314E9CC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8314E9D0: 80FF000C  lwz r7, 0xc(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314E9D4: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 8314E9D8: 4198FFE8  blt cr6, 0x8314e9c0
	if ctx.cr[6].lt {
	pc = 0x8314E9C0; continue 'dispatch;
	}
	// 8314E9DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8314E9E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8314E9E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314E9E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314E9EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314E9F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314E9F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314E9F8 size=136
    let mut pc: u32 = 0x8314E9F8;
    'dispatch: loop {
        match pc {
            0x8314E9F8 => {
    //   block [0x8314E9F8..0x8314EA80)
	// 8314E9F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314E9FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8314EA00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8314EA04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8314EA08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314EA0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314EA10: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EA14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314EA18: 419A0050  beq cr6, 0x8314ea68
	if ctx.cr[6].eq {
	pc = 0x8314EA68; continue 'dispatch;
	}
	// 8314EA1C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314EA20: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8314EA24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314EA28: 4099002C  ble cr6, 0x8314ea54
	if !ctx.cr[6].gt {
	pc = 0x8314EA54; continue 'dispatch;
	}
	// 8314EA2C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EA30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8314EA34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314EA38: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EA3C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314EA40: 4E800421  bctrl
	ctx.lr = 0x8314EA44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314EA44: 813F000C  lwz r9, 0xc(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314EA48: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8314EA4C: 7F1E4840  cmplw cr6, r30, r9
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8314EA50: 4198FFDC  blt cr6, 0x8314ea2c
	if ctx.cr[6].lt {
	pc = 0x8314EA2C; continue 'dispatch;
	}
	// 8314EA54: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EA58: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314EA5C: 48010D0D  bl 0x8315f768
	ctx.lr = 0x8314EA60;
	sub_8315F768(ctx, base);
	// 8314EA60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8314EA64: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8314EA68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314EA6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314EA70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314EA74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8314EA78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314EA7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314EA80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314EA80 size=140
    let mut pc: u32 = 0x8314EA80;
    'dispatch: loop {
        match pc {
            0x8314EA80 => {
    //   block [0x8314EA80..0x8314EB0C)
	// 8314EA80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314EA84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8314EA88: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8314EA8C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8314EA90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314EA94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314EA98: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8314EA9C: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314EAA0: 7F053040  cmplw cr6, r5, r6
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[6].u32, &mut ctx.xer);
	// 8314EAA4: 41980018  blt cr6, 0x8314eabc
	if ctx.cr[6].lt {
	pc = 0x8314EABC; continue 'dispatch;
	}
	// 8314EAA8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314EAAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314EAB0: 388B4EF0  addi r4, r11, 0x4ef0
	ctx.r[4].s64 = ctx.r[11].s64 + 20208;
	// 8314EAB4: 48011085  bl 0x8315fb38
	ctx.lr = 0x8314EAB8;
	sub_8315FB38(ctx, base);
	// 8314EAB8: 4800003C  b 0x8314eaf4
	pc = 0x8314EAF4; continue 'dispatch;
	// 8314EABC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EAC0: 54BE103A  slwi r30, r5, 2
	ctx.r[30].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8314EAC4: 7D4BF02E  lwzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8314EAC8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8314EACC: 419A0028  beq cr6, 0x8314eaf4
	if ctx.cr[6].eq {
	pc = 0x8314EAF4; continue 'dispatch;
	}
	// 8314EAD0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8314EAD4: 7C6BF02E  lwzx r3, r11, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8314EAD8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EADC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EAE0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8314EAE4: 4E800421  bctrl
	ctx.lr = 0x8314EAE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314EAE8: 80FF0008  lwz r7, 8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EAEC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8314EAF0: 7D07F12E  stwx r8, r7, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[7].u32.wrapping_add(ctx.r[30].u32), ctx.r[8].u32) };
	// 8314EAF4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314EAF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314EAFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314EB00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8314EB04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314EB08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314EB10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314EB10 size=124
    let mut pc: u32 = 0x8314EB10;
    'dispatch: loop {
        match pc {
            0x8314EB10 => {
    //   block [0x8314EB10..0x8314EB8C)
	// 8314EB10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314EB14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8314EB18: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314EB1C: 80C3000C  lwz r6, 0xc(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314EB20: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8314EB24: 7F053040  cmplw cr6, r5, r6
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[6].u32, &mut ctx.xer);
	// 8314EB28: 41980028  blt cr6, 0x8314eb50
	if ctx.cr[6].lt {
	pc = 0x8314EB50; continue 'dispatch;
	}
	// 8314EB2C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314EB30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314EB34: 388B4EF0  addi r4, r11, 0x4ef0
	ctx.r[4].s64 = ctx.r[11].s64 + 20208;
	// 8314EB38: 48011001  bl 0x8315fb38
	ctx.lr = 0x8314EB3C;
	sub_8315FB38(ctx, base);
	// 8314EB3C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314EB40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8314EB44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314EB48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314EB4C: 4E800020  blr
	return;
	// 8314EB50: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EB54: 54AB103A  slwi r11, r5, 2
	ctx.r[11].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8314EB58: 7D2A582E  lwzx r9, r10, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314EB5C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8314EB60: 419AFFDC  beq cr6, 0x8314eb3c
	if ctx.cr[6].eq {
	pc = 0x8314EB3C; continue 'dispatch;
	}
	// 8314EB64: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8314EB68: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314EB6C: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EB70: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314EB74: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8314EB78: 4E800421  bctrl
	ctx.lr = 0x8314EB7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314EB7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8314EB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314EB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314EB88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314EB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8314EB90 size=232
    let mut pc: u32 = 0x8314EB90;
    'dispatch: loop {
        match pc {
            0x8314EB90 => {
    //   block [0x8314EB90..0x8314EC78)
	// 8314EB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314EB94: 480595C9  bl 0x831a815c
	ctx.lr = 0x8314EB98;
	sub_831A8130(ctx, base);
	// 8314EB98: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 8314EB9C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314EBA0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8314EBA4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8314EBA8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8314EBAC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314EBB0: 833A0388  lwz r25, 0x388(r26)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(904 as u32) ) } as u64;
	// 8314EBB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314EBB8: 409900B4  ble cr6, 0x8314ec6c
	if !ctx.cr[6].gt {
	pc = 0x8314EC6C; continue 'dispatch;
	}
	// 8314EBBC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8314EBC0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8314EBC4: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8314EBC8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EBCC: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314EBD0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8314EBD4: 419A0084  beq cr6, 0x8314ec58
	if ctx.cr[6].eq {
	pc = 0x8314EC58; continue 'dispatch;
	}
	// 8314EBD8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8314EBDC: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314EBE0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EBE4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314EBE8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8314EBEC: 4E800421  bctrl
	ctx.lr = 0x8314EBF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314EBF0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8314EBF4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8314EBF8: 4BFF1441  bl 0x83140038
	ctx.lr = 0x8314EBFC;
	sub_83140038(ctx, base);
	// 8314EBFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8314EC00: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8314EC04: 419A0034  beq cr6, 0x8314ec38
	if ctx.cr[6].eq {
	pc = 0x8314EC38; continue 'dispatch;
	}
	// 8314EC08: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EC0C: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EC10: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314EC14: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EC18: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8314EC1C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8314EC20: 4E800421  bctrl
	ctx.lr = 0x8314EC24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314EC24: 811C0034  lwz r8, 0x34(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(52 as u32) ) } as u64;
	// 8314EC28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8314EC2C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8314EC30: 4E800421  bctrl
	ctx.lr = 0x8314EC34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314EC34: 48000008  b 0x8314ec3c
	pc = 0x8314EC3C; continue 'dispatch;
	// 8314EC38: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8314EC3C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EC40: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8314EC44: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314EC48: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EC4C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314EC50: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8314EC54: 4E800421  bctrl
	ctx.lr = 0x8314EC58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314EC58: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314EC5C: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8314EC60: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8314EC64: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8314EC68: 4198FF60  blt cr6, 0x8314ebc8
	if ctx.cr[6].lt {
	pc = 0x8314EBC8; continue 'dispatch;
	}
	// 8314EC6C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8314EC70: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 8314EC74: 48059538  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314EC78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8314EC78 size=232
    let mut pc: u32 = 0x8314EC78;
    'dispatch: loop {
        match pc {
            0x8314EC78 => {
    //   block [0x8314EC78..0x8314ED60)
	// 8314EC78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314EC7C: 480594E1  bl 0x831a815c
	ctx.lr = 0x8314EC80;
	sub_831A8130(ctx, base);
	// 8314EC80: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 8314EC84: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314EC88: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8314EC8C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8314EC90: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8314EC94: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314EC98: 833A0388  lwz r25, 0x388(r26)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(904 as u32) ) } as u64;
	// 8314EC9C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314ECA0: 409900B4  ble cr6, 0x8314ed54
	if !ctx.cr[6].gt {
	pc = 0x8314ED54; continue 'dispatch;
	}
	// 8314ECA4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8314ECA8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8314ECAC: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8314ECB0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314ECB4: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314ECB8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8314ECBC: 419A0084  beq cr6, 0x8314ed40
	if ctx.cr[6].eq {
	pc = 0x8314ED40; continue 'dispatch;
	}
	// 8314ECC0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8314ECC4: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314ECC8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314ECCC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314ECD0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8314ECD4: 4E800421  bctrl
	ctx.lr = 0x8314ECD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314ECD8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8314ECDC: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8314ECE0: 4BFF1359  bl 0x83140038
	ctx.lr = 0x8314ECE4;
	sub_83140038(ctx, base);
	// 8314ECE4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8314ECE8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8314ECEC: 419A0034  beq cr6, 0x8314ed20
	if ctx.cr[6].eq {
	pc = 0x8314ED20; continue 'dispatch;
	}
	// 8314ECF0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314ECF4: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314ECF8: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314ECFC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314ED00: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8314ED04: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8314ED08: 4E800421  bctrl
	ctx.lr = 0x8314ED0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314ED0C: 811C0038  lwz r8, 0x38(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(56 as u32) ) } as u64;
	// 8314ED10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8314ED14: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8314ED18: 4E800421  bctrl
	ctx.lr = 0x8314ED1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314ED1C: 48000008  b 0x8314ed24
	pc = 0x8314ED24; continue 'dispatch;
	// 8314ED20: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8314ED24: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314ED28: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 8314ED2C: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314ED30: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314ED34: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314ED38: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8314ED3C: 4E800421  bctrl
	ctx.lr = 0x8314ED40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314ED40: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314ED44: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 8314ED48: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8314ED4C: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8314ED50: 4198FF60  blt cr6, 0x8314ecb0
	if ctx.cr[6].lt {
	pc = 0x8314ECB0; continue 'dispatch;
	}
	// 8314ED54: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8314ED58: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 8314ED5C: 48059450  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314ED60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314ED60 size=24
    let mut pc: u32 = 0x8314ED60;
    'dispatch: loop {
        match pc {
            0x8314ED60 => {
    //   block [0x8314ED60..0x8314ED78)
	// 8314ED60: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8314ED64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314ED68: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8314ED6C: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314ED70: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8314ED74: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314ED78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314ED78 size=96
    let mut pc: u32 = 0x8314ED78;
    'dispatch: loop {
        match pc {
            0x8314ED78 => {
    //   block [0x8314ED78..0x8314EDD8)
	// 8314ED78: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314ED7C: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314ED80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314ED84: 419A0040  beq cr6, 0x8314edc4
	if ctx.cr[6].eq {
	pc = 0x8314EDC4; continue 'dispatch;
	}
	// 8314ED88: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8314ED8C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8314ED90: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314ED94: 81490008  lwz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314ED98: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314ED9C: 88AA0000  lbz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EDA0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8314EDA4: 7D254850  subf r9, r5, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[5].s64;
	// 8314EDA8: 419A0014  beq cr6, 0x8314edbc
	if ctx.cr[6].eq {
	pc = 0x8314EDBC; continue 'dispatch;
	}
	// 8314EDAC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8314EDB0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8314EDB4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8314EDB8: 419AFFE0  beq cr6, 0x8314ed98
	if ctx.cr[6].eq {
	pc = 0x8314ED98; continue 'dispatch;
	}
	// 8314EDBC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8314EDC0: 419A0018  beq cr6, 0x8314edd8
	if ctx.cr[6].eq {
		sub_8314EDD8(ctx, base);
		return;
	}
	// 8314EDC4: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 8314EDC8: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8314EDCC: 7F073040  cmplw cr6, r7, r6
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[6].u32, &mut ctx.xer);
	// 8314EDD0: 4198FFAC  blt cr6, 0x8314ed7c
	if ctx.cr[6].lt {
	pc = 0x8314ED7C; continue 'dispatch;
	}
	// 8314EDD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314EDD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314EDD8 size=8
    let mut pc: u32 = 0x8314EDD8;
    'dispatch: loop {
        match pc {
            0x8314EDD8 => {
    //   block [0x8314EDD8..0x8314EDE0)
	// 8314EDD8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8314EDDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314EDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8314EDE0 size=332
    let mut pc: u32 = 0x8314EDE0;
    'dispatch: loop {
        match pc {
            0x8314EDE0 => {
    //   block [0x8314EDE0..0x8314EF2C)
	// 8314EDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314EDE4: 48059379  bl 0x831a815c
	ctx.lr = 0x8314EDE8;
	sub_831A8130(ctx, base);
	// 8314EDE8: DBC1FFB0  stfd f30, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[30].u64 ) };
	// 8314EDEC: DBE1FFB8  stfd f31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 8314EDF0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314EDF4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8314EDF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8314EDFC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8314EE00: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8314EE04: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 8314EE08: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8314EE0C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8314EE10: FFC0F890  fmr f30, f31
	ctx.f[30].f64 = ctx.f[31].f64;
	// 8314EE14: 419A001C  beq cr6, 0x8314ee30
	if ctx.cr[6].eq {
	pc = 0x8314EE30; continue 'dispatch;
	}
	// 8314EE18: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EE1C: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 8314EE20: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8314EE24: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314EE28: 4E800421  bctrl
	ctx.lr = 0x8314EE2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314EE2C: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 8314EE30: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8314EE34: 409A000C  bne cr6, 0x8314ee40
	if !ctx.cr[6].eq {
	pc = 0x8314EE40; continue 'dispatch;
	}
	// 8314EE38: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8314EE3C: C3EB08A8  lfs f31, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8314EE40: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314EE44: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8314EE48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314EE4C: 409900CC  ble cr6, 0x8314ef18
	if !ctx.cr[6].gt {
	pc = 0x8314EF18; continue 'dispatch;
	}
	// 8314EE50: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8314EE54: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 8314EE58: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EE5C: 7D5F582E  lwzx r10, r31, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314EE60: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8314EE64: 419A00A0  beq cr6, 0x8314ef04
	if ctx.cr[6].eq {
	pc = 0x8314EF04; continue 'dispatch;
	}
	// 8314EE68: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8314EE6C: 7F6BDB78  mr r11, r27
	ctx.r[11].u64 = ctx.r[27].u64;
	// 8314EE70: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314EE74: 81490008  lwz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EE78: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EE7C: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EE80: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8314EE84: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 8314EE88: 419A0014  beq cr6, 0x8314ee9c
	if ctx.cr[6].eq {
	pc = 0x8314EE9C; continue 'dispatch;
	}
	// 8314EE8C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8314EE90: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8314EE94: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8314EE98: 419AFFE0  beq cr6, 0x8314ee78
	if ctx.cr[6].eq {
	pc = 0x8314EE78; continue 'dispatch;
	}
	// 8314EE9C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8314EEA0: 409A0064  bne cr6, 0x8314ef04
	if !ctx.cr[6].eq {
	pc = 0x8314EF04; continue 'dispatch;
	}
	// 8314EEA4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8314EEA8: 93590000  stw r26, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 8314EEAC: 419A0034  beq cr6, 0x8314eee0
	if ctx.cr[6].eq {
	pc = 0x8314EEE0; continue 'dispatch;
	}
	// 8314EEB0: 2F1D0017  cmpwi cr6, r29, 0x17
	ctx.cr[6].compare_i32(ctx.r[29].s32, 23, &mut ctx.xer);
	// 8314EEB4: 419A002C  beq cr6, 0x8314eee0
	if ctx.cr[6].eq {
	pc = 0x8314EEE0; continue 'dispatch;
	}
	// 8314EEB8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EEBC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8314EEC0: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8314EEC4: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314EEC8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EECC: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EED0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8314EED4: 4E800421  bctrl
	ctx.lr = 0x8314EED8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314EED8: EFE1F82A  fadds f31, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ((ctx.f[1].f64 + ctx.f[31].f64) as f32) as f64;
	// 8314EEDC: 48000028  b 0x8314ef04
	pc = 0x8314EF04; continue 'dispatch;
	// 8314EEE0: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EEE4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8314EEE8: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8314EEEC: 7C7F582E  lwzx r3, r31, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314EEF0: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EEF4: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EEF8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8314EEFC: 4E800421  bctrl
	ctx.lr = 0x8314EF00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314EF00: EFE107F2  fmuls f31, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = (((ctx.f[1].f64 * ctx.f[31].f64) as f32) as f64);
	// 8314EF04: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314EF08: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8314EF0C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 8314EF10: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8314EF14: 4198FF44  blt cr6, 0x8314ee58
	if ctx.cr[6].lt {
	pc = 0x8314EE58; continue 'dispatch;
	}
	// 8314EF18: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8314EF1C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8314EF20: CBC1FFB0  lfd f30, -0x50(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 8314EF24: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 8314EF28: 48059284  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314EF30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314EF30 size=20
    let mut pc: u32 = 0x8314EF30;
    'dispatch: loop {
        match pc {
            0x8314EF30 => {
    //   block [0x8314EF30..0x8314EF44)
	// 8314EF30: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8314EF34: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314EF38: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314EF3C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8314EF40: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314EF44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314EF44 size=100
    let mut pc: u32 = 0x8314EF44;
    'dispatch: loop {
        match pc {
            0x8314EF44 => {
    //   block [0x8314EF44..0x8314EFA8)
	// 8314EF44: 810B0008  lwz r8, 8(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EF48: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 8314EF4C: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EF50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314EF54: 419A0044  beq cr6, 0x8314ef98
	if ctx.cr[6].eq {
	pc = 0x8314EF98; continue 'dispatch;
	}
	// 8314EF58: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8314EF5C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8314EF60: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314EF64: 81490008  lwz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EF68: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EF6C: 88CA0000  lbz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EF70: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8314EF74: 7D264850  subf r9, r6, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[6].s64;
	// 8314EF78: 419A0014  beq cr6, 0x8314ef8c
	if ctx.cr[6].eq {
	pc = 0x8314EF8C; continue 'dispatch;
	}
	// 8314EF7C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8314EF80: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8314EF84: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8314EF88: 419AFFE0  beq cr6, 0x8314ef68
	if ctx.cr[6].eq {
	pc = 0x8314EF68; continue 'dispatch;
	}
	// 8314EF8C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8314EF90: 409A0008  bne cr6, 0x8314ef98
	if !ctx.cr[6].eq {
	pc = 0x8314EF98; continue 'dispatch;
	}
	// 8314EF94: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 8314EF98: 34E7FFFF  addic. r7, r7, -1
	ctx.xer.ca = (ctx.r[7].u32 > (!(-1 as u32)));
	ctx.r[7].s64 = ctx.r[7].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 8314EF9C: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8314EFA0: 4082FFAC  bne 0x8314ef4c
	if !ctx.cr[0].eq {
	pc = 0x8314EF4C; continue 'dispatch;
	}
	// 8314EFA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314EFA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314EFA8 size=116
    let mut pc: u32 = 0x8314EFA8;
    'dispatch: loop {
        match pc {
            0x8314EFA8 => {
    //   block [0x8314EFA8..0x8314F01C)
	// 8314EFA8: 80C3000C  lwz r6, 0xc(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314EFAC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8314EFB0: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8314EFB4: 419A0060  beq cr6, 0x8314f014
	if ctx.cr[6].eq {
	pc = 0x8314F014; continue 'dispatch;
	}
	// 8314EFB8: 81030008  lwz r8, 8(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EFBC: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EFC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314EFC4: 419A0040  beq cr6, 0x8314f004
	if ctx.cr[6].eq {
	pc = 0x8314F004; continue 'dispatch;
	}
	// 8314EFC8: 556A003E  slwi r10, r11, 0
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8314EFCC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8314EFD0: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314EFD4: 81490008  lwz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314EFD8: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EFDC: 88AA0000  lbz r5, 0(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314EFE0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8314EFE4: 7D254850  subf r9, r5, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[5].s64;
	// 8314EFE8: 419A0014  beq cr6, 0x8314effc
	if ctx.cr[6].eq {
	pc = 0x8314EFFC; continue 'dispatch;
	}
	// 8314EFEC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8314EFF0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8314EFF4: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8314EFF8: 419AFFE0  beq cr6, 0x8314efd8
	if ctx.cr[6].eq {
	pc = 0x8314EFD8; continue 'dispatch;
	}
	// 8314EFFC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8314F000: 419A001C  beq cr6, 0x8314f01c
	if ctx.cr[6].eq {
		sub_8314F01C(ctx, base);
		return;
	}
	// 8314F004: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 8314F008: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 8314F00C: 7F073040  cmplw cr6, r7, r6
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[6].u32, &mut ctx.xer);
	// 8314F010: 4198FFAC  blt cr6, 0x8314efbc
	if ctx.cr[6].lt {
	pc = 0x8314EFBC; continue 'dispatch;
	}
	// 8314F014: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F01C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314F01C size=28
    let mut pc: u32 = 0x8314F01C;
    'dispatch: loop {
        match pc {
            0x8314F01C => {
    //   block [0x8314F01C..0x8314F038)
	// 8314F01C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314F020: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8314F024: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314F028: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314F02C: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314F030: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8314F034: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314F038 size=8
    let mut pc: u32 = 0x8314F038;
    'dispatch: loop {
        match pc {
            0x8314F038 => {
    //   block [0x8314F038..0x8314F040)
	// 8314F038: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8314F03C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314F040 size=20
    let mut pc: u32 = 0x8314F040;
    'dispatch: loop {
        match pc {
            0x8314F040 => {
    //   block [0x8314F040..0x8314F054)
	// 8314F040: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314F044: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8314F048: 814B003C  lwz r10, 0x3c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(60 as u32) ) } as u64;
	// 8314F04C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314F050: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F054(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8314F054 size=4
    let mut pc: u32 = 0x8314F054;
    'dispatch: loop {
        match pc {
            0x8314F054 => {
    //   block [0x8314F054..0x8314F058)
	// 8314F054: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8314F058 size=8
    let mut pc: u32 = 0x8314F058;
    'dispatch: loop {
        match pc {
            0x8314F058 => {
    //   block [0x8314F058..0x8314F060)
	// 8314F058: D0230020  stfs f1, 0x20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8314F05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8314F060 size=8
    let mut pc: u32 = 0x8314F060;
    'dispatch: loop {
        match pc {
            0x8314F060 => {
    //   block [0x8314F060..0x8314F068)
	// 8314F060: C0230020  lfs f1, 0x20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8314F064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8314F068 size=12
    let mut pc: u32 = 0x8314F068;
    'dispatch: loop {
        match pc {
            0x8314F068 => {
    //   block [0x8314F068..0x8314F074)
	// 8314F068: D0230018  stfs f1, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8314F06C: D043001C  stfs f2, 0x1c(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8314F070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8314F078 size=8
    let mut pc: u32 = 0x8314F078;
    'dispatch: loop {
        match pc {
            0x8314F078 => {
    //   block [0x8314F078..0x8314F080)
	// 8314F078: C0230018  lfs f1, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8314F07C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x8314F080 size=8
    let mut pc: u32 = 0x8314F080;
    'dispatch: loop {
        match pc {
            0x8314F080 => {
    //   block [0x8314F080..0x8314F088)
	// 8314F080: C023001C  lfs f1, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8314F084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8314F088 size=228
    let mut pc: u32 = 0x8314F088;
    'dispatch: loop {
        match pc {
            0x8314F088 => {
    //   block [0x8314F088..0x8314F16C)
	// 8314F088: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314F08C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8314F090: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8314F094: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8314F098: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 8314F09C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8314F0A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314F0A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314F0A8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8314F0AC: C1BF0018  lfs f13, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314F0B0: C01F001C  lfs f0, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314F0B4: EC00687A  fmadds f0, f0, f1, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[1].f64 + ctx.f[13].f64) as f32) as f64);
	// 8314F0B8: C18B08A4  lfs f12, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8314F0BC: C1BF0020  lfs f13, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314F0C0: D1BF0024  stfs f13, 0x24(r31)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8314F0C4: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 8314F0C8: 40990080  ble cr6, 0x8314f148
	if !ctx.cr[6].gt {
	pc = 0x8314F148; continue 'dispatch;
	}
	// 8314F0CC: EFED0028  fsubs f31, f13, f0
	ctx.f[31].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8314F0D0: EFCD002A  fadds f30, f13, f0
	ctx.f[30].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 8314F0D4: FF1F6000  fcmpu cr6, f31, f12
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[12].f64);
	// 8314F0D8: 41990008  bgt cr6, 0x8314f0e0
	if ctx.cr[6].gt {
	pc = 0x8314F0E0; continue 'dispatch;
	}
	// 8314F0DC: FFE06090  fmr f31, f12
	ctx.f[31].f64 = ctx.f[12].f64;
	// 8314F0E0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8314F0E4: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314F0E8: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 8314F0EC: 41980008  blt cr6, 0x8314f0f4
	if ctx.cr[6].lt {
	pc = 0x8314F0F4; continue 'dispatch;
	}
	// 8314F0F0: FFC00090  fmr f30, f0
	ctx.f[30].f64 = ctx.f[0].f64;
	// 8314F0F4: 48007245  bl 0x83156338
	ctx.lr = 0x8314F0F8;
	sub_83156338(ctx, base);
	// 8314F0F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8314F0FC: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314F100: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314F104: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8314F108: 4E800421  bctrl
	ctx.lr = 0x8314F10C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314F10C: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314F110: 7C6907B4  extsw r9, r3
	ctx.r[9].s64 = ctx.r[3].s32 as i64;
	// 8314F114: EC1EF828  fsubs f0, f30, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[30].f64 - ctx.f[31].f64) as f32) as f64);
	// 8314F118: 7D4807B4  extsw r8, r10
	ctx.r[8].s64 = ctx.r[10].s32 as i64;
	// 8314F11C: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 8314F120: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8314F124: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 8314F128: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8314F12C: FD606E9C  fcfid f11, f13
	ctx.f[11].f64 = (ctx.f[13].s64 as f64);
	// 8314F130: FD40669C  fcfid f10, f12
	ctx.f[10].f64 = (ctx.f[12].s64 as f64);
	// 8314F134: FD205818  frsp f9, f11
	ctx.f[9].f64 = (ctx.f[11].f64 as f32) as f64;
	// 8314F138: FD005018  frsp f8, f10
	ctx.f[8].f64 = (ctx.f[10].f64 as f32) as f64;
	// 8314F13C: ECE94024  fdivs f7, f9, f8
	ctx.f[7].f64 = ((ctx.f[9].f64 / ctx.f[8].f64) as f32) as f64;
	// 8314F140: ECC7F83A  fmadds f6, f7, f0, f31
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64);
	// 8314F144: D0DF0024  stfs f6, 0x24(r31)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8314F148: C03F0024  lfs f1, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8314F14C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8314F150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314F154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314F158: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 8314F15C: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8314F160: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8314F164: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314F168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8314F170 size=76
    let mut pc: u32 = 0x8314F170;
    'dispatch: loop {
        match pc {
            0x8314F170 => {
    //   block [0x8314F170..0x8314F1BC)
	// 8314F170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314F174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8314F178: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8314F17C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314F180: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314F184: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8314F188: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8314F18C: 419A0018  beq cr6, 0x8314f1a4
	if ctx.cr[6].eq {
	pc = 0x8314F1A4; continue 'dispatch;
	}
	// 8314F190: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314F194: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 8314F198: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314F19C: 4E800421  bctrl
	ctx.lr = 0x8314F1A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314F1A0: D03F0024  stfs f1, 0x24(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8314F1A4: C03F0024  lfs f1, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8314F1A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8314F1AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314F1B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314F1B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314F1B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8314F1C0 size=508
    let mut pc: u32 = 0x8314F1C0;
    'dispatch: loop {
        match pc {
            0x8314F1C0 => {
    //   block [0x8314F1C0..0x8314F3BC)
	// 8314F1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314F1C4: 48058F9D  bl 0x831a8160
	ctx.lr = 0x8314F1C8;
	sub_831A8130(ctx, base);
	// 8314F1C8: DBE1FFC0  stfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[31].u64 ) };
	// 8314F1CC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314F1D0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8314F1D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314F1D8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8314F1DC: 80DD0008  lwz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314F1E0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314F1E4: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314F1E8: 48011E59  bl 0x83161040
	ctx.lr = 0x8314F1EC;
	sub_83161040(ctx, base);
	// 8314F1EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8314F1F0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8314F1F4: 409A0044  bne cr6, 0x8314f238
	if !ctx.cr[6].eq {
	pc = 0x8314F238; continue 'dispatch;
	}
	// 8314F1F8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F1FC: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8314F200: 388B4FA8  addi r4, r11, 0x4fa8
	ctx.r[4].s64 = ctx.r[11].s64 + 20392;
	// 8314F204: 4801093D  bl 0x8315fb40
	ctx.lr = 0x8314F208;
	sub_8315FB40(ctx, base);
	// 8314F208: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314F20C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8314F210: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8314F214: 419A0010  beq cr6, 0x8314f224
	if ctx.cr[6].eq {
	pc = 0x8314F224; continue 'dispatch;
	}
	// 8314F218: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314F21C: 4801054D  bl 0x8315f768
	ctx.lr = 0x8314F220;
	sub_8315F768(ctx, base);
	// 8314F220: 939F000C  stw r28, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[28].u32 ) };
	// 8314F224: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F228: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 8314F22C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8314F230: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 8314F234: 48058F7C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8314F238: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8314F23C: 4BF04DFD  bl 0x83054038
	ctx.lr = 0x8314F240;
	sub_83054038(ctx, base);
	// 8314F240: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 8314F244: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8314F248: 48011721  bl 0x83160968
	ctx.lr = 0x8314F24C;
	sub_83160968(ctx, base);
	// 8314F24C: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314F250: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F254: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314F258: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8314F25C: 38AB4F90  addi r5, r11, 0x4f90
	ctx.r[5].s64 = ctx.r[11].s64 + 20368;
	// 8314F260: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8314F264: 55441838  slwi r4, r10, 3
	ctx.r[4].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8314F268: 48010499  bl 0x8315f700
	ctx.lr = 0x8314F26C;
	sub_8315F700(ctx, base);
	// 8314F26C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8314F270: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8314F274: 409A0030  bne cr6, 0x8314f2a4
	if !ctx.cr[6].eq {
	pc = 0x8314F2A4; continue 'dispatch;
	}
	// 8314F278: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F27C: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8314F280: 388B4F84  addi r4, r11, 0x4f84
	ctx.r[4].s64 = ctx.r[11].s64 + 20356;
	// 8314F284: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F288: 480108B9  bl 0x8315fb40
	ctx.lr = 0x8314F28C;
	sub_8315FB40(ctx, base);
	// 8314F28C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314F290: 4BFFEDF1  bl 0x8314e080
	ctx.lr = 0x8314F294;
	sub_8314E080(ctx, base);
	// 8314F294: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F298: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8314F29C: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 8314F2A0: 48058F10  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8314F2A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8314F2A8: 80DD0008  lwz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314F2AC: 80BD0004  lwz r5, 4(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314F2B0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314F2B4: 48011D8D  bl 0x83161040
	ctx.lr = 0x8314F2B8;
	sub_83161040(ctx, base);
	// 8314F2B8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8314F2BC: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8314F2C0: 409A002C  bne cr6, 0x8314f2ec
	if !ctx.cr[6].eq {
	pc = 0x8314F2EC; continue 'dispatch;
	}
	// 8314F2C4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F2C8: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8314F2CC: 388B4F78  addi r4, r11, 0x4f78
	ctx.r[4].s64 = ctx.r[11].s64 + 20344;
	// 8314F2D0: 48010871  bl 0x8315fb40
	ctx.lr = 0x8314F2D4;
	sub_8315FB40(ctx, base);
	// 8314F2D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314F2D8: 4BFFEDA9  bl 0x8314e080
	ctx.lr = 0x8314F2DC;
	sub_8314E080(ctx, base);
	// 8314F2DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F2E0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8314F2E4: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 8314F2E8: 48058EC8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 8314F2EC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314F2F0: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8314F2F4: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 8314F2F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314F2FC: 409900A8  ble cr6, 0x8314f3a4
	if !ctx.cr[6].gt {
	pc = 0x8314F3A4; continue 'dispatch;
	}
	// 8314F300: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8314F304: C3EBCC2C  lfs f31, -0x33d4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-13268 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8314F308: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8314F30C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8314F310: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8314F314: 48010D95  bl 0x831600a8
	ctx.lr = 0x8314F318;
	sub_831600A8(ctx, base);
	// 8314F318: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8314F31C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8314F320: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8314F324: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8314F328: 48010D81  bl 0x831600a8
	ctx.lr = 0x8314F32C;
	sub_831600A8(ctx, base);
	// 8314F32C: 5749043E  clrlwi r9, r26, 0x10
	ctx.r[9].u64 = ctx.r[26].u32 as u64 & 0x0000FFFFu64;
	// 8314F330: 5468043E  clrlwi r8, r3, 0x10
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 8314F334: C01D0010  lfs f0, 0x10(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314F338: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 8314F33C: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8314F340: FD405E9C  fcfid f10, f11
	ctx.f[10].f64 = (ctx.f[11].s64 as f64);
	// 8314F344: F9010058  std r8, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u64 ) };
	// 8314F348: FD205018  frsp f9, f10
	ctx.f[9].f64 = (ctx.f[10].f64 as f32) as f64;
	// 8314F34C: C9010058  lfd f8, 0x58(r1)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 8314F350: C1BD000C  lfs f13, 0xc(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314F354: 80FF000C  lwz r7, 0xc(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314F358: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 8314F35C: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8314F360: ECE907F2  fmuls f7, f9, f31
	ctx.f[7].f64 = (((ctx.f[9].f64 * ctx.f[31].f64) as f32) as f64);
	// 8314F364: ECC7033A  fmadds f6, f7, f12, f0
	ctx.f[6].f64 = (((ctx.f[7].f64 * ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64);
	// 8314F368: 7CDC3D2E  stfsx f6, r28, r7
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[7].u32), tmp.u32) };
	// 8314F36C: C0BD0018  lfs f5, 0x18(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 8314F370: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314F374: C09D0014  lfs f4, 0x14(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 8314F378: 7CDC5A14  add r6, r28, r11
	ctx.r[6].u64 = ctx.r[28].u64 + ctx.r[11].u64;
	// 8314F37C: FC60469C  fcfid f3, f8
	ctx.f[3].f64 = (ctx.f[8].s64 as f64);
	// 8314F380: 3B9C0008  addi r28, r28, 8
	ctx.r[28].s64 = ctx.r[28].s64 + 8;
	// 8314F384: EC442828  fsubs f2, f4, f5
	ctx.f[2].f64 = (((ctx.f[4].f64 - ctx.f[5].f64) as f32) as f64);
	// 8314F388: FC201818  frsp f1, f3
	ctx.f[1].f64 = (ctx.f[3].f64 as f32) as f64;
	// 8314F38C: EC0107F2  fmuls f0, f1, f31
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[31].f64) as f32) as f64);
	// 8314F390: EDA2283A  fmadds f13, f2, f0, f5
	ctx.f[13].f64 = (((ctx.f[2].f64 * ctx.f[0].f64 + ctx.f[5].f64) as f32) as f64);
	// 8314F394: D1A60004  stfs f13, 4(r6)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 8314F398: 80BF0008  lwz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314F39C: 7F1E2840  cmplw cr6, r30, r5
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[5].u32, &mut ctx.xer);
	// 8314F3A0: 4198FF68  blt cr6, 0x8314f308
	if ctx.cr[6].lt {
	pc = 0x8314F308; continue 'dispatch;
	}
	// 8314F3A4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8314F3A8: 480115C1  bl 0x83160968
	ctx.lr = 0x8314F3AC;
	sub_83160968(ctx, base);
	// 8314F3AC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8314F3B0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8314F3B4: CBE1FFC0  lfd f31, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 8314F3B8: 48058DF8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314F3C0 size=176
    let mut pc: u32 = 0x8314F3C0;
    'dispatch: loop {
        match pc {
            0x8314F3C0 => {
    //   block [0x8314F3C0..0x8314F470)
	// 8314F3C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314F3C4: 48058DA1  bl 0x831a8164
	ctx.lr = 0x8314F3C8;
	sub_831A8130(ctx, base);
	// 8314F3C8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314F3CC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8314F3D0: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8314F3D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314F3D8: 419A0088  beq cr6, 0x8314f460
	if ctx.cr[6].eq {
	pc = 0x8314F460; continue 'dispatch;
	}
	// 8314F3DC: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314F3E0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8314F3E4: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 8314F3E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314F3EC: 40990058  ble cr6, 0x8314f444
	if !ctx.cr[6].gt {
	pc = 0x8314F444; continue 'dispatch;
	}
	// 8314F3F0: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 8314F3F4: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8314F3F8: 7D5E582E  lwzx r10, r30, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8314F3FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8314F400: 419A0030  beq cr6, 0x8314f430
	if ctx.cr[6].eq {
	pc = 0x8314F430; continue 'dispatch;
	}
	// 8314F404: 555F003E  slwi r31, r10, 0
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 8314F408: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314F40C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8314F410: 419A0010  beq cr6, 0x8314f420
	if ctx.cr[6].eq {
	pc = 0x8314F420; continue 'dispatch;
	}
	// 8314F414: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314F418: 48010351  bl 0x8315f768
	ctx.lr = 0x8314F41C;
	sub_8315F768(ctx, base);
	// 8314F41C: 937F000C  stw r27, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 8314F420: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 8314F424: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8314F428: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314F42C: 48010855  bl 0x8315fc80
	ctx.lr = 0x8314F430;
	sub_8315FC80(ctx, base);
	// 8314F430: 817D0014  lwz r11, 0x14(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314F434: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 8314F438: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8314F43C: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8314F440: 4198FFB4  blt cr6, 0x8314f3f4
	if ctx.cr[6].lt {
	pc = 0x8314F3F4; continue 'dispatch;
	}
	// 8314F444: 809D0018  lwz r4, 0x18(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8314F448: 807D0004  lwz r3, 4(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314F44C: 4801031D  bl 0x8315f768
	ctx.lr = 0x8314F450;
	sub_8315F768(ctx, base);
	// 8314F450: 937D0018  stw r27, 0x18(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(24 as u32), ctx.r[27].u32 ) };
	// 8314F454: 937D0014  stw r27, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[27].u32 ) };
	// 8314F458: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8314F45C: 48058D58  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8314F460: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8314F464: 917D0014  stw r11, 0x14(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8314F468: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8314F46C: 48058D48  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8314F470 size=232
    let mut pc: u32 = 0x8314F470;
    'dispatch: loop {
        match pc {
            0x8314F470 => {
    //   block [0x8314F470..0x8314F558)
	// 8314F470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314F474: 48058CF9  bl 0x831a816c
	ctx.lr = 0x8314F478;
	sub_831A8130(ctx, base);
	// 8314F478: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314F47C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8314F480: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F484: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8314F488: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8314F48C: 38CB4FC0  addi r6, r11, 0x4fc0
	ctx.r[6].s64 = ctx.r[11].s64 + 20416;
	// 8314F490: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8314F494: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8314F498: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 8314F49C: 4801077D  bl 0x8315fc18
	ctx.lr = 0x8314F4A0;
	sub_8315FC18(ctx, base);
	// 8314F4A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314F4A4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8314F4A8: 419A0084  beq cr6, 0x8314f52c
	if ctx.cr[6].eq {
	pc = 0x8314F52C; continue 'dispatch;
	}
	// 8314F4AC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F4B0: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 8314F4B4: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 8314F4B8: 392B4E50  addi r9, r11, 0x4e50
	ctx.r[9].s64 = ctx.r[11].s64 + 20048;
	// 8314F4BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8314F4C0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8314F4C4: 390A4E68  addi r8, r10, 0x4e68
	ctx.r[8].s64 = ctx.r[10].s64 + 20072;
	// 8314F4C8: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314F4CC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8314F4D0: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8314F4D4: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314F4D8: 90DF000C  stw r6, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 8314F4DC: C01E0014  lfs f0, 0x14(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314F4E0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8314F4E4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8314F4E8: 550B003E  slwi r11, r8, 0
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8314F4EC: D01F0010  stfs f0, 0x10(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 8314F4F0: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8314F4F4: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8314F4F8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314F4FC: 4E800421  bctrl
	ctx.lr = 0x8314F500;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314F500: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8314F504: 409A0048  bne cr6, 0x8314f54c
	if !ctx.cr[6].eq {
	pc = 0x8314F54C; continue 'dispatch;
	}
	// 8314F508: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314F50C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8314F510: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314F514: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314F518: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314F51C: 4E800421  bctrl
	ctx.lr = 0x8314F520;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314F520: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F524: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314F528: 48058C94  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8314F52C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F530: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8314F534: 388B4FB4  addi r4, r11, 0x4fb4
	ctx.r[4].s64 = ctx.r[11].s64 + 20404;
	// 8314F538: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F53C: 48010605  bl 0x8315fb40
	ctx.lr = 0x8314F540;
	sub_8315FB40(ctx, base);
	// 8314F540: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F544: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314F548: 48058C74  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8314F54C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314F550: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314F554: 48058C68  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314F558 size=76
    let mut pc: u32 = 0x8314F558;
    'dispatch: loop {
        match pc {
            0x8314F558 => {
    //   block [0x8314F558..0x8314F5A4)
	// 8314F558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314F55C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8314F560: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8314F564: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314F568: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314F56C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F570: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8314F574: 392B4E50  addi r9, r11, 0x4e50
	ctx.r[9].s64 = ctx.r[11].s64 + 20048;
	// 8314F578: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8314F57C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8314F580: 419A0010  beq cr6, 0x8314f590
	if ctx.cr[6].eq {
	pc = 0x8314F590; continue 'dispatch;
	}
	// 8314F584: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 8314F588: 480106F9  bl 0x8315fc80
	ctx.lr = 0x8314F58C;
	sub_8315FC80(ctx, base);
	// 8314F58C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314F590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8314F594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314F598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314F59C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314F5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314F5A8 size=192
    let mut pc: u32 = 0x8314F5A8;
    'dispatch: loop {
        match pc {
            0x8314F5A8 => {
    //   block [0x8314F5A8..0x8314F668)
	// 8314F5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314F5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8314F5B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8314F5B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8314F5B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314F5BC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F5C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8314F5C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8314F5C8: 38CB4FDC  addi r6, r11, 0x4fdc
	ctx.r[6].s64 = ctx.r[11].s64 + 20444;
	// 8314F5CC: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8314F5D0: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8314F5D4: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8314F5D8: 48010641  bl 0x8315fc18
	ctx.lr = 0x8314F5DC;
	sub_8315FC18(ctx, base);
	// 8314F5DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314F5E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8314F5E4: 419A004C  beq cr6, 0x8314f630
	if ctx.cr[6].eq {
	pc = 0x8314F630; continue 'dispatch;
	}
	// 8314F5E8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F5EC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8314F5F0: 394B4E80  addi r10, r11, 0x4e80
	ctx.r[10].s64 = ctx.r[11].s64 + 20096;
	// 8314F5F4: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8314F5F8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8314F5FC: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8314F600: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314F604: 4E800421  bctrl
	ctx.lr = 0x8314F608;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314F608: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8314F60C: 409A0040  bne cr6, 0x8314f64c
	if !ctx.cr[6].eq {
	pc = 0x8314F64C; continue 'dispatch;
	}
	// 8314F610: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314F614: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8314F618: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314F61C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314F620: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314F624: 4E800421  bctrl
	ctx.lr = 0x8314F628;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314F628: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F62C: 48000024  b 0x8314f650
	pc = 0x8314F650; continue 'dispatch;
	// 8314F630: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F634: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8314F638: 388B4FD0  addi r4, r11, 0x4fd0
	ctx.r[4].s64 = ctx.r[11].s64 + 20432;
	// 8314F63C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F640: 48010501  bl 0x8315fb40
	ctx.lr = 0x8314F644;
	sub_8315FB40(ctx, base);
	// 8314F644: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F648: 48000008  b 0x8314f650
	pc = 0x8314F650; continue 'dispatch;
	// 8314F64C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314F650: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314F654: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314F658: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314F65C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8314F660: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314F664: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314F668 size=76
    let mut pc: u32 = 0x8314F668;
    'dispatch: loop {
        match pc {
            0x8314F668 => {
    //   block [0x8314F668..0x8314F6B4)
	// 8314F668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314F66C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8314F670: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8314F674: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314F678: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314F67C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F680: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8314F684: 392B4E80  addi r9, r11, 0x4e80
	ctx.r[9].s64 = ctx.r[11].s64 + 20096;
	// 8314F688: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8314F68C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8314F690: 419A0010  beq cr6, 0x8314f6a0
	if ctx.cr[6].eq {
	pc = 0x8314F6A0; continue 'dispatch;
	}
	// 8314F694: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8314F698: 480105E9  bl 0x8315fc80
	ctx.lr = 0x8314F69C;
	sub_8315FC80(ctx, base);
	// 8314F69C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314F6A0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8314F6A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314F6A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314F6AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314F6B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F6B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314F6B8 size=192
    let mut pc: u32 = 0x8314F6B8;
    'dispatch: loop {
        match pc {
            0x8314F6B8 => {
    //   block [0x8314F6B8..0x8314F778)
	// 8314F6B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314F6BC: 48058AB1  bl 0x831a816c
	ctx.lr = 0x8314F6C0;
	sub_831A8130(ctx, base);
	// 8314F6C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314F6C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8314F6C8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F6CC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8314F6D0: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8314F6D4: 38CB4FF0  addi r6, r11, 0x4ff0
	ctx.r[6].s64 = ctx.r[11].s64 + 20464;
	// 8314F6D8: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8314F6DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8314F6E0: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8314F6E4: 48010535  bl 0x8315fc18
	ctx.lr = 0x8314F6E8;
	sub_8315FC18(ctx, base);
	// 8314F6E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314F6EC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8314F6F0: 419A005C  beq cr6, 0x8314f74c
	if ctx.cr[6].eq {
	pc = 0x8314F74C; continue 'dispatch;
	}
	// 8314F6F4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F6F8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8314F6FC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8314F700: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 8314F704: 392B4EA0  addi r9, r11, 0x4ea0
	ctx.r[9].s64 = ctx.r[11].s64 + 20128;
	// 8314F708: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8314F70C: 552B003E  slwi r11, r9, 0
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8314F710: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8314F714: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 8314F718: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314F71C: 4E800421  bctrl
	ctx.lr = 0x8314F720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314F720: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8314F724: 409A0048  bne cr6, 0x8314f76c
	if !ctx.cr[6].eq {
	pc = 0x8314F76C; continue 'dispatch;
	}
	// 8314F728: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314F72C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8314F730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314F734: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 8314F738: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314F73C: 4E800421  bctrl
	ctx.lr = 0x8314F740;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314F740: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F744: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314F748: 48058A74  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8314F74C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F750: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8314F754: 388B4FE4  addi r4, r11, 0x4fe4
	ctx.r[4].s64 = ctx.r[11].s64 + 20452;
	// 8314F758: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F75C: 480103E5  bl 0x8315fb40
	ctx.lr = 0x8314F760;
	sub_8315FB40(ctx, base);
	// 8314F760: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F764: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314F768: 48058A54  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8314F76C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314F770: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314F774: 48058A48  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F778(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314F778 size=76
    let mut pc: u32 = 0x8314F778;
    'dispatch: loop {
        match pc {
            0x8314F778 => {
    //   block [0x8314F778..0x8314F7C4)
	// 8314F778: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314F77C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8314F780: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8314F784: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314F788: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314F78C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F790: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8314F794: 392B4EA0  addi r9, r11, 0x4ea0
	ctx.r[9].s64 = ctx.r[11].s64 + 20128;
	// 8314F798: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8314F79C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8314F7A0: 419A0010  beq cr6, 0x8314f7b0
	if ctx.cr[6].eq {
	pc = 0x8314F7B0; continue 'dispatch;
	}
	// 8314F7A4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8314F7A8: 480104D9  bl 0x8315fc80
	ctx.lr = 0x8314F7AC;
	sub_8315FC80(ctx, base);
	// 8314F7AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314F7B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8314F7B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314F7B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314F7BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314F7C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F7C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314F7C8 size=188
    let mut pc: u32 = 0x8314F7C8;
    'dispatch: loop {
        match pc {
            0x8314F7C8 => {
    //   block [0x8314F7C8..0x8314F884)
	// 8314F7C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314F7CC: 480589A1  bl 0x831a816c
	ctx.lr = 0x8314F7D0;
	sub_831A8130(ctx, base);
	// 8314F7D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314F7D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314F7D8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8314F7DC: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314F7E0: 7F053040  cmplw cr6, r5, r6
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[6].u32, &mut ctx.xer);
	// 8314F7E4: 41980020  blt cr6, 0x8314f804
	if ctx.cr[6].lt {
	pc = 0x8314F804; continue 'dispatch;
	}
	// 8314F7E8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F7EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F7F0: 388B5068  addi r4, r11, 0x5068
	ctx.r[4].s64 = ctx.r[11].s64 + 20584;
	// 8314F7F4: 48010345  bl 0x8315fb38
	ctx.lr = 0x8314F7F8;
	sub_8315FB38(ctx, base);
	// 8314F7F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F7FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314F800: 480589BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8314F804: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314F808: 54BE103A  slwi r30, r5, 2
	ctx.r[30].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[30].u64 = ctx.r[30].u32 as u64;
	// 8314F80C: 7D4BF02E  lwzx r10, r11, r30
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8314F810: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8314F814: 419A0020  beq cr6, 0x8314f834
	if ctx.cr[6].eq {
	pc = 0x8314F834; continue 'dispatch;
	}
	// 8314F818: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F81C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F820: 388B502C  addi r4, r11, 0x502c
	ctx.r[4].s64 = ctx.r[11].s64 + 20524;
	// 8314F824: 48010305  bl 0x8315fb28
	ctx.lr = 0x8314F828;
	sub_8315FB28(ctx, base);
	// 8314F828: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F82C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314F830: 4805898C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8314F834: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8314F838: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314F83C: 4BFFFD6D  bl 0x8314f5a8
	ctx.lr = 0x8314F840;
	sub_8314F5A8(ctx, base);
	// 8314F840: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314F844: 7C6BF12E  stwx r3, r11, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32), ctx.r[3].u32) };
	// 8314F848: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314F84C: 7D2AF02E  lwzx r9, r10, r30
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 8314F850: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8314F854: 409A0024  bne cr6, 0x8314f878
	if !ctx.cr[6].eq {
	pc = 0x8314F878; continue 'dispatch;
	}
	// 8314F858: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F85C: 80BD000C  lwz r5, 0xc(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314F860: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F864: 388B5000  addi r4, r11, 0x5000
	ctx.r[4].s64 = ctx.r[11].s64 + 20480;
	// 8314F868: 480102C1  bl 0x8315fb28
	ctx.lr = 0x8314F86C;
	sub_8315FB28(ctx, base);
	// 8314F86C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F870: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314F874: 48058948  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8314F878: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8314F87C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314F880: 4805893C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8314F888 size=196
    let mut pc: u32 = 0x8314F888;
    'dispatch: loop {
        match pc {
            0x8314F888 => {
    //   block [0x8314F888..0x8314F94C)
	// 8314F888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314F88C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8314F890: DBE1FFF0  stfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[31].u64 ) };
	// 8314F894: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314F898: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F89C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8314F8A0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8314F8A4: 38CB50C0  addi r6, r11, 0x50c0
	ctx.r[6].s64 = ctx.r[11].s64 + 20672;
	// 8314F8A8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8314F8AC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8314F8B0: 38600030  li r3, 0x30
	ctx.r[3].s64 = 48;
	// 8314F8B4: 48010365  bl 0x8315fc18
	ctx.lr = 0x8314F8B8;
	sub_8315FC18(ctx, base);
	// 8314F8B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8314F8BC: 419A0064  beq cr6, 0x8314f920
	if ctx.cr[6].eq {
	pc = 0x8314F920; continue 'dispatch;
	}
	// 8314F8C0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 8314F8C4: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 8314F8C8: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 8314F8CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8314F8D0: 38E84F38  addi r7, r8, 0x4f38
	ctx.r[7].s64 = ctx.r[8].s64 + 20280;
	// 8314F8D4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8314F8D8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8314F8DC: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8314F8E0: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8314F8E4: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8314F8E8: C1A908A8  lfs f13, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 8314F8EC: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8314F8F0: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8314F8F4: D1A3001C  stfs f13, 0x1c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8314F8F8: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8314F8FC: D3E30020  stfs f31, 0x20(r3)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8314F900: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 8314F904: D3E30024  stfs f31, 0x24(r3)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 8314F908: 90C3002C  stw r6, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[6].u32 ) };
	// 8314F90C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8314F910: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314F914: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314F918: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314F91C: 4E800020  blr
	return;
	// 8314F920: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F924: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8314F928: 388B50B0  addi r4, r11, 0x50b0
	ctx.r[4].s64 = ctx.r[11].s64 + 20656;
	// 8314F92C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F930: 48010211  bl 0x8315fb40
	ctx.lr = 0x8314F934;
	sub_8315FB40(ctx, base);
	// 8314F934: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314F938: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8314F93C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314F940: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314F944: CBE1FFF0  lfd f31, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314F948: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314F950 size=76
    let mut pc: u32 = 0x8314F950;
    'dispatch: loop {
        match pc {
            0x8314F950 => {
    //   block [0x8314F950..0x8314F99C)
	// 8314F950: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314F954: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8314F958: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8314F95C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314F960: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314F964: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F968: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 8314F96C: 392B4F38  addi r9, r11, 0x4f38
	ctx.r[9].s64 = ctx.r[11].s64 + 20280;
	// 8314F970: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8314F974: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8314F978: 419A0010  beq cr6, 0x8314f988
	if ctx.cr[6].eq {
	pc = 0x8314F988; continue 'dispatch;
	}
	// 8314F97C: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 8314F980: 48010301  bl 0x8315fc80
	ctx.lr = 0x8314F984;
	sub_8315FC80(ctx, base);
	// 8314F984: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314F988: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8314F98C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8314F990: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8314F994: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8314F998: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314F9A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314F9A0 size=156
    let mut pc: u32 = 0x8314F9A0;
    'dispatch: loop {
        match pc {
            0x8314F9A0 => {
    //   block [0x8314F9A0..0x8314FA3C)
	// 8314F9A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314F9A4: 480587C9  bl 0x831a816c
	ctx.lr = 0x8314F9A8;
	sub_831A8130(ctx, base);
	// 8314F9A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314F9AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8314F9B0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314F9B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8314F9B8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8314F9BC: 38CB50DC  addi r6, r11, 0x50dc
	ctx.r[6].s64 = ctx.r[11].s64 + 20700;
	// 8314F9C0: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8314F9C4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8314F9C8: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8314F9CC: 4801024D  bl 0x8315fc18
	ctx.lr = 0x8314F9D0;
	sub_8315FC18(ctx, base);
	// 8314F9D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314F9D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8314F9D8: 419A0044  beq cr6, 0x8314fa1c
	if ctx.cr[6].eq {
	pc = 0x8314FA1C; continue 'dispatch;
	}
	// 8314F9DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8314F9E0: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8314F9E4: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314F9E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8314F9EC: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 8314F9F0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8314F9F4: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8314F9F8: 4BFFF7C9  bl 0x8314f1c0
	ctx.lr = 0x8314F9FC;
	sub_8314F1C0(ctx, base);
	// 8314F9FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8314FA00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FA04: 409A0010  bne cr6, 0x8314fa14
	if !ctx.cr[6].eq {
	pc = 0x8314FA14; continue 'dispatch;
	}
	// 8314FA08: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8314FA0C: 48010275  bl 0x8315fc80
	ctx.lr = 0x8314FA10;
	sub_8315FC80(ctx, base);
	// 8314FA10: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314FA14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314FA18: 480587A4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8314FA1C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314FA20: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8314FA24: 388B50D0  addi r4, r11, 0x50d0
	ctx.r[4].s64 = ctx.r[11].s64 + 20688;
	// 8314FA28: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314FA2C: 48010115  bl 0x8315fb40
	ctx.lr = 0x8314FA30;
	sub_8315FB40(ctx, base);
	// 8314FA30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314FA34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314FA38: 48058784  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314FA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8314FA40 size=648
    let mut pc: u32 = 0x8314FA40;
    'dispatch: loop {
        match pc {
            0x8314FA40 => {
    //   block [0x8314FA40..0x8314FCC8)
	// 8314FA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314FA44: 48058725  bl 0x831a8168
	ctx.lr = 0x8314FA48;
	sub_831A8130(ctx, base);
	// 8314FA48: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314FA4C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8314FA50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314FA54: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8314FA58: 80DD0010  lwz r6, 0x10(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8314FA5C: 80BD000C  lwz r5, 0xc(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314FA60: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314FA64: 480115DD  bl 0x83161040
	ctx.lr = 0x8314FA68;
	sub_83161040(ctx, base);
	// 8314FA68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8314FA6C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8314FA70: 409A0034  bne cr6, 0x8314faa4
	if !ctx.cr[6].eq {
	pc = 0x8314FAA4; continue 'dispatch;
	}
	// 8314FA74: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314FA78: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8314FA7C: 388B5128  addi r4, r11, 0x5128
	ctx.r[4].s64 = ctx.r[11].s64 + 20776;
	// 8314FA80: 480100C1  bl 0x8315fb40
	ctx.lr = 0x8314FA84;
	sub_8315FB40(ctx, base);
	// 8314FA84: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314FA88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FA8C: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314FA90: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8314FA94: 4E800421  bctrl
	ctx.lr = 0x8314FA98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314FA98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314FA9C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8314FAA0: 48058718  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8314FAA4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8314FAA8: 4BF04591  bl 0x83054038
	ctx.lr = 0x8314FAAC;
	sub_83054038(ctx, base);
	// 8314FAAC: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 8314FAB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8314FAB4: 48010EB5  bl 0x83160968
	ctx.lr = 0x8314FAB8;
	sub_83160968(ctx, base);
	// 8314FAB8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314FABC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314FAC0: 419A01C0  beq cr6, 0x8314fc80
	if ctx.cr[6].eq {
	pc = 0x8314FC80; continue 'dispatch;
	}
	// 8314FAC4: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 8314FAC8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314FACC: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 8314FAD0: 38AA5110  addi r5, r10, 0x5110
	ctx.r[5].s64 = ctx.r[10].s64 + 20752;
	// 8314FAD4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8314FAD8: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 8314FADC: 4800FC25  bl 0x8315f700
	ctx.lr = 0x8314FAE0;
	sub_8315F700(ctx, base);
	// 8314FAE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8314FAE4: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 8314FAE8: 409A0038  bne cr6, 0x8314fb20
	if !ctx.cr[6].eq {
	pc = 0x8314FB20; continue 'dispatch;
	}
	// 8314FAEC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314FAF0: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8314FAF4: 388B5104  addi r4, r11, 0x5104
	ctx.r[4].s64 = ctx.r[11].s64 + 20740;
	// 8314FAF8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314FAFC: 48010045  bl 0x8315fb40
	ctx.lr = 0x8314FB00;
	sub_8315FB40(ctx, base);
	// 8314FB00: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314FB04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FB08: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314FB0C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8314FB10: 4E800421  bctrl
	ctx.lr = 0x8314FB14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314FB14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314FB18: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8314FB1C: 4805869C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8314FB20: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314FB24: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8314FB28: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8314FB2C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314FB30: 40990024  ble cr6, 0x8314fb54
	if !ctx.cr[6].gt {
	pc = 0x8314FB54; continue 'dispatch;
	}
	// 8314FB34: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 8314FB38: 813F0018  lwz r9, 0x18(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8314FB3C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8314FB40: 7FCB492E  stwx r30, r11, r9
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32), ctx.r[30].u32) };
	// 8314FB44: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8314FB48: 811F0014  lwz r8, 0x14(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314FB4C: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 8314FB50: 4198FFE8  blt cr6, 0x8314fb38
	if ctx.cr[6].lt {
	pc = 0x8314FB38; continue 'dispatch;
	}
	// 8314FB54: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8314FB58: 80DD0010  lwz r6, 0x10(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8314FB5C: 80BD000C  lwz r5, 0xc(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8314FB60: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314FB64: 480114DD  bl 0x83161040
	ctx.lr = 0x8314FB68;
	sub_83161040(ctx, base);
	// 8314FB68: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8314FB6C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8314FB70: 409A0034  bne cr6, 0x8314fba4
	if !ctx.cr[6].eq {
	pc = 0x8314FBA4; continue 'dispatch;
	}
	// 8314FB74: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314FB78: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8314FB7C: 388B50F8  addi r4, r11, 0x50f8
	ctx.r[4].s64 = ctx.r[11].s64 + 20728;
	// 8314FB80: 4800FFC1  bl 0x8315fb40
	ctx.lr = 0x8314FB84;
	sub_8315FB40(ctx, base);
	// 8314FB84: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314FB88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FB8C: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314FB90: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8314FB94: 4E800421  bctrl
	ctx.lr = 0x8314FB98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314FB98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314FB9C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8314FBA0: 48058618  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8314FBA4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314FBA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314FBAC: 409900CC  ble cr6, 0x8314fc78
	if !ctx.cr[6].gt {
	pc = 0x8314FC78; continue 'dispatch;
	}
	// 8314FBB0: 7FDCF378  mr r28, r30
	ctx.r[28].u64 = ctx.r[30].u64;
	// 8314FBB4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8314FBB8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8314FBBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8314FBC0: 480103E1  bl 0x8315ffa0
	ctx.lr = 0x8314FBC4;
	sub_8315FFA0(ctx, base);
	// 8314FBC4: 546B063E  clrlwi r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	// 8314FBC8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8314FBCC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8314FBD0: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8314FBD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8314FBD8: 480107E1  bl 0x831603b8
	ctx.lr = 0x8314FBDC;
	sub_831603B8(ctx, base);
	// 8314FBDC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8314FBE0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8314FBE4: D021006C  stfs f1, 0x6c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 8314FBE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8314FBEC: 480107CD  bl 0x831603b8
	ctx.lr = 0x8314FBF0;
	sub_831603B8(ctx, base);
	// 8314FBF0: 38A00003  li r5, 3
	ctx.r[5].s64 = 3;
	// 8314FBF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8314FBF8: D0210070  stfs f1, 0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 8314FBFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8314FC00: 480107B9  bl 0x831603b8
	ctx.lr = 0x8314FC04;
	sub_831603B8(ctx, base);
	// 8314FC04: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8314FC08: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8314FC0C: D0210074  stfs f1, 0x74(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 8314FC10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8314FC14: 480107A5  bl 0x831603b8
	ctx.lr = 0x8314FC18;
	sub_831603B8(ctx, base);
	// 8314FC18: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8314FC1C: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8314FC20: D0210078  stfs f1, 0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 8314FC24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8314FC28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8314FC2C: 48010DAD  bl 0x831609d8
	ctx.lr = 0x8314FC30;
	sub_831609D8(ctx, base);
	// 8314FC30: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8314FC34: 8121005C  lwz r9, 0x5c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8314FC38: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 8314FC3C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314FC40: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 8314FC44: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 8314FC48: 4BFFFD59  bl 0x8314f9a0
	ctx.lr = 0x8314FC4C;
	sub_8314F9A0(ctx, base);
	// 8314FC4C: 811F0018  lwz r8, 0x18(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8314FC50: 7C7C412E  stwx r3, r28, r8
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[28].u32.wrapping_add(ctx.r[8].u32), ctx.r[3].u32) };
	// 8314FC54: 80FF0018  lwz r7, 0x18(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8314FC58: 7CDC382E  lwzx r6, r28, r7
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[7].u32)) } as u64;
	// 8314FC5C: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 8314FC60: 419A002C  beq cr6, 0x8314fc8c
	if ctx.cr[6].eq {
	pc = 0x8314FC8C; continue 'dispatch;
	}
	// 8314FC64: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314FC68: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8314FC6C: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 8314FC70: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8314FC74: 4198FF40  blt cr6, 0x8314fbb4
	if ctx.cr[6].lt {
	pc = 0x8314FBB4; continue 'dispatch;
	}
	// 8314FC78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8314FC7C: 48010CED  bl 0x83160968
	ctx.lr = 0x8314FC80;
	sub_83160968(ctx, base);
	// 8314FC80: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8314FC84: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8314FC88: 48058530  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8314FC8C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314FC90: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8314FC94: 388B50EC  addi r4, r11, 0x50ec
	ctx.r[4].s64 = ctx.r[11].s64 + 20716;
	// 8314FC98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314FC9C: 4800FEA5  bl 0x8315fb40
	ctx.lr = 0x8314FCA0;
	sub_8315FB40(ctx, base);
	// 8314FCA0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8314FCA4: 48010CC5  bl 0x83160968
	ctx.lr = 0x8314FCA8;
	sub_83160968(ctx, base);
	// 8314FCA8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314FCAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FCB0: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 8314FCB4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 8314FCB8: 4E800421  bctrl
	ctx.lr = 0x8314FCBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314FCBC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314FCC0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8314FCC4: 480584F4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314FCC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314FCC8 size=124
    let mut pc: u32 = 0x8314FCC8;
    'dispatch: loop {
        match pc {
            0x8314FCC8 => {
    //   block [0x8314FCC8..0x8314FD44)
	// 8314FCC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314FCCC: 480584A1  bl 0x831a816c
	ctx.lr = 0x8314FCD0;
	sub_831A8130(ctx, base);
	// 8314FCD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314FCD4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8314FCD8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314FCDC: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8314FCE0: 38AB5148  addi r5, r11, 0x5148
	ctx.r[5].s64 = ctx.r[11].s64 + 20808;
	// 8314FCE4: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 8314FCE8: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314FCEC: 4801000D  bl 0x8315fcf8
	ctx.lr = 0x8314FCF0;
	sub_8315FCF8(ctx, base);
	// 8314FCF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8314FCF4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8314FCF8: 419A002C  beq cr6, 0x8314fd24
	if ctx.cr[6].eq {
	pc = 0x8314FD24; continue 'dispatch;
	}
	// 8314FCFC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8314FD00: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8314FD04: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8314FD08: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8314FD0C: 480066B5  bl 0x831563c0
	ctx.lr = 0x8314FD10;
	sub_831563C0(ctx, base);
	// 8314FD10: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8314FD14: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 8314FD18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FD1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314FD20: 4805849C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8314FD24: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8314FD28: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8314FD2C: 388B513C  addi r4, r11, 0x513c
	ctx.r[4].s64 = ctx.r[11].s64 + 20796;
	// 8314FD30: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314FD34: 4800FE0D  bl 0x8315fb40
	ctx.lr = 0x8314FD38;
	sub_8315FB40(ctx, base);
	// 8314FD38: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8314FD3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8314FD40: 4805847C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314FD48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314FD48 size=148
    let mut pc: u32 = 0x8314FD48;
    'dispatch: loop {
        match pc {
            0x8314FD48 => {
    //   block [0x8314FD48..0x8314FDDC)
	// 8314FD48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314FD4C: 48058419  bl 0x831a8164
	ctx.lr = 0x8314FD50;
	sub_831A8130(ctx, base);
	// 8314FD50: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314FD54: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8314FD58: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314FD5C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8314FD60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314FD64: 83DD0388  lwz r30, 0x388(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(904 as u32) ) } as u64;
	// 8314FD68: 419A006C  beq cr6, 0x8314fdd4
	if ctx.cr[6].eq {
	pc = 0x8314FDD4; continue 'dispatch;
	}
	// 8314FD6C: 3BEBFFFC  addi r31, r11, -4
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	// 8314FD70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8314FD74: 419A0060  beq cr6, 0x8314fdd4
	if ctx.cr[6].eq {
	pc = 0x8314FDD4; continue 'dispatch;
	}
	// 8314FD78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8314FD7C: 4BFF023D  bl 0x8313ffb8
	ctx.lr = 0x8314FD80;
	sub_8313FFB8(ctx, base);
	// 8314FD80: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8314FD84: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8314FD88: 4BFF0229  bl 0x8313ffb0
	ctx.lr = 0x8314FD8C;
	sub_8313FFB0(ctx, base);
	// 8314FD8C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8314FD90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FD94: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8314FD98: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8314FD9C: 4BFFDBAD  bl 0x8314d948
	ctx.lr = 0x8314FDA0;
	sub_8314D948(ctx, base);
	// 8314FDA0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8314FDA4: 419A001C  beq cr6, 0x8314fdc0
	if ctx.cr[6].eq {
	pc = 0x8314FDC0; continue 'dispatch;
	}
	// 8314FDA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8314FDAC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8314FDB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FDB4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314FDB8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8314FDBC: 4E800421  bctrl
	ctx.lr = 0x8314FDC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8314FDC0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314FDC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314FDC8: 419A000C  beq cr6, 0x8314fdd4
	if ctx.cr[6].eq {
	pc = 0x8314FDD4; continue 'dispatch;
	}
	// 8314FDCC: 37EBFFFC  addic. r31, r11, -4
	ctx.xer.ca = (ctx.r[11].u32 > (!(-4 as u32)));
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8314FDD0: 4082FFA8  bne 0x8314fd78
	if !ctx.cr[0].eq {
	pc = 0x8314FD78; continue 'dispatch;
	}
	// 8314FDD4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8314FDD8: 480583DC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314FDE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314FDE0 size=144
    let mut pc: u32 = 0x8314FDE0;
    'dispatch: loop {
        match pc {
            0x8314FDE0 => {
    //   block [0x8314FDE0..0x8314FE70)
	// 8314FDE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314FDE4: 48058381  bl 0x831a8164
	ctx.lr = 0x8314FDE8;
	sub_831A8130(ctx, base);
	// 8314FDE8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314FDEC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314FDF0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8314FDF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314FDF8: 3BEBFFFC  addi r31, r11, -4
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	// 8314FDFC: 409A0008  bne cr6, 0x8314fe04
	if !ctx.cr[6].eq {
	pc = 0x8314FE04; continue 'dispatch;
	}
	// 8314FE00: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8314FE04: 83A40388  lwz r29, 0x388(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(904 as u32) ) } as u64;
	// 8314FE08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8314FE0C: 419A005C  beq cr6, 0x8314fe68
	if ctx.cr[6].eq {
	pc = 0x8314FE68; continue 'dispatch;
	}
	// 8314FE10: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314FE14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314FE18: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 8314FE1C: 409A0008  bne cr6, 0x8314fe24
	if !ctx.cr[6].eq {
	pc = 0x8314FE24; continue 'dispatch;
	}
	// 8314FE20: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8314FE24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8314FE28: 4BFF0191  bl 0x8313ffb8
	ctx.lr = 0x8314FE2C;
	sub_8313FFB8(ctx, base);
	// 8314FE2C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8314FE30: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8314FE34: 4BFF017D  bl 0x8313ffb0
	ctx.lr = 0x8314FE38;
	sub_8313FFB0(ctx, base);
	// 8314FE38: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8314FE3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FE40: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8314FE44: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8314FE48: 4BFFDB01  bl 0x8314d948
	ctx.lr = 0x8314FE4C;
	sub_8314D948(ctx, base);
	// 8314FE4C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8314FE50: 419A000C  beq cr6, 0x8314fe5c
	if ctx.cr[6].eq {
	pc = 0x8314FE5C; continue 'dispatch;
	}
	// 8314FE54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FE58: 4BFFD8A9  bl 0x8314d700
	ctx.lr = 0x8314FE5C;
	sub_8314D700(ctx, base);
	// 8314FE5C: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8314FE60: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8314FE64: 409AFFAC  bne cr6, 0x8314fe10
	if !ctx.cr[6].eq {
	pc = 0x8314FE10; continue 'dispatch;
	}
	// 8314FE68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8314FE6C: 48058348  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314FE70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314FE70 size=120
    let mut pc: u32 = 0x8314FE70;
    'dispatch: loop {
        match pc {
            0x8314FE70 => {
    //   block [0x8314FE70..0x8314FEE8)
	// 8314FE70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314FE74: 480582F5  bl 0x831a8168
	ctx.lr = 0x8314FE78;
	sub_831A8130(ctx, base);
	// 8314FE78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314FE7C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314FE80: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8314FE84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314FE88: 419A0058  beq cr6, 0x8314fee0
	if ctx.cr[6].eq {
	pc = 0x8314FEE0; continue 'dispatch;
	}
	// 8314FE8C: 3BEBFFFC  addi r31, r11, -4
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	// 8314FE90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8314FE94: 419A004C  beq cr6, 0x8314fee0
	if ctx.cr[6].eq {
	pc = 0x8314FEE0; continue 'dispatch;
	}
	// 8314FE98: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 8314FE9C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314FEA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314FEA4: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 8314FEA8: 409A0008  bne cr6, 0x8314feb0
	if !ctx.cr[6].eq {
	pc = 0x8314FEB0; continue 'dispatch;
	}
	// 8314FEAC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8314FEB0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8314FEB4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8314FEB8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8314FEBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FEC0: 4BFFDA89  bl 0x8314d948
	ctx.lr = 0x8314FEC4;
	sub_8314D948(ctx, base);
	// 8314FEC4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8314FEC8: 419A000C  beq cr6, 0x8314fed4
	if ctx.cr[6].eq {
	pc = 0x8314FED4; continue 'dispatch;
	}
	// 8314FECC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FED0: 4BFFD831  bl 0x8314d700
	ctx.lr = 0x8314FED4;
	sub_8314D700(ctx, base);
	// 8314FED4: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8314FED8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8314FEDC: 409AFFC0  bne cr6, 0x8314fe9c
	if !ctx.cr[6].eq {
	pc = 0x8314FE9C; continue 'dispatch;
	}
	// 8314FEE0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8314FEE4: 480582D4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314FEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314FEE8 size=144
    let mut pc: u32 = 0x8314FEE8;
    'dispatch: loop {
        match pc {
            0x8314FEE8 => {
    //   block [0x8314FEE8..0x8314FF78)
	// 8314FEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314FEEC: 48058279  bl 0x831a8164
	ctx.lr = 0x8314FEF0;
	sub_831A8130(ctx, base);
	// 8314FEF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314FEF4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314FEF8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8314FEFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314FF00: 3BEBFFFC  addi r31, r11, -4
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	// 8314FF04: 409A0008  bne cr6, 0x8314ff0c
	if !ctx.cr[6].eq {
	pc = 0x8314FF0C; continue 'dispatch;
	}
	// 8314FF08: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8314FF0C: 83A40388  lwz r29, 0x388(r4)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(904 as u32) ) } as u64;
	// 8314FF10: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8314FF14: 419A005C  beq cr6, 0x8314ff70
	if ctx.cr[6].eq {
	pc = 0x8314FF70; continue 'dispatch;
	}
	// 8314FF18: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314FF1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314FF20: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 8314FF24: 409A0008  bne cr6, 0x8314ff2c
	if !ctx.cr[6].eq {
	pc = 0x8314FF2C; continue 'dispatch;
	}
	// 8314FF28: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8314FF2C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8314FF30: 4BFF0089  bl 0x8313ffb8
	ctx.lr = 0x8314FF34;
	sub_8313FFB8(ctx, base);
	// 8314FF34: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8314FF38: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8314FF3C: 4BFF0075  bl 0x8313ffb0
	ctx.lr = 0x8314FF40;
	sub_8313FFB0(ctx, base);
	// 8314FF40: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8314FF44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FF48: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8314FF4C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8314FF50: 4BFFD9F9  bl 0x8314d948
	ctx.lr = 0x8314FF54;
	sub_8314D948(ctx, base);
	// 8314FF54: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8314FF58: 419A000C  beq cr6, 0x8314ff64
	if ctx.cr[6].eq {
	pc = 0x8314FF64; continue 'dispatch;
	}
	// 8314FF5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FF60: 4BFFD719  bl 0x8314d678
	ctx.lr = 0x8314FF64;
	sub_8314D678(ctx, base);
	// 8314FF64: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8314FF68: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8314FF6C: 409AFFAC  bne cr6, 0x8314ff18
	if !ctx.cr[6].eq {
	pc = 0x8314FF18; continue 'dispatch;
	}
	// 8314FF70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8314FF74: 48058240  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314FF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314FF78 size=120
    let mut pc: u32 = 0x8314FF78;
    'dispatch: loop {
        match pc {
            0x8314FF78 => {
    //   block [0x8314FF78..0x8314FFF0)
	// 8314FF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314FF7C: 480581ED  bl 0x831a8168
	ctx.lr = 0x8314FF80;
	sub_831A8130(ctx, base);
	// 8314FF80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314FF84: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8314FF88: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8314FF8C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314FF90: 419A0058  beq cr6, 0x8314ffe8
	if ctx.cr[6].eq {
	pc = 0x8314FFE8; continue 'dispatch;
	}
	// 8314FF94: 3BEBFFFC  addi r31, r11, -4
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	// 8314FF98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8314FF9C: 419A004C  beq cr6, 0x8314ffe8
	if ctx.cr[6].eq {
	pc = 0x8314FFE8; continue 'dispatch;
	}
	// 8314FFA0: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 8314FFA4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8314FFA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8314FFAC: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 8314FFB0: 409A0008  bne cr6, 0x8314ffb8
	if !ctx.cr[6].eq {
	pc = 0x8314FFB8; continue 'dispatch;
	}
	// 8314FFB4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8314FFB8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8314FFBC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8314FFC0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8314FFC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FFC8: 4BFFD981  bl 0x8314d948
	ctx.lr = 0x8314FFCC;
	sub_8314D948(ctx, base);
	// 8314FFCC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8314FFD0: 419A000C  beq cr6, 0x8314ffdc
	if ctx.cr[6].eq {
	pc = 0x8314FFDC; continue 'dispatch;
	}
	// 8314FFD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8314FFD8: 4BFFD6A1  bl 0x8314d678
	ctx.lr = 0x8314FFDC;
	sub_8314D678(ctx, base);
	// 8314FFDC: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8314FFE0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8314FFE4: 409AFFC0  bne cr6, 0x8314ffa4
	if !ctx.cr[6].eq {
	pc = 0x8314FFA4; continue 'dispatch;
	}
	// 8314FFE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8314FFEC: 480581CC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8314FFF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8314FFF0 size=184
    let mut pc: u32 = 0x8314FFF0;
    'dispatch: loop {
        match pc {
            0x8314FFF0 => {
    //   block [0x8314FFF0..0x831500A8)
	// 8314FFF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8314FFF4: 48058171  bl 0x831a8164
	ctx.lr = 0x8314FFF8;
	sub_831A8130(ctx, base);
	// 8314FFF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8314FFFC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83150000: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83150004: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83150008: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8315000C: 3BEBFFFC  addi r31, r11, -4
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	// 83150010: 409A0008  bne cr6, 0x83150018
	if !ctx.cr[6].eq {
	pc = 0x83150018; continue 'dispatch;
	}
	// 83150014: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83150018: 83C40388  lwz r30, 0x388(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(904 as u32) ) } as u64;
	// 8315001C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83150020: 419A0070  beq cr6, 0x83150090
	if ctx.cr[6].eq {
	pc = 0x83150090; continue 'dispatch;
	}
	// 83150024: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83150028: 4BFEFF91  bl 0x8313ffb8
	ctx.lr = 0x8315002C;
	sub_8313FFB8(ctx, base);
	// 8315002C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83150030: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83150034: 4BFEFF7D  bl 0x8313ffb0
	ctx.lr = 0x83150038;
	sub_8313FFB0(ctx, base);
	// 83150038: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 8315003C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83150040: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83150044: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83150048: 4BFFD901  bl 0x8314d948
	ctx.lr = 0x8315004C;
	sub_8314D948(ctx, base);
	// 8315004C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83150050: 419A002C  beq cr6, 0x8315007c
	if ctx.cr[6].eq {
	pc = 0x8315007C; continue 'dispatch;
	}
	// 83150054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83150058: 4BEB8DC9  bl 0x83008e20
	ctx.lr = 0x8315005C;
	sub_83008E20(ctx, base);
	// 8315005C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83150060: 419A003C  beq cr6, 0x8315009c
	if ctx.cr[6].eq {
	pc = 0x8315009C; continue 'dispatch;
	}
	// 83150064: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83150068: 419A0034  beq cr6, 0x8315009c
	if ctx.cr[6].eq {
	pc = 0x8315009C; continue 'dispatch;
	}
	// 8315006C: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83150070: 419A002C  beq cr6, 0x8315009c
	if ctx.cr[6].eq {
	pc = 0x8315009C; continue 'dispatch;
	}
	// 83150074: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 83150078: 419A0024  beq cr6, 0x8315009c
	if ctx.cr[6].eq {
	pc = 0x8315009C; continue 'dispatch;
	}
	// 8315007C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83150080: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83150084: 419A000C  beq cr6, 0x83150090
	if ctx.cr[6].eq {
	pc = 0x83150090; continue 'dispatch;
	}
	// 83150088: 37EBFFFC  addic. r31, r11, -4
	ctx.xer.ca = (ctx.r[11].u32 > (!(-4 as u32)));
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8315008C: 4082FF98  bne 0x83150024
	if !ctx.cr[0].eq {
	pc = 0x83150024; continue 'dispatch;
	}
	// 83150090: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83150094: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83150098: 4805811C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8315009C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831500A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831500A4: 48058110  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831500A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831500A8 size=132
    let mut pc: u32 = 0x831500A8;
    'dispatch: loop {
        match pc {
            0x831500A8 => {
    //   block [0x831500A8..0x8315012C)
	// 831500A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831500AC: 480580B5  bl 0x831a8160
	ctx.lr = 0x831500B0;
	sub_831A8130(ctx, base);
	// 831500B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831500B4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 831500B8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831500BC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 831500C0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 831500C4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 831500C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831500CC: 419A0058  beq cr6, 0x83150124
	if ctx.cr[6].eq {
	pc = 0x83150124; continue 'dispatch;
	}
	// 831500D0: 3BEBFFFC  addi r31, r11, -4
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	// 831500D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831500D8: 419A004C  beq cr6, 0x83150124
	if ctx.cr[6].eq {
	pc = 0x83150124; continue 'dispatch;
	}
	// 831500DC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 831500E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831500E4: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 831500E8: 409A0008  bne cr6, 0x831500f0
	if !ctx.cr[6].eq {
	pc = 0x831500F0; continue 'dispatch;
	}
	// 831500EC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831500F0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 831500F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831500F8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831500FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83150100: 4BFFD849  bl 0x8314d948
	ctx.lr = 0x83150104;
	sub_8314D948(ctx, base);
	// 83150104: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83150108: 419A0010  beq cr6, 0x83150118
	if ctx.cr[6].eq {
	pc = 0x83150118; continue 'dispatch;
	}
	// 8315010C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 83150110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83150114: 4BFFD675  bl 0x8314d788
	ctx.lr = 0x83150118;
	sub_8314D788(ctx, base);
	// 83150118: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 8315011C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83150120: 409AFFBC  bne cr6, 0x831500dc
	if !ctx.cr[6].eq {
	pc = 0x831500DC; continue 'dispatch;
	}
	// 83150124: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83150128: 48058088  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150130 size=352
    let mut pc: u32 = 0x83150130;
    'dispatch: loop {
        match pc {
            0x83150130 => {
    //   block [0x83150130..0x83150290)
	// 83150130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83150134: 4805802D  bl 0x831a8160
	ctx.lr = 0x83150138;
	sub_831A8130(ctx, base);
	// 83150138: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315013C: 83840094  lwz r28, 0x94(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(148 as u32) ) } as u64;
	// 83150140: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83150144: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83150148: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8315014C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83150150: 409A0010  bne cr6, 0x83150160
	if !ctx.cr[6].eq {
	pc = 0x83150160; continue 'dispatch;
	}
	// 83150154: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83150158: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8315015C: 48058054  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83150160: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 83150164: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83150168: 394A9BC9  addi r10, r10, -0x6437
	ctx.r[10].s64 = ctx.r[10].s64 + -25655;
	// 8315016C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150170: 890A0000  lbz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150174: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83150178: 7D284850  subf r9, r8, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[8].s64;
	// 8315017C: 419A0014  beq cr6, 0x83150190
	if ctx.cr[6].eq {
	pc = 0x83150190; continue 'dispatch;
	}
	// 83150180: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83150184: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83150188: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8315018C: 419AFFE0  beq cr6, 0x8315016c
	if ctx.cr[6].eq {
	pc = 0x8315016C; continue 'dispatch;
	}
	// 83150190: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83150194: 419AFFC0  beq cr6, 0x83150154
	if ctx.cr[6].eq {
	pc = 0x83150154; continue 'dispatch;
	}
	// 83150198: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8315019C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831501A0: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 831501A4: 409A0008  bne cr6, 0x831501ac
	if !ctx.cr[6].eq {
	pc = 0x831501AC; continue 'dispatch;
	}
	// 831501A8: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 831501AC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831501B0: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 831501B4: 419A00D0  beq cr6, 0x83150284
	if ctx.cr[6].eq {
	pc = 0x83150284; continue 'dispatch;
	}
	// 831501B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831501BC: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 831501C0: 4BD2E669  bl 0x82e7e828
	ctx.lr = 0x831501C4;
	sub_82E7E828(ctx, base);
	// 831501C4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831501C8: 895F0000  lbz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831501CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831501D0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831501D4: 419A0014  beq cr6, 0x831501e8
	if ctx.cr[6].eq {
	pc = 0x831501E8; continue 'dispatch;
	}
	// 831501D8: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 831501DC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 831501E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831501E4: 419AFFE0  beq cr6, 0x831501c4
	if ctx.cr[6].eq {
	pc = 0x831501C4; continue 'dispatch;
	}
	// 831501E8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831501EC: 409A0084  bne cr6, 0x83150270
	if !ctx.cr[6].eq {
	pc = 0x83150270; continue 'dispatch;
	}
	// 831501F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831501F4: 4BEB8C2D  bl 0x83008e20
	ctx.lr = 0x831501F8;
	sub_83008E20(ctx, base);
	// 831501F8: 817A0010  lwz r11, 0x10(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(16 as u32) ) } as u64;
	// 831501FC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83150200: 409A0024  bne cr6, 0x83150224
	if !ctx.cr[6].eq {
	pc = 0x83150224; continue 'dispatch;
	}
	// 83150204: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83150208: 419A0034  beq cr6, 0x8315023c
	if ctx.cr[6].eq {
	pc = 0x8315023C; continue 'dispatch;
	}
	// 8315020C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83150210: 419A002C  beq cr6, 0x8315023c
	if ctx.cr[6].eq {
	pc = 0x8315023C; continue 'dispatch;
	}
	// 83150214: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83150218: 419A0024  beq cr6, 0x8315023c
	if ctx.cr[6].eq {
	pc = 0x8315023C; continue 'dispatch;
	}
	// 8315021C: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 83150220: 48000018  b 0x83150238
	pc = 0x83150238; continue 'dispatch;
	// 83150224: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83150228: 419A0014  beq cr6, 0x8315023c
	if ctx.cr[6].eq {
	pc = 0x8315023C; continue 'dispatch;
	}
	// 8315022C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83150230: 419A000C  beq cr6, 0x8315023c
	if ctx.cr[6].eq {
	pc = 0x8315023C; continue 'dispatch;
	}
	// 83150234: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83150238: 409A0038  bne cr6, 0x83150270
	if !ctx.cr[6].eq {
	pc = 0x83150270; continue 'dispatch;
	}
	// 8315023C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83150240: 419A0020  beq cr6, 0x83150260
	if ctx.cr[6].eq {
	pc = 0x83150260; continue 'dispatch;
	}
	// 83150244: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83150248: 4BFFD6F9  bl 0x8314d940
	ctx.lr = 0x8315024C;
	sub_8314D940(ctx, base);
	// 8315024C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83150250: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83150254: 4BFFD6ED  bl 0x8314d940
	ctx.lr = 0x83150258;
	sub_8314D940(ctx, base);
	// 83150258: 7F1F1800  cmpw cr6, r31, r3
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[3].s32, &mut ctx.xer);
	// 8315025C: 40980008  bge cr6, 0x83150264
	if !ctx.cr[6].lt {
	pc = 0x83150264; continue 'dispatch;
	}
	// 83150260: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 83150264: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150268: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8315026C: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83150270: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83150274: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83150278: 419A000C  beq cr6, 0x83150284
	if ctx.cr[6].eq {
	pc = 0x83150284; continue 'dispatch;
	}
	// 8315027C: 37CBFFFC  addic. r30, r11, -4
	ctx.xer.ca = (ctx.r[11].u32 > (!(-4 as u32)));
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83150280: 4082FF38  bne 0x831501b8
	if !ctx.cr[0].eq {
	pc = 0x831501B8; continue 'dispatch;
	}
	// 83150284: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83150288: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8315028C: 48057F24  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150290 size=144
    let mut pc: u32 = 0x83150290;
    'dispatch: loop {
        match pc {
            0x83150290 => {
    //   block [0x83150290..0x83150320)
	// 83150290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83150294: 48057ECD  bl 0x831a8160
	ctx.lr = 0x83150298;
	sub_831A8130(ctx, base);
	// 83150298: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315029C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831502A0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 831502A4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 831502A8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831502AC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831502B0: 3BEBFFFC  addi r31, r11, -4
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	// 831502B4: 409A0008  bne cr6, 0x831502bc
	if !ctx.cr[6].eq {
	pc = 0x831502BC; continue 'dispatch;
	}
	// 831502B8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831502BC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 831502C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831502C4: 419A0044  beq cr6, 0x83150308
	if ctx.cr[6].eq {
	pc = 0x83150308; continue 'dispatch;
	}
	// 831502C8: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 831502CC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 831502D0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831502D4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831502D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831502DC: 4BFFD66D  bl 0x8314d948
	ctx.lr = 0x831502E0;
	sub_8314D948(ctx, base);
	// 831502E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831502E4: 419A0010  beq cr6, 0x831502f4
	if ctx.cr[6].eq {
	pc = 0x831502F4; continue 'dispatch;
	}
	// 831502E8: 7F1DD840  cmplw cr6, r29, r27
	ctx.cr[6].compare_u32(ctx.r[29].u32, ctx.r[27].u32, &mut ctx.xer);
	// 831502EC: 419A0028  beq cr6, 0x83150314
	if ctx.cr[6].eq {
	pc = 0x83150314; continue 'dispatch;
	}
	// 831502F0: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 831502F4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831502F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831502FC: 419A000C  beq cr6, 0x83150308
	if ctx.cr[6].eq {
	pc = 0x83150308; continue 'dispatch;
	}
	// 83150300: 37EBFFFC  addic. r31, r11, -4
	ctx.xer.ca = (ctx.r[11].u32 > (!(-4 as u32)));
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83150304: 4082FFC8  bne 0x831502cc
	if !ctx.cr[0].eq {
	pc = 0x831502CC; continue 'dispatch;
	}
	// 83150308: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8315030C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83150310: 48057EA0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83150314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83150318: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8315031C: 48057E94  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150320 size=120
    let mut pc: u32 = 0x83150320;
    'dispatch: loop {
        match pc {
            0x83150320 => {
    //   block [0x83150320..0x83150398)
	// 83150320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83150324: 48057E45  bl 0x831a8168
	ctx.lr = 0x83150328;
	sub_831A8130(ctx, base);
	// 83150328: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315032C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83150330: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83150334: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83150338: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8315033C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83150340: 419A0040  beq cr6, 0x83150380
	if ctx.cr[6].eq {
	pc = 0x83150380; continue 'dispatch;
	}
	// 83150344: 3BEBFFFC  addi r31, r11, -4
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	// 83150348: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8315034C: 419A0034  beq cr6, 0x83150380
	if ctx.cr[6].eq {
	pc = 0x83150380; continue 'dispatch;
	}
	// 83150350: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83150354: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83150358: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8315035C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83150360: 4BFFD5E9  bl 0x8314d948
	ctx.lr = 0x83150364;
	sub_8314D948(ctx, base);
	// 83150364: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83150368: 409A0024  bne cr6, 0x8315038c
	if !ctx.cr[6].eq {
	pc = 0x8315038C; continue 'dispatch;
	}
	// 8315036C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83150370: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83150374: 419A000C  beq cr6, 0x83150380
	if ctx.cr[6].eq {
	pc = 0x83150380; continue 'dispatch;
	}
	// 83150378: 37EBFFFC  addic. r31, r11, -4
	ctx.xer.ca = (ctx.r[11].u32 > (!(-4 as u32)));
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8315037C: 4082FFD4  bne 0x83150350
	if !ctx.cr[0].eq {
	pc = 0x83150350; continue 'dispatch;
	}
	// 83150380: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83150384: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83150388: 48057E30  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8315038C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83150390: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83150394: 48057E24  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83150398 size=68
    let mut pc: u32 = 0x83150398;
    'dispatch: loop {
        match pc {
            0x83150398 => {
    //   block [0x83150398..0x831503DC)
	// 83150398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315039C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831503A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831503A4: 4BFFFEED  bl 0x83150290
	ctx.lr = 0x831503A8;
	sub_83150290(ctx, base);
	// 831503A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831503AC: 419A0018  beq cr6, 0x831503c4
	if ctx.cr[6].eq {
	pc = 0x831503C4; continue 'dispatch;
	}
	// 831503B0: 4BFFD5E9  bl 0x8314d998
	ctx.lr = 0x831503B4;
	sub_8314D998(ctx, base);
	// 831503B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831503B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831503BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831503C0: 4E800020  blr
	return;
	// 831503C4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831503C8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 831503CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831503D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831503D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831503D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831503E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831503E0 size=100
    let mut pc: u32 = 0x831503E0;
    'dispatch: loop {
        match pc {
            0x831503E0 => {
    //   block [0x831503E0..0x83150444)
	// 831503E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831503E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831503E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831503EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831503F0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 831503F4: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 831503F8: 4BFFFF29  bl 0x83150320
	ctx.lr = 0x831503FC;
	sub_83150320(ctx, base);
	// 831503FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83150400: 419A0024  beq cr6, 0x83150424
	if ctx.cr[6].eq {
	pc = 0x83150424; continue 'dispatch;
	}
	// 83150404: 4BFFD59D  bl 0x8314d9a0
	ctx.lr = 0x83150408;
	sub_8314D9A0(ctx, base);
	// 83150408: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8315040C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83150410: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83150414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83150418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8315041C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83150420: 4E800020  blr
	return;
	// 83150424: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83150428: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8315042C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83150430: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83150434: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83150438: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8315043C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83150440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83150448 size=16
    let mut pc: u32 = 0x83150448;
    'dispatch: loop {
        match pc {
            0x83150448 => {
    //   block [0x83150448..0x83150458)
	// 83150448: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8315044C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83150450: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83150454: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83150458 size=20
    let mut pc: u32 = 0x83150458;
    'dispatch: loop {
        match pc {
            0x83150458 => {
    //   block [0x83150458..0x8315046C)
	// 83150458: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8315045C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 83150460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83150464: 409AFFF4  bne cr6, 0x83150458
	if !ctx.cr[6].eq {
	pc = 0x83150458; continue 'dispatch;
	}
	// 83150468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150470 size=232
    let mut pc: u32 = 0x83150470;
    'dispatch: loop {
        match pc {
            0x83150470 => {
    //   block [0x83150470..0x83150558)
	// 83150470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83150474: 48057CF5  bl 0x831a8168
	ctx.lr = 0x83150478;
	sub_831A8130(ctx, base);
	// 83150478: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315047C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83150480: 3BDC0004  addi r30, r28, 4
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	// 83150484: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 83150488: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8315048C: 419A00A4  beq cr6, 0x83150530
	if ctx.cr[6].eq {
	pc = 0x83150530; continue 'dispatch;
	}
	// 83150490: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 83150494: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83150498: 419A0098  beq cr6, 0x83150530
	if ctx.cr[6].eq {
	pc = 0x83150530; continue 'dispatch;
	}
	// 8315049C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 831504A0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831504A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831504A8: 3BEBFFFC  addi r31, r11, -4
	ctx.r[31].s64 = ctx.r[11].s64 + -4;
	// 831504AC: 409A0008  bne cr6, 0x831504b4
	if !ctx.cr[6].eq {
	pc = 0x831504B4; continue 'dispatch;
	}
	// 831504B0: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 831504B4: 35630004  addic. r11, r3, 4
	ctx.xer.ca = (ctx.r[3].u32 > (!(4 as u32)));
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831504B8: 4182008C  beq 0x83150544
	if ctx.cr[0].eq {
	pc = 0x83150544; continue 'dispatch;
	}
	// 831504BC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831504C0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831504C4: 409A000C  bne cr6, 0x831504d0
	if !ctx.cr[6].eq {
	pc = 0x831504D0; continue 'dispatch;
	}
	// 831504C8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831504CC: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831504D0: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831504D4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831504D8: 409A000C  bne cr6, 0x831504e4
	if !ctx.cr[6].eq {
	pc = 0x831504E4; continue 'dispatch;
	}
	// 831504DC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831504E0: 915E0004  stw r10, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 831504E4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831504E8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831504EC: 419A000C  beq cr6, 0x831504f8
	if ctx.cr[6].eq {
	pc = 0x831504F8; continue 'dispatch;
	}
	// 831504F0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831504F4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 831504F8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831504FC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83150500: 419A000C  beq cr6, 0x8315050c
	if ctx.cr[6].eq {
	pc = 0x8315050C; continue 'dispatch;
	}
	// 83150504: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150508: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8315050C: 93AB0004  stw r29, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 83150510: 93AB0000  stw r29, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83150514: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150518: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8315051C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150520: 4E800421  bctrl
	ctx.lr = 0x83150524;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83150524: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83150528: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8315052C: 409AFF74  bne cr6, 0x831504a0
	if !ctx.cr[6].eq {
	pc = 0x831504A0; continue 'dispatch;
	}
	// 83150530: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 83150534: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83150538: 4800F749  bl 0x8315fc80
	ctx.lr = 0x8315053C;
	sub_8315FC80(ctx, base);
	// 8315053C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83150540: 48057C78  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83150544: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83150548: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8315054C: 388B37A8  addi r4, r11, 0x37a8
	ctx.r[4].s64 = ctx.r[11].s64 + 14248;
	// 83150550: 4800F5C9  bl 0x8315fb18
	ctx.lr = 0x83150554;
	sub_8315FB18(ctx, base);
	// 83150554: 48000000  b 0x83150554
	pc = 0x83150554; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150558 size=288
    let mut pc: u32 = 0x83150558;
    'dispatch: loop {
        match pc {
            0x83150558 => {
    //   block [0x83150558..0x83150678)
	// 83150558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315055C: 48057C0D  bl 0x831a8168
	ctx.lr = 0x83150560;
	sub_831A8130(ctx, base);
	// 83150560: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83150564: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83150568: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8315056C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83150570: 4BFFFBC1  bl 0x83150130
	ctx.lr = 0x83150574;
	sub_83150130(ctx, base);
	// 83150574: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83150578: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8315057C: 419A00D0  beq cr6, 0x8315064c
	if ctx.cr[6].eq {
	pc = 0x8315064C; continue 'dispatch;
	}
	// 83150580: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83150584: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83150588: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8315058C: 419800C0  blt cr6, 0x8315064c
	if ctx.cr[6].lt {
	pc = 0x8315064C; continue 'dispatch;
	}
	// 83150590: 83DF0098  lwz r30, 0x98(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 83150594: 4BFFD3AD  bl 0x8314d940
	ctx.lr = 0x83150598;
	sub_8314D940(ctx, base);
	// 83150598: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8315059C: 4099000C  ble cr6, 0x831505a8
	if !ctx.cr[6].gt {
	pc = 0x831505A8; continue 'dispatch;
	}
	// 831505A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831505A4: 4800002C  b 0x831505d0
	pc = 0x831505D0; continue 'dispatch;
	// 831505A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831505AC: 83DF0098  lwz r30, 0x98(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 831505B0: 4BFFD391  bl 0x8314d940
	ctx.lr = 0x831505B4;
	sub_8314D940(ctx, base);
	// 831505B4: 7F03F000  cmpw cr6, r3, r30
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[30].s32, &mut ctx.xer);
	// 831505B8: 409A0014  bne cr6, 0x831505cc
	if !ctx.cr[6].eq {
	pc = 0x831505CC; continue 'dispatch;
	}
	// 831505BC: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 831505C0: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 831505C4: 555EDFFE  rlwinm r30, r10, 0x1b, 0x1f, 0x1f
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 831505C8: 48000008  b 0x831505d0
	pc = 0x831505D0; continue 'dispatch;
	// 831505CC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 831505D0: 817C000C  lwz r11, 0xc(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 831505D4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831505D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831505DC: 419A003C  beq cr6, 0x83150618
	if ctx.cr[6].eq {
	pc = 0x83150618; continue 'dispatch;
	}
	// 831505E0: 817F0380  lwz r11, 0x380(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(896 as u32) ) } as u64;
	// 831505E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831505E8: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 831505EC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 831505F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 831505F4: 409A0014  bne cr6, 0x83150608
	if !ctx.cr[6].eq {
	pc = 0x83150608; continue 'dispatch;
	}
	// 831505F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831505FC: 4BF02B4D  bl 0x83053148
	ctx.lr = 0x83150600;
	sub_83053148(ctx, base);
	// 83150600: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 83150604: 48000008  b 0x8315060c
	pc = 0x8315060C; continue 'dispatch;
	// 83150608: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 8315060C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83150610: 807C000C  lwz r3, 0xc(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 83150614: 48005CFD  bl 0x83156310
	ctx.lr = 0x83150618;
	sub_83156310(ctx, base);
	// 83150618: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 8315061C: 409A0050  bne cr6, 0x8315066c
	if !ctx.cr[6].eq {
	pc = 0x8315066C; continue 'dispatch;
	}
	// 83150620: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 83150624: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83150628: 409A0030  bne cr6, 0x83150658
	if !ctx.cr[6].eq {
	pc = 0x83150658; continue 'dispatch;
	}
	// 8315062C: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 83150630: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 83150634: 4BFEFB55  bl 0x83140188
	ctx.lr = 0x83150638;
	sub_83140188(ctx, base);
	// 83150638: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8315063C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83150640: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150644: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150648: 4E800421  bctrl
	ctx.lr = 0x8315064C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8315064C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83150650: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83150654: 48057B64  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83150658: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8315065C: 4BFFD0A5  bl 0x8314d700
	ctx.lr = 0x83150660;
	sub_8314D700(ctx, base);
	// 83150660: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83150664: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83150668: 48057B50  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8315066C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83150670: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83150674: 48057B44  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150678 size=144
    let mut pc: u32 = 0x83150678;
    'dispatch: loop {
        match pc {
            0x83150678 => {
    //   block [0x83150678..0x83150708)
	// 83150678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315067C: 48057AE9  bl 0x831a8164
	ctx.lr = 0x83150680;
	sub_831A8130(ctx, base);
	// 83150680: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83150684: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83150688: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8315068C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83150690: 83BF0388  lwz r29, 0x388(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(904 as u32) ) } as u64;
	// 83150694: 4BFFFEC5  bl 0x83150558
	ctx.lr = 0x83150698;
	sub_83150558(ctx, base);
	// 83150698: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8315069C: 419A0060  beq cr6, 0x831506fc
	if ctx.cr[6].eq {
	pc = 0x831506FC; continue 'dispatch;
	}
	// 831506A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831506A4: 4BFEF915  bl 0x8313ffb8
	ctx.lr = 0x831506A8;
	sub_8313FFB8(ctx, base);
	// 831506A8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 831506AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831506B0: 4BFEF901  bl 0x8313ffb0
	ctx.lr = 0x831506B4;
	sub_8313FFB0(ctx, base);
	// 831506B4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831506B8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 831506BC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831506C0: 388B5158  addi r4, r11, 0x5158
	ctx.r[4].s64 = ctx.r[11].s64 + 20824;
	// 831506C4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831506C8: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 831506CC: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 831506D0: 4BFFD429  bl 0x8314daf8
	ctx.lr = 0x831506D4;
	sub_8314DAF8(ctx, base);
	// 831506D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831506D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831506DC: 419A0020  beq cr6, 0x831506fc
	if ctx.cr[6].eq {
	pc = 0x831506FC; continue 'dispatch;
	}
	// 831506E0: 4BFFCF11  bl 0x8314d5f0
	ctx.lr = 0x831506E4;
	sub_8314D5F0(ctx, base);
	// 831506E4: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 831506E8: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 831506EC: 4BFEF9FD  bl 0x831400e8
	ctx.lr = 0x831506F0;
	sub_831400E8(ctx, base);
	// 831506F0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831506F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831506F8: 48057ABC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831506FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83150700: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83150704: 48057AB0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150708 size=284
    let mut pc: u32 = 0x83150708;
    'dispatch: loop {
        match pc {
            0x83150708 => {
    //   block [0x83150708..0x83150824)
	// 83150708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315070C: 48057A59  bl 0x831a8164
	ctx.lr = 0x83150710;
	sub_831A8130(ctx, base);
	// 83150710: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83150714: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83150718: 480192C1  bl 0x831699d8
	ctx.lr = 0x8315071C;
	sub_831699D8(ctx, base);
	// 8315071C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83150720: 3BBF0004  addi r29, r31, 4
	ctx.r[29].s64 = ctx.r[31].s64 + 4;
	// 83150724: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83150728: 419A00E0  beq cr6, 0x83150808
	if ctx.cr[6].eq {
	pc = 0x83150808; continue 'dispatch;
	}
	// 8315072C: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 83150730: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83150734: 419A00D4  beq cr6, 0x83150808
	if ctx.cr[6].eq {
	pc = 0x83150808; continue 'dispatch;
	}
	// 83150738: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8315073C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83150740: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83150744: 3B8BFFFC  addi r28, r11, -4
	ctx.r[28].s64 = ctx.r[11].s64 + -4;
	// 83150748: 409A0008  bne cr6, 0x83150750
	if !ctx.cr[6].eq {
	pc = 0x83150750; continue 'dispatch;
	}
	// 8315074C: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 83150750: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83150754: 4BFFD0AD  bl 0x8314d800
	ctx.lr = 0x83150758;
	sub_8314D800(ctx, base);
	// 83150758: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8315075C: 4BEB86C5  bl 0x83008e20
	ctx.lr = 0x83150760;
	sub_83008E20(ctx, base);
	// 83150760: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83150764: 419A000C  beq cr6, 0x83150770
	if ctx.cr[6].eq {
	pc = 0x83150770; continue 'dispatch;
	}
	// 83150768: 2F030005  cmpwi cr6, r3, 5
	ctx.cr[6].compare_i32(ctx.r[3].s32, 5, &mut ctx.xer);
	// 8315076C: 409A0090  bne cr6, 0x831507fc
	if !ctx.cr[6].eq {
	pc = 0x831507FC; continue 'dispatch;
	}
	// 83150770: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 83150774: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83150778: 419A0098  beq cr6, 0x83150810
	if ctx.cr[6].eq {
	pc = 0x83150810; continue 'dispatch;
	}
	// 8315077C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150780: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83150784: 409A000C  bne cr6, 0x83150790
	if !ctx.cr[6].eq {
	pc = 0x83150790; continue 'dispatch;
	}
	// 83150788: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8315078C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83150790: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 83150794: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83150798: 409A000C  bne cr6, 0x831507a4
	if !ctx.cr[6].eq {
	pc = 0x831507A4; continue 'dispatch;
	}
	// 8315079C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831507A0: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831507A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831507A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831507AC: 419A000C  beq cr6, 0x831507b8
	if ctx.cr[6].eq {
	pc = 0x831507B8; continue 'dispatch;
	}
	// 831507B0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831507B4: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 831507B8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831507BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831507C0: 419A000C  beq cr6, 0x831507cc
	if ctx.cr[6].eq {
	pc = 0x831507CC; continue 'dispatch;
	}
	// 831507C4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831507C8: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831507CC: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 831507D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831507D4: 937F0000  stw r27, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[27].u32 ) };
	// 831507D8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831507DC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831507E0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831507E4: 4E800421  bctrl
	ctx.lr = 0x831507E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831507E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831507EC: 409A0010  bne cr6, 0x831507fc
	if !ctx.cr[6].eq {
	pc = 0x831507FC; continue 'dispatch;
	}
	// 831507F0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831507F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831507F8: 4BFFAC69  bl 0x8314b460
	ctx.lr = 0x831507FC;
	sub_8314B460(ctx, base);
	// 831507FC: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 83150800: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83150804: 409AFF38  bne cr6, 0x8315073c
	if !ctx.cr[6].eq {
	pc = 0x8315073C; continue 'dispatch;
	}
	// 83150808: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8315080C: 480579A8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83150810: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83150814: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83150818: 388B37A8  addi r4, r11, 0x37a8
	ctx.r[4].s64 = ctx.r[11].s64 + 14248;
	// 8315081C: 4800F2FD  bl 0x8315fb18
	ctx.lr = 0x83150820;
	sub_8315FB18(ctx, base);
	// 83150820: 48000000  b 0x83150820
	pc = 0x83150820; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150828(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150828 size=276
    let mut pc: u32 = 0x83150828;
    'dispatch: loop {
        match pc {
            0x83150828 => {
    //   block [0x83150828..0x8315093C)
	// 83150828: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315082C: 48057935  bl 0x831a8160
	ctx.lr = 0x83150830;
	sub_831A8130(ctx, base);
	// 83150830: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83150834: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83150838: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8315083C: 3BC30004  addi r30, r3, 4
	ctx.r[30].s64 = ctx.r[3].s64 + 4;
	// 83150840: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83150844: 419A00DC  beq cr6, 0x83150920
	if ctx.cr[6].eq {
	pc = 0x83150920; continue 'dispatch;
	}
	// 83150848: 3BABFFFC  addi r29, r11, -4
	ctx.r[29].s64 = ctx.r[11].s64 + -4;
	// 8315084C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83150850: 419A00D0  beq cr6, 0x83150920
	if ctx.cr[6].eq {
	pc = 0x83150920; continue 'dispatch;
	}
	// 83150854: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83150858: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8315085C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83150860: 3B6BFFFC  addi r27, r11, -4
	ctx.r[27].s64 = ctx.r[11].s64 + -4;
	// 83150864: 409A0008  bne cr6, 0x8315086c
	if !ctx.cr[6].eq {
	pc = 0x8315086C; continue 'dispatch;
	}
	// 83150868: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 8315086C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83150870: 4BEB8C69  bl 0x830094d8
	ctx.lr = 0x83150874;
	sub_830094D8(ctx, base);
	// 83150874: 7F03D040  cmplw cr6, r3, r26
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[26].u32, &mut ctx.xer);
	// 83150878: 409A009C  bne cr6, 0x83150914
	if !ctx.cr[6].eq {
	pc = 0x83150914; continue 'dispatch;
	}
	// 8315087C: 3BFD0004  addi r31, r29, 4
	ctx.r[31].s64 = ctx.r[29].s64 + 4;
	// 83150880: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83150884: 419A00A4  beq cr6, 0x83150928
	if ctx.cr[6].eq {
	pc = 0x83150928; continue 'dispatch;
	}
	// 83150888: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8315088C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83150890: 409A000C  bne cr6, 0x8315089c
	if !ctx.cr[6].eq {
	pc = 0x8315089C; continue 'dispatch;
	}
	// 83150894: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83150898: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8315089C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831508A0: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831508A4: 409A000C  bne cr6, 0x831508b0
	if !ctx.cr[6].eq {
	pc = 0x831508B0; continue 'dispatch;
	}
	// 831508A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831508AC: 917E0004  stw r11, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831508B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831508B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831508B8: 419A000C  beq cr6, 0x831508c4
	if ctx.cr[6].eq {
	pc = 0x831508C4; continue 'dispatch;
	}
	// 831508BC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831508C0: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 831508C4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831508C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831508CC: 419A000C  beq cr6, 0x831508d8
	if ctx.cr[6].eq {
	pc = 0x831508D8; continue 'dispatch;
	}
	// 831508D0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831508D4: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831508D8: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 831508DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831508E0: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 831508E4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831508E8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831508EC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831508F0: 4E800421  bctrl
	ctx.lr = 0x831508F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831508F4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831508F8: 409A0014  bne cr6, 0x8315090c
	if !ctx.cr[6].eq {
	pc = 0x8315090C; continue 'dispatch;
	}
	// 831508FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83150900: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83150904: 4BFFAB5D  bl 0x8314b460
	ctx.lr = 0x83150908;
	sub_8314B460(ctx, base);
	// 83150908: 4BFFFF78  b 0x83150880
	pc = 0x83150880; continue 'dispatch;
	// 8315090C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83150910: 409AFF70  bne cr6, 0x83150880
	if !ctx.cr[6].eq {
	pc = 0x83150880; continue 'dispatch;
	}
	// 83150914: 7F7DDB78  mr r29, r27
	ctx.r[29].u64 = ctx.r[27].u64;
	// 83150918: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 8315091C: 409AFF3C  bne cr6, 0x83150858
	if !ctx.cr[6].eq {
	pc = 0x83150858; continue 'dispatch;
	}
	// 83150920: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83150924: 4805788C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83150928: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8315092C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83150930: 388B37A8  addi r4, r11, 0x37a8
	ctx.r[4].s64 = ctx.r[11].s64 + 14248;
	// 83150934: 4800F1E5  bl 0x8315fb18
	ctx.lr = 0x83150938;
	sub_8315FB18(ctx, base);
	// 83150938: 48000000  b 0x83150938
	pc = 0x83150938; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150940 size=292
    let mut pc: u32 = 0x83150940;
    'dispatch: loop {
        match pc {
            0x83150940 => {
    //   block [0x83150940..0x83150A64)
	// 83150940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83150944: 48057819  bl 0x831a815c
	ctx.lr = 0x83150948;
	sub_831A8130(ctx, base);
	// 83150948: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315094C: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83150950: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 83150954: 3BA30004  addi r29, r3, 4
	ctx.r[29].s64 = ctx.r[3].s64 + 4;
	// 83150958: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8315095C: 419A00EC  beq cr6, 0x83150a48
	if ctx.cr[6].eq {
	pc = 0x83150A48; continue 'dispatch;
	}
	// 83150960: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 83150964: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83150968: 419A00E0  beq cr6, 0x83150a48
	if ctx.cr[6].eq {
	pc = 0x83150A48; continue 'dispatch;
	}
	// 8315096C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83150970: 3B40FFFF  li r26, -1
	ctx.r[26].s64 = -1;
	// 83150974: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83150978: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8315097C: 3B6BFFFC  addi r27, r11, -4
	ctx.r[27].s64 = ctx.r[11].s64 + -4;
	// 83150980: 409A0008  bne cr6, 0x83150988
	if !ctx.cr[6].eq {
	pc = 0x83150988; continue 'dispatch;
	}
	// 83150984: 7F9BE378  mr r27, r28
	ctx.r[27].u64 = ctx.r[28].u64;
	// 83150988: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8315098C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83150990: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83150994: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83150998: 4BFFCFB1  bl 0x8314d948
	ctx.lr = 0x8315099C;
	sub_8314D948(ctx, base);
	// 8315099C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831509A0: 419A009C  beq cr6, 0x83150a3c
	if ctx.cr[6].eq {
	pc = 0x83150A3C; continue 'dispatch;
	}
	// 831509A4: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 831509A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831509AC: 419A00A4  beq cr6, 0x83150a50
	if ctx.cr[6].eq {
	pc = 0x83150A50; continue 'dispatch;
	}
	// 831509B0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 831509B4: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831509B8: 409A000C  bne cr6, 0x831509c4
	if !ctx.cr[6].eq {
	pc = 0x831509C4; continue 'dispatch;
	}
	// 831509BC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831509C0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831509C4: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 831509C8: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 831509CC: 409A000C  bne cr6, 0x831509d8
	if !ctx.cr[6].eq {
	pc = 0x831509D8; continue 'dispatch;
	}
	// 831509D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831509D4: 917D0004  stw r11, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831509D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831509DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831509E0: 419A000C  beq cr6, 0x831509ec
	if ctx.cr[6].eq {
	pc = 0x831509EC; continue 'dispatch;
	}
	// 831509E4: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831509E8: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 831509EC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831509F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831509F4: 419A000C  beq cr6, 0x83150a00
	if ctx.cr[6].eq {
	pc = 0x83150A00; continue 'dispatch;
	}
	// 831509F8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831509FC: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83150A00: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 83150A04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83150A08: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 83150A0C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150A10: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83150A14: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150A18: 4E800421  bctrl
	ctx.lr = 0x83150A1C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83150A1C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83150A20: 409A0014  bne cr6, 0x83150a34
	if !ctx.cr[6].eq {
	pc = 0x83150A34; continue 'dispatch;
	}
	// 83150A24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83150A28: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83150A2C: 4BFFAA35  bl 0x8314b460
	ctx.lr = 0x83150A30;
	sub_8314B460(ctx, base);
	// 83150A30: 4BFFFF78  b 0x831509a8
	pc = 0x831509A8; continue 'dispatch;
	// 83150A34: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83150A38: 409AFF70  bne cr6, 0x831509a8
	if !ctx.cr[6].eq {
	pc = 0x831509A8; continue 'dispatch;
	}
	// 83150A3C: 7F7EDB78  mr r30, r27
	ctx.r[30].u64 = ctx.r[27].u64;
	// 83150A40: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83150A44: 409AFF30  bne cr6, 0x83150974
	if !ctx.cr[6].eq {
	pc = 0x83150974; continue 'dispatch;
	}
	// 83150A48: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83150A4C: 48057760  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 83150A50: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83150A54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83150A58: 388B37A8  addi r4, r11, 0x37a8
	ctx.r[4].s64 = ctx.r[11].s64 + 14248;
	// 83150A5C: 4800F0BD  bl 0x8315fb18
	ctx.lr = 0x83150A60;
	sub_8315FB18(ctx, base);
	// 83150A60: 48000000  b 0x83150a60
	pc = 0x83150A60; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150A68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150A68 size=132
    let mut pc: u32 = 0x83150A68;
    'dispatch: loop {
        match pc {
            0x83150A68 => {
    //   block [0x83150A68..0x83150AEC)
	// 83150A68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83150A6C: 48057701  bl 0x831a816c
	ctx.lr = 0x83150A70;
	sub_831A8130(ctx, base);
	// 83150A70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83150A74: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83150A78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83150A7C: 816B7F48  lwz r11, 0x7f48(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32584 as u32) ) } as u64;
	// 83150A80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83150A84: 419A0060  beq cr6, 0x83150ae4
	if ctx.cr[6].eq {
	pc = 0x83150AE4; continue 'dispatch;
	}
	// 83150A88: 3BEBFFF4  addi r31, r11, -0xc
	ctx.r[31].s64 = ctx.r[11].s64 + -12;
	// 83150A8C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83150A90: 419A0054  beq cr6, 0x83150ae4
	if ctx.cr[6].eq {
	pc = 0x83150AE4; continue 'dispatch;
	}
	// 83150A94: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83150A98: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83150A9C: 7F03E840  cmplw cr6, r3, r29
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83150AA0: 409A0030  bne cr6, 0x83150ad0
	if !ctx.cr[6].eq {
	pc = 0x83150AD0; continue 'dispatch;
	}
	// 83150AA4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150AA8: 814B0094  lwz r10, 0x94(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) } as u64;
	// 83150AAC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150AB0: 4E800421  bctrl
	ctx.lr = 0x83150AB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83150AB4: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83150AB8: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150ABC: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150AC0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83150AC4: 4E800421  bctrl
	ctx.lr = 0x83150AC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83150AC8: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 83150ACC: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 83150AD0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83150AD4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83150AD8: 419A000C  beq cr6, 0x83150ae4
	if ctx.cr[6].eq {
	pc = 0x83150AE4; continue 'dispatch;
	}
	// 83150ADC: 37EBFFF4  addic. r31, r11, -0xc
	ctx.xer.ca = (ctx.r[11].u32 > (!(-12 as u32)));
	ctx.r[31].s64 = ctx.r[11].s64 + -12;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83150AE0: 4082FFB8  bne 0x83150a98
	if !ctx.cr[0].eq {
	pc = 0x83150A98; continue 'dispatch;
	}
	// 83150AE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83150AE8: 480576D4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150AF0 size=104
    let mut pc: u32 = 0x83150AF0;
    'dispatch: loop {
        match pc {
            0x83150AF0 => {
    //   block [0x83150AF0..0x83150B58)
	// 83150AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83150AF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83150AF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83150AFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83150B00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83150B04: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83150B08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83150B0C: 419A0014  beq cr6, 0x83150b20
	if ctx.cr[6].eq {
	pc = 0x83150B20; continue 'dispatch;
	}
	// 83150B10: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150B14: 814B0094  lwz r10, 0x94(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) } as u64;
	// 83150B18: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150B1C: 4E800421  bctrl
	ctx.lr = 0x83150B20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83150B20: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83150B24: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83150B28: 419A001C  beq cr6, 0x83150b44
	if ctx.cr[6].eq {
	pc = 0x83150B44; continue 'dispatch;
	}
	// 83150B2C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150B30: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150B34: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150B38: 4E800421  bctrl
	ctx.lr = 0x83150B3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83150B3C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83150B40: 913F0030  stw r9, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[9].u32 ) };
	// 83150B44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83150B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83150B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83150B50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83150B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83150B58 size=8
    let mut pc: u32 = 0x83150B58;
    'dispatch: loop {
        match pc {
            0x83150B58 => {
    //   block [0x83150B58..0x83150B60)
	// 83150B58: 90830030  stw r4, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[4].u32 ) };
	// 83150B5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83150B60 size=16
    let mut pc: u32 = 0x83150B60;
    'dispatch: loop {
        match pc {
            0x83150B60 => {
    //   block [0x83150B60..0x83150B70)
	// 83150B60: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 83150B64: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83150B68: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83150B6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83150B70 size=20
    let mut pc: u32 = 0x83150B70;
    'dispatch: loop {
        match pc {
            0x83150B70 => {
    //   block [0x83150B70..0x83150B84)
	// 83150B70: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83150B74: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 83150B78: 386BFFFF  addi r3, r11, -1
	ctx.r[3].s64 = ctx.r[11].s64 + -1;
	// 83150B7C: 906A0034  stw r3, 0x34(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[3].u32 ) };
	// 83150B80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150B88 size=76
    let mut pc: u32 = 0x83150B88;
    'dispatch: loop {
        match pc {
            0x83150B88 => {
    //   block [0x83150B88..0x83150BD4)
	// 83150B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83150B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83150B90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83150B94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83150B98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83150B9C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83150BA0: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 83150BA4: 392B5168  addi r9, r11, 0x5168
	ctx.r[9].s64 = ctx.r[11].s64 + 20840;
	// 83150BA8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83150BAC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83150BB0: 419A0010  beq cr6, 0x83150bc0
	if ctx.cr[6].eq {
	pc = 0x83150BC0; continue 'dispatch;
	}
	// 83150BB4: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 83150BB8: 4800F0C9  bl 0x8315fc80
	ctx.lr = 0x83150BBC;
	sub_8315FC80(ctx, base);
	// 83150BBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83150BC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83150BC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83150BC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83150BCC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83150BD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150BD8 size=216
    let mut pc: u32 = 0x83150BD8;
    'dispatch: loop {
        match pc {
            0x83150BD8 => {
    //   block [0x83150BD8..0x83150CB0)
	// 83150BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83150BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83150BE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83150BE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83150BE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83150BEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83150BF0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83150BF4: 93C40000  stw r30, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83150BF8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83150BFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83150C00: 419A0028  beq cr6, 0x83150c28
	if ctx.cr[6].eq {
	pc = 0x83150C28; continue 'dispatch;
	}
	// 83150C04: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83150C08: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83150C0C: 4099001C  ble cr6, 0x83150c28
	if !ctx.cr[6].gt {
	pc = 0x83150C28; continue 'dispatch;
	}
	// 83150C10: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83150C14: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83150C18: 388B518C  addi r4, r11, 0x518c
	ctx.r[4].s64 = ctx.r[11].s64 + 20876;
	// 83150C1C: 4800EEFD  bl 0x8315fb18
	ctx.lr = 0x83150C20;
	sub_8315FB18(ctx, base);
	// 83150C20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83150C24: 4BFF0CB5  bl 0x831418d8
	ctx.lr = 0x83150C28;
	sub_831418D8(ctx, base);
	// 83150C28: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83150C2C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83150C30: 419A0014  beq cr6, 0x83150c44
	if ctx.cr[6].eq {
	pc = 0x83150C44; continue 'dispatch;
	}
	// 83150C34: 3D608339  lis r11, -0x7cc7
	ctx.r[11].s64 = -2093416448;
	// 83150C38: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 83150C3C: 386B7F48  addi r3, r11, 0x7f48
	ctx.r[3].s64 = ctx.r[11].s64 + 32584;
	// 83150C40: 4BFEF549  bl 0x83140188
	ctx.lr = 0x83150C44;
	sub_83140188(ctx, base);
	// 83150C44: 807F002C  lwz r3, 0x2c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83150C48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83150C4C: 419A0014  beq cr6, 0x83150c60
	if ctx.cr[6].eq {
	pc = 0x83150C60; continue 'dispatch;
	}
	// 83150C50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150C54: 814B0094  lwz r10, 0x94(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(148 as u32) ) } as u64;
	// 83150C58: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150C5C: 4E800421  bctrl
	ctx.lr = 0x83150C60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83150C60: 807F0030  lwz r3, 0x30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83150C64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83150C68: 419A0018  beq cr6, 0x83150c80
	if ctx.cr[6].eq {
	pc = 0x83150C80; continue 'dispatch;
	}
	// 83150C6C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150C70: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150C74: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150C78: 4E800421  bctrl
	ctx.lr = 0x83150C7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83150C7C: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 83150C80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150C84: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83150C88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83150C8C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83150C90: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150C94: 4E800421  bctrl
	ctx.lr = 0x83150C98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83150C98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83150C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83150CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83150CA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83150CA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83150CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150CB0 size=452
    let mut pc: u32 = 0x83150CB0;
    'dispatch: loop {
        match pc {
            0x83150CB0 => {
    //   block [0x83150CB0..0x83150E74)
	// 83150CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83150CB4: 480574B5  bl 0x831a8168
	ctx.lr = 0x83150CB8;
	sub_831A8130(ctx, base);
	// 83150CB8: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83150CBC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83150CC0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83150CC4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83150CC8: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 83150CCC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83150CD0: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 83150CD4: 396100F0  addi r11, r1, 0xf0
	ctx.r[11].s64 = ctx.r[1].s64 + 240;
	// 83150CD8: 93E10070  stw r31, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[31].u32 ) };
	// 83150CDC: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 83150CE0: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 83150CE4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150CE8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83150CEC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83150CF0: 4200FFF8  bdnz 0x83150ce8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83150CE8; continue 'dispatch;
	}
	// 83150CF4: 39610110  addi r11, r1, 0x110
	ctx.r[11].s64 = ctx.r[1].s64 + 272;
	// 83150CF8: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 83150CFC: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 83150D00: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150D04: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83150D08: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83150D0C: 4200FFF8  bdnz 0x83150d04
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83150D04; continue 'dispatch;
	}
	// 83150D10: 93E10130  stw r31, 0x130(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(304 as u32), ctx.r[31].u32 ) };
	// 83150D14: 39610138  addi r11, r1, 0x138
	ctx.r[11].s64 = ctx.r[1].s64 + 312;
	// 83150D18: 93E10134  stw r31, 0x134(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), ctx.r[31].u32 ) };
	// 83150D1C: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 83150D20: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 83150D24: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150D28: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83150D2C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83150D30: 4200FFF8  bdnz 0x83150d28
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83150D28; continue 'dispatch;
	}
	// 83150D34: 39610158  addi r11, r1, 0x158
	ctx.r[11].s64 = ctx.r[1].s64 + 344;
	// 83150D38: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 83150D3C: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 83150D40: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150D44: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83150D48: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83150D4C: 4200FFF8  bdnz 0x83150d44
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83150D44; continue 'dispatch;
	}
	// 83150D50: 3961017C  addi r11, r1, 0x17c
	ctx.r[11].s64 = ctx.r[1].s64 + 380;
	// 83150D54: 93E10178  stw r31, 0x178(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(376 as u32), ctx.r[31].u32 ) };
	// 83150D58: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 83150D5C: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 83150D60: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150D64: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83150D68: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83150D6C: 4200FFF8  bdnz 0x83150d64
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83150D64; continue 'dispatch;
	}
	// 83150D70: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83150D74: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83150D78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83150D7C: 4BFF4845  bl 0x831455c0
	ctx.lr = 0x83150D80;
	sub_831455C0(ctx, base);
	// 83150D80: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83150D84: 409A0028  bne cr6, 0x83150dac
	if !ctx.cr[6].eq {
	pc = 0x83150DAC; continue 'dispatch;
	}
	// 83150D88: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83150D8C: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83150D90: 388B5204  addi r4, r11, 0x5204
	ctx.r[4].s64 = ctx.r[11].s64 + 20996;
	// 83150D94: 4800ED95  bl 0x8315fb28
	ctx.lr = 0x83150D98;
	sub_8315FB28(ctx, base);
	// 83150D98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83150D9C: 4BFFFD55  bl 0x83150af0
	ctx.lr = 0x83150DA0;
	sub_83150AF0(ctx, base);
	// 83150DA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83150DA4: 382101E0  addi r1, r1, 0x1e0
	ctx.r[1].s64 = ctx.r[1].s64 + 480;
	// 83150DA8: 48057410  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83150DAC: 8161006C  lwz r11, 0x6c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83150DB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83150DB4: 409A002C  bne cr6, 0x83150de0
	if !ctx.cr[6].eq {
	pc = 0x83150DE0; continue 'dispatch;
	}
	// 83150DB8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83150DBC: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 83150DC0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83150DC4: 388B51D4  addi r4, r11, 0x51d4
	ctx.r[4].s64 = ctx.r[11].s64 + 20948;
	// 83150DC8: 4800ED61  bl 0x8315fb28
	ctx.lr = 0x83150DCC;
	sub_8315FB28(ctx, base);
	// 83150DCC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83150DD0: 4BFFFD21  bl 0x83150af0
	ctx.lr = 0x83150DD4;
	sub_83150AF0(ctx, base);
	// 83150DD4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83150DD8: 382101E0  addi r1, r1, 0x1e0
	ctx.r[1].s64 = ctx.r[1].s64 + 480;
	// 83150DDC: 480573DC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83150DE0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83150DE4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 83150DE8: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 83150DEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83150DF0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 83150DF4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 83150DF8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83150DFC: 480077A5  bl 0x831585a0
	ctx.lr = 0x83150E00;
	sub_831585A0(ctx, base);
	// 83150E00: 907E0030  stw r3, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[3].u32 ) };
	// 83150E04: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83150E08: 409A0024  bne cr6, 0x83150e2c
	if !ctx.cr[6].eq {
	pc = 0x83150E2C; continue 'dispatch;
	}
	// 83150E0C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83150E10: 388B51B0  addi r4, r11, 0x51b0
	ctx.r[4].s64 = ctx.r[11].s64 + 20912;
	// 83150E14: 4800ED05  bl 0x8315fb18
	ctx.lr = 0x83150E18;
	sub_8315FB18(ctx, base);
	// 83150E18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83150E1C: 4BFFFCD5  bl 0x83150af0
	ctx.lr = 0x83150E20;
	sub_83150AF0(ctx, base);
	// 83150E20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83150E24: 382101E0  addi r1, r1, 0x1e0
	ctx.r[1].s64 = ctx.r[1].s64 + 480;
	// 83150E28: 48057390  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83150E2C: 395E0018  addi r10, r30, 0x18
	ctx.r[10].s64 = ctx.r[30].s64 + 24;
	// 83150E30: 93BE002C  stw r29, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 83150E34: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 83150E38: 39200005  li r9, 5
	ctx.r[9].s64 = 5;
	// 83150E3C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83150E40: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150E44: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83150E48: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83150E4C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83150E50: 4200FFF0  bdnz 0x83150e40
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83150E40; continue 'dispatch;
	}
	// 83150E54: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150E58: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83150E5C: 814B0090  lwz r10, 0x90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(144 as u32) ) } as u64;
	// 83150E60: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150E64: 4E800421  bctrl
	ctx.lr = 0x83150E68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83150E68: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83150E6C: 382101E0  addi r1, r1, 0x1e0
	ctx.r[1].s64 = ctx.r[1].s64 + 480;
	// 83150E70: 48057348  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150E78 size=76
    let mut pc: u32 = 0x83150E78;
    'dispatch: loop {
        match pc {
            0x83150E78 => {
    //   block [0x83150E78..0x83150EC4)
	// 83150E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83150E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83150E80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83150E84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83150E88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83150E8C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83150E90: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 83150E94: 392B5168  addi r9, r11, 0x5168
	ctx.r[9].s64 = ctx.r[11].s64 + 20840;
	// 83150E98: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83150E9C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83150EA0: 419A0010  beq cr6, 0x83150eb0
	if ctx.cr[6].eq {
	pc = 0x83150EB0; continue 'dispatch;
	}
	// 83150EA4: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 83150EA8: 4800EDD9  bl 0x8315fc80
	ctx.lr = 0x83150EAC;
	sub_8315FC80(ctx, base);
	// 83150EAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83150EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83150EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83150EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83150EBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83150EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83150EC8 size=256
    let mut pc: u32 = 0x83150EC8;
    'dispatch: loop {
        match pc {
            0x83150EC8 => {
    //   block [0x83150EC8..0x83150FC8)
	// 83150EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83150ECC: 48057295  bl 0x831a8160
	ctx.lr = 0x83150ED0;
	sub_831A8130(ctx, base);
	// 83150ED0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83150ED4: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 83150ED8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83150EDC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83150EE0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83150EE4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83150EE8: 93DC0000  stw r30, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83150EEC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 83150EF0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83150EF4: 38CB522C  addi r6, r11, 0x522c
	ctx.r[6].s64 = ctx.r[11].s64 + 21036;
	// 83150EF8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 83150EFC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83150F00: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 83150F04: 4800ED15  bl 0x8315fc18
	ctx.lr = 0x83150F08;
	sub_8315FC18(ctx, base);
	// 83150F08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83150F0C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83150F10: 419A0084  beq cr6, 0x83150f94
	if ctx.cr[6].eq {
	pc = 0x83150F94; continue 'dispatch;
	}
	// 83150F14: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83150F18: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 83150F1C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83150F20: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83150F24: 392B5170  addi r9, r11, 0x5170
	ctx.r[9].s64 = ctx.r[11].s64 + 20848;
	// 83150F28: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83150F2C: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 83150F30: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 83150F34: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 83150F38: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83150F3C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83150F40: 93DF0018  stw r30, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 83150F44: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 83150F48: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 83150F4C: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 83150F50: 9BDF0028  stb r30, 0x28(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u8 ) };
	// 83150F54: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 83150F58: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 83150F5C: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 83150F60: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 83150F64: 4BFFFD4D  bl 0x83150cb0
	ctx.lr = 0x83150F68;
	sub_83150CB0(ctx, base);
	// 83150F68: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83150F6C: 409A0050  bne cr6, 0x83150fbc
	if !ctx.cr[6].eq {
	pc = 0x83150FBC; continue 'dispatch;
	}
	// 83150F70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83150F74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83150F78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83150F7C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83150F80: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83150F84: 4E800421  bctrl
	ctx.lr = 0x83150F88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83150F88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83150F8C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83150F90: 48057220  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83150F94: 3960FFFD  li r11, -3
	ctx.r[11].s64 = -3;
	// 83150F98: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83150F9C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83150FA0: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83150FA4: 388A5234  addi r4, r10, 0x5234
	ctx.r[4].s64 = ctx.r[10].s64 + 21044;
	// 83150FA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83150FAC: 4800EB95  bl 0x8315fb40
	ctx.lr = 0x83150FB0;
	sub_8315FB40(ctx, base);
	// 83150FB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83150FB4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83150FB8: 480571F8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83150FBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83150FC0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83150FC4: 480571EC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83150FC8 size=20
    let mut pc: u32 = 0x83150FC8;
    'dispatch: loop {
        match pc {
            0x83150FC8 => {
    //   block [0x83150FC8..0x83150FDC)
	// 83150FC8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83150FCC: 816A0014  lwz r11, 0x14(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 83150FD0: 7C645A14  add r3, r4, r11
	ctx.r[3].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 83150FD4: 906A0014  stw r3, 0x14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 83150FD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83150FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83150FE0 size=112
    let mut pc: u32 = 0x83150FE0;
    'dispatch: loop {
        match pc {
            0x83150FE0 => {
    //   block [0x83150FE0..0x83151050)
	// 83150FE0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83150FE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83150FE8: 419A0058  beq cr6, 0x83151040
	if ctx.cr[6].eq {
	pc = 0x83151040; continue 'dispatch;
	}
	// 83150FEC: 396BFFF8  addi r11, r11, -8
	ctx.r[11].s64 = ctx.r[11].s64 + -8;
	// 83150FF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83150FF4: 419A004C  beq cr6, 0x83151040
	if ctx.cr[6].eq {
	pc = 0x83151040; continue 'dispatch;
	}
	// 83150FF8: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83150FFC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83151000: 390AFFF8  addi r8, r10, -8
	ctx.r[8].s64 = ctx.r[10].s64 + -8;
	// 83151004: 409A0008  bne cr6, 0x8315100c
	if !ctx.cr[6].eq {
	pc = 0x8315100C; continue 'dispatch;
	}
	// 83151008: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8315100C: E94B0018  ld r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 83151010: 2B2A0000  cmpldi cr6, r10, 0
	ctx.cr[6].compare_u64(ctx.r[10].u64, 0, &mut ctx.xer);
	// 83151014: 419A001C  beq cr6, 0x83151030
	if ctx.cr[6].eq {
	pc = 0x83151030; continue 'dispatch;
	}
	// 83151018: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8315101C: 812B0014  lwz r9, 0x14(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83151020: 5487003E  slwi r7, r4, 0
	ctx.r[7].u32 = ctx.r[4].u32.wrapping_shl(0);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 83151024: 7D475050  subf r10, r7, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 83151028: 7CCA4A14  add r6, r10, r9
	ctx.r[6].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8315102C: 90CB0014  stw r6, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 83151030: F88B0018  std r4, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[4].u64 ) };
	// 83151034: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 83151038: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 8315103C: 409AFFBC  bne cr6, 0x83150ff8
	if !ctx.cr[6].eq {
	pc = 0x83150FF8; continue 'dispatch;
	}
	// 83151040: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83151044: 356B0001  addic. r11, r11, 1
	ctx.xer.ca = (ctx.r[11].u32 > (!(1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83151048: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8315104C: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83151050 size=16
    let mut pc: u32 = 0x83151050;
    'dispatch: loop {
        match pc {
            0x83151050 => {
    //   block [0x83151050..0x83151060)
	// 83151050: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83151054: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83151058: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 8315105C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83151060 size=140
    let mut pc: u32 = 0x83151060;
    'dispatch: loop {
        match pc {
            0x83151060 => {
    //   block [0x83151060..0x831510EC)
	// 83151060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83151064: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83151068: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315106C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83151070: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83151074: 38CB525C  addi r6, r11, 0x525c
	ctx.r[6].s64 = ctx.r[11].s64 + 21084;
	// 83151078: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 8315107C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83151080: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 83151084: 4800EB95  bl 0x8315fc18
	ctx.lr = 0x83151088;
	sub_8315FC18(ctx, base);
	// 83151088: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8315108C: 419A0038  beq cr6, 0x831510c4
	if ctx.cr[6].eq {
	pc = 0x831510C4; continue 'dispatch;
	}
	// 83151090: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83151094: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83151098: 392A4C24  addi r9, r10, 0x4c24
	ctx.r[9].s64 = ctx.r[10].s64 + 19492;
	// 8315109C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831510A0: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831510A4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 831510A8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831510AC: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 831510B0: F9630018  std r11, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 831510B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831510B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831510BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831510C0: 4E800020  blr
	return;
	// 831510C4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831510C8: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 831510CC: 388B5250  addi r4, r11, 0x5250
	ctx.r[4].s64 = ctx.r[11].s64 + 21072;
	// 831510D0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831510D4: 4800EA6D  bl 0x8315fb40
	ctx.lr = 0x831510D8;
	sub_8315FB40(ctx, base);
	// 831510D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831510DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831510E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831510E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831510E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831510F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831510F0 size=124
    let mut pc: u32 = 0x831510F0;
    'dispatch: loop {
        match pc {
            0x831510F0 => {
    //   block [0x831510F0..0x8315116C)
	// 831510F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831510F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831510F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831510FC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83151100: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 83151104: 38AB5274  addi r5, r11, 0x5274
	ctx.r[5].s64 = ctx.r[11].s64 + 21108;
	// 83151108: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8315110C: 38600014  li r3, 0x14
	ctx.r[3].s64 = 20;
	// 83151110: 4800EBE9  bl 0x8315fcf8
	ctx.lr = 0x83151114;
	sub_8315FCF8(ctx, base);
	// 83151114: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83151118: 419A002C  beq cr6, 0x83151144
	if ctx.cr[6].eq {
	pc = 0x83151144; continue 'dispatch;
	}
	// 8315111C: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83151120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83151124: 392A5248  addi r9, r10, 0x5248
	ctx.r[9].s64 = ctx.r[10].s64 + 21064;
	// 83151128: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8315112C: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83151130: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83151134: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83151138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8315113C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83151140: 4E800020  blr
	return;
	// 83151144: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83151148: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8315114C: 388B5268  addi r4, r11, 0x5268
	ctx.r[4].s64 = ctx.r[11].s64 + 21096;
	// 83151150: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83151154: 4800E9ED  bl 0x8315fb40
	ctx.lr = 0x83151158;
	sub_8315FB40(ctx, base);
	// 83151158: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8315115C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83151160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83151164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83151168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83151170 size=24
    let mut pc: u32 = 0x83151170;
    'dispatch: loop {
        match pc {
            0x83151170 => {
    //   block [0x83151170..0x83151188)
	// 83151170: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83151174: 38840008  addi r4, r4, 8
	ctx.r[4].s64 = ctx.r[4].s64 + 8;
	// 83151178: 409A0008  bne cr6, 0x83151180
	if !ctx.cr[6].eq {
	pc = 0x83151180; continue 'dispatch;
	}
	// 8315117C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83151180: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 83151184: 4BFFA2DC  b 0x8314b460
	sub_8314B460(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83151188 size=24
    let mut pc: u32 = 0x83151188;
    'dispatch: loop {
        match pc {
            0x83151188 => {
    //   block [0x83151188..0x831511A0)
	// 83151188: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8315118C: 38840008  addi r4, r4, 8
	ctx.r[4].s64 = ctx.r[4].s64 + 8;
	// 83151190: 409A0008  bne cr6, 0x83151198
	if !ctx.cr[6].eq {
	pc = 0x83151198; continue 'dispatch;
	}
	// 83151194: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83151198: 38630004  addi r3, r3, 4
	ctx.r[3].s64 = ctx.r[3].s64 + 4;
	// 8315119C: 4BFEEFEC  b 0x83140188
	sub_83140188(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831511A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831511A0 size=76
    let mut pc: u32 = 0x831511A0;
    'dispatch: loop {
        match pc {
            0x831511A0 => {
    //   block [0x831511A0..0x831511EC)
	// 831511A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831511A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831511A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831511AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831511B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831511B4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831511B8: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 831511BC: 392B5248  addi r9, r11, 0x5248
	ctx.r[9].s64 = ctx.r[11].s64 + 21064;
	// 831511C0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831511C4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831511C8: 419A0010  beq cr6, 0x831511d8
	if ctx.cr[6].eq {
	pc = 0x831511D8; continue 'dispatch;
	}
	// 831511CC: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 831511D0: 4800EAB1  bl 0x8315fc80
	ctx.lr = 0x831511D4;
	sub_8315FC80(ctx, base);
	// 831511D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831511D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831511DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831511E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831511E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831511E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831511F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831511F0 size=56
    let mut pc: u32 = 0x831511F0;
    'dispatch: loop {
        match pc {
            0x831511F0 => {
    //   block [0x831511F0..0x83151228)
	// 831511F0: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 831511F4: C8040000  lfd f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 831511F8: C9A50000  lfd f13, 0(r5)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 831511FC: C9840008  lfd f12, 8(r4)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 83151200: FD60682A  fadd f11, f0, f13
	ctx.f[11].f64 = ctx.f[0].f64 + ctx.f[13].f64;
	// 83151204: C9450008  lfd f10, 8(r5)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 83151208: FD2C502A  fadd f9, f12, f10
	ctx.f[9].f64 = ctx.f[12].f64 + ctx.f[10].f64;
	// 8315120C: D961FFF0  stfd f11, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[11].u64 ) };
	// 83151210: D921FFF8  stfd f9, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[9].u64 ) };
	// 83151214: E92B0008  ld r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 83151218: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8315121C: F9430000  std r10, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 83151220: F9230008  std r9, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
	// 83151224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83151228 size=64
    let mut pc: u32 = 0x83151228;
    'dispatch: loop {
        match pc {
            0x83151228 => {
    //   block [0x83151228..0x83151268)
	// 83151228: C8040000  lfd f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 8315122C: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 83151230: C9A40008  lfd f13, 8(r4)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 83151234: C9850008  lfd f12, 8(r5)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 83151238: FD6C0372  fmul f11, f12, f13
	ctx.f[11].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 8315123C: C9450000  lfd f10, 0(r5)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 83151240: FD2C0032  fmul f9, f12, f0
	ctx.f[9].f64 = ctx.f[12].f64 * ctx.f[0].f64;
	// 83151244: FD0A5838  fmsub f8, f10, f0, f11
	ctx.f[8].f64 = ctx.f[10].f64 * ctx.f[0].f64 - ctx.f[11].f64;
	// 83151248: D901FFF0  stfd f8, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[8].u64 ) };
	// 8315124C: FCED4ABA  fmadd f7, f13, f10, f9
	ctx.f[7].f64 = ctx.f[13].f64 * ctx.f[10].f64 + ctx.f[9].f64;
	// 83151250: D8E1FFF8  stfd f7, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[7].u64 ) };
	// 83151254: E92B0008  ld r9, 8(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 83151258: E94B0000  ld r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8315125C: F9430000  std r10, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 83151260: F9230008  std r9, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
	// 83151264: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83151268 size=92
    let mut pc: u32 = 0x83151268;
    'dispatch: loop {
        match pc {
            0x83151268 => {
    //   block [0x83151268..0x831512C4)
	// 83151268: C8050008  lfd f0, 8(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 8315126C: 3D608205  lis r11, -0x7dfb
	ctx.r[11].s64 = -2113601536;
	// 83151270: FDA00032  fmul f13, f0, f0
	ctx.f[13].f64 = ctx.f[0].f64 * ctx.f[0].f64;
	// 83151274: C9850000  lfd f12, 0(r5)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 83151278: C9640008  lfd f11, 8(r4)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 8315127C: 3941FFF0  addi r10, r1, -0x10
	ctx.r[10].s64 = ctx.r[1].s64 + -16;
	// 83151280: C9440000  lfd f10, 0(r4)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 83151284: FD2B0032  fmul f9, f11, f0
	ctx.f[9].f64 = ctx.f[11].f64 * ctx.f[0].f64;
	// 83151288: FD0A0032  fmul f8, f10, f0
	ctx.f[8].f64 = ctx.f[10].f64 * ctx.f[0].f64;
	// 8315128C: C80BE3A0  lfd f0, -0x1c60(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-7264 as u32) ) };
	// 83151290: FCEC6B3A  fmadd f7, f12, f12, f13
	ctx.f[7].f64 = ctx.f[12].f64 * ctx.f[12].f64 + ctx.f[13].f64;
	// 83151294: FCCA4B3A  fmadd f6, f10, f12, f9
	ctx.f[6].f64 = ctx.f[10].f64 * ctx.f[12].f64 + ctx.f[9].f64;
	// 83151298: FCAB4338  fmsub f5, f11, f12, f8
	ctx.f[5].f64 = ctx.f[11].f64 * ctx.f[12].f64 - ctx.f[8].f64;
	// 8315129C: FC803824  fdiv f4, f0, f7
	ctx.f[4].f64 = ctx.f[0].f64 / ctx.f[7].f64;
	// 831512A0: FC660132  fmul f3, f6, f4
	ctx.f[3].f64 = ctx.f[6].f64 * ctx.f[4].f64;
	// 831512A4: D861FFF0  stfd f3, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.f[3].u64 ) };
	// 831512A8: FC450132  fmul f2, f5, f4
	ctx.f[2].f64 = ctx.f[5].f64 * ctx.f[4].f64;
	// 831512AC: D841FFF8  stfd f2, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[2].u64 ) };
	// 831512B0: E90A0008  ld r8, 8(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 831512B4: E92A0000  ld r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 831512B8: F9230000  std r9, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 831512BC: F9030008  std r8, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u64 ) };
	// 831512C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831512C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831512C8 size=28
    let mut pc: u32 = 0x831512C8;
    'dispatch: loop {
        match pc {
            0x831512C8 => {
    //   block [0x831512C8..0x831512E4)
	// 831512C8: C8040000  lfd f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 831512CC: FDA00072  fmul f13, f0, f1
	ctx.f[13].f64 = ctx.f[0].f64 * ctx.f[1].f64;
	// 831512D0: D9A30000  stfd f13, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.f[13].u64 ) };
	// 831512D4: C9840008  lfd f12, 8(r4)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 831512D8: FD6C0072  fmul f11, f12, f1
	ctx.f[11].f64 = ctx.f[12].f64 * ctx.f[1].f64;
	// 831512DC: D9630008  stfd f11, 8(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.f[11].u64 ) };
	// 831512E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831512E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831512E8 size=8
    let mut pc: u32 = 0x831512E8;
    'dispatch: loop {
        match pc {
            0x831512E8 => {
    //   block [0x831512E8..0x831512F0)
	// 831512E8: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831512EC: 4800762C  b 0x83158918
	sub_83158918(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831512F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831512F0 size=80
    let mut pc: u32 = 0x831512F0;
    'dispatch: loop {
        match pc {
            0x831512F0 => {
    //   block [0x831512F0..0x83151340)
	// 831512F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831512F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831512F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831512FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83151300: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83151304: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83151308: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8315130C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83151310: 4E800421  bctrl
	ctx.lr = 0x83151314;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83151314: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83151318: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8315131C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83151320: 81090008  lwz r8, 8(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 83151324: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 83151328: 4E800421  bctrl
	ctx.lr = 0x8315132C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8315132C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83151330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83151334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83151338: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8315133C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83151340 size=48
    let mut pc: u32 = 0x83151340;
    'dispatch: loop {
        match pc {
            0x83151340 => {
    //   block [0x83151340..0x83151370)
	// 83151340: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83151344: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 83151348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8315134C: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 83151350: 392A52AC  addi r9, r10, 0x52ac
	ctx.r[9].s64 = ctx.r[10].s64 + 21164;
	// 83151354: 90A3001C  stw r5, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 83151358: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8315135C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83151360: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83151364: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83151368: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8315136C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83151370 size=48
    let mut pc: u32 = 0x83151370;
    'dispatch: loop {
        match pc {
            0x83151370 => {
    //   block [0x83151370..0x831513A0)
	// 83151370: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83151374: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 83151378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8315137C: 90C30010  stw r6, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 83151380: 392A52AC  addi r9, r10, 0x52ac
	ctx.r[9].s64 = ctx.r[10].s64 + 21164;
	// 83151384: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 83151388: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8315138C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83151390: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83151394: 91030018  stw r8, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 83151398: 90A3001C  stw r5, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[5].u32 ) };
	// 8315139C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831513A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831513A0 size=16
    let mut pc: u32 = 0x831513A0;
    'dispatch: loop {
        match pc {
            0x831513A0 => {
    //   block [0x831513A0..0x831513B0)
	// 831513A0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831513A4: 394B52AC  addi r10, r11, 0x52ac
	ctx.r[10].s64 = ctx.r[11].s64 + 21164;
	// 831513A8: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831513AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831513B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831513B0 size=44
    let mut pc: u32 = 0x831513B0;
    'dispatch: loop {
        match pc {
            0x831513B0 => {
    //   block [0x831513B0..0x831513DC)
	// 831513B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831513B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831513B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831513BC: 4800E8FD  bl 0x8315fcb8
	ctx.lr = 0x831513C0;
	sub_8315FCB8(ctx, base);
	// 831513C0: 396300B4  addi r11, r3, 0xb4
	ctx.r[11].s64 = ctx.r[3].s64 + 180;
	// 831513C4: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831513C8: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831513CC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831513D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831513D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831513D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831513E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831513E0 size=112
    let mut pc: u32 = 0x831513E0;
    'dispatch: loop {
        match pc {
            0x831513E0 => {
    //   block [0x831513E0..0x83151450)
	// 831513E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831513E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831513E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831513EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831513F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831513F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831513F8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 831513FC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83151400: 409A0038  bne cr6, 0x83151438
	if !ctx.cr[6].eq {
	pc = 0x83151438; continue 'dispatch;
	}
	// 83151404: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83151408: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8315140C: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83151410: 419A0010  beq cr6, 0x83151420
	if ctx.cr[6].eq {
	pc = 0x83151420; continue 'dispatch;
	}
	// 83151414: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151418: 4800E351  bl 0x8315f768
	ctx.lr = 0x8315141C;
	sub_8315F768(ctx, base);
	// 8315141C: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 83151420: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83151424: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83151428: 419A0010  beq cr6, 0x83151438
	if ctx.cr[6].eq {
	pc = 0x83151438; continue 'dispatch;
	}
	// 8315142C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151430: 4800E339  bl 0x8315f768
	ctx.lr = 0x83151434;
	sub_8315F768(ctx, base);
	// 83151434: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 83151438: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8315143C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83151440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83151444: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83151448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8315144C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83151450 size=28
    let mut pc: u32 = 0x83151450;
    'dispatch: loop {
        match pc {
            0x83151450 => {
    //   block [0x83151450..0x8315146C)
	// 83151450: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83151454: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83151458: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8315145C: 91440004  stw r10, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 83151460: 81230018  lwz r9, 0x18(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 83151464: 91240008  stw r9, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 83151468: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83151470 size=12
    let mut pc: u32 = 0x83151470;
    'dispatch: loop {
        match pc {
            0x83151470 => {
    //   block [0x83151470..0x8315147C)
	// 83151470: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83151474: 806B0058  lwz r3, 0x58(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 83151478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83151480 size=12
    let mut pc: u32 = 0x83151480;
    'dispatch: loop {
        match pc {
            0x83151480 => {
    //   block [0x83151480..0x8315148C)
	// 83151480: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83151484: 806B005C  lwz r3, 0x5c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 83151488: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83151490 size=164
    let mut pc: u32 = 0x83151490;
    'dispatch: loop {
        match pc {
            0x83151490 => {
    //   block [0x83151490..0x83151534)
	// 83151490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83151494: 48056CD9  bl 0x831a816c
	ctx.lr = 0x83151498;
	sub_831A8130(ctx, base);
	// 83151498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315149C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831514A0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831514A4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831514A8: 814B0058  lwz r10, 0x58(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(88 as u32) ) } as u64;
	// 831514AC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831514B0: 409A0058  bne cr6, 0x83151508
	if !ctx.cr[6].eq {
	pc = 0x83151508; continue 'dispatch;
	}
	// 831514B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831514B8: 3945000F  addi r10, r5, 0xf
	ctx.r[10].s64 = ctx.r[5].s64 + 15;
	// 831514BC: 555E0036  rlwinm r30, r10, 0, 0, 0x1b
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 831514C0: 812B0014  lwz r9, 0x14(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831514C4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831514C8: 4E800421  bctrl
	ctx.lr = 0x831514CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831514CC: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 831514D0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 831514D4: 38A852D0  addi r5, r8, 0x52d0
	ctx.r[5].s64 = ctx.r[8].s64 + 21200;
	// 831514D8: 38E00002  li r7, 2
	ctx.r[7].s64 = 2;
	// 831514DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831514E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831514E4: 4800E21D  bl 0x8315f700
	ctx.lr = 0x831514E8;
	sub_8315F700(ctx, base);
	// 831514E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831514EC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 831514F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831514F4: 409A0020  bne cr6, 0x83151514
	if !ctx.cr[6].eq {
	pc = 0x83151514; continue 'dispatch;
	}
	// 831514F8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831514FC: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83151500: 388B52C4  addi r4, r11, 0x52c4
	ctx.r[4].s64 = ctx.r[11].s64 + 21188;
	// 83151504: 4800E63D  bl 0x8315fb40
	ctx.lr = 0x83151508;
	sub_8315FB40(ctx, base);
	// 83151508: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8315150C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83151510: 48056CAC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83151514: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83151518: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8315151C: 916A0058  stw r11, 0x58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83151520: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83151524: 93C9005C  stw r30, 0x5c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 83151528: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8315152C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83151530: 48056C8C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83151538 size=84
    let mut pc: u32 = 0x83151538;
    'dispatch: loop {
        match pc {
            0x83151538 => {
    //   block [0x83151538..0x8315158C)
	// 83151538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315153C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83151540: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83151544: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83151548: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8315154C: 809F0018  lwz r4, 0x18(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83151550: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83151554: 419A0024  beq cr6, 0x83151578
	if ctx.cr[6].eq {
	pc = 0x83151578; continue 'dispatch;
	}
	// 83151558: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8315155C: 4800E20D  bl 0x8315f768
	ctx.lr = 0x83151560;
	sub_8315F768(ctx, base);
	// 83151560: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83151564: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83151568: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8315156C: 916A0058  stw r11, 0x58(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83151570: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83151574: 9169005C  stw r11, 0x5c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83151578: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8315157C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83151580: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83151584: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83151588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83151590 size=8
    let mut pc: u32 = 0x83151590;
    'dispatch: loop {
        match pc {
            0x83151590 => {
    //   block [0x83151590..0x83151598)
	// 83151590: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83151594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83151598 size=76
    let mut pc: u32 = 0x83151598;
    'dispatch: loop {
        match pc {
            0x83151598 => {
    //   block [0x83151598..0x831515E4)
	// 83151598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315159C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831515A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831515A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831515A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831515AC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831515B0: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 831515B4: 392B52AC  addi r9, r11, 0x52ac
	ctx.r[9].s64 = ctx.r[11].s64 + 21164;
	// 831515B8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831515BC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831515C0: 419A0010  beq cr6, 0x831515d0
	if ctx.cr[6].eq {
	pc = 0x831515D0; continue 'dispatch;
	}
	// 831515C4: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 831515C8: 4800E6B9  bl 0x8315fc80
	ctx.lr = 0x831515CC;
	sub_8315FC80(ctx, base);
	// 831515CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831515D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831515D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831515D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831515DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831515E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831515E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831515E8 size=244
    let mut pc: u32 = 0x831515E8;
    'dispatch: loop {
        match pc {
            0x831515E8 => {
    //   block [0x831515E8..0x831516DC)
	// 831515E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831515EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831515F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831515F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831515F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831515FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83151600: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83151604: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83151608: 409A00B8  bne cr6, 0x831516c0
	if !ctx.cr[6].eq {
	pc = 0x831516C0; continue 'dispatch;
	}
	// 8315160C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83151610: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83151614: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83151618: 4E800421  bctrl
	ctx.lr = 0x8315161C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8315161C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83151620: 3D208219  lis r9, -0x7de7
	ctx.r[9].s64 = -2112290816;
	// 83151624: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151628: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8315162C: 38A952F8  addi r5, r9, 0x52f8
	ctx.r[5].s64 = ctx.r[9].s64 + 21240;
	// 83151630: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 83151634: 4800E00D  bl 0x8315f640
	ctx.lr = 0x83151638;
	sub_8315F640(ctx, base);
	// 83151638: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 8315163C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83151640: 409A0030  bne cr6, 0x83151670
	if !ctx.cr[6].eq {
	pc = 0x83151670; continue 'dispatch;
	}
	// 83151644: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83151648: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8315164C: 388B52EC  addi r4, r11, 0x52ec
	ctx.r[4].s64 = ctx.r[11].s64 + 21228;
	// 83151650: 4800E4F1  bl 0x8315fb40
	ctx.lr = 0x83151654;
	sub_8315FB40(ctx, base);
	// 83151654: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83151658: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8315165C: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 83151660: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83151664: 4E800421  bctrl
	ctx.lr = 0x83151668;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83151668: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8315166C: 48000058  b 0x831516c4
	pc = 0x831516C4; continue 'dispatch;
	// 83151670: 3D608210  lis r11, -0x7df0
	ctx.r[11].s64 = -2112880640;
	// 83151674: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151678: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8315167C: 38ABEDB8  addi r5, r11, -0x1248
	ctx.r[5].s64 = ctx.r[11].s64 + -4680;
	// 83151680: 388001A0  li r4, 0x1a0
	ctx.r[4].s64 = 416;
	// 83151684: 4800DFBD  bl 0x8315f640
	ctx.lr = 0x83151688;
	sub_8315F640(ctx, base);
	// 83151688: 907F0014  stw r3, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 8315168C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83151690: 409A0030  bne cr6, 0x831516c0
	if !ctx.cr[6].eq {
	pc = 0x831516C0; continue 'dispatch;
	}
	// 83151694: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83151698: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8315169C: 388B52E0  addi r4, r11, 0x52e0
	ctx.r[4].s64 = ctx.r[11].s64 + 21216;
	// 831516A0: 4800E4A1  bl 0x8315fb40
	ctx.lr = 0x831516A4;
	sub_8315FB40(ctx, base);
	// 831516A4: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831516A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831516AC: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 831516B0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831516B4: 4E800421  bctrl
	ctx.lr = 0x831516B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831516B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831516BC: 48000008  b 0x831516c4
	pc = 0x831516C4; continue 'dispatch;
	// 831516C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831516C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831516C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831516CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831516D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831516D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831516D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831516E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831516E0 size=136
    let mut pc: u32 = 0x831516E0;
    'dispatch: loop {
        match pc {
            0x831516E0 => {
    //   block [0x831516E0..0x83151768)
	// 831516E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831516E4: 48056A89  bl 0x831a816c
	ctx.lr = 0x831516E8;
	sub_831A8130(ctx, base);
	// 831516E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831516EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831516F0: 3BA00004  li r29, 4
	ctx.r[29].s64 = 4;
	// 831516F4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 831516F8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831516FC: 480105AD  bl 0x83161ca8
	ctx.lr = 0x83151700;
	sub_83161CA8(ctx, base);
	// 83151700: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83151704: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83151708: 48010511  bl 0x83161c18
	ctx.lr = 0x8315170C;
	sub_83161C18(ctx, base);
	// 8315170C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83151710: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83151714: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83151718: 40980008  bge cr6, 0x83151720
	if !ctx.cr[6].lt {
	pc = 0x83151720; continue 'dispatch;
	}
	// 8315171C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 83151720: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83151724: 419A0014  beq cr6, 0x83151738
	if ctx.cr[6].eq {
	pc = 0x83151738; continue 'dispatch;
	}
	// 83151728: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8315172C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83151730: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83151734: 48056DDD  bl 0x831a8510
	ctx.lr = 0x83151738;
	sub_831A8510(ctx, base);
	// 83151738: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8315173C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83151740: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83151744: 480104E5  bl 0x83161c28
	ctx.lr = 0x83151748;
	sub_83161C28(ctx, base);
	// 83151748: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8315174C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83151750: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151754: 48010565  bl 0x83161cb8
	ctx.lr = 0x83151758;
	sub_83161CB8(ctx, base);
	// 83151758: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8315175C: 4082FF98  bne 0x831516f4
	if !ctx.cr[0].eq {
	pc = 0x831516F4; continue 'dispatch;
	}
	// 83151760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83151764: 48056A58  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83151768 size=116
    let mut pc: u32 = 0x83151768;
    'dispatch: loop {
        match pc {
            0x83151768 => {
    //   block [0x83151768..0x831517DC)
	// 83151768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315176C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83151770: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83151774: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83151778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315177C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83151780: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83151784: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83151788: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8315178C: 419A000C  beq cr6, 0x83151798
	if ctx.cr[6].eq {
	pc = 0x83151798; continue 'dispatch;
	}
	// 83151790: 48002871  bl 0x83154000
	ctx.lr = 0x83151794;
	sub_83154000(ctx, base);
	// 83151794: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83151798: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8315179C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831517A0: 419A000C  beq cr6, 0x831517ac
	if ctx.cr[6].eq {
	pc = 0x831517AC; continue 'dispatch;
	}
	// 831517A4: 480103A5  bl 0x83161b48
	ctx.lr = 0x831517A8;
	sub_83161B48(ctx, base);
	// 831517A8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 831517AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831517B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831517B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831517B8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831517BC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831517C0: 4E800421  bctrl
	ctx.lr = 0x831517C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831517C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831517C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831517CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831517D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831517D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831517D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831517E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831517E0 size=120
    let mut pc: u32 = 0x831517E0;
    'dispatch: loop {
        match pc {
            0x831517E0 => {
    //   block [0x831517E0..0x83151858)
	// 831517E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831517E4: 48056989  bl 0x831a816c
	ctx.lr = 0x831517E8;
	sub_831A8130(ctx, base);
	// 831517E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831517EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831517F0: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 831517F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831517F8: 4800E4C1  bl 0x8315fcb8
	ctx.lr = 0x831517FC;
	sub_8315FCB8(ctx, base);
	// 831517FC: 3BA30010  addi r29, r3, 0x10
	ctx.r[29].s64 = ctx.r[3].s64 + 16;
	// 83151800: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83151804: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83151808: 409A000C  bne cr6, 0x83151814
	if !ctx.cr[6].eq {
	pc = 0x83151814; continue 'dispatch;
	}
	// 8315180C: 1C7F0088  mulli r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 * 136;
	// 83151810: 48000020  b 0x83151830
	pc = 0x83151830; continue 'dispatch;
	// 83151814: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 83151818: 409A000C  bne cr6, 0x83151824
	if !ctx.cr[6].eq {
	pc = 0x83151824; continue 'dispatch;
	}
	// 8315181C: 57E3502A  slwi r3, r31, 0xa
	ctx.r[3].u32 = ctx.r[31].u32.wrapping_shl(10);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83151820: 48000010  b 0x83151830
	pc = 0x83151830; continue 'dispatch;
	// 83151824: 2F1E0002  cmpwi cr6, r30, 2
	ctx.cr[6].compare_i32(ctx.r[30].s32, 2, &mut ctx.xer);
	// 83151828: 409A0008  bne cr6, 0x83151830
	if !ctx.cr[6].eq {
	pc = 0x83151830; continue 'dispatch;
	}
	// 8315182C: 1C7F012C  mulli r3, r31, 0x12c
	ctx.r[3].s64 = ctx.r[31].s64 * 300;
	// 83151830: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83151834: 38800090  li r4, 0x90
	ctx.r[4].s64 = 144;
	// 83151838: 48010DF1  bl 0x83162628
	ctx.lr = 0x8315183C;
	sub_83162628(ctx, base);
	// 8315183C: 7FA3EA14  add r29, r3, r29
	ctx.r[29].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 83151840: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83151844: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83151848: 48001891  bl 0x831530d8
	ctx.lr = 0x8315184C;
	sub_831530D8(ctx, base);
	// 8315184C: 7C63EA14  add r3, r3, r29
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 83151850: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83151854: 48056968  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83151858 size=360
    let mut pc: u32 = 0x83151858;
    'dispatch: loop {
        match pc {
            0x83151858 => {
    //   block [0x83151858..0x831519C0)
	// 83151858: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315185C: 480568FD  bl 0x831a8158
	ctx.lr = 0x83151860;
	sub_831A8130(ctx, base);
	// 83151860: DBE1FFB0  stfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 83151864: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83151868: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8315186C: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 83151870: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83151874: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83151878: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8315187C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83151880: 38AB5330  addi r5, r11, 0x5330
	ctx.r[5].s64 = ctx.r[11].s64 + 21296;
	// 83151884: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 83151888: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8315188C: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 83151890: 7D384B78  mr r24, r9
	ctx.r[24].u64 = ctx.r[9].u64;
	// 83151894: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 83151898: 4800E461  bl 0x8315fcf8
	ctx.lr = 0x8315189C;
	sub_8315FCF8(ctx, base);
	// 8315189C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831518A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831518A4: 419A0070  beq cr6, 0x83151914
	if ctx.cr[6].eq {
	pc = 0x83151914; continue 'dispatch;
	}
	// 831518A8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831518AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831518B0: 394B5310  addi r10, r11, 0x5310
	ctx.r[10].s64 = ctx.r[11].s64 + 21264;
	// 831518B4: 909F0004  stw r4, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 831518B8: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 831518BC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831518C0: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 831518C4: 909F000C  stw r4, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 831518C8: 409A0070  bne cr6, 0x83151938
	if !ctx.cr[6].eq {
	pc = 0x83151938; continue 'dispatch;
	}
	// 831518CC: 1C9D0088  mulli r4, r29, 0x88
	ctx.r[4].s64 = ctx.r[29].s64 * 136;
	// 831518D0: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 831518D4: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 831518D8: 38A00090  li r5, 0x90
	ctx.r[5].s64 = 144;
	// 831518DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831518E0: 48010D09  bl 0x831625e8
	ctx.lr = 0x831518E4;
	sub_831625E8(ctx, base);
	// 831518E4: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 831518E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831518EC: 409A0070  bne cr6, 0x8315195c
	if !ctx.cr[6].eq {
	pc = 0x8315195C; continue 'dispatch;
	}
	// 831518F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831518F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831518F8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831518FC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83151900: 4E800421  bctrl
	ctx.lr = 0x83151904;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83151904: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83151908: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8315190C: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 83151910: 48056898  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83151914: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83151918: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8315191C: 388B5324  addi r4, r11, 0x5324
	ctx.r[4].s64 = ctx.r[11].s64 + 21284;
	// 83151920: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83151924: 4800E21D  bl 0x8315fb40
	ctx.lr = 0x83151928;
	sub_8315FB40(ctx, base);
	// 83151928: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8315192C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83151930: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 83151934: 48056874  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83151938: 2F1C0001  cmpwi cr6, r28, 1
	ctx.cr[6].compare_i32(ctx.r[28].s32, 1, &mut ctx.xer);
	// 8315193C: 409A000C  bne cr6, 0x83151948
	if !ctx.cr[6].eq {
	pc = 0x83151948; continue 'dispatch;
	}
	// 83151940: 57A4502A  slwi r4, r29, 0xa
	ctx.r[4].u32 = ctx.r[29].u32.wrapping_shl(10);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83151944: 4BFFFF8C  b 0x831518d0
	pc = 0x831518D0; continue 'dispatch;
	// 83151948: 2F1C0002  cmpwi cr6, r28, 2
	ctx.cr[6].compare_i32(ctx.r[28].s32, 2, &mut ctx.xer);
	// 8315194C: 409A000C  bne cr6, 0x83151958
	if !ctx.cr[6].eq {
	pc = 0x83151958; continue 'dispatch;
	}
	// 83151950: 1C9D012C  mulli r4, r29, 0x12c
	ctx.r[4].s64 = ctx.r[29].s64 * 300;
	// 83151954: 4BFFFF7C  b 0x831518d0
	pc = 0x831518D0; continue 'dispatch;
	// 83151958: 909F0008  stw r4, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 8315195C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83151960: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83151964: 7F0AC378  mr r10, r24
	ctx.r[10].u64 = ctx.r[24].u64;
	// 83151968: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 8315196C: 38AB5318  addi r5, r11, 0x5318
	ctx.r[5].s64 = ctx.r[11].s64 + 21272;
	// 83151970: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 83151974: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 83151978: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 8315197C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83151980: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83151984: 48002B6D  bl 0x831544f0
	ctx.lr = 0x83151988;
	sub_831544F0(ctx, base);
	// 83151988: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 8315198C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83151990: 419AFF60  beq cr6, 0x831518f0
	if ctx.cr[6].eq {
	pc = 0x831518F0; continue 'dispatch;
	}
	// 83151994: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83151998: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8315199C: 419A0014  beq cr6, 0x831519b0
	if ctx.cr[6].eq {
	pc = 0x831519B0; continue 'dispatch;
	}
	// 831519A0: 3D608315  lis r11, -0x7ceb
	ctx.r[11].s64 = -2095775744;
	// 831519A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831519A8: 388B16E0  addi r4, r11, 0x16e0
	ctx.r[4].s64 = ctx.r[11].s64 + 5856;
	// 831519AC: 48001935  bl 0x831532e0
	ctx.lr = 0x831519B0;
	sub_831532E0(ctx, base);
	// 831519B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831519B4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 831519B8: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 831519BC: 480567EC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831519C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831519C0 size=76
    let mut pc: u32 = 0x831519C0;
    'dispatch: loop {
        match pc {
            0x831519C0 => {
    //   block [0x831519C0..0x83151A0C)
	// 831519C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831519C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831519C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831519CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831519D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831519D4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831519D8: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 831519DC: 392B5310  addi r9, r11, 0x5310
	ctx.r[9].s64 = ctx.r[11].s64 + 21264;
	// 831519E0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831519E4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831519E8: 419A0010  beq cr6, 0x831519f8
	if ctx.cr[6].eq {
	pc = 0x831519F8; continue 'dispatch;
	}
	// 831519EC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 831519F0: 4800E291  bl 0x8315fc80
	ctx.lr = 0x831519F4;
	sub_8315FC80(ctx, base);
	// 831519F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831519F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831519FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83151A00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83151A04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83151A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83151A10 size=76
    let mut pc: u32 = 0x83151A10;
    'dispatch: loop {
        match pc {
            0x83151A10 => {
    //   block [0x83151A10..0x83151A5C)
	// 83151A10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83151A14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83151A18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83151A1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83151A20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83151A24: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83151A28: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 83151A2C: 392B5350  addi r9, r11, 0x5350
	ctx.r[9].s64 = ctx.r[11].s64 + 21328;
	// 83151A30: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83151A34: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83151A38: 419A0010  beq cr6, 0x83151a48
	if ctx.cr[6].eq {
	pc = 0x83151A48; continue 'dispatch;
	}
	// 83151A3C: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 83151A40: 4800E241  bl 0x8315fc80
	ctx.lr = 0x83151A44;
	sub_8315FC80(ctx, base);
	// 83151A44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83151A48: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83151A4C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83151A50: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83151A54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83151A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83151A60 size=176
    let mut pc: u32 = 0x83151A60;
    'dispatch: loop {
        match pc {
            0x83151A60 => {
    //   block [0x83151A60..0x83151B10)
	// 83151A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83151A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83151A68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83151A6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83151A70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83151A74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83151A78: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83151A7C: 397F0008  addi r11, r31, 8
	ctx.r[11].s64 = ctx.r[31].s64 + 8;
	// 83151A80: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83151A84: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 83151A88: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 83151A8C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83151A90: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83151A94: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83151A98: 4200FFF8  bdnz 0x83151a90
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83151A90; continue 'dispatch;
	}
	// 83151A9C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83151AA0: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 83151AA4: 93DF002C  stw r30, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[30].u32 ) };
	// 83151AA8: 38A0006C  li r5, 0x6c
	ctx.r[5].s64 = 108;
	// 83151AAC: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 83151AB0: 394B535C  addi r10, r11, 0x535c
	ctx.r[10].s64 = ctx.r[11].s64 + 21340;
	// 83151AB4: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 83151AB8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83151ABC: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 83151AC0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83151AC4: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 83151AC8: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 83151ACC: 9BDF0040  stb r30, 0x40(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u8 ) };
	// 83151AD0: 9BDF0041  stb r30, 0x41(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(65 as u32), ctx.r[30].u8 ) };
	// 83151AD4: 93DF0044  stw r30, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[30].u32 ) };
	// 83151AD8: 48056709  bl 0x831a81e0
	ctx.lr = 0x83151ADC;
	sub_831A81E0(ctx, base);
	// 83151ADC: 38A00074  li r5, 0x74
	ctx.r[5].s64 = 116;
	// 83151AE0: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 83151AE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83151AE8: 387F00B8  addi r3, r31, 0xb8
	ctx.r[3].s64 = ctx.r[31].s64 + 184;
	// 83151AEC: 480566F5  bl 0x831a81e0
	ctx.lr = 0x83151AF0;
	sub_831A81E0(ctx, base);
	// 83151AF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83151AF4: 93DF012C  stw r30, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[30].u32 ) };
	// 83151AF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83151AFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83151B00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83151B04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83151B08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83151B0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83151B10 size=936
    let mut pc: u32 = 0x83151B10;
    'dispatch: loop {
        match pc {
            0x83151B10 => {
    //   block [0x83151B10..0x83151EB8)
	// 83151B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83151B14: 48056645  bl 0x831a8158
	ctx.lr = 0x83151B18;
	sub_831A8130(ctx, base);
	// 83151B18: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83151B1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83151B20: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151B24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83151B28: 419A0388  beq cr6, 0x83151eb0
	if ctx.cr[6].eq {
	pc = 0x83151EB0; continue 'dispatch;
	}
	// 83151B2C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83151B30: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 83151B34: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83151B38: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83151B3C: 409A0008  bne cr6, 0x83151b44
	if !ctx.cr[6].eq {
	pc = 0x83151B44; continue 'dispatch;
	}
	// 83151B40: 933F0038  stw r25, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[25].u32 ) };
	// 83151B44: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83151B48: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83151B4C: 409A0314  bne cr6, 0x83151e60
	if !ctx.cr[6].eq {
	pc = 0x83151E60; continue 'dispatch;
	}
	// 83151B50: 817F012C  lwz r11, 0x12c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 83151B54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83151B58: 409A00E0  bne cr6, 0x83151c38
	if !ctx.cr[6].eq {
	pc = 0x83151C38; continue 'dispatch;
	}
	// 83151B5C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83151B60: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151B64: 48010145  bl 0x83161ca8
	ctx.lr = 0x83151B68;
	sub_83161CA8(ctx, base);
	// 83151B68: 38BF00B8  addi r5, r31, 0xb8
	ctx.r[5].s64 = ctx.r[31].s64 + 184;
	// 83151B6C: 80810064  lwz r4, 0x64(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83151B70: 80610060  lwz r3, 0x60(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83151B74: 48011785  bl 0x831632f8
	ctx.lr = 0x83151B78;
	sub_831632F8(ctx, base);
	// 83151B78: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83151B7C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83151B80: 419A00A8  beq cr6, 0x83151c28
	if ctx.cr[6].eq {
	pc = 0x83151C28; continue 'dispatch;
	}
	// 83151B84: 817F00C0  lwz r11, 0xc0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 83151B88: 815F00C4  lwz r10, 0xc4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(196 as u32) ) } as u64;
	// 83151B8C: 7D6907B4  extsw r9, r11
	ctx.r[9].s64 = ctx.r[11].s32 as i64;
	// 83151B90: 891F00BD  lbz r8, 0xbd(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(189 as u32) ) } as u64;
	// 83151B94: 931F012C  stw r24, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[24].u32 ) };
	// 83151B98: F9210068  std r9, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u64 ) };
	// 83151B9C: C8010068  lfd f0, 0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 83151BA0: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 83151BA4: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 83151BA8: FD806818  frsp f12, f13
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	// 83151BAC: D19F0028  stfs f12, 0x28(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 83151BB0: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 83151BB4: 933F0044  stw r25, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[25].u32 ) };
	// 83151BB8: 90FF002C  stw r7, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[7].u32 ) };
	// 83151BBC: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83151BC0: 48011009  bl 0x83162bc8
	ctx.lr = 0x83151BC4;
	sub_83162BC8(ctx, base);
	// 83151BC4: 88DF00BD  lbz r6, 0xbd(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(189 as u32) ) } as u64;
	// 83151BC8: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83151BCC: 7CC40774  extsb r4, r6
	ctx.r[4].s64 = ctx.r[6].s8 as i64;
	// 83151BD0: 48011009  bl 0x83162bd8
	ctx.lr = 0x83151BD4;
	sub_83162BD8(ctx, base);
	// 83151BD4: A09F00CC  lhz r4, 0xcc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 83151BD8: 80BF00C0  lwz r5, 0xc0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 83151BDC: 7C840734  extsh r4, r4
	ctx.r[4].s64 = ctx.r[4].s16 as i64;
	// 83151BE0: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83151BE4: 4801106D  bl 0x83162c50
	ctx.lr = 0x83151BE8;
	sub_83162C50(ctx, base);
	// 83151BE8: 887F00BD  lbz r3, 0xbd(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(189 as u32) ) } as u64;
	// 83151BEC: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 83151BF0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83151BF4: 419A0034  beq cr6, 0x83151c28
	if ctx.cr[6].eq {
	pc = 0x83151C28; continue 'dispatch;
	}
	// 83151BF8: 3BBF00CE  addi r29, r31, 0xce
	ctx.r[29].s64 = ctx.r[31].s64 + 206;
	// 83151BFC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83151C00: A0DD0010  lhz r6, 0x10(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 83151C04: A0BD0000  lhz r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83151C08: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83151C0C: 48010FD5  bl 0x83162be0
	ctx.lr = 0x83151C10;
	sub_83162BE0(ctx, base);
	// 83151C10: 897F00BD  lbz r11, 0xbd(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(189 as u32) ) } as u64;
	// 83151C14: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83151C18: 3BBD0002  addi r29, r29, 2
	ctx.r[29].s64 = ctx.r[29].s64 + 2;
	// 83151C1C: 7D6A0774  extsb r10, r11
	ctx.r[10].s64 = ctx.r[11].s8 as i64;
	// 83151C20: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83151C24: 4198FFD8  blt cr6, 0x83151bfc
	if ctx.cr[6].lt {
	pc = 0x83151BFC; continue 'dispatch;
	}
	// 83151C28: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83151C2C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151C30: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83151C34: 48010085  bl 0x83161cb8
	ctx.lr = 0x83151C38;
	sub_83161CB8(ctx, base);
	// 83151C38: 817F012C  lwz r11, 0x12c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(300 as u32) ) } as u64;
	// 83151C3C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83151C40: 409A01F4  bne cr6, 0x83151e34
	if !ctx.cr[6].eq {
	pc = 0x83151E34; continue 'dispatch;
	}
	// 83151C44: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 83151C48: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83151C4C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151C50: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	// 83151C54: 48010055  bl 0x83161ca8
	ctx.lr = 0x83151C58;
	sub_83161CA8(ctx, base);
	// 83151C58: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83151C5C: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83151C60: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83151C64: 4801162D  bl 0x83163290
	ctx.lr = 0x83151C68;
	sub_83163290(ctx, base);
	// 83151C68: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83151C6C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83151C70: 419A0018  beq cr6, 0x83151c88
	if ctx.cr[6].eq {
	pc = 0x83151C88; continue 'dispatch;
	}
	// 83151C74: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83151C78: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151C7C: 4801003D  bl 0x83161cb8
	ctx.lr = 0x83151C80;
	sub_83161CB8(ctx, base);
	// 83151C80: 933F012C  stw r25, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[25].u32 ) };
	// 83151C84: 4BFFFEB0  b 0x83151b34
	pc = 0x83151B34; continue 'dispatch;
	// 83151C88: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83151C8C: 7F3BCB78  mr r27, r25
	ctx.r[27].u64 = ctx.r[25].u64;
	// 83151C90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83151C94: 40990058  ble cr6, 0x83151cec
	if !ctx.cr[6].gt {
	pc = 0x83151CEC; continue 'dispatch;
	}
	// 83151C98: 3BA10070  addi r29, r1, 0x70
	ctx.r[29].s64 = ctx.r[1].s64 + 112;
	// 83151C9C: 3BC10090  addi r30, r1, 0x90
	ctx.r[30].s64 = ctx.r[1].s64 + 144;
	// 83151CA0: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 83151CA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83151CA8: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83151CAC: 4800FF6D  bl 0x83161c18
	ctx.lr = 0x83151CB0;
	sub_83161C18(ctx, base);
	// 83151CB0: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83151CB4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83151CB8: 419A0110  beq cr6, 0x83151dc8
	if ctx.cr[6].eq {
	pc = 0x83151DC8; continue 'dispatch;
	}
	// 83151CBC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151CC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83151CC4: 419A0104  beq cr6, 0x83151dc8
	if ctx.cr[6].eq {
	pc = 0x83151DC8; continue 'dispatch;
	}
	// 83151CC8: 813F002C  lwz r9, 0x2c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83151CCC: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 83151CD0: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83151CD4: 557AF0BE  srwi r26, r11, 2
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 83151CD8: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 83151CDC: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83151CE0: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 83151CE4: 7F1B4840  cmplw cr6, r27, r9
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83151CE8: 4198FFBC  blt cr6, 0x83151ca4
	if ctx.cr[6].lt {
	pc = 0x83151CA4; continue 'dispatch;
	}
	// 83151CEC: 80DF0044  lwz r6, 0x44(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83151CF0: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 83151CF4: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83151CF8: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 83151CFC: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 83151D00: 811F002C  lwz r8, 0x2c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83151D04: 7FA65850  subf r29, r6, r11
	ctx.r[29].s64 = ctx.r[11].s64 - ctx.r[6].s64;
	// 83151D08: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83151D0C: 80C1005C  lwz r6, 0x5c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83151D10: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83151D14: 80A10058  lwz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83151D18: 48010949  bl 0x83162660
	ctx.lr = 0x83151D1C;
	sub_83162660(ctx, base);
	// 83151D1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83151D20: 7F1EE840  cmplw cr6, r30, r29
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[29].u32, &mut ctx.xer);
	// 83151D24: 40990008  ble cr6, 0x83151d2c
	if !ctx.cr[6].gt {
	pc = 0x83151D2C; continue 'dispatch;
	}
	// 83151D28: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 83151D2C: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83151D30: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83151D34: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83151D38: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83151D3C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151D40: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83151D44: 4800FF75  bl 0x83161cb8
	ctx.lr = 0x83151D48;
	sub_83161CB8(ctx, base);
	// 83151D48: 815F002C  lwz r10, 0x2c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83151D4C: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 83151D50: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83151D54: 40990038  ble cr6, 0x83151d8c
	if !ctx.cr[6].gt {
	pc = 0x83151D8C; continue 'dispatch;
	}
	// 83151D58: 57DB103A  slwi r27, r30, 2
	ctx.r[27].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 83151D5C: 3BA10090  addi r29, r1, 0x90
	ctx.r[29].s64 = ctx.r[1].s64 + 144;
	// 83151D60: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 83151D64: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83151D68: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83151D6C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83151D70: 4800FEB9  bl 0x83161c28
	ctx.lr = 0x83151D74;
	sub_83161C28(ctx, base);
	// 83151D74: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83151D78: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83151D7C: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83151D80: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 83151D84: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83151D88: 4198FFDC  blt cr6, 0x83151d64
	if ctx.cr[6].lt {
	pc = 0x83151D64; continue 'dispatch;
	}
	// 83151D8C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83151D90: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151D94: 4800FF15  bl 0x83161ca8
	ctx.lr = 0x83151D98;
	sub_83161CA8(ctx, base);
	// 83151D98: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83151D9C: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83151DA0: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83151DA4: 480114ED  bl 0x83163290
	ctx.lr = 0x83151DA8;
	sub_83163290(ctx, base);
	// 83151DA8: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83151DAC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151DB0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83151DB4: 419A002C  beq cr6, 0x83151de0
	if ctx.cr[6].eq {
	pc = 0x83151DE0; continue 'dispatch;
	}
	// 83151DB8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83151DBC: 4800FEFD  bl 0x83161cb8
	ctx.lr = 0x83151DC0;
	sub_83161CB8(ctx, base);
	// 83151DC0: 933F012C  stw r25, 0x12c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(300 as u32), ctx.r[25].u32 ) };
	// 83151DC4: 4BFFFD70  b 0x83151b34
	pc = 0x83151B34; continue 'dispatch;
	// 83151DC8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83151DCC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151DD0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83151DD4: 4800FEE5  bl 0x83161cb8
	ctx.lr = 0x83151DD8;
	sub_83161CB8(ctx, base);
	// 83151DD8: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 83151DDC: 480563CC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83151DE0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83151DE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83151DE8: 4800FDA1  bl 0x83161b88
	ctx.lr = 0x83151DEC;
	sub_83161B88(ctx, base);
	// 83151DEC: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83151DF0: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83151DF4: 7D2A5851  subf. r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83151DF8: 4181003C  bgt 0x83151e34
	if ctx.cr[0].gt {
	pc = 0x83151E34; continue 'dispatch;
	}
	// 83151DFC: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83151E00: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151E04: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83151E08: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83151E0C: 4800FD6D  bl 0x83161b78
	ctx.lr = 0x83151E10;
	sub_83161B78(ctx, base);
	// 83151E10: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83151E14: 2B0B0012  cmplwi cr6, r11, 0x12
	ctx.cr[6].compare_u32(ctx.r[11].u32, 18 as u32, &mut ctx.xer);
	// 83151E18: 4199000C  bgt cr6, 0x83151e24
	if ctx.cr[6].gt {
	pc = 0x83151E24; continue 'dispatch;
	}
	// 83151E1C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83151E20: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83151E24: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83151E28: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151E2C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83151E30: 4800FD59  bl 0x83161b88
	ctx.lr = 0x83151E34;
	sub_83161B88(ctx, base);
	// 83151E34: 897F0041  lbz r11, 0x41(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(65 as u32) ) } as u64;
	// 83151E38: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83151E3C: 409A0074  bne cr6, 0x83151eb0
	if !ctx.cr[6].eq {
	pc = 0x83151EB0; continue 'dispatch;
	}
	// 83151E40: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151E44: 4800FE5D  bl 0x83161ca0
	ctx.lr = 0x83151E48;
	sub_83161CA0(ctx, base);
	// 83151E48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83151E4C: 409A0064  bne cr6, 0x83151eb0
	if !ctx.cr[6].eq {
	pc = 0x83151EB0; continue 'dispatch;
	}
	// 83151E50: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83151E54: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83151E58: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 83151E5C: 4805634C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83151E60: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83151E64: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83151E68: 409A0008  bne cr6, 0x83151e70
	if !ctx.cr[6].eq {
	pc = 0x83151E70; continue 'dispatch;
	}
	// 83151E6C: 933F0038  stw r25, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[25].u32 ) };
	// 83151E70: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83151E74: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83151E78: 409A0038  bne cr6, 0x83151eb0
	if !ctx.cr[6].eq {
	pc = 0x83151EB0; continue 'dispatch;
	}
	// 83151E7C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 83151E80: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151E84: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83151E88: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83151E8C: 4800FCED  bl 0x83161b78
	ctx.lr = 0x83151E90;
	sub_83161B78(ctx, base);
	// 83151E90: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83151E94: 2B0B0012  cmplwi cr6, r11, 0x12
	ctx.cr[6].compare_u32(ctx.r[11].u32, 18 as u32, &mut ctx.xer);
	// 83151E98: 40990008  ble cr6, 0x83151ea0
	if !ctx.cr[6].gt {
	pc = 0x83151EA0; continue 'dispatch;
	}
	// 83151E9C: 931F0038  stw r24, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[24].u32 ) };
	// 83151EA0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83151EA4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83151EA8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83151EAC: 4800FCDD  bl 0x83161b88
	ctx.lr = 0x83151EB0;
	sub_83161B88(ctx, base);
	// 83151EB0: 38210120  addi r1, r1, 0x120
	ctx.r[1].s64 = ctx.r[1].s64 + 288;
	// 83151EB4: 480562F4  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83151EB8 size=76
    let mut pc: u32 = 0x83151EB8;
    'dispatch: loop {
        match pc {
            0x83151EB8 => {
    //   block [0x83151EB8..0x83151F04)
	// 83151EB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83151EBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83151EC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83151EC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83151EC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83151ECC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83151ED0: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 83151ED4: 392B5350  addi r9, r11, 0x5350
	ctx.r[9].s64 = ctx.r[11].s64 + 21328;
	// 83151ED8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83151EDC: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83151EE0: 419A0010  beq cr6, 0x83151ef0
	if ctx.cr[6].eq {
	pc = 0x83151EF0; continue 'dispatch;
	}
	// 83151EE4: 38800130  li r4, 0x130
	ctx.r[4].s64 = 304;
	// 83151EE8: 4800DD99  bl 0x8315fc80
	ctx.lr = 0x83151EEC;
	sub_8315FC80(ctx, base);
	// 83151EEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83151EF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83151EF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83151EF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83151EFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83151F00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83151F08 size=168
    let mut pc: u32 = 0x83151F08;
    'dispatch: loop {
        match pc {
            0x83151F08 => {
    //   block [0x83151F08..0x83151FB0)
	// 83151F08: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 83151F0C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83151F10: 2B05002C  cmplwi cr6, r5, 0x2c
	ctx.cr[6].compare_u32(ctx.r[5].u32, 44 as u32, &mut ctx.xer);
	// 83151F14: 4098000C  bge cr6, 0x83151f20
	if !ctx.cr[6].lt {
	pc = 0x83151F20; continue 'dispatch;
	}
	// 83151F18: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83151F1C: 4E800020  blr
	return;
	// 83151F20: 8944001B  lbz r10, 0x1b(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(27 as u32) ) } as u64;
	// 83151F24: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83151F28: 8924001A  lbz r9, 0x1a(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(26 as u32) ) } as u64;
	// 83151F2C: 5548403E  rotlwi r8, r10, 8
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 83151F30: 8944002B  lbz r10, 0x2b(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(43 as u32) ) } as u64;
	// 83151F34: 88E40019  lbz r7, 0x19(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(25 as u32) ) } as u64;
	// 83151F38: 7D054B78  or r5, r8, r9
	ctx.r[5].u64 = ctx.r[8].u64 | ctx.r[9].u64;
	// 83151F3C: 8924002A  lbz r9, 0x2a(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(42 as u32) ) } as u64;
	// 83151F40: 554A403E  rotlwi r10, r10, 8
	ctx.r[10].u64 = ((ctx.r[10].u32).rotate_left(8)) as u64;
	// 83151F44: 88C40018  lbz r6, 0x18(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 83151F48: 54A8402E  slwi r8, r5, 8
	ctx.r[8].u32 = ctx.r[5].u32.wrapping_shl(8);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83151F4C: 88A40029  lbz r5, 0x29(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(41 as u32) ) } as u64;
	// 83151F50: 8BE40028  lbz r31, 0x28(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 83151F54: 7D083B78  or r8, r8, r7
	ctx.r[8].u64 = ctx.r[8].u64 | ctx.r[7].u64;
	// 83151F58: 7D474B78  or r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 83151F5C: 550A402E  slwi r10, r8, 8
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83151F60: 54E9402E  slwi r9, r7, 8
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(8);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83151F64: 7D483378  or r8, r10, r6
	ctx.r[8].u64 = ctx.r[10].u64 | ctx.r[6].u64;
	// 83151F68: 7D272B78  or r7, r9, r5
	ctx.r[7].u64 = ctx.r[9].u64 | ctx.r[5].u64;
	// 83151F6C: 7D0607B4  extsw r6, r8
	ctx.r[6].s64 = ctx.r[8].s32 as i64;
	// 83151F70: 54E5402E  slwi r5, r7, 8
	ctx.r[5].u32 = ctx.r[7].u32.wrapping_shl(8);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83151F74: F8C1FFF0  std r6, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[6].u64 ) };
	// 83151F78: C801FFF0  lfd f0, -0x10(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83151F7C: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 83151F80: 7CAAFB78  or r10, r5, r31
	ctx.r[10].u64 = ctx.r[5].u64 | ctx.r[31].u64;
	// 83151F84: FD806818  frsp f12, f13
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	// 83151F88: D18B0028  stfs f12, 0x28(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 83151F8C: 89240016  lbz r9, 0x16(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(22 as u32) ) } as u64;
	// 83151F90: 7D280774  extsb r8, r9
	ctx.r[8].s64 = ctx.r[9].s8 as i64;
	// 83151F94: 910B002C  stw r8, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[8].u32 ) };
	// 83151F98: 88E40020  lbz r7, 0x20(r4)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 83151F9C: 7CE60774  extsb r6, r7
	ctx.r[6].s64 = ctx.r[7].s8 as i64;
	// 83151FA0: 7CAA33D6  divw r5, r10, r6
	ctx.r[5].s32 = ctx.r[10].s32 / ctx.r[6].s32;
	// 83151FA4: 90AB0030  stw r5, 0x30(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(48 as u32), ctx.r[5].u32 ) };
	// 83151FA8: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 83151FAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83151FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83151FB0 size=236
    let mut pc: u32 = 0x83151FB0;
    'dispatch: loop {
        match pc {
            0x83151FB0 => {
    //   block [0x83151FB0..0x8315209C)
	// 83151FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83151FB4: 480561B9  bl 0x831a816c
	ctx.lr = 0x83151FB8;
	sub_831A8130(ctx, base);
	// 83151FB8: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83151FBC: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83151FC0: 7FC65396  divwu r30, r6, r10
	ctx.r[30].u32 = ctx.r[6].u32 / ctx.r[10].u32;
	// 83151FC4: 7F1E4840  cmplw cr6, r30, r9
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83151FC8: 41980008  blt cr6, 0x83151fd0
	if ctx.cr[6].lt {
	pc = 0x83151FD0; continue 'dispatch;
	}
	// 83151FCC: 7D3E4B78  mr r30, r9
	ctx.r[30].u64 = ctx.r[9].u64;
	// 83151FD0: 7F1E2040  cmplw cr6, r30, r4
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[4].u32, &mut ctx.xer);
	// 83151FD4: 41980008  blt cr6, 0x83151fdc
	if ctx.cr[6].lt {
	pc = 0x83151FDC; continue 'dispatch;
	}
	// 83151FD8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83151FDC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83151FE0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83151FE4: 419A00A0  beq cr6, 0x83152084
	if ctx.cr[6].eq {
	pc = 0x83152084; continue 'dispatch;
	}
	// 83151FE8: 3D40820C  lis r10, -0x7df4
	ctx.r[10].s64 = -2113142784;
	// 83151FEC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 83151FF0: C1AA7490  lfs f13, 0x7490(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(29840 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83151FF4: C0094E28  lfs f0, 0x4e28(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20008 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83151FF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83151FFC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83152000: 419A0078  beq cr6, 0x83152078
	if ctx.cr[6].eq {
	pc = 0x83152078; continue 'dispatch;
	}
	// 83152004: 57E4103A  slwi r4, r31, 2
	ctx.r[4].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 83152008: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 8315200C: 89650001  lbz r11, 1(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(1 as u32) ) } as u64;
	// 83152010: 8BA50000  lbz r29, 0(r5)
	ctx.r[29].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152014: 556B403E  rotlwi r11, r11, 8
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(8)) as u64;
	// 83152018: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8315201C: 7D6BEB78  or r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 | ctx.r[29].u64;
	// 83152020: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 83152024: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83152028: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8315202C: 4198001C  blt cr6, 0x83152048
	if ctx.cr[6].lt {
	pc = 0x83152048; continue 'dispatch;
	}
	// 83152030: F961FFD0  std r11, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.r[11].u64 ) };
	// 83152034: C981FFD0  lfd f12, -0x30(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 83152038: FD60669C  fcfid f11, f12
	ctx.f[11].f64 = (ctx.f[12].s64 as f64);
	// 8315203C: FD405818  frsp f10, f11
	ctx.f[10].f64 = (ctx.f[11].f64 as f32) as f64;
	// 83152040: ED2A0032  fmuls f9, f10, f0
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 83152044: 48000018  b 0x8315205c
	pc = 0x8315205C; continue 'dispatch;
	// 83152048: F961FFD8  std r11, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.r[11].u64 ) };
	// 8315204C: C981FFD8  lfd f12, -0x28(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 83152050: FD60669C  fcfid f11, f12
	ctx.f[11].f64 = (ctx.f[12].s64 as f64);
	// 83152054: FD405818  frsp f10, f11
	ctx.f[10].f64 = (ctx.f[11].f64 as f32) as f64;
	// 83152058: ED2A0372  fmuls f9, f10, f13
	ctx.f[9].f64 = (((ctx.f[10].f64 * ctx.f[13].f64) as f32) as f64);
	// 8315205C: 7D29252E  stfsx f9, r9, r4
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 83152060: 38C60001  addi r6, r6, 1
	ctx.r[6].s64 = ctx.r[6].s64 + 1;
	// 83152064: 38A50002  addi r5, r5, 2
	ctx.r[5].s64 = ctx.r[5].s64 + 2;
	// 83152068: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8315206C: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83152070: 7F065840  cmplw cr6, r6, r11
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83152074: 4198FF98  blt cr6, 0x8315200c
	if ctx.cr[6].lt {
	pc = 0x8315200C; continue 'dispatch;
	}
	// 83152078: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 8315207C: 7F1FF040  cmplw cr6, r31, r30
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[30].u32, &mut ctx.xer);
	// 83152080: 4198FF78  blt cr6, 0x83151ff8
	if ctx.cr[6].lt {
	pc = 0x83151FF8; continue 'dispatch;
	}
	// 83152084: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83152088: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8315208C: 7D5F59D6  mullw r10, r31, r11
	ctx.r[10].s64 = (ctx.r[31].s32 as i64) * (ctx.r[11].s32 as i64);
	// 83152090: 5549083C  slwi r9, r10, 1
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(1);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83152094: 91270000  stw r9, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83152098: 48056124  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831520A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831520A0 size=204
    let mut pc: u32 = 0x831520A0;
    'dispatch: loop {
        match pc {
            0x831520A0 => {
    //   block [0x831520A0..0x8315216C)
	// 831520A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831520A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831520A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831520AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831520B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831520B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831520B8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831520BC: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 831520C0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831520C4: 392B5374  addi r9, r11, 0x5374
	ctx.r[9].s64 = ctx.r[11].s64 + 21364;
	// 831520C8: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 831520CC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 831520D0: 909F0018  stw r4, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 831520D4: 38A00060  li r5, 0x60
	ctx.r[5].s64 = 96;
	// 831520D8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 831520DC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831520E0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831520E4: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 831520E8: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 831520EC: 93DF0014  stw r30, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 831520F0: C00A7F6C  lfs f0, 0x7f6c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32620 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831520F4: D01F001C  stfs f0, 0x1c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 831520F8: 93DF0020  stw r30, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[30].u32 ) };
	// 831520FC: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 83152100: 480560E1  bl 0x831a81e0
	ctx.lr = 0x83152104;
	sub_831A81E0(ctx, base);
	// 83152104: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 83152108: 38A00060  li r5, 0x60
	ctx.r[5].s64 = 96;
	// 8315210C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83152110: 480560D1  bl 0x831a81e0
	ctx.lr = 0x83152114;
	sub_831A81E0(ctx, base);
	// 83152114: 3D008334  lis r8, -0x7ccc
	ctx.r[8].s64 = -2093744128;
	// 83152118: 93DF00E8  stw r30, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[30].u32 ) };
	// 8315211C: 3CE08334  lis r7, -0x7ccc
	ctx.r[7].s64 = -2093744128;
	// 83152120: 93DF00EC  stw r30, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[30].u32 ) };
	// 83152124: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83152128: 93DF00F0  stw r30, 0xf0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(240 as u32), ctx.r[30].u32 ) };
	// 8315212C: 93DF00F4  stw r30, 0xf4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(244 as u32), ctx.r[30].u32 ) };
	// 83152130: 93DF00F8  stw r30, 0xf8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(248 as u32), ctx.r[30].u32 ) };
	// 83152134: 93DF00FC  stw r30, 0xfc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(252 as u32), ctx.r[30].u32 ) };
	// 83152138: C0085FF0  lfs f0, 0x5ff0(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(24560 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8315213C: D01F0100  stfs f0, 0x100(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(256 as u32), tmp.u32 ) };
	// 83152140: C0075FEC  lfs f0, 0x5fec(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24556 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83152144: D01F0104  stfs f0, 0x104(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(260 as u32), tmp.u32 ) };
	// 83152148: C0075FEC  lfs f0, 0x5fec(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(24556 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8315214C: 93DF010C  stw r30, 0x10c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(268 as u32), ctx.r[30].u32 ) };
	// 83152150: D01F0108  stfs f0, 0x108(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(264 as u32), tmp.u32 ) };
	// 83152154: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83152158: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8315215C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83152160: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83152164: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83152168: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83152170 size=96
    let mut pc: u32 = 0x83152170;
    'dispatch: loop {
        match pc {
            0x83152170 => {
    //   block [0x83152170..0x831521D0)
	// 83152170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83152174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83152178: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8315217C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83152180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83152184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83152188: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8315218C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83152190: 394B5374  addi r10, r11, 0x5374
	ctx.r[10].s64 = ctx.r[11].s64 + 21364;
	// 83152194: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83152198: 4BFF4011  bl 0x831461a8
	ctx.lr = 0x8315219C;
	sub_831461A8(ctx, base);
	// 8315219C: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 831521A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831521A4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 831521A8: 419A0010  beq cr6, 0x831521b8
	if ctx.cr[6].eq {
	pc = 0x831521B8; continue 'dispatch;
	}
	// 831521AC: 38800110  li r4, 0x110
	ctx.r[4].s64 = 272;
	// 831521B0: 4800DAD1  bl 0x8315fc80
	ctx.lr = 0x831521B4;
	sub_8315FC80(ctx, base);
	// 831521B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831521B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831521BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831521C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831521C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831521C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831521CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831521D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831521D0 size=108
    let mut pc: u32 = 0x831521D0;
    'dispatch: loop {
        match pc {
            0x831521D0 => {
    //   block [0x831521D0..0x8315223C)
	// 831521D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831521D4: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 831521D8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 831521DC: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 831521E0: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 831521E4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831521E8: 38E8537C  addi r7, r8, 0x537c
	ctx.r[7].s64 = ctx.r[8].s64 + 21372;
	// 831521EC: C1AB08A8  lfs f13, 0x8a8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831521F0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 831521F4: C00908A4  lfs f0, 0x8a4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831521F8: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 831521FC: D1A3001C  stfs f13, 0x1c(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 83152200: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 83152204: D0030020  stfs f0, 0x20(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 83152208: 39630044  addi r11, r3, 0x44
	ctx.r[11].s64 = ctx.r[3].s64 + 68;
	// 8315220C: 91430014  stw r10, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 83152210: 39230064  addi r9, r3, 0x64
	ctx.r[9].s64 = ctx.r[3].s64 + 100;
	// 83152214: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 83152218: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8315221C: D00BFFE0  stfs f0, -0x20(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 83152220: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 83152224: 7D0951AE  stbx r8, r9, r10
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[8].u8) };
	// 83152228: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8315222C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83152230: 2B0A0008  cmplwi cr6, r10, 8
	ctx.cr[6].compare_u32(ctx.r[10].u32, 8 as u32, &mut ctx.xer);
	// 83152234: 4198FFE8  blt cr6, 0x8315221c
	if ctx.cr[6].lt {
	pc = 0x8315221C; continue 'dispatch;
	}
	// 83152238: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83152240 size=120
    let mut pc: u32 = 0x83152240;
    'dispatch: loop {
        match pc {
            0x83152240 => {
    //   block [0x83152240..0x831522B8)
	// 83152240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83152244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83152248: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8315224C: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 83152250: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83152254: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83152258: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8315225C: C3EBB184  lfs f31, -0x4e7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 83152260: D03F001C  stfs f1, 0x1c(r31)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 83152264: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 83152268: 41990010  bgt cr6, 0x83152278
	if ctx.cr[6].gt {
	pc = 0x83152278; continue 'dispatch;
	}
	// 8315226C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83152270: 388B53B0  addi r4, r11, 0x53b0
	ctx.r[4].s64 = ctx.r[11].s64 + 21424;
	// 83152274: 4800001C  b 0x83152290
	pc = 0x83152290; continue 'dispatch;
	// 83152278: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8315227C: C3EB4E60  lfs f31, 0x4e60(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20064 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 83152280: FF01F800  fcmpu cr6, f1, f31
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 83152284: 4099001C  ble cr6, 0x831522a0
	if !ctx.cr[6].gt {
	pc = 0x831522A0; continue 'dispatch;
	}
	// 83152288: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8315228C: 388B5384  addi r4, r11, 0x5384
	ctx.r[4].s64 = ctx.r[11].s64 + 21380;
	// 83152290: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83152294: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83152298: 4800D891  bl 0x8315fb28
	ctx.lr = 0x8315229C;
	sub_8315FB28(ctx, base);
	// 8315229C: D3FF001C  stfs f31, 0x1c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 831522A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831522A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831522A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831522AC: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831522B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831522B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831522B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831522B8 size=884
    let mut pc: u32 = 0x831522B8;
    'dispatch: loop {
        match pc {
            0x831522B8 => {
    //   block [0x831522B8..0x8315262C)
	// 831522B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831522BC: 48055E89  bl 0x831a8144
	ctx.lr = 0x831522C0;
	sub_831A8130(ctx, base);
	// 831522C0: 3981FF90  addi r12, r1, -0x70
	ctx.r[12].s64 = ctx.r[1].s64 + -112;
	// 831522C4: 480567B5  bl 0x831a8a78
	ctx.lr = 0x831522C8;
	sub_831A8A40(ctx, base);
	// 831522C8: E981F000  ld r12, -0x1000(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-4096 as u32) ) };
	// 831522CC: 9421EE80  stwu r1, -0x1180(r1)
	ea = ctx.r[1].u32.wrapping_add(-4480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831522D0: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 831522D4: D02100BC  stfs f1, 0xbc(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), tmp.u32 ) };
	// 831522D8: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 831522DC: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 831522E0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 831522E4: 916100B0  stw r11, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 831522E8: 7C962378  mr r22, r4
	ctx.r[22].u64 = ctx.r[4].u64;
	// 831522EC: 92E100B4  stw r23, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[23].u32 ) };
	// 831522F0: C3B8001C  lfs f29, 0x1c(r24)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(28 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 831522F4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 831522F8: C3F80020  lfs f31, 0x20(r24)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(32 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 831522FC: 7D1D4378  mr r29, r8
	ctx.r[29].u64 = ctx.r[8].u64;
	// 83152300: 93E100B8  stw r31, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[31].u32 ) };
	// 83152304: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 83152308: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 8315230C: 394100D0  addi r10, r1, 0xd0
	ctx.r[10].s64 = ctx.r[1].s64 + 208;
	// 83152310: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83152314: 91490000  stw r10, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83152318: 810100B0  lwz r8, 0xb0(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 8315231C: 394A0204  addi r10, r10, 0x204
	ctx.r[10].s64 = ctx.r[10].s64 + 516;
	// 83152320: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 83152324: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83152328: 4198FFE8  blt cr6, 0x83152310
	if ctx.cr[6].lt {
	pc = 0x83152310; continue 'dispatch;
	}
	// 8315232C: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 83152330: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152334: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83152338: 419A0018  beq cr6, 0x83152350
	if ctx.cr[6].eq {
	pc = 0x83152350; continue 'dispatch;
	}
	// 8315233C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152340: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 83152344: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152348: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8315234C: 4E800421  bctrl
	ctx.lr = 0x83152350;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83152350: 816100B4  lwz r11, 0xb4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 83152354: 814100B8  lwz r10, 0xb8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 83152358: 7D735B78  mr r19, r11
	ctx.r[19].u64 = ctx.r[11].u64;
	// 8315235C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83152360: 4098004C  bge cr6, 0x831523ac
	if !ctx.cr[6].lt {
	pc = 0x831523AC; continue 'dispatch;
	}
	// 83152364: 7EC3B378  mr r3, r22
	ctx.r[3].u64 = ctx.r[22].u64;
	// 83152368: 4BFEA229  bl 0x8313c590
	ctx.lr = 0x8315236C;
	sub_8313C590(ctx, base);
	// 8315236C: 814100B0  lwz r10, 0xb0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 83152370: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 83152374: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83152378: 419A0020  beq cr6, 0x83152398
	if ctx.cr[6].eq {
	pc = 0x83152398; continue 'dispatch;
	}
	// 8315237C: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 83152380: 92EA0000  stw r23, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 83152384: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83152388: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 8315238C: 812100B0  lwz r9, 0xb0(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 83152390: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83152394: 4198FFEC  blt cr6, 0x83152380
	if ctx.cr[6].lt {
	pc = 0x83152380; continue 'dispatch;
	}
	// 83152398: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8315239C: 38211180  addi r1, r1, 0x1180
	ctx.r[1].s64 = ctx.r[1].s64 + 4480;
	// 831523A0: 3981FF90  addi r12, r1, -0x70
	ctx.r[12].s64 = ctx.r[1].s64 + -112;
	// 831523A4: 48056721  bl 0x831a8ac4
	ctx.lr = 0x831523A8;
	sub_831A8A8C(ctx, base);
	// 831523A8: 48055DEC  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
	// 831523AC: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 831523B0: 7EFFBB78  mr r31, r23
	ctx.r[31].u64 = ctx.r[23].u64;
	// 831523B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831523B8: 419A0230  beq cr6, 0x831525e8
	if ctx.cr[6].eq {
	pc = 0x831525E8; continue 'dispatch;
	}
	// 831523BC: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 831523C0: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 831523C4: 7EB65850  subf r21, r22, r11
	ctx.r[21].s64 = ctx.r[11].s64 - ctx.r[22].s64;
	// 831523C8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831523CC: 3B360050  addi r25, r22, 0x50
	ctx.r[25].s64 = ctx.r[22].s64 + 80;
	// 831523D0: 7EDBB378  mr r27, r22
	ctx.r[27].u64 = ctx.r[22].u64;
	// 831523D4: C38A08A4  lfs f28, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 831523D8: 3B980044  addi r28, r24, 0x44
	ctx.r[28].s64 = ctx.r[24].s64 + 68;
	// 831523DC: 3B580064  addi r26, r24, 0x64
	ctx.r[26].s64 = ctx.r[24].s64 + 100;
	// 831523E0: C3CB08A8  lfs f30, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 831523E4: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 831523E8: 7D75C8AE  lbzx r11, r21, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 831523EC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831523F0: 419A014C  beq cr6, 0x8315253c
	if ctx.cr[6].eq {
	pc = 0x8315253C; continue 'dispatch;
	}
	// 831523F4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831523F8: 419A0030  beq cr6, 0x83152428
	if ctx.cr[6].eq {
	pc = 0x83152428; continue 'dispatch;
	}
	// 831523FC: 89790000  lbz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152400: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83152404: 409A0024  bne cr6, 0x83152428
	if !ctx.cr[6].eq {
	pc = 0x83152428; continue 'dispatch;
	}
	// 83152408: 81760040  lwz r11, 0x40(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(64 as u32) ) } as u64;
	// 8315240C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83152410: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83152414: 40980008  bge cr6, 0x8315241c
	if !ctx.cr[6].lt {
	pc = 0x8315241C; continue 'dispatch;
	}
	// 83152418: 807B0000  lwz r3, 0(r27)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8315241C: 57A5103A  slwi r5, r29, 2
	ctx.r[5].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 83152420: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83152424: 48055DBD  bl 0x831a81e0
	ctx.lr = 0x83152428;
	sub_831A81E0(ctx, base);
	// 83152428: 81760040  lwz r11, 0x40(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(64 as u32) ) } as u64;
	// 8315242C: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 83152430: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83152434: 40980008  bge cr6, 0x8315243c
	if !ctx.cr[6].lt {
	pc = 0x8315243C; continue 'dispatch;
	}
	// 83152438: 80DB0000  lwz r6, 0(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8315243C: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 83152440: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83152444: 41980010  blt cr6, 0x83152454
	if ctx.cr[6].lt {
	pc = 0x83152454; continue 'dispatch;
	}
	// 83152448: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 8315244C: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 83152450: 4800000C  b 0x8315245c
	pc = 0x8315245C; continue 'dispatch;
	// 83152454: 7D75D82E  lwzx r11, r21, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 83152458: 80E100B8  lwz r7, 0xb8(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 8315245C: 7D3AF8AE  lbzx r9, r26, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83152460: C3F80020  lfs f31, 0x20(r24)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(32 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 83152464: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 83152468: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 8315246C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83152470: 419A001C  beq cr6, 0x8315248c
	if ctx.cr[6].eq {
	pc = 0x8315248C; continue 'dispatch;
	}
	// 83152474: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83152478: FDA0E090  fmr f13, f28
	ctx.f[13].f64 = ctx.f[28].f64;
	// 8315247C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83152480: 7EFAF9AE  stbx r23, r26, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32), ctx.r[23].u8) };
	// 83152484: 7E8AA378  mr r10, r20
	ctx.r[10].u64 = ctx.r[20].u64;
	// 83152488: 4800000C  b 0x83152494
	pc = 0x83152494; continue 'dispatch;
	// 8315248C: C1BCFFE0  lfs f13, -0x20(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83152490: C01C0000  lfs f0, 0(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83152494: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83152498: 419A0060  beq cr6, 0x831524f8
	if ctx.cr[6].eq {
	pc = 0x831524F8; continue 'dispatch;
	}
	// 8315249C: 57A9103A  slwi r9, r29, 2
	ctx.r[9].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 831524A0: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 831524A4: 7D293214  add r9, r9, r6
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[6].u64;
	// 831524A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831524AC: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 831524B0: 41980028  blt cr6, 0x831524d8
	if ctx.cr[6].lt {
	pc = 0x831524D8; continue 'dispatch;
	}
	// 831524B4: 7F0A3840  cmplw cr6, r10, r7
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[7].u32, &mut ctx.xer);
	// 831524B8: 40980020  bge cr6, 0x831524d8
	if !ctx.cr[6].lt {
	pc = 0x831524D8; continue 'dispatch;
	}
	// 831524BC: EFFFF028  fsubs f31, f31, f30
	ctx.f[31].f64 = (((ctx.f[31].f64 - ctx.f[30].f64) as f32) as f64);
	// 831524C0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 831524C4: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 831524C8: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831524CC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 831524D0: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 831524D4: 4098FFE0  bge cr6, 0x831524b4
	if !ctx.cr[6].lt {
	pc = 0x831524B4; continue 'dispatch;
	}
	// 831524D8: ED9EF828  fsubs f12, f30, f31
	ctx.f[12].f64 = (((ctx.f[30].f64 - ctx.f[31].f64) as f32) as f64);
	// 831524DC: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831524E0: ED6007F2  fmuls f11, f0, f31
	ctx.f[11].f64 = (((ctx.f[0].f64 * ctx.f[31].f64) as f32) as f64);
	// 831524E4: EFFFE82A  fadds f31, f31, f29
	ctx.f[31].f64 = ((ctx.f[31].f64 + ctx.f[29].f64) as f32) as f64;
	// 831524E8: ED4C5B7A  fmadds f10, f12, f13, f11
	ctx.f[10].f64 = (((ctx.f[12].f64 * ctx.f[13].f64 + ctx.f[11].f64) as f32) as f64);
	// 831524EC: D1490000  stfs f10, 0(r9)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831524F0: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 831524F4: 4082FFB8  bne 0x831524ac
	if !ctx.cr[0].eq {
	pc = 0x831524AC; continue 'dispatch;
	}
	// 831524F8: D1BCFFE0  stfs f13, -0x20(r28)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 831524FC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83152500: D01C0000  stfs f0, 0(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 83152504: 9A990000  stb r20, 0(r25)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[20].u8 ) };
	// 83152508: 409A00C8  bne cr6, 0x831525d0
	if !ctx.cr[6].eq {
	pc = 0x831525D0; continue 'dispatch;
	}
	// 8315250C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83152510: 419A00C0  beq cr6, 0x831525d0
	if ctx.cr[6].eq {
	pc = 0x831525D0; continue 'dispatch;
	}
	// 83152514: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83152518: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8315251C: 7C8B3214  add r4, r11, r6
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 83152520: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 83152524: 38640004  addi r3, r4, 4
	ctx.r[3].s64 = ctx.r[4].s64 + 4;
	// 83152528: 5565003A  rlwinm r5, r11, 0, 0, 0x1d
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8315252C: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 83152530: 48056469  bl 0x831a8998
	ctx.lr = 0x83152534;
	sub_831A8998(ctx, base);
	// 83152534: FFE0F090  fmr f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[30].f64;
	// 83152538: 48000094  b 0x831525cc
	pc = 0x831525CC; continue 'dispatch;
	// 8315253C: D39C0000  stfs f28, 0(r28)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 83152540: D39CFFE0  stfs f28, -0x20(r28)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 83152544: 7D7AF8AE  lbzx r11, r26, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83152548: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8315254C: 419A0008  beq cr6, 0x83152554
	if ctx.cr[6].eq {
	pc = 0x83152554; continue 'dispatch;
	}
	// 83152550: 7EFAF9AE  stbx r23, r26, r31
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[26].u32.wrapping_add(ctx.r[31].u32), ctx.r[23].u8) };
	// 83152554: 89790000  lbz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152558: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8315255C: 409A002C  bne cr6, 0x83152588
	if !ctx.cr[6].eq {
	pc = 0x83152588; continue 'dispatch;
	}
	// 83152560: 81760040  lwz r11, 0x40(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(64 as u32) ) } as u64;
	// 83152564: 7EEABB78  mr r10, r23
	ctx.r[10].u64 = ctx.r[23].u64;
	// 83152568: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8315256C: 40980008  bge cr6, 0x83152574
	if !ctx.cr[6].lt {
	pc = 0x83152574; continue 'dispatch;
	}
	// 83152570: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152574: 57AB103A  slwi r11, r29, 2
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83152578: 57C5103A  slwi r5, r30, 2
	ctx.r[5].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 8315257C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83152580: 7C6B5214  add r3, r11, r10
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83152584: 48055C5D  bl 0x831a81e0
	ctx.lr = 0x83152588;
	sub_831A81E0(ctx, base);
	// 83152588: 397EFFFF  addi r11, r30, -1
	ctx.r[11].s64 = ctx.r[30].s64 + -1;
	// 8315258C: C0180020  lfs f0, 0x20(r24)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83152590: 796A0020  clrldi r10, r11, 0x20
	ctx.r[10].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 83152594: F9410060  std r10, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u64 ) };
	// 83152598: C9A10060  lfd f13, 0x60(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 8315259C: FD806E9C  fcfid f12, f13
	ctx.f[12].f64 = (ctx.f[13].s64 as f64);
	// 831525A0: FD606018  frsp f11, f12
	ctx.f[11].f64 = (ctx.f[12].f64 as f32) as f64;
	// 831525A4: ED4B077A  fmadds f10, f11, f29, f0
	ctx.f[10].f64 = (((ctx.f[11].f64 * ctx.f[29].f64 + ctx.f[0].f64) as f32) as f64);
	// 831525A8: FD20565E  fctidz f9, f10
	ctx.f[9].s64 = if ctx.f[10].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[10].f64.trunc() as i64 };
	// 831525AC: D9210050  stfd f9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[9].u64 ) };
	// 831525B0: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831525B4: F9010058  std r8, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u64 ) };
	// 831525B8: C9010058  lfd f8, 0x58(r1)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 831525BC: FCE0469C  fcfid f7, f8
	ctx.f[7].f64 = (ctx.f[8].s64 as f64);
	// 831525C0: FCC03818  frsp f6, f7
	ctx.f[6].f64 = (ctx.f[7].f64 as f32) as f64;
	// 831525C4: ECAA3028  fsubs f5, f10, f6
	ctx.f[5].f64 = (((ctx.f[10].f64 - ctx.f[6].f64) as f32) as f64);
	// 831525C8: EFE5E82A  fadds f31, f5, f29
	ctx.f[31].f64 = ((ctx.f[5].f64 + ctx.f[29].f64) as f32) as f64;
	// 831525CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831525D0: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 831525D4: 3B390001  addi r25, r25, 1
	ctx.r[25].s64 = ctx.r[25].s64 + 1;
	// 831525D8: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 831525DC: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 831525E0: 7F1F9840  cmplw cr6, r31, r19
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[19].u32, &mut ctx.xer);
	// 831525E4: 4198FE04  blt cr6, 0x831523e8
	if ctx.cr[6].lt {
	pc = 0x831523E8; continue 'dispatch;
	}
	// 831525E8: 814100B0  lwz r10, 0xb0(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 831525EC: D3F80020  stfs f31, 0x20(r24)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 831525F0: 92760044  stw r19, 0x44(r22)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[22].u32.wrapping_add(68 as u32), ctx.r[19].u32 ) };
	// 831525F4: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 831525F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831525FC: 419A0020  beq cr6, 0x8315261c
	if ctx.cr[6].eq {
	pc = 0x8315261C; continue 'dispatch;
	}
	// 83152600: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 83152604: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83152608: 92EA0000  stw r23, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u32 ) };
	// 8315260C: 812100B0  lwz r9, 0xb0(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 83152610: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83152614: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83152618: 4198FFEC  blt cr6, 0x83152604
	if ctx.cr[6].lt {
	pc = 0x83152604; continue 'dispatch;
	}
	// 8315261C: 38211180  addi r1, r1, 0x1180
	ctx.r[1].s64 = ctx.r[1].s64 + 4480;
	// 83152620: 3981FF90  addi r12, r1, -0x70
	ctx.r[12].s64 = ctx.r[1].s64 + -112;
	// 83152624: 480564A1  bl 0x831a8ac4
	ctx.lr = 0x83152628;
	sub_831A8A8C(ctx, base);
	// 83152628: 48055B6C  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83152630 size=96
    let mut pc: u32 = 0x83152630;
    'dispatch: loop {
        match pc {
            0x83152630 => {
    //   block [0x83152630..0x83152690)
	// 83152630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83152634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83152638: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8315263C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83152640: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83152644: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83152648: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8315264C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83152650: 394B537C  addi r10, r11, 0x537c
	ctx.r[10].s64 = ctx.r[11].s64 + 21372;
	// 83152654: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83152658: 4BFF3B51  bl 0x831461a8
	ctx.lr = 0x8315265C;
	sub_831461A8(ctx, base);
	// 8315265C: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 83152660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83152664: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83152668: 419A0010  beq cr6, 0x83152678
	if ctx.cr[6].eq {
	pc = 0x83152678; continue 'dispatch;
	}
	// 8315266C: 3880006C  li r4, 0x6c
	ctx.r[4].s64 = 108;
	// 83152670: 4800D611  bl 0x8315fc80
	ctx.lr = 0x83152674;
	sub_8315FC80(ctx, base);
	// 83152674: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83152678: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8315267C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83152680: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83152684: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83152688: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8315268C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83152690 size=160
    let mut pc: u32 = 0x83152690;
    'dispatch: loop {
        match pc {
            0x83152690 => {
    //   block [0x83152690..0x83152730)
	// 83152690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83152694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83152698: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8315269C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831526A0: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 831526A4: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 831526A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831526AC: 3FE08334  lis r31, -0x7ccc
	ctx.r[31].s64 = -2093744128;
	// 831526B0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 831526B4: C03F5FB8  lfs f1, 0x5fb8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24504 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 831526B8: 4805BC19  bl 0x831ae2d0
	ctx.lr = 0x831526BC;
	sub_831AE2D0(ctx, base);
	// 831526BC: 3FC08334  lis r30, -0x7ccc
	ctx.r[30].s64 = -2093744128;
	// 831526C0: FFC00818  frsp f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = (ctx.f[1].f64 as f32) as f64;
	// 831526C4: C03E5FB4  lfs f1, 0x5fb4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24500 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 831526C8: 4805BC09  bl 0x831ae2d0
	ctx.lr = 0x831526CC;
	sub_831AE2D0(ctx, base);
	// 831526CC: FDA00818  frsp f13, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (ctx.f[1].f64 as f32) as f64;
	// 831526D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 831526D4: C03E5FB4  lfs f1, 0x5fb4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24500 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 831526D8: C00B08A8  lfs f0, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831526DC: ED9E6824  fdivs f12, f30, f13
	ctx.f[12].f64 = ((ctx.f[30].f64 / ctx.f[13].f64) as f32) as f64;
	// 831526E0: ED6C0028  fsubs f11, f12, f0
	ctx.f[11].f64 = (((ctx.f[12].f64 - ctx.f[0].f64) as f32) as f64);
	// 831526E4: EC4B07FA  fmadds f2, f11, f31, f0
	ctx.f[2].f64 = (((ctx.f[11].f64 * ctx.f[31].f64 + ctx.f[0].f64) as f32) as f64);
	// 831526E8: 48058DC1  bl 0x831ab4a8
	ctx.lr = 0x831526EC;
	sub_831AB4A8(ctx, base);
	// 831526EC: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 831526F0: C1BE5FB4  lfs f13, 0x5fb4(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24500 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831526F4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 831526F8: 40980008  bge cr6, 0x83152700
	if !ctx.cr[6].lt {
	pc = 0x83152700; continue 'dispatch;
	}
	// 831526FC: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 83152700: C03F5FB8  lfs f1, 0x5fb8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24504 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83152704: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 83152708: 41990008  bgt cr6, 0x83152710
	if ctx.cr[6].gt {
	pc = 0x83152710; continue 'dispatch;
	}
	// 8315270C: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 83152710: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83152714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83152718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8315271C: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 83152720: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 83152724: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83152728: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8315272C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83152730 size=208
    let mut pc: u32 = 0x83152730;
    'dispatch: loop {
        match pc {
            0x83152730 => {
    //   block [0x83152730..0x83152800)
	// 83152730: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83152734: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 83152738: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 8315273C: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 83152740: 394B53E0  addi r10, r11, 0x53e0
	ctx.r[10].s64 = ctx.r[11].s64 + 21472;
	// 83152744: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 83152748: 3963001C  addi r11, r3, 0x1c
	ctx.r[11].s64 = ctx.r[3].s64 + 28;
	// 8315274C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83152750: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83152754: 38C00007  li r6, 7
	ctx.r[6].s64 = 7;
	// 83152758: 9103000C  stw r8, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8315275C: 91030010  stw r8, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u32 ) };
	// 83152760: 38EA3114  addi r7, r10, 0x3114
	ctx.r[7].s64 = ctx.r[10].s64 + 12564;
	// 83152764: 91030014  stw r8, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 83152768: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8315276C: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 83152770: 910B0050  stw r8, 0x50(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 83152774: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 83152778: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 8315277C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83152780: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83152784: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83152788: 4200FFF8  bdnz 0x83152780
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83152780; continue 'dispatch;
	}
	// 8315278C: 394B0030  addi r10, r11, 0x30
	ctx.r[10].s64 = ctx.r[11].s64 + 48;
	// 83152790: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 83152794: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83152798: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8315279C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831527A0: 4200FFF8  bdnz 0x83152798
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83152798; continue 'dispatch;
	}
	// 831527A4: 394B0054  addi r10, r11, 0x54
	ctx.r[10].s64 = ctx.r[11].s64 + 84;
	// 831527A8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 831527AC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831527B0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 831527B4: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831527B8: 4200FFF8  bdnz 0x831527b0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831527B0; continue 'dispatch;
	}
	// 831527BC: 394B0074  addi r10, r11, 0x74
	ctx.r[10].s64 = ctx.r[11].s64 + 116;
	// 831527C0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 831527C4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 831527C8: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 831527CC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 831527D0: 4200FFF8  bdnz 0x831527c8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831527C8; continue 'dispatch;
	}
	// 831527D4: 34C6FFFF  addic. r6, r6, -1
	ctx.xer.ca = (ctx.r[6].u32 > (!(-1 as u32)));
	ctx.r[6].s64 = ctx.r[6].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 831527D8: 396B0094  addi r11, r11, 0x94
	ctx.r[11].s64 = ctx.r[11].s64 + 148;
	// 831527DC: 4080FF8C  bge 0x83152768
	if !ctx.cr[0].lt {
	pc = 0x83152768; continue 'dispatch;
	}
	// 831527E0: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 831527E4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 831527E8: C1ABC7EC  lfs f13, -0x3814(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-14356 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831527EC: C00A08A4  lfs f0, 0x8a4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831527F0: D1A304BC  stfs f13, 0x4bc(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1212 as u32), tmp.u32 ) };
	// 831527F4: D00304C0  stfs f0, 0x4c0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1216 as u32), tmp.u32 ) };
	// 831527F8: D00304C4  stfs f0, 0x4c4(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1220 as u32), tmp.u32 ) };
	// 831527FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83152800 size=724
    let mut pc: u32 = 0x83152800;
    'dispatch: loop {
        match pc {
            0x83152800 => {
    //   block [0x83152800..0x83152AD4)
	// 83152800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83152804: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83152808: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8315280C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83152810: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83152814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83152818: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8315281C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83152820: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152824: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83152828: 419A0014  beq cr6, 0x8315283c
	if ctx.cr[6].eq {
	pc = 0x8315283C; continue 'dispatch;
	}
	// 8315282C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152830: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152834: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83152838: 4E800421  bctrl
	ctx.lr = 0x8315283C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8315283C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83152840: C1BF04C0  lfs f13, 0x4c0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83152844: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83152848: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8315284C: 419A0270  beq cr6, 0x83152abc
	if ctx.cr[6].eq {
	pc = 0x83152ABC; continue 'dispatch;
	}
	// 83152850: C1BF04C4  lfs f13, 0x4c4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1220 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83152854: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 83152858: 419A0264  beq cr6, 0x83152abc
	if ctx.cr[6].eq {
	pc = 0x83152ABC; continue 'dispatch;
	}
	// 8315285C: 807E0044  lwz r3, 0x44(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 83152860: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83152864: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83152868: 419A0254  beq cr6, 0x83152abc
	if ctx.cr[6].eq {
	pc = 0x83152ABC; continue 'dispatch;
	}
	// 8315286C: 397F0078  addi r11, r31, 0x78
	ctx.r[11].s64 = ctx.r[31].s64 + 120;
	// 83152870: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83152874: 3BFE0050  addi r31, r30, 0x50
	ctx.r[31].s64 = ctx.r[30].s64 + 80;
	// 83152878: 7D5F28AE  lbzx r10, r31, r5
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[5].u32)) } as u64;
	// 8315287C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83152880: 419A0228  beq cr6, 0x83152aa8
	if ctx.cr[6].eq {
	pc = 0x83152AA8; continue 'dispatch;
	}
	// 83152884: 815E0040  lwz r10, 0x40(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 83152888: 7F055040  cmplw cr6, r5, r10
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[10].u32, &mut ctx.xer);
	// 8315288C: 41980010  blt cr6, 0x8315289c
	if ctx.cr[6].lt {
	pc = 0x8315289C; continue 'dispatch;
	}
	// 83152890: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83152894: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83152898: 4800000C  b 0x831528a4
	pc = 0x831528A4; continue 'dispatch;
	// 8315289C: 811E0048  lwz r8, 0x48(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 831528A0: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 831528A4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 831528A8: 2F080004  cmpwi cr6, r8, 4
	ctx.cr[6].compare_i32(ctx.r[8].s32, 4, &mut ctx.xer);
	// 831528AC: 41980184  blt cr6, 0x83152a30
	if ctx.cr[6].lt {
	pc = 0x83152A30; continue 'dispatch;
	}
	// 831528B0: 3948FFFC  addi r10, r8, -4
	ctx.r[10].s64 = ctx.r[8].s64 + -4;
	// 831528B4: 554AF0BE  srwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shr(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831528B8: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 831528BC: 39460008  addi r10, r6, 8
	ctx.r[10].s64 = ctx.r[6].s64 + 8;
	// 831528C0: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 831528C4: C00AFFF8  lfs f0, -8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-8 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831528C8: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831528CC: C1ABFFF8  lfs f13, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831528D0: ED800372  fmuls f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 831528D4: C16B0020  lfs f11, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 831528D8: C14B0000  lfs f10, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 831528DC: C12B001C  lfs f9, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 831528E0: C10BFFFC  lfs f8, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 831528E4: C0EBFFDC  lfs f7, -0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-36 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 831528E8: C0CBFFBC  lfs f6, -0x44(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-68 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 831528EC: C0ABFFD8  lfs f5, -0x28(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-40 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 831528F0: C08BFFB8  lfs f4, -0x48(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-72 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 831528F4: C06BFFAC  lfs f3, -0x54(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-84 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 831528F8: D12B0020  stfs f9, 0x20(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 831528FC: EC4B62BA  fmadds f2, f11, f10, f12
	ctx.f[2].f64 = (((ctx.f[11].f64 * ctx.f[10].f64 + ctx.f[12].f64) as f32) as f64);
	// 83152900: D18B001C  stfs f12, 0x1c(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 83152904: D0ABFFDC  stfs f5, -0x24(r11)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 83152908: EC29123A  fmadds f1, f9, f8, f2
	ctx.f[1].f64 = (((ctx.f[9].f64 * ctx.f[8].f64 + ctx.f[2].f64) as f32) as f64);
	// 8315290C: EC0709BC  fnmsubs f0, f7, f6, f1
	ctx.f[0].f64 = -(((ctx.f[7].f64 * ctx.f[6].f64 - ctx.f[1].f64) as f32) as f64);
	// 83152910: EDA5013C  fnmsubs f13, f5, f4, f0
	ctx.f[13].f64 = -(((ctx.f[5].f64 * ctx.f[4].f64 - ctx.f[0].f64) as f32) as f64);
	// 83152914: D1ABFFD8  stfs f13, -0x28(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 83152918: C18AFFFC  lfs f12, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 8315291C: ED6D00F2  fmuls f11, f13, f3
	ctx.f[11].f64 = (((ctx.f[13].f64 * ctx.f[3].f64) as f32) as f64);
	// 83152920: D16AFFF8  stfs f11, -8(r10)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 83152924: C14BFFF8  lfs f10, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 83152928: C12B0000  lfs f9, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 8315292C: C10BFFFC  lfs f8, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 83152930: C0EBFFDC  lfs f7, -0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-36 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 83152934: C0CBFFBC  lfs f6, -0x44(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-68 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 83152938: C0ABFFD8  lfs f5, -0x28(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-40 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 8315293C: C08BFFB8  lfs f4, -0x48(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-72 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 83152940: C06B0020  lfs f3, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 83152944: C04BFFAC  lfs f2, -0x54(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-84 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 83152948: C02B001C  lfs f1, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8315294C: D02B0020  stfs f1, 0x20(r11)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 83152950: EC0C02B2  fmuls f0, f12, f10
	ctx.f[0].f64 = (((ctx.f[12].f64 * ctx.f[10].f64) as f32) as f64);
	// 83152954: D00B001C  stfs f0, 0x1c(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 83152958: D0ABFFDC  stfs f5, -0x24(r11)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 8315295C: EDA3027A  fmadds f13, f3, f9, f0
	ctx.f[13].f64 = (((ctx.f[3].f64 * ctx.f[9].f64 + ctx.f[0].f64) as f32) as f64);
	// 83152960: ED816A3A  fmadds f12, f1, f8, f13
	ctx.f[12].f64 = (((ctx.f[1].f64 * ctx.f[8].f64 + ctx.f[13].f64) as f32) as f64);
	// 83152964: ED6761BC  fnmsubs f11, f7, f6, f12
	ctx.f[11].f64 = -(((ctx.f[7].f64 * ctx.f[6].f64 - ctx.f[12].f64) as f32) as f64);
	// 83152968: ED45593C  fnmsubs f10, f5, f4, f11
	ctx.f[10].f64 = -(((ctx.f[5].f64 * ctx.f[4].f64 - ctx.f[11].f64) as f32) as f64);
	// 8315296C: D14BFFD8  stfs f10, -0x28(r11)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 83152970: C12A0000  lfs f9, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 83152974: ED0A00B2  fmuls f8, f10, f2
	ctx.f[8].f64 = (((ctx.f[10].f64 * ctx.f[2].f64) as f32) as f64);
	// 83152978: D10AFFFC  stfs f8, -4(r10)
	tmp.f32 = (ctx.f[8].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 8315297C: C0EBFFF8  lfs f7, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 83152980: C0CB0000  lfs f6, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 83152984: C0ABFFFC  lfs f5, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 83152988: C08BFFDC  lfs f4, -0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-36 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 8315298C: C06BFFBC  lfs f3, -0x44(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-68 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 83152990: C04BFFD8  lfs f2, -0x28(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-40 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 83152994: C02BFFB8  lfs f1, -0x48(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-72 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83152998: C00B0020  lfs f0, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8315299C: C1ABFFAC  lfs f13, -0x54(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-84 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831529A0: C18B001C  lfs f12, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831529A4: D18B0020  stfs f12, 0x20(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 831529A8: ED6901F2  fmuls f11, f9, f7
	ctx.f[11].f64 = (((ctx.f[9].f64 * ctx.f[7].f64) as f32) as f64);
	// 831529AC: D16B001C  stfs f11, 0x1c(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 831529B0: D04BFFDC  stfs f2, -0x24(r11)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 831529B4: ED4059BA  fmadds f10, f0, f6, f11
	ctx.f[10].f64 = (((ctx.f[0].f64 * ctx.f[6].f64 + ctx.f[11].f64) as f32) as f64);
	// 831529B8: ED2C517A  fmadds f9, f12, f5, f10
	ctx.f[9].f64 = (((ctx.f[12].f64 * ctx.f[5].f64 + ctx.f[10].f64) as f32) as f64);
	// 831529BC: ED0448FC  fnmsubs f8, f4, f3, f9
	ctx.f[8].f64 = -(((ctx.f[4].f64 * ctx.f[3].f64 - ctx.f[9].f64) as f32) as f64);
	// 831529C0: ECE2407C  fnmsubs f7, f2, f1, f8
	ctx.f[7].f64 = -(((ctx.f[2].f64 * ctx.f[1].f64 - ctx.f[8].f64) as f32) as f64);
	// 831529C4: D0EBFFD8  stfs f7, -0x28(r11)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 831529C8: C0CA0004  lfs f6, 4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 831529CC: ECA70372  fmuls f5, f7, f13
	ctx.f[5].f64 = (((ctx.f[7].f64 * ctx.f[13].f64) as f32) as f64);
	// 831529D0: D0AA0000  stfs f5, 0(r10)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 831529D4: C08B0020  lfs f4, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 831529D8: C06B0000  lfs f3, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 831529DC: C04B001C  lfs f2, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 831529E0: C02BFFFC  lfs f1, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 831529E4: C00BFFDC  lfs f0, -0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-36 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831529E8: C1ABFFBC  lfs f13, -0x44(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-68 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 831529EC: C18BFFF8  lfs f12, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 831529F0: ED660332  fmuls f11, f6, f12
	ctx.f[11].f64 = (((ctx.f[6].f64 * ctx.f[12].f64) as f32) as f64);
	// 831529F4: ED4458FA  fmadds f10, f4, f3, f11
	ctx.f[10].f64 = (((ctx.f[4].f64 * ctx.f[3].f64 + ctx.f[11].f64) as f32) as f64);
	// 831529F8: D04B0020  stfs f2, 0x20(r11)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 831529FC: D16B001C  stfs f11, 0x1c(r11)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 83152A00: ED22507A  fmadds f9, f2, f1, f10
	ctx.f[9].f64 = (((ctx.f[2].f64 * ctx.f[1].f64 + ctx.f[10].f64) as f32) as f64);
	// 83152A04: ED004B7C  fnmsubs f8, f0, f13, f9
	ctx.f[8].f64 = -(((ctx.f[0].f64 * ctx.f[13].f64 - ctx.f[9].f64) as f32) as f64);
	// 83152A08: C0EBFFD8  lfs f7, -0x28(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-40 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 83152A0C: C0CBFFB8  lfs f6, -0x48(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-72 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 83152A10: ECA741BC  fnmsubs f5, f7, f6, f8
	ctx.f[5].f64 = -(((ctx.f[7].f64 * ctx.f[6].f64 - ctx.f[8].f64) as f32) as f64);
	// 83152A14: C08BFFAC  lfs f4, -0x54(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-84 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 83152A18: D0EBFFDC  stfs f7, -0x24(r11)
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 83152A1C: D0ABFFD8  stfs f5, -0x28(r11)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 83152A20: EC650132  fmuls f3, f5, f4
	ctx.f[3].f64 = (((ctx.f[5].f64 * ctx.f[4].f64) as f32) as f64);
	// 83152A24: D06A0004  stfs f3, 4(r10)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 83152A28: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 83152A2C: 4082FE98  bne 0x831528c4
	if !ctx.cr[0].eq {
	pc = 0x831528C4; continue 'dispatch;
	}
	// 83152A30: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 83152A34: 40980074  bge cr6, 0x83152aa8
	if !ctx.cr[6].lt {
	pc = 0x83152AA8; continue 'dispatch;
	}
	// 83152A38: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83152A3C: 7D274050  subf r9, r7, r8
	ctx.r[9].s64 = ctx.r[8].s64 - ctx.r[7].s64;
	// 83152A40: 7D4A3214  add r10, r10, r6
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[6].u64;
	// 83152A44: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83152A48: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83152A4C: C1ABFFF8  lfs f13, -8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-8 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83152A50: ED800372  fmuls f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 83152A54: C16B0020  lfs f11, 0x20(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 83152A58: C14B0000  lfs f10, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 83152A5C: C12B001C  lfs f9, 0x1c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 83152A60: C10BFFFC  lfs f8, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 83152A64: C0EBFFDC  lfs f7, -0x24(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-36 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 83152A68: C0CBFFBC  lfs f6, -0x44(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-68 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 83152A6C: C0ABFFD8  lfs f5, -0x28(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-40 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 83152A70: C08BFFB8  lfs f4, -0x48(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-72 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 83152A74: C06BFFAC  lfs f3, -0x54(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-84 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 83152A78: D18B001C  stfs f12, 0x1c(r11)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 83152A7C: EC4B62BA  fmadds f2, f11, f10, f12
	ctx.f[2].f64 = (((ctx.f[11].f64 * ctx.f[10].f64 + ctx.f[12].f64) as f32) as f64);
	// 83152A80: D12B0020  stfs f9, 0x20(r11)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 83152A84: D0ABFFDC  stfs f5, -0x24(r11)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-36 as u32), tmp.u32 ) };
	// 83152A88: EC29123A  fmadds f1, f9, f8, f2
	ctx.f[1].f64 = (((ctx.f[9].f64 * ctx.f[8].f64 + ctx.f[2].f64) as f32) as f64);
	// 83152A8C: EC0709BC  fnmsubs f0, f7, f6, f1
	ctx.f[0].f64 = -(((ctx.f[7].f64 * ctx.f[6].f64 - ctx.f[1].f64) as f32) as f64);
	// 83152A90: EDA5013C  fnmsubs f13, f5, f4, f0
	ctx.f[13].f64 = -(((ctx.f[5].f64 * ctx.f[4].f64 - ctx.f[0].f64) as f32) as f64);
	// 83152A94: D1ABFFD8  stfs f13, -0x28(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-40 as u32), tmp.u32 ) };
	// 83152A98: ED8D00F2  fmuls f12, f13, f3
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[3].f64) as f32) as f64);
	// 83152A9C: D18A0000  stfs f12, 0(r10)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 83152AA0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83152AA4: 4082FFA0  bne 0x83152a44
	if !ctx.cr[0].eq {
	pc = 0x83152A44; continue 'dispatch;
	}
	// 83152AA8: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 83152AAC: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 83152AB0: 396B0094  addi r11, r11, 0x94
	ctx.r[11].s64 = ctx.r[11].s64 + 148;
	// 83152AB4: 7F051840  cmplw cr6, r5, r3
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[3].u32, &mut ctx.xer);
	// 83152AB8: 4198FDC0  blt cr6, 0x83152878
	if ctx.cr[6].lt {
	pc = 0x83152878; continue 'dispatch;
	}
	// 83152ABC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83152AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83152AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83152AC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83152ACC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83152AD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83152AD8 size=132
    let mut pc: u32 = 0x83152AD8;
    'dispatch: loop {
        match pc {
            0x83152AD8 => {
    //   block [0x83152AD8..0x83152B5C)
	// 83152AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83152ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83152AE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83152AE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83152AE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83152AEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83152AF0: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83152AF4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83152AF8: 392A53E0  addi r9, r10, 0x53e0
	ctx.r[9].s64 = ctx.r[10].s64 + 21472;
	// 83152AFC: 397F04BC  addi r11, r31, 0x4bc
	ctx.r[11].s64 = ctx.r[31].s64 + 1212;
	// 83152B00: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83152B04: 3D208219  lis r9, -0x7de7
	ctx.r[9].s64 = -2112290816;
	// 83152B08: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 83152B0C: 39293114  addi r9, r9, 0x3114
	ctx.r[9].s64 = ctx.r[9].s64 + 12564;
	// 83152B10: 396BFF6C  addi r11, r11, -0x94
	ctx.r[11].s64 = ctx.r[11].s64 + -148;
	// 83152B14: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83152B18: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83152B1C: 4080FFF4  bge 0x83152b10
	if !ctx.cr[0].lt {
	pc = 0x83152B10; continue 'dispatch;
	}
	// 83152B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83152B24: 4BFF3685  bl 0x831461a8
	ctx.lr = 0x83152B28;
	sub_831461A8(ctx, base);
	// 83152B28: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 83152B2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83152B30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83152B34: 419A0010  beq cr6, 0x83152b44
	if ctx.cr[6].eq {
	pc = 0x83152B44; continue 'dispatch;
	}
	// 83152B38: 388004C8  li r4, 0x4c8
	ctx.r[4].s64 = 1224;
	// 83152B3C: 4800D145  bl 0x8315fc80
	ctx.lr = 0x83152B40;
	sub_8315FC80(ctx, base);
	// 83152B40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83152B44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83152B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83152B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83152B50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83152B54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83152B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83152B60 size=176
    let mut pc: u32 = 0x83152B60;
    'dispatch: loop {
        match pc {
            0x83152B60 => {
    //   block [0x83152B60..0x83152C10)
	// 83152B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83152B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83152B68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83152B6C: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83152B70: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83152B74: 409A0034  bne cr6, 0x83152ba8
	if !ctx.cr[6].eq {
	pc = 0x83152BA8; continue 'dispatch;
	}
	// 83152B78: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83152B7C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152B80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83152B84: 419A007C  beq cr6, 0x83152c00
	if ctx.cr[6].eq {
	pc = 0x83152C00; continue 'dispatch;
	}
	// 83152B88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152B8C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152B90: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83152B94: 4E800421  bctrl
	ctx.lr = 0x83152B98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83152B98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83152B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83152BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83152BA4: 4E800020  blr
	return;
	// 83152BA8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83152BAC: 81440048  lwz r10, 0x48(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 83152BB0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83152BB4: 91640044  stw r11, 0x44(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83152BB8: 3D008204  lis r8, -0x7dfc
	ctx.r[8].s64 = -2113667072;
	// 83152BBC: 99240050  stb r9, 0x50(r4)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[4].u32.wrapping_add(80 as u32), ctx.r[9].u8 ) };
	// 83152BC0: C1A3001C  lfs f13, 0x1c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83152BC4: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83152BC8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83152BCC: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83152BD0: C184004C  lfs f12, 0x4c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 83152BD4: 79670020  clrldi r7, r11, 0x20
	ctx.r[7].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 83152BD8: C008DD6C  lfs f0, -0x2294(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-8852 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83152BDC: F8E10050  std r7, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u64 ) };
	// 83152BE0: C9610050  lfd f11, 0x50(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83152BE4: FD405E9C  fcfid f10, f11
	ctx.f[10].f64 = (ctx.f[11].s64 as f64);
	// 83152BE8: FD205018  frsp f9, f10
	ctx.f[9].f64 = (ctx.f[10].f64 as f32) as f64;
	// 83152BEC: ED096024  fdivs f8, f9, f12
	ctx.f[8].f64 = ((ctx.f[9].f64 / ctx.f[12].f64) as f32) as f64;
	// 83152BF0: ECE80032  fmuls f7, f8, f0
	ctx.f[7].f64 = (((ctx.f[8].f64 * ctx.f[0].f64) as f32) as f64);
	// 83152BF4: FF076800  fcmpu cr6, f7, f13
	ctx.cr[6].compare_f64(ctx.f[7].f64, ctx.f[13].f64);
	// 83152BF8: 41980008  blt cr6, 0x83152c00
	if ctx.cr[6].lt {
	pc = 0x83152C00; continue 'dispatch;
	}
	// 83152BFC: 91230024  stw r9, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[9].u32 ) };
	// 83152C00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83152C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83152C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83152C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83152C10 size=96
    let mut pc: u32 = 0x83152C10;
    'dispatch: loop {
        match pc {
            0x83152C10 => {
    //   block [0x83152C10..0x83152C70)
	// 83152C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83152C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83152C18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83152C1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83152C20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83152C24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83152C28: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83152C2C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83152C30: 394B53E8  addi r10, r11, 0x53e8
	ctx.r[10].s64 = ctx.r[11].s64 + 21480;
	// 83152C34: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83152C38: 4BFF3571  bl 0x831461a8
	ctx.lr = 0x83152C3C;
	sub_831461A8(ctx, base);
	// 83152C3C: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 83152C40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83152C44: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83152C48: 419A0010  beq cr6, 0x83152c58
	if ctx.cr[6].eq {
	pc = 0x83152C58; continue 'dispatch;
	}
	// 83152C4C: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 83152C50: 4800D031  bl 0x8315fc80
	ctx.lr = 0x83152C54;
	sub_8315FC80(ctx, base);
	// 83152C54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83152C58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83152C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83152C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83152C64: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83152C68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83152C6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83152C70 size=28
    let mut pc: u32 = 0x83152C70;
    'dispatch: loop {
        match pc {
            0x83152C70 => {
    //   block [0x83152C70..0x83152C8C)
	// 83152C70: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 83152C74: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83152C78: 409A0024  bne cr6, 0x83152c9c
	if !ctx.cr[6].eq {
		sub_83152C9C(ctx, base);
		return;
	}
	// 83152C7C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83152C80: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152C84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83152C88: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152C8C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83152C8C size=16
    let mut pc: u32 = 0x83152C8C;
    'dispatch: loop {
        match pc {
            0x83152C8C => {
    //   block [0x83152C8C..0x83152C9C)
	// 83152C8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152C90: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152C94: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83152C98: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152C9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83152C9C size=44
    let mut pc: u32 = 0x83152C9C;
    'dispatch: loop {
        match pc {
            0x83152C9C => {
    //   block [0x83152C9C..0x83152CC8)
	// 83152C9C: 81440040  lwz r10, 0x40(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 83152CA0: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83152CA4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83152CA8: 419A0018  beq cr6, 0x83152cc0
	if ctx.cr[6].eq {
	pc = 0x83152CC0; continue 'dispatch;
	}
	// 83152CAC: 39640050  addi r11, r4, 0x50
	ctx.r[11].s64 = ctx.r[4].s64 + 80;
	// 83152CB0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83152CB4: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83152CB8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83152CBC: 4200FFF8  bdnz 0x83152cb4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83152CB4; continue 'dispatch;
	}
	// 83152CC0: 91240044  stw r9, 0x44(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 83152CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83152CC8 size=96
    let mut pc: u32 = 0x83152CC8;
    'dispatch: loop {
        match pc {
            0x83152CC8 => {
    //   block [0x83152CC8..0x83152D28)
	// 83152CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83152CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83152CD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83152CD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83152CD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83152CDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83152CE0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83152CE4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83152CE8: 394B53F0  addi r10, r11, 0x53f0
	ctx.r[10].s64 = ctx.r[11].s64 + 21488;
	// 83152CEC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83152CF0: 4BFF34B9  bl 0x831461a8
	ctx.lr = 0x83152CF4;
	sub_831461A8(ctx, base);
	// 83152CF4: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 83152CF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83152CFC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83152D00: 419A0010  beq cr6, 0x83152d10
	if ctx.cr[6].eq {
	pc = 0x83152D10; continue 'dispatch;
	}
	// 83152D04: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 83152D08: 4800CF79  bl 0x8315fc80
	ctx.lr = 0x83152D0C;
	sub_8315FC80(ctx, base);
	// 83152D0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83152D10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83152D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83152D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83152D1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83152D20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83152D24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83152D28 size=124
    let mut pc: u32 = 0x83152D28;
    'dispatch: loop {
        match pc {
            0x83152D28 => {
    //   block [0x83152D28..0x83152DA4)
	// 83152D28: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 83152D2C: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 83152D30: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83152D34: 3FE08219  lis r31, -0x7de7
	ctx.r[31].s64 = -2112290816;
	// 83152D38: D0230020  stfs f1, 0x20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 83152D3C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83152D40: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 83152D44: 3BFF53F8  addi r31, r31, 0x53f8
	ctx.r[31].s64 = ctx.r[31].s64 + 21496;
	// 83152D48: 90830018  stw r4, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 83152D4C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83152D50: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 83152D54: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 83152D58: 90E30010  stw r7, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 83152D5C: 90E30014  stw r7, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 83152D60: 93E30000  stw r31, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83152D64: 90C3001C  stw r6, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[6].u32 ) };
	// 83152D68: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83152D6C: 91430048  stw r10, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 83152D70: 9103004C  stw r8, 0x4c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(76 as u32), ctx.r[8].u32 ) };
	// 83152D74: 90E30050  stw r7, 0x50(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 83152D78: 419A0024  beq cr6, 0x83152d9c
	if ctx.cr[6].eq {
	pc = 0x83152D9C; continue 'dispatch;
	}
	// 83152D7C: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 83152D80: 39230028  addi r9, r3, 0x28
	ctx.r[9].s64 = ctx.r[3].s64 + 40;
	// 83152D84: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152D88: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83152D8C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83152D90: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83152D94: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 83152D98: 4082FFEC  bne 0x83152d84
	if !ctx.cr[0].eq {
	pc = 0x83152D84; continue 'dispatch;
	}
	// 83152D9C: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 83152DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83152DA8 size=580
    let mut pc: u32 = 0x83152DA8;
    'dispatch: loop {
        match pc {
            0x83152DA8 => {
    //   block [0x83152DA8..0x83152FEC)
	// 83152DA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83152DAC: 4805539D  bl 0x831a8148
	ctx.lr = 0x83152DB0;
	sub_831A8130(ctx, base);
	// 83152DB0: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83152DB4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83152DB8: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 83152DBC: 807A004C  lwz r3, 0x4c(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(76 as u32) ) } as u64;
	// 83152DC0: 817A0048  lwz r11, 0x48(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(72 as u32) ) } as u64;
	// 83152DC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83152DC8: 4E800421  bctrl
	ctx.lr = 0x83152DCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83152DCC: 815A001C  lwz r10, 0x1c(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 83152DD0: 3B200004  li r25, 4
	ctx.r[25].s64 = 4;
	// 83152DD4: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83152DD8: 409A0008  bne cr6, 0x83152de0
	if !ctx.cr[6].eq {
	pc = 0x83152DE0; continue 'dispatch;
	}
	// 83152DDC: 3B200002  li r25, 2
	ctx.r[25].s64 = 2;
	// 83152DE0: 817A0024  lwz r11, 0x24(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 83152DE4: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 83152DE8: 82F80048  lwz r23, 0x48(r24)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(72 as u32) ) } as u64;
	// 83152DEC: 7EBFAB78  mr r31, r21
	ctx.r[31].u64 = ctx.r[21].u64;
	// 83152DF0: 7EBBAB78  mr r27, r21
	ctx.r[27].u64 = ctx.r[21].u64;
	// 83152DF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83152DF8: 409901D8  ble cr6, 0x83152fd0
	if !ctx.cr[6].gt {
	pc = 0x83152FD0; continue 'dispatch;
	}
	// 83152DFC: 7F16C378  mr r22, r24
	ctx.r[22].u64 = ctx.r[24].u64;
	// 83152E00: 3B9A0028  addi r28, r26, 0x28
	ctx.r[28].s64 = ctx.r[26].s64 + 40;
	// 83152E04: 3A800001  li r20, 1
	ctx.r[20].s64 = 1;
	// 83152E08: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83152E0C: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152E10: 4800EE99  bl 0x83161ca8
	ctx.lr = 0x83152E14;
	sub_83161CA8(ctx, base);
	// 83152E14: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83152E18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83152E1C: 409A0020  bne cr6, 0x83152e3c
	if !ctx.cr[6].eq {
	pc = 0x83152E3C; continue 'dispatch;
	}
	// 83152E20: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83152E24: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152E28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83152E2C: 4800EE8D  bl 0x83161cb8
	ctx.lr = 0x83152E30;
	sub_83161CB8(ctx, base);
	// 83152E30: 7D7BC214  add r11, r27, r24
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[24].u64;
	// 83152E34: 9AAB0050  stb r21, 0x50(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[21].u8 ) };
	// 83152E38: 48000180  b 0x83152fb8
	pc = 0x83152FB8; continue 'dispatch;
	// 83152E3C: 81580040  lwz r10, 0x40(r24)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(64 as u32) ) } as u64;
	// 83152E40: 7EBEAB78  mr r30, r21
	ctx.r[30].u64 = ctx.r[21].u64;
	// 83152E44: 7F1B5040  cmplw cr6, r27, r10
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83152E48: 40980008  bge cr6, 0x83152e50
	if !ctx.cr[6].lt {
	pc = 0x83152E50; continue 'dispatch;
	}
	// 83152E4C: 83D60000  lwz r30, 0(r22)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152E50: 7FEBCB96  divwu r31, r11, r25
	ctx.r[31].u32 = ctx.r[11].u32 / ctx.r[25].u32;
	// 83152E54: 7F17F840  cmplw cr6, r23, r31
	ctx.cr[6].compare_u32(ctx.r[23].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83152E58: 41990008  bgt cr6, 0x83152e60
	if ctx.cr[6].gt {
	pc = 0x83152E60; continue 'dispatch;
	}
	// 83152E5C: 7EFFBB78  mr r31, r23
	ctx.r[31].u64 = ctx.r[23].u64;
	// 83152E60: 817A001C  lwz r11, 0x1c(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(28 as u32) ) } as u64;
	// 83152E64: 7FBFC9D6  mullw r29, r31, r25
	ctx.r[29].s64 = (ctx.r[31].s32 as i64) * (ctx.r[25].s32 as i64);
	// 83152E68: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83152E6C: 41980100  blt cr6, 0x83152f6c
	if ctx.cr[6].lt {
	pc = 0x83152F6C; continue 'dispatch;
	}
	// 83152E70: 409A010C  bne cr6, 0x83152f7c
	if !ctx.cr[6].eq {
	pc = 0x83152F7C; continue 'dispatch;
	}
	// 83152E74: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83152E78: 7EA9AB78  mr r9, r21
	ctx.r[9].u64 = ctx.r[21].u64;
	// 83152E7C: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 83152E80: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 83152E84: 4198009C  blt cr6, 0x83152f20
	if ctx.cr[6].lt {
	pc = 0x83152F20; continue 'dispatch;
	}
	// 83152E88: 393FFFFC  addi r9, r31, -4
	ctx.r[9].s64 = ctx.r[31].s64 + -4;
	// 83152E8C: 395E0008  addi r10, r30, 8
	ctx.r[10].s64 = ctx.r[30].s64 + 8;
	// 83152E90: 5529F0BE  srwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shr(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83152E94: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83152E98: 39090001  addi r8, r9, 1
	ctx.r[8].s64 = ctx.r[9].s64 + 1;
	// 83152E9C: 5509103A  slwi r9, r8, 2
	ctx.r[9].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83152EA0: A0CBFFFC  lhz r6, -4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83152EA4: 3508FFFF  addic. r8, r8, -1
	ctx.xer.ca = (ctx.r[8].u32 > (!(-1 as u32)));
	ctx.r[8].s64 = ctx.r[8].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83152EA8: 7CC40734  extsh r4, r6
	ctx.r[4].s64 = ctx.r[6].s16 as i64;
	// 83152EAC: F8810058  std r4, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[4].u64 ) };
	// 83152EB0: C8010058  lfd f0, 0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83152EB4: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 83152EB8: FD806818  frsp f12, f13
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	// 83152EBC: D18AFFF8  stfs f12, -8(r10)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 83152EC0: A06BFFFE  lhz r3, -2(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-2 as u32) ) } as u64;
	// 83152EC4: 7C650734  extsh r5, r3
	ctx.r[5].s64 = ctx.r[3].s16 as i64;
	// 83152EC8: F8A10060  std r5, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[5].u64 ) };
	// 83152ECC: C9610060  lfd f11, 0x60(r1)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 83152ED0: FD405E9C  fcfid f10, f11
	ctx.f[10].f64 = (ctx.f[11].s64 as f64);
	// 83152ED4: FD205018  frsp f9, f10
	ctx.f[9].f64 = (ctx.f[10].f64 as f32) as f64;
	// 83152ED8: D12AFFFC  stfs f9, -4(r10)
	tmp.f32 = (ctx.f[9].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), tmp.u32 ) };
	// 83152EDC: A08B0000  lhz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152EE0: 7C860734  extsh r6, r4
	ctx.r[6].s64 = ctx.r[4].s16 as i64;
	// 83152EE4: F8C10068  std r6, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[6].u64 ) };
	// 83152EE8: C9010068  lfd f8, 0x68(r1)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 83152EEC: FCE0469C  fcfid f7, f8
	ctx.f[7].f64 = (ctx.f[8].s64 as f64);
	// 83152EF0: FCC03818  frsp f6, f7
	ctx.f[6].f64 = (ctx.f[7].f64 as f32) as f64;
	// 83152EF4: D0CA0000  stfs f6, 0(r10)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 83152EF8: A0AB0002  lhz r5, 2(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 83152EFC: 7CA30734  extsh r3, r5
	ctx.r[3].s64 = ctx.r[5].s16 as i64;
	// 83152F00: F8610070  std r3, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[3].u64 ) };
	// 83152F04: C8A10070  lfd f5, 0x70(r1)
	ctx.f[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) };
	// 83152F08: FC802E9C  fcfid f4, f5
	ctx.f[4].f64 = (ctx.f[5].s64 as f64);
	// 83152F0C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83152F10: FC602018  frsp f3, f4
	ctx.f[3].f64 = (ctx.f[4].f64 as f32) as f64;
	// 83152F14: D06A0004  stfs f3, 4(r10)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 83152F18: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 83152F1C: 4082FF84  bne 0x83152ea0
	if !ctx.cr[0].eq {
	pc = 0x83152EA0; continue 'dispatch;
	}
	// 83152F20: 7F09F840  cmplw cr6, r9, r31
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83152F24: 40980058  bge cr6, 0x83152f7c
	if !ctx.cr[6].lt {
	pc = 0x83152F7C; continue 'dispatch;
	}
	// 83152F28: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83152F2C: 552B083C  slwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83152F30: 7D0AF214  add r8, r10, r30
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[30].u64;
	// 83152F34: 7D4B3A14  add r10, r11, r7
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[7].u64;
	// 83152F38: 7D69F850  subf r11, r9, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[9].s64;
	// 83152F3C: A12A0000  lhz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152F40: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83152F44: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 83152F48: 7D260734  extsh r6, r9
	ctx.r[6].s64 = ctx.r[9].s16 as i64;
	// 83152F4C: F8C10078  std r6, 0x78(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u64 ) };
	// 83152F50: C8010078  lfd f0, 0x78(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) };
	// 83152F54: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 83152F58: FD806818  frsp f12, f13
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	// 83152F5C: D1880000  stfs f12, 0(r8)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 83152F60: 39080004  addi r8, r8, 4
	ctx.r[8].s64 = ctx.r[8].s64 + 4;
	// 83152F64: 4082FFD8  bne 0x83152f3c
	if !ctx.cr[0].eq {
	pc = 0x83152F3C; continue 'dispatch;
	}
	// 83152F68: 48000014  b 0x83152f7c
	pc = 0x83152F7C; continue 'dispatch;
	// 83152F6C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83152F70: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83152F74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83152F78: 48055599  bl 0x831a8510
	ctx.lr = 0x83152F7C;
	sub_831A8510(ctx, base);
	// 83152F7C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83152F80: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83152F84: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83152F88: 4800ED31  bl 0x83161cb8
	ctx.lr = 0x83152F8C;
	sub_83161CB8(ctx, base);
	// 83152F8C: 7D7FB850  subf r11, r31, r23
	ctx.r[11].s64 = ctx.r[23].s64 - ctx.r[31].s64;
	// 83152F90: 7CABC9D7  mullw. r5, r11, r25
	ctx.r[5].s64 = (ctx.r[11].s32 as i64) * (ctx.r[25].s32 as i64);
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 83152F94: 41820014  beq 0x83152fa8
	if ctx.cr[0].eq {
	pc = 0x83152FA8; continue 'dispatch;
	}
	// 83152F98: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83152F9C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83152FA0: 7C6BF214  add r3, r11, r30
	ctx.r[3].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83152FA4: 4805523D  bl 0x831a81e0
	ctx.lr = 0x83152FA8;
	sub_831A81E0(ctx, base);
	// 83152FA8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83152FAC: 419A000C  beq cr6, 0x83152fb8
	if ctx.cr[6].eq {
	pc = 0x83152FB8; continue 'dispatch;
	}
	// 83152FB0: 7D7BC214  add r11, r27, r24
	ctx.r[11].u64 = ctx.r[27].u64 + ctx.r[24].u64;
	// 83152FB4: 9A8B0050  stb r20, 0x50(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(80 as u32), ctx.r[20].u8 ) };
	// 83152FB8: 817A0024  lwz r11, 0x24(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 83152FBC: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 83152FC0: 3AD60004  addi r22, r22, 4
	ctx.r[22].s64 = ctx.r[22].s64 + 4;
	// 83152FC4: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 83152FC8: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83152FCC: 4198FE3C  blt cr6, 0x83152e08
	if ctx.cr[6].lt {
	pc = 0x83152E08; continue 'dispatch;
	}
	// 83152FD0: 817A0050  lwz r11, 0x50(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(80 as u32) ) } as u64;
	// 83152FD4: 815A0024  lwz r10, 0x24(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(36 as u32) ) } as u64;
	// 83152FD8: 7D3F5A14  add r9, r31, r11
	ctx.r[9].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 83152FDC: 913A0050  stw r9, 0x50(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 83152FE0: 91580044  stw r10, 0x44(r24)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[24].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 83152FE4: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 83152FE8: 480551B0  b 0x831a8198
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83152FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83152FF0 size=96
    let mut pc: u32 = 0x83152FF0;
    'dispatch: loop {
        match pc {
            0x83152FF0 => {
    //   block [0x83152FF0..0x83153050)
	// 83152FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83152FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83152FF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83152FFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83153000: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83153004: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83153008: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8315300C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83153010: 394B53F8  addi r10, r11, 0x53f8
	ctx.r[10].s64 = ctx.r[11].s64 + 21496;
	// 83153014: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83153018: 4BFF3191  bl 0x831461a8
	ctx.lr = 0x8315301C;
	sub_831461A8(ctx, base);
	// 8315301C: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 83153020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83153024: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83153028: 419A0010  beq cr6, 0x83153038
	if ctx.cr[6].eq {
	pc = 0x83153038; continue 'dispatch;
	}
	// 8315302C: 38800054  li r4, 0x54
	ctx.r[4].s64 = 84;
	// 83153030: 4800CC51  bl 0x8315fc80
	ctx.lr = 0x83153034;
	sub_8315FC80(ctx, base);
	// 83153034: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83153038: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8315303C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83153040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83153044: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83153048: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8315304C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83153050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83153050 size=16
    let mut pc: u32 = 0x83153050;
    'dispatch: loop {
        match pc {
            0x83153050 => {
    //   block [0x83153050..0x83153060)
	// 83153050: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83153054: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83153058: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8315305C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83153060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83153060 size=16
    let mut pc: u32 = 0x83153060;
    'dispatch: loop {
        match pc {
            0x83153060 => {
    //   block [0x83153060..0x83153070)
	// 83153060: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83153064: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83153068: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8315306C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83153070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83153070 size=4
    let mut pc: u32 = 0x83153070;
    'dispatch: loop {
        match pc {
            0x83153070 => {
    //   block [0x83153070..0x83153074)
	// 83153070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83153078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83153078 size=96
    let mut pc: u32 = 0x83153078;
    'dispatch: loop {
        match pc {
            0x83153078 => {
    //   block [0x83153078..0x831530D8)
	// 83153078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315307C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83153080: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83153084: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83153088: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315308C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83153090: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83153094: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83153098: 394B5400  addi r10, r11, 0x5400
	ctx.r[10].s64 = ctx.r[11].s64 + 21504;
	// 8315309C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831530A0: 4BFF3109  bl 0x831461a8
	ctx.lr = 0x831530A4;
	sub_831461A8(ctx, base);
	// 831530A4: 57C907FE  clrlwi r9, r30, 0x1f
	ctx.r[9].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 831530A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831530AC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 831530B0: 419A0010  beq cr6, 0x831530c0
	if ctx.cr[6].eq {
	pc = 0x831530C0; continue 'dispatch;
	}
	// 831530B4: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 831530B8: 4800CBC9  bl 0x8315fc80
	ctx.lr = 0x831530BC;
	sub_8315FC80(ctx, base);
	// 831530BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831530C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831530C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831530C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831530CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831530D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831530D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831530D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831530D8 size=184
    let mut pc: u32 = 0x831530D8;
    'dispatch: loop {
        match pc {
            0x831530D8 => {
    //   block [0x831530D8..0x83153190)
	// 831530D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831530DC: 4805508D  bl 0x831a8168
	ctx.lr = 0x831530E0;
	sub_831A8130(ctx, base);
	// 831530E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831530E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831530E8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 831530EC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831530F0: 4800CBC9  bl 0x8315fcb8
	ctx.lr = 0x831530F4;
	sub_8315FCB8(ctx, base);
	// 831530F4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831530F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831530FC: 3BDC0728  addi r30, r28, 0x728
	ctx.r[30].s64 = ctx.r[28].s64 + 1832;
	// 83153100: 419A0020  beq cr6, 0x83153120
	if ctx.cr[6].eq {
	pc = 0x83153120; continue 'dispatch;
	}
	// 83153104: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 83153108: 38800200  li r4, 0x200
	ctx.r[4].s64 = 512;
	// 8315310C: 38600400  li r3, 0x400
	ctx.r[3].s64 = 1024;
	// 83153110: 4800F519  bl 0x83162628
	ctx.lr = 0x83153114;
	sub_83162628(ctx, base);
	// 83153114: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83153118: 7FC3F214  add r30, r3, r30
	ctx.r[30].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 8315311C: 4082FFE8  bne 0x83153104
	if !ctx.cr[0].eq {
	pc = 0x83153104; continue 'dispatch;
	}
	// 83153120: 2B1D0001  cmplwi cr6, r29, 1
	ctx.cr[6].compare_u32(ctx.r[29].u32, 1 as u32, &mut ctx.xer);
	// 83153124: 41980044  blt cr6, 0x83153168
	if ctx.cr[6].lt {
	pc = 0x83153168; continue 'dispatch;
	}
	// 83153128: 419A0034  beq cr6, 0x8315315c
	if ctx.cr[6].eq {
	pc = 0x8315315C; continue 'dispatch;
	}
	// 8315312C: 2B1D0003  cmplwi cr6, r29, 3
	ctx.cr[6].compare_u32(ctx.r[29].u32, 3 as u32, &mut ctx.xer);
	// 83153130: 41980024  blt cr6, 0x83153154
	if ctx.cr[6].lt {
	pc = 0x83153154; continue 'dispatch;
	}
	// 83153134: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83153138: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8315313C: 388B5408  addi r4, r11, 0x5408
	ctx.r[4].s64 = ctx.r[11].s64 + 21512;
	// 83153140: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83153144: 4800C9E5  bl 0x8315fb28
	ctx.lr = 0x83153148;
	sub_8315FB28(ctx, base);
	// 83153148: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8315314C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83153150: 48055068  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83153154: 397E004C  addi r11, r30, 0x4c
	ctx.r[11].s64 = ctx.r[30].s64 + 76;
	// 83153158: 48000020  b 0x83153178
	pc = 0x83153178; continue 'dispatch;
	// 8315315C: 480069A5  bl 0x83159b00
	ctx.lr = 0x83153160;
	sub_83159B00(ctx, base);
	// 83153160: 7D63F214  add r11, r3, r30
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 83153164: 48000014  b 0x83153178
	pc = 0x83153178; continue 'dispatch;
	// 83153168: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8315316C: 4800CB4D  bl 0x8315fcb8
	ctx.lr = 0x83153170;
	sub_8315FCB8(ctx, base);
	// 83153170: 7D63F214  add r11, r3, r30
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 83153174: 396B0130  addi r11, r11, 0x130
	ctx.r[11].s64 = ctx.r[11].s64 + 304;
	// 83153178: 578A1838  slwi r10, r28, 3
	ctx.r[10].u32 = ctx.r[28].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8315317C: 7D5C5050  subf r10, r28, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[28].s64;
	// 83153180: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83153184: 386B009C  addi r3, r11, 0x9c
	ctx.r[3].s64 = ctx.r[11].s64 + 156;
	// 83153188: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8315318C: 4805502C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83153190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83153190 size=148
    let mut pc: u32 = 0x83153190;
    'dispatch: loop {
        match pc {
            0x83153190 => {
    //   block [0x83153190..0x83153224)
	// 83153190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83153194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83153198: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8315319C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831531A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831531A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831531A8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831531AC: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 831531B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831531B4: 419A0018  beq cr6, 0x831531cc
	if ctx.cr[6].eq {
	pc = 0x831531CC; continue 'dispatch;
	}
	// 831531B8: 80BE0048  lwz r5, 0x48(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 831531BC: 809E0040  lwz r4, 0x40(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 831531C0: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 831531C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831531C8: 4E800421  bctrl
	ctx.lr = 0x831531CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831531CC: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 831531D0: 809E0048  lwz r4, 0x48(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 831531D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831531D8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831531DC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831531E0: 4E800421  bctrl
	ctx.lr = 0x831531E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831531E4: 387F0074  addi r3, r31, 0x74
	ctx.r[3].s64 = ctx.r[31].s64 + 116;
	// 831531E8: 48005829  bl 0x83158a10
	ctx.lr = 0x831531EC;
	sub_83158A10(ctx, base);
	// 831531EC: 2F030008  cmpwi cr6, r3, 8
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8, &mut ctx.xer);
	// 831531F0: 409A001C  bne cr6, 0x8315320c
	if !ctx.cr[6].eq {
	pc = 0x8315320C; continue 'dispatch;
	}
	// 831531F4: 815F0068  lwz r10, 0x68(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 831531F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831531FC: 916A003C  stw r11, 0x3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83153200: 813F0724  lwz r9, 0x724(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1828 as u32) ) } as u64;
	// 83153204: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83153208: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8315320C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83153210: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83153214: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83153218: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8315321C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83153220: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83153228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83153228 size=120
    let mut pc: u32 = 0x83153228;
    'dispatch: loop {
        match pc {
            0x83153228 => {
    //   block [0x83153228..0x831532A0)
	// 83153228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315322C: 48054F41  bl 0x831a816c
	ctx.lr = 0x83153230;
	sub_831A8130(ctx, base);
	// 83153230: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83153234: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83153238: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8315323C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83153240: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83153244: 814B0038  lwz r10, 0x38(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) } as u64;
	// 83153248: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8315324C: 419A000C  beq cr6, 0x83153258
	if ctx.cr[6].eq {
	pc = 0x83153258; continue 'dispatch;
	}
	// 83153250: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 83153254: 409A0014  bne cr6, 0x83153268
	if !ctx.cr[6].eq {
	pc = 0x83153268; continue 'dispatch;
	}
	// 83153258: 93CB003C  stw r30, 0x3c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 8315325C: 93CB0038  stw r30, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 83153260: 9BAB0041  stb r29, 0x41(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(65 as u32), ctx.r[29].u8 ) };
	// 83153264: 93AB0044  stw r29, 0x44(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(68 as u32), ctx.r[29].u32 ) };
	// 83153268: 387F0074  addi r3, r31, 0x74
	ctx.r[3].s64 = ctx.r[31].s64 + 116;
	// 8315326C: 4800675D  bl 0x831599c8
	ctx.lr = 0x83153270;
	sub_831599C8(ctx, base);
	// 83153270: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83153274: C1BF06E0  lfs f13, 0x6e0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1760 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83153278: 93BF06E4  stw r29, 0x6e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1764 as u32), ctx.r[29].u32 ) };
	// 8315327C: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83153280: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 83153284: 40990008  ble cr6, 0x8315328c
	if !ctx.cr[6].gt {
	pc = 0x8315328C; continue 'dispatch;
	}
	// 83153288: 93DF06E8  stw r30, 0x6e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1768 as u32), ctx.r[30].u32 ) };
	// 8315328C: 817F0724  lwz r11, 0x724(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1828 as u32) ) } as u64;
	// 83153290: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 83153294: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83153298: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8315329C: 48054F20  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831532A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831532A0 size=20
    let mut pc: u32 = 0x831532A0;
    'dispatch: loop {
        match pc {
            0x831532A0 => {
    //   block [0x831532A0..0x831532B4)
	// 831532A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831532A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831532A8: 386B0074  addi r3, r11, 0x74
	ctx.r[3].s64 = ctx.r[11].s64 + 116;
	// 831532AC: 914B06E8  stw r10, 0x6e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1768 as u32), ctx.r[10].u32 ) };
	// 831532B0: 48006768  b 0x83159a18
	sub_83159A18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831532B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831532B8 size=20
    let mut pc: u32 = 0x831532B8;
    'dispatch: loop {
        match pc {
            0x831532B8 => {
    //   block [0x831532B8..0x831532CC)
	// 831532B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831532BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 831532C0: 386B0074  addi r3, r11, 0x74
	ctx.r[3].s64 = ctx.r[11].s64 + 116;
	// 831532C4: 914B06E8  stw r10, 0x6e8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1768 as u32), ctx.r[10].u32 ) };
	// 831532C8: 480067A8  b 0x83159a70
	sub_83159A70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831532D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831532D0 size=8
    let mut pc: u32 = 0x831532D0;
    'dispatch: loop {
        match pc {
            0x831532D0 => {
    //   block [0x831532D0..0x831532D8)
	// 831532D0: 9083070C  stw r4, 0x70c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1804 as u32), ctx.r[4].u32 ) };
	// 831532D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831532D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831532D8 size=8
    let mut pc: u32 = 0x831532D8;
    'dispatch: loop {
        match pc {
            0x831532D8 => {
    //   block [0x831532D8..0x831532E0)
	// 831532D8: 8063070C  lwz r3, 0x70c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1804 as u32) ) } as u64;
	// 831532DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831532E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831532E0 size=12
    let mut pc: u32 = 0x831532E0;
    'dispatch: loop {
        match pc {
            0x831532E0 => {
    //   block [0x831532E0..0x831532EC)
	// 831532E0: 9083003C  stw r4, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[4].u32 ) };
	// 831532E4: 90A30040  stw r5, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[5].u32 ) };
	// 831532E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831532F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831532F0 size=128
    let mut pc: u32 = 0x831532F0;
    'dispatch: loop {
        match pc {
            0x831532F0 => {
    //   block [0x831532F0..0x83153370)
	// 831532F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831532F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831532F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831532FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83153300: FC000890  fmr f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = ctx.f[1].f64;
	// 83153304: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83153308: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8315330C: C1AB5448  lfs f13, 0x5448(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21576 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83153310: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83153314: 41980014  blt cr6, 0x83153328
	if ctx.cr[6].lt {
	pc = 0x83153328; continue 'dispatch;
	}
	// 83153318: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8315331C: C1AB5444  lfs f13, 0x5444(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21572 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83153320: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 83153324: 40990008  ble cr6, 0x8315332c
	if !ctx.cr[6].gt {
	pc = 0x8315332C; continue 'dispatch;
	}
	// 83153328: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 8315332C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83153330: 3D408205  lis r10, -0x7dfb
	ctx.r[10].s64 = -2113601536;
	// 83153334: C1AB0F98  lfs f13, 0xf98(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(3992 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83153338: C82AAA10  lfd f1, -0x55f0(r10)
	ctx.f[1].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(-22000 as u32) ) };
	// 8315333C: EC400372  fmuls f2, f0, f13
	ctx.f[2].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 83153340: 48058169  bl 0x831ab4a8
	ctx.lr = 0x83153344;
	sub_831AB4A8(ctx, base);
	// 83153344: FC000818  frsp f0, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (ctx.f[1].f64 as f32) as f64;
	// 83153348: D01F071C  stfs f0, 0x71c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1820 as u32), tmp.u32 ) };
	// 8315334C: 387F0654  addi r3, r31, 0x654
	ctx.r[3].s64 = ctx.r[31].s64 + 1620;
	// 83153350: C1BF0718  lfs f13, 0x718(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1816 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83153354: EC2D0032  fmuls f1, f13, f0
	ctx.f[1].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 83153358: 4BFFEEE9  bl 0x83152240
	ctx.lr = 0x8315335C;
	sub_83152240(ctx, base);
	// 8315335C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83153360: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83153364: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83153368: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8315336C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83153370(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83153370 size=176
    let mut pc: u32 = 0x83153370;
    'dispatch: loop {
        match pc {
            0x83153370 => {
    //   block [0x83153370..0x83153420)
	// 83153370: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83153374: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83153378: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315337C: C0030018  lfs f0, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83153380: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83153384: EDA00172  fmuls f13, f0, f5
	ctx.f[13].f64 = (((ctx.f[0].f64 * ctx.f[5].f64) as f32) as f64);
	// 83153388: D0C30030  stfs f6, 0x30(r3)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 8315338C: ED800132  fmuls f12, f0, f4
	ctx.f[12].f64 = (((ctx.f[0].f64 * ctx.f[4].f64) as f32) as f64);
	// 83153390: D023001C  stfs f1, 0x1c(r3)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 83153394: ED6000F2  fmuls f11, f0, f3
	ctx.f[11].f64 = (((ctx.f[0].f64 * ctx.f[3].f64) as f32) as f64);
	// 83153398: D0430020  stfs f2, 0x20(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 8315339C: ED4000B2  fmuls f10, f0, f2
	ctx.f[10].f64 = (((ctx.f[0].f64 * ctx.f[2].f64) as f32) as f64);
	// 831533A0: D0630024  stfs f3, 0x24(r3)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 831533A4: ED200072  fmuls f9, f0, f1
	ctx.f[9].f64 = (((ctx.f[0].f64 * ctx.f[1].f64) as f32) as f64);
	// 831533A8: D0830028  stfs f4, 0x28(r3)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 831533AC: D0A3002C  stfs f5, 0x2c(r3)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 831533B0: FC203090  fmr f1, f6
	ctx.f[1].f64 = ctx.f[6].f64;
	// 831533B4: C00B9F7C  lfs f0, -0x6084(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24708 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 831533B8: 38630074  addi r3, r3, 0x74
	ctx.r[3].s64 = ctx.r[3].s64 + 116;
	// 831533BC: ED0D0032  fmuls f8, f13, f0
	ctx.f[8].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 831533C0: ECEC0032  fmuls f7, f12, f0
	ctx.f[7].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 831533C4: ECCB0032  fmuls f6, f11, f0
	ctx.f[6].f64 = (((ctx.f[11].f64 * ctx.f[0].f64) as f32) as f64);
	// 831533C8: ECAA0032  fmuls f5, f10, f0
	ctx.f[5].f64 = (((ctx.f[10].f64 * ctx.f[0].f64) as f32) as f64);
	// 831533CC: EC890032  fmuls f4, f9, f0
	ctx.f[4].f64 = (((ctx.f[9].f64 * ctx.f[0].f64) as f32) as f64);
	// 831533D0: FC60465E  fctidz f3, f8
	ctx.f[3].s64 = if ctx.f[8].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[8].f64.trunc() as i64 };
	// 831533D4: D8610050  stfd f3, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[3].u64 ) };
	// 831533D8: FC403E5E  fctidz f2, f7
	ctx.f[2].s64 = if ctx.f[7].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[7].f64.trunc() as i64 };
	// 831533DC: D8410058  stfd f2, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[2].u64 ) };
	// 831533E0: FC00365E  fctidz f0, f6
	ctx.f[0].s64 = if ctx.f[6].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[6].f64.trunc() as i64 };
	// 831533E4: D8010060  stfd f0, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.f[0].u64 ) };
	// 831533E8: FDA02E5E  fctidz f13, f5
	ctx.f[13].s64 = if ctx.f[5].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[5].f64.trunc() as i64 };
	// 831533EC: D9A10068  stfd f13, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.f[13].u64 ) };
	// 831533F0: FD80265E  fctidz f12, f4
	ctx.f[12].s64 = if ctx.f[4].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[4].f64.trunc() as i64 };
	// 831533F4: D9810070  stfd f12, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.f[12].u64 ) };
	// 831533F8: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831533FC: 80E1005C  lwz r7, 0x5c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83153400: 80C10064  lwz r6, 0x64(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83153404: 80A1006C  lwz r5, 0x6c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83153408: 80810074  lwz r4, 0x74(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 8315340C: 480055C5  bl 0x831589d0
	ctx.lr = 0x83153410;
	sub_831589D0(ctx, base);
	// 83153410: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83153414: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83153418: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8315341C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83153420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83153420 size=148
    let mut pc: u32 = 0x83153420;
    'dispatch: loop {
        match pc {
            0x83153420 => {
    //   block [0x83153420..0x831534B4)
	// 83153420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83153424: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83153428: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8315342C: DBA1FFD8  stfd f29, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[29].u64 ) };
	// 83153430: DBC1FFE0  stfd f30, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[30].u64 ) };
	// 83153434: DBE1FFE8  stfd f31, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 83153438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315343C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83153440: FFA00890  fmr f29, f1
	ctx.f[29].f64 = ctx.f[1].f64;
	// 83153444: FFE01090  fmr f31, f2
	ctx.f[31].f64 = ctx.f[2].f64;
	// 83153448: 4BFFF249  bl 0x83152690
	ctx.lr = 0x8315344C;
	sub_83152690(ctx, base);
	// 8315344C: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 83153450: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 83153454: 4BFFF23D  bl 0x83152690
	ctx.lr = 0x83153458;
	sub_83152690(ctx, base);
	// 83153458: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8315345C: FC400890  fmr f2, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[1].f64;
	// 83153460: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83153464: FF1D0000  fcmpu cr6, f29, f0
	ctx.cr[6].compare_f64(ctx.f[29].f64, ctx.f[0].f64);
	// 83153468: 409A0020  bne cr6, 0x83153488
	if !ctx.cr[6].eq {
	pc = 0x83153488; continue 'dispatch;
	}
	// 8315346C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83153470: C1AB08A8  lfs f13, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83153474: FF1F6800  fcmpu cr6, f31, f13
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[13].f64);
	// 83153478: 409A0010  bne cr6, 0x83153488
	if !ctx.cr[6].eq {
	pc = 0x83153488; continue 'dispatch;
	}
	// 8315347C: FC400090  fmr f2, f0
	ctx.f[2].f64 = ctx.f[0].f64;
	// 83153480: FC200090  fmr f1, f0
	ctx.f[1].f64 = ctx.f[0].f64;
	// 83153484: 48000008  b 0x8315348c
	pc = 0x8315348C; continue 'dispatch;
	// 83153488: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 8315348C: 387F0188  addi r3, r31, 0x188
	ctx.r[3].s64 = ctx.r[31].s64 + 392;
	// 83153490: 48006C69  bl 0x8315a0f8
	ctx.lr = 0x83153494;
	sub_8315A0F8(ctx, base);
	// 83153494: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83153498: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8315349C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831534A0: CBA1FFD8  lfd f29, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 831534A4: CBC1FFE0  lfd f30, -0x20(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 831534A8: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831534AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831534B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831534B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831534B8 size=24
    let mut pc: u32 = 0x831534B8;
    'dispatch: loop {
        match pc {
            0x831534B8 => {
    //   block [0x831534B8..0x831534D0)
	// 831534B8: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 831534BC: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 831534C0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831534C4: 80830720  lwz r4, 0x720(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1824 as u32) ) } as u64;
	// 831534C8: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831534CC: 4BFF61AC  b 0x83149678
	sub_83149678(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831534D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x831534D0 size=8
    let mut pc: u32 = 0x831534D0;
    'dispatch: loop {
        match pc {
            0x831534D0 => {
    //   block [0x831534D0..0x831534D8)
	// 831534D0: D02306E0  stfs f1, 0x6e0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1760 as u32), tmp.u32 ) };
	// 831534D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831534D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831534D8 size=12
    let mut pc: u32 = 0x831534D8;
    'dispatch: loop {
        match pc {
            0x831534D8 => {
    //   block [0x831534D8..0x831534E4)
	// 831534D8: 8163006C  lwz r11, 0x6c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(108 as u32) ) } as u64;
	// 831534DC: 806B0050  lwz r3, 0x50(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(80 as u32) ) } as u64;
	// 831534E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831534E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831534E8 size=420
    let mut pc: u32 = 0x831534E8;
    'dispatch: loop {
        match pc {
            0x831534E8 => {
    //   block [0x831534E8..0x8315368C)
	// 831534E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831534EC: 48054C79  bl 0x831a8164
	ctx.lr = 0x831534F0;
	sub_831A8130(ctx, base);
	// 831534F0: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 831534F4: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 831534F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831534FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83153500: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 83153504: 7D5B5378  mr r27, r10
	ctx.r[27].u64 = ctx.r[10].u64;
	// 83153508: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 8315350C: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83153510: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 83153514: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83153518: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 8315351C: 38AA5490  addi r5, r10, 0x5490
	ctx.r[5].s64 = ctx.r[10].s64 + 21648;
	// 83153520: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 83153524: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83153528: C3EB08A4  lfs f31, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8315352C: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83153530: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83153534: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 83153538: D3DF0018  stfs f30, 0x18(r31)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8315353C: D3FF001C  stfs f31, 0x1c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 83153540: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 83153544: D3FF0020  stfs f31, 0x20(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 83153548: 90FF0014  stw r7, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[7].u32 ) };
	// 8315354C: D3FF0024  stfs f31, 0x24(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 83153550: 3C608219  lis r3, -0x7de7
	ctx.r[3].s64 = -2112290816;
	// 83153554: D3FF0028  stfs f31, 0x28(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), tmp.u32 ) };
	// 83153558: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8315355C: D3FF002C  stfs f31, 0x2c(r31)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 83153560: 38A35484  addi r5, r3, 0x5484
	ctx.r[5].s64 = ctx.r[3].s64 + 21636;
	// 83153564: C0085B70  lfs f0, 0x5b70(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(23408 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83153568: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 8315356C: D01F0030  stfs f0, 0x30(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 83153570: 90DF0038  stw r6, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[6].u32 ) };
	// 83153574: 93DF003C  stw r30, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[30].u32 ) };
	// 83153578: 387F0074  addi r3, r31, 0x74
	ctx.r[3].s64 = ctx.r[31].s64 + 116;
	// 8315357C: 93DF0040  stw r30, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[30].u32 ) };
	// 83153580: 913F0044  stw r9, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 83153584: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 83153588: 93DF006C  stw r30, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[30].u32 ) };
	// 8315358C: 93DF0070  stw r30, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[30].u32 ) };
	// 83153590: 4BFFEB11  bl 0x831520a0
	ctx.lr = 0x83153594;
	sub_831520A0(ctx, base);
	// 83153594: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83153598: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8315359C: 93DF0184  stw r30, 0x184(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), ctx.r[30].u32 ) };
	// 831535A0: 38AB5474  addi r5, r11, 0x5474
	ctx.r[5].s64 = ctx.r[11].s64 + 21620;
	// 831535A4: 387F0188  addi r3, r31, 0x188
	ctx.r[3].s64 = ctx.r[31].s64 + 392;
	// 831535A8: 4BFFF189  bl 0x83152730
	ctx.lr = 0x831535AC;
	sub_83152730(ctx, base);
	// 831535AC: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 831535B0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831535B4: 93DF0650  stw r30, 0x650(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1616 as u32), ctx.r[30].u32 ) };
	// 831535B8: 38AA5464  addi r5, r10, 0x5464
	ctx.r[5].s64 = ctx.r[10].s64 + 21604;
	// 831535BC: 387F0654  addi r3, r31, 0x654
	ctx.r[3].s64 = ctx.r[31].s64 + 1620;
	// 831535C0: 4BFFEC11  bl 0x831521d0
	ctx.lr = 0x831535C4;
	sub_831521D0(ctx, base);
	// 831535C4: 3D208219  lis r9, -0x7de7
	ctx.r[9].s64 = -2112290816;
	// 831535C8: 3D008219  lis r8, -0x7de7
	ctx.r[8].s64 = -2112290816;
	// 831535CC: 93DF06C0  stw r30, 0x6c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1728 as u32), ctx.r[30].u32 ) };
	// 831535D0: 38A95458  addi r5, r9, 0x5458
	ctx.r[5].s64 = ctx.r[9].s64 + 21592;
	// 831535D4: D3FF06E0  stfs f31, 0x6e0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1760 as u32), tmp.u32 ) };
	// 831535D8: 3CE08219  lis r7, -0x7de7
	ctx.r[7].s64 = -2112290816;
	// 831535DC: 939F06DC  stw r28, 0x6dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1756 as u32), ctx.r[28].u32 ) };
	// 831535E0: 3CC08219  lis r6, -0x7de7
	ctx.r[6].s64 = -2112290816;
	// 831535E4: 93DF06CC  stw r30, 0x6cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1740 as u32), ctx.r[30].u32 ) };
	// 831535E8: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 831535EC: 93DF06D0  stw r30, 0x6d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1744 as u32), ctx.r[30].u32 ) };
	// 831535F0: 388853E8  addi r4, r8, 0x53e8
	ctx.r[4].s64 = ctx.r[8].s64 + 21480;
	// 831535F4: 93DF06D4  stw r30, 0x6d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1748 as u32), ctx.r[30].u32 ) };
	// 831535F8: 3867544C  addi r3, r7, 0x544c
	ctx.r[3].s64 = ctx.r[7].s64 + 21580;
	// 831535FC: 93DF06D8  stw r30, 0x6d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1752 as u32), ctx.r[30].u32 ) };
	// 83153600: 394653F0  addi r10, r6, 0x53f0
	ctx.r[10].s64 = ctx.r[6].s64 + 21488;
	// 83153604: 93DF06E4  stw r30, 0x6e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1764 as u32), ctx.r[30].u32 ) };
	// 83153608: 93DF06E8  stw r30, 0x6e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1768 as u32), ctx.r[30].u32 ) };
	// 8315360C: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 83153610: 90BF06C8  stw r5, 0x6c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1736 as u32), ctx.r[5].u32 ) };
	// 83153614: C00908A8  lfs f0, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83153618: 909F06C4  stw r4, 0x6c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1732 as u32), ctx.r[4].u32 ) };
	// 8315361C: 397F0048  addi r11, r31, 0x48
	ctx.r[11].s64 = ctx.r[31].s64 + 72;
	// 83153620: 93DF06EC  stw r30, 0x6ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1772 as u32), ctx.r[30].u32 ) };
	// 83153624: 907F06F4  stw r3, 0x6f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1780 as u32), ctx.r[3].u32 ) };
	// 83153628: 939F0708  stw r28, 0x708(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1800 as u32), ctx.r[28].u32 ) };
	// 8315362C: 93DF06F8  stw r30, 0x6f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1784 as u32), ctx.r[30].u32 ) };
	// 83153630: 915F06F0  stw r10, 0x6f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1776 as u32), ctx.r[10].u32 ) };
	// 83153634: 93DF06FC  stw r30, 0x6fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1788 as u32), ctx.r[30].u32 ) };
	// 83153638: 93DF0700  stw r30, 0x700(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1792 as u32), ctx.r[30].u32 ) };
	// 8315363C: 93DF0704  stw r30, 0x704(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1796 as u32), ctx.r[30].u32 ) };
	// 83153640: 93DF070C  stw r30, 0x70c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1804 as u32), ctx.r[30].u32 ) };
	// 83153644: 39400008  li r10, 8
	ctx.r[10].s64 = 8;
	// 83153648: 93DF0710  stw r30, 0x710(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1808 as u32), ctx.r[30].u32 ) };
	// 8315364C: 93DF0714  stw r30, 0x714(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1812 as u32), ctx.r[30].u32 ) };
	// 83153650: C1BD000C  lfs f13, 0xc(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83153654: ED9E6824  fdivs f12, f30, f13
	ctx.f[12].f64 = ((ctx.f[30].f64 / ctx.f[13].f64) as f32) as f64;
	// 83153658: D19F0718  stfs f12, 0x718(r31)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1816 as u32), tmp.u32 ) };
	// 8315365C: D01F071C  stfs f0, 0x71c(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1820 as u32), tmp.u32 ) };
	// 83153660: 911F0720  stw r8, 0x720(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1824 as u32), ctx.r[8].u32 ) };
	// 83153664: 937F0724  stw r27, 0x724(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1828 as u32), ctx.r[27].u32 ) };
	// 83153668: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8315366C: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83153670: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 83153674: 4200FFF8  bdnz 0x8315366c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8315366C; continue 'dispatch;
	}
	// 83153678: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8315367C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83153680: CBC1FFC0  lfd f30, -0x40(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-64 as u32) ) };
	// 83153684: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 83153688: 48054B2C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83153690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83153690 size=184
    let mut pc: u32 = 0x83153690;
    'dispatch: loop {
        match pc {
            0x83153690 => {
    //   block [0x83153690..0x83153748)
	// 83153690: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83153694: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83153698: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8315369C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831536A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831536A4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831536A8: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 831536AC: 392B5490  addi r9, r11, 0x5490
	ctx.r[9].s64 = ctx.r[11].s64 + 21648;
	// 831536B0: 390A53F0  addi r8, r10, 0x53f0
	ctx.r[8].s64 = ctx.r[10].s64 + 21488;
	// 831536B4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 831536B8: 387F06F0  addi r3, r31, 0x6f0
	ctx.r[3].s64 = ctx.r[31].s64 + 1776;
	// 831536BC: 911F06F0  stw r8, 0x6f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1776 as u32), ctx.r[8].u32 ) };
	// 831536C0: 4BFF2AE9  bl 0x831461a8
	ctx.lr = 0x831536C4;
	sub_831461A8(ctx, base);
	// 831536C4: 3CE08219  lis r7, -0x7de7
	ctx.r[7].s64 = -2112290816;
	// 831536C8: 387F06C4  addi r3, r31, 0x6c4
	ctx.r[3].s64 = ctx.r[31].s64 + 1732;
	// 831536CC: 38C753E8  addi r6, r7, 0x53e8
	ctx.r[6].s64 = ctx.r[7].s64 + 21480;
	// 831536D0: 90DF06C4  stw r6, 0x6c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1732 as u32), ctx.r[6].u32 ) };
	// 831536D4: 4BFF2AD5  bl 0x831461a8
	ctx.lr = 0x831536D8;
	sub_831461A8(ctx, base);
	// 831536D8: 3CA08219  lis r5, -0x7de7
	ctx.r[5].s64 = -2112290816;
	// 831536DC: 387F0654  addi r3, r31, 0x654
	ctx.r[3].s64 = ctx.r[31].s64 + 1620;
	// 831536E0: 3885537C  addi r4, r5, 0x537c
	ctx.r[4].s64 = ctx.r[5].s64 + 21372;
	// 831536E4: 909F0654  stw r4, 0x654(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1620 as u32), ctx.r[4].u32 ) };
	// 831536E8: 4BFF2AC1  bl 0x831461a8
	ctx.lr = 0x831536EC;
	sub_831461A8(ctx, base);
	// 831536EC: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831536F0: 387F0188  addi r3, r31, 0x188
	ctx.r[3].s64 = ctx.r[31].s64 + 392;
	// 831536F4: 392B53E0  addi r9, r11, 0x53e0
	ctx.r[9].s64 = ctx.r[11].s64 + 21472;
	// 831536F8: 396304BC  addi r11, r3, 0x4bc
	ctx.r[11].s64 = ctx.r[3].s64 + 1212;
	// 831536FC: 913F0188  stw r9, 0x188(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(392 as u32), ctx.r[9].u32 ) };
	// 83153700: 3D208219  lis r9, -0x7de7
	ctx.r[9].s64 = -2112290816;
	// 83153704: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 83153708: 39293114  addi r9, r9, 0x3114
	ctx.r[9].s64 = ctx.r[9].s64 + 12564;
	// 8315370C: 396BFF6C  addi r11, r11, -0x94
	ctx.r[11].s64 = ctx.r[11].s64 + -148;
	// 83153710: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83153714: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83153718: 4080FFF4  bge 0x8315370c
	if !ctx.cr[0].lt {
	pc = 0x8315370C; continue 'dispatch;
	}
	// 8315371C: 4BFF2A8D  bl 0x831461a8
	ctx.lr = 0x83153720;
	sub_831461A8(ctx, base);
	// 83153720: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83153724: 387F0074  addi r3, r31, 0x74
	ctx.r[3].s64 = ctx.r[31].s64 + 116;
	// 83153728: 394B5374  addi r10, r11, 0x5374
	ctx.r[10].s64 = ctx.r[11].s64 + 21364;
	// 8315372C: 915F0074  stw r10, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 83153730: 4BFF2A79  bl 0x831461a8
	ctx.lr = 0x83153734;
	sub_831461A8(ctx, base);
	// 83153734: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83153738: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8315373C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83153740: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83153744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83153748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83153748 size=380
    let mut pc: u32 = 0x83153748;
    'dispatch: loop {
        match pc {
            0x83153748 => {
    //   block [0x83153748..0x831538C4)
	// 83153748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315374C: 48054A21  bl 0x831a816c
	ctx.lr = 0x83153750;
	sub_831A8130(ctx, base);
	// 83153750: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83153754: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83153758: 809F0720  lwz r4, 0x720(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1824 as u32) ) } as u64;
	// 8315375C: 2F04FFFF  cmpwi cr6, r4, -1
	ctx.cr[6].compare_i32(ctx.r[4].s32, -1, &mut ctx.xer);
	// 83153760: 419A0010  beq cr6, 0x83153770
	if ctx.cr[6].eq {
	pc = 0x83153770; continue 'dispatch;
	}
	// 83153764: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83153768: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8315376C: 4BFF5E1D  bl 0x83149588
	ctx.lr = 0x83153770;
	sub_83149588(ctx, base);
	// 83153770: 809F0714  lwz r4, 0x714(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1812 as u32) ) } as u64;
	// 83153774: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83153778: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8315377C: 419A001C  beq cr6, 0x83153798
	if ctx.cr[6].eq {
	pc = 0x83153798; continue 'dispatch;
	}
	// 83153780: 80BF0710  lwz r5, 0x710(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1808 as u32) ) } as u64;
	// 83153784: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83153788: 419A0010  beq cr6, 0x83153798
	if ctx.cr[6].eq {
	pc = 0x83153798; continue 'dispatch;
	}
	// 8315378C: 387F06F0  addi r3, r31, 0x6f0
	ctx.r[3].s64 = ctx.r[31].s64 + 1776;
	// 83153790: 4BFF2C71  bl 0x83146400
	ctx.lr = 0x83153794;
	sub_83146400(ctx, base);
	// 83153794: 93BF0710  stw r29, 0x710(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1808 as u32), ctx.r[29].u32 ) };
	// 83153798: 80BF06EC  lwz r5, 0x6ec(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1772 as u32) ) } as u64;
	// 8315379C: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 831537A0: 419A0014  beq cr6, 0x831537b4
	if ctx.cr[6].eq {
	pc = 0x831537B4; continue 'dispatch;
	}
	// 831537A4: 389F06F0  addi r4, r31, 0x6f0
	ctx.r[4].s64 = ctx.r[31].s64 + 1776;
	// 831537A8: 387F06C4  addi r3, r31, 0x6c4
	ctx.r[3].s64 = ctx.r[31].s64 + 1732;
	// 831537AC: 4BFF2C55  bl 0x83146400
	ctx.lr = 0x831537B0;
	sub_83146400(ctx, base);
	// 831537B0: 93BF06EC  stw r29, 0x6ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1772 as u32), ctx.r[29].u32 ) };
	// 831537B4: 80BF06C0  lwz r5, 0x6c0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1728 as u32) ) } as u64;
	// 831537B8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 831537BC: 419A0014  beq cr6, 0x831537d0
	if ctx.cr[6].eq {
	pc = 0x831537D0; continue 'dispatch;
	}
	// 831537C0: 389F06C4  addi r4, r31, 0x6c4
	ctx.r[4].s64 = ctx.r[31].s64 + 1732;
	// 831537C4: 387F0654  addi r3, r31, 0x654
	ctx.r[3].s64 = ctx.r[31].s64 + 1620;
	// 831537C8: 4BFF2C39  bl 0x83146400
	ctx.lr = 0x831537CC;
	sub_83146400(ctx, base);
	// 831537CC: 93BF06C0  stw r29, 0x6c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1728 as u32), ctx.r[29].u32 ) };
	// 831537D0: 80BF0650  lwz r5, 0x650(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1616 as u32) ) } as u64;
	// 831537D4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 831537D8: 419A0014  beq cr6, 0x831537ec
	if ctx.cr[6].eq {
	pc = 0x831537EC; continue 'dispatch;
	}
	// 831537DC: 389F0654  addi r4, r31, 0x654
	ctx.r[4].s64 = ctx.r[31].s64 + 1620;
	// 831537E0: 387F0188  addi r3, r31, 0x188
	ctx.r[3].s64 = ctx.r[31].s64 + 392;
	// 831537E4: 4BFF2C1D  bl 0x83146400
	ctx.lr = 0x831537E8;
	sub_83146400(ctx, base);
	// 831537E8: 93BF0650  stw r29, 0x650(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1616 as u32), ctx.r[29].u32 ) };
	// 831537EC: 80BF0184  lwz r5, 0x184(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(388 as u32) ) } as u64;
	// 831537F0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 831537F4: 419A0014  beq cr6, 0x83153808
	if ctx.cr[6].eq {
	pc = 0x83153808; continue 'dispatch;
	}
	// 831537F8: 389F0188  addi r4, r31, 0x188
	ctx.r[4].s64 = ctx.r[31].s64 + 392;
	// 831537FC: 387F0074  addi r3, r31, 0x74
	ctx.r[3].s64 = ctx.r[31].s64 + 116;
	// 83153800: 4BFF2C01  bl 0x83146400
	ctx.lr = 0x83153804;
	sub_83146400(ctx, base);
	// 83153804: 93BF0184  stw r29, 0x184(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), ctx.r[29].u32 ) };
	// 83153808: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8315380C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83153810: 419A001C  beq cr6, 0x8315382c
	if ctx.cr[6].eq {
	pc = 0x8315382C; continue 'dispatch;
	}
	// 83153814: 80BF0070  lwz r5, 0x70(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(112 as u32) ) } as u64;
	// 83153818: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8315381C: 419A0010  beq cr6, 0x8315382c
	if ctx.cr[6].eq {
	pc = 0x8315382C; continue 'dispatch;
	}
	// 83153820: 389F0074  addi r4, r31, 0x74
	ctx.r[4].s64 = ctx.r[31].s64 + 116;
	// 83153824: 4BFF2BDD  bl 0x83146400
	ctx.lr = 0x83153828;
	sub_83146400(ctx, base);
	// 83153828: 93BF0070  stw r29, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[29].u32 ) };
	// 8315382C: 807F0714  lwz r3, 0x714(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1812 as u32) ) } as u64;
	// 83153830: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83153834: 419A001C  beq cr6, 0x83153850
	if ctx.cr[6].eq {
	pc = 0x83153850; continue 'dispatch;
	}
	// 83153838: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8315383C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83153840: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83153844: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83153848: 4E800421  bctrl
	ctx.lr = 0x8315384C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8315384C: 93BF0714  stw r29, 0x714(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1812 as u32), ctx.r[29].u32 ) };
	// 83153850: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 83153854: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83153858: 419A001C  beq cr6, 0x83153874
	if ctx.cr[6].eq {
	pc = 0x83153874; continue 'dispatch;
	}
	// 8315385C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83153860: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83153864: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83153868: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8315386C: 4E800421  bctrl
	ctx.lr = 0x83153870;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83153870: 93BF006C  stw r29, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[29].u32 ) };
	// 83153874: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83153878: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8315387C: 419A0018  beq cr6, 0x83153894
	if ctx.cr[6].eq {
	pc = 0x83153894; continue 'dispatch;
	}
	// 83153880: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83153884: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83153888: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8315388C: 4E800421  bctrl
	ctx.lr = 0x83153890;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83153890: 93BF0068  stw r29, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 83153894: 3BFF0048  addi r31, r31, 0x48
	ctx.r[31].s64 = ctx.r[31].s64 + 72;
	// 83153898: 3BC00008  li r30, 8
	ctx.r[30].s64 = 8;
	// 8315389C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831538A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831538A4: 419A000C  beq cr6, 0x831538b0
	if ctx.cr[6].eq {
	pc = 0x831538B0; continue 'dispatch;
	}
	// 831538A8: 4800E2A1  bl 0x83161b48
	ctx.lr = 0x831538AC;
	sub_83161B48(ctx, base);
	// 831538AC: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 831538B0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 831538B4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 831538B8: 4082FFE4  bne 0x8315389c
	if !ctx.cr[0].eq {
	pc = 0x8315389C; continue 'dispatch;
	}
	// 831538BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831538C0: 480548FC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831538C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831538C8 size=180
    let mut pc: u32 = 0x831538C8;
    'dispatch: loop {
        match pc {
            0x831538C8 => {
    //   block [0x831538C8..0x8315397C)
	// 831538C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831538CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831538D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831538D4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831538D8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 831538DC: 38AB54C0  addi r5, r11, 0x54c0
	ctx.r[5].s64 = ctx.r[11].s64 + 21696;
	// 831538E0: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 831538E4: 3860004C  li r3, 0x4c
	ctx.r[3].s64 = 76;
	// 831538E8: 4800C411  bl 0x8315fcf8
	ctx.lr = 0x831538EC;
	sub_8315FCF8(ctx, base);
	// 831538EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831538F0: 419A0064  beq cr6, 0x83153954
	if ctx.cr[6].eq {
	pc = 0x83153954; continue 'dispatch;
	}
	// 831538F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831538F8: 39430008  addi r10, r3, 8
	ctx.r[10].s64 = ctx.r[3].s64 + 8;
	// 831538FC: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83153900: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 83153904: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83153908: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8315390C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83153910: 4200FFF8  bdnz 0x83153908
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83153908; continue 'dispatch;
	}
	// 83153914: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83153918: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8315391C: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83153920: 91630030  stw r11, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83153924: 392A5368  addi r9, r10, 0x5368
	ctx.r[9].s64 = ctx.r[10].s64 + 21352;
	// 83153928: 91630034  stw r11, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8315392C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83153930: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83153934: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 83153938: 99630040  stb r11, 0x40(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[11].u8 ) };
	// 8315393C: 99630041  stb r11, 0x41(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(65 as u32), ctx.r[11].u8 ) };
	// 83153940: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83153944: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83153948: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8315394C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83153950: 4E800020  blr
	return;
	// 83153954: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83153958: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 8315395C: 388B54CC  addi r4, r11, 0x54cc
	ctx.r[4].s64 = ctx.r[11].s64 + 21708;
	// 83153960: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83153964: 4800C1DD  bl 0x8315fb40
	ctx.lr = 0x83153968;
	sub_8315FB40(ctx, base);
	// 83153968: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8315396C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83153970: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83153974: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83153978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83153980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83153980 size=628
    let mut pc: u32 = 0x83153980;
    'dispatch: loop {
        match pc {
            0x83153980 => {
    //   block [0x83153980..0x83153BF4)
	// 83153980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83153984: 480547CD  bl 0x831a8150
	ctx.lr = 0x83153988;
	sub_831A8130(ctx, base);
	// 83153988: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315398C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83153990: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83153994: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83153998: 419A0254  beq cr6, 0x83153bec
	if ctx.cr[6].eq {
	pc = 0x83153BEC; continue 'dispatch;
	}
	// 8315399C: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 831539A0: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 831539A4: 3AC0FFFF  li r22, -1
	ctx.r[22].s64 = -1;
	// 831539A8: 3AE00002  li r23, 2
	ctx.r[23].s64 = 2;
	// 831539AC: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 831539B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831539B4: 409A0008  bne cr6, 0x831539bc
	if !ctx.cr[6].eq {
	pc = 0x831539BC; continue 'dispatch;
	}
	// 831539B8: 933F0038  stw r25, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[25].u32 ) };
	// 831539BC: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 831539C0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831539C4: 409A0218  bne cr6, 0x83153bdc
	if !ctx.cr[6].eq {
	pc = 0x83153BDC; continue 'dispatch;
	}
	// 831539C8: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 831539CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831539D0: 409A0048  bne cr6, 0x83153a18
	if !ctx.cr[6].eq {
	pc = 0x83153A18; continue 'dispatch;
	}
	// 831539D4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 831539D8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831539DC: 4800E2CD  bl 0x83161ca8
	ctx.lr = 0x831539E0;
	sub_83161CA8(ctx, base);
	// 831539E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831539E4: 80A10064  lwz r5, 0x64(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 831539E8: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 831539EC: 4BFFE51D  bl 0x83151f08
	ctx.lr = 0x831539F0;
	sub_83151F08(ctx, base);
	// 831539F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831539F4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831539F8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 831539FC: 419A0014  beq cr6, 0x83153a10
	if ctx.cr[6].eq {
	pc = 0x83153A10; continue 'dispatch;
	}
	// 83153A00: 931F0048  stw r24, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[24].u32 ) };
	// 83153A04: 38A0002C  li r5, 0x2c
	ctx.r[5].s64 = 44;
	// 83153A08: 933F0044  stw r25, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[25].u32 ) };
	// 83153A0C: 48000008  b 0x83153a14
	pc = 0x83153A14; continue 'dispatch;
	// 83153A10: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83153A14: 4800E2A5  bl 0x83161cb8
	ctx.lr = 0x83153A18;
	sub_83161CB8(ctx, base);
	// 83153A18: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83153A1C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83153A20: 409A0194  bne cr6, 0x83153bb4
	if !ctx.cr[6].eq {
	pc = 0x83153BB4; continue 'dispatch;
	}
	// 83153A24: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83153A28: 7F3ACB78  mr r26, r25
	ctx.r[26].u64 = ctx.r[25].u64;
	// 83153A2C: 815F0030  lwz r10, 0x30(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83153A30: 93210050  stw r25, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[25].u32 ) };
	// 83153A34: 7D2B5051  subf. r9, r11, r10
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83153A38: 4082000C  bne 0x83153a44
	if !ctx.cr[0].eq {
	pc = 0x83153A44; continue 'dispatch;
	}
	// 83153A3C: 933F0048  stw r25, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[25].u32 ) };
	// 83153A40: 4BFFFF6C  b 0x831539ac
	pc = 0x831539AC; continue 'dispatch;
	// 83153A44: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83153A48: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83153A4C: 4800E25D  bl 0x83161ca8
	ctx.lr = 0x83153A50;
	sub_83161CA8(ctx, base);
	// 83153A50: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83153A54: 7F3BCB78  mr r27, r25
	ctx.r[27].u64 = ctx.r[25].u64;
	// 83153A58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83153A5C: 40990058  ble cr6, 0x83153ab4
	if !ctx.cr[6].gt {
	pc = 0x83153AB4; continue 'dispatch;
	}
	// 83153A60: 3BA10070  addi r29, r1, 0x70
	ctx.r[29].s64 = ctx.r[1].s64 + 112;
	// 83153A64: 3BC10090  addi r30, r1, 0x90
	ctx.r[30].s64 = ctx.r[1].s64 + 144;
	// 83153A68: 3B9F0008  addi r28, r31, 8
	ctx.r[28].s64 = ctx.r[31].s64 + 8;
	// 83153A6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83153A70: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 83153A74: 4800E1A5  bl 0x83161c18
	ctx.lr = 0x83153A78;
	sub_83161C18(ctx, base);
	// 83153A78: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83153A7C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83153A80: 419A011C  beq cr6, 0x83153b9c
	if ctx.cr[6].eq {
	pc = 0x83153B9C; continue 'dispatch;
	}
	// 83153A84: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83153A88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83153A8C: 419A0110  beq cr6, 0x83153b9c
	if ctx.cr[6].eq {
	pc = 0x83153B9C; continue 'dispatch;
	}
	// 83153A90: 813F002C  lwz r9, 0x2c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83153A94: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 83153A98: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83153A9C: 557AF0BE  srwi r26, r11, 2
	ctx.r[26].u32 = ctx.r[11].u32.wrapping_shr(2);
	ctx.r[26].u64 = ctx.r[26].u32 as u64;
	// 83153AA0: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 83153AA4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 83153AA8: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 83153AAC: 7F1B4840  cmplw cr6, r27, r9
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83153AB0: 4198FFBC  blt cr6, 0x83153a6c
	if ctx.cr[6].lt {
	pc = 0x83153A6C; continue 'dispatch;
	}
	// 83153AB4: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83153AB8: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 83153ABC: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83153AC0: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 83153AC4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 83153AC8: 80C1005C  lwz r6, 0x5c(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83153ACC: 7C8A5850  subf r4, r10, r11
	ctx.r[4].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83153AD0: 80A10058  lwz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83153AD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83153AD8: 4BFFE4D9  bl 0x83151fb0
	ctx.lr = 0x83153ADC;
	sub_83151FB0(ctx, base);
	// 83153ADC: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83153AE0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83153AE4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83153AE8: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83153AEC: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83153AF0: 7D2BF214  add r9, r11, r30
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83153AF4: 913F0044  stw r9, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[9].u32 ) };
	// 83153AF8: 4800E1C1  bl 0x83161cb8
	ctx.lr = 0x83153AFC;
	sub_83161CB8(ctx, base);
	// 83153AFC: 811F002C  lwz r8, 0x2c(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83153B00: 7F3CCB78  mr r28, r25
	ctx.r[28].u64 = ctx.r[25].u64;
	// 83153B04: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 83153B08: 40990038  ble cr6, 0x83153b40
	if !ctx.cr[6].gt {
	pc = 0x83153B40; continue 'dispatch;
	}
	// 83153B0C: 57DB103A  slwi r27, r30, 2
	ctx.r[27].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[27].u64 = ctx.r[27].u32 as u64;
	// 83153B10: 3BA10090  addi r29, r1, 0x90
	ctx.r[29].s64 = ctx.r[1].s64 + 144;
	// 83153B14: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 83153B18: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83153B1C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83153B20: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83153B24: 4800E105  bl 0x83161c28
	ctx.lr = 0x83153B28;
	sub_83161C28(ctx, base);
	// 83153B28: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83153B2C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83153B30: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83153B34: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 83153B38: 7F1C5840  cmplw cr6, r28, r11
	ctx.cr[6].compare_u32(ctx.r[28].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83153B3C: 4198FFDC  blt cr6, 0x83153b18
	if ctx.cr[6].lt {
	pc = 0x83153B18; continue 'dispatch;
	}
	// 83153B40: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83153B44: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83153B48: 7D2A5851  subf. r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83153B4C: 41810038  bgt 0x83153b84
	if ctx.cr[0].gt {
	pc = 0x83153B84; continue 'dispatch;
	}
	// 83153B50: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83153B54: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83153B58: 7EC5B378  mr r5, r22
	ctx.r[5].u64 = ctx.r[22].u64;
	// 83153B5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83153B60: 4800E019  bl 0x83161b78
	ctx.lr = 0x83153B64;
	sub_83161B78(ctx, base);
	// 83153B64: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83153B68: 2B0B002C  cmplwi cr6, r11, 0x2c
	ctx.cr[6].compare_u32(ctx.r[11].u32, 44 as u32, &mut ctx.xer);
	// 83153B6C: 41990008  bgt cr6, 0x83153b74
	if ctx.cr[6].gt {
	pc = 0x83153B74; continue 'dispatch;
	}
	// 83153B70: 92FF0038  stw r23, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[23].u32 ) };
	// 83153B74: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83153B78: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83153B7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83153B80: 4800E009  bl 0x83161b88
	ctx.lr = 0x83153B84;
	sub_83161B88(ctx, base);
	// 83153B84: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83153B88: 815F0044  lwz r10, 0x44(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83153B8C: 7D2A5851  subf. r9, r10, r11
	ctx.r[9].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 83153B90: 40820024  bne 0x83153bb4
	if !ctx.cr[0].eq {
	pc = 0x83153BB4; continue 'dispatch;
	}
	// 83153B94: 933F0048  stw r25, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[25].u32 ) };
	// 83153B98: 4BFFFE14  b 0x831539ac
	pc = 0x831539AC; continue 'dispatch;
	// 83153B9C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83153BA0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83153BA4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 83153BA8: 4800E111  bl 0x83161cb8
	ctx.lr = 0x83153BAC;
	sub_83161CB8(ctx, base);
	// 83153BAC: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 83153BB0: 480545F0  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 83153BB4: 897F0041  lbz r11, 0x41(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(65 as u32) ) } as u64;
	// 83153BB8: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83153BBC: 409A0030  bne cr6, 0x83153bec
	if !ctx.cr[6].eq {
	pc = 0x83153BEC; continue 'dispatch;
	}
	// 83153BC0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83153BC4: 4800E0DD  bl 0x83161ca0
	ctx.lr = 0x83153BC8;
	sub_83161CA0(ctx, base);
	// 83153BC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83153BCC: 409A0020  bne cr6, 0x83153bec
	if !ctx.cr[6].eq {
	pc = 0x83153BEC; continue 'dispatch;
	}
	// 83153BD0: 92FF0038  stw r23, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[23].u32 ) };
	// 83153BD4: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 83153BD8: 480545C8  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
	// 83153BDC: 817F003C  lwz r11, 0x3c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83153BE0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83153BE4: 409A0008  bne cr6, 0x83153bec
	if !ctx.cr[6].eq {
	pc = 0x83153BEC; continue 'dispatch;
	}
	// 83153BE8: 933F0038  stw r25, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[25].u32 ) };
	// 83153BEC: 38210130  addi r1, r1, 0x130
	ctx.r[1].s64 = ctx.r[1].s64 + 304;
	// 83153BF0: 480545B0  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83153BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83153BF8 size=76
    let mut pc: u32 = 0x83153BF8;
    'dispatch: loop {
        match pc {
            0x83153BF8 => {
    //   block [0x83153BF8..0x83153C44)
	// 83153BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83153BFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83153C00: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83153C04: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83153C08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83153C0C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83153C10: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 83153C14: 392B5350  addi r9, r11, 0x5350
	ctx.r[9].s64 = ctx.r[11].s64 + 21328;
	// 83153C18: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83153C1C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83153C20: 419A0010  beq cr6, 0x83153c30
	if ctx.cr[6].eq {
	pc = 0x83153C30; continue 'dispatch;
	}
	// 83153C24: 3880004C  li r4, 0x4c
	ctx.r[4].s64 = 76;
	// 83153C28: 4800C059  bl 0x8315fc80
	ctx.lr = 0x83153C2C;
	sub_8315FC80(ctx, base);
	// 83153C2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83153C30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83153C34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83153C38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83153C3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83153C40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83153C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83153C48 size=676
    let mut pc: u32 = 0x83153C48;
    'dispatch: loop {
        match pc {
            0x83153C48 => {
    //   block [0x83153C48..0x83153EEC)
	// 83153C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83153C4C: 48054515  bl 0x831a8160
	ctx.lr = 0x83153C50;
	sub_831A8130(ctx, base);
	// 83153C50: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 83153C54: 48054E1D  bl 0x831a8a70
	ctx.lr = 0x83153C58;
	sub_831A8A40(ctx, base);
	// 83153C58: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83153C5C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83153C60: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83153C64: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83153C68: C01D001C  lfs f0, 0x1c(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83153C6C: C3CB08A8  lfs f30, 0x8a8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2216 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 83153C70: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 83153C74: 409A0034  bne cr6, 0x83153ca8
	if !ctx.cr[6].eq {
	pc = 0x83153CA8; continue 'dispatch;
	}
	// 83153C78: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83153C7C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83153C80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83153C84: 419A024C  beq cr6, 0x83153ed0
	if ctx.cr[6].eq {
	pc = 0x83153ED0; continue 'dispatch;
	}
	// 83153C88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83153C8C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83153C90: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83153C94: 4E800421  bctrl
	ctx.lr = 0x83153C98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83153C98: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 83153C9C: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 83153CA0: 48054E1D  bl 0x831a8abc
	ctx.lr = 0x83153CA4;
	sub_831A8A8C(ctx, base);
	// 83153CA4: 4805450C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83153CA8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83153CAC: C3EBB184  lfs f31, -0x4e7c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 83153CB0: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 83153CB4: 4199002C  bgt cr6, 0x83153ce0
	if ctx.cr[6].gt {
	pc = 0x83153CE0; continue 'dispatch;
	}
	// 83153CB8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83153CBC: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83153CC0: 388B5504  addi r4, r11, 0x5504
	ctx.r[4].s64 = ctx.r[11].s64 + 21764;
	// 83153CC4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83153CC8: 4800BE61  bl 0x8315fb28
	ctx.lr = 0x83153CCC;
	sub_8315FB28(ctx, base);
	// 83153CCC: D3FD001C  stfs f31, 0x1c(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 83153CD0: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 83153CD4: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 83153CD8: 48054DE5  bl 0x831a8abc
	ctx.lr = 0x83153CDC;
	sub_831A8A8C(ctx, base);
	// 83153CDC: 480544D4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83153CE0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 83153CE4: C36B4E60  lfs f27, 0x4e60(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20064 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 83153CE8: FF00D800  fcmpu cr6, f0, f27
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[27].f64);
	// 83153CEC: 4099002C  ble cr6, 0x83153d18
	if !ctx.cr[6].gt {
	pc = 0x83153D18; continue 'dispatch;
	}
	// 83153CF0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83153CF4: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83153CF8: 388B54D8  addi r4, r11, 0x54d8
	ctx.r[4].s64 = ctx.r[11].s64 + 21720;
	// 83153CFC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83153D00: 4800BE29  bl 0x8315fb28
	ctx.lr = 0x83153D04;
	sub_8315FB28(ctx, base);
	// 83153D04: D37D001C  stfs f27, 0x1c(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[27].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 83153D08: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 83153D0C: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 83153D10: 48054DAD  bl 0x831a8abc
	ctx.lr = 0x83153D14;
	sub_831A8A8C(ctx, base);
	// 83153D14: 4805449C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83153D18: 815B0040  lwz r10, 0x40(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(64 as u32) ) } as u64;
	// 83153D1C: C1BB004C  lfs f13, 0x4c(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(76 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83153D20: 839B0048  lwz r28, 0x48(r27)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(72 as u32) ) } as u64;
	// 83153D24: EF4D0032  fmuls f26, f13, f0
	ctx.f[26].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 83153D28: FF800090  fmr f28, f0
	ctx.f[28].f64 = ctx.f[0].f64;
	// 83153D2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83153D30: 419A001C  beq cr6, 0x83153d4c
	if ctx.cr[6].eq {
	pc = 0x83153D4C; continue 'dispatch;
	}
	// 83153D34: 397B0050  addi r11, r27, 0x50
	ctx.r[11].s64 = ctx.r[27].s64 + 80;
	// 83153D38: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 83153D3C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83153D40: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83153D44: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83153D48: 4200FFF8  bdnz 0x83153d40
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83153D40; continue 'dispatch;
	}
	// 83153D4C: C01D0020  lfs f0, 0x20(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83153D50: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83153D54: 419A0024  beq cr6, 0x83153d78
	if ctx.cr[6].eq {
	pc = 0x83153D78; continue 'dispatch;
	}
	// 83153D58: 397CFFFF  addi r11, r28, -1
	ctx.r[11].s64 = ctx.r[28].s64 + -1;
	// 83153D5C: C1BD001C  lfs f13, 0x1c(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83153D60: 796A0020  clrldi r10, r11, 0x20
	ctx.r[10].u64 = ctx.r[11].u64 & 0x00000000FFFFFFFFu64;
	// 83153D64: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 83153D68: C9810050  lfd f12, 0x50(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83153D6C: FD60669C  fcfid f11, f12
	ctx.f[11].f64 = (ctx.f[12].s64 as f64);
	// 83153D70: FD405818  frsp f10, f11
	ctx.f[10].f64 = (ctx.f[11].f64 as f32) as f64;
	// 83153D74: EC0A037A  fmadds f0, f10, f13, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 83153D78: 897D0064  lbz r11, 0x64(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(100 as u32) ) } as u64;
	// 83153D7C: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83153D80: 409A0008  bne cr6, 0x83153d88
	if !ctx.cr[6].eq {
	pc = 0x83153D88; continue 'dispatch;
	}
	// 83153D84: EC00F02A  fadds f0, f0, f30
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[30].f64) as f32) as f64;
	// 83153D88: FFE00090  fmr f31, f0
	ctx.f[31].f64 = ctx.f[0].f64;
	// 83153D8C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83153D90: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83153D94: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83153D98: 419A0138  beq cr6, 0x83153ed0
	if ctx.cr[6].eq {
	pc = 0x83153ED0; continue 'dispatch;
	}
	// 83153D9C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83153DA0: C3AB08A4  lfs f29, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 83153DA4: 2B1A0080  cmplwi cr6, r26, 0x80
	ctx.cr[6].compare_u32(ctx.r[26].u32, 128 as u32, &mut ctx.xer);
	// 83153DA8: 41990128  bgt cr6, 0x83153ed0
	if ctx.cr[6].gt {
	pc = 0x83153ED0; continue 'dispatch;
	}
	// 83153DAC: 3B5A0001  addi r26, r26, 1
	ctx.r[26].s64 = ctx.r[26].s64 + 1;
	// 83153DB0: FF1FD800  fcmpu cr6, f31, f27
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[27].f64);
	// 83153DB4: 4099000C  ble cr6, 0x83153dc0
	if !ctx.cr[6].gt {
	pc = 0x83153DC0; continue 'dispatch;
	}
	// 83153DB8: FC00D890  fmr f0, f27
	ctx.f[0].f64 = ctx.f[27].f64;
	// 83153DBC: 48000008  b 0x83153dc4
	pc = 0x83153DC4; continue 'dispatch;
	// 83153DC0: FC00F890  fmr f0, f31
	ctx.f[0].f64 = ctx.f[31].f64;
	// 83153DC4: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 83153DC8: 40990118  ble cr6, 0x83153ee0
	if !ctx.cr[6].gt {
	pc = 0x83153EE0; continue 'dispatch;
	}
	// 83153DCC: 897D0064  lbz r11, 0x64(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(100 as u32) ) } as u64;
	// 83153DD0: FDA00090  fmr f13, f0
	ctx.f[13].f64 = ctx.f[0].f64;
	// 83153DD4: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83153DD8: 409A0010  bne cr6, 0x83153de8
	if !ctx.cr[6].eq {
	pc = 0x83153DE8; continue 'dispatch;
	}
	// 83153DDC: FF00E800  fcmpu cr6, f0, f29
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[29].f64);
	// 83153DE0: 419A0008  beq cr6, 0x83153de8
	if ctx.cr[6].eq {
	pc = 0x83153DE8; continue 'dispatch;
	}
	// 83153DE4: EDA0F028  fsubs f13, f0, f30
	ctx.f[13].f64 = (((ctx.f[0].f64 - ctx.f[30].f64) as f32) as f64);
	// 83153DE8: C01D0020  lfs f0, 0x20(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83153DEC: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 83153DF0: C1BD001C  lfs f13, 0x1c(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83153DF4: ED6C6824  fdivs f11, f12, f13
	ctx.f[11].f64 = ((ctx.f[12].f64 / ctx.f[13].f64) as f32) as f64;
	// 83153DF8: ED4BF02A  fadds f10, f11, f30
	ctx.f[10].f64 = ((ctx.f[11].f64 + ctx.f[30].f64) as f32) as f64;
	// 83153DFC: FD20565E  fctidz f9, f10
	ctx.f[9].s64 = if ctx.f[10].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[10].f64.trunc() as i64 };
	// 83153E00: D9210050  stfd f9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[9].u64 ) };
	// 83153E04: 80E10054  lwz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83153E08: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 83153E0C: F9410058  std r10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u64 ) };
	// 83153E10: C9010058  lfd f8, 0x58(r1)
	ctx.f[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 83153E14: FCE0469C  fcfid f7, f8
	ctx.f[7].f64 = (ctx.f[8].s64 as f64);
	// 83153E18: FCC03818  frsp f6, f7
	ctx.f[6].f64 = (ctx.f[7].f64 as f32) as f64;
	// 83153E1C: ECAA3028  fsubs f5, f10, f6
	ctx.f[5].f64 = (((ctx.f[10].f64 - ctx.f[6].f64) as f32) as f64);
	// 83153E20: EC9E2828  fsubs f4, f30, f5
	ctx.f[4].f64 = (((ctx.f[30].f64 - ctx.f[5].f64) as f32) as f64);
	// 83153E24: EC640732  fmuls f3, f4, f28
	ctx.f[3].f64 = (((ctx.f[4].f64 * ctx.f[28].f64) as f32) as f64);
	// 83153E28: FF03F000  fcmpu cr6, f3, f30
	ctx.cr[6].compare_f64(ctx.f[3].f64, ctx.f[30].f64);
	// 83153E2C: 40980008  bge cr6, 0x83153e34
	if !ctx.cr[6].lt {
	pc = 0x83153E34; continue 'dispatch;
	}
	// 83153E30: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 83153E34: 2B070080  cmplwi cr6, r7, 0x80
	ctx.cr[6].compare_u32(ctx.r[7].u32, 128 as u32, &mut ctx.xer);
	// 83153E38: 40990008  ble cr6, 0x83153e40
	if !ctx.cr[6].gt {
	pc = 0x83153E40; continue 'dispatch;
	}
	// 83153E3C: 38E00080  li r7, 0x80
	ctx.r[7].s64 = 128;
	// 83153E40: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 83153E44: 419A0020  beq cr6, 0x83153e64
	if ctx.cr[6].eq {
	pc = 0x83153E64; continue 'dispatch;
	}
	// 83153E48: 3947FFFF  addi r10, r7, -1
	ctx.r[10].s64 = ctx.r[7].s64 + -1;
	// 83153E4C: 79490020  clrldi r9, r10, 0x20
	ctx.r[9].u64 = ctx.r[10].u64 & 0x00000000FFFFFFFFu64;
	// 83153E50: F9210060  std r9, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u64 ) };
	// 83153E54: C9810060  lfd f12, 0x60(r1)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	// 83153E58: FD60669C  fcfid f11, f12
	ctx.f[11].f64 = (ctx.f[12].s64 as f64);
	// 83153E5C: FD405818  frsp f10, f11
	ctx.f[10].f64 = (ctx.f[11].f64 as f32) as f64;
	// 83153E60: EC0A037A  fmadds f0, f10, f13, f0
	ctx.f[0].f64 = (((ctx.f[10].f64 * ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64);
	// 83153E64: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83153E68: 409A0008  bne cr6, 0x83153e70
	if !ctx.cr[6].eq {
	pc = 0x83153E70; continue 'dispatch;
	}
	// 83153E6C: EC00F02A  fadds f0, f0, f30
	ctx.f[0].f64 = ((ctx.f[0].f64 + ctx.f[30].f64) as f32) as f64;
	// 83153E70: FC00065E  fctidz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[0].f64.trunc() as i64 };
	// 83153E74: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 83153E78: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83153E7C: 2B1F0080  cmplwi cr6, r31, 0x80
	ctx.cr[6].compare_u32(ctx.r[31].u32, 128 as u32, &mut ctx.xer);
	// 83153E80: 40990008  ble cr6, 0x83153e88
	if !ctx.cr[6].gt {
	pc = 0x83153E88; continue 'dispatch;
	}
	// 83153E84: 3BE00080  li r31, 0x80
	ctx.r[31].s64 = 128;
	// 83153E88: 7FC8F378  mr r8, r30
	ctx.r[8].u64 = ctx.r[30].u64;
	// 83153E8C: FC20D090  fmr f1, f26
	ctx.f[1].f64 = ctx.f[26].f64;
	// 83153E90: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83153E94: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83153E98: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83153E9C: 4BFFE41D  bl 0x831522b8
	ctx.lr = 0x83153EA0;
	sub_831522B8(ctx, base);
	// 83153EA0: 7BEB0020  clrldi r11, r31, 0x20
	ctx.r[11].u64 = ctx.r[31].u64 & 0x00000000FFFFFFFFu64;
	// 83153EA4: 7FC3F214  add r30, r3, r30
	ctx.r[30].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 83153EA8: F9610068  std r11, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u64 ) };
	// 83153EAC: C8010068  lfd f0, 0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 83153EB0: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 83153EB4: FD806818  frsp f12, f13
	ctx.f[12].f64 = (ctx.f[13].f64 as f32) as f64;
	// 83153EB8: EFFF6028  fsubs f31, f31, f12
	ctx.f[31].f64 = (((ctx.f[31].f64 - ctx.f[12].f64) as f32) as f64);
	// 83153EBC: FF1FE800  fcmpu cr6, f31, f29
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[29].f64);
	// 83153EC0: 40980008  bge cr6, 0x83153ec8
	if !ctx.cr[6].lt {
	pc = 0x83153EC8; continue 'dispatch;
	}
	// 83153EC4: FFE0E890  fmr f31, f29
	ctx.f[31].f64 = ctx.f[29].f64;
	// 83153EC8: 7F1EE040  cmplw cr6, r30, r28
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[28].u32, &mut ctx.xer);
	// 83153ECC: 4198FED8  blt cr6, 0x83153da4
	if ctx.cr[6].lt {
	pc = 0x83153DA4; continue 'dispatch;
	}
	// 83153ED0: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 83153ED4: 3981FFC8  addi r12, r1, -0x38
	ctx.r[12].s64 = ctx.r[1].s64 + -56;
	// 83153ED8: 48054BE5  bl 0x831a8abc
	ctx.lr = 0x83153EDC;
	sub_831A8A8C(ctx, base);
	// 83153EDC: 480542D4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83153EE0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83153EE4: 7CFEE050  subf r7, r30, r28
	ctx.r[7].s64 = ctx.r[28].s64 - ctx.r[30].s64;
	// 83153EE8: 4BFFFFA0  b 0x83153e88
	pc = 0x83153E88; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83153EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83153EF0 size=268
    let mut pc: u32 = 0x83153EF0;
    'dispatch: loop {
        match pc {
            0x83153EF0 => {
    //   block [0x83153EF0..0x83153FFC)
	// 83153EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83153EF4: 48054269  bl 0x831a815c
	ctx.lr = 0x83153EF8;
	sub_831A8130(ctx, base);
	// 83153EF8: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 83153EFC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83153F00: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 83153F04: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 83153F08: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83153F0C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 83153F10: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83153F14: 7CFF3B78  mr r31, r7
	ctx.r[31].u64 = ctx.r[7].u64;
	// 83153F18: 7D3C4B78  mr r28, r9
	ctx.r[28].u64 = ctx.r[9].u64;
	// 83153F1C: 7D595378  mr r25, r10
	ctx.r[25].u64 = ctx.r[10].u64;
	// 83153F20: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83153F24: 409A0014  bne cr6, 0x83153f38
	if !ctx.cr[6].eq {
	pc = 0x83153F38; continue 'dispatch;
	}
	// 83153F28: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83153F2C: 38A0FFFE  li r5, -2
	ctx.r[5].s64 = -2;
	// 83153F30: 388B5560  addi r4, r11, 0x5560
	ctx.r[4].s64 = ctx.r[11].s64 + 21856;
	// 83153F34: 480000B0  b 0x83153fe4
	pc = 0x83153FE4; continue 'dispatch;
	// 83153F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83153F3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83153F40: 419A0024  beq cr6, 0x83153f64
	if ctx.cr[6].eq {
	pc = 0x83153F64; continue 'dispatch;
	}
	// 83153F44: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 83153F48: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83153F4C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83153F50: 419A002C  beq cr6, 0x83153f7c
	if ctx.cr[6].eq {
	pc = 0x83153F7C; continue 'dispatch;
	}
	// 83153F54: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83153F58: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83153F5C: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 83153F60: 4198FFE8  blt cr6, 0x83153f48
	if ctx.cr[6].lt {
	pc = 0x83153F48; continue 'dispatch;
	}
	// 83153F64: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83153F68: 409A0024  bne cr6, 0x83153f8c
	if !ctx.cr[6].eq {
	pc = 0x83153F8C; continue 'dispatch;
	}
	// 83153F6C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83153F70: 38A0FFFE  li r5, -2
	ctx.r[5].s64 = -2;
	// 83153F74: 388B5554  addi r4, r11, 0x5554
	ctx.r[4].s64 = ctx.r[11].s64 + 21844;
	// 83153F78: 4800006C  b 0x83153fe4
	pc = 0x83153FE4; continue 'dispatch;
	// 83153F7C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83153F80: 38A0FFFE  li r5, -2
	ctx.r[5].s64 = -2;
	// 83153F84: 388B5548  addi r4, r11, 0x5548
	ctx.r[4].s64 = ctx.r[11].s64 + 21832;
	// 83153F88: 4800005C  b 0x83153fe4
	pc = 0x83153FE4; continue 'dispatch;
	// 83153F8C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83153F90: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83153F94: 38AB5540  addi r5, r11, 0x5540
	ctx.r[5].s64 = ctx.r[11].s64 + 21824;
	// 83153F98: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83153F9C: 38600054  li r3, 0x54
	ctx.r[3].s64 = 84;
	// 83153FA0: 4800BD59  bl 0x8315fcf8
	ctx.lr = 0x83153FA4;
	sub_8315FCF8(ctx, base);
	// 83153FA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83153FA8: 419A0030  beq cr6, 0x83153fd8
	if ctx.cr[6].eq {
	pc = 0x83153FD8; continue 'dispatch;
	}
	// 83153FAC: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 83153FB0: 93210054  stw r25, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[25].u32 ) };
	// 83153FB4: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83153FB8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 83153FBC: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 83153FC0: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 83153FC4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 83153FC8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83153FCC: 4BFFED5D  bl 0x83152d28
	ctx.lr = 0x83153FD0;
	sub_83152D28(ctx, base);
	// 83153FD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83153FD4: 409A001C  bne cr6, 0x83153ff0
	if !ctx.cr[6].eq {
	pc = 0x83153FF0; continue 'dispatch;
	}
	// 83153FD8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83153FDC: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83153FE0: 388B5534  addi r4, r11, 0x5534
	ctx.r[4].s64 = ctx.r[11].s64 + 21812;
	// 83153FE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83153FE8: 4800BB59  bl 0x8315fb40
	ctx.lr = 0x83153FEC;
	sub_8315FB40(ctx, base);
	// 83153FEC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83153FF0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83153FF4: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 83153FF8: 480541B4  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83154000 size=76
    let mut pc: u32 = 0x83154000;
    'dispatch: loop {
        match pc {
            0x83154000 => {
    //   block [0x83154000..0x8315404C)
	// 83154000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83154004: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83154008: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8315400C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83154010: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83154014: 4BFFF735  bl 0x83153748
	ctx.lr = 0x83154018;
	sub_83153748(ctx, base);
	// 83154018: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8315401C: 419A001C  beq cr6, 0x83154038
	if ctx.cr[6].eq {
	pc = 0x83154038; continue 'dispatch;
	}
	// 83154020: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83154024: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83154028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8315402C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83154030: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83154034: 4E800421  bctrl
	ctx.lr = 0x83154038;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83154038: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8315403C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83154040: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83154044: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83154048: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83154050 size=8
    let mut pc: u32 = 0x83154050;
    'dispatch: loop {
        match pc {
            0x83154050 => {
    //   block [0x83154050..0x83154058)
	// 83154050: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 83154054: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83154058 size=8
    let mut pc: u32 = 0x83154058;
    'dispatch: loop {
        match pc {
            0x83154058 => {
    //   block [0x83154058..0x83154060)
	// 83154058: 4BFFF138  b 0x83153190
	sub_83153190(ctx, base);
	return;
	// 8315405C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83154060 size=32
    let mut pc: u32 = 0x83154060;
    'dispatch: loop {
        match pc {
            0x83154060 => {
    //   block [0x83154060..0x83154080)
	// 83154060: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83154064: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83154068: 419A0018  beq cr6, 0x83154080
	if ctx.cr[6].eq {
		sub_83154080(ctx, base);
		return;
	}
	// 8315406C: 8163070C  lwz r11, 0x70c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1804 as u32) ) } as u64;
	// 83154070: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83154074: 419A000C  beq cr6, 0x83154080
	if ctx.cr[6].eq {
		sub_83154080(ctx, base);
		return;
	}
	// 83154078: 38630074  addi r3, r3, 0x74
	ctx.r[3].s64 = ctx.r[3].s64 + 116;
	// 8315407C: 48004974  b 0x831589f0
	sub_831589F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83154080 size=8
    let mut pc: u32 = 0x83154080;
    'dispatch: loop {
        match pc {
            0x83154080 => {
    //   block [0x83154080..0x83154088)
	// 83154080: 38630074  addi r3, r3, 0x74
	ctx.r[3].s64 = ctx.r[3].s64 + 116;
	// 83154084: 4800497C  b 0x83158a00
	sub_83158A00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83154088 size=48
    let mut pc: u32 = 0x83154088;
    'dispatch: loop {
        match pc {
            0x83154088 => {
    //   block [0x83154088..0x831540B8)
	// 83154088: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8315408C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 83154090: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 83154094: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 83154098: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8315409C: 806A0018  lwz r3, 0x18(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 831540A0: 419A0018  beq cr6, 0x831540b8
	if ctx.cr[6].eq {
		sub_831540B8(ctx, base);
		return;
	}
	// 831540A4: 814B070C  lwz r10, 0x70c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1804 as u32) ) } as u64;
	// 831540A8: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 831540AC: 419A000C  beq cr6, 0x831540b8
	if ctx.cr[6].eq {
		sub_831540B8(ctx, base);
		return;
	}
	// 831540B0: 808B0720  lwz r4, 0x720(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1824 as u32) ) } as u64;
	// 831540B4: 4BFF550C  b 0x831495c0
	sub_831495C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831540B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831540B8 size=8
    let mut pc: u32 = 0x831540B8;
    'dispatch: loop {
        match pc {
            0x831540B8 => {
    //   block [0x831540B8..0x831540C0)
	// 831540B8: 808B0720  lwz r4, 0x720(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1824 as u32) ) } as u64;
	// 831540BC: 4BFF552C  b 0x831495e8
	sub_831495E8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831540C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831540C0 size=48
    let mut pc: u32 = 0x831540C0;
    'dispatch: loop {
        match pc {
            0x831540C0 => {
    //   block [0x831540C0..0x831540F0)
	// 831540C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831540C4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831540C8: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 831540CC: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 831540D0: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 831540D4: 806A0018  lwz r3, 0x18(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 831540D8: 419A0018  beq cr6, 0x831540f0
	if ctx.cr[6].eq {
		sub_831540F0(ctx, base);
		return;
	}
	// 831540DC: 814B070C  lwz r10, 0x70c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1804 as u32) ) } as u64;
	// 831540E0: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 831540E4: 419A000C  beq cr6, 0x831540f0
	if ctx.cr[6].eq {
		sub_831540F0(ctx, base);
		return;
	}
	// 831540E8: 808B0720  lwz r4, 0x720(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1824 as u32) ) } as u64;
	// 831540EC: 4BFF551C  b 0x83149608
	sub_83149608(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831540F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831540F0 size=8
    let mut pc: u32 = 0x831540F0;
    'dispatch: loop {
        match pc {
            0x831540F0 => {
    //   block [0x831540F0..0x831540F8)
	// 831540F0: 808B0720  lwz r4, 0x720(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1824 as u32) ) } as u64;
	// 831540F4: 4BFF554C  b 0x83149640
	sub_83149640(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831540F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831540F8 size=84
    let mut pc: u32 = 0x831540F8;
    'dispatch: loop {
        match pc {
            0x831540F8 => {
    //   block [0x831540F8..0x8315414C)
	// 831540F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831540FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83154100: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83154104: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83154108: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315410C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83154110: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83154114: 4BFFF57D  bl 0x83153690
	ctx.lr = 0x83154118;
	sub_83153690(ctx, base);
	// 83154118: 57CB07FE  clrlwi r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8315411C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83154120: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83154124: 419A0010  beq cr6, 0x83154134
	if ctx.cr[6].eq {
	pc = 0x83154134; continue 'dispatch;
	}
	// 83154128: 38800728  li r4, 0x728
	ctx.r[4].s64 = 1832;
	// 8315412C: 4800BB55  bl 0x8315fc80
	ctx.lr = 0x83154130;
	sub_8315FC80(ctx, base);
	// 83154130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83154134: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83154138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8315413C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83154140: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83154144: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83154148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83154150 size=928
    let mut pc: u32 = 0x83154150;
    'dispatch: loop {
        match pc {
            0x83154150 => {
    //   block [0x83154150..0x831544F0)
	// 83154150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83154154: 48054005  bl 0x831a8158
	ctx.lr = 0x83154158;
	sub_831A8130(ctx, base);
	// 83154158: DBE1FFB0  stfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[31].u64 ) };
	// 8315415C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83154160: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83154164: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83154168: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8315416C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 83154170: 7D384B78  mr r24, r9
	ctx.r[24].u64 = ctx.r[9].u64;
	// 83154174: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83154178: 7D5C5378  mr r28, r10
	ctx.r[28].u64 = ctx.r[10].u64;
	// 8315417C: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 83154180: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83154184: 4099003C  ble cr6, 0x831541c0
	if !ctx.cr[6].gt {
	pc = 0x831541C0; continue 'dispatch;
	}
	// 83154188: 3BBF0048  addi r29, r31, 0x48
	ctx.r[29].s64 = ctx.r[31].s64 + 72;
	// 8315418C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83154190: 38A00200  li r5, 0x200
	ctx.r[5].s64 = 512;
	// 83154194: 38800400  li r4, 0x400
	ctx.r[4].s64 = 1024;
	// 83154198: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8315419C: 4800E44D  bl 0x831625e8
	ctx.lr = 0x831541A0;
	sub_831625E8(ctx, base);
	// 831541A0: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 831541A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831541A8: 419A0048  beq cr6, 0x831541f0
	if ctx.cr[6].eq {
	pc = 0x831541F0; continue 'dispatch;
	}
	// 831541AC: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831541B0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 831541B4: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 831541B8: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831541BC: 4198FFD0  blt cr6, 0x8315418c
	if ctx.cr[6].lt {
	pc = 0x8315418C; continue 'dispatch;
	}
	// 831541C0: 2B1C0001  cmplwi cr6, r28, 1
	ctx.cr[6].compare_u32(ctx.r[28].u32, 1 as u32, &mut ctx.xer);
	// 831541C4: 41980064  blt cr6, 0x83154228
	if ctx.cr[6].lt {
	pc = 0x83154228; continue 'dispatch;
	}
	// 831541C8: 419A0050  beq cr6, 0x83154218
	if ctx.cr[6].eq {
	pc = 0x83154218; continue 'dispatch;
	}
	// 831541CC: 2B1C0003  cmplwi cr6, r28, 3
	ctx.cr[6].compare_u32(ctx.r[28].u32, 3 as u32, &mut ctx.xer);
	// 831541D0: 41980038  blt cr6, 0x83154208
	if ctx.cr[6].lt {
	pc = 0x83154208; continue 'dispatch;
	}
	// 831541D4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831541D8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 831541DC: 388B559C  addi r4, r11, 0x559c
	ctx.r[4].s64 = ctx.r[11].s64 + 21916;
	// 831541E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831541E4: 4800B945  bl 0x8315fb28
	ctx.lr = 0x831541E8;
	sub_8315FB28(ctx, base);
	// 831541E8: 935F0068  stw r26, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[26].u32 ) };
	// 831541EC: 48000090  b 0x8315427c
	pc = 0x8315427C; continue 'dispatch;
	// 831541F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831541F4: 4BFFF555  bl 0x83153748
	ctx.lr = 0x831541F8;
	sub_83153748(ctx, base);
	// 831541F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831541FC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83154200: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 83154204: 48053FA4  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83154208: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8315420C: 4BFFF6BD  bl 0x831538c8
	ctx.lr = 0x83154210;
	sub_831538C8(ctx, base);
	// 83154210: 907F0068  stw r3, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 83154214: 48000068  b 0x8315427c
	pc = 0x8315427C; continue 'dispatch;
	// 83154218: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8315421C: 480059DD  bl 0x83159bf8
	ctx.lr = 0x83154220;
	sub_83159BF8(ctx, base);
	// 83154220: 907F0068  stw r3, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[3].u32 ) };
	// 83154224: 48000058  b 0x8315427c
	pc = 0x8315427C; continue 'dispatch;
	// 83154228: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8315422C: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83154230: 38AB54C0  addi r5, r11, 0x54c0
	ctx.r[5].s64 = ctx.r[11].s64 + 21696;
	// 83154234: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83154238: 38600130  li r3, 0x130
	ctx.r[3].s64 = 304;
	// 8315423C: 4800BABD  bl 0x8315fcf8
	ctx.lr = 0x83154240;
	sub_8315FCF8(ctx, base);
	// 83154240: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83154244: 419A0010  beq cr6, 0x83154254
	if ctx.cr[6].eq {
	pc = 0x83154254; continue 'dispatch;
	}
	// 83154248: 4BFFD819  bl 0x83151a60
	ctx.lr = 0x8315424C;
	sub_83151A60(ctx, base);
	// 8315424C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83154250: 409A001C  bne cr6, 0x8315426c
	if !ctx.cr[6].eq {
	pc = 0x8315426C; continue 'dispatch;
	}
	// 83154254: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83154258: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8315425C: 388B5494  addi r4, r11, 0x5494
	ctx.r[4].s64 = ctx.r[11].s64 + 21652;
	// 83154260: 4800B8B9  bl 0x8315fb18
	ctx.lr = 0x83154264;
	sub_8315FB18(ctx, base);
	// 83154264: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 83154268: 48000010  b 0x83154278
	pc = 0x83154278; continue 'dispatch;
	// 8315426C: 39430048  addi r10, r3, 0x48
	ctx.r[10].s64 = ctx.r[3].s64 + 72;
	// 83154270: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83154274: 914300B4  stw r10, 0xb4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 83154278: 917F0068  stw r11, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 8315427C: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83154280: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83154284: 419AFF6C  beq cr6, 0x831541f0
	if ctx.cr[6].eq {
	pc = 0x831541F0; continue 'dispatch;
	}
	// 83154288: 813F0044  lwz r9, 0x44(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 8315428C: 3CE08315  lis r7, -0x7ceb
	ctx.r[7].s64 = -2095775744;
	// 83154290: 3CC08219  lis r6, -0x7de7
	ctx.r[6].s64 = -2112290816;
	// 83154294: 391F0048  addi r8, r31, 0x48
	ctx.r[8].s64 = ctx.r[31].s64 + 72;
	// 83154298: 38865590  addi r4, r6, 0x5590
	ctx.r[4].s64 = ctx.r[6].s64 + 21904;
	// 8315429C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 831542A0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 831542A4: 39274050  addi r9, r7, 0x4050
	ctx.r[9].s64 = ctx.r[7].s64 + 16464;
	// 831542A8: 807F0068  lwz r3, 0x68(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 831542AC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831542B0: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 831542B4: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831542B8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831542BC: 80FF0068  lwz r7, 0x68(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 831542C0: 80DF004C  lwz r6, 0x4c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 831542C4: 90C7000C  stw r6, 0xc(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 831542C8: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 831542CC: 80FF0050  lwz r7, 0x50(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 831542D0: 90EB0010  stw r7, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[7].u32 ) };
	// 831542D4: 80DF0068  lwz r6, 0x68(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 831542D8: 817F0054  lwz r11, 0x54(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 831542DC: 91660014  stw r11, 0x14(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 831542E0: 80FF0068  lwz r7, 0x68(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 831542E4: 80DF0058  lwz r6, 0x58(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) } as u64;
	// 831542E8: 90C70018  stw r6, 0x18(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(24 as u32), ctx.r[6].u32 ) };
	// 831542EC: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 831542F0: 80FF005C  lwz r7, 0x5c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(92 as u32) ) } as u64;
	// 831542F4: 90EB001C  stw r7, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[7].u32 ) };
	// 831542F8: 80DF0068  lwz r6, 0x68(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 831542FC: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 83154300: 91660020  stw r11, 0x20(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83154304: 80FF0068  lwz r7, 0x68(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 83154308: 80DF0064  lwz r6, 0x64(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 8315430C: 90C70024  stw r6, 0x24(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(36 as u32), ctx.r[6].u32 ) };
	// 83154310: C03F0018  lfs f1, 0x18(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83154314: 80FF0014  lwz r7, 0x14(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83154318: 4BFFFBD9  bl 0x83153ef0
	ctx.lr = 0x8315431C;
	sub_83153EF0(ctx, base);
	// 8315431C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83154320: 907F006C  stw r3, 0x6c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 83154324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83154328: 419AFECC  beq cr6, 0x831541f4
	if ctx.cr[6].eq {
	pc = 0x831541F4; continue 'dispatch;
	}
	// 8315432C: C0DF0030  lfs f6, 0x30(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 83154330: C0BF002C  lfs f5, 0x2c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 83154334: C09F0028  lfs f4, 0x28(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 83154338: C07F0024  lfs f3, 0x24(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 8315433C: C05F0020  lfs f2, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 83154340: C03F001C  lfs f1, 0x1c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83154344: 4BFFF02D  bl 0x83153370
	ctx.lr = 0x83154348;
	sub_83153370(ctx, base);
	// 83154348: C01F071C  lfs f0, 0x71c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1820 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8315434C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 83154350: C1BF0718  lfs f13, 0x718(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1816 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 83154354: 3B9F0654  addi r28, r31, 0x654
	ctx.r[28].s64 = ctx.r[31].s64 + 1620;
	// 83154358: EC000372  fmuls f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 * ctx.f[13].f64) as f32) as f64);
	// 8315435C: D01F0670  stfs f0, 0x670(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1648 as u32), tmp.u32 ) };
	// 83154360: C3EBB184  lfs f31, -0x4e7c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-20092 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 83154364: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 83154368: 41990010  bgt cr6, 0x83154378
	if ctx.cr[6].gt {
	pc = 0x83154378; continue 'dispatch;
	}
	// 8315436C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83154370: 388B53B0  addi r4, r11, 0x53b0
	ctx.r[4].s64 = ctx.r[11].s64 + 21424;
	// 83154374: 4800001C  b 0x83154390
	pc = 0x83154390; continue 'dispatch;
	// 83154378: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8315437C: C3EB4E60  lfs f31, 0x4e60(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20064 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 83154380: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 83154384: 4099001C  ble cr6, 0x831543a0
	if !ctx.cr[6].gt {
	pc = 0x831543A0; continue 'dispatch;
	}
	// 83154388: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8315438C: 388B5384  addi r4, r11, 0x5384
	ctx.r[4].s64 = ctx.r[11].s64 + 21380;
	// 83154390: 38A00080  li r5, 0x80
	ctx.r[5].s64 = 128;
	// 83154394: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83154398: 4800B791  bl 0x8315fb28
	ctx.lr = 0x8315439C;
	sub_8315FB28(ctx, base);
	// 8315439C: D3FC001C  stfs f31, 0x1c(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 831543A0: 83D90020  lwz r30, 0x20(r25)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(32 as u32) ) } as u64;
	// 831543A4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831543A8: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 831543AC: 83B90014  lwz r29, 0x14(r25)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(20 as u32) ) } as u64;
	// 831543B0: 38AB5578  addi r5, r11, 0x5578
	ctx.r[5].s64 = ctx.r[11].s64 + 21880;
	// 831543B4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831543B8: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 831543BC: 4800B93D  bl 0x8315fcf8
	ctx.lr = 0x831543C0;
	sub_8315FCF8(ctx, base);
	// 831543C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831543C4: 419A003C  beq cr6, 0x83154400
	if ctx.cr[6].eq {
	pc = 0x83154400; continue 'dispatch;
	}
	// 831543C8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831543CC: 93C30018  stw r30, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 831543D0: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 831543D4: 93A3001C  stw r29, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[29].u32 ) };
	// 831543D8: 392B5580  addi r9, r11, 0x5580
	ctx.r[9].s64 = ctx.r[11].s64 + 21888;
	// 831543DC: 93030020  stw r24, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[24].u32 ) };
	// 831543E0: 390A5400  addi r8, r10, 0x5400
	ctx.r[8].s64 = ctx.r[10].s64 + 21504;
	// 831543E4: 93430008  stw r26, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[26].u32 ) };
	// 831543E8: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 831543EC: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 831543F0: 9343000C  stw r26, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[26].u32 ) };
	// 831543F4: 93430010  stw r26, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[26].u32 ) };
	// 831543F8: 93430014  stw r26, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 831543FC: 4800001C  b 0x83154418
	pc = 0x83154418; continue 'dispatch;
	// 83154400: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83154404: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83154408: 388B556C  addi r4, r11, 0x556c
	ctx.r[4].s64 = ctx.r[11].s64 + 21868;
	// 8315440C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83154410: 4800B731  bl 0x8315fb40
	ctx.lr = 0x83154414;
	sub_8315FB40(ctx, base);
	// 83154414: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83154418: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8315441C: 907F0714  stw r3, 0x714(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1812 as u32), ctx.r[3].u32 ) };
	// 83154420: 419AFDD0  beq cr6, 0x831541f0
	if ctx.cr[6].eq {
	pc = 0x831541F0; continue 'dispatch;
	}
	// 83154424: 3BBF0074  addi r29, r31, 0x74
	ctx.r[29].s64 = ctx.r[31].s64 + 116;
	// 83154428: 807F006C  lwz r3, 0x6c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(108 as u32) ) } as u64;
	// 8315442C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83154430: 4BFF1F11  bl 0x83146340
	ctx.lr = 0x83154434;
	sub_83146340(ctx, base);
	// 83154434: 907F0070  stw r3, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 83154438: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8315443C: 419AFDB4  beq cr6, 0x831541f0
	if ctx.cr[6].eq {
	pc = 0x831541F0; continue 'dispatch;
	}
	// 83154440: 3BDF0188  addi r30, r31, 0x188
	ctx.r[30].s64 = ctx.r[31].s64 + 392;
	// 83154444: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83154448: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8315444C: 4BFF1EF5  bl 0x83146340
	ctx.lr = 0x83154450;
	sub_83146340(ctx, base);
	// 83154450: 907F0184  stw r3, 0x184(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(388 as u32), ctx.r[3].u32 ) };
	// 83154454: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83154458: 419AFD98  beq cr6, 0x831541f0
	if ctx.cr[6].eq {
	pc = 0x831541F0; continue 'dispatch;
	}
	// 8315445C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83154460: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83154464: 4BFF1EDD  bl 0x83146340
	ctx.lr = 0x83154468;
	sub_83146340(ctx, base);
	// 83154468: 907F0650  stw r3, 0x650(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1616 as u32), ctx.r[3].u32 ) };
	// 8315446C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83154470: 419AFD80  beq cr6, 0x831541f0
	if ctx.cr[6].eq {
	pc = 0x831541F0; continue 'dispatch;
	}
	// 83154474: 3BDF06C4  addi r30, r31, 0x6c4
	ctx.r[30].s64 = ctx.r[31].s64 + 1732;
	// 83154478: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8315447C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83154480: 4BFF1EC1  bl 0x83146340
	ctx.lr = 0x83154484;
	sub_83146340(ctx, base);
	// 83154484: 907F06C0  stw r3, 0x6c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1728 as u32), ctx.r[3].u32 ) };
	// 83154488: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8315448C: 419AFD64  beq cr6, 0x831541f0
	if ctx.cr[6].eq {
	pc = 0x831541F0; continue 'dispatch;
	}
	// 83154490: 3BBF06F0  addi r29, r31, 0x6f0
	ctx.r[29].s64 = ctx.r[31].s64 + 1776;
	// 83154494: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83154498: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8315449C: 4BFF1EA5  bl 0x83146340
	ctx.lr = 0x831544A0;
	sub_83146340(ctx, base);
	// 831544A0: 907F06EC  stw r3, 0x6ec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1772 as u32), ctx.r[3].u32 ) };
	// 831544A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831544A8: 419AFD48  beq cr6, 0x831541f0
	if ctx.cr[6].eq {
	pc = 0x831541F0; continue 'dispatch;
	}
	// 831544AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831544B0: 809F0714  lwz r4, 0x714(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1812 as u32) ) } as u64;
	// 831544B4: 4BFF1E8D  bl 0x83146340
	ctx.lr = 0x831544B8;
	sub_83146340(ctx, base);
	// 831544B8: 907F0710  stw r3, 0x710(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1808 as u32), ctx.r[3].u32 ) };
	// 831544BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831544C0: 419AFD30  beq cr6, 0x831541f0
	if ctx.cr[6].eq {
	pc = 0x831541F0; continue 'dispatch;
	}
	// 831544C4: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 831544C8: 809F0714  lwz r4, 0x714(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1812 as u32) ) } as u64;
	// 831544CC: 806B0018  lwz r3, 0x18(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831544D0: 4BFF5061  bl 0x83149530
	ctx.lr = 0x831544D4;
	sub_83149530(ctx, base);
	// 831544D4: 907F0720  stw r3, 0x720(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1824 as u32), ctx.r[3].u32 ) };
	// 831544D8: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 831544DC: 419AFD14  beq cr6, 0x831541f0
	if ctx.cr[6].eq {
	pc = 0x831541F0; continue 'dispatch;
	}
	// 831544E0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831544E4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831544E8: CBE1FFB0  lfd f31, -0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 831544EC: 48053CBC  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831544F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831544F0 size=248
    let mut pc: u32 = 0x831544F0;
    'dispatch: loop {
        match pc {
            0x831544F0 => {
    //   block [0x831544F0..0x831545E8)
	// 831544F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831544F4: 48053C69  bl 0x831a815c
	ctx.lr = 0x831544F8;
	sub_831A8130(ctx, base);
	// 831544F8: DBE1FFB8  stfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 831544FC: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83154500: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83154504: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 83154508: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8315450C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83154510: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 83154514: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 83154518: 38AB55E4  addi r5, r11, 0x55e4
	ctx.r[5].s64 = ctx.r[11].s64 + 21988;
	// 8315451C: 38600728  li r3, 0x728
	ctx.r[3].s64 = 1832;
	// 83154520: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83154524: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 83154528: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 8315452C: 7D5A5378  mr r26, r10
	ctx.r[26].u64 = ctx.r[10].u64;
	// 83154530: 4800B7C9  bl 0x8315fcf8
	ctx.lr = 0x83154534;
	sub_8315FCF8(ctx, base);
	// 83154534: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83154538: 419A0030  beq cr6, 0x83154568
	if ctx.cr[6].eq {
	pc = 0x83154568; continue 'dispatch;
	}
	// 8315453C: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 83154540: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 83154544: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 83154548: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 8315454C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 83154550: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83154554: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83154558: 4BFFEF91  bl 0x831534e8
	ctx.lr = 0x8315455C;
	sub_831534E8(ctx, base);
	// 8315455C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83154560: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83154564: 409A0028  bne cr6, 0x8315458c
	if !ctx.cr[6].eq {
	pc = 0x8315458C; continue 'dispatch;
	}
	// 83154568: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8315456C: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83154570: 388B55D8  addi r4, r11, 0x55d8
	ctx.r[4].s64 = ctx.r[11].s64 + 21976;
	// 83154574: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83154578: 4800B5C9  bl 0x8315fb40
	ctx.lr = 0x8315457C;
	sub_8315FB40(ctx, base);
	// 8315457C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83154580: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83154584: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 83154588: 48053C24  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 8315458C: 7F29CB78  mr r9, r25
	ctx.r[9].u64 = ctx.r[25].u64;
	// 83154590: 814100F4  lwz r10, 0xf4(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(244 as u32) ) } as u64;
	// 83154594: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 83154598: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 8315459C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 831545A0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 831545A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831545A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831545AC: 4BFFFBA5  bl 0x83154150
	ctx.lr = 0x831545B0;
	sub_83154150(ctx, base);
	// 831545B0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831545B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831545B8: 409A0024  bne cr6, 0x831545dc
	if !ctx.cr[6].eq {
	pc = 0x831545DC; continue 'dispatch;
	}
	// 831545BC: 4BFFF18D  bl 0x83153748
	ctx.lr = 0x831545C0;
	sub_83153748(ctx, base);
	// 831545C0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831545C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831545C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831545CC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831545D0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831545D4: 4E800421  bctrl
	ctx.lr = 0x831545D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831545D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831545DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831545E0: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 831545E4: 48053BC8  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831545E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831545E8 size=12
    let mut pc: u32 = 0x831545E8;
    'dispatch: loop {
        match pc {
            0x831545E8 => {
    //   block [0x831545E8..0x831545F4)
	// 831545E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831545EC: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831545F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831545F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831545F8 size=8
    let mut pc: u32 = 0x831545F8;
    'dispatch: loop {
        match pc {
            0x831545F8 => {
    //   block [0x831545F8..0x83154600)
	// 831545F8: 80630010  lwz r3, 0x10(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 831545FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83154600 size=8
    let mut pc: u32 = 0x83154600;
    'dispatch: loop {
        match pc {
            0x83154600 => {
    //   block [0x83154600..0x83154608)
	// 83154600: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83154604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83154608 size=76
    let mut pc: u32 = 0x83154608;
    'dispatch: loop {
        match pc {
            0x83154608 => {
    //   block [0x83154608..0x83154654)
	// 83154608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315460C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83154610: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83154614: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83154618: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8315461C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83154620: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 83154624: 392B55F8  addi r9, r11, 0x55f8
	ctx.r[9].s64 = ctx.r[11].s64 + 22008;
	// 83154628: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8315462C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83154630: 419A0010  beq cr6, 0x83154640
	if ctx.cr[6].eq {
	pc = 0x83154640; continue 'dispatch;
	}
	// 83154634: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 83154638: 4800B649  bl 0x8315fc80
	ctx.lr = 0x8315463C;
	sub_8315FC80(ctx, base);
	// 8315463C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83154640: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83154644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83154648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8315464C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83154650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83154658 size=40
    let mut pc: u32 = 0x83154658;
    'dispatch: loop {
        match pc {
            0x83154658 => {
    //   block [0x83154658..0x83154680)
	// 83154658: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8315465C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83154660: 409A0020  bne cr6, 0x83154680
	if !ctx.cr[6].eq {
		sub_83154680(ctx, base);
		return;
	}
	// 83154664: 81630038  lwz r11, 0x38(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 83154668: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8315466C: 409A0014  bne cr6, 0x83154680
	if !ctx.cr[6].eq {
		sub_83154680(ctx, base);
		return;
	}
	// 83154670: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83154674: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83154678: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8315467C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83154680 size=8
    let mut pc: u32 = 0x83154680;
    'dispatch: loop {
        match pc {
            0x83154680 => {
    //   block [0x83154680..0x83154688)
	// 83154680: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83154684: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83154688 size=140
    let mut pc: u32 = 0x83154688;
    'dispatch: loop {
        match pc {
            0x83154688 => {
    //   block [0x83154688..0x83154714)
	// 83154688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315468C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83154690: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83154694: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83154698: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 8315469C: 80A3000C  lwz r5, 0xc(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 831546A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 831546A4: 80830008  lwz r4, 8(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831546A8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 831546AC: 38CB4924  addi r6, r11, 0x4924
	ctx.r[6].s64 = ctx.r[11].s64 + 18724;
	// 831546B0: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 831546B4: 4800B565  bl 0x8315fc18
	ctx.lr = 0x831546B8;
	sub_8315FC18(ctx, base);
	// 831546B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831546BC: 419A002C  beq cr6, 0x831546e8
	if ctx.cr[6].eq {
	pc = 0x831546E8; continue 'dispatch;
	}
	// 831546C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831546C4: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831546C8: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831546CC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831546D0: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 831546D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831546D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831546DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831546E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831546E4: 4E800020  blr
	return;
	// 831546E8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831546EC: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 831546F0: 388B4918  addi r4, r11, 0x4918
	ctx.r[4].s64 = ctx.r[11].s64 + 18712;
	// 831546F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831546F8: 4800B449  bl 0x8315fb40
	ctx.lr = 0x831546FC;
	sub_8315FB40(ctx, base);
	// 831546FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83154700: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83154704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83154708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8315470C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83154710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83154718 size=128
    let mut pc: u32 = 0x83154718;
    'dispatch: loop {
        match pc {
            0x83154718 => {
    //   block [0x83154718..0x83154798)
	// 83154718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315471C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83154720: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83154724: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83154728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315472C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83154730: 3BFE0014  addi r31, r30, 0x14
	ctx.r[31].s64 = ctx.r[30].s64 + 20;
	// 83154734: 80BE0014  lwz r5, 0x14(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 83154738: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8315473C: 419A001C  beq cr6, 0x83154758
	if ctx.cr[6].eq {
	pc = 0x83154758; continue 'dispatch;
	}
	// 83154740: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83154744: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83154748: 4BFF3B29  bl 0x83148270
	ctx.lr = 0x8315474C;
	sub_83148270(ctx, base);
	// 8315474C: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83154750: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83154754: 409AFFEC  bne cr6, 0x83154740
	if !ctx.cr[6].eq {
	pc = 0x83154740; continue 'dispatch;
	}
	// 83154758: 80BE0028  lwz r5, 0x28(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 8315475C: 3BFE0028  addi r31, r30, 0x28
	ctx.r[31].s64 = ctx.r[30].s64 + 40;
	// 83154760: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83154764: 419A001C  beq cr6, 0x83154780
	if ctx.cr[6].eq {
	pc = 0x83154780; continue 'dispatch;
	}
	// 83154768: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8315476C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83154770: 4BFF3B01  bl 0x83148270
	ctx.lr = 0x83154774;
	sub_83148270(ctx, base);
	// 83154774: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83154778: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 8315477C: 409AFFEC  bne cr6, 0x83154768
	if !ctx.cr[6].eq {
	pc = 0x83154768; continue 'dispatch;
	}
	// 83154780: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83154784: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83154788: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8315478C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83154790: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83154794: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83154798 size=172
    let mut pc: u32 = 0x83154798;
    'dispatch: loop {
        match pc {
            0x83154798 => {
    //   block [0x83154798..0x83154844)
	// 83154798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315479C: 480539D1  bl 0x831a816c
	ctx.lr = 0x831547A0;
	sub_831A8130(ctx, base);
	// 831547A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831547A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831547A8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831547AC: 3BDF0014  addi r30, r31, 0x14
	ctx.r[30].s64 = ctx.r[31].s64 + 20;
	// 831547B0: 80BF0014  lwz r5, 0x14(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831547B4: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 831547B8: 419A003C  beq cr6, 0x831547f4
	if ctx.cr[6].eq {
	pc = 0x831547F4; continue 'dispatch;
	}
	// 831547BC: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 831547C0: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 831547C4: 409A0024  bne cr6, 0x831547e8
	if !ctx.cr[6].eq {
	pc = 0x831547E8; continue 'dispatch;
	}
	// 831547C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831547CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 831547D0: 4BFF3AA1  bl 0x83148270
	ctx.lr = 0x831547D4;
	sub_83148270(ctx, base);
	// 831547D4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831547D8: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831547DC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831547E0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831547E4: 48000008  b 0x831547ec
	pc = 0x831547EC; continue 'dispatch;
	// 831547E8: 80A50004  lwz r5, 4(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 831547EC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 831547F0: 409AFFCC  bne cr6, 0x831547bc
	if !ctx.cr[6].eq {
	pc = 0x831547BC; continue 'dispatch;
	}
	// 831547F4: 80BF0028  lwz r5, 0x28(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 831547F8: 3BDF0028  addi r30, r31, 0x28
	ctx.r[30].s64 = ctx.r[31].s64 + 40;
	// 831547FC: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83154800: 419A003C  beq cr6, 0x8315483c
	if ctx.cr[6].eq {
	pc = 0x8315483C; continue 'dispatch;
	}
	// 83154804: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 83154808: 7F0BE840  cmplw cr6, r11, r29
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[29].u32, &mut ctx.xer);
	// 8315480C: 409A0024  bne cr6, 0x83154830
	if !ctx.cr[6].eq {
	pc = 0x83154830; continue 'dispatch;
	}
	// 83154810: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83154814: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83154818: 4BFF3A59  bl 0x83148270
	ctx.lr = 0x8315481C;
	sub_83148270(ctx, base);
	// 8315481C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83154820: 80A30000  lwz r5, 0(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83154824: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 83154828: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8315482C: 48000008  b 0x83154834
	pc = 0x83154834; continue 'dispatch;
	// 83154830: 80A50004  lwz r5, 4(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 83154834: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83154838: 409AFFCC  bne cr6, 0x83154804
	if !ctx.cr[6].eq {
	pc = 0x83154804; continue 'dispatch;
	}
	// 8315483C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83154840: 4805397C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83154848 size=12
    let mut pc: u32 = 0x83154848;
    'dispatch: loop {
        match pc {
            0x83154848 => {
    //   block [0x83154848..0x83154854)
	// 83154848: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8315484C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83154850: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154854(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83154854 size=12
    let mut pc: u32 = 0x83154854;
    'dispatch: loop {
        match pc {
            0x83154854 => {
    //   block [0x83154854..0x83154860)
	// 83154854: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 83154858: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8315485C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83154860 size=28
    let mut pc: u32 = 0x83154860;
    'dispatch: loop {
        match pc {
            0x83154860 => {
    //   block [0x83154860..0x8315487C)
	// 83154860: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 83154864: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83154868: 419A0020  beq cr6, 0x83154888
	if ctx.cr[6].eq {
		sub_8315487C(ctx, base);
		return;
	}
	// 8315486C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83154870: 812A0044  lwz r9, 0x44(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(68 as u32) ) } as u64;
	// 83154874: 2F090003  cmpwi cr6, r9, 3
	ctx.cr[6].compare_i32(ctx.r[9].s32, 3, &mut ctx.xer);
	// 83154878: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8315487C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8315487C size=40
    let mut pc: u32 = 0x8315487C;
    'dispatch: loop {
        match pc {
            0x8315487C => {
    //   block [0x8315487C..0x831548A4)
	// 8315487C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83154880: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83154884: 409AFFE8  bne cr6, 0x8315486c
	if !ctx.cr[6].eq {
		sub_83154860(ctx, base);
		return;
	}
	// 83154888: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8315488C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83154890: 419A0020  beq cr6, 0x831548b0
	if ctx.cr[6].eq {
		sub_831548A4(ctx, base);
		return;
	}
	// 83154894: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83154898: 812A0044  lwz r9, 0x44(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(68 as u32) ) } as u64;
	// 8315489C: 2F090001  cmpwi cr6, r9, 1
	ctx.cr[6].compare_i32(ctx.r[9].s32, 1, &mut ctx.xer);
	// 831548A0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831548A4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831548A4 size=24
    let mut pc: u32 = 0x831548A4;
    'dispatch: loop {
        match pc {
            0x831548A4 => {
    //   block [0x831548A4..0x831548BC)
	// 831548A4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831548A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831548AC: 409AFFE8  bne cr6, 0x83154894
	if !ctx.cr[6].eq {
		sub_8315487C(ctx, base);
		return;
	}
	// 831548B0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831548B4: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 831548B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831548C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831548C0 size=84
    let mut pc: u32 = 0x831548C0;
    'dispatch: loop {
        match pc {
            0x831548C0 => {
    //   block [0x831548C0..0x83154914)
	// 831548C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831548C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831548C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831548CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831548D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831548D4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831548D8: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831548DC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 831548E0: 41810020  bgt 0x83154900
	if ctx.cr[0].gt {
	pc = 0x83154900; continue 'dispatch;
	}
	// 831548E4: 4BFFFE35  bl 0x83154718
	ctx.lr = 0x831548E8;
	sub_83154718(ctx, base);
	// 831548E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831548EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831548F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831548F4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831548F8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831548FC: 4E800421  bctrl
	ctx.lr = 0x83154900;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83154900: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83154904: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83154908: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8315490C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83154910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83154918 size=172
    let mut pc: u32 = 0x83154918;
    'dispatch: loop {
        match pc {
            0x83154918 => {
    //   block [0x83154918..0x831549C4)
	// 83154918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315491C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83154920: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83154924: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83154928: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315492C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83154930: 83DF0004  lwz r30, 4(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83154934: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83154938: 419A0028  beq cr6, 0x83154960
	if ctx.cr[6].eq {
	pc = 0x83154960; continue 'dispatch;
	}
	// 8315493C: 4BFFFD4D  bl 0x83154688
	ctx.lr = 0x83154940;
	sub_83154688(ctx, base);
	// 83154940: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83154944: 419A004C  beq cr6, 0x83154990
	if ctx.cr[6].eq {
	pc = 0x83154990; continue 'dispatch;
	}
	// 83154948: 93C30000  stw r30, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8315494C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83154950: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83154954: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83154958: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 8315495C: 48000040  b 0x8315499c
	pc = 0x8315499C; continue 'dispatch;
	// 83154960: 4BFFFD29  bl 0x83154688
	ctx.lr = 0x83154964;
	sub_83154688(ctx, base);
	// 83154964: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 83154968: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8315496C: 419A0024  beq cr6, 0x83154990
	if ctx.cr[6].eq {
	pc = 0x83154990; continue 'dispatch;
	}
	// 83154970: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83154974: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83154978: 419A0020  beq cr6, 0x83154998
	if ctx.cr[6].eq {
	pc = 0x83154998; continue 'dispatch;
	}
	// 8315497C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83154980: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 83154984: 388B49A0  addi r4, r11, 0x49a0
	ctx.r[4].s64 = ctx.r[11].s64 + 18848;
	// 83154988: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8315498C: 4800B1B5  bl 0x8315fb40
	ctx.lr = 0x83154990;
	sub_8315FB40(ctx, base);
	// 83154990: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83154994: 48000018  b 0x831549ac
	pc = 0x831549AC; continue 'dispatch;
	// 83154998: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8315499C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 831549A0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831549A4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831549A8: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 831549AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831549B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831549B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831549B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831549BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831549C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831549C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831549C8 size=196
    let mut pc: u32 = 0x831549C8;
    'dispatch: loop {
        match pc {
            0x831549C8 => {
    //   block [0x831549C8..0x83154A8C)
	// 831549C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831549CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831549D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831549D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831549D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831549DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831549E0: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 831549E4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831549E8: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 831549EC: 38CB5608  addi r6, r11, 0x5608
	ctx.r[6].s64 = ctx.r[11].s64 + 22024;
	// 831549F0: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 831549F4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831549F8: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 831549FC: 4800B21D  bl 0x8315fc18
	ctx.lr = 0x83154A00;
	sub_8315FC18(ctx, base);
	// 83154A00: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83154A04: 419A0058  beq cr6, 0x83154a5c
	if ctx.cr[6].eq {
	pc = 0x83154A5C; continue 'dispatch;
	}
	// 83154A08: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83154A0C: 93C30008  stw r30, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 83154A10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83154A14: 390A55F8  addi r8, r10, 0x55f8
	ctx.r[8].s64 = ctx.r[10].s64 + 22008;
	// 83154A18: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83154A1C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83154A20: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83154A24: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83154A28: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83154A2C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83154A30: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83154A34: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83154A38: 93E3001C  stw r31, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[31].u32 ) };
	// 83154A3C: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 83154A40: 91630024  stw r11, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83154A44: 91630028  stw r11, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83154A48: 9163002C  stw r11, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83154A4C: 93E30030  stw r31, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[31].u32 ) };
	// 83154A50: 91430034  stw r10, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 83154A54: 91630038  stw r11, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83154A58: 4800001C  b 0x83154a74
	pc = 0x83154A74; continue 'dispatch;
	// 83154A5C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83154A60: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83154A64: 388B55FC  addi r4, r11, 0x55fc
	ctx.r[4].s64 = ctx.r[11].s64 + 22012;
	// 83154A68: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83154A6C: 4800B0D5  bl 0x8315fb40
	ctx.lr = 0x83154A70;
	sub_8315FB40(ctx, base);
	// 83154A70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83154A74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83154A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83154A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83154A80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83154A84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83154A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83154A90 size=100
    let mut pc: u32 = 0x83154A90;
    'dispatch: loop {
        match pc {
            0x83154A90 => {
    //   block [0x83154A90..0x83154AF4)
	// 83154A90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83154A94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83154A98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83154A9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83154AA0: 9081007C  stw r4, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[4].u32 ) };
	// 83154AA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83154AA8: 3881007C  addi r4, r1, 0x7c
	ctx.r[4].s64 = ctx.r[1].s64 + 124;
	// 83154AAC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 83154AB0: 4BFFFE69  bl 0x83154918
	ctx.lr = 0x83154AB4;
	sub_83154918(ctx, base);
	// 83154AB4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83154AB8: 409A0018  bne cr6, 0x83154ad0
	if !ctx.cr[6].eq {
	pc = 0x83154AD0; continue 'dispatch;
	}
	// 83154ABC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83154AC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83154AC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83154AC8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83154ACC: 4E800020  blr
	return;
	// 83154AD0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83154AD4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83154AD8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83154ADC: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83154AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83154AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83154AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83154AEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83154AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83154AF8 size=236
    let mut pc: u32 = 0x83154AF8;
    'dispatch: loop {
        match pc {
            0x83154AF8 => {
    //   block [0x83154AF8..0x83154BE4)
	// 83154AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83154AFC: 48053671  bl 0x831a816c
	ctx.lr = 0x83154B00;
	sub_831A8130(ctx, base);
	// 83154B00: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83154B04: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83154B08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83154B0C: 83DD0028  lwz r30, 0x28(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 83154B10: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83154B14: 419A004C  beq cr6, 0x83154b60
	if ctx.cr[6].eq {
	pc = 0x83154B60; continue 'dispatch;
	}
	// 83154B18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83154B1C: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83154B20: 4BFF4329  bl 0x83148e48
	ctx.lr = 0x83154B24;
	sub_83148E48(ctx, base);
	// 83154B24: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83154B28: 419A0044  beq cr6, 0x83154b6c
	if ctx.cr[6].eq {
	pc = 0x83154B6C; continue 'dispatch;
	}
	// 83154B2C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83154B30: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83154B34: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83154B38: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83154B3C: 4BFFFDDD  bl 0x83154918
	ctx.lr = 0x83154B40;
	sub_83154918(ctx, base);
	// 83154B40: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83154B44: 419A0064  beq cr6, 0x83154ba8
	if ctx.cr[6].eq {
	pc = 0x83154BA8; continue 'dispatch;
	}
	// 83154B48: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83154B4C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83154B50: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83154B54: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83154B58: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83154B5C: 409AFFBC  bne cr6, 0x83154b18
	if !ctx.cr[6].eq {
	pc = 0x83154B18; continue 'dispatch;
	}
	// 83154B60: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83154B64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83154B68: 48053654  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83154B6C: 83DD0028  lwz r30, 0x28(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 83154B70: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83154B74: 419A0028  beq cr6, 0x83154b9c
	if ctx.cr[6].eq {
	pc = 0x83154B9C; continue 'dispatch;
	}
	// 83154B78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83154B7C: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83154B80: 4BFFFC19  bl 0x83154798
	ctx.lr = 0x83154B84;
	sub_83154798(ctx, base);
	// 83154B84: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83154B88: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83154B8C: 4BFF3E75  bl 0x83148a00
	ctx.lr = 0x83154B90;
	sub_83148A00(ctx, base);
	// 83154B90: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83154B94: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83154B98: 409AFFE0  bne cr6, 0x83154b78
	if !ctx.cr[6].eq {
	pc = 0x83154B78; continue 'dispatch;
	}
	// 83154B9C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83154BA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83154BA4: 48053618  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83154BA8: 83DD0028  lwz r30, 0x28(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(40 as u32) ) } as u64;
	// 83154BAC: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83154BB0: 419A0028  beq cr6, 0x83154bd8
	if ctx.cr[6].eq {
	pc = 0x83154BD8; continue 'dispatch;
	}
	// 83154BB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83154BB8: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83154BBC: 4BFFFBDD  bl 0x83154798
	ctx.lr = 0x83154BC0;
	sub_83154798(ctx, base);
	// 83154BC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83154BC4: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83154BC8: 4BFF3E39  bl 0x83148a00
	ctx.lr = 0x83154BCC;
	sub_83148A00(ctx, base);
	// 83154BCC: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83154BD0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83154BD4: 409AFFE0  bne cr6, 0x83154bb4
	if !ctx.cr[6].eq {
	pc = 0x83154BB4; continue 'dispatch;
	}
	// 83154BD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83154BDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83154BE0: 480535DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83154BE8 size=212
    let mut pc: u32 = 0x83154BE8;
    'dispatch: loop {
        match pc {
            0x83154BE8 => {
    //   block [0x83154BE8..0x83154CBC)
	// 83154BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83154BEC: 4805357D  bl 0x831a8168
	ctx.lr = 0x83154BF0;
	sub_831A8130(ctx, base);
	// 83154BF0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83154BF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83154BF8: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83154BFC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83154C00: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83154C04: 38E00004  li r7, 4
	ctx.r[7].s64 = 4;
	// 83154C08: 38CB5608  addi r6, r11, 0x5608
	ctx.r[6].s64 = ctx.r[11].s64 + 22024;
	// 83154C0C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83154C10: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83154C14: 3860003C  li r3, 0x3c
	ctx.r[3].s64 = 60;
	// 83154C18: 4800B001  bl 0x8315fc18
	ctx.lr = 0x83154C1C;
	sub_8315FC18(ctx, base);
	// 83154C1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83154C20: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83154C24: 419A0078  beq cr6, 0x83154c9c
	if ctx.cr[6].eq {
	pc = 0x83154C9C; continue 'dispatch;
	}
	// 83154C28: 3D408219  lis r10, -0x7de7
	ctx.r[10].s64 = -2112290816;
	// 83154C2C: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83154C30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83154C34: 390A55F8  addi r8, r10, 0x55f8
	ctx.r[8].s64 = ctx.r[10].s64 + 22008;
	// 83154C38: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83154C3C: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83154C40: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83154C44: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 83154C48: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 83154C4C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83154C50: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83154C54: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83154C58: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83154C5C: 93DF001C  stw r30, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[30].u32 ) };
	// 83154C60: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 83154C64: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83154C68: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83154C6C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 83154C70: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 83154C74: 915F0034  stw r10, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 83154C78: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 83154C7C: 4BFFFE7D  bl 0x83154af8
	ctx.lr = 0x83154C80;
	sub_83154AF8(ctx, base);
	// 83154C80: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83154C84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83154C88: 409A000C  bne cr6, 0x83154c94
	if !ctx.cr[6].eq {
	pc = 0x83154C94; continue 'dispatch;
	}
	// 83154C8C: 4BFFFC35  bl 0x831548c0
	ctx.lr = 0x83154C90;
	sub_831548C0(ctx, base);
	// 83154C90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83154C94: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83154C98: 48053520  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83154C9C: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83154CA0: 38A0FFFD  li r5, -3
	ctx.r[5].s64 = -3;
	// 83154CA4: 388B5614  addi r4, r11, 0x5614
	ctx.r[4].s64 = ctx.r[11].s64 + 22036;
	// 83154CA8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83154CAC: 4800AE95  bl 0x8315fb40
	ctx.lr = 0x83154CB0;
	sub_8315FB40(ctx, base);
	// 83154CB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83154CB4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83154CB8: 48053500  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154CC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83154CC0 size=12
    let mut pc: u32 = 0x83154CC0;
    'dispatch: loop {
        match pc {
            0x83154CC0 => {
    //   block [0x83154CC0..0x83154CCC)
	// 83154CC0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83154CC4: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83154CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83154CD0 size=76
    let mut pc: u32 = 0x83154CD0;
    'dispatch: loop {
        match pc {
            0x83154CD0 => {
    //   block [0x83154CD0..0x83154D1C)
	// 83154CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83154CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83154CD8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83154CDC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83154CE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83154CE4: 3D608219  lis r11, -0x7de7
	ctx.r[11].s64 = -2112290816;
	// 83154CE8: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 83154CEC: 392B562C  addi r9, r11, 0x562c
	ctx.r[9].s64 = ctx.r[11].s64 + 22060;
	// 83154CF0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83154CF4: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83154CF8: 419A0010  beq cr6, 0x83154d08
	if ctx.cr[6].eq {
	pc = 0x83154D08; continue 'dispatch;
	}
	// 83154CFC: 3880003C  li r4, 0x3c
	ctx.r[4].s64 = 60;
	// 83154D00: 4800AF81  bl 0x8315fc80
	ctx.lr = 0x83154D04;
	sub_8315FC80(ctx, base);
	// 83154D04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83154D08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83154D0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83154D10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83154D14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83154D18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83154D20 size=8
    let mut pc: u32 = 0x83154D20;
    'dispatch: loop {
        match pc {
            0x83154D20 => {
    //   block [0x83154D20..0x83154D28)
	// 83154D20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83154D24: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83154D28 size=20
    let mut pc: u32 = 0x83154D28;
    'dispatch: loop {
        match pc {
            0x83154D28 => {
    //   block [0x83154D28..0x83154D3C)
	// 83154D28: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83154D2C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83154D30: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83154D34: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83154D38: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154D3C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83154D3C size=4
    let mut pc: u32 = 0x83154D3C;
    'dispatch: loop {
        match pc {
            0x83154D3C => {
    //   block [0x83154D3C..0x83154D40)
	// 83154D3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83154D40 size=60
    let mut pc: u32 = 0x83154D40;
    'dispatch: loop {
        match pc {
            0x83154D40 => {
    //   block [0x83154D40..0x83154D7C)
	// 83154D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83154D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83154D48: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83154D4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83154D50: 419A0018  beq cr6, 0x83154d68
	if ctx.cr[6].eq {
	pc = 0x83154D68; continue 'dispatch;
	}
	// 83154D54: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83154D58: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83154D5C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83154D60: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83154D64: 4E800421  bctrl
	ctx.lr = 0x83154D68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83154D68: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83154D6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83154D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83154D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83154D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83154D80 size=280
    let mut pc: u32 = 0x83154D80;
    'dispatch: loop {
        match pc {
            0x83154D80 => {
    //   block [0x83154D80..0x83154E98)
	// 83154D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83154D84: 480533DD  bl 0x831a8160
	ctx.lr = 0x83154D88;
	sub_831A8130(ctx, base);
	// 83154D88: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83154D8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83154D90: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83154D94: 80DF0044  lwz r6, 0x44(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83154D98: 807F003C  lwz r3, 0x3c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) } as u64;
	// 83154D9C: 80BF0040  lwz r5, 0x40(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83154DA0: 4800C2A1  bl 0x83161040
	ctx.lr = 0x83154DA4;
	sub_83161040(ctx, base);
	// 83154DA4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83154DA8: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83154DAC: 409A000C  bne cr6, 0x83154db8
	if !ctx.cr[6].eq {
	pc = 0x83154DB8; continue 'dispatch;
	}
	// 83154DB0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83154DB4: 480533FC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
	// 83154DB8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83154DBC: 4BEFF27D  bl 0x83054038
	ctx.lr = 0x83154DC0;
	sub_83054038(ctx, base);
	// 83154DC0: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 83154DC4: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83154DC8: 4099000C  ble cr6, 0x83154dd4
	if !ctx.cr[6].gt {
	pc = 0x83154DD4; continue 'dispatch;
	}
	// 83154DCC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83154DD0: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83154DD4: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83154DD8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83154DDC: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83154DE0: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 83154DE4: 9B5F0024  stb r26, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[26].u8 ) };
	// 83154DE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83154DEC: B35F0026  sth r26, 0x26(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(38 as u32), ctx.r[26].u16 ) };
	// 83154DF0: 40990070  ble cr6, 0x83154e60
	if !ctx.cr[6].gt {
	pc = 0x83154E60; continue 'dispatch;
	}
	// 83154DF4: 3BBF0010  addi r29, r31, 0x10
	ctx.r[29].s64 = ctx.r[31].s64 + 16;
	// 83154DF8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83154DFC: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83154E00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83154E04: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83154E08: 4800BBD1  bl 0x831609d8
	ctx.lr = 0x83154E0C;
	sub_831609D8(ctx, base);
	// 83154E0C: 817F0040  lwz r11, 0x40(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83154E10: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83154E14: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83154E18: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83154E1C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83154E20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83154E24: 7D0B4850  subf r8, r11, r9
	ctx.r[8].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 83154E28: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83154E2C: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83154E30: 911DFFFC  stw r8, -4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-4 as u32), ctx.r[8].u32 ) };
	// 83154E34: 4800BBA5  bl 0x831609d8
	ctx.lr = 0x83154E38;
	sub_831609D8(ctx, base);
	// 83154E38: 88C10058  lbz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83154E3C: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 83154E40: 409A000C  bne cr6, 0x83154e4c
	if !ctx.cr[6].eq {
	pc = 0x83154E4C; continue 'dispatch;
	}
	// 83154E44: 9B7F0024  stb r27, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[27].u8 ) };
	// 83154E48: B3DF0026  sth r30, 0x26(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(38 as u32), ctx.r[30].u16 ) };
	// 83154E4C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83154E50: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 83154E54: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 83154E58: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83154E5C: 4198FF9C  blt cr6, 0x83154df8
	if ctx.cr[6].lt {
	pc = 0x83154DF8; continue 'dispatch;
	}
	// 83154E60: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 83154E64: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83154E68: 409A001C  bne cr6, 0x83154e84
	if !ctx.cr[6].eq {
	pc = 0x83154E84; continue 'dispatch;
	}
	// 83154E6C: 897F0024  lbz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83154E70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83154E74: 409A0010  bne cr6, 0x83154e84
	if !ctx.cr[6].eq {
	pc = 0x83154E84; continue 'dispatch;
	}
	// 83154E78: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 83154E7C: 9B7F0024  stb r27, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[27].u8 ) };
	// 83154E80: B35F0026  sth r26, 0x26(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(38 as u32), ctx.r[26].u16 ) };
	// 83154E84: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83154E88: 4800BAE1  bl 0x83160968
	ctx.lr = 0x83154E8C;
	sub_83160968(ctx, base);
	// 83154E8C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83154E90: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83154E94: 4805331C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83154E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83154E98 size=484
    let mut pc: u32 = 0x83154E98;
    'dispatch: loop {
        match pc {
            0x83154E98 => {
    //   block [0x83154E98..0x8315507C)
	// 83154E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83154E9C: 480532CD  bl 0x831a8168
	ctx.lr = 0x83154EA0;
	sub_831A8130(ctx, base);
	// 83154EA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83154EA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83154EA8: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83154EAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83154EB0: 419A01C4  beq cr6, 0x83155074
	if ctx.cr[6].eq {
	pc = 0x83155074; continue 'dispatch;
	}
	// 83154EB4: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83154EB8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83154EBC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83154EC0: 409A0008  bne cr6, 0x83154ec8
	if !ctx.cr[6].eq {
	pc = 0x83154EC8; continue 'dispatch;
	}
	// 83154EC4: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 83154EC8: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83154ECC: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83154ED0: 409A0198  bne cr6, 0x83155068
	if !ctx.cr[6].eq {
	pc = 0x83155068; continue 'dispatch;
	}
	// 83154ED4: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83154ED8: 3B800002  li r28, 2
	ctx.r[28].s64 = 2;
	// 83154EDC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 83154EE0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83154EE4: 409A008C  bne cr6, 0x83154f70
	if !ctx.cr[6].eq {
	pc = 0x83154F70; continue 'dispatch;
	}
	// 83154EE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83154EEC: 4BFFFE95  bl 0x83154d80
	ctx.lr = 0x83154EF0;
	sub_83154D80(ctx, base);
	// 83154EF0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83154EF4: 419A0078  beq cr6, 0x83154f6c
	if ctx.cr[6].eq {
	pc = 0x83154F6C; continue 'dispatch;
	}
	// 83154EF8: 38BF0048  addi r5, r31, 0x48
	ctx.r[5].s64 = ctx.r[31].s64 + 72;
	// 83154EFC: 807F0040  lwz r3, 0x40(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83154F00: 809F0044  lwz r4, 0x44(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83154F04: 4800E3F5  bl 0x831632f8
	ctx.lr = 0x83154F08;
	sub_831632F8(ctx, base);
	// 83154F08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83154F0C: 419A0064  beq cr6, 0x83154f70
	if ctx.cr[6].eq {
	pc = 0x83154F70; continue 'dispatch;
	}
	// 83154F10: A17F0084  lhz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 83154F14: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83154F18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83154F1C: 409A0018  bne cr6, 0x83154f34
	if !ctx.cr[6].eq {
	pc = 0x83154F34; continue 'dispatch;
	}
	// 83154F20: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83154F24: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 83154F28: 9BDF0024  stb r30, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u8 ) };
	// 83154F2C: B3DF0026  sth r30, 0x26(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(38 as u32), ctx.r[30].u16 ) };
	// 83154F30: 48000038  b 0x83154f68
	pc = 0x83154F68; continue 'dispatch;
	// 83154F34: 813F0094  lwz r9, 0x94(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83154F38: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 83154F3C: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 83154F40: 811F0044  lwz r8, 0x44(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 83154F44: 7CEB4850  subf r7, r11, r9
	ctx.r[7].s64 = ctx.r[9].s64 - ctx.r[11].s64;
	// 83154F48: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83154F4C: 7CC94050  subf r6, r9, r8
	ctx.r[6].s64 = ctx.r[8].s64 - ctx.r[9].s64;
	// 83154F50: 9BBF0024  stb r29, 0x24(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u8 ) };
	// 83154F54: 90FF0018  stw r7, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[7].u32 ) };
	// 83154F58: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83154F5C: 913F001C  stw r9, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 83154F60: 90DF0020  stw r6, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[6].u32 ) };
	// 83154F64: B3BF0026  sth r29, 0x26(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(38 as u32), ctx.r[29].u16 ) };
	// 83154F68: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83154F6C: 939F0034  stw r28, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[28].u32 ) };
	// 83154F70: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 83154F74: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83154F78: 409A00C8  bne cr6, 0x83155040
	if !ctx.cr[6].eq {
	pc = 0x83155040; continue 'dispatch;
	}
	// 83154F7C: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 83154F80: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83154F84: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83154F88: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83154F8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83154F90: 4800CBE9  bl 0x83161b78
	ctx.lr = 0x83154F94;
	sub_83161B78(ctx, base);
	// 83154F94: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83154F98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83154F9C: 409AFFE4  bne cr6, 0x83154f80
	if !ctx.cr[6].eq {
	pc = 0x83154F80; continue 'dispatch;
	}
	// 83154FA0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83154FA4: 4800C485  bl 0x83161428
	ctx.lr = 0x83154FA8;
	sub_83161428(ctx, base);
	// 83154FA8: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 83154FAC: 40990080  ble cr6, 0x8315502c
	if !ctx.cr[6].gt {
	pc = 0x8315502C; continue 'dispatch;
	}
	// 83154FB0: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 83154FB4: 897F0032  lbz r11, 0x32(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(50 as u32) ) } as u64;
	// 83154FB8: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83154FBC: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83154FC0: 40980064  bge cr6, 0x83155024
	if !ctx.cr[6].lt {
	pc = 0x83155024; continue 'dispatch;
	}
	// 83154FC4: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83154FC8: 813F0040  lwz r9, 0x40(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) } as u64;
	// 83154FCC: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 83154FD0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83154FD4: 7D4AFA14  add r10, r10, r31
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[31].u64;
	// 83154FD8: 55681838  slwi r8, r11, 3
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83154FDC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83154FE0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83154FE4: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 83154FE8: 7CEB4A14  add r7, r11, r9
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 83154FEC: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 83154FF0: 7CC8F82E  lwzx r6, r8, r31
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83154FF4: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 83154FF8: 4800CBA1  bl 0x83161b98
	ctx.lr = 0x83154FFC;
	sub_83161B98(ctx, base);
	// 83154FFC: 88BF0024  lbz r5, 0x24(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83155000: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83155004: 419A0014  beq cr6, 0x83155018
	if ctx.cr[6].eq {
	pc = 0x83155018; continue 'dispatch;
	}
	// 83155008: A17F0026  lhz r11, 0x26(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(38 as u32) ) } as u64;
	// 8315500C: 895F0032  lbz r10, 0x32(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(50 as u32) ) } as u64;
	// 83155010: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83155014: 419A0010  beq cr6, 0x83155024
	if ctx.cr[6].eq {
	pc = 0x83155024; continue 'dispatch;
	}
	// 83155018: 897F0032  lbz r11, 0x32(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(50 as u32) ) } as u64;
	// 8315501C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83155020: 997F0032  stb r11, 0x32(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(50 as u32), ctx.r[11].u8 ) };
	// 83155024: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83155028: 4082FF8C  bne 0x83154fb4
	if !ctx.cr[0].eq {
	pc = 0x83154FB4; continue 'dispatch;
	}
	// 8315502C: 897F0032  lbz r11, 0x32(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(50 as u32) ) } as u64;
	// 83155030: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83155034: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83155038: 409A0008  bne cr6, 0x83155040
	if !ctx.cr[6].eq {
	pc = 0x83155040; continue 'dispatch;
	}
	// 8315503C: 9BBF0031  stb r29, 0x31(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(49 as u32), ctx.r[29].u8 ) };
	// 83155040: 897F0031  lbz r11, 0x31(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(49 as u32) ) } as u64;
	// 83155044: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83155048: 409A002C  bne cr6, 0x83155074
	if !ctx.cr[6].eq {
	pc = 0x83155074; continue 'dispatch;
	}
	// 8315504C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83155050: 4800CC51  bl 0x83161ca0
	ctx.lr = 0x83155054;
	sub_83161CA0(ctx, base);
	// 83155054: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83155058: 409A001C  bne cr6, 0x83155074
	if !ctx.cr[6].eq {
	pc = 0x83155074; continue 'dispatch;
	}
	// 8315505C: 939F0028  stw r28, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[28].u32 ) };
	// 83155060: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83155064: 48053154  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83155068: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8315506C: 409A0008  bne cr6, 0x83155074
	if !ctx.cr[6].eq {
	pc = 0x83155074; continue 'dispatch;
	}
	// 83155070: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 83155074: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83155078: 48053140  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83155080 size=88
    let mut pc: u32 = 0x83155080;
    'dispatch: loop {
        match pc {
            0x83155080 => {
    //   block [0x83155080..0x831550D8)
	// 83155080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83155084: 480530E9  bl 0x831a816c
	ctx.lr = 0x83155088;
	sub_831A8130(ctx, base);
	// 83155088: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315508C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83155090: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83155094: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83155098: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 8315509C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831550A0: 93DF00C0  stw r30, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 831550A4: 387F00C4  addi r3, r31, 0xc4
	ctx.r[3].s64 = ctx.r[31].s64 + 196;
	// 831550A8: 48057449  bl 0x831ac4f0
	ctx.lr = 0x831550AC;
	sub_831AC4F0(ctx, base);
	// 831550AC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 831550B0: 38E00800  li r7, 0x800
	ctx.r[7].s64 = 2048;
	// 831550B4: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 831550B8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831550BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831550C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831550C4: 4801321D  bl 0x831682e0
	ctx.lr = 0x831550C8;
	sub_831682E0(ctx, base);
	// 831550C8: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 831550CC: 48013225  bl 0x831682f0
	ctx.lr = 0x831550D0;
	sub_831682F0(ctx, base);
	// 831550D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831550D4: 480530E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831550D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831550D8 size=80
    let mut pc: u32 = 0x831550D8;
    'dispatch: loop {
        match pc {
            0x831550D8 => {
    //   block [0x831550D8..0x83155128)
	// 831550D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831550DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831550E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831550E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831550E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831550EC: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 831550F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831550F4: 419A0008  beq cr6, 0x831550fc
	if ctx.cr[6].eq {
	pc = 0x831550FC; continue 'dispatch;
	}
	// 831550F8: 48014751  bl 0x83169848
	ctx.lr = 0x831550FC;
	sub_83169848(ctx, base);
	// 831550FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83155100: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83155104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83155108: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8315510C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83155110: 4E800421  bctrl
	ctx.lr = 0x83155114;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83155114: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83155118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8315511C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83155120: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83155124: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83155128 size=112
    let mut pc: u32 = 0x83155128;
    'dispatch: loop {
        match pc {
            0x83155128 => {
    //   block [0x83155128..0x83155198)
	// 83155128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315512C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83155130: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83155134: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83155138: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315513C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83155140: 807E00B4  lwz r3, 0xb4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(180 as u32) ) } as u64;
	// 83155144: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83155148: 409A000C  bne cr6, 0x83155154
	if !ctx.cr[6].eq {
	pc = 0x83155154; continue 'dispatch;
	}
	// 8315514C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83155150: 48000030  b 0x83155180
	pc = 0x83155180; continue 'dispatch;
	// 83155154: 4801483D  bl 0x83169990
	ctx.lr = 0x83155158;
	sub_83169990(ctx, base);
	// 83155158: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8315515C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83155160: 419A001C  beq cr6, 0x8315517c
	if ctx.cr[6].eq {
	pc = 0x8315517C; continue 'dispatch;
	}
	// 83155164: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83155168: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8315516C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83155170: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83155174: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83155178: 4E800421  bctrl
	ctx.lr = 0x8315517C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8315517C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83155180: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83155184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83155188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8315518C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83155190: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83155194: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83155198 size=8
    let mut pc: u32 = 0x83155198;
    'dispatch: loop {
        match pc {
            0x83155198 => {
    //   block [0x83155198..0x831551A0)
	// 83155198: 806300B4  lwz r3, 0xb4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) } as u64;
	// 8315519C: 48013B3C  b 0x83168cd8
	sub_83168CD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831551A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831551A0 size=284
    let mut pc: u32 = 0x831551A0;
    'dispatch: loop {
        match pc {
            0x831551A0 => {
    //   block [0x831551A0..0x831552BC)
	// 831551A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831551A4: 48052FC1  bl 0x831a8164
	ctx.lr = 0x831551A8;
	sub_831A8130(ctx, base);
	// 831551A8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831551AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831551B0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831551B4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831551B8: 4800CAF1  bl 0x83161ca8
	ctx.lr = 0x831551BC;
	sub_83161CA8(ctx, base);
	// 831551BC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831551C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831551C4: 409A0010  bne cr6, 0x831551d4
	if !ctx.cr[6].eq {
	pc = 0x831551D4; continue 'dispatch;
	}
	// 831551C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831551CC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831551D0: 48052FE4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831551D4: 3CC07FFF  lis r6, 0x7fff
	ctx.r[6].s64 = 2147418112;
	// 831551D8: 807E003C  lwz r3, 0x3c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 831551DC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 831551E0: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831551E4: 60C6FFFF  ori r6, r6, 0xffff
	ctx.r[6].u64 = ctx.r[6].u64 | 65535;
	// 831551E8: 4800BE59  bl 0x83161040
	ctx.lr = 0x831551EC;
	sub_83161040(ctx, base);
	// 831551EC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831551F0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 831551F4: 419AFFD4  beq cr6, 0x831551c8
	if ctx.cr[6].eq {
	pc = 0x831551C8; continue 'dispatch;
	}
	// 831551F8: 4BEFEE41  bl 0x83054038
	ctx.lr = 0x831551FC;
	sub_83054038(ctx, base);
	// 831551FC: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83155200: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83155204: 907E0008  stw r3, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 83155208: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8315520C: 419A0070  beq cr6, 0x8315527c
	if ctx.cr[6].eq {
	pc = 0x8315527C; continue 'dispatch;
	}
	// 83155210: 3BBE0010  addi r29, r30, 0x10
	ctx.r[29].s64 = ctx.r[30].s64 + 16;
	// 83155214: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 83155218: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 8315521C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83155220: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83155224: 4800B7B5  bl 0x831609d8
	ctx.lr = 0x83155228;
	sub_831609D8(ctx, base);
	// 83155228: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8315522C: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83155230: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 83155234: 8121006C  lwz r9, 0x6c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83155238: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8315523C: 7D0B5050  subf r8, r11, r10
	ctx.r[8].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83155240: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83155244: 911DFFFC  stw r8, -4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-4 as u32), ctx.r[8].u32 ) };
	// 83155248: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8315524C: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83155250: 4800B789  bl 0x831609d8
	ctx.lr = 0x83155254;
	sub_831609D8(ctx, base);
	// 83155254: 88C10068  lbz r6, 0x68(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83155258: 2B060001  cmplwi cr6, r6, 1
	ctx.cr[6].compare_u32(ctx.r[6].u32, 1 as u32, &mut ctx.xer);
	// 8315525C: 409A000C  bne cr6, 0x83155268
	if !ctx.cr[6].eq {
	pc = 0x83155268; continue 'dispatch;
	}
	// 83155260: 9B7E0024  stb r27, 0x24(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[27].u8 ) };
	// 83155264: B3FE0026  sth r31, 0x26(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(38 as u32), ctx.r[31].u16 ) };
	// 83155268: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8315526C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83155270: 3BBD0008  addi r29, r29, 8
	ctx.r[29].s64 = ctx.r[29].s64 + 8;
	// 83155274: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83155278: 4198FF9C  blt cr6, 0x83155214
	if ctx.cr[6].lt {
	pc = 0x83155214; continue 'dispatch;
	}
	// 8315527C: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 83155280: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83155284: 409A0014  bne cr6, 0x83155298
	if !ctx.cr[6].eq {
	pc = 0x83155298; continue 'dispatch;
	}
	// 83155288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8315528C: 937E0008  stw r27, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 83155290: 9B7E0024  stb r27, 0x24(r30)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[27].u8 ) };
	// 83155294: B17E0026  sth r11, 0x26(r30)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[30].u32.wrapping_add(38 as u32), ctx.r[11].u16 ) };
	// 83155298: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8315529C: 4800B6CD  bl 0x83160968
	ctx.lr = 0x831552A0;
	sub_83160968(ctx, base);
	// 831552A0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831552A4: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831552A8: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831552AC: 4800CA0D  bl 0x83161cb8
	ctx.lr = 0x831552B0;
	sub_83161CB8(ctx, base);
	// 831552B0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831552B4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831552B8: 48052EFC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831552C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831552C0 size=104
    let mut pc: u32 = 0x831552C0;
    'dispatch: loop {
        match pc {
            0x831552C0 => {
    //   block [0x831552C0..0x83155328)
	// 831552C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831552C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831552C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831552CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831552D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831552D4: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 831552D8: 48013F91  bl 0x83169268
	ctx.lr = 0x831552DC;
	sub_83169268(ctx, base);
	// 831552DC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831552E0: 409A0018  bne cr6, 0x831552f8
	if !ctx.cr[6].eq {
	pc = 0x831552F8; continue 'dispatch;
	}
	// 831552E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831552E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831552EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831552F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831552F4: 4E800020  blr
	return;
	// 831552F8: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 831552FC: 4800C86D  bl 0x83161b68
	ctx.lr = 0x83155300;
	sub_83161B68(ctx, base);
	// 83155300: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83155304: 48012FED  bl 0x831682f0
	ctx.lr = 0x83155308;
	sub_831682F0(ctx, base);
	// 83155308: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8315530C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83155310: 917F01C4  stw r11, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[11].u32 ) };
	// 83155314: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83155318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8315531C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83155320: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83155324: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83155328 size=8
    let mut pc: u32 = 0x83155328;
    'dispatch: loop {
        match pc {
            0x83155328 => {
    //   block [0x83155328..0x83155330)
	// 83155328: 806300B4  lwz r3, 0xb4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) } as u64;
	// 8315532C: 4801399C  b 0x83168cc8
	sub_83168CC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83155330 size=368
    let mut pc: u32 = 0x83155330;
    'dispatch: loop {
        match pc {
            0x83155330 => {
    //   block [0x83155330..0x831554A0)
	// 83155330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83155334: 48052E35  bl 0x831a8168
	ctx.lr = 0x83155338;
	sub_831A8130(ctx, base);
	// 83155338: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315533C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83155340: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83155344: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83155348: 419A0150  beq cr6, 0x83155498
	if ctx.cr[6].eq {
	pc = 0x83155498; continue 'dispatch;
	}
	// 8315534C: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83155350: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83155354: 409A001C  bne cr6, 0x83155370
	if !ctx.cr[6].eq {
	pc = 0x83155370; continue 'dispatch;
	}
	// 83155358: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8315535C: 48013F0D  bl 0x83169268
	ctx.lr = 0x83155360;
	sub_83169268(ctx, base);
	// 83155360: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83155364: 409A000C  bne cr6, 0x83155370
	if !ctx.cr[6].eq {
	pc = 0x83155370; continue 'dispatch;
	}
	// 83155368: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8315536C: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83155370: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83155374: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83155378: 409A0120  bne cr6, 0x83155498
	if !ctx.cr[6].eq {
	pc = 0x83155498; continue 'dispatch;
	}
	// 8315537C: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83155380: 48012F69  bl 0x831682e8
	ctx.lr = 0x83155384;
	sub_831682E8(ctx, base);
	// 83155384: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83155388: 2F1C0003  cmpwi cr6, r28, 3
	ctx.cr[6].compare_i32(ctx.r[28].s32, 3, &mut ctx.xer);
	// 8315538C: 409A000C  bne cr6, 0x83155398
	if !ctx.cr[6].eq {
	pc = 0x83155398; continue 'dispatch;
	}
	// 83155390: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83155394: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 83155398: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8315539C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 831553A0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831553A4: 409A0018  bne cr6, 0x831553bc
	if !ctx.cr[6].eq {
	pc = 0x831553BC; continue 'dispatch;
	}
	// 831553A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831553AC: 4BFFFDF5  bl 0x831551a0
	ctx.lr = 0x831553B0;
	sub_831551A0(ctx, base);
	// 831553B0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831553B4: 409A0008  bne cr6, 0x831553bc
	if !ctx.cr[6].eq {
	pc = 0x831553BC; continue 'dispatch;
	}
	// 831553B8: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 831553BC: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 831553C0: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 831553C4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831553C8: 409A0018  bne cr6, 0x831553e0
	if !ctx.cr[6].eq {
	pc = 0x831553E0; continue 'dispatch;
	}
	// 831553CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831553D0: 4BFFFEF1  bl 0x831552c0
	ctx.lr = 0x831553D4;
	sub_831552C0(ctx, base);
	// 831553D4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831553D8: 409A0008  bne cr6, 0x831553e0
	if !ctx.cr[6].eq {
	pc = 0x831553E0; continue 'dispatch;
	}
	// 831553DC: 93DF0034  stw r30, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[30].u32 ) };
	// 831553E0: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 831553E4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 831553E8: 409A00B0  bne cr6, 0x83155498
	if !ctx.cr[6].eq {
	pc = 0x83155498; continue 'dispatch;
	}
	// 831553EC: 817F01C4  lwz r11, 0x1c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(452 as u32) ) } as u64;
	// 831553F0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831553F4: 409A0068  bne cr6, 0x8315545c
	if !ctx.cr[6].eq {
	pc = 0x8315545C; continue 'dispatch;
	}
	// 831553F8: 897F0032  lbz r11, 0x32(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(50 as u32) ) } as u64;
	// 831553FC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83155400: 38BF00C4  addi r5, r31, 0xc4
	ctx.r[5].s64 = ctx.r[31].s64 + 196;
	// 83155404: 809F00C0  lwz r4, 0xc0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 83155408: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 8315540C: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 83155410: 556B183E  rotlwi r11, r11, 3
	ctx.r[11].u64 = ((ctx.r[11].u32).rotate_left(3)) as u64;
	// 83155414: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83155418: 7CCBFA14  add r6, r11, r31
	ctx.r[6].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8315541C: 7CE9F82E  lwzx r7, r9, r31
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 83155420: 80C6000C  lwz r6, 0xc(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 83155424: 48012EBD  bl 0x831682e0
	ctx.lr = 0x83155428;
	sub_831682E0(ctx, base);
	// 83155428: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8315542C: 409A0030  bne cr6, 0x8315545c
	if !ctx.cr[6].eq {
	pc = 0x8315545C; continue 'dispatch;
	}
	// 83155430: 897F0024  lbz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83155434: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83155438: 419A0014  beq cr6, 0x8315544c
	if ctx.cr[6].eq {
	pc = 0x8315544C; continue 'dispatch;
	}
	// 8315543C: A17F0026  lhz r11, 0x26(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(38 as u32) ) } as u64;
	// 83155440: 895F0032  lbz r10, 0x32(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(50 as u32) ) } as u64;
	// 83155444: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83155448: 419A0010  beq cr6, 0x83155458
	if ctx.cr[6].eq {
	pc = 0x83155458; continue 'dispatch;
	}
	// 8315544C: 897F0032  lbz r11, 0x32(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(50 as u32) ) } as u64;
	// 83155450: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83155454: 997F0032  stb r11, 0x32(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(50 as u32), ctx.r[11].u8 ) };
	// 83155458: 93DF01C4  stw r30, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[30].u32 ) };
	// 8315545C: 897F0032  lbz r11, 0x32(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(50 as u32) ) } as u64;
	// 83155460: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83155464: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 83155468: 4098000C  bge cr6, 0x83155474
	if !ctx.cr[6].lt {
	pc = 0x83155474; continue 'dispatch;
	}
	// 8315546C: 93BF01C4  stw r29, 0x1c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(452 as u32), ctx.r[29].u32 ) };
	// 83155470: 48000010  b 0x83155480
	pc = 0x83155480; continue 'dispatch;
	// 83155474: 9BBF0031  stb r29, 0x31(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(49 as u32), ctx.r[29].u8 ) };
	// 83155478: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 8315547C: 48012E8D  bl 0x83168308
	ctx.lr = 0x83155480;
	sub_83168308(ctx, base);
	// 83155480: 897F0031  lbz r11, 0x31(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(49 as u32) ) } as u64;
	// 83155484: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83155488: 409A0010  bne cr6, 0x83155498
	if !ctx.cr[6].eq {
	pc = 0x83155498; continue 'dispatch;
	}
	// 8315548C: 2F1C0002  cmpwi cr6, r28, 2
	ctx.cr[6].compare_i32(ctx.r[28].s32, 2, &mut ctx.xer);
	// 83155490: 409A0008  bne cr6, 0x83155498
	if !ctx.cr[6].eq {
	pc = 0x83155498; continue 'dispatch;
	}
	// 83155494: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 83155498: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8315549C: 48052D1C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831554A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831554A0 size=80
    let mut pc: u32 = 0x831554A0;
    'dispatch: loop {
        match pc {
            0x831554A0 => {
    //   block [0x831554A0..0x831554F0)
	// 831554A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831554A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831554A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831554AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831554B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831554B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831554B8: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831554BC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831554C0: 4E800421  bctrl
	ctx.lr = 0x831554C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831554C4: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831554C8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831554CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831554D0: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 831554D4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 831554D8: 4E800421  bctrl
	ctx.lr = 0x831554DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831554DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831554E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831554E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831554E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831554EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831554F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831554F0 size=144
    let mut pc: u32 = 0x831554F0;
    'dispatch: loop {
        match pc {
            0x831554F0 => {
    //   block [0x831554F0..0x83155580)
	// 831554F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831554F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831554F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831554FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83155500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83155504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83155508: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8315550C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83155510: 419A0014  beq cr6, 0x83155524
	if ctx.cr[6].eq {
	pc = 0x83155524; continue 'dispatch;
	}
	// 83155514: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83155518: 419A000C  beq cr6, 0x83155524
	if ctx.cr[6].eq {
	pc = 0x83155524; continue 'dispatch;
	}
	// 8315551C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83155520: 409A0044  bne cr6, 0x83155564
	if !ctx.cr[6].eq {
	pc = 0x83155564; continue 'dispatch;
	}
	// 83155524: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83155528: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8315552C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83155530: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83155534: 4E800421  bctrl
	ctx.lr = 0x83155538;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83155538: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 8315553C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83155540: 419A0028  beq cr6, 0x83155568
	if ctx.cr[6].eq {
	pc = 0x83155568; continue 'dispatch;
	}
	// 83155544: 83DF0020  lwz r30, 0x20(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83155548: 83FF0018  lwz r31, 0x18(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8315554C: 817E002C  lwz r11, 0x2c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 83155550: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83155554: 419A000C  beq cr6, 0x83155560
	if ctx.cr[6].eq {
	pc = 0x83155560; continue 'dispatch;
	}
	// 83155558: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8315555C: 4BFF66DD  bl 0x8314bc38
	ctx.lr = 0x83155560;
	sub_8314BC38(ctx, base);
	// 83155560: 93FE002C  stw r31, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[31].u32 ) };
	// 83155564: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83155568: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8315556C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83155570: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83155574: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83155578: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8315557C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83155580 size=20
    let mut pc: u32 = 0x83155580;
    'dispatch: loop {
        match pc {
            0x83155580 => {
    //   block [0x83155580..0x83155594)
	// 83155580: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83155584: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83155588: 5549DFFE  rlwinm r9, r10, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8315558C: 69230001  xori r3, r9, 1
	ctx.r[3].u64 = ctx.r[9].u64 ^ 1;
	// 83155590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83155598 size=8
    let mut pc: u32 = 0x83155598;
    'dispatch: loop {
        match pc {
            0x83155598 => {
    //   block [0x83155598..0x831555A0)
	// 83155598: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 8315559C: 4BFF691C  b 0x8314beb8
	sub_8314BEB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831555A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831555A0 size=8
    let mut pc: u32 = 0x831555A0;
    'dispatch: loop {
        match pc {
            0x831555A0 => {
    //   block [0x831555A0..0x831555A8)
	// 831555A0: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 831555A4: 4BFF6924  b 0x8314bec8
	sub_8314BEC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831555A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831555A8 size=8
    let mut pc: u32 = 0x831555A8;
    'dispatch: loop {
        match pc {
            0x831555A8 => {
    //   block [0x831555A8..0x831555B0)
	// 831555A8: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 831555AC: 4BFF693C  b 0x8314bee8
	sub_8314BEE8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831555B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831555B0 size=8
    let mut pc: u32 = 0x831555B0;
    'dispatch: loop {
        match pc {
            0x831555B0 => {
    //   block [0x831555B0..0x831555B8)
	// 831555B0: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 831555B4: 4BFF69BC  b 0x8314bf70
	sub_8314BF70(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831555B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831555B8 size=8
    let mut pc: u32 = 0x831555B8;
    'dispatch: loop {
        match pc {
            0x831555B8 => {
    //   block [0x831555B8..0x831555C0)
	// 831555B8: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 831555BC: 4BFF6964  b 0x8314bf20
	sub_8314BF20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831555C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831555C0 size=8
    let mut pc: u32 = 0x831555C0;
    'dispatch: loop {
        match pc {
            0x831555C0 => {
    //   block [0x831555C0..0x831555C8)
	// 831555C0: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 831555C4: 4BFF6974  b 0x8314bf38
	sub_8314BF38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831555C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831555C8 size=8
    let mut pc: u32 = 0x831555C8;
    'dispatch: loop {
        match pc {
            0x831555C8 => {
    //   block [0x831555C8..0x831555D0)
	// 831555C8: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 831555CC: 4BFF698C  b 0x8314bf58
	sub_8314BF58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831555D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831555D0 size=8
    let mut pc: u32 = 0x831555D0;
    'dispatch: loop {
        match pc {
            0x831555D0 => {
    //   block [0x831555D0..0x831555D8)
	// 831555D0: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 831555D4: 4BFF6904  b 0x8314bed8
	sub_8314BED8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831555D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831555D8 size=12
    let mut pc: u32 = 0x831555D8;
    'dispatch: loop {
        match pc {
            0x831555D8 => {
    //   block [0x831555D8..0x831555E4)
	// 831555D8: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 831555DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831555E0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831555E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831555E4 size=8
    let mut pc: u32 = 0x831555E4;
    'dispatch: loop {
        match pc {
            0x831555E4 => {
    //   block [0x831555E4..0x831555EC)
	// 831555E4: 4BFF69C4  b 0x8314bfa8
	sub_8314BFA8(ctx, base);
	return;
	// 831555E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831555F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831555F0 size=16
    let mut pc: u32 = 0x831555F0;
    'dispatch: loop {
        match pc {
            0x831555F0 => {
    //   block [0x831555F0..0x83155600)
	// 831555F0: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 831555F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831555F8: 419A0008  beq cr6, 0x83155600
	if ctx.cr[6].eq {
		sub_83155600(ctx, base);
		return;
	}
	// 831555FC: 4BEB3E84  b 0x83009480
	sub_83009480(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83155600 size=8
    let mut pc: u32 = 0x83155600;
    'dispatch: loop {
        match pc {
            0x83155600 => {
    //   block [0x83155600..0x83155608)
	// 83155600: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83155604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83155608 size=120
    let mut pc: u32 = 0x83155608;
    'dispatch: loop {
        match pc {
            0x83155608 => {
    //   block [0x83155608..0x83155680)
	// 83155608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315560C: 48052B61  bl 0x831a816c
	ctx.lr = 0x83155610;
	sub_831A8130(ctx, base);
	// 83155610: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83155614: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83155618: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8315561C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83155620: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83155624: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83155628: 419A0028  beq cr6, 0x83155650
	if ctx.cr[6].eq {
	pc = 0x83155650; continue 'dispatch;
	}
	// 8315562C: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83155630: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83155634: 419A000C  beq cr6, 0x83155640
	if ctx.cr[6].eq {
	pc = 0x83155640; continue 'dispatch;
	}
	// 83155638: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 8315563C: 409A0014  bne cr6, 0x83155650
	if !ctx.cr[6].eq {
	pc = 0x83155650; continue 'dispatch;
	}
	// 83155640: 93AB002C  stw r29, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 83155644: 93AB0028  stw r29, 0x28(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 83155648: 9BCB0031  stb r30, 0x31(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(49 as u32), ctx.r[30].u8 ) };
	// 8315564C: 9BCB0032  stb r30, 0x32(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(50 as u32), ctx.r[30].u8 ) };
	// 83155650: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83155654: 4BFF6F3D  bl 0x8314c590
	ctx.lr = 0x83155658;
	sub_8314C590(ctx, base);
	// 83155658: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8315565C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83155660: 409A0010  bne cr6, 0x83155670
	if !ctx.cr[6].eq {
	pc = 0x83155670; continue 'dispatch;
	}
	// 83155664: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83155668: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8315566C: 4BFF6FE5  bl 0x8314c650
	ctx.lr = 0x83155670;
	sub_8314C650(ctx, base);
	// 83155670: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 83155674: 93DF0024  stw r30, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[30].u32 ) };
	// 83155678: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8315567C: 48052B40  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83155680 size=64
    let mut pc: u32 = 0x83155680;
    'dispatch: loop {
        match pc {
            0x83155680 => {
    //   block [0x83155680..0x831556C0)
	// 83155680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83155684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83155688: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8315568C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83155690: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83155694: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83155698: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8315569C: 419A0008  beq cr6, 0x831556a4
	if ctx.cr[6].eq {
	pc = 0x831556A4; continue 'dispatch;
	}
	// 831556A0: 4BFF6F29  bl 0x8314c5c8
	ctx.lr = 0x831556A4;
	sub_8314C5C8(ctx, base);
	// 831556A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831556A8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 831556AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831556B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831556B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831556B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831556BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831556C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831556C0 size=64
    let mut pc: u32 = 0x831556C0;
    'dispatch: loop {
        match pc {
            0x831556C0 => {
    //   block [0x831556C0..0x83155700)
	// 831556C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831556C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831556C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831556CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831556D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831556D4: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 831556D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831556DC: 419A0008  beq cr6, 0x831556e4
	if ctx.cr[6].eq {
	pc = 0x831556E4; continue 'dispatch;
	}
	// 831556E0: 4BFF6F21  bl 0x8314c600
	ctx.lr = 0x831556E4;
	sub_8314C600(ctx, base);
	// 831556E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831556E8: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 831556EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831556F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831556F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831556F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831556FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83155700 size=12
    let mut pc: u32 = 0x83155700;
    'dispatch: loop {
        match pc {
            0x83155700 => {
    //   block [0x83155700..0x8315570C)
	// 83155700: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83155704: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83155708: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8315570C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8315570C size=8
    let mut pc: u32 = 0x8315570C;
    'dispatch: loop {
        match pc {
            0x8315570C => {
    //   block [0x8315570C..0x83155714)
	// 8315570C: 4BFF6F2C  b 0x8314c638
	sub_8314C638(ctx, base);
	return;
	// 83155710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83155718 size=72
    let mut pc: u32 = 0x83155718;
    'dispatch: loop {
        match pc {
            0x83155718 => {
    //   block [0x83155718..0x83155760)
	// 83155718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315571C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83155720: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83155724: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83155728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315572C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83155730: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83155734: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83155738: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8315573C: 419A0008  beq cr6, 0x83155744
	if ctx.cr[6].eq {
	pc = 0x83155744; continue 'dispatch;
	}
	// 83155740: 4BFF6F11  bl 0x8314c650
	ctx.lr = 0x83155744;
	sub_8314C650(ctx, base);
	// 83155744: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 83155748: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8315574C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83155750: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83155754: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83155758: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8315575C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83155760 size=396
    let mut pc: u32 = 0x83155760;
    'dispatch: loop {
        match pc {
            0x83155760 => {
    //   block [0x83155760..0x831558EC)
	// 83155760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83155764: 48052A09  bl 0x831a816c
	ctx.lr = 0x83155768;
	sub_831A8130(ctx, base);
	// 83155768: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8315576C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83155770: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83155774: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83155778: 419A000C  beq cr6, 0x83155784
	if ctx.cr[6].eq {
	pc = 0x83155784; continue 'dispatch;
	}
	// 8315577C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83155780: 409A0164  bne cr6, 0x831558e4
	if !ctx.cr[6].eq {
	pc = 0x831558E4; continue 'dispatch;
	}
	// 83155784: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83155788: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8315578C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83155790: 419A0014  beq cr6, 0x831557a4
	if ctx.cr[6].eq {
	pc = 0x831557A4; continue 'dispatch;
	}
	// 83155794: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83155798: 419A000C  beq cr6, 0x831557a4
	if ctx.cr[6].eq {
	pc = 0x831557A4; continue 'dispatch;
	}
	// 8315579C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 831557A0: 409A0094  bne cr6, 0x83155834
	if !ctx.cr[6].eq {
	pc = 0x83155834; continue 'dispatch;
	}
	// 831557A4: 7FBEEB78  mr r30, r29
	ctx.r[30].u64 = ctx.r[29].u64;
	// 831557A8: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 831557AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831557B0: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 831557B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831557B8: 4E800421  bctrl
	ctx.lr = 0x831557BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831557BC: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 831557C0: 81090028  lwz r8, 0x28(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 831557C4: 2F080003  cmpwi cr6, r8, 3
	ctx.cr[6].compare_i32(ctx.r[8].s32, 3, &mut ctx.xer);
	// 831557C8: 419A00C4  beq cr6, 0x8315588c
	if ctx.cr[6].eq {
	pc = 0x8315588C; continue 'dispatch;
	}
	// 831557CC: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 831557D0: 4BFF64E9  bl 0x8314bcb8
	ctx.lr = 0x831557D4;
	sub_8314BCB8(ctx, base);
	// 831557D4: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 831557D8: 2B1E0003  cmplwi cr6, r30, 3
	ctx.cr[6].compare_u32(ctx.r[30].u32, 3 as u32, &mut ctx.xer);
	// 831557DC: 4198FFCC  blt cr6, 0x831557a8
	if ctx.cr[6].lt {
	pc = 0x831557A8; continue 'dispatch;
	}
	// 831557E0: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 831557E4: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 831557E8: 2F0A0002  cmpwi cr6, r10, 2
	ctx.cr[6].compare_i32(ctx.r[10].s32, 2, &mut ctx.xer);
	// 831557EC: 409A0048  bne cr6, 0x83155834
	if !ctx.cr[6].eq {
	pc = 0x83155834; continue 'dispatch;
	}
	// 831557F0: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 831557F4: 4BFF6665  bl 0x8314be58
	ctx.lr = 0x831557F8;
	sub_8314BE58(ctx, base);
	// 831557F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831557FC: 409A0038  bne cr6, 0x83155834
	if !ctx.cr[6].eq {
	pc = 0x83155834; continue 'dispatch;
	}
	// 83155800: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83155804: 4BFF6675  bl 0x8314be78
	ctx.lr = 0x83155808;
	sub_8314BE78(ctx, base);
	// 83155808: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8315580C: 409A0028  bne cr6, 0x83155834
	if !ctx.cr[6].eq {
	pc = 0x83155834; continue 'dispatch;
	}
	// 83155810: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83155814: 93AB002C  stw r29, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 83155818: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8315581C: 4BFF6DE5  bl 0x8314c600
	ctx.lr = 0x83155820;
	sub_8314C600(ctx, base);
	// 83155820: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83155824: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83155828: 812A0010  lwz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 8315582C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83155830: 4E800421  bctrl
	ctx.lr = 0x83155834;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83155834: 807F0020  lwz r3, 0x20(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83155838: 4BFF6611  bl 0x8314be48
	ctx.lr = 0x8315583C;
	sub_8314BE48(ctx, base);
	// 8315583C: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83155840: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83155844: 409A0064  bne cr6, 0x831558a8
	if !ctx.cr[6].eq {
	pc = 0x831558A8; continue 'dispatch;
	}
	// 83155848: 2B030002  cmplwi cr6, r3, 2
	ctx.cr[6].compare_u32(ctx.r[3].u32, 2 as u32, &mut ctx.xer);
	// 8315584C: 40980098  bge cr6, 0x831558e4
	if !ctx.cr[6].lt {
	pc = 0x831558E4; continue 'dispatch;
	}
	// 83155850: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83155854: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83155858: 419A0044  beq cr6, 0x8315589c
	if ctx.cr[6].eq {
	pc = 0x8315589C; continue 'dispatch;
	}
	// 8315585C: 93AB002C  stw r29, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 83155860: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83155864: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83155868: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8315586C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83155870: 4E800421  bctrl
	ctx.lr = 0x83155874;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83155874: 813F001C  lwz r9, 0x1c(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83155878: 81690028  lwz r11, 0x28(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(40 as u32) ) } as u64;
	// 8315587C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83155880: 419A001C  beq cr6, 0x8315589c
	if ctx.cr[6].eq {
	pc = 0x8315589C; continue 'dispatch;
	}
	// 83155884: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83155888: 409A005C  bne cr6, 0x831558e4
	if !ctx.cr[6].eq {
	pc = 0x831558E4; continue 'dispatch;
	}
	// 8315588C: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83155890: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83155894: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83155898: 48052924  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8315589C: 93BF0014  stw r29, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[29].u32 ) };
	// 831558A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831558A4: 48052918  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831558A8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831558AC: 419A0030  beq cr6, 0x831558dc
	if ctx.cr[6].eq {
	pc = 0x831558DC; continue 'dispatch;
	}
	// 831558B0: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 831558B4: 40990030  ble cr6, 0x831558e4
	if !ctx.cr[6].gt {
	pc = 0x831558E4; continue 'dispatch;
	}
	// 831558B8: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 831558BC: 41990028  bgt cr6, 0x831558e4
	if ctx.cr[6].gt {
	pc = 0x831558E4; continue 'dispatch;
	}
	// 831558C0: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 831558C4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831558C8: 409A001C  bne cr6, 0x831558e4
	if !ctx.cr[6].eq {
	pc = 0x831558E4; continue 'dispatch;
	}
	// 831558CC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 831558D0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 831558D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831558D8: 480528E4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831558DC: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 831558E0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 831558E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831558E8: 480528D4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831558F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831558F0 size=32
    let mut pc: u32 = 0x831558F0;
    'dispatch: loop {
        match pc {
            0x831558F0 => {
    //   block [0x831558F0..0x83155910)
	// 831558F0: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831558F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831558F8: 419A0018  beq cr6, 0x83155910
	if ctx.cr[6].eq {
		sub_83155910(ctx, base);
		return;
	}
	// 831558FC: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83155900: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83155904: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83155908: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 8315590C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83155910 size=12
    let mut pc: u32 = 0x83155910;
    'dispatch: loop {
        match pc {
            0x83155910 => {
    //   block [0x83155910..0x8315591C)
	// 83155910: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 83155914: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 83155918: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83155920 size=16
    let mut pc: u32 = 0x83155920;
    'dispatch: loop {
        match pc {
            0x83155920 => {
    //   block [0x83155920..0x83155930)
	// 83155920: 80630020  lwz r3, 0x20(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83155924: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83155928: 419A0008  beq cr6, 0x83155930
	if ctx.cr[6].eq {
		sub_83155930(ctx, base);
		return;
	}
	// 8315592C: 4BFF653C  b 0x8314be68
	sub_8314BE68(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83155930 size=8
    let mut pc: u32 = 0x83155930;
    'dispatch: loop {
        match pc {
            0x83155930 => {
    //   block [0x83155930..0x83155938)
	// 83155930: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83155934: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155938(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83155938 size=76
    let mut pc: u32 = 0x83155938;
    'dispatch: loop {
        match pc {
            0x83155938 => {
    //   block [0x83155938..0x83155984)
	// 83155938: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315593C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83155940: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83155944: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83155948: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8315594C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83155950: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83155954: 419A001C  beq cr6, 0x83155970
	if ctx.cr[6].eq {
	pc = 0x83155970; continue 'dispatch;
	}
	// 83155958: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8315595C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83155960: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 83155964: 4E800421  bctrl
	ctx.lr = 0x83155968;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83155968: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 8315596C: 913F001C  stw r9, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[9].u32 ) };
	// 83155970: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83155974: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83155978: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8315597C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83155980: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83155988 size=108
    let mut pc: u32 = 0x83155988;
    'dispatch: loop {
        match pc {
            0x83155988 => {
    //   block [0x83155988..0x831559F4)
	// 83155988: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8315598C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83155990: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83155994: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83155998: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8315599C: 807F001C  lwz r3, 0x1c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 831559A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831559A4: 409A001C  bne cr6, 0x831559c0
	if !ctx.cr[6].eq {
	pc = 0x831559C0; continue 'dispatch;
	}
	// 831559A8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831559AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831559B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831559B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831559B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831559BC: 4E800020  blr
	return;
	// 831559C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831559C4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 831559C8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831559CC: 4E800421  bctrl
	ctx.lr = 0x831559D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831559D0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831559D4: 419A000C  beq cr6, 0x831559e0
	if ctx.cr[6].eq {
	pc = 0x831559E0; continue 'dispatch;
	}
	// 831559D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831559DC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 831559E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831559E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831559E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831559EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831559F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831559F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831559F8 size=12
    let mut pc: u32 = 0x831559F8;
    'dispatch: loop {
        match pc {
            0x831559F8 => {
    //   block [0x831559F8..0x83155A04)
	// 831559F8: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 831559FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83155A00: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83155A04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83155A04 size=8
    let mut pc: u32 = 0x83155A04;
    'dispatch: loop {
        match pc {
            0x83155A04 => {
    //   block [0x83155A04..0x83155A0C)
	// 83155A04: 908B0038  stw r4, 0x38(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(56 as u32), ctx.r[4].u32 ) };
	// 83155A08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


