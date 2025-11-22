pub fn sub_824F5F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F5F68 size=24
    let mut pc: u32 = 0x824F5F68;
    'dispatch: loop {
        match pc {
            0x824F5F68 => {
    //   block [0x824F5F68..0x824F5F80)
	// 824F5F68: 548B1838  slwi r11, r4, 3
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824F5F6C: 7D645A14  add r11, r4, r11
	ctx.r[11].u64 = ctx.r[4].u64 + ctx.r[11].u64;
	// 824F5F70: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824F5F74: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824F5F78: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 824F5F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F5F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F5F80 size=1036
    let mut pc: u32 = 0x824F5F80;
    'dispatch: loop {
        match pc {
            0x824F5F80 => {
    //   block [0x824F5F80..0x824F638C)
	// 824F5F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F5F84: 4803F105  bl 0x82535088
	ctx.lr = 0x824F5F88;
	sub_82535080(ctx, base);
	// 824F5F88: DBA1FF60  stfd f29, -0xa0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[29].u64 ) };
	// 824F5F8C: DBC1FF68  stfd f30, -0x98(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), ctx.f[30].u64 ) };
	// 824F5F90: DBE1FF70  stfd f31, -0x90(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[31].u64 ) };
	// 824F5F94: 3980FF50  li r12, -0xb0
	ctx.r[12].s64 = -176;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F6390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F6390 size=1208
    let mut pc: u32 = 0x824F6390;
    'dispatch: loop {
        match pc {
            0x824F6390 => {
    //   block [0x824F6390..0x824F6848)
	// 824F6390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F6394: 4803ECED  bl 0x82535080
	ctx.lr = 0x824F6398;
	sub_82535080(ctx, base);
	// 824F6398: DBA1FF50  stfd f29, -0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.f[29].u64 ) };
	// 824F639C: DBC1FF58  stfd f30, -0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 824F63A0: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 824F63A4: 3980FF40  li r12, -0xc0
	ctx.r[12].s64 = -192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F6848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824F6848 size=304
    let mut pc: u32 = 0x824F6848;
    'dispatch: loop {
        match pc {
            0x824F6848 => {
    //   block [0x824F6848..0x824F6978)
	// 824F6848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F684C: 4803E865  bl 0x825350b0
	ctx.lr = 0x824F6850;
	sub_82535080(ctx, base);
	// 824F6850: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824F6854: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824F6858: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824F685C: C12B1FF8  lfs f9, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 824F6860: FD604890  fmr f11, f9
	ctx.f[11].f64 = ctx.f[9].f64;
	// 824F6864: FC004890  fmr f0, f9
	ctx.f[0].f64 = ctx.f[9].f64;
	// 824F6868: 4099010C  ble cr6, 0x824f6974
	if !ctx.cr[6].gt {
	pc = 0x824F6974; continue 'dispatch;
	}
	// 824F686C: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 824F6870: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 824F6874: 3BA3FFFF  addi r29, r3, -1
	ctx.r[29].s64 = ctx.r[3].s64 + -1;
	// 824F6878: 3BCA9F60  addi r30, r10, -0x60a0
	ctx.r[30].s64 = ctx.r[10].s64 + -24736;
	// 824F687C: 38880004  addi r4, r8, 4
	ctx.r[4].s64 = ctx.r[8].s64 + 4;
	// 824F6880: C1892280  lfs f12, 0x2280(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8832 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824F6884: 38C60004  addi r6, r6, 4
	ctx.r[6].s64 = ctx.r[6].s64 + 4;
	// 824F6888: 39650010  addi r11, r5, 0x10
	ctx.r[11].s64 = ctx.r[5].s64 + 16;
	// 824F688C: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 824F6890: 3B600020  li r27, 0x20
	ctx.r[27].s64 = 32;
	// 824F6894: 3B80FFF0  li r28, -0x10
	ctx.r[28].s64 = -16;
	// 824F6898: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 824F689C: 81060000  lwz r8, 0(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F6978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824F6978 size=472
    let mut pc: u32 = 0x824F6978;
    'dispatch: loop {
        match pc {
            0x824F6978 => {
    //   block [0x824F6978..0x824F6B50)
	// 824F6978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F697C: 4803E72D  bl 0x825350a8
	ctx.lr = 0x824F6980;
	sub_82535080(ctx, base);
	// 824F6980: 83410054  lwz r26, 0x54(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824F6984: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 824F6988: 546A043E  clrlwi r10, r3, 0x10
	ctx.r[10].u64 = ctx.r[3].u32 as u64 & 0x0000FFFFu64;
	// 824F698C: 39230001  addi r9, r3, 1
	ctx.r[9].s64 = ctx.r[3].s64 + 1;
	// 824F6990: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 824F6994: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824F6998: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F699C: 3B2B0004  addi r25, r11, 4
	ctx.r[25].s64 = ctx.r[11].s64 + 4;
	// 824F69A0: 98CB0003  stb r6, 3(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(3 as u32), ctx.r[6].u8 ) };
	// 824F69A4: 1CC50044  mulli r6, r5, 0x44
	ctx.r[6].s64 = ctx.r[5].s64 * 68;
	// 824F69A8: D02B0008  stfs f1, 8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824F69AC: B14B0000  sth r10, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u16 ) };
	// 824F69B0: D04B000C  stfs f2, 0xc(r11)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824F69B4: 38C60027  addi r6, r6, 0x27
	ctx.r[6].s64 = ctx.r[6].s64 + 39;
	// 824F69B8: 54C60036  rlwinm r6, r6, 0, 0, 0x1b
	ctx.r[6].u64 = ctx.r[6].u32 as u64 & 0xFFFFFFFFu64;
	// 824F69BC: 90CB0004  stw r6, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 824F69C0: 4099002C  ble cr6, 0x824f69ec
	if !ctx.cr[6].gt {
	pc = 0x824F69EC; continue 'dispatch;
	}
	// 824F69C4: 554A32B2  rlwinm r10, r10, 6, 0xa, 0x19
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x03FFFFFFu64;
	// 824F69C8: 7D475050  subf r10, r7, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[7].s64;
	// 824F69CC: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 824F69D0: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 824F69D4: 80C70000  lwz r6, 0(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F69D8: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 824F69DC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824F69E0: 7CCA392E  stwx r6, r10, r7
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[7].u32), ctx.r[6].u32) };
	// 824F69E4: 38E70004  addi r7, r7, 4
	ctx.r[7].s64 = ctx.r[7].s64 + 4;
	// 824F69E8: 409AFFEC  bne cr6, 0x824f69d4
	if !ctx.cr[6].eq {
	pc = 0x824F69D4; continue 'dispatch;
	}
	// 824F69EC: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 824F69F0: A14B0000  lhz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F69F4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824F69F8: 5547083E  rotlwi r7, r10, 1
	ctx.r[7].u64 = ((ctx.r[10].u32).rotate_left(1)) as u64;
	// 824F69FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824F6A00: 7CEA3A14  add r7, r10, r7
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 824F6A04: C1291FF8  lfs f9, 0x1ff8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8184 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 824F6A08: 813A0000  lwz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F6A0C: FD604890  fmr f11, f9
	ctx.f[11].f64 = ctx.f[9].f64;
	// 824F6A10: 38A90010  addi r5, r9, 0x10
	ctx.r[5].s64 = ctx.r[9].s64 + 16;
	// 824F6A14: FC004890  fmr f0, f9
	ctx.f[0].f64 = ctx.f[9].f64;
	// 824F6A18: 5549303E  rotlwi r9, r10, 6
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(6)) as u64;
	// 824F6A1C: 7D495A14  add r10, r9, r11
	ctx.r[10].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824F6A20: 54E92036  slwi r9, r7, 4
	ctx.r[9].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 824F6A24: 394A0014  addi r10, r10, 0x14
	ctx.r[10].s64 = ctx.r[10].s64 + 20;
	// 824F6A28: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 824F6A2C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 824F6A30: 4099010C  ble cr6, 0x824f6b3c
	if !ctx.cr[6].gt {
	pc = 0x824F6B3C; continue 'dispatch;
	}
	// 824F6A34: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 824F6A38: 38CA0004  addi r6, r10, 4
	ctx.r[6].s64 = ctx.r[10].s64 + 4;
	// 824F6A3C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 824F6A40: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 824F6A44: 3BCA9F60  addi r30, r10, -0x60a0
	ctx.r[30].s64 = ctx.r[10].s64 + -24736;
	// 824F6A48: C1892280  lfs f12, 0x2280(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8832 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824F6A4C: 3BA3FFFF  addi r29, r3, -1
	ctx.r[29].s64 = ctx.r[3].s64 + -1;
	// 824F6A50: 39650010  addi r11, r5, 0x10
	ctx.r[11].s64 = ctx.r[5].s64 + 16;
	// 824F6A54: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 824F6A58: 3B600020  li r27, 0x20
	ctx.r[27].s64 = 32;
	// 824F6A5C: 3B80FFF0  li r28, -0x10
	ctx.r[28].s64 = -16;
	// 824F6A60: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 824F6A64: 80E60000  lwz r7, 0(r6)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F6B50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824F6B50 size=116
    let mut pc: u32 = 0x824F6B50;
    'dispatch: loop {
        match pc {
            0x824F6B50 => {
    //   block [0x824F6B50..0x824F6BC4)
	// 824F6B50: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 824F6B54: C0040010  lfs f0, 0x10(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824F6B58: C1640004  lfs f11, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824F6B5C: FD405850  fneg f10, f11
	ctx.f[10].u64 = ctx.f[11].u64 ^ 0x8000_0000_0000_0000u64;
	// 824F6B60: ED610028  fsubs f11, f1, f0
	ctx.f[11].f64 = (((ctx.f[1].f64 - ctx.f[0].f64) as f32) as f64);
	// 824F6B64: C1ABC38C  lfs f13, -0x3c74(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15476 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824F6B68: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 824F6B6C: FD806850  fneg f12, f13
	ctx.f[12].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 824F6B70: C1ABC388  lfs f13, -0x3c78(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-15480 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824F6B74: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824F6B78: EDAD00B2  fmuls f13, f13, f2
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[2].f64) as f32) as f64);
	// 824F6B7C: ED4A5828  fsubs f10, f10, f11
	ctx.f[10].f64 = (((ctx.f[10].f64 - ctx.f[11].f64) as f32) as f64);
	// 824F6B80: C12B2068  lfs f9, 0x2068(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8296 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 824F6B84: ED8C0032  fmuls f12, f12, f0
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 824F6B88: ED0C6828  fsubs f8, f12, f13
	ctx.f[8].f64 = (((ctx.f[12].f64 - ctx.f[13].f64) as f32) as f64);
	// 824F6B8C: FD88636E  fsel f12, f8, f13, f12
	ctx.f[12].f64 = if ctx.f[8].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[12].f64 };
	// 824F6B90: EDAC6A7A  fmadds f13, f12, f9, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[9].f64 + ctx.f[13].f64) as f32) as f64);
	// 824F6B94: EC0C002A  fadds f0, f12, f0
	ctx.f[0].f64 = ((ctx.f[12].f64 + ctx.f[0].f64) as f32) as f64;
	// 824F6B98: EC2B6028  fsubs f1, f11, f12
	ctx.f[1].f64 = (((ctx.f[11].f64 - ctx.f[12].f64) as f32) as f64);
	// 824F6B9C: FF0A6800  fcmpu cr6, f10, f13
	ctx.cr[6].compare_f64(ctx.f[10].f64, ctx.f[13].f64);
	// 824F6BA0: 4099000C  ble cr6, 0x824f6bac
	if !ctx.cr[6].gt {
	pc = 0x824F6BAC; continue 'dispatch;
	}
	// 824F6BA4: EC005028  fsubs f0, f0, f10
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[10].f64) as f32) as f64);
	// 824F6BA8: EC2A082A  fadds f1, f10, f1
	ctx.f[1].f64 = ((ctx.f[10].f64 + ctx.f[1].f64) as f32) as f64;
	// 824F6BAC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F6BB0: C1AB2600  lfs f13, 0x2600(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(9728 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824F6BB4: ED806828  fsubs f12, f0, f13
	ctx.f[12].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 824F6BB8: FC0C036E  fsel f0, f12, f13, f0
	ctx.f[0].f64 = if ctx.f[12].f64 >= 0.0 { ctx.f[13].f64 } else { ctx.f[0].f64 };
	// 824F6BBC: D0040010  stfs f0, 0x10(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 824F6BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F6BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824F6BC8 size=24
    let mut pc: u32 = 0x824F6BC8;
    'dispatch: loop {
        match pc {
            0x824F6BC8 => {
    //   block [0x824F6BC8..0x824F6BE0)
	// 824F6BC8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824F6BCC: C00B1FF8  lfs f0, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824F6BD0: D003000C  stfs f0, 0xc(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824F6BD4: D0030014  stfs f0, 0x14(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), tmp.u32 ) };
	// 824F6BD8: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824F6BDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F6BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824F6BE0 size=2892
    let mut pc: u32 = 0x824F6BE0;
    'dispatch: loop {
        match pc {
            0x824F6BE0 => {
    //   block [0x824F6BE0..0x824F772C)
	// 824F6BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F6BE4: 4803E49D  bl 0x82535080
	ctx.lr = 0x824F6BE8;
	sub_82535080(ctx, base);
	// 824F6BE8: DBA1FF50  stfd f29, -0xb0(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-176 as u32), ctx.f[29].u64 ) };
	// 824F6BEC: DBC1FF58  stfd f30, -0xa8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-168 as u32), ctx.f[30].u64 ) };
	// 824F6BF0: DBE1FF60  stfd f31, -0xa0(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-160 as u32), ctx.f[31].u64 ) };
	// 824F6BF4: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 824F6BF8: A0E30004  lhz r7, 4(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F6BFC: 3961FF40  addi r11, r1, -0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + -192;
	// 824F6C00: 90C1002C  stw r6, 0x2c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(44 as u32), ctx.r[6].u32 ) };
	// 824F6C04: 8923000A  lbz r9, 0xa(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(10 as u32) ) } as u64;
	// 824F6C08: 7CA50774  extsb r5, r5
	ctx.r[5].s64 = ctx.r[5].s8 as i64;
	// 824F6C0C: 83E40030  lwz r31, 0x30(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 824F6C10: 83C40034  lwz r30, 0x34(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(52 as u32) ) } as u64;
	// 824F6C14: 2F050000  cmpwi cr6, r5, 0
	ctx.cr[6].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 824F6C18: C0481FF8  lfs f2, 0x1ff8(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(8184 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 824F6C1C: 39030010  addi r8, r3, 0x10
	ctx.r[8].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F7730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F7730 size=276
    let mut pc: u32 = 0x824F7730;
    'dispatch: loop {
        match pc {
            0x824F7730 => {
    //   block [0x824F7730..0x824F7844)
	// 824F7730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F7734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F7738: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824F773C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824F7740: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F7848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824F7848 size=8
    let mut pc: u32 = 0x824F7848;
    'dispatch: loop {
        match pc {
            0x824F7848 => {
    //   block [0x824F7848..0x824F7850)
	// 824F7848: D0240004  stfs f1, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824F784C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F7850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824F7850 size=12
    let mut pc: u32 = 0x824F7850;
    'dispatch: loop {
        match pc {
            0x824F7850 => {
    //   block [0x824F7850..0x824F785C)
	// 824F7850: D0250000  stfs f1, 0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824F7854: D0450004  stfs f2, 4(r5)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824F7858: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F7860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824F7860 size=20
    let mut pc: u32 = 0x824F7860;
    'dispatch: loop {
        match pc {
            0x824F7860 => {
    //   block [0x824F7860..0x824F7874)
	// 824F7860: D0270000  stfs f1, 0(r7)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 824F7864: D0470004  stfs f2, 4(r7)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 824F7868: D0670008  stfs f3, 8(r7)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), tmp.u32 ) };
	// 824F786C: D087000C  stfs f4, 0xc(r7)
	tmp.f32 = (ctx.f[4].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824F7870: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F7878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824F7878 size=3356
    let mut pc: u32 = 0x824F7878;
    'dispatch: loop {
        match pc {
            0x824F7878 => {
    //   block [0x824F7878..0x824F7940)
	// 824F7878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F787C: 4803D805  bl 0x82535080
	ctx.lr = 0x824F7880;
	sub_82535080(ctx, base);
	// 824F7880: 81630114  lwz r11, 0x114(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(276 as u32) ) } as u64;
	// 824F7884: C003010C  lfs f0, 0x10c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(268 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824F7888: C1A30050  lfs f13, 0x50(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(80 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824F788C: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 824F7890: 7D6807B4  extsw r8, r11
	ctx.r[8].s64 = ctx.r[11].s32 as i64;
	// 824F7894: ECCD0032  fmuls f6, f13, f0
	ctx.f[6].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 824F7898: 38E3010C  addi r7, r3, 0x10c
	ctx.r[7].s64 = ctx.r[3].s64 + 268;
	// 824F789C: C1830058  lfs f12, 0x58(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(88 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824F78A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824F78A4: 90C1002C  stw r6, 0x2c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(44 as u32), ctx.r[6].u32 ) };
	// 824F78A8: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 824F78AC: ECEC0032  fmuls f7, f12, f0
	ctx.f[7].f64 = (((ctx.f[12].f64 * ctx.f[0].f64) as f32) as f64);
	// 824F78B0: 7D7A5B78  mr r26, r11
	ctx.r[26].u64 = ctx.r[11].u64;
	// 824F78B4: 91410024  stw r10, 0x24(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), ctx.r[10].u32 ) };
	// 824F78B8: F901FED0  std r8, -0x130(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-304 as u32), ctx.r[8].u64 ) };
	// 824F78BC: 390A0010  addi r8, r10, 0x10
	ctx.r[8].s64 = ctx.r[10].s64 + 16;
	// 824F78C0: 90E1FEB0  stw r7, -0x150(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-336 as u32), ctx.r[7].u32 ) };
	// 824F78C4: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 824F78C8: 7D7D5B78  mr r29, r11
	ctx.r[29].u64 = ctx.r[11].u64;
	// 824F78CC: 9161FE78  stw r11, -0x188(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-392 as u32), ctx.r[11].u32 ) };
	// 824F78D0: 3C808200  lis r4, -0x7e00
	ctx.r[4].s64 = -2113929216;
	// 824F78D4: 9161FE8C  stw r11, -0x174(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-372 as u32), ctx.r[11].u32 ) };
	// 824F78D8: 3CE0820D  lis r7, -0x7df3
	ctx.r[7].s64 = -2113077248;
	// 824F78DC: 9341FEB8  stw r26, -0x148(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-328 as u32), ctx.r[26].u32 ) };
	// 824F78E0: 9101FEC4  stw r8, -0x13c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-316 as u32), ctx.r[8].u32 ) };
	// 824F78E4: 3D00820A  lis r8, -0x7df6
	ctx.r[8].s64 = -2113273856;
	// 824F78E8: 9381FE74  stw r28, -0x18c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-396 as u32), ctx.r[28].u32 ) };
	// 824F78EC: FD200090  fmr f9, f0
	ctx.f[9].f64 = ctx.f[0].f64;
	// 824F78F0: 3A889F60  addi r20, r8, -0x60a0
	ctx.r[20].s64 = ctx.r[8].s64 + -24736;
	// 824F78F4: 93A1FE70  stw r29, -0x190(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-400 as u32), ctx.r[29].u32 ) };
	// 824F78F8: 3D008282  lis r8, -0x7d7e
	ctx.r[8].s64 = -2105409536;
	// 824F78FC: C1042950  lfs f8, 0x2950(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(10576 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 824F7900: C0A72460  lfs f5, 0x2460(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(9312 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 824F7904: 39E00050  li r15, 0x50
	ctx.r[15].s64 = 80;
	// 824F7908: 3AE8C3A8  addi r23, r8, -0x3c58
	ctx.r[23].s64 = ctx.r[8].s64 + -15448;
	// 824F790C: 3D008282  lis r8, -0x7d7e
	ctx.r[8].s64 = -2105409536;
	// 824F7910: 3BC00040  li r30, 0x40
	ctx.r[30].s64 = 64;
	// 824F7914: 3B28C3C8  addi r25, r8, -0x3c38
	ctx.r[25].s64 = ctx.r[8].s64 + -15416;
	// 824F7918: 3B600020  li r27, 0x20
	ctx.r[27].s64 = 32;
	// 824F791C: 92E1FED8  stw r23, -0x128(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-296 as u32), ctx.r[23].u32 ) };
	// 824F7920: 3A400010  li r18, 0x10
	ctx.r[18].s64 = 16;
	// 824F7924: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 824F7928: 39C0FFF4  li r14, -0xc
	ctx.r[14].s64 = -12;
	// 824F792C: C9A1FED0  lfd f13, -0x130(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-304 as u32) ) };
	// 824F7930: 9321FED0  stw r25, -0x130(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-304 as u32), ctx.r[25].u32 ) };
	// 824F7934: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 824F7938: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 824F793C: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	pc = 0x824F7940; continue 'dispatch;
            }
            0x824F7940 => {
    //   block [0x824F7940..0x824F79DC)
	// 824F7940: 890A0003  lbz r8, 3(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 824F7944: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 824F7948: 2B07001C  cmplwi cr6, r7, 0x1c
	ctx.cr[6].compare_u32(ctx.r[7].u32, 28 as u32, &mut ctx.xer);
	// 824F794C: 4199FFF4  bgt cr6, 0x824f7940
	if ctx.cr[6].gt {
	pc = 0x824F7940; continue 'dispatch;
	}
	// 824F7950: 3D80824F  lis r12, -0x7db1
	ctx.r[12].s64 = -2108751872;
	// 824F7954: 398C7968  addi r12, r12, 0x7968
	ctx.r[12].s64 = ctx.r[12].s64 + 31080;
	// 824F7958: 54E0103A  slwi r0, r7, 2
	ctx.r[0].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 824F795C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 824F7960: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 824F7964: 4E800420  bctr
	match ctx.r[7].u64 {
		0 => {
	pc = 0x824F8584; continue 'dispatch;
		},
		1 => {
	pc = 0x824F79DC; continue 'dispatch;
		},
		2 => {
	pc = 0x824F81D4; continue 'dispatch;
		},
		3 => {
	pc = 0x824F7A68; continue 'dispatch;
		},
		4 => {
	pc = 0x824F7940; continue 'dispatch;
		},
		5 => {
	pc = 0x824F7B8C; continue 'dispatch;
		},
		6 => {
	pc = 0x824F7B80; continue 'dispatch;
		},
		7 => {
	pc = 0x824F7B80; continue 'dispatch;
		},
		8 => {
	pc = 0x824F7C20; continue 'dispatch;
		},
		9 => {
	pc = 0x824F7BF8; continue 'dispatch;
		},
		10 => {
	pc = 0x824F7DA0; continue 'dispatch;
		},
		11 => {
	pc = 0x824F7CA4; continue 'dispatch;
		},
		12 => {
	pc = 0x824F7CB0; continue 'dispatch;
		},
		13 => {
	pc = 0x824F7D30; continue 'dispatch;
		},
		14 => {
	pc = 0x824F7D08; continue 'dispatch;
		},
		15 => {
	pc = 0x824F7B8C; continue 'dispatch;
		},
		16 => {
	pc = 0x824F7B8C; continue 'dispatch;
		},
		17 => {
	pc = 0x824F7B34; continue 'dispatch;
		},
		18 => {
	pc = 0x824F7B34; continue 'dispatch;
		},
		19 => {
	pc = 0x824F7A80; continue 'dispatch;
		},
		20 => {
	pc = 0x824F7A80; continue 'dispatch;
		},
		21 => {
	pc = 0x824F7E18; continue 'dispatch;
		},
		22 => {
	pc = 0x824F7ED8; continue 'dispatch;
		},
		23 => {
	pc = 0x824F8104; continue 'dispatch;
		},
		24 => {
	pc = 0x824F7FDC; continue 'dispatch;
		},
		25 => {
	pc = 0x824F7940; continue 'dispatch;
		},
		26 => {
	pc = 0x824F810C; continue 'dispatch;
		},
		27 => {
	pc = 0x824F81E0; continue 'dispatch;
		},
		28 => {
	pc = 0x824F82C8; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 824F7968: 824F8584  lwz r18, -0x7a7c(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-31356 as u32) ) } as u64;
	// 824F796C: 824F79DC  lwz r18, 0x79dc(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31196 as u32) ) } as u64;
	// 824F7970: 824F81D4  lwz r18, -0x7e2c(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-32300 as u32) ) } as u64;
	// 824F7974: 824F7A68  lwz r18, 0x7a68(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31336 as u32) ) } as u64;
	// 824F7978: 824F7940  lwz r18, 0x7940(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31040 as u32) ) } as u64;
	// 824F797C: 824F7B8C  lwz r18, 0x7b8c(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31628 as u32) ) } as u64;
	// 824F7980: 824F7B80  lwz r18, 0x7b80(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31616 as u32) ) } as u64;
	// 824F7984: 824F7B80  lwz r18, 0x7b80(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31616 as u32) ) } as u64;
	// 824F7988: 824F7C20  lwz r18, 0x7c20(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31776 as u32) ) } as u64;
	// 824F798C: 824F7BF8  lwz r18, 0x7bf8(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31736 as u32) ) } as u64;
	// 824F7990: 824F7DA0  lwz r18, 0x7da0(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(32160 as u32) ) } as u64;
	// 824F7994: 824F7CA4  lwz r18, 0x7ca4(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31908 as u32) ) } as u64;
	// 824F7998: 824F7CB0  lwz r18, 0x7cb0(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31920 as u32) ) } as u64;
	// 824F799C: 824F7D30  lwz r18, 0x7d30(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(32048 as u32) ) } as u64;
	// 824F79A0: 824F7D08  lwz r18, 0x7d08(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(32008 as u32) ) } as u64;
	// 824F79A4: 824F7B8C  lwz r18, 0x7b8c(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31628 as u32) ) } as u64;
	// 824F79A8: 824F7B8C  lwz r18, 0x7b8c(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31628 as u32) ) } as u64;
	// 824F79AC: 824F7B34  lwz r18, 0x7b34(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31540 as u32) ) } as u64;
	// 824F79B0: 824F7B34  lwz r18, 0x7b34(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31540 as u32) ) } as u64;
	// 824F79B4: 824F7A80  lwz r18, 0x7a80(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31360 as u32) ) } as u64;
	// 824F79B8: 824F7A80  lwz r18, 0x7a80(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31360 as u32) ) } as u64;
	// 824F79BC: 824F7E18  lwz r18, 0x7e18(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(32280 as u32) ) } as u64;
	// 824F79C0: 824F7ED8  lwz r18, 0x7ed8(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(32472 as u32) ) } as u64;
	// 824F79C4: 824F8104  lwz r18, -0x7efc(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-32508 as u32) ) } as u64;
	// 824F79C8: 824F7FDC  lwz r18, 0x7fdc(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(32732 as u32) ) } as u64;
	// 824F79CC: 824F7940  lwz r18, 0x7940(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(31040 as u32) ) } as u64;
	// 824F79D0: 824F810C  lwz r18, -0x7ef4(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-32500 as u32) ) } as u64;
	// 824F79D4: 824F81E0  lwz r18, -0x7e20(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-32288 as u32) ) } as u64;
	// 824F79D8: 824F82C8  lwz r18, -0x7d38(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-32056 as u32) ) } as u64;
            }
            0x824F79DC => {
    //   block [0x824F79DC..0x824F7A68)
	// 824F79DC: A10A0004  lhz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F79E0: A0AA0006  lhz r5, 6(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(6 as u32) ) } as u64;
	// 824F79E4: 5507383E  rotlwi r7, r8, 7
	ctx.r[7].u64 = ((ctx.r[8].u32).rotate_left(7)) as u64;
	// 824F79E8: 8B4A0001  lbz r26, 1(r10)
	ctx.r[26].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(1 as u32) ) } as u64;
	// 824F79EC: 54A8383E  rotlwi r8, r5, 7
	ctx.r[8].u64 = ((ctx.r[5].u32).rotate_left(7)) as u64;
	// 824F79F0: 80AA000C  lwz r5, 0xc(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 824F79F4: 7F873214  add r28, r7, r6
	ctx.r[28].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 824F79F8: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824F79FC: 7FA83214  add r29, r8, r6
	ctx.r[29].u64 = ctx.r[8].u64 + ctx.r[6].u64;
	// 824F7A00: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 824F7A04: 9341FEB8  stw r26, -0x148(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-328 as u32), ctx.r[26].u32 ) };
	// 824F7A08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824F7A0C: 90A1FE8C  stw r5, -0x174(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-372 as u32), ctx.r[5].u32 ) };
	// 824F7A10: 9381FE74  stw r28, -0x18c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-396 as u32), ctx.r[28].u32 ) };
	// 824F7A14: 93A1FE70  stw r29, -0x190(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-400 as u32), ctx.r[29].u32 ) };
	// 824F7A18: 409AFF28  bne cr6, 0x824f7940
	if !ctx.cr[6].eq {
	pc = 0x824F7940; continue 'dispatch;
	}
	// 824F7A1C: 890A0003  lbz r8, 3(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 824F7A20: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 824F7A24: 2F070004  cmpwi cr6, r7, 4
	ctx.cr[6].compare_i32(ctx.r[7].s32, 4, &mut ctx.xer);
	// 824F7A28: 4099FF18  ble cr6, 0x824f7940
	if !ctx.cr[6].gt {
	pc = 0x824F7940; continue 'dispatch;
	}
	// 824F7A2C: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 824F7A30: 2F070019  cmpwi cr6, r7, 0x19
	ctx.cr[6].compare_i32(ctx.r[7].s32, 25, &mut ctx.xer);
	// 824F7A34: 4098FF0C  bge cr6, 0x824f7940
	if !ctx.cr[6].lt {
	pc = 0x824F7940; continue 'dispatch;
	}
	// 824F7A38: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 824F7A3C: 7D050774  extsb r5, r8
	ctx.r[5].s64 = ctx.r[8].s8 as i64;
	// 824F7A40: 7D07B8AE  lbzx r8, r7, r23
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 824F7A44: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824F7A48: 7D05C8AE  lbzx r8, r5, r25
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[5].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824F7A4C: 5508103E  rotlwi r8, r8, 2
	ctx.r[8].u64 = ((ctx.r[8].u32).rotate_left(2)) as u64;
	// 824F7A50: 7D284A14  add r9, r8, r9
	ctx.r[9].u64 = ctx.r[8].u64 + ctx.r[9].u64;
	// 824F7A54: 890A0003  lbz r8, 3(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 824F7A58: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 824F7A5C: 2F070004  cmpwi cr6, r7, 4
	ctx.cr[6].compare_i32(ctx.r[7].s32, 4, &mut ctx.xer);
	// 824F7A60: 4199FFCC  bgt cr6, 0x824f7a2c
	if ctx.cr[6].gt {
	pc = 0x824F7A2C; continue 'dispatch;
	}
	// 824F7A64: 4BFFFEDC  b 0x824f7940
	pc = 0x824F7940; continue 'dispatch;
            }
            0x824F7A68 => {
    //   block [0x824F7A68..0x824F7A80)
	// 824F7A68: 88EA0004  lbz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F7A6C: 7F480734  extsh r8, r26
	ctx.r[8].s64 = ctx.r[26].s16 as i64;
	// 824F7A70: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 824F7A74: 7D0741D6  mullw r8, r7, r8
	ctx.r[8].s64 = (ctx.r[7].s32 as i64) * (ctx.r[8].s32 as i64);
	// 824F7A78: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 824F7A7C: 4BFFFEC4  b 0x824f7940
	pc = 0x824F7940; continue 'dispatch;
            }
            0x824F7A80 => {
    //   block [0x824F7A80..0x824F7B34)
	// 824F7A80: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	pc = 0x824F7B34; continue 'dispatch;
            }
            0x824F7B34 => {
    //   block [0x824F7B34..0x824F7B80)
	// 824F7B34: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 824F7B38: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824F7B3C: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 824F7B40: C188002C  lfs f12, 0x2c(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(44 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824F7B44: ED8C0172  fmuls f12, f12, f5
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[5].f64) as f32) as f64);
	// 824F7B48: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 824F7B4C: 40990048  ble cr6, 0x824f7b94
	if !ctx.cr[6].gt {
	pc = 0x824F7B94; continue 'dispatch;
	}
	// 824F7B50: 80A1FE78  lwz r5, -0x188(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-392 as u32) ) } as u64;
	// 824F7B54: 80E1FEC4  lwz r7, -0x13c(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-316 as u32) ) } as u64;
	// 824F7B58: 38A50001  addi r5, r5, 1
	ctx.r[5].s64 = ctx.r[5].s64 + 1;
	// 824F7B5C: 91670008  stw r11, 8(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824F7B60: 90A1FE78  stw r5, -0x188(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-392 as u32), ctx.r[5].u32 ) };
	// 824F7B64: 38A70010  addi r5, r7, 0x10
	ctx.r[5].s64 = ctx.r[7].s64 + 16;
	// 824F7B68: 90A1FEC4  stw r5, -0x13c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-316 as u32), ctx.r[5].u32 ) };
	// 824F7B6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824F7B70: 90A70000  stw r5, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 824F7B74: 80A1FE8C  lwz r5, -0x174(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-372 as u32) ) } as u64;
	// 824F7B78: 90A70004  stw r5, 4(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824F7B7C: 48000018  b 0x824f7b94
	pc = 0x824F7B94; continue 'dispatch;
            }
            0x824F7B80 => {
    //   block [0x824F7B80..0x824F7B8C)
	// 824F7B80: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 824F7B84: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	// 824F7B88: 4800000C  b 0x824f7b94
	pc = 0x824F7B94; continue 'dispatch;
            }
            0x824F7B8C => {
    //   block [0x824F7B8C..0x824F7BF8)
	// 824F7B8C: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 824F7B90: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	pc = 0x824F7BF8; continue 'dispatch;
            }
            0x824F7BF8 => {
    //   block [0x824F7BF8..0x824F7C20)
	// 824F7BF8: C00A000C  lfs f0, 0xc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824F7BFC: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 824F7C00: C1690008  lfs f11, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824F7C04: C1890004  lfs f12, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824F7C08: EC0B002A  fadds f0, f11, f0
	ctx.f[0].f64 = ((ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64;
	// 824F7C0C: D00A000C  stfs f0, 0xc(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), tmp.u32 ) };
	// 824F7C10: 88EA0003  lbz r7, 3(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 824F7C14: 394A0050  addi r10, r10, 0x50
	ctx.r[10].s64 = ctx.r[10].s64 + 80;
	// 824F7C18: 7CE70774  extsb r7, r7
	ctx.r[7].s64 = ctx.r[7].s8 as i64;
	// 824F7C1C: 48000014  b 0x824f7c30
	pc = 0x824F7C30; continue 'dispatch;
            }
            0x824F7C20 => {
    //   block [0x824F7C20..0x824F7CA4)
	// 824F7C20: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 824F7C24: C1890004  lfs f12, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824F7C28: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 824F7C2C: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	pc = 0x824F7CA4; continue 'dispatch;
            }
            0x824F7CA4 => {
    //   block [0x824F7CA4..0x824F7CB0)
	// 824F7CA4: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 824F7CA8: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 824F7CAC: 4800000C  b 0x824f7cb8
	pc = 0x824F7CB8; continue 'dispatch;
            }
            0x824F7CB0 => {
    //   block [0x824F7CB0..0x824F7D08)
	// 824F7CB0: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 824F7CB4: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	pc = 0x824F7D08; continue 'dispatch;
            }
            0x824F7D08 => {
    //   block [0x824F7D08..0x824F7D30)
	// 824F7D08: C00A001C  lfs f0, 0x1c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824F7D0C: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 824F7D10: C1690008  lfs f11, 8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 824F7D14: C1890004  lfs f12, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824F7D18: EC0B002A  fadds f0, f11, f0
	ctx.f[0].f64 = ((ctx.f[11].f64 + ctx.f[0].f64) as f32) as f64;
	// 824F7D1C: D00A001C  stfs f0, 0x1c(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 824F7D20: 88EA0003  lbz r7, 3(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(3 as u32) ) } as u64;
	// 824F7D24: 394A0040  addi r10, r10, 0x40
	ctx.r[10].s64 = ctx.r[10].s64 + 64;
	// 824F7D28: 7CE70774  extsb r7, r7
	ctx.r[7].s64 = ctx.r[7].s8 as i64;
	// 824F7D2C: 48000014  b 0x824f7d40
	pc = 0x824F7D40; continue 'dispatch;
            }
            0x824F7D30 => {
    //   block [0x824F7D30..0x824F7DA0)
	// 824F7D30: 7D070774  extsb r7, r8
	ctx.r[7].s64 = ctx.r[8].s8 as i64;
	// 824F7D34: C1890004  lfs f12, 4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824F7D38: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 824F7D3C: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	pc = 0x824F7DA0; continue 'dispatch;
            }
            0x824F7DA0 => {
    //   block [0x824F7DA0..0x824F7E18)
	// 824F7DA0: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	pc = 0x824F7E18; continue 'dispatch;
            }
            0x824F7E18 => {
    //   block [0x824F7E18..0x824F7ED8)
	// 824F7E18: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	pc = 0x824F7ED8; continue 'dispatch;
            }
            0x824F7ED8 => {
    //   block [0x824F7ED8..0x824F7FDC)
	// 824F7ED8: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	pc = 0x824F7FDC; continue 'dispatch;
            }
            0x824F7FDC => {
    //   block [0x824F7FDC..0x824F8104)
	// 824F7FDC: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 824F7FE0: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 824F7FE4: 80E80010  lwz r7, 0x10(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 824F7FE8: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 824F7FEC: 409A007C  bne cr6, 0x824f8068
	if !ctx.cr[6].eq {
	pc = 0x824F8068; continue 'dispatch;
	}
	// 824F7FF0: 88FC0000  lbz r7, 0(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F7FF4: 98E1FEE0  stb r7, -0x120(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-288 as u32), ctx.r[7].u8 ) };
	// 824F7FF8: 88FC0001  lbz r7, 1(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(1 as u32) ) } as u64;
	// 824F7FFC: 98E1FEE1  stb r7, -0x11f(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(-287 as u32), ctx.r[7].u8 ) };
	// 824F8000: 80FC0004  lwz r7, 4(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F8004: 90E1FEE4  stw r7, -0x11c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-284 as u32), ctx.r[7].u32 ) };
	// 824F8008: 38E1FEF0  addi r7, r1, -0x110
	ctx.r[7].s64 = ctx.r[1].s64 + -272;
	pc = 0x824F8104; continue 'dispatch;
            }
            0x824F8104 => {
    //   block [0x824F8104..0x824F810C)
	// 824F8104: 394A0020  addi r10, r10, 0x20
	ctx.r[10].s64 = ctx.r[10].s64 + 32;
	// 824F8108: 4BFFF838  b 0x824f7940
	pc = 0x824F7940; continue 'dispatch;
            }
            0x824F810C => {
    //   block [0x824F810C..0x824F81D4)
	// 824F810C: 390A0010  addi r8, r10, 0x10
	ctx.r[8].s64 = ctx.r[10].s64 + 16;
	// 824F8110: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824F8114: 419A00B4  beq cr6, 0x824f81c8
	if ctx.cr[6].eq {
	pc = 0x824F81C8; continue 'dispatch;
	}
	// 824F8118: A0EA0000  lhz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F811C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F8120: 54E3303E  rotlwi r3, r7, 6
	ctx.r[3].u64 = ((ctx.r[7].u32).rotate_left(6)) as u64;
	// 824F8124: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 824F8128: 7CE35214  add r7, r3, r10
	ctx.r[7].u64 = ctx.r[3].u64 + ctx.r[10].u64;
	// 824F812C: 38E70014  addi r7, r7, 0x14
	ctx.r[7].s64 = ctx.r[7].s64 + 20;
	// 824F8130: 409900A4  ble cr6, 0x824f81d4
	if !ctx.cr[6].gt {
	pc = 0x824F81D4; continue 'dispatch;
	}
	// 824F8134: 80670004  lwz r3, 4(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	pc = 0x824F81D4; continue 'dispatch;
            }
            0x824F81D4 => {
    //   block [0x824F81D4..0x824F81E0)
	// 824F81D4: 810A0004  lwz r8, 4(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F81D8: 7D485214  add r10, r8, r10
	ctx.r[10].u64 = ctx.r[8].u64 + ctx.r[10].u64;
	// 824F81DC: 4BFFF764  b 0x824f7940
	pc = 0x824F7940; continue 'dispatch;
            }
            0x824F81E0 => {
    //   block [0x824F81E0..0x824F82C8)
	// 824F81E0: 390A0010  addi r8, r10, 0x10
	ctx.r[8].s64 = ctx.r[10].s64 + 16;
	// 824F81E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824F81E8: 419A00C0  beq cr6, 0x824f82a8
	if ctx.cr[6].eq {
	pc = 0x824F82A8; continue 'dispatch;
	}
	// 824F81EC: A0EA0000  lhz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F81F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F81F4: 1C670130  mulli r3, r7, 0x130
	ctx.r[3].s64 = ctx.r[7].s64 * 304;
	// 824F81F8: 54FF083E  rotlwi r31, r7, 1
	ctx.r[31].u64 = ((ctx.r[7].u32).rotate_left(1)) as u64;
	// 824F81FC: 7C635214  add r3, r3, r10
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[10].u64;
	// 824F8200: 7CA7FA15  add. r5, r7, r31
	ctx.r[5].u64 = ctx.r[7].u64 + ctx.r[31].u64;
	ctx.cr[0].compare_i32(ctx.r[5].s32, 0, &mut ctx.xer);
	// 824F8204: 38E30020  addi r7, r3, 0x20
	ctx.r[7].s64 = ctx.r[3].s64 + 32;
	// 824F8208: 408100B4  ble 0x824f82bc
	if !ctx.cr[0].gt {
	pc = 0x824F82BC; continue 'dispatch;
	}
	// 824F820C: 80670004  lwz r3, 4(r7)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	pc = 0x824F82C8; continue 'dispatch;
            }
            0x824F82C8 => {
    //   block [0x824F82C8..0x824F8584)
	// 824F82C8: A0EA0004  lhz r7, 4(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F82CC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 824F82D0: 3A6A0004  addi r19, r10, 4
	ctx.r[19].s64 = ctx.r[10].s64 + 4;
	// 824F82D4: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 824F82D8: 419A0038  beq cr6, 0x824f8310
	if ctx.cr[6].eq {
	pc = 0x824F8310; continue 'dispatch;
	}
	// 824F82DC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824F82E0: A0B30000  lhz r5, 0(r19)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F82E4: 806A0008  lwz r3, 8(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824F82E8: 1C8504F4  mulli r4, r5, 0x4f4
	ctx.r[4].s64 = ctx.r[5].s64 * 1268;
	// 824F82EC: 7C843A14  add r4, r4, r7
	ctx.r[4].u64 = ctx.r[4].u64 + ctx.r[7].u64;
	// 824F82F0: 38E70040  addi r7, r7, 0x40
	ctx.r[7].s64 = ctx.r[7].s64 + 64;
	// 824F82F4: 7CA45214  add r5, r4, r10
	ctx.r[5].u64 = ctx.r[4].u64 + ctx.r[10].u64;
	// 824F82F8: 88A50044  lbz r5, 0x44(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(68 as u32) ) } as u64;
	// 824F82FC: 7CA819AE  stbx r5, r8, r3
	unsafe { crate::rt::store_u8(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), ctx.r[5].u8) };
	// 824F8300: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 824F8304: A0B30000  lhz r5, 0(r19)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F8308: 7F082800  cmpw cr6, r8, r5
	ctx.cr[6].compare_i32(ctx.r[8].s32, ctx.r[5].s32, &mut ctx.xer);
	// 824F830C: 4198FFD4  blt cr6, 0x824f82e0
	if ctx.cr[6].lt {
	pc = 0x824F82E0; continue 'dispatch;
	}
	// 824F8310: A1130000  lhz r8, 0(r19)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[19].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F8314: 388A0020  addi r4, r10, 0x20
	ctx.r[4].s64 = ctx.r[10].s64 + 32;
	// 824F8318: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824F831C: 5507183E  rotlwi r7, r8, 3
	ctx.r[7].u64 = ((ctx.r[8].u32).rotate_left(3)) as u64;
	// 824F8320: 7CE83A14  add r7, r8, r7
	ctx.r[7].u64 = ctx.r[8].u64 + ctx.r[7].u64;
	// 824F8324: 54E72036  slwi r7, r7, 4
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824F8328: 7CE75214  add r7, r7, r10
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 824F832C: 38670020  addi r3, r7, 0x20
	ctx.r[3].s64 = ctx.r[7].s64 + 32;
	// 824F8330: 419A0238  beq cr6, 0x824f8568
	if ctx.cr[6].eq {
	pc = 0x824F8568; continue 'dispatch;
	}
	// 824F8334: 5507043E  clrlwi r7, r8, 0x10
	ctx.r[7].u64 = ctx.r[8].u32 as u64 & 0x0000FFFFu64;
	// 824F8338: 3A000000  li r16, 0
	ctx.r[16].s64 = 0;
	// 824F833C: 1CE704F0  mulli r7, r7, 0x4f0
	ctx.r[7].s64 = ctx.r[7].s64 * 1264;
	// 824F8340: 7CE75214  add r7, r7, r10
	ctx.r[7].u64 = ctx.r[7].u64 + ctx.r[10].u64;
	// 824F8344: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 824F8348: 38E70040  addi r7, r7, 0x40
	ctx.r[7].s64 = ctx.r[7].s64 + 64;
	// 824F834C: 4099022C  ble cr6, 0x824f8578
	if !ctx.cr[6].gt {
	pc = 0x824F8578; continue 'dispatch;
	}
	// 824F8350: 7CF13B78  mr r17, r7
	ctx.r[17].u64 = ctx.r[7].u64;
	// 824F8354: 5747083C  slwi r7, r26, 1
	ctx.r[7].u32 = ctx.r[26].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824F8358: 39040010  addi r8, r4, 0x10
	ctx.r[8].s64 = ctx.r[4].s64 + 16;
	// 824F835C: 7CFA3A14  add r7, r26, r7
	ctx.r[7].u64 = ctx.r[26].u64 + ctx.r[7].u64;
	// 824F8360: 3883000C  addi r4, r3, 0xc
	ctx.r[4].s64 = ctx.r[3].s64 + 12;
	// 824F8364: 54E7083C  slwi r7, r7, 1
	ctx.r[7].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 824F8368: 90E1FECC  stw r7, -0x134(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-308 as u32), ctx.r[7].u32 ) };
	// 824F836C: 80F10004  lwz r7, 4(r17)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F8370: 3B2B0004  addi r25, r11, 4
	ctx.r[25].s64 = ctx.r[11].s64 + 4;
	// 824F8374: 80710000  lwz r3, 0(r17)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[17].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F8378: 56183032  slwi r24, r16, 6
	ctx.r[24].u32 = ctx.r[16].u32.wrapping_shl(6);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 824F837C: 7FC73214  add r30, r7, r6
	ctx.r[30].u64 = ctx.r[7].u64 + ctx.r[6].u64;
	// 824F8380: 7C661A14  add r3, r6, r3
	ctx.r[3].u64 = ctx.r[6].u64 + ctx.r[3].u64;
	// 824F8384: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 824F8388: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 824F838C: 7D3F4B78  mr r31, r9
	ctx.r[31].u64 = ctx.r[9].u64;
	// 824F8390: 38EB001C  addi r7, r11, 0x1c
	ctx.r[7].s64 = ctx.r[11].s64 + 28;
	// 824F8394: 3AE30040  addi r23, r3, 0x40
	ctx.r[23].s64 = ctx.r[3].s64 + 64;
	// 824F8398: 3ADE0040  addi r22, r30, 0x40
	ctx.r[22].s64 = ctx.r[30].s64 + 64;
	// 824F839C: 3B9E0050  addi r28, r30, 0x50
	ctx.r[28].s64 = ctx.r[30].s64 + 80;
	// 824F83A0: 3B630050  addi r27, r3, 0x50
	ctx.r[27].s64 = ctx.r[3].s64 + 80;
	pc = 0x824F8584; continue 'dispatch;
            }
            0x824F8584 => {
    //   block [0x824F8584..0x824F8594)
	// 824F8584: 8161FE78  lwz r11, -0x188(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-392 as u32) ) } as u64;
	// 824F8588: 81410024  lwz r10, 0x24(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(36 as u32) ) } as u64;
	// 824F858C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824F8590: 4803CB40  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F8598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F8598 size=4
    let mut pc: u32 = 0x824F8598;
    'dispatch: loop {
        match pc {
            0x824F8598 => {
    //   block [0x824F8598..0x824F859C)
	// 824F8598: 4BFFF2E0  b 0x824f7878
	sub_824F7878(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F85A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824F85A0 size=844
    let mut pc: u32 = 0x824F85A0;
    'dispatch: loop {
        match pc {
            0x824F85A0 => {
    //   block [0x824F85A0..0x824F88EC)
	// 824F85A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F85A4: 4803CAF5  bl 0x82535098
	ctx.lr = 0x824F85A8;
	sub_82535080(ctx, base);
	// 824F85A8: 3961FF50  addi r11, r1, -0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + -176;
	// 824F85AC: C1A40040  lfs f13, 0x40(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824F85B0: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 824F85B4: 39460110  addi r10, r6, 0x110
	ctx.r[10].s64 = ctx.r[6].s64 + 272;
	// 824F85B8: 3B600010  li r27, 0x10
	ctx.r[27].s64 = 16;
	// 824F85BC: 3B800020  li r28, 0x20
	ctx.r[28].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F88F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F88F0 size=304
    let mut pc: u32 = 0x824F88F0;
    'dispatch: loop {
        match pc {
            0x824F88F0 => {
    //   block [0x824F88F0..0x824F8A20)
	// 824F88F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F88F4: 4803C7C9  bl 0x825350bc
	ctx.lr = 0x824F88F8;
	sub_82535080(ctx, base);
	// 824F88F8: 392300E0  addi r9, r3, 0xe0
	ctx.r[9].s64 = ctx.r[3].s64 + 224;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F8A20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F8A20 size=180
    let mut pc: u32 = 0x824F8A20;
    'dispatch: loop {
        match pc {
            0x824F8A20 => {
    //   block [0x824F8A20..0x824F8AD4)
	// 824F8A20: 3D20820A  lis r9, -0x7df6
	ctx.r[9].s64 = -2113273856;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F8AD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x824F8AD8 size=756
    let mut pc: u32 = 0x824F8AD8;
    'dispatch: loop {
        match pc {
            0x824F8AD8 => {
    //   block [0x824F8AD8..0x824F8DCC)
	// 824F8AD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F8ADC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824F8AE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824F8AE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824F8AE8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F8AEC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 824F8AF0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824F8AF4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 824F8AF8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 824F8AFC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824F8B00: 4BFFFF21  bl 0x824f8a20
	ctx.lr = 0x824F8B04;
	sub_824F8A20(ctx, base);
	// 824F8B04: C01F001C  lfs f0, 0x1c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824F8B08: C1A10060  lfs f13, 0x60(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824F8B0C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824F8B10: 40990020  ble cr6, 0x824f8b30
	if !ctx.cr[6].gt {
	pc = 0x824F8B30; continue 'dispatch;
	}
	// 824F8B14: C1BF0020  lfs f13, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824F8B18: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 824F8B1C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 824F8B20: 41980010  blt cr6, 0x824f8b30
	if ctx.cr[6].lt {
	pc = 0x824F8B30; continue 'dispatch;
	}
	// 824F8B24: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824F8B28: C02B1FF8  lfs f1, 0x1ff8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 824F8B2C: 48000288  b 0x824f8db4
	pc = 0x824F8DB4; continue 'dispatch;
	// 824F8B30: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 824F8B34: C1BF0014  lfs f13, 0x14(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 824F8B38: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 824F8B3C: C19F0020  lfs f12, 0x20(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 824F8B40: 39630140  addi r11, r3, 0x140
	ctx.r[11].s64 = ctx.r[3].s64 + 320;
	// 824F8B44: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 824F8B48: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F8DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824F8DD0 size=816
    let mut pc: u32 = 0x824F8DD0;
    'dispatch: loop {
        match pc {
            0x824F8DD0 => {
    //   block [0x824F8DD0..0x824F8E4C)
	// 824F8DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F8DD4: 4803C2D5  bl 0x825350a8
	ctx.lr = 0x824F8DD8;
	sub_82535080(ctx, base);
	// 824F8DD8: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F8DDC: 3BE00030  li r31, 0x30
	ctx.r[31].s64 = 48;
	// 824F8DE0: A1240006  lhz r9, 6(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 824F8DE4: 3F40820D  lis r26, -0x7df3
	ctx.r[26].s64 = -2113077248;
	// 824F8DE8: 556A383E  rotlwi r10, r11, 7
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(7)) as u64;
	// 824F8DEC: 552B383E  rotlwi r11, r9, 7
	ctx.r[11].u64 = ((ctx.r[9].u32).rotate_left(7)) as u64;
	// 824F8DF0: 7D0A2A14  add r8, r10, r5
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[5].u64;
	// 824F8DF4: 7FCB2A14  add r30, r11, r5
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 824F8DF8: 39640010  addi r11, r4, 0x10
	ctx.r[11].s64 = ctx.r[4].s64 + 16;
	// 824F8DFC: C01A1FF8  lfs f0, 0x1ff8(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824F8E00: 38A80010  addi r5, r8, 0x10
	ctx.r[5].s64 = ctx.r[8].s64 + 16;
	// 824F8E04: 38880020  addi r4, r8, 0x20
	ctx.r[4].s64 = ctx.r[8].s64 + 32;
	pc = 0x824F8E4C; continue 'dispatch;
            }
            0x824F8E4C => {
    //   block [0x824F8E4C..0x824F8ED8)
	// 824F8E4C: 890B0003  lbz r8, 3(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 824F8E50: 7D1A0774  extsb r26, r8
	ctx.r[26].s64 = ctx.r[8].s8 as i64;
	// 824F8E54: 2B1A0018  cmplwi cr6, r26, 0x18
	ctx.cr[6].compare_u32(ctx.r[26].u32, 24 as u32, &mut ctx.xer);
	// 824F8E58: 4199FFF4  bgt cr6, 0x824f8e4c
	if ctx.cr[6].gt {
	pc = 0x824F8E4C; continue 'dispatch;
	}
	// 824F8E5C: 3D808250  lis r12, -0x7db0
	ctx.r[12].s64 = -2108686336;
	// 824F8E60: 398C8E74  addi r12, r12, -0x718c
	ctx.r[12].s64 = ctx.r[12].s64 + -29068;
	// 824F8E64: 5740103A  slwi r0, r26, 2
	ctx.r[0].u32 = ctx.r[26].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 824F8E68: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 824F8E6C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 824F8E70: 4E800420  bctr
	match ctx.r[26].u64 {
		0 => {
	pc = 0x824F90FC; continue 'dispatch;
		},
		1 => {
	pc = 0x824F90FC; continue 'dispatch;
		},
		2 => {
	pc = 0x824F8EFC; continue 'dispatch;
		},
		3 => {
	pc = 0x824F8ED8; continue 'dispatch;
		},
		4 => {
	pc = 0x824F8E4C; continue 'dispatch;
		},
		5 => {
	pc = 0x824F9034; continue 'dispatch;
		},
		6 => {
	pc = 0x824F9098; continue 'dispatch;
		},
		7 => {
	pc = 0x824F8ED8; continue 'dispatch;
		},
		8 => {
	pc = 0x824F8ED8; continue 'dispatch;
		},
		9 => {
	pc = 0x824F8ED8; continue 'dispatch;
		},
		10 => {
	pc = 0x824F8E4C; continue 'dispatch;
		},
		11 => {
	pc = 0x824F8ED8; continue 'dispatch;
		},
		12 => {
	pc = 0x824F8ED8; continue 'dispatch;
		},
		13 => {
	pc = 0x824F8ED8; continue 'dispatch;
		},
		14 => {
	pc = 0x824F8ED8; continue 'dispatch;
		},
		15 => {
	pc = 0x824F8F08; continue 'dispatch;
		},
		16 => {
	pc = 0x824F8F08; continue 'dispatch;
		},
		17 => {
	pc = 0x824F8F08; continue 'dispatch;
		},
		18 => {
	pc = 0x824F8F08; continue 'dispatch;
		},
		19 => {
	pc = 0x824F8F74; continue 'dispatch;
		},
		20 => {
	pc = 0x824F8F74; continue 'dispatch;
		},
		21 => {
	pc = 0x824F90FC; continue 'dispatch;
		},
		22 => {
	pc = 0x824F90FC; continue 'dispatch;
		},
		23 => {
	pc = 0x824F8ED8; continue 'dispatch;
		},
		24 => {
	pc = 0x824F8ED8; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 824F8E74: 824F90FC  lwz r18, -0x6f04(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28420 as u32) ) } as u64;
	// 824F8E78: 824F90FC  lwz r18, -0x6f04(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28420 as u32) ) } as u64;
	// 824F8E7C: 824F8EFC  lwz r18, -0x7104(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28932 as u32) ) } as u64;
	// 824F8E80: 824F8ED8  lwz r18, -0x7128(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28968 as u32) ) } as u64;
	// 824F8E84: 824F8E4C  lwz r18, -0x71b4(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-29108 as u32) ) } as u64;
	// 824F8E88: 824F9034  lwz r18, -0x6fcc(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28620 as u32) ) } as u64;
	// 824F8E8C: 824F9098  lwz r18, -0x6f68(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28520 as u32) ) } as u64;
	// 824F8E90: 824F8ED8  lwz r18, -0x7128(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28968 as u32) ) } as u64;
	// 824F8E94: 824F8ED8  lwz r18, -0x7128(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28968 as u32) ) } as u64;
	// 824F8E98: 824F8ED8  lwz r18, -0x7128(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28968 as u32) ) } as u64;
	// 824F8E9C: 824F8E4C  lwz r18, -0x71b4(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-29108 as u32) ) } as u64;
	// 824F8EA0: 824F8ED8  lwz r18, -0x7128(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28968 as u32) ) } as u64;
	// 824F8EA4: 824F8ED8  lwz r18, -0x7128(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28968 as u32) ) } as u64;
	// 824F8EA8: 824F8ED8  lwz r18, -0x7128(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28968 as u32) ) } as u64;
	// 824F8EAC: 824F8ED8  lwz r18, -0x7128(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28968 as u32) ) } as u64;
	// 824F8EB0: 824F8F08  lwz r18, -0x70f8(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28920 as u32) ) } as u64;
	// 824F8EB4: 824F8F08  lwz r18, -0x70f8(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28920 as u32) ) } as u64;
	// 824F8EB8: 824F8F08  lwz r18, -0x70f8(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28920 as u32) ) } as u64;
	// 824F8EBC: 824F8F08  lwz r18, -0x70f8(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28920 as u32) ) } as u64;
	// 824F8EC0: 824F8F74  lwz r18, -0x708c(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28812 as u32) ) } as u64;
	// 824F8EC4: 824F8F74  lwz r18, -0x708c(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28812 as u32) ) } as u64;
	// 824F8EC8: 824F90FC  lwz r18, -0x6f04(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28420 as u32) ) } as u64;
	// 824F8ECC: 824F90FC  lwz r18, -0x6f04(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28420 as u32) ) } as u64;
	// 824F8ED0: 824F8ED8  lwz r18, -0x7128(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28968 as u32) ) } as u64;
	// 824F8ED4: 824F8ED8  lwz r18, -0x7128(r15)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[15].u32.wrapping_add(-28968 as u32) ) } as u64;
            }
            0x824F8ED8 => {
    //   block [0x824F8ED8..0x824F8EFC)
	// 824F8ED8: 7F5A0774  extsb r26, r26
	ctx.r[26].s64 = ctx.r[26].s8 as i64;
	// 824F8EDC: 7D080774  extsb r8, r8
	ctx.r[8].s64 = ctx.r[8].s8 as i64;
	// 824F8EE0: 7D08C8AE  lbzx r8, r8, r25
	ctx.r[8].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 824F8EE4: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 824F8EE8: 890B0003  lbz r8, 3(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(3 as u32) ) } as u64;
	// 824F8EEC: 7D180774  extsb r24, r8
	ctx.r[24].s64 = ctx.r[8].s8 as i64;
	// 824F8EF0: 7F18D000  cmpw cr6, r24, r26
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[26].s32, &mut ctx.xer);
	// 824F8EF4: 419AFFE8  beq cr6, 0x824f8edc
	if ctx.cr[6].eq {
	pc = 0x824F8EDC; continue 'dispatch;
	}
	// 824F8EF8: 4BFFFF54  b 0x824f8e4c
	pc = 0x824F8E4C; continue 'dispatch;
            }
            0x824F8EFC => {
    //   block [0x824F8EFC..0x824F8F08)
	// 824F8EFC: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824F8F00: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 824F8F04: 4BFFFF48  b 0x824f8e4c
	pc = 0x824F8E4C; continue 'dispatch;
            }
            0x824F8F08 => {
    //   block [0x824F8F08..0x824F8F74)
	// 824F8F08: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 824F8F0C: 419901F0  bgt cr6, 0x824f90fc
	if ctx.cr[6].gt {
	pc = 0x824F90FC; continue 'dispatch;
	}
	pc = 0x824F8F74; continue 'dispatch;
            }
            0x824F8F74 => {
    //   block [0x824F8F74..0x824F9034)
	// 824F8F74: 7F093000  cmpw cr6, r9, r6
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[6].s32, &mut ctx.xer);
	// 824F8F78: 41990184  bgt cr6, 0x824f90fc
	if ctx.cr[6].gt {
	pc = 0x824F90FC; continue 'dispatch;
	}
	pc = 0x824F9034; continue 'dispatch;
            }
            0x824F9034 => {
    //   block [0x824F9034..0x824F9098)
	// 824F9034: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 824F9038: 419900C4  bgt cr6, 0x824f90fc
	if ctx.cr[6].gt {
	pc = 0x824F90FC; continue 'dispatch;
	}
	pc = 0x824F9098; continue 'dispatch;
            }
            0x824F9098 => {
    //   block [0x824F9098..0x824F90FC)
	// 824F9098: 7F0A3000  cmpw cr6, r10, r6
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[6].s32, &mut ctx.xer);
	// 824F909C: 41990060  bgt cr6, 0x824f90fc
	if ctx.cr[6].gt {
	pc = 0x824F90FC; continue 'dispatch;
	}
	pc = 0x824F90FC; continue 'dispatch;
            }
            0x824F90FC => {
    //   block [0x824F90FC..0x824F9100)
	// 824F90FC: 4803BFFC  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9100 size=20
    let mut pc: u32 = 0x824F9100;
    'dispatch: loop {
        match pc {
            0x824F9100 => {
    //   block [0x824F9100..0x824F9114)
	// 824F9100: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9104: 7C8A0774  extsb r10, r4
	ctx.r[10].s64 = ctx.r[4].s8 as i64;
	// 824F9108: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824F910C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824F9110: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9114(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9114 size=32
    let mut pc: u32 = 0x824F9114;
    'dispatch: loop {
        match pc {
            0x824F9114 => {
    //   block [0x824F9114..0x824F9134)
	// 824F9114: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824F9118: 419A001C  beq cr6, 0x824f9134
	if ctx.cr[6].eq {
		sub_824F9134(ctx, base);
		return;
	}
	// 824F911C: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 824F9120: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9124: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824F9128: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 824F912C: 409AFFE8  bne cr6, 0x824f9114
	if !ctx.cr[6].eq {
	pc = 0x824F9114; continue 'dispatch;
	}
	// 824F9130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9134(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9134 size=8
    let mut pc: u32 = 0x824F9134;
    'dispatch: loop {
        match pc {
            0x824F9134 => {
    //   block [0x824F9134..0x824F913C)
	// 824F9134: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824F9138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824F9140 size=892
    let mut pc: u32 = 0x824F9140;
    'dispatch: loop {
        match pc {
            0x824F9140 => {
    //   block [0x824F9140..0x824F94BC)
	// 824F9140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824F9144: 4803BF4D  bl 0x82535090
	ctx.lr = 0x824F9148;
	sub_82535080(ctx, base);
	// 824F9148: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824F914C: 3F80820A  lis r28, -0x7df6
	ctx.r[28].s64 = -2113273856;
	// 824F9150: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9154: 3A600000  li r19, 0
	ctx.r[19].s64 = 0;
	// 824F9158: 3AEB26C0  addi r23, r11, 0x26c0
	ctx.r[23].s64 = ctx.r[11].s64 + 9920;
	// 824F915C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 824F9160: 817CAFFC  lwz r11, -0x5004(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-20484 as u32) ) } as u64;
	// 824F9164: 7E749B78  mr r20, r19
	ctx.r[20].u64 = ctx.r[19].u64;
	// 824F9168: 7EFBBB78  mr r27, r23
	ctx.r[27].u64 = ctx.r[23].u64;
	// 824F916C: 55720FFE  srwi r18, r11, 0x1f
	ctx.r[18].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[18].u64 = ctx.r[18].u32 as u64;
	// 824F9170: 3D608282  lis r11, -0x7d7e
	ctx.r[11].s64 = -2105409536;
	// 824F9174: 3B0BC390  addi r24, r11, -0x3c70
	ctx.r[24].s64 = ctx.r[11].s64 + -15472;
	// 824F9178: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F917C: 3BCB287C  addi r30, r11, 0x287c
	ctx.r[30].s64 = ctx.r[11].s64 + 10364;
	// 824F9180: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9184: 3AAB2874  addi r21, r11, 0x2874
	ctx.r[21].s64 = ctx.r[11].s64 + 10356;
	// 824F9188: 3D60820A  lis r11, -0x7df6
	ctx.r[11].s64 = -2113273856;
	// 824F918C: 3B4BB088  addi r26, r11, -0x4f78
	ctx.r[26].s64 = ctx.r[11].s64 + -20344;
	// 824F9190: 3D600209  lis r11, 0x209
	ctx.r[11].s64 = 34144256;
	// 824F9194: 6176A5AD  ori r22, r11, 0xa5ad
	ctx.r[22].u64 = ctx.r[11].u64 | 42413;
	// 824F9198: 897A0000  lbz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F919C: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 824F91A0: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F91A4: 2B0B002E  cmplwi cr6, r11, 0x2e
	ctx.cr[6].compare_u32(ctx.r[11].u32, 46 as u32, &mut ctx.xer);
	// 824F91A8: 419A0024  beq cr6, 0x824f91cc
	if ctx.cr[6].eq {
	pc = 0x824F91CC; continue 'dispatch;
	}
	// 824F91AC: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824F91B0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824F91B4: 419A029C  beq cr6, 0x824f9450
	if ctx.cr[6].eq {
	pc = 0x824F9450; continue 'dispatch;
	}
	// 824F91B8: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824F91BC: 897F0000  lbz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F91C0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824F91C4: 2F0B002E  cmpwi cr6, r11, 0x2e
	ctx.cr[6].compare_i32(ctx.r[11].s32, 46, &mut ctx.xer);
	// 824F91C8: 409AFFE8  bne cr6, 0x824f91b0
	if !ctx.cr[6].eq {
	pc = 0x824F91B0; continue 'dispatch;
	}
	// 824F91CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824F91D0: 7EA4AB78  mr r4, r21
	ctx.r[4].u64 = ctx.r[21].u64;
	// 824F91D4: 4803B4E5  bl 0x825346b8
	ctx.lr = 0x824F91D8;
	sub_825346B8(ctx, base);
	// 824F91D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F91DC: 419A0184  beq cr6, 0x824f9360
	if ctx.cr[6].eq {
	pc = 0x824F9360; continue 'dispatch;
	}
	// 824F91E0: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 824F91E4: 419A000C  beq cr6, 0x824f91f0
	if ctx.cr[6].eq {
	pc = 0x824F91F0; continue 'dispatch;
	}
	// 824F91E8: 2F190002  cmpwi cr6, r25, 2
	ctx.cr[6].compare_i32(ctx.r[25].s32, 2, &mut ctx.xer);
	// 824F91EC: 409A0174  bne cr6, 0x824f9360
	if !ctx.cr[6].eq {
	pc = 0x824F9360; continue 'dispatch;
	}
	// 824F91F0: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 824F91F4: 9A610050  stb r19, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[19].u8 ) };
	// 824F91F8: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 824F91FC: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9200: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824F9204: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824F9208: 409AFFF4  bne cr6, 0x824f91fc
	if !ctx.cr[6].eq {
	pc = 0x824F91FC; continue 'dispatch;
	}
	// 824F920C: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 824F9210: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824F9214: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824F9218: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 824F921C: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824F9220: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9224: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824F9228: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824F922C: 409AFFF4  bne cr6, 0x824f9220
	if !ctx.cr[6].eq {
	pc = 0x824F9220; continue 'dispatch;
	}
	// 824F9230: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 824F9234: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824F9238: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824F923C: 216B00FE  subfic r11, r11, 0xfe
	ctx.xer.ca = ctx.r[11].u32 <= 254 as u32;
	ctx.r[11].s64 = (254 as i64) - ctx.r[11].s64;
	// 824F9240: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824F9244: 4098002C  bge cr6, 0x824f9270
	if !ctx.cr[6].lt {
	pc = 0x824F9270; continue 'dispatch;
	}
	// 824F9248: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 824F924C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824F9250: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9254: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824F9258: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824F925C: 409AFFF4  bne cr6, 0x824f9250
	if !ctx.cr[6].eq {
	pc = 0x824F9250; continue 'dispatch;
	}
	// 824F9260: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 824F9264: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824F9268: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824F926C: 4800002C  b 0x824f9298
	pc = 0x824F9298; continue 'dispatch;
	// 824F9270: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824F9274: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824F9278: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F927C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824F9280: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824F9284: 409AFFF4  bne cr6, 0x824f9278
	if !ctx.cr[6].eq {
	pc = 0x824F9278; continue 'dispatch;
	}
	// 824F9288: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 824F928C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824F9290: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824F9294: 20AB00FE  subfic r5, r11, 0xfe
	ctx.xer.ca = ctx.r[11].u32 <= 254 as u32;
	ctx.r[5].s64 = (254 as i64) - ctx.r[11].s64;
	// 824F9298: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824F929C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824F92A0: 4803B499  bl 0x82534738
	ctx.lr = 0x824F92A4;
	sub_82534738(ctx, base);
	// 824F92A4: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 824F92A8: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 824F92AC: 896A0000  lbz r11, 0(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F92B0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824F92B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824F92B8: 409AFFF4  bne cr6, 0x824f92ac
	if !ctx.cr[6].eq {
	pc = 0x824F92AC; continue 'dispatch;
	}
	// 824F92BC: 7D495050  subf r10, r9, r10
	ctx.r[10].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 824F92C0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824F92C4: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824F92C8: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 824F92CC: 554A003E  slwi r10, r10, 0
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824F92D0: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F92D4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824F92D8: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 824F92DC: 409AFFF4  bne cr6, 0x824f92d0
	if !ctx.cr[6].eq {
	pc = 0x824F92D0; continue 'dispatch;
	}
	// 824F92E0: 7D695850  subf r11, r9, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[9].s64;
	// 824F92E4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824F92E8: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824F92EC: 216B00FE  subfic r11, r11, 0xfe
	ctx.xer.ca = ctx.r[11].u32 <= 254 as u32;
	ctx.r[11].s64 = (254 as i64) - ctx.r[11].s64;
	// 824F92F0: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824F92F4: 4098002C  bge cr6, 0x824f9320
	if !ctx.cr[6].lt {
	pc = 0x824F9320; continue 'dispatch;
	}
	// 824F92F8: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 824F92FC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824F9300: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9304: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824F9308: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824F930C: 409AFFF4  bne cr6, 0x824f9300
	if !ctx.cr[6].eq {
	pc = 0x824F9300; continue 'dispatch;
	}
	// 824F9310: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 824F9314: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824F9318: 5565003E  slwi r5, r11, 0
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 824F931C: 4800002C  b 0x824f9348
	pc = 0x824F9348; continue 'dispatch;
	// 824F9320: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 824F9324: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 824F9328: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F932C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824F9330: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824F9334: 409AFFF4  bne cr6, 0x824f9328
	if !ctx.cr[6].eq {
	pc = 0x824F9328; continue 'dispatch;
	}
	// 824F9338: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 824F933C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 824F9340: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824F9344: 20AB00FE  subfic r5, r11, 0xfe
	ctx.xer.ca = ctx.r[11].u32 <= 254 as u32;
	ctx.r[5].s64 = (254 as i64) - ctx.r[11].s64;
	// 824F9348: 572B103A  slwi r11, r25, 2
	ctx.r[11].u32 = ctx.r[25].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824F934C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824F9350: 7C8BC02E  lwzx r4, r11, r24
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 824F9354: 4803B3E5  bl 0x82534738
	ctx.lr = 0x824F9358;
	sub_82534738(ctx, base);
	// 824F9358: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824F935C: 4BF6FC35  bl 0x82468f90
	ctx.lr = 0x824F9360;
	sub_82468F90(ctx, base);
	// 824F9360: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 824F9364: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9368: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 824F936C: 2F0A002E  cmpwi cr6, r10, 0x2e
	ctx.cr[6].compare_i32(ctx.r[10].s32, 46, &mut ctx.xer);
	// 824F9370: 419A0020  beq cr6, 0x824f9390
	if ctx.cr[6].eq {
	pc = 0x824F9390; continue 'dispatch;
	}
	// 824F9374: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824F9378: 419A00D8  beq cr6, 0x824f9450
	if ctx.cr[6].eq {
	pc = 0x824F9450; continue 'dispatch;
	}
	// 824F937C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824F9380: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9384: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 824F9388: 2F0A002E  cmpwi cr6, r10, 0x2e
	ctx.cr[6].compare_i32(ctx.r[10].s32, 46, &mut ctx.xer);
	// 824F938C: 409AFFE8  bne cr6, 0x824f9374
	if !ctx.cr[6].eq {
	pc = 0x824F9374; continue 'dispatch;
	}
	// 824F9390: 564A063E  clrlwi r10, r18, 0x18
	ctx.r[10].u64 = ctx.r[18].u32 as u64 & 0x000000FFu64;
	// 824F9394: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824F9398: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824F939C: 419A0060  beq cr6, 0x824f93fc
	if ctx.cr[6].eq {
	pc = 0x824F93FC; continue 'dispatch;
	}
	// 824F93A0: 817CAFFC  lwz r11, -0x5004(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-20484 as u32) ) } as u64;
	// 824F93A4: 556B007E  clrlwi r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 824F93A8: 7D7FEA78  xor r31, r11, r29
	ctx.r[31].u64 = ctx.r[11].u64 ^ ctx.r[29].u64;
	// 824F93AC: 4800013D  bl 0x824f94e8
	ctx.lr = 0x824F93B0;
	sub_824F94E8(ctx, base);
	// 824F93B0: 7FEB07B4  extsw r11, r31
	ctx.r[11].s64 = ctx.r[31].s32 as i64;
	// 824F93B4: 7F2B1800  cmpd cr6, r11, r3
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[3].s64, &mut ctx.xer);
	// 824F93B8: 40990084  ble cr6, 0x824f943c
	if !ctx.cr[6].gt {
	pc = 0x824F943C; continue 'dispatch;
	}
	// 824F93BC: 7D635850  subf r11, r3, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[3].s64;
	// 824F93C0: 7F2BB000  cmpd cr6, r11, r22
	ctx.cr[6].compare_i64(ctx.r[11].s64, ctx.r[22].s64, &mut ctx.xer);
	// 824F93C4: 40980078  bge cr6, 0x824f943c
	if !ctx.cr[6].lt {
	pc = 0x824F943C; continue 'dispatch;
	}
	// 824F93C8: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 824F93CC: 419A000C  beq cr6, 0x824f93d8
	if ctx.cr[6].eq {
	pc = 0x824F93D8; continue 'dispatch;
	}
	// 824F93D0: 2F190002  cmpwi cr6, r25, 2
	ctx.cr[6].compare_i32(ctx.r[25].s32, 2, &mut ctx.xer);
	// 824F93D4: 409A0094  bne cr6, 0x824f9468
	if !ctx.cr[6].eq {
	pc = 0x824F9468; continue 'dispatch;
	}
	// 824F93D8: 3974FFFA  addi r11, r20, -6
	ctx.r[11].s64 = ctx.r[20].s64 + -6;
	// 824F93DC: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 824F93E0: 41990088  bgt cr6, 0x824f9468
	if ctx.cr[6].gt {
	pc = 0x824F9468; continue 'dispatch;
	}
	// 824F93E4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F93E8: 386B27F0  addi r3, r11, 0x27f0
	ctx.r[3].s64 = ctx.r[11].s64 + 10224;
	// 824F93EC: 4BF6FBA5  bl 0x82468f90
	ctx.lr = 0x824F93F0;
	sub_82468F90(ctx, base);
	// 824F93F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824F93F4: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 824F93F8: 4803BCE8  b 0x825350e0
	sub_825350D0(ctx, base);
	return;
	// 824F93FC: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9400: 7E699B78  mr r9, r19
	ctx.r[9].u64 = ctx.r[19].u64;
	// 824F9404: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 824F9408: 419A0020  beq cr6, 0x824f9428
	if ctx.cr[6].eq {
	pc = 0x824F9428; continue 'dispatch;
	}
	// 824F940C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824F9410: 1D290017  mulli r9, r9, 0x17
	ctx.r[9].s64 = ctx.r[9].s64 * 23;
	// 824F9414: 894B0000  lbz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9418: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 824F941C: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 824F9420: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824F9424: 409AFFE8  bne cr6, 0x824f940c
	if !ctx.cr[6].eq {
	pc = 0x824F940C; continue 'dispatch;
	}
	// 824F9428: 7D2BEA78  xor r11, r9, r29
	ctx.r[11].u64 = ctx.r[9].u64 ^ ctx.r[29].u64;
	// 824F942C: 556A007E  clrlwi r10, r11, 1
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 824F9430: 817CAFFC  lwz r11, -0x5004(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-20484 as u32) ) } as u64;
	// 824F9434: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 824F9438: 419A0064  beq cr6, 0x824f949c
	if ctx.cr[6].eq {
	pc = 0x824F949C; continue 'dispatch;
	}
	// 824F943C: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 824F9440: 39770030  addi r11, r23, 0x30
	ctx.r[11].s64 = ctx.r[23].s64 + 48;
	// 824F9444: 3A940001  addi r20, r20, 1
	ctx.r[20].s64 = ctx.r[20].s64 + 1;
	// 824F9448: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824F944C: 4198FD4C  blt cr6, 0x824f9198
	if ctx.cr[6].lt {
	pc = 0x824F9198; continue 'dispatch;
	}
	// 824F9450: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F9454: 386B2770  addi r3, r11, 0x2770
	ctx.r[3].s64 = ctx.r[11].s64 + 10096;
	// 824F9458: 4BF6FB39  bl 0x82468f90
	ctx.lr = 0x824F945C;
	sub_82468F90(ctx, base);
	// 824F945C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824F9460: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 824F9464: 4803BC7C  b 0x825350e0
	sub_825350D0(ctx, base);
	return;
	// 824F9468: 2F190004  cmpwi cr6, r25, 4
	ctx.cr[6].compare_i32(ctx.r[25].s32, 4, &mut ctx.xer);
	// 824F946C: 419A0024  beq cr6, 0x824f9490
	if ctx.cr[6].eq {
	pc = 0x824F9490; continue 'dispatch;
	}
	// 824F9470: 2F14000A  cmpwi cr6, r20, 0xa
	ctx.cr[6].compare_i32(ctx.r[20].s32, 10, &mut ctx.xer);
	// 824F9474: 4198001C  blt cr6, 0x824f9490
	if ctx.cr[6].lt {
	pc = 0x824F9490; continue 'dispatch;
	}
	// 824F9478: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 824F947C: 386B26F0  addi r3, r11, 0x26f0
	ctx.r[3].s64 = ctx.r[11].s64 + 9968;
	// 824F9480: 4BF6FB11  bl 0x82468f90
	ctx.lr = 0x824F9484;
	sub_82468F90(ctx, base);
	// 824F9484: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824F9488: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 824F948C: 4803BC54  b 0x825350e0
	sub_825350D0(ctx, base);
	return;
	// 824F9490: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824F9494: 382101D0  addi r1, r1, 0x1d0
	ctx.r[1].s64 = ctx.r[1].s64 + 464;
	// 824F9498: 4803BC48  b 0x825350e0
	sub_825350D0(ctx, base);
	return;
	// 824F949C: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 824F94A0: 419A000C  beq cr6, 0x824f94ac
	if ctx.cr[6].eq {
	pc = 0x824F94AC; continue 'dispatch;
	}
	// 824F94A4: 2F190002  cmpwi cr6, r25, 2
	ctx.cr[6].compare_i32(ctx.r[25].s32, 2, &mut ctx.xer);
	// 824F94A8: 409AFFC0  bne cr6, 0x824f9468
	if !ctx.cr[6].eq {
	pc = 0x824F9468; continue 'dispatch;
	}
	// 824F94AC: 3974FFFA  addi r11, r20, -6
	ctx.r[11].s64 = ctx.r[20].s64 + -6;
	// 824F94B0: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 824F94B4: 4099FF30  ble cr6, 0x824f93e4
	if !ctx.cr[6].gt {
	pc = 0x824F93E4; continue 'dispatch;
	}
	// 824F94B8: 4BFFFFB0  b 0x824f9468
	pc = 0x824F9468; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F94C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F94C0 size=20
    let mut pc: u32 = 0x824F94C0;
    'dispatch: loop {
        match pc {
            0x824F94C0 => {
    //   block [0x824F94C0..0x824F94D4)
	// 824F94C0: 3D60829A  lis r11, -0x7d66
	ctx.r[11].s64 = -2103836672;
	// 824F94C4: 814B15FC  lwz r10, 0x15fc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5628 as u32) ) } as u64;
	// 824F94C8: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824F94CC: 914B15FC  stw r10, 0x15fc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(5628 as u32), ctx.r[10].u32 ) };
	// 824F94D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F94D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F94D8 size=4
    let mut pc: u32 = 0x824F94D8;
    'dispatch: loop {
        match pc {
            0x824F94D8 => {
    //   block [0x824F94D8..0x824F94DC)
	// 824F94D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F94E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F94E0 size=4
    let mut pc: u32 = 0x824F94E0;
    'dispatch: loop {
        match pc {
            0x824F94E0 => {
    //   block [0x824F94E0..0x824F94E4)
	// 824F94E0: 48040180  b 0x82539660
	sub_82539660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F94E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F94E8 size=8
    let mut pc: u32 = 0x824F94E8;
    'dispatch: loop {
        match pc {
            0x824F94E8 => {
    //   block [0x824F94E8..0x824F94F0)
	// 824F94E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824F94EC: 48040174  b 0x82539660
	sub_82539660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F94F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F94F0 size=20
    let mut pc: u32 = 0x824F94F0;
    'dispatch: loop {
        match pc {
            0x824F94F0 => {
    //   block [0x824F94F0..0x824F9504)
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
    // ---- function 0x824F9508 size=8
    let mut pc: u32 = 0x824F9508;
    'dispatch: loop {
        match pc {
            0x824F9508 => {
    //   block [0x824F9508..0x824F9510)
	// 824F9508: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F950C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9510 size=32
    let mut pc: u32 = 0x824F9510;
    'dispatch: loop {
        match pc {
            0x824F9510 => {
    //   block [0x824F9510..0x824F9530)
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
    // ---- function 0x824F9560 size=12
    let mut pc: u32 = 0x824F9560;
    'dispatch: loop {
        match pc {
            0x824F9560 => {
    //   block [0x824F9560..0x824F956C)
	// 824F9560: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824F9564: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9568: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F956C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F956C size=8
    let mut pc: u32 = 0x824F956C;
    'dispatch: loop {
        match pc {
            0x824F956C => {
    //   block [0x824F956C..0x824F9574)
	// 824F956C: 4800003C  b 0x824f95a8
	sub_824F95A8(ctx, base);
	return;
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
    //   block [0x824F9620..0x824F9684)
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
    //   block [0x824F96A8..0x824F96F0)
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
    //   block [0x824F96F0..0x824F976C)
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
    // ---- function 0x824F97B8 size=8
    let mut pc: u32 = 0x824F97B8;
    'dispatch: loop {
        match pc {
            0x824F97B8 => {
    //   block [0x824F97B8..0x824F97C0)
	// 824F97B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F97BC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F97C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F97C0 size=44
    let mut pc: u32 = 0x824F97C0;
    'dispatch: loop {
        match pc {
            0x824F97C0 => {
    //   block [0x824F97C0..0x824F97EC)
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
    // ---- function 0x824F9818 size=12
    let mut pc: u32 = 0x824F9818;
    'dispatch: loop {
        match pc {
            0x824F9818 => {
    //   block [0x824F9818..0x824F9824)
	// 824F9818: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824F981C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9820: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9824(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9824 size=8
    let mut pc: u32 = 0x824F9824;
    'dispatch: loop {
        match pc {
            0x824F9824 => {
    //   block [0x824F9824..0x824F982C)
	// 824F9824: 4800003C  b 0x824f9860
	sub_824F9860(ctx, base);
	return;
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
    //   block [0x824F98D8..0x824F993C)
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
    // ---- function 0x824F9970 size=12
    let mut pc: u32 = 0x824F9970;
    'dispatch: loop {
        match pc {
            0x824F9970 => {
    //   block [0x824F9970..0x824F997C)
	// 824F9970: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824F9974: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9978: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F997C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F997C size=8
    let mut pc: u32 = 0x824F997C;
    'dispatch: loop {
        match pc {
            0x824F997C => {
    //   block [0x824F997C..0x824F9984)
	// 824F997C: 4800B664  b 0x82504fe0
	sub_82504FE0(ctx, base);
	return;
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
    // ---- function 0x824F9A08 size=8
    let mut pc: u32 = 0x824F9A08;
    'dispatch: loop {
        match pc {
            0x824F9A08 => {
    //   block [0x824F9A08..0x824F9A10)
	// 824F9A08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9A0C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9A10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9A10 size=16
    let mut pc: u32 = 0x824F9A10;
    'dispatch: loop {
        match pc {
            0x824F9A10 => {
    //   block [0x824F9A10..0x824F9A20)
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
    // ---- function 0x824F9A48 size=8
    let mut pc: u32 = 0x824F9A48;
    'dispatch: loop {
        match pc {
            0x824F9A48 => {
    //   block [0x824F9A48..0x824F9A50)
	// 824F9A48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9A4C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9A50 size=80
    let mut pc: u32 = 0x824F9A50;
    'dispatch: loop {
        match pc {
            0x824F9A50 => {
    //   block [0x824F9A50..0x824F9AA0)
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
    //   block [0x824F9AB0..0x824F9B2C)
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
    //   block [0x824F9B30..0x824F9B94)
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
    //   block [0x824F9BB0..0x824F9C14)
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


pub fn sub_824F9C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9C18 size=12
    let mut pc: u32 = 0x824F9C18;
    'dispatch: loop {
        match pc {
            0x824F9C18 => {
    //   block [0x824F9C18..0x824F9C24)
	// 824F9C18: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824F9C1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9C20: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9C24(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9C24 size=8
    let mut pc: u32 = 0x824F9C24;
    'dispatch: loop {
        match pc {
            0x824F9C24 => {
    //   block [0x824F9C24..0x824F9C2C)
	// 824F9C24: 4800C954  b 0x82506578
	sub_82506578(ctx, base);
	return;
	// 824F9C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9C30 size=20
    let mut pc: u32 = 0x824F9C30;
    'dispatch: loop {
        match pc {
            0x824F9C30 => {
    //   block [0x824F9C30..0x824F9C44)
	// 824F9C30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9C34: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9C38: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9C3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9C40: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
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


pub fn sub_824F9C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9C78 size=12
    let mut pc: u32 = 0x824F9C78;
    'dispatch: loop {
        match pc {
            0x824F9C78 => {
    //   block [0x824F9C78..0x824F9C84)
	// 824F9C78: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824F9C7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9C80: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9C84(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9C84 size=8
    let mut pc: u32 = 0x824F9C84;
    'dispatch: loop {
        match pc {
            0x824F9C84 => {
    //   block [0x824F9C84..0x824F9C8C)
	// 824F9C84: 4800C924  b 0x825065a8
	sub_825065A8(ctx, base);
	return;
	// 824F9C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9C90 size=20
    let mut pc: u32 = 0x824F9C90;
    'dispatch: loop {
        match pc {
            0x824F9C90 => {
    //   block [0x824F9C90..0x824F9CA4)
	// 824F9C90: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9C94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9C98: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9C9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9CA0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
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


pub fn sub_824F9CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9CE0 size=20
    let mut pc: u32 = 0x824F9CE0;
    'dispatch: loop {
        match pc {
            0x824F9CE0 => {
    //   block [0x824F9CE0..0x824F9CF4)
	// 824F9CE0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9CE4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9CE8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9CEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9CF0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9CF8 size=12
    let mut pc: u32 = 0x824F9CF8;
    'dispatch: loop {
        match pc {
            0x824F9CF8 => {
    //   block [0x824F9CF8..0x824F9D04)
	// 824F9CF8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824F9CFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9D00: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9D04(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9D04 size=8
    let mut pc: u32 = 0x824F9D04;
    'dispatch: loop {
        match pc {
            0x824F9D04 => {
    //   block [0x824F9D04..0x824F9D0C)
	// 824F9D04: 4800D6A4  b 0x825073a8
	sub_825073A8(ctx, base);
	return;
	// 824F9D08: 4E800020  blr
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


pub fn sub_824F9D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9D48 size=12
    let mut pc: u32 = 0x824F9D48;
    'dispatch: loop {
        match pc {
            0x824F9D48 => {
    //   block [0x824F9D48..0x824F9D54)
	// 824F9D48: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824F9D4C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9D50: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9D54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9D54 size=8
    let mut pc: u32 = 0x824F9D54;
    'dispatch: loop {
        match pc {
            0x824F9D54 => {
    //   block [0x824F9D54..0x824F9D5C)
	// 824F9D54: 4800EC6C  b 0x825089c0
	sub_825089C0(ctx, base);
	return;
	// 824F9D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9D60 size=20
    let mut pc: u32 = 0x824F9D60;
    'dispatch: loop {
        match pc {
            0x824F9D60 => {
    //   block [0x824F9D60..0x824F9D74)
	// 824F9D60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9D64: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9D68: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9D6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9D70: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
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


pub fn sub_824F9DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9DA8 size=12
    let mut pc: u32 = 0x824F9DA8;
    'dispatch: loop {
        match pc {
            0x824F9DA8 => {
    //   block [0x824F9DA8..0x824F9DB4)
	// 824F9DA8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824F9DAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9DB0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9DB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9DB4 size=8
    let mut pc: u32 = 0x824F9DB4;
    'dispatch: loop {
        match pc {
            0x824F9DB4 => {
    //   block [0x824F9DB4..0x824F9DBC)
	// 824F9DB4: 4800E98C  b 0x82508740
	sub_82508740(ctx, base);
	return;
	// 824F9DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9DC0 size=20
    let mut pc: u32 = 0x824F9DC0;
    'dispatch: loop {
        match pc {
            0x824F9DC0 => {
    //   block [0x824F9DC0..0x824F9DD4)
	// 824F9DC0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9DC4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9DC8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9DCC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9DD0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
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


pub fn sub_824F9E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9E08 size=12
    let mut pc: u32 = 0x824F9E08;
    'dispatch: loop {
        match pc {
            0x824F9E08 => {
    //   block [0x824F9E08..0x824F9E14)
	// 824F9E08: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824F9E0C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9E10: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9E14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9E14 size=8
    let mut pc: u32 = 0x824F9E14;
    'dispatch: loop {
        match pc {
            0x824F9E14 => {
    //   block [0x824F9E14..0x824F9E1C)
	// 824F9E14: 4800EBC4  b 0x825089d8
	sub_825089D8(ctx, base);
	return;
	// 824F9E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9E20 size=20
    let mut pc: u32 = 0x824F9E20;
    'dispatch: loop {
        match pc {
            0x824F9E20 => {
    //   block [0x824F9E20..0x824F9E34)
	// 824F9E20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9E24: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824F9E28: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824F9E2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824F9E30: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
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
    // ---- function 0x824F9E80 size=8
    let mut pc: u32 = 0x824F9E80;
    'dispatch: loop {
        match pc {
            0x824F9E80 => {
    //   block [0x824F9E80..0x824F9E88)
	// 824F9E80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9E84: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9E88 size=44
    let mut pc: u32 = 0x824F9E88;
    'dispatch: loop {
        match pc {
            0x824F9E88 => {
    //   block [0x824F9E88..0x824F9EB4)
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
    //   block [0x824F9EC8..0x824F9F2C)
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
    // ---- function 0x824F9F48 size=8
    let mut pc: u32 = 0x824F9F48;
    'dispatch: loop {
        match pc {
            0x824F9F48 => {
    //   block [0x824F9F48..0x824F9F50)
	// 824F9F48: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824F9F4C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824F9F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824F9F50 size=32
    let mut pc: u32 = 0x824F9F50;
    'dispatch: loop {
        match pc {
            0x824F9F50 => {
    //   block [0x824F9F50..0x824F9F70)
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
    //   block [0x824F9F88..0x824F9FE8)
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
    // ---- function 0x824FA000 size=8
    let mut pc: u32 = 0x824FA000;
    'dispatch: loop {
        match pc {
            0x824FA000 => {
    //   block [0x824FA000..0x824FA008)
	// 824FA000: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA004: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA008 size=32
    let mut pc: u32 = 0x824FA008;
    'dispatch: loop {
        match pc {
            0x824FA008 => {
    //   block [0x824FA008..0x824FA028)
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
    // ---- function 0x824FA050 size=8
    let mut pc: u32 = 0x824FA050;
    'dispatch: loop {
        match pc {
            0x824FA050 => {
    //   block [0x824FA050..0x824FA058)
	// 824FA050: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA054: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA058(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA058 size=24
    let mut pc: u32 = 0x824FA058;
    'dispatch: loop {
        match pc {
            0x824FA058 => {
    //   block [0x824FA058..0x824FA070)
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
    // ---- function 0x824FA098 size=8
    let mut pc: u32 = 0x824FA098;
    'dispatch: loop {
        match pc {
            0x824FA098 => {
    //   block [0x824FA098..0x824FA0A0)
	// 824FA098: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA09C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA0A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA0A0 size=56
    let mut pc: u32 = 0x824FA0A0;
    'dispatch: loop {
        match pc {
            0x824FA0A0 => {
    //   block [0x824FA0A0..0x824FA0D8)
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
    //   block [0x824FA108..0x824FA184)
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
    //   block [0x824FA190..0x824FA1F4)
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
    // ---- function 0x824FA210 size=8
    let mut pc: u32 = 0x824FA210;
    'dispatch: loop {
        match pc {
            0x824FA210 => {
    //   block [0x824FA210..0x824FA218)
	// 824FA210: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA214: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA218 size=56
    let mut pc: u32 = 0x824FA218;
    'dispatch: loop {
        match pc {
            0x824FA218 => {
    //   block [0x824FA218..0x824FA250)
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
    //   block [0x824FA260..0x824FA2C4)
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
    // ---- function 0x824FA308 size=8
    let mut pc: u32 = 0x824FA308;
    'dispatch: loop {
        match pc {
            0x824FA308 => {
    //   block [0x824FA308..0x824FA310)
	// 824FA308: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA30C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA310 size=32
    let mut pc: u32 = 0x824FA310;
    'dispatch: loop {
        match pc {
            0x824FA310 => {
    //   block [0x824FA310..0x824FA330)
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
    //   block [0x824FA340..0x824FA3D8)
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
    // ---- function 0x824FA3F8 size=8
    let mut pc: u32 = 0x824FA3F8;
    'dispatch: loop {
        match pc {
            0x824FA3F8 => {
    //   block [0x824FA3F8..0x824FA400)
	// 824FA3F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA3FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA400 size=56
    let mut pc: u32 = 0x824FA400;
    'dispatch: loop {
        match pc {
            0x824FA400 => {
    //   block [0x824FA400..0x824FA438)
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
    //   block [0x824FA450..0x824FA524)
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
    //   block [0x824FA528..0x824FA58C)
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
    // ---- function 0x824FA5A8 size=8
    let mut pc: u32 = 0x824FA5A8;
    'dispatch: loop {
        match pc {
            0x824FA5A8 => {
    //   block [0x824FA5A8..0x824FA5B0)
	// 824FA5A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA5AC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA5B0 size=44
    let mut pc: u32 = 0x824FA5B0;
    'dispatch: loop {
        match pc {
            0x824FA5B0 => {
    //   block [0x824FA5B0..0x824FA5DC)
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
    // ---- function 0x824FA610 size=8
    let mut pc: u32 = 0x824FA610;
    'dispatch: loop {
        match pc {
            0x824FA610 => {
    //   block [0x824FA610..0x824FA618)
	// 824FA610: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA614: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA618 size=32
    let mut pc: u32 = 0x824FA618;
    'dispatch: loop {
        match pc {
            0x824FA618 => {
    //   block [0x824FA618..0x824FA638)
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
    //   block [0x824FA648..0x824FA6AC)
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
    // ---- function 0x824FA6C8 size=8
    let mut pc: u32 = 0x824FA6C8;
    'dispatch: loop {
        match pc {
            0x824FA6C8 => {
    //   block [0x824FA6C8..0x824FA6D0)
	// 824FA6C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA6CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA6D0 size=56
    let mut pc: u32 = 0x824FA6D0;
    'dispatch: loop {
        match pc {
            0x824FA6D0 => {
    //   block [0x824FA6D0..0x824FA708)
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
    //   block [0x824FA720..0x824FA784)
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
    //   block [0x824FA788..0x824FA7D0)
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
    // ---- function 0x824FA7E8 size=8
    let mut pc: u32 = 0x824FA7E8;
    'dispatch: loop {
        match pc {
            0x824FA7E8 => {
    //   block [0x824FA7E8..0x824FA7F0)
	// 824FA7E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA7EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA7F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA7F0 size=48
    let mut pc: u32 = 0x824FA7F0;
    'dispatch: loop {
        match pc {
            0x824FA7F0 => {
    //   block [0x824FA7F0..0x824FA820)
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
    //   block [0x824FA830..0x824FA894)
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


pub fn sub_824FA8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA8A0 size=20
    let mut pc: u32 = 0x824FA8A0;
    'dispatch: loop {
        match pc {
            0x824FA8A0 => {
    //   block [0x824FA8A0..0x824FA8B4)
	// 824FA8A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA8A4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FA8A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FA8AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FA8B0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA8B8 size=12
    let mut pc: u32 = 0x824FA8B8;
    'dispatch: loop {
        match pc {
            0x824FA8B8 => {
    //   block [0x824FA8B8..0x824FA8C4)
	// 824FA8B8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824FA8BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA8C0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA8C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA8C4 size=8
    let mut pc: u32 = 0x824FA8C4;
    'dispatch: loop {
        match pc {
            0x824FA8C4 => {
    //   block [0x824FA8C4..0x824FA8CC)
	// 824FA8C4: 480073BC  b 0x82501c80
	sub_82501C80(ctx, base);
	return;
	// 824FA8C8: 4E800020  blr
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
    // ---- function 0x824FA920 size=8
    let mut pc: u32 = 0x824FA920;
    'dispatch: loop {
        match pc {
            0x824FA920 => {
    //   block [0x824FA920..0x824FA928)
	// 824FA920: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FA924: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FA928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FA928 size=24
    let mut pc: u32 = 0x824FA928;
    'dispatch: loop {
        match pc {
            0x824FA928 => {
    //   block [0x824FA928..0x824FA940)
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
    //   block [0x824FA950..0x824FA9DC)
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
    //   block [0x824FA9E0..0x824FAA44)
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
    // ---- function 0x824FAAC0 size=12
    let mut pc: u32 = 0x824FAAC0;
    'dispatch: loop {
        match pc {
            0x824FAAC0 => {
    //   block [0x824FAAC0..0x824FAACC)
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


pub fn sub_824FAACC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAACC size=32
    let mut pc: u32 = 0x824FAACC;
    'dispatch: loop {
        match pc {
            0x824FAACC => {
    //   block [0x824FAACC..0x824FAAEC)
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
    //   block [0x824FABA0..0x824FAC30)
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
    // ---- function 0x824FAC30 size=164
    let mut pc: u32 = 0x824FAC30;
    'dispatch: loop {
        match pc {
            0x824FAC30 => {
    //   block [0x824FAC30..0x824FACD4)
	// 824FAC30: 39650010  addi r11, r5, 0x10
	ctx.r[11].s64 = ctx.r[5].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FACD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FACD4 size=92
    let mut pc: u32 = 0x824FACD4;
    'dispatch: loop {
        match pc {
            0x824FACD4 => {
    //   block [0x824FACD4..0x824FAD30)
	// 824FACD4: FD405890  fmr f10, f11
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[10].f64 = ctx.f[11].f64;
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
    //   block [0x824FADC0..0x824FAE18)
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
    //   block [0x824FAE48..0x824FAEF4)
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
	ctx.r[5].s64 = ctx.r[11].s64 * 56;
	// 824FAEB0: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 824FAEB4: 4BF69205  bl 0x824640b8
	ctx.lr = 0x824FAEB8;
	sub_824640B8(ctx, base);
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
    //   block [0x824FAF00..0x824FAF64)
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
    // ---- function 0x824FAF80 size=8
    let mut pc: u32 = 0x824FAF80;
    'dispatch: loop {
        match pc {
            0x824FAF80 => {
    //   block [0x824FAF80..0x824FAF88)
	// 824FAF80: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FAF84: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAF88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAF88 size=32
    let mut pc: u32 = 0x824FAF88;
    'dispatch: loop {
        match pc {
            0x824FAF88 => {
    //   block [0x824FAF88..0x824FAFA8)
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
    // ---- function 0x824FAFD8 size=8
    let mut pc: u32 = 0x824FAFD8;
    'dispatch: loop {
        match pc {
            0x824FAFD8 => {
    //   block [0x824FAFD8..0x824FAFE0)
	// 824FAFD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FAFDC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FAFE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FAFE0 size=32
    let mut pc: u32 = 0x824FAFE0;
    'dispatch: loop {
        match pc {
            0x824FAFE0 => {
    //   block [0x824FAFE0..0x824FB000)
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
    // ---- function 0x824FB028 size=12
    let mut pc: u32 = 0x824FB028;
    'dispatch: loop {
        match pc {
            0x824FB028 => {
    //   block [0x824FB028..0x824FB034)
	// 824FB028: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824FB02C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FB030: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB034(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB034 size=8
    let mut pc: u32 = 0x824FB034;
    'dispatch: loop {
        match pc {
            0x824FB034 => {
    //   block [0x824FB034..0x824FB03C)
	// 824FB034: 4800003C  b 0x824fb070
	sub_824FB070(ctx, base);
	return;
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
    // ---- function 0x824FB100 size=8
    let mut pc: u32 = 0x824FB100;
    'dispatch: loop {
        match pc {
            0x824FB100 => {
    //   block [0x824FB100..0x824FB108)
	// 824FB100: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FB104: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB108 size=44
    let mut pc: u32 = 0x824FB108;
    'dispatch: loop {
        match pc {
            0x824FB108 => {
    //   block [0x824FB108..0x824FB134)
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
    //   block [0x824FB148..0x824FB1D8)
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
	// 824FB188: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824FB18C: 48000034  b 0x824fb1c0
	pc = 0x824FB1C0; continue 'dispatch;
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
	// 824FB1B8: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 824FB1BC: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
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
    //   block [0x824FB1D8..0x824FB24C)
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


pub fn sub_824FB258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB258 size=12
    let mut pc: u32 = 0x824FB258;
    'dispatch: loop {
        match pc {
            0x824FB258 => {
    //   block [0x824FB258..0x824FB264)
	// 824FB258: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824FB25C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FB260: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB264(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB264 size=8
    let mut pc: u32 = 0x824FB264;
    'dispatch: loop {
        match pc {
            0x824FB264 => {
    //   block [0x824FB264..0x824FB26C)
	// 824FB264: 48013C44  b 0x8250eea8
	sub_8250EEA8(ctx, base);
	return;
	// 824FB268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB270 size=20
    let mut pc: u32 = 0x824FB270;
    'dispatch: loop {
        match pc {
            0x824FB270 => {
    //   block [0x824FB270..0x824FB284)
	// 824FB270: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB274: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824FB278: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824FB27C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824FB280: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
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
    // ---- function 0x824FB2D8 size=8
    let mut pc: u32 = 0x824FB2D8;
    'dispatch: loop {
        match pc {
            0x824FB2D8 => {
    //   block [0x824FB2D8..0x824FB2E0)
	// 824FB2D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824FB2DC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824FB2E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824FB2E0 size=32
    let mut pc: u32 = 0x824FB2E0;
    'dispatch: loop {
        match pc {
            0x824FB2E0 => {
    //   block [0x824FB2E0..0x824FB300)
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
    //   block [0x824FB310..0x824FB374)
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
	ctx.r[11].s64 = (ctx.r[11].s32 as i64) * (ctx.r[5].s32 as i64);
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
    //   block [0x824FB3A8..0x824FB6F4)
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
    //   block [0x824FB6F8..0x824FB78C)
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
    //   block [0x824FB790..0x824FB82C)
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


