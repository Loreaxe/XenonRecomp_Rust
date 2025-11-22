pub fn sub_822E14A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E14A8 size=352
    let mut pc: u32 = 0x822E14A8;
    'dispatch: loop {
        match pc {
            0x822E14A8 => {
    //   block [0x822E14A8..0x822E1608)
	// 822E14A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E14AC: 48EC6CBD  bl 0x831a8168
	ctx.lr = 0x822E14B0;
	sub_831A8130(ctx, base);
	// 822E14B0: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 822E14B4: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 822E14B8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E14BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E14C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E14C4: 3B9E0020  addi r28, r30, 0x20
	ctx.r[28].s64 = ctx.r[30].s64 + 32;
	// 822E14C8: 389F0120  addi r4, r31, 0x120
	ctx.r[4].s64 = ctx.r[31].s64 + 288;
	// 822E14CC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822E14D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E14D4: 48B9A7F5  bl 0x82e7bcc8
	ctx.lr = 0x822E14D8;
	sub_82E7BCC8(ctx, base);
	// 822E14D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E14DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822E14E0: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E14E4: C02B9450  lfs f1, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822E14E8: 4BFEEFE1  bl 0x822d04c8
	ctx.lr = 0x822E14EC;
	sub_822D04C8(ctx, base);
	// 822E14EC: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E14F0: C3DE0030  lfs f30, 0x30(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 822E14F4: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822E14F8: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822E14FC: 4BFEEA15  bl 0x822cff10
	ctx.lr = 0x822E1500;
	sub_822CFF10(ctx, base);
	// 822E1500: C01F0160  lfs f0, 0x160(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E1504: EC01F838  fmsubs f0, f1, f0, f31
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64 - ctx.f[31].f64) as f32) as f64);
	// 822E1508: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822E150C: FF00F000  fcmpu cr6, f0, f30
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[30].f64);
	// 822E1510: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E1514: 4199002C  bgt cr6, 0x822e1540
	if ctx.cr[6].gt {
	pc = 0x822E1540; continue 'dispatch;
	}
	// 822E1518: C1BE0034  lfs f13, 0x34(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E151C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 822E1520: 419A0018  beq cr6, 0x822e1538
	if ctx.cr[6].eq {
	pc = 0x822E1538; continue 'dispatch;
	}
	// 822E1524: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E1528: 4BFEE9E9  bl 0x822cff10
	ctx.lr = 0x822E152C;
	sub_822CFF10(ctx, base);
	// 822E152C: C01F0160  lfs f0, 0x160(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E1530: C1BE0030  lfs f13, 0x30(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E1534: EFE16838  fmsubs f31, f1, f0, f13
	ctx.f[31].f64 = (((ctx.f[1].f64 * ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 822E1538: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 822E153C: 48000024  b 0x822e1560
	pc = 0x822E1560; continue 'dispatch;
	// 822E1540: EDBFF028  fsubs f13, f31, f30
	ctx.f[13].f64 = (((ctx.f[31].f64 - ctx.f[30].f64) as f32) as f64);
	// 822E1544: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 822E1548: 41990018  bgt cr6, 0x822e1560
	if ctx.cr[6].gt {
	pc = 0x822E1560; continue 'dispatch;
	}
	// 822E154C: C1BE0034  lfs f13, 0x34(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E1550: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 822E1554: 419A0008  beq cr6, 0x822e155c
	if ctx.cr[6].eq {
	pc = 0x822E155C; continue 'dispatch;
	}
	// 822E1558: FFE0F090  fmr f31, f30
	ctx.f[31].f64 = ctx.f[30].f64;
	// 822E155C: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 822E1560: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822E1564: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E1568: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822E156C: 4BFEEC65  bl 0x822d01d0
	ctx.lr = 0x822E1570;
	sub_822D01D0(ctx, base);
	// 822E1570: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822E1574: 389F00E0  addi r4, r31, 0xe0
	ctx.r[4].s64 = ctx.r[31].s64 + 224;
	// 822E1578: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822E157C: 48B9A74D  bl 0x82e7bcc8
	ctx.lr = 0x822E1580;
	sub_82E7BCC8(ctx, base);
	// 822E1580: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E1584: 13C0E0C7  vcmpequd (lvx128) v30, v0, v28
	tmp.u32 = ctx.r[28].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E1608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E1608 size=140
    let mut pc: u32 = 0x822E1608;
    'dispatch: loop {
        match pc {
            0x822E1608 => {
    //   block [0x822E1608..0x822E1694)
	// 822E1608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E160C: 48EC6B5D  bl 0x831a8168
	ctx.lr = 0x822E1610;
	sub_831A8130(ctx, base);
	// 822E1610: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E1614: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822E1618: 13E02407  vcmpneb. (lvlx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E161C: 3BA00010  li r29, 0x10
	ctx.r[29].s64 = 16;
	// 822E1620: 3BC00020  li r30, 0x20
	ctx.r[30].s64 = 32;
	// 822E1624: 3BE00030  li r31, 0x30
	ctx.r[31].s64 = 48;
	// 822E1628: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E162C: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E1630: 13DD2407  vcmpneb. (lvlx128) v30, v29, v4
	tmp.u32 = ctx.r[29].u32 + ctx.r[4].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E1634: 396B00E0  addi r11, r11, 0xe0
	ctx.r[11].s64 = ctx.r[11].s64 + 224;
	// 822E1638: 13BE2407  vcmpneb. (lvlx128) v29, v30, v4
	tmp.u32 = ctx.r[30].u32 + ctx.r[4].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E163C: 139F2407  vcmpneb. (lvlx128) v28, v31, v4
	tmp.u32 = ctx.r[31].u32 + ctx.r[4].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E1698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E1698 size=760
    let mut pc: u32 = 0x822E1698;
    'dispatch: loop {
        match pc {
            0x822E1698 => {
    //   block [0x822E1698..0x822E1990)
	// 822E1698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E169C: 48EC6AD1  bl 0x831a816c
	ctx.lr = 0x822E16A0;
	sub_831A8130(ctx, base);
	// 822E16A0: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 822E16A4: 48EC73C9  bl 0x831a8a6c
	ctx.lr = 0x822E16A8;
	sub_831A8A40(ctx, base);
	// 822E16A8: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E16AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822E16B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E16B4: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 822E16B8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 822E16BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E16C0: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 822E16C4: 38BE0020  addi r5, r30, 0x20
	ctx.r[5].s64 = ctx.r[30].s64 + 32;
	// 822E16C8: D3FE002C  stfs f31, 0x2c(r30)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 822E16CC: 915E0030  stw r10, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 822E16D0: 913E0060  stw r9, 0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 822E16D4: 389F0120  addi r4, r31, 0x120
	ctx.r[4].s64 = ctx.r[31].s64 + 288;
	// 822E16D8: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 822E16DC: 48B9A5ED  bl 0x82e7bcc8
	ctx.lr = 0x822E16E0;
	sub_82E7BCC8(ctx, base);
	// 822E16E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E16E4: 388100E0  addi r4, r1, 0xe0
	ctx.r[4].s64 = ctx.r[1].s64 + 224;
	// 822E16E8: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E16EC: C02B9450  lfs f1, -0x6bb0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822E16F0: 4BFEEDD9  bl 0x822d04c8
	ctx.lr = 0x822E16F4;
	sub_822D04C8(ctx, base);
	// 822E16F4: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E16F8: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 822E16FC: 4BFEE815  bl 0x822cff10
	ctx.lr = 0x822E1700;
	sub_822CFF10(ctx, base);
	// 822E1700: C01F0160  lfs f0, 0x160(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E1704: EC01F038  fmsubs f0, f1, f0, f30
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[0].f64 - ctx.f[30].f64) as f32) as f64);
	// 822E1708: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E170C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E1710: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 822E1714: C38BA9F0  lfs f28, -0x5610(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22032 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 822E1718: C32A9534  lfs f25, -0x6acc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[25].f64 = (tmp.f32 as f64);
	// 822E171C: C3A908A8  lfs f29, 0x8a8(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2216 as u32) ) };
	ctx.f[29].f64 = (tmp.f32 as f64);
	// 822E1720: FF00E000  fcmpu cr6, f0, f28
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[28].f64);
	// 822E1724: 4199001C  bgt cr6, 0x822e1740
	if ctx.cr[6].gt {
	pc = 0x822E1740; continue 'dispatch;
	}
	// 822E1728: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E172C: 4BFEE7E5  bl 0x822cff10
	ctx.lr = 0x822E1730;
	sub_822CFF10(ctx, base);
	// 822E1730: C01F0160  lfs f0, 0x160(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E1734: D3BE002C  stfs f29, 0x2c(r30)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 822E1738: EFC1E038  fmsubs f30, f1, f0, f28
	ctx.f[30].f64 = (((ctx.f[1].f64 * ctx.f[0].f64 - ctx.f[28].f64) as f32) as f64);
	// 822E173C: 48000018  b 0x822e1754
	pc = 0x822E1754; continue 'dispatch;
	// 822E1740: EC1EE028  fsubs f0, f30, f28
	ctx.f[0].f64 = (((ctx.f[30].f64 - ctx.f[28].f64) as f32) as f64);
	// 822E1744: FF00F800  fcmpu cr6, f0, f31
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[31].f64);
	// 822E1748: 4199000C  bgt cr6, 0x822e1754
	if ctx.cr[6].gt {
	pc = 0x822E1754; continue 'dispatch;
	}
	// 822E174C: D33E002C  stfs f25, 0x2c(r30)
	tmp.f32 = (ctx.f[25].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 822E1750: FFC0E090  fmr f30, f28
	ctx.f[30].f64 = ctx.f[28].f64;
	// 822E1754: 817F0168  lwz r11, 0x168(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) } as u64;
	// 822E1758: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 822E175C: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E1760: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E1764: 419A004C  beq cr6, 0x822e17b0
	if ctx.cr[6].eq {
	pc = 0x822E17B0; continue 'dispatch;
	}
	// 822E1768: 38A10100  addi r5, r1, 0x100
	ctx.r[5].s64 = ctx.r[1].s64 + 256;
	// 822E176C: 4BFEE8B5  bl 0x822d0020
	ctx.lr = 0x822E1770;
	sub_822D0020(ctx, base);
	// 822E1770: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 822E1774: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E1778: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 822E177C: 4BFEEA55  bl 0x822d01d0
	ctx.lr = 0x822E1780;
	sub_822D01D0(ctx, base);
	// 822E1780: 807F0168  lwz r3, 0x168(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) } as u64;
	// 822E1784: 3BA100C0  addi r29, r1, 0xc0
	ctx.r[29].s64 = ctx.r[1].s64 + 192;
	// 822E1788: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E178C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E1790: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E1794: 4E800421  bctrl
	ctx.lr = 0x822E1798;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E1798: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E179C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 822E17A0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822E17A4: 48B9A525  bl 0x82e7bcc8
	ctx.lr = 0x822E17A8;
	sub_82E7BCC8(ctx, base);
	// 822E17A8: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 822E17AC: 48000030  b 0x822e17dc
	pc = 0x822E17DC; continue 'dispatch;
	// 822E17B0: 38A10120  addi r5, r1, 0x120
	ctx.r[5].s64 = ctx.r[1].s64 + 288;
	// 822E17B4: 4BFEE86D  bl 0x822d0020
	ctx.lr = 0x822E17B8;
	sub_822D0020(ctx, base);
	// 822E17B8: 38A100D0  addi r5, r1, 0xd0
	ctx.r[5].s64 = ctx.r[1].s64 + 208;
	// 822E17BC: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E17C0: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 822E17C4: 4BFEEA0D  bl 0x822d01d0
	ctx.lr = 0x822E17C8;
	sub_822D01D0(ctx, base);
	// 822E17C8: 38A100D0  addi r5, r1, 0xd0
	ctx.r[5].s64 = ctx.r[1].s64 + 208;
	// 822E17CC: 389F00E0  addi r4, r31, 0xe0
	ctx.r[4].s64 = ctx.r[31].s64 + 224;
	// 822E17D0: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 822E17D4: 48B9A4F5  bl 0x82e7bcc8
	ctx.lr = 0x822E17D8;
	sub_82E7BCC8(ctx, base);
	// 822E17D8: 396100D0  addi r11, r1, 0xd0
	ctx.r[11].s64 = ctx.r[1].s64 + 208;
	// 822E17DC: 39400040  li r10, 0x40
	ctx.r[10].s64 = 64;
	// 822E17E0: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E1990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E1990 size=380
    let mut pc: u32 = 0x822E1990;
    'dispatch: loop {
        match pc {
            0x822E1990 => {
    //   block [0x822E1990..0x822E1B0C)
	// 822E1990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E1994: 48EC67D9  bl 0x831a816c
	ctx.lr = 0x822E1998;
	sub_831A8130(ctx, base);
	// 822E1998: 3981FFE0  addi r12, r1, -0x20
	ctx.r[12].s64 = ctx.r[1].s64 + -32;
	// 822E199C: 48EC70D9  bl 0x831a8a74
	ctx.lr = 0x822E19A0;
	sub_831A8A40(ctx, base);
	// 822E19A0: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E19A4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822E19A8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822E19AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E19B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E19B4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822E19B8: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 822E19BC: C38A08A8  lfs f28, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[28].f64 = (tmp.f32 as f64);
	// 822E19C0: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822E19C4: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E19C8: D3810054  stfs f28, 0x54(r1)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 822E19CC: D3E10058  stfs f31, 0x58(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 822E19D0: D3E1005C  stfs f31, 0x5c(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 822E19D4: C01F0160  lfs f0, 0x160(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E19D8: C1BE0018  lfs f13, 0x18(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E19DC: FFA00090  fmr f29, f0
	ctx.f[29].f64 = ctx.f[0].f64;
	// 822E19E0: C369964C  lfs f27, -0x69b4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[27].f64 = (tmp.f32 as f64);
	// 822E19E4: EFC06EFA  fmadds f30, f0, f27, f13
	ctx.f[30].f64 = (((ctx.f[0].f64 * ctx.f[27].f64 + ctx.f[13].f64) as f32) as f64);
	// 822E19E8: 4BFEE529  bl 0x822cff10
	ctx.lr = 0x822E19EC;
	sub_822CFF10(ctx, base);
	// 822E19EC: EC010772  fmuls f0, f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[1].f64 * ctx.f[29].f64) as f32) as f64);
	// 822E19F0: FF1E0000  fcmpu cr6, f30, f0
	ctx.cr[6].compare_f64(ctx.f[30].f64, ctx.f[0].f64);
	// 822E19F4: 40990024  ble cr6, 0x822e1a18
	if !ctx.cr[6].gt {
	pc = 0x822E1A18; continue 'dispatch;
	}
	// 822E19F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E19FC: C01E0018  lfs f0, 0x18(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E1A00: EFDD06FC  fnmsubs f30, f29, f27, f0
	ctx.f[30].f64 = -(((ctx.f[29].f64 * ctx.f[27].f64 - ctx.f[0].f64) as f32) as f64);
	// 822E1A04: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822E1A08: D3E10058  stfs f31, 0x58(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 822E1A0C: D381005C  stfs f28, 0x5c(r1)
	tmp.f32 = (ctx.f[28].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 822E1A10: C00B9534  lfs f0, -0x6acc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E1A14: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 822E1A18: C01E0018  lfs f0, 0x18(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E1A1C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822E1A20: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E1A24: EC20E824  fdivs f1, f0, f29
	ctx.f[1].f64 = ((ctx.f[0].f64 / ctx.f[29].f64) as f32) as f64;
	// 822E1A28: 4BFEE7A9  bl 0x822d01d0
	ctx.lr = 0x822E1A2C;
	sub_822D01D0(ctx, base);
	// 822E1A2C: 3BBF00E0  addi r29, r31, 0xe0
	ctx.r[29].s64 = ctx.r[31].s64 + 224;
	// 822E1A30: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822E1A34: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E1A38: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 822E1A3C: 48B9A28D  bl 0x82e7bcc8
	ctx.lr = 0x822E1A40;
	sub_82E7BCC8(ctx, base);
	// 822E1A40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E1A44: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 822E1A48: C01F0160  lfs f0, 0x160(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E1A4C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 822E1A50: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E1A54: EC3E0024  fdivs f1, f30, f0
	ctx.f[1].f64 = ((ctx.f[30].f64 / ctx.f[0].f64) as f32) as f64;
	// 822E1A58: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E1B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E1B10 size=76
    let mut pc: u32 = 0x822E1B10;
    'dispatch: loop {
        match pc {
            0x822E1B10 => {
    //   block [0x822E1B10..0x822E1B5C)
	// 822E1B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E1B14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E1B18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E1B1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E1B20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E1B24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E1B28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E1B2C: 4BFFF67D  bl 0x822e11a8
	ctx.lr = 0x822E1B30;
	sub_822E11A8(ctx, base);
	// 822E1B30: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E1B34: 4182000C  beq 0x822e1b40
	if ctx.cr[0].eq {
	pc = 0x822E1B40; continue 'dispatch;
	}
	// 822E1B38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E1B3C: 48B1089D  bl 0x82df23d8
	ctx.lr = 0x822E1B40;
	sub_82DF23D8(ctx, base);
	// 822E1B40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E1B44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E1B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E1B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E1B50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E1B54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E1B58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E1B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E1B60 size=444
    let mut pc: u32 = 0x822E1B60;
    'dispatch: loop {
        match pc {
            0x822E1B60 => {
    //   block [0x822E1B60..0x822E1D1C)
	// 822E1B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E1B64: 48EC65E9  bl 0x831a814c
	ctx.lr = 0x822E1B68;
	sub_831A8130(ctx, base);
	// 822E1B68: DBE1FF98  stfd f31, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[31].u64 ) };
	// 822E1B6C: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E1B70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E1B74: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822E1B78: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 822E1B7C: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 822E1B80: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 822E1B84: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 822E1B88: 7D354B78  mr r21, r9
	ctx.r[21].u64 = ctx.r[9].u64;
	// 822E1B8C: 4822F565  bl 0x825110f0
	ctx.lr = 0x822E1B90;
	sub_825110F0(ctx, base);
	// 822E1B90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E1B94: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E1B98: 392BA9AC  addi r9, r11, -0x5654
	ctx.r[9].s64 = ctx.r[11].s64 + -22100;
	// 822E1B9C: 394AA998  addi r10, r10, -0x5668
	ctx.r[10].s64 = ctx.r[10].s64 + -22120;
	// 822E1BA0: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E1BA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E1BA8: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 822E1BAC: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 822E1BB0: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 822E1BB4: 3B200010  li r25, 0x10
	ctx.r[25].s64 = 16;
	// 822E1BB8: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 822E1BBC: 39296880  addi r9, r9, 0x6880
	ctx.r[9].s64 = ctx.r[9].s64 + 26752;
	// 822E1BC0: 3B400020  li r26, 0x20
	ctx.r[26].s64 = 32;
	// 822E1BC4: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 822E1BC8: 3B600030  li r27, 0x30
	ctx.r[27].s64 = 48;
	// 822E1BCC: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 822E1BD0: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 822E1BD4: 3BDF00E0  addi r30, r31, 0xe0
	ctx.r[30].s64 = ctx.r[31].s64 + 224;
	// 822E1BD8: 917F00D8  stw r11, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 822E1BDC: 395F0168  addi r10, r31, 0x168
	ctx.r[10].s64 = ctx.r[31].s64 + 360;
	// 822E1BE0: 93BF00DC  stw r29, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[29].u32 ) };
	// 822E1BE4: 13F94C07  vcmpneb. (lvlx128) v31, v25, v9
	tmp.u32 = ctx.r[25].u32 + ctx.r[9].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E1BE8: 13DA4C07  vcmpneb. (lvlx128) v30, v26, v9
	tmp.u32 = ctx.r[26].u32 + ctx.r[9].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E1BEC: 3BBF00C0  addi r29, r31, 0xc0
	ctx.r[29].s64 = ctx.r[31].s64 + 192;
	// 822E1BF0: 13BB4C07  vcmpneb. (lvlx128) v29, v27, v9
	tmp.u32 = ctx.r[27].u32 + ctx.r[9].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E1BF4: 3B9F00CC  addi r28, r31, 0xcc
	ctx.r[28].s64 = ctx.r[31].s64 + 204;
	// 822E1BF8: 13804C07  vcmpneb. (lvlx128) v28, v0, v9
	tmp.u32 = ctx.r[9].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E1D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E1D20 size=20
    let mut pc: u32 = 0x822E1D20;
    'dispatch: loop {
        match pc {
            0x822E1D20 => {
    //   block [0x822E1D20..0x822E1D34)
	// 822E1D20: 81230168  lwz r9, 0x168(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(360 as u32) ) } as u64;
	// 822E1D24: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 822E1D28: 39630168  addi r11, r3, 0x168
	ctx.r[11].s64 = ctx.r[3].s64 + 360;
	// 822E1D2C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822E1D30: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E1D34(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E1D34 size=24
    let mut pc: u32 = 0x822E1D34;
    'dispatch: loop {
        match pc {
            0x822E1D34 => {
    //   block [0x822E1D34..0x822E1D4C)
	// 822E1D34: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E1D38: 392A0018  addi r9, r10, 0x18
	ctx.r[9].s64 = ctx.r[10].s64 + 24;
	// 822E1D3C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 822E1D40: 38690004  addi r3, r9, 4
	ctx.r[3].s64 = ctx.r[9].s64 + 4;
	// 822E1D44: 910A0018  stw r8, 0x18(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(24 as u32), ctx.r[8].u32 ) };
	// 822E1D48: 4BFE2718  b 0x822c4460
	sub_822C4460(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E1D4C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E1D4C size=4
    let mut pc: u32 = 0x822E1D4C;
    'dispatch: loop {
        match pc {
            0x822E1D4C => {
    //   block [0x822E1D4C..0x822E1D50)
	// 822E1D4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E1D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E1D50 size=112
    let mut pc: u32 = 0x822E1D50;
    'dispatch: loop {
        match pc {
            0x822E1D50 => {
    //   block [0x822E1D50..0x822E1DC0)
	// 822E1D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E1D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E1D58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E1D5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E1D60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E1D64: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E1D68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E1D6C: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 822E1D70: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822E1D74: 4BFFF2A5  bl 0x822e1018
	ctx.lr = 0x822E1D78;
	sub_822E1018(ctx, base);
	// 822E1D78: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822E1D7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E1D80: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822E1D84: 4BFDE27D  bl 0x822c0000
	ctx.lr = 0x822E1D88;
	sub_822C0000(ctx, base);
	// 822E1D88: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E1D8C: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822E1D90: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E1D94: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E1D98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E1D9C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822E1DA0: 419A0008  beq cr6, 0x822e1da8
	if ctx.cr[6].eq {
	pc = 0x822E1DA8; continue 'dispatch;
	}
	// 822E1DA4: 4BFDEAED  bl 0x822c0890
	ctx.lr = 0x822E1DA8;
	sub_822C0890(ctx, base);
	// 822E1DA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E1DAC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E1DB0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E1DB4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E1DB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E1DBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E1DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E1DC0 size=112
    let mut pc: u32 = 0x822E1DC0;
    'dispatch: loop {
        match pc {
            0x822E1DC0 => {
    //   block [0x822E1DC0..0x822E1E30)
	// 822E1DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E1DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E1DC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E1DCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E1DD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E1DD4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E1DD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E1DDC: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 822E1DE0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822E1DE4: 4BFFF2FD  bl 0x822e10e0
	ctx.lr = 0x822E1DE8;
	sub_822E10E0(ctx, base);
	// 822E1DE8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822E1DEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E1DF0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822E1DF4: 4BFDE20D  bl 0x822c0000
	ctx.lr = 0x822E1DF8;
	sub_822C0000(ctx, base);
	// 822E1DF8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E1DFC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822E1E00: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E1E04: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E1E08: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E1E0C: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822E1E10: 419A0008  beq cr6, 0x822e1e18
	if ctx.cr[6].eq {
	pc = 0x822E1E18; continue 'dispatch;
	}
	// 822E1E14: 4BFDEA7D  bl 0x822c0890
	ctx.lr = 0x822E1E18;
	sub_822C0890(ctx, base);
	// 822E1E18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E1E1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E1E20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E1E24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E1E28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E1E2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E1E30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E1E30 size=556
    let mut pc: u32 = 0x822E1E30;
    'dispatch: loop {
        match pc {
            0x822E1E30 => {
    //   block [0x822E1E30..0x822E205C)
	// 822E1E30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E1E34: 48EC6335  bl 0x831a8168
	ctx.lr = 0x822E1E38;
	sub_831A8130(ctx, base);
	// 822E1E38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E1E3C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822E1E40: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E1E44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E1E48: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822E1E4C: 41820038  beq 0x822e1e84
	if ctx.cr[0].eq {
	pc = 0x822E1E84; continue 'dispatch;
	}
	// 822E1E50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E1E54: 48EC7B35  bl 0x831a9988
	ctx.lr = 0x822E1E58;
	sub_831A9988(ctx, base);
	// 822E1E58: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E1E5C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E1E60: 386B35EC  addi r3, r11, 0x35ec
	ctx.r[3].s64 = ctx.r[11].s64 + 13804;
	// 822E1E64: 48EC6295  bl 0x831a80f8
	ctx.lr = 0x822E1E68;
	sub_831A80F8(ctx, base);
	// 822E1E68: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E1E6C: 41820018  beq 0x822e1e84
	if ctx.cr[0].eq {
	pc = 0x822E1E84; continue 'dispatch;
	}
	// 822E1E70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E1E74: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 822E1E78: 4BFFF631  bl 0x822e14a8
	ctx.lr = 0x822E1E7C;
	sub_822E14A8(ctx, base);
	// 822E1E7C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822E1E80: 480001D4  b 0x822e2054
	pc = 0x822E2054; continue 'dispatch;
	// 822E1E84: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822E1E88: 419A01BC  beq cr6, 0x822e2044
	if ctx.cr[6].eq {
	pc = 0x822E2044; continue 'dispatch;
	}
	// 822E1E8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E1E90: 48EC7AF9  bl 0x831a9988
	ctx.lr = 0x822E1E94;
	sub_831A9988(ctx, base);
	// 822E1E94: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E1E98: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E1E9C: 386B35BC  addi r3, r11, 0x35bc
	ctx.r[3].s64 = ctx.r[11].s64 + 13756;
	// 822E1EA0: 48EC6259  bl 0x831a80f8
	ctx.lr = 0x822E1EA4;
	sub_831A80F8(ctx, base);
	// 822E1EA4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E1EA8: 41820014  beq 0x822e1ebc
	if ctx.cr[0].eq {
	pc = 0x822E1EBC; continue 'dispatch;
	}
	// 822E1EAC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E1EB0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 822E1EB4: 48000545  bl 0x822e23f8
	ctx.lr = 0x822E1EB8;
	sub_822E23F8(ctx, base);
	// 822E1EB8: 4BFFFFC4  b 0x822e1e7c
	pc = 0x822E1E7C; continue 'dispatch;
	// 822E1EBC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822E1EC0: 419A0184  beq cr6, 0x822e2044
	if ctx.cr[6].eq {
	pc = 0x822E2044; continue 'dispatch;
	}
	// 822E1EC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E1EC8: 48EC7AC1  bl 0x831a9988
	ctx.lr = 0x822E1ECC;
	sub_831A9988(ctx, base);
	// 822E1ECC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E1ED0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E1ED4: 386B3B00  addi r3, r11, 0x3b00
	ctx.r[3].s64 = ctx.r[11].s64 + 15104;
	// 822E1ED8: 48EC6221  bl 0x831a80f8
	ctx.lr = 0x822E1EDC;
	sub_831A80F8(ctx, base);
	// 822E1EDC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E1EE0: 41820014  beq 0x822e1ef4
	if ctx.cr[0].eq {
	pc = 0x822E1EF4; continue 'dispatch;
	}
	// 822E1EE4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E1EE8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 822E1EEC: 4BFFF0C5  bl 0x822e0fb0
	ctx.lr = 0x822E1EF0;
	sub_822E0FB0(ctx, base);
	// 822E1EF0: 4BFFFF8C  b 0x822e1e7c
	pc = 0x822E1E7C; continue 'dispatch;
	// 822E1EF4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822E1EF8: 419A014C  beq cr6, 0x822e2044
	if ctx.cr[6].eq {
	pc = 0x822E2044; continue 'dispatch;
	}
	// 822E1EFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E1F00: 48EC7A89  bl 0x831a9988
	ctx.lr = 0x822E1F04;
	sub_831A9988(ctx, base);
	// 822E1F04: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E1F08: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E1F0C: 386B3AD0  addi r3, r11, 0x3ad0
	ctx.r[3].s64 = ctx.r[11].s64 + 15056;
	// 822E1F10: 48EC61E9  bl 0x831a80f8
	ctx.lr = 0x822E1F14;
	sub_831A80F8(ctx, base);
	// 822E1F14: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E1F18: 41820014  beq 0x822e1f2c
	if ctx.cr[0].eq {
	pc = 0x822E1F2C; continue 'dispatch;
	}
	// 822E1F1C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E1F20: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 822E1F24: 4BFFF045  bl 0x822e0f68
	ctx.lr = 0x822E1F28;
	sub_822E0F68(ctx, base);
	// 822E1F28: 4BFFFF54  b 0x822e1e7c
	pc = 0x822E1E7C; continue 'dispatch;
	// 822E1F2C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822E1F30: 419A0114  beq cr6, 0x822e2044
	if ctx.cr[6].eq {
	pc = 0x822E2044; continue 'dispatch;
	}
	// 822E1F34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E1F38: 48EC7A51  bl 0x831a9988
	ctx.lr = 0x822E1F3C;
	sub_831A9988(ctx, base);
	// 822E1F3C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E1F40: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E1F44: 386B3AA4  addi r3, r11, 0x3aa4
	ctx.r[3].s64 = ctx.r[11].s64 + 15012;
	// 822E1F48: 48EC61B1  bl 0x831a80f8
	ctx.lr = 0x822E1F4C;
	sub_831A80F8(ctx, base);
	// 822E1F4C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E1F50: 41820014  beq 0x822e1f64
	if ctx.cr[0].eq {
	pc = 0x822E1F64; continue 'dispatch;
	}
	// 822E1F54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E1F58: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 822E1F5C: 4BFFFA35  bl 0x822e1990
	ctx.lr = 0x822E1F60;
	sub_822E1990(ctx, base);
	// 822E1F60: 4BFFFF1C  b 0x822e1e7c
	pc = 0x822E1E7C; continue 'dispatch;
	// 822E1F64: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822E1F68: 419A00DC  beq cr6, 0x822e2044
	if ctx.cr[6].eq {
	pc = 0x822E2044; continue 'dispatch;
	}
	// 822E1F6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E1F70: 48EC7A19  bl 0x831a9988
	ctx.lr = 0x822E1F74;
	sub_831A9988(ctx, base);
	// 822E1F74: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E1F78: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E1F7C: 386B3A70  addi r3, r11, 0x3a70
	ctx.r[3].s64 = ctx.r[11].s64 + 14960;
	// 822E1F80: 48EC6179  bl 0x831a80f8
	ctx.lr = 0x822E1F84;
	sub_831A80F8(ctx, base);
	// 822E1F84: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E1F88: 41820014  beq 0x822e1f9c
	if ctx.cr[0].eq {
	pc = 0x822E1F9C; continue 'dispatch;
	}
	// 822E1F8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E1F90: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 822E1F94: 4BFFFD8D  bl 0x822e1d20
	ctx.lr = 0x822E1F98;
	sub_822E1D20(ctx, base);
	// 822E1F98: 4BFFFEE4  b 0x822e1e7c
	pc = 0x822E1E7C; continue 'dispatch;
	// 822E1F9C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822E1FA0: 419A00A4  beq cr6, 0x822e2044
	if ctx.cr[6].eq {
	pc = 0x822E2044; continue 'dispatch;
	}
	// 822E1FA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E1FA8: 48EC79E1  bl 0x831a9988
	ctx.lr = 0x822E1FAC;
	sub_831A9988(ctx, base);
	// 822E1FAC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E1FB0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E1FB4: 386B3A44  addi r3, r11, 0x3a44
	ctx.r[3].s64 = ctx.r[11].s64 + 14916;
	// 822E1FB8: 48EC6141  bl 0x831a80f8
	ctx.lr = 0x822E1FBC;
	sub_831A80F8(ctx, base);
	// 822E1FBC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E1FC0: 41820014  beq 0x822e1fd4
	if ctx.cr[0].eq {
	pc = 0x822E1FD4; continue 'dispatch;
	}
	// 822E1FC4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E1FC8: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 822E1FCC: 4BFFF6CD  bl 0x822e1698
	ctx.lr = 0x822E1FD0;
	sub_822E1698(ctx, base);
	// 822E1FD0: 4BFFFEAC  b 0x822e1e7c
	pc = 0x822E1E7C; continue 'dispatch;
	// 822E1FD4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822E1FD8: 419A006C  beq cr6, 0x822e2044
	if ctx.cr[6].eq {
	pc = 0x822E2044; continue 'dispatch;
	}
	// 822E1FDC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E1FE0: 48EC79A9  bl 0x831a9988
	ctx.lr = 0x822E1FE4;
	sub_831A9988(ctx, base);
	// 822E1FE4: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E1FE8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E1FEC: 386B3A18  addi r3, r11, 0x3a18
	ctx.r[3].s64 = ctx.r[11].s64 + 14872;
	// 822E1FF0: 48EC6109  bl 0x831a80f8
	ctx.lr = 0x822E1FF4;
	sub_831A80F8(ctx, base);
	// 822E1FF4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E1FF8: 41820014  beq 0x822e200c
	if ctx.cr[0].eq {
	pc = 0x822E200C; continue 'dispatch;
	}
	// 822E1FFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E2000: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 822E2004: 4BFFEF05  bl 0x822e0f08
	ctx.lr = 0x822E2008;
	sub_822E0F08(ctx, base);
	// 822E2008: 4BFFFE74  b 0x822e1e7c
	pc = 0x822E1E7C; continue 'dispatch;
	// 822E200C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822E2010: 419A0034  beq cr6, 0x822e2044
	if ctx.cr[6].eq {
	pc = 0x822E2044; continue 'dispatch;
	}
	// 822E2014: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E2018: 48EC7971  bl 0x831a9988
	ctx.lr = 0x822E201C;
	sub_831A9988(ctx, base);
	// 822E201C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E2020: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E2024: 386B39EC  addi r3, r11, 0x39ec
	ctx.r[3].s64 = ctx.r[11].s64 + 14828;
	// 822E2028: 48EC60D1  bl 0x831a80f8
	ctx.lr = 0x822E202C;
	sub_831A80F8(ctx, base);
	// 822E202C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E2030: 41820014  beq 0x822e2044
	if ctx.cr[0].eq {
	pc = 0x822E2044; continue 'dispatch;
	}
	// 822E2034: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E2038: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 822E203C: 4BFFEEFD  bl 0x822e0f38
	ctx.lr = 0x822E2040;
	sub_822E0F38(ctx, base);
	// 822E2040: 4BFFFE3C  b 0x822e1e7c
	pc = 0x822E1E7C; continue 'dispatch;
	// 822E2044: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822E2048: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E204C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E2050: 4822F549  bl 0x82511598
	ctx.lr = 0x822E2054;
	sub_82511598(ctx, base);
	// 822E2054: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E2058: 48EC6160  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E2060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E2060 size=580
    let mut pc: u32 = 0x822E2060;
    'dispatch: loop {
        match pc {
            0x822E2060 => {
    //   block [0x822E2060..0x822E22A4)
	// 822E2060: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E2064: 48EC60E1  bl 0x831a8144
	ctx.lr = 0x822E2068;
	sub_831A8130(ctx, base);
	// 822E2068: DBE1FF88  stfd f31, -0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.f[31].u64 ) };
	// 822E206C: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E2070: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E2074: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822E2078: 7C952378  mr r21, r4
	ctx.r[21].u64 = ctx.r[4].u64;
	// 822E207C: 7CB42B78  mr r20, r5
	ctx.r[20].u64 = ctx.r[5].u64;
	// 822E2080: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 822E2084: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 822E2088: 7D134378  mr r19, r8
	ctx.r[19].u64 = ctx.r[8].u64;
	// 822E208C: 4822F065  bl 0x825110f0
	ctx.lr = 0x822E2090;
	sub_825110F0(ctx, base);
	// 822E2090: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E2094: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E2098: 396BA9AC  addi r11, r11, -0x5654
	ctx.r[11].s64 = ctx.r[11].s64 + -22100;
	// 822E209C: 394AA998  addi r10, r10, -0x5668
	ctx.r[10].s64 = ctx.r[10].s64 + -22120;
	// 822E20A0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822E20A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E20A8: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 822E20AC: 3B000010  li r24, 0x10
	ctx.r[24].s64 = 16;
	// 822E20B0: 93DF00C0  stw r30, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 822E20B4: 3B200020  li r25, 0x20
	ctx.r[25].s64 = 32;
	// 822E20B8: 93DF00C4  stw r30, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[30].u32 ) };
	// 822E20BC: 3B400030  li r26, 0x30
	ctx.r[26].s64 = 48;
	// 822E20C0: 93DF00CC  stw r30, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[30].u32 ) };
	// 822E20C4: 3B9F00E0  addi r28, r31, 0xe0
	ctx.r[28].s64 = ctx.r[31].s64 + 224;
	// 822E20C8: 93DF00D0  stw r30, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[30].u32 ) };
	// 822E20CC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822E20D0: 93DF00D4  stw r30, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[30].u32 ) };
	// 822E20D4: 3AFF0168  addi r23, r31, 0x168
	ctx.r[23].s64 = ctx.r[31].s64 + 360;
	// 822E20D8: 93DF00D8  stw r30, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[30].u32 ) };
	// 822E20DC: 3BBF00C0  addi r29, r31, 0xc0
	ctx.r[29].s64 = ctx.r[31].s64 + 192;
	// 822E20E0: 937F00DC  stw r27, 0xdc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(220 as u32), ctx.r[27].u32 ) };
	// 822E20E4: 13F89C07  vcmpneb. (lvlx128) v31, v24, v19
	tmp.u32 = ctx.r[24].u32 + ctx.r[19].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E20E8: 13D99C07  vcmpneb. (lvlx128) v30, v25, v19
	tmp.u32 = ctx.r[25].u32 + ctx.r[19].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E20EC: 3B7F00CC  addi r27, r31, 0xcc
	ctx.r[27].s64 = ctx.r[31].s64 + 204;
	// 822E20F0: 13BA9C07  vcmpneb. (lvlx128) v29, v26, v19
	tmp.u32 = ctx.r[26].u32 + ctx.r[19].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E20F4: 13809C07  vcmpneb. (lvlx128) v28, v0, v19
	tmp.u32 = ctx.r[19].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E22A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E22A8 size=332
    let mut pc: u32 = 0x822E22A8;
    'dispatch: loop {
        match pc {
            0x822E22A8 => {
    //   block [0x822E22A8..0x822E23F4)
	// 822E22A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E22AC: 48EC5EC1  bl 0x831a816c
	ctx.lr = 0x822E22B0;
	sub_831A8130(ctx, base);
	// 822E22B0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E22B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E22B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E22BC: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 822E22C0: 409A0008  bne cr6, 0x822e22c8
	if !ctx.cr[6].eq {
	pc = 0x822E22C8; continue 'dispatch;
	}
	// 822E22C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E22C8: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 822E22CC: 482264D5  bl 0x825087a0
	ctx.lr = 0x822E22D0;
	sub_825087A0(ctx, base);
	// 822E22D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E22D4: 809F00CC  lwz r4, 0xcc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E22D8: 4BFEEAF1  bl 0x822d0dc8
	ctx.lr = 0x822E22DC;
	sub_822D0DC8(ctx, base);
	// 822E22DC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E22E0: 3D408325  lis r10, -0x7cdb
	ctx.r[10].s64 = -2094727168;
	// 822E22E4: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E22E8: 38CB2D50  addi r6, r11, 0x2d50
	ctx.r[6].s64 = ctx.r[11].s64 + 11600;
	// 822E22EC: 38AA2D24  addi r5, r10, 0x2d24
	ctx.r[5].s64 = ctx.r[10].s64 + 11556;
	// 822E22F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 822E22F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E22F8: 48EC7C51  bl 0x831a9f48
	ctx.lr = 0x822E22FC;
	sub_831A9F48(ctx, base);
	// 822E22FC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822E2300: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E2304: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E2308: 419A000C  beq cr6, 0x822e2314
	if ctx.cr[6].eq {
	pc = 0x822E2314; continue 'dispatch;
	}
	// 822E230C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 822E2310: 4BFDE581  bl 0x822c0890
	ctx.lr = 0x822E2314;
	sub_822C0890(ctx, base);
	// 822E2314: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E2318: 83BF00DC  lwz r29, 0xdc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(220 as u32) ) } as u64;
	// 822E231C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822E2320: 4822D1F9  bl 0x8250f518
	ctx.lr = 0x822E2324;
	sub_8250F518(ctx, base);
	// 822E2324: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E2328: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 822E232C: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 822E2330: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822E2334: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 822E2338: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 822E233C: C02BA9F4  lfs f1, -0x560c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-22028 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822E2340: 4BFFBF89  bl 0x822de2c8
	ctx.lr = 0x822E2344;
	sub_822DE2C8(ctx, base);
	// 822E2344: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E2348: 3BDF00D4  addi r30, r31, 0xd4
	ctx.r[30].s64 = ctx.r[31].s64 + 212;
	// 822E234C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 822E2350: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 822E2354: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2358: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 822E235C: 4BFE2105  bl 0x822c4460
	ctx.lr = 0x822E2360;
	sub_822C4460(ctx, base);
	// 822E2360: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 822E2364: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E2368: 419A0008  beq cr6, 0x822e2370
	if ctx.cr[6].eq {
	pc = 0x822E2370; continue 'dispatch;
	}
	// 822E236C: 4BFDE525  bl 0x822c0890
	ctx.lr = 0x822E2370;
	sub_822C0890(ctx, base);
	// 822E2370: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822E2374: 48B0F91D  bl 0x82df1c90
	ctx.lr = 0x822E2378;
	sub_82DF1C90(ctx, base);
	// 822E2378: 817F0168  lwz r11, 0x168(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) } as u64;
	// 822E237C: 3BBF0168  addi r29, r31, 0x168
	ctx.r[29].s64 = ctx.r[31].s64 + 360;
	// 822E2380: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E2384: 419A0068  beq cr6, 0x822e23ec
	if ctx.cr[6].eq {
	pc = 0x822E23EC; continue 'dispatch;
	}
	// 822E2388: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E238C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2390: 48007D89  bl 0x822ea118
	ctx.lr = 0x822E2394;
	sub_822EA118(ctx, base);
	// 822E2394: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E2398: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E239C: 388BA914  addi r4, r11, -0x56ec
	ctx.r[4].s64 = ctx.r[11].s64 + -22252;
	// 822E23A0: 38A00087  li r5, 0x87
	ctx.r[5].s64 = 135;
	// 822E23A4: 3860000C  li r3, 0xc
	ctx.r[3].s64 = 12;
	// 822E23A8: 48B10041  bl 0x82df23e8
	ctx.lr = 0x822E23AC;
	sub_82DF23E8(ctx, base);
	// 822E23AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E23B0: 41820024  beq 0x822e23d4
	if ctx.cr[0].eq {
	pc = 0x822E23D4; continue 'dispatch;
	}
	// 822E23B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E23B8: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 822E23BC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822E23C0: 396BA98C  addi r11, r11, -0x5674
	ctx.r[11].s64 = ctx.r[11].s64 + -22132;
	// 822E23C4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822E23C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E23CC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E23D0: 48000008  b 0x822e23d8
	pc = 0x822E23D8; continue 'dispatch;
	// 822E23D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E23D8: 387F0170  addi r3, r31, 0x170
	ctx.r[3].s64 = ctx.r[31].s64 + 368;
	// 822E23DC: 4BFFF9E5  bl 0x822e1dc0
	ctx.lr = 0x822E23E0;
	sub_822E1DC0(ctx, base);
	// 822E23E0: 809F0170  lwz r4, 0x170(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 822E23E4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E23E8: 48B306D9  bl 0x82e12ac0
	ctx.lr = 0x822E23EC;
	sub_82E12AC0(ctx, base);
	// 822E23EC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 822E23F0: 48EC5DCC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E23F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E23F8 size=104
    let mut pc: u32 = 0x822E23F8;
    'dispatch: loop {
        match pc {
            0x822E23F8 => {
    //   block [0x822E23F8..0x822E2460)
	// 822E23F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E23FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E2400: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E2404: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E2408: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E240C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E2410: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E2414: 808B00C0  lwz r4, 0xc0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(192 as u32) ) } as u64;
	// 822E2418: 80AB00C8  lwz r5, 0xc8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(200 as u32) ) } as u64;
	// 822E241C: 4BFED41D  bl 0x822cf838
	ctx.lr = 0x822E2420;
	sub_822CF838(ctx, base);
	// 822E2420: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E2424: 395F0018  addi r10, r31, 0x18
	ctx.r[10].s64 = ctx.r[31].s64 + 24;
	// 822E2428: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 822E242C: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 822E2430: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2434: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 822E2438: 4BFE2029  bl 0x822c4460
	ctx.lr = 0x822E243C;
	sub_822C4460(ctx, base);
	// 822E243C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822E2440: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E2444: 419A0008  beq cr6, 0x822e244c
	if ctx.cr[6].eq {
	pc = 0x822E244C; continue 'dispatch;
	}
	// 822E2448: 4BFDE449  bl 0x822c0890
	ctx.lr = 0x822E244C;
	sub_822C0890(ctx, base);
	// 822E244C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E2450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E2454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E2458: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E245C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E2460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E2460 size=88
    let mut pc: u32 = 0x822E2460;
    'dispatch: loop {
        match pc {
            0x822E2460 => {
    //   block [0x822E2460..0x822E24B8)
	// 822E2460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E2464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E2468: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E246C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E2470: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E2474: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E2478: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E247C: 397F00C0  addi r11, r31, 0xc0
	ctx.r[11].s64 = ctx.r[31].s64 + 192;
	// 822E2480: 395E0018  addi r10, r30, 0x18
	ctx.r[10].s64 = ctx.r[30].s64 + 24;
	// 822E2484: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 822E2488: 813F00C0  lwz r9, 0xc0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 822E248C: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 822E2490: 913E0018  stw r9, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[9].u32 ) };
	// 822E2494: 4BFE1FCD  bl 0x822c4460
	ctx.lr = 0x822E2498;
	sub_822C4460(ctx, base);
	// 822E2498: 817F00C8  lwz r11, 0xc8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 822E249C: 917E0020  stw r11, 0x20(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 822E24A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E24A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E24A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E24AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E24B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E24B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E24B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822E24B8 size=60
    let mut pc: u32 = 0x822E24B8;
    'dispatch: loop {
        match pc {
            0x822E24B8 => {
    //   block [0x822E24B8..0x822E24F4)
	// 822E24B8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 822E24BC: 13E02407  vcmpneb. (lvlx128) v31, v0, v4
	tmp.u32 = ctx.r[4].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E24C0: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 822E24C4: 39400030  li r10, 0x30
	ctx.r[10].s64 = 48;
	// 822E24C8: 396300E0  addi r11, r3, 0xe0
	ctx.r[11].s64 = ctx.r[3].s64 + 224;
	// 822E24CC: 13C82407  vcmpneb. (lvlx128) v30, v8, v4
	tmp.u32 = ctx.r[8].u32 + ctx.r[4].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E24D0: 13A92407  vcmpneb. (lvlx128) v29, v9, v4
	tmp.u32 = ctx.r[9].u32 + ctx.r[4].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E24D4: 138A2407  vcmpneb. (lvlx128) v28, v10, v4
	tmp.u32 = ctx.r[10].u32 + ctx.r[4].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E24F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E24F4 size=8
    let mut pc: u32 = 0x822E24F4;
    'dispatch: loop {
        match pc {
            0x822E24F4 => {
    //   block [0x822E24F4..0x822E24FC)
	// 822E24F4: 4BFEE474  b 0x822d0968
	sub_822D0968(ctx, base);
	return;
	// 822E24F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E2500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E2500 size=80
    let mut pc: u32 = 0x822E2500;
    'dispatch: loop {
        match pc {
            0x822E2500 => {
    //   block [0x822E2500..0x822E2550)
	// 822E2500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E2504: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E2508: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E250C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E2510: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2514: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E2518: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E251C: 396B0064  addi r11, r11, 0x64
	ctx.r[11].s64 = ctx.r[11].s64 + 100;
	// 822E2520: 409A0008  bne cr6, 0x822e2528
	if !ctx.cr[6].eq {
	pc = 0x822E2528; continue 'dispatch;
	}
	// 822E2524: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E2528: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E252C: 38840004  addi r4, r4, 4
	ctx.r[4].s64 = ctx.r[4].s64 + 4;
	// 822E2530: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 822E2534: 4BFE1F2D  bl 0x822c4460
	ctx.lr = 0x822E2538;
	sub_822C4460(ctx, base);
	// 822E2538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E253C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E2540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E2544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E2548: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E254C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E2550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E2550 size=220
    let mut pc: u32 = 0x822E2550;
    'dispatch: loop {
        match pc {
            0x822E2550 => {
    //   block [0x822E2550..0x822E262C)
	// 822E2550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E2554: 48EC5C15  bl 0x831a8168
	ctx.lr = 0x822E2558;
	sub_831A8130(ctx, base);
	// 822E2558: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E255C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822E2560: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E2564: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E2568: 579D063F  clrlwi. r29, r28, 0x18
	ctx.r[29].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822E256C: 41820038  beq 0x822e25a4
	if ctx.cr[0].eq {
	pc = 0x822E25A4; continue 'dispatch;
	}
	// 822E2570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E2574: 48EC7415  bl 0x831a9988
	ctx.lr = 0x822E2578;
	sub_831A9988(ctx, base);
	// 822E2578: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E257C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E2580: 386B35EC  addi r3, r11, 0x35ec
	ctx.r[3].s64 = ctx.r[11].s64 + 13804;
	// 822E2584: 48EC5B75  bl 0x831a80f8
	ctx.lr = 0x822E2588;
	sub_831A80F8(ctx, base);
	// 822E2588: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E258C: 41820018  beq 0x822e25a4
	if ctx.cr[0].eq {
	pc = 0x822E25A4; continue 'dispatch;
	}
	// 822E2590: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E2594: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 822E2598: 4BFFD549  bl 0x822dfae0
	ctx.lr = 0x822E259C;
	sub_822DFAE0(ctx, base);
	// 822E259C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822E25A0: 48000084  b 0x822e2624
	pc = 0x822E2624; continue 'dispatch;
	// 822E25A4: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822E25A8: 419A006C  beq cr6, 0x822e2614
	if ctx.cr[6].eq {
	pc = 0x822E2614; continue 'dispatch;
	}
	// 822E25AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E25B0: 48EC73D9  bl 0x831a9988
	ctx.lr = 0x822E25B4;
	sub_831A9988(ctx, base);
	// 822E25B4: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E25B8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E25BC: 386B35BC  addi r3, r11, 0x35bc
	ctx.r[3].s64 = ctx.r[11].s64 + 13756;
	// 822E25C0: 48EC5B39  bl 0x831a80f8
	ctx.lr = 0x822E25C4;
	sub_831A80F8(ctx, base);
	// 822E25C4: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E25C8: 41820014  beq 0x822e25dc
	if ctx.cr[0].eq {
	pc = 0x822E25DC; continue 'dispatch;
	}
	// 822E25CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E25D0: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 822E25D4: 4BFFFE25  bl 0x822e23f8
	ctx.lr = 0x822E25D8;
	sub_822E23F8(ctx, base);
	// 822E25D8: 4BFFFFC4  b 0x822e259c
	pc = 0x822E259C; continue 'dispatch;
	// 822E25DC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822E25E0: 419A0034  beq cr6, 0x822e2614
	if ctx.cr[6].eq {
	pc = 0x822E2614; continue 'dispatch;
	}
	// 822E25E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E25E8: 48EC73A1  bl 0x831a9988
	ctx.lr = 0x822E25EC;
	sub_831A9988(ctx, base);
	// 822E25EC: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E25F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E25F4: 386B3B38  addi r3, r11, 0x3b38
	ctx.r[3].s64 = ctx.r[11].s64 + 15160;
	// 822E25F8: 48EC5B01  bl 0x831a80f8
	ctx.lr = 0x822E25FC;
	sub_831A80F8(ctx, base);
	// 822E25FC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E2600: 41820014  beq 0x822e2614
	if ctx.cr[0].eq {
	pc = 0x822E2614; continue 'dispatch;
	}
	// 822E2604: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E2608: 387EFFD8  addi r3, r30, -0x28
	ctx.r[3].s64 = ctx.r[30].s64 + -40;
	// 822E260C: 4BFFFE55  bl 0x822e2460
	ctx.lr = 0x822E2610;
	sub_822E2460(ctx, base);
	// 822E2610: 4BFFFF8C  b 0x822e259c
	pc = 0x822E259C; continue 'dispatch;
	// 822E2614: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822E2618: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E261C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E2620: 4822EF79  bl 0x82511598
	ctx.lr = 0x822E2624;
	sub_82511598(ctx, base);
	// 822E2624: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E2628: 48EC5B90  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E2630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E2630 size=180
    let mut pc: u32 = 0x822E2630;
    'dispatch: loop {
        match pc {
            0x822E2630 => {
    //   block [0x822E2630..0x822E26E4)
	// 822E2630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E2634: 48EC5B39  bl 0x831a816c
	ctx.lr = 0x822E2638;
	sub_831A8130(ctx, base);
	// 822E2638: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 822E263C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E2640: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E2644: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822E2648: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E264C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E2650: 4822EAA1  bl 0x825110f0
	ctx.lr = 0x822E2654;
	sub_825110F0(ctx, base);
	// 822E2654: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E2658: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E265C: 392BAA10  addi r9, r11, -0x55f0
	ctx.r[9].s64 = ctx.r[11].s64 + -22000;
	// 822E2660: 394AA9FC  addi r10, r10, -0x5604
	ctx.r[10].s64 = ctx.r[10].s64 + -22020;
	// 822E2664: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E2668: 3D208338  lis r9, -0x7cc8
	ctx.r[9].s64 = -2093481984;
	// 822E266C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E2670: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 822E2674: 38C96880  addi r6, r9, 0x6880
	ctx.r[6].s64 = ctx.r[9].s64 + 26752;
	// 822E2678: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 822E267C: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 822E2680: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 822E2684: 917F00C4  stw r11, 0xc4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 822E2688: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 822E268C: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 822E2690: 395F00E0  addi r10, r31, 0xe0
	ctx.r[10].s64 = ctx.r[31].s64 + 224;
	// 822E2694: 917F00D0  stw r11, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[11].u32 ) };
	// 822E2698: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 822E269C: 387F012C  addi r3, r31, 0x12c
	ctx.r[3].s64 = ctx.r[31].s64 + 300;
	// 822E26A0: 917F00D8  stw r11, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 822E26A4: 13E73407  vcmpneb. (lvlx128) v31, v7, v6
	tmp.u32 = ctx.r[7].u32 + ctx.r[6].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E26A8: 13C83407  vcmpneb. (lvlx128) v30, v8, v6
	tmp.u32 = ctx.r[8].u32 + ctx.r[6].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E26AC: 13A93407  vcmpneb. (lvlx128) v29, v9, v6
	tmp.u32 = ctx.r[9].u32 + ctx.r[6].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E26B0: 13803407  vcmpneb. (lvlx128) v28, v0, v6
	tmp.u32 = ctx.r[6].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E26E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E26E8 size=8
    let mut pc: u32 = 0x822E26E8;
    'dispatch: loop {
        match pc {
            0x822E26E8 => {
    //   block [0x822E26E8..0x822E26F0)
	// 822E26E8: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 822E26EC: 48000004  b 0x822e26f0
	sub_822E26F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E26F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E26F0 size=76
    let mut pc: u32 = 0x822E26F0;
    'dispatch: loop {
        match pc {
            0x822E26F0 => {
    //   block [0x822E26F0..0x822E273C)
	// 822E26F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E26F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E26F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E26FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E2700: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E2704: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E2708: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E270C: 4BFFCF8D  bl 0x822df698
	ctx.lr = 0x822E2710;
	sub_822DF698(ctx, base);
	// 822E2710: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E2714: 4182000C  beq 0x822e2720
	if ctx.cr[0].eq {
	pc = 0x822E2720; continue 'dispatch;
	}
	// 822E2718: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E271C: 48B0FCBD  bl 0x82df23d8
	ctx.lr = 0x822E2720;
	sub_82DF23D8(ctx, base);
	// 822E2720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E2724: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E2728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E272C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E2730: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E2734: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E2738: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E2740(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E2740 size=540
    let mut pc: u32 = 0x822E2740;
    'dispatch: loop {
        match pc {
            0x822E2740 => {
    //   block [0x822E2740..0x822E295C)
	// 822E2740: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E2744: 48EC5A25  bl 0x831a8168
	ctx.lr = 0x822E2748;
	sub_831A8130(ctx, base);
	// 822E2748: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 822E274C: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E2750: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E2754: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E2758: 389F0028  addi r4, r31, 0x28
	ctx.r[4].s64 = ctx.r[31].s64 + 40;
	// 822E275C: 409A0008  bne cr6, 0x822e2764
	if !ctx.cr[6].eq {
	pc = 0x822E2764; continue 'dispatch;
	}
	// 822E2760: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E2764: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 822E2768: 48226039  bl 0x825087a0
	ctx.lr = 0x822E276C;
	sub_825087A0(ctx, base);
	// 822E276C: 817F0124  lwz r11, 0x124(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(292 as u32) ) } as u64;
	// 822E2770: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 822E2774: 4198014C  blt cr6, 0x822e28c0
	if ctx.cr[6].lt {
	pc = 0x822E28C0; continue 'dispatch;
	}
	// 822E2778: 419A00CC  beq cr6, 0x822e2844
	if ctx.cr[6].eq {
	pc = 0x822E2844; continue 'dispatch;
	}
	// 822E277C: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 822E2780: 409801D0  bge cr6, 0x822e2950
	if !ctx.cr[6].lt {
	pc = 0x822E2950; continue 'dispatch;
	}
	// 822E2784: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E2788: 83BF00CC  lwz r29, 0xcc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E278C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822E2790: C3FF0128  lfs f31, 0x128(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 822E2794: 839F0120  lwz r28, 0x120(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 822E2798: 3BDF00D4  addi r30, r31, 0xd4
	ctx.r[30].s64 = ctx.r[31].s64 + 212;
	// 822E279C: 4822CD7D  bl 0x8250f518
	ctx.lr = 0x822E27A0;
	sub_8250F518(ctx, base);
	// 822E27A0: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 822E27A4: 38BF0028  addi r5, r31, 0x28
	ctx.r[5].s64 = ctx.r[31].s64 + 40;
	// 822E27A8: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822E27AC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E27B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E27B4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 822E27B8: 4BFEE711  bl 0x822d0ec8
	ctx.lr = 0x822E27BC;
	sub_822D0EC8(ctx, base);
	// 822E27BC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E27C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E27C4: 4BFFFD3D  bl 0x822e2500
	ctx.lr = 0x822E27C8;
	sub_822E2500(ctx, base);
	// 822E27C8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822E27CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E27D0: 419A0008  beq cr6, 0x822e27d8
	if ctx.cr[6].eq {
	pc = 0x822E27D8; continue 'dispatch;
	}
	// 822E27D4: 4BFDE0BD  bl 0x822c0890
	ctx.lr = 0x822E27D8;
	sub_822C0890(ctx, base);
	// 822E27D8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822E27DC: 48B0F4B5  bl 0x82df1c90
	ctx.lr = 0x822E27E0;
	sub_82DF1C90(ctx, base);
	// 822E27E0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E27E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E27E8: 386BFF9C  addi r3, r11, -0x64
	ctx.r[3].s64 = ctx.r[11].s64 + -100;
	// 822E27EC: 409A0008  bne cr6, 0x822e27f4
	if !ctx.cr[6].eq {
	pc = 0x822E27F4; continue 'dispatch;
	}
	// 822E27F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822E27F4: 389F012C  addi r4, r31, 0x12c
	ctx.r[4].s64 = ctx.r[31].s64 + 300;
	// 822E27F8: 48004421  bl 0x822e6c18
	ctx.lr = 0x822E27FC;
	sub_822E6C18(ctx, base);
	// 822E27FC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2800: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E2804: 3BCBFF9C  addi r30, r11, -0x64
	ctx.r[30].s64 = ctx.r[11].s64 + -100;
	// 822E2808: 409A0008  bne cr6, 0x822e2810
	if !ctx.cr[6].eq {
	pc = 0x822E2810; continue 'dispatch;
	}
	// 822E280C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822E2810: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E2814: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822E2818: 4822CD01  bl 0x8250f518
	ctx.lr = 0x822E281C;
	sub_8250F518(ctx, base);
	// 822E281C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2820: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E2824: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 822E2828: 409A0008  bne cr6, 0x822e2830
	if !ctx.cr[6].eq {
	pc = 0x822E2830; continue 'dispatch;
	}
	// 822E282C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822E2830: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E2834: 4824587D  bl 0x825280b0
	ctx.lr = 0x822E2838;
	sub_825280B0(ctx, base);
	// 822E2838: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822E283C: 48B0F455  bl 0x82df1c90
	ctx.lr = 0x822E2840;
	sub_82DF1C90(ctx, base);
	// 822E2840: 48000110  b 0x822e2950
	pc = 0x822E2950; continue 'dispatch;
	// 822E2844: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E2848: 83BF00CC  lwz r29, 0xcc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E284C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822E2850: C3FF0128  lfs f31, 0x128(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 822E2854: 839F0120  lwz r28, 0x120(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 822E2858: 3BDF00D4  addi r30, r31, 0xd4
	ctx.r[30].s64 = ctx.r[31].s64 + 212;
	// 822E285C: 4822CCBD  bl 0x8250f518
	ctx.lr = 0x822E2860;
	sub_8250F518(ctx, base);
	// 822E2860: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 822E2864: 38BF0028  addi r5, r31, 0x28
	ctx.r[5].s64 = ctx.r[31].s64 + 40;
	// 822E2868: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822E286C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E2870: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 822E2874: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 822E2878: 4BFEE651  bl 0x822d0ec8
	ctx.lr = 0x822E287C;
	sub_822D0EC8(ctx, base);
	// 822E287C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E2880: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E2884: 4BFFFC7D  bl 0x822e2500
	ctx.lr = 0x822E2888;
	sub_822E2500(ctx, base);
	// 822E2888: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 822E288C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E2890: 419A0008  beq cr6, 0x822e2898
	if ctx.cr[6].eq {
	pc = 0x822E2898; continue 'dispatch;
	}
	// 822E2894: 4BFDDFFD  bl 0x822c0890
	ctx.lr = 0x822E2898;
	sub_822C0890(ctx, base);
	// 822E2898: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822E289C: 48B0F3F5  bl 0x82df1c90
	ctx.lr = 0x822E28A0;
	sub_82DF1C90(ctx, base);
	// 822E28A0: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E28A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E28A8: 386BFF9C  addi r3, r11, -0x64
	ctx.r[3].s64 = ctx.r[11].s64 + -100;
	// 822E28AC: 409A0008  bne cr6, 0x822e28b4
	if !ctx.cr[6].eq {
	pc = 0x822E28B4; continue 'dispatch;
	}
	// 822E28B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822E28B4: 389F012C  addi r4, r31, 0x12c
	ctx.r[4].s64 = ctx.r[31].s64 + 300;
	// 822E28B8: 48004361  bl 0x822e6c18
	ctx.lr = 0x822E28BC;
	sub_822E6C18(ctx, base);
	// 822E28BC: 48000094  b 0x822e2950
	pc = 0x822E2950; continue 'dispatch;
	// 822E28C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E28C4: 83BF00CC  lwz r29, 0xcc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 822E28C8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822E28CC: C3FF0128  lfs f31, 0x128(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(296 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 822E28D0: 839F0120  lwz r28, 0x120(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(288 as u32) ) } as u64;
	// 822E28D4: 3BDF00D4  addi r30, r31, 0xd4
	ctx.r[30].s64 = ctx.r[31].s64 + 212;
	// 822E28D8: 4822CC41  bl 0x8250f518
	ctx.lr = 0x822E28DC;
	sub_8250F518(ctx, base);
	// 822E28DC: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 822E28E0: 38BF0028  addi r5, r31, 0x28
	ctx.r[5].s64 = ctx.r[31].s64 + 40;
	// 822E28E4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822E28E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E28EC: 38610078  addi r3, r1, 0x78
	ctx.r[3].s64 = ctx.r[1].s64 + 120;
	// 822E28F0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 822E28F4: 4BFEE515  bl 0x822d0e08
	ctx.lr = 0x822E28F8;
	sub_822D0E08(ctx, base);
	// 822E28F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E28FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E2900: 4BFFC041  bl 0x822de940
	ctx.lr = 0x822E2904;
	sub_822DE940(ctx, base);
	// 822E2904: 8061007C  lwz r3, 0x7c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(124 as u32) ) } as u64;
	// 822E2908: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E290C: 419A0008  beq cr6, 0x822e2914
	if ctx.cr[6].eq {
	pc = 0x822E2914; continue 'dispatch;
	}
	// 822E2910: 4BFDDF81  bl 0x822c0890
	ctx.lr = 0x822E2914;
	sub_822C0890(ctx, base);
	// 822E2914: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822E2918: 48B0F379  bl 0x82df1c90
	ctx.lr = 0x822E291C;
	sub_82DF1C90(ctx, base);
	// 822E291C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2920: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E2924: 386BFEF0  addi r3, r11, -0x110
	ctx.r[3].s64 = ctx.r[11].s64 + -272;
	// 822E2928: 409A0008  bne cr6, 0x822e2930
	if !ctx.cr[6].eq {
	pc = 0x822E2930; continue 'dispatch;
	}
	// 822E292C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822E2930: 389F012C  addi r4, r31, 0x12c
	ctx.r[4].s64 = ctx.r[31].s64 + 300;
	// 822E2934: 480042E5  bl 0x822e6c18
	ctx.lr = 0x822E2938;
	sub_822E6C18(ctx, base);
	// 822E2938: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E293C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E2940: 386BFEF0  addi r3, r11, -0x110
	ctx.r[3].s64 = ctx.r[11].s64 + -272;
	// 822E2944: 409A0008  bne cr6, 0x822e294c
	if !ctx.cr[6].eq {
	pc = 0x822E294C; continue 'dispatch;
	}
	// 822E2948: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822E294C: 48004435  bl 0x822e6d80
	ctx.lr = 0x822E2950;
	sub_822E6D80(ctx, base);
	// 822E2950: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 822E2954: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 822E2958: 48EC5860  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E2960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E2960 size=384
    let mut pc: u32 = 0x822E2960;
    'dispatch: loop {
        match pc {
            0x822E2960 => {
    //   block [0x822E2960..0x822E2AE0)
	// 822E2960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E2964: 48EC5809  bl 0x831a816c
	ctx.lr = 0x822E2968;
	sub_831A8130(ctx, base);
	// 822E2968: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E296C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E2970: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2974: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E2978: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 822E297C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 822E2980: 4BFEF401  bl 0x822d1d80
	ctx.lr = 0x822E2984;
	sub_822D1D80(ctx, base);
	// 822E2984: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E2988: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822E298C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822E2990: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822E2994: 4BFEF575  bl 0x822d1f08
	ctx.lr = 0x822E2998;
	sub_822D1F08(ctx, base);
	// 822E2998: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E299C: 3BDF00C0  addi r30, r31, 0xc0
	ctx.r[30].s64 = ctx.r[31].s64 + 192;
	// 822E29A0: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 822E29A4: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 822E29A8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E29AC: 917F00C0  stw r11, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 822E29B0: 4BFE1AB1  bl 0x822c4460
	ctx.lr = 0x822E29B4;
	sub_822C4460(ctx, base);
	// 822E29B4: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 822E29B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E29BC: 419A0008  beq cr6, 0x822e29c4
	if ctx.cr[6].eq {
	pc = 0x822E29C4; continue 'dispatch;
	}
	// 822E29C0: 4BFDDED1  bl 0x822c0890
	ctx.lr = 0x822E29C4;
	sub_822C0890(ctx, base);
	// 822E29C4: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E29C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E29CC: 409A0018  bne cr6, 0x822e29e4
	if !ctx.cr[6].eq {
	pc = 0x822E29E4; continue 'dispatch;
	}
	// 822E29D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E29D4: 4BFEF3C5  bl 0x822d1d98
	ctx.lr = 0x822E29D8;
	sub_822D1D98(ctx, base);
	// 822E29D8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822E29DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 822E29E0: 48EC57DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 822E29E4: 48B1BDFD  bl 0x82dfe7e0
	ctx.lr = 0x822E29E8;
	sub_82DFE7E0(ctx, base);
	// 822E29E8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E29EC: 40820064  bne 0x822e2a50
	if !ctx.cr[0].eq {
	pc = 0x822E2A50; continue 'dispatch;
	}
	// 822E29F0: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E29F4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E29F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E29FC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 822E2A00: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 822E2A04: 419A0024  beq cr6, 0x822e2a28
	if ctx.cr[6].eq {
	pc = 0x822E2A28; continue 'dispatch;
	}
	// 822E2A08: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822E2A0C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 822E2A10: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822E2A14: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 822E2A18: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822E2A1C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 822E2A20: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822E2A24: 4082FFE8  bne 0x822e2a0c
	if !ctx.cr[0].eq {
	pc = 0x822E2A0C; continue 'dispatch;
	}
	// 822E2A28: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 822E2A2C: 4820805D  bl 0x824eaa88
	ctx.lr = 0x822E2A30;
	sub_824EAA88(ctx, base);
	// 822E2A30: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E2A34: 38C00042  li r6, 0x42
	ctx.r[6].s64 = 66;
	// 822E2A38: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2A3C: 38ABAA38  addi r5, r11, -0x55c8
	ctx.r[5].s64 = ctx.r[11].s64 + -21960;
	// 822E2A40: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 822E2A44: 488D773D  bl 0x82bba180
	ctx.lr = 0x822E2A48;
	sub_82BBA180(ctx, base);
	// 822E2A48: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 822E2A4C: 48B0F245  bl 0x82df1c90
	ctx.lr = 0x822E2A50;
	sub_82DF1C90(ctx, base);
	// 822E2A50: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E2A54: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2A58: 4BFEC949  bl 0x822cf3a0
	ctx.lr = 0x822E2A5C;
	sub_822CF3A0(ctx, base);
	// 822E2A5C: 907F00C8  stw r3, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[3].u32 ) };
	// 822E2A60: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E2A64: 387F012C  addi r3, r31, 0x12c
	ctx.r[3].s64 = ctx.r[31].s64 + 300;
	// 822E2A68: 48B11169  bl 0x82df3bd0
	ctx.lr = 0x822E2A6C;
	sub_82DF3BD0(ctx, base);
	// 822E2A6C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822E2A70: 80BF00C8  lwz r5, 0xc8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(200 as u32) ) } as u64;
	// 822E2A74: 816B16D8  lwz r11, 0x16d8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5848 as u32) ) } as u64;
	// 822E2A78: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E2A7C: 409A0018  bne cr6, 0x822e2a94
	if !ctx.cr[6].eq {
	pc = 0x822E2A94; continue 'dispatch;
	}
	// 822E2A80: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822E2A84: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E2A88: 4BFEF311  bl 0x822d1d98
	ctx.lr = 0x822E2A8C;
	sub_822D1D98(ctx, base);
	// 822E2A8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E2A90: 4BFFFF4C  b 0x822e29dc
	pc = 0x822E29DC; continue 'dispatch;
	// 822E2A94: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822E2A98: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2A9C: 4BFECD9D  bl 0x822cf838
	ctx.lr = 0x822E2AA0;
	sub_822CF838(ctx, base);
	// 822E2AA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E2AA4: 3BDF00CC  addi r30, r31, 0xcc
	ctx.r[30].s64 = ctx.r[31].s64 + 204;
	// 822E2AA8: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 822E2AAC: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 822E2AB0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2AB4: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 822E2AB8: 4BFE19A9  bl 0x822c4460
	ctx.lr = 0x822E2ABC;
	sub_822C4460(ctx, base);
	// 822E2ABC: 80610074  lwz r3, 0x74(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 822E2AC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E2AC4: 419A0008  beq cr6, 0x822e2acc
	if ctx.cr[6].eq {
	pc = 0x822E2ACC; continue 'dispatch;
	}
	// 822E2AC8: 4BFDDDC9  bl 0x822c0890
	ctx.lr = 0x822E2ACC;
	sub_822C0890(ctx, base);
	// 822E2ACC: 389F00E0  addi r4, r31, 0xe0
	ctx.r[4].s64 = ctx.r[31].s64 + 224;
	// 822E2AD0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2AD4: 4BFEDE95  bl 0x822d0968
	ctx.lr = 0x822E2AD8;
	sub_822D0968(ctx, base);
	// 822E2AD8: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 822E2ADC: 4BFFFFA8  b 0x822e2a84
	pc = 0x822E2A84; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E2AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E2AE0 size=488
    let mut pc: u32 = 0x822E2AE0;
    'dispatch: loop {
        match pc {
            0x822E2AE0 => {
    //   block [0x822E2AE0..0x822E2CC8)
	// 822E2AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E2AE4: 48EC5679  bl 0x831a815c
	ctx.lr = 0x822E2AE8;
	sub_831A8130(ctx, base);
	// 822E2AE8: 3981FFC0  addi r12, r1, -0x40
	ctx.r[12].s64 = ctx.r[1].s64 + -64;
	// 822E2AEC: 48EC5F89  bl 0x831a8a74
	ctx.lr = 0x822E2AF0;
	sub_831A8A40(ctx, base);
	// 822E2AF0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E2AF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E2AF8: FFC00890  fmr f30, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].f64 = ctx.f[1].f64;
	// 822E2AFC: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 822E2B00: FFE01090  fmr f31, f2
	ctx.f[31].f64 = ctx.f[2].f64;
	// 822E2B04: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 822E2B08: FFA01890  fmr f29, f3
	ctx.f[29].f64 = ctx.f[3].f64;
	// 822E2B0C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 822E2B10: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 822E2B14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2B18: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E2B1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E2B20: 4E800421  bctrl
	ctx.lr = 0x822E2B24;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E2B24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2B28: FF800890  fmr f28, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[28].f64 = ctx.f[1].f64;
	// 822E2B2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E2B30: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 822E2B34: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 822E2B38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E2B3C: 4E800421  bctrl
	ctx.lr = 0x822E2B40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E2B40: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2B44: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822E2B48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E2B4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E2B50: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 822E2B54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E2B58: 4E800421  bctrl
	ctx.lr = 0x822E2B5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E2B5C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822E2B60: C3CB08A4  lfs f30, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 822E2B64: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 822E2B68: 40980020  bge cr6, 0x822e2b88
	if !ctx.cr[6].lt {
	pc = 0x822E2B88; continue 'dispatch;
	}
	// 822E2B6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2B70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E2B74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E2B78: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 822E2B7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E2B80: 4E800421  bctrl
	ctx.lr = 0x822E2B84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E2B84: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822E2B88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2B8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E2B90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E2B94: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 822E2B98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E2B9C: 4E800421  bctrl
	ctx.lr = 0x822E2BA0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E2BA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2BA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E2BA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E2BAC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 822E2BB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E2BB4: 4E800421  bctrl
	ctx.lr = 0x822E2BB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E2BB8: FF01F800  fcmpu cr6, f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[31].f64);
	// 822E2BBC: 40980008  bge cr6, 0x822e2bc4
	if !ctx.cr[6].lt {
	pc = 0x822E2BC4; continue 'dispatch;
	}
	// 822E2BC0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822E2BC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2BC8: FFDFF7EE  fsel f30, f31, f31, f30
	ctx.f[30].f64 = if ctx.f[31].f64 >= 0.0 { ctx.f[31].f64 } else { ctx.f[30].f64 };
	// 822E2BCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E2BD0: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E2BD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E2BD8: 4E800421  bctrl
	ctx.lr = 0x822E2BDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E2BDC: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822E2BE0: 480000B8  b 0x822e2c98
	pc = 0x822E2C98; continue 'dispatch;
	// 822E2BE4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2BE8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822E2BEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E2BF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E2BF4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 822E2BF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E2BFC: 4E800421  bctrl
	ctx.lr = 0x822E2C00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E2C00: 817B0000  lwz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2C04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E2C08: 3BABFF34  addi r29, r11, -0xcc
	ctx.r[29].s64 = ctx.r[11].s64 + -204;
	// 822E2C0C: 409A0008  bne cr6, 0x822e2c14
	if !ctx.cr[6].eq {
	pc = 0x822E2C14; continue 'dispatch;
	}
	// 822E2C10: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 822E2C14: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822E2C18: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E2C1C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822E2C20: 48B990A9  bl 0x82e7bcc8
	ctx.lr = 0x822E2C24;
	sub_82E7BCC8(ctx, base);
	// 822E2C24: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 822E2C28: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822E2C2C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E2C30: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822E2C34: 48B99095  bl 0x82e7bcc8
	ctx.lr = 0x822E2C38;
	sub_82E7BCC8(ctx, base);
	// 822E2C38: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E2C3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E2C40: 7F25CB78  mr r5, r25
	ctx.r[5].u64 = ctx.r[25].u64;
	// 822E2C44: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 822E2C48: 4BFE4D59  bl 0x822c79a0
	ctx.lr = 0x822E2C4C;
	sub_822C79A0(ctx, base);
	// 822E2C4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2C50: FC20E890  fmr f1, f29
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[29].f64;
	// 822E2C54: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E2C58: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822E2C5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E2C60: 4E800421  bctrl
	ctx.lr = 0x822E2C64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E2C64: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2C68: FF60F890  fmr f27, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[27].f64 = ctx.f[31].f64;
	// 822E2C6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E2C70: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E2C74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E2C78: 4E800421  bctrl
	ctx.lr = 0x822E2C7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E2C7C: FFE00890  fmr f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822E2C80: FF1BF800  fcmpu cr6, f27, f31
	ctx.cr[6].compare_f64(ctx.f[27].f64, ctx.f[31].f64);
	// 822E2C84: 4199001C  bgt cr6, 0x822e2ca0
	if ctx.cr[6].gt {
	pc = 0x822E2CA0; continue 'dispatch;
	}
	// 822E2C88: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 822E2C8C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 822E2C90: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E2CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E2CC8 size=572
    let mut pc: u32 = 0x822E2CC8;
    'dispatch: loop {
        match pc {
            0x822E2CC8 => {
    //   block [0x822E2CC8..0x822E2F04)
	// 822E2CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E2CCC: 48EC549D  bl 0x831a8168
	ctx.lr = 0x822E2CD0;
	sub_831A8130(ctx, base);
	// 822E2CD0: DBE1FFD0  stfd f31, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 822E2CD4: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E2CD8: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822E2CDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E2CE0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822E2CE4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822E2CE8: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 822E2CEC: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E2CF0: FF010000  fcmpu cr6, f1, f0
	ctx.cr[6].compare_f64(ctx.f[1].f64, ctx.f[0].f64);
	// 822E2CF4: 40980014  bge cr6, 0x822e2d08
	if !ctx.cr[6].lt {
	pc = 0x822E2D08; continue 'dispatch;
	}
	// 822E2CF8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2CFC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E2D00: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E2D04: 4E800421  bctrl
	ctx.lr = 0x822E2D08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E2D08: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2D0C: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 822E2D10: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 822E2D14: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 822E2D18: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E2D1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E2D20: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 822E2D24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E2D28: 4E800421  bctrl
	ctx.lr = 0x822E2D2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E2D2C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 822E2D30: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 822E2D34: 386100A0  addi r3, r1, 0xa0
	ctx.r[3].s64 = ctx.r[1].s64 + 160;
	// 822E2D38: 48B9A5E9  bl 0x82e7d320
	ctx.lr = 0x822E2D3C;
	sub_82E7D320(ctx, base);
	// 822E2D3C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E2D40: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 822E2D44: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822E2D48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E2D4C: 386100E0  addi r3, r1, 0xe0
	ctx.r[3].s64 = ctx.r[1].s64 + 224;
	// 822E2D50: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E2F08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E2F08 size=236
    let mut pc: u32 = 0x822E2F08;
    'dispatch: loop {
        match pc {
            0x822E2F08 => {
    //   block [0x822E2F08..0x822E2FF4)
	// 822E2F08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E2F0C: 48EC5259  bl 0x831a8164
	ctx.lr = 0x822E2F10;
	sub_831A8130(ctx, base);
	// 822E2F10: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 822E2F14: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 822E2F18: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E2F1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E2F20: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 822E2F24: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2F28: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E2F2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E2F30: 4E800421  bctrl
	ctx.lr = 0x822E2F34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E2F34: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E2F38: 357DFFFF  addic. r11, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E2F3C: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 822E2F40: 41810008  bgt 0x822e2f48
	if ctx.cr[0].gt {
	pc = 0x822E2F48; continue 'dispatch;
	}
	// 822E2F44: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 822E2F48: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822E2F4C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822E2F50: 40990090  ble cr6, 0x822e2fe0
	if !ctx.cr[6].gt {
	pc = 0x822E2FE0; continue 'dispatch;
	}
	// 822E2F54: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822E2F58: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E2F5C: C3CB08A4  lfs f30, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 822E2F60: C3EA9534  lfs f31, -0x6acc(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 822E2F64: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E2F68: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 822E2F6C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 822E2F70: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 822E2F74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E2F78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E2F7C: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 822E2F80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E2F84: 4E800421  bctrl
	ctx.lr = 0x822E2F88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E2F88: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822E2F8C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822E2F90: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822E2F94: 48B9A38D  bl 0x82e7d320
	ctx.lr = 0x822E2F98;
	sub_82E7D320(ctx, base);
	// 822E2F98: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 822E2F9C: 13E0D8C7  vcmpequd (lvx128) v31, v0, v27
	tmp.u32 = ctx.r[27].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E2FA0: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 822E2FA4: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 822E2FA8: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E2FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E2FF8 size=272
    let mut pc: u32 = 0x822E2FF8;
    'dispatch: loop {
        match pc {
            0x822E2FF8 => {
    //   block [0x822E2FF8..0x822E3108)
	// 822E2FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E2FFC: 48EC5169  bl 0x831a8164
	ctx.lr = 0x822E3000;
	sub_831A8130(ctx, base);
	// 822E3000: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 822E3004: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E3108 size=176
    let mut pc: u32 = 0x822E3108;
    'dispatch: loop {
        match pc {
            0x822E3108 => {
    //   block [0x822E3108..0x822E31B8)
	// 822E3108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E310C: 48EC5061  bl 0x831a816c
	ctx.lr = 0x822E3110;
	sub_831A8130(ctx, base);
	// 822E3110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E3114: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 822E3118: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E311C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E3120: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822E3124: 409A000C  bne cr6, 0x822e3130
	if !ctx.cr[6].eq {
	pc = 0x822E3130; continue 'dispatch;
	}
	// 822E3128: 13E028C7  vcmpequd (lvx128) v31, v0, v5
	tmp.u32 = ctx.r[5].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E312C: 4800007C  b 0x822e31a8
	pc = 0x822E31A8; continue 'dispatch;
	// 822E3130: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 822E3134: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E3138: 4BFFFDD1  bl 0x822e2f08
	ctx.lr = 0x822E313C;
	sub_822E2F08(ctx, base);
	// 822E313C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822E3140: 4098000C  bge cr6, 0x822e314c
	if !ctx.cr[6].lt {
	pc = 0x822E314C; continue 'dispatch;
	}
	// 822E3144: 7FE3FA14  add r31, r3, r31
	ctx.r[31].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 822E3148: 4800000C  b 0x822e3154
	pc = 0x822E3154; continue 'dispatch;
	// 822E314C: 7D63FA14  add r11, r3, r31
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[31].u64;
	// 822E3150: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 822E3154: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E3158: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E315C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E3160: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E3164: 4E800421  bctrl
	ctx.lr = 0x822E3168;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E3168: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 822E316C: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 822E3170: 41980008  blt cr6, 0x822e3178
	if ctx.cr[6].lt {
	pc = 0x822E3178; continue 'dispatch;
	}
	// 822E3174: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 822E3178: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E317C: 40980008  bge cr6, 0x822e3184
	if !ctx.cr[6].lt {
	pc = 0x822E3184; continue 'dispatch;
	}
	// 822E3180: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E3184: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E3188: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 822E318C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822E3190: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E3194: 816A0028  lwz r11, 0x28(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 822E3198: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E319C: 4E800421  bctrl
	ctx.lr = 0x822E31A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E31A0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 822E31A4: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E31B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E31B8 size=192
    let mut pc: u32 = 0x822E31B8;
    'dispatch: loop {
        match pc {
            0x822E31B8 => {
    //   block [0x822E31B8..0x822E3278)
	// 822E31B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E31BC: 48EC4F99  bl 0x831a8154
	ctx.lr = 0x822E31C0;
	sub_831A8130(ctx, base);
	// 822E31C0: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E31C4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E31C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E31CC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 822E31D0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 822E31D4: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 822E31D8: 835F0000  lwz r26, 0(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E31DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E31E0: 3B210050  addi r25, r1, 0x50
	ctx.r[25].s64 = ctx.r[1].s64 + 80;
	// 822E31E4: 3B010060  addi r24, r1, 0x60
	ctx.r[24].s64 = ctx.r[1].s64 + 96;
	// 822E31E8: 3AE10090  addi r23, r1, 0x90
	ctx.r[23].s64 = ctx.r[1].s64 + 144;
	// 822E31EC: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E31F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E31F4: 4E800421  bctrl
	ctx.lr = 0x822E31F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E31F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E31FC: 817A0030  lwz r11, 0x30(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(48 as u32) ) } as u64;
	// 822E3200: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E3204: 7EE5BB78  mr r5, r23
	ctx.r[5].u64 = ctx.r[23].u64;
	// 822E3208: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 822E320C: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 822E3210: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E3214: 4E800421  bctrl
	ctx.lr = 0x822E3218;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E3218: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822E321C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822E3220: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822E3224: 48B9A0FD  bl 0x82e7d320
	ctx.lr = 0x822E3228;
	sub_82E7D320(ctx, base);
	// 822E3228: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 822E322C: 13E0D8C7  vcmpequd (lvx128) v31, v0, v27
	tmp.u32 = ctx.r[27].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E3230: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 822E3234: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 822E3238: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 822E323C: 13C058C7  vcmpequd (lvx128) v30, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E3278 size=68
    let mut pc: u32 = 0x822E3278;
    'dispatch: loop {
        match pc {
            0x822E3278 => {
    //   block [0x822E3278..0x822E32BC)
	// 822E3278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E327C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E3280: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E3284: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E3288: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E328C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E3290: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E3294: 396BAA7C  addi r11, r11, -0x5584
	ctx.r[11].s64 = ctx.r[11].s64 + -21892;
	// 822E3298: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E329C: 41820008  beq 0x822e32a4
	if ctx.cr[0].eq {
	pc = 0x822E32A4; continue 'dispatch;
	}
	// 822E32A0: 4BFDCFC9  bl 0x822c0268
	ctx.lr = 0x822E32A4;
	sub_822C0268(ctx, base);
	// 822E32A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E32A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E32AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E32B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E32B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E32B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E32C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E32C0 size=8
    let mut pc: u32 = 0x822E32C0;
    'dispatch: loop {
        match pc {
            0x822E32C0 => {
    //   block [0x822E32C0..0x822E32C8)
	// 822E32C0: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E32C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E32C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822E32C8 size=8
    let mut pc: u32 = 0x822E32C8;
    'dispatch: loop {
        match pc {
            0x822E32C8 => {
    //   block [0x822E32C8..0x822E32D0)
	// 822E32C8: C0230020  lfs f1, 0x20(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822E32CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E32D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E32D0 size=12
    let mut pc: u32 = 0x822E32D0;
    'dispatch: loop {
        match pc {
            0x822E32D0 => {
    //   block [0x822E32D0..0x822E32DC)
	// 822E32D0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E32D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E32D8: 4BFED9E8  b 0x822d0cc0
	sub_822D0CC0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E32E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E32E0 size=8
    let mut pc: u32 = 0x822E32E0;
    'dispatch: loop {
        match pc {
            0x822E32E0 => {
    //   block [0x822E32E0..0x822E32E8)
	// 822E32E0: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E32E4: 4BFECC3C  b 0x822cff20
	sub_822CFF20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E32E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E32E8 size=40
    let mut pc: u32 = 0x822E32E8;
    'dispatch: loop {
        match pc {
            0x822E32E8 => {
    //   block [0x822E32E8..0x822E3310)
	// 822E32E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E32EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E32F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E32F4: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E32F8: 4BFECC39  bl 0x822cff30
	ctx.lr = 0x822E32FC;
	sub_822CFF30(ctx, base);
	// 822E32FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822E3300: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E3304: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E3308: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E330C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E3310 size=8
    let mut pc: u32 = 0x822E3310;
    'dispatch: loop {
        match pc {
            0x822E3310 => {
    //   block [0x822E3310..0x822E3318)
	// 822E3310: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E3314: 4BFECBEC  b 0x822cff00
	sub_822CFF00(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E3318 size=8
    let mut pc: u32 = 0x822E3318;
    'dispatch: loop {
        match pc {
            0x822E3318 => {
    //   block [0x822E3318..0x822E3320)
	// 822E3318: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E331C: 4BFECBF4  b 0x822cff10
	sub_822CFF10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E3320 size=8
    let mut pc: u32 = 0x822E3320;
    'dispatch: loop {
        match pc {
            0x822E3320 => {
    //   block [0x822E3320..0x822E3328)
	// 822E3320: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E3324: 4BFED1A4  b 0x822d04c8
	sub_822D04C8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E3328 size=20
    let mut pc: u32 = 0x822E3328;
    'dispatch: loop {
        match pc {
            0x822E3328 => {
    //   block [0x822E3328..0x822E333C)
	// 822E3328: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E332C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E3330: 409A000C  bne cr6, 0x822e333c
	if !ctx.cr[6].eq {
		sub_822E333C(ctx, base);
		return;
	}
	// 822E3334: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822E3338: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E333C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E333C size=16
    let mut pc: u32 = 0x822E333C;
    'dispatch: loop {
        match pc {
            0x822E333C => {
    //   block [0x822E333C..0x822E334C)
	// 822E333C: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 822E3340: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822E3344: 7D631670  srawi r3, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822E3348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E3350 size=8
    let mut pc: u32 = 0x822E3350;
    'dispatch: loop {
        match pc {
            0x822E3350 => {
    //   block [0x822E3350..0x822E3358)
	// 822E3350: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E3354: 4BFED614  b 0x822d0968
	sub_822D0968(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E3358 size=8
    let mut pc: u32 = 0x822E3358;
    'dispatch: loop {
        match pc {
            0x822E3358 => {
    //   block [0x822E3358..0x822E3360)
	// 822E3358: 80630004  lwz r3, 4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E335C: 4850C3DC  b 0x827ef738
	sub_827EF738(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E3360 size=56
    let mut pc: u32 = 0x822E3360;
    'dispatch: loop {
        match pc {
            0x822E3360 => {
    //   block [0x822E3360..0x822E3398)
	// 822E3360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E3364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E3368: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E336C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E3370: 80A4002C  lwz r5, 0x2c(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 822E3374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E3378: 80840024  lwz r4, 0x24(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 822E337C: 4BFEC4BD  bl 0x822cf838
	ctx.lr = 0x822E3380;
	sub_822CF838(ctx, base);
	// 822E3380: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E3384: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E3388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E338C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E3390: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E3394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E3398 size=196
    let mut pc: u32 = 0x822E3398;
    'dispatch: loop {
        match pc {
            0x822E3398 => {
    //   block [0x822E3398..0x822E345C)
	// 822E3398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E339C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E33A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E33A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E33A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E33AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E33B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E33B4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 822E33B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E33BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E33C0: 4BFDD579  bl 0x822c0938
	ctx.lr = 0x822E33C4;
	sub_822C0938(ctx, base);
	// 822E33C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E33C8: 41820028  beq 0x822e33f0
	if ctx.cr[0].eq {
	pc = 0x822E33F0; continue 'dispatch;
	}
	// 822E33CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E33D0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822E33D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822E33D8: 392BAAC8  addi r9, r11, -0x5538
	ctx.r[9].s64 = ctx.r[11].s64 + -21816;
	// 822E33DC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822E33E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E33E4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E33E8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822E33EC: 48000008  b 0x822e33f4
	pc = 0x822E33F4; continue 'dispatch;
	// 822E33F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E33F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E33F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E33FC: 409A0044  bne cr6, 0x822e3440
	if !ctx.cr[6].eq {
	pc = 0x822E3440; continue 'dispatch;
	}
	// 822E3400: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E3404: 419A001C  beq cr6, 0x822e3420
	if ctx.cr[6].eq {
	pc = 0x822E3420; continue 'dispatch;
	}
	// 822E3408: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E340C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822E3410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E3414: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E3418: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E341C: 4E800421  bctrl
	ctx.lr = 0x822E3420;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E3420: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E3424: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822E3428: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E342C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 822E3430: 816B3B74  lwz r11, 0x3b74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15220 as u32) ) } as u64;
	// 822E3434: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822E3438: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822E343C: 4BFDCBC5  bl 0x822c0000
	ctx.lr = 0x822E3440;
	sub_822C0000(ctx, base);
	// 822E3440: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E3444: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E3448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E344C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E3450: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E3454: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E3458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822E3460 size=336
    let mut pc: u32 = 0x822E3460;
    'dispatch: loop {
        match pc {
            0x822E3460 => {
    //   block [0x822E3460..0x822E35B0)
	// 822E3460: 7D632850  subf r11, r3, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[3].s64;
	// 822E3464: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822E3468: 2F0B0028  cmpwi cr6, r11, 0x28
	ctx.cr[6].compare_i32(ctx.r[11].s32, 40, &mut ctx.xer);
	// 822E346C: 40990150  ble cr6, 0x822e35bc
	if !ctx.cr[6].gt {
		sub_822E35BC(ctx, base);
		return;
	}
	// 822E3470: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 822E3474: C0030000  lfs f0, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3478: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 822E347C: 7D4B0194  addze r10, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[10].s64 = tmp.s64;
	// 822E3480: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822E3484: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 822E3488: 7DAB1C2E  lfsx f13, r11, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E348C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 822E3490: 40980014  bge cr6, 0x822e34a4
	if !ctx.cr[6].lt {
	pc = 0x822E34A4; continue 'dispatch;
	}
	// 822E3494: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 822E3498: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E349C: 7DAB1D2E  stfsx f13, r11, r3
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 822E34A0: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E34A4: 7C091C2E  lfsx f0, r9, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E34A8: 7DAB1C2E  lfsx f13, r11, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E34AC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E34B0: 4098000C  bge cr6, 0x822e34bc
	if !ctx.cr[6].lt {
	pc = 0x822E34BC; continue 'dispatch;
	}
	// 822E34B4: 7DA91D2E  stfsx f13, r9, r3
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 822E34B8: 7C0B1D2E  stfsx f0, r11, r3
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 822E34BC: 7C0B1C2E  lfsx f0, r11, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E34C0: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E34C4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E34C8: 4098000C  bge cr6, 0x822e34d4
	if !ctx.cr[6].lt {
	pc = 0x822E34D4; continue 'dispatch;
	}
	// 822E34CC: 7DAB1D2E  stfsx f13, r11, r3
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 822E34D0: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E34D4: 7D4B2050  subf r10, r11, r4
	ctx.r[10].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 822E34D8: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E34DC: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E34E0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E34E4: 4098000C  bge cr6, 0x822e34f0
	if !ctx.cr[6].lt {
	pc = 0x822E34F0; continue 'dispatch;
	}
	// 822E34E8: D1A40000  stfs f13, 0(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E34EC: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E34F0: 7C0B242E  lfsx f0, r11, r4
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E34F4: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E34F8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E34FC: 4098000C  bge cr6, 0x822e3508
	if !ctx.cr[6].lt {
	pc = 0x822E3508; continue 'dispatch;
	}
	// 822E3500: 7DAB252E  stfsx f13, r11, r4
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[4].u32), tmp.u32) };
	// 822E3504: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3508: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E350C: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E3510: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E3514: 4098000C  bge cr6, 0x822e3520
	if !ctx.cr[6].lt {
	pc = 0x822E3520; continue 'dispatch;
	}
	// 822E3518: D1A40000  stfs f13, 0(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E351C: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3520: 7D4B2850  subf r10, r11, r5
	ctx.r[10].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 822E3524: 7D292850  subf r9, r9, r5
	ctx.r[9].s64 = ctx.r[5].s64 - ctx.r[9].s64;
	// 822E3528: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E352C: C1A90000  lfs f13, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E3530: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E3534: 4098000C  bge cr6, 0x822e3540
	if !ctx.cr[6].lt {
	pc = 0x822E3540; continue 'dispatch;
	}
	// 822E3538: D1AA0000  stfs f13, 0(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E353C: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3540: C0050000  lfs f0, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3544: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E3548: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E354C: 4098000C  bge cr6, 0x822e3558
	if !ctx.cr[6].lt {
	pc = 0x822E3558; continue 'dispatch;
	}
	// 822E3550: D1A50000  stfs f13, 0(r5)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3554: D00A0000  stfs f0, 0(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3558: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E355C: C1A90000  lfs f13, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E3560: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E3564: 4098000C  bge cr6, 0x822e3570
	if !ctx.cr[6].lt {
	pc = 0x822E3570; continue 'dispatch;
	}
	// 822E3568: D1AA0000  stfs f13, 0(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E356C: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3570: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3574: 7DAB1C2E  lfsx f13, r11, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E3578: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E357C: 4098000C  bge cr6, 0x822e3588
	if !ctx.cr[6].lt {
	pc = 0x822E3588; continue 'dispatch;
	}
	// 822E3580: D1A40000  stfs f13, 0(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3584: 7C0B1D2E  stfsx f0, r11, r3
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 822E3588: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E358C: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E3590: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E3594: 4098000C  bge cr6, 0x822e35a0
	if !ctx.cr[6].lt {
	pc = 0x822E35A0; continue 'dispatch;
	}
	// 822E3598: D1AA0000  stfs f13, 0(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E359C: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E35A0: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E35A4: 7DAB1C2E  lfsx f13, r11, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E35A8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E35AC: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E35B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822E35B0 size=12
    let mut pc: u32 = 0x822E35B0;
    'dispatch: loop {
        match pc {
            0x822E35B0 => {
    //   block [0x822E35B0..0x822E35BC)
	// 822E35B0: D1A40000  stfs f13, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E35B4: 7C0B1D2E  stfsx f0, r11, r3
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 822E35B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E35BC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822E35BC size=64
    let mut pc: u32 = 0x822E35BC;
    'dispatch: loop {
        match pc {
            0x822E35BC => {
    //   block [0x822E35BC..0x822E35FC)
	// 822E35BC: C0040000  lfs f0, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E35C0: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E35C4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E35C8: 4098000C  bge cr6, 0x822e35d4
	if !ctx.cr[6].lt {
	pc = 0x822E35D4; continue 'dispatch;
	}
	// 822E35CC: D1A40000  stfs f13, 0(r4)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E35D0: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E35D4: C0050000  lfs f0, 0(r5)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E35D8: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E35DC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E35E0: 4098000C  bge cr6, 0x822e35ec
	if !ctx.cr[6].lt {
	pc = 0x822E35EC; continue 'dispatch;
	}
	// 822E35E4: D1A50000  stfs f13, 0(r5)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E35E8: D0040000  stfs f0, 0(r4)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E35EC: C0040000  lfs f0, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E35F0: C1A30000  lfs f13, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E35F4: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E35F8: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E35FC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822E35FC size=12
    let mut pc: u32 = 0x822E35FC;
    'dispatch: loop {
        match pc {
            0x822E35FC => {
    //   block [0x822E35FC..0x822E3608)
	// 822E35FC: D1A40000  stfs f13, 0(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3600: D0030000  stfs f0, 0(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3604: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E3608 size=116
    let mut pc: u32 = 0x822E3608;
    'dispatch: loop {
        match pc {
            0x822E3608 => {
    //   block [0x822E3608..0x822E367C)
	// 822E3608: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E360C: 48EC4B61  bl 0x831a816c
	ctx.lr = 0x822E3610;
	sub_831A8130(ctx, base);
	// 822E3610: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E3614: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E3618: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 822E361C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822E3620: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 822E3624: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822E3628: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E362C: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E3630: 4BFECA29  bl 0x822d0058
	ctx.lr = 0x822E3634;
	sub_822D0058(ctx, base);
	// 822E3634: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822E3638: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822E363C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822E3640: 48B99CE1  bl 0x82e7d320
	ctx.lr = 0x822E3644;
	sub_82E7D320(ctx, base);
	// 822E3644: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822E3648: 41980028  blt cr6, 0x822e3670
	if ctx.cr[6].lt {
	pc = 0x822E3670; continue 'dispatch;
	}
	// 822E364C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E3650: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822E3654: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E3680 size=124
    let mut pc: u32 = 0x822E3680;
    'dispatch: loop {
        match pc {
            0x822E3680 => {
    //   block [0x822E3680..0x822E36FC)
	// 822E3680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E3684: 48EC4AE1  bl 0x831a8164
	ctx.lr = 0x822E3688;
	sub_831A8130(ctx, base);
	// 822E3688: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E368C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E3690: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 822E3694: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 822E3698: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 822E369C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822E36A0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 822E36A4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E36A8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822E36AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E36B0: 4BFEC9A9  bl 0x822d0058
	ctx.lr = 0x822E36B4;
	sub_822D0058(ctx, base);
	// 822E36B4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 822E36B8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E36BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E36C0: 48B99C61  bl 0x82e7d320
	ctx.lr = 0x822E36C4;
	sub_82E7D320(ctx, base);
	// 822E36C4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822E36C8: 41980028  blt cr6, 0x822e36f0
	if ctx.cr[6].lt {
	pc = 0x822E36F0; continue 'dispatch;
	}
	// 822E36CC: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E36D0: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822E36D4: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E3700 size=124
    let mut pc: u32 = 0x822E3700;
    'dispatch: loop {
        match pc {
            0x822E3700 => {
    //   block [0x822E3700..0x822E377C)
	// 822E3700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E3704: 48EC4A61  bl 0x831a8164
	ctx.lr = 0x822E3708;
	sub_831A8130(ctx, base);
	// 822E3708: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E370C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E3710: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 822E3714: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 822E3718: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 822E371C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 822E3720: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 822E3724: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E3728: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822E372C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822E3730: 4BFEC9F1  bl 0x822d0120
	ctx.lr = 0x822E3734;
	sub_822D0120(ctx, base);
	// 822E3734: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 822E3738: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E373C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E3740: 48B99BE1  bl 0x82e7d320
	ctx.lr = 0x822E3744;
	sub_82E7D320(ctx, base);
	// 822E3744: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822E3748: 41980028  blt cr6, 0x822e3770
	if ctx.cr[6].lt {
	pc = 0x822E3770; continue 'dispatch;
	}
	// 822E374C: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E3750: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822E3754: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E3780 size=124
    let mut pc: u32 = 0x822E3780;
    'dispatch: loop {
        match pc {
            0x822E3780 => {
    //   block [0x822E3780..0x822E37FC)
	// 822E3780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E3784: 48EC49E1  bl 0x831a8164
	ctx.lr = 0x822E3788;
	sub_831A8130(ctx, base);
	// 822E3788: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E378C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E3790: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 822E3794: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 822E3798: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 822E379C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 822E37A0: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 822E37A4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E37A8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822E37AC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822E37B0: 4BFEC9C9  bl 0x822d0178
	ctx.lr = 0x822E37B4;
	sub_822D0178(ctx, base);
	// 822E37B4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 822E37B8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E37BC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E37C0: 48B99B61  bl 0x82e7d320
	ctx.lr = 0x822E37C4;
	sub_82E7D320(ctx, base);
	// 822E37C4: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822E37C8: 41980028  blt cr6, 0x822e37f0
	if ctx.cr[6].lt {
	pc = 0x822E37F0; continue 'dispatch;
	}
	// 822E37CC: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E37D0: 57AA103A  slwi r10, r29, 2
	ctx.r[10].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822E37D4: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E3800 size=484
    let mut pc: u32 = 0x822E3800;
    'dispatch: loop {
        match pc {
            0x822E3800 => {
    //   block [0x822E3800..0x822E39E4)
	// 822E3800: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E3804: 48EC4965  bl 0x831a8168
	ctx.lr = 0x822E3808;
	sub_831A8130(ctx, base);
	// 822E3808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E380C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E3810: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E3814: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822E3818: 7D7EE850  subf r11, r30, r29
	ctx.r[11].s64 = ctx.r[29].s64 - ctx.r[30].s64;
	// 822E381C: 38BDFFFC  addi r5, r29, -4
	ctx.r[5].s64 = ctx.r[29].s64 + -4;
	// 822E3820: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822E3824: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E3828: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 822E382C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 822E3830: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822E3834: 7FEBF214  add r31, r11, r30
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 822E3838: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E383C: 4BFFFC25  bl 0x822e3460
	ctx.lr = 0x822E3840;
	sub_822E3460(ctx, base);
	// 822E3840: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 822E3844: 393F0004  addi r9, r31, 4
	ctx.r[9].s64 = ctx.r[31].s64 + 4;
	// 822E3848: 7F1EF840  cmplw cr6, r30, r31
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[31].u32, &mut ctx.xer);
	// 822E384C: 4098002C  bge cr6, 0x822e3878
	if !ctx.cr[6].lt {
	pc = 0x822E3878; continue 'dispatch;
	}
	// 822E3850: C00B0000  lfs f0, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3854: 394BFFFC  addi r10, r11, -4
	ctx.r[10].s64 = ctx.r[11].s64 + -4;
	// 822E3858: C1ABFFFC  lfs f13, -4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E385C: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 822E3860: 41980018  blt cr6, 0x822e3878
	if ctx.cr[6].lt {
	pc = 0x822E3878; continue 'dispatch;
	}
	// 822E3864: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E3868: 41980010  blt cr6, 0x822e3878
	if ctx.cr[6].lt {
	pc = 0x822E3878; continue 'dispatch;
	}
	// 822E386C: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 822E3870: 7F1E5040  cmplw cr6, r30, r10
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822E3874: 4198FFDC  blt cr6, 0x822e3850
	if ctx.cr[6].lt {
	pc = 0x822E3850; continue 'dispatch;
	}
	// 822E3878: 7F09E840  cmplw cr6, r9, r29
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822E387C: 40980028  bge cr6, 0x822e38a4
	if !ctx.cr[6].lt {
	pc = 0x822E38A4; continue 'dispatch;
	}
	// 822E3880: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E3884: C0090000  lfs f0, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3888: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E388C: 41980018  blt cr6, 0x822e38a4
	if ctx.cr[6].lt {
	pc = 0x822E38A4; continue 'dispatch;
	}
	// 822E3890: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 822E3894: 41980010  blt cr6, 0x822e38a4
	if ctx.cr[6].lt {
	pc = 0x822E38A4; continue 'dispatch;
	}
	// 822E3898: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 822E389C: 7F09E840  cmplw cr6, r9, r29
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822E38A0: 4198FFE4  blt cr6, 0x822e3884
	if ctx.cr[6].lt {
	pc = 0x822E3884; continue 'dispatch;
	}
	// 822E38A4: 7D2A4B78  mr r10, r9
	ctx.r[10].u64 = ctx.r[9].u64;
	// 822E38A8: 7D675B78  mr r7, r11
	ctx.r[7].u64 = ctx.r[11].u64;
	// 822E38AC: 48000038  b 0x822e38e4
	pc = 0x822E38E4; continue 'dispatch;
	// 822E38B0: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E38B4: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E38B8: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E38BC: 41980024  blt cr6, 0x822e38e0
	if ctx.cr[6].lt {
	pc = 0x822E38E0; continue 'dispatch;
	}
	// 822E38C0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 822E38C4: 41980028  blt cr6, 0x822e38ec
	if ctx.cr[6].lt {
	pc = 0x822E38EC; continue 'dispatch;
	}
	// 822E38C8: 7D284B78  mr r8, r9
	ctx.r[8].u64 = ctx.r[9].u64;
	// 822E38CC: FC006890  fmr f0, f13
	ctx.f[0].f64 = ctx.f[13].f64;
	// 822E38D0: C1A80000  lfs f13, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E38D4: D0080000  stfs f0, 0(r8)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E38D8: D1AA0000  stfs f13, 0(r10)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E38DC: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 822E38E0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 822E38E4: 7F0AE840  cmplw cr6, r10, r29
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822E38E8: 4198FFC8  blt cr6, 0x822e38b0
	if ctx.cr[6].lt {
	pc = 0x822E38B0; continue 'dispatch;
	}
	// 822E38EC: 7F07F040  cmplw cr6, r7, r30
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822E38F0: 40990044  ble cr6, 0x822e3934
	if !ctx.cr[6].gt {
	pc = 0x822E3934; continue 'dispatch;
	}
	// 822E38F4: 3907FFFC  addi r8, r7, -4
	ctx.r[8].s64 = ctx.r[7].s64 + -4;
	// 822E38F8: C0080000  lfs f0, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E38FC: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E3900: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E3904: 4198001C  blt cr6, 0x822e3920
	if ctx.cr[6].lt {
	pc = 0x822E3920; continue 'dispatch;
	}
	// 822E3908: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 822E390C: 41980024  blt cr6, 0x822e3930
	if ctx.cr[6].lt {
	pc = 0x822E3930; continue 'dispatch;
	}
	// 822E3910: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 822E3914: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E3918: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E391C: D1A80000  stfs f13, 0(r8)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3920: 38E7FFFC  addi r7, r7, -4
	ctx.r[7].s64 = ctx.r[7].s64 + -4;
	// 822E3924: 3908FFFC  addi r8, r8, -4
	ctx.r[8].s64 = ctx.r[8].s64 + -4;
	// 822E3928: 7F1E3840  cmplw cr6, r30, r7
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[7].u32, &mut ctx.xer);
	// 822E392C: 4198FFCC  blt cr6, 0x822e38f8
	if ctx.cr[6].lt {
	pc = 0x822E38F8; continue 'dispatch;
	}
	// 822E3930: 7F07F040  cmplw cr6, r7, r30
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822E3934: 409A0044  bne cr6, 0x822e3978
	if !ctx.cr[6].eq {
	pc = 0x822E3978; continue 'dispatch;
	}
	// 822E3938: 7F0AE840  cmplw cr6, r10, r29
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822E393C: 419A0094  beq cr6, 0x822e39d0
	if ctx.cr[6].eq {
	pc = 0x822E39D0; continue 'dispatch;
	}
	// 822E3940: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822E3944: 419A0014  beq cr6, 0x822e3958
	if ctx.cr[6].eq {
	pc = 0x822E3958; continue 'dispatch;
	}
	// 822E3948: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E394C: C1A90000  lfs f13, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E3950: D1AB0000  stfs f13, 0(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3954: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3958: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 822E395C: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 822E3960: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822E3964: C1A60000  lfs f13, 0(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E3968: C0080000  lfs f0, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E396C: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3970: D1A80000  stfs f13, 0(r8)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3974: 4BFFFF68  b 0x822e38dc
	pc = 0x822E38DC; continue 'dispatch;
	// 822E3978: 7F0AE840  cmplw cr6, r10, r29
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[29].u32, &mut ctx.xer);
	// 822E397C: 38E7FFFC  addi r7, r7, -4
	ctx.r[7].s64 = ctx.r[7].s64 + -4;
	// 822E3980: 409A0038  bne cr6, 0x822e39b8
	if !ctx.cr[6].eq {
	pc = 0x822E39B8; continue 'dispatch;
	}
	// 822E3984: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 822E3988: 7F075840  cmplw cr6, r7, r11
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E398C: 419A0014  beq cr6, 0x822e39a0
	if ctx.cr[6].eq {
	pc = 0x822E39A0; continue 'dispatch;
	}
	// 822E3990: C0070000  lfs f0, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3994: C1AB0000  lfs f13, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E3998: D1A70000  stfs f13, 0(r7)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E399C: D00B0000  stfs f0, 0(r11)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E39A0: 3929FFFC  addi r9, r9, -4
	ctx.r[9].s64 = ctx.r[9].s64 + -4;
	// 822E39A4: C00B0000  lfs f0, 0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E39A8: C1A90000  lfs f13, 0(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E39AC: D1AB0000  stfs f13, 0(r11)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E39B0: D0090000  stfs f0, 0(r9)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E39B4: 4BFFFF30  b 0x822e38e4
	pc = 0x822E38E4; continue 'dispatch;
	// 822E39B8: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 822E39BC: C1A70000  lfs f13, 0(r7)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E39C0: C0080000  lfs f0, 0(r8)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E39C4: D1A80000  stfs f13, 0(r8)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E39C8: D0070000  stfs f0, 0(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E39CC: 4BFFFF14  b 0x822e38e0
	pc = 0x822E38E0; continue 'dispatch;
	// 822E39D0: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E39D4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822E39D8: 913C0004  stw r9, 4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822E39DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E39E0: 48EC47D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E39E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E39E8 size=8
    let mut pc: u32 = 0x822E39E8;
    'dispatch: loop {
        match pc {
            0x822E39E8 => {
    //   block [0x822E39E8..0x822E39F0)
	// 822E39E8: 7F032040  cmplw cr6, r3, r4
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[4].u32, &mut ctx.xer);
	// 822E39EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E39F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E39F0 size=8
    let mut pc: u32 = 0x822E39F0;
    'dispatch: loop {
        match pc {
            0x822E39F0 => {
    //   block [0x822E39F0..0x822E39F8)
	// 822E39F0: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 822E39F4: 48000070  b 0x822e3a64
	sub_822E3A44(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E39F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822E39F8 size=64
    let mut pc: u32 = 0x822E39F8;
    'dispatch: loop {
        match pc {
            0x822E39F8 => {
    //   block [0x822E39F8..0x822E3A38)
	// 822E39F8: C1AB0000  lfs f13, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E39FC: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 822E3A00: C0030000  lfs f0, 0(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3A04: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 822E3A08: 40980030  bge cr6, 0x822e3a38
	if !ctx.cr[6].lt {
		sub_822E3A38(ctx, base);
		return;
	}
	// 822E3A0C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 822E3A10: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E3A14: 419A001C  beq cr6, 0x822e3a30
	if ctx.cr[6].eq {
	pc = 0x822E3A30; continue 'dispatch;
	}
	// 822E3A18: 39200004  li r9, 4
	ctx.r[9].s64 = 4;
	// 822E3A1C: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 822E3A20: 7F0A1840  cmplw cr6, r10, r3
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[3].u32, &mut ctx.xer);
	// 822E3A24: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3A28: 7C09552E  stfsx f0, r9, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), tmp.u32) };
	// 822E3A2C: 409AFFF0  bne cr6, 0x822e3a1c
	if !ctx.cr[6].eq {
	pc = 0x822E3A1C; continue 'dispatch;
	}
	// 822E3A30: D1A30000  stfs f13, 0(r3)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3A34: 4800002C  b 0x822e3a60
	sub_822E3A44(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822E3A38 size=12
    let mut pc: u32 = 0x822E3A38;
    'dispatch: loop {
        match pc {
            0x822E3A38 => {
    //   block [0x822E3A38..0x822E3A44)
	// 822E3A38: C00BFFFC  lfs f0, -4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3A3C: 394BFFFC  addi r10, r11, -4
	ctx.r[10].s64 = ctx.r[11].s64 + -4;
	// 822E3A40: 48000014  b 0x822e3a54
	sub_822E3A44(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3A44(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822E3A44 size=44
    let mut pc: u32 = 0x822E3A44;
    'dispatch: loop {
        match pc {
            0x822E3A44 => {
    //   block [0x822E3A44..0x822E3A70)
	// 822E3A44: D0090000  stfs f0, 0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3A48: 7D495378  mr r9, r10
	ctx.r[9].u64 = ctx.r[10].u64;
	// 822E3A4C: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 822E3A50: C00A0000  lfs f0, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3A54: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 822E3A58: 4198FFEC  blt cr6, 0x822e3a44
	if ctx.cr[6].lt {
	pc = 0x822E3A44; continue 'dispatch;
	}
	// 822E3A5C: D1A90000  stfs f13, 0(r9)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E3A60: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822E3A64: 7F0B2040  cmplw cr6, r11, r4
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[4].u32, &mut ctx.xer);
	// 822E3A68: 409AFF90  bne cr6, 0x822e39f8
	if !ctx.cr[6].eq {
		sub_822E39F8(ctx, base);
		return;
	}
	// 822E3A6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3A70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E3A70 size=16
    let mut pc: u32 = 0x822E3A70;
    'dispatch: loop {
        match pc {
            0x822E3A70 => {
    //   block [0x822E3A70..0x822E3A80)
	// 822E3A70: 39640001  addi r11, r4, 1
	ctx.r[11].s64 = ctx.r[4].s64 + 1;
	// 822E3A74: 7C872378  mr r7, r4
	ctx.r[7].u64 = ctx.r[4].u64;
	// 822E3A78: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822E3A7C: 4800003C  b 0x822e3ab8
	sub_822E3A80(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3A80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822E3A80 size=112
    let mut pc: u32 = 0x822E3A80;
    'dispatch: loop {
        match pc {
            0x822E3A80 => {
    //   block [0x822E3A80..0x822E3AF0)
	// 822E3A80: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822E3A84: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 822E3A88: C00A0000  lfs f0, 0(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3A8C: C1AAFFFC  lfs f13, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E3A90: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E3A94: 40980008  bge cr6, 0x822e3a9c
	if !ctx.cr[6].lt {
	pc = 0x822E3A9C; continue 'dispatch;
	}
	// 822E3A98: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 822E3A9C: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 822E3AA0: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822E3AA4: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 822E3AA8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 822E3AAC: 550B083C  slwi r11, r8, 1
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822E3AB0: 7C091C2E  lfsx f0, r9, r3
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[3].u32)) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3AB4: 7C0A1D2E  stfsx f0, r10, r3
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 822E3AB8: 7F0B2800  cmpw cr6, r11, r5
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[5].s32, &mut ctx.xer);
	// 822E3ABC: 4198FFC4  blt cr6, 0x822e3a80
	if ctx.cr[6].lt {
	pc = 0x822E3A80; continue 'dispatch;
	}
	// 822E3AC0: 409A001C  bne cr6, 0x822e3adc
	if !ctx.cr[6].eq {
	pc = 0x822E3ADC; continue 'dispatch;
	}
	// 822E3AC4: 54AA103A  slwi r10, r5, 2
	ctx.r[10].u32 = ctx.r[5].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822E3AC8: 548B103A  slwi r11, r4, 2
	ctx.r[11].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822E3ACC: 7D4A1A14  add r10, r10, r3
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 822E3AD0: 3885FFFF  addi r4, r5, -1
	ctx.r[4].s64 = ctx.r[5].s64 + -1;
	// 822E3AD4: C00AFFFC  lfs f0, -4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3AD8: 7C0B1D2E  stfsx f0, r11, r3
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 822E3ADC: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 822E3AE0: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 822E3AE4: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 822E3AE8: 7F072000  cmpw cr6, r7, r4
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[4].s32, &mut ctx.xer);
	// 822E3AEC: 48000030  b 0x822e3b1c
	sub_822E3AF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822E3AF0 size=64
    let mut pc: u32 = 0x822E3AF0;
    'dispatch: loop {
        match pc {
            0x822E3AF0 => {
    //   block [0x822E3AF0..0x822E3B30)
	// 822E3AF0: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 822E3AF4: 7D291A14  add r9, r9, r3
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[3].u64;
	// 822E3AF8: C0090000  lfs f0, 0(r9)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3AFC: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 822E3B00: 40980024  bge cr6, 0x822e3b24
	if !ctx.cr[6].lt {
	pc = 0x822E3B24; continue 'dispatch;
	}
	// 822E3B04: 5548103A  slwi r8, r10, 2
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 822E3B08: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 822E3B0C: 392BFFFF  addi r9, r11, -1
	ctx.r[9].s64 = ctx.r[11].s64 + -1;
	// 822E3B10: 7F075000  cmpw cr6, r7, r10
	ctx.cr[6].compare_i32(ctx.r[7].s32, ctx.r[10].s32, &mut ctx.xer);
	// 822E3B14: 7D2B0E70  srawi r11, r9, 1
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[9].s32 >> 1) as i64;
	// 822E3B18: 7C081D2E  stfsx f0, r8, r3
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[8].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 822E3B1C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 822E3B20: 4198FFD0  blt cr6, 0x822e3af0
	if ctx.cr[6].lt {
	pc = 0x822E3AF0; continue 'dispatch;
	}
	// 822E3B24: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822E3B28: 7C2B1D2E  stfsx f1, r11, r3
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32), tmp.u32) };
	// 822E3B2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E3B30 size=88
    let mut pc: u32 = 0x822E3B30;
    'dispatch: loop {
        match pc {
            0x822E3B30 => {
    //   block [0x822E3B30..0x822E3B88)
	// 822E3B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E3B34: 48EC4635  bl 0x831a8168
	ctx.lr = 0x822E3B38;
	sub_831A8130(ctx, base);
	// 822E3B38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E3B3C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E3B40: 7D7D2050  subf r11, r29, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[29].s64;
	// 822E3B44: 7D7C1670  srawi r28, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[28].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822E3B48: 7F8B0E70  srawi r11, r28, 1
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[28].s32 >> 1) as i64;
	// 822E3B4C: 7FEB0195  addze. r31, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[31].s64 = tmp.s64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822E3B50: 40810030  ble 0x822e3b80
	if !ctx.cr[0].gt {
	pc = 0x822E3B80; continue 'dispatch;
	}
	// 822E3B54: 57EB103A  slwi r11, r31, 2
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822E3B58: 7FCBEA14  add r30, r11, r29
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 822E3B5C: 3BDEFFFC  addi r30, r30, -4
	ctx.r[30].s64 = ctx.r[30].s64 + -4;
	// 822E3B60: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 822E3B64: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822E3B68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E3B6C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E3B70: C03E0000  lfs f1, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822E3B74: 4BFFFEFD  bl 0x822e3a70
	ctx.lr = 0x822E3B78;
	sub_822E3A70(ctx, base);
	// 822E3B78: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822E3B7C: 4199FFE0  bgt cr6, 0x822e3b5c
	if ctx.cr[6].gt {
	pc = 0x822E3B5C; continue 'dispatch;
	}
	// 822E3B80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E3B84: 48EC4634  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E3B88 size=92
    let mut pc: u32 = 0x822E3B88;
    'dispatch: loop {
        match pc {
            0x822E3B88 => {
    //   block [0x822E3B88..0x822E3BE4)
	// 822E3B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E3B8C: 48EC45E1  bl 0x831a816c
	ctx.lr = 0x822E3B90;
	sub_831A8130(ctx, base);
	// 822E3B90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E3B94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E3B98: 7FFE2050  subf r31, r30, r4
	ctx.r[31].s64 = ctx.r[4].s64 - ctx.r[30].s64;
	// 822E3B9C: 7FEB1670  srawi r11, r31, 2
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[31].s32 >> 2) as i64;
	// 822E3BA0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 822E3BA4: 40990038  ble cr6, 0x822e3bdc
	if !ctx.cr[6].gt {
	pc = 0x822E3BDC; continue 'dispatch;
	}
	// 822E3BA8: 3BBEFFFC  addi r29, r30, -4
	ctx.r[29].s64 = ctx.r[30].s64 + -4;
	// 822E3BAC: 7C3DFC2E  lfsx f1, r29, r31
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32)) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822E3BB0: 397FFFFC  addi r11, r31, -4
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	// 822E3BB4: C01E0000  lfs f0, 0(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3BB8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E3BBC: 7C1DFD2E  stfsx f0, r29, r31
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[29].u32.wrapping_add(ctx.r[31].u32), tmp.u32) };
	// 822E3BC0: 7D651670  srawi r5, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822E3BC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E3BC8: 4BFFFEA9  bl 0x822e3a70
	ctx.lr = 0x822E3BCC;
	sub_822E3A70(ctx, base);
	// 822E3BCC: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 822E3BD0: 7FEB1670  srawi r11, r31, 2
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[31].s32 >> 2) as i64;
	// 822E3BD4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 822E3BD8: 4199FFD4  bgt cr6, 0x822e3bac
	if ctx.cr[6].gt {
	pc = 0x822E3BAC; continue 'dispatch;
	}
	// 822E3BDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E3BE0: 48EC45DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E3BE8 size=264
    let mut pc: u32 = 0x822E3BE8;
    'dispatch: loop {
        match pc {
            0x822E3BE8 => {
    //   block [0x822E3BE8..0x822E3CF0)
	// 822E3BE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E3BEC: 48EC4579  bl 0x831a8164
	ctx.lr = 0x822E3BF0;
	sub_831A8130(ctx, base);
	// 822E3BF0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E3BF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E3BF8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E3BFC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E3C00: 7D7EF850  subf r11, r30, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 822E3C04: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822E3C08: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 822E3C0C: 409900C4  ble cr6, 0x822e3cd0
	if !ctx.cr[6].gt {
	pc = 0x822E3CD0; continue 'dispatch;
	}
	// 822E3C10: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822E3C14: 40990080  ble cr6, 0x822e3c94
	if !ctx.cr[6].gt {
	pc = 0x822E3C94; continue 'dispatch;
	}
	// 822E3C18: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822E3C1C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E3C20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E3C24: 4BFFFBDD  bl 0x822e3800
	ctx.lr = 0x822E3C28;
	sub_822E3800(ctx, base);
	// 822E3C28: 7FAB0E70  srawi r11, r29, 1
	ctx.xer.ca = (ctx.r[29].s32 < 0) && ((ctx.r[29].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[29].s32 >> 1) as i64;
	// 822E3C2C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 822E3C30: 7D6A0E70  srawi r10, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 822E3C34: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 822E3C38: 7FAA5A14  add r29, r10, r11
	ctx.r[29].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 822E3C3C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822E3C40: 83810054  lwz r28, 0x54(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822E3C44: 83610050  lwz r27, 0x50(r1)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E3C48: 7D7CF850  subf r11, r28, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[28].s64;
	// 822E3C4C: 7D5ED850  subf r10, r30, r27
	ctx.r[10].s64 = ctx.r[27].s64 - ctx.r[30].s64;
	// 822E3C50: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 822E3C54: 554A003A  rlwinm r10, r10, 0, 0, 0x1d
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 822E3C58: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822E3C5C: 40980018  bge cr6, 0x822e3c74
	if !ctx.cr[6].lt {
	pc = 0x822E3C74; continue 'dispatch;
	}
	// 822E3C60: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822E3C64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E3C68: 4BFFFF81  bl 0x822e3be8
	ctx.lr = 0x822E3C6C;
	sub_822E3BE8(ctx, base);
	// 822E3C6C: 7F9EE378  mr r30, r28
	ctx.r[30].u64 = ctx.r[28].u64;
	// 822E3C70: 48000014  b 0x822e3c84
	pc = 0x822E3C84; continue 'dispatch;
	// 822E3C74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E3C78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822E3C7C: 4BFFFF6D  bl 0x822e3be8
	ctx.lr = 0x822E3C80;
	sub_822E3BE8(ctx, base);
	// 822E3C80: 7F7FDB78  mr r31, r27
	ctx.r[31].u64 = ctx.r[27].u64;
	// 822E3C84: 7D7EF850  subf r11, r30, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 822E3C88: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822E3C8C: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 822E3C90: 4199FF80  bgt cr6, 0x822e3c10
	if ctx.cr[6].gt {
	pc = 0x822E3C10; continue 'dispatch;
	}
	// 822E3C94: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 822E3C98: 40990038  ble cr6, 0x822e3cd0
	if !ctx.cr[6].gt {
	pc = 0x822E3CD0; continue 'dispatch;
	}
	// 822E3C9C: 7D7EF850  subf r11, r30, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 822E3CA0: 556B003A  rlwinm r11, r11, 0, 0, 0x1d
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 822E3CA4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 822E3CA8: 40990018  ble cr6, 0x822e3cc0
	if !ctx.cr[6].gt {
	pc = 0x822E3CC0; continue 'dispatch;
	}
	// 822E3CAC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E3CB0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822E3CB4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E3CB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E3CBC: 4BFFFE75  bl 0x822e3b30
	ctx.lr = 0x822E3CC0;
	sub_822E3B30(ctx, base);
	// 822E3CC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E3CC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E3CC8: 4BFFFEC1  bl 0x822e3b88
	ctx.lr = 0x822E3CCC;
	sub_822E3B88(ctx, base);
	// 822E3CCC: 4800001C  b 0x822e3ce8
	pc = 0x822E3CE8; continue 'dispatch;
	// 822E3CD0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 822E3CD4: 40990014  ble cr6, 0x822e3ce8
	if !ctx.cr[6].gt {
	pc = 0x822E3CE8; continue 'dispatch;
	}
	// 822E3CD8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822E3CDC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E3CE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E3CE4: 4BFFFD05  bl 0x822e39e8
	ctx.lr = 0x822E3CE8;
	sub_822E39E8(ctx, base);
	// 822E3CE8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 822E3CEC: 48EC44C8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E3CF0 size=388
    let mut pc: u32 = 0x822E3CF0;
    'dispatch: loop {
        match pc {
            0x822E3CF0 => {
    //   block [0x822E3CF0..0x822E3E74)
	// 822E3CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E3CF4: 48EC4465  bl 0x831a8158
	ctx.lr = 0x822E3CF8;
	sub_831A8130(ctx, base);
	// 822E3CF8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E3CFC: 7D585378  mr r24, r10
	ctx.r[24].u64 = ctx.r[10].u64;
	// 822E3D00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E3D04: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E3D08: 7D394B78  mr r25, r9
	ctx.r[25].u64 = ctx.r[9].u64;
	// 822E3D0C: 392AAADC  addi r9, r10, -0x5524
	ctx.r[9].s64 = ctx.r[10].s64 + -21796;
	// 822E3D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E3D14: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E3D18: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 822E3D1C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E3D20: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 822E3D24: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822E3D28: 7D1C4378  mr r28, r8
	ctx.r[28].u64 = ctx.r[8].u64;
	// 822E3D2C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822E3D30: 395F0024  addi r10, r31, 0x24
	ctx.r[10].s64 = ctx.r[31].s64 + 36;
	// 822E3D34: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 822E3D38: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 822E3D3C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 822E3D40: 3BBF000C  addi r29, r31, 0xc
	ctx.r[29].s64 = ctx.r[31].s64 + 12;
	// 822E3D44: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E3D48: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 822E3D4C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E3D50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E3D54: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 822E3D58: 419A0024  beq cr6, 0x822e3d7c
	if ctx.cr[6].eq {
	pc = 0x822E3D7C; continue 'dispatch;
	}
	// 822E3D5C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822E3D60: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 822E3D64: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822E3D68: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 822E3D6C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822E3D70: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 822E3D74: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822E3D78: 4082FFE8  bne 0x822e3d60
	if !ctx.cr[0].eq {
	pc = 0x822E3D60; continue 'dispatch;
	}
	// 822E3D7C: D03F0030  stfs f1, 0x30(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), tmp.u32 ) };
	// 822E3D80: 90BF002C  stw r5, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[5].u32 ) };
	// 822E3D84: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822E3D88: 80840000  lwz r4, 0(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E3D8C: 4BFEBAAD  bl 0x822cf838
	ctx.lr = 0x822E3D90;
	sub_822CF838(ctx, base);
	// 822E3D90: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E3D94: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E3D98: 387E0004  addi r3, r30, 4
	ctx.r[3].s64 = ctx.r[30].s64 + 4;
	// 822E3D9C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 822E3DA0: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822E3DA4: 4BFE06BD  bl 0x822c4460
	ctx.lr = 0x822E3DA8;
	sub_822C4460(ctx, base);
	// 822E3DA8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 822E3DAC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E3DB0: 419A0008  beq cr6, 0x822e3db8
	if ctx.cr[6].eq {
	pc = 0x822E3DB8; continue 'dispatch;
	}
	// 822E3DB4: 4BFDCADD  bl 0x822c0890
	ctx.lr = 0x822E3DB8;
	sub_822C0890(ctx, base);
	// 822E3DB8: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 822E3DBC: 419A0020  beq cr6, 0x822e3ddc
	if ctx.cr[6].eq {
	pc = 0x822E3DDC; continue 'dispatch;
	}
	// 822E3DC0: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 822E3DC4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822E3DC8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E3DCC: 4BFF8A75  bl 0x822dc840
	ctx.lr = 0x822E3DD0;
	sub_822DC840(ctx, base);
	// 822E3DD0: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 822E3DD4: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 822E3DD8: 4082FFEC  bne 0x822e3dc4
	if !ctx.cr[0].eq {
	pc = 0x822E3DC4; continue 'dispatch;
	}
	// 822E3DDC: 2B190000  cmplwi cr6, r25, 0
	ctx.cr[6].compare_u32(ctx.r[25].u32, 0 as u32, &mut ctx.xer);
	// 822E3DE0: 419A002C  beq cr6, 0x822e3e0c
	if ctx.cr[6].eq {
	pc = 0x822E3E0C; continue 'dispatch;
	}
	// 822E3DE4: 7F3ECB78  mr r30, r25
	ctx.r[30].u64 = ctx.r[25].u64;
	// 822E3DE8: C01C0000  lfs f0, 0(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3DEC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822E3DF0: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 822E3DF4: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822E3DF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E3DFC: 4BFF8A45  bl 0x822dc840
	ctx.lr = 0x822E3E00;
	sub_822DC840(ctx, base);
	// 822E3E00: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 822E3E04: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 822E3E08: 4082FFE0  bne 0x822e3de8
	if !ctx.cr[0].eq {
	pc = 0x822E3DE8; continue 'dispatch;
	}
	// 822E3E0C: 570B063F  clrlwi. r11, r24, 0x18
	ctx.r[11].u64 = ctx.r[24].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E3E10: 41820024  beq 0x822e3e34
	if ctx.cr[0].eq {
	pc = 0x822E3E34; continue 'dispatch;
	}
	// 822E3E14: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822E3E18: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822E3E1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E3E20: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3E24: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822E3E28: 4BFF8A19  bl 0x822dc840
	ctx.lr = 0x822E3E2C;
	sub_822DC840(ctx, base);
	// 822E3E2C: 933F001C  stw r25, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[25].u32 ) };
	// 822E3E30: 4800000C  b 0x822e3e3c
	pc = 0x822E3E3C; continue 'dispatch;
	// 822E3E34: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 822E3E38: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 822E3E3C: 809F0014  lwz r4, 0x14(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822E3E40: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E3E44: 7D632050  subf r11, r3, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[3].s64;
	// 822E3E48: 7D651670  srawi r5, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[5].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822E3E4C: 4BFFFD9D  bl 0x822e3be8
	ctx.lr = 0x822E3E50;
	sub_822E3BE8(ctx, base);
	// 822E3E50: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822E3E54: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E3E58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E3E5C: C00BFFFC  lfs f0, -4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3E60: C1AA0000  lfs f13, 0(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E3E64: EC006828  fsubs f0, f0, f13
	ctx.f[0].f64 = (((ctx.f[0].f64 - ctx.f[13].f64) as f32) as f64);
	// 822E3E68: D01F0020  stfs f0, 0x20(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), tmp.u32 ) };
	// 822E3E6C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 822E3E70: 48EC4338  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E3E78 size=92
    let mut pc: u32 = 0x822E3E78;
    'dispatch: loop {
        match pc {
            0x822E3E78 => {
    //   block [0x822E3E78..0x822E3ED4)
	// 822E3E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E3E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E3E80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E3E84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E3E88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E3E8C: 807F0028  lwz r3, 0x28(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 822E3E90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E3E94: 419A0008  beq cr6, 0x822e3e9c
	if ctx.cr[6].eq {
	pc = 0x822E3E9C; continue 'dispatch;
	}
	// 822E3E98: 4BFDC9F9  bl 0x822c0890
	ctx.lr = 0x822E3E9C;
	sub_822C0890(ctx, base);
	// 822E3E9C: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 822E3EA0: 48186161  bl 0x8246a000
	ctx.lr = 0x822E3EA4;
	sub_8246A000(ctx, base);
	// 822E3EA4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E3EA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E3EAC: 419A0008  beq cr6, 0x822e3eb4
	if ctx.cr[6].eq {
	pc = 0x822E3EB4; continue 'dispatch;
	}
	// 822E3EB0: 4BFDC9E1  bl 0x822c0890
	ctx.lr = 0x822E3EB4;
	sub_822C0890(ctx, base);
	// 822E3EB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E3EB8: 396BAA7C  addi r11, r11, -0x5584
	ctx.r[11].s64 = ctx.r[11].s64 + -21892;
	// 822E3EBC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E3EC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E3EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E3EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E3ECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E3ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E3ED8 size=76
    let mut pc: u32 = 0x822E3ED8;
    'dispatch: loop {
        match pc {
            0x822E3ED8 => {
    //   block [0x822E3ED8..0x822E3F24)
	// 822E3ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E3EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E3EE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E3EE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E3EE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E3EEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E3EF0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E3EF4: 4BFFFF85  bl 0x822e3e78
	ctx.lr = 0x822E3EF8;
	sub_822E3E78(ctx, base);
	// 822E3EF8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E3EFC: 4182000C  beq 0x822e3f08
	if ctx.cr[0].eq {
	pc = 0x822E3F08; continue 'dispatch;
	}
	// 822E3F00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E3F04: 4BFDC365  bl 0x822c0268
	ctx.lr = 0x822E3F08;
	sub_822C0268(ctx, base);
	// 822E3F08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E3F0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E3F10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E3F14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E3F18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E3F1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E3F20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E3F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E3F28 size=404
    let mut pc: u32 = 0x822E3F28;
    'dispatch: loop {
        match pc {
            0x822E3F28 => {
    //   block [0x822E3F28..0x822E40BC)
	// 822E3F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E3F2C: 48EC4231  bl 0x831a815c
	ctx.lr = 0x822E3F30;
	sub_831A8130(ctx, base);
	// 822E3F30: DBA1FFA8  stfd f29, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[29].u64 ) };
	// 822E3F34: DBC1FFB0  stfd f30, -0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-80 as u32), ctx.f[30].u64 ) };
	// 822E3F38: DBE1FFB8  stfd f31, -0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-72 as u32), ctx.f[31].u64 ) };
	// 822E3F3C: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E3F40: 54CB07FE  clrlwi r11, r6, 0x1f
	ctx.r[11].u64 = ctx.r[6].u32 as u64 & 0x00000001u64;
	// 822E3F44: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822E3F48: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 822E3F4C: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 822E3F50: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 822E3F54: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822E3F58: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 822E3F5C: 409A0010  bne cr6, 0x822e3f6c
	if !ctx.cr[6].eq {
	pc = 0x822E3F6C; continue 'dispatch;
	}
	// 822E3F60: FFC0F890  fmr f30, f31
	ctx.f[30].f64 = ctx.f[31].f64;
	// 822E3F64: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 822E3F68: 48000014  b 0x822e3f7c
	pc = 0x822E3F7C; continue 'dispatch;
	// 822E3F6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E3F70: 7FFCFB78  mr r28, r31
	ctx.r[28].u64 = ctx.r[31].u64;
	// 822E3F74: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E3F78: EFDF0032  fmuls f30, f31, f0
	ctx.f[30].f64 = (((ctx.f[31].f64 * ctx.f[0].f64) as f32) as f64);
	// 822E3F7C: 54CBF87E  srwi r11, r6, 1
	ctx.r[11].u32 = ctx.r[6].u32.wrapping_shr(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822E3F80: 93E1008C  stw r31, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[31].u32 ) };
	// 822E3F84: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 822E3F88: 93E10088  stw r31, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[31].u32 ) };
	// 822E3F8C: 7FFDFB78  mr r29, r31
	ctx.r[29].u64 = ctx.r[31].u64;
	// 822E3F90: 93E1007C  stw r31, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[31].u32 ) };
	// 822E3F94: 93C10084  stw r30, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 822E3F98: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 822E3F9C: 93A10074  stw r29, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[29].u32 ) };
	// 822E3FA0: 93E10078  stw r31, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 822E3FA4: 41980054  blt cr6, 0x822e3ff8
	if ctx.cr[6].lt {
	pc = 0x822E3FF8; continue 'dispatch;
	}
	// 822E3FA8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 822E3FAC: 7BAB0020  clrldi r11, r29, 0x20
	ctx.r[11].u64 = ctx.r[29].u64 & 0x00000000FFFFFFFFu64;
	// 822E3FB0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822E3FB4: F9610068  std r11, 0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u64 ) };
	// 822E3FB8: C8010068  lfd f0, 0x68(r1)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) };
	// 822E3FBC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 822E3FC0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822E3FC4: FC000018  frsp f0, f0
	ctx.f[0].f64 = (ctx.f[0].f64 as f32) as f64;
	// 822E3FC8: EFA0F7FA  fmadds f29, f0, f31, f30
	ctx.f[29].f64 = (((ctx.f[0].f64 * ctx.f[31].f64 + ctx.f[30].f64) as f32) as f64);
	// 822E3FCC: D3A10060  stfs f29, 0x60(r1)
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 822E3FD0: 4BFF8871  bl 0x822dc840
	ctx.lr = 0x822E3FD4;
	sub_822DC840(ctx, base);
	// 822E3FD4: D3A10060  stfs f29, 0x60(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[29].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 822E3FD8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822E3FDC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822E3FE0: 4BFF8861  bl 0x822dc840
	ctx.lr = 0x822E3FE4;
	sub_822DC840(ctx, base);
	// 822E3FE4: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 822E3FE8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 822E3FEC: 4082FFC0  bne 0x822e3fac
	if !ctx.cr[0].eq {
	pc = 0x822E3FAC; continue 'dispatch;
	}
	// 822E3FF0: 83C10084  lwz r30, 0x84(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 822E3FF4: 83A10074  lwz r29, 0x74(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 822E3FF8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E3FFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E4000: 388BAB28  addi r4, r11, -0x54d8
	ctx.r[4].s64 = ctx.r[11].s64 + -21720;
	// 822E4004: 38A00046  li r5, 0x46
	ctx.r[5].s64 = 70;
	// 822E4008: 38600034  li r3, 0x34
	ctx.r[3].s64 = 52;
	// 822E400C: 4BFDC3CD  bl 0x822c03d8
	ctx.lr = 0x822E4010;
	sub_822C03D8(ctx, base);
	// 822E4010: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E4014: 4182005C  beq 0x822e4070
	if ctx.cr[0].eq {
	pc = 0x822E4070; continue 'dispatch;
	}
	// 822E4018: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822E401C: 409A000C  bne cr6, 0x822e4028
	if !ctx.cr[6].eq {
	pc = 0x822E4028; continue 'dispatch;
	}
	// 822E4020: 7FE9FB78  mr r9, r31
	ctx.r[9].u64 = ctx.r[31].u64;
	// 822E4024: 48000010  b 0x822e4034
	pc = 0x822E4034; continue 'dispatch;
	// 822E4028: 81610078  lwz r11, 0x78(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 822E402C: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 822E4030: 7D691670  srawi r9, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822E4034: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822E4038: 409A000C  bne cr6, 0x822e4044
	if !ctx.cr[6].eq {
	pc = 0x822E4044; continue 'dispatch;
	}
	// 822E403C: 7FE7FB78  mr r7, r31
	ctx.r[7].u64 = ctx.r[31].u64;
	// 822E4040: 48000010  b 0x822e4050
	pc = 0x822E4050; continue 'dispatch;
	// 822E4044: 81610088  lwz r11, 0x88(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 822E4048: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 822E404C: 7D671670  srawi r7, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822E4050: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 822E4054: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822E4058: 7FA8EB78  mr r8, r29
	ctx.r[8].u64 = ctx.r[29].u64;
	// 822E405C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 822E4060: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 822E4064: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822E4068: 4BFFFC89  bl 0x822e3cf0
	ctx.lr = 0x822E406C;
	sub_822E3CF0(ctx, base);
	// 822E406C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E4070: 93F90000  stw r31, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 822E4074: 3BD90004  addi r30, r25, 4
	ctx.r[30].s64 = ctx.r[25].s64 + 4;
	// 822E4078: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E407C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E4080: 4BFFF319  bl 0x822e3398
	ctx.lr = 0x822E4084;
	sub_822E3398(ctx, base);
	// 822E4084: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822E4088: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E408C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E4090: 4BFDBF71  bl 0x822c0000
	ctx.lr = 0x822E4094;
	sub_822C0000(ctx, base);
	// 822E4094: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822E4098: 48185F69  bl 0x8246a000
	ctx.lr = 0x822E409C;
	sub_8246A000(ctx, base);
	// 822E409C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822E40A0: 48185F61  bl 0x8246a000
	ctx.lr = 0x822E40A4;
	sub_8246A000(ctx, base);
	// 822E40A4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 822E40A8: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 822E40AC: CBA1FFA8  lfd f29, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[29].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-88 as u32) ) };
	// 822E40B0: CBC1FFB0  lfd f30, -0x50(r1)
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-80 as u32) ) };
	// 822E40B4: CBE1FFB8  lfd f31, -0x48(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-72 as u32) ) };
	// 822E40B8: 48EC40F4  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E40C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E40C0 size=112
    let mut pc: u32 = 0x822E40C0;
    'dispatch: loop {
        match pc {
            0x822E40C0 => {
    //   block [0x822E40C0..0x822E4130)
	// 822E40C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E40C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E40C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E40CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E40D0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 822E40D4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E40D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E40DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E40E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E40E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E40E8: C3FF0030  lfs f31, 0x30(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 822E40EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E40F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E40F4: 4E800421  bctrl
	ctx.lr = 0x822E40F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E40F8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 822E40FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E4100: 80BF002C  lwz r5, 0x2c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 822E4104: 389F0024  addi r4, r31, 0x24
	ctx.r[4].s64 = ctx.r[31].s64 + 36;
	// 822E4108: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822E410C: 4BFFFE1D  bl 0x822e3f28
	ctx.lr = 0x822E4110;
	sub_822E3F28(ctx, base);
	// 822E4110: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E4114: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E4118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E411C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E4120: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 822E4124: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E4128: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E412C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E4130 size=164
    let mut pc: u32 = 0x822E4130;
    'dispatch: loop {
        match pc {
            0x822E4130 => {
    //   block [0x822E4130..0x822E41D4)
	// 822E4130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E4134: 48EC4031  bl 0x831a8164
	ctx.lr = 0x822E4138;
	sub_831A8130(ctx, base);
	// 822E4138: DBE1FFC8  stfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 822E413C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4140: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E4144: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822E4148: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E414C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822E4150: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E4154: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 822E4158: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E415C: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 822E4160: 4BFEDC21  bl 0x822d1d80
	ctx.lr = 0x822E4164;
	sub_822D1D80(ctx, base);
	// 822E4164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E4168: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822E416C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 822E4170: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E4174: 4BFEDD95  bl 0x822d1f08
	ctx.lr = 0x822E4178;
	sub_822D1F08(ctx, base);
	// 822E4178: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E417C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E4180: 4BFEB221  bl 0x822cf3a0
	ctx.lr = 0x822E4184;
	sub_822CF3A0(ctx, base);
	// 822E4184: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 822E4188: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 822E418C: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822E4190: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822E4194: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E4198: 4BFFFD91  bl 0x822e3f28
	ctx.lr = 0x822E419C;
	sub_822E3F28(ctx, base);
	// 822E419C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822E41A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E41A4: 419A0008  beq cr6, 0x822e41ac
	if ctx.cr[6].eq {
	pc = 0x822E41AC; continue 'dispatch;
	}
	// 822E41A8: 4BFDC6E9  bl 0x822c0890
	ctx.lr = 0x822E41AC;
	sub_822C0890(ctx, base);
	// 822E41AC: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822E41B0: 4BFEDBE9  bl 0x822d1d98
	ctx.lr = 0x822E41B4;
	sub_822D1D98(ctx, base);
	// 822E41B4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E41B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E41BC: 419A0008  beq cr6, 0x822e41c4
	if ctx.cr[6].eq {
	pc = 0x822E41C4; continue 'dispatch;
	}
	// 822E41C0: 4BFDC6D1  bl 0x822c0890
	ctx.lr = 0x822E41C4;
	sub_822C0890(ctx, base);
	// 822E41C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E41C8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 822E41CC: CBE1FFC8  lfd f31, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 822E41D0: 48EC3FE4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E41D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E41D8 size=8
    let mut pc: u32 = 0x822E41D8;
    'dispatch: loop {
        match pc {
            0x822E41D8 => {
    //   block [0x822E41D8..0x822E41E0)
	// 822E41D8: 806300B4  lwz r3, 0xb4(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) } as u64;
	// 822E41DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E41E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E41E0 size=16
    let mut pc: u32 = 0x822E41E0;
    'dispatch: loop {
        match pc {
            0x822E41E0 => {
    //   block [0x822E41E0..0x822E41F0)
	// 822E41E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E41E4: 806B009C  lwz r3, 0x9c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) } as u64;
	// 822E41E8: 808B00A4  lwz r4, 0xa4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(164 as u32) ) } as u64;
	// 822E41EC: 4BFEB1F4  b 0x822cf3e0
	sub_822CF3E0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E41F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E41F0 size=56
    let mut pc: u32 = 0x822E41F0;
    'dispatch: loop {
        match pc {
            0x822E41F0 => {
    //   block [0x822E41F0..0x822E4228)
	// 822E41F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E41F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E41F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E41FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4200: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E4204: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 822E4208: 4BFEBD19  bl 0x822cff20
	ctx.lr = 0x822E420C;
	sub_822CFF20(ctx, base);
	// 822E420C: C01F00D0  lfs f0, 0xd0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E4210: EC21002A  fadds f1, f1, f0
	ctx.f[1].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 822E4214: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E4218: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E421C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E4220: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E4224: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4228(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E4228 size=248
    let mut pc: u32 = 0x822E4228;
    'dispatch: loop {
        match pc {
            0x822E4228 => {
    //   block [0x822E4228..0x822E4320)
	// 822E4228: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E422C: 48EC3F39  bl 0x831a8164
	ctx.lr = 0x822E4230;
	sub_831A8130(ctx, base);
	// 822E4230: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 822E4234: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 822E4238: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E423C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E4240: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E4244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E4248: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 822E424C: 4BFEBCD5  bl 0x822cff20
	ctx.lr = 0x822E4250;
	sub_822CFF20(ctx, base);
	// 822E4250: C01D00D0  lfs f0, 0xd0(r29)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(208 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E4254: EFE1002A  fadds f31, f1, f0
	ctx.f[31].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 822E4258: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822E425C: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822E4260: C3CB08A4  lfs f30, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 822E4264: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 822E4268: 41980028  blt cr6, 0x822e4290
	if ctx.cr[6].lt {
	pc = 0x822E4290; continue 'dispatch;
	}
	// 822E426C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E4270: 4BFEBCA1  bl 0x822cff10
	ctx.lr = 0x822E4274;
	sub_822CFF10(ctx, base);
	// 822E4274: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 822E4278: 41990018  bgt cr6, 0x822e4290
	if ctx.cr[6].gt {
	pc = 0x822E4290; continue 'dispatch;
	}
	// 822E427C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E4280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E4284: 4BFEBE65  bl 0x822d00e8
	ctx.lr = 0x822E4288;
	sub_822D00E8(ctx, base);
	// 822E4288: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822E428C: 48000084  b 0x822e4310
	pc = 0x822E4310; continue 'dispatch;
	// 822E4290: FF1FF000  fcmpu cr6, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 822E4294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E4298: 4199001C  bgt cr6, 0x822e42b4
	if ctx.cr[6].gt {
	pc = 0x822E42B4; continue 'dispatch;
	}
	// 822E429C: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 822E42A0: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 822E42A4: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 822E42A8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822E42AC: 4BFEBE75  bl 0x822d0120
	ctx.lr = 0x822E42B0;
	sub_822D0120(ctx, base);
	// 822E42B0: 48000038  b 0x822e42e8
	pc = 0x822E42E8; continue 'dispatch;
	// 822E42B4: 3BA10070  addi r29, r1, 0x70
	ctx.r[29].s64 = ctx.r[1].s64 + 112;
	// 822E42B8: 3B810080  addi r28, r1, 0x80
	ctx.r[28].s64 = ctx.r[1].s64 + 128;
	// 822E42BC: 3B610060  addi r27, r1, 0x60
	ctx.r[27].s64 = ctx.r[1].s64 + 96;
	// 822E42C0: 4BFEBC51  bl 0x822cff10
	ctx.lr = 0x822E42C4;
	sub_822CFF10(ctx, base);
	// 822E42C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E42C8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 822E42CC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 822E42D0: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 822E42D4: 4BFEBE4D  bl 0x822d0120
	ctx.lr = 0x822E42D8;
	sub_822D0120(ctx, base);
	// 822E42D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E42DC: 4BFEBC35  bl 0x822cff10
	ctx.lr = 0x822E42E0;
	sub_822CFF10(ctx, base);
	// 822E42E0: EC1F0828  fsubs f0, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 822E42E4: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822E42E8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 822E42EC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 822E42F0: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 822E42F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822E42F8: 13E05C07  vcmpneb. (lvlx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E4320 size=280
    let mut pc: u32 = 0x822E4320;
    'dispatch: loop {
        match pc {
            0x822E4320 => {
    //   block [0x822E4320..0x822E4438)
	// 822E4320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E4324: 48EC3E41  bl 0x831a8164
	ctx.lr = 0x822E4328;
	sub_831A8130(ctx, base);
	// 822E4328: DBC1FFC0  stfd f30, -0x40(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-64 as u32), ctx.f[30].u64 ) };
	// 822E432C: DBE1FFC8  stfd f31, -0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[31].u64 ) };
	// 822E4330: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4334: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E4338: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 822E433C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E4340: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E4344: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 822E4348: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 822E434C: 4BFEBBD5  bl 0x822cff20
	ctx.lr = 0x822E4350;
	sub_822CFF20(ctx, base);
	// 822E4350: C01B00D0  lfs f0, 0xd0(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(208 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E4354: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822E4358: EFE1002A  fadds f31, f1, f0
	ctx.f[31].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 822E435C: D3E10050  stfs f31, 0x50(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822E4360: C3CB08A4  lfs f30, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 822E4364: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 822E4368: 41980030  blt cr6, 0x822e4398
	if ctx.cr[6].lt {
	pc = 0x822E4398; continue 'dispatch;
	}
	// 822E436C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E4370: 4BFEBBA1  bl 0x822cff10
	ctx.lr = 0x822E4374;
	sub_822CFF10(ctx, base);
	// 822E4374: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 822E4378: 41990020  bgt cr6, 0x822e4398
	if ctx.cr[6].gt {
	pc = 0x822E4398; continue 'dispatch;
	}
	// 822E437C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 822E4380: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822E4384: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E4388: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E438C: 4BFEBCCD  bl 0x822d0058
	ctx.lr = 0x822E4390;
	sub_822D0058(ctx, base);
	// 822E4390: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822E4394: 48000094  b 0x822e4428
	pc = 0x822E4428; continue 'dispatch;
	// 822E4398: FF1FF000  fcmpu cr6, f31, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 822E439C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E43A0: 41990038  bgt cr6, 0x822e43d8
	if ctx.cr[6].gt {
	pc = 0x822E43D8; continue 'dispatch;
	}
	// 822E43A4: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 822E43A8: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 822E43AC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 822E43B0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822E43B4: 4BFEBD6D  bl 0x822d0120
	ctx.lr = 0x822E43B8;
	sub_822D0120(ctx, base);
	// 822E43B8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 822E43BC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E4438 size=16
    let mut pc: u32 = 0x822E4438;
    'dispatch: loop {
        match pc {
            0x822E4438 => {
    //   block [0x822E4438..0x822E4448)
	// 822E4438: 816300AC  lwz r11, 0xac(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 822E443C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 822E4440: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 822E4444: 4BFFFDE4  b 0x822e4228
	sub_822E4228(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E4448 size=24
    let mut pc: u32 = 0x822E4448;
    'dispatch: loop {
        match pc {
            0x822E4448 => {
    //   block [0x822E4448..0x822E4460)
	// 822E4448: 816300AC  lwz r11, 0xac(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(172 as u32) ) } as u64;
	// 822E444C: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 822E4450: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 822E4454: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 822E4458: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 822E445C: 4BFFFEC4  b 0x822e4320
	sub_822E4320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E4460 size=16
    let mut pc: u32 = 0x822E4460;
    'dispatch: loop {
        match pc {
            0x822E4460 => {
    //   block [0x822E4460..0x822E4470)
	// 822E4460: 816300B4  lwz r11, 0xb4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) } as u64;
	// 822E4464: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 822E4468: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 822E446C: 4BFFFDBC  b 0x822e4228
	sub_822E4228(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E4470 size=24
    let mut pc: u32 = 0x822E4470;
    'dispatch: loop {
        match pc {
            0x822E4470 => {
    //   block [0x822E4470..0x822E4488)
	// 822E4470: 816300B4  lwz r11, 0xb4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(180 as u32) ) } as u64;
	// 822E4474: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 822E4478: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 822E447C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 822E4480: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 822E4484: 4BFFFE9C  b 0x822e4320
	sub_822E4320(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4488(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E4488 size=196
    let mut pc: u32 = 0x822E4488;
    'dispatch: loop {
        match pc {
            0x822E4488 => {
    //   block [0x822E4488..0x822E454C)
	// 822E4488: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E448C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E4490: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E4494: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E4498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E449C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E44A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E44A4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 822E44A8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E44AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E44B0: 4BFDC489  bl 0x822c0938
	ctx.lr = 0x822E44B4;
	sub_822C0938(ctx, base);
	// 822E44B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E44B8: 41820028  beq 0x822e44e0
	if ctx.cr[0].eq {
	pc = 0x822E44E0; continue 'dispatch;
	}
	// 822E44BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E44C0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822E44C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822E44C8: 392BAB98  addi r9, r11, -0x5468
	ctx.r[9].s64 = ctx.r[11].s64 + -21608;
	// 822E44CC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822E44D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E44D4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E44D8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822E44DC: 48000008  b 0x822e44e4
	pc = 0x822E44E4; continue 'dispatch;
	// 822E44E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E44E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E44E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E44EC: 409A0044  bne cr6, 0x822e4530
	if !ctx.cr[6].eq {
	pc = 0x822E4530; continue 'dispatch;
	}
	// 822E44F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E44F4: 419A001C  beq cr6, 0x822e4510
	if ctx.cr[6].eq {
	pc = 0x822E4510; continue 'dispatch;
	}
	// 822E44F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E44FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822E4500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E4504: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E4508: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E450C: 4E800421  bctrl
	ctx.lr = 0x822E4510;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E4510: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E4514: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822E4518: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E451C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 822E4520: 816B3C3C  lwz r11, 0x3c3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15420 as u32) ) } as u64;
	// 822E4524: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822E4528: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822E452C: 4BFDBAD5  bl 0x822c0000
	ctx.lr = 0x822E4530;
	sub_822C0000(ctx, base);
	// 822E4530: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E4534: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E4538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E453C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E4540: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E4544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E4548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E4550 size=196
    let mut pc: u32 = 0x822E4550;
    'dispatch: loop {
        match pc {
            0x822E4550 => {
    //   block [0x822E4550..0x822E4614)
	// 822E4550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E4554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E4558: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E455C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E4560: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4564: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E4568: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E456C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 822E4570: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E4574: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E4578: 4BFDC3C1  bl 0x822c0938
	ctx.lr = 0x822E457C;
	sub_822C0938(ctx, base);
	// 822E457C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E4580: 41820028  beq 0x822e45a8
	if ctx.cr[0].eq {
	pc = 0x822E45A8; continue 'dispatch;
	}
	// 822E4584: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E4588: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822E458C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822E4590: 392BABAC  addi r9, r11, -0x5454
	ctx.r[9].s64 = ctx.r[11].s64 + -21588;
	// 822E4594: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822E4598: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E459C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E45A0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822E45A4: 48000008  b 0x822e45ac
	pc = 0x822E45AC; continue 'dispatch;
	// 822E45A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E45AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E45B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E45B4: 409A0044  bne cr6, 0x822e45f8
	if !ctx.cr[6].eq {
	pc = 0x822E45F8; continue 'dispatch;
	}
	// 822E45B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E45BC: 419A001C  beq cr6, 0x822e45d8
	if ctx.cr[6].eq {
	pc = 0x822E45D8; continue 'dispatch;
	}
	// 822E45C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E45C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822E45C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E45CC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E45D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E45D4: 4E800421  bctrl
	ctx.lr = 0x822E45D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E45D8: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E45DC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822E45E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E45E4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 822E45E8: 816B3C3C  lwz r11, 0x3c3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15420 as u32) ) } as u64;
	// 822E45EC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822E45F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822E45F4: 4BFDBA0D  bl 0x822c0000
	ctx.lr = 0x822E45F8;
	sub_822C0000(ctx, base);
	// 822E45F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E45FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E4600: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E4604: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E4608: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E460C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E4610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E4618 size=76
    let mut pc: u32 = 0x822E4618;
    'dispatch: loop {
        match pc {
            0x822E4618 => {
    //   block [0x822E4618..0x822E4664)
	// 822E4618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E461C: 48EC3B51  bl 0x831a816c
	ctx.lr = 0x822E4620;
	sub_831A8130(ctx, base);
	// 822E4620: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4624: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E4628: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822E462C: 809F00AC  lwz r4, 0xac(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 822E4630: 4BFFFBF9  bl 0x822e4228
	ctx.lr = 0x822E4634;
	sub_822E4228(ctx, base);
	// 822E4634: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 822E4638: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 822E463C: 83BF00B4  lwz r29, 0xb4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 822E4640: 4BFEB8F9  bl 0x822cff38
	ctx.lr = 0x822E4644;
	sub_822CFF38(ctx, base);
	// 822E4644: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E4648: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E464C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822E4650: 4BFEC1E1  bl 0x822d0830
	ctx.lr = 0x822E4654;
	sub_822D0830(ctx, base);
	// 822E4654: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 822E4658: 4BFEB8D9  bl 0x822cff30
	ctx.lr = 0x822E465C;
	sub_822CFF30(ctx, base);
	// 822E465C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E4660: 48EC3B5C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E4668 size=96
    let mut pc: u32 = 0x822E4668;
    'dispatch: loop {
        match pc {
            0x822E4668 => {
    //   block [0x822E4668..0x822E46C8)
	// 822E4668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E466C: 48EC3B01  bl 0x831a816c
	ctx.lr = 0x822E4670;
	sub_831A8130(ctx, base);
	// 822E4670: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4674: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E4678: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822E467C: 809F00B4  lwz r4, 0xb4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 822E4680: 4BFFFBA9  bl 0x822e4228
	ctx.lr = 0x822E4684;
	sub_822E4228(ctx, base);
	// 822E4684: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 822E4688: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 822E468C: 4BFEBA5D  bl 0x822d00e8
	ctx.lr = 0x822E4690;
	sub_822D00E8(ctx, base);
	// 822E4690: C0010064  lfs f0, 0x64(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E4694: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 822E4698: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 822E469C: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 822E46A0: 83BF00AC  lwz r29, 0xac(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 822E46A4: 4BFEB895  bl 0x822cff38
	ctx.lr = 0x822E46A8;
	sub_822CFF38(ctx, base);
	// 822E46A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E46AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E46B0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822E46B4: 4BFEC17D  bl 0x822d0830
	ctx.lr = 0x822E46B8;
	sub_822D0830(ctx, base);
	// 822E46B8: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 822E46BC: 4BFEB875  bl 0x822cff30
	ctx.lr = 0x822E46C0;
	sub_822CFF30(ctx, base);
	// 822E46C0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 822E46C4: 48EC3AF8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E46C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E46C8 size=224
    let mut pc: u32 = 0x822E46C8;
    'dispatch: loop {
        match pc {
            0x822E46C8 => {
    //   block [0x822E46C8..0x822E47A8)
	// 822E46C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E46CC: 48EC3A91  bl 0x831a815c
	ctx.lr = 0x822E46D0;
	sub_831A8130(ctx, base);
	// 822E46D0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E46D4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E46D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E46DC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E46E0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 822E46E4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 822E46E8: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E46EC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 822E46F0: 4BFEB149  bl 0x822cf838
	ctx.lr = 0x822E46F4;
	sub_822CF838(ctx, base);
	// 822E46F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E46F8: 3B9F00AC  addi r28, r31, 0xac
	ctx.r[28].s64 = ctx.r[31].s64 + 172;
	// 822E46FC: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 822E4700: 387C0004  addi r3, r28, 4
	ctx.r[3].s64 = ctx.r[28].s64 + 4;
	// 822E4704: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E4708: 917F00AC  stw r11, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 822E470C: 4BFDFD55  bl 0x822c4460
	ctx.lr = 0x822E4710;
	sub_822C4460(ctx, base);
	// 822E4710: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822E4714: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E4718: 419A0008  beq cr6, 0x822e4720
	if ctx.cr[6].eq {
	pc = 0x822E4720; continue 'dispatch;
	}
	// 822E471C: 4BFDC175  bl 0x822c0890
	ctx.lr = 0x822E4720;
	sub_822C0890(ctx, base);
	// 822E4720: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 822E4724: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E4728: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822E472C: 4BFEB10D  bl 0x822cf838
	ctx.lr = 0x822E4730;
	sub_822CF838(ctx, base);
	// 822E4730: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E4734: 3BBF00B4  addi r29, r31, 0xb4
	ctx.r[29].s64 = ctx.r[31].s64 + 180;
	// 822E4738: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 822E473C: 387D0004  addi r3, r29, 4
	ctx.r[3].s64 = ctx.r[29].s64 + 4;
	// 822E4740: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E4744: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 822E4748: 4BFDFD19  bl 0x822c4460
	ctx.lr = 0x822E474C;
	sub_822C4460(ctx, base);
	// 822E474C: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 822E4750: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E4754: 419A0008  beq cr6, 0x822e475c
	if ctx.cr[6].eq {
	pc = 0x822E475C; continue 'dispatch;
	}
	// 822E4758: 4BFDC139  bl 0x822c0890
	ctx.lr = 0x822E475C;
	sub_822C0890(ctx, base);
	// 822E475C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E4760: 397F009C  addi r11, r31, 0x9c
	ctx.r[11].s64 = ctx.r[31].s64 + 156;
	// 822E4764: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 822E4768: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 822E476C: 915F009C  stw r10, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 822E4770: 4BFDFCF1  bl 0x822c4460
	ctx.lr = 0x822E4774;
	sub_822C4460(ctx, base);
	// 822E4774: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822E4778: 935F00A4  stw r26, 0xa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(164 as u32), ctx.r[26].u32 ) };
	// 822E477C: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 822E4780: 937F00A8  stw r27, 0xa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(168 as u32), ctx.r[27].u32 ) };
	// 822E4784: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E4788: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E478C: D01F00D0  stfs f0, 0xd0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 822E4790: 4BFEC1D9  bl 0x822d0968
	ctx.lr = 0x822E4794;
	sub_822D0968(ctx, base);
	// 822E4794: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 822E4798: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E479C: 4BFEC1CD  bl 0x822d0968
	ctx.lr = 0x822E47A0;
	sub_822D0968(ctx, base);
	// 822E47A0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 822E47A4: 48EC3A08  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E47A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E47A8 size=156
    let mut pc: u32 = 0x822E47A8;
    'dispatch: loop {
        match pc {
            0x822E47A8 => {
    //   block [0x822E47A8..0x822E4844)
	// 822E47A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E47AC: 48EC39C1  bl 0x831a816c
	ctx.lr = 0x822E47B0;
	sub_831A8130(ctx, base);
	// 822E47B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E47B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E47B8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E47BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E47C0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E47C4: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 822E47C8: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 822E47CC: 38600040  li r3, 0x40
	ctx.r[3].s64 = 64;
	// 822E47D0: 48B0DC19  bl 0x82df23e8
	ctx.lr = 0x822E47D4;
	sub_82DF23E8(ctx, base);
	// 822E47D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E47D8: 41820038  beq 0x822e4810
	if ctx.cr[0].eq {
	pc = 0x822E4810; continue 'dispatch;
	}
	// 822E47DC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822E47E0: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E47E4: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822E47E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822E47EC: C1BF0004  lfs f13, 4(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E47F0: C19F0008  lfs f12, 8(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 822E47F4: D1A10054  stfs f13, 0x54(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 822E47F8: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E47FC: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 822E4800: D1810058  stfs f12, 0x58(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 822E4804: 48871CAD  bl 0x82b564b0
	ctx.lr = 0x822E4808;
	sub_82B564B0(ctx, base);
	// 822E4808: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E480C: 48000008  b 0x822e4814
	pc = 0x822E4814; continue 'dispatch;
	// 822E4810: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822E4814: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 822E4818: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 822E481C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E4820: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E4824: 4BFFFC65  bl 0x822e4488
	ctx.lr = 0x822E4828;
	sub_822E4488(ctx, base);
	// 822E4828: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822E482C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E4830: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E4834: 4BFDB7CD  bl 0x822c0000
	ctx.lr = 0x822E4838;
	sub_822C0000(ctx, base);
	// 822E4838: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E483C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E4840: 48EC397C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E4848 size=136
    let mut pc: u32 = 0x822E4848;
    'dispatch: loop {
        match pc {
            0x822E4848 => {
    //   block [0x822E4848..0x822E48D0)
	// 822E4848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E484C: 48EC3921  bl 0x831a816c
	ctx.lr = 0x822E4850;
	sub_831A8130(ctx, base);
	// 822E4850: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4854: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E4858: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E485C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E4860: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 822E4864: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 822E4868: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 822E486C: 48B0DB7D  bl 0x82df23e8
	ctx.lr = 0x822E4870;
	sub_82DF23E8(ctx, base);
	// 822E4870: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822E4874: 41820028  beq 0x822e489c
	if ctx.cr[0].eq {
	pc = 0x822E489C; continue 'dispatch;
	}
	// 822E4878: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E487C: 48B74E25  bl 0x82e596a0
	ctx.lr = 0x822E4880;
	sub_82E596A0(ctx, base);
	// 822E4880: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E4884: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E4888: 394AABC0  addi r10, r10, -0x5440
	ctx.r[10].s64 = ctx.r[10].s64 + -21568;
	// 822E488C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822E4890: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 822E4894: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 822E4898: 48000008  b 0x822e48a0
	pc = 0x822E48A0; continue 'dispatch;
	// 822E489C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822E48A0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 822E48A4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 822E48A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E48AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E48B0: 4BFFFCA1  bl 0x822e4550
	ctx.lr = 0x822E48B4;
	sub_822E4550(ctx, base);
	// 822E48B4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822E48B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E48BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E48C0: 4BFDB741  bl 0x822c0000
	ctx.lr = 0x822E48C4;
	sub_822C0000(ctx, base);
	// 822E48C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E48C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E48CC: 48EC38F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E48D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E48D0 size=608
    let mut pc: u32 = 0x822E48D0;
    'dispatch: loop {
        match pc {
            0x822E48D0 => {
    //   block [0x822E48D0..0x822E4B30)
	// 822E48D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E48D4: 48EC3885  bl 0x831a8158
	ctx.lr = 0x822E48D8;
	sub_831A8130(ctx, base);
	// 822E48D8: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E48DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E48E0: 13E028C7  vcmpequd (lvx128) v31, v0, v5
	tmp.u32 = ctx.r[5].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E48E4: 396000C0  li r11, 0xc0
	ctx.r[11].s64 = 192;
	// 822E48E8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E48EC: 3D208200  lis r9, -0x7e00
	ctx.r[9].s64 = -2113929216;
	// 822E48F0: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
	// 822E48F4: 3BA10070  addi r29, r1, 0x70
	ctx.r[29].s64 = ctx.r[1].s64 + 112;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4B30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E4B30 size=328
    let mut pc: u32 = 0x822E4B30;
    'dispatch: loop {
        match pc {
            0x822E4B30 => {
    //   block [0x822E4B30..0x822E4C78)
	// 822E4B30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E4B34: 48EC3639  bl 0x831a816c
	ctx.lr = 0x822E4B38;
	sub_831A8130(ctx, base);
	// 822E4B38: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 822E4B3C: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 822E4B40: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4B44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E4B48: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822E4B4C: 3BDF00D0  addi r30, r31, 0xd0
	ctx.r[30].s64 = ctx.r[31].s64 + 208;
	// 822E4B50: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 822E4B54: 4BFEB3CD  bl 0x822cff20
	ctx.lr = 0x822E4B58;
	sub_822CFF20(ctx, base);
	// 822E4B58: C01F00D0  lfs f0, 0xd0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E4B5C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822E4B60: EC01002A  fadds f0, f1, f0
	ctx.f[0].f64 = ((ctx.f[1].f64 + ctx.f[0].f64) as f32) as f64;
	// 822E4B64: C3CB08A4  lfs f30, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[30].f64 = (tmp.f32 as f64);
	// 822E4B68: EFE0F82A  fadds f31, f0, f31
	ctx.f[31].f64 = ((ctx.f[0].f64 + ctx.f[31].f64) as f32) as f64;
	// 822E4B6C: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 822E4B70: 40990028  ble cr6, 0x822e4b98
	if !ctx.cr[6].gt {
	pc = 0x822E4B98; continue 'dispatch;
	}
	// 822E4B74: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 822E4B78: 4BFEB399  bl 0x822cff10
	ctx.lr = 0x822E4B7C;
	sub_822CFF10(ctx, base);
	// 822E4B7C: FF1F0800  fcmpu cr6, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[1].f64);
	// 822E4B80: 40980010  bge cr6, 0x822e4b90
	if !ctx.cr[6].lt {
	pc = 0x822E4B90; continue 'dispatch;
	}
	// 822E4B84: D3DE0000  stfs f30, 0(r30)
	tmp.f32 = (ctx.f[30].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E4B88: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822E4B8C: 480000DC  b 0x822e4c68
	pc = 0x822E4C68; continue 'dispatch;
	// 822E4B90: FF1FF000  fcmpu cr6, f31, f30
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[30].f64);
	// 822E4B94: 41990014  bgt cr6, 0x822e4ba8
	if ctx.cr[6].gt {
	pc = 0x822E4BA8; continue 'dispatch;
	}
	// 822E4B98: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 822E4B9C: D3FE0000  stfs f31, 0(r30)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E4BA0: FC20F090  fmr f1, f30
	ctx.f[1].f64 = ctx.f[30].f64;
	// 822E4BA4: 48000024  b 0x822e4bc8
	pc = 0x822E4BC8; continue 'dispatch;
	// 822E4BA8: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 822E4BAC: 4BFEB365  bl 0x822cff10
	ctx.lr = 0x822E4BB0;
	sub_822CFF10(ctx, base);
	// 822E4BB0: EC1F0828  fsubs f0, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 822E4BB4: D01E0000  stfs f0, 0(r30)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), tmp.u32 ) };
	// 822E4BB8: 83BF00B4  lwz r29, 0xb4(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 822E4BBC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E4BC0: 4BFEB351  bl 0x822cff10
	ctx.lr = 0x822E4BC4;
	sub_822CFF10(ctx, base);
	// 822E4BC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E4BC8: 4BFEB369  bl 0x822cff30
	ctx.lr = 0x822E4BCC;
	sub_822CFF30(ctx, base);
	// 822E4BCC: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 822E4BD0: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 822E4BD4: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 822E4BD8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 822E4BDC: 4BFEB47D  bl 0x822d0058
	ctx.lr = 0x822E4BE0;
	sub_822D0058(ctx, base);
	// 822E4BE0: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 822E4BE4: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 822E4BE8: 13E0F407  vcmpneb. (lvlx128) v31, v0, v30
	tmp.u32 = ctx.r[30].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4C78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E4C78 size=84
    let mut pc: u32 = 0x822E4C78;
    'dispatch: loop {
        match pc {
            0x822E4C78 => {
    //   block [0x822E4C78..0x822E4CCC)
	// 822E4C78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E4C7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E4C80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E4C84: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 822E4C88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4C8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E4C90: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 822E4C94: 4BFFFE9D  bl 0x822e4b30
	ctx.lr = 0x822E4C98;
	sub_822E4B30(ctx, base);
	// 822E4C98: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E4C9C: 40820018  bne 0x822e4cb4
	if !ctx.cr[0].eq {
	pc = 0x822E4CB4; continue 'dispatch;
	}
	// 822E4CA0: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 822E4CA4: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 822E4CA8: 4BFEB259  bl 0x822cff00
	ctx.lr = 0x822E4CAC;
	sub_822CFF00(ctx, base);
	// 822E4CAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E4CB0: 4BFFF9B9  bl 0x822e4668
	ctx.lr = 0x822E4CB4;
	sub_822E4668(ctx, base);
	// 822E4CB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E4CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E4CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E4CC0: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E4CC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E4CC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E4CD0 size=184
    let mut pc: u32 = 0x822E4CD0;
    'dispatch: loop {
        match pc {
            0x822E4CD0 => {
    //   block [0x822E4CD0..0x822E4D88)
	// 822E4CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E4CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E4CD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E4CDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E4CE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4CE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E4CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E4CEC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 822E4CF0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E4CF4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E4CF8: 4BFDBC41  bl 0x822c0938
	ctx.lr = 0x822E4CFC;
	sub_822C0938(ctx, base);
	// 822E4CFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E4D00: 41820028  beq 0x822e4d28
	if ctx.cr[0].eq {
	pc = 0x822E4D28; continue 'dispatch;
	}
	// 822E4D04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E4D08: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822E4D0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822E4D10: 392BAB84  addi r9, r11, -0x547c
	ctx.r[9].s64 = ctx.r[11].s64 + -21628;
	// 822E4D14: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822E4D18: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E4D1C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E4D20: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822E4D24: 48000008  b 0x822e4d2c
	pc = 0x822E4D2C; continue 'dispatch;
	// 822E4D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E4D2C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E4D30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E4D34: 409A0038  bne cr6, 0x822e4d6c
	if !ctx.cr[6].eq {
	pc = 0x822E4D6C; continue 'dispatch;
	}
	// 822E4D38: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E4D3C: 419A0010  beq cr6, 0x822e4d4c
	if ctx.cr[6].eq {
	pc = 0x822E4D4C; continue 'dispatch;
	}
	// 822E4D40: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822E4D44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E4D48: 487A6879  bl 0x82a8b5c0
	ctx.lr = 0x822E4D4C;
	sub_82A8B5C0(ctx, base);
	// 822E4D4C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E4D50: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822E4D54: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E4D58: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 822E4D5C: 816B3C3C  lwz r11, 0x3c3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15420 as u32) ) } as u64;
	// 822E4D60: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822E4D64: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822E4D68: 4BFDB299  bl 0x822c0000
	ctx.lr = 0x822E4D6C;
	sub_822C0000(ctx, base);
	// 822E4D6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E4D70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E4D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E4D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E4D7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E4D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E4D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E4D88 size=92
    let mut pc: u32 = 0x822E4D88;
    'dispatch: loop {
        match pc {
            0x822E4D88 => {
    //   block [0x822E4D88..0x822E4DE4)
	// 822E4D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E4D8C: 48EC33E1  bl 0x831a816c
	ctx.lr = 0x822E4D90;
	sub_831A8130(ctx, base);
	// 822E4D90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4D94: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E4D98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E4D9C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E4DA0: 2B1E0040  cmplwi cr6, r30, 0x40
	ctx.cr[6].compare_u32(ctx.r[30].u32, 64 as u32, &mut ctx.xer);
	// 822E4DA4: 41980008  blt cr6, 0x822e4dac
	if ctx.cr[6].lt {
	pc = 0x822E4DAC; continue 'dispatch;
	}
	// 822E4DA8: 4803BA61  bl 0x82320808
	ctx.lr = 0x822E4DAC;
	sub_82320808(ctx, base);
	// 822E4DAC: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E4DB0: 57CBE8FA  rlwinm r11, r30, 0x1d, 3, 0x1d
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000007u64;
	// 822E4DB4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 822E4DB8: 57CA06FE  clrlwi r10, r30, 0x1b
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x0000001Fu64;
	// 822E4DBC: 7D2A5030  slw r10, r9, r10
	if (ctx.r[10].u8 & 0x20) != 0 {
		ctx.r[10].u64 = 0;
	} else {
		ctx.r[10].u64 = ((ctx.r[9].u32) << ((ctx.r[10].u8 & 0x1F) as u32)) as u64;
	}
	// 822E4DC0: 7D2BF82E  lwzx r9, r11, r31
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 822E4DC4: 4182000C  beq 0x822e4dd0
	if ctx.cr[0].eq {
	pc = 0x822E4DD0; continue 'dispatch;
	}
	// 822E4DC8: 7D4A4B78  or r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[9].u64;
	// 822E4DCC: 48000008  b 0x822e4dd4
	pc = 0x822E4DD4; continue 'dispatch;
	// 822E4DD0: 7D2A5078  andc r10, r9, r10
	ctx.r[10].u64 = ctx.r[9].u64 & !ctx.r[10].u64;
	// 822E4DD4: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 822E4DD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E4DDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E4DE0: 48EC33DC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E4DE8 size=112
    let mut pc: u32 = 0x822E4DE8;
    'dispatch: loop {
        match pc {
            0x822E4DE8 => {
    //   block [0x822E4DE8..0x822E4E58)
	// 822E4DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E4DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E4DF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E4DF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E4DF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4DFC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E4E00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E4E04: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 822E4E08: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822E4E0C: 4BFFFEC5  bl 0x822e4cd0
	ctx.lr = 0x822E4E10;
	sub_822E4CD0(ctx, base);
	// 822E4E10: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822E4E14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E4E18: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822E4E1C: 4BFDB1E5  bl 0x822c0000
	ctx.lr = 0x822E4E20;
	sub_822C0000(ctx, base);
	// 822E4E20: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E4E24: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822E4E28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E4E2C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E4E30: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E4E34: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822E4E38: 419A0008  beq cr6, 0x822e4e40
	if ctx.cr[6].eq {
	pc = 0x822E4E40; continue 'dispatch;
	}
	// 822E4E3C: 4BFDBA55  bl 0x822c0890
	ctx.lr = 0x822E4E40;
	sub_822C0890(ctx, base);
	// 822E4E40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E4E44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E4E48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E4E4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E4E50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E4E54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E4E58 size=104
    let mut pc: u32 = 0x822E4E58;
    'dispatch: loop {
        match pc {
            0x822E4E58 => {
    //   block [0x822E4E58..0x822E4EC0)
	// 822E4E58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E4E5C: 48EC3311  bl 0x831a816c
	ctx.lr = 0x822E4E60;
	sub_831A8130(ctx, base);
	// 822E4E60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4E64: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E4E68: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822E4E6C: 395D0004  addi r10, r29, 4
	ctx.r[10].s64 = ctx.r[29].s64 + 4;
	// 822E4E70: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 822E4E74: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E4E78: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E4E7C: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 822E4E80: 4080FFF4  bge 0x822e4e74
	if !ctx.cr[0].lt {
	pc = 0x822E4E74; continue 'dispatch;
	}
	// 822E4E84: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 822E4E88: 913D0000  stw r9, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E4E8C: 419A0028  beq cr6, 0x822e4eb4
	if ctx.cr[6].eq {
	pc = 0x822E4EB4; continue 'dispatch;
	}
	// 822E4E90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E4E94: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 822E4E98: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E4E9C: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E4EA0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E4EA4: 4BFFFEE5  bl 0x822e4d88
	ctx.lr = 0x822E4EA8;
	sub_822E4D88(ctx, base);
	// 822E4EA8: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822E4EAC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 822E4EB0: 4082FFE8  bne 0x822e4e98
	if !ctx.cr[0].eq {
	pc = 0x822E4E98; continue 'dispatch;
	}
	// 822E4EB4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E4EB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E4EBC: 48EC3300  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E4EC0 size=108
    let mut pc: u32 = 0x822E4EC0;
    'dispatch: loop {
        match pc {
            0x822E4EC0 => {
    //   block [0x822E4EC0..0x822E4F2C)
	// 822E4EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E4EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E4EC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E4ECC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4ED0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E4ED4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822E4ED8: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 822E4EDC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 822E4EE0: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E4EE4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E4EE8: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 822E4EEC: 4080FFF0  bge 0x822e4edc
	if !ctx.cr[0].lt {
	pc = 0x822E4EDC; continue 'dispatch;
	}
	// 822E4EF0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822E4EF4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E4EF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E4EFC: 4BFFFF5D  bl 0x822e4e58
	ctx.lr = 0x822E4F00;
	sub_822E4E58(ctx, base);
	// 822E4F00: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E4F04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E4F08: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E4F0C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822E4F10: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E4F14: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E4F18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E4F1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E4F20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E4F24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E4F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E4F30 size=112
    let mut pc: u32 = 0x822E4F30;
    'dispatch: loop {
        match pc {
            0x822E4F30 => {
    //   block [0x822E4F30..0x822E4FA0)
	// 822E4F30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E4F34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E4F38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E4F3C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4F40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E4F44: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822E4F48: 395F0004  addi r10, r31, 4
	ctx.r[10].s64 = ctx.r[31].s64 + 4;
	// 822E4F4C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 822E4F50: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E4F54: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E4F58: 394AFFFC  addi r10, r10, -4
	ctx.r[10].s64 = ctx.r[10].s64 + -4;
	// 822E4F5C: 4080FFF0  bge 0x822e4f4c
	if !ctx.cr[0].lt {
	pc = 0x822E4F4C; continue 'dispatch;
	}
	// 822E4F60: 90810050  stw r4, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u32 ) };
	// 822E4F64: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E4F68: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822E4F6C: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822E4F70: 4BFFFEE9  bl 0x822e4e58
	ctx.lr = 0x822E4F74;
	sub_822E4E58(ctx, base);
	// 822E4F74: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E4F78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E4F7C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E4F80: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822E4F84: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E4F88: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E4F8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E4F90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E4F94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E4F98: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E4F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E4FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E4FA0 size=360
    let mut pc: u32 = 0x822E4FA0;
    'dispatch: loop {
        match pc {
            0x822E4FA0 => {
    //   block [0x822E4FA0..0x822E5108)
	// 822E4FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E4FA4: 48EC31BD  bl 0x831a8160
	ctx.lr = 0x822E4FA8;
	sub_831A8130(ctx, base);
	// 822E4FA8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E4FAC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822E4FB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E4FB4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 822E4FB8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 822E4FBC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E4FC0: 48B73F31  bl 0x82e58ef0
	ctx.lr = 0x822E4FC4;
	sub_82E58EF0(ctx, base);
	// 822E4FC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E4FC8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822E4FCC: 396BAC30  addi r11, r11, -0x53d0
	ctx.r[11].s64 = ctx.r[11].s64 + -21456;
	// 822E4FD0: 3B9F0094  addi r28, r31, 0x94
	ctx.r[28].s64 = ctx.r[31].s64 + 148;
	// 822E4FD4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E4FD8: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 822E4FDC: 93DF0098  stw r30, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[30].u32 ) };
	// 822E4FE0: C00A08A4  lfs f0, 0x8a4(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E4FE4: 93DF009C  stw r30, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[30].u32 ) };
	// 822E4FE8: 93DF00A0  stw r30, 0xa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(160 as u32), ctx.r[30].u32 ) };
	// 822E4FEC: 93DF00AC  stw r30, 0xac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(172 as u32), ctx.r[30].u32 ) };
	// 822E4FF0: 93DF00B0  stw r30, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 822E4FF4: 93DF00B4  stw r30, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[30].u32 ) };
	// 822E4FF8: 93DF00B8  stw r30, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[30].u32 ) };
	// 822E4FFC: D01F00D0  stfs f0, 0xd0(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), tmp.u32 ) };
	// 822E5000: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5004: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E5008: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 822E500C: 409A0008  bne cr6, 0x822e5014
	if !ctx.cr[6].eq {
	pc = 0x822E5014; continue 'dispatch;
	}
	// 822E5010: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E5014: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E5018: 48223789  bl 0x825087a0
	ctx.lr = 0x822E501C;
	sub_825087A0(ctx, base);
	// 822E501C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E5020: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E5024: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E5028: 388AABC8  addi r4, r10, -0x5438
	ctx.r[4].s64 = ctx.r[10].s64 + -21560;
	// 822E502C: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 822E5030: C00BA1C4  lfs f0, -0x5e3c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24124 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E5034: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 822E5038: D01F00D4  stfs f0, 0xd4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), tmp.u32 ) };
	// 822E503C: 4BFDB39D  bl 0x822c03d8
	ctx.lr = 0x822E5040;
	sub_822C03D8(ctx, base);
	// 822E5040: 7C7D1B79  or. r29, r3, r3
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822E5044: 41820094  beq 0x822e50d8
	if ctx.cr[0].eq {
	pc = 0x822E50D8; continue 'dispatch;
	}
	// 822E5048: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E504C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822E5050: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 822E5054: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 822E5058: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822E505C: 48BBB6D5  bl 0x82ea0730
	ctx.lr = 0x822E5060;
	sub_82EA0730(ctx, base);
	// 822E5060: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 822E5064: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 822E5068: C03F00D4  lfs f1, 0xd4(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822E506C: 48C35185  bl 0x82f1a1f0
	ctx.lr = 0x822E5070;
	sub_82F1A1F0(ctx, base);
	// 822E5070: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 822E5074: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822E5078: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822E507C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 822E5080: 808B679C  lwz r4, 0x679c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(26524 as u32) ) } as u64;
	// 822E5084: 4BFFFEAD  bl 0x822e4f30
	ctx.lr = 0x822E5088;
	sub_822E4F30(ctx, base);
	// 822E5088: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 822E508C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822E5090: 4BFFFE31  bl 0x822e4ec0
	ctx.lr = 0x822E5094;
	sub_822E4EC0(ctx, base);
	// 822E5094: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 822E5098: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E509C: E89A0000  ld r4, 0(r26)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) };
	// 822E50A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822E50A4: E8630000  ld r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 822E50A8: 481A71E1  bl 0x8248c288
	ctx.lr = 0x822E50AC;
	sub_8248C288(ctx, base);
	// 822E50AC: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 822E50B0: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 822E50B4: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 822E50B8: 38EBBA80  addi r7, r11, -0x4580
	ctx.r[7].s64 = ctx.r[11].s64 + -17792;
	// 822E50BC: 38CA6910  addi r6, r10, 0x6910
	ctx.r[6].s64 = ctx.r[10].s64 + 26896;
	// 822E50C0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822E50C4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822E50C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E50CC: 481AD0D5  bl 0x824921a0
	ctx.lr = 0x822E50D0;
	sub_824921A0(ctx, base);
	// 822E50D0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E50D4: 48000008  b 0x822e50dc
	pc = 0x822E50DC; continue 'dispatch;
	// 822E50D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E50DC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822E50E0: 4BFFFD09  bl 0x822e4de8
	ctx.lr = 0x822E50E4;
	sub_822E4DE8(ctx, base);
	// 822E50E4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E50E8: 41820014  beq 0x822e50fc
	if ctx.cr[0].eq {
	pc = 0x822E50FC; continue 'dispatch;
	}
	// 822E50EC: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E50F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E50F4: 419A0008  beq cr6, 0x822e50fc
	if ctx.cr[6].eq {
	pc = 0x822E50FC; continue 'dispatch;
	}
	// 822E50F8: 48003171  bl 0x822e8268
	ctx.lr = 0x822E50FC;
	sub_822E8268(ctx, base);
	// 822E50FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5100: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 822E5104: 48EC30AC  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E5108 size=112
    let mut pc: u32 = 0x822E5108;
    'dispatch: loop {
        match pc {
            0x822E5108 => {
    //   block [0x822E5108..0x822E5178)
	// 822E5108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E510C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E5110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E5114: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5118: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E511C: 807F00B8  lwz r3, 0xb8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 822E5120: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E5124: 419A0008  beq cr6, 0x822e512c
	if ctx.cr[6].eq {
	pc = 0x822E512C; continue 'dispatch;
	}
	// 822E5128: 4BFDB769  bl 0x822c0890
	ctx.lr = 0x822E512C;
	sub_822C0890(ctx, base);
	// 822E512C: 807F00B0  lwz r3, 0xb0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 822E5130: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E5134: 419A0008  beq cr6, 0x822e513c
	if ctx.cr[6].eq {
	pc = 0x822E513C; continue 'dispatch;
	}
	// 822E5138: 4BFDB759  bl 0x822c0890
	ctx.lr = 0x822E513C;
	sub_822C0890(ctx, base);
	// 822E513C: 807F00A0  lwz r3, 0xa0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 822E5140: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E5144: 419A0008  beq cr6, 0x822e514c
	if ctx.cr[6].eq {
	pc = 0x822E514C; continue 'dispatch;
	}
	// 822E5148: 4BFDB749  bl 0x822c0890
	ctx.lr = 0x822E514C;
	sub_822C0890(ctx, base);
	// 822E514C: 807F0098  lwz r3, 0x98(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 822E5150: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E5154: 419A0008  beq cr6, 0x822e515c
	if ctx.cr[6].eq {
	pc = 0x822E515C; continue 'dispatch;
	}
	// 822E5158: 4BFDB739  bl 0x822c0890
	ctx.lr = 0x822E515C;
	sub_822C0890(ctx, base);
	// 822E515C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5160: 48B73C09  bl 0x82e58d68
	ctx.lr = 0x822E5164;
	sub_82E58D68(ctx, base);
	// 822E5164: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E5168: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E516C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E5170: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E5174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E5178 size=76
    let mut pc: u32 = 0x822E5178;
    'dispatch: loop {
        match pc {
            0x822E5178 => {
    //   block [0x822E5178..0x822E51C4)
	// 822E5178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E517C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E5180: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E5184: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E5188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E518C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E5190: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E5194: 4BFFFF75  bl 0x822e5108
	ctx.lr = 0x822E5198;
	sub_822E5108(ctx, base);
	// 822E5198: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E519C: 4182000C  beq 0x822e51a8
	if ctx.cr[0].eq {
	pc = 0x822E51A8; continue 'dispatch;
	}
	// 822E51A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E51A4: 48B0D235  bl 0x82df23d8
	ctx.lr = 0x822E51A8;
	sub_82DF23D8(ctx, base);
	// 822E51A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E51AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E51B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E51B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E51B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E51BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E51C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E51C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E51C8 size=128
    let mut pc: u32 = 0x822E51C8;
    'dispatch: loop {
        match pc {
            0x822E51C8 => {
    //   block [0x822E51C8..0x822E5248)
	// 822E51C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E51CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E51D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E51D4: DBE1FFE8  stfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[31].u64 ) };
	// 822E51D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E51DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E51E0: FFE01090  fmr f31, f2
	ctx.f[31].f64 = ctx.f[2].f64;
	// 822E51E4: 388000D0  li r4, 0xd0
	ctx.r[4].s64 = 208;
	// 822E51E8: 897F0038  lbz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 822E51EC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E51F0: 41820020  beq 0x822e5210
	if ctx.cr[0].eq {
	pc = 0x822E5210; continue 'dispatch;
	}
	// 822E51F4: FC000850  fneg f0, f1
	ctx.f[0].u64 = ctx.f[1].u64 ^ 0x8000_0000_0000_0000u64;
	// 822E51F8: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822E51FC: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E5200: 48B295B9  bl 0x82e0e7b8
	ctx.lr = 0x822E5204;
	sub_82E0E7B8(ctx, base);
	// 822E5204: FC00F850  fneg f0, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = ctx.f[31].u64 ^ 0x8000_0000_0000_0000u64;
	// 822E5208: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822E520C: 48000014  b 0x822e5220
	pc = 0x822E5220; continue 'dispatch;
	// 822E5210: D0210050  stfs f1, 0x50(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822E5214: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E5218: 48B295A1  bl 0x82e0e7b8
	ctx.lr = 0x822E521C;
	sub_82E0E7B8(ctx, base);
	// 822E521C: D3E10050  stfs f31, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 822E5220: 388000CC  li r4, 0xcc
	ctx.r[4].s64 = 204;
	// 822E5224: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E5228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E522C: 48B2958D  bl 0x82e0e7b8
	ctx.lr = 0x822E5230;
	sub_82E0E7B8(ctx, base);
	// 822E5230: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E5234: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E5238: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E523C: CBE1FFE8  lfd f31, -0x18(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E5240: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E5244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E5248 size=156
    let mut pc: u32 = 0x822E5248;
    'dispatch: loop {
        match pc {
            0x822E5248 => {
    //   block [0x822E5248..0x822E52E4)
	// 822E5248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E524C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E5250: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E5254: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E5258: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E525C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E5260: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E5264: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 822E5268: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E526C: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5270: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 822E5274: 409A000C  bne cr6, 0x822e5280
	if !ctx.cr[6].eq {
	pc = 0x822E5280; continue 'dispatch;
	}
	// 822E5278: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E527C: 48000010  b 0x822e528c
	pc = 0x822E528C; continue 'dispatch;
	// 822E5280: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E5284: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 822E5288: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822E528C: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 822E5290: 38E0000C  li r7, 0xc
	ctx.r[7].s64 = 12;
	// 822E5294: 7CCB5396  divwu r6, r11, r10
	ctx.r[6].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 822E5298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E529C: 481AEDDD  bl 0x82494078
	ctx.lr = 0x822E52A0;
	sub_82494078(ctx, base);
	// 822E52A0: 807E0050  lwz r3, 0x50(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E52A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E52A8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E52AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E52B0: 4E800421  bctrl
	ctx.lr = 0x822E52B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E52B4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E52B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E52BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822E52C0: 48B96A09  bl 0x82e7bcc8
	ctx.lr = 0x822E52C4;
	sub_82E7BCC8(ctx, base);
	// 822E52C4: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E52E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E52E8 size=184
    let mut pc: u32 = 0x822E52E8;
    'dispatch: loop {
        match pc {
            0x822E52E8 => {
    //   block [0x822E52E8..0x822E53A0)
	// 822E52E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E52EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E52F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E52F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E52F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E52FC: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 822E5300: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 822E5304: 1D6A008C  mulli r11, r10, 0x8c
	ctx.r[11].s64 = ctx.r[10].s64 * 140;
	// 822E5308: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 822E530C: 3BCB0084  addi r30, r11, 0x84
	ctx.r[30].s64 = ctx.r[11].s64 + 132;
	// 822E5310: 816B0088  lwz r11, 0x88(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(136 as u32) ) } as u64;
	// 822E5314: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E5318: 409A0070  bne cr6, 0x822e5388
	if !ctx.cr[6].eq {
	pc = 0x822E5388; continue 'dispatch;
	}
	// 822E531C: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E5320: 397E0008  addi r11, r30, 8
	ctx.r[11].s64 = ctx.r[30].s64 + 8;
	// 822E5324: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822E5328: 409A002C  bne cr6, 0x822e5354
	if !ctx.cr[6].eq {
	pc = 0x822E5354; continue 'dispatch;
	}
	// 822E532C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5330: 2B090020  cmplwi cr6, r9, 0x20
	ctx.cr[6].compare_u32(ctx.r[9].u32, 32 as u32, &mut ctx.xer);
	// 822E5334: 40980038  bge cr6, 0x822e536c
	if !ctx.cr[6].lt {
	pc = 0x822E536C; continue 'dispatch;
	}
	// 822E5338: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 822E533C: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 822E5340: 7FE9592E  stwx r31, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[31].u32) };
	// 822E5344: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5348: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 822E534C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E5350: 4800001C  b 0x822e536c
	pc = 0x822E536C; continue 'dispatch;
	// 822E5354: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5358: 7F09F840  cmplw cr6, r9, r31
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[31].u32, &mut ctx.xer);
	// 822E535C: 419A002C  beq cr6, 0x822e5388
	if ctx.cr[6].eq {
	pc = 0x822E5388; continue 'dispatch;
	}
	// 822E5360: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5364: 5529103A  slwi r9, r9, 2
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 822E5368: 7FE9592E  stwx r31, r9, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32), ctx.r[31].u32) };
	// 822E536C: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5370: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E5374: 7D635214  add r11, r3, r10
	ctx.r[11].u64 = ctx.r[3].u64 + ctx.r[10].u64;
	// 822E5378: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 822E537C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E5380: 4E800421  bctrl
	ctx.lr = 0x822E5384;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E5384: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 822E5388: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E538C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E5390: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E5394: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E5398: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E539C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E53A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E53A0 size=192
    let mut pc: u32 = 0x822E53A0;
    'dispatch: loop {
        match pc {
            0x822E53A0 => {
    //   block [0x822E53A0..0x822E5460)
	// 822E53A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E53A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E53A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E53AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E53B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E53B4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 822E53B8: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 822E53BC: 897F0026  lbz r11, 0x26(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(38 as u32) ) } as u64;
	// 822E53C0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E53C4: 40820014  bne 0x822e53d8
	if !ctx.cr[0].eq {
	pc = 0x822E53D8; continue 'dispatch;
	}
	// 822E53C8: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 822E53CC: 409A007C  bne cr6, 0x822e5448
	if !ctx.cr[6].eq {
	pc = 0x822E5448; continue 'dispatch;
	}
	// 822E53D0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 822E53D4: 48000074  b 0x822e5448
	pc = 0x822E5448; continue 'dispatch;
	// 822E53D8: 419A0044  beq cr6, 0x822e541c
	if ctx.cr[6].eq {
	pc = 0x822E541C; continue 'dispatch;
	}
	// 822E53DC: 2B050002  cmplwi cr6, r5, 2
	ctx.cr[6].compare_u32(ctx.r[5].u32, 2 as u32, &mut ctx.xer);
	// 822E53E0: 419A0034  beq cr6, 0x822e5414
	if ctx.cr[6].eq {
	pc = 0x822E5414; continue 'dispatch;
	}
	// 822E53E4: 2B050006  cmplwi cr6, r5, 6
	ctx.cr[6].compare_u32(ctx.r[5].u32, 6 as u32, &mut ctx.xer);
	// 822E53E8: 409A0064  bne cr6, 0x822e544c
	if !ctx.cr[6].eq {
	pc = 0x822E544C; continue 'dispatch;
	}
	// 822E53EC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 822E53F0: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 822E53F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E53F8: 4BFFFEF1  bl 0x822e52e8
	ctx.lr = 0x822E53FC;
	sub_822E52E8(ctx, base);
	// 822E53FC: 388000D0  li r4, 0xd0
	ctx.r[4].s64 = 208;
	// 822E5400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5404: 80BF0028  lwz r5, 0x28(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 822E5408: 4BFFFEE1  bl 0x822e52e8
	ctx.lr = 0x822E540C;
	sub_822E52E8(ctx, base);
	// 822E540C: 80BF002C  lwz r5, 0x2c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 822E5410: 48000030  b 0x822e5440
	pc = 0x822E5440; continue 'dispatch;
	// 822E5414: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 822E5418: 4BFFFFD8  b 0x822e53f0
	pc = 0x822E53F0; continue 'dispatch;
	// 822E541C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822E5420: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 822E5424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5428: 4BFFFEC1  bl 0x822e52e8
	ctx.lr = 0x822E542C;
	sub_822E52E8(ctx, base);
	// 822E542C: 388000D0  li r4, 0xd0
	ctx.r[4].s64 = 208;
	// 822E5430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5434: 80BF0030  lwz r5, 0x30(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 822E5438: 4BFFFEB1  bl 0x822e52e8
	ctx.lr = 0x822E543C;
	sub_822E52E8(ctx, base);
	// 822E543C: 80BF0034  lwz r5, 0x34(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 822E5440: 388000CC  li r4, 0xcc
	ctx.r[4].s64 = 204;
	// 822E5444: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5448: 4BFFFEA1  bl 0x822e52e8
	ctx.lr = 0x822E544C;
	sub_822E52E8(ctx, base);
	// 822E544C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E5450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E5454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E5458: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E545C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E5460 size=152
    let mut pc: u32 = 0x822E5460;
    'dispatch: loop {
        match pc {
            0x822E5460 => {
    //   block [0x822E5460..0x822E54F8)
	// 822E5460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E5464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E5468: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E546C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E5470: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5474: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E5478: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E547C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 822E5480: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E5484: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5488: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 822E548C: 409A000C  bne cr6, 0x822e5498
	if !ctx.cr[6].eq {
	pc = 0x822E5498; continue 'dispatch;
	}
	// 822E5490: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E5494: 48000014  b 0x822e54a8
	pc = 0x822E54A8; continue 'dispatch;
	// 822E5498: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E549C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822E54A0: 7D655850  subf r11, r5, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[5].s64;
	// 822E54A4: 7CCB53D6  divw r6, r11, r10
	ctx.r[6].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 822E54A8: 38E00014  li r7, 0x14
	ctx.r[7].s64 = 20;
	// 822E54AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E54B0: 481AEBC9  bl 0x82494078
	ctx.lr = 0x822E54B4;
	sub_82494078(ctx, base);
	// 822E54B4: 807E0040  lwz r3, 0x40(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 822E54B8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E54BC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E54C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E54C4: 4E800421  bctrl
	ctx.lr = 0x822E54C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E54C8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E54CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E54D0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822E54D4: 48B967F5  bl 0x82e7bcc8
	ctx.lr = 0x822E54D8;
	sub_82E7BCC8(ctx, base);
	// 822E54D8: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E54F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E54F8 size=76
    let mut pc: u32 = 0x822E54F8;
    'dispatch: loop {
        match pc {
            0x822E54F8 => {
    //   block [0x822E54F8..0x822E5544)
	// 822E54F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E54FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E5500: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E5504: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5508: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E550C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E5510: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E5514: 419A0008  beq cr6, 0x822e551c
	if ctx.cr[6].eq {
	pc = 0x822E551C; continue 'dispatch;
	}
	// 822E5518: 4BFDB379  bl 0x822c0890
	ctx.lr = 0x822E551C;
	sub_822C0890(ctx, base);
	// 822E551C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E5520: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 822E5524: 396B9B84  addi r11, r11, -0x647c
	ctx.r[11].s64 = ctx.r[11].s64 + -25724;
	// 822E5528: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E552C: 48B0DEFD  bl 0x82df3428
	ctx.lr = 0x822E5530;
	sub_82DF3428(ctx, base);
	// 822E5530: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E5534: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E5538: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E553C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E5540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E5548 size=80
    let mut pc: u32 = 0x822E5548;
    'dispatch: loop {
        match pc {
            0x822E5548 => {
    //   block [0x822E5548..0x822E5598)
	// 822E5548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E554C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E5550: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E5554: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5558: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E555C: 48B1923D  bl 0x82dfe798
	ctx.lr = 0x822E5560;
	sub_82DFE798(ctx, base);
	// 822E5560: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E5564: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E5568: 394AAC48  addi r10, r10, -0x53b8
	ctx.r[10].s64 = ctx.r[10].s64 + -21432;
	// 822E556C: 387F002C  addi r3, r31, 0x2c
	ctx.r[3].s64 = ctx.r[31].s64 + 44;
	// 822E5570: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822E5574: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 822E5578: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822E557C: 48EB6305  bl 0x8319b880
	ctx.lr = 0x822E5580;
	sub_8319B880(ctx, base);
	// 822E5580: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5584: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E5588: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E558C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E5590: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E5594: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E5598 size=76
    let mut pc: u32 = 0x822E5598;
    'dispatch: loop {
        match pc {
            0x822E5598 => {
    //   block [0x822E5598..0x822E55E4)
	// 822E5598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E559C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E55A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E55A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E55A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E55AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E55B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E55B4: 4BFFFF45  bl 0x822e54f8
	ctx.lr = 0x822E55B8;
	sub_822E54F8(ctx, base);
	// 822E55B8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E55BC: 4182000C  beq 0x822e55c8
	if ctx.cr[0].eq {
	pc = 0x822E55C8; continue 'dispatch;
	}
	// 822E55C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E55C4: 48B0CE15  bl 0x82df23d8
	ctx.lr = 0x822E55C8;
	sub_82DF23D8(ctx, base);
	// 822E55C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E55CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E55D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E55D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E55D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E55DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E55E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E55E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E55E8 size=44
    let mut pc: u32 = 0x822E55E8;
    'dispatch: loop {
        match pc {
            0x822E55E8 => {
    //   block [0x822E55E8..0x822E5614)
	// 822E55E8: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E55EC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E55F0: 892B001D  lbz r9, 0x1d(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 822E55F4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822E55F8: 409A0030  bne cr6, 0x822e5628
	if !ctx.cr[6].eq {
		sub_822E5614(ctx, base);
		return;
	}
	// 822E55FC: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5600: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E5604: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E5608: 4098000C  bge cr6, 0x822e5614
	if !ctx.cr[6].lt {
		sub_822E5614(ctx, base);
		return;
	}
	// 822E560C: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E5610: 4800000C  b 0x822e561c
	sub_822E5614(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5614(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E5614 size=60
    let mut pc: u32 = 0x822E5614;
    'dispatch: loop {
        match pc {
            0x822E5614 => {
    //   block [0x822E5614..0x822E5650)
	// 822E5614: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 822E5618: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E561C: 890B001D  lbz r8, 0x1d(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 822E5620: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 822E5624: 419AFFDC  beq cr6, 0x822e5600
	if ctx.cr[6].eq {
		sub_822E55E8(ctx, base);
		return;
	}
	// 822E5628: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E562C: 9141FFF0  stw r10, -0x10(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[10].u32 ) };
	// 822E5630: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E5634: 419A001C  beq cr6, 0x822e5650
	if ctx.cr[6].eq {
		sub_822E5650(ctx, base);
		return;
	}
	// 822E5638: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E563C: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E5640: 7F095040  cmplw cr6, r9, r10
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822E5644: 4198000C  blt cr6, 0x822e5650
	if ctx.cr[6].lt {
		sub_822E5650(ctx, base);
		return;
	}
	// 822E5648: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
	// 822E564C: 4800000C  b 0x822e5658
	sub_822E5650(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E5650 size=20
    let mut pc: u32 = 0x822E5650;
    'dispatch: loop {
        match pc {
            0x822E5650 => {
    //   block [0x822E5650..0x822E5664)
	// 822E5650: 9161FFF4  stw r11, -0xc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), ctx.r[11].u32 ) };
	// 822E5654: 3961FFF4  addi r11, r1, -0xc
	ctx.r[11].s64 = ctx.r[1].s64 + -12;
	// 822E5658: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E565C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E5660: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5668(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E5668 size=120
    let mut pc: u32 = 0x822E5668;
    'dispatch: loop {
        match pc {
            0x822E5668 => {
    //   block [0x822E5668..0x822E56E0)
	// 822E5668: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E566C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E5670: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E5674: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E5678: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E567C: 90A10094  stw r5, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[5].u32 ) };
	// 822E5680: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E5684: 38A10094  addi r5, r1, 0x94
	ctx.r[5].s64 = ctx.r[1].s64 + 148;
	// 822E5688: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E568C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E5690: 4BFFFF59  bl 0x822e55e8
	ctx.lr = 0x822E5694;
	sub_822E55E8(ctx, base);
	// 822E5694: 815E0004  lwz r10, 4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5698: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E569C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E56A0: 7F0B5040  cmplw cr6, r11, r10
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822E56A4: 409A0014  bne cr6, 0x822e56b8
	if !ctx.cr[6].eq {
	pc = 0x822E56B8; continue 'dispatch;
	}
	// 822E56A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E56AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E56B0: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E56B4: 48000014  b 0x822e56c8
	pc = 0x822E56C8; continue 'dispatch;
	// 822E56B8: 80AB0018  lwz r5, 0x18(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 822E56BC: 808B0010  lwz r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E56C0: 4BFEA179  bl 0x822cf838
	ctx.lr = 0x822E56C4;
	sub_822CF838(ctx, base);
	// 822E56C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E56C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E56CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E56D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E56D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E56D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E56DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E56E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E56E0 size=136
    let mut pc: u32 = 0x822E56E0;
    'dispatch: loop {
        match pc {
            0x822E56E0 => {
    //   block [0x822E56E0..0x822E5768)
	// 822E56E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E56E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E56E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E56EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E56F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E56F4: 39450004  addi r10, r5, 4
	ctx.r[10].s64 = ctx.r[5].s64 + 4;
	// 822E56F8: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 822E56FC: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 822E5700: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5704: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822E5708: 81250004  lwz r9, 4(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E570C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822E5710: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 822E5714: 419A0024  beq cr6, 0x822e5738
	if ctx.cr[6].eq {
	pc = 0x822E5738; continue 'dispatch;
	}
	// 822E5718: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 822E571C: 7CE000A6  mfmsr r7
	ctx.r[7].u64 = ctx.msr;
	// 822E5720: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822E5724: 7D004828  lwarx r8, 0, r9
	// lwarx
	let ea = ctx.r[9].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[8].u64 = ctx.reserved.u32 as u64;
	// 822E5728: 39080001  addi r8, r8, 1
	ctx.r[8].s64 = ctx.r[8].s64 + 1;
	// 822E572C: 7D00492D  stwcx. r8, 0, r9
	// stwcx.
	let addr = ctx.r[9].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[8].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 822E5730: 7CE10164  mtmsrd r7, 1
	ctx.msr = (ctx.r[7].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822E5734: 4082FFE8  bne 0x822e571c
	if !ctx.cr[0].eq {
	pc = 0x822E571C; continue 'dispatch;
	}
	// 822E5738: 81250008  lwz r9, 8(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E573C: 912B0008  stw r9, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 822E5740: 806A0000  lwz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5744: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E5748: 419A0008  beq cr6, 0x822e5750
	if ctx.cr[6].eq {
	pc = 0x822E5750; continue 'dispatch;
	}
	// 822E574C: 4BFDB145  bl 0x822c0890
	ctx.lr = 0x822E5750;
	sub_822C0890(ctx, base);
	// 822E5750: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5754: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E5758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E575C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E5760: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E5764: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E5768 size=116
    let mut pc: u32 = 0x822E5768;
    'dispatch: loop {
        match pc {
            0x822E5768 => {
    //   block [0x822E5768..0x822E57DC)
	// 822E5768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E576C: 48EC29F5  bl 0x831a8160
	ctx.lr = 0x822E5770;
	sub_831A8130(ctx, base);
	// 822E5770: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5774: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822E5778: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822E577C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E5780: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E5784: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 822E5788: 38C00020  li r6, 0x20
	ctx.r[6].s64 = 32;
	// 822E578C: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822E5790: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 822E5794: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 822E5798: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 822E579C: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 822E57A0: 48B0C929  bl 0x82df20c8
	ctx.lr = 0x822E57A4;
	sub_82DF20C8(ctx, base);
	// 822E57A4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822E57A8: 41820028  beq 0x822e57d0
	if ctx.cr[0].eq {
	pc = 0x822E57D0; continue 'dispatch;
	}
	// 822E57AC: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 822E57B0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822E57B4: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 822E57B8: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 822E57BC: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 822E57C0: 48207911  bl 0x824ed0d0
	ctx.lr = 0x822E57C4;
	sub_824ED0D0(ctx, base);
	// 822E57C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E57C8: 9B5F001C  stb r26, 0x1c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[26].u8 ) };
	// 822E57CC: 997F001D  stb r11, 0x1d(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(29 as u32), ctx.r[11].u8 ) };
	// 822E57D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E57D4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 822E57D8: 48EC29D8  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E57E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E57E0 size=104
    let mut pc: u32 = 0x822E57E0;
    'dispatch: loop {
        match pc {
            0x822E57E0 => {
    //   block [0x822E57E0..0x822E5848)
	// 822E57E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E57E4: 48EC2985  bl 0x831a8168
	ctx.lr = 0x822E57E8;
	sub_831A8130(ctx, base);
	// 822E57E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E57EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E57F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E57F4: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 822E57F8: 897E001D  lbz r11, 0x1d(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(29 as u32) ) } as u64;
	// 822E57FC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E5800: 409A0040  bne cr6, 0x822e5840
	if !ctx.cr[6].eq {
	pc = 0x822E5840; continue 'dispatch;
	}
	// 822E5804: 3F808335  lis r28, -0x7ccb
	ctx.r[28].s64 = -2093678592;
	// 822E5808: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E580C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E5810: 4BFFFFD1  bl 0x822e57e0
	ctx.lr = 0x822E5814;
	sub_822E57E0(ctx, base);
	// 822E5814: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E5818: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E581C: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5820: 484F7581  bl 0x827dcda0
	ctx.lr = 0x822E5824;
	sub_827DCDA0(ctx, base);
	// 822E5824: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E5828: 807C110C  lwz r3, 0x110c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822E582C: 48B0C95D  bl 0x82df2188
	ctx.lr = 0x822E5830;
	sub_82DF2188(ctx, base);
	// 822E5830: 897F001D  lbz r11, 0x1d(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(29 as u32) ) } as u64;
	// 822E5834: 7FFEFB78  mr r30, r31
	ctx.r[30].u64 = ctx.r[31].u64;
	// 822E5838: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E583C: 419AFFCC  beq cr6, 0x822e5808
	if ctx.cr[6].eq {
	pc = 0x822E5808; continue 'dispatch;
	}
	// 822E5840: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E5844: 48EC2974  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E5848 size=84
    let mut pc: u32 = 0x822E5848;
    'dispatch: loop {
        match pc {
            0x822E5848 => {
    //   block [0x822E5848..0x822E589C)
	// 822E5848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E584C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E5850: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E5854: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5858: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E585C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5860: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5864: 4BFFFF7D  bl 0x822e57e0
	ctx.lr = 0x822E5868;
	sub_822E57E0(ctx, base);
	// 822E5868: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E586C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E5870: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822E5874: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822E5878: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E587C: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822E5880: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5884: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822E5888: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E588C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E5890: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E5894: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E5898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E58A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E58A0 size=548
    let mut pc: u32 = 0x822E58A0;
    'dispatch: loop {
        match pc {
            0x822E58A0 => {
    //   block [0x822E58A0..0x822E5AC4)
	// 822E58A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E58A4: 48EC28BD  bl 0x831a8160
	ctx.lr = 0x822E58A8;
	sub_831A8130(ctx, base);
	// 822E58A8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E58AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E58B0: 3D600FFF  lis r11, 0xfff
	ctx.r[11].s64 = 268369920;
	// 822E58B4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 822E58B8: 616BFFFE  ori r11, r11, 0xfffe
	ctx.r[11].u64 = ctx.r[11].u64 | 65534;
	// 822E58BC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 822E58C0: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E58C4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 822E58C8: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 822E58CC: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E58D0: 41980048  blt cr6, 0x822e5918
	if ctx.cr[6].lt {
	pc = 0x822E5918; continue 'dispatch;
	}
	// 822E58D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E58D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E58DC: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 822E58E0: 4BFDFFE9  bl 0x822c58c8
	ctx.lr = 0x822E58E4;
	sub_822C58C8(ctx, base);
	// 822E58E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822E58E8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822E58EC: 4BFDFF2D  bl 0x822c5818
	ctx.lr = 0x822E58F0;
	sub_822C5818(ctx, base);
	// 822E58F0: 4BFDE9C1  bl 0x822c42b0
	ctx.lr = 0x822E58F4;
	sub_822C42B0(ctx, base);
	// 822E58F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E58F8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822E58FC: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 822E5900: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 822E5904: 4BFDFB6D  bl 0x822c5470
	ctx.lr = 0x822E5908;
	sub_822C5470(ctx, base);
	// 822E5908: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822E590C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822E5910: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E5914: 4BFDF3CD  bl 0x822c4ce0
	ctx.lr = 0x822E5918;
	sub_822C4CE0(ctx, base);
	// 822E5918: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E591C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 822E5920: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 822E5924: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 822E5928: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822E592C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E5930: 4BFFFE39  bl 0x822e5768
	ctx.lr = 0x822E5934;
	sub_822E5768(ctx, base);
	// 822E5934: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E5938: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E593C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822E5940: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822E5944: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E5948: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822E594C: 409A0018  bne cr6, 0x822e5964
	if !ctx.cr[6].eq {
	pc = 0x822E5964; continue 'dispatch;
	}
	// 822E5950: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 822E5954: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5958: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 822E595C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5960: 4800003C  b 0x822e599c
	pc = 0x822E599C; continue 'dispatch;
	// 822E5964: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E5968: 41820020  beq 0x822e5988
	if ctx.cr[0].eq {
	pc = 0x822E5988; continue 'dispatch;
	}
	// 822E596C: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 822E5970: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5974: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5978: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822E597C: 409A0024  bne cr6, 0x822e59a0
	if !ctx.cr[6].eq {
	pc = 0x822E59A0; continue 'dispatch;
	}
	// 822E5980: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 822E5984: 4800001C  b 0x822e59a0
	pc = 0x822E59A0; continue 'dispatch;
	// 822E5988: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 822E598C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5990: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E5994: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822E5998: 409A0008  bne cr6, 0x822e59a0
	if !ctx.cr[6].eq {
	pc = 0x822E59A0; continue 'dispatch;
	}
	// 822E599C: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 822E59A0: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E59A4: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 822E59A8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 822E59AC: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 822E59B0: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E59B4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822E59B8: 409A00F0  bne cr6, 0x822e5aa8
	if !ctx.cr[6].eq {
	pc = 0x822E5AA8; continue 'dispatch;
	}
	// 822E59BC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 822E59C0: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E59C4: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E59C8: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E59CC: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E59D0: 409A0054  bne cr6, 0x822e5a24
	if !ctx.cr[6].eq {
	pc = 0x822E5A24; continue 'dispatch;
	}
	// 822E59D4: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E59D8: 892A001C  lbz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E59DC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822E59E0: 419A0054  beq cr6, 0x822e5a34
	if ctx.cr[6].eq {
	pc = 0x822E5A34; continue 'dispatch;
	}
	// 822E59E4: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E59E8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E59EC: 409A0010  bne cr6, 0x822e59fc
	if !ctx.cr[6].eq {
	pc = 0x822E59FC; continue 'dispatch;
	}
	// 822E59F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E59F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E59F8: 48249B91  bl 0x8252f588
	ctx.lr = 0x822E59FC;
	sub_8252F588(ctx, base);
	// 822E59FC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5A00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E5A04: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 822E5A08: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5A0C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5A10: 9B6B001C  stb r27, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[27].u8 ) };
	// 822E5A14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5A18: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5A1C: 48249BD5  bl 0x8252f5f0
	ctx.lr = 0x822E5A20;
	sub_8252F5F0(ctx, base);
	// 822E5A20: 48000074  b 0x822e5a94
	pc = 0x822E5A94; continue 'dispatch;
	// 822E5A24: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5A28: 892A001C  lbz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E5A2C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822E5A30: 409A0028  bne cr6, 0x822e5a58
	if !ctx.cr[6].eq {
	pc = 0x822E5A58; continue 'dispatch;
	}
	// 822E5A34: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5A38: 9BA9001C  stb r29, 0x1c(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 822E5A3C: 9BAA001C  stb r29, 0x1c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 822E5A40: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5A44: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5A48: 9B6A001C  stb r27, 0x1c(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(28 as u32), ctx.r[27].u8 ) };
	// 822E5A4C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5A50: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5A54: 48000040  b 0x822e5a94
	pc = 0x822E5A94; continue 'dispatch;
	// 822E5A58: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5A5C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E5A60: 409A0010  bne cr6, 0x822e5a70
	if !ctx.cr[6].eq {
	pc = 0x822E5A70; continue 'dispatch;
	}
	// 822E5A64: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E5A68: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E5A6C: 48249B85  bl 0x8252f5f0
	ctx.lr = 0x822E5A70;
	sub_8252F5F0(ctx, base);
	// 822E5A70: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5A74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E5A78: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 822E5A7C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5A80: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5A84: 9B6B001C  stb r27, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[27].u8 ) };
	// 822E5A88: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5A8C: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5A90: 48249AF9  bl 0x8252f588
	ctx.lr = 0x822E5A94;
	sub_8252F588(ctx, base);
	// 822E5A94: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5A98: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 822E5A9C: 894A001C  lbz r10, 0x1c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E5AA0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822E5AA4: 419AFF1C  beq cr6, 0x822e59c0
	if ctx.cr[6].eq {
	pc = 0x822E59C0; continue 'dispatch;
	}
	// 822E5AA8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5AAC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 822E5AB0: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 822E5AB4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5AB8: 9BAB001C  stb r29, 0x1c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[29].u8 ) };
	// 822E5ABC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 822E5AC0: 48EC26F0  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E5AC8 size=236
    let mut pc: u32 = 0x822E5AC8;
    'dispatch: loop {
        match pc {
            0x822E5AC8 => {
    //   block [0x822E5AC8..0x822E5BB4)
	// 822E5AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E5ACC: 48EC2695  bl 0x831a8160
	ctx.lr = 0x822E5AD0;
	sub_831A8130(ctx, base);
	// 822E5AD0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5AD4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 822E5AD8: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 822E5ADC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E5AE0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 822E5AE4: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 822E5AE8: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5AEC: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5AF0: 894B001D  lbz r10, 0x1d(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 822E5AF4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822E5AF8: 409A0038  bne cr6, 0x822e5b30
	if !ctx.cr[6].eq {
	pc = 0x822E5B30; continue 'dispatch;
	}
	// 822E5AFC: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5B00: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E5B04: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 822E5B08: 7D295010  subfc r9, r9, r10
	ctx.xer.ca = ctx.r[10].u32 >= ctx.r[9].u32;
	ctx.r[9].s64 = ctx.r[10].s64 - ctx.r[9].s64;
	// 822E5B0C: 7D294910  subfe r9, r9, r9
	let x = (!ctx.r[9].u32);
	let y = ctx.r[9].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[9].u32 = res;
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 822E5B10: 553D07FF  clrlwi. r29, r9, 0x1f
	ctx.r[29].u64 = ctx.r[9].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822E5B14: 4182000C  beq 0x822e5b20
	if ctx.cr[0].eq {
	pc = 0x822E5B20; continue 'dispatch;
	}
	// 822E5B18: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5B1C: 48000008  b 0x822e5b24
	pc = 0x822E5B24; continue 'dispatch;
	// 822E5B20: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E5B24: 892B001D  lbz r9, 0x1d(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(29 as u32) ) } as u64;
	// 822E5B28: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822E5B2C: 419AFFD4  beq cr6, 0x822e5b00
	if ctx.cr[6].eq {
	pc = 0x822E5B00; continue 'dispatch;
	}
	// 822E5B30: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 822E5B34: 57AA063F  clrlwi. r10, r29, 0x18
	ctx.r[10].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E5B38: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822E5B3C: 41820044  beq 0x822e5b80
	if ctx.cr[0].eq {
	pc = 0x822E5B80; continue 'dispatch;
	}
	// 822E5B40: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E5B44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E5B48: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5B4C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E5B50: 409A0028  bne cr6, 0x822e5b78
	if !ctx.cr[6].eq {
	pc = 0x822E5B78; continue 'dispatch;
	}
	// 822E5B54: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E5B58: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E5B5C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 822E5B60: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 822E5B64: 4BFFFD3D  bl 0x822e58a0
	ctx.lr = 0x822E5B68;
	sub_822E58A0(ctx, base);
	// 822E5B68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E5B6C: 9B5F0004  stb r26, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 822E5B70: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5B74: 48000030  b 0x822e5ba4
	pc = 0x822E5BA4; continue 'dispatch;
	// 822E5B78: 48574F11  bl 0x8285aa88
	ctx.lr = 0x822E5B7C;
	sub_8285AA88(ctx, base);
	// 822E5B7C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E5B80: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E5B84: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5B88: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E5B8C: 40980010  bge cr6, 0x822e5b9c
	if !ctx.cr[6].lt {
	pc = 0x822E5B9C; continue 'dispatch;
	}
	// 822E5B90: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822E5B94: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E5B98: 4BFFFFC0  b 0x822e5b58
	pc = 0x822E5B58; continue 'dispatch;
	// 822E5B9C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822E5BA0: 995F0004  stb r10, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u8 ) };
	// 822E5BA4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E5BA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5BAC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 822E5BB0: 48EC2600  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E5BB8 size=248
    let mut pc: u32 = 0x822E5BB8;
    'dispatch: loop {
        match pc {
            0x822E5BB8 => {
    //   block [0x822E5BB8..0x822E5CB0)
	// 822E5BB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E5BBC: 48EC25A9  bl 0x831a8164
	ctx.lr = 0x822E5BC0;
	sub_831A8130(ctx, base);
	// 822E5BC0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5BC4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 822E5BC8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822E5BCC: 3B9F0004  addi r28, r31, 4
	ctx.r[28].s64 = ctx.r[31].s64 + 4;
	// 822E5BD0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 822E5BD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E5BD8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822E5BDC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5BE0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822E5BE4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E5BE8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 822E5BEC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822E5BF0: 4BFDE871  bl 0x822c4460
	ctx.lr = 0x822E5BF4;
	sub_822C4460(ctx, base);
	// 822E5BF4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822E5BF8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5BFC: 4BFE97A5  bl 0x822cf3a0
	ctx.lr = 0x822E5C00;
	sub_822CF3A0(ctx, base);
	// 822E5C00: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 822E5C04: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E5C08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E5C0C: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 822E5C10: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822E5C14: 419A0024  beq cr6, 0x822e5c38
	if ctx.cr[6].eq {
	pc = 0x822E5C38; continue 'dispatch;
	}
	// 822E5C18: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 822E5C1C: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 822E5C20: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822E5C24: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 822E5C28: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822E5C2C: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 822E5C30: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822E5C34: 4082FFE8  bne 0x822e5c1c
	if !ctx.cr[0].eq {
	pc = 0x822E5C1C; continue 'dispatch;
	}
	// 822E5C38: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 822E5C3C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 822E5C40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E5C44: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822E5C48: 4BFFFA99  bl 0x822e56e0
	ctx.lr = 0x822E5C4C;
	sub_822E56E0(ctx, base);
	// 822E5C4C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E5C50: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822E5C54: 4820747D  bl 0x824ed0d0
	ctx.lr = 0x822E5C58;
	sub_824ED0D0(ctx, base);
	// 822E5C58: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822E5C5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E5C60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E5C64: 4BFFFE65  bl 0x822e5ac8
	ctx.lr = 0x822E5C68;
	sub_822E5AC8(ctx, base);
	// 822E5C68: 80610068  lwz r3, 0x68(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 822E5C6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E5C70: 419A0008  beq cr6, 0x822e5c78
	if ctx.cr[6].eq {
	pc = 0x822E5C78; continue 'dispatch;
	}
	// 822E5C74: 4BFDAC1D  bl 0x822c0890
	ctx.lr = 0x822E5C78;
	sub_822C0890(ctx, base);
	// 822E5C78: 80610078  lwz r3, 0x78(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(120 as u32) ) } as u64;
	// 822E5C7C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E5C80: 419A0008  beq cr6, 0x822e5c88
	if ctx.cr[6].eq {
	pc = 0x822E5C88; continue 'dispatch;
	}
	// 822E5C84: 4BFDAC0D  bl 0x822c0890
	ctx.lr = 0x822E5C88;
	sub_822C0890(ctx, base);
	// 822E5C88: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E5C8C: 419A000C  beq cr6, 0x822e5c98
	if ctx.cr[6].eq {
	pc = 0x822E5C98; continue 'dispatch;
	}
	// 822E5C90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5C94: 4BFDABFD  bl 0x822c0890
	ctx.lr = 0x822E5C98;
	sub_822C0890(ctx, base);
	// 822E5C98: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5C9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E5CA0: 419A0008  beq cr6, 0x822e5ca8
	if ctx.cr[6].eq {
	pc = 0x822E5CA8; continue 'dispatch;
	}
	// 822E5CA4: 4BFDABED  bl 0x822c0890
	ctx.lr = 0x822E5CA8;
	sub_822C0890(ctx, base);
	// 822E5CA8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 822E5CAC: 48EC2508  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E5CB0 size=64
    let mut pc: u32 = 0x822E5CB0;
    'dispatch: loop {
        match pc {
            0x822E5CB0 => {
    //   block [0x822E5CB0..0x822E5CF0)
	// 822E5CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E5CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E5CB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E5CBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5CC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E5CC4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 822E5CC8: 909F0000  stw r4, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 822E5CCC: 419A000C  beq cr6, 0x822e5cd8
	if ctx.cr[6].eq {
	pc = 0x822E5CD8; continue 'dispatch;
	}
	// 822E5CD0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 822E5CD4: 48BE8B2D  bl 0x82ece800
	ctx.lr = 0x822E5CD8;
	sub_82ECE800(ctx, base);
	// 822E5CD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5CDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E5CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E5CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E5CE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E5CEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E5CF0 size=68
    let mut pc: u32 = 0x822E5CF0;
    'dispatch: loop {
        match pc {
            0x822E5CF0 => {
    //   block [0x822E5CF0..0x822E5D34)
	// 822E5CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E5CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E5CF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E5CFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5D00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E5D04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E5D08: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E5D0C: 396BAC74  addi r11, r11, -0x538c
	ctx.r[11].s64 = ctx.r[11].s64 + -21388;
	// 822E5D10: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E5D14: 41820008  beq 0x822e5d1c
	if ctx.cr[0].eq {
	pc = 0x822E5D1C; continue 'dispatch;
	}
	// 822E5D18: 4BFDA551  bl 0x822c0268
	ctx.lr = 0x822E5D1C;
	sub_822C0268(ctx, base);
	// 822E5D1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5D20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E5D24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E5D28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E5D2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E5D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E5D38 size=68
    let mut pc: u32 = 0x822E5D38;
    'dispatch: loop {
        match pc {
            0x822E5D38 => {
    //   block [0x822E5D38..0x822E5D7C)
	// 822E5D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E5D3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E5D40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E5D44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5D48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E5D4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E5D50: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E5D54: 396BAC5C  addi r11, r11, -0x53a4
	ctx.r[11].s64 = ctx.r[11].s64 + -21412;
	// 822E5D58: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E5D5C: 41820008  beq 0x822e5d64
	if ctx.cr[0].eq {
	pc = 0x822E5D64; continue 'dispatch;
	}
	// 822E5D60: 4BFDA509  bl 0x822c0268
	ctx.lr = 0x822E5D64;
	sub_822C0268(ctx, base);
	// 822E5D64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5D68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E5D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E5D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E5D74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E5D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822E5D80 size=64
    let mut pc: u32 = 0x822E5D80;
    'dispatch: loop {
        match pc {
            0x822E5D80 => {
    //   block [0x822E5D80..0x822E5DC0)
	// 822E5D80: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E5D84: 3961FFF0  addi r11, r1, -0x10
	ctx.r[11].s64 = ctx.r[1].s64 + -16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E5DC0 size=8
    let mut pc: u32 = 0x822E5DC0;
    'dispatch: loop {
        match pc {
            0x822E5DC0 => {
    //   block [0x822E5DC0..0x822E5DC8)
	// 822E5DC0: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E5DC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E5DC8 size=12
    let mut pc: u32 = 0x822E5DC8;
    'dispatch: loop {
        match pc {
            0x822E5DC8 => {
    //   block [0x822E5DC8..0x822E5DD4)
	// 822E5DC8: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E5DCC: 386B00A0  addi r3, r11, 0xa0
	ctx.r[3].s64 = ctx.r[11].s64 + 160;
	// 822E5DD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E5DD8 size=76
    let mut pc: u32 = 0x822E5DD8;
    'dispatch: loop {
        match pc {
            0x822E5DD8 => {
    //   block [0x822E5DD8..0x822E5E24)
	// 822E5DD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E5DDC: 48EC2391  bl 0x831a816c
	ctx.lr = 0x822E5DE0;
	sub_831A8130(ctx, base);
	// 822E5DE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5DE4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E5DE8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E5DEC: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5DF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E5DF4: 419A000C  beq cr6, 0x822e5e00
	if ctx.cr[6].eq {
	pc = 0x822E5E00; continue 'dispatch;
	}
	// 822E5DF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5DFC: 48BE8A05  bl 0x82ece800
	ctx.lr = 0x822E5E00;
	sub_82ECE800(ctx, base);
	// 822E5E00: 809D004C  lwz r4, 0x4c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E5E04: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5E08: 48BE8C39  bl 0x82ecea40
	ctx.lr = 0x822E5E0C;
	sub_82ECEA40(ctx, base);
	// 822E5E0C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E5E10: 419A000C  beq cr6, 0x822e5e1c
	if ctx.cr[6].eq {
	pc = 0x822E5E1C; continue 'dispatch;
	}
	// 822E5E14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5E18: 48BE71F1  bl 0x82ecd008
	ctx.lr = 0x822E5E1C;
	sub_82ECD008(ctx, base);
	// 822E5E1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E5E20: 48EC239C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E5E28 size=76
    let mut pc: u32 = 0x822E5E28;
    'dispatch: loop {
        match pc {
            0x822E5E28 => {
    //   block [0x822E5E28..0x822E5E74)
	// 822E5E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E5E2C: 48EC2341  bl 0x831a816c
	ctx.lr = 0x822E5E30;
	sub_831A8130(ctx, base);
	// 822E5E30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5E34: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E5E38: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E5E3C: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5E40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E5E44: 419A000C  beq cr6, 0x822e5e50
	if ctx.cr[6].eq {
	pc = 0x822E5E50; continue 'dispatch;
	}
	// 822E5E48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5E4C: 48BE89B5  bl 0x82ece800
	ctx.lr = 0x822E5E50;
	sub_82ECE800(ctx, base);
	// 822E5E50: 809D004C  lwz r4, 0x4c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E5E54: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E5E58: 48BE7719  bl 0x82ecd570
	ctx.lr = 0x822E5E5C;
	sub_82ECD570(ctx, base);
	// 822E5E5C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E5E60: 419A000C  beq cr6, 0x822e5e6c
	if ctx.cr[6].eq {
	pc = 0x822E5E6C; continue 'dispatch;
	}
	// 822E5E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5E68: 48BE71A1  bl 0x82ecd008
	ctx.lr = 0x822E5E6C;
	sub_82ECD008(ctx, base);
	// 822E5E6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E5E70: 48EC234C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E5E78 size=96
    let mut pc: u32 = 0x822E5E78;
    'dispatch: loop {
        match pc {
            0x822E5E78 => {
    //   block [0x822E5E78..0x822E5ED8)
	// 822E5E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E5E7C: 48EC22F1  bl 0x831a816c
	ctx.lr = 0x822E5E80;
	sub_831A8130(ctx, base);
	// 822E5E80: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5E84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E5E88: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822E5E8C: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E5E90: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E5E94: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E5E98: 419A000C  beq cr6, 0x822e5ea4
	if ctx.cr[6].eq {
	pc = 0x822E5EA4; continue 'dispatch;
	}
	// 822E5E9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5EA0: 48BE8961  bl 0x82ece800
	ctx.lr = 0x822E5EA4;
	sub_82ECE800(ctx, base);
	// 822E5EA4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E5EA8: 83DE004C  lwz r30, 0x4c(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E5EAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E5EB0: 480029D9  bl 0x822e8888
	ctx.lr = 0x822E5EB4;
	sub_822E8888(ctx, base);
	// 822E5EB4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E5EB8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E5EBC: 48BEEE6D  bl 0x82ed4d28
	ctx.lr = 0x822E5EC0;
	sub_82ED4D28(ctx, base);
	// 822E5EC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E5EC4: 419A000C  beq cr6, 0x822e5ed0
	if ctx.cr[6].eq {
	pc = 0x822E5ED0; continue 'dispatch;
	}
	// 822E5EC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5ECC: 48BE713D  bl 0x82ecd008
	ctx.lr = 0x822E5ED0;
	sub_82ECD008(ctx, base);
	// 822E5ED0: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 822E5ED4: 48EC22E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E5ED8 size=124
    let mut pc: u32 = 0x822E5ED8;
    'dispatch: loop {
        match pc {
            0x822E5ED8 => {
    //   block [0x822E5ED8..0x822E5F54)
	// 822E5ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E5EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E5EE0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E5EE4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E5EE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5EEC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E5EF0: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E5EF4: 908B002C  stw r4, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[4].u32 ) };
	// 822E5EF8: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E5EFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E5F00: 419A0010  beq cr6, 0x822e5f10
	if ctx.cr[6].eq {
	pc = 0x822E5F10; continue 'dispatch;
	}
	// 822E5F04: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822E5F08: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E5F0C: 48000008  b 0x822e5f14
	pc = 0x822E5F14; continue 'dispatch;
	// 822E5F10: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822E5F14: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E5F18: 41820024  beq 0x822e5f3c
	if ctx.cr[0].eq {
	pc = 0x822E5F3C; continue 'dispatch;
	}
	// 822E5F1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5F20: 48BE88E1  bl 0x82ece800
	ctx.lr = 0x822E5F24;
	sub_82ECE800(ctx, base);
	// 822E5F24: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E5F28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5F2C: 809E004C  lwz r4, 0x4c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E5F30: 48BEB941  bl 0x82ed1870
	ctx.lr = 0x822E5F34;
	sub_82ED1870(ctx, base);
	// 822E5F34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5F38: 48BE70D1  bl 0x82ecd008
	ctx.lr = 0x822E5F3C;
	sub_82ECD008(ctx, base);
	// 822E5F3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E5F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E5F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E5F48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E5F4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E5F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E5F58 size=88
    let mut pc: u32 = 0x822E5F58;
    'dispatch: loop {
        match pc {
            0x822E5F58 => {
    //   block [0x822E5F58..0x822E5FB0)
	// 822E5F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E5F5C: 48EC2211  bl 0x831a816c
	ctx.lr = 0x822E5F60;
	sub_831A8130(ctx, base);
	// 822E5F60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5F64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E5F68: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822E5F6C: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E5F70: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E5F74: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E5F78: 419A000C  beq cr6, 0x822e5f84
	if ctx.cr[6].eq {
	pc = 0x822E5F84; continue 'dispatch;
	}
	// 822E5F7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5F80: 48BE8881  bl 0x82ece800
	ctx.lr = 0x822E5F84;
	sub_82ECE800(ctx, base);
	// 822E5F84: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822E5F88: 807E004C  lwz r3, 0x4c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E5F8C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E5F90: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822E5F94: 48BEEE35  bl 0x82ed4dc8
	ctx.lr = 0x822E5F98;
	sub_82ED4DC8(ctx, base);
	// 822E5F98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E5F9C: 419A000C  beq cr6, 0x822e5fa8
	if ctx.cr[6].eq {
	pc = 0x822E5FA8; continue 'dispatch;
	}
	// 822E5FA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E5FA4: 48BE7065  bl 0x82ecd008
	ctx.lr = 0x822E5FA8;
	sub_82ECD008(ctx, base);
	// 822E5FA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E5FAC: 48EC2210  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E5FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E5FB0 size=220
    let mut pc: u32 = 0x822E5FB0;
    'dispatch: loop {
        match pc {
            0x822E5FB0 => {
    //   block [0x822E5FB0..0x822E608C)
	// 822E5FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E5FB4: 48EC21B1  bl 0x831a8164
	ctx.lr = 0x822E5FB8;
	sub_831A8130(ctx, base);
	// 822E5FB8: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E5FBC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E5FC0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E5FC4: 817E004C  lwz r11, 0x4c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E5FC8: 83AB0008  lwz r29, 8(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E5FCC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 822E5FD0: 419A000C  beq cr6, 0x822e5fdc
	if ctx.cr[6].eq {
	pc = 0x822E5FDC; continue 'dispatch;
	}
	// 822E5FD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E5FD8: 48BE8829  bl 0x82ece800
	ctx.lr = 0x822E5FDC;
	sub_82ECE800(ctx, base);
	// 822E5FDC: 13E0F8C7  vcmpequd (lvx128) v31, v0, v31
	tmp.u32 = ctx.r[31].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E5FE0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E6090 size=12
    let mut pc: u32 = 0x822E6090;
    'dispatch: loop {
        match pc {
            0x822E6090 => {
    //   block [0x822E6090..0x822E609C)
	// 822E6090: E8A50000  ld r5, 0(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 822E6094: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E6098: 48BEF0C8  b 0x82ed5160
	sub_82ED5160(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E60A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E60A0 size=80
    let mut pc: u32 = 0x822E60A0;
    'dispatch: loop {
        match pc {
            0x822E60A0 => {
    //   block [0x822E60A0..0x822E60F0)
	// 822E60A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E60A4: 48EC20C9  bl 0x831a816c
	ctx.lr = 0x822E60A8;
	sub_831A8130(ctx, base);
	// 822E60A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E60AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E60B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E60B4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E60B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E60BC: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 822E60C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E60C4: 4E800421  bctrl
	ctx.lr = 0x822E60C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E60C8: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E60CC: E8BD0000  ld r5, 0(r29)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) };
	// 822E60D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E60D4: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E60D8: 4182000C  beq 0x822e60e4
	if ctx.cr[0].eq {
	pc = 0x822E60E4; continue 'dispatch;
	}
	// 822E60DC: 48BEF1CD  bl 0x82ed52a8
	ctx.lr = 0x822E60E0;
	sub_82ED52A8(ctx, base);
	// 822E60E0: 48000008  b 0x822e60e8
	pc = 0x822E60E8; continue 'dispatch;
	// 822E60E4: 48BEF07D  bl 0x82ed5160
	ctx.lr = 0x822E60E8;
	sub_82ED5160(ctx, base);
	// 822E60E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E60EC: 48EC20D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E60F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822E60F0 size=20
    let mut pc: u32 = 0x822E60F0;
    'dispatch: loop {
        match pc {
            0x822E60F0 => {
    //   block [0x822E60F0..0x822E6104)
	// 822E60F0: 8164004C  lwz r11, 0x4c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E60F4: 394000D0  li r10, 0xd0
	ctx.r[10].s64 = 208;
	// 822E60F8: 13EB50C7  vcmpequd (lvx128) v31, v11, v10
	tmp.u32 = ctx.r[11].u32 + ctx.r[10].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6108 size=160
    let mut pc: u32 = 0x822E6108;
    'dispatch: loop {
        match pc {
            0x822E6108 => {
    //   block [0x822E6108..0x822E61A8)
	// 822E6108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E610C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6110: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6114: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6118: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E611C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E6120: 396BAC84  addi r11, r11, -0x537c
	ctx.r[11].s64 = ctx.r[11].s64 + -21372;
	// 822E6124: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E6128: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E612C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E6130: 419A000C  beq cr6, 0x822e613c
	if ctx.cr[6].eq {
	pc = 0x822E613C; continue 'dispatch;
	}
	// 822E6134: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E6138: 48006579  bl 0x822ec6b0
	ctx.lr = 0x822E613C;
	sub_822EC6B0(ctx, base);
	// 822E613C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E6140: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6144: 484F3E95  bl 0x827d9fd8
	ctx.lr = 0x822E6148;
	sub_827D9FD8(ctx, base);
	// 822E6148: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E614C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E6150: 419A000C  beq cr6, 0x822e615c
	if ctx.cr[6].eq {
	pc = 0x822E615C; continue 'dispatch;
	}
	// 822E6154: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822E6158: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 822E615C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 822E6160: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E6164: 419A0008  beq cr6, 0x822e616c
	if ctx.cr[6].eq {
	pc = 0x822E616C; continue 'dispatch;
	}
	// 822E6168: 4BFDA729  bl 0x822c0890
	ctx.lr = 0x822E616C;
	sub_822C0890(ctx, base);
	// 822E616C: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E6170: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E6174: 419A0008  beq cr6, 0x822e617c
	if ctx.cr[6].eq {
	pc = 0x822E617C; continue 'dispatch;
	}
	// 822E6178: 480020F1  bl 0x822e8268
	ctx.lr = 0x822E617C;
	sub_822E8268(ctx, base);
	// 822E617C: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 822E6180: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E6184: 419A0008  beq cr6, 0x822e618c
	if ctx.cr[6].eq {
	pc = 0x822E618C; continue 'dispatch;
	}
	// 822E6188: 480020E1  bl 0x822e8268
	ctx.lr = 0x822E618C;
	sub_822E8268(ctx, base);
	// 822E618C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6190: 481AAB81  bl 0x82490d10
	ctx.lr = 0x822E6194;
	sub_82490D10(ctx, base);
	// 822E6194: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E6198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E619C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E61A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E61A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E61A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E61A8 size=100
    let mut pc: u32 = 0x822E61A8;
    'dispatch: loop {
        match pc {
            0x822E61A8 => {
    //   block [0x822E61A8..0x822E620C)
	// 822E61A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E61AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E61B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E61B4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E61B8: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E61BC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E61C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E61C4: 388B00A0  addi r4, r11, 0xa0
	ctx.r[4].s64 = ctx.r[11].s64 + 160;
	// 822E61C8: 480024D1  bl 0x822e8698
	ctx.lr = 0x822E61CC;
	sub_822E8698(ctx, base);
	// 822E61CC: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 822E61D0: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 822E61D4: 13E01C07  vcmpneb. (lvlx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E61D8: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 822E61DC: 13C91C07  vcmpneb. (lvlx128) v30, v9, v3
	tmp.u32 = ctx.r[9].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[62] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E61E0: 13AA1C07  vcmpneb. (lvlx128) v29, v10, v3
	tmp.u32 = ctx.r[10].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E61E4: 138B1C07  vcmpneb. (lvlx128) v28, v11, v3
	tmp.u32 = ctx.r[11].u32 + ctx.r[3].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E6210 size=68
    let mut pc: u32 = 0x822E6210;
    'dispatch: loop {
        match pc {
            0x822E6210 => {
    //   block [0x822E6210..0x822E6254)
	// 822E6210: 8164004C  lwz r11, 0x4c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E6214: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822E6218: 812B0078  lwz r9, 0x78(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 822E621C: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822E6220: 40990028  ble cr6, 0x822e6248
	if !ctx.cr[6].gt {
	pc = 0x822E6248; continue 'dispatch;
	}
	// 822E6224: 810B0074  lwz r8, 0x74(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 822E6228: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 822E622C: 80EB0000  lwz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6230: 7F072840  cmplw cr6, r7, r5
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[5].u32, &mut ctx.xer);
	// 822E6234: 419A0020  beq cr6, 0x822e6254
	if ctx.cr[6].eq {
		sub_822E6254(ctx, base);
		return;
	}
	// 822E6238: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822E623C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 822E6240: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 822E6244: 4198FFE8  blt cr6, 0x822e622c
	if ctx.cr[6].lt {
	pc = 0x822E622C; continue 'dispatch;
	}
	// 822E6248: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E624C: F9630000  std r11, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 822E6250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6254(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E6254 size=16
    let mut pc: u32 = 0x822E6254;
    'dispatch: loop {
        match pc {
            0x822E6254 => {
    //   block [0x822E6254..0x822E6264)
	// 822E6254: 554B2036  slwi r11, r10, 4
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822E6258: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 822E625C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 822E6260: 4BFFFFEC  b 0x822e624c
	sub_822E6210(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E6268 size=76
    let mut pc: u32 = 0x822E6268;
    'dispatch: loop {
        match pc {
            0x822E6268 => {
    //   block [0x822E6268..0x822E62B4)
	// 822E6268: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 822E626C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822E6270: 812B0078  lwz r9, 0x78(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(120 as u32) ) } as u64;
	// 822E6274: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822E6278: 40990024  ble cr6, 0x822e629c
	if !ctx.cr[6].gt {
	pc = 0x822E629C; continue 'dispatch;
	}
	// 822E627C: 816B0074  lwz r11, 0x74(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(116 as u32) ) } as u64;
	// 822E6280: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6284: 7F082040  cmplw cr6, r8, r4
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[4].u32, &mut ctx.xer);
	// 822E6288: 419A002C  beq cr6, 0x822e62b4
	if ctx.cr[6].eq {
		sub_822E62B4(ctx, base);
		return;
	}
	// 822E628C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822E6290: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 822E6294: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 822E6298: 4198FFE8  blt cr6, 0x822e6280
	if ctx.cr[6].lt {
	pc = 0x822E6280; continue 'dispatch;
	}
	// 822E629C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E62A0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 822E62A4: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 822E62A8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 822E62AC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 822E62B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E62B4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E62B4 size=8
    let mut pc: u32 = 0x822E62B4;
    'dispatch: loop {
        match pc {
            0x822E62B4 => {
    //   block [0x822E62B4..0x822E62BC)
	// 822E62B4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 822E62B8: 4BFFFFE8  b 0x822e62a0
	sub_822E6268(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E62C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E62C0 size=64
    let mut pc: u32 = 0x822E62C0;
    'dispatch: loop {
        match pc {
            0x822E62C0 => {
    //   block [0x822E62C0..0x822E6300)
	// 822E62C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E62C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E62C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E62CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E62D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E62D4: 80640048  lwz r3, 0x48(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(72 as u32) ) } as u64;
	// 822E62D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E62DC: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 822E62E0: 419A0008  beq cr6, 0x822e62e8
	if ctx.cr[6].eq {
	pc = 0x822E62E8; continue 'dispatch;
	}
	// 822E62E4: 48001F65  bl 0x822e8248
	ctx.lr = 0x822E62E8;
	sub_822E8248(ctx, base);
	// 822E62E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E62EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E62F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E62F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E62F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E62FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6300 size=76
    let mut pc: u32 = 0x822E6300;
    'dispatch: loop {
        match pc {
            0x822E6300 => {
    //   block [0x822E6300..0x822E634C)
	// 822E6300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6308: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E630C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6310: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6314: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E6318: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E631C: 4BFFFDED  bl 0x822e6108
	ctx.lr = 0x822E6320;
	sub_822E6108(ctx, base);
	// 822E6320: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E6324: 4182000C  beq 0x822e6330
	if ctx.cr[0].eq {
	pc = 0x822E6330; continue 'dispatch;
	}
	// 822E6328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E632C: 48B0C0AD  bl 0x82df23d8
	ctx.lr = 0x822E6330;
	sub_82DF23D8(ctx, base);
	// 822E6330: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6334: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E6338: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E633C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E6340: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E6344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E6348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6350 size=140
    let mut pc: u32 = 0x822E6350;
    'dispatch: loop {
        match pc {
            0x822E6350 => {
    //   block [0x822E6350..0x822E63DC)
	// 822E6350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6354: 48EC1E15  bl 0x831a8168
	ctx.lr = 0x822E6358;
	sub_831A8130(ctx, base);
	// 822E6358: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E635C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6360: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822E6364: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E6368: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E636C: 38A00031  li r5, 0x31
	ctx.r[5].s64 = 49;
	// 822E6370: 38800160  li r4, 0x160
	ctx.r[4].s64 = 352;
	// 822E6374: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E6378: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822E637C: 48BBA3B5  bl 0x82ea0730
	ctx.lr = 0x822E6380;
	sub_82EA0730(ctx, base);
	// 822E6380: 39600160  li r11, 0x160
	ctx.r[11].s64 = 352;
	// 822E6384: 3D408338  lis r10, -0x7cc8
	ctx.r[10].s64 = -2093481984;
	// 822E6388: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 822E638C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E6390: 38AA69F0  addi r5, r10, 0x69f0
	ctx.r[5].s64 = ctx.r[10].s64 + 27120;
	// 822E6394: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6398: 48BEFF89  bl 0x82ed6320
	ctx.lr = 0x822E639C;
	sub_82ED6320(ctx, base);
	// 822E639C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822E63A0: 93810050  stw r28, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[28].u32 ) };
	// 822E63A4: 387F004C  addi r3, r31, 0x4c
	ctx.r[3].s64 = ctx.r[31].s64 + 76;
	// 822E63A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822E63AC: 480008A5  bl 0x822e6c50
	ctx.lr = 0x822E63B0;
	sub_822E6C50(ctx, base);
	// 822E63B0: 93FC000C  stw r31, 0xc(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822E63B4: 387F0048  addi r3, r31, 0x48
	ctx.r[3].s64 = ctx.r[31].s64 + 72;
	// 822E63B8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E63BC: 48000895  bl 0x822e6c50
	ctx.lr = 0x822E63C0;
	sub_822E6C50(ctx, base);
	// 822E63C0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E63C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E63C8: 481A8FD9  bl 0x8248f3a0
	ctx.lr = 0x822E63CC;
	sub_8248F3A0(ctx, base);
	// 822E63CC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822E63D0: 48001E99  bl 0x822e8268
	ctx.lr = 0x822E63D4;
	sub_822E8268(ctx, base);
	// 822E63D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E63D8: 48EC1DE0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E63E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E63E0 size=88
    let mut pc: u32 = 0x822E63E0;
    'dispatch: loop {
        match pc {
            0x822E63E0 => {
    //   block [0x822E63E0..0x822E6438)
	// 822E63E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E63E4: 48EC1D89  bl 0x831a816c
	ctx.lr = 0x822E63E8;
	sub_831A8130(ctx, base);
	// 822E63E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E63EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E63F0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 822E63F4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 822E63F8: 481AAB29  bl 0x82490f20
	ctx.lr = 0x822E63FC;
	sub_82490F20(ctx, base);
	// 822E63FC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E6400: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E6404: 394AAC84  addi r10, r10, -0x537c
	ctx.r[10].s64 = ctx.r[10].s64 + -21372;
	// 822E6408: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E640C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822E6410: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822E6414: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 822E6418: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E641C: 917F004C  stw r11, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 822E6420: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822E6424: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822E6428: 4BFFFF29  bl 0x822e6350
	ctx.lr = 0x822E642C;
	sub_822E6350(ctx, base);
	// 822E642C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6430: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E6434: 48EC1D88  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6438 size=60
    let mut pc: u32 = 0x822E6438;
    'dispatch: loop {
        match pc {
            0x822E6438 => {
    //   block [0x822E6438..0x822E6474)
	// 822E6438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E643C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6440: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6444: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6448: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E644C: 8086001C  lwz r4, 0x1c(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E6450: 8065001C  lwz r3, 0x1c(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E6454: 481A57F5  bl 0x8248bc48
	ctx.lr = 0x822E6458;
	sub_8248BC48(ctx, base);
	// 822E6458: 987F0000  stb r3, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u8 ) };
	// 822E645C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6460: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E6464: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6468: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E646C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E6470: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6478(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6478 size=120
    let mut pc: u32 = 0x822E6478;
    'dispatch: loop {
        match pc {
            0x822E6478 => {
    //   block [0x822E6478..0x822E64F0)
	// 822E6478: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E647C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6480: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E6484: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E648C: 8166000C  lwz r11, 0xc(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E6490: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E6494: 4800000C  b 0x822e64a0
	pc = 0x822E64A0; continue 'dispatch;
	// 822E6498: 7D665B78  mr r6, r11
	ctx.r[6].u64 = ctx.r[11].u64;
	// 822E649C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E64A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E64A4: 409AFFF4  bne cr6, 0x822e6498
	if !ctx.cr[6].eq {
	pc = 0x822E6498; continue 'dispatch;
	}
	// 822E64A8: 81680000  lwz r11, 0(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E64AC: 7D244B78  mr r4, r9
	ctx.r[4].u64 = ctx.r[9].u64;
	// 822E64B0: 83C6001C  lwz r30, 0x1c(r6)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E64B4: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 822E64B8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E64BC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E64C0: 4E800421  bctrl
	ctx.lr = 0x822E64C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E64C4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E64C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E64CC: 481A577D  bl 0x8248bc48
	ctx.lr = 0x822E64D0;
	sub_8248BC48(ctx, base);
	// 822E64D0: 987F0000  stb r3, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u8 ) };
	// 822E64D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E64D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E64DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E64E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E64E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E64E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E64EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E64F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E64F0 size=100
    let mut pc: u32 = 0x822E64F0;
    'dispatch: loop {
        match pc {
            0x822E64F0 => {
    //   block [0x822E64F0..0x822E6554)
	// 822E64F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E64F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E64F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E64FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6504: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6508: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E650C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 822E6510: 7D044378  mr r4, r8
	ctx.r[4].u64 = ctx.r[8].u64;
	// 822E6514: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 822E6518: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E651C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E6520: 4E800421  bctrl
	ctx.lr = 0x822E6524;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E6524: 817E0020  lwz r11, 0x20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 822E6528: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E652C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 822E6530: 481A5719  bl 0x8248bc48
	ctx.lr = 0x822E6534;
	sub_8248BC48(ctx, base);
	// 822E6534: 987F0000  stb r3, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u8 ) };
	// 822E6538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E653C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E6540: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6544: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E6548: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E654C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E6550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6558 size=60
    let mut pc: u32 = 0x822E6558;
    'dispatch: loop {
        match pc {
            0x822E6558 => {
    //   block [0x822E6558..0x822E6594)
	// 822E6558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E655C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6560: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6564: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6568: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E656C: 8086001C  lwz r4, 0x1c(r6)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E6570: 80650024  lwz r3, 0x24(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(36 as u32) ) } as u64;
	// 822E6574: 481A56D5  bl 0x8248bc48
	ctx.lr = 0x822E6578;
	sub_8248BC48(ctx, base);
	// 822E6578: 987F0000  stb r3, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u8 ) };
	// 822E657C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6580: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E6584: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6588: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E658C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E6590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6598 size=196
    let mut pc: u32 = 0x822E6598;
    'dispatch: loop {
        match pc {
            0x822E6598 => {
    //   block [0x822E6598..0x822E665C)
	// 822E6598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E659C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E65A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E65A4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E65A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E65AC: 80BF01C4  lwz r5, 0x1c4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(452 as u32) ) } as u64;
	// 822E65B0: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 822E65B4: 419A004C  beq cr6, 0x822e6600
	if ctx.cr[6].eq {
	pc = 0x822E6600; continue 'dispatch;
	}
	// 822E65B8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E65BC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822E65C0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822E65C4: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 822E65C8: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 822E65CC: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 822E65D0: 41980014  blt cr6, 0x822e65e4
	if ctx.cr[6].lt {
	pc = 0x822E65E4; continue 'dispatch;
	}
	// 822E65D4: 38C0000D  li r6, 0xd
	ctx.r[6].s64 = 13;
	// 822E65D8: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 822E65DC: 48BB9A85  bl 0x82ea0060
	ctx.lr = 0x822E65E0;
	sub_82EA0060(ctx, base);
	// 822E65E0: 48000020  b 0x822e6600
	pc = 0x822E6600; continue 'dispatch;
	// 822E65E4: 8143009C  lwz r10, 0x9c(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 822E65E8: 39630098  addi r11, r3, 0x98
	ctx.r[11].s64 = ctx.r[3].s64 + 152;
	// 822E65EC: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 822E65F0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822E65F4: 9143009C  stw r10, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 822E65F8: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E65FC: 90A30098  stw r5, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[5].u32 ) };
	// 822E6600: 817F01DC  lwz r11, 0x1dc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(476 as u32) ) } as u64;
	// 822E6604: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E6608: 40820020  bne 0x822e6628
	if !ctx.cr[0].eq {
	pc = 0x822E6628; continue 'dispatch;
	}
	// 822E660C: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6610: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 822E6614: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 822E6618: 809F01D4  lwz r4, 0x1d4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(468 as u32) ) } as u64;
	// 822E661C: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 822E6620: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822E6624: 48BBA18D  bl 0x82ea07b0
	ctx.lr = 0x822E6628;
	sub_82EA07B0(ctx, base);
	// 822E6628: 387F0140  addi r3, r31, 0x140
	ctx.r[3].s64 = ctx.r[31].s64 + 320;
	// 822E662C: 48BBBA15  bl 0x82ea2040
	ctx.lr = 0x822E6630;
	sub_82EA2040(ctx, base);
	// 822E6630: 387F0100  addi r3, r31, 0x100
	ctx.r[3].s64 = ctx.r[31].s64 + 256;
	// 822E6634: 48BBBFF5  bl 0x82ea2628
	ctx.lr = 0x822E6638;
	sub_82EA2628(ctx, base);
	// 822E6638: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 822E663C: 48BBBFED  bl 0x82ea2628
	ctx.lr = 0x822E6640;
	sub_82EA2628(ctx, base);
	// 822E6640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6644: 48BBBFE5  bl 0x82ea2628
	ctx.lr = 0x822E6648;
	sub_82EA2628(ctx, base);
	// 822E6648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E664C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E6654: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E6658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6660 size=64
    let mut pc: u32 = 0x822E6660;
    'dispatch: loop {
        match pc {
            0x822E6660 => {
    //   block [0x822E6660..0x822E66A0)
	// 822E6660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6668: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E666C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6670: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E6674: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6678: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E667C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 822E6680: 419A0008  beq cr6, 0x822e6688
	if ctx.cr[6].eq {
	pc = 0x822E6688; continue 'dispatch;
	}
	// 822E6684: 48001BC5  bl 0x822e8248
	ctx.lr = 0x822E6688;
	sub_822E8248(ctx, base);
	// 822E6688: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E668C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E6690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E6698: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E669C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E66A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E66A0 size=124
    let mut pc: u32 = 0x822E66A0;
    'dispatch: loop {
        match pc {
            0x822E66A0 => {
    //   block [0x822E66A0..0x822E671C)
	// 822E66A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E66A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E66A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E66AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E66B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E66B4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 822E66B8: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E66BC: 40820024  bne 0x822e66e0
	if !ctx.cr[0].eq {
	pc = 0x822E66E0; continue 'dispatch;
	}
	// 822E66C0: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E66C4: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 822E66C8: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 822E66CC: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E66D0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 822E66D4: 1CAB000C  mulli r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 * 12;
	// 822E66D8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822E66DC: 48BBA0D5  bl 0x82ea07b0
	ctx.lr = 0x822E66E0;
	sub_82EA07B0(ctx, base);
	// 822E66E0: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E66E4: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E66E8: 40820020  bne 0x822e6708
	if !ctx.cr[0].eq {
	pc = 0x822E6708; continue 'dispatch;
	}
	// 822E66EC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E66F0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 822E66F4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 822E66F8: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E66FC: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 822E6700: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822E6704: 48BBA0AD  bl 0x82ea07b0
	ctx.lr = 0x822E6708;
	sub_82EA07B0(ctx, base);
	// 822E6708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E670C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E6714: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E6718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6720 size=196
    let mut pc: u32 = 0x822E6720;
    'dispatch: loop {
        match pc {
            0x822E6720 => {
    //   block [0x822E6720..0x822E67E4)
	// 822E6720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6728: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E672C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6730: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6734: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E6738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E673C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 822E6740: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E6744: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E6748: 4BFDA1F1  bl 0x822c0938
	ctx.lr = 0x822E674C;
	sub_822C0938(ctx, base);
	// 822E674C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E6750: 41820028  beq 0x822e6778
	if ctx.cr[0].eq {
	pc = 0x822E6778; continue 'dispatch;
	}
	// 822E6754: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E6758: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822E675C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822E6760: 392BACBC  addi r9, r11, -0x5344
	ctx.r[9].s64 = ctx.r[11].s64 + -21316;
	// 822E6764: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822E6768: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E676C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E6770: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822E6774: 48000008  b 0x822e677c
	pc = 0x822E677C; continue 'dispatch;
	// 822E6778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E677C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E6780: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E6784: 409A0044  bne cr6, 0x822e67c8
	if !ctx.cr[6].eq {
	pc = 0x822E67C8; continue 'dispatch;
	}
	// 822E6788: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E678C: 419A001C  beq cr6, 0x822e67a8
	if ctx.cr[6].eq {
	pc = 0x822E67A8; continue 'dispatch;
	}
	// 822E6790: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6794: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E6798: 419A0008  beq cr6, 0x822e67a0
	if ctx.cr[6].eq {
	pc = 0x822E67A0; continue 'dispatch;
	}
	// 822E679C: 48001ACD  bl 0x822e8268
	ctx.lr = 0x822E67A0;
	sub_822E8268(ctx, base);
	// 822E67A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E67A4: 4BFD9AC5  bl 0x822c0268
	ctx.lr = 0x822E67A8;
	sub_822C0268(ctx, base);
	// 822E67A8: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E67AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822E67B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E67B4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 822E67B8: 816B3E3C  lwz r11, 0x3e3c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(15932 as u32) ) } as u64;
	// 822E67BC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822E67C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822E67C4: 4BFD983D  bl 0x822c0000
	ctx.lr = 0x822E67C8;
	sub_822C0000(ctx, base);
	// 822E67C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E67CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E67D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E67D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E67D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E67DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E67E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E67E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E67E8 size=72
    let mut pc: u32 = 0x822E67E8;
    'dispatch: loop {
        match pc {
            0x822E67E8 => {
    //   block [0x822E67E8..0x822E6830)
	// 822E67E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E67EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E67F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E67F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E67F8: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E67FC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E6800: 419A001C  beq cr6, 0x822e681c
	if ctx.cr[6].eq {
	pc = 0x822E681C; continue 'dispatch;
	}
	// 822E6804: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6808: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E680C: 419A0008  beq cr6, 0x822e6814
	if ctx.cr[6].eq {
	pc = 0x822E6814; continue 'dispatch;
	}
	// 822E6810: 48001A59  bl 0x822e8268
	ctx.lr = 0x822E6814;
	sub_822E8268(ctx, base);
	// 822E6814: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6818: 4BFD9A51  bl 0x822c0268
	ctx.lr = 0x822E681C;
	sub_822C0268(ctx, base);
	// 822E681C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E6820: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6824: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E6828: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E682C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E6830 size=416
    let mut pc: u32 = 0x822E6830;
    'dispatch: loop {
        match pc {
            0x822E6830 => {
    //   block [0x822E6830..0x822E69D0)
	// 822E6830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6834: 48EC1935  bl 0x831a8168
	ctx.lr = 0x822E6838;
	sub_831A8130(ctx, base);
	// 822E6838: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E683C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822E6840: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822E6844: 93E10084  stw r31, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[31].u32 ) };
	// 822E6848: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E684C: 93E10088  stw r31, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[31].u32 ) };
	// 822E6850: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 822E6854: 93E1008C  stw r31, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[31].u32 ) };
	// 822E6858: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822E685C: 4BFF5B3D  bl 0x822dc398
	ctx.lr = 0x822E6860;
	sub_822DC398(ctx, base);
	// 822E6860: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 822E6864: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	// 822E6868: 389EFFF8  addi r4, r30, -8
	ctx.r[4].s64 = ctx.r[30].s64 + -8;
	// 822E686C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822E6870: 4BFF5FD1  bl 0x822dc840
	ctx.lr = 0x822E6874;
	sub_822DC840(ctx, base);
	// 822E6874: 389EFFFC  addi r4, r30, -4
	ctx.r[4].s64 = ctx.r[30].s64 + -4;
	// 822E6878: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822E687C: 4BFF5FC5  bl 0x822dc840
	ctx.lr = 0x822E6880;
	sub_822DC840(ctx, base);
	// 822E6880: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E6884: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822E6888: 4BFF5FB9  bl 0x822dc840
	ctx.lr = 0x822E688C;
	sub_822DC840(ctx, base);
	// 822E688C: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822E6890: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 822E6894: 4082FFD4  bne 0x822e6868
	if !ctx.cr[0].eq {
	pc = 0x822E6868; continue 'dispatch;
	}
	// 822E6898: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 822E689C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E68A0: 409A000C  bne cr6, 0x822e68ac
	if !ctx.cr[6].eq {
	pc = 0x822E68AC; continue 'dispatch;
	}
	// 822E68A4: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 822E68A8: 48000010  b 0x822e68b8
	pc = 0x822E68B8; continue 'dispatch;
	// 822E68AC: 81410088  lwz r10, 0x88(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 822E68B0: 7D6B5050  subf r11, r11, r10
	ctx.r[11].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 822E68B4: 7D6B1670  srawi r11, r11, 2
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 2) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 2) as i64;
	// 822E68B8: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 822E68BC: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 822E68C0: 7D6B5396  divwu r11, r11, r10
	ctx.r[11].u32 = ctx.r[11].u32 / ctx.r[10].u32;
	// 822E68C4: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 822E68C8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E68CC: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 822E68D0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822E68D4: 4818377D  bl 0x8246a050
	ctx.lr = 0x822E68D8;
	sub_8246A050(ctx, base);
	// 822E68D8: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 822E68DC: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 822E68E0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 822E68E4: 93E10090  stw r31, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[31].u32 ) };
	// 822E68E8: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 822E68EC: 93E10094  stw r31, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[31].u32 ) };
	// 822E68F0: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 822E68F4: 91610098  stw r11, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 822E68F8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822E68FC: 93E1009C  stw r31, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[31].u32 ) };
	// 822E6900: 93E100A0  stw r31, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[31].u32 ) };
	// 822E6904: 916100A4  stw r11, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 822E6908: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 822E690C: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 822E6910: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 822E6914: 48C79305  bl 0x82f5fc18
	ctx.lr = 0x822E6918;
	sub_82F5FC18(ctx, base);
	// 822E6918: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
	// 822E691C: 81410090  lwz r10, 0x90(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 822E6920: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 822E6924: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 822E6928: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 822E692C: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6930: 38800060  li r4, 0x60
	ctx.r[4].s64 = 96;
	// 822E6934: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 822E6938: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 822E693C: 81610094  lwz r11, 0x94(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 822E6940: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 822E6944: 48BB9DED  bl 0x82ea0730
	ctx.lr = 0x822E6948;
	sub_82EA0730(ctx, base);
	// 822E6948: 39600060  li r11, 0x60
	ctx.r[11].s64 = 96;
	// 822E694C: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 822E6950: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 822E6954: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 822E6958: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 822E695C: C02AF614  lfs f1, -0x9ec(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2540 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822E6960: 48C34F21  bl 0x82f1b880
	ctx.lr = 0x822E6964;
	sub_82F1B880(ctx, base);
	// 822E6964: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E6968: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 822E696C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822E6970: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 822E6974: 480002DD  bl 0x822e6c50
	ctx.lr = 0x822E6978;
	sub_822E6C50(ctx, base);
	// 822E6978: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E697C: 419A000C  beq cr6, 0x822e6988
	if ctx.cr[6].eq {
	pc = 0x822E6988; continue 'dispatch;
	}
	// 822E6980: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6984: 480018E5  bl 0x822e8268
	ctx.lr = 0x822E6988;
	sub_822E8268(ctx, base);
	// 822E6988: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E698C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6990: C00B9528  lfs f0, -0x6ad8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E6994: D00A0010  stfs f0, 0x10(r10)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 822E6998: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 822E699C: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E69A0: 40820018  bne 0x822e69b8
	if !ctx.cr[0].eq {
	pc = 0x822E69B8; continue 'dispatch;
	}
	// 822E69A4: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 822E69A8: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 822E69AC: 55652036  slwi r5, r11, 4
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 822E69B0: 80810068  lwz r4, 0x68(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 822E69B4: 48BB9DFD  bl 0x82ea07b0
	ctx.lr = 0x822E69B8;
	sub_82EA07B0(ctx, base);
	// 822E69B8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 822E69BC: 4BFFFCE5  bl 0x822e66a0
	ctx.lr = 0x822E69C0;
	sub_822E66A0(ctx, base);
	// 822E69C0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822E69C4: 4818363D  bl 0x8246a000
	ctx.lr = 0x822E69C8;
	sub_8246A000(ctx, base);
	// 822E69C8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 822E69CC: 48EC17EC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E69D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E69D0 size=132
    let mut pc: u32 = 0x822E69D0;
    'dispatch: loop {
        match pc {
            0x822E69D0 => {
    //   block [0x822E69D0..0x822E6A54)
	// 822E69D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E69D4: 48EC1795  bl 0x831a8168
	ctx.lr = 0x822E69D8;
	sub_831A8130(ctx, base);
	// 822E69D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E69DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E69E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E69E4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 822E69E8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E69EC: 388BACCC  addi r4, r11, -0x5334
	ctx.r[4].s64 = ctx.r[11].s64 + -21300;
	// 822E69F0: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 822E69F4: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 822E69F8: 4BFD99E1  bl 0x822c03d8
	ctx.lr = 0x822E69FC;
	sub_822C03D8(ctx, base);
	// 822E69FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E6A00: 41820014  beq 0x822e6a14
	if ctx.cr[0].eq {
	pc = 0x822E6A14; continue 'dispatch;
	}
	// 822E6A04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E6A08: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E6A0C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E6A10: 48000008  b 0x822e6a18
	pc = 0x822E6A18; continue 'dispatch;
	// 822E6A14: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822E6A18: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 822E6A1C: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 822E6A20: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E6A24: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E6A28: 4BFFFCF9  bl 0x822e6720
	ctx.lr = 0x822E6A2C;
	sub_822E6720(ctx, base);
	// 822E6A2C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822E6A30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E6A34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E6A38: 4BFD95C9  bl 0x822c0000
	ctx.lr = 0x822E6A3C;
	sub_822C0000(ctx, base);
	// 822E6A3C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E6A40: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6A44: 4BFFFDED  bl 0x822e6830
	ctx.lr = 0x822E6A48;
	sub_822E6830(ctx, base);
	// 822E6A48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E6A4C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E6A50: 48EC1768  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6A58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E6A58 size=8
    let mut pc: u32 = 0x822E6A58;
    'dispatch: loop {
        match pc {
            0x822E6A58 => {
    //   block [0x822E6A58..0x822E6A60)
	// 822E6A58: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E6A5C: 484F357C  b 0x827d9fd8
	sub_827D9FD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6A60 size=144
    let mut pc: u32 = 0x822E6A60;
    'dispatch: loop {
        match pc {
            0x822E6A60 => {
    //   block [0x822E6A60..0x822E6AF0)
	// 822E6A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6A68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6A6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6A70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E6A74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E6A78: 396BAD08  addi r11, r11, -0x52f8
	ctx.r[11].s64 = ctx.r[11].s64 + -21240;
	// 822E6A7C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E6A80: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E6A84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E6A88: 419A000C  beq cr6, 0x822e6a94
	if ctx.cr[6].eq {
	pc = 0x822E6A94; continue 'dispatch;
	}
	// 822E6A8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E6A90: 48005C21  bl 0x822ec6b0
	ctx.lr = 0x822E6A94;
	sub_822EC6B0(ctx, base);
	// 822E6A94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E6A98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6A9C: 484F353D  bl 0x827d9fd8
	ctx.lr = 0x822E6AA0;
	sub_827D9FD8(ctx, base);
	// 822E6AA0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E6AA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822E6AA8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E6AAC: 419A0008  beq cr6, 0x822e6ab4
	if ctx.cr[6].eq {
	pc = 0x822E6AB4; continue 'dispatch;
	}
	// 822E6AB0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 822E6AB4: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 822E6AB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E6ABC: 419A0008  beq cr6, 0x822e6ac4
	if ctx.cr[6].eq {
	pc = 0x822E6AC4; continue 'dispatch;
	}
	// 822E6AC0: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 822E6AC4: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 822E6AC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E6ACC: 419A0008  beq cr6, 0x822e6ad4
	if ctx.cr[6].eq {
	pc = 0x822E6AD4; continue 'dispatch;
	}
	// 822E6AD0: 48001799  bl 0x822e8268
	ctx.lr = 0x822E6AD4;
	sub_822E8268(ctx, base);
	// 822E6AD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6AD8: 481AA239  bl 0x82490d10
	ctx.lr = 0x822E6ADC;
	sub_82490D10(ctx, base);
	// 822E6ADC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E6AE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6AE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E6AE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E6AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6AF0 size=100
    let mut pc: u32 = 0x822E6AF0;
    'dispatch: loop {
        match pc {
            0x822E6AF0 => {
    //   block [0x822E6AF0..0x822E6B54)
	// 822E6AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6AF4: 48EC1679  bl 0x831a816c
	ctx.lr = 0x822E6AF8;
	sub_831A8130(ctx, base);
	// 822E6AF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6AFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E6B00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E6B04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E6B08: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E6B0C: 48005C25  bl 0x822ec730
	ctx.lr = 0x822E6B10;
	sub_822EC730(ctx, base);
	// 822E6B10: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 822E6B14: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E6B18: 419A0034  beq cr6, 0x822e6b4c
	if ctx.cr[6].eq {
	pc = 0x822E6B4C; continue 'dispatch;
	}
	// 822E6B1C: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6B20: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E6B24: 419A000C  beq cr6, 0x822e6b30
	if ctx.cr[6].eq {
	pc = 0x822E6B30; continue 'dispatch;
	}
	// 822E6B28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6B2C: 48BE7CD5  bl 0x82ece800
	ctx.lr = 0x822E6B30;
	sub_82ECE800(ctx, base);
	// 822E6B30: 809D0048  lwz r4, 0x48(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 822E6B34: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6B38: 48BE7F09  bl 0x82ecea40
	ctx.lr = 0x822E6B3C;
	sub_82ECEA40(ctx, base);
	// 822E6B3C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E6B40: 419A000C  beq cr6, 0x822e6b4c
	if ctx.cr[6].eq {
	pc = 0x822E6B4C; continue 'dispatch;
	}
	// 822E6B44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6B48: 48BE64C1  bl 0x82ecd008
	ctx.lr = 0x822E6B4C;
	sub_82ECD008(ctx, base);
	// 822E6B4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E6B50: 48EC166C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6B58 size=112
    let mut pc: u32 = 0x822E6B58;
    'dispatch: loop {
        match pc {
            0x822E6B58 => {
    //   block [0x822E6B58..0x822E6BC8)
	// 822E6B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6B5C: 48EC1611  bl 0x831a816c
	ctx.lr = 0x822E6B60;
	sub_831A8130(ctx, base);
	// 822E6B60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6B64: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E6B68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E6B6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E6B70: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E6B74: 48005CCD  bl 0x822ec840
	ctx.lr = 0x822E6B78;
	sub_822EC840(ctx, base);
	// 822E6B78: 817D0048  lwz r11, 0x48(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 822E6B7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E6B80: 419A0040  beq cr6, 0x822e6bc0
	if ctx.cr[6].eq {
	pc = 0x822E6BC0; continue 'dispatch;
	}
	// 822E6B84: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E6B88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E6B8C: 419A0034  beq cr6, 0x822e6bc0
	if ctx.cr[6].eq {
	pc = 0x822E6BC0; continue 'dispatch;
	}
	// 822E6B90: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6B94: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E6B98: 419A000C  beq cr6, 0x822e6ba4
	if ctx.cr[6].eq {
	pc = 0x822E6BA4; continue 'dispatch;
	}
	// 822E6B9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6BA0: 48BE7C61  bl 0x82ece800
	ctx.lr = 0x822E6BA4;
	sub_82ECE800(ctx, base);
	// 822E6BA4: 809D0048  lwz r4, 0x48(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 822E6BA8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6BAC: 48BE69C5  bl 0x82ecd570
	ctx.lr = 0x822E6BB0;
	sub_82ECD570(ctx, base);
	// 822E6BB0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E6BB4: 419A000C  beq cr6, 0x822e6bc0
	if ctx.cr[6].eq {
	pc = 0x822E6BC0; continue 'dispatch;
	}
	// 822E6BB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6BBC: 48BE644D  bl 0x82ecd008
	ctx.lr = 0x822E6BC0;
	sub_82ECD008(ctx, base);
	// 822E6BC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E6BC4: 48EC15F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6BC8 size=80
    let mut pc: u32 = 0x822E6BC8;
    'dispatch: loop {
        match pc {
            0x822E6BC8 => {
    //   block [0x822E6BC8..0x822E6C18)
	// 822E6BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6BD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E6BD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6BD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6BDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E6BE0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E6BE4: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E6BE8: 48001B21  bl 0x822e8708
	ctx.lr = 0x822E6BEC;
	sub_822E8708(ctx, base);
	// 822E6BEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E6BF0: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 822E6BF4: 48001B5D  bl 0x822e8750
	ctx.lr = 0x822E6BF8;
	sub_822E8750(ctx, base);
	// 822E6BF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6BFC: 481A8BBD  bl 0x8248f7b8
	ctx.lr = 0x822E6C00;
	sub_8248F7B8(ctx, base);
	// 822E6C00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E6C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E6C0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E6C10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E6C14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6C18 size=56
    let mut pc: u32 = 0x822E6C18;
    'dispatch: loop {
        match pc {
            0x822E6C18 => {
    //   block [0x822E6C18..0x822E6C50)
	// 822E6C18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6C1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6C20: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6C24: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6C28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E6C2C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 822E6C30: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E6C34: 48B0C57D  bl 0x82df31b0
	ctx.lr = 0x822E6C38;
	sub_82DF31B0(ctx, base);
	// 822E6C38: 907F0070  stw r3, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[3].u32 ) };
	// 822E6C3C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E6C40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6C44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E6C48: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E6C4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6C50 size=92
    let mut pc: u32 = 0x822E6C50;
    'dispatch: loop {
        match pc {
            0x822E6C50 => {
    //   block [0x822E6C50..0x822E6CAC)
	// 822E6C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6C58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E6C5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6C60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6C64: 83E40000  lwz r31, 0(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6C68: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E6C6C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E6C70: 419A000C  beq cr6, 0x822e6c7c
	if ctx.cr[6].eq {
	pc = 0x822E6C7C; continue 'dispatch;
	}
	// 822E6C74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6C78: 480015D1  bl 0x822e8248
	ctx.lr = 0x822E6C7C;
	sub_822E8248(ctx, base);
	// 822E6C7C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6C80: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 822E6C84: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E6C88: 419A0008  beq cr6, 0x822e6c90
	if ctx.cr[6].eq {
	pc = 0x822E6C90; continue 'dispatch;
	}
	// 822E6C8C: 480015DD  bl 0x822e8268
	ctx.lr = 0x822E6C90;
	sub_822E8268(ctx, base);
	// 822E6C90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E6C94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E6C98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6C9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E6CA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E6CA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E6CA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6CB0 size=68
    let mut pc: u32 = 0x822E6CB0;
    'dispatch: loop {
        match pc {
            0x822E6CB0 => {
    //   block [0x822E6CB0..0x822E6CF4)
	// 822E6CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6CB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6CBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6CC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E6CC4: 481AA25D  bl 0x82490f20
	ctx.lr = 0x822E6CC8;
	sub_82490F20(ctx, base);
	// 822E6CC8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E6CCC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822E6CD0: 396BAD08  addi r11, r11, -0x52f8
	ctx.r[11].s64 = ctx.r[11].s64 + -21240;
	// 822E6CD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6CD8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E6CDC: 915F0048  stw r10, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[10].u32 ) };
	// 822E6CE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E6CE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6CE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E6CEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E6CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6CF8 size=76
    let mut pc: u32 = 0x822E6CF8;
    'dispatch: loop {
        match pc {
            0x822E6CF8 => {
    //   block [0x822E6CF8..0x822E6D44)
	// 822E6CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6D00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E6D04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6D08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6D0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E6D10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E6D14: 4BFFFD4D  bl 0x822e6a60
	ctx.lr = 0x822E6D18;
	sub_822E6A60(ctx, base);
	// 822E6D18: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E6D1C: 4182000C  beq 0x822e6d28
	if ctx.cr[0].eq {
	pc = 0x822E6D28; continue 'dispatch;
	}
	// 822E6D20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6D24: 48B0B6B5  bl 0x82df23d8
	ctx.lr = 0x822E6D28;
	sub_82DF23D8(ctx, base);
	// 822E6D28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6D2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E6D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E6D38: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E6D3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E6D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6D48 size=56
    let mut pc: u32 = 0x822E6D48;
    'dispatch: loop {
        match pc {
            0x822E6D48 => {
    //   block [0x822E6D48..0x822E6D80)
	// 822E6D48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6D4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6D50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6D54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6D58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E6D5C: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 822E6D60: 4BFFFEF1  bl 0x822e6c50
	ctx.lr = 0x822E6D64;
	sub_822E6C50(ctx, base);
	// 822E6D64: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E6D68: 93EB000C  stw r31, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822E6D6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E6D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E6D78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E6D7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6D80 size=248
    let mut pc: u32 = 0x822E6D80;
    'dispatch: loop {
        match pc {
            0x822E6D80 => {
    //   block [0x822E6D80..0x822E6E78)
	// 822E6D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6D84: 48EC13E1  bl 0x831a8164
	ctx.lr = 0x822E6D88;
	sub_831A8130(ctx, base);
	// 822E6D88: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6D8C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E6D90: 83FE0004  lwz r31, 4(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E6D94: 837F0008  lwz r27, 8(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E6D98: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 822E6D9C: 419A000C  beq cr6, 0x822e6da8
	if ctx.cr[6].eq {
	pc = 0x822E6DA8; continue 'dispatch;
	}
	// 822E6DA0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822E6DA4: 48BE7A5D  bl 0x82ece800
	ctx.lr = 0x822E6DA8;
	sub_82ECE800(ctx, base);
	// 822E6DA8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6DAC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822E6DB0: 38A00031  li r5, 0x31
	ctx.r[5].s64 = 49;
	// 822E6DB4: 38800180  li r4, 0x180
	ctx.r[4].s64 = 384;
	// 822E6DB8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822E6DBC: 48BB9975  bl 0x82ea0730
	ctx.lr = 0x822E6DC0;
	sub_82EA0730(ctx, base);
	// 822E6DC0: 39600180  li r11, 0x180
	ctx.r[11].s64 = 384;
	// 822E6DC4: 38DF00E0  addi r6, r31, 0xe0
	ctx.r[6].s64 = ctx.r[31].s64 + 224;
	// 822E6DC8: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 822E6DCC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E6DD0: 80FF002C  lwz r7, 0x2c(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 822E6DD4: 3B9F0010  addi r28, r31, 0x10
	ctx.r[28].s64 = ctx.r[31].s64 + 16;
	// 822E6DD8: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E6DDC: 48000DE5  bl 0x822e7bc0
	ctx.lr = 0x822E6DE0;
	sub_822E7BC0(ctx, base);
	// 822E6DE0: 397E0048  addi r11, r30, 0x48
	ctx.r[11].s64 = ctx.r[30].s64 + 72;
	// 822E6DE4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E6DE8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822E6DEC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 822E6DF0: 480BF6C9  bl 0x823a64b8
	ctx.lr = 0x822E6DF4;
	sub_823A64B8(ctx, base);
	// 822E6DF4: 3D40FFAD  lis r10, -0x53
	ctx.r[10].s64 = -5439488;
	// 822E6DF8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 822E6DFC: 615DFF2F  ori r29, r10, 0xff2f
	ctx.r[29].u64 = ctx.r[10].u64 | 65327;
	// 822E6E00: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E6E04: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822E6E08: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E6E0C: 815E0048  lwz r10, 0x48(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 822E6E10: 93CA000C  stw r30, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 822E6E14: 806B7A1C  lwz r3, 0x7a1c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(31260 as u32) ) } as u64;
	// 822E6E18: 48BDFA69  bl 0x82ec6880
	ctx.lr = 0x822E6E1C;
	sub_82EC6880(ctx, base);
	// 822E6E1C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 822E6E20: 38801130  li r4, 0x1130
	ctx.r[4].s64 = 4400;
	// 822E6E24: FBA10050  std r29, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u64 ) };
	// 822E6E28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E6E2C: E8AB0000  ld r5, 0(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 822E6E30: 48BEE331  bl 0x82ed5160
	ctx.lr = 0x822E6E34;
	sub_82ED5160(ctx, base);
	// 822E6E34: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E6E38: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E6E3C: 388BAD3C  addi r4, r11, -0x52c4
	ctx.r[4].s64 = ctx.r[11].s64 + -21188;
	// 822E6E40: 38A00144  li r5, 0x144
	ctx.r[5].s64 = 324;
	// 822E6E44: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 822E6E48: 4BFD9591  bl 0x822c03d8
	ctx.lr = 0x822E6E4C;
	sub_822C03D8(ctx, base);
	// 822E6E4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E6E50: 41820010  beq 0x822e6e60
	if ctx.cr[0].eq {
	pc = 0x822E6E60; continue 'dispatch;
	}
	// 822E6E54: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822E6E58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E6E5C: 48000895  bl 0x822e76f0
	ctx.lr = 0x822E6E60;
	sub_822E76F0(ctx, base);
	// 822E6E60: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 822E6E64: 419A000C  beq cr6, 0x822e6e70
	if ctx.cr[6].eq {
	pc = 0x822E6E70; continue 'dispatch;
	}
	// 822E6E68: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822E6E6C: 48BE619D  bl 0x82ecd008
	ctx.lr = 0x822E6E70;
	sub_82ECD008(ctx, base);
	// 822E6E70: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 822E6E74: 48EC1340  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6E78 size=80
    let mut pc: u32 = 0x822E6E78;
    'dispatch: loop {
        match pc {
            0x822E6E78 => {
    //   block [0x822E6E78..0x822E6EC8)
	// 822E6E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6E80: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E6E84: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6E88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6E8C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E6E90: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E6E94: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6E98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E6E9C: 419A0010  beq cr6, 0x822e6eac
	if ctx.cr[6].eq {
	pc = 0x822E6EAC; continue 'dispatch;
	}
	// 822E6EA0: 48BB9189  bl 0x82ea0028
	ctx.lr = 0x822E6EA4;
	sub_82EA0028(ctx, base);
	// 822E6EA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E6EA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E6EAC: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 822E6EB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E6EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E6EBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E6EC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E6EC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E6EC8 size=44
    let mut pc: u32 = 0x822E6EC8;
    'dispatch: loop {
        match pc {
            0x822E6EC8 => {
    //   block [0x822E6EC8..0x822E6EF4)
	// 822E6EC8: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822E6ECC: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 822E6ED0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E6ED4: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 822E6ED8: 2F072800  cmpwi cr6, r7, 0x2800
	ctx.cr[6].compare_i32(ctx.r[7].s32, 10240, &mut ctx.xer);
	// 822E6EDC: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822E6EE0: 388AAD74  addi r4, r10, -0x528c
	ctx.r[4].s64 = ctx.r[10].s64 + -21132;
	// 822E6EE4: 40990010  ble cr6, 0x822e6ef4
	if !ctx.cr[6].gt {
		sub_822E6EF4(ctx, base);
		return;
	}
	// 822E6EE8: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 822E6EEC: 38A00082  li r5, 0x82
	ctx.r[5].s64 = 130;
	// 822E6EF0: 48B0B400  b 0x82df22f0
	sub_82DF22F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6EF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E6EF4 size=12
    let mut pc: u32 = 0x822E6EF4;
    'dispatch: loop {
        match pc {
            0x822E6EF4 => {
    //   block [0x822E6EF4..0x822E6F00)
	// 822E6EF4: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 822E6EF8: 38A00085  li r5, 0x85
	ctx.r[5].s64 = 133;
	// 822E6EFC: 48B0B3F4  b 0x82df22f0
	sub_82DF22F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E6F00 size=16
    let mut pc: u32 = 0x822E6F00;
    'dispatch: loop {
        match pc {
            0x822E6F00 => {
    //   block [0x822E6F00..0x822E6F10)
	// 822E6F00: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822E6F04: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E6F08: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822E6F0C: 48B0B27C  b 0x82df2188
	sub_82DF2188(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E6F10 size=20
    let mut pc: u32 = 0x822E6F10;
    'dispatch: loop {
        match pc {
            0x822E6F10 => {
    //   block [0x822E6F10..0x822E6F24)
	// 822E6F10: 80630008  lwz r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E6F14: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6F18: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 822E6F1C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E6F20: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6F28 size=96
    let mut pc: u32 = 0x822E6F28;
    'dispatch: loop {
        match pc {
            0x822E6F28 => {
    //   block [0x822E6F28..0x822E6F88)
	// 822E6F28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6F2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E6F30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E6F34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6F38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E6F3C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6F40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E6F44: 419A0030  beq cr6, 0x822e6f74
	if ctx.cr[6].eq {
	pc = 0x822E6F74; continue 'dispatch;
	}
	// 822E6F48: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E6F4C: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E6F50: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822E6F54: 40820018  bne 0x822e6f6c
	if !ctx.cr[0].eq {
	pc = 0x822E6F6C; continue 'dispatch;
	}
	// 822E6F58: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6F5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822E6F60: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E6F64: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E6F68: 4E800421  bctrl
	ctx.lr = 0x822E6F6C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E6F6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E6F70: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E6F74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E6F78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E6F7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E6F80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E6F84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E6F88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E6F88 size=468
    let mut pc: u32 = 0x822E6F88;
    'dispatch: loop {
        match pc {
            0x822E6F88 => {
    //   block [0x822E6F88..0x822E715C)
	// 822E6F88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E6F8C: 48EC11D9  bl 0x831a8164
	ctx.lr = 0x822E6F90;
	sub_831A8130(ctx, base);
	// 822E6F90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E6F94: 3D208332  lis r9, -0x7cce
	ctx.r[9].s64 = -2093875200;
	// 822E6F98: 3FA08332  lis r29, -0x7cce
	ctx.r[29].s64 = -2093875200;
	// 822E6F9C: 3D60822E  lis r11, -0x7dd2
	ctx.r[11].s64 = -2110914560;
	// 822E6FA0: 3D40822E  lis r10, -0x7dd2
	ctx.r[10].s64 = -2110914560;
	// 822E6FA4: 396B6EC8  addi r11, r11, 0x6ec8
	ctx.r[11].s64 = ctx.r[11].s64 + 28360;
	// 822E6FA8: 394A6F00  addi r10, r10, 0x6f00
	ctx.r[10].s64 = ctx.r[10].s64 + 28416;
	// 822E6FAC: 917DBDBC  stw r11, -0x4244(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(-16964 as u32), ctx.r[11].u32 ) };
	// 822E6FB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E6FB4: 9149BDC0  stw r10, -0x4240(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-16960 as u32), ctx.r[10].u32 ) };
	// 822E6FB8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 822E6FBC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 822E6FC0: 38600D80  li r3, 0xd80
	ctx.r[3].s64 = 3456;
	// 822E6FC4: 4BFFFF05  bl 0x822e6ec8
	ctx.lr = 0x822E6FC8;
	sub_822E6EC8(ctx, base);
	// 822E6FC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E6FCC: 41820010  beq 0x822e6fdc
	if ctx.cr[0].eq {
	pc = 0x822E6FDC; continue 'dispatch;
	}
	// 822E6FD0: 48BBDB09  bl 0x82ea4ad8
	ctx.lr = 0x822E6FD4;
	sub_82EA4AD8(ctx, base);
	// 822E6FD4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 822E6FD8: 48000008  b 0x822e6fe0
	pc = 0x822E6FE0; continue 'dispatch;
	// 822E6FDC: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 822E6FE0: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 822E6FE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E6FE8: 4BFFFF41  bl 0x822e6f28
	ctx.lr = 0x822E6FEC;
	sub_822E6F28(ctx, base);
	// 822E6FEC: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 822E6FF0: 817DBDBC  lwz r11, -0x4244(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-16964 as u32) ) } as u64;
	// 822E6FF4: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 822E6FF8: 38600330  li r3, 0x330
	ctx.r[3].s64 = 816;
	// 822E6FFC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E7000: 4E800421  bctrl
	ctx.lr = 0x822E7004;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E7004: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E7008: 41820018  beq 0x822e7020
	if ctx.cr[0].eq {
	pc = 0x822E7020; continue 'dispatch;
	}
	// 822E700C: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 822E7010: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7014: 48BB92FD  bl 0x82ea0310
	ctx.lr = 0x822E7018;
	sub_82EA0310(ctx, base);
	// 822E7018: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E701C: 48000008  b 0x822e7024
	pc = 0x822E7024; continue 'dispatch;
	// 822E7020: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E7024: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 822E7028: 4BFFFE51  bl 0x822e6e78
	ctx.lr = 0x822E702C;
	sub_822E6E78(ctx, base);
	// 822E702C: 3D60822C  lis r11, -0x7dd4
	ctx.r[11].s64 = -2111045632;
	// 822E7030: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E7034: 809F0004  lwz r4, 4(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E7038: 38AB0000  addi r5, r11, 0
	ctx.r[5].s64 = ctx.r[11].s64 + 0;
	// 822E703C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7040: 48BBBB19  bl 0x82ea2b58
	ctx.lr = 0x822E7044;
	sub_82EA2B58(ctx, base);
	// 822E7044: 83CD0000  lwz r30, 0(r13)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7048: 3BA00014  li r29, 0x14
	ctx.r[29].s64 = 20;
	// 822E704C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E7050: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E7054: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 822E7058: 48BB9909  bl 0x82ea0960
	ctx.lr = 0x822E705C;
	sub_82EA0960(ctx, base);
	// 822E705C: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 822E7060: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E7064: 7C7DF02E  lwzx r3, r29, r30
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[29].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 822E7068: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822E706C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7070: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E7074: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E7078: 4E800421  bctrl
	ctx.lr = 0x822E707C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E707C: 3FE08338  lis r31, -0x7cc8
	ctx.r[31].s64 = -2093481984;
	// 822E7080: 3C800007  lis r4, 7
	ctx.r[4].s64 = 458752;
	// 822E7084: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E7088: 6084D000  ori r4, r4, 0xd000
	ctx.r[4].u64 = ctx.r[4].u64 | 53248;
	// 822E708C: 807F6A30  lwz r3, 0x6a30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(27184 as u32) ) } as u64;
	// 822E7090: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7094: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E7098: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E709C: 4E800421  bctrl
	ctx.lr = 0x822E70A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E70A0: 807F6A30  lwz r3, 0x6a30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(27184 as u32) ) } as u64;
	// 822E70A4: 3C800003  lis r4, 3
	ctx.r[4].s64 = 196608;
	// 822E70A8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E70AC: 6084E800  ori r4, r4, 0xe800
	ctx.r[4].u64 = ctx.r[4].u64 | 59392;
	// 822E70B0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E70B4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E70B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E70BC: 4E800421  bctrl
	ctx.lr = 0x822E70C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E70C0: 807F6A30  lwz r3, 0x6a30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(27184 as u32) ) } as u64;
	// 822E70C4: 3C800001  lis r4, 1
	ctx.r[4].s64 = 65536;
	// 822E70C8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E70CC: 6084F400  ori r4, r4, 0xf400
	ctx.r[4].u64 = ctx.r[4].u64 | 62464;
	// 822E70D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E70D4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E70D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E70DC: 4E800421  bctrl
	ctx.lr = 0x822E70E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E70E0: 807F6A30  lwz r3, 0x6a30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(27184 as u32) ) } as u64;
	// 822E70E4: 3C800000  lis r4, 0
	ctx.r[4].s64 = 0;
	// 822E70E8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E70EC: 6084FA00  ori r4, r4, 0xfa00
	ctx.r[4].u64 = ctx.r[4].u64 | 64000;
	// 822E70F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E70F4: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E70F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E70FC: 4E800421  bctrl
	ctx.lr = 0x822E7100;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E7100: 807F6A30  lwz r3, 0x6a30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(27184 as u32) ) } as u64;
	// 822E7104: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E7108: 38807D00  li r4, 0x7d00
	ctx.r[4].s64 = 32000;
	// 822E710C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7110: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E7114: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E7118: 4E800421  bctrl
	ctx.lr = 0x822E711C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E711C: 807F6A30  lwz r3, 0x6a30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(27184 as u32) ) } as u64;
	// 822E7120: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E7124: 38803E80  li r4, 0x3e80
	ctx.r[4].s64 = 16000;
	// 822E7128: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E712C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E7130: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E7134: 4E800421  bctrl
	ctx.lr = 0x822E7138;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E7138: 807F6A30  lwz r3, 0x6a30(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(27184 as u32) ) } as u64;
	// 822E713C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E7140: 38803E80  li r4, 0x3e80
	ctx.r[4].s64 = 16000;
	// 822E7144: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7148: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E714C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E7150: 4E800421  bctrl
	ctx.lr = 0x822E7154;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E7154: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E7158: 48EC105C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7160 size=168
    let mut pc: u32 = 0x822E7160;
    'dispatch: loop {
        match pc {
            0x822E7160 => {
    //   block [0x822E7160..0x822E7208)
	// 822E7160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E7164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E7168: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E716C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E7170: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E7174: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 822E7178: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E717C: 806B6A30  lwz r3, 0x6a30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27184 as u32) ) } as u64;
	// 822E7180: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7184: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 822E7188: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E718C: 4E800421  bctrl
	ctx.lr = 0x822E7190;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E7190: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E7194: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822E7198: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E719C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E71A0: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E71A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E71A8: 4E800421  bctrl
	ctx.lr = 0x822E71AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E71AC: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E71B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822E71B4: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 822E71B8: 419A0018  beq cr6, 0x822e71d0
	if ctx.cr[6].eq {
	pc = 0x822E71D0; continue 'dispatch;
	}
	// 822E71BC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E71C0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822E71C4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822E71C8: 48BB97F1  bl 0x82ea09b8
	ctx.lr = 0x822E71CC;
	sub_82EA09B8(ctx, base);
	// 822E71CC: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 822E71D0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E71D4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E71D8: 419A000C  beq cr6, 0x822e71e4
	if ctx.cr[6].eq {
	pc = 0x822E71E4; continue 'dispatch;
	}
	// 822E71DC: 48BB8E4D  bl 0x82ea0028
	ctx.lr = 0x822E71E0;
	sub_82EA0028(ctx, base);
	// 822E71E0: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 822E71E4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 822E71E8: 4BFFFD41  bl 0x822e6f28
	ctx.lr = 0x822E71EC;
	sub_822E6F28(ctx, base);
	// 822E71EC: 48BBBAA5  bl 0x82ea2c90
	ctx.lr = 0x822E71F0;
	sub_82EA2C90(ctx, base);
	// 822E71F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E71F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E71F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E71FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E7200: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E7204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E7208 size=20
    let mut pc: u32 = 0x822E7208;
    'dispatch: loop {
        match pc {
            0x822E7208 => {
    //   block [0x822E7208..0x822E721C)
	// 822E7208: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E720C: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822E7210: 396BADAC  addi r11, r11, -0x5254
	ctx.r[11].s64 = ctx.r[11].s64 + -21076;
	// 822E7214: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E7218: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E7220 size=16
    let mut pc: u32 = 0x822E7220;
    'dispatch: loop {
        match pc {
            0x822E7220 => {
    //   block [0x822E7220..0x822E7230)
	// 822E7220: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E7224: 396BADAC  addi r11, r11, -0x5254
	ctx.r[11].s64 = ctx.r[11].s64 + -21076;
	// 822E7228: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E722C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7230 size=68
    let mut pc: u32 = 0x822E7230;
    'dispatch: loop {
        match pc {
            0x822E7230 => {
    //   block [0x822E7230..0x822E7274)
	// 822E7230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E7234: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E7238: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E723C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E7240: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E7244: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E7248: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E724C: 396BADAC  addi r11, r11, -0x5254
	ctx.r[11].s64 = ctx.r[11].s64 + -21076;
	// 822E7250: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E7254: 41820008  beq 0x822e725c
	if ctx.cr[0].eq {
	pc = 0x822E725C; continue 'dispatch;
	}
	// 822E7258: 4BFD9011  bl 0x822c0268
	ctx.lr = 0x822E725C;
	sub_822C0268(ctx, base);
	// 822E725C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E7260: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E7264: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E7268: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E726C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E7270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7278 size=112
    let mut pc: u32 = 0x822E7278;
    'dispatch: loop {
        match pc {
            0x822E7278 => {
    //   block [0x822E7278..0x822E72E8)
	// 822E7278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E727C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E7280: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E7284: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E7288: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E728C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E7290: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E7294: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E7298: 388B9B98  addi r4, r11, -0x6468
	ctx.r[4].s64 = ctx.r[11].s64 + -25704;
	// 822E729C: 38A00058  li r5, 0x58
	ctx.r[5].s64 = 88;
	// 822E72A0: 3860002C  li r3, 0x2c
	ctx.r[3].s64 = 44;
	// 822E72A4: 48B0B145  bl 0x82df23e8
	ctx.lr = 0x822E72A8;
	sub_82DF23E8(ctx, base);
	// 822E72A8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E72AC: 41820010  beq 0x822e72bc
	if ctx.cr[0].eq {
	pc = 0x822E72BC; continue 'dispatch;
	}
	// 822E72B0: 48007CE9  bl 0x822eef98
	ctx.lr = 0x822E72B4;
	sub_822EEF98(ctx, base);
	// 822E72B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E72B8: 48000008  b 0x822e72c0
	pc = 0x822E72C0; continue 'dispatch;
	// 822E72BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822E72C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E72C4: 889E0068  lbz r4, 0x68(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(104 as u32) ) } as u64;
	// 822E72C8: 48B175B1  bl 0x82dfe878
	ctx.lr = 0x822E72CC;
	sub_82DFE878(ctx, base);
	// 822E72CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E72D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E72D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E72D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E72DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E72E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E72E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E72E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E72E8 size=164
    let mut pc: u32 = 0x822E72E8;
    'dispatch: loop {
        match pc {
            0x822E72E8 => {
    //   block [0x822E72E8..0x822E738C)
	// 822E72E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E72EC: 48EC0E81  bl 0x831a816c
	ctx.lr = 0x822E72F0;
	sub_831A8130(ctx, base);
	// 822E72F0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E72F4: 3D60822E  lis r11, -0x7dd2
	ctx.r[11].s64 = -2110914560;
	// 822E72F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E72FC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822E7300: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822E7304: 396B7278  addi r11, r11, 0x7278
	ctx.r[11].s64 = ctx.r[11].s64 + 29304;
	// 822E7308: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 822E730C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 822E7310: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822E7314: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822E7318: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 822E731C: 48B175DD  bl 0x82dfe8f8
	ctx.lr = 0x822E7320;
	sub_82DFE8F8(ctx, base);
	// 822E7320: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 822E7324: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 822E7328: E8A10050  ld r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 822E732C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E7330: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 822E7334: 48B1A565  bl 0x82e01898
	ctx.lr = 0x822E7338;
	sub_82E01898(ctx, base);
	// 822E7338: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E733C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E7340: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E7344: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E7348: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E734C: 419A0024  beq cr6, 0x822e7370
	if ctx.cr[6].eq {
	pc = 0x822E7370; continue 'dispatch;
	}
	// 822E7350: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 822E7354: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 822E7358: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822E735C: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 822E7360: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822E7364: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 822E7368: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 822E736C: 4082FFE8  bne 0x822e7354
	if !ctx.cr[0].eq {
	pc = 0x822E7354; continue 'dispatch;
	}
	// 822E7370: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 822E7374: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E7378: 419A0008  beq cr6, 0x822e7380
	if ctx.cr[6].eq {
	pc = 0x822E7380; continue 'dispatch;
	}
	// 822E737C: 4BFD9515  bl 0x822c0890
	ctx.lr = 0x822E7380;
	sub_822C0890(ctx, base);
	// 822E7380: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E7384: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 822E7388: 48EC0E34  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7390 size=108
    let mut pc: u32 = 0x822E7390;
    'dispatch: loop {
        match pc {
            0x822E7390 => {
    //   block [0x822E7390..0x822E73FC)
	// 822E7390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E7394: 48EC0DD5  bl 0x831a8168
	ctx.lr = 0x822E7398;
	sub_831A8130(ctx, base);
	// 822E7398: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E739C: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E73A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E73A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E73A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E73AC: 388B4224  addi r4, r11, 0x4224
	ctx.r[4].s64 = ctx.r[11].s64 + 16932;
	// 822E73B0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E73B4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 822E73B8: 48B0C651  bl 0x82df3a08
	ctx.lr = 0x822E73BC;
	sub_82DF3A08(ctx, base);
	// 822E73BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822E73C0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822E73C4: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822E73C8: 48001D41  bl 0x822e9108
	ctx.lr = 0x822E73CC;
	sub_822E9108(ctx, base);
	// 822E73CC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 822E73D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E73D4: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E73D8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 822E73DC: 4BFFFF0D  bl 0x822e72e8
	ctx.lr = 0x822E73E0;
	sub_822E72E8(ctx, base);
	// 822E73E0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 822E73E4: 48B0C045  bl 0x82df3428
	ctx.lr = 0x822E73E8;
	sub_82DF3428(ctx, base);
	// 822E73E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E73EC: 48B0C03D  bl 0x82df3428
	ctx.lr = 0x822E73F0;
	sub_82DF3428(ctx, base);
	// 822E73F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E73F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E73F8: 48EC0DC0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7400 size=256
    let mut pc: u32 = 0x822E7400;
    'dispatch: loop {
        match pc {
            0x822E7400 => {
    //   block [0x822E7400..0x822E7500)
	// 822E7400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E7404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E7408: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E740C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E7410: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E7414: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E7418: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E741C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E7420: 388BADBC  addi r4, r11, -0x5244
	ctx.r[4].s64 = ctx.r[11].s64 + -21060;
	// 822E7424: 48B0C5E5  bl 0x82df3a08
	ctx.lr = 0x822E7428;
	sub_82DF3A08(ctx, base);
	// 822E7428: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822E742C: 3D60822F  lis r11, -0x7dd1
	ctx.r[11].s64 = -2110849024;
	// 822E7430: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 822E7434: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822E7438: 388BE448  addi r4, r11, -0x1bb8
	ctx.r[4].s64 = ctx.r[11].s64 + -7096;
	// 822E743C: 4BFEACE5  bl 0x822d2120
	ctx.lr = 0x822E7440;
	sub_822D2120(ctx, base);
	// 822E7440: 3D60822F  lis r11, -0x7dd1
	ctx.r[11].s64 = -2110849024;
	// 822E7444: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822E7448: 9BE1005C  stb r31, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u8 ) };
	// 822E744C: 396BF730  addi r11, r11, -0x8d0
	ctx.r[11].s64 = ctx.r[11].s64 + -2256;
	// 822E7450: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 822E7454: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 822E7458: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 822E745C: 4BFEAD6D  bl 0x822d21c8
	ctx.lr = 0x822E7460;
	sub_822D21C8(ctx, base);
	// 822E7460: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E7464: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 822E7468: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 822E746C: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 822E7470: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822E7474: 48B1BD95  bl 0x82e03208
	ctx.lr = 0x822E7478;
	sub_82E03208(ctx, base);
	// 822E7478: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E747C: 48B0BFAD  bl 0x82df3428
	ctx.lr = 0x822E7480;
	sub_82DF3428(ctx, base);
	// 822E7480: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E7484: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E7488: 388BADB0  addi r4, r11, -0x5250
	ctx.r[4].s64 = ctx.r[11].s64 + -21072;
	// 822E748C: 48B0C57D  bl 0x82df3a08
	ctx.lr = 0x822E7490;
	sub_82DF3A08(ctx, base);
	// 822E7490: 93E10080  stw r31, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u32 ) };
	// 822E7494: 3D60822F  lis r11, -0x7dd1
	ctx.r[11].s64 = -2110849024;
	// 822E7498: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822E749C: 388BE448  addi r4, r11, -0x1bb8
	ctx.r[4].s64 = ctx.r[11].s64 + -7096;
	// 822E74A0: 4BFEAC81  bl 0x822d2120
	ctx.lr = 0x822E74A4;
	sub_822D2120(ctx, base);
	// 822E74A4: 3D60822F  lis r11, -0x7dd1
	ctx.r[11].s64 = -2110849024;
	// 822E74A8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822E74AC: 93E10060  stw r31, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u32 ) };
	// 822E74B0: 396BF730  addi r11, r11, -0x8d0
	ctx.r[11].s64 = ctx.r[11].s64 + -2256;
	// 822E74B4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822E74B8: 9941005C  stb r10, 0x5c(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u8 ) };
	// 822E74BC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 822E74C0: E8810058  ld r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) };
	// 822E74C4: 4BFEAD05  bl 0x822d21c8
	ctx.lr = 0x822E74C8;
	sub_822D21C8(ctx, base);
	// 822E74C8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 822E74CC: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 822E74D0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822E74D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822E74D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E74DC: 48B1BD2D  bl 0x82e03208
	ctx.lr = 0x822E74E0;
	sub_82E03208(ctx, base);
	// 822E74E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E74E4: 48B0BF45  bl 0x82df3428
	ctx.lr = 0x822E74E8;
	sub_82DF3428(ctx, base);
	// 822E74E8: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 822E74EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E74F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E74F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E74F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E74FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7500 size=100
    let mut pc: u32 = 0x822E7500;
    'dispatch: loop {
        match pc {
            0x822E7500 => {
    //   block [0x822E7500..0x822E7564)
	// 822E7500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E7504: 48EC0C69  bl 0x831a816c
	ctx.lr = 0x822E7508;
	sub_831A8130(ctx, base);
	// 822E7508: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E750C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E7510: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822E7514: 3BDFFFFC  addi r30, r31, -4
	ctx.r[30].s64 = ctx.r[31].s64 + -4;
	// 822E7518: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E751C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E7520: 48BF16B1  bl 0x82ed8bd0
	ctx.lr = 0x822E7524;
	sub_82ED8BD0(ctx, base);
	// 822E7524: 357FFFFC  addic. r11, r31, -4
	ctx.xer.ca = (ctx.r[31].u32 > (!(-4 as u32)));
	ctx.r[11].s64 = ctx.r[31].s64 + -4;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E7528: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E752C: 40820008  bne 0x822e7534
	if !ctx.cr[0].eq {
	pc = 0x822E7534; continue 'dispatch;
	}
	// 822E7530: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E7534: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E7538: 48BF2071  bl 0x82ed95a8
	ctx.lr = 0x822E753C;
	sub_82ED95A8(ctx, base);
	// 822E753C: 281E0000  cmplwi r30, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E7540: 4182001C  beq 0x822e755c
	if ctx.cr[0].eq {
	pc = 0x822E755C; continue 'dispatch;
	}
	// 822E7544: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7548: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822E754C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E7550: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E7554: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E7558: 4E800421  bctrl
	ctx.lr = 0x822E755C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E755C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E7560: 48EC0C5C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7568(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7568 size=104
    let mut pc: u32 = 0x822E7568;
    'dispatch: loop {
        match pc {
            0x822E7568 => {
    //   block [0x822E7568..0x822E75D0)
	// 822E7568: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E756C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E7570: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E7574: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E7578: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E757C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E7580: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E7584: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E7588: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E758C: 419A002C  beq cr6, 0x822e75b8
	if ctx.cr[6].eq {
	pc = 0x822E75B8; continue 'dispatch;
	}
	// 822E7590: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7594: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E7598: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E759C: 4E800421  bctrl
	ctx.lr = 0x822E75A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E75A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E75A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E75A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E75AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E75B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E75B4: 4E800421  bctrl
	ctx.lr = 0x822E75B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E75B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E75BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E75C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E75C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E75C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E75CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E75D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E75D0 size=68
    let mut pc: u32 = 0x822E75D0;
    'dispatch: loop {
        match pc {
            0x822E75D0 => {
    //   block [0x822E75D0..0x822E7614)
	// 822E75D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E75D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E75D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E75DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E75E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E75E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E75E8: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E75EC: 396BBA8C  addi r11, r11, -0x4574
	ctx.r[11].s64 = ctx.r[11].s64 + -17780;
	// 822E75F0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E75F4: 41820008  beq 0x822e75fc
	if ctx.cr[0].eq {
	pc = 0x822E75FC; continue 'dispatch;
	}
	// 822E75F8: 4BFD8C71  bl 0x822c0268
	ctx.lr = 0x822E75FC;
	sub_822C0268(ctx, base);
	// 822E75FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E7600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E7604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E7608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E760C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E7610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7618 size=68
    let mut pc: u32 = 0x822E7618;
    'dispatch: loop {
        match pc {
            0x822E7618 => {
    //   block [0x822E7618..0x822E765C)
	// 822E7618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E761C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E7620: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E7624: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E7628: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E762C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E7630: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E7634: 396BBA64  addi r11, r11, -0x459c
	ctx.r[11].s64 = ctx.r[11].s64 + -17820;
	// 822E7638: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E763C: 41820008  beq 0x822e7644
	if ctx.cr[0].eq {
	pc = 0x822E7644; continue 'dispatch;
	}
	// 822E7640: 4BFD8C29  bl 0x822c0268
	ctx.lr = 0x822E7644;
	sub_822C0268(ctx, base);
	// 822E7644: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E7648: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E764C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E7650: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E7654: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E7658: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7660 size=140
    let mut pc: u32 = 0x822E7660;
    'dispatch: loop {
        match pc {
            0x822E7660 => {
    //   block [0x822E7660..0x822E76EC)
	// 822E7660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E7664: 48EC0B05  bl 0x831a8168
	ctx.lr = 0x822E7668;
	sub_831A8130(ctx, base);
	// 822E7668: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E766C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822E7670: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E7674: 897D0010  lbz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E7678: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 822E767C: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 822E7680: 83EB000C  lwz r31, 0xc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7684: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E7688: 419A005C  beq cr6, 0x822e76e4
	if ctx.cr[6].eq {
	pc = 0x822E76E4; continue 'dispatch;
	}
	// 822E768C: 807E0164  lwz r3, 0x164(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(356 as u32) ) } as u64;
	// 822E7690: 48229951  bl 0x82510fe0
	ctx.lr = 0x822E7694;
	sub_82510FE0(ctx, base);
	// 822E7694: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822E7698: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E769C: 48229945  bl 0x82510fe0
	ctx.lr = 0x822E76A0;
	sub_82510FE0(ctx, base);
	// 822E76A0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E76A4: 481A4475  bl 0x8248bb18
	ctx.lr = 0x822E76A8;
	sub_8248BB18(ctx, base);
	// 822E76A8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822E76AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E76B0: 48229931  bl 0x82510fe0
	ctx.lr = 0x822E76B4;
	sub_82510FE0(ctx, base);
	// 822E76B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E76B8: 807E0164  lwz r3, 0x164(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(356 as u32) ) } as u64;
	// 822E76BC: 48229925  bl 0x82510fe0
	ctx.lr = 0x822E76C0;
	sub_82510FE0(ctx, base);
	// 822E76C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E76C4: 481A4455  bl 0x8248bb18
	ctx.lr = 0x822E76C8;
	sub_8248BB18(ctx, base);
	// 822E76C8: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E76CC: 4082000C  bne 0x822e76d8
	if !ctx.cr[0].eq {
	pc = 0x822E76D8; continue 'dispatch;
	}
	// 822E76D0: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E76D4: 41820010  beq 0x822e76e4
	if ctx.cr[0].eq {
	pc = 0x822E76E4; continue 'dispatch;
	}
	// 822E76D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E76DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E76E0: 48BEEF09  bl 0x82ed65e8
	ctx.lr = 0x822E76E4;
	sub_82ED65E8(ctx, base);
	// 822E76E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E76E8: 48EC0AD0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E76F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E76F0 size=96
    let mut pc: u32 = 0x822E76F0;
    'dispatch: loop {
        match pc {
            0x822E76F0 => {
    //   block [0x822E76F0..0x822E7750)
	// 822E76F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E76F4: 48EC0A79  bl 0x831a816c
	ctx.lr = 0x822E76F8;
	sub_831A8130(ctx, base);
	// 822E76F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E76FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E7700: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E7704: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E7708: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822E770C: 396BBA8C  addi r11, r11, -0x4574
	ctx.r[11].s64 = ctx.r[11].s64 + -17780;
	// 822E7710: 394ABAC8  addi r10, r10, -0x4538
	ctx.r[10].s64 = ctx.r[10].s64 + -17720;
	// 822E7714: 3929BAB0  addi r9, r9, -0x4550
	ctx.r[9].s64 = ctx.r[9].s64 + -17744;
	// 822E7718: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E771C: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822E7720: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E7724: 913F0004  stw r9, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822E7728: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E772C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E7730: 3BDF0004  addi r30, r31, 4
	ctx.r[30].s64 = ctx.r[31].s64 + 4;
	// 822E7734: 48BF19DD  bl 0x82ed9110
	ctx.lr = 0x822E7738;
	sub_82ED9110(ctx, base);
	// 822E7738: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E773C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E7740: 48BF1D39  bl 0x82ed9478
	ctx.lr = 0x822E7744;
	sub_82ED9478(ctx, base);
	// 822E7744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E7748: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E774C: 48EC0A70  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E7750 size=8
    let mut pc: u32 = 0x822E7750;
    'dispatch: loop {
        match pc {
            0x822E7750 => {
    //   block [0x822E7750..0x822E7758)
	// 822E7750: 3863FFFC  addi r3, r3, -4
	ctx.r[3].s64 = ctx.r[3].s64 + -4;
	// 822E7754: 4800040C  b 0x822e7b60
	sub_822E7B60(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7758 size=224
    let mut pc: u32 = 0x822E7758;
    'dispatch: loop {
        match pc {
            0x822E7758 => {
    //   block [0x822E7758..0x822E7838)
	// 822E7758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E775C: 48EC0A0D  bl 0x831a8168
	ctx.lr = 0x822E7760;
	sub_831A8130(ctx, base);
	// 822E7760: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E7764: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822E7768: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E776C: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7770: 4800000C  b 0x822e777c
	pc = 0x822E777C; continue 'dispatch;
	// 822E7774: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 822E7778: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E777C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E7780: 409AFFF4  bne cr6, 0x822e7774
	if !ctx.cr[6].eq {
	pc = 0x822E7774; continue 'dispatch;
	}
	// 822E7784: 892A0010  lbz r9, 0x10(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E7788: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E778C: 7D290774  extsb r9, r9
	ctx.r[9].s64 = ctx.r[9].s8 as i64;
	// 822E7790: 7D295214  add r9, r9, r10
	ctx.r[9].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 822E7794: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7798: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E779C: 4800000C  b 0x822e77a8
	pc = 0x822E77A8; continue 'dispatch;
	// 822E77A0: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 822E77A4: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E77A8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822E77AC: 409AFFF4  bne cr6, 0x822e77a0
	if !ctx.cr[6].eq {
	pc = 0x822E77A0; continue 'dispatch;
	}
	// 822E77B0: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E77B4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822E77B8: 811D000C  lwz r8, 0xc(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E77BC: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 822E77C0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 822E77C4: 83C8000C  lwz r30, 0xc(r8)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E77C8: 83EB000C  lwz r31, 0xc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E77CC: 419A0064  beq cr6, 0x822e7830
	if ctx.cr[6].eq {
	pc = 0x822E7830; continue 'dispatch;
	}
	// 822E77D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E77D4: 419A005C  beq cr6, 0x822e7830
	if ctx.cr[6].eq {
	pc = 0x822E7830; continue 'dispatch;
	}
	// 822E77D8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822E77DC: 419A0054  beq cr6, 0x822e7830
	if ctx.cr[6].eq {
	pc = 0x822E7830; continue 'dispatch;
	}
	// 822E77E0: 7F09F040  cmplw cr6, r9, r30
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822E77E4: 419A0008  beq cr6, 0x822e77ec
	if ctx.cr[6].eq {
	pc = 0x822E77EC; continue 'dispatch;
	}
	// 822E77E8: 7D3F4B78  mr r31, r9
	ctx.r[31].u64 = ctx.r[9].u64;
	// 822E77EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E77F0: 482297F1  bl 0x82510fe0
	ctx.lr = 0x822E77F4;
	sub_82510FE0(ctx, base);
	// 822E77F4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822E77F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E77FC: 482297E5  bl 0x82510fe0
	ctx.lr = 0x822E7800;
	sub_82510FE0(ctx, base);
	// 822E7800: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E7804: 481A4315  bl 0x8248bb18
	ctx.lr = 0x822E7808;
	sub_8248BB18(ctx, base);
	// 822E7808: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E780C: 41820024  beq 0x822e7830
	if ctx.cr[0].eq {
	pc = 0x822E7830; continue 'dispatch;
	}
	// 822E7810: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E7814: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E7818: 481A9219  bl 0x82490a30
	ctx.lr = 0x822E781C;
	sub_82490A30(ctx, base);
	// 822E781C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E7820: 41820010  beq 0x822e7830
	if ctx.cr[0].eq {
	pc = 0x822E7830; continue 'dispatch;
	}
	// 822E7824: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7828: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 822E782C: B14B0096  sth r10, 0x96(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(150 as u32), ctx.r[10].u16 ) };
	// 822E7830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E7834: 48EC0984  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7838 size=128
    let mut pc: u32 = 0x822E7838;
    'dispatch: loop {
        match pc {
            0x822E7838 => {
    //   block [0x822E7838..0x822E78B8)
	// 822E7838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E783C: 48EC0931  bl 0x831a816c
	ctx.lr = 0x822E7840;
	sub_831A8130(ctx, base);
	// 822E7840: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E7844: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E7848: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E784C: 81240010  lwz r9, 0x10(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E7850: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7854: 83EA000C  lwz r31, 0xc(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7858: 83C9000C  lwz r30, 0xc(r9)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E785C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E7860: 419A0050  beq cr6, 0x822e78b0
	if ctx.cr[6].eq {
	pc = 0x822E78B0; continue 'dispatch;
	}
	// 822E7864: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E7868: 419A0048  beq cr6, 0x822e78b0
	if ctx.cr[6].eq {
	pc = 0x822E78B0; continue 'dispatch;
	}
	// 822E786C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822E7870: 419A0040  beq cr6, 0x822e78b0
	if ctx.cr[6].eq {
	pc = 0x822E78B0; continue 'dispatch;
	}
	// 822E7874: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822E7878: 419A0008  beq cr6, 0x822e7880
	if ctx.cr[6].eq {
	pc = 0x822E7880; continue 'dispatch;
	}
	// 822E787C: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 822E7880: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E7884: 4822975D  bl 0x82510fe0
	ctx.lr = 0x822E7888;
	sub_82510FE0(ctx, base);
	// 822E7888: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E788C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E7890: 48229751  bl 0x82510fe0
	ctx.lr = 0x822E7894;
	sub_82510FE0(ctx, base);
	// 822E7894: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E7898: 481A4281  bl 0x8248bb18
	ctx.lr = 0x822E789C;
	sub_8248BB18(ctx, base);
	// 822E789C: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E78A0: 41820010  beq 0x822e78b0
	if ctx.cr[0].eq {
	pc = 0x822E78B0; continue 'dispatch;
	}
	// 822E78A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E78A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E78AC: 481A8E25  bl 0x824906d0
	ctx.lr = 0x822E78B0;
	sub_824906D0(ctx, base);
	// 822E78B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E78B4: 48EC0908  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E78B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E78B8 size=140
    let mut pc: u32 = 0x822E78B8;
    'dispatch: loop {
        match pc {
            0x822E78B8 => {
    //   block [0x822E78B8..0x822E7944)
	// 822E78B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E78BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E78C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E78C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E78C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E78CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E78D0: 548A07FF  clrlwi. r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E78D4: 396BBA80  addi r11, r11, -0x4580
	ctx.r[11].s64 = ctx.r[11].s64 + -17792;
	// 822E78D8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E78DC: 41820050  beq 0x822e792c
	if ctx.cr[0].eq {
	pc = 0x822E792C; continue 'dispatch;
	}
	// 822E78E0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E78E4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822E78E8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822E78EC: 81630044  lwz r11, 0x44(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 822E78F0: 81430034  lwz r10, 0x34(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 822E78F4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 822E78F8: 41980018  blt cr6, 0x822e7910
	if ctx.cr[6].lt {
	pc = 0x822E7910; continue 'dispatch;
	}
	// 822E78FC: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 822E7900: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822E7904: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822E7908: 48BB8759  bl 0x82ea0060
	ctx.lr = 0x822E790C;
	sub_82EA0060(ctx, base);
	// 822E790C: 48000020  b 0x822e792c
	pc = 0x822E792C; continue 'dispatch;
	// 822E7910: 81430044  lwz r10, 0x44(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(68 as u32) ) } as u64;
	// 822E7914: 39630040  addi r11, r3, 0x40
	ctx.r[11].s64 = ctx.r[3].s64 + 64;
	// 822E7918: 81630040  lwz r11, 0x40(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) } as u64;
	// 822E791C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822E7920: 91430044  stw r10, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[10].u32 ) };
	// 822E7924: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E7928: 93E30040  stw r31, 0x40(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), ctx.r[31].u32 ) };
	// 822E792C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E7930: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E7934: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E7938: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E793C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E7940: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7948 size=232
    let mut pc: u32 = 0x822E7948;
    'dispatch: loop {
        match pc {
            0x822E7948 => {
    //   block [0x822E7948..0x822E7A30)
	// 822E7948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E794C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E7950: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E7954: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E7958: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E795C: 81240020  lwz r9, 0x20(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) } as u64;
	// 822E7960: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7964: 4800000C  b 0x822e7970
	pc = 0x822E7970; continue 'dispatch;
	// 822E7968: 7D695B78  mr r9, r11
	ctx.r[9].u64 = ctx.r[11].u64;
	// 822E796C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7970: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E7974: 409AFFF4  bne cr6, 0x822e7968
	if !ctx.cr[6].eq {
	pc = 0x822E7968; continue 'dispatch;
	}
	// 822E7978: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 822E797C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7980: 4800000C  b 0x822e798c
	pc = 0x822E798C; continue 'dispatch;
	// 822E7984: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 822E7988: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E798C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822E7990: 409AFFF4  bne cr6, 0x822e7984
	if !ctx.cr[6].eq {
	pc = 0x822E7984; continue 'dispatch;
	}
	// 822E7994: 83E30008  lwz r31, 8(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E7998: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E799C: 7D085A78  xor r8, r8, r11
	ctx.r[8].u64 = ctx.r[8].u64 ^ ctx.r[11].u64;
	// 822E79A0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E79A4: 7D1E4A78  xor r30, r8, r9
	ctx.r[30].u64 = ctx.r[8].u64 ^ ctx.r[9].u64;
	// 822E79A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E79AC: 40990024  ble cr6, 0x822e79d0
	if !ctx.cr[6].gt {
	pc = 0x822E79D0; continue 'dispatch;
	}
	// 822E79B0: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E79B4: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E79B8: 7F08F040  cmplw cr6, r8, r30
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[30].u32, &mut ctx.xer);
	// 822E79BC: 419A0018  beq cr6, 0x822e79d4
	if ctx.cr[6].eq {
	pc = 0x822E79D4; continue 'dispatch;
	}
	// 822E79C0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822E79C4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 822E79C8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822E79CC: 4198FFE8  blt cr6, 0x822e79b4
	if ctx.cr[6].lt {
	pc = 0x822E79B4; continue 'dispatch;
	}
	// 822E79D0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 822E79D4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E79D8: 40980040  bge cr6, 0x822e7a18
	if !ctx.cr[6].lt {
	pc = 0x822E7A18; continue 'dispatch;
	}
	// 822E79DC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E79E0: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E79E4: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 822E79E8: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822E79EC: 409A0010  bne cr6, 0x822e79fc
	if !ctx.cr[6].eq {
	pc = 0x822E79FC; continue 'dispatch;
	}
	// 822E79F0: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 822E79F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E79F8: 48BBEE89  bl 0x82ea6880
	ctx.lr = 0x822E79FC;
	sub_82EA6880(ctx, base);
	// 822E79FC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E7A00: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7A04: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 822E7A08: 7FCB512E  stwx r30, r11, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32), ctx.r[30].u32) };
	// 822E7A0C: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E7A10: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 822E7A14: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E7A18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E7A1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E7A20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E7A24: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E7A28: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E7A2C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7A30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7A30 size=300
    let mut pc: u32 = 0x822E7A30;
    'dispatch: loop {
        match pc {
            0x822E7A30 => {
    //   block [0x822E7A30..0x822E7B5C)
	// 822E7A30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E7A34: 48EC0729  bl 0x831a815c
	ctx.lr = 0x822E7A38;
	sub_831A8130(ctx, base);
	// 822E7A38: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E7A3C: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7A40: 3B200018  li r25, 0x18
	ctx.r[25].s64 = 24;
	// 822E7A44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E7A48: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E7A4C: 7D79D02E  lwzx r11, r25, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 822E7A50: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7A54: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E7A58: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E7A5C: 40980020  bge cr6, 0x822e7a7c
	if !ctx.cr[6].lt {
	pc = 0x822E7A7C; continue 'dispatch;
	}
	// 822E7A60: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822E7A64: 3929BAE0  addi r9, r9, -0x4520
	ctx.r[9].s64 = ctx.r[9].s64 + -17696;
	// 822E7A68: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E7A6C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 822E7A70: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 822E7A74: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822E7A78: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 822E7A7C: 89640010  lbz r11, 0x10(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E7A80: 807E0164  lwz r3, 0x164(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(356 as u32) ) } as u64;
	// 822E7A84: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 822E7A88: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 822E7A8C: 83EB000C  lwz r31, 0xc(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7A90: 48229551  bl 0x82510fe0
	ctx.lr = 0x822E7A94;
	sub_82510FE0(ctx, base);
	// 822E7A94: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822E7A98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E7A9C: 48229545  bl 0x82510fe0
	ctx.lr = 0x822E7AA0;
	sub_82510FE0(ctx, base);
	// 822E7AA0: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E7AA4: 481A4075  bl 0x8248bb18
	ctx.lr = 0x822E7AA8;
	sub_8248BB18(ctx, base);
	// 822E7AA8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822E7AAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E7AB0: 48229531  bl 0x82510fe0
	ctx.lr = 0x822E7AB4;
	sub_82510FE0(ctx, base);
	// 822E7AB4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 822E7AB8: 807E0164  lwz r3, 0x164(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(356 as u32) ) } as u64;
	// 822E7ABC: 48229525  bl 0x82510fe0
	ctx.lr = 0x822E7AC0;
	sub_82510FE0(ctx, base);
	// 822E7AC0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822E7AC4: 481A4055  bl 0x8248bb18
	ctx.lr = 0x822E7AC8;
	sub_8248BB18(ctx, base);
	// 822E7AC8: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 822E7ACC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822E7AD0: 578B063F  clrlwi. r11, r28, 0x18
	ctx.r[11].u64 = ctx.r[28].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E7AD4: 409A002C  bne cr6, 0x822e7b00
	if !ctx.cr[6].eq {
	pc = 0x822E7B00; continue 'dispatch;
	}
	// 822E7AD8: 41820010  beq 0x822e7ae8
	if ctx.cr[0].eq {
	pc = 0x822E7AE8; continue 'dispatch;
	}
	// 822E7ADC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E7AE0: 809E0164  lwz r4, 0x164(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(356 as u32) ) } as u64;
	// 822E7AE4: 481A911D  bl 0x82490c00
	ctx.lr = 0x822E7AE8;
	sub_82490C00(ctx, base);
	// 822E7AE8: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E7AEC: 41820038  beq 0x822e7b24
	if ctx.cr[0].eq {
	pc = 0x822E7B24; continue 'dispatch;
	}
	// 822E7AF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E7AF4: 807E0164  lwz r3, 0x164(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(356 as u32) ) } as u64;
	// 822E7AF8: 481A9109  bl 0x82490c00
	ctx.lr = 0x822E7AFC;
	sub_82490C00(ctx, base);
	// 822E7AFC: 48000028  b 0x822e7b24
	pc = 0x822E7B24; continue 'dispatch;
	// 822E7B00: 41820010  beq 0x822e7b10
	if ctx.cr[0].eq {
	pc = 0x822E7B10; continue 'dispatch;
	}
	// 822E7B04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E7B08: 809E0164  lwz r4, 0x164(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(356 as u32) ) } as u64;
	// 822E7B0C: 481A8ECD  bl 0x824909d8
	ctx.lr = 0x822E7B10;
	sub_824909D8(ctx, base);
	// 822E7B10: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E7B14: 41820010  beq 0x822e7b24
	if ctx.cr[0].eq {
	pc = 0x822E7B24; continue 'dispatch;
	}
	// 822E7B18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E7B1C: 807E0164  lwz r3, 0x164(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(356 as u32) ) } as u64;
	// 822E7B20: 481A8EB9  bl 0x824909d8
	ctx.lr = 0x822E7B24;
	sub_824909D8(ctx, base);
	// 822E7B24: 7D59D02E  lwzx r10, r25, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 822E7B28: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7B2C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E7B30: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E7B34: 40980020  bge cr6, 0x822e7b54
	if !ctx.cr[6].lt {
	pc = 0x822E7B54; continue 'dispatch;
	}
	// 822E7B38: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822E7B3C: 3929BADC  addi r9, r9, -0x4524
	ctx.r[9].s64 = ctx.r[9].s64 + -17700;
	// 822E7B40: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E7B44: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 822E7B48: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 822E7B4C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822E7B50: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 822E7B54: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 822E7B58: 48EC0654  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7B60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7B60 size=96
    let mut pc: u32 = 0x822E7B60;
    'dispatch: loop {
        match pc {
            0x822E7B60 => {
    //   block [0x822E7B60..0x822E7BC0)
	// 822E7B60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E7B64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E7B68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E7B6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E7B70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E7B74: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E7B78: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 822E7B7C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822E7B80: 396BBAB0  addi r11, r11, -0x4550
	ctx.r[11].s64 = ctx.r[11].s64 + -17744;
	// 822E7B84: 394ABA8C  addi r10, r10, -0x4574
	ctx.r[10].s64 = ctx.r[10].s64 + -17780;
	// 822E7B88: 3929BA64  addi r9, r9, -0x459c
	ctx.r[9].s64 = ctx.r[9].s64 + -17820;
	// 822E7B8C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E7B90: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822E7B94: 548807FF  clrlwi. r8, r4, 0x1f
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 822E7B98: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 822E7B9C: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E7BA0: 41820008  beq 0x822e7ba8
	if ctx.cr[0].eq {
	pc = 0x822E7BA8; continue 'dispatch;
	}
	// 822E7BA4: 4BFD86C5  bl 0x822c0268
	ctx.lr = 0x822E7BA8;
	sub_822C0268(ctx, base);
	// 822E7BA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E7BAC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E7BB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E7BB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E7BB8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E7BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7BC0 size=140
    let mut pc: u32 = 0x822E7BC0;
    'dispatch: loop {
        match pc {
            0x822E7BC0 => {
    //   block [0x822E7BC0..0x822E7C4C)
	// 822E7BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E7BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E7BC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E7BCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E7BD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E7BD4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E7BD8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 822E7BDC: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 822E7BE0: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 822E7BE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E7BE8: 48BEE739  bl 0x82ed6320
	ctx.lr = 0x822E7BEC;
	sub_82ED6320(ctx, base);
	// 822E7BEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E7BF0: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 822E7BF4: 93DF0164  stw r30, 0x164(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(356 as u32), ctx.r[30].u32 ) };
	// 822E7BF8: 392BBAF4  addi r9, r11, -0x450c
	ctx.r[9].s64 = ctx.r[11].s64 + -17676;
	// 822E7BFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E7C00: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 822E7C04: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E7C08: 917F0168  stw r11, 0x168(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(360 as u32), ctx.r[11].u32 ) };
	// 822E7C0C: 3D208335  lis r9, -0x7ccb
	ctx.r[9].s64 = -2093678592;
	// 822E7C10: 917F016C  stw r11, 0x16c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(364 as u32), ctx.r[11].u32 ) };
	// 822E7C14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E7C18: 911F0170  stw r8, 0x170(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(368 as u32), ctx.r[8].u32 ) };
	// 822E7C1C: 816A1754  lwz r11, 0x1754(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5972 as u32) ) } as u64;
	// 822E7C20: 917F0160  stw r11, 0x160(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), ctx.r[11].u32 ) };
	// 822E7C24: 81691758  lwz r11, 0x1758(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(5976 as u32) ) } as u64;
	// 822E7C28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 822E7C2C: 93EA1754  stw r31, 0x1754(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(5972 as u32), ctx.r[31].u32 ) };
	// 822E7C30: 91691758  stw r11, 0x1758(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(5976 as u32), ctx.r[11].u32 ) };
	// 822E7C34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E7C38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E7C3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E7C40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E7C44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E7C48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7C50 size=192
    let mut pc: u32 = 0x822E7C50;
    'dispatch: loop {
        match pc {
            0x822E7C50 => {
    //   block [0x822E7C50..0x822E7D10)
	// 822E7C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E7C54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E7C58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E7C5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E7C60: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E7C64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E7C68: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 822E7C6C: 396BBAF4  addi r11, r11, -0x450c
	ctx.r[11].s64 = ctx.r[11].s64 + -17676;
	// 822E7C70: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E7C74: 816A1754  lwz r11, 0x1754(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5972 as u32) ) } as u64;
	// 822E7C78: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 822E7C7C: 409A001C  bne cr6, 0x822e7c98
	if !ctx.cr[6].eq {
	pc = 0x822E7C98; continue 'dispatch;
	}
	// 822E7C80: 817F0160  lwz r11, 0x160(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) } as u64;
	// 822E7C84: 916A1754  stw r11, 0x1754(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(5972 as u32), ctx.r[11].u32 ) };
	// 822E7C88: 48000028  b 0x822e7cb0
	pc = 0x822E7CB0; continue 'dispatch;
	// 822E7C8C: 816A0160  lwz r11, 0x160(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(352 as u32) ) } as u64;
	// 822E7C90: 7F0BF840  cmplw cr6, r11, r31
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[31].u32, &mut ctx.xer);
	// 822E7C94: 419A0014  beq cr6, 0x822e7ca8
	if ctx.cr[6].eq {
	pc = 0x822E7CA8; continue 'dispatch;
	}
	// 822E7C98: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E7C9C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 822E7CA0: 4082FFEC  bne 0x822e7c8c
	if !ctx.cr[0].eq {
	pc = 0x822E7C8C; continue 'dispatch;
	}
	// 822E7CA4: 48000018  b 0x822e7cbc
	pc = 0x822E7CBC; continue 'dispatch;
	// 822E7CA8: 817F0160  lwz r11, 0x160(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) } as u64;
	// 822E7CAC: 916A0160  stw r11, 0x160(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(352 as u32), ctx.r[11].u32 ) };
	// 822E7CB0: 3D20DEAD  lis r9, -0x2153
	ctx.r[9].s64 = -559087616;
	// 822E7CB4: 6129BEEF  ori r9, r9, 0xbeef
	ctx.r[9].u64 = ctx.r[9].u64 | 48879;
	// 822E7CB8: 913F0160  stw r9, 0x160(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(352 as u32), ctx.r[9].u32 ) };
	// 822E7CBC: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 822E7CC0: 816A1758  lwz r11, 0x1758(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(5976 as u32) ) } as u64;
	// 822E7CC4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 822E7CC8: 916A1758  stw r11, 0x1758(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(5976 as u32), ctx.r[11].u32 ) };
	// 822E7CCC: 817F0170  lwz r11, 0x170(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(368 as u32) ) } as u64;
	// 822E7CD0: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E7CD4: 40820020  bne 0x822e7cf4
	if !ctx.cr[0].eq {
	pc = 0x822E7CF4; continue 'dispatch;
	}
	// 822E7CD8: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7CDC: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 822E7CE0: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 822E7CE4: 809F0168  lwz r4, 0x168(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(360 as u32) ) } as u64;
	// 822E7CE8: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 822E7CEC: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 822E7CF0: 48BB8AC1  bl 0x82ea07b0
	ctx.lr = 0x822E7CF4;
	sub_82EA07B0(ctx, base);
	// 822E7CF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E7CF8: 48BEE689  bl 0x82ed6380
	ctx.lr = 0x822E7CFC;
	sub_82ED6380(ctx, base);
	// 822E7CFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E7D00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E7D04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E7D08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E7D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7D10 size=160
    let mut pc: u32 = 0x822E7D10;
    'dispatch: loop {
        match pc {
            0x822E7D10 => {
    //   block [0x822E7D10..0x822E7DB0)
	// 822E7D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E7D14: 48EC0455  bl 0x831a8168
	ctx.lr = 0x822E7D18;
	sub_831A8130(ctx, base);
	// 822E7D18: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E7D1C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E7D20: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 822E7D24: 3BFD0168  addi r31, r29, 0x168
	ctx.r[31].s64 = ctx.r[29].s64 + 360;
	// 822E7D28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E7D2C: 813D016C  lwz r9, 0x16c(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(364 as u32) ) } as u64;
	// 822E7D30: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822E7D34: 40990024  ble cr6, 0x822e7d58
	if !ctx.cr[6].gt {
	pc = 0x822E7D58; continue 'dispatch;
	}
	// 822E7D38: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7D3C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7D40: 7F08E040  cmplw cr6, r8, r28
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[28].u32, &mut ctx.xer);
	// 822E7D44: 419A0064  beq cr6, 0x822e7da8
	if ctx.cr[6].eq {
	pc = 0x822E7DA8; continue 'dispatch;
	}
	// 822E7D48: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 822E7D4C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 822E7D50: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 822E7D54: 4198FFE8  blt cr6, 0x822e7d3c
	if ctx.cr[6].lt {
	pc = 0x822E7D3C; continue 'dispatch;
	}
	// 822E7D58: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 822E7D5C: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 822E7D60: 40990034  ble cr6, 0x822e7d94
	if !ctx.cr[6].gt {
	pc = 0x822E7D94; continue 'dispatch;
	}
	// 822E7D64: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E7D68: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E7D6C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E7D70: 4BFFFCC1  bl 0x822e7a30
	ctx.lr = 0x822E7D74;
	sub_822E7A30(ctx, base);
	// 822E7D74: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E7D78: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 822E7D7C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 822E7D80: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E7D84: 5569103A  slwi r9, r11, 2
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 822E7D88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7D8C: 7D29582E  lwzx r9, r9, r11
	ctx.r[9].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822E7D90: 7D2A592E  stwx r9, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[9].u32) };
	// 822E7D94: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E7D98: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E7D9C: 48BEE3CD  bl 0x82ed6168
	ctx.lr = 0x822E7DA0;
	sub_82ED6168(ctx, base);
	// 822E7DA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E7DA4: 48EC0414  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 822E7DA8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 822E7DAC: 4BFFFFB0  b 0x822e7d5c
	pc = 0x822E7D5C; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E7DB0 size=96
    let mut pc: u32 = 0x822E7DB0;
    'dispatch: loop {
        match pc {
            0x822E7DB0 => {
    //   block [0x822E7DB0..0x822E7E10)
	// 822E7DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E7DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E7DB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E7DBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E7DC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E7DC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E7DC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E7DCC: 4BFFFE85  bl 0x822e7c50
	ctx.lr = 0x822E7DD0;
	sub_822E7C50(ctx, base);
	// 822E7DD0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E7DD4: 41820020  beq 0x822e7df4
	if ctx.cr[0].eq {
	pc = 0x822E7DF4; continue 'dispatch;
	}
	// 822E7DD8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7DDC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 822E7DE0: 38C00031  li r6, 0x31
	ctx.r[6].s64 = 49;
	// 822E7DE4: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E7DE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E7DEC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822E7DF0: 48BB89C1  bl 0x82ea07b0
	ctx.lr = 0x822E7DF4;
	sub_82EA07B0(ctx, base);
	// 822E7DF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E7DF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E7DFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E7E00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E7E04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E7E08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E7E0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E7E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E7E10 size=1004
    let mut pc: u32 = 0x822E7E10;
    'dispatch: loop {
        match pc {
            0x822E7E10 => {
    //   block [0x822E7E10..0x822E81FC)
	// 822E7E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E7E14: 48EC0341  bl 0x831a8154
	ctx.lr = 0x822E7E18;
	sub_831A8130(ctx, base);
	// 822E7E18: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E7E1C: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7E20: 3AE00018  li r23, 0x18
	ctx.r[23].s64 = 24;
	// 822E7E24: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E7E28: 7D5BB82E  lwzx r10, r27, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 822E7E2C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7E30: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E7E34: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E7E38: 40980020  bge cr6, 0x822e7e58
	if !ctx.cr[6].lt {
	pc = 0x822E7E58; continue 'dispatch;
	}
	// 822E7E3C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822E7E40: 3929BB44  addi r9, r9, -0x44bc
	ctx.r[9].s64 = ctx.r[9].s64 + -17596;
	// 822E7E44: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E7E48: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 822E7E4C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 822E7E50: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822E7E54: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 822E7E58: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E7E5C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E7E60: 409A0028  bne cr6, 0x822e7e88
	if !ctx.cr[6].eq {
	pc = 0x822E7E88; continue 'dispatch;
	}
	// 822E7E64: 7D5BB82E  lwzx r10, r27, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 822E7E68: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7E6C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E7E70: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E7E74: 40980380  bge cr6, 0x822e81f4
	if !ctx.cr[6].lt {
	pc = 0x822E81F4; continue 'dispatch;
	}
	// 822E7E78: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822E7E7C: 3929BADC  addi r9, r9, -0x4524
	ctx.r[9].s64 = ctx.r[9].s64 + -17700;
	// 822E7E80: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E7E84: 48000360  b 0x822e81e4
	pc = 0x822E81E4; continue 'dispatch;
	// 822E7E88: 807D0164  lwz r3, 0x164(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(356 as u32) ) } as u64;
	// 822E7E8C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7E90: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E7E94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E7E98: 4E800421  bctrl
	ctx.lr = 0x822E7E9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E7E9C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E7EA0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 822E7EA4: 389D00A0  addi r4, r29, 0xa0
	ctx.r[4].s64 = ctx.r[29].s64 + 160;
	// 822E7EA8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E7EAC: C02B9F7C  lfs f1, -0x6084(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24708 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822E7EB0: 48BB8FD1  bl 0x82ea0e80
	ctx.lr = 0x822E7EB4;
	sub_82EA0E80(ctx, base);
	// 822E7EB4: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7EB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E7EBC: 409A0024  bne cr6, 0x822e7ee0
	if !ctx.cr[6].eq {
	pc = 0x822E7EE0; continue 'dispatch;
	}
	// 822E7EC0: 807D0164  lwz r3, 0x164(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(356 as u32) ) } as u64;
	// 822E7EC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7EC8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 822E7ECC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E7ED0: 4E800421  bctrl
	ctx.lr = 0x822E7ED4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E7ED4: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E7ED8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E7EDC: 48BECE4D  bl 0x82ed4d28
	ctx.lr = 0x822E7EE0;
	sub_82ED4D28(ctx, base);
	// 822E7EE0: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 822E7EE4: 83FD0154  lwz r31, 0x154(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(340 as u32) ) } as u64;
	// 822E7EE8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 822E7EEC: 3FC08000  lis r30, -0x8000
	ctx.r[30].s64 = -2147483648;
	// 822E7EF0: 93410060  stw r26, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[26].u32 ) };
	// 822E7EF4: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 822E7EF8: 93410064  stw r26, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 822E7EFC: 7C7BC82E  lwzx r3, r27, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 822E7F00: 55641036  rlwinm r4, r11, 2, 0, 0x1b
	ctx.r[4].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 822E7F04: 93C10068  stw r30, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 822E7F08: 8123002C  lwz r9, 0x2c(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 822E7F0C: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 822E7F10: 7D4B2214  add r10, r11, r4
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 822E7F14: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E7F18: 4199000C  bgt cr6, 0x822e7f24
	if ctx.cr[6].gt {
	pc = 0x822E7F24; continue 'dispatch;
	}
	// 822E7F1C: 91430020  stw r10, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 822E7F20: 48000018  b 0x822e7f38
	pc = 0x822E7F38; continue 'dispatch;
	// 822E7F24: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7F28: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 822E7F2C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E7F30: 4E800421  bctrl
	ctx.lr = 0x822E7F34;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E7F34: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E7F38: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 822E7F3C: 7FE9F378  or r9, r31, r30
	ctx.r[9].u64 = ctx.r[31].u64 | ctx.r[30].u64;
	// 822E7F40: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 822E7F44: 7D5BB82E  lwzx r10, r27, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 822E7F48: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 822E7F4C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7F50: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E7F54: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E7F58: 40980020  bge cr6, 0x822e7f78
	if !ctx.cr[6].lt {
	pc = 0x822E7F78; continue 'dispatch;
	}
	// 822E7F5C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822E7F60: 3929BB34  addi r9, r9, -0x44cc
	ctx.r[9].s64 = ctx.r[9].s64 + -17612;
	// 822E7F64: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E7F68: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 822E7F6C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 822E7F70: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822E7F74: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 822E7F78: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E7F7C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E7F80: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822E7F84: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 822E7F88: 3929BAA4  addi r9, r9, -0x455c
	ctx.r[9].s64 = ctx.r[9].s64 + -17756;
	// 822E7F8C: 38FD0010  addi r7, r29, 0x10
	ctx.r[7].s64 = ctx.r[29].s64 + 16;
	// 822E7F90: 91010078  stw r8, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[8].u32 ) };
	// 822E7F94: C00BBA78  lfs f0, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E7F98: 816A0038  lwz r11, 0x38(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(56 as u32) ) } as u64;
	// 822E7F9C: D0010074  stfs f0, 0x74(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 822E7FA0: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 822E7FA4: 90E1007C  stw r7, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[7].u32 ) };
	// 822E7FA8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822E7FAC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 822E7FB0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E7FB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E7FB8: 4E800421  bctrl
	ctx.lr = 0x822E7FBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E7FBC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 822E7FC0: 7D5BB82E  lwzx r10, r27, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 822E7FC4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E7FC8: 3929BA80  addi r9, r9, -0x4580
	ctx.r[9].s64 = ctx.r[9].s64 + -17792;
	// 822E7FCC: 3B0BBADC  addi r24, r11, -0x4524
	ctx.r[24].s64 = ctx.r[11].s64 + -17700;
	// 822E7FD0: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 822E7FD4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E7FD8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E7FDC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E7FE0: 40980018  bge cr6, 0x822e7ff8
	if !ctx.cr[6].lt {
	pc = 0x822E7FF8; continue 'dispatch;
	}
	// 822E7FE4: 930B0000  stw r24, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 822E7FE8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 822E7FEC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 822E7FF0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822E7FF4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 822E7FF8: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 822E7FFC: 7F5CD378  mr r28, r26
	ctx.r[28].u64 = ctx.r[26].u64;
	// 822E8000: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E8004: 40990078  ble cr6, 0x822e807c
	if !ctx.cr[6].gt {
	pc = 0x822E807C; continue 'dispatch;
	}
	// 822E8008: 3BDD0168  addi r30, r29, 0x168
	ctx.r[30].s64 = ctx.r[29].s64 + 360;
	// 822E800C: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 822E8010: 80FE0004  lwz r7, 4(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8014: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 822E8018: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 822E801C: 4099002C  ble cr6, 0x822e8048
	if !ctx.cr[6].gt {
	pc = 0x822E8048; continue 'dispatch;
	}
	// 822E8020: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 822E8024: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8028: 7D1F402E  lwzx r8, r31, r8
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 822E802C: 80CA0000  lwz r6, 0(r10)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8030: 7F064040  cmplw cr6, r6, r8
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[8].u32, &mut ctx.xer);
	// 822E8034: 419A0018  beq cr6, 0x822e804c
	if ctx.cr[6].eq {
	pc = 0x822E804C; continue 'dispatch;
	}
	// 822E8038: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 822E803C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 822E8040: 7F093800  cmpw cr6, r9, r7
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[7].s32, &mut ctx.xer);
	// 822E8044: 4198FFE8  blt cr6, 0x822e802c
	if ctx.cr[6].lt {
	pc = 0x822E802C; continue 'dispatch;
	}
	// 822E8048: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 822E804C: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 822E8050: 409A001C  bne cr6, 0x822e806c
	if !ctx.cr[6].eq {
	pc = 0x822E806C; continue 'dispatch;
	}
	// 822E8054: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 822E8058: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822E805C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E8060: 7C9F582E  lwzx r4, r31, r11
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 822E8064: 4BFFF9CD  bl 0x822e7a30
	ctx.lr = 0x822E8068;
	sub_822E7A30(ctx, base);
	// 822E8068: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 822E806C: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 822E8070: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 822E8074: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822E8078: 4198FF98  blt cr6, 0x822e8010
	if ctx.cr[6].lt {
	pc = 0x822E8010; continue 'dispatch;
	}
	// 822E807C: 815D016C  lwz r10, 0x16c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(364 as u32) ) } as u64;
	// 822E8080: 7F5ED378  mr r30, r26
	ctx.r[30].u64 = ctx.r[26].u64;
	// 822E8084: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E8088: 40990074  ble cr6, 0x822e80fc
	if !ctx.cr[6].gt {
	pc = 0x822E80FC; continue 'dispatch;
	}
	// 822E808C: 7F5FD378  mr r31, r26
	ctx.r[31].u64 = ctx.r[26].u64;
	// 822E8090: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 822E8094: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E8098: 4099002C  ble cr6, 0x822e80c4
	if !ctx.cr[6].gt {
	pc = 0x822E80C4; continue 'dispatch;
	}
	// 822E809C: 811D0168  lwz r8, 0x168(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(360 as u32) ) } as u64;
	// 822E80A0: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 822E80A4: 7D08F82E  lwzx r8, r8, r31
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 822E80A8: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E80AC: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 822E80B0: 419A0018  beq cr6, 0x822e80c8
	if ctx.cr[6].eq {
	pc = 0x822E80C8; continue 'dispatch;
	}
	// 822E80B4: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
	// 822E80B8: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 822E80BC: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822E80C0: 4198FFE8  blt cr6, 0x822e80a8
	if ctx.cr[6].lt {
	pc = 0x822E80A8; continue 'dispatch;
	}
	// 822E80C4: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 822E80C8: 2F09FFFF  cmpwi cr6, r9, -1
	ctx.cr[6].compare_i32(ctx.r[9].s32, -1, &mut ctx.xer);
	// 822E80CC: 409A001C  bne cr6, 0x822e80e8
	if !ctx.cr[6].eq {
	pc = 0x822E80E8; continue 'dispatch;
	}
	// 822E80D0: 817D0168  lwz r11, 0x168(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(360 as u32) ) } as u64;
	// 822E80D4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E80D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E80DC: 7C8BF82E  lwzx r4, r11, r31
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 822E80E0: 4BFFF951  bl 0x822e7a30
	ctx.lr = 0x822E80E4;
	sub_822E7A30(ctx, base);
	// 822E80E4: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 822E80E8: 815D016C  lwz r10, 0x16c(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(364 as u32) ) } as u64;
	// 822E80EC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 822E80F0: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 822E80F4: 7F1E5000  cmpw cr6, r30, r10
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[10].s32, &mut ctx.xer);
	// 822E80F8: 4198FF98  blt cr6, 0x822e8090
	if ctx.cr[6].lt {
	pc = 0x822E8090; continue 'dispatch;
	}
	// 822E80FC: 813D0170  lwz r9, 0x170(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(368 as u32) ) } as u64;
	// 822E8100: 3BFD0168  addi r31, r29, 0x168
	ctx.r[31].s64 = ctx.r[29].s64 + 360;
	// 822E8104: 552A00BE  clrlwi r10, r9, 2
	ctx.r[10].u64 = ctx.r[9].u32 as u64 & 0x3FFFFFFFu64;
	// 822E8108: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 822E810C: 4098004C  bge cr6, 0x822e8158
	if !ctx.cr[6].lt {
	pc = 0x822E8158; continue 'dispatch;
	}
	// 822E8110: 55290001  rlwinm. r9, r9, 0, 0, 0
	ctx.r[9].u64 = ctx.r[9].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 822E8114: 4082001C  bne 0x822e8130
	if !ctx.cr[0].eq {
	pc = 0x822E8130; continue 'dispatch;
	}
	// 822E8118: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 822E811C: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8120: 5545103A  slwi r5, r10, 2
	ctx.r[5].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 822E8124: 7C7BC82E  lwzx r3, r27, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 822E8128: 48BB8689  bl 0x82ea07b0
	ctx.lr = 0x822E812C;
	sub_82EA07B0(ctx, base);
	// 822E812C: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 822E8130: 38A00017  li r5, 0x17
	ctx.r[5].s64 = 23;
	// 822E8134: 7C7BC82E  lwzx r3, r27, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 822E8138: 5564103A  slwi r4, r11, 2
	ctx.r[4].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 822E813C: 48BB85F5  bl 0x82ea0730
	ctx.lr = 0x822E8140;
	sub_82EA0730(ctx, base);
	// 822E8140: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E8144: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 822E8148: 554A0042  rlwinm r10, r10, 0, 1, 1
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 822E814C: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 822E8150: 7D4A5B78  or r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 | ctx.r[11].u64;
	// 822E8154: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822E8158: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E815C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E8160: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E8164: 40990020  ble cr6, 0x822e8184
	if !ctx.cr[6].gt {
	pc = 0x822E8184; continue 'dispatch;
	}
	// 822E8168: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 822E816C: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8170: 356BFFFF  addic. r11, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E8174: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 822E8178: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 822E817C: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 822E8180: 4082FFEC  bne 0x822e816c
	if !ctx.cr[0].eq {
	pc = 0x822E816C; continue 'dispatch;
	}
	// 822E8184: 7C7BC82E  lwzx r3, r27, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 822E8188: 8081006C  lwz r4, 0x6c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 822E818C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 822E8190: 90830020  stw r4, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 822E8194: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E8198: 409A0014  bne cr6, 0x822e81ac
	if !ctx.cr[6].eq {
	pc = 0x822E81AC; continue 'dispatch;
	}
	// 822E819C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E81A0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 822E81A4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E81A8: 4E800421  bctrl
	ctx.lr = 0x822E81AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E81AC: 81610068  lwz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 822E81B0: 556A0001  rlwinm. r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 822E81B4: 40820018  bne 0x822e81cc
	if !ctx.cr[0].eq {
	pc = 0x822E81CC; continue 'dispatch;
	}
	// 822E81B8: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 822E81BC: 7C7BC82E  lwzx r3, r27, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 822E81C0: 5565103A  slwi r5, r11, 2
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 822E81C4: 80810060  lwz r4, 0x60(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 822E81C8: 48BB85E9  bl 0x82ea07b0
	ctx.lr = 0x822E81CC;
	sub_82EA07B0(ctx, base);
	// 822E81CC: 7D5BB82E  lwzx r10, r27, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 822E81D0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E81D4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E81D8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E81DC: 40980018  bge cr6, 0x822e81f4
	if !ctx.cr[6].lt {
	pc = 0x822E81F4; continue 'dispatch;
	}
	// 822E81E0: 930B0000  stw r24, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 822E81E4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 822E81E8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 822E81EC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 822E81F0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 822E81F4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 822E81F8: 48EBFFAC  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E8200 size=72
    let mut pc: u32 = 0x822E8200;
    'dispatch: loop {
        match pc {
            0x822E8200 => {
    //   block [0x822E8200..0x822E8248)
	// 822E8200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E8204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E8208: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E820C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E8210: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822E8214: 83EB1754  lwz r31, 0x1754(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5972 as u32) ) } as u64;
	// 822E8218: 281F0000  cmplwi r31, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E821C: 41820018  beq 0x822e8234
	if ctx.cr[0].eq {
	pc = 0x822E8234; continue 'dispatch;
	}
	// 822E8220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E8224: 4BFFFBED  bl 0x822e7e10
	ctx.lr = 0x822E8228;
	sub_822E7E10(ctx, base);
	// 822E8228: 83FF0160  lwz r31, 0x160(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) } as u64;
	// 822E822C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E8230: 409AFFF0  bne cr6, 0x822e8220
	if !ctx.cr[6].eq {
	pc = 0x822E8220; continue 'dispatch;
	}
	// 822E8234: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E8238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E823C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E8240: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E8244: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E8248 size=12
    let mut pc: u32 = 0x822E8248;
    'dispatch: loop {
        match pc {
            0x822E8248 => {
    //   block [0x822E8248..0x822E8254)
	// 822E8248: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E824C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E8250: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8254(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E8254 size=16
    let mut pc: u32 = 0x822E8254;
    'dispatch: loop {
        match pc {
            0x822E8254 => {
    //   block [0x822E8254..0x822E8264)
	// 822E8254: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 822E8258: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 822E825C: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 822E8260: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8268(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E8268 size=12
    let mut pc: u32 = 0x822E8268;
    'dispatch: loop {
        match pc {
            0x822E8268 => {
    //   block [0x822E8268..0x822E8274)
	// 822E8268: A1630004  lhz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E826C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E8270: 4D820020  beqlr
	if ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8274(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E8274 size=20
    let mut pc: u32 = 0x822E8274;
    'dispatch: loop {
        match pc {
            0x822E8274 => {
    //   block [0x822E8274..0x822E8288)
	// 822E8274: A1630006  lhz r11, 6(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(6 as u32) ) } as u64;
	// 822E8278: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 822E827C: 7D6B0735  extsh. r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E8280: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 822E8284: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E8288 size=20
    let mut pc: u32 = 0x822E8288;
    'dispatch: loop {
        match pc {
            0x822E8288 => {
    //   block [0x822E8288..0x822E829C)
	// 822E8288: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E828C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822E8290: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8294: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E8298: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E829C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E829C size=4
    let mut pc: u32 = 0x822E829C;
    'dispatch: loop {
        match pc {
            0x822E829C => {
    //   block [0x822E829C..0x822E82A0)
	// 822E829C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E82A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E82A0 size=100
    let mut pc: u32 = 0x822E82A0;
    'dispatch: loop {
        match pc {
            0x822E82A0 => {
    //   block [0x822E82A0..0x822E8304)
	// 822E82A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E82A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E82A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E82AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E82B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E82B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E82B8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E82BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E82C0: 396BBB5C  addi r11, r11, -0x44a4
	ctx.r[11].s64 = ctx.r[11].s64 + -17572;
	// 822E82C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E82C8: 48BB8351  bl 0x82ea0618
	ctx.lr = 0x822E82CC;
	sub_82EA0618(ctx, base);
	// 822E82CC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E82D0: 41820018  beq 0x822e82e8
	if ctx.cr[0].eq {
	pc = 0x822E82E8; continue 'dispatch;
	}
	// 822E82D4: 3D608332  lis r11, -0x7cce
	ctx.r[11].s64 = -2093875200;
	// 822E82D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E82DC: 816BBDC0  lwz r11, -0x4240(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16960 as u32) ) } as u64;
	// 822E82E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E82E4: 4E800421  bctrl
	ctx.lr = 0x822E82E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E82E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E82EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E82F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E82F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E82F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E82FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E8300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E8308 size=20
    let mut pc: u32 = 0x822E8308;
    'dispatch: loop {
        match pc {
            0x822E8308 => {
    //   block [0x822E8308..0x822E831C)
	// 822E8308: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E830C: 90830004  stw r4, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822E8310: 396BBB7C  addi r11, r11, -0x4484
	ctx.r[11].s64 = ctx.r[11].s64 + -17540;
	// 822E8314: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E8318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E8320 size=192
    let mut pc: u32 = 0x822E8320;
    'dispatch: loop {
        match pc {
            0x822E8320 => {
    //   block [0x822E8320..0x822E83E0)
	// 822E8320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E8324: 48EBFE41  bl 0x831a8164
	ctx.lr = 0x822E8328;
	sub_831A8130(ctx, base);
	// 822E8328: 9421FC50  stwu r1, -0x3b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-944 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E832C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E8330: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 822E8334: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 822E8338: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E833C: 839D0014  lwz r28, 0x14(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 822E8340: 808B6A30  lwz r4, 0x6a30(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(27184 as u32) ) } as u64;
	// 822E8344: 48BB7FCD  bl 0x82ea0310
	ctx.lr = 0x822E8348;
	sub_82EA0310(ctx, base);
	// 822E8348: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E834C: 48BBA495  bl 0x82ea27e0
	ctx.lr = 0x822E8350;
	sub_82EA27E0(ctx, base);
	// 822E8350: 83ED0000  lwz r31, 0(r13)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8354: 3BC00014  li r30, 0x14
	ctx.r[30].s64 = 20;
	// 822E8358: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E835C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E8360: 7C7EF82E  lwzx r3, r30, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 822E8364: 48BB85FD  bl 0x82ea0960
	ctx.lr = 0x822E8368;
	sub_82EA0960(ctx, base);
	// 822E8368: 7D7EF82E  lwzx r11, r30, r31
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 822E836C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 822E8370: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 822E8374: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 822E8378: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822E837C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8380: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E8384: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E8388: 4E800421  bctrl
	ctx.lr = 0x822E838C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E838C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E8390: 48B11529  bl 0x82df98b8
	ctx.lr = 0x822E8394;
	sub_82DF98B8(ctx, base);
	// 822E8394: 7C7EF82E  lwzx r3, r30, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 822E8398: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822E839C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E83A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E83A4: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E83A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E83AC: 4E800421  bctrl
	ctx.lr = 0x822E83B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E83B0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822E83B4: 7C7EF82E  lwzx r3, r30, r31
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[30].u32.wrapping_add(ctx.r[31].u32)) } as u64;
	// 822E83B8: 48BB8601  bl 0x82ea09b8
	ctx.lr = 0x822E83BC;
	sub_82EA09B8(ctx, base);
	// 822E83BC: 48BBA44D  bl 0x82ea2808
	ctx.lr = 0x822E83C0;
	sub_82EA2808(ctx, base);
	// 822E83C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E83C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E83C8: 396BBB5C  addi r11, r11, -0x44a4
	ctx.r[11].s64 = ctx.r[11].s64 + -17572;
	// 822E83CC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 822E83D0: 48BB8249  bl 0x82ea0618
	ctx.lr = 0x822E83D4;
	sub_82EA0618(ctx, base);
	// 822E83D4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 822E83D8: 382103B0  addi r1, r1, 0x3b0
	ctx.r[1].s64 = ctx.r[1].s64 + 944;
	// 822E83DC: 48EBFDD8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E83E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E83E0 size=76
    let mut pc: u32 = 0x822E83E0;
    'dispatch: loop {
        match pc {
            0x822E83E0 => {
    //   block [0x822E83E0..0x822E842C)
	// 822E83E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E83E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E83E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E83EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E83F0: 3D60822F  lis r11, -0x7dd1
	ctx.r[11].s64 = -2110849024;
	// 822E83F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E83F8: 38CB8320  addi r6, r11, -0x7ce0
	ctx.r[6].s64 = ctx.r[11].s64 + -31968;
	// 822E83FC: 48B0A4ED  bl 0x82df28e8
	ctx.lr = 0x822E8400;
	sub_82DF28E8(ctx, base);
	// 822E8400: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E8404: 3D400020  lis r10, 0x20
	ctx.r[10].s64 = 2097152;
	// 822E8408: 396BBB84  addi r11, r11, -0x447c
	ctx.r[11].s64 = ctx.r[11].s64 + -17532;
	// 822E840C: 915F0014  stw r10, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[10].u32 ) };
	// 822E8410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E8414: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E8418: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E841C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E8420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E8424: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E8428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E8430 size=76
    let mut pc: u32 = 0x822E8430;
    'dispatch: loop {
        match pc {
            0x822E8430 => {
    //   block [0x822E8430..0x822E847C)
	// 822E8430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E8434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E8438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E843C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E8440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E8444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E8448: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E844C: 48B0A3A5  bl 0x82df27f0
	ctx.lr = 0x822E8450;
	sub_82DF27F0(ctx, base);
	// 822E8450: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E8454: 4182000C  beq 0x822e8460
	if ctx.cr[0].eq {
	pc = 0x822E8460; continue 'dispatch;
	}
	// 822E8458: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E845C: 48B09F7D  bl 0x82df23d8
	ctx.lr = 0x822E8460;
	sub_82DF23D8(ctx, base);
	// 822E8460: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E8464: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E8468: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E846C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E8470: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E8474: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E8478: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E8480 size=92
    let mut pc: u32 = 0x822E8480;
    'dispatch: loop {
        match pc {
            0x822E8480 => {
    //   block [0x822E8480..0x822E84DC)
	// 822E8480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E8484: 48EBFCE9  bl 0x831a816c
	ctx.lr = 0x822E8488;
	sub_831A8130(ctx, base);
	// 822E8488: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E848C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E8490: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E8494: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E8498: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 822E849C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E84A0: 388BBB9C  addi r4, r11, -0x4464
	ctx.r[4].s64 = ctx.r[11].s64 + -17508;
	// 822E84A4: 38A000A4  li r5, 0xa4
	ctx.r[5].s64 = 164;
	// 822E84A8: 38600018  li r3, 0x18
	ctx.r[3].s64 = 24;
	// 822E84AC: 48B09F3D  bl 0x82df23e8
	ctx.lr = 0x822E84B0;
	sub_82DF23E8(ctx, base);
	// 822E84B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E84B4: 41820014  beq 0x822e84c8
	if ctx.cr[0].eq {
	pc = 0x822E84C8; continue 'dispatch;
	}
	// 822E84B8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 822E84BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E84C0: 4BFFFF21  bl 0x822e83e0
	ctx.lr = 0x822E84C4;
	sub_822E83E0(ctx, base);
	// 822E84C4: 48000008  b 0x822e84cc
	pc = 0x822E84CC; continue 'dispatch;
	// 822E84C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 822E84CC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E84D0: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 822E84D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E84D8: 48EBFCE4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E84E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822E84E0 size=296
    let mut pc: u32 = 0x822E84E0;
    'dispatch: loop {
        match pc {
            0x822E84E0 => {
    //   block [0x822E84E0..0x822E8608)
	// 822E84E0: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 822E84E4: 3881004C  addi r4, r1, 0x4c
	ctx.r[4].s64 = ctx.r[1].s64 + 76;
	// 822E84E8: D0E1004C  stfs f7, 0x4c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[7].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(76 as u32), tmp.u32 ) };
	// 822E84EC: D061002C  stfs f3, 0x2c(r1)
	tmp.f32 = (ctx.f[3].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(44 as u32), tmp.u32 ) };
	// 822E84F0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 822E84F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822E84F8: D021001C  stfs f1, 0x1c(r1)
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 822E84FC: 3921FFE0  addi r9, r1, -0x20
	ctx.r[9].s64 = ctx.r[1].s64 + -32;
	// 822E8500: D0410024  stfs f2, 0x24(r1)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(36 as u32), tmp.u32 ) };
	// 822E8504: 38C10044  addi r6, r1, 0x44
	ctx.r[6].s64 = ctx.r[1].s64 + 68;
	// 822E8508: D0A1003C  stfs f5, 0x3c(r1)
	tmp.f32 = (ctx.f[5].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 822E850C: 13002407  vcmpneb. (lvlx128) v24, v0, v4
	tmp.u32 = ctx.r[4].u32;
	// load shuffled into ctx.v[56] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E8510: 38A10024  addi r5, r1, 0x24
	ctx.r[5].s64 = ctx.r[1].s64 + 36;
	// 822E8514: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E8518: 3961003C  addi r11, r1, 0x3c
	ctx.r[11].s64 = ctx.r[1].s64 + 60;
	// 822E851C: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E8520: 3941001C  addi r10, r1, 0x1c
	ctx.r[10].s64 = ctx.r[1].s64 + 28;
	// 822E8524: D001FFE0  stfs f0, -0x20(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), tmp.u32 ) };
	// 822E8528: 38E1FFE8  addi r7, r1, -0x18
	ctx.r[7].s64 = ctx.r[1].s64 + -24;
	// 822E852C: 13E04C07  vcmpneb. (lvlx128) v31, v0, v9
	tmp.u32 = ctx.r[9].u32;
	// load shuffled into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E8530: 3921002C  addi r9, r1, 0x2c
	ctx.r[9].s64 = ctx.r[1].s64 + 44;
	// 822E8534: D0C10044  stfs f6, 0x44(r1)
	tmp.f32 = (ctx.f[6].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(68 as u32), tmp.u32 ) };
	// 822E8538: 3901FFE4  addi r8, r1, -0x1c
	ctx.r[8].s64 = ctx.r[1].s64 + -28;
	// 822E853C: 13805C07  vcmpneb. (lvlx128) v28, v0, v11
	tmp.u32 = ctx.r[11].u32;
	// load shuffled into ctx.v[60] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E8540: 39610064  addi r11, r1, 0x64
	ctx.r[11].s64 = ctx.r[1].s64 + 100;
	// 822E8544: 13605407  vcmpneb. (lvlx128) v27, v0, v10
	tmp.u32 = ctx.r[10].u32;
	// load shuffled into ctx.v[59] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E8548: 3941006C  addi r10, r1, 0x6c
	ctx.r[10].s64 = ctx.r[1].s64 + 108;
	// 822E854C: 13403407  vcmpneb. (lvlx128) v26, v0, v6
	tmp.u32 = ctx.r[6].u32;
	// load shuffled into ctx.v[58] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E8550: 38C10074  addi r6, r1, 0x74
	ctx.r[6].s64 = ctx.r[1].s64 + 116;
	// 822E8554: 13202C07  vcmpneb. (lvlx128) v25, v0, v5
	tmp.u32 = ctx.r[5].u32;
	// load shuffled into ctx.v[57] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E8558: 38A1FFE8  addi r5, r1, -0x18
	ctx.r[5].s64 = ctx.r[1].s64 + -24;
	// 822E855C: D001FFE8  stfs f0, -0x18(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
	// 822E8560: 3BE1005C  addi r31, r1, 0x5c
	ctx.r[31].s64 = ctx.r[1].s64 + 92;
	// 822E8564: 13A03C07  vcmpneb. (lvlx128) v29, v0, v7
	tmp.u32 = ctx.r[7].u32;
	// load shuffled into ctx.v[61] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E8568: 38E10034  addi r7, r1, 0x34
	ctx.r[7].s64 = ctx.r[1].s64 + 52;
	// 822E856C: D1410064  stfs f10, 0x64(r1)
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 822E8570: D161006C  stfs f11, 0x6c(r1)
	tmp.f32 = (ctx.f[11].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 822E8574: D1810074  stfs f12, 0x74(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 822E8578: D1A1FFE4  stfs f13, -0x1c(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-28 as u32), tmp.u32 ) };
	// 822E857C: 12E04C07  vcmpneb. (lvlx128) v23, v0, v9
	tmp.u32 = ctx.r[9].u32;
	// load shuffled into ctx.v[55] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E8580: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 822E8584: D001FFE8  stfs f0, -0x18(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8608(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x822E8608 size=140
    let mut pc: u32 = 0x822E8608;
    'dispatch: loop {
        match pc {
            0x822E8608 => {
    //   block [0x822E8608..0x822E8694)
	// 822E8608: C0040014  lfs f0, 0x14(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E860C: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 822E8610: 8161FFF4  lwz r11, -0xc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 822E8614: C0040010  lfs f0, 0x10(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E8618: D001FFF0  stfs f0, -0x10(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 822E861C: 8121FFF0  lwz r9, -0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 822E8620: C1A40018  lfs f13, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E8624: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E8628: D1A1FFF8  stfs f13, -8(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 822E862C: 8141FFF8  lwz r10, -8(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E8630: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822E8634: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E8638: C1840008  lfs f12, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 822E863C: C0040004  lfs f0, 4(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E8640: C1A40000  lfs f13, 0(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E8644: D1A1FFF0  stfs f13, -0x10(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), tmp.u32 ) };
	// 822E8648: 8121FFF0  lwz r9, -0x10(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) } as u64;
	// 822E864C: D181FFF8  stfs f12, -8(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), tmp.u32 ) };
	// 822E8650: 8161FFF8  lwz r11, -8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E8654: D001FFF4  stfs f0, -0xc(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-12 as u32), tmp.u32 ) };
	// 822E8658: 8141FFF4  lwz r10, -0xc(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-12 as u32) ) } as u64;
	// 822E865C: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 822E8660: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 822E8664: 9123000C  stw r9, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 822E8668: C004000C  lfs f0, 0xc(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E866C: D003001C  stfs f0, 0x1c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 822E8670: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 822E8674: 894B0010  lbz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 822E8678: 7D4A0774  extsb r10, r10
	ctx.r[10].s64 = ctx.r[10].s8 as i64;
	// 822E867C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 822E8680: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E8684: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 822E8688: 8164002C  lwz r11, 0x2c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 822E868C: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 822E8690: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E8698 size=112
    let mut pc: u32 = 0x822E8698;
    'dispatch: loop {
        match pc {
            0x822E8698 => {
    //   block [0x822E8698..0x822E8708)
	// 822E8698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E869C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E86A0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E86A4: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E86A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E86AC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 822E86B0: 38810080  addi r4, r1, 0x80
	ctx.r[4].s64 = ctx.r[1].s64 + 128;
	// 822E86B4: 48BB86DD  bl 0x82ea0d90
	ctx.lr = 0x822E86B8;
	sub_82EA0D90(ctx, base);
	// 822E86B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E86BC: C18100B8  lfs f12, 0xb8(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 822E86C0: C16100A8  lfs f11, 0xa8(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 822E86C4: C1410098  lfs f10, 0x98(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(152 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 822E86C8: C1210088  lfs f9, 0x88(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) };
	ctx.f[9].f64 = (tmp.f32 as f64);
	// 822E86CC: C10100B4  lfs f8, 0xb4(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) };
	ctx.f[8].f64 = (tmp.f32 as f64);
	// 822E86D0: C0E100A4  lfs f7, 0xa4(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) };
	ctx.f[7].f64 = (tmp.f32 as f64);
	// 822E86D4: C0C10094  lfs f6, 0x94(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) };
	ctx.f[6].f64 = (tmp.f32 as f64);
	// 822E86D8: C0A10084  lfs f5, 0x84(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) };
	ctx.f[5].f64 = (tmp.f32 as f64);
	// 822E86DC: C08100B0  lfs f4, 0xb0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 822E86E0: C06100A0  lfs f3, 0xa0(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(160 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 822E86E4: C0410090  lfs f2, 0x90(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 822E86E8: C0210080  lfs f1, 0x80(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822E86EC: 4BFFFDF5  bl 0x822e84e0
	ctx.lr = 0x822E86F0;
	sub_822E84E0(ctx, base);
	// 822E86F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E86F4: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 822E86F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E86FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E8700: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E8704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E8708 size=12
    let mut pc: u32 = 0x822E8708;
    'dispatch: loop {
        match pc {
            0x822E8708 => {
    //   block [0x822E8708..0x822E8714)
	// 822E8708: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E870C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E8710: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8714(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E8714 size=16
    let mut pc: u32 = 0x822E8714;
    'dispatch: loop {
        match pc {
            0x822E8714 => {
    //   block [0x822E8714..0x822E8724)
	// 822E8714: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E8718: 908B002C  stw r4, 0x2c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(44 as u32), ctx.r[4].u32 ) };
	// 822E871C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E8720: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8724(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E8724 size=36
    let mut pc: u32 = 0x822E8724;
    'dispatch: loop {
        match pc {
            0x822E8724 => {
    //   block [0x822E8724..0x822E8748)
	// 822E8724: 3D408335  lis r10, -0x7ccb
	ctx.r[10].s64 = -2093678592;
	// 822E8728: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822E872C: 814A666C  lwz r10, 0x666c(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(26220 as u32) ) } as u64;
	// 822E8730: 7F045040  cmplw cr6, r4, r10
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822E8734: 409A0008  bne cr6, 0x822e873c
	if !ctx.cr[6].eq {
	pc = 0x822E873C; continue 'dispatch;
	}
	// 822E8738: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E873C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E8740: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 822E8744: 48BE93D4  b 0x82ed1b18
	sub_82ED1B18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E8748 size=4
    let mut pc: u32 = 0x822E8748;
    'dispatch: loop {
        match pc {
            0x822E8748 => {
    //   block [0x822E8748..0x822E874C)
	// 822E8748: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E8750 size=104
    let mut pc: u32 = 0x822E8750;
    'dispatch: loop {
        match pc {
            0x822E8750 => {
    //   block [0x822E8750..0x822E87B8)
	// 822E8750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E8754: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E8758: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E875C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E8760: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E8764: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E8768: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 822E876C: 419A0034  beq cr6, 0x822e87a0
	if ctx.cr[6].eq {
	pc = 0x822E87A0; continue 'dispatch;
	}
	// 822E8770: 83FE0008  lwz r31, 8(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E8774: 909E002C  stw r4, 0x2c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(44 as u32), ctx.r[4].u32 ) };
	// 822E8778: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E877C: 419A0024  beq cr6, 0x822e87a0
	if ctx.cr[6].eq {
	pc = 0x822E87A0; continue 'dispatch;
	}
	// 822E8780: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E8784: 48BE607D  bl 0x82ece800
	ctx.lr = 0x822E8788;
	sub_82ECE800(ctx, base);
	// 822E8788: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822E878C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E8790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E8794: 48BE90DD  bl 0x82ed1870
	ctx.lr = 0x822E8798;
	sub_82ED1870(ctx, base);
	// 822E8798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E879C: 48BE486D  bl 0x82ecd008
	ctx.lr = 0x822E87A0;
	sub_82ECD008(ctx, base);
	// 822E87A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E87A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E87A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E87AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E87B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E87B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E87B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E87B8 size=64
    let mut pc: u32 = 0x822E87B8;
    'dispatch: loop {
        match pc {
            0x822E87B8 => {
    //   block [0x822E87B8..0x822E87F8)
	// 822E87B8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E87BC: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E87C0: 91440008  stw r10, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822E87C4: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E87C8: 892A0035  lbz r9, 0x35(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(53 as u32) ) } as u64;
	// 822E87CC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822E87D0: 409A0008  bne cr6, 0x822e87d8
	if !ctx.cr[6].eq {
	pc = 0x822E87D8; continue 'dispatch;
	}
	// 822E87D4: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822E87D8: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E87DC: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822E87E0: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E87E4: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E87E8: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E87EC: 409A000C  bne cr6, 0x822e87f8
	if !ctx.cr[6].eq {
		sub_822E87F8(ctx, base);
		return;
	}
	// 822E87F0: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E87F4: 48000020  b 0x822e8814
	sub_822E8810(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E87F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E87F8 size=24
    let mut pc: u32 = 0x822E87F8;
    'dispatch: loop {
        match pc {
            0x822E87F8 => {
    //   block [0x822E87F8..0x822E8810)
	// 822E87F8: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E87FC: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8800: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E8804: 409A000C  bne cr6, 0x822e8810
	if !ctx.cr[6].eq {
		sub_822E8810(ctx, base);
		return;
	}
	// 822E8808: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E880C: 48000008  b 0x822e8814
	sub_822E8810(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E8810 size=16
    let mut pc: u32 = 0x822E8810;
    'dispatch: loop {
        match pc {
            0x822E8810 => {
    //   block [0x822E8810..0x822E8820)
	// 822E8810: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822E8814: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 822E8818: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E881C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E8820 size=64
    let mut pc: u32 = 0x822E8820;
    'dispatch: loop {
        match pc {
            0x822E8820 => {
    //   block [0x822E8820..0x822E8860)
	// 822E8820: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8824: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E8828: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822E882C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E8830: 892A0035  lbz r9, 0x35(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(53 as u32) ) } as u64;
	// 822E8834: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822E8838: 409A0008  bne cr6, 0x822e8840
	if !ctx.cr[6].eq {
	pc = 0x822E8840; continue 'dispatch;
	}
	// 822E883C: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 822E8840: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8844: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822E8848: 81430004  lwz r10, 4(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E884C: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8850: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E8854: 409A000C  bne cr6, 0x822e8860
	if !ctx.cr[6].eq {
		sub_822E8860(ctx, base);
		return;
	}
	// 822E8858: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E885C: 48000020  b 0x822e887c
	sub_822E8878(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E8860 size=24
    let mut pc: u32 = 0x822E8860;
    'dispatch: loop {
        match pc {
            0x822E8860 => {
    //   block [0x822E8860..0x822E8878)
	// 822E8860: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8864: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E8868: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E886C: 409A000C  bne cr6, 0x822e8878
	if !ctx.cr[6].eq {
		sub_822E8878(ctx, base);
		return;
	}
	// 822E8870: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822E8874: 48000008  b 0x822e887c
	sub_822E8878(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E8878 size=16
    let mut pc: u32 = 0x822E8878;
    'dispatch: loop {
        match pc {
            0x822E8878 => {
    //   block [0x822E8878..0x822E8888)
	// 822E8878: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E887C: 908B0008  stw r4, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 822E8880: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E8884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E8888 size=596
    let mut pc: u32 = 0x822E8888;
    'dispatch: loop {
        match pc {
            0x822E8888 => {
    //   block [0x822E8888..0x822E8ADC)
	// 822E8888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E888C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E8890: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E8894: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E8898: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E889C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 822E88A0: C1840038  lfs f12, 0x38(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 822E88A4: C1640008  lfs f11, 8(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E8AE0 size=364
    let mut pc: u32 = 0x822E8AE0;
    'dispatch: loop {
        match pc {
            0x822E8AE0 => {
    //   block [0x822E8AE0..0x822E8C4C)
	// 822E8AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E8AE4: 48EBF685  bl 0x831a8168
	ctx.lr = 0x822E8AE8;
	sub_831A8130(ctx, base);
	// 822E8AE8: 9421FD60  stwu r1, -0x2a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-672 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E8AEC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822E8AF0: 13E0E8C7  vcmpequd (lvx128) v31, v0, v29
	tmp.u32 = ctx.r[29].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E8C50 size=84
    let mut pc: u32 = 0x822E8C50;
    'dispatch: loop {
        match pc {
            0x822E8C50 => {
    //   block [0x822E8C50..0x822E8CA4)
	// 822E8C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E8C54: 48EBF519  bl 0x831a816c
	ctx.lr = 0x822E8C58;
	sub_831A8130(ctx, base);
	// 822E8C58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E8C5C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E8C60: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E8C64: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 822E8C68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E8C6C: 409A0030  bne cr6, 0x822e8c9c
	if !ctx.cr[6].eq {
	pc = 0x822E8C9C; continue 'dispatch;
	}
	// 822E8C70: 3FA08335  lis r29, -0x7ccb
	ctx.r[29].s64 = -2093678592;
	// 822E8C74: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E8C78: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E8C7C: 4BFFFFD5  bl 0x822e8c50
	ctx.lr = 0x822E8C80;
	sub_822E8C50(ctx, base);
	// 822E8C80: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E8C84: 807D110C  lwz r3, 0x110c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822E8C88: 83FF0000  lwz r31, 0(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8C8C: 48B094FD  bl 0x82df2188
	ctx.lr = 0x822E8C90;
	sub_82DF2188(ctx, base);
	// 822E8C90: 897F0035  lbz r11, 0x35(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(53 as u32) ) } as u64;
	// 822E8C94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E8C98: 419AFFDC  beq cr6, 0x822e8c74
	if ctx.cr[6].eq {
	pc = 0x822E8C74; continue 'dispatch;
	}
	// 822E8C9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E8CA0: 48EBF51C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E8CA8 size=120
    let mut pc: u32 = 0x822E8CA8;
    'dispatch: loop {
        match pc {
            0x822E8CA8 => {
    //   block [0x822E8CA8..0x822E8D20)
	// 822E8CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E8CAC: 48EBF4B5  bl 0x831a8160
	ctx.lr = 0x822E8CB0;
	sub_831A8130(ctx, base);
	// 822E8CB0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E8CB4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 822E8CB8: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822E8CBC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E8CC0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E8CC4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 822E8CC8: 38C00038  li r6, 0x38
	ctx.r[6].s64 = 56;
	// 822E8CCC: 806B110C  lwz r3, 0x110c(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4364 as u32) ) } as u64;
	// 822E8CD0: 388A08B0  addi r4, r10, 0x8b0
	ctx.r[4].s64 = ctx.r[10].s64 + 2224;
	// 822E8CD4: 38A0002D  li r5, 0x2d
	ctx.r[5].s64 = 45;
	// 822E8CD8: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 822E8CDC: 7D1A4378  mr r26, r8
	ctx.r[26].u64 = ctx.r[8].u64;
	// 822E8CE0: 48B093E9  bl 0x82df20c8
	ctx.lr = 0x822E8CE4;
	sub_82DF20C8(ctx, base);
	// 822E8CE4: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822E8CE8: 4182002C  beq 0x822e8d14
	if ctx.cr[0].eq {
	pc = 0x822E8D14; continue 'dispatch;
	}
	// 822E8CEC: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 822E8CF0: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 822E8CF4: 93BF0004  stw r29, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 822E8CF8: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	// 822E8CFC: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 822E8D00: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 822E8D04: 48EBF80D  bl 0x831a8510
	ctx.lr = 0x822E8D08;
	sub_831A8510(ctx, base);
	// 822E8D08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E8D0C: 9B5F0034  stb r26, 0x34(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[26].u8 ) };
	// 822E8D10: 997F0035  stb r11, 0x35(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(53 as u32), ctx.r[11].u8 ) };
	// 822E8D14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E8D18: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 822E8D1C: 48EBF494  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8D20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E8D20 size=84
    let mut pc: u32 = 0x822E8D20;
    'dispatch: loop {
        match pc {
            0x822E8D20 => {
    //   block [0x822E8D20..0x822E8D74)
	// 822E8D20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E8D24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E8D28: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E8D2C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E8D30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E8D34: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8D38: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8D3C: 4BFFFF15  bl 0x822e8c50
	ctx.lr = 0x822E8D40;
	sub_822E8C50(ctx, base);
	// 822E8D40: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8D44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E8D48: 914A0004  stw r10, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822E8D4C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822E8D50: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8D54: 914A0000  stw r10, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822E8D58: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8D5C: 916B0008  stw r11, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822E8D60: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E8D64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E8D68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E8D6C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E8D70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E8D78 size=548
    let mut pc: u32 = 0x822E8D78;
    'dispatch: loop {
        match pc {
            0x822E8D78 => {
    //   block [0x822E8D78..0x822E8F9C)
	// 822E8D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E8D7C: 48EBF3E5  bl 0x831a8160
	ctx.lr = 0x822E8D80;
	sub_831A8130(ctx, base);
	// 822E8D80: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E8D84: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E8D88: 3D600666  lis r11, 0x666
	ctx.r[11].s64 = 107347968;
	// 822E8D8C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 822E8D90: 616B6665  ori r11, r11, 0x6665
	ctx.r[11].u64 = ctx.r[11].u64 | 26213;
	// 822E8D94: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 822E8D98: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E8D9C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 822E8DA0: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 822E8DA4: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E8DA8: 41980048  blt cr6, 0x822e8df0
	if ctx.cr[6].lt {
	pc = 0x822E8DF0; continue 'dispatch;
	}
	// 822E8DAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E8DB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E8DB4: 388B9BCC  addi r4, r11, -0x6434
	ctx.r[4].s64 = ctx.r[11].s64 + -25652;
	// 822E8DB8: 4BFDCB11  bl 0x822c58c8
	ctx.lr = 0x822E8DBC;
	sub_822C58C8(ctx, base);
	// 822E8DBC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 822E8DC0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822E8DC4: 4BFDCA55  bl 0x822c5818
	ctx.lr = 0x822E8DC8;
	sub_822C5818(ctx, base);
	// 822E8DC8: 4BFDB4E9  bl 0x822c42b0
	ctx.lr = 0x822E8DCC;
	sub_822C42B0(ctx, base);
	// 822E8DCC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E8DD0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 822E8DD4: 396B94A0  addi r11, r11, -0x6b60
	ctx.r[11].s64 = ctx.r[11].s64 + -27488;
	// 822E8DD8: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 822E8DDC: 4BFDC695  bl 0x822c5470
	ctx.lr = 0x822E8DE0;
	sub_822C5470(ctx, base);
	// 822E8DE0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 822E8DE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 822E8DE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E8DEC: 4BFDBEF5  bl 0x822c4ce0
	ctx.lr = 0x822E8DF0;
	sub_822C4CE0(ctx, base);
	// 822E8DF0: 809E0004  lwz r4, 4(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8DF4: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 822E8DF8: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 822E8DFC: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 822E8E00: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822E8E04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E8E08: 4BFFFEA1  bl 0x822e8ca8
	ctx.lr = 0x822E8E0C;
	sub_822E8CA8(ctx, base);
	// 822E8E0C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E8E10: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8E14: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822E8E18: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 822E8E1C: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E8E20: 915E0008  stw r10, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822E8E24: 409A0018  bne cr6, 0x822e8e3c
	if !ctx.cr[6].eq {
	pc = 0x822E8E3C; continue 'dispatch;
	}
	// 822E8E28: 938B0004  stw r28, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 822E8E2C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8E30: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 822E8E34: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8E38: 4800003C  b 0x822e8e74
	pc = 0x822E8E74; continue 'dispatch;
	// 822E8E3C: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E8E40: 41820020  beq 0x822e8e60
	if ctx.cr[0].eq {
	pc = 0x822E8E60; continue 'dispatch;
	}
	// 822E8E44: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 822E8E48: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8E4C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8E50: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822E8E54: 409A0024  bne cr6, 0x822e8e78
	if !ctx.cr[6].eq {
	pc = 0x822E8E78; continue 'dispatch;
	}
	// 822E8E58: 938B0000  stw r28, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 822E8E5C: 4800001C  b 0x822e8e78
	pc = 0x822E8E78; continue 'dispatch;
	// 822E8E60: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 822E8E64: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8E68: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E8E6C: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 822E8E70: 409A0008  bne cr6, 0x822e8e78
	if !ctx.cr[6].eq {
	pc = 0x822E8E78; continue 'dispatch;
	}
	// 822E8E74: 938B0008  stw r28, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 822E8E78: 815C0004  lwz r10, 4(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8E7C: 397C0004  addi r11, r28, 4
	ctx.r[11].s64 = ctx.r[28].s64 + 4;
	// 822E8E80: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 822E8E84: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 822E8E88: 894A0034  lbz r10, 0x34(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 822E8E8C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822E8E90: 409A00F0  bne cr6, 0x822e8f80
	if !ctx.cr[6].eq {
	pc = 0x822E8F80; continue 'dispatch;
	}
	// 822E8E94: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 822E8E98: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8E9C: 81440004  lwz r10, 4(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8EA0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8EA4: 7F044840  cmplw cr6, r4, r9
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E8EA8: 409A0054  bne cr6, 0x822e8efc
	if !ctx.cr[6].eq {
	pc = 0x822E8EFC; continue 'dispatch;
	}
	// 822E8EAC: 814A0008  lwz r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E8EB0: 892A0034  lbz r9, 0x34(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 822E8EB4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822E8EB8: 419A0054  beq cr6, 0x822e8f0c
	if ctx.cr[6].eq {
	pc = 0x822E8F0C; continue 'dispatch;
	}
	// 822E8EBC: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E8EC0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E8EC4: 409A0010  bne cr6, 0x822e8ed4
	if !ctx.cr[6].eq {
	pc = 0x822E8ED4; continue 'dispatch;
	}
	// 822E8EC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E8ECC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E8ED0: 4BFFF8E9  bl 0x822e87b8
	ctx.lr = 0x822E8ED4;
	sub_822E87B8(ctx, base);
	// 822E8ED4: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8ED8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E8EDC: 9BAB0034  stb r29, 0x34(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[29].u8 ) };
	// 822E8EE0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8EE4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8EE8: 9B6B0034  stb r27, 0x34(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[27].u8 ) };
	// 822E8EEC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8EF0: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8EF4: 4BFFF92D  bl 0x822e8820
	ctx.lr = 0x822E8EF8;
	sub_822E8820(ctx, base);
	// 822E8EF8: 48000074  b 0x822e8f6c
	pc = 0x822E8F6C; continue 'dispatch;
	// 822E8EFC: 814A0000  lwz r10, 0(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8F00: 892A0034  lbz r9, 0x34(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 822E8F04: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 822E8F08: 409A0028  bne cr6, 0x822e8f30
	if !ctx.cr[6].eq {
	pc = 0x822E8F30; continue 'dispatch;
	}
	// 822E8F0C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8F10: 9BA90034  stb r29, 0x34(r9)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[9].u32.wrapping_add(52 as u32), ctx.r[29].u8 ) };
	// 822E8F14: 9BAA0034  stb r29, 0x34(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[29].u8 ) };
	// 822E8F18: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8F1C: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8F20: 9B6A0034  stb r27, 0x34(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(52 as u32), ctx.r[27].u8 ) };
	// 822E8F24: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8F28: 83EB0004  lwz r31, 4(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8F2C: 48000040  b 0x822e8f6c
	pc = 0x822E8F6C; continue 'dispatch;
	// 822E8F30: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8F34: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E8F38: 409A0010  bne cr6, 0x822e8f48
	if !ctx.cr[6].eq {
	pc = 0x822E8F48; continue 'dispatch;
	}
	// 822E8F3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E8F40: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E8F44: 4BFFF8DD  bl 0x822e8820
	ctx.lr = 0x822E8F48;
	sub_822E8820(ctx, base);
	// 822E8F48: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8F4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E8F50: 9BAB0034  stb r29, 0x34(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[29].u8 ) };
	// 822E8F54: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8F58: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8F5C: 9B6B0034  stb r27, 0x34(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[27].u8 ) };
	// 822E8F60: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8F64: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8F68: 4BFFF851  bl 0x822e87b8
	ctx.lr = 0x822E8F6C;
	sub_822E87B8(ctx, base);
	// 822E8F6C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8F70: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 822E8F74: 894A0034  lbz r10, 0x34(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 822E8F78: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822E8F7C: 419AFF1C  beq cr6, 0x822e8e98
	if ctx.cr[6].eq {
	pc = 0x822E8E98; continue 'dispatch;
	}
	// 822E8F80: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8F84: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 822E8F88: 939A0000  stw r28, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 822E8F8C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8F90: 9BAB0034  stb r29, 0x34(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(52 as u32), ctx.r[29].u8 ) };
	// 822E8F94: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 822E8F98: 48EBF218  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E8FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E8FA0 size=264
    let mut pc: u32 = 0x822E8FA0;
    'dispatch: loop {
        match pc {
            0x822E8FA0 => {
    //   block [0x822E8FA0..0x822E90A8)
	// 822E8FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E8FA4: 48EBF1BD  bl 0x831a8160
	ctx.lr = 0x822E8FA8;
	sub_831A8130(ctx, base);
	// 822E8FA8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E8FAC: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 822E8FB0: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 822E8FB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E8FB8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 822E8FBC: 7F5DD378  mr r29, r26
	ctx.r[29].u64 = ctx.r[26].u64;
	// 822E8FC0: 83DC0004  lwz r30, 4(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8FC4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E8FC8: 894B0035  lbz r10, 0x35(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(53 as u32) ) } as u64;
	// 822E8FCC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822E8FD0: 409A0040  bne cr6, 0x822e9010
	if !ctx.cr[6].eq {
	pc = 0x822E9010; continue 'dispatch;
	}
	// 822E8FD4: C01B001C  lfs f0, 0x1c(r27)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E8FD8: C1AB0028  lfs f13, 0x28(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E8FDC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 822E8FE0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E8FE4: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 822E8FE8: 41980008  blt cr6, 0x822e8ff0
	if ctx.cr[6].lt {
	pc = 0x822E8FF0; continue 'dispatch;
	}
	// 822E8FEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 822E8FF0: 555D063F  clrlwi. r29, r10, 0x18
	ctx.r[29].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 822E8FF4: 4182000C  beq 0x822e9000
	if ctx.cr[0].eq {
	pc = 0x822E9000; continue 'dispatch;
	}
	// 822E8FF8: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E8FFC: 48000008  b 0x822e9004
	pc = 0x822E9004; continue 'dispatch;
	// 822E9000: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E9004: 894B0035  lbz r10, 0x35(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(53 as u32) ) } as u64;
	// 822E9008: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 822E900C: 419AFFCC  beq cr6, 0x822e8fd8
	if ctx.cr[6].eq {
	pc = 0x822E8FD8; continue 'dispatch;
	}
	// 822E9010: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 822E9014: 57AB063F  clrlwi. r11, r29, 0x18
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E9018: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822E901C: 41820048  beq 0x822e9064
	if ctx.cr[0].eq {
	pc = 0x822E9064; continue 'dispatch;
	}
	// 822E9020: 817C0004  lwz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E9024: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E9028: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E902C: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E9030: 409A002C  bne cr6, 0x822e905c
	if !ctx.cr[6].eq {
	pc = 0x822E905C; continue 'dispatch;
	}
	// 822E9034: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 822E9038: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E903C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 822E9040: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 822E9044: 4BFFFD35  bl 0x822e8d78
	ctx.lr = 0x822E9048;
	sub_822E8D78(ctx, base);
	// 822E9048: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E904C: 9B5F0004  stb r26, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[26].u8 ) };
	// 822E9050: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E9054: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E9058: 48000044  b 0x822e909c
	pc = 0x822E909C; continue 'dispatch;
	// 822E905C: 482660E5  bl 0x8254f140
	ctx.lr = 0x822E9060;
	sub_8254F140(ctx, base);
	// 822E9060: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 822E9064: C00A0028  lfs f0, 0x28(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 822E9068: 7F4BD378  mr r11, r26
	ctx.r[11].u64 = ctx.r[26].u64;
	// 822E906C: C1BB001C  lfs f13, 0x1c(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 822E9070: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 822E9074: 41980008  blt cr6, 0x822e907c
	if ctx.cr[6].lt {
	pc = 0x822E907C; continue 'dispatch;
	}
	// 822E9078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E907C: 556B063F  clrlwi. r11, r11, 0x18
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E9080: 41820010  beq 0x822e9090
	if ctx.cr[0].eq {
	pc = 0x822E9090; continue 'dispatch;
	}
	// 822E9084: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 822E9088: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E908C: 4BFFFFAC  b 0x822e9038
	pc = 0x822E9038; continue 'dispatch;
	// 822E9090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E9094: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 822E9098: 997F0004  stb r11, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 822E909C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E90A0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 822E90A4: 48EBF10C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E90A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E90A8 size=92
    let mut pc: u32 = 0x822E90A8;
    'dispatch: loop {
        match pc {
            0x822E90A8 => {
    //   block [0x822E90A8..0x822E9104)
	// 822E90A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E90AC: 48EBF0BD  bl 0x831a8168
	ctx.lr = 0x822E90B0;
	sub_831A8130(ctx, base);
	// 822E90B0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E90B4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E90B8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 822E90BC: 4BFFFC65  bl 0x822e8d20
	ctx.lr = 0x822E90C0;
	sub_822E8D20(ctx, base);
	// 822E90C0: 83FC0004  lwz r31, 4(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E90C4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822E90C8: 40990034  ble cr6, 0x822e90fc
	if !ctx.cr[6].gt {
	pc = 0x822E90FC; continue 'dispatch;
	}
	// 822E90CC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 822E90D0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E90D4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 822E90D8: 7C9E5A14  add r4, r30, r11
	ctx.r[4].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 822E90DC: 4BFFF52D  bl 0x822e8608
	ctx.lr = 0x822E90E0;
	sub_822E8608(ctx, base);
	// 822E90E0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 822E90E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E90E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E90EC: 4BFFFEB5  bl 0x822e8fa0
	ctx.lr = 0x822E90F0;
	sub_822E8FA0(ctx, base);
	// 822E90F0: 37FFFFFF  addic. r31, r31, -1
	ctx.xer.ca = (ctx.r[31].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 822E90F4: 3BDE0030  addi r30, r30, 0x30
	ctx.r[30].s64 = ctx.r[30].s64 + 48;
	// 822E90F8: 4082FFD8  bne 0x822e90d0
	if !ctx.cr[0].eq {
	pc = 0x822E90D0; continue 'dispatch;
	}
	// 822E90FC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 822E9100: 48EBF0B8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E9108 size=132
    let mut pc: u32 = 0x822E9108;
    'dispatch: loop {
        match pc {
            0x822E9108 => {
    //   block [0x822E9108..0x822E918C)
	// 822E9108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E910C: 48EBF05D  bl 0x831a8168
	ctx.lr = 0x822E9110;
	sub_831A8130(ctx, base);
	// 822E9110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E9114: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E9118: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E911C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 822E9120: 48B09FD1  bl 0x82df30f0
	ctx.lr = 0x822E9124;
	sub_82DF30F0(ctx, base);
	// 822E9124: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E9128: 48B0A309  bl 0x82df3430
	ctx.lr = 0x822E912C;
	sub_82DF3430(ctx, base);
	// 822E912C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 822E9130: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E9134: 48B0A2FD  bl 0x82df3430
	ctx.lr = 0x822E9138;
	sub_82DF3430(ctx, base);
	// 822E9138: 7D7C1A14  add r11, r28, r3
	ctx.r[11].u64 = ctx.r[28].u64 + ctx.r[3].u64;
	// 822E913C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E9140: 388B0008  addi r4, r11, 8
	ctx.r[4].s64 = ctx.r[11].s64 + 8;
	// 822E9144: 48DDE99D  bl 0x830c7ae0
	ctx.lr = 0x822E9148;
	sub_830C7AE0(ctx, base);
	// 822E9148: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E914C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E9150: 388BBBD4  addi r4, r11, -0x442c
	ctx.r[4].s64 = ctx.r[11].s64 + -17452;
	// 822E9154: 48B0A425  bl 0x82df3578
	ctx.lr = 0x822E9158;
	sub_82DF3578(ctx, base);
	// 822E9158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E915C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E9160: 48B0A579  bl 0x82df36d8
	ctx.lr = 0x822E9164;
	sub_82DF36D8(ctx, base);
	// 822E9164: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E9168: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E916C: 388B9FD8  addi r4, r11, -0x6028
	ctx.r[4].s64 = ctx.r[11].s64 + -24616;
	// 822E9170: 48B0A409  bl 0x82df3578
	ctx.lr = 0x822E9174;
	sub_82DF3578(ctx, base);
	// 822E9174: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E9178: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E917C: 48B0A55D  bl 0x82df36d8
	ctx.lr = 0x822E9180;
	sub_82DF36D8(ctx, base);
	// 822E9180: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E9184: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E9188: 48EBF030  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E9190 size=192
    let mut pc: u32 = 0x822E9190;
    'dispatch: loop {
        match pc {
            0x822E9190 => {
    //   block [0x822E9190..0x822E9250)
	// 822E9190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E9194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E9198: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E919C: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E91A0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 822E91A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E91A8: 4817BBE1  bl 0x82464d88
	ctx.lr = 0x822E91AC;
	sub_82464D88(ctx, base);
	// 822E91AC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E91B0: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 822E91B4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E91B8: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 822E91BC: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E9250 size=76
    let mut pc: u32 = 0x822E9250;
    'dispatch: loop {
        match pc {
            0x822E9250 => {
    //   block [0x822E9250..0x822E929C)
	// 822E9250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E9254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E9258: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E925C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E9260: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E9264: 48B09275  bl 0x82df24d8
	ctx.lr = 0x822E9268;
	sub_82DF24D8(ctx, base);
	// 822E9268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E926C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E9270: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 822E9274: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822E9278: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 822E927C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822E9280: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 822E9284: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 822E9288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E928C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E9290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E9294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E9298: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E92A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E92A0 size=84
    let mut pc: u32 = 0x822E92A0;
    'dispatch: loop {
        match pc {
            0x822E92A0 => {
    //   block [0x822E92A0..0x822E92F4)
	// 822E92A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E92A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E92A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E92AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E92B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E92B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E92B8: 3BFE0004  addi r31, r30, 4
	ctx.r[31].s64 = ctx.r[30].s64 + 4;
	// 822E92BC: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 822E92C0: 48180D41  bl 0x8246a000
	ctx.lr = 0x822E92C4;
	sub_8246A000(ctx, base);
	// 822E92C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E92C8: 48180D39  bl 0x8246a000
	ctx.lr = 0x822E92CC;
	sub_8246A000(ctx, base);
	// 822E92CC: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E92D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 822E92D4: 419A0008  beq cr6, 0x822e92dc
	if ctx.cr[6].eq {
	pc = 0x822E92DC; continue 'dispatch;
	}
	// 822E92D8: 4BFFEF91  bl 0x822e8268
	ctx.lr = 0x822E92DC;
	sub_822E8268(ctx, base);
	// 822E92DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E92E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E92E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E92E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E92EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E92F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E92F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E92F8 size=188
    let mut pc: u32 = 0x822E92F8;
    'dispatch: loop {
        match pc {
            0x822E92F8 => {
    //   block [0x822E92F8..0x822E93B4)
	// 822E92F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E92FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E9300: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E9304: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E9308: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E930C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E9310: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E9314: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 822E9318: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 822E931C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E9320: 4BFD7619  bl 0x822c0938
	ctx.lr = 0x822E9324;
	sub_822C0938(ctx, base);
	// 822E9324: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E9328: 41820028  beq 0x822e9350
	if ctx.cr[0].eq {
	pc = 0x822E9350; continue 'dispatch;
	}
	// 822E932C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E9330: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 822E9334: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 822E9338: 392BBBF4  addi r9, r11, -0x440c
	ctx.r[9].s64 = ctx.r[11].s64 + -17420;
	// 822E933C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 822E9340: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E9344: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 822E9348: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 822E934C: 48000008  b 0x822e9354
	pc = 0x822E9354; continue 'dispatch;
	// 822E9350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E9354: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E9358: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 822E935C: 409A003C  bne cr6, 0x822e9398
	if !ctx.cr[6].eq {
	pc = 0x822E9398; continue 'dispatch;
	}
	// 822E9360: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E9364: 419A0014  beq cr6, 0x822e9378
	if ctx.cr[6].eq {
	pc = 0x822E9378; continue 'dispatch;
	}
	// 822E9368: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E936C: 4BFFFF35  bl 0x822e92a0
	ctx.lr = 0x822E9370;
	sub_822E92A0(ctx, base);
	// 822E9370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E9374: 4BFD6EF5  bl 0x822c0268
	ctx.lr = 0x822E9378;
	sub_822C0268(ctx, base);
	// 822E9378: 3D608325  lis r11, -0x7cdb
	ctx.r[11].s64 = -2094727168;
	// 822E937C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 822E9380: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E9384: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 822E9388: 816B4234  lwz r11, 0x4234(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16948 as u32) ) } as u64;
	// 822E938C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 822E9390: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 822E9394: 4BFD6C6D  bl 0x822c0000
	ctx.lr = 0x822E9398;
	sub_822C0000(ctx, base);
	// 822E9398: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E939C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E93A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E93A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E93A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E93AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E93B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E93B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E93B8 size=64
    let mut pc: u32 = 0x822E93B8;
    'dispatch: loop {
        match pc {
            0x822E93B8 => {
    //   block [0x822E93B8..0x822E93F8)
	// 822E93B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E93BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E93C0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E93C4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E93C8: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 822E93CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E93D0: 419A0014  beq cr6, 0x822e93e4
	if ctx.cr[6].eq {
	pc = 0x822E93E4; continue 'dispatch;
	}
	// 822E93D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E93D8: 4BFFFEC9  bl 0x822e92a0
	ctx.lr = 0x822E93DC;
	sub_822E92A0(ctx, base);
	// 822E93DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E93E0: 4BFD6E89  bl 0x822c0268
	ctx.lr = 0x822E93E4;
	sub_822C0268(ctx, base);
	// 822E93E4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 822E93E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E93EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E93F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E93F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E93F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x822E93F8 size=792
    let mut pc: u32 = 0x822E93F8;
    'dispatch: loop {
        match pc {
            0x822E93F8 => {
    //   block [0x822E93F8..0x822E9710)
	// 822E93F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E93FC: 48EBED45  bl 0x831a8140
	ctx.lr = 0x822E9400;
	sub_831A8130(ctx, base);
	// 822E9400: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E9404: 826D0000  lwz r19, 0(r13)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E9408: 3A400014  li r18, 0x14
	ctx.r[18].s64 = 20;
	// 822E940C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 822E9410: 38A00027  li r5, 0x27
	ctx.r[5].s64 = 39;
	// 822E9414: 388000C0  li r4, 0xc0
	ctx.r[4].s64 = 192;
	// 822E9418: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 822E941C: 7C72982E  lwzx r3, r18, r19
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[18].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 822E9420: 48BB7311  bl 0x82ea0730
	ctx.lr = 0x822E9424;
	sub_82EA0730(ctx, base);
	// 822E9424: 396000C0  li r11, 0xc0
	ctx.r[11].s64 = 192;
	// 822E9428: 3D408332  lis r10, -0x7cce
	ctx.r[10].s64 = -2093875200;
	// 822E942C: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 822E9430: 38A0000C  li r5, 0xc
	ctx.r[5].s64 = 12;
	// 822E9434: C02AF614  lfs f1, -0x9ec(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-2540 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 822E9438: 48C2F569  bl 0x82f189a0
	ctx.lr = 0x822E943C;
	sub_82F189A0(ctx, base);
	// 822E943C: 3AA00000  li r21, 0
	ctx.r[21].s64 = 0;
	// 822E9440: 817A0004  lwz r11, 4(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E9444: 3A80000C  li r20, 0xc
	ctx.r[20].s64 = 12;
	// 822E9448: 813A0008  lwz r9, 8(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E944C: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 822E9450: 7EAAAB78  mr r10, r21
	ctx.r[10].u64 = ctx.r[21].u64;
	// 822E9454: 48000034  b 0x822e9488
	pc = 0x822E9488; continue 'dispatch;
	// 822E9458: 810B0004  lwz r8, 4(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E945C: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 822E9460: 409A000C  bne cr6, 0x822e946c
	if !ctx.cr[6].eq {
	pc = 0x822E946C; continue 'dispatch;
	}
	// 822E9464: 7EA8AB78  mr r8, r21
	ctx.r[8].u64 = ctx.r[21].u64;
	// 822E9468: 48000010  b 0x822e9478
	pc = 0x822E9478; continue 'dispatch;
	// 822E946C: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E9470: 7D083850  subf r8, r8, r7
	ctx.r[8].s64 = ctx.r[7].s64 - ctx.r[8].s64;
	// 822E9474: 7D08A3D6  divw r8, r8, r20
	ctx.r[8].s32 = ctx.r[8].s32 / ctx.r[20].s32;
	// 822E9478: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 822E947C: 396B0010  addi r11, r11, 0x10
	ctx.r[11].s64 = ctx.r[11].s64 + 16;
	// 822E9480: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 822E9484: 554A043E  clrlwi r10, r10, 0x10
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 822E9488: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 822E948C: 409AFFCC  bne cr6, 0x822e9458
	if !ctx.cr[6].eq {
	pc = 0x822E9458; continue 'dispatch;
	}
	// 822E9490: 555F043E  clrlwi r31, r10, 0x10
	ctx.r[31].u64 = ctx.r[10].u32 as u64 & 0x0000FFFFu64;
	// 822E9494: 3B790004  addi r27, r25, 4
	ctx.r[27].s64 = ctx.r[25].s64 + 4;
	// 822E9498: 1D7F0003  mulli r11, r31, 3
	ctx.r[11].s64 = ctx.r[31].s64 * 3;
	// 822E949C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 822E94A0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 822E94A4: 4BFF2EF5  bl 0x822dc398
	ctx.lr = 0x822E94A8;
	sub_822DC398(ctx, base);
	// 822E94A8: 3B190014  addi r24, r25, 0x14
	ctx.r[24].s64 = ctx.r[25].s64 + 20;
	// 822E94AC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E94B0: 7F03C378  mr r3, r24
	ctx.r[3].u64 = ctx.r[24].u64;
	// 822E94B4: 4BFF2FDD  bl 0x822dc490
	ctx.lr = 0x822E94B8;
	sub_822DC490(ctx, base);
	// 822E94B8: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E94BC: 83DA0004  lwz r30, 4(r26)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E94C0: 7F1E5840  cmplw cr6, r30, r11
	ctx.cr[6].compare_u32(ctx.r[30].u32, ctx.r[11].u32, &mut ctx.xer);
	// 822E94C4: 419A01C0  beq cr6, 0x822e9684
	if ctx.cr[6].eq {
	pc = 0x822E9684; continue 'dispatch;
	}
	// 822E94C8: 3AE00001  li r23, 1
	ctx.r[23].s64 = 1;
	// 822E94CC: 396100B0  addi r11, r1, 0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + 176;
	// 822E94D0: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 822E94D4: 13E058C7  vcmpequd (lvx128) v31, v0, v11
	tmp.u32 = ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 822E94D8: 9AA10090  stb r21, 0x90(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[21].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E9710 size=160
    let mut pc: u32 = 0x822E9710;
    'dispatch: loop {
        match pc {
            0x822E9710 => {
    //   block [0x822E9710..0x822E97B0)
	// 822E9710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E9714: 48EBEA55  bl 0x831a8168
	ctx.lr = 0x822E9718;
	sub_831A8130(ctx, base);
	// 822E9718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E971C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 822E9720: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E9724: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 822E9728: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 822E972C: 388BBC04  addi r4, r11, -0x43fc
	ctx.r[4].s64 = ctx.r[11].s64 + -17404;
	// 822E9730: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 822E9734: 38600024  li r3, 0x24
	ctx.r[3].s64 = 36;
	// 822E9738: 4BFD6CA1  bl 0x822c03d8
	ctx.lr = 0x822E973C;
	sub_822C03D8(ctx, base);
	// 822E973C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E9740: 41820030  beq 0x822e9770
	if ctx.cr[0].eq {
	pc = 0x822E9770; continue 'dispatch;
	}
	// 822E9744: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 822E9748: 39430004  addi r10, r3, 4
	ctx.r[10].s64 = ctx.r[3].s64 + 4;
	// 822E974C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 822E9750: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E9754: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 822E9758: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 822E975C: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 822E9760: 91630018  stw r11, 0x18(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 822E9764: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 822E9768: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 822E976C: 48000008  b 0x822e9774
	pc = 0x822E9774; continue 'dispatch;
	// 822E9770: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 822E9774: 93FE0000  stw r31, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 822E9778: 3BBE0004  addi r29, r30, 4
	ctx.r[29].s64 = ctx.r[30].s64 + 4;
	// 822E977C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E9780: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E9784: 4BFFFB75  bl 0x822e92f8
	ctx.lr = 0x822E9788;
	sub_822E92F8(ctx, base);
	// 822E9788: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 822E978C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 822E9790: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 822E9794: 4BFD686D  bl 0x822c0000
	ctx.lr = 0x822E9798;
	sub_822C0000(ctx, base);
	// 822E9798: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 822E979C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E97A0: 4BFFFC59  bl 0x822e93f8
	ctx.lr = 0x822E97A4;
	sub_822E93F8(ctx, base);
	// 822E97A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E97A8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 822E97AC: 48EBEA0C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E97B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E97B0 size=16
    let mut pc: u32 = 0x822E97B0;
    'dispatch: loop {
        match pc {
            0x822E97B0 => {
    //   block [0x822E97B0..0x822E97C0)
	// 822E97B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 822E97B4: 886B0061  lbz r3, 0x61(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(97 as u32) ) } as u64;
	// 822E97B8: 988B0061  stb r4, 0x61(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(97 as u32), ctx.r[4].u8 ) };
	// 822E97BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E97C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E97C0 size=80
    let mut pc: u32 = 0x822E97C0;
    'dispatch: loop {
        match pc {
            0x822E97C0 => {
    //   block [0x822E97C0..0x822E9810)
	// 822E97C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E97C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 822E97C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 822E97CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 822E97D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E97D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 822E97D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E97DC: 48BEF48D  bl 0x82ed8c68
	ctx.lr = 0x822E97E0;
	sub_82ED8C68(ctx, base);
	// 822E97E0: 817F00D0  lwz r11, 0xd0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 822E97E4: 387F00D0  addi r3, r31, 0xd0
	ctx.r[3].s64 = ctx.r[31].s64 + 208;
	// 822E97E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 822E97EC: 816B0040  lwz r11, 0x40(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(64 as u32) ) } as u64;
	// 822E97F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 822E97F4: 4E800421  bctrl
	ctx.lr = 0x822E97F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 822E97F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E97FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 822E9800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 822E9804: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 822E9808: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 822E980C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E9810 size=100
    let mut pc: u32 = 0x822E9810;
    'dispatch: loop {
        match pc {
            0x822E9810 => {
    //   block [0x822E9810..0x822E9874)
	// 822E9810: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E9814: 48EBE959  bl 0x831a816c
	ctx.lr = 0x822E9818;
	sub_831A8130(ctx, base);
	// 822E9818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E981C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 822E9820: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 822E9824: 817D005C  lwz r11, 0x5c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 822E9828: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E982C: 419A0034  beq cr6, 0x822e9860
	if ctx.cr[6].eq {
	pc = 0x822E9860; continue 'dispatch;
	}
	// 822E9830: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E9834: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E9838: 419A000C  beq cr6, 0x822e9844
	if ctx.cr[6].eq {
	pc = 0x822E9844; continue 'dispatch;
	}
	// 822E983C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E9840: 48BE4FC1  bl 0x82ece800
	ctx.lr = 0x822E9844;
	sub_82ECE800(ctx, base);
	// 822E9844: 809D005C  lwz r4, 0x5c(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(92 as u32) ) } as u64;
	// 822E9848: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 822E984C: 48BE3D25  bl 0x82ecd570
	ctx.lr = 0x822E9850;
	sub_82ECD570(ctx, base);
	// 822E9850: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E9854: 419A000C  beq cr6, 0x822e9860
	if ctx.cr[6].eq {
	pc = 0x822E9860; continue 'dispatch;
	}
	// 822E9858: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E985C: 48BE37AD  bl 0x82ecd008
	ctx.lr = 0x822E9860;
	sub_82ECD008(ctx, base);
	// 822E9860: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E9864: 809D0004  lwz r4, 4(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E9868: 48002FD9  bl 0x822ec840
	ctx.lr = 0x822E986C;
	sub_822EC840(ctx, base);
	// 822E986C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 822E9870: 48EBE94C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9878 size=12
    let mut pc: u32 = 0x822E9878;
    'dispatch: loop {
        match pc {
            0x822E9878 => {
    //   block [0x822E9878..0x822E9884)
	// 822E9878: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E987C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 822E9880: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9884(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9884 size=12
    let mut pc: u32 = 0x822E9884;
    'dispatch: loop {
        match pc {
            0x822E9884 => {
    //   block [0x822E9884..0x822E9890)
	// 822E9884: 89630061  lbz r11, 0x61(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(97 as u32) ) } as u64;
	// 822E9888: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 822E988C: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9890 size=8
    let mut pc: u32 = 0x822E9890;
    'dispatch: loop {
        match pc {
            0x822E9890 => {
    //   block [0x822E9890..0x822E9898)
	// 822E9890: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 822E9894: 484F0744  b 0x827d9fd8
	sub_827D9FD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E9898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x822E9898 size=4
    let mut pc: u32 = 0x822E9898;
    'dispatch: loop {
        match pc {
            0x822E9898 => {
    //   block [0x822E9898..0x822E989C)
	// 822E9898: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_822E98A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x822E98A0 size=96
    let mut pc: u32 = 0x822E98A0;
    'dispatch: loop {
        match pc {
            0x822E98A0 => {
    //   block [0x822E98A0..0x822E9900)
	// 822E98A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 822E98A4: 48EBE8C9  bl 0x831a816c
	ctx.lr = 0x822E98A8;
	sub_831A8130(ctx, base);
	// 822E98A8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 822E98AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 822E98B0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 822E98B4: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E98B8: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 822E98BC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E98C0: 419A000C  beq cr6, 0x822e98cc
	if ctx.cr[6].eq {
	pc = 0x822E98CC; continue 'dispatch;
	}
	// 822E98C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E98C8: 48BE4F39  bl 0x82ece800
	ctx.lr = 0x822E98CC;
	sub_82ECE800(ctx, base);
	// 822E98CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 822E98D0: 83DE0004  lwz r30, 4(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 822E98D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 822E98D8: 4BFFEFB1  bl 0x822e8888
	ctx.lr = 0x822E98DC;
	sub_822E8888(ctx, base);
	// 822E98DC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 822E98E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 822E98E4: 48BE27B5  bl 0x82ecc098
	ctx.lr = 0x822E98E8;
	sub_82ECC098(ctx, base);
	// 822E98E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 822E98EC: 419A000C  beq cr6, 0x822e98f8
	if ctx.cr[6].eq {
	pc = 0x822E98F8; continue 'dispatch;
	}
	// 822E98F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 822E98F4: 48BE3715  bl 0x82ecd008
	ctx.lr = 0x822E98F8;
	sub_82ECD008(ctx, base);
	// 822E98F8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 822E98FC: 48EBE8C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


