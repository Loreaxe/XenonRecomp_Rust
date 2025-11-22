pub fn sub_8317439C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317439C size=36
    let mut pc: u32 = 0x8317439C;
    'dispatch: loop {
        match pc {
            0x8317439C => {
    //   block [0x8317439C..0x831743C0)
	// 8317439C: 831743B8  lwz r24, 0x43b8(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(17336 as u32) ) } as u64;
	// 831743A0: 831743B8  lwz r24, 0x43b8(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(17336 as u32) ) } as u64;
	// 831743A4: 831743C0  lwz r24, 0x43c0(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(17344 as u32) ) } as u64;
	// 831743A8: 831743C0  lwz r24, 0x43c0(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(17344 as u32) ) } as u64;
	// 831743AC: 831743B8  lwz r24, 0x43b8(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(17336 as u32) ) } as u64;
	// 831743B0: 831743B8  lwz r24, 0x43b8(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(17336 as u32) ) } as u64;
	// 831743B4: 831743B8  lwz r24, 0x43b8(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(17336 as u32) ) } as u64;
	// 831743B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831743BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831743C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831743C0 size=8
    let mut pc: u32 = 0x831743C0;
    'dispatch: loop {
        match pc {
            0x831743C0 => {
    //   block [0x831743C0..0x831743C8)
	// 831743C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831743C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831743C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831743C8 size=24
    let mut pc: u32 = 0x831743C8;
    'dispatch: loop {
        match pc {
            0x831743C8 => {
    //   block [0x831743C8..0x831743E0)
	// 831743C8: 396303F4  addi r11, r3, 0x3f4
	ctx.r[11].s64 = ctx.r[3].s64 + 1012;
	// 831743CC: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 831743D0: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 831743D4: 90CB0008  stw r6, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 831743D8: 9163040C  stw r11, 0x40c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1036 as u32), ctx.r[11].u32 ) };
	// 831743DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831743E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831743E0 size=140
    let mut pc: u32 = 0x831743E0;
    'dispatch: loop {
        match pc {
            0x831743E0 => {
    //   block [0x831743E0..0x8317446C)
	// 831743E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831743E4: 48033D85  bl 0x831a8168
	ctx.lr = 0x831743E8;
	sub_831A8130(ctx, base);
	// 831743E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831743EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831743F0: 817D040C  lwz r11, 0x40c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1036 as u32) ) } as u64;
	// 831743F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831743F8: 409A0018  bne cr6, 0x83174410
	if !ctx.cr[6].eq {
	pc = 0x83174410; continue 'dispatch;
	}
	// 831743FC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83174400: 386B9EE4  addi r3, r11, -0x611c
	ctx.r[3].s64 = ctx.r[11].s64 + -24860;
	// 83174404: 48008D3D  bl 0x8317d140
	ctx.lr = 0x83174408;
	sub_8317D140(ctx, base);
	// 83174408: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317440C: 48033DAC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83174410: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 83174414: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83174418: 3BCA0003  addi r30, r10, 3
	ctx.r[30].s64 = ctx.r[10].s64 + 3;
	// 8317441C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83174420: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83174424: 7D7EF9D6  mullw r11, r30, r31
	ctx.r[11].s64 = (ctx.r[30].s32 as i64) * (ctx.r[31].s32 as i64);
	// 83174428: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317442C: 40980018  bge cr6, 0x83174444
	if !ctx.cr[6].lt {
	pc = 0x83174444; continue 'dispatch;
	}
	// 83174430: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83174434: 386B9EA8  addi r3, r11, -0x6158
	ctx.r[3].s64 = ctx.r[11].s64 + -24920;
	// 83174438: 48008D09  bl 0x8317d140
	ctx.lr = 0x8317443C;
	sub_8317D140(ctx, base);
	// 8317443C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83174440: 48033D78  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83174444: 4BFFF06D  bl 0x831734b0
	ctx.lr = 0x83174448;
	sub_831734B0(ctx, base);
	// 83174448: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317444C: 409A0018  bne cr6, 0x83174464
	if !ctx.cr[6].eq {
	pc = 0x83174464; continue 'dispatch;
	}
	// 83174450: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 83174454: 807D0048  lwz r3, 0x48(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 83174458: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8317445C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83174460: 48013FE1  bl 0x83188440
	ctx.lr = 0x83174464;
	sub_83188440(ctx, base);
	// 83174464: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83174468: 48033D50  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83174470 size=60
    let mut pc: u32 = 0x83174470;
    'dispatch: loop {
        match pc {
            0x83174470 => {
    //   block [0x83174470..0x831744AC)
	// 83174470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83174474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83174478: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317447C: 4801072D  bl 0x83184ba8
	ctx.lr = 0x83174480;
	sub_83184BA8(ctx, base);
	// 83174480: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83174484: 419A0018  beq cr6, 0x8317449c
	if ctx.cr[6].eq {
	pc = 0x8317449C; continue 'dispatch;
	}
	// 83174488: 3860FECE  li r3, -0x132
	ctx.r[3].s64 = -306;
	// 8317448C: 4BFFF13D  bl 0x831735c8
	ctx.lr = 0x83174490;
	sub_831735C8(ctx, base);
	// 83174490: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83174494: 386B9F1C  addi r3, r11, -0x60e4
	ctx.r[3].s64 = ctx.r[11].s64 + -24804;
	// 83174498: 48008CA9  bl 0x8317d140
	ctx.lr = 0x8317449C;
	sub_8317D140(ctx, base);
	// 8317449C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831744A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831744A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831744A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831744B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831744B0 size=316
    let mut pc: u32 = 0x831744B0;
    'dispatch: loop {
        match pc {
            0x831744B0 => {
    //   block [0x831744B0..0x831745EC)
	// 831744B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831744B4: 48033CB9  bl 0x831a816c
	ctx.lr = 0x831744B8;
	sub_831A8130(ctx, base);
	// 831744B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831744BC: 83E30048  lwz r31, 0x48(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 831744C0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 831744C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831744C8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 831744CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831744D0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 831744D4: 4800512D  bl 0x83179600
	ctx.lr = 0x831744D8;
	sub_83179600(ctx, base);
	// 831744D8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 831744DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831744E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831744E4: 4800511D  bl 0x83179600
	ctx.lr = 0x831744E8;
	sub_83179600(ctx, base);
	// 831744E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831744EC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831744F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831744F4: 4800510D  bl 0x83179600
	ctx.lr = 0x831744F8;
	sub_83179600(ctx, base);
	// 831744F8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 831744FC: 38800017  li r4, 0x17
	ctx.r[4].s64 = 23;
	// 83174500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174504: 480050FD  bl 0x83179600
	ctx.lr = 0x83174508;
	sub_83179600(ctx, base);
	// 83174508: 7D7EE9D6  mullw r11, r30, r29
	ctx.r[11].s64 = (ctx.r[30].s32 as i64) * (ctx.r[29].s32 as i64);
	// 8317450C: 1D6B03E8  mulli r11, r11, 0x3e8
	ctx.r[11].s64 = ctx.r[11].s64 * 1000;
	// 83174510: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 83174514: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 83174518: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 8317451C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 83174520: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83174524: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 83174528: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 8317452C: C00B9450  lfs f0, -0x6bb0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27568 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 83174530: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 83174534: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 83174538: 7DA057AE  stfiwx f13, 0, r10
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 8317453C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83174540: 7D6A07B4  extsw r10, r11
	ctx.r[10].s64 = ctx.r[11].s32 as i64;
	// 83174544: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 83174548: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 8317454C: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 83174550: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 83174554: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 83174558: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 8317455C: 41990008  bgt cr6, 0x83174564
	if ctx.cr[6].gt {
	pc = 0x83174564; continue 'dispatch;
	}
	// 83174560: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 83174564: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83174568: 3880002D  li r4, 0x2d
	ctx.r[4].s64 = 45;
	// 8317456C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174570: 48005091  bl 0x83179600
	ctx.lr = 0x83174574;
	sub_83179600(ctx, base);
	// 83174574: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83174578: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 8317457C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174580: 48005081  bl 0x83179600
	ctx.lr = 0x83174584;
	sub_83179600(ctx, base);
	// 83174584: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83174588: 3880002A  li r4, 0x2a
	ctx.r[4].s64 = 42;
	// 8317458C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174590: 48005071  bl 0x83179600
	ctx.lr = 0x83174594;
	sub_83179600(ctx, base);
	// 83174594: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 83174598: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8317459C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831745A0: 48005061  bl 0x83179600
	ctx.lr = 0x831745A4;
	sub_83179600(ctx, base);
	// 831745A4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831745A8: 38800033  li r4, 0x33
	ctx.r[4].s64 = 51;
	// 831745AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831745B0: 48005051  bl 0x83179600
	ctx.lr = 0x831745B4;
	sub_83179600(ctx, base);
	// 831745B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831745B8: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 831745BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831745C0: 48005041  bl 0x83179600
	ctx.lr = 0x831745C4;
	sub_83179600(ctx, base);
	// 831745C4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831745C8: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 831745CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831745D0: 48005031  bl 0x83179600
	ctx.lr = 0x831745D4;
	sub_83179600(ctx, base);
	// 831745D4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831745D8: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 831745DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831745E0: 48009D21  bl 0x8317e300
	ctx.lr = 0x831745E4;
	sub_8317E300(ctx, base);
	// 831745E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831745E8: 48033BD4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831745F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831745F0 size=168
    let mut pc: u32 = 0x831745F0;
    'dispatch: loop {
        match pc {
            0x831745F0 => {
    //   block [0x831745F0..0x83174698)
	// 831745F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831745F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831745F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831745FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83174600: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83174604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83174608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317460C: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83174610: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83174614: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83174618: 48010679  bl 0x83184c90
	ctx.lr = 0x8317461C;
	sub_83184C90(ctx, base);
	// 8317461C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83174620: 419A0018  beq cr6, 0x83174638
	if ctx.cr[6].eq {
	pc = 0x83174638; continue 'dispatch;
	}
	// 83174624: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83174628: 386B9F70  addi r3, r11, -0x6090
	ctx.r[3].s64 = ctx.r[11].s64 + -24720;
	// 8317462C: 48008B15  bl 0x8317d140
	ctx.lr = 0x83174630;
	sub_8317D140(ctx, base);
	// 83174630: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83174634: 4800004C  b 0x83174680
	pc = 0x83174680; continue 'dispatch;
	// 83174638: 3D608317  lis r11, -0x7ce9
	ctx.r[11].s64 = -2095644672;
	// 8317463C: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 83174640: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83174644: 388B3620  addi r4, r11, 0x3620
	ctx.r[4].s64 = ctx.r[11].s64 + 13856;
	// 83174648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317464C: 48012F75  bl 0x831875c0
	ctx.lr = 0x83174650;
	sub_831875C0(ctx, base);
	// 83174650: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83174654: 419A0020  beq cr6, 0x83174674
	if ctx.cr[6].eq {
	pc = 0x83174674; continue 'dispatch;
	}
	// 83174658: 3860FED1  li r3, -0x12f
	ctx.r[3].s64 = -303;
	// 8317465C: 4BFFEF6D  bl 0x831735c8
	ctx.lr = 0x83174660;
	sub_831735C8(ctx, base);
	// 83174660: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83174664: 386B9F3C  addi r3, r11, -0x60c4
	ctx.r[3].s64 = ctx.r[11].s64 + -24772;
	// 83174668: 48008AD9  bl 0x8317d140
	ctx.lr = 0x8317466C;
	sub_8317D140(ctx, base);
	// 8317466C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83174670: 48000010  b 0x83174680
	pc = 0x83174680; continue 'dispatch;
	// 83174674: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174678: 4BFFFD69  bl 0x831743e0
	ctx.lr = 0x8317467C;
	sub_831743E0(ctx, base);
	// 8317467C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83174680: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83174684: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83174688: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317468C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83174690: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83174694: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83174698 size=16
    let mut pc: u32 = 0x83174698;
    'dispatch: loop {
        match pc {
            0x83174698 => {
    //   block [0x83174698..0x831746A8)
	// 83174698: 80A30460  lwz r5, 0x460(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1120 as u32) ) } as u64;
	// 8317469C: 8083045C  lwz r4, 0x45c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1116 as u32) ) } as u64;
	// 831746A0: 80630458  lwz r3, 0x458(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1112 as u32) ) } as u64;
	// 831746A4: 4BFC012C  b 0x831347d0
	sub_831347D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831746A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831746A8 size=96
    let mut pc: u32 = 0x831746A8;
    'dispatch: loop {
        match pc {
            0x831746A8 => {
    //   block [0x831746A8..0x83174708)
	// 831746A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831746AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831746B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831746B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831746B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831746BC: 4BFFECCD  bl 0x83173388
	ctx.lr = 0x831746C0;
	sub_83173388(ctx, base);
	// 831746C0: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 831746C4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831746C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831746CC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831746D0: 409A0024  bne cr6, 0x831746f4
	if !ctx.cr[6].eq {
	pc = 0x831746F4; continue 'dispatch;
	}
	// 831746D4: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 831746D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831746DC: 409A0008  bne cr6, 0x831746e4
	if !ctx.cr[6].eq {
	pc = 0x831746E4; continue 'dispatch;
	}
	// 831746E0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831746E4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 831746E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831746EC: 409A0008  bne cr6, 0x831746f4
	if !ctx.cr[6].eq {
	pc = 0x831746F4; continue 'dispatch;
	}
	// 831746F0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831746F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831746F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831746FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83174700: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83174704: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83174708 size=84
    let mut pc: u32 = 0x83174708;
    'dispatch: loop {
        match pc {
            0x83174708 => {
    //   block [0x83174708..0x8317475C)
	// 83174708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317470C: 48033A61  bl 0x831a816c
	ctx.lr = 0x83174710;
	sub_831A8130(ctx, base);
	// 83174710: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83174714: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83174718: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317471C: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83174720: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83174724: 917F0480  stw r11, 0x480(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1152 as u32), ctx.r[11].u32 ) };
	// 83174728: 817E001C  lwz r11, 0x1c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 8317472C: 917F0484  stw r11, 0x484(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1156 as u32), ctx.r[11].u32 ) };
	// 83174730: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 83174734: 93BF048C  stw r29, 0x48c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1164 as u32), ctx.r[29].u32 ) };
	// 83174738: 917F0488  stw r11, 0x488(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1160 as u32), ctx.r[11].u32 ) };
	// 8317473C: 48013075  bl 0x831877b0
	ctx.lr = 0x83174740;
	sub_831877B0(ctx, base);
	// 83174740: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83174744: 396B0012  addi r11, r11, 0x12
	ctx.r[11].s64 = ctx.r[11].s64 + 18;
	// 83174748: 917F0490  stw r11, 0x490(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1168 as u32), ctx.r[11].u32 ) };
	// 8317474C: 93BF0498  stw r29, 0x498(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1176 as u32), ctx.r[29].u32 ) };
	// 83174750: 93BF049C  stw r29, 0x49c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1180 as u32), ctx.r[29].u32 ) };
	// 83174754: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83174758: 48033A64  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83174760 size=68
    let mut pc: u32 = 0x83174760;
    'dispatch: loop {
        match pc {
            0x83174760 => {
    //   block [0x83174760..0x831747A4)
	// 83174760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83174764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83174768: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317476C: 81630024  lwz r11, 0x24(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 83174770: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83174774: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83174778: 419A001C  beq cr6, 0x83174794
	if ctx.cr[6].eq {
	pc = 0x83174794; continue 'dispatch;
	}
	// 8317477C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83174780: 419A0014  beq cr6, 0x83174794
	if ctx.cr[6].eq {
	pc = 0x83174794; continue 'dispatch;
	}
	// 83174784: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83174788: 386B9FA0  addi r3, r11, -0x6060
	ctx.r[3].s64 = ctx.r[11].s64 + -24672;
	// 8317478C: 480089B5  bl 0x8317d140
	ctx.lr = 0x83174790;
	sub_8317D140(ctx, base);
	// 83174790: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83174794: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83174798: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317479C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831747A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831747A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831747A8 size=216
    let mut pc: u32 = 0x831747A8;
    'dispatch: loop {
        match pc {
            0x831747A8 => {
    //   block [0x831747A8..0x83174880)
	// 831747A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831747AC: 480339C1  bl 0x831a816c
	ctx.lr = 0x831747B0;
	sub_831A8130(ctx, base);
	// 831747B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831747B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831747B8: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 831747BC: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 831747C0: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 831747C4: 480023D5  bl 0x83176b98
	ctx.lr = 0x831747C8;
	sub_83176B98(ctx, base);
	// 831747C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831747CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831747D0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831747D4: 48013015  bl 0x831877e8
	ctx.lr = 0x831747D8;
	sub_831877E8(ctx, base);
	// 831747D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831747DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831747E0: 409A0024  bne cr6, 0x83174804
	if !ctx.cr[6].eq {
	pc = 0x83174804; continue 'dispatch;
	}
	// 831747E4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831747E8: 386B9FEC  addi r3, r11, -0x6014
	ctx.r[3].s64 = ctx.r[11].s64 + -24596;
	// 831747EC: 48008955  bl 0x8317d140
	ctx.lr = 0x831747F0;
	sub_8317D140(ctx, base);
	// 831747F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831747F4: 480130F5  bl 0x831878e8
	ctx.lr = 0x831747F8;
	sub_831878E8(ctx, base);
	// 831747F8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 831747FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83174800: 480339BC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83174804: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83174808: 917F00BC  stw r11, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 8317480C: 93BF00C0  stw r29, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[29].u32 ) };
	// 83174810: 48002771  bl 0x83176f80
	ctx.lr = 0x83174814;
	sub_83176F80(ctx, base);
	// 83174814: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83174818: 409A0050  bne cr6, 0x83174868
	if !ctx.cr[6].eq {
	pc = 0x83174868; continue 'dispatch;
	}
	// 8317481C: 3C800002  lis r4, 2
	ctx.r[4].s64 = 131072;
	// 83174820: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174824: 48012FC5  bl 0x831877e8
	ctx.lr = 0x83174828;
	sub_831877E8(ctx, base);
	// 83174828: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317482C: 409A0024  bne cr6, 0x83174850
	if !ctx.cr[6].eq {
	pc = 0x83174850; continue 'dispatch;
	}
	// 83174830: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83174834: 386B9FC4  addi r3, r11, -0x603c
	ctx.r[3].s64 = ctx.r[11].s64 + -24636;
	// 83174838: 48008909  bl 0x8317d140
	ctx.lr = 0x8317483C;
	sub_8317D140(ctx, base);
	// 8317483C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174840: 480130A9  bl 0x831878e8
	ctx.lr = 0x83174844;
	sub_831878E8(ctx, base);
	// 83174844: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83174848: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317484C: 48033970  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83174850: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 83174854: 907F0414  stw r3, 0x414(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1044 as u32), ctx.r[3].u32 ) };
	// 83174858: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317485C: 917F0418  stw r11, 0x418(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1048 as u32), ctx.r[11].u32 ) };
	// 83174860: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83174864: 48033958  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83174868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317486C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83174870: 917F0414  stw r11, 0x414(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1044 as u32), ctx.r[11].u32 ) };
	// 83174874: 917F0418  stw r11, 0x418(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1048 as u32), ctx.r[11].u32 ) };
	// 83174878: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317487C: 48033940  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83174880 size=444
    let mut pc: u32 = 0x83174880;
    'dispatch: loop {
        match pc {
            0x83174880 => {
    //   block [0x83174880..0x83174A3C)
	// 83174880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83174884: 480338C9  bl 0x831a814c
	ctx.lr = 0x83174888;
	sub_831A8130(ctx, base);
	// 83174888: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317488C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83174890: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83174894: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 83174898: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8317489C: 419A018C  beq cr6, 0x83174a28
	if ctx.cr[6].eq {
	pc = 0x83174A28; continue 'dispatch;
	}
	// 831748A0: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 831748A4: 419A0184  beq cr6, 0x83174a28
	if ctx.cr[6].eq {
	pc = 0x83174A28; continue 'dispatch;
	}
	// 831748A8: 38A00204  li r5, 0x204
	ctx.r[5].s64 = 516;
	// 831748AC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831748B0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 831748B4: 4803392D  bl 0x831a81e0
	ctx.lr = 0x831748B8;
	sub_831A81E0(ctx, base);
	// 831748B8: 2F1F0800  cmpwi cr6, r31, 0x800
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2048, &mut ctx.xer);
	// 831748BC: 40980018  bge cr6, 0x831748d4
	if !ctx.cr[6].lt {
	pc = 0x831748D4; continue 'dispatch;
	}
	// 831748C0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831748C4: 386BA064  addi r3, r11, -0x5f9c
	ctx.r[3].s64 = ctx.r[11].s64 + -24476;
	// 831748C8: 48008879  bl 0x8317d140
	ctx.lr = 0x831748CC;
	sub_8317D140(ctx, base);
	// 831748CC: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 831748D0: 480338CC  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 831748D4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831748D8: 3AC00002  li r22, 2
	ctx.r[22].s64 = 2;
	// 831748DC: 3AFE0800  addi r23, r30, 0x800
	ctx.r[23].s64 = ctx.r[30].s64 + 2048;
	// 831748E0: 3B1FF800  addi r24, r31, -0x800
	ctx.r[24].s64 = ctx.r[31].s64 + -2048;
	// 831748E4: 3AABA040  addi r21, r11, -0x5fc0
	ctx.r[21].s64 = ctx.r[11].s64 + -24512;
	// 831748E8: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 831748EC: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 831748F0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 831748F4: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 831748F8: 48014511  bl 0x83188e08
	ctx.lr = 0x831748FC;
	sub_83188E08(ctx, base);
	// 831748FC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83174900: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83174904: 409A0010  bne cr6, 0x83174914
	if !ctx.cr[6].eq {
	pc = 0x83174914; continue 'dispatch;
	}
	// 83174908: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 8317490C: 48008835  bl 0x8317d140
	ctx.lr = 0x83174910;
	sub_8317D140(ctx, base);
	// 83174910: 480000FC  b 0x83174a0c
	pc = 0x83174A0C; continue 'dispatch;
	// 83174914: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83174918: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317491C: 4801459D  bl 0x83188eb8
	ctx.lr = 0x83174920;
	sub_83188EB8(ctx, base);
	// 83174920: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83174924: 409A00D4  bne cr6, 0x831749f8
	if !ctx.cr[6].eq {
	pc = 0x831749F8; continue 'dispatch;
	}
	// 83174928: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317492C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83174930: 409A00C8  bne cr6, 0x831749f8
	if !ctx.cr[6].eq {
	pc = 0x831749F8; continue 'dispatch;
	}
	// 83174934: 7F7CDB78  mr r28, r27
	ctx.r[28].u64 = ctx.r[27].u64;
	// 83174938: 578B2036  slwi r11, r28, 4
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317493C: 395C00C0  addi r10, r28, 0xc0
	ctx.r[10].s64 = ctx.r[28].s64 + 192;
	// 83174940: 7FEBD214  add r31, r11, r26
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 83174944: 555E063E  clrlwi r30, r10, 0x18
	ctx.r[30].u64 = ctx.r[10].u32 as u64 & 0x000000FFu64;
	// 83174948: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8317494C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83174950: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83174954: 937F0004  stw r27, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[27].u32 ) };
	// 83174958: 937F0008  stw r27, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[27].u32 ) };
	// 8317495C: 937F000C  stw r27, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[27].u32 ) };
	// 83174960: 937F0010  stw r27, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 83174964: 48013DBD  bl 0x83188720
	ctx.lr = 0x83174968;
	sub_83188720(ctx, base);
	// 83174968: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317496C: 409A0078  bne cr6, 0x831749e4
	if !ctx.cr[6].eq {
	pc = 0x831749E4; continue 'dispatch;
	}
	// 83174970: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83174974: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83174978: 409A006C  bne cr6, 0x831749e4
	if !ctx.cr[6].eq {
	pc = 0x831749E4; continue 'dispatch;
	}
	// 8317497C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83174980: 933F0004  stw r25, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[25].u32 ) };
	// 83174984: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83174988: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317498C: 48014015  bl 0x831889a0
	ctx.lr = 0x83174990;
	sub_831889A0(ctx, base);
	// 83174990: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83174994: 409A0010  bne cr6, 0x831749a4
	if !ctx.cr[6].eq {
	pc = 0x831749A4; continue 'dispatch;
	}
	// 83174998: 80610058  lwz r3, 0x58(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8317499C: 48003BB5  bl 0x83178550
	ctx.lr = 0x831749A0;
	sub_83178550(ctx, base);
	// 831749A0: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 831749A4: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 831749A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831749AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831749B0: 48014041  bl 0x831889f0
	ctx.lr = 0x831749B4;
	sub_831889F0(ctx, base);
	// 831749B4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831749B8: 409A000C  bne cr6, 0x831749c4
	if !ctx.cr[6].eq {
	pc = 0x831749C4; continue 'dispatch;
	}
	// 831749BC: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 831749C0: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 831749C4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 831749C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831749CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831749D0: 48014049  bl 0x83188a18
	ctx.lr = 0x831749D4;
	sub_83188A18(ctx, base);
	// 831749D4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831749D8: 409A000C  bne cr6, 0x831749e4
	if !ctx.cr[6].eq {
	pc = 0x831749E4; continue 'dispatch;
	}
	// 831749DC: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 831749E0: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 831749E4: 397C0001  addi r11, r28, 1
	ctx.r[11].s64 = ctx.r[28].s64 + 1;
	// 831749E8: 557C063E  clrlwi r28, r11, 0x18
	ctx.r[28].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 831749EC: 2B1C0020  cmplwi cr6, r28, 0x20
	ctx.cr[6].compare_u32(ctx.r[28].u32, 32 as u32, &mut ctx.xer);
	// 831749F0: 4198FF48  blt cr6, 0x83174938
	if ctx.cr[6].lt {
	pc = 0x83174938; continue 'dispatch;
	}
	// 831749F4: 933A0000  stw r25, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[25].u32 ) };
	// 831749F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831749FC: 48013CED  bl 0x831886e8
	ctx.lr = 0x83174A00;
	sub_831886E8(ctx, base);
	// 83174A00: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 83174A04: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83174A08: 419A002C  beq cr6, 0x83174a34
	if ctx.cr[6].eq {
	pc = 0x83174A34; continue 'dispatch;
	}
	// 83174A0C: 3AD60001  addi r22, r22, 1
	ctx.r[22].s64 = ctx.r[22].s64 + 1;
	// 83174A10: 3B18F800  addi r24, r24, -0x800
	ctx.r[24].s64 = ctx.r[24].s64 + -2048;
	// 83174A14: 3AF70800  addi r23, r23, 0x800
	ctx.r[23].s64 = ctx.r[23].s64 + 2048;
	// 83174A18: 2F160003  cmpwi cr6, r22, 3
	ctx.cr[6].compare_i32(ctx.r[22].s32, 3, &mut ctx.xer);
	// 83174A1C: 4099FED4  ble cr6, 0x831748f0
	if !ctx.cr[6].gt {
	pc = 0x831748F0; continue 'dispatch;
	}
	// 83174A20: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 83174A24: 48033778  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
	// 83174A28: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83174A2C: 386BA010  addi r3, r11, -0x5ff0
	ctx.r[3].s64 = ctx.r[11].s64 + -24560;
	// 83174A30: 48008711  bl 0x8317d140
	ctx.lr = 0x83174A34;
	sub_8317D140(ctx, base);
	// 83174A34: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 83174A38: 48033764  b 0x831a819c
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83174A40 size=32
    let mut pc: u32 = 0x83174A40;
    'dispatch: loop {
        match pc {
            0x83174A40 => {
    //   block [0x83174A40..0x83174A60)
	// 83174A40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83174A44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83174A48: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83174A4C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83174A50: 409A0024  bne cr6, 0x83174a74
	if !ctx.cr[6].eq {
		sub_83174A74(ctx, base);
		return;
	}
	// 83174A54: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83174A58: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83174A5C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83174A60 size=12
    let mut pc: u32 = 0x83174A60;
    'dispatch: loop {
        match pc {
            0x83174A60 => {
    //   block [0x83174A60..0x83174A6C)
	// 83174A60: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83174A64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83174A68: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174A6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83174A6C size=8
    let mut pc: u32 = 0x83174A6C;
    'dispatch: loop {
        match pc {
            0x83174A6C => {
    //   block [0x83174A6C..0x83174A74)
	// 83174A6C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83174A70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174A74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83174A74 size=24
    let mut pc: u32 = 0x83174A74;
    'dispatch: loop {
        match pc {
            0x83174A74 => {
    //   block [0x83174A74..0x83174A8C)
	// 83174A74: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83174A78: 216B0000  subfic r11, r11, 0
	ctx.xer.ca = ctx.r[11].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[11].s64;
	// 83174A7C: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 83174A80: 556B003C  rlwinm r11, r11, 0, 0, 0x1e
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83174A84: 386B0003  addi r3, r11, 3
	ctx.r[3].s64 = ctx.r[11].s64 + 3;
	// 83174A88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83174A90 size=20
    let mut pc: u32 = 0x83174A90;
    'dispatch: loop {
        match pc {
            0x83174A90 => {
    //   block [0x83174A90..0x83174AA4)
	// 83174A90: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83174A94: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83174A98: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83174A9C: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 83174AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83174AA8 size=32
    let mut pc: u32 = 0x83174AA8;
    'dispatch: loop {
        match pc {
            0x83174AA8 => {
    //   block [0x83174AA8..0x83174AC8)
	// 83174AA8: 39630480  addi r11, r3, 0x480
	ctx.r[11].s64 = ctx.r[3].s64 + 1152;
	// 83174AAC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83174AB0: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 83174AB4: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 83174AB8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 83174ABC: 4099000C  ble cr6, 0x83174ac8
	if !ctx.cr[6].gt {
		sub_83174AC8(ctx, base);
		return;
	}
	// 83174AC0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83174AC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83174AC8 size=20
    let mut pc: u32 = 0x83174AC8;
    'dispatch: loop {
        match pc {
            0x83174AC8 => {
    //   block [0x83174AC8..0x83174ADC)
	// 83174AC8: 806B0008  lwz r3, 8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 83174ACC: 914B000C  stw r10, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 83174AD0: 7D432214  add r10, r3, r4
	ctx.r[10].u64 = ctx.r[3].u64 + ctx.r[4].u64;
	// 83174AD4: 914B0008  stw r10, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 83174AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83174AE0 size=68
    let mut pc: u32 = 0x83174AE0;
    'dispatch: loop {
        match pc {
            0x83174AE0 => {
    //   block [0x83174AE0..0x83174B24)
	// 83174AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83174AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83174AE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83174AEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83174AF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83174AF4: 4BFFE895  bl 0x83173388
	ctx.lr = 0x83174AF8;
	sub_83173388(ctx, base);
	// 83174AF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83174AFC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83174B00: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 83174B04: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 83174B08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83174B0C: 4E800421  bctrl
	ctx.lr = 0x83174B10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83174B10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83174B14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83174B18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83174B1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83174B20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83174B28 size=68
    let mut pc: u32 = 0x83174B28;
    'dispatch: loop {
        match pc {
            0x83174B28 => {
    //   block [0x83174B28..0x83174B6C)
	// 83174B28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83174B2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83174B30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83174B34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83174B38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83174B3C: 4BFFE84D  bl 0x83173388
	ctx.lr = 0x83174B40;
	sub_83173388(ctx, base);
	// 83174B40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83174B44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83174B48: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 83174B4C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 83174B50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83174B54: 4E800421  bctrl
	ctx.lr = 0x83174B58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83174B58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83174B5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83174B60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83174B64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83174B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83174B70 size=144
    let mut pc: u32 = 0x83174B70;
    'dispatch: loop {
        match pc {
            0x83174B70 => {
    //   block [0x83174B70..0x83174C00)
	// 83174B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83174B74: 480335F1  bl 0x831a8164
	ctx.lr = 0x83174B78;
	sub_831A8130(ctx, base);
	// 83174B78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83174B7C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83174B80: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83174B84: 3BAB9AA8  addi r29, r11, -0x6558
	ctx.r[29].s64 = ctx.r[11].s64 + -25944;
	// 83174B88: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83174B8C: 3BDDFFD8  addi r30, r29, -0x28
	ctx.r[30].s64 = ctx.r[29].s64 + -40;
	// 83174B90: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 83174B94: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 83174B98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174B9C: 4BFBA2F5  bl 0x8312ee90
	ctx.lr = 0x83174BA0;
	sub_8312EE90(ctx, base);
	// 83174BA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83174BA4: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 83174BA8: 419A0028  beq cr6, 0x83174bd0
	if ctx.cr[6].eq {
	pc = 0x83174BD0; continue 'dispatch;
	}
	// 83174BAC: 3BFF0020  addi r31, r31, 0x20
	ctx.r[31].s64 = ctx.r[31].s64 + 32;
	// 83174BB0: 397D0100  addi r11, r29, 0x100
	ctx.r[11].s64 = ctx.r[29].s64 + 256;
	// 83174BB4: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 83174BB8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83174BBC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83174BC0: 4198FFD4  blt cr6, 0x83174b94
	if ctx.cr[6].lt {
	pc = 0x83174B94; continue 'dispatch;
	}
	// 83174BC4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83174BC8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83174BCC: 480335E8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83174BD0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 83174BD4: 40990020  ble cr6, 0x83174bf4
	if !ctx.cr[6].gt {
	pc = 0x83174BF4; continue 'dispatch;
	}
	// 83174BD8: 3BFDFFD8  addi r31, r29, -0x28
	ctx.r[31].s64 = ctx.r[29].s64 + -40;
	// 83174BDC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83174BE0: 4BFBA391  bl 0x8312ef70
	ctx.lr = 0x83174BE4;
	sub_8312EF70(ctx, base);
	// 83174BE4: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 83174BE8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 83174BEC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 83174BF0: 409AFFEC  bne cr6, 0x83174bdc
	if !ctx.cr[6].eq {
	pc = 0x83174BDC; continue 'dispatch;
	}
	// 83174BF4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83174BF8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83174BFC: 480335B8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83174C00 size=76
    let mut pc: u32 = 0x83174C00;
    'dispatch: loop {
        match pc {
            0x83174C00 => {
    //   block [0x83174C00..0x83174C4C)
	// 83174C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83174C04: 48033569  bl 0x831a816c
	ctx.lr = 0x83174C08;
	sub_831A8130(ctx, base);
	// 83174C08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83174C0C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83174C10: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83174C14: 3BCB9A80  addi r30, r11, -0x6580
	ctx.r[30].s64 = ctx.r[11].s64 + -25984;
	// 83174C18: 7FDFF378  mr r31, r30
	ctx.r[31].u64 = ctx.r[30].u64;
	// 83174C1C: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83174C20: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83174C24: 419A000C  beq cr6, 0x83174c30
	if ctx.cr[6].eq {
	pc = 0x83174C30; continue 'dispatch;
	}
	// 83174C28: 4BFBA349  bl 0x8312ef70
	ctx.lr = 0x83174C2C;
	sub_8312EF70(ctx, base);
	// 83174C2C: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83174C30: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 83174C34: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
	// 83174C38: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83174C3C: 4198FFE0  blt cr6, 0x83174c1c
	if ctx.cr[6].lt {
	pc = 0x83174C1C; continue 'dispatch;
	}
	// 83174C40: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83174C44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83174C48: 48033574  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83174C50 size=16
    let mut pc: u32 = 0x83174C50;
    'dispatch: loop {
        match pc {
            0x83174C50 => {
    //   block [0x83174C50..0x83174C60)
	// 83174C50: 8163053C  lwz r11, 0x53c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1340 as u32) ) } as u64;
	// 83174C54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83174C58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83174C5C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83174C60 size=8
    let mut pc: u32 = 0x83174C60;
    'dispatch: loop {
        match pc {
            0x83174C60 => {
    //   block [0x83174C60..0x83174C68)
	// 83174C60: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83174C64: 4BFBA39C  b 0x8312f000
	sub_8312F000(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83174C68 size=4
    let mut pc: u32 = 0x83174C68;
    'dispatch: loop {
        match pc {
            0x83174C68 => {
    //   block [0x83174C68..0x83174C6C)
	// 83174C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83174C70 size=16
    let mut pc: u32 = 0x83174C70;
    'dispatch: loop {
        match pc {
            0x83174C70 => {
    //   block [0x83174C70..0x83174C80)
	// 83174C70: 8163053C  lwz r11, 0x53c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1340 as u32) ) } as u64;
	// 83174C74: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83174C78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83174C7C: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174C80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83174C80 size=8
    let mut pc: u32 = 0x83174C80;
    'dispatch: loop {
        match pc {
            0x83174C80 => {
    //   block [0x83174C80..0x83174C88)
	// 83174C80: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83174C84: 4BFBA414  b 0x8312f098
	sub_8312F098(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83174C88 size=4
    let mut pc: u32 = 0x83174C88;
    'dispatch: loop {
        match pc {
            0x83174C88 => {
    //   block [0x83174C88..0x83174C8C)
	// 83174C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83174C90 size=208
    let mut pc: u32 = 0x83174C90;
    'dispatch: loop {
        match pc {
            0x83174C90 => {
    //   block [0x83174C90..0x83174D60)
	// 83174C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83174C94: 480334D1  bl 0x831a8164
	ctx.lr = 0x83174C98;
	sub_831A8130(ctx, base);
	// 83174C98: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83174C9C: 83E30028  lwz r31, 0x28(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 83174CA0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 83174CA4: 83A3002C  lwz r29, 0x2c(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 83174CA8: 83C30030  lwz r30, 0x30(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 83174CAC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83174CB0: 83830008  lwz r28, 8(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83174CB4: 8363000C  lwz r27, 0xc(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83174CB8: 41990020  bgt cr6, 0x83174cd8
	if ctx.cr[6].gt {
	pc = 0x83174CD8; continue 'dispatch;
	}
	// 83174CBC: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83174CC0: 41990018  bgt cr6, 0x83174cd8
	if ctx.cr[6].gt {
	pc = 0x83174CD8; continue 'dispatch;
	}
	// 83174CC4: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83174CC8: 409A0010  bne cr6, 0x83174cd8
	if !ctx.cr[6].eq {
	pc = 0x83174CD8; continue 'dispatch;
	}
	// 83174CCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83174CD0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83174CD4: 480334E0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 83174CD8: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 83174CDC: 4199001C  bgt cr6, 0x83174cf8
	if ctx.cr[6].gt {
	pc = 0x83174CF8; continue 'dispatch;
	}
	// 83174CE0: 2F1F0010  cmpwi cr6, r31, 0x10
	ctx.cr[6].compare_i32(ctx.r[31].s32, 16, &mut ctx.xer);
	// 83174CE4: 40990014  ble cr6, 0x83174cf8
	if !ctx.cr[6].gt {
	pc = 0x83174CF8; continue 'dispatch;
	}
	// 83174CE8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83174CEC: 386BA0E4  addi r3, r11, -0x5f1c
	ctx.r[3].s64 = ctx.r[11].s64 + -24348;
	// 83174CF0: 48008451  bl 0x8317d140
	ctx.lr = 0x83174CF4;
	sub_8317D140(ctx, base);
	// 83174CF4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83174CF8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83174CFC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83174D00: 4BFFF2C1  bl 0x83173fc0
	ctx.lr = 0x83174D04;
	sub_83173FC0(ctx, base);
	// 83174D04: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 83174D08: 40980014  bge cr6, 0x83174d1c
	if !ctx.cr[6].lt {
	pc = 0x83174D1C; continue 'dispatch;
	}
	// 83174D0C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83174D10: 386BA0C0  addi r3, r11, -0x5f40
	ctx.r[3].s64 = ctx.r[11].s64 + -24384;
	// 83174D14: 4800842D  bl 0x8317d140
	ctx.lr = 0x83174D18;
	sub_8317D140(ctx, base);
	// 83174D18: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83174D1C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 83174D20: 40990034  ble cr6, 0x83174d54
	if !ctx.cr[6].gt {
	pc = 0x83174D54; continue 'dispatch;
	}
	// 83174D24: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83174D28: 3BABA09C  addi r29, r11, -0x5f64
	ctx.r[29].s64 = ctx.r[11].s64 + -24420;
	// 83174D2C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83174D30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83174D34: 409A0010  bne cr6, 0x83174d44
	if !ctx.cr[6].eq {
	pc = 0x83174D44; continue 'dispatch;
	}
	// 83174D38: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83174D3C: 48008405  bl 0x8317d140
	ctx.lr = 0x83174D40;
	sub_8317D140(ctx, base);
	// 83174D40: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83174D44: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 83174D48: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 83174D4C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83174D50: 409AFFDC  bne cr6, 0x83174d2c
	if !ctx.cr[6].eq {
	pc = 0x83174D2C; continue 'dispatch;
	}
	// 83174D54: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 83174D58: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83174D5C: 48033458  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83174D60 size=348
    let mut pc: u32 = 0x83174D60;
    'dispatch: loop {
        match pc {
            0x83174D60 => {
    //   block [0x83174D60..0x83174EBC)
	// 83174D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83174D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83174D68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83174D6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83174D70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83174D74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83174D78: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83174D7C: 419A0128  beq cr6, 0x83174ea4
	if ctx.cr[6].eq {
	pc = 0x83174EA4; continue 'dispatch;
	}
	// 83174D80: 4BFFFED1  bl 0x83174c50
	ctx.lr = 0x83174D84;
	sub_83174C50(ctx, base);
	// 83174D84: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83174D88: 48008C09  bl 0x8317d990
	ctx.lr = 0x83174D8C;
	sub_8317D990(ctx, base);
	// 83174D8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174D90: 480014A1  bl 0x83176230
	ctx.lr = 0x83174D94;
	sub_83176230(ctx, base);
	// 83174D94: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83174D98: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83174D9C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83174DA0: 48008BF1  bl 0x8317d990
	ctx.lr = 0x83174DA4;
	sub_8317D990(ctx, base);
	// 83174DA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174DA8: 48002261  bl 0x83177008
	ctx.lr = 0x83174DAC;
	sub_83177008(ctx, base);
	// 83174DAC: 807F00B8  lwz r3, 0xb8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 83174DB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83174DB4: 419A0008  beq cr6, 0x83174dbc
	if ctx.cr[6].eq {
	pc = 0x83174DBC; continue 'dispatch;
	}
	// 83174DB8: 48001DF1  bl 0x83176ba8
	ctx.lr = 0x83174DBC;
	sub_83176BA8(ctx, base);
	// 83174DBC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83174DC0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83174DC4: 419A0008  beq cr6, 0x83174dcc
	if ctx.cr[6].eq {
	pc = 0x83174DCC; continue 'dispatch;
	}
	// 83174DC8: 4BFC0E91  bl 0x83135c58
	ctx.lr = 0x83174DCC;
	sub_83135C58(ctx, base);
	// 83174DCC: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83174DD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83174DD4: 419A0008  beq cr6, 0x83174ddc
	if ctx.cr[6].eq {
	pc = 0x83174DDC; continue 'dispatch;
	}
	// 83174DD8: 480065D9  bl 0x8317b3b0
	ctx.lr = 0x83174DDC;
	sub_8317B3B0(ctx, base);
	// 83174DDC: 807F0454  lwz r3, 0x454(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1108 as u32) ) } as u64;
	// 83174DE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83174DE4: 419A0014  beq cr6, 0x83174df8
	if ctx.cr[6].eq {
	pc = 0x83174DF8; continue 'dispatch;
	}
	// 83174DE8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83174DEC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83174DF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83174DF4: 4E800421  bctrl
	ctx.lr = 0x83174DF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83174DF8: 807F0474  lwz r3, 0x474(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1140 as u32) ) } as u64;
	// 83174DFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83174E00: 419A0014  beq cr6, 0x83174e14
	if ctx.cr[6].eq {
	pc = 0x83174E14; continue 'dispatch;
	}
	// 83174E04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83174E08: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83174E0C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83174E10: 4E800421  bctrl
	ctx.lr = 0x83174E14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83174E14: 807F04F4  lwz r3, 0x4f4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1268 as u32) ) } as u64;
	// 83174E18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83174E1C: 419A0008  beq cr6, 0x83174e24
	if ctx.cr[6].eq {
	pc = 0x83174E24; continue 'dispatch;
	}
	// 83174E20: 48026A61  bl 0x8319b880
	ctx.lr = 0x83174E24;
	sub_8319B880(ctx, base);
	// 83174E24: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83174E28: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83174E2C: 419A0008  beq cr6, 0x83174e34
	if ctx.cr[6].eq {
	pc = 0x83174E34; continue 'dispatch;
	}
	// 83174E30: 4BFFF641  bl 0x83174470
	ctx.lr = 0x83174E34;
	sub_83174470(ctx, base);
	// 83174E34: 387F04A0  addi r3, r31, 0x4a0
	ctx.r[3].s64 = ctx.r[31].s64 + 1184;
	// 83174E38: 4BFFE4C1  bl 0x831732f8
	ctx.lr = 0x83174E3C;
	sub_831732F8(ctx, base);
	// 83174E3C: 387F04C4  addi r3, r31, 0x4c4
	ctx.r[3].s64 = ctx.r[31].s64 + 1220;
	// 83174E40: 4BFFE4B9  bl 0x831732f8
	ctx.lr = 0x83174E44;
	sub_831732F8(ctx, base);
	// 83174E44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174E48: 48012AA1  bl 0x831878e8
	ctx.lr = 0x83174E4C;
	sub_831878E8(ctx, base);
	// 83174E4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174E50: 48012991  bl 0x831877e0
	ctx.lr = 0x83174E54;
	sub_831877E0(ctx, base);
	// 83174E54: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83174E58: 419A0010  beq cr6, 0x83174e68
	if ctx.cr[6].eq {
	pc = 0x83174E68; continue 'dispatch;
	}
	// 83174E5C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83174E60: 386BA108  addi r3, r11, -0x5ef8
	ctx.r[3].s64 = ctx.r[11].s64 + -24312;
	// 83174E64: 480082DD  bl 0x8317d140
	ctx.lr = 0x83174E68;
	sub_8317D140(ctx, base);
	// 83174E68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174E6C: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 83174E70: 4BFFFE01  bl 0x83174c70
	ctx.lr = 0x83174E74;
	sub_83174C70(ctx, base);
	// 83174E74: 807F053C  lwz r3, 0x53c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1340 as u32) ) } as u64;
	// 83174E78: 4BFBA189  bl 0x8312f000
	ctx.lr = 0x83174E7C;
	sub_8312F000(ctx, base);
	// 83174E7C: 83DF053C  lwz r30, 0x53c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1340 as u32) ) } as u64;
	// 83174E80: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83174E84: 41980020  blt cr6, 0x83174ea4
	if ctx.cr[6].lt {
	pc = 0x83174EA4; continue 'dispatch;
	}
	// 83174E88: 38A00540  li r5, 0x540
	ctx.r[5].s64 = 1344;
	// 83174E8C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83174E90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174E94: 4803334D  bl 0x831a81e0
	ctx.lr = 0x83174E98;
	sub_831A81E0(ctx, base);
	// 83174E98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83174E9C: 93DF053C  stw r30, 0x53c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1340 as u32), ctx.r[30].u32 ) };
	// 83174EA0: 4BFBA1F9  bl 0x8312f098
	ctx.lr = 0x83174EA4;
	sub_8312F098(ctx, base);
	// 83174EA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83174EA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83174EAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83174EB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83174EB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83174EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83174EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83174EC0 size=316
    let mut pc: u32 = 0x83174EC0;
    'dispatch: loop {
        match pc {
            0x83174EC0 => {
    //   block [0x83174EC0..0x83174FFC)
	// 83174EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83174EC4: 480332A9  bl 0x831a816c
	ctx.lr = 0x83174EC8;
	sub_831A8130(ctx, base);
	// 83174EC8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83174ECC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83174ED0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83174ED4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83174ED8: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83174EDC: 419A010C  beq cr6, 0x83174fe8
	if ctx.cr[6].eq {
	pc = 0x83174FE8; continue 'dispatch;
	}
	// 83174EE0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83174EE4: 419A0104  beq cr6, 0x83174fe8
	if ctx.cr[6].eq {
	pc = 0x83174FE8; continue 'dispatch;
	}
	// 83174EE8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 83174EEC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83174EF0: 39200006  li r9, 6
	ctx.r[9].s64 = 6;
	// 83174EF4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 83174EF8: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 83174EFC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 83174F00: 4200FFF8  bdnz 0x83174ef8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x83174EF8; continue 'dispatch;
	}
	// 83174F04: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 83174F08: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 83174F0C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83174F10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174F14: 480332CD  bl 0x831a81e0
	ctx.lr = 0x83174F18;
	sub_831A81E0(ctx, base);
	// 83174F18: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83174F1C: 41990018  bgt cr6, 0x83174f34
	if ctx.cr[6].gt {
	pc = 0x83174F34; continue 'dispatch;
	}
	// 83174F20: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83174F24: 386BA148  addi r3, r11, -0x5eb8
	ctx.r[3].s64 = ctx.r[11].s64 + -24248;
	// 83174F28: 48008219  bl 0x8317d140
	ctx.lr = 0x83174F2C;
	sub_8317D140(ctx, base);
	// 83174F2C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 83174F30: 4803328C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83174F34: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 83174F38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83174F3C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83174F40: 48014AF9  bl 0x83189a38
	ctx.lr = 0x83174F44;
	sub_83189A38(ctx, base);
	// 83174F44: 89610090  lbz r11, 0x90(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) } as u64;
	// 83174F48: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83174F4C: 409A007C  bne cr6, 0x83174fc8
	if !ctx.cr[6].eq {
	pc = 0x83174FC8; continue 'dispatch;
	}
	// 83174F50: 89610091  lbz r11, 0x91(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(145 as u32) ) } as u64;
	// 83174F54: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 83174F58: 409A0070  bne cr6, 0x83174fc8
	if !ctx.cr[6].eq {
	pc = 0x83174FC8; continue 'dispatch;
	}
	// 83174F5C: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 83174F60: 4BFFFAE1  bl 0x83174a40
	ctx.lr = 0x83174F64;
	sub_83174A40(ctx, base);
	// 83174F64: 816100A4  lwz r11, 0xa4(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 83174F68: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83174F6C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83174F70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83174F74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83174F78: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83174F7C: 816100A8  lwz r11, 0xa8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(168 as u32) ) } as u64;
	// 83174F80: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83174F84: 816100B0  lwz r11, 0xb0(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 83174F88: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 83174F8C: 816100BC  lwz r11, 0xbc(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(188 as u32) ) } as u64;
	// 83174F90: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 83174F94: 896100B8  lbz r11, 0xb8(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(184 as u32) ) } as u64;
	// 83174F98: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83174F9C: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 83174FA0: 48003389  bl 0x83178328
	ctx.lr = 0x83174FA4;
	sub_83178328(ctx, base);
	// 83174FA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83174FA8: 4BFFFAE9  bl 0x83174a90
	ctx.lr = 0x83174FAC;
	sub_83174A90(ctx, base);
	// 83174FAC: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 83174FB0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83174FB4: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 83174FB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174FBC: 48033555  bl 0x831a8510
	ctx.lr = 0x83174FC0;
	sub_831A8510(ctx, base);
	// 83174FC0: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 83174FC4: 480331F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83174FC8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83174FCC: 38A00034  li r5, 0x34
	ctx.r[5].s64 = 52;
	// 83174FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83174FD4: 4803353D  bl 0x831a8510
	ctx.lr = 0x83174FD8;
	sub_831A8510(ctx, base);
	// 83174FD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83174FDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83174FE0: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 83174FE4: 480331D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83174FE8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83174FEC: 386BA120  addi r3, r11, -0x5ee0
	ctx.r[3].s64 = ctx.r[11].s64 + -24288;
	// 83174FF0: 48008151  bl 0x8317d140
	ctx.lr = 0x83174FF4;
	sub_8317D140(ctx, base);
	// 83174FF4: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 83174FF8: 480331C4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83175000 size=116
    let mut pc: u32 = 0x83175000;
    'dispatch: loop {
        match pc {
            0x83175000 => {
    //   block [0x83175000..0x83175074)
	// 83175000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83175004: 48033169  bl 0x831a816c
	ctx.lr = 0x83175008;
	sub_831A8130(ctx, base);
	// 83175008: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317500C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83175010: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83175014: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83175018: 40980010  bge cr6, 0x83175028
	if !ctx.cr[6].lt {
	pc = 0x83175028; continue 'dispatch;
	}
	// 8317501C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83175020: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83175024: 48033198  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83175028: 817E0480  lwz r11, 0x480(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1152 as u32) ) } as u64;
	// 8317502C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83175030: 419A0014  beq cr6, 0x83175044
	if ctx.cr[6].eq {
	pc = 0x83175044; continue 'dispatch;
	}
	// 83175034: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83175038: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317503C: 4BFFFA6D  bl 0x83174aa8
	ctx.lr = 0x83175040;
	sub_83174AA8(ctx, base);
	// 83175040: 4800000C  b 0x8317504c
	pc = 0x8317504C; continue 'dispatch;
	// 83175044: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83175048: 4BFFFA99  bl 0x83174ae0
	ctx.lr = 0x8317504C;
	sub_83174AE0(ctx, base);
	// 8317504C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83175050: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83175054: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83175058: 48033189  bl 0x831a81e0
	ctx.lr = 0x8317505C;
	sub_831A81E0(ctx, base);
	// 8317505C: 397F001F  addi r11, r31, 0x1f
	ctx.r[11].s64 = ctx.r[31].s64 + 31;
	// 83175060: 93FE0498  stw r31, 0x498(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1176 as u32), ctx.r[31].u32 ) };
	// 83175064: 55630034  rlwinm r3, r11, 0, 0, 0x1a
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 83175068: 907E049C  stw r3, 0x49c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(1180 as u32), ctx.r[3].u32 ) };
	// 8317506C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83175070: 4803314C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83175078 size=116
    let mut pc: u32 = 0x83175078;
    'dispatch: loop {
        match pc {
            0x83175078 => {
    //   block [0x83175078..0x831750EC)
	// 83175078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317507C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83175080: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83175084: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83175088: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317508C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83175090: 817F0494  lwz r11, 0x494(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1172 as u32) ) } as u64;
	// 83175094: 83DF0498  lwz r30, 0x498(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1176 as u32) ) } as u64;
	// 83175098: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317509C: 4199000C  bgt cr6, 0x831750a8
	if ctx.cr[6].gt {
	pc = 0x831750A8; continue 'dispatch;
	}
	// 831750A0: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831750A4: 409A0010  bne cr6, 0x831750b4
	if !ctx.cr[6].eq {
	pc = 0x831750B4; continue 'dispatch;
	}
	// 831750A8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831750AC: 386BA170  addi r3, r11, -0x5e90
	ctx.r[3].s64 = ctx.r[11].s64 + -24208;
	// 831750B0: 48008091  bl 0x8317d140
	ctx.lr = 0x831750B4;
	sub_8317D140(ctx, base);
	// 831750B4: 817F0480  lwz r11, 0x480(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1152 as u32) ) } as u64;
	// 831750B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831750BC: 409A000C  bne cr6, 0x831750c8
	if !ctx.cr[6].eq {
	pc = 0x831750C8; continue 'dispatch;
	}
	// 831750C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831750C4: 4BFFFA65  bl 0x83174b28
	ctx.lr = 0x831750C8;
	sub_83174B28(ctx, base);
	// 831750C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831750CC: 917F0498  stw r11, 0x498(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1176 as u32), ctx.r[11].u32 ) };
	// 831750D0: 917F049C  stw r11, 0x49c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1180 as u32), ctx.r[11].u32 ) };
	// 831750D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831750D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831750DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831750E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831750E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831750E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831750F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831750F0 size=224
    let mut pc: u32 = 0x831750F0;
    'dispatch: loop {
        match pc {
            0x831750F0 => {
    //   block [0x831750F0..0x831751D0)
	// 831750F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831750F4: 48033075  bl 0x831a8168
	ctx.lr = 0x831750F8;
	sub_831A8130(ctx, base);
	// 831750F8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831750FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83175100: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83175104: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83175108: 4BFFFB89  bl 0x83174c90
	ctx.lr = 0x8317510C;
	sub_83174C90(ctx, base);
	// 8317510C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83175110: 409A0018  bne cr6, 0x83175128
	if !ctx.cr[6].eq {
	pc = 0x83175128; continue 'dispatch;
	}
	// 83175114: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83175118: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317511C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83175120: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83175124: 48033094  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83175128: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 8317512C: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83175130: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 83175134: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83175138: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8317513C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175140: 4BFFED09  bl 0x83173e48
	ctx.lr = 0x83175144;
	sub_83173E48(ctx, base);
	// 83175144: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83175148: 4BFFED41  bl 0x83173e88
	ctx.lr = 0x8317514C;
	sub_83173E88(ctx, base);
	// 8317514C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83175150: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83175154: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 83175158: 394A000F  addi r10, r10, 0xf
	ctx.r[10].s64 = ctx.r[10].s64 + 15;
	// 8317515C: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 83175160: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83175164: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 83175168: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317516C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 83175170: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83175174: 55482036  slwi r8, r10, 4
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 83175178: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 8317517C: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 83175180: 3929007F  addi r9, r9, 0x7f
	ctx.r[9].s64 = ctx.r[9].s64 + 127;
	// 83175184: 7D293E70  srawi r9, r9, 7
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 7) as i64;
	// 83175188: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 8317518C: 7D080E70  srawi r8, r8, 1
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 1) as i64;
	// 83175190: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 83175194: 7D673E70  srawi r7, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 83175198: 7D6941D6  mullw r11, r9, r8
	ctx.r[11].s64 = (ctx.r[9].s32 as i64) * (ctx.r[8].s32 as i64);
	// 8317519C: 7D270194  addze r9, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[9].s64 = tmp.s64;
	// 831751A0: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s64 = (ctx.r[9].s32 as i64) * (ctx.r[10].s32 as i64);
	// 831751A4: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831751A8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831751AC: 556A402E  slwi r10, r11, 8
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831751B0: 556B482C  slwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831751B4: 394A0080  addi r10, r10, 0x80
	ctx.r[10].s64 = ctx.r[10].s64 + 128;
	// 831751B8: 396B0100  addi r11, r11, 0x100
	ctx.r[11].s64 = ctx.r[11].s64 + 256;
	// 831751BC: 7D4AF1D6  mullw r10, r10, r30
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[30].s32 as i64);
	// 831751C0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831751C4: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831751C8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831751CC: 48032FEC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831751D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831751D0 size=220
    let mut pc: u32 = 0x831751D0;
    'dispatch: loop {
        match pc {
            0x831751D0 => {
    //   block [0x831751D0..0x831752AC)
	// 831751D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831751D4: 48032F99  bl 0x831a816c
	ctx.lr = 0x831751D8;
	sub_831A8130(ctx, base);
	// 831751D8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831751DC: 3921006C  addi r9, r1, 0x6c
	ctx.r[9].s64 = ctx.r[1].s64 + 108;
	// 831751E0: 39010068  addi r8, r1, 0x68
	ctx.r[8].s64 = ctx.r[1].s64 + 104;
	// 831751E4: 38E10064  addi r7, r1, 0x64
	ctx.r[7].s64 = ctx.r[1].s64 + 100;
	// 831751E8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 831751EC: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 831751F0: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 831751F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831751F8: 4BFFEB31  bl 0x83173d28
	ctx.lr = 0x831751FC;
	sub_83173D28(ctx, base);
	// 831751FC: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83175200: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83175204: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175208: 4BFFFEE9  bl 0x831750f0
	ctx.lr = 0x8317520C;
	sub_831750F0(ctx, base);
	// 8317520C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 83175210: 38A10074  addi r5, r1, 0x74
	ctx.r[5].s64 = ctx.r[1].s64 + 116;
	// 83175214: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 83175218: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317521C: 4BFFED15  bl 0x83173f30
	ctx.lr = 0x83175220;
	sub_83173F30(ctx, base);
	// 83175220: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83175224: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175228: 4BFFECE9  bl 0x83173f10
	ctx.lr = 0x8317522C;
	sub_83173F10(ctx, base);
	// 8317522C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83175230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175234: 4BFFECED  bl 0x83173f20
	ctx.lr = 0x83175238;
	sub_83173F20(ctx, base);
	// 83175238: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317523C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83175240: 386B0002  addi r3, r11, 2
	ctx.r[3].s64 = ctx.r[11].s64 + 2;
	// 83175244: 4800DF15  bl 0x83183158
	ctx.lr = 0x83175248;
	sub_83183158(ctx, base);
	// 83175248: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 8317524C: 4BFFED15  bl 0x83173f60
	ctx.lr = 0x83175250;
	sub_83173F60(ctx, base);
	// 83175250: 7D6A1A14  add r11, r10, r3
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 83175254: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83175258: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 8317525C: 7D6BEA14  add r11, r11, r29
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[29].u64;
	// 83175260: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 83175264: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83175268: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317526C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83175270: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83175274: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83175278: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8317527C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83175280: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83175284: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83175288: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 8317528C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83175290: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83175294: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83175298: 8141006C  lwz r10, 0x6c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 8317529C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 831752A0: 386B00C0  addi r3, r11, 0xc0
	ctx.r[3].s64 = ctx.r[11].s64 + 192;
	// 831752A4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831752A8: 48032F14  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831752B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831752B0 size=160
    let mut pc: u32 = 0x831752B0;
    'dispatch: loop {
        match pc {
            0x831752B0 => {
    //   block [0x831752B0..0x83175350)
	// 831752B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831752B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831752B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831752BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831752C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831752C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831752C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831752CC: 409A0010  bne cr6, 0x831752dc
	if !ctx.cr[6].eq {
	pc = 0x831752DC; continue 'dispatch;
	}
	// 831752D0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831752D4: 386BA214  addi r3, r11, -0x5dec
	ctx.r[3].s64 = ctx.r[11].s64 + -24044;
	// 831752D8: 48000058  b 0x83175330
	pc = 0x83175330; continue 'dispatch;
	// 831752DC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831752E0: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 831752E4: 419A0044  beq cr6, 0x83175328
	if ctx.cr[6].eq {
	pc = 0x83175328; continue 'dispatch;
	}
	// 831752E8: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 831752EC: 419A003C  beq cr6, 0x83175328
	if ctx.cr[6].eq {
	pc = 0x83175328; continue 'dispatch;
	}
	// 831752F0: 2F0B0009  cmpwi cr6, r11, 9
	ctx.cr[6].compare_i32(ctx.r[11].s32, 9, &mut ctx.xer);
	// 831752F4: 419A0028  beq cr6, 0x8317531c
	if ctx.cr[6].eq {
	pc = 0x8317531C; continue 'dispatch;
	}
	// 831752F8: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 831752FC: 419A0020  beq cr6, 0x8317531c
	if ctx.cr[6].eq {
	pc = 0x8317531C; continue 'dispatch;
	}
	// 83175300: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175304: 4BFFFECD  bl 0x831751d0
	ctx.lr = 0x83175308;
	sub_831751D0(ctx, base);
	// 83175308: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317530C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175310: 4BFFEC59  bl 0x83173f68
	ctx.lr = 0x83175314;
	sub_83173F68(ctx, base);
	// 83175314: 7C63F214  add r3, r3, r30
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	// 83175318: 48000020  b 0x83175338
	pc = 0x83175338; continue 'dispatch;
	// 8317531C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83175320: 386BA1E0  addi r3, r11, -0x5e20
	ctx.r[3].s64 = ctx.r[11].s64 + -24096;
	// 83175324: 4800000C  b 0x83175330
	pc = 0x83175330; continue 'dispatch;
	// 83175328: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317532C: 386BA1AC  addi r3, r11, -0x5e54
	ctx.r[3].s64 = ctx.r[11].s64 + -24148;
	// 83175330: 48007E11  bl 0x8317d140
	ctx.lr = 0x83175334;
	sub_8317D140(ctx, base);
	// 83175334: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83175338: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317533C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83175340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83175344: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83175348: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317534C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83175350 size=1268
    let mut pc: u32 = 0x83175350;
    'dispatch: loop {
        match pc {
            0x83175350 => {
    //   block [0x83175350..0x83175844)
	// 83175350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83175354: 48032DDD  bl 0x831a8130
	ctx.lr = 0x83175358;
	sub_831A8130(ctx, base);
	// 83175358: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317535C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83175360: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83175364: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83175368: 3A400000  li r18, 0
	ctx.r[18].s64 = 0;
	// 8317536C: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 83175370: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83175374: 833E0010  lwz r25, 0x10(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 83175378: 82BE0000  lwz r21, 0(r30)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317537C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 83175380: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 83175384: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 83175388: 4BFFEB89  bl 0x83173f10
	ctx.lr = 0x8317538C;
	sub_83173F10(ctx, base);
	// 8317538C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83175390: 38790002  addi r3, r25, 2
	ctx.r[3].s64 = ctx.r[25].s64 + 2;
	// 83175394: 4800DDC5  bl 0x83183158
	ctx.lr = 0x83175398;
	sub_83183158(ctx, base);
	// 83175398: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8317539C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831753A0: 4BFFEB81  bl 0x83173f20
	ctx.lr = 0x831753A4;
	sub_83173F20(ctx, base);
	// 831753A4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 831753A8: 7C6F1B78  mr r15, r3
	ctx.r[15].u64 = ctx.r[3].u64;
	// 831753AC: 3BEB9A78  addi r31, r11, -0x6588
	ctx.r[31].s64 = ctx.r[11].s64 + -25992;
	// 831753B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831753B4: 393F01CC  addi r9, r31, 0x1cc
	ctx.r[9].s64 = ctx.r[31].s64 + 460;
	// 831753B8: 391F013C  addi r8, r31, 0x13c
	ctx.r[8].s64 = ctx.r[31].s64 + 316;
	// 831753BC: 38FF0004  addi r7, r31, 4
	ctx.r[7].s64 = ctx.r[31].s64 + 4;
	// 831753C0: 91E10070  stw r15, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[15].u32 ) };
	// 831753C4: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 831753C8: 38BF0134  addi r5, r31, 0x134
	ctx.r[5].s64 = ctx.r[31].s64 + 308;
	// 831753CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831753D0: 4BFFE959  bl 0x83173d28
	ctx.lr = 0x831753D4;
	sub_83173D28(ctx, base);
	// 831753D4: 38BF002C  addi r5, r31, 0x2c
	ctx.r[5].s64 = ctx.r[31].s64 + 44;
	// 831753D8: 389F0138  addi r4, r31, 0x138
	ctx.r[4].s64 = ctx.r[31].s64 + 312;
	// 831753DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831753E0: 4BFFFD11  bl 0x831750f0
	ctx.lr = 0x831753E4;
	sub_831750F0(ctx, base);
	// 831753E4: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 831753E8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831753EC: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 831753F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831753F4: 4BFFEB3D  bl 0x83173f30
	ctx.lr = 0x831753F8;
	sub_83173F30(ctx, base);
	// 831753F8: 4BFFEB69  bl 0x83173f60
	ctx.lr = 0x831753FC;
	sub_83173F60(ctx, base);
	// 831753FC: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 83175400: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83175404: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83175408: 90610054  stw r3, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 8317540C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83175410: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83175414: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83175418: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8317541C: 3B6B0080  addi r27, r11, 0x80
	ctx.r[27].s64 = ctx.r[11].s64 + 128;
	// 83175420: 4BFFFBE1  bl 0x83175000
	ctx.lr = 0x83175424;
	sub_83175000(ctx, base);
	// 83175424: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 83175428: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8317542C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83175430: 480123B9  bl 0x831877e8
	ctx.lr = 0x83175434;
	sub_831877E8(ctx, base);
	// 83175434: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83175438: 7C701B78  mr r16, r3
	ctx.r[16].u64 = ctx.r[3].u64;
	// 8317543C: 388B0040  addi r4, r11, 0x40
	ctx.r[4].s64 = ctx.r[11].s64 + 64;
	// 83175440: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83175444: 480123A5  bl 0x831877e8
	ctx.lr = 0x83175448;
	sub_831877E8(ctx, base);
	// 83175448: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8317544C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83175450: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83175454: 48012395  bl 0x831877e8
	ctx.lr = 0x83175458;
	sub_831877E8(ctx, base);
	// 83175458: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8317545C: 38A10078  addi r5, r1, 0x78
	ctx.r[5].s64 = ctx.r[1].s64 + 120;
	// 83175460: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83175464: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83175468: 4BFFECA1  bl 0x83174108
	ctx.lr = 0x8317546C;
	sub_83174108(ctx, base);
	// 8317546C: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 83175470: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83175474: 419A003C  beq cr6, 0x831754b0
	if ctx.cr[6].eq {
	pc = 0x831754B0; continue 'dispatch;
	}
	// 83175478: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317547C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83175480: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83175484: 38AB0002  addi r5, r11, 2
	ctx.r[5].s64 = ctx.r[11].s64 + 2;
	// 83175488: 4800DCE1  bl 0x83183168
	ctx.lr = 0x8317548C;
	sub_83183168(ctx, base);
	// 8317548C: 4800DD7D  bl 0x83183208
	ctx.lr = 0x83175490;
	sub_83183208(ctx, base);
	// 83175490: 82430014  lwz r18, 0x14(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 83175494: 2B120000  cmplwi cr6, r18, 0
	ctx.cr[6].compare_u32(ctx.r[18].u32, 0 as u32, &mut ctx.xer);
	// 83175498: 419A0018  beq cr6, 0x831754b0
	if ctx.cr[6].eq {
	pc = 0x831754B0; continue 'dispatch;
	}
	// 8317549C: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 831754A0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831754A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831754A8: 4BFFED89  bl 0x83174230
	ctx.lr = 0x831754AC;
	sub_83174230(ctx, base);
	// 831754AC: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 831754B0: 7EA3AB78  mr r3, r21
	ctx.r[3].u64 = ctx.r[21].u64;
	// 831754B4: 4BFFEEC5  bl 0x83174378
	ctx.lr = 0x831754B8;
	sub_83174378(ctx, base);
	// 831754B8: 7C711B78  mr r17, r3
	ctx.r[17].u64 = ctx.r[3].u64;
	// 831754BC: 2F110001  cmpwi cr6, r17, 1
	ctx.cr[6].compare_i32(ctx.r[17].s32, 1, &mut ctx.xer);
	// 831754C0: 409A0028  bne cr6, 0x831754e8
	if !ctx.cr[6].eq {
	pc = 0x831754E8; continue 'dispatch;
	}
	// 831754C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831754C8: 809F013C  lwz r4, 0x13c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 831754CC: 4801231D  bl 0x831877e8
	ctx.lr = 0x831754D0;
	sub_831877E8(ctx, base);
	// 831754D0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 831754D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831754D8: 809F01CC  lwz r4, 0x1cc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(460 as u32) ) } as u64;
	// 831754DC: 4801230D  bl 0x831877e8
	ctx.lr = 0x831754E0;
	sub_831877E8(ctx, base);
	// 831754E0: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 831754E4: 4800000C  b 0x831754f0
	pc = 0x831754F0; continue 'dispatch;
	// 831754E8: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 831754EC: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 831754F0: 7DE47B78  mr r4, r15
	ctx.r[4].u64 = ctx.r[15].u64;
	// 831754F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831754F8: 480122F1  bl 0x831877e8
	ctx.lr = 0x831754FC;
	sub_831877E8(ctx, base);
	// 831754FC: 3963001F  addi r11, r3, 0x1f
	ctx.r[11].s64 = ctx.r[3].s64 + 31;
	// 83175500: 81C10058  lwz r14, 0x58(r1)
	ctx.r[14].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83175504: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83175508: 556B0034  rlwinm r11, r11, 0, 0, 0x1a
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8317550C: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 83175510: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 83175514: 480122D5  bl 0x831877e8
	ctx.lr = 0x83175518;
	sub_831877E8(ctx, base);
	// 83175518: 7C6F1B78  mr r15, r3
	ctx.r[15].u64 = ctx.r[3].u64;
	// 8317551C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83175520: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83175524: 480122C5  bl 0x831877e8
	ctx.lr = 0x83175528;
	sub_831877E8(ctx, base);
	// 83175528: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 8317552C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83175530: 8081005C  lwz r4, 0x5c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83175534: 480122B5  bl 0x831877e8
	ctx.lr = 0x83175538;
	sub_831877E8(ctx, base);
	// 83175538: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8317553C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83175540: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83175544: 480122A5  bl 0x831877e8
	ctx.lr = 0x83175548;
	sub_831877E8(ctx, base);
	// 83175548: 90610074  stw r3, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 8317554C: 2B100000  cmplwi cr6, r16, 0
	ctx.cr[6].compare_u32(ctx.r[16].u32, 0 as u32, &mut ctx.xer);
	// 83175550: 419A02D4  beq cr6, 0x83175824
	if ctx.cr[6].eq {
	pc = 0x83175824; continue 'dispatch;
	}
	// 83175554: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 83175558: 419A02CC  beq cr6, 0x83175824
	if ctx.cr[6].eq {
	pc = 0x83175824; continue 'dispatch;
	}
	// 8317555C: 2F140000  cmpwi cr6, r20, 0
	ctx.cr[6].compare_i32(ctx.r[20].s32, 0, &mut ctx.xer);
	// 83175560: 409A02C4  bne cr6, 0x83175824
	if !ctx.cr[6].eq {
	pc = 0x83175824; continue 'dispatch;
	}
	// 83175564: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 83175568: 409A02BC  bne cr6, 0x83175824
	if !ctx.cr[6].eq {
	pc = 0x83175824; continue 'dispatch;
	}
	// 8317556C: 2B0F0000  cmplwi cr6, r15, 0
	ctx.cr[6].compare_u32(ctx.r[15].u32, 0 as u32, &mut ctx.xer);
	// 83175570: 419A02B4  beq cr6, 0x83175824
	if ctx.cr[6].eq {
	pc = 0x83175824; continue 'dispatch;
	}
	// 83175574: 82810058  lwz r20, 0x58(r1)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83175578: 2B140000  cmplwi cr6, r20, 0
	ctx.cr[6].compare_u32(ctx.r[20].u32, 0 as u32, &mut ctx.xer);
	// 8317557C: 419A02A8  beq cr6, 0x83175824
	if ctx.cr[6].eq {
	pc = 0x83175824; continue 'dispatch;
	}
	// 83175580: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 83175584: 419A02A0  beq cr6, 0x83175824
	if ctx.cr[6].eq {
	pc = 0x83175824; continue 'dispatch;
	}
	// 83175588: 2B130000  cmplwi cr6, r19, 0
	ctx.cr[6].compare_u32(ctx.r[19].u32, 0 as u32, &mut ctx.xer);
	// 8317558C: 419A0298  beq cr6, 0x83175824
	if ctx.cr[6].eq {
	pc = 0x83175824; continue 'dispatch;
	}
	// 83175590: 82610060  lwz r19, 0x60(r1)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83175594: 2B130000  cmplwi cr6, r19, 0
	ctx.cr[6].compare_u32(ctx.r[19].u32, 0 as u32, &mut ctx.xer);
	// 83175598: 419A028C  beq cr6, 0x83175824
	if ctx.cr[6].eq {
	pc = 0x83175824; continue 'dispatch;
	}
	// 8317559C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831755A0: 419A0284  beq cr6, 0x83175824
	if ctx.cr[6].eq {
	pc = 0x83175824; continue 'dispatch;
	}
	// 831755A4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 831755A8: 419A027C  beq cr6, 0x83175824
	if ctx.cr[6].eq {
	pc = 0x83175824; continue 'dispatch;
	}
	// 831755AC: 2F110001  cmpwi cr6, r17, 1
	ctx.cr[6].compare_i32(ctx.r[17].s32, 1, &mut ctx.xer);
	// 831755B0: 409A0020  bne cr6, 0x831755d0
	if !ctx.cr[6].eq {
	pc = 0x831755D0; continue 'dispatch;
	}
	// 831755B4: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 831755B8: 419A000C  beq cr6, 0x831755c4
	if ctx.cr[6].eq {
	pc = 0x831755C4; continue 'dispatch;
	}
	// 831755BC: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 831755C0: 409A0010  bne cr6, 0x831755d0
	if !ctx.cr[6].eq {
	pc = 0x831755D0; continue 'dispatch;
	}
	// 831755C4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831755C8: 386BA2DC  addi r3, r11, -0x5d24
	ctx.r[3].s64 = ctx.r[11].s64 + -23844;
	// 831755CC: 48000260  b 0x8317582c
	pc = 0x8317582C; continue 'dispatch;
	// 831755D0: 3976003F  addi r11, r22, 0x3f
	ctx.r[11].s64 = ctx.r[22].s64 + 63;
	// 831755D4: 38E1005C  addi r7, r1, 0x5c
	ctx.r[7].s64 = ctx.r[1].s64 + 92;
	// 831755D8: 557B0032  rlwinm r27, r11, 0, 0, 0x19
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 831755DC: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 831755E0: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 831755E4: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 831755E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831755EC: 937F0130  stw r27, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[27].u32 ) };
	// 831755F0: 4BFFE859  bl 0x83173e48
	ctx.lr = 0x831755F4;
	sub_83173E48(ctx, base);
	// 831755F4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831755F8: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 831755FC: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83175600: 3B8BD3A8  addi r28, r11, -0x2c58
	ctx.r[28].s64 = ctx.r[11].s64 + -11352;
	// 83175604: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83175608: 7EE9BB78  mr r9, r23
	ctx.r[9].u64 = ctx.r[23].u64;
	// 8317560C: 2F150001  cmpwi cr6, r21, 1
	ctx.cr[6].compare_i32(ctx.r[21].s32, 1, &mut ctx.xer);
	// 83175610: 917C0044  stw r11, 0x44(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83175614: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83175618: 935C00B4  stw r26, 0xb4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(180 as u32), ctx.r[26].u32 ) };
	// 8317561C: 835C010C  lwz r26, 0x10c(r28)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(268 as u32) ) } as u64;
	// 83175620: 931C00C4  stw r24, 0xc4(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(196 as u32), ctx.r[24].u32 ) };
	// 83175624: 917C0048  stw r11, 0x48(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83175628: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8317562C: 917C004C  stw r11, 0x4c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 83175630: 915C0050  stw r10, 0x50(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 83175634: 913C0054  stw r9, 0x54(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 83175638: 917C0058  stw r11, 0x58(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8317563C: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 83175640: 915C005C  stw r10, 0x5c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 83175644: 933C0060  stw r25, 0x60(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(96 as u32), ctx.r[25].u32 ) };
	// 83175648: 917C0064  stw r11, 0x64(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 8317564C: 419A0044  beq cr6, 0x83175690
	if ctx.cr[6].eq {
	pc = 0x83175690; continue 'dispatch;
	}
	// 83175650: 2F150002  cmpwi cr6, r21, 2
	ctx.cr[6].compare_i32(ctx.r[21].s32, 2, &mut ctx.xer);
	// 83175654: 419A0014  beq cr6, 0x83175668
	if ctx.cr[6].eq {
	pc = 0x83175668; continue 'dispatch;
	}
	// 83175658: 2F150003  cmpwi cr6, r21, 3
	ctx.cr[6].compare_i32(ctx.r[21].s32, 3, &mut ctx.xer);
	// 8317565C: 409A0058  bne cr6, 0x831756b4
	if !ctx.cr[6].eq {
	pc = 0x831756B4; continue 'dispatch;
	}
	// 83175660: 389C00C8  addi r4, r28, 0xc8
	ctx.r[4].s64 = ctx.r[28].s64 + 200;
	// 83175664: 48000030  b 0x83175694
	pc = 0x83175694; continue 'dispatch;
	// 83175668: 389C0068  addi r4, r28, 0x68
	ctx.r[4].s64 = ctx.r[28].s64 + 104;
	// 8317566C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 83175670: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 83175674: 48032E9D  bl 0x831a8510
	ctx.lr = 0x83175678;
	sub_831A8510(ctx, base);
	// 83175678: 39400800  li r10, 0x800
	ctx.r[10].s64 = 2048;
	// 8317567C: 937D0458  stw r27, 0x458(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1112 as u32), ctx.r[27].u32 ) };
	// 83175680: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83175684: 396BF800  addi r11, r11, -0x800
	ctx.r[11].s64 = ctx.r[11].s64 + -2048;
	// 83175688: 915D0460  stw r10, 0x460(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1120 as u32), ctx.r[10].u32 ) };
	// 8317568C: 48000024  b 0x831756b0
	pc = 0x831756B0; continue 'dispatch;
	// 83175690: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83175694: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 83175698: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8317569C: 48032E75  bl 0x831a8510
	ctx.lr = 0x831756A0;
	sub_831A8510(ctx, base);
	// 831756A0: 937D0458  stw r27, 0x458(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1112 as u32), ctx.r[27].u32 ) };
	// 831756A4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831756A8: 935D0460  stw r26, 0x460(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1120 as u32), ctx.r[26].u32 ) };
	// 831756AC: 7D7A5850  subf r11, r26, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[26].s64;
	// 831756B0: 917D045C  stw r11, 0x45c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1116 as u32), ctx.r[11].u32 ) };
	// 831756B4: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 831756B8: 934100A8  stw r26, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[26].u32 ) };
	// 831756BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831756C0: 419A0018  beq cr6, 0x831756d8
	if ctx.cr[6].eq {
	pc = 0x831756D8; continue 'dispatch;
	}
	// 831756C4: 7D4BD3D6  divw r10, r11, r26
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[26].s32;
	// 831756C8: 7D4AD1D6  mullw r10, r10, r26
	ctx.r[10].s64 = (ctx.r[10].s32 as i64) * (ctx.r[26].s32 as i64);
	// 831756CC: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831756D0: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831756D4: 917F0134  stw r11, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 831756D8: 807E0024  lwz r3, 0x24(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 831756DC: 4BFFE7AD  bl 0x83173e88
	ctx.lr = 0x831756E0;
	sub_83173E88(ctx, base);
	// 831756E0: 815F0134  lwz r10, 0x134(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 831756E4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831756E8: 387C0044  addi r3, r28, 0x44
	ctx.r[3].s64 = ctx.r[28].s64 + 68;
	// 831756EC: 92010084  stw r16, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[16].u32 ) };
	// 831756F0: 7E459378  mr r5, r18
	ctx.r[5].u64 = ctx.r[18].u64;
	// 831756F4: 932100AC  stw r25, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[25].u32 ) };
	// 831756F8: 38810078  addi r4, r1, 0x78
	ctx.r[4].s64 = ctx.r[1].s64 + 120;
	// 831756FC: 91E100BC  stw r15, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[15].u32 ) };
	// 83175700: 91C100C0  stw r14, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[14].u32 ) };
	// 83175704: 91410088  stw r10, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 83175708: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8317570C: 916100B8  stw r11, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 83175710: 9141008C  stw r10, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[10].u32 ) };
	// 83175714: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83175718: 91410090  stw r10, 0x90(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), ctx.r[10].u32 ) };
	// 8317571C: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 83175720: 914100B0  stw r10, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[10].u32 ) };
	// 83175724: 8141006C  lwz r10, 0x6c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 83175728: 914100B4  stw r10, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 8317572C: 4801226D  bl 0x83187998
	ctx.lr = 0x83175730;
	sub_83187998(ctx, base);
	// 83175730: 2F150001  cmpwi cr6, r21, 1
	ctx.cr[6].compare_i32(ctx.r[21].s32, 1, &mut ctx.xer);
	// 83175734: 419A000C  beq cr6, 0x83175740
	if ctx.cr[6].eq {
	pc = 0x83175740; continue 'dispatch;
	}
	// 83175738: 2F150009  cmpwi cr6, r21, 9
	ctx.cr[6].compare_i32(ctx.r[21].s32, 9, &mut ctx.xer);
	// 8317573C: 409A000C  bne cr6, 0x83175748
	if !ctx.cr[6].eq {
	pc = 0x83175748; continue 'dispatch;
	}
	// 83175740: 387C00AC  addi r3, r28, 0xac
	ctx.r[3].s64 = ctx.r[28].s64 + 172;
	// 83175744: 4801029D  bl 0x831859e0
	ctx.lr = 0x83175748;
	sub_831859E0(ctx, base);
	// 83175748: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317574C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 83175750: 4800F349  bl 0x83184a98
	ctx.lr = 0x83175754;
	sub_83184A98(ctx, base);
	// 83175754: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83175758: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8317575C: 409A0024  bne cr6, 0x83175780
	if !ctx.cr[6].eq {
	pc = 0x83175780; continue 'dispatch;
	}
	// 83175760: 3860FECF  li r3, -0x131
	ctx.r[3].s64 = -305;
	// 83175764: 4BFFDE65  bl 0x831735c8
	ctx.lr = 0x83175768;
	sub_831735C8(ctx, base);
	// 83175768: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317576C: 386BA2B0  addi r3, r11, -0x5d50
	ctx.r[3].s64 = ctx.r[11].s64 + -23888;
	// 83175770: 480079D1  bl 0x8317d140
	ctx.lr = 0x83175774;
	sub_8317D140(ctx, base);
	// 83175774: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83175778: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 8317577C: 48032A04  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
	// 83175780: 3D608317  lis r11, -0x7ce9
	ctx.r[11].s64 = -2095644672;
	// 83175784: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83175788: 388B3620  addi r4, r11, 0x3620
	ctx.r[4].s64 = ctx.r[11].s64 + 13856;
	// 8317578C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175790: 48011E31  bl 0x831875c0
	ctx.lr = 0x83175794;
	sub_831875C0(ctx, base);
	// 83175794: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83175798: 419A0024  beq cr6, 0x831757bc
	if ctx.cr[6].eq {
	pc = 0x831757BC; continue 'dispatch;
	}
	// 8317579C: 3860FED1  li r3, -0x12f
	ctx.r[3].s64 = -303;
	// 831757A0: 4BFFDE29  bl 0x831735c8
	ctx.lr = 0x831757A4;
	sub_831735C8(ctx, base);
	// 831757A4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831757A8: 386BA288  addi r3, r11, -0x5d78
	ctx.r[3].s64 = ctx.r[11].s64 + -23928;
	// 831757AC: 48007995  bl 0x8317d140
	ctx.lr = 0x831757B0;
	sub_8317D140(ctx, base);
	// 831757B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831757B4: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 831757B8: 480329C8  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
	// 831757BC: 7E83A378  mr r3, r20
	ctx.r[3].u64 = ctx.r[20].u64;
	// 831757C0: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831757C4: 4BFFE545  bl 0x83173d08
	ctx.lr = 0x831757C8;
	sub_83173D08(ctx, base);
	// 831757C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831757CC: 907D04F4  stw r3, 0x4f4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1268 as u32), ctx.r[3].u32 ) };
	// 831757D0: 409A001C  bne cr6, 0x831757ec
	if !ctx.cr[6].eq {
	pc = 0x831757EC; continue 'dispatch;
	}
	// 831757D4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831757D8: 386BA260  addi r3, r11, -0x5da0
	ctx.r[3].s64 = ctx.r[11].s64 + -23968;
	// 831757DC: 48007965  bl 0x8317d140
	ctx.lr = 0x831757E0;
	sub_8317D140(ctx, base);
	// 831757E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831757E4: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 831757E8: 48032998  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
	// 831757EC: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 831757F0: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 831757F4: 7E649B78  mr r4, r19
	ctx.r[4].u64 = ctx.r[19].u64;
	// 831757F8: 80A10070  lwz r5, 0x70(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 831757FC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83175800: 917D0438  stw r11, 0x438(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1080 as u32), ctx.r[11].u32 ) };
	// 83175804: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83175808: 917D043C  stw r11, 0x43c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1084 as u32), ctx.r[11].u32 ) };
	// 8317580C: 4BFFEBBD  bl 0x831743c8
	ctx.lr = 0x83175810;
	sub_831743C8(ctx, base);
	// 83175810: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175814: 92FD04F8  stw r23, 0x4f8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1272 as u32), ctx.r[23].u32 ) };
	// 83175818: 92FD0538  stw r23, 0x538(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(1336 as u32), ctx.r[23].u32 ) };
	// 8317581C: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 83175820: 48032960  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
	// 83175824: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83175828: 386BA244  addi r3, r11, -0x5dbc
	ctx.r[3].s64 = ctx.r[11].s64 + -23996;
	// 8317582C: 48007915  bl 0x8317d140
	ctx.lr = 0x83175830;
	sub_8317D140(ctx, base);
	// 83175830: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83175834: 480120B5  bl 0x831878e8
	ctx.lr = 0x83175838;
	sub_831878E8(ctx, base);
	// 83175838: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317583C: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 83175840: 48032940  b 0x831a8180
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83175848 size=112
    let mut pc: u32 = 0x83175848;
    'dispatch: loop {
        match pc {
            0x83175848 => {
    //   block [0x83175848..0x831758B8)
	// 83175848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317584C: 48032921  bl 0x831a816c
	ctx.lr = 0x83175850;
	sub_831A8130(ctx, base);
	// 83175850: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83175854: 3FA08345  lis r29, -0x7cbb
	ctx.r[29].s64 = -2092630016;
	// 83175858: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317585C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83175860: 3BCBC040  addi r30, r11, -0x3fc0
	ctx.r[30].s64 = ctx.r[11].s64 + -16320;
	// 83175864: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83175868: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317586C: 419A001C  beq cr6, 0x83175888
	if ctx.cr[6].eq {
	pc = 0x83175888; continue 'dispatch;
	}
	// 83175870: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83175874: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83175878: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317587C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83175880: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83175884: 4E800421  bctrl
	ctx.lr = 0x83175888;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83175888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317588C: 4BFFF4D5  bl 0x83174d60
	ctx.lr = 0x83175890;
	sub_83174D60(ctx, base);
	// 83175890: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83175894: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83175898: 419A0018  beq cr6, 0x831758b0
	if ctx.cr[6].eq {
	pc = 0x831758B0; continue 'dispatch;
	}
	// 8317589C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831758A0: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 831758A4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 831758A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831758AC: 4E800421  bctrl
	ctx.lr = 0x831758B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831758B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831758B4: 48032908  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831758B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831758B8 size=996
    let mut pc: u32 = 0x831758B8;
    'dispatch: loop {
        match pc {
            0x831758B8 => {
    //   block [0x831758B8..0x83175C9C)
	// 831758B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831758BC: 4803289D  bl 0x831a8158
	ctx.lr = 0x831758C0;
	sub_831A8130(ctx, base);
	// 831758C0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831758C4: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 831758C8: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 831758CC: 409A001C  bne cr6, 0x831758e8
	if !ctx.cr[6].eq {
	pc = 0x831758E8; continue 'dispatch;
	}
	// 831758D0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831758D4: 386BA438  addi r3, r11, -0x5bc8
	ctx.r[3].s64 = ctx.r[11].s64 + -23496;
	// 831758D8: 48007869  bl 0x8317d140
	ctx.lr = 0x831758DC;
	sub_8317D140(ctx, base);
	// 831758DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831758E0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 831758E4: 480328C4  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 831758E8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831758EC: 4BFFEE75  bl 0x83174760
	ctx.lr = 0x831758F0;
	sub_83174760(ctx, base);
	// 831758F0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831758F4: 409A039C  bne cr6, 0x83175c90
	if !ctx.cr[6].eq {
	pc = 0x83175C90; continue 'dispatch;
	}
	// 831758F8: 4BFFDA91  bl 0x83173388
	ctx.lr = 0x831758FC;
	sub_83173388(ctx, base);
	// 831758FC: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83175900: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83175904: 397A006C  addi r11, r26, 0x6c
	ctx.r[11].s64 = ctx.r[26].s64 + 108;
	// 83175908: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 8317590C: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83175910: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 83175914: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83175918: 409A0014  bne cr6, 0x8317592c
	if !ctx.cr[6].eq {
	pc = 0x8317592C; continue 'dispatch;
	}
	// 8317591C: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83175920: 396B0540  addi r11, r11, 0x540
	ctx.r[11].s64 = ctx.r[11].s64 + 1344;
	// 83175924: 2F1D0008  cmpwi cr6, r29, 8
	ctx.cr[6].compare_i32(ctx.r[29].s32, 8, &mut ctx.xer);
	// 83175928: 4198FFE4  blt cr6, 0x8317590c
	if ctx.cr[6].lt {
	pc = 0x8317590C; continue 'dispatch;
	}
	// 8317592C: 2F1D0008  cmpwi cr6, r29, 8
	ctx.cr[6].compare_i32(ctx.r[29].s32, 8, &mut ctx.xer);
	// 83175930: 409A0024  bne cr6, 0x83175954
	if !ctx.cr[6].eq {
	pc = 0x83175954; continue 'dispatch;
	}
	// 83175934: 3860FFF5  li r3, -0xb
	ctx.r[3].s64 = -11;
	// 83175938: 4BFFDC91  bl 0x831735c8
	ctx.lr = 0x8317593C;
	sub_831735C8(ctx, base);
	// 8317593C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83175940: 386BA3E8  addi r3, r11, -0x5c18
	ctx.r[3].s64 = ctx.r[11].s64 + -23576;
	// 83175944: 480077FD  bl 0x8317d140
	ctx.lr = 0x83175948;
	sub_8317D140(ctx, base);
	// 83175948: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317594C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83175950: 48032858  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83175954: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83175958: 4BFFED51  bl 0x831746a8
	ctx.lr = 0x8317595C;
	sub_831746A8(ctx, base);
	// 8317595C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83175960: 409A001C  bne cr6, 0x8317597c
	if !ctx.cr[6].eq {
	pc = 0x8317597C; continue 'dispatch;
	}
	// 83175964: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83175968: 386BA3A8  addi r3, r11, -0x5c58
	ctx.r[3].s64 = ctx.r[11].s64 + -23640;
	// 8317596C: 480077D5  bl 0x8317d140
	ctx.lr = 0x83175970;
	sub_8317D140(ctx, base);
	// 83175970: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83175974: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83175978: 48032830  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 8317597C: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 83175980: 57B8103A  slwi r24, r29, 2
	ctx.r[24].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 83175984: 3B2B9A80  addi r25, r11, -0x6580
	ctx.r[25].s64 = ctx.r[11].s64 + -25984;
	// 83175988: 7C78C82E  lwzx r3, r24, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 8317598C: 4BFB9675  bl 0x8312f000
	ctx.lr = 0x83175990;
	sub_8312F000(ctx, base);
	// 83175990: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83175994: 419802FC  blt cr6, 0x83175c90
	if ctx.cr[6].lt {
	pc = 0x83175C90; continue 'dispatch;
	}
	// 83175998: 38A00540  li r5, 0x540
	ctx.r[5].s64 = 1344;
	// 8317599C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831759A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831759A4: 4803283D  bl 0x831a81e0
	ctx.lr = 0x831759A8;
	sub_831A81E0(ctx, base);
	// 831759A8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831759AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831759B0: 4BFFED59  bl 0x83174708
	ctx.lr = 0x831759B4;
	sub_83174708(ctx, base);
	// 831759B4: 3BBF0008  addi r29, r31, 8
	ctx.r[29].s64 = ctx.r[31].s64 + 8;
	// 831759B8: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 831759BC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 831759C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831759C4: 48032B4D  bl 0x831a8510
	ctx.lr = 0x831759C8;
	sub_831A8510(ctx, base);
	// 831759C8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831759CC: 4BFFF2C5  bl 0x83174c90
	ctx.lr = 0x831759D0;
	sub_83174C90(ctx, base);
	// 831759D0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831759D4: 409A0010  bne cr6, 0x831759e4
	if !ctx.cr[6].eq {
	pc = 0x831759E4; continue 'dispatch;
	}
	// 831759D8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 831759DC: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 831759E0: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 831759E4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831759E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831759EC: 4BFFF965  bl 0x83175350
	ctx.lr = 0x831759F0;
	sub_83175350(ctx, base);
	// 831759F0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831759F4: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 831759F8: 939F0048  stw r28, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[28].u32 ) };
	// 831759FC: 409A0024  bne cr6, 0x83175a20
	if !ctx.cr[6].eq {
	pc = 0x83175A20; continue 'dispatch;
	}
	// 83175A00: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83175A04: 386BA384  addi r3, r11, -0x5c7c
	ctx.r[3].s64 = ctx.r[11].s64 + -23676;
	// 83175A08: 48007739  bl 0x8317d140
	ctx.lr = 0x83175A0C;
	sub_8317D140(ctx, base);
	// 83175A0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175A10: 4BFFFE39  bl 0x83175848
	ctx.lr = 0x83175A14;
	sub_83175848(ctx, base);
	// 83175A14: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83175A18: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83175A1C: 4803278C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83175A20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175A24: 4BFFE9BD  bl 0x831743e0
	ctx.lr = 0x83175A28;
	sub_831743E0(ctx, base);
	// 83175A28: 39210058  addi r9, r1, 0x58
	ctx.r[9].s64 = ctx.r[1].s64 + 88;
	// 83175A2C: 3901005C  addi r8, r1, 0x5c
	ctx.r[8].s64 = ctx.r[1].s64 + 92;
	// 83175A30: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 83175A34: 38C10064  addi r6, r1, 0x64
	ctx.r[6].s64 = ctx.r[1].s64 + 100;
	// 83175A38: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 83175A3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83175A40: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83175A44: 4BFFE2E5  bl 0x83173d28
	ctx.lr = 0x83175A48;
	sub_83173D28(ctx, base);
	// 83175A48: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83175A4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175A50: 80DA000C  lwz r6, 0xc(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 83175A54: 80BA0008  lwz r5, 8(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 83175A58: 4BFFEA59  bl 0x831744b0
	ctx.lr = 0x83175A5C;
	sub_831744B0(ctx, base);
	// 83175A5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175A60: 4BFFEC39  bl 0x83174698
	ctx.lr = 0x83175A64;
	sub_83174698(ctx, base);
	// 83175A64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83175A68: 907F0454  stw r3, 0x454(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1108 as u32), ctx.r[3].u32 ) };
	// 83175A6C: 409A0024  bne cr6, 0x83175a90
	if !ctx.cr[6].eq {
	pc = 0x83175A90; continue 'dispatch;
	}
	// 83175A70: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83175A74: 386BA360  addi r3, r11, -0x5ca0
	ctx.r[3].s64 = ctx.r[11].s64 + -23712;
	// 83175A78: 480076C9  bl 0x8317d140
	ctx.lr = 0x83175A7C;
	sub_8317D140(ctx, base);
	// 83175A7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175A80: 4BFFFDC9  bl 0x83175848
	ctx.lr = 0x83175A84;
	sub_83175848(ctx, base);
	// 83175A84: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83175A88: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83175A8C: 4803271C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83175A90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83175A94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83175A98: 4BFBE001  bl 0x83133a98
	ctx.lr = 0x83175A9C;
	sub_83133A98(ctx, base);
	// 83175A9C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83175AA0: 907F0474  stw r3, 0x474(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1140 as u32), ctx.r[3].u32 ) };
	// 83175AA4: 409A0024  bne cr6, 0x83175ac8
	if !ctx.cr[6].eq {
	pc = 0x83175AC8; continue 'dispatch;
	}
	// 83175AA8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83175AAC: 386BA33C  addi r3, r11, -0x5cc4
	ctx.r[3].s64 = ctx.r[11].s64 + -23748;
	// 83175AB0: 48007691  bl 0x8317d140
	ctx.lr = 0x83175AB4;
	sub_8317D140(ctx, base);
	// 83175AB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175AB8: 4BFFFD91  bl 0x83175848
	ctx.lr = 0x83175ABC;
	sub_83175848(ctx, base);
	// 83175ABC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83175AC0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83175AC4: 480326E4  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83175AC8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83175ACC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 83175AD0: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 83175AD4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83175AD8: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 83175ADC: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 83175AE0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83175AE4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 83175AE8: 93BF0044  stw r29, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[29].u32 ) };
	// 83175AEC: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 83175AF0: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83175AF4: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 83175AF8: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83175AFC: 48003A4D  bl 0x83179548
	ctx.lr = 0x83175B00;
	sub_83179548(ctx, base);
	// 83175B00: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83175B04: 419A000C  beq cr6, 0x83175b10
	if ctx.cr[6].eq {
	pc = 0x83175B10; continue 'dispatch;
	}
	// 83175B08: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 83175B0C: 4800000C  b 0x83175b18
	pc = 0x83175B18; continue 'dispatch;
	// 83175B10: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83175B14: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83175B18: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83175B1C: 93BF0078  stw r29, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[29].u32 ) };
	// 83175B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175B24: 93BF007C  stw r29, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[29].u32 ) };
	// 83175B28: 9BDF0080  stb r30, 0x80(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 83175B2C: 9BDF0081  stb r30, 0x81(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 83175B30: 9BDF0082  stb r30, 0x82(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(130 as u32), ctx.r[30].u8 ) };
	// 83175B34: 9BDF0083  stb r30, 0x83(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(131 as u32), ctx.r[30].u8 ) };
	// 83175B38: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 83175B3C: 4801442D  bl 0x83189f68
	ctx.lr = 0x83175B40;
	sub_83189F68(ctx, base);
	// 83175B40: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83175B44: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175B48: 48014459  bl 0x83189fa0
	ctx.lr = 0x83175B4C;
	sub_83189FA0(ctx, base);
	// 83175B4C: 807F0454  lwz r3, 0x454(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1108 as u32) ) } as u64;
	// 83175B50: 93BF0060  stw r29, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[29].u32 ) };
	// 83175B54: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 83175B58: 48005851  bl 0x8317b3a8
	ctx.lr = 0x83175B5C;
	sub_8317B3A8(ctx, base);
	// 83175B5C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83175B60: 907F004C  stw r3, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[3].u32 ) };
	// 83175B64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175B68: 419A0124  beq cr6, 0x83175c8c
	if ctx.cr[6].eq {
	pc = 0x83175C8C; continue 'dispatch;
	}
	// 83175B6C: 4800099D  bl 0x83176508
	ctx.lr = 0x83175B70;
	sub_83176508(ctx, base);
	// 83175B70: 807F0454  lwz r3, 0x454(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1108 as u32) ) } as u64;
	// 83175B74: 4BFBFC65  bl 0x831357d8
	ctx.lr = 0x83175B78;
	sub_831357D8(ctx, base);
	// 83175B78: 809F004C  lwz r4, 0x4c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83175B7C: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 83175B80: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 83175B84: 4BFBFC95  bl 0x83135818
	ctx.lr = 0x83175B88;
	sub_83135818(ctx, base);
	// 83175B88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175B8C: 4BFFEC1D  bl 0x831747a8
	ctx.lr = 0x83175B90;
	sub_831747A8(ctx, base);
	// 83175B90: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83175B94: 419AFE78  beq cr6, 0x83175a0c
	if ctx.cr[6].eq {
	pc = 0x83175A0C; continue 'dispatch;
	}
	// 83175B98: 80DB000C  lwz r6, 0xc(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 83175B9C: 80BB0008  lwz r5, 8(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 83175BA0: 809F00C0  lwz r4, 0xc0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 83175BA4: 807F00BC  lwz r3, 0xbc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 83175BA8: 48000FF9  bl 0x83176ba0
	ctx.lr = 0x83175BAC;
	sub_83176BA0(ctx, base);
	// 83175BAC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83175BB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83175BB4: 409A0024  bne cr6, 0x83175bd8
	if !ctx.cr[6].eq {
	pc = 0x83175BD8; continue 'dispatch;
	}
	// 83175BB8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83175BBC: 386BA320  addi r3, r11, -0x5ce0
	ctx.r[3].s64 = ctx.r[11].s64 + -23776;
	// 83175BC0: 48007581  bl 0x8317d140
	ctx.lr = 0x83175BC4;
	sub_8317D140(ctx, base);
	// 83175BC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175BC8: 4BFFFC81  bl 0x83175848
	ctx.lr = 0x83175BCC;
	sub_83175848(ctx, base);
	// 83175BCC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83175BD0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83175BD4: 480325D4  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83175BD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175BDC: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83175BE0: 917F00B8  stw r11, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 83175BE4: 480012E5  bl 0x83176ec8
	ctx.lr = 0x83175BE8;
	sub_83176EC8(ctx, base);
	// 83175BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175BEC: 480013B5  bl 0x83176fa0
	ctx.lr = 0x83175BF0;
	sub_83176FA0(ctx, base);
	// 83175BF0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83175BF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175BF8: 917F0410  stw r11, 0x410(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1040 as u32), ctx.r[11].u32 ) };
	// 83175BFC: 48001435  bl 0x83177030
	ctx.lr = 0x83175C00;
	sub_83177030(ctx, base);
	// 83175C00: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83175C04: 419A0024  beq cr6, 0x83175c28
	if ctx.cr[6].eq {
	pc = 0x83175C28; continue 'dispatch;
	}
	// 83175C08: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83175C0C: 386BA2F8  addi r3, r11, -0x5d08
	ctx.r[3].s64 = ctx.r[11].s64 + -23816;
	// 83175C10: 48007531  bl 0x8317d140
	ctx.lr = 0x83175C14;
	sub_8317D140(ctx, base);
	// 83175C14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175C18: 4BFFFC31  bl 0x83175848
	ctx.lr = 0x83175C1C;
	sub_83175848(ctx, base);
	// 83175C1C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83175C20: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83175C24: 48032584  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
	// 83175C28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175C2C: 4800146D  bl 0x83177098
	ctx.lr = 0x83175C30;
	sub_83177098(ctx, base);
	// 83175C30: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175C34: 480020A5  bl 0x83177cd8
	ctx.lr = 0x83175C38;
	sub_83177CD8(ctx, base);
	// 83175C38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175C3C: 48002BB5  bl 0x831787f0
	ctx.lr = 0x83175C40;
	sub_831787F0(ctx, base);
	// 83175C40: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 83175C44: 93BF0000  stw r29, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 83175C48: 93DF050C  stw r30, 0x50c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1292 as u32), ctx.r[30].u32 ) };
	// 83175C4C: 93DF04FC  stw r30, 0x4fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1276 as u32), ctx.r[30].u32 ) };
	// 83175C50: 93DF0500  stw r30, 0x500(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1280 as u32), ctx.r[30].u32 ) };
	// 83175C54: 93DF0504  stw r30, 0x504(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1284 as u32), ctx.r[30].u32 ) };
	// 83175C58: 93DF0524  stw r30, 0x524(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1316 as u32), ctx.r[30].u32 ) };
	// 83175C5C: 917F0520  stw r11, 0x520(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1312 as u32), ctx.r[11].u32 ) };
	// 83175C60: 93DF0528  stw r30, 0x528(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1320 as u32), ctx.r[30].u32 ) };
	// 83175C64: 93DF052C  stw r30, 0x52c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1324 as u32), ctx.r[30].u32 ) };
	// 83175C68: 93DF0530  stw r30, 0x530(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1328 as u32), ctx.r[30].u32 ) };
	// 83175C6C: 93DF0534  stw r30, 0x534(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1332 as u32), ctx.r[30].u32 ) };
	// 83175C70: 7D78C82E  lwzx r11, r24, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 83175C74: 917F053C  stw r11, 0x53c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1340 as u32), ctx.r[11].u32 ) };
	// 83175C78: 7C78C82E  lwzx r3, r24, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 83175C7C: 4BFB941D  bl 0x8312f098
	ctx.lr = 0x83175C80;
	sub_8312F098(ctx, base);
	// 83175C80: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83175C84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175C88: 4098000C  bge cr6, 0x83175c94
	if !ctx.cr[6].lt {
	pc = 0x83175C94; continue 'dispatch;
	}
	// 83175C8C: 4BFFFBBD  bl 0x83175848
	ctx.lr = 0x83175C90;
	sub_83175848(ctx, base);
	// 83175C90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83175C94: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 83175C98: 48032510  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83175CA0 size=236
    let mut pc: u32 = 0x83175CA0;
    'dispatch: loop {
        match pc {
            0x83175CA0 => {
    //   block [0x83175CA0..0x83175D8C)
	// 83175CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83175CA4: 480324B1  bl 0x831a8154
	ctx.lr = 0x83175CA8;
	sub_831A8130(ctx, base);
	// 83175CA8: 9421FD20  stwu r1, -0x2e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-736 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83175CAC: 3FC08345  lis r30, -0x7cbb
	ctx.r[30].s64 = -2092630016;
	// 83175CB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83175CB4: 817EA374  lwz r11, -0x5c8c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83175CB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83175CBC: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83175CC0: 3BABBF68  addi r29, r11, -0x4098
	ctx.r[29].s64 = ctx.r[11].s64 + -16536;
	// 83175CC4: 419A008C  beq cr6, 0x83175d50
	if ctx.cr[6].eq {
	pc = 0x83175D50; continue 'dispatch;
	}
	// 83175CC8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83175CCC: 839F0030  lwz r28, 0x30(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 83175CD0: 837F002C  lwz r27, 0x2c(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 83175CD4: 38610090  addi r3, r1, 0x90
	ctx.r[3].s64 = ctx.r[1].s64 + 144;
	// 83175CD8: 388BA464  addi r4, r11, -0x5b9c
	ctx.r[4].s64 = ctx.r[11].s64 + -23452;
	// 83175CDC: 835F0024  lwz r26, 0x24(r31)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83175CE0: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83175CE4: 833F0020  lwz r25, 0x20(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 83175CE8: 831F001C  lwz r24, 0x1c(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 83175CEC: 82FF0018  lwz r23, 0x18(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 83175CF0: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 83175CF4: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 83175CF8: 811F000C  lwz r8, 0xc(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 83175CFC: 80FF0008  lwz r7, 8(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83175D00: 80DF0004  lwz r6, 4(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 83175D04: 80BF0000  lwz r5, 0(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83175D08: 93810084  stw r28, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[28].u32 ) };
	// 83175D0C: 9361007C  stw r27, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[27].u32 ) };
	// 83175D10: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 83175D14: 9341006C  stw r26, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[26].u32 ) };
	// 83175D18: 93210064  stw r25, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[25].u32 ) };
	// 83175D1C: 9301005C  stw r24, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[24].u32 ) };
	// 83175D20: 92E10054  stw r23, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[23].u32 ) };
	// 83175D24: 48032DB5  bl 0x831a8ad8
	ctx.lr = 0x83175D28;
	sub_831A8AD8(ctx, base);
	// 83175D28: 807EA374  lwz r3, -0x5c8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83175D2C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83175D30: 419A0020  beq cr6, 0x83175d50
	if ctx.cr[6].eq {
	pc = 0x83175D50; continue 'dispatch;
	}
	// 83175D34: 39610090  addi r11, r1, 0x90
	ctx.r[11].s64 = ctx.r[1].s64 + 144;
	// 83175D38: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 83175D3C: 917D000C  stw r11, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83175D40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83175D44: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83175D48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83175D4C: 4E800421  bctrl
	ctx.lr = 0x83175D50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83175D50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175D54: 4BFFFB65  bl 0x831758b8
	ctx.lr = 0x83175D58;
	sub_831758B8(ctx, base);
	// 83175D58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83175D5C: 807EA374  lwz r3, -0x5c8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83175D60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83175D64: 419A001C  beq cr6, 0x83175d80
	if ctx.cr[6].eq {
	pc = 0x83175D80; continue 'dispatch;
	}
	// 83175D68: 93FD0074  stw r31, 0x74(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(116 as u32), ctx.r[31].u32 ) };
	// 83175D6C: 389D006C  addi r4, r29, 0x6c
	ctx.r[4].s64 = ctx.r[29].s64 + 108;
	// 83175D70: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83175D74: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83175D78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83175D7C: 4E800421  bctrl
	ctx.lr = 0x83175D80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83175D80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175D84: 382102E0  addi r1, r1, 0x2e0
	ctx.r[1].s64 = ctx.r[1].s64 + 736;
	// 83175D88: 4803241C  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83175D90 size=36
    let mut pc: u32 = 0x83175D90;
    'dispatch: loop {
        match pc {
            0x83175D90 => {
    //   block [0x83175D90..0x83175DB4)
	// 83175D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83175D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83175D98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83175D9C: 4BFAF43D  bl 0x831251d8
	ctx.lr = 0x83175DA0;
	sub_831251D8(ctx, base);
	// 83175DA0: 1C6303E8  mulli r3, r3, 0x3e8
	ctx.r[3].s64 = ctx.r[3].s64 * 1000;
	// 83175DA4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83175DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83175DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83175DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83175DB8 size=84
    let mut pc: u32 = 0x83175DB8;
    'dispatch: loop {
        match pc {
            0x83175DB8 => {
    //   block [0x83175DB8..0x83175E0C)
	// 83175DB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83175DBC: 480323AD  bl 0x831a8168
	ctx.lr = 0x83175DC0;
	sub_831A8130(ctx, base);
	// 83175DC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83175DC4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83175DC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83175DCC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83175DD0: 386BD4B8  addi r3, r11, -0x2b48
	ctx.r[3].s64 = ctx.r[11].s64 + -11080;
	// 83175DD4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83175DD8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83175DDC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83175DE0: 4BFFCEB1  bl 0x83172c90
	ctx.lr = 0x83175DE4;
	sub_83172C90(ctx, base);
	// 83175DE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175DE8: 48000AC1  bl 0x831768a8
	ctx.lr = 0x83175DEC;
	sub_831768A8(ctx, base);
	// 83175DEC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83175DF0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83175DF4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83175DF8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83175DFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175E00: 4BFFD379  bl 0x83173178
	ctx.lr = 0x83175E04;
	sub_83173178(ctx, base);
	// 83175E04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83175E08: 480323B0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175E10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83175E10 size=52
    let mut pc: u32 = 0x83175E10;
    'dispatch: loop {
        match pc {
            0x83175E10 => {
    //   block [0x83175E10..0x83175E44)
	// 83175E10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83175E14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83175E18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83175E1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83175E20: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83175E24: 48000A85  bl 0x831768a8
	ctx.lr = 0x83175E28;
	sub_831768A8(ctx, base);
	// 83175E28: 387F04A0  addi r3, r31, 0x4a0
	ctx.r[3].s64 = ctx.r[31].s64 + 1184;
	// 83175E2C: 4BFFD4CD  bl 0x831732f8
	ctx.lr = 0x83175E30;
	sub_831732F8(ctx, base);
	// 83175E30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83175E34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83175E38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83175E3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83175E40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83175E48 size=4
    let mut pc: u32 = 0x83175E48;
    'dispatch: loop {
        match pc {
            0x83175E48 => {
    //   block [0x83175E48..0x83175E4C)
	// 83175E48: 4808E1E0  b 0x83204028
	sub_83204028(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83175E50 size=4
    let mut pc: u32 = 0x83175E50;
    'dispatch: loop {
        match pc {
            0x83175E50 => {
    //   block [0x83175E50..0x83175E54)
	// 83175E50: 4808DC68  b 0x83203ab8
	sub_83203AB8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175E58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83175E58 size=20
    let mut pc: u32 = 0x83175E58;
    'dispatch: loop {
        match pc {
            0x83175E58 => {
    //   block [0x83175E58..0x83175E6C)
	// 83175E58: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 83175E5C: 7C862378  mr r6, r4
	ctx.r[6].u64 = ctx.r[4].u64;
	// 83175E60: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83175E64: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 83175E68: 4808E268  b 0x832040d0
	sub_832040D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83175E70 size=4
    let mut pc: u32 = 0x83175E70;
    'dispatch: loop {
        match pc {
            0x83175E70 => {
    //   block [0x83175E70..0x83175E74)
	// 83175E70: 4808E008  b 0x83203e78
	sub_83203E78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83175E78 size=4
    let mut pc: u32 = 0x83175E78;
    'dispatch: loop {
        match pc {
            0x83175E78 => {
    //   block [0x83175E78..0x83175E7C)
	// 83175E78: 4808E038  b 0x83203eb0
	sub_83203EB0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83175E80 size=4
    let mut pc: u32 = 0x83175E80;
    'dispatch: loop {
        match pc {
            0x83175E80 => {
    //   block [0x83175E80..0x83175E84)
	// 83175E80: 4808DAE8  b 0x83203968
	sub_83203968(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83175E88 size=164
    let mut pc: u32 = 0x83175E88;
    'dispatch: loop {
        match pc {
            0x83175E88 => {
    //   block [0x83175E88..0x83175EC8)
	// 83175E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83175E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83175E90: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83175E94: 4808D1CD  bl 0x83203060
	ctx.lr = 0x83175E98;
	sub_83203060(ctx, base);
	// 83175E98: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 83175E9C: 4199007C  bgt cr6, 0x83175f18
	if ctx.cr[6].gt {
	pc = 0x83175F18; continue 'dispatch;
	}
	// 83175EA0: 3D808317  lis r12, -0x7ce9
	ctx.r[12].s64 = -2095644672;
	// 83175EA4: 398C5EB8  addi r12, r12, 0x5eb8
	ctx.r[12].s64 = ctx.r[12].s64 + 24248;
	// 83175EA8: 5460103A  slwi r0, r3, 2
	ctx.r[0].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83175EAC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83175EB0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83175EB4: 4E800420  bctr
	match ctx.r[3].u64 {
		0 => {
	pc = 0x83175EC8; continue 'dispatch;
		},
		1 => {
	pc = 0x83175EDC; continue 'dispatch;
		},
		2 => {
	pc = 0x83175EF0; continue 'dispatch;
		},
		3 => {
	pc = 0x83175F04; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 83175EB8: 83175EC8  lwz r24, 0x5ec8(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24264 as u32) ) } as u64;
	// 83175EBC: 83175EDC  lwz r24, 0x5edc(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24284 as u32) ) } as u64;
	// 83175EC0: 83175EF0  lwz r24, 0x5ef0(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24304 as u32) ) } as u64;
	// 83175EC4: 83175F04  lwz r24, 0x5f04(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(24324 as u32) ) } as u64;
            }
            0x83175EC8 => {
    //   block [0x83175EC8..0x83175EDC)
	// 83175EC8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83175ECC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83175ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83175ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83175ED8: 4E800020  blr
	return;
            }
            0x83175EDC => {
    //   block [0x83175EDC..0x83175EF0)
	// 83175EDC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83175EE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83175EE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83175EE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83175EEC: 4E800020  blr
	return;
            }
            0x83175EF0 => {
    //   block [0x83175EF0..0x83175F04)
	// 83175EF0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83175EF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83175EF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83175EFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83175F00: 4E800020  blr
	return;
            }
            0x83175F04 => {
    //   block [0x83175F04..0x83175F2C)
	// 83175F04: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 83175F08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83175F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83175F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83175F14: 4E800020  blr
	return;
	// 83175F18: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83175F1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83175F20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83175F24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83175F28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175F30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83175F30 size=4
    let mut pc: u32 = 0x83175F30;
    'dispatch: loop {
        match pc {
            0x83175F30 => {
    //   block [0x83175F30..0x83175F34)
	// 83175F30: 4808DA70  b 0x832039a0
	sub_832039A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175F38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83175F38 size=60
    let mut pc: u32 = 0x83175F38;
    'dispatch: loop {
        match pc {
            0x83175F38 => {
    //   block [0x83175F38..0x83175F74)
	// 83175F38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83175F3C: 48032231  bl 0x831a816c
	ctx.lr = 0x83175F40;
	sub_831A8130(ctx, base);
	// 83175F40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83175F44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83175F48: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83175F4C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83175F50: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83175F54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83175F58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83175F5C: 4808DADD  bl 0x83203a38
	ctx.lr = 0x83175F60;
	sub_83203A38(ctx, base);
	// 83175F60: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 83175F64: 2F1F0003  cmpwi cr6, r31, 3
	ctx.cr[6].compare_i32(ctx.r[31].s32, 3, &mut ctx.xer);
	// 83175F68: 4198FFE8  blt cr6, 0x83175f50
	if ctx.cr[6].lt {
	pc = 0x83175F50; continue 'dispatch;
	}
	// 83175F6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83175F70: 4803224C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83175F78 size=40
    let mut pc: u32 = 0x83175F78;
    'dispatch: loop {
        match pc {
            0x83175F78 => {
    //   block [0x83175F78..0x83175FA0)
	// 83175F78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83175F7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83175F80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83175F84: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83175F88: 4808D1B1  bl 0x83203138
	ctx.lr = 0x83175F8C;
	sub_83203138(ctx, base);
	// 83175F8C: 4BFB7A45  bl 0x8312d9d0
	ctx.lr = 0x83175F90;
	sub_8312D9D0(ctx, base);
	// 83175F90: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83175F94: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83175F98: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83175F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83175FA0 size=4
    let mut pc: u32 = 0x83175FA0;
    'dispatch: loop {
        match pc {
            0x83175FA0 => {
    //   block [0x83175FA0..0x83175FA4)
	// 83175FA0: 4808D218  b 0x832031b8
	sub_832031B8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83175FA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83175FA8 size=164
    let mut pc: u32 = 0x83175FA8;
    'dispatch: loop {
        match pc {
            0x83175FA8 => {
    //   block [0x83175FA8..0x8317604C)
	// 83175FA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83175FAC: 480321BD  bl 0x831a8168
	ctx.lr = 0x83175FB0;
	sub_831A8130(ctx, base);
	// 83175FB0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83175FB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83175FB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83175FBC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83175FC0: 4808D0F9  bl 0x832030b8
	ctx.lr = 0x83175FC4;
	sub_832030B8(ctx, base);
	// 83175FC4: 4BFFFDCD  bl 0x83175d90
	ctx.lr = 0x83175FC8;
	sub_83175D90(ctx, base);
	// 83175FC8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83175FCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83175FD0: 4808D091  bl 0x83203060
	ctx.lr = 0x83175FD4;
	sub_83203060(ctx, base);
	// 83175FD4: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83175FD8: 419A003C  beq cr6, 0x83176014
	if ctx.cr[6].eq {
	pc = 0x83176014; continue 'dispatch;
	}
	// 83175FDC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83175FE0: 815F0294  lwz r10, 0x294(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 83175FE4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83175FE8: 41980008  blt cr6, 0x83175ff0
	if ctx.cr[6].lt {
	pc = 0x83175FF0; continue 'dispatch;
	}
	// 83175FEC: 917F0294  stw r11, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[11].u32 ) };
	// 83175FF0: FB9F0298  std r28, 0x298(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(664 as u32), ctx.r[28].u64 ) };
	// 83175FF4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83175FF8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83175FFC: 409A0048  bne cr6, 0x83176044
	if !ctx.cr[6].eq {
	pc = 0x83176044; continue 'dispatch;
	}
	// 83176000: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 83176004: 616BBB80  ori r11, r11, 0xbb80
	ctx.r[11].u64 = ctx.r[11].u64 | 48000;
	// 83176008: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317600C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83176010: 480321A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83176014: E97F0298  ld r11, 0x298(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(664 as u32) ) };
	// 83176018: 3D40000F  lis r10, 0xf
	ctx.r[10].s64 = 983040;
	// 8317601C: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83176020: 7D6BE050  subf r11, r11, r28
	ctx.r[11].s64 = ctx.r[28].s64 - ctx.r[11].s64;
	// 83176024: 7D0807B4  extsw r8, r8
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
	// 83176028: 61494240  ori r9, r10, 0x4240
	ctx.r[9].u64 = ctx.r[10].u64 | 16960;
	// 8317602C: 815F0294  lwz r10, 0x294(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 83176030: 7D6B41D2  mulld r11, r11, r8
	ctx.r[11].s64 = ctx.r[11].s64 * ctx.r[8].s64;
	// 83176034: 7D6B4BD2  divd r11, r11, r9
	ctx.r[11].s64 = ctx.r[11].s64 / ctx.r[9].s64;
	// 83176038: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8317603C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83176040: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83176044: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83176048: 48032170  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176050 size=136
    let mut pc: u32 = 0x83176050;
    'dispatch: loop {
        match pc {
            0x83176050 => {
    //   block [0x83176050..0x831760D8)
	// 83176050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176054: 48032119  bl 0x831a816c
	ctx.lr = 0x83176058;
	sub_831A8130(ctx, base);
	// 83176058: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317605C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83176060: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83176064: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83176068: 4BFFCD31  bl 0x83172d98
	ctx.lr = 0x8317606C;
	sub_83172D98(ctx, base);
	// 8317606C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83176070: 419A0018  beq cr6, 0x83176088
	if ctx.cr[6].eq {
	pc = 0x83176088; continue 'dispatch;
	}
	// 83176074: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176078: 386BA4A8  addi r3, r11, -0x5b58
	ctx.r[3].s64 = ctx.r[11].s64 + -23384;
	// 8317607C: 480070C5  bl 0x8317d140
	ctx.lr = 0x83176080;
	sub_8317D140(ctx, base);
	// 83176080: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83176084: 48032138  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83176088: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8317608C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83176090: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83176094: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176098: 4BFFFD21  bl 0x83175db8
	ctx.lr = 0x8317609C;
	sub_83175DB8(ctx, base);
	// 8317609C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831760A0: 4BFFCCF9  bl 0x83172d98
	ctx.lr = 0x831760A4;
	sub_83172D98(ctx, base);
	// 831760A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831760A8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 831760AC: 419A0024  beq cr6, 0x831760d0
	if ctx.cr[6].eq {
	pc = 0x831760D0; continue 'dispatch;
	}
	// 831760B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831760B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831760B8: 4808E261  bl 0x83204318
	ctx.lr = 0x831760BC;
	sub_83204318(ctx, base);
	// 831760BC: 3D608317  lis r11, -0x7ce9
	ctx.r[11].s64 = -2095644672;
	// 831760C0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 831760C4: 388B5FA8  addi r4, r11, 0x5fa8
	ctx.r[4].s64 = ctx.r[11].s64 + 24488;
	// 831760C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831760CC: 4BFFD785  bl 0x83173850
	ctx.lr = 0x831760D0;
	sub_83173850(ctx, base);
	// 831760D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831760D4: 480320E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831760D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831760D8 size=92
    let mut pc: u32 = 0x831760D8;
    'dispatch: loop {
        match pc {
            0x831760D8 => {
    //   block [0x831760D8..0x83176134)
	// 831760D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831760DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831760E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831760E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831760E8: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 831760EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831760F0: 4BFFC799  bl 0x83172888
	ctx.lr = 0x831760F4;
	sub_83172888(ctx, base);
	// 831760F4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831760F8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831760FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176100: 4BFFD751  bl 0x83173850
	ctx.lr = 0x83176104;
	sub_83173850(ctx, base);
	// 83176104: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176108: 4BFFCC91  bl 0x83172d98
	ctx.lr = 0x8317610C;
	sub_83172D98(ctx, base);
	// 8317610C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83176110: 419A0008  beq cr6, 0x83176118
	if ctx.cr[6].eq {
	pc = 0x83176118; continue 'dispatch;
	}
	// 83176114: 4808E0F5  bl 0x83204208
	ctx.lr = 0x83176118;
	sub_83204208(ctx, base);
	// 83176118: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317611C: 4BFFFCF5  bl 0x83175e10
	ctx.lr = 0x83176120;
	sub_83175E10(ctx, base);
	// 83176120: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176124: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176128: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317612C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176138 size=88
    let mut pc: u32 = 0x83176138;
    'dispatch: loop {
        match pc {
            0x83176138 => {
    //   block [0x83176138..0x83176190)
	// 83176138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317613C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83176140: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176144: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 83176148: 4800EF31  bl 0x83185078
	ctx.lr = 0x8317614C;
	sub_83185078(ctx, base);
	// 8317614C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83176150: 419A002C  beq cr6, 0x8317617c
	if ctx.cr[6].eq {
	pc = 0x8317617C; continue 'dispatch;
	}
	// 83176154: 3860FEC9  li r3, -0x137
	ctx.r[3].s64 = -311;
	// 83176158: 4BFFD471  bl 0x831735c8
	ctx.lr = 0x8317615C;
	sub_831735C8(ctx, base);
	// 8317615C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176160: 386BA4F4  addi r3, r11, -0x5b0c
	ctx.r[3].s64 = ctx.r[11].s64 + -23308;
	// 83176164: 48006FDD  bl 0x8317d140
	ctx.lr = 0x83176168;
	sub_8317D140(ctx, base);
	// 83176168: 3860FEC9  li r3, -0x137
	ctx.r[3].s64 = -311;
	// 8317616C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176170: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176174: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176178: 4E800020  blr
	return;
	// 8317617C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83176180: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176184: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176188: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317618C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176190 size=88
    let mut pc: u32 = 0x83176190;
    'dispatch: loop {
        match pc {
            0x83176190 => {
    //   block [0x83176190..0x831761E8)
	// 83176190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176194: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83176198: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317619C: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 831761A0: 4800DAB9  bl 0x83183c58
	ctx.lr = 0x831761A4;
	sub_83183C58(ctx, base);
	// 831761A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831761A8: 419A002C  beq cr6, 0x831761d4
	if ctx.cr[6].eq {
	pc = 0x831761D4; continue 'dispatch;
	}
	// 831761AC: 3860FECD  li r3, -0x133
	ctx.r[3].s64 = -307;
	// 831761B0: 4BFFD419  bl 0x831735c8
	ctx.lr = 0x831761B4;
	sub_831735C8(ctx, base);
	// 831761B4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831761B8: 386BA510  addi r3, r11, -0x5af0
	ctx.r[3].s64 = ctx.r[11].s64 + -23280;
	// 831761BC: 48006F85  bl 0x8317d140
	ctx.lr = 0x831761C0;
	sub_8317D140(ctx, base);
	// 831761C0: 3860FECD  li r3, -0x133
	ctx.r[3].s64 = -307;
	// 831761C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831761C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831761CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831761D0: 4E800020  blr
	return;
	// 831761D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831761D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831761DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831761E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831761E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831761E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831761E8 size=56
    let mut pc: u32 = 0x831761E8;
    'dispatch: loop {
        match pc {
            0x831761E8 => {
    //   block [0x831761E8..0x83176220)
	// 831761E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831761EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831761F0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831761F4: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 831761F8: 4800DB21  bl 0x83183d18
	ctx.lr = 0x831761FC;
	sub_83183D18(ctx, base);
	// 831761FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83176200: 419A0010  beq cr6, 0x83176210
	if ctx.cr[6].eq {
	pc = 0x83176210; continue 'dispatch;
	}
	// 83176204: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176208: 386BA52C  addi r3, r11, -0x5ad4
	ctx.r[3].s64 = ctx.r[11].s64 + -23252;
	// 8317620C: 48006F35  bl 0x8317d140
	ctx.lr = 0x83176210;
	sub_8317D140(ctx, base);
	// 83176210: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176214: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176218: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317621C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176220 size=16
    let mut pc: u32 = 0x83176220;
    'dispatch: loop {
        match pc {
            0x83176220 => {
    //   block [0x83176220..0x83176230)
	// 83176220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83176224: 9163008C  stw r11, 0x8c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 83176228: 91630090  stw r11, 0x90(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8317622C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176230 size=208
    let mut pc: u32 = 0x83176230;
    'dispatch: loop {
        match pc {
            0x83176230 => {
    //   block [0x83176230..0x83176300)
	// 83176230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176234: 48031F39  bl 0x831a816c
	ctx.lr = 0x83176238;
	sub_831A8130(ctx, base);
	// 83176238: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317623C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83176240: 83BF0048  lwz r29, 0x48(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83176244: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83176248: 419A00B0  beq cr6, 0x831762f8
	if ctx.cr[6].eq {
	pc = 0x831762F8; continue 'dispatch;
	}
	// 8317624C: 48007C35  bl 0x8317de80
	ctx.lr = 0x83176250;
	sub_8317DE80(ctx, base);
	// 83176250: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83176254: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83176258: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 8317625C: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 83176260: 4800EA31  bl 0x83184c90
	ctx.lr = 0x83176264;
	sub_83184C90(ctx, base);
	// 83176264: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83176268: 419A0018  beq cr6, 0x83176280
	if ctx.cr[6].eq {
	pc = 0x83176280; continue 'dispatch;
	}
	// 8317626C: 3860FECC  li r3, -0x134
	ctx.r[3].s64 = -308;
	// 83176270: 4BFFD359  bl 0x831735c8
	ctx.lr = 0x83176274;
	sub_831735C8(ctx, base);
	// 83176274: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176278: 386BA554  addi r3, r11, -0x5aac
	ctx.r[3].s64 = ctx.r[11].s64 + -23212;
	// 8317627C: 48006EC5  bl 0x8317d140
	ctx.lr = 0x83176280;
	sub_8317D140(ctx, base);
	// 83176280: 387F04A0  addi r3, r31, 0x4a0
	ctx.r[3].s64 = ctx.r[31].s64 + 1184;
	// 83176284: 93BF0048  stw r29, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[29].u32 ) };
	// 83176288: 4BFFCD39  bl 0x83172fc0
	ctx.lr = 0x8317628C;
	sub_83172FC0(ctx, base);
	// 8317628C: 387F04C4  addi r3, r31, 0x4c4
	ctx.r[3].s64 = ctx.r[31].s64 + 1220;
	// 83176290: 4BFFCD31  bl 0x83172fc0
	ctx.lr = 0x83176294;
	sub_83172FC0(ctx, base);
	// 83176294: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83176298: 93DF04F0  stw r30, 0x4f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1264 as u32), ctx.r[30].u32 ) };
	// 8317629C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831762A0: 419A0008  beq cr6, 0x831762a8
	if ctx.cr[6].eq {
	pc = 0x831762A8; continue 'dispatch;
	}
	// 831762A4: 48005175  bl 0x8317b418
	ctx.lr = 0x831762A8;
	sub_8317B418(ctx, base);
	// 831762A8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 831762AC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831762B0: 419A0008  beq cr6, 0x831762b8
	if ctx.cr[6].eq {
	pc = 0x831762B8; continue 'dispatch;
	}
	// 831762B4: 4BFBF90D  bl 0x83135bc0
	ctx.lr = 0x831762B8;
	sub_83135BC0(ctx, base);
	// 831762B8: 817F0504  lwz r11, 0x504(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1284 as u32) ) } as u64;
	// 831762BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831762C0: 409A0020  bne cr6, 0x831762e0
	if !ctx.cr[6].eq {
	pc = 0x831762E0; continue 'dispatch;
	}
	// 831762C4: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 831762C8: 93DF050C  stw r30, 0x50c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1292 as u32), ctx.r[30].u32 ) };
	// 831762CC: 93DF0508  stw r30, 0x508(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1288 as u32), ctx.r[30].u32 ) };
	// 831762D0: 93DF0500  stw r30, 0x500(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1280 as u32), ctx.r[30].u32 ) };
	// 831762D4: 93DF0524  stw r30, 0x524(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1316 as u32), ctx.r[30].u32 ) };
	// 831762D8: 917F0520  stw r11, 0x520(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1312 as u32), ctx.r[11].u32 ) };
	// 831762DC: 4800000C  b 0x831762e8
	pc = 0x831762E8; continue 'dispatch;
	// 831762E0: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 831762E4: 917F050C  stw r11, 0x50c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1292 as u32), ctx.r[11].u32 ) };
	// 831762E8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831762EC: 93DF0538  stw r30, 0x538(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1336 as u32), ctx.r[30].u32 ) };
	// 831762F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831762F4: 480027ED  bl 0x83178ae0
	ctx.lr = 0x831762F8;
	sub_83178AE0(ctx, base);
	// 831762F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831762FC: 48031EC0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176300 size=120
    let mut pc: u32 = 0x83176300;
    'dispatch: loop {
        match pc {
            0x83176300 => {
    //   block [0x83176300..0x83176378)
	// 83176300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83176308: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317630C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176310: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83176314: 4BFFC435  bl 0x83172748
	ctx.lr = 0x83176318;
	sub_83172748(ctx, base);
	// 83176318: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317631C: 419A0024  beq cr6, 0x83176340
	if ctx.cr[6].eq {
	pc = 0x83176340; continue 'dispatch;
	}
	// 83176320: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176324: 386BA574  addi r3, r11, -0x5a8c
	ctx.r[3].s64 = ctx.r[11].s64 + -23180;
	// 83176328: 48006E19  bl 0x8317d140
	ctx.lr = 0x8317632C;
	sub_8317D140(ctx, base);
	// 8317632C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176338: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317633C: 4E800020  blr
	return;
	// 83176340: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176344: 4BFFFEED  bl 0x83176230
	ctx.lr = 0x83176348;
	sub_83176230(ctx, base);
	// 83176348: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317634C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176350: 480027A9  bl 0x83178af8
	ctx.lr = 0x83176354;
	sub_83178AF8(ctx, base);
	// 83176354: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83176358: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317635C: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 83176360: 4BFBF861  bl 0x83135bc0
	ctx.lr = 0x83176364;
	sub_83135BC0(ctx, base);
	// 83176364: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317636C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176370: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176378 size=24
    let mut pc: u32 = 0x83176378;
    'dispatch: loop {
        match pc {
            0x83176378 => {
    //   block [0x83176378..0x83176390)
	// 83176378: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317637C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83176380: 814B01F0  lwz r10, 0x1f0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(496 as u32) ) } as u64;
	// 83176384: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83176388: 419A0008  beq cr6, 0x83176390
	if ctx.cr[6].eq {
		sub_83176390(ctx, base);
		return;
	}
	// 8317638C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176390 size=52
    let mut pc: u32 = 0x83176390;
    'dispatch: loop {
        match pc {
            0x83176390 => {
    //   block [0x83176390..0x831763C4)
	// 83176390: 814B04A0  lwz r10, 0x4a0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1184 as u32) ) } as u64;
	// 83176394: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83176398: 409A0020  bne cr6, 0x831763b8
	if !ctx.cr[6].eq {
	pc = 0x831763B8; continue 'dispatch;
	}
	// 8317639C: 814B04BC  lwz r10, 0x4bc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1212 as u32) ) } as u64;
	// 831763A0: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831763A4: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831763A8: 814A01F4  lwz r10, 0x1f4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(500 as u32) ) } as u64;
	// 831763AC: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 831763B0: 409A0008  bne cr6, 0x831763b8
	if !ctx.cr[6].eq {
	pc = 0x831763B8; continue 'dispatch;
	}
	// 831763B4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831763B8: 814B04C4  lwz r10, 0x4c4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1220 as u32) ) } as u64;
	// 831763BC: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 831763C0: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831763C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831763C4 size=24
    let mut pc: u32 = 0x831763C4;
    'dispatch: loop {
        match pc {
            0x831763C4 => {
    //   block [0x831763C4..0x831763DC)
	// 831763C4: 814B04E0  lwz r10, 0x4e0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1248 as u32) ) } as u64;
	// 831763C8: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831763CC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831763D0: 816B01F4  lwz r11, 0x1f4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(500 as u32) ) } as u64;
	// 831763D4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831763D8: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831763DC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831763DC size=8
    let mut pc: u32 = 0x831763DC;
    'dispatch: loop {
        match pc {
            0x831763DC => {
    //   block [0x831763DC..0x831763E4)
	// 831763DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831763E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831763E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831763E8 size=284
    let mut pc: u32 = 0x831763E8;
    'dispatch: loop {
        match pc {
            0x831763E8 => {
    //   block [0x831763E8..0x83176504)
	// 831763E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831763EC: 48031D81  bl 0x831a816c
	ctx.lr = 0x831763F0;
	sub_831A8130(ctx, base);
	// 831763F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831763F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831763F8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 831763FC: 4BFFC34D  bl 0x83172748
	ctx.lr = 0x83176400;
	sub_83172748(ctx, base);
	// 83176400: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83176404: 419A0018  beq cr6, 0x8317641c
	if ctx.cr[6].eq {
	pc = 0x8317641C; continue 'dispatch;
	}
	// 83176408: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317640C: 386BA5C0  addi r3, r11, -0x5a40
	ctx.r[3].s64 = ctx.r[11].s64 + -23104;
	// 83176410: 48006D31  bl 0x8317d140
	ctx.lr = 0x83176414;
	sub_8317D140(ctx, base);
	// 83176414: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83176418: 48031DA4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317641C: 897F0082  lbz r11, 0x82(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(130 as u32) ) } as u64;
	// 83176420: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83176424: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83176428: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317642C: 409A000C  bne cr6, 0x83176438
	if !ctx.cr[6].eq {
	pc = 0x83176438; continue 'dispatch;
	}
	// 83176430: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 83176434: 419A00C8  beq cr6, 0x831764fc
	if ctx.cr[6].eq {
	pc = 0x831764FC; continue 'dispatch;
	}
	// 83176438: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317643C: 409A001C  bne cr6, 0x83176458
	if !ctx.cr[6].eq {
	pc = 0x83176458; continue 'dispatch;
	}
	// 83176440: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 83176444: 409A0014  bne cr6, 0x83176458
	if !ctx.cr[6].eq {
	pc = 0x83176458; continue 'dispatch;
	}
	// 83176448: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317644C: 4BFFFF2D  bl 0x83176378
	ctx.lr = 0x83176450;
	sub_83176378(ctx, base);
	// 83176450: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83176454: 409A00A8  bne cr6, 0x831764fc
	if !ctx.cr[6].eq {
	pc = 0x831764FC; continue 'dispatch;
	}
	// 83176458: 4BFFD081  bl 0x831734d8
	ctx.lr = 0x8317645C;
	sub_831734D8(ctx, base);
	// 8317645C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83176460: 409A003C  bne cr6, 0x8317649c
	if !ctx.cr[6].eq {
	pc = 0x8317649C; continue 'dispatch;
	}
	// 83176464: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83176468: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317646C: 409A0030  bne cr6, 0x8317649c
	if !ctx.cr[6].eq {
	pc = 0x8317649C; continue 'dispatch;
	}
	// 83176470: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83176474: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83176478: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317647C: 4800304D  bl 0x831794c8
	ctx.lr = 0x83176480;
	sub_831794C8(ctx, base);
	// 83176480: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83176484: 409A0010  bne cr6, 0x83176494
	if !ctx.cr[6].eq {
	pc = 0x83176494; continue 'dispatch;
	}
	// 83176488: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317648C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83176490: 409A000C  bne cr6, 0x8317649c
	if !ctx.cr[6].eq {
	pc = 0x8317649C; continue 'dispatch;
	}
	// 83176494: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176498: 480079E9  bl 0x8317de80
	ctx.lr = 0x8317649C;
	sub_8317DE80(ctx, base);
	// 8317649C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831764A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831764A4: 4800ECAD  bl 0x83185150
	ctx.lr = 0x831764A8;
	sub_83185150(ctx, base);
	// 831764A8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831764AC: 419A0034  beq cr6, 0x831764e0
	if ctx.cr[6].eq {
	pc = 0x831764E0; continue 'dispatch;
	}
	// 831764B0: 3860FECA  li r3, -0x136
	ctx.r[3].s64 = -310;
	// 831764B4: 4BFFD115  bl 0x831735c8
	ctx.lr = 0x831764B8;
	sub_831735C8(ctx, base);
	// 831764B8: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 831764BC: 409A0010  bne cr6, 0x831764cc
	if !ctx.cr[6].eq {
	pc = 0x831764CC; continue 'dispatch;
	}
	// 831764C0: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 831764C4: 388BBDBC  addi r4, r11, -0x4244
	ctx.r[4].s64 = ctx.r[11].s64 + -16964;
	// 831764C8: 4800000C  b 0x831764d4
	pc = 0x831764D4; continue 'dispatch;
	// 831764CC: 3D608204  lis r11, -0x7dfc
	ctx.r[11].s64 = -2113667072;
	// 831764D0: 388BBDB8  addi r4, r11, -0x4248
	ctx.r[4].s64 = ctx.r[11].s64 + -16968;
	// 831764D4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831764D8: 386BA59C  addi r3, r11, -0x5a64
	ctx.r[3].s64 = ctx.r[11].s64 + -23140;
	// 831764DC: 48006C65  bl 0x8317d140
	ctx.lr = 0x831764E0;
	sub_8317D140(ctx, base);
	// 831764E0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831764E4: 387F04A0  addi r3, r31, 0x4a0
	ctx.r[3].s64 = ctx.r[31].s64 + 1184;
	// 831764E8: 4BFFCBA1  bl 0x83173088
	ctx.lr = 0x831764EC;
	sub_83173088(ctx, base);
	// 831764EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831764F0: 387F04C4  addi r3, r31, 0x4c4
	ctx.r[3].s64 = ctx.r[31].s64 + 1220;
	// 831764F4: 4BFFCB95  bl 0x83173088
	ctx.lr = 0x831764F8;
	sub_83173088(ctx, base);
	// 831764F8: 9BBF0082  stb r29, 0x82(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(130 as u32), ctx.r[29].u8 ) };
	// 831764FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83176500: 48031CBC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83176508 size=80
    let mut pc: u32 = 0x83176508;
    'dispatch: loop {
        match pc {
            0x83176508 => {
    //   block [0x83176508..0x83176558)
	// 83176508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317650C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83176510: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176514: 80A3045C  lwz r5, 0x45c(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1116 as u32) ) } as u64;
	// 83176518: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8317651C: 7CAB07B4  extsw r11, r5
	ctx.r[11].s64 = ctx.r[5].s32 as i64;
	// 83176520: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 83176524: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176528: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 8317652C: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 83176530: C80BA5E8  lfd f0, -0x5a18(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(-23064 as u32) ) };
	// 83176534: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 83176538: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 8317653C: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 83176540: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83176544: 4BFFC1BD  bl 0x83172700
	ctx.lr = 0x83176548;
	sub_83172700(ctx, base);
	// 83176548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317654C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176558 size=72
    let mut pc: u32 = 0x83176558;
    'dispatch: loop {
        match pc {
            0x83176558 => {
    //   block [0x83176558..0x831765A0)
	// 83176558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317655C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83176560: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83176564: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176568: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317656C: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 83176570: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83176574: 419A0018  beq cr6, 0x8317658c
	if ctx.cr[6].eq {
	pc = 0x8317658C; continue 'dispatch;
	}
	// 83176578: 48004ED9  bl 0x8317b450
	ctx.lr = 0x8317657C;
	sub_8317B450(ctx, base);
	// 8317657C: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83176580: 409A000C  bne cr6, 0x8317658c
	if !ctx.cr[6].eq {
	pc = 0x8317658C; continue 'dispatch;
	}
	// 83176584: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176588: 4BFFFC61  bl 0x831761e8
	ctx.lr = 0x8317658C;
	sub_831761E8(ctx, base);
	// 8317658C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176590: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176594: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176598: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317659C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831765A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831765A0 size=164
    let mut pc: u32 = 0x831765A0;
    'dispatch: loop {
        match pc {
            0x831765A0 => {
    //   block [0x831765A0..0x83176644)
	// 831765A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831765A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831765A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831765AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831765B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831765B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831765B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831765BC: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 831765C0: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 831765C4: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831765C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831765CC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 831765D0: 409AFFF4  bne cr6, 0x831765c4
	if !ctx.cr[6].eq {
	pc = 0x831765C4; continue 'dispatch;
	}
	// 831765D4: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831765D8: 815F043C  lwz r10, 0x43c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1084 as u32) ) } as u64;
	// 831765DC: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 831765E0: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831765E4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831765E8: 40990024  ble cr6, 0x8317660c
	if !ctx.cr[6].gt {
	pc = 0x8317660C; continue 'dispatch;
	}
	// 831765EC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831765F0: 386BA5F0  addi r3, r11, -0x5a10
	ctx.r[3].s64 = ctx.r[11].s64 + -23056;
	// 831765F4: 48006B4D  bl 0x8317d140
	ctx.lr = 0x831765F8;
	sub_8317D140(ctx, base);
	// 831765F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831765FC: 80BF043C  lwz r5, 0x43c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1084 as u32) ) } as u64;
	// 83176600: 807F0438  lwz r3, 0x438(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 83176604: 48035EED  bl 0x831ac4f0
	ctx.lr = 0x83176608;
	sub_831AC4F0(ctx, base);
	// 83176608: 48000024  b 0x8317662c
	pc = 0x8317662C; continue 'dispatch;
	// 8317660C: 815F0438  lwz r10, 0x438(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 83176610: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 83176614: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83176618: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8317661C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83176620: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 83176624: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83176628: 409AFFEC  bne cr6, 0x83176614
	if !ctx.cr[6].eq {
	pc = 0x83176614; continue 'dispatch;
	}
	// 8317662C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83176630: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176634: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176638: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317663C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176640: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176648 size=60
    let mut pc: u32 = 0x83176648;
    'dispatch: loop {
        match pc {
            0x83176648 => {
    //   block [0x83176648..0x83176684)
	// 83176648: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317664C: 48031B21  bl 0x831a816c
	ctx.lr = 0x83176650;
	sub_831A8130(ctx, base);
	// 83176650: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176654: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83176658: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8317665C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83176660: 4BFFFF41  bl 0x831765a0
	ctx.lr = 0x83176664;
	sub_831765A0(ctx, base);
	// 83176664: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83176668: 93DF0448  stw r30, 0x448(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1096 as u32), ctx.r[30].u32 ) };
	// 8317666C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83176670: 93BF044C  stw r29, 0x44c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1100 as u32), ctx.r[29].u32 ) };
	// 83176674: 917F0444  stw r11, 0x444(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1092 as u32), ctx.r[11].u32 ) };
	// 83176678: 915F0440  stw r10, 0x440(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1088 as u32), ctx.r[10].u32 ) };
	// 8317667C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83176680: 48031B3C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176688 size=92
    let mut pc: u32 = 0x83176688;
    'dispatch: loop {
        match pc {
            0x83176688 => {
    //   block [0x83176688..0x831766E4)
	// 83176688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317668C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83176690: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83176694: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176698: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317669C: 480072BD  bl 0x8317d958
	ctx.lr = 0x831766A0;
	sub_8317D958(ctx, base);
	// 831766A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831766A4: 409A0018  bne cr6, 0x831766bc
	if !ctx.cr[6].eq {
	pc = 0x831766BC; continue 'dispatch;
	}
	// 831766A8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831766AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831766B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831766B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831766B8: 4E800020  blr
	return;
	// 831766BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831766C0: 480071B9  bl 0x8317d878
	ctx.lr = 0x831766C4;
	sub_8317D878(ctx, base);
	// 831766C4: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 831766C8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 831766CC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 831766D0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831766D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831766D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831766DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831766E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831766E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831766E8 size=300
    let mut pc: u32 = 0x831766E8;
    'dispatch: loop {
        match pc {
            0x831766E8 => {
    //   block [0x831766E8..0x83176814)
	// 831766E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831766EC: 48031A7D  bl 0x831a8168
	ctx.lr = 0x831766F0;
	sub_831A8130(ctx, base);
	// 831766F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831766F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831766F8: 4BFFCC91  bl 0x83173388
	ctx.lr = 0x831766FC;
	sub_83173388(ctx, base);
	// 831766FC: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83176700: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83176704: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176708: 939F0538  stw r28, 0x538(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1336 as u32), ctx.r[28].u32 ) };
	// 8317670C: 480023D5  bl 0x83178ae0
	ctx.lr = 0x83176710;
	sub_83178AE0(ctx, base);
	// 83176710: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83176714: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83176718: 419A007C  beq cr6, 0x83176794
	if ctx.cr[6].eq {
	pc = 0x83176794; continue 'dispatch;
	}
	// 8317671C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176720: 4BFFDED1  bl 0x831745f0
	ctx.lr = 0x83176724;
	sub_831745F0(ctx, base);
	// 83176724: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83176728: 419A0018  beq cr6, 0x83176740
	if ctx.cr[6].eq {
	pc = 0x83176740; continue 'dispatch;
	}
	// 8317672C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176730: 386BA638  addi r3, r11, -0x59c8
	ctx.r[3].s64 = ctx.r[11].s64 + -22984;
	// 83176734: 48006A0D  bl 0x8317d140
	ctx.lr = 0x83176738;
	sub_8317D140(ctx, base);
	// 83176738: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317673C: 48031A7C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83176740: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83176744: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176748: 4BFFCB49  bl 0x83173290
	ctx.lr = 0x8317674C;
	sub_83173290(ctx, base);
	// 8317674C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83176750: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176754: 4BFFCB3D  bl 0x83173290
	ctx.lr = 0x83176758;
	sub_83173290(ctx, base);
	// 83176758: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317675C: 4800095D  bl 0x831770b8
	ctx.lr = 0x83176760;
	sub_831770B8(ctx, base);
	// 83176760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176764: 480008CD  bl 0x83177030
	ctx.lr = 0x83176768;
	sub_83177030(ctx, base);
	// 83176768: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317676C: 419A0018  beq cr6, 0x83176784
	if ctx.cr[6].eq {
	pc = 0x83176784; continue 'dispatch;
	}
	// 83176770: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176774: 386BA60C  addi r3, r11, -0x59f4
	ctx.r[3].s64 = ctx.r[11].s64 + -23028;
	// 83176778: 480069C9  bl 0x8317d140
	ctx.lr = 0x8317677C;
	sub_8317D140(ctx, base);
	// 8317677C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83176780: 48031A38  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83176784: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176788: 48000911  bl 0x83177098
	ctx.lr = 0x8317678C;
	sub_83177098(ctx, base);
	// 8317678C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176790: 48001549  bl 0x83177cd8
	ctx.lr = 0x83176794;
	sub_83177CD8(ctx, base);
	// 83176794: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176798: 4BFFFA89  bl 0x83176220
	ctx.lr = 0x8317679C;
	sub_83176220(ctx, base);
	// 8317679C: 4BFFF99D  bl 0x83176138
	ctx.lr = 0x831767A0;
	sub_83176138(ctx, base);
	// 831767A0: 897F0082  lbz r11, 0x82(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(130 as u32) ) } as u64;
	// 831767A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831767A8: 7D640774  extsb r4, r11
	ctx.r[4].s64 = ctx.r[11].s8 as i64;
	// 831767AC: 4BFFFC3D  bl 0x831763e8
	ctx.lr = 0x831767B0;
	sub_831763E8(ctx, base);
	// 831767B0: 3BDF04A0  addi r30, r31, 0x4a0
	ctx.r[30].s64 = ctx.r[31].s64 + 1184;
	// 831767B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831767B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831767BC: 4BFFC8CD  bl 0x83173088
	ctx.lr = 0x831767C0;
	sub_83173088(ctx, base);
	// 831767C0: 3BBF04C4  addi r29, r31, 0x4c4
	ctx.r[29].s64 = ctx.r[31].s64 + 1220;
	// 831767C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831767C8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831767CC: 4BFFC8BD  bl 0x83173088
	ctx.lr = 0x831767D0;
	sub_83173088(ctx, base);
	// 831767D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831767D4: 4BFFC795  bl 0x83172f68
	ctx.lr = 0x831767D8;
	sub_83172F68(ctx, base);
	// 831767D8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831767DC: 4BFFC78D  bl 0x83172f68
	ctx.lr = 0x831767E0;
	sub_83172F68(ctx, base);
	// 831767E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831767E4: 809F007C  lwz r4, 0x7c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 831767E8: 4BFFBEC1  bl 0x831726a8
	ctx.lr = 0x831767EC;
	sub_831726A8(ctx, base);
	// 831767EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831767F0: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 831767F4: 4BFFBEC5  bl 0x831726b8
	ctx.lr = 0x831767F8;
	sub_831726B8(ctx, base);
	// 831767F8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 831767FC: 939F0094  stw r28, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[28].u32 ) };
	// 83176800: 9B9F0081  stb r28, 0x81(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(129 as u32), ctx.r[28].u8 ) };
	// 83176804: 939F04F8  stw r28, 0x4f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1272 as u32), ctx.r[28].u32 ) };
	// 83176808: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317680C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83176810: 480319A8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176818 size=140
    let mut pc: u32 = 0x83176818;
    'dispatch: loop {
        match pc {
            0x83176818 => {
    //   block [0x83176818..0x831768A4)
	// 83176818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317681C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83176820: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83176824: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83176828: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317682C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83176830: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83176834: 4BFFBF15  bl 0x83172748
	ctx.lr = 0x83176838;
	sub_83172748(ctx, base);
	// 83176838: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317683C: 419A0014  beq cr6, 0x83176850
	if ctx.cr[6].eq {
	pc = 0x83176850; continue 'dispatch;
	}
	// 83176840: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176844: 386BA660  addi r3, r11, -0x59a0
	ctx.r[3].s64 = ctx.r[11].s64 + -22944;
	// 83176848: 480068F9  bl 0x8317d140
	ctx.lr = 0x8317684C;
	sub_8317D140(ctx, base);
	// 8317684C: 48000040  b 0x8317688c
	pc = 0x8317688C; continue 'dispatch;
	// 83176850: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83176854: 4800713D  bl 0x8317d990
	ctx.lr = 0x83176858;
	sub_8317D990(ctx, base);
	// 83176858: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317685C: 4BFFF9D5  bl 0x83176230
	ctx.lr = 0x83176860;
	sub_83176230(ctx, base);
	// 83176860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83176864: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83176868: 93DF0450  stw r30, 0x450(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1104 as u32), ctx.r[30].u32 ) };
	// 8317686C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176870: 917F0468  stw r11, 0x468(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1128 as u32), ctx.r[11].u32 ) };
	// 83176874: 915F0464  stw r10, 0x464(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1124 as u32), ctx.r[10].u32 ) };
	// 83176878: 917F046C  stw r11, 0x46c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1132 as u32), ctx.r[11].u32 ) };
	// 8317687C: 917F0470  stw r11, 0x470(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1136 as u32), ctx.r[11].u32 ) };
	// 83176880: 4BFFFE69  bl 0x831766e8
	ctx.lr = 0x83176884;
	sub_831766E8(ctx, base);
	// 83176884: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83176888: 48007109  bl 0x8317d990
	ctx.lr = 0x8317688C;
	sub_8317D990(ctx, base);
	// 8317688C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83176890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176898: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317689C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831768A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831768A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831768A8 size=120
    let mut pc: u32 = 0x831768A8;
    'dispatch: loop {
        match pc {
            0x831768A8 => {
    //   block [0x831768A8..0x83176920)
	// 831768A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831768AC: 480318C1  bl 0x831a816c
	ctx.lr = 0x831768B0;
	sub_831A8130(ctx, base);
	// 831768B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831768B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831768B8: 48006861  bl 0x8317d118
	ctx.lr = 0x831768BC;
	sub_8317D118(ctx, base);
	// 831768BC: 3FA08345  lis r29, -0x7cbb
	ctx.r[29].s64 = -2092630016;
	// 831768C0: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831768C4: 3BCBCC10  addi r30, r11, -0x33f0
	ctx.r[30].s64 = ctx.r[11].s64 + -13296;
	// 831768C8: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 831768CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831768D0: 419A001C  beq cr6, 0x831768ec
	if ctx.cr[6].eq {
	pc = 0x831768EC; continue 'dispatch;
	}
	// 831768D4: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 831768D8: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 831768DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831768E0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 831768E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831768E8: 4E800421  bctrl
	ctx.lr = 0x831768EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831768EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831768F0: 4BFFFA11  bl 0x83176300
	ctx.lr = 0x831768F4;
	sub_83176300(ctx, base);
	// 831768F4: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 831768F8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831768FC: 419A0018  beq cr6, 0x83176914
	if ctx.cr[6].eq {
	pc = 0x83176914; continue 'dispatch;
	}
	// 83176900: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83176904: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 83176908: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317690C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83176910: 4E800421  bctrl
	ctx.lr = 0x83176914;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83176914: 48006815  bl 0x8317d128
	ctx.lr = 0x83176918;
	sub_8317D128(ctx, base);
	// 83176918: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317691C: 480318A0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176920 size=16
    let mut pc: u32 = 0x83176920;
    'dispatch: loop {
        match pc {
            0x83176920 => {
    //   block [0x83176920..0x83176930)
	// 83176920: 3CC07FFF  lis r6, 0x7fff
	ctx.r[6].s64 = 2147418112;
	// 83176924: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83176928: 60C6FFFF  ori r6, r6, 0xffff
	ctx.r[6].u64 = ctx.r[6].u64 | 65535;
	// 8317692C: 4BFFFD1C  b 0x83176648
	sub_83176648(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176930 size=144
    let mut pc: u32 = 0x83176930;
    'dispatch: loop {
        match pc {
            0x83176930 => {
    //   block [0x83176930..0x831769C0)
	// 83176930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176934: 48031835  bl 0x831a8168
	ctx.lr = 0x83176938;
	sub_831A8130(ctx, base);
	// 83176938: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317693C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83176940: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83176944: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 83176948: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8317694C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83176950: 48007041  bl 0x8317d990
	ctx.lr = 0x83176954;
	sub_8317D990(ctx, base);
	// 83176954: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83176958: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317695C: 48001F95  bl 0x831788f0
	ctx.lr = 0x83176960;
	sub_831788F0(ctx, base);
	// 83176960: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83176964: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176968: 48002191  bl 0x83178af8
	ctx.lr = 0x8317696C;
	sub_83178AF8(ctx, base);
	// 8317696C: 817F0454  lwz r11, 0x454(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1108 as u32) ) } as u64;
	// 83176970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176974: 917F0450  stw r11, 0x450(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1104 as u32), ctx.r[11].u32 ) };
	// 83176978: 4BFFF8B9  bl 0x83176230
	ctx.lr = 0x8317697C;
	sub_83176230(ctx, base);
	// 8317697C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176980: 4BFFFD69  bl 0x831766e8
	ctx.lr = 0x83176984;
	sub_831766E8(ctx, base);
	// 83176984: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83176988: 48007009  bl 0x8317d990
	ctx.lr = 0x8317698C;
	sub_8317D990(ctx, base);
	// 8317698C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83176990: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176994: 4BFFFF8D  bl 0x83176920
	ctx.lr = 0x83176998;
	sub_83176920(ctx, base);
	// 83176998: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8317699C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831769A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831769A4: 4BF5113D  bl 0x830c7ae0
	ctx.lr = 0x831769A8;
	sub_830C7AE0(ctx, base);
	// 831769A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 831769AC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831769B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831769B4: 4BF5112D  bl 0x830c7ae0
	ctx.lr = 0x831769B8;
	sub_830C7AE0(ctx, base);
	// 831769B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831769BC: 480317FC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831769C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831769C0 size=72
    let mut pc: u32 = 0x831769C0;
    'dispatch: loop {
        match pc {
            0x831769C0 => {
    //   block [0x831769C0..0x83176A08)
	// 831769C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831769C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831769C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831769CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831769D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831769D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831769D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831769DC: 4800673D  bl 0x8317d118
	ctx.lr = 0x831769E0;
	sub_8317D118(ctx, base);
	// 831769E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831769E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831769E8: 4BFFFE31  bl 0x83176818
	ctx.lr = 0x831769EC;
	sub_83176818(ctx, base);
	// 831769EC: 4800673D  bl 0x8317d128
	ctx.lr = 0x831769F0;
	sub_8317D128(ctx, base);
	// 831769F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831769F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831769F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831769FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83176A00: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176A04: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176A08 size=124
    let mut pc: u32 = 0x83176A08;
    'dispatch: loop {
        match pc {
            0x83176A08 => {
    //   block [0x83176A08..0x83176A84)
	// 83176A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176A0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83176A10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83176A14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83176A18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176A1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83176A20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83176A24: 4BFFBD25  bl 0x83172748
	ctx.lr = 0x83176A28;
	sub_83172748(ctx, base);
	// 83176A28: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83176A2C: 419A0014  beq cr6, 0x83176a40
	if ctx.cr[6].eq {
	pc = 0x83176A40; continue 'dispatch;
	}
	// 83176A30: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176A34: 386BA6B4  addi r3, r11, -0x594c
	ctx.r[3].s64 = ctx.r[11].s64 + -22860;
	// 83176A38: 48006709  bl 0x8317d140
	ctx.lr = 0x83176A3C;
	sub_8317D140(ctx, base);
	// 83176A3C: 48000030  b 0x83176a6c
	pc = 0x83176A6C; continue 'dispatch;
	// 83176A40: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83176A44: 409A0014  bne cr6, 0x83176a58
	if !ctx.cr[6].eq {
	pc = 0x83176A58; continue 'dispatch;
	}
	// 83176A48: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176A4C: 386BA68C  addi r3, r11, -0x5974
	ctx.r[3].s64 = ctx.r[11].s64 + -22900;
	// 83176A50: 480066F1  bl 0x8317d140
	ctx.lr = 0x83176A54;
	sub_8317D140(ctx, base);
	// 83176A54: 48000018  b 0x83176a6c
	pc = 0x83176A6C; continue 'dispatch;
	// 83176A58: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 83176A5C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83176A60: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83176A64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176A68: 4BFFFEC9  bl 0x83176930
	ctx.lr = 0x83176A6C;
	sub_83176930(ctx, base);
	// 83176A6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83176A70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176A74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176A78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83176A7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176A80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176A88 size=132
    let mut pc: u32 = 0x83176A88;
    'dispatch: loop {
        match pc {
            0x83176A88 => {
    //   block [0x83176A88..0x83176B0C)
	// 83176A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176A8C: 480316DD  bl 0x831a8168
	ctx.lr = 0x83176A90;
	sub_831A8130(ctx, base);
	// 83176A90: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176A94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83176A98: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83176A9C: 4800667D  bl 0x8317d118
	ctx.lr = 0x83176AA0;
	sub_8317D118(ctx, base);
	// 83176AA0: 3F808345  lis r28, -0x7cbb
	ctx.r[28].s64 = -2092630016;
	// 83176AA4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83176AA8: 3BEBC118  addi r31, r11, -0x3ee8
	ctx.r[31].s64 = ctx.r[11].s64 + -16104;
	// 83176AAC: 807CA374  lwz r3, -0x5c8c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83176AB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83176AB4: 419A0020  beq cr6, 0x83176ad4
	if ctx.cr[6].eq {
	pc = 0x83176AD4; continue 'dispatch;
	}
	// 83176AB8: 93DF000C  stw r30, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[30].u32 ) };
	// 83176ABC: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 83176AC0: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83176AC4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83176AC8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83176ACC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83176AD0: 4E800421  bctrl
	ctx.lr = 0x83176AD4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83176AD4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83176AD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83176ADC: 4BFFFF2D  bl 0x83176a08
	ctx.lr = 0x83176AE0;
	sub_83176A08(ctx, base);
	// 83176AE0: 807CA374  lwz r3, -0x5c8c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83176AE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83176AE8: 419A0018  beq cr6, 0x83176b00
	if ctx.cr[6].eq {
	pc = 0x83176B00; continue 'dispatch;
	}
	// 83176AEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83176AF0: 389F006C  addi r4, r31, 0x6c
	ctx.r[4].s64 = ctx.r[31].s64 + 108;
	// 83176AF4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83176AF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83176AFC: 4E800421  bctrl
	ctx.lr = 0x83176B00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83176B00: 48006629  bl 0x8317d128
	ctx.lr = 0x83176B04;
	sub_8317D128(ctx, base);
	// 83176B04: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83176B08: 480316B0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176B10 size=104
    let mut pc: u32 = 0x83176B10;
    'dispatch: loop {
        match pc {
            0x83176B10 => {
    //   block [0x83176B10..0x83176B78)
	// 83176B10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176B14: 4803164D  bl 0x831a8160
	ctx.lr = 0x83176B18;
	sub_831A8130(ctx, base);
	// 83176B18: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176B1C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83176B20: 83E300B8  lwz r31, 0xb8(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 83176B24: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 83176B28: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 83176B2C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 83176B30: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 83176B34: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 83176B38: 480009F1  bl 0x83177528
	ctx.lr = 0x83176B3C;
	sub_83177528(ctx, base);
	// 83176B3C: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 83176B40: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 83176B44: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 83176B48: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 83176B4C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83176B50: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83176B54: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 83176B58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176B5C: 4801379D  bl 0x8318a2f8
	ctx.lr = 0x83176B60;
	sub_8318A2F8(ctx, base);
	// 83176B60: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83176B64: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 83176B68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176B6C: 48012F55  bl 0x83189ac0
	ctx.lr = 0x83176B70;
	sub_83189AC0(ctx, base);
	// 83176B70: 38210190  addi r1, r1, 0x190
	ctx.r[1].s64 = ctx.r[1].s64 + 400;
	// 83176B74: 4803163C  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176B78 size=20
    let mut pc: u32 = 0x83176B78;
    'dispatch: loop {
        match pc {
            0x83176B78 => {
    //   block [0x83176B78..0x83176B8C)
	// 83176B78: 81240010  lwz r9, 0x10(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 83176B7C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 83176B80: 8104000C  lwz r8, 0xc(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 83176B84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83176B88: 4BFFFF88  b 0x83176b10
	sub_83176B10(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176B90 size=4
    let mut pc: u32 = 0x83176B90;
    'dispatch: loop {
        match pc {
            0x83176B90 => {
    //   block [0x83176B90..0x83176B94)
	// 83176B90: 48013AF8  b 0x8318a688
	sub_8318A688(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176B98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176B98 size=8
    let mut pc: u32 = 0x83176B98;
    'dispatch: loop {
        match pc {
            0x83176B98 => {
    //   block [0x83176B98..0x83176BA0)
	// 83176B98: 3860201F  li r3, 0x201f
	ctx.r[3].s64 = 8223;
	// 83176B9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176BA0 size=4
    let mut pc: u32 = 0x83176BA0;
    'dispatch: loop {
        match pc {
            0x83176BA0 => {
    //   block [0x83176BA0..0x83176BA4)
	// 83176BA0: 48013D38  b 0x8318a8d8
	sub_8318A8D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176BA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176BA8 size=4
    let mut pc: u32 = 0x83176BA8;
    'dispatch: loop {
        match pc {
            0x83176BA8 => {
    //   block [0x83176BA8..0x83176BAC)
	// 83176BA8: 48013C28  b 0x8318a7d0
	sub_8318A7D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176BB0 size=8
    let mut pc: u32 = 0x83176BB0;
    'dispatch: loop {
        match pc {
            0x83176BB0 => {
    //   block [0x83176BB0..0x83176BB8)
	// 83176BB0: 806300B8  lwz r3, 0xb8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 83176BB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176BB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83176BB8 size=48
    let mut pc: u32 = 0x83176BB8;
    'dispatch: loop {
        match pc {
            0x83176BB8 => {
    //   block [0x83176BB8..0x83176BE8)
	// 83176BB8: 7C8B1E70  srawi r11, r4, 3
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[4].s32 >> 3) as i64;
	// 83176BBC: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83176BC0: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83176BC4: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 83176BC8: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83176BCC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83176BD0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83176BD4: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 83176BD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83176BDC: 814B00D0  lwz r10, 0xd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 83176BE0: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83176BE4: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176BE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176BE8 size=8
    let mut pc: u32 = 0x83176BE8;
    'dispatch: loop {
        match pc {
            0x83176BE8 => {
    //   block [0x83176BE8..0x83176BF0)
	// 83176BE8: 806B00E0  lwz r3, 0xe0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(224 as u32) ) } as u64;
	// 83176BEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176BF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83176BF0 size=48
    let mut pc: u32 = 0x83176BF0;
    'dispatch: loop {
        match pc {
            0x83176BF0 => {
    //   block [0x83176BF0..0x83176C20)
	// 83176BF0: 7C8B1E70  srawi r11, r4, 3
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[4].s32 >> 3) as i64;
	// 83176BF4: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83176BF8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83176BFC: 7D6B2050  subf r11, r11, r4
	ctx.r[11].s64 = ctx.r[4].s64 - ctx.r[11].s64;
	// 83176C00: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83176C04: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 83176C08: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83176C0C: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 83176C10: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 83176C14: 814B00D0  lwz r10, 0xd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 83176C18: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83176C1C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176C20 size=8
    let mut pc: u32 = 0x83176C20;
    'dispatch: loop {
        match pc {
            0x83176C20 => {
    //   block [0x83176C20..0x83176C28)
	// 83176C20: 806B00E8  lwz r3, 0xe8(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(232 as u32) ) } as u64;
	// 83176C24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176C28 size=108
    let mut pc: u32 = 0x83176C28;
    'dispatch: loop {
        match pc {
            0x83176C28 => {
    //   block [0x83176C28..0x83176C94)
	// 83176C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83176C30: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176C34: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83176C38: 419A0048  beq cr6, 0x83176c80
	if ctx.cr[6].eq {
	pc = 0x83176C80; continue 'dispatch;
	}
	// 83176C3C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 83176C40: 419A002C  beq cr6, 0x83176c6c
	if ctx.cr[6].eq {
	pc = 0x83176C6C; continue 'dispatch;
	}
	// 83176C44: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83176C48: 419A0010  beq cr6, 0x83176c58
	if ctx.cr[6].eq {
	pc = 0x83176C58; continue 'dispatch;
	}
	// 83176C4C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176C50: 386BA6E4  addi r3, r11, -0x591c
	ctx.r[3].s64 = ctx.r[11].s64 + -22812;
	// 83176C54: 480064ED  bl 0x8317d140
	ctx.lr = 0x83176C58;
	sub_8317D140(ctx, base);
	// 83176C58: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 83176C5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176C68: 4E800020  blr
	return;
	// 83176C6C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83176C70: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176C74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176C78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176C7C: 4E800020  blr
	return;
	// 83176C80: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83176C84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176C88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176C8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176C98 size=12
    let mut pc: u32 = 0x83176C98;
    'dispatch: loop {
        match pc {
            0x83176C98 => {
    //   block [0x83176C98..0x83176CA4)
	// 83176C98: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 83176C9C: 9164004C  stw r11, 0x4c(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 83176CA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176CA8 size=16
    let mut pc: u32 = 0x83176CA8;
    'dispatch: loop {
        match pc {
            0x83176CA8 => {
    //   block [0x83176CA8..0x83176CB8)
	// 83176CA8: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 83176CAC: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 83176CB0: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 83176CB4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176CB8 size=84
    let mut pc: u32 = 0x83176CB8;
    'dispatch: loop {
        match pc {
            0x83176CB8 => {
    //   block [0x83176CB8..0x83176D0C)
	// 83176CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83176CC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83176CC4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176CC8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83176CCC: 806300B8  lwz r3, 0xb8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 83176CD0: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83176CD4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83176CD8: 48013E41  bl 0x8318ab18
	ctx.lr = 0x83176CDC;
	sub_8318AB18(ctx, base);
	// 83176CDC: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83176CE0: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83176CE4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83176CE8: 915F0050  stw r10, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 83176CEC: 913F0054  stw r9, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 83176CF0: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 83176CF4: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83176CF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83176CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176D04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176D08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176D10 size=156
    let mut pc: u32 = 0x83176D10;
    'dispatch: loop {
        match pc {
            0x83176D10 => {
    //   block [0x83176D10..0x83176DAC)
	// 83176D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83176D18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83176D1C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176D20: 81630098  lwz r11, 0x98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(152 as u32) ) } as u64;
	// 83176D24: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 83176D28: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83176D2C: 419A0068  beq cr6, 0x83176d94
	if ctx.cr[6].eq {
	pc = 0x83176D94; continue 'dispatch;
	}
	// 83176D30: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83176D34: 419A0048  beq cr6, 0x83176d7c
	if ctx.cr[6].eq {
	pc = 0x83176D7C; continue 'dispatch;
	}
	// 83176D38: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83176D3C: 419A0028  beq cr6, 0x83176d64
	if ctx.cr[6].eq {
	pc = 0x83176D64; continue 'dispatch;
	}
	// 83176D40: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176D44: 386BA70C  addi r3, r11, -0x58f4
	ctx.r[3].s64 = ctx.r[11].s64 + -22772;
	// 83176D48: 480063F9  bl 0x8317d140
	ctx.lr = 0x83176D4C;
	sub_8317D140(ctx, base);
	// 83176D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176D50: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176D54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176D58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176D5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176D60: 4E800020  blr
	return;
	// 83176D64: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 83176D68: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176D6C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176D70: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176D74: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176D78: 4E800020  blr
	return;
	// 83176D7C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83176D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176D8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176D90: 4E800020  blr
	return;
	// 83176D94: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83176D98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176D9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176DA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176DA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176DB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176DB0 size=156
    let mut pc: u32 = 0x83176DB0;
    'dispatch: loop {
        match pc {
            0x83176DB0 => {
    //   block [0x83176DB0..0x83176E4C)
	// 83176DB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176DB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83176DB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83176DBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176DC0: 8163009C  lwz r11, 0x9c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(156 as u32) ) } as u64;
	// 83176DC4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83176DC8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83176DCC: 419A0068  beq cr6, 0x83176e34
	if ctx.cr[6].eq {
	pc = 0x83176E34; continue 'dispatch;
	}
	// 83176DD0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83176DD4: 419A0048  beq cr6, 0x83176e1c
	if ctx.cr[6].eq {
	pc = 0x83176E1C; continue 'dispatch;
	}
	// 83176DD8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83176DDC: 419A0028  beq cr6, 0x83176e04
	if ctx.cr[6].eq {
	pc = 0x83176E04; continue 'dispatch;
	}
	// 83176DE0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176DE4: 386BA734  addi r3, r11, -0x58cc
	ctx.r[3].s64 = ctx.r[11].s64 + -22732;
	// 83176DE8: 48006359  bl 0x8317d140
	ctx.lr = 0x83176DEC;
	sub_8317D140(ctx, base);
	// 83176DEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176DF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176DFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176E00: 4E800020  blr
	return;
	// 83176E04: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 83176E08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176E0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176E10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176E14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176E18: 4E800020  blr
	return;
	// 83176E1C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83176E20: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176E24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176E28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176E2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176E30: 4E800020  blr
	return;
	// 83176E34: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83176E38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176E3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176E40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176E44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176E48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176E50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176E50 size=116
    let mut pc: u32 = 0x83176E50;
    'dispatch: loop {
        match pc {
            0x83176E50 => {
    //   block [0x83176E50..0x83176EC4)
	// 83176E50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176E54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83176E58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83176E5C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176E60: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 83176E64: 2B030001  cmplwi cr6, r3, 1
	ctx.cr[6].compare_u32(ctx.r[3].u32, 1 as u32, &mut ctx.xer);
	// 83176E68: 41980044  blt cr6, 0x83176eac
	if ctx.cr[6].lt {
	pc = 0x83176EAC; continue 'dispatch;
	}
	// 83176E6C: 419A0028  beq cr6, 0x83176e94
	if ctx.cr[6].eq {
	pc = 0x83176E94; continue 'dispatch;
	}
	// 83176E70: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176E74: 386BA758  addi r3, r11, -0x58a8
	ctx.r[3].s64 = ctx.r[11].s64 + -22696;
	// 83176E78: 480062C9  bl 0x8317d140
	ctx.lr = 0x83176E7C;
	sub_8317D140(ctx, base);
	// 83176E7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176E80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176E84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176E88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176E8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176E90: 4E800020  blr
	return;
	// 83176E94: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83176E98: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176E9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176EA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176EA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176EA8: 4E800020  blr
	return;
	// 83176EAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83176EB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176EB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176EB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176EBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83176EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176EC8 size=36
    let mut pc: u32 = 0x83176EC8;
    'dispatch: loop {
        match pc {
            0x83176EC8 => {
    //   block [0x83176EC8..0x83176EEC)
	// 83176EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83176ED0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176ED4: 4BFFFCDD  bl 0x83176bb0
	ctx.lr = 0x83176ED8;
	sub_83176BB0(ctx, base);
	// 83176ED8: 48013B91  bl 0x8318aa68
	ctx.lr = 0x83176EDC;
	sub_8318AA68(ctx, base);
	// 83176EDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176EE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176EE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83176EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176EF0 size=104
    let mut pc: u32 = 0x83176EF0;
    'dispatch: loop {
        match pc {
            0x83176EF0 => {
    //   block [0x83176EF0..0x83176F58)
	// 83176EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176EF4: 48031275  bl 0x831a8168
	ctx.lr = 0x83176EF8;
	sub_831A8130(ctx, base);
	// 83176EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176EFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83176F00: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83176F04: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83176F08: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83176F0C: 4BFFB83D  bl 0x83172748
	ctx.lr = 0x83176F10;
	sub_83172748(ctx, base);
	// 83176F10: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83176F14: 419A0018  beq cr6, 0x83176f2c
	if ctx.cr[6].eq {
	pc = 0x83176F2C; continue 'dispatch;
	}
	// 83176F18: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176F1C: 386BA778  addi r3, r11, -0x5888
	ctx.r[3].s64 = ctx.r[11].s64 + -22664;
	// 83176F20: 48006221  bl 0x8317d140
	ctx.lr = 0x83176F24;
	sub_8317D140(ctx, base);
	// 83176F24: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83176F28: 48031290  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83176F2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176F30: 4BFFFC81  bl 0x83176bb0
	ctx.lr = 0x83176F34;
	sub_83176BB0(ctx, base);
	// 83176F34: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83176F38: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83176F3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83176F40: 48013B31  bl 0x8318aa70
	ctx.lr = 0x83176F44;
	sub_8318AA70(ctx, base);
	// 83176F44: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 83176F48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176F4C: 4BC8F705  bl 0x82e06650
	ctx.lr = 0x83176F50;
	sub_82E06650(ctx, base);
	// 83176F50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83176F54: 48031264  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176F58 size=32
    let mut pc: u32 = 0x83176F58;
    'dispatch: loop {
        match pc {
            0x83176F58 => {
    //   block [0x83176F58..0x83176F78)
	// 83176F58: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 83176F5C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83176F60: 419A0018  beq cr6, 0x83176f78
	if ctx.cr[6].eq {
		sub_83176F78(ctx, base);
		return;
	}
	// 83176F64: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 83176F68: 419A0010  beq cr6, 0x83176f78
	if ctx.cr[6].eq {
		sub_83176F78(ctx, base);
		return;
	}
	// 83176F6C: 2F0B0008  cmpwi cr6, r11, 8
	ctx.cr[6].compare_i32(ctx.r[11].s32, 8, &mut ctx.xer);
	// 83176F70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83176F74: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176F78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176F78 size=8
    let mut pc: u32 = 0x83176F78;
    'dispatch: loop {
        match pc {
            0x83176F78 => {
    //   block [0x83176F78..0x83176F80)
	// 83176F78: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83176F7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176F80 size=24
    let mut pc: u32 = 0x83176F80;
    'dispatch: loop {
        match pc {
            0x83176F80 => {
    //   block [0x83176F80..0x83176F98)
	// 83176F80: 81630020  lwz r11, 0x20(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(32 as u32) ) } as u64;
	// 83176F84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83176F88: 419A0010  beq cr6, 0x83176f98
	if ctx.cr[6].eq {
		sub_83176F98(ctx, base);
		return;
	}
	// 83176F8C: 2F0B0101  cmpwi cr6, r11, 0x101
	ctx.cr[6].compare_i32(ctx.r[11].s32, 257, &mut ctx.xer);
	// 83176F90: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83176F94: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83176F98 size=8
    let mut pc: u32 = 0x83176F98;
    'dispatch: loop {
        match pc {
            0x83176F98 => {
    //   block [0x83176F98..0x83176FA0)
	// 83176F98: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83176F9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83176FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83176FA0 size=104
    let mut pc: u32 = 0x83176FA0;
    'dispatch: loop {
        match pc {
            0x83176FA0 => {
    //   block [0x83176FA0..0x83177008)
	// 83176FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83176FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83176FA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83176FAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83176FB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83176FB4: 387F0008  addi r3, r31, 8
	ctx.r[3].s64 = ctx.r[31].s64 + 8;
	// 83176FB8: 4BFFFFC9  bl 0x83176f80
	ctx.lr = 0x83176FBC;
	sub_83176F80(ctx, base);
	// 83176FBC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83176FC0: 409A0030  bne cr6, 0x83176ff0
	if !ctx.cr[6].eq {
	pc = 0x83176FF0; continue 'dispatch;
	}
	// 83176FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83176FC8: 809F0418  lwz r4, 0x418(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1048 as u32) ) } as u64;
	// 83176FCC: 807F0414  lwz r3, 0x414(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1044 as u32) ) } as u64;
	// 83176FD0: 4BFBD801  bl 0x831347d0
	ctx.lr = 0x83176FD4;
	sub_831347D0(ctx, base);
	// 83176FD4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83176FD8: 409A001C  bne cr6, 0x83176ff4
	if !ctx.cr[6].eq {
	pc = 0x83176FF4; continue 'dispatch;
	}
	// 83176FDC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83176FE0: 386BA7AC  addi r3, r11, -0x5854
	ctx.r[3].s64 = ctx.r[11].s64 + -22612;
	// 83176FE4: 4800615D  bl 0x8317d140
	ctx.lr = 0x83176FE8;
	sub_8317D140(ctx, base);
	// 83176FE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83176FEC: 4BFFE85D  bl 0x83175848
	ctx.lr = 0x83176FF0;
	sub_83175848(ctx, base);
	// 83176FF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83176FF4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83176FF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83176FFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177000: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83177004: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177008(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83177008 size=12
    let mut pc: u32 = 0x83177008;
    'dispatch: loop {
        match pc {
            0x83177008 => {
    //   block [0x83177008..0x83177014)
	// 83177008: 81630410  lwz r11, 0x410(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1040 as u32) ) } as u64;
	// 8317700C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83177010: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177014(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83177014 size=20
    let mut pc: u32 = 0x83177014;
    'dispatch: loop {
        match pc {
            0x83177014 => {
    //   block [0x83177014..0x83177028)
	// 83177014: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83177018: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317701C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 83177020: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83177024: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83177028 size=4
    let mut pc: u32 = 0x83177028;
    'dispatch: loop {
        match pc {
            0x83177028 => {
    //   block [0x83177028..0x8317702C)
	// 83177028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83177030 size=100
    let mut pc: u32 = 0x83177030;
    'dispatch: loop {
        match pc {
            0x83177030 => {
    //   block [0x83177030..0x83177094)
	// 83177030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83177034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83177038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317703C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83177040: 4BFFFF19  bl 0x83176f58
	ctx.lr = 0x83177044;
	sub_83176F58(ctx, base);
	// 83177044: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83177048: 409A0018  bne cr6, 0x83177060
	if !ctx.cr[6].eq {
	pc = 0x83177060; continue 'dispatch;
	}
	// 8317704C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83177050: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177054: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177058: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317705C: 4E800020  blr
	return;
	// 83177060: 80AA0410  lwz r5, 0x410(r10)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1040 as u32) ) } as u64;
	// 83177064: 2B050000  cmplwi cr6, r5, 0
	ctx.cr[6].compare_u32(ctx.r[5].u32, 0 as u32, &mut ctx.xer);
	// 83177068: 419AFFE4  beq cr6, 0x8317704c
	if ctx.cr[6].eq {
	pc = 0x8317704C; continue 'dispatch;
	}
	// 8317706C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83177070: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 83177074: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83177078: 4800E1A1  bl 0x83185218
	ctx.lr = 0x8317707C;
	sub_83185218(ctx, base);
	// 8317707C: 21630000  subfic r11, r3, 0
	ctx.xer.ca = ctx.r[3].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[3].s64;
	// 83177080: 7C6B5910  subfe r3, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[3].u32 = res;
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 83177084: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177088: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317708C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177090: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177098(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83177098 size=28
    let mut pc: u32 = 0x83177098;
    'dispatch: loop {
        match pc {
            0x83177098 => {
    //   block [0x83177098..0x831770B4)
	// 83177098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317709C: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 831770A0: 91630428  stw r11, 0x428(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1064 as u32), ctx.r[11].u32 ) };
	// 831770A4: 9163042C  stw r11, 0x42c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1068 as u32), ctx.r[11].u32 ) };
	// 831770A8: 91630430  stw r11, 0x430(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1072 as u32), ctx.r[11].u32 ) };
	// 831770AC: 91430424  stw r10, 0x424(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1060 as u32), ctx.r[10].u32 ) };
	// 831770B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831770B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831770B8 size=12
    let mut pc: u32 = 0x831770B8;
    'dispatch: loop {
        match pc {
            0x831770B8 => {
    //   block [0x831770B8..0x831770C4)
	// 831770B8: 80630410  lwz r3, 0x410(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1040 as u32) ) } as u64;
	// 831770BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831770C0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831770C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831770C4 size=16
    let mut pc: u32 = 0x831770C4;
    'dispatch: loop {
        match pc {
            0x831770C4 => {
    //   block [0x831770C4..0x831770D4)
	// 831770C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831770C8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831770CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831770D0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831770D4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831770D4 size=4
    let mut pc: u32 = 0x831770D4;
    'dispatch: loop {
        match pc {
            0x831770D4 => {
    //   block [0x831770D4..0x831770D8)
	// 831770D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831770D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831770D8 size=128
    let mut pc: u32 = 0x831770D8;
    'dispatch: loop {
        match pc {
            0x831770D8 => {
    //   block [0x831770D8..0x83177158)
	// 831770D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831770DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831770E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831770E4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831770E8: 8163042C  lwz r11, 0x42c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1068 as u32) ) } as u64;
	// 831770EC: 83E300B8  lwz r31, 0xb8(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 831770F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831770F4: 409A0010  bne cr6, 0x83177104
	if !ctx.cr[6].eq {
	pc = 0x83177104; continue 'dispatch;
	}
	// 831770F8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831770FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83177100: 4800003C  b 0x8317713c
	pc = 0x8317713C; continue 'dispatch;
	// 83177104: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83177108: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8317710C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83177110: 38AAA7E0  addi r5, r10, -0x5820
	ctx.r[5].s64 = ctx.r[10].s64 + -22560;
	// 83177114: 81430430  lwz r10, 0x430(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1072 as u32) ) } as u64;
	// 83177118: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8317711C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 83177120: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83177124: 388AA7D8  addi r4, r10, -0x5828
	ctx.r[4].s64 = ctx.r[10].s64 + -22568;
	// 83177128: 4BFBE171  bl 0x83135298
	ctx.lr = 0x8317712C;
	sub_83135298(ctx, base);
	// 8317712C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83177130: 419AFFC8  beq cr6, 0x831770f8
	if ctx.cr[6].eq {
	pc = 0x831770F8; continue 'dispatch;
	}
	// 83177134: 80A1005C  lwz r5, 0x5c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83177138: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8317713C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83177140: 48013941  bl 0x8318aa80
	ctx.lr = 0x83177144;
	sub_8318AA80(ctx, base);
	// 83177144: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83177148: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317714C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177150: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83177154: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177158(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83177158 size=104
    let mut pc: u32 = 0x83177158;
    'dispatch: loop {
        match pc {
            0x83177158 => {
    //   block [0x83177158..0x831771C0)
	// 83177158: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317715C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83177160: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83177164: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83177168: 4BFFFDF1  bl 0x83176f58
	ctx.lr = 0x8317716C;
	sub_83176F58(ctx, base);
	// 8317716C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83177170: 409A0018  bne cr6, 0x83177188
	if !ctx.cr[6].eq {
	pc = 0x83177188; continue 'dispatch;
	}
	// 83177174: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83177178: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317717C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177184: 4E800020  blr
	return;
	// 83177188: 816A0410  lwz r11, 0x410(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1040 as u32) ) } as u64;
	// 8317718C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83177190: 419AFFE4  beq cr6, 0x83177174
	if ctx.cr[6].eq {
	pc = 0x83177174; continue 'dispatch;
	}
	// 83177194: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 83177198: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 8317719C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 831771A0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 831771A4: 4800E075  bl 0x83185218
	ctx.lr = 0x831771A8;
	sub_83185218(ctx, base);
	// 831771A8: 21630000  subfic r11, r3, 0
	ctx.xer.ca = ctx.r[3].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[3].s64;
	// 831771AC: 7C6B5910  subfe r3, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[3].u32 = res;
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 831771B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831771B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831771B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831771BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831771C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831771C0 size=48
    let mut pc: u32 = 0x831771C0;
    'dispatch: loop {
        match pc {
            0x831771C0 => {
    //   block [0x831771C0..0x831771F0)
	// 831771C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831771C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831771C8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831771CC: 48013835  bl 0x8318aa00
	ctx.lr = 0x831771D0;
	sub_8318AA00(ctx, base);
	// 831771D0: 3D608317  lis r11, -0x7ce9
	ctx.r[11].s64 = -2095644672;
	// 831771D4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831771D8: 386B3398  addi r3, r11, 0x3398
	ctx.r[3].s64 = ctx.r[11].s64 + 13208;
	// 831771DC: 480134FD  bl 0x8318a6d8
	ctx.lr = 0x831771E0;
	sub_8318A6D8(ctx, base);
	// 831771E0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831771E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831771E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831771EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831771F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831771F0 size=96
    let mut pc: u32 = 0x831771F0;
    'dispatch: loop {
        match pc {
            0x831771F0 => {
    //   block [0x831771F0..0x83177250)
	// 831771F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831771F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831771F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831771FC: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 83177200: 81690058  lwz r11, 0x58(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(88 as u32) ) } as u64;
	// 83177204: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83177208: 409A0028  bne cr6, 0x83177230
	if !ctx.cr[6].eq {
	pc = 0x83177230; continue 'dispatch;
	}
	// 8317720C: 80840030  lwz r4, 0x30(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 83177210: 4BFFF9E1  bl 0x83176bf0
	ctx.lr = 0x83177214;
	sub_83176BF0(ctx, base);
	// 83177214: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83177218: 419A000C  beq cr6, 0x83177224
	if ctx.cr[6].eq {
	pc = 0x83177224; continue 'dispatch;
	}
	// 8317721C: 9069005C  stw r3, 0x5c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 83177220: 4800000C  b 0x8317722c
	pc = 0x8317722C; continue 'dispatch;
	// 83177224: 39600011  li r11, 0x11
	ctx.r[11].s64 = 17;
	// 83177228: 9169005C  stw r11, 0x5c(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8317722C: 8169005C  lwz r11, 0x5c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(92 as u32) ) } as u64;
	// 83177230: 7D234B78  mr r3, r9
	ctx.r[3].u64 = ctx.r[9].u64;
	// 83177234: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 83177238: 4BFFF979  bl 0x83176bb0
	ctx.lr = 0x8317723C;
	sub_83176BB0(ctx, base);
	// 8317723C: 4801382D  bl 0x8318aa68
	ctx.lr = 0x83177240;
	sub_8318AA68(ctx, base);
	// 83177240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317724C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177250 size=88
    let mut pc: u32 = 0x83177250;
    'dispatch: loop {
        match pc {
            0x83177250 => {
    //   block [0x83177250..0x831772A8)
	// 83177250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83177254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83177258: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317725C: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 83177260: 80890030  lwz r4, 0x30(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(48 as u32) ) } as u64;
	// 83177264: 4BFFF955  bl 0x83176bb8
	ctx.lr = 0x83177268;
	sub_83176BB8(ctx, base);
	// 83177268: 3943FFFF  addi r10, r3, -1
	ctx.r[10].s64 = ctx.r[3].s64 + -1;
	// 8317726C: 81690098  lwz r11, 0x98(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(152 as u32) ) } as u64;
	// 83177270: 7D4A0034  cntlzw r10, r10
	ctx.r[10].u64 = if ctx.r[10].u32 == 0 { 32 } else { ctx.r[10].u32.leading_zeros() as u64 };
	// 83177274: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83177278: 5543DFFE  rlwinm r3, r10, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 8317727C: 419A001C  beq cr6, 0x83177298
	if ctx.cr[6].eq {
	pc = 0x83177298; continue 'dispatch;
	}
	// 83177280: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 83177284: 8089009C  lwz r4, 0x9c(r9)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(156 as u32) ) } as u64;
	// 83177288: 48013A19  bl 0x8318aca0
	ctx.lr = 0x8317728C;
	sub_8318ACA0(ctx, base);
	// 8317728C: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 83177290: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83177294: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83177298: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317729C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831772A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831772A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831772A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x831772A8 size=176
    let mut pc: u32 = 0x831772A8;
    'dispatch: loop {
        match pc {
            0x831772A8 => {
    //   block [0x831772A8..0x83177358)
	// 831772A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831772AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831772B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831772B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831772B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831772BC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 831772C0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 831772C4: 808B000C  lwz r4, 0xc(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 831772C8: 83CB0010  lwz r30, 0x10(r11)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 831772CC: 909F0044  stw r4, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[4].u32 ) };
	// 831772D0: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 831772D4: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 831772D8: 2F0A0003  cmpwi cr6, r10, 3
	ctx.cr[6].compare_i32(ctx.r[10].s32, 3, &mut ctx.xer);
	// 831772DC: 419A0018  beq cr6, 0x831772f4
	if ctx.cr[6].eq {
	pc = 0x831772F4; continue 'dispatch;
	}
	// 831772E0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 831772E4: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831772E8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 831772EC: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 831772F0: 4800004C  b 0x8317733c
	pc = 0x8317733C; continue 'dispatch;
	// 831772F4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 831772F8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 831772FC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83177300: 48000FE1  bl 0x831782e0
	ctx.lr = 0x83177304;
	sub_831782E0(ctx, base);
	// 83177304: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83177308: 387F0004  addi r3, r31, 4
	ctx.r[3].s64 = ctx.r[31].s64 + 4;
	// 8317730C: 80A1005C  lwz r5, 0x5c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83177310: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83177314: 4BFFF995  bl 0x83176ca8
	ctx.lr = 0x83177318;
	sub_83176CA8(ctx, base);
	// 83177318: 7FCB0E70  srawi r11, r30, 1
	ctx.xer.ca = (ctx.r[30].s32 < 0) && ((ctx.r[30].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[30].s32 >> 1) as i64;
	// 8317731C: 387F0014  addi r3, r31, 0x14
	ctx.r[3].s64 = ctx.r[31].s64 + 20;
	// 83177320: 80A10060  lwz r5, 0x60(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 83177324: 7CCB0194  addze r6, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[6].s64 = tmp.s64;
	// 83177328: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317732C: 4BFFF97D  bl 0x83176ca8
	ctx.lr = 0x83177330;
	sub_83176CA8(ctx, base);
	// 83177330: 80A10064  lwz r5, 0x64(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 83177334: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83177338: 387F0024  addi r3, r31, 0x24
	ctx.r[3].s64 = ctx.r[31].s64 + 36;
	// 8317733C: 4BFFF96D  bl 0x83176ca8
	ctx.lr = 0x83177340;
	sub_83176CA8(ctx, base);
	// 83177340: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83177344: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177348: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317734C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83177350: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83177354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177358 size=124
    let mut pc: u32 = 0x83177358;
    'dispatch: loop {
        match pc {
            0x83177358 => {
    //   block [0x83177358..0x831773D4)
	// 83177358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317735C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83177360: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83177364: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83177368: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317736C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83177370: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83177374: 4BFFF99D  bl 0x83176d10
	ctx.lr = 0x83177378;
	sub_83176D10(ctx, base);
	// 83177378: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317737C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83177380: 917E0060  stw r11, 0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 83177384: 4BFFFA2D  bl 0x83176db0
	ctx.lr = 0x83177388;
	sub_83176DB0(ctx, base);
	// 83177388: 907E0064  stw r3, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 8317738C: 817F00A0  lwz r11, 0xa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(160 as u32) ) } as u64;
	// 83177390: 917E0068  stw r11, 0x68(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 83177394: 817F00A4  lwz r11, 0xa4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(164 as u32) ) } as u64;
	// 83177398: 917E006C  stw r11, 0x6c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 8317739C: 817F00A8  lwz r11, 0xa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(168 as u32) ) } as u64;
	// 831773A0: 917E0070  stw r11, 0x70(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 831773A4: 807F00AC  lwz r3, 0xac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(172 as u32) ) } as u64;
	// 831773A8: 4BFFFAA9  bl 0x83176e50
	ctx.lr = 0x831773AC;
	sub_83176E50(ctx, base);
	// 831773AC: 907E0074  stw r3, 0x74(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(116 as u32), ctx.r[3].u32 ) };
	// 831773B0: 807F00B0  lwz r3, 0xb0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(176 as u32) ) } as u64;
	// 831773B4: 4BFFFA9D  bl 0x83176e50
	ctx.lr = 0x831773B8;
	sub_83176E50(ctx, base);
	// 831773B8: 907E0078  stw r3, 0x78(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(120 as u32), ctx.r[3].u32 ) };
	// 831773BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831773C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831773C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831773C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831773CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831773D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831773D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831773D8 size=8
    let mut pc: u32 = 0x831773D8;
    'dispatch: loop {
        match pc {
            0x831773D8 => {
    //   block [0x831773D8..0x831773E0)
	// 831773D8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 831773DC: 4BFFFB14  b 0x83176ef0
	sub_83176EF0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831773E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831773E0 size=328
    let mut pc: u32 = 0x831773E0;
    'dispatch: loop {
        match pc {
            0x831773E0 => {
    //   block [0x831773E0..0x83177528)
	// 831773E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831773E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831773E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831773EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831773F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831773F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831773F8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831773FC: 83DF0410  lwz r30, 0x410(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1040 as u32) ) } as u64;
	// 83177400: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83177404: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83177408: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317740C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83177410: 4E800421  bctrl
	ctx.lr = 0x83177414;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83177414: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83177418: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317741C: 419A0034  beq cr6, 0x83177450
	if ctx.cr[6].eq {
	pc = 0x83177450; continue 'dispatch;
	}
	// 83177420: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83177424: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 83177428: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8317742C: 38AAA7F0  addi r5, r10, -0x5810
	ctx.r[5].s64 = ctx.r[10].s64 + -22544;
	// 83177430: 815F0414  lwz r10, 0x414(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1044 as u32) ) } as u64;
	// 83177434: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 83177438: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8317743C: 3D40821A  lis r10, -0x7de6
	ctx.r[10].s64 = -2112225280;
	// 83177440: 388AA7E8  addi r4, r10, -0x5818
	ctx.r[4].s64 = ctx.r[10].s64 + -22552;
	// 83177444: 4BFBDE55  bl 0x83135298
	ctx.lr = 0x83177448;
	sub_83135298(ctx, base);
	// 83177448: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317744C: 409A001C  bne cr6, 0x83177468
	if !ctx.cr[6].eq {
	pc = 0x83177468; continue 'dispatch;
	}
	// 83177450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83177454: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83177458: 917F042C  stw r11, 0x42c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1068 as u32), ctx.r[11].u32 ) };
	// 8317745C: 917F0430  stw r11, 0x430(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1072 as u32), ctx.r[11].u32 ) };
	// 83177460: 915F0428  stw r10, 0x428(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1064 as u32), ctx.r[10].u32 ) };
	// 83177464: 480000AC  b 0x83177510
	pc = 0x83177510; continue 'dispatch;
	// 83177468: 807F041C  lwz r3, 0x41c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1052 as u32) ) } as u64;
	// 8317746C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83177470: 419A0080  beq cr6, 0x831774f0
	if ctx.cr[6].eq {
	pc = 0x831774F0; continue 'dispatch;
	}
	// 83177474: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83177478: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317747C: 48031095  bl 0x831a8510
	ctx.lr = 0x83177480;
	sub_831A8510(ctx, base);
	// 83177480: 817F041C  lwz r11, 0x41c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1052 as u32) ) } as u64;
	// 83177484: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 83177488: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317748C: 3CA07FFF  lis r5, 0x7fff
	ctx.r[5].s64 = 2147418112;
	// 83177490: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 83177494: 60A5FFFF  ori r5, r5, 0xffff
	ctx.r[5].u64 = ctx.r[5].u64 | 65535;
	// 83177498: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8317749C: 917F042C  stw r11, 0x42c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1068 as u32), ctx.r[11].u32 ) };
	// 831774A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831774A4: 915F0430  stw r10, 0x430(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1072 as u32), ctx.r[10].u32 ) };
	// 831774A8: 913F0428  stw r9, 0x428(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1064 as u32), ctx.r[9].u32 ) };
	// 831774AC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831774B0: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 831774B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831774B8: 4E800421  bctrl
	ctx.lr = 0x831774BC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831774BC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831774C0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 831774C4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831774C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831774CC: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 831774D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831774D4: 4E800421  bctrl
	ctx.lr = 0x831774D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831774D8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831774DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831774E0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 831774E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831774E8: 4E800421  bctrl
	ctx.lr = 0x831774EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831774EC: 48000024  b 0x83177510
	pc = 0x83177510; continue 'dispatch;
	// 831774F0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831774F4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 831774F8: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831774FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83177500: 917F042C  stw r11, 0x42c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1068 as u32), ctx.r[11].u32 ) };
	// 83177504: 915F0430  stw r10, 0x430(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1072 as u32), ctx.r[10].u32 ) };
	// 83177508: 913F0428  stw r9, 0x428(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1064 as u32), ctx.r[9].u32 ) };
	// 8317750C: 4BFFFC4D  bl 0x83177158
	ctx.lr = 0x83177510;
	sub_83177158(ctx, base);
	// 83177510: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83177514: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177518: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317751C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83177520: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83177524: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177528 size=172
    let mut pc: u32 = 0x83177528;
    'dispatch: loop {
        match pc {
            0x83177528 => {
    //   block [0x83177528..0x831775D4)
	// 83177528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317752C: 48030C41  bl 0x831a816c
	ctx.lr = 0x83177530;
	sub_831A8130(ctx, base);
	// 83177530: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83177534: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83177538: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317753C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83177540: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 83177544: 4BFFF6E5  bl 0x83176c28
	ctx.lr = 0x83177548;
	sub_83176C28(ctx, base);
	// 83177548: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317754C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83177550: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83177554: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83177558: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317755C: 4BFFFD4D  bl 0x831772a8
	ctx.lr = 0x83177560;
	sub_831772A8(ctx, base);
	// 83177560: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83177564: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83177568: 4BFFF731  bl 0x83176c98
	ctx.lr = 0x8317756C;
	sub_83176C98(ctx, base);
	// 8317756C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83177570: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83177574: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83177578: 4BFFF741  bl 0x83176cb8
	ctx.lr = 0x8317757C;
	sub_83176CB8(ctx, base);
	// 8317757C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83177580: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83177584: 4BFFFDD5  bl 0x83177358
	ctx.lr = 0x83177588;
	sub_83177358(ctx, base);
	// 83177588: 817F0098  lwz r11, 0x98(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8317758C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83177590: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83177594: 917E0088  stw r11, 0x88(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 83177598: 817F009C  lwz r11, 0x9c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 8317759C: 917E008C  stw r11, 0x8c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 831775A0: 4BFFFCB1  bl 0x83177250
	ctx.lr = 0x831775A4;
	sub_83177250(ctx, base);
	// 831775A4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831775A8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831775AC: 917E0090  stw r11, 0x90(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 831775B0: 809F0030  lwz r4, 0x30(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 831775B4: 4BFFF63D  bl 0x83176bf0
	ctx.lr = 0x831775B8;
	sub_83176BF0(ctx, base);
	// 831775B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831775BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831775C0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831775C4: 917E0094  stw r11, 0x94(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 831775C8: 4BFFFC29  bl 0x831771f0
	ctx.lr = 0x831775CC;
	sub_831771F0(ctx, base);
	// 831775CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831775D0: 48030BEC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831775D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831775D8 size=64
    let mut pc: u32 = 0x831775D8;
    'dispatch: loop {
        match pc {
            0x831775D8 => {
    //   block [0x831775D8..0x83177618)
	// 831775D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831775DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831775E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831775E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831775E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831775EC: 817F0410  lwz r11, 0x410(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1040 as u32) ) } as u64;
	// 831775F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831775F4: 419A0010  beq cr6, 0x83177604
	if ctx.cr[6].eq {
	pc = 0x83177604; continue 'dispatch;
	}
	// 831775F8: 4BFFFDE9  bl 0x831773e0
	ctx.lr = 0x831775FC;
	sub_831773E0(ctx, base);
	// 831775FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83177600: 4BFFFAD9  bl 0x831770d8
	ctx.lr = 0x83177604;
	sub_831770D8(ctx, base);
	// 83177604: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177608: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317760C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177610: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83177614: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177618 size=72
    let mut pc: u32 = 0x83177618;
    'dispatch: loop {
        match pc {
            0x83177618 => {
    //   block [0x83177618..0x83177660)
	// 83177618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317761C: 48030B51  bl 0x831a816c
	ctx.lr = 0x83177620;
	sub_831A8130(ctx, base);
	// 83177620: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83177624: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83177628: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8317762C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 83177630: 4BFFF581  bl 0x83176bb0
	ctx.lr = 0x83177634;
	sub_83176BB0(ctx, base);
	// 83177634: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83177638: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317763C: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83177640: 4BFFFEE9  bl 0x83177528
	ctx.lr = 0x83177644;
	sub_83177528(ctx, base);
	// 83177644: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 83177648: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8317764C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83177650: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83177654: 480134F5  bl 0x8318ab48
	ctx.lr = 0x83177658;
	sub_8318AB48(ctx, base);
	// 83177658: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 8317765C: 48030B60  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177660 size=112
    let mut pc: u32 = 0x83177660;
    'dispatch: loop {
        match pc {
            0x83177660 => {
    //   block [0x83177660..0x831776D0)
	// 83177660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83177664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83177668: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317766C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83177670: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83177674: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83177678: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8317767C: 4BFFFBD5  bl 0x83177250
	ctx.lr = 0x83177680;
	sub_83177250(ctx, base);
	// 83177680: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83177684: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83177688: 419A0008  beq cr6, 0x83177690
	if ctx.cr[6].eq {
	pc = 0x83177690; continue 'dispatch;
	}
	// 8317768C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83177690: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 83177694: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83177698: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317769C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 831776A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831776A4: 4BFFFF75  bl 0x83177618
	ctx.lr = 0x831776A8;
	sub_83177618(ctx, base);
	// 831776A8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831776AC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831776B0: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 831776B4: 915F0058  stw r10, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 831776B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831776BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831776C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831776C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 831776C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831776CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831776D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831776D0 size=20
    let mut pc: u32 = 0x831776D0;
    'dispatch: loop {
        match pc {
            0x831776D0 => {
    //   block [0x831776D0..0x831776E4)
	// 831776D0: 816304F8  lwz r11, 0x4f8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1272 as u32) ) } as u64;
	// 831776D4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831776D8: 409A000C  bne cr6, 0x831776e4
	if !ctx.cr[6].eq {
		sub_831776E4(ctx, base);
		return;
	}
	// 831776DC: 908304F8  stw r4, 0x4f8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1272 as u32), ctx.r[4].u32 ) };
	// 831776E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831776E4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831776E4 size=8
    let mut pc: u32 = 0x831776E4;
    'dispatch: loop {
        match pc {
            0x831776E4 => {
    //   block [0x831776E4..0x831776EC)
	// 831776E4: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 831776E8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831776EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831776EC size=12
    let mut pc: u32 = 0x831776EC;
    'dispatch: loop {
        match pc {
            0x831776EC => {
    //   block [0x831776EC..0x831776F8)
	// 831776EC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831776F0: 386BA7F8  addi r3, r11, -0x5808
	ctx.r[3].s64 = ctx.r[11].s64 + -22536;
	// 831776F4: 48005A4C  b 0x8317d140
	sub_8317D140(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831776F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831776F8 size=80
    let mut pc: u32 = 0x831776F8;
    'dispatch: loop {
        match pc {
            0x831776F8 => {
    //   block [0x831776F8..0x83177748)
	// 831776F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831776FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83177700: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83177704: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83177708: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317770C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83177710: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83177714: 817F0424  lwz r11, 0x424(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1060 as u32) ) } as u64;
	// 83177718: 815E0030  lwz r10, 0x30(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8317771C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 83177720: 40980008  bge cr6, 0x83177728
	if !ctx.cr[6].lt {
	pc = 0x83177728; continue 'dispatch;
	}
	// 83177724: 4BFFFEB5  bl 0x831775d8
	ctx.lr = 0x83177728;
	sub_831775D8(ctx, base);
	// 83177728: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8317772C: 917F0424  stw r11, 0x424(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1060 as u32), ctx.r[11].u32 ) };
	// 83177730: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83177734: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177738: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317773C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83177740: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83177744: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177748 size=120
    let mut pc: u32 = 0x83177748;
    'dispatch: loop {
        match pc {
            0x83177748 => {
    //   block [0x83177748..0x831777C0)
	// 83177748: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317774C: 48030A19  bl 0x831a8164
	ctx.lr = 0x83177750;
	sub_831A8130(ctx, base);
	// 83177750: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83177754: 81640038  lwz r11, 0x38(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 83177758: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317775C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83177760: 837F040C  lwz r27, 0x40c(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1036 as u32) ) } as u64;
	// 83177764: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83177768: 838B0004  lwz r28, 4(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317776C: 4BFFBD45  bl 0x831734b0
	ctx.lr = 0x83177770;
	sub_831734B0(ctx, base);
	// 83177770: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83177774: 419A0018  beq cr6, 0x8317778c
	if ctx.cr[6].eq {
	pc = 0x8317778C; continue 'dispatch;
	}
	// 83177778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317777C: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 83177780: 917E0048  stw r11, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 83177784: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83177788: 48030A2C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8317778C: 397F03F4  addi r11, r31, 0x3f4
	ctx.r[11].s64 = ctx.r[31].s64 + 1012;
	// 83177790: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 83177794: 419AFFE4  beq cr6, 0x83177778
	if ctx.cr[6].eq {
	pc = 0x83177778; continue 'dispatch;
	}
	// 83177798: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8317779C: 419A0014  beq cr6, 0x831777b0
	if ctx.cr[6].eq {
	pc = 0x831777B0; continue 'dispatch;
	}
	// 831777A0: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 831777A4: 4099000C  ble cr6, 0x831777b0
	if !ctx.cr[6].gt {
	pc = 0x831777B0; continue 'dispatch;
	}
	// 831777A8: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 831777AC: 3B9CFFFC  addi r28, r28, -4
	ctx.r[28].s64 = ctx.r[28].s64 + -4;
	// 831777B0: 93BE0044  stw r29, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[29].u32 ) };
	// 831777B4: 939E0048  stw r28, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[28].u32 ) };
	// 831777B8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831777BC: 480309F8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831777C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831777C0 size=8
    let mut pc: u32 = 0x831777C0;
    'dispatch: loop {
        match pc {
            0x831777C0 => {
    //   block [0x831777C0..0x831777C8)
	// 831777C0: 1CA503E8  mulli r5, r5, 0x3e8
	ctx.r[5].s64 = ctx.r[5].s64 * 1000;
	// 831777C4: 48014014  b 0x8318b7d8
	sub_8318B7D8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831777C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831777C8 size=24
    let mut pc: u32 = 0x831777C8;
    'dispatch: loop {
        match pc {
            0x831777C8 => {
    //   block [0x831777C8..0x831777E0)
	// 831777C8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831777CC: 419A001C  beq cr6, 0x831777e8
	if ctx.cr[6].eq {
		sub_831777E8(ctx, base);
		return;
	}
	// 831777D0: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 831777D4: 419A000C  beq cr6, 0x831777e0
	if ctx.cr[6].eq {
		sub_831777E0(ctx, base);
		return;
	}
	// 831777D8: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 831777DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831777E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831777E0 size=8
    let mut pc: u32 = 0x831777E0;
    'dispatch: loop {
        match pc {
            0x831777E0 => {
    //   block [0x831777E0..0x831777E8)
	// 831777E0: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 831777E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831777E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831777E8 size=8
    let mut pc: u32 = 0x831777E8;
    'dispatch: loop {
        match pc {
            0x831777E8 => {
    //   block [0x831777E8..0x831777F0)
	// 831777E8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831777EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831777F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831777F0 size=156
    let mut pc: u32 = 0x831777F0;
    'dispatch: loop {
        match pc {
            0x831777F0 => {
    //   block [0x831777F0..0x83177830)
	// 831777F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831777F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831777F8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831777FC: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 83177800: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 83177804: 41990068  bgt cr6, 0x8317786c
	if ctx.cr[6].gt {
	pc = 0x8317786C; continue 'dispatch;
	}
	// 83177808: 3D808317  lis r12, -0x7ce9
	ctx.r[12].s64 = -2095644672;
	// 8317780C: 398C7820  addi r12, r12, 0x7820
	ctx.r[12].s64 = ctx.r[12].s64 + 30752;
	// 83177810: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83177814: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83177818: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8317781C: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x83177878; continue 'dispatch;
		},
		1 => {
	pc = 0x83177830; continue 'dispatch;
		},
		2 => {
	pc = 0x83177844; continue 'dispatch;
		},
		3 => {
	pc = 0x83177858; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 83177820: 83177878  lwz r24, 0x7878(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(30840 as u32) ) } as u64;
	// 83177824: 83177830  lwz r24, 0x7830(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(30768 as u32) ) } as u64;
	// 83177828: 83177844  lwz r24, 0x7844(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(30788 as u32) ) } as u64;
	// 8317782C: 83177858  lwz r24, 0x7858(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(30808 as u32) ) } as u64;
            }
            0x83177830 => {
    //   block [0x83177830..0x83177844)
	// 83177830: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83177834: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317783C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177840: 4E800020  blr
	return;
            }
            0x83177844 => {
    //   block [0x83177844..0x83177858)
	// 83177844: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 83177848: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317784C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177850: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177854: 4E800020  blr
	return;
            }
            0x83177858 => {
    //   block [0x83177858..0x83177878)
	// 83177858: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8317785C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177860: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177864: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177868: 4E800020  blr
	return;
	// 8317786C: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83177870: 386BA834  addi r3, r11, -0x57cc
	ctx.r[3].s64 = ctx.r[11].s64 + -22476;
	// 83177874: 480058CD  bl 0x8317d140
	ctx.lr = 0x83177878;
	sub_8317D140(ctx, base);
            }
            0x83177878 => {
    //   block [0x83177878..0x8317788C)
	// 83177878: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317787C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177888: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177890 size=132
    let mut pc: u32 = 0x83177890;
    'dispatch: loop {
        match pc {
            0x83177890 => {
    //   block [0x83177890..0x83177914)
	// 83177890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83177894: 480308D9  bl 0x831a816c
	ctx.lr = 0x83177898;
	sub_831A8130(ctx, base);
	// 83177898: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317789C: 81640058  lwz r11, 0x58(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(88 as u32) ) } as u64;
	// 831778A0: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 831778A4: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 831778A8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 831778AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831778B0: 40990028  ble cr6, 0x831778d8
	if !ctx.cr[6].gt {
	pc = 0x831778D8; continue 'dispatch;
	}
	// 831778B4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 831778B8: 40990018  ble cr6, 0x831778d0
	if !ctx.cr[6].gt {
	pc = 0x831778D0; continue 'dispatch;
	}
	// 831778BC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 831778C0: 409A0018  bne cr6, 0x831778d8
	if !ctx.cr[6].eq {
	pc = 0x831778D8; continue 'dispatch;
	}
	// 831778C4: 8964006C  lbz r11, 0x6c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(108 as u32) ) } as u64;
	// 831778C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831778CC: 409A0018  bne cr6, 0x831778e4
	if !ctx.cr[6].eq {
	pc = 0x831778E4; continue 'dispatch;
	}
	// 831778D0: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 831778D4: 48000010  b 0x831778e4
	pc = 0x831778E4; continue 'dispatch;
	// 831778D8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831778DC: 386BA864  addi r3, r11, -0x579c
	ctx.r[3].s64 = ctx.r[11].s64 + -22428;
	// 831778E0: 48005861  bl 0x8317d140
	ctx.lr = 0x831778E4;
	sub_8317D140(ctx, base);
	// 831778E4: 4BFFBBCD  bl 0x831734b0
	ctx.lr = 0x831778E8;
	sub_831734B0(ctx, base);
	// 831778E8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831778EC: 409A001C  bne cr6, 0x83177908
	if !ctx.cr[6].eq {
	pc = 0x83177908; continue 'dispatch;
	}
	// 831778F0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831778F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831778F8: 48013349  bl 0x8318ac40
	ctx.lr = 0x831778FC;
	sub_8318AC40(ctx, base);
	// 831778FC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83177900: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83177904: 419A0008  beq cr6, 0x8317790c
	if ctx.cr[6].eq {
	pc = 0x8317790C; continue 'dispatch;
	}
	// 83177908: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317790C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83177910: 480308AC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83177918 size=80
    let mut pc: u32 = 0x83177918;
    'dispatch: loop {
        match pc {
            0x83177918 => {
    //   block [0x83177918..0x83177968)
	// 83177918: 81640058  lwz r11, 0x58(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(88 as u32) ) } as u64;
	// 8317791C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 83177920: 91630098  stw r11, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 83177924: 8164005C  lwz r11, 0x5c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(92 as u32) ) } as u64;
	// 83177928: 9163009C  stw r11, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 8317792C: 8964006C  lbz r11, 0x6c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(108 as u32) ) } as u64;
	// 83177930: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83177934: 916300A0  stw r11, 0xa0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 83177938: 8964006D  lbz r11, 0x6d(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(109 as u32) ) } as u64;
	// 8317793C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83177940: 916300A4  stw r11, 0xa4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 83177944: 8964006E  lbz r11, 0x6e(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(110 as u32) ) } as u64;
	// 83177948: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 8317794C: 916300A8  stw r11, 0xa8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 83177950: 8164003C  lwz r11, 0x3c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 83177954: 916300AC  stw r11, 0xac(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 83177958: 81640040  lwz r11, 0x40(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 8317795C: 914300B4  stw r10, 0xb4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 83177960: 916300B0  stw r11, 0xb0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 83177964: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177968 size=140
    let mut pc: u32 = 0x83177968;
    'dispatch: loop {
        match pc {
            0x83177968 => {
    //   block [0x83177968..0x831779F4)
	// 83177968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317796C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83177970: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83177974: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83177978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317797C: 81440038  lwz r10, 0x38(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 83177980: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 83177984: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 83177988: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8317798C: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 83177990: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 83177994: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 83177998: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 8317799C: 917F009C  stw r11, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 831779A0: 419A0040  beq cr6, 0x831779e0
	if ctx.cr[6].eq {
	pc = 0x831779E0; continue 'dispatch;
	}
	// 831779A4: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 831779A8: 40990038  ble cr6, 0x831779e0
	if !ctx.cr[6].gt {
	pc = 0x831779E0; continue 'dispatch;
	}
	// 831779AC: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 831779B0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831779B4: 388AFFFC  addi r4, r10, -4
	ctx.r[4].s64 = ctx.r[10].s64 + -4;
	// 831779B8: 38690004  addi r3, r9, 4
	ctx.r[3].s64 = ctx.r[9].s64 + 4;
	// 831779BC: 48013395  bl 0x8318ad50
	ctx.lr = 0x831779C0;
	sub_8318AD50(ctx, base);
	// 831779C0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831779C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831779C8: 419A0018  beq cr6, 0x831779e0
	if ctx.cr[6].eq {
	pc = 0x831779E0; continue 'dispatch;
	}
	// 831779CC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831779D0: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 831779D4: 4099000C  ble cr6, 0x831779e0
	if !ctx.cr[6].gt {
	pc = 0x831779E0; continue 'dispatch;
	}
	// 831779D8: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 831779DC: 915F009C  stw r10, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	// 831779E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831779E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831779E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831779EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831779F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831779F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831779F8 size=64
    let mut pc: u32 = 0x831779F8;
    'dispatch: loop {
        match pc {
            0x831779F8 => {
    //   block [0x831779F8..0x83177A38)
	// 831779F8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 831779FC: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83177A00: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83177A04: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83177A08: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 83177A0C: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83177A10: A163000E  lhz r11, 0xe(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 83177A14: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 83177A18: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83177A1C: A163000C  lhz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83177A20: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 83177A24: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83177A28: A163000C  lhz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83177A2C: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 83177A30: 91640014  stw r11, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 83177A34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177A38 size=120
    let mut pc: u32 = 0x83177A38;
    'dispatch: loop {
        match pc {
            0x83177A38 => {
    //   block [0x83177A38..0x83177AB0)
	// 83177A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83177A3C: 4803072D  bl 0x831a8168
	ctx.lr = 0x83177A40;
	sub_831A8130(ctx, base);
	// 83177A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83177A44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83177A48: 4BFFAD01  bl 0x83172748
	ctx.lr = 0x83177A4C;
	sub_83172748(ctx, base);
	// 83177A4C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83177A50: 419A0018  beq cr6, 0x83177a68
	if ctx.cr[6].eq {
	pc = 0x83177A68; continue 'dispatch;
	}
	// 83177A54: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83177A58: 386BA898  addi r3, r11, -0x5768
	ctx.r[3].s64 = ctx.r[11].s64 + -22376;
	// 83177A5C: 480056E5  bl 0x8317d140
	ctx.lr = 0x83177A60;
	sub_8317D140(ctx, base);
	// 83177A60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83177A64: 48030754  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83177A68: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83177A6C: 83DF0088  lwz r30, 0x88(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 83177A70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83177A74: 83BF008C  lwz r29, 0x8c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 83177A78: 839F0090  lwz r28, 0x90(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 83177A7C: 4BFFFC55  bl 0x831776d0
	ctx.lr = 0x83177A80;
	sub_831776D0(ctx, base);
	// 83177A80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83177A84: 4BFFB03D  bl 0x83172ac0
	ctx.lr = 0x83177A88;
	sub_83172AC0(ctx, base);
	// 83177A88: 7F1DE000  cmpw cr6, r29, r28
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83177A8C: 4099001C  ble cr6, 0x83177aa8
	if !ctx.cr[6].gt {
	pc = 0x83177AA8; continue 'dispatch;
	}
	// 83177A90: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83177A94: 4800C465  bl 0x83183ef8
	ctx.lr = 0x83177A98;
	sub_83183EF8(ctx, base);
	// 83177A98: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 83177A9C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83177AA0: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 83177AA4: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 83177AA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83177AAC: 4803070C  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177AB0 size=108
    let mut pc: u32 = 0x83177AB0;
    'dispatch: loop {
        match pc {
            0x83177AB0 => {
    //   block [0x83177AB0..0x83177B1C)
	// 83177AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83177AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83177AB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83177ABC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83177AC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83177AC4: 4BFFAC85  bl 0x83172748
	ctx.lr = 0x83177AC8;
	sub_83172748(ctx, base);
	// 83177AC8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83177ACC: 419A0028  beq cr6, 0x83177af4
	if ctx.cr[6].eq {
	pc = 0x83177AF4; continue 'dispatch;
	}
	// 83177AD0: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83177AD4: 386BA8C4  addi r3, r11, -0x573c
	ctx.r[3].s64 = ctx.r[11].s64 + -22332;
	// 83177AD8: 48005669  bl 0x8317d140
	ctx.lr = 0x83177ADC;
	sub_8317D140(ctx, base);
	// 83177ADC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83177AE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177AE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177AE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177AEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83177AF0: 4E800020  blr
	return;
	// 83177AF4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83177AF8: 4BFFAFC9  bl 0x83172ac0
	ctx.lr = 0x83177AFC;
	sub_83172AC0(ctx, base);
	// 83177AFC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83177B00: 419AFFDC  beq cr6, 0x83177adc
	if ctx.cr[6].eq {
	pc = 0x83177ADC; continue 'dispatch;
	}
	// 83177B04: 48010B65  bl 0x83188668
	ctx.lr = 0x83177B08;
	sub_83188668(ctx, base);
	// 83177B08: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177B0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177B10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177B14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83177B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177B20 size=52
    let mut pc: u32 = 0x83177B20;
    'dispatch: loop {
        match pc {
            0x83177B20 => {
    //   block [0x83177B20..0x83177B54)
	// 83177B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83177B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83177B28: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83177B2C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83177B30: 48010E21  bl 0x83188950
	ctx.lr = 0x83177B34;
	sub_83188950(ctx, base);
	// 83177B34: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83177B38: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83177B3C: 409A0008  bne cr6, 0x83177b44
	if !ctx.cr[6].eq {
	pc = 0x83177B44; continue 'dispatch;
	}
	// 83177B40: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83177B44: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177B48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177B4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177B50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177B58 size=52
    let mut pc: u32 = 0x83177B58;
    'dispatch: loop {
        match pc {
            0x83177B58 => {
    //   block [0x83177B58..0x83177B8C)
	// 83177B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83177B5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83177B60: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83177B64: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83177B68: 48010E11  bl 0x83188978
	ctx.lr = 0x83177B6C;
	sub_83188978(ctx, base);
	// 83177B6C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83177B70: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83177B74: 409A0008  bne cr6, 0x83177b7c
	if !ctx.cr[6].eq {
	pc = 0x83177B7C; continue 'dispatch;
	}
	// 83177B78: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83177B7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177B80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177B84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177B90 size=72
    let mut pc: u32 = 0x83177B90;
    'dispatch: loop {
        match pc {
            0x83177B90 => {
    //   block [0x83177B90..0x83177BD8)
	// 83177B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83177B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83177B98: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83177B9C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83177BA0: 48010D39  bl 0x831888d8
	ctx.lr = 0x83177BA4;
	sub_831888D8(ctx, base);
	// 83177BA4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83177BA8: 419A0018  beq cr6, 0x83177bc0
	if ctx.cr[6].eq {
	pc = 0x83177BC0; continue 'dispatch;
	}
	// 83177BAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83177BB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177BBC: 4E800020  blr
	return;
	// 83177BC0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83177BC4: 55631838  slwi r3, r11, 3
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 83177BC8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177BCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177BD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177BD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177BD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177BD8 size=140
    let mut pc: u32 = 0x83177BD8;
    'dispatch: loop {
        match pc {
            0x83177BD8 => {
    //   block [0x83177BD8..0x83177C64)
	// 83177BD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83177BDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83177BE0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83177BE4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83177BE8: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 83177BEC: 4801100D  bl 0x83188bf8
	ctx.lr = 0x83177BF0;
	sub_83188BF8(ctx, base);
	// 83177BF0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83177BF4: 419A0018  beq cr6, 0x83177c0c
	if ctx.cr[6].eq {
	pc = 0x83177C0C; continue 'dispatch;
	}
	// 83177BF8: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 83177BFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177C08: 4E800020  blr
	return;
	// 83177C0C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83177C10: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83177C14: 419A003C  beq cr6, 0x83177c50
	if ctx.cr[6].eq {
	pc = 0x83177C50; continue 'dispatch;
	}
	// 83177C18: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83177C1C: 419A0020  beq cr6, 0x83177c3c
	if ctx.cr[6].eq {
	pc = 0x83177C3C; continue 'dispatch;
	}
	// 83177C20: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 83177C24: 409AFFD4  bne cr6, 0x83177bf8
	if !ctx.cr[6].eq {
	pc = 0x83177BF8; continue 'dispatch;
	}
	// 83177C28: 38600061  li r3, 0x61
	ctx.r[3].s64 = 97;
	// 83177C2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177C30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177C34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177C38: 4E800020  blr
	return;
	// 83177C3C: 38600051  li r3, 0x51
	ctx.r[3].s64 = 81;
	// 83177C40: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177C4C: 4E800020  blr
	return;
	// 83177C50: 38600021  li r3, 0x21
	ctx.r[3].s64 = 33;
	// 83177C54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177C58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177C5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177C68 size=52
    let mut pc: u32 = 0x83177C68;
    'dispatch: loop {
        match pc {
            0x83177C68 => {
    //   block [0x83177C68..0x83177C9C)
	// 83177C68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83177C6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83177C70: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83177C74: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83177C78: 48010BE9  bl 0x83188860
	ctx.lr = 0x83177C7C;
	sub_83188860(ctx, base);
	// 83177C7C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83177C80: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83177C84: 409A0008  bne cr6, 0x83177c8c
	if !ctx.cr[6].eq {
	pc = 0x83177C8C; continue 'dispatch;
	}
	// 83177C88: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83177C8C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177C90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177C94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177C98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177CA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177CA0 size=52
    let mut pc: u32 = 0x83177CA0;
    'dispatch: loop {
        match pc {
            0x83177CA0 => {
    //   block [0x83177CA0..0x83177CD4)
	// 83177CA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83177CA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83177CA8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83177CAC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83177CB0: 48010BD9  bl 0x83188888
	ctx.lr = 0x83177CB4;
	sub_83188888(ctx, base);
	// 83177CB4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83177CB8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83177CBC: 409A0008  bne cr6, 0x83177cc4
	if !ctx.cr[6].eq {
	pc = 0x83177CC4; continue 'dispatch;
	}
	// 83177CC0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83177CC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83177CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83177CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83177CD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177CD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83177CD8 size=112
    let mut pc: u32 = 0x83177CD8;
    'dispatch: loop {
        match pc {
            0x83177CD8 => {
    //   block [0x83177CD8..0x83177D48)
	// 83177CD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83177CDC: 394300D4  addi r10, r3, 0xd4
	ctx.r[10].s64 = ctx.r[3].s64 + 212;
	// 83177CE0: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 83177CE4: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 83177CE8: 916300C4  stw r11, 0xc4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 83177CEC: 916300C8  stw r11, 0xc8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 83177CF0: 916300CC  stw r11, 0xcc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 83177CF4: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 83177CF8: 916AFFFC  stw r11, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 83177CFC: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83177D00: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 83177D04: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83177D08: 916A0010  stw r11, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 83177D0C: 910A0014  stw r8, 0x14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 83177D10: 394A0024  addi r10, r10, 0x24
	ctx.r[10].s64 = ctx.r[10].s64 + 36;
	// 83177D14: 409AFFE0  bne cr6, 0x83177cf4
	if !ctx.cr[6].eq {
	pc = 0x83177CF4; continue 'dispatch;
	}
	// 83177D18: 394301F8  addi r10, r3, 0x1f8
	ctx.r[10].s64 = ctx.r[3].s64 + 504;
	// 83177D1C: 916301F0  stw r11, 0x1f0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(496 as u32), ctx.r[11].u32 ) };
	// 83177D20: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 83177D24: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 83177D28: 916AFFFC  stw r11, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 83177D2C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83177D30: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83177D34: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 83177D38: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 83177D3C: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 83177D40: 409AFFE4  bne cr6, 0x83177d24
	if !ctx.cr[6].eq {
	pc = 0x83177D24; continue 'dispatch;
	}
	// 83177D44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83177D48 size=36
    let mut pc: u32 = 0x83177D48;
    'dispatch: loop {
        match pc {
            0x83177D48 => {
    //   block [0x83177D48..0x83177D6C)
	// 83177D48: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 83177D4C: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 83177D50: 41990064  bgt cr6, 0x83177db4
	if ctx.cr[6].gt {
		sub_83177DB4(ctx, base);
		return;
	}
	// 83177D54: 3D808317  lis r12, -0x7ce9
	ctx.r[12].s64 = -2095644672;
	// 83177D58: 398C7D6C  addi r12, r12, 0x7d6c
	ctx.r[12].s64 = ctx.r[12].s64 + 32108;
	// 83177D5C: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 83177D60: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 83177D64: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 83177D68: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
			// ERROR: 0x83177D8C
			return;
		},
		1 => {
			// ERROR: 0x83177D94
			return;
		},
		2 => {
			// ERROR: 0x83177D94
			return;
		},
		3 => {
			// ERROR: 0x83177D94
			return;
		},
		4 => {
			// ERROR: 0x83177D94
			return;
		},
		5 => {
			// ERROR: 0x83177D9C
			return;
		},
		6 => {
			// ERROR: 0x83177DA4
			return;
		},
		7 => {
			// ERROR: 0x83177DAC
			return;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177D6C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83177D6C size=40
    let mut pc: u32 = 0x83177D6C;
    'dispatch: loop {
        match pc {
            0x83177D6C => {
    //   block [0x83177D6C..0x83177D94)
	// 83177D6C: 83177D8C  lwz r24, 0x7d8c(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(32140 as u32) ) } as u64;
	// 83177D70: 83177D94  lwz r24, 0x7d94(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(32148 as u32) ) } as u64;
	// 83177D74: 83177D94  lwz r24, 0x7d94(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(32148 as u32) ) } as u64;
	// 83177D78: 83177D94  lwz r24, 0x7d94(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(32148 as u32) ) } as u64;
	// 83177D7C: 83177D94  lwz r24, 0x7d94(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(32148 as u32) ) } as u64;
	// 83177D80: 83177D9C  lwz r24, 0x7d9c(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(32156 as u32) ) } as u64;
	// 83177D84: 83177DA4  lwz r24, 0x7da4(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(32164 as u32) ) } as u64;
	// 83177D88: 83177DAC  lwz r24, 0x7dac(r23)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(32172 as u32) ) } as u64;
	// 83177D8C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83177D90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177D94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83177D94 size=8
    let mut pc: u32 = 0x83177D94;
    'dispatch: loop {
        match pc {
            0x83177D94 => {
    //   block [0x83177D94..0x83177D9C)
	// 83177D94: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 83177D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177D9C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83177D9C size=8
    let mut pc: u32 = 0x83177D9C;
    'dispatch: loop {
        match pc {
            0x83177D9C => {
    //   block [0x83177D9C..0x83177DA4)
	// 83177D9C: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 83177DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177DA4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83177DA4 size=8
    let mut pc: u32 = 0x83177DA4;
    'dispatch: loop {
        match pc {
            0x83177DA4 => {
    //   block [0x83177DA4..0x83177DAC)
	// 83177DA4: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 83177DA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177DAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83177DAC size=8
    let mut pc: u32 = 0x83177DAC;
    'dispatch: loop {
        match pc {
            0x83177DAC => {
    //   block [0x83177DAC..0x83177DB4)
	// 83177DAC: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 83177DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177DB4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83177DB4 size=8
    let mut pc: u32 = 0x83177DB4;
    'dispatch: loop {
        match pc {
            0x83177DB4 => {
    //   block [0x83177DB4..0x83177DBC)
	// 83177DB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83177DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83177DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83177DC0 size=656
    let mut pc: u32 = 0x83177DC0;
    'dispatch: loop {
        match pc {
            0x83177DC0 => {
    //   block [0x83177DC0..0x83178050)
	// 83177DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83177DC4: 48030399  bl 0x831a815c
	ctx.lr = 0x83177DC8;
	sub_831A8130(ctx, base);
	// 83177DC8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83177DCC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83177DD0: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 83177DD4: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 83177DD8: 817F04A0  lwz r11, 0x4a0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1184 as u32) ) } as u64;
	// 83177DDC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83177DE0: 409A0044  bne cr6, 0x83177e24
	if !ctx.cr[6].eq {
	pc = 0x83177E24; continue 'dispatch;
	}
	// 83177DE4: 817F04B8  lwz r11, 0x4b8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1208 as u32) ) } as u64;
	// 83177DE8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83177DEC: 409A0038  bne cr6, 0x83177e24
	if !ctx.cr[6].eq {
	pc = 0x83177E24; continue 'dispatch;
	}
	// 83177DF0: 83DF04BC  lwz r30, 0x4bc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1212 as u32) ) } as u64;
	// 83177DF4: 57CB2036  slwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83177DF8: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83177DFC: 814B01F4  lwz r10, 0x1f4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(500 as u32) ) } as u64;
	// 83177E00: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83177E04: 409A0020  bne cr6, 0x83177e24
	if !ctx.cr[6].eq {
	pc = 0x83177E24; continue 'dispatch;
	}
	// 83177E08: 816B01F8  lwz r11, 0x1f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(504 as u32) ) } as u64;
	// 83177E0C: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 83177E10: 409A0014  bne cr6, 0x83177e24
	if !ctx.cr[6].eq {
	pc = 0x83177E24; continue 'dispatch;
	}
	// 83177E14: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83177E18: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83177E1C: 4BFFAF1D  bl 0x83172d38
	ctx.lr = 0x83177E20;
	sub_83172D38(ctx, base);
	// 83177E20: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 83177E24: 817F04C4  lwz r11, 0x4c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1220 as u32) ) } as u64;
	// 83177E28: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 83177E2C: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 83177E30: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83177E34: 409A003C  bne cr6, 0x83177e70
	if !ctx.cr[6].eq {
	pc = 0x83177E70; continue 'dispatch;
	}
	// 83177E38: 817F04DC  lwz r11, 0x4dc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1244 as u32) ) } as u64;
	// 83177E3C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83177E40: 409A0030  bne cr6, 0x83177e70
	if !ctx.cr[6].eq {
	pc = 0x83177E70; continue 'dispatch;
	}
	// 83177E44: 839F04E0  lwz r28, 0x4e0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1248 as u32) ) } as u64;
	// 83177E48: 578B2036  slwi r11, r28, 4
	ctx.r[11].u32 = ctx.r[28].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83177E4C: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 83177E50: 816B01F4  lwz r11, 0x1f4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(500 as u32) ) } as u64;
	// 83177E54: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83177E58: 409A0018  bne cr6, 0x83177e70
	if !ctx.cr[6].eq {
	pc = 0x83177E70; continue 'dispatch;
	}
	// 83177E5C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83177E60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83177E64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83177E68: 4BFFAED1  bl 0x83172d38
	ctx.lr = 0x83177E6C;
	sub_83172D38(ctx, base);
	// 83177E6C: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 83177E70: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 83177E74: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83177E78: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 83177E7C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83177E80: 393F01F8  addi r9, r31, 0x1f8
	ctx.r[9].s64 = ctx.r[31].s64 + 504;
	// 83177E84: 8169FFFC  lwz r11, -4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-4 as u32) ) } as u64;
	// 83177E88: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83177E8C: 409A0040  bne cr6, 0x83177ecc
	if !ctx.cr[6].eq {
	pc = 0x83177ECC; continue 'dispatch;
	}
	// 83177E90: 396AFFFE  addi r11, r10, -2
	ctx.r[11].s64 = ctx.r[10].s64 + -2;
	// 83177E94: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83177E98: 419A0034  beq cr6, 0x83177ecc
	if ctx.cr[6].eq {
	pc = 0x83177ECC; continue 'dispatch;
	}
	// 83177E9C: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83177EA0: 419A002C  beq cr6, 0x83177ecc
	if ctx.cr[6].eq {
	pc = 0x83177ECC; continue 'dispatch;
	}
	// 83177EA4: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 83177EA8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83177EAC: 409A0008  bne cr6, 0x83177eb4
	if !ctx.cr[6].eq {
	pc = 0x83177EB4; continue 'dispatch;
	}
	// 83177EB0: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 83177EB4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83177EB8: 409A0008  bne cr6, 0x83177ec0
	if !ctx.cr[6].eq {
	pc = 0x83177EC0; continue 'dispatch;
	}
	// 83177EBC: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 83177EC0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83177EC4: 409A0008  bne cr6, 0x83177ecc
	if !ctx.cr[6].eq {
	pc = 0x83177ECC; continue 'dispatch;
	}
	// 83177EC8: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83177ECC: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 83177ED0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83177ED4: 409A0040  bne cr6, 0x83177f14
	if !ctx.cr[6].eq {
	pc = 0x83177F14; continue 'dispatch;
	}
	// 83177ED8: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 83177EDC: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83177EE0: 419A0034  beq cr6, 0x83177f14
	if ctx.cr[6].eq {
	pc = 0x83177F14; continue 'dispatch;
	}
	// 83177EE4: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83177EE8: 419A002C  beq cr6, 0x83177f14
	if ctx.cr[6].eq {
	pc = 0x83177F14; continue 'dispatch;
	}
	// 83177EEC: 81690010  lwz r11, 0x10(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 83177EF0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83177EF4: 409A0008  bne cr6, 0x83177efc
	if !ctx.cr[6].eq {
	pc = 0x83177EFC; continue 'dispatch;
	}
	// 83177EF8: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 83177EFC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83177F00: 409A0008  bne cr6, 0x83177f08
	if !ctx.cr[6].eq {
	pc = 0x83177F08; continue 'dispatch;
	}
	// 83177F04: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 83177F08: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83177F0C: 409A0008  bne cr6, 0x83177f14
	if !ctx.cr[6].eq {
	pc = 0x83177F14; continue 'dispatch;
	}
	// 83177F10: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83177F14: 8169001C  lwz r11, 0x1c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 83177F18: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83177F1C: 409A003C  bne cr6, 0x83177f58
	if !ctx.cr[6].eq {
	pc = 0x83177F58; continue 'dispatch;
	}
	// 83177F20: 7F0AF000  cmpw cr6, r10, r30
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83177F24: 419A0034  beq cr6, 0x83177f58
	if ctx.cr[6].eq {
	pc = 0x83177F58; continue 'dispatch;
	}
	// 83177F28: 7F0AE000  cmpw cr6, r10, r28
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83177F2C: 419A002C  beq cr6, 0x83177f58
	if ctx.cr[6].eq {
	pc = 0x83177F58; continue 'dispatch;
	}
	// 83177F30: 81690020  lwz r11, 0x20(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 83177F34: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83177F38: 409A0008  bne cr6, 0x83177f40
	if !ctx.cr[6].eq {
	pc = 0x83177F40; continue 'dispatch;
	}
	// 83177F3C: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 83177F40: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83177F44: 409A0008  bne cr6, 0x83177f4c
	if !ctx.cr[6].eq {
	pc = 0x83177F4C; continue 'dispatch;
	}
	// 83177F48: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 83177F4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83177F50: 409A0008  bne cr6, 0x83177f58
	if !ctx.cr[6].eq {
	pc = 0x83177F58; continue 'dispatch;
	}
	// 83177F54: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83177F58: 8169002C  lwz r11, 0x2c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(44 as u32) ) } as u64;
	// 83177F5C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83177F60: 409A0040  bne cr6, 0x83177fa0
	if !ctx.cr[6].eq {
	pc = 0x83177FA0; continue 'dispatch;
	}
	// 83177F64: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 83177F68: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83177F6C: 419A0034  beq cr6, 0x83177fa0
	if ctx.cr[6].eq {
	pc = 0x83177FA0; continue 'dispatch;
	}
	// 83177F70: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83177F74: 419A002C  beq cr6, 0x83177fa0
	if ctx.cr[6].eq {
	pc = 0x83177FA0; continue 'dispatch;
	}
	// 83177F78: 81690030  lwz r11, 0x30(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(48 as u32) ) } as u64;
	// 83177F7C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83177F80: 409A0008  bne cr6, 0x83177f88
	if !ctx.cr[6].eq {
	pc = 0x83177F88; continue 'dispatch;
	}
	// 83177F84: 3B400001  li r26, 1
	ctx.r[26].s64 = 1;
	// 83177F88: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83177F8C: 409A0008  bne cr6, 0x83177f94
	if !ctx.cr[6].eq {
	pc = 0x83177F94; continue 'dispatch;
	}
	// 83177F90: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 83177F94: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83177F98: 409A0008  bne cr6, 0x83177fa0
	if !ctx.cr[6].eq {
	pc = 0x83177FA0; continue 'dispatch;
	}
	// 83177F9C: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 83177FA0: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 83177FA4: 39290040  addi r9, r9, 0x40
	ctx.r[9].s64 = ctx.r[9].s64 + 64;
	// 83177FA8: 396AFFFE  addi r11, r10, -2
	ctx.r[11].s64 = ctx.r[10].s64 + -2;
	// 83177FAC: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 83177FB0: 4198FED4  blt cr6, 0x83177e84
	if ctx.cr[6].lt {
	pc = 0x83177E84; continue 'dispatch;
	}
	// 83177FB4: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 83177FB8: 419A000C  beq cr6, 0x83177fc4
	if ctx.cr[6].eq {
	pc = 0x83177FC4; continue 'dispatch;
	}
	// 83177FBC: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 83177FC0: 409A0014  bne cr6, 0x83177fd4
	if !ctx.cr[6].eq {
	pc = 0x83177FD4; continue 'dispatch;
	}
	// 83177FC4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83177FC8: 38800043  li r4, 0x43
	ctx.r[4].s64 = 67;
	// 83177FCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83177FD0: 4BFFA6F9  bl 0x831726c8
	ctx.lr = 0x83177FD4;
	sub_831726C8(ctx, base);
	// 83177FD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83177FD8: 4BFFAAE9  bl 0x83172ac0
	ctx.lr = 0x83177FDC;
	sub_83172AC0(ctx, base);
	// 83177FDC: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 83177FE0: 409A0010  bne cr6, 0x83177ff0
	if !ctx.cr[6].eq {
	pc = 0x83177FF0; continue 'dispatch;
	}
	// 83177FE4: 81631F94  lwz r11, 0x1f94(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8084 as u32) ) } as u64;
	// 83177FE8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83177FEC: 4800002C  b 0x83178018
	pc = 0x83178018; continue 'dispatch;
	// 83177FF0: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 83177FF4: 409A0020  bne cr6, 0x83178014
	if !ctx.cr[6].eq {
	pc = 0x83178014; continue 'dispatch;
	}
	// 83177FF8: 81631F98  lwz r11, 0x1f98(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8088 as u32) ) } as u64;
	// 83177FFC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83178000: 409A001C  bne cr6, 0x8317801c
	if !ctx.cr[6].eq {
	pc = 0x8317801C; continue 'dispatch;
	}
	// 83178004: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83178008: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317800C: 4BFFA69D  bl 0x831726a8
	ctx.lr = 0x83178010;
	sub_831726A8(ctx, base);
	// 83178010: 4800000C  b 0x8317801c
	pc = 0x8317801C; continue 'dispatch;
	// 83178014: 2F1A0000  cmpwi cr6, r26, 0
	ctx.cr[6].compare_i32(ctx.r[26].s32, 0, &mut ctx.xer);
	// 83178018: 409A0010  bne cr6, 0x83178028
	if !ctx.cr[6].eq {
	pc = 0x83178028; continue 'dispatch;
	}
	// 8317801C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83178020: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178024: 4BFFA685  bl 0x831726a8
	ctx.lr = 0x83178028;
	sub_831726A8(ctx, base);
	// 83178028: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 8317802C: 409A001C  bne cr6, 0x83178048
	if !ctx.cr[6].eq {
	pc = 0x83178048; continue 'dispatch;
	}
	// 83178030: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 83178034: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178038: 4BFFA851  bl 0x83172888
	ctx.lr = 0x8317803C;
	sub_83172888(ctx, base);
	// 8317803C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83178040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178044: 4BFFA665  bl 0x831726a8
	ctx.lr = 0x83178048;
	sub_831726A8(ctx, base);
	// 83178048: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317804C: 48030160  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178050(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83178050 size=120
    let mut pc: u32 = 0x83178050;
    'dispatch: loop {
        match pc {
            0x83178050 => {
    //   block [0x83178050..0x831780C8)
	// 83178050: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83178054: 48030119  bl 0x831a816c
	ctx.lr = 0x83178058;
	sub_831A8130(ctx, base);
	// 83178058: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317805C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83178060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83178064: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83178068: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317806C: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 83178070: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83178074: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83178078: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317807C: 480106A5  bl 0x83188720
	ctx.lr = 0x83178080;
	sub_83188720(ctx, base);
	// 83178080: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83178084: 409A003C  bne cr6, 0x831780c0
	if !ctx.cr[6].eq {
	pc = 0x831780C0; continue 'dispatch;
	}
	// 83178088: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317808C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83178090: 409A0030  bne cr6, 0x831780c0
	if !ctx.cr[6].eq {
	pc = 0x831780C0; continue 'dispatch;
	}
	// 83178094: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 83178098: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8317809C: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 831780A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831780A4: 480109ED  bl 0x83188a90
	ctx.lr = 0x831780A8;
	sub_83188A90(ctx, base);
	// 831780A8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831780AC: 409A0014  bne cr6, 0x831780c0
	if !ctx.cr[6].eq {
	pc = 0x831780C0; continue 'dispatch;
	}
	// 831780B0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831780B4: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 831780B8: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831780BC: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 831780C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831780C4: 480300F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831780C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831780C8 size=136
    let mut pc: u32 = 0x831780C8;
    'dispatch: loop {
        match pc {
            0x831780C8 => {
    //   block [0x831780C8..0x83178150)
	// 831780C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831780CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831780D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 831780D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831780D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831780DC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831780E0: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 831780E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831780E8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831780EC: 48010635  bl 0x83188720
	ctx.lr = 0x831780F0;
	sub_83188720(ctx, base);
	// 831780F0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831780F4: 409A0040  bne cr6, 0x83178134
	if !ctx.cr[6].eq {
	pc = 0x83178134; continue 'dispatch;
	}
	// 831780F8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831780FC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83178100: 409A0034  bne cr6, 0x83178134
	if !ctx.cr[6].eq {
	pc = 0x83178134; continue 'dispatch;
	}
	// 83178104: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83178108: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 8317810C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178110: 480109D1  bl 0x83188ae0
	ctx.lr = 0x83178114;
	sub_83188AE0(ctx, base);
	// 83178114: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83178118: 409A001C  bne cr6, 0x83178134
	if !ctx.cr[6].eq {
	pc = 0x83178134; continue 'dispatch;
	}
	// 8317811C: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83178120: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83178124: 409A0008  bne cr6, 0x8317812c
	if !ctx.cr[6].eq {
	pc = 0x8317812C; continue 'dispatch;
	}
	// 83178128: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8317812C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83178130: 48000008  b 0x83178138
	pc = 0x83178138; continue 'dispatch;
	// 83178134: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83178138: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317813C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83178140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83178144: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83178148: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317814C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x83178150 size=52
    let mut pc: u32 = 0x83178150;
    'dispatch: loop {
        match pc {
            0x83178150 => {
    //   block [0x83178150..0x83178184)
	// 83178150: 816300C8  lwz r11, 0xc8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(200 as u32) ) } as u64;
	// 83178154: 7D6A1E70  srawi r10, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 83178158: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 8317815C: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83178160: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 83178164: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83178168: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317816C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83178170: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 83178174: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83178178: 814B00D0  lwz r10, 0xd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 8317817C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 83178180: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178184(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83178184 size=8
    let mut pc: u32 = 0x83178184;
    'dispatch: loop {
        match pc {
            0x83178184 => {
    //   block [0x83178184..0x8317818C)
	// 83178184: 806B00E4  lwz r3, 0xe4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(228 as u32) ) } as u64;
	// 83178188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83178190 size=336
    let mut pc: u32 = 0x83178190;
    'dispatch: loop {
        match pc {
            0x83178190 => {
    //   block [0x83178190..0x831782E0)
	// 83178190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83178194: 4802FFA1  bl 0x831a8134
	ctx.lr = 0x83178198;
	sub_831A8130(ctx, base);
	// 83178198: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317819C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831781A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831781A4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 831781A8: 835D0048  lwz r26, 0x48(r29)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 831781AC: 831E0020  lwz r24, 0x20(r30)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 831781B0: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 831781B4: 4BFFF615  bl 0x831777c8
	ctx.lr = 0x831781B8;
	sub_831777C8(ctx, base);
	// 831781B8: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 831781BC: 82DE0000  lwz r22, 0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831781C0: 82BE0004  lwz r21, 4(r30)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 831781C4: 829E0008  lwz r20, 8(r30)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 831781C8: 827E000C  lwz r19, 0xc(r30)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 831781CC: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 831781D0: 4BFFF621  bl 0x831777f0
	ctx.lr = 0x831781D4;
	sub_831777F0(ctx, base);
	// 831781D4: 7C721B78  mr r18, r3
	ctx.r[18].u64 = ctx.r[3].u64;
	// 831781D8: 837E0034  lwz r27, 0x34(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 831781DC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 831781E0: 839E0018  lwz r28, 0x18(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 831781E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831781E8: 835E0030  lwz r26, 0x30(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 831781EC: 823E002C  lwz r17, 0x2c(r30)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 831781F0: 821E0024  lwz r16, 0x24(r30)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 831781F4: 81FE0028  lwz r15, 0x28(r30)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 831781F8: 48003C71  bl 0x8317be68
	ctx.lr = 0x831781FC;
	sub_8317BE68(ctx, base);
	// 831781FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83178200: 419A0010  beq cr6, 0x83178210
	if ctx.cr[6].eq {
	pc = 0x83178210; continue 'dispatch;
	}
	// 83178204: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83178208: 386BA8F8  addi r3, r11, -0x5708
	ctx.r[3].s64 = ctx.r[11].s64 + -22280;
	// 8317820C: 48004F35  bl 0x8317d140
	ctx.lr = 0x83178210;
	sub_8317D140(ctx, base);
	// 83178210: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83178214: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83178218: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8317821C: 4BFFF5A5  bl 0x831777c0
	ctx.lr = 0x83178220;
	sub_831777C0(ctx, base);
	// 83178220: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 83178224: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 83178228: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317822C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83178230: 4BFFF591  bl 0x831777c0
	ctx.lr = 0x83178234;
	sub_831777C0(ctx, base);
	// 83178234: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83178238: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317823C: 931F0000  stw r24, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 83178240: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83178244: 92FF0008  stw r23, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 83178248: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317824C: 92DF000C  stw r22, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[22].u32 ) };
	// 83178250: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83178254: 92BF0010  stw r21, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[21].u32 ) };
	// 83178258: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 8317825C: 929F0014  stw r20, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[20].u32 ) };
	// 83178260: 927F0018  stw r19, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[19].u32 ) };
	// 83178264: 925F001C  stw r18, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[18].u32 ) };
	// 83178268: 933F0024  stw r25, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[25].u32 ) };
	// 8317826C: 937F0028  stw r27, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[27].u32 ) };
	// 83178270: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 83178274: 923F0030  stw r17, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[17].u32 ) };
	// 83178278: 935F0038  stw r26, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[26].u32 ) };
	// 8317827C: 921F003C  stw r16, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[16].u32 ) };
	// 83178280: 91FF0040  stw r15, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[15].u32 ) };
	// 83178284: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 83178288: 4BFFF4C1  bl 0x83177748
	ctx.lr = 0x8317828C;
	sub_83177748(ctx, base);
	// 8317828C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83178290: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83178294: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83178298: 4BFFF6D1  bl 0x83177968
	ctx.lr = 0x8317829C;
	sub_83177968(ctx, base);
	// 8317829C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831782A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 831782A4: 80DF009C  lwz r6, 0x9c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 831782A8: 80BF0098  lwz r5, 0x98(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 831782AC: 4BFFF5E5  bl 0x83177890
	ctx.lr = 0x831782B0;
	sub_83177890(ctx, base);
	// 831782B0: 397F0060  addi r11, r31, 0x60
	ctx.r[11].s64 = ctx.r[31].s64 + 96;
	// 831782B4: 393E0048  addi r9, r30, 0x48
	ctx.r[9].s64 = ctx.r[30].s64 + 72;
	// 831782B8: 907F004C  stw r3, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[3].u32 ) };
	// 831782BC: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 831782C0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 831782C4: E9490000  ld r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 831782C8: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 831782CC: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 831782D0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 831782D4: 4200FFF0  bdnz 0x831782c4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x831782C4; continue 'dispatch;
	}
	// 831782D8: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 831782DC: 4802FEA8  b 0x831a8184
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831782E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831782E0 size=60
    let mut pc: u32 = 0x831782E0;
    'dispatch: loop {
        match pc {
            0x831782E0 => {
    //   block [0x831782E0..0x8317831C)
	// 831782E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831782E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831782E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831782EC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831782F0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 831782F4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 831782F8: 4800F8E1  bl 0x83187bd8
	ctx.lr = 0x831782FC;
	sub_83187BD8(ctx, base);
	// 831782FC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83178300: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 83178304: 4BFFF6F5  bl 0x831779f8
	ctx.lr = 0x83178308;
	sub_831779F8(ctx, base);
	// 83178308: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317830C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83178310: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83178314: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83178318: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83178320 size=4
    let mut pc: u32 = 0x83178320;
    'dispatch: loop {
        match pc {
            0x83178320 => {
    //   block [0x83178320..0x83178324)
	// 83178320: 4BFFFE30  b 0x83178150
	sub_83178150(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83178328 size=284
    let mut pc: u32 = 0x83178328;
    'dispatch: loop {
        match pc {
            0x83178328 => {
    //   block [0x83178328..0x83178444)
	// 83178328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317832C: 4802FE31  bl 0x831a815c
	ctx.lr = 0x83178330;
	sub_831A8130(ctx, base);
	// 83178330: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83178334: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 83178338: 3B40FFFF  li r26, -1
	ctx.r[26].s64 = -1;
	// 8317833C: 2F040800  cmpwi cr6, r4, 0x800
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2048, &mut ctx.xer);
	// 83178340: 935C0014  stw r26, 0x14(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 83178344: 419800F8  blt cr6, 0x8317843c
	if ctx.cr[6].lt {
	pc = 0x8317843C; continue 'dispatch;
	}
	// 83178348: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317834C: 419A00F0  beq cr6, 0x8317843c
	if ctx.cr[6].eq {
	pc = 0x8317843C; continue 'dispatch;
	}
	// 83178350: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83178354: 3B600002  li r27, 2
	ctx.r[27].s64 = 2;
	// 83178358: 3BA30800  addi r29, r3, 0x800
	ctx.r[29].s64 = ctx.r[3].s64 + 2048;
	// 8317835C: 3BC4F800  addi r30, r4, -0x800
	ctx.r[30].s64 = ctx.r[4].s64 + -2048;
	// 83178360: 3B2BA918  addi r25, r11, -0x56e8
	ctx.r[25].s64 = ctx.r[11].s64 + -22248;
	// 83178364: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83178368: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317836C: 48010A9D  bl 0x83188e08
	ctx.lr = 0x83178370;
	sub_83188E08(ctx, base);
	// 83178370: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83178374: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 83178378: 409A0010  bne cr6, 0x83178388
	if !ctx.cr[6].eq {
	pc = 0x83178388; continue 'dispatch;
	}
	// 8317837C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 83178380: 48004DC1  bl 0x8317d140
	ctx.lr = 0x83178384;
	sub_8317D140(ctx, base);
	// 83178384: 48000034  b 0x831783b8
	pc = 0x831783B8; continue 'dispatch;
	// 83178388: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8317838C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178390: 48010B29  bl 0x83188eb8
	ctx.lr = 0x83178394;
	sub_83188EB8(ctx, base);
	// 83178394: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83178398: 409A0010  bne cr6, 0x831783a8
	if !ctx.cr[6].eq {
	pc = 0x831783A8; continue 'dispatch;
	}
	// 8317839C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831783A0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831783A4: 419A0030  beq cr6, 0x831783d4
	if ctx.cr[6].eq {
	pc = 0x831783D4; continue 'dispatch;
	}
	// 831783A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831783AC: 935C0014  stw r26, 0x14(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[26].u32 ) };
	// 831783B0: 935C0018  stw r26, 0x18(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[26].u32 ) };
	// 831783B4: 48010335  bl 0x831886e8
	ctx.lr = 0x831783B8;
	sub_831886E8(ctx, base);
	// 831783B8: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 831783BC: 3BDEF800  addi r30, r30, -0x800
	ctx.r[30].s64 = ctx.r[30].s64 + -2048;
	// 831783C0: 3BBD0800  addi r29, r29, 0x800
	ctx.r[29].s64 = ctx.r[29].s64 + 2048;
	// 831783C4: 2F1B0003  cmpwi cr6, r27, 3
	ctx.cr[6].compare_i32(ctx.r[27].s32, 3, &mut ctx.xer);
	// 831783C8: 4099FF9C  ble cr6, 0x83178364
	if !ctx.cr[6].gt {
	pc = 0x83178364; continue 'dispatch;
	}
	// 831783CC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831783D0: 4802FDDC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 831783D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831783D8: 4BFFF749  bl 0x83177b20
	ctx.lr = 0x831783DC;
	sub_83177B20(ctx, base);
	// 831783DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831783E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831783E4: 917C0014  stw r11, 0x14(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 831783E8: 4BFFF7F1  bl 0x83177bd8
	ctx.lr = 0x831783EC;
	sub_83177BD8(ctx, base);
	// 831783EC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 831783F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831783F4: 917C0018  stw r11, 0x18(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 831783F8: 4BFFF8A9  bl 0x83177ca0
	ctx.lr = 0x831783FC;
	sub_83177CA0(ctx, base);
	// 831783FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83178400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178404: 917C0020  stw r11, 0x20(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 83178408: 4BFFF861  bl 0x83177c68
	ctx.lr = 0x8317840C;
	sub_83177C68(ctx, base);
	// 8317840C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83178410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178414: 917C0024  stw r11, 0x24(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83178418: 4BFFF779  bl 0x83177b90
	ctx.lr = 0x8317841C;
	sub_83177B90(ctx, base);
	// 8317841C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83178420: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178424: 917C001C  stw r11, 0x1c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83178428: 4BFFF731  bl 0x83177b58
	ctx.lr = 0x8317842C;
	sub_83177B58(ctx, base);
	// 8317842C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 83178430: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178434: 917C0030  stw r11, 0x30(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83178438: 480102B1  bl 0x831886e8
	ctx.lr = 0x8317843C;
	sub_831886E8(ctx, base);
	// 8317843C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 83178440: 4802FD6C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178448(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83178448 size=260
    let mut pc: u32 = 0x83178448;
    'dispatch: loop {
        match pc {
            0x83178448 => {
    //   block [0x83178448..0x8317854C)
	// 83178448: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317844C: 4802FD0D  bl 0x831a8158
	ctx.lr = 0x83178450;
	sub_831A8130(ctx, base);
	// 83178450: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83178454: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83178458: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8317845C: 817A01F0  lwz r11, 0x1f0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(496 as u32) ) } as u64;
	// 83178460: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83178464: 419A00E0  beq cr6, 0x83178544
	if ctx.cr[6].eq {
	pc = 0x83178544; continue 'dispatch;
	}
	// 83178468: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8317846C: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 83178470: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	// 83178474: 395D0020  addi r10, r29, 0x20
	ctx.r[10].s64 = ctx.r[29].s64 + 32;
	// 83178478: 57AB2036  slwi r11, r29, 4
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8317847C: 393D00C0  addi r9, r29, 0xc0
	ctx.r[9].s64 = ctx.r[29].s64 + 192;
	// 83178480: 7FEBD214  add r31, r11, r26
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 83178484: 55592036  slwi r25, r10, 4
	ctx.r[25].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 83178488: 553E063E  clrlwi r30, r9, 0x18
	ctx.r[30].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 8317848C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83178490: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83178494: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83178498: 939F01F4  stw r28, 0x1f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[28].u32 ) };
	// 8317849C: 939F01F8  stw r28, 0x1f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(504 as u32), ctx.r[28].u32 ) };
	// 831784A0: 939F01FC  stw r28, 0x1fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[28].u32 ) };
	// 831784A4: 7F99D12E  stwx r28, r25, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32), ctx.r[28].u32) };
	// 831784A8: 48010279  bl 0x83188720
	ctx.lr = 0x831784AC;
	sub_83188720(ctx, base);
	// 831784AC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831784B0: 409A0078  bne cr6, 0x83178528
	if !ctx.cr[6].eq {
	pc = 0x83178528; continue 'dispatch;
	}
	// 831784B4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831784B8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831784BC: 409A006C  bne cr6, 0x83178528
	if !ctx.cr[6].eq {
	pc = 0x83178528; continue 'dispatch;
	}
	// 831784C0: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 831784C4: 931F01F4  stw r24, 0x1f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[24].u32 ) };
	// 831784C8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831784CC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831784D0: 480104D1  bl 0x831889a0
	ctx.lr = 0x831784D4;
	sub_831889A0(ctx, base);
	// 831784D4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831784D8: 409A0010  bne cr6, 0x831784e8
	if !ctx.cr[6].eq {
	pc = 0x831784E8; continue 'dispatch;
	}
	// 831784DC: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 831784E0: 4BFFF869  bl 0x83177d48
	ctx.lr = 0x831784E4;
	sub_83177D48(ctx, base);
	// 831784E4: 907F01F8  stw r3, 0x1f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(504 as u32), ctx.r[3].u32 ) };
	// 831784E8: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 831784EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831784F0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 831784F4: 480104FD  bl 0x831889f0
	ctx.lr = 0x831784F8;
	sub_831889F0(ctx, base);
	// 831784F8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831784FC: 409A000C  bne cr6, 0x83178508
	if !ctx.cr[6].eq {
	pc = 0x83178508; continue 'dispatch;
	}
	// 83178500: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83178504: 917F01FC  stw r11, 0x1fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[11].u32 ) };
	// 83178508: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 8317850C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83178510: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83178514: 48010505  bl 0x83188a18
	ctx.lr = 0x83178518;
	sub_83188A18(ctx, base);
	// 83178518: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317851C: 409A000C  bne cr6, 0x83178528
	if !ctx.cr[6].eq {
	pc = 0x83178528; continue 'dispatch;
	}
	// 83178520: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83178524: 7D79D12E  stwx r11, r25, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32), ctx.r[11].u32) };
	// 83178528: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 8317852C: 557D063E  clrlwi r29, r11, 0x18
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 83178530: 2B1D0020  cmplwi cr6, r29, 0x20
	ctx.cr[6].compare_u32(ctx.r[29].u32, 32 as u32, &mut ctx.xer);
	// 83178534: 4198FF40  blt cr6, 0x83178474
	if ctx.cr[6].lt {
	pc = 0x83178474; continue 'dispatch;
	}
	// 83178538: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 8317853C: 931A01F0  stw r24, 0x1f0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(496 as u32), ctx.r[24].u32 ) };
	// 83178540: 4BFFF881  bl 0x83177dc0
	ctx.lr = 0x83178544;
	sub_83177DC0(ctx, base);
	// 83178544: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83178548: 4802FC60  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83178550 size=4
    let mut pc: u32 = 0x83178550;
    'dispatch: loop {
        match pc {
            0x83178550 => {
    //   block [0x83178550..0x83178554)
	// 83178550: 4BFFF7F8  b 0x83177d48
	sub_83177D48(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83178558 size=316
    let mut pc: u32 = 0x83178558;
    'dispatch: loop {
        match pc {
            0x83178558 => {
    //   block [0x83178558..0x83178694)
	// 83178558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317855C: 4802FC09  bl 0x831a8164
	ctx.lr = 0x83178560;
	sub_831A8130(ctx, base);
	// 83178560: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83178564: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83178568: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8317856C: 4BFFA1DD  bl 0x83172748
	ctx.lr = 0x83178570;
	sub_83172748(ctx, base);
	// 83178570: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83178574: 419A0030  beq cr6, 0x831785a4
	if ctx.cr[6].eq {
	pc = 0x831785A4; continue 'dispatch;
	}
	// 83178578: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317857C: 386BA93C  addi r3, r11, -0x56c4
	ctx.r[3].s64 = ctx.r[11].s64 + -22212;
	// 83178580: 48004BC1  bl 0x8317d140
	ctx.lr = 0x83178584;
	sub_8317D140(ctx, base);
	// 83178584: 38A000A0  li r5, 0xa0
	ctx.r[5].s64 = 160;
	// 83178588: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317858C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83178590: 4802FC51  bl 0x831a81e0
	ctx.lr = 0x83178594;
	sub_831A81E0(ctx, base);
	// 83178594: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83178598: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317859C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831785A0: 4802FC14  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831785A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831785A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831785AC: 4BFFF125  bl 0x831776d0
	ctx.lr = 0x831785B0;
	sub_831776D0(ctx, base);
	// 831785B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831785B4: 4BFFA50D  bl 0x83172ac0
	ctx.lr = 0x831785B8;
	sub_83172AC0(ctx, base);
	// 831785B8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 831785BC: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 831785C0: 419AFFC4  beq cr6, 0x83178584
	if ctx.cr[6].eq {
	pc = 0x83178584; continue 'dispatch;
	}
	// 831785C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831785C8: 4800B7C9  bl 0x83183d90
	ctx.lr = 0x831785CC;
	sub_83183D90(ctx, base);
	// 831785CC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 831785D0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 831785D4: 419AFFC0  beq cr6, 0x83178594
	if ctx.cr[6].eq {
	pc = 0x83178594; continue 'dispatch;
	}
	// 831785D8: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 831785DC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831785E0: 409A0058  bne cr6, 0x83178638
	if !ctx.cr[6].eq {
	pc = 0x83178638; continue 'dispatch;
	}
	// 831785E4: 839F0018  lwz r28, 0x18(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 831785E8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 831785EC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 831785F0: 40990048  ble cr6, 0x83178638
	if !ctx.cr[6].gt {
	pc = 0x83178638; continue 'dispatch;
	}
	// 831785F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831785F8: 4BFFF4B9  bl 0x83177ab0
	ctx.lr = 0x831785FC;
	sub_83177AB0(ctx, base);
	// 831785FC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83178600: 409A0034  bne cr6, 0x83178634
	if !ctx.cr[6].eq {
	pc = 0x83178634; continue 'dispatch;
	}
	// 83178604: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83178608: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317860C: 4800B8ED  bl 0x83183ef8
	ctx.lr = 0x83178610;
	sub_83183EF8(ctx, base);
	// 83178610: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 83178614: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83178618: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 8317861C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83178620: 917F0094  stw r11, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 83178624: 4800B76D  bl 0x83183d90
	ctx.lr = 0x83178628;
	sub_83183D90(ctx, base);
	// 83178628: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8317862C: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 83178630: 4198FFC4  blt cr6, 0x831785f4
	if ctx.cr[6].lt {
	pc = 0x831785F4; continue 'dispatch;
	}
	// 83178634: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83178638: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 8317863C: 419AFF58  beq cr6, 0x83178594
	if ctx.cr[6].eq {
	pc = 0x83178594; continue 'dispatch;
	}
	// 83178640: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 83178644: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178648: 909F0088  stw r4, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[4].u32 ) };
	// 8317864C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83178650: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 83178654: 4BFFF2C5  bl 0x83177918
	ctx.lr = 0x83178658;
	sub_83177918(ctx, base);
	// 83178658: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8317865C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83178660: 4BFFFB31  bl 0x83178190
	ctx.lr = 0x83178664;
	sub_83178190(ctx, base);
	// 83178664: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 83178668: 815B0030  lwz r10, 0x30(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 8317866C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83178670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178674: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 83178678: 915F00C8  stw r10, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 8317867C: 4BFFF07D  bl 0x831776f8
	ctx.lr = 0x83178680;
	sub_831776F8(ctx, base);
	// 83178680: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 83178684: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178688: 4BFFEFD9  bl 0x83177660
	ctx.lr = 0x8317868C;
	sub_83177660(ctx, base);
	// 8317868C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83178690: 4802FB24  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83178698 size=344
    let mut pc: u32 = 0x83178698;
    'dispatch: loop {
        match pc {
            0x83178698 => {
    //   block [0x83178698..0x831787F0)
	// 83178698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317869C: 4802FAC1  bl 0x831a815c
	ctx.lr = 0x831786A0;
	sub_831A8130(ctx, base);
	// 831786A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831786A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 831786A8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 831786AC: 2F050800  cmpwi cr6, r5, 0x800
	ctx.cr[6].compare_i32(ctx.r[5].s32, 2048, &mut ctx.xer);
	// 831786B0: 817E00C4  lwz r11, 0xc4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 831786B4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831786B8: 917E00C4  stw r11, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 831786BC: 4198012C  blt cr6, 0x831787e8
	if ctx.cr[6].lt {
	pc = 0x831787E8; continue 'dispatch;
	}
	// 831786C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831786C4: 419A0124  beq cr6, 0x831787e8
	if ctx.cr[6].eq {
	pc = 0x831787E8; continue 'dispatch;
	}
	// 831786C8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 831786CC: 4801073D  bl 0x83188e08
	ctx.lr = 0x831786D0;
	sub_83188E08(ctx, base);
	// 831786D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831786D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831786D8: 409A0018  bne cr6, 0x831786f0
	if !ctx.cr[6].eq {
	pc = 0x831786F0; continue 'dispatch;
	}
	// 831786DC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831786E0: 386BA968  addi r3, r11, -0x5698
	ctx.r[3].s64 = ctx.r[11].s64 + -22168;
	// 831786E4: 48004A5D  bl 0x8317d140
	ctx.lr = 0x831786E8;
	sub_8317D140(ctx, base);
	// 831786E8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831786EC: 4802FAC0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 831786F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 831786F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831786F8: 480107C1  bl 0x83188eb8
	ctx.lr = 0x831786FC;
	sub_83188EB8(ctx, base);
	// 831786FC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83178700: 409A00E0  bne cr6, 0x831787e0
	if !ctx.cr[6].eq {
	pc = 0x831787E0; continue 'dispatch;
	}
	// 83178704: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83178708: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8317870C: 409A00D4  bne cr6, 0x831787e0
	if !ctx.cr[6].eq {
	pc = 0x831787E0; continue 'dispatch;
	}
	// 83178710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178714: 4BFFF9B5  bl 0x831780c8
	ctx.lr = 0x83178718;
	sub_831780C8(ctx, base);
	// 83178718: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317871C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178720: 4BFFF401  bl 0x83177b20
	ctx.lr = 0x83178724;
	sub_83177B20(ctx, base);
	// 83178724: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83178728: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317872C: 4BFFF4AD  bl 0x83177bd8
	ctx.lr = 0x83178730;
	sub_83177BD8(ctx, base);
	// 83178730: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 83178734: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178738: 4BFFF459  bl 0x83177b90
	ctx.lr = 0x8317873C;
	sub_83177B90(ctx, base);
	// 8317873C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 83178740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178744: 4BFFF415  bl 0x83177b58
	ctx.lr = 0x83178748;
	sub_83177B58(ctx, base);
	// 83178748: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8317874C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 83178750: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83178754: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178758: 4BFFF8F9  bl 0x83178050
	ctx.lr = 0x8317875C;
	sub_83178050(ctx, base);
	// 8317875C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83178760: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83178764: 4BFFFCE5  bl 0x83178448
	ctx.lr = 0x83178768;
	sub_83178448(ctx, base);
	// 83178768: 817E00CC  lwz r11, 0xcc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 8317876C: 815E00C4  lwz r10, 0xc4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 83178770: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83178774: 55691838  slwi r9, r11, 3
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83178778: 80E10054  lwz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317877C: 390AFFFF  addi r8, r10, -1
	ctx.r[8].s64 = ctx.r[10].s64 + -1;
	// 83178780: 80C10058  lwz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83178784: 394B0006  addi r10, r11, 6
	ctx.r[10].s64 = ctx.r[11].s64 + 6;
	// 83178788: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8317878C: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 83178790: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83178794: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 83178798: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8317879C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831787A0: 910B00D4  stw r8, 0xd4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(212 as u32), ctx.r[8].u32 ) };
	// 831787A4: 7CEAF12E  stwx r7, r10, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[7].u32) };
	// 831787A8: 93AB00E0  stw r29, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[29].u32 ) };
	// 831787AC: 938B00E4  stw r28, 0xe4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(228 as u32), ctx.r[28].u32 ) };
	// 831787B0: 936B00E8  stw r27, 0xe8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(232 as u32), ctx.r[27].u32 ) };
	// 831787B4: 934B00EC  stw r26, 0xec(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(236 as u32), ctx.r[26].u32 ) };
	// 831787B8: 932B00F0  stw r25, 0xf0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(240 as u32), ctx.r[25].u32 ) };
	// 831787BC: 90CB00DC  stw r6, 0xdc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(220 as u32), ctx.r[6].u32 ) };
	// 831787C0: 90AB00D0  stw r5, 0xd0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(208 as u32), ctx.r[5].u32 ) };
	// 831787C4: 817E00CC  lwz r11, 0xcc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 831787C8: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831787CC: 7D6A1E70  srawi r10, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 831787D0: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 831787D4: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831787D8: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 831787DC: 917E00CC  stw r11, 0xcc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 831787E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831787E4: 4800FF05  bl 0x831886e8
	ctx.lr = 0x831787E8;
	sub_831886E8(ctx, base);
	// 831787E8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831787EC: 4802F9C0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831787F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831787F0 size=20
    let mut pc: u32 = 0x831787F0;
    'dispatch: loop {
        match pc {
            0x831787F0 => {
    //   block [0x831787F0..0x83178804)
	// 831787F0: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 831787F4: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 831787F8: 388B8698  addi r4, r11, -0x7968
	ctx.r[4].s64 = ctx.r[11].s64 + -31080;
	// 831787FC: 80650048  lwz r3, 0x48(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(72 as u32) ) } as u64;
	// 83178800: 48000C20  b 0x83179420
	sub_83179420(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83178808 size=232
    let mut pc: u32 = 0x83178808;
    'dispatch: loop {
        match pc {
            0x83178808 => {
    //   block [0x83178808..0x831788F0)
	// 83178808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317880C: 4802F95D  bl 0x831a8168
	ctx.lr = 0x83178810;
	sub_831A8130(ctx, base);
	// 83178810: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83178814: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83178818: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317881C: 4BFF9F2D  bl 0x83172748
	ctx.lr = 0x83178820;
	sub_83172748(ctx, base);
	// 83178820: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83178824: 419A0018  beq cr6, 0x8317883c
	if ctx.cr[6].eq {
	pc = 0x8317883C; continue 'dispatch;
	}
	// 83178828: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 8317882C: 386BA9E4  addi r3, r11, -0x561c
	ctx.r[3].s64 = ctx.r[11].s64 + -22044;
	// 83178830: 48004911  bl 0x8317d140
	ctx.lr = 0x83178834;
	sub_8317D140(ctx, base);
	// 83178834: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83178838: 4802F980  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8317883C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 83178840: 409A0018  bne cr6, 0x83178858
	if !ctx.cr[6].eq {
	pc = 0x83178858; continue 'dispatch;
	}
	// 83178844: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83178848: 386BA9BC  addi r3, r11, -0x5644
	ctx.r[3].s64 = ctx.r[11].s64 + -22084;
	// 8317884C: 480048F5  bl 0x8317d140
	ctx.lr = 0x83178850;
	sub_8317D140(ctx, base);
	// 83178850: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83178854: 4802F964  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83178858: 3F808345  lis r28, -0x7cbb
	ctx.r[28].s64 = -2092630016;
	// 8317885C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83178860: 3BABC8B0  addi r29, r11, -0x3750
	ctx.r[29].s64 = ctx.r[11].s64 + -14160;
	// 83178864: 807CA374  lwz r3, -0x5c8c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83178868: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317886C: 419A0020  beq cr6, 0x8317888c
	if ctx.cr[6].eq {
	pc = 0x8317888C; continue 'dispatch;
	}
	// 83178870: 93FD000C  stw r31, 0xc(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83178874: 389D0004  addi r4, r29, 4
	ctx.r[4].s64 = ctx.r[29].s64 + 4;
	// 83178878: 93DD0018  stw r30, 0x18(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
	// 8317887C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83178880: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83178884: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83178888: 4E800421  bctrl
	ctx.lr = 0x8317888C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317888C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83178890: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83178894: 4BFBD2CD  bl 0x83135b60
	ctx.lr = 0x83178898;
	sub_83135B60(ctx, base);
	// 83178898: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317889C: 40980020  bge cr6, 0x831788bc
	if !ctx.cr[6].lt {
	pc = 0x831788BC; continue 'dispatch;
	}
	// 831788A0: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 831788A4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831788A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 831788AC: 386BA98C  addi r3, r11, -0x5674
	ctx.r[3].s64 = ctx.r[11].s64 + -22132;
	// 831788B0: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 831788B4: 4800488D  bl 0x8317d140
	ctx.lr = 0x831788B8;
	sub_8317D140(ctx, base);
	// 831788B8: 48000010  b 0x831788c8
	pc = 0x831788C8; continue 'dispatch;
	// 831788BC: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 831788C0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831788C4: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 831788C8: 807CA374  lwz r3, -0x5c8c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 831788CC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831788D0: 419A0018  beq cr6, 0x831788e8
	if ctx.cr[6].eq {
	pc = 0x831788E8; continue 'dispatch;
	}
	// 831788D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831788D8: 389D006C  addi r4, r29, 0x6c
	ctx.r[4].s64 = ctx.r[29].s64 + 108;
	// 831788DC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 831788E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831788E4: 4E800421  bctrl
	ctx.lr = 0x831788E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831788E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831788EC: 4802F8CC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831788F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831788F0 size=160
    let mut pc: u32 = 0x831788F0;
    'dispatch: loop {
        match pc {
            0x831788F0 => {
    //   block [0x831788F0..0x83178990)
	// 831788F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831788F4: 4802F879  bl 0x831a816c
	ctx.lr = 0x831788F8;
	sub_831A8130(ctx, base);
	// 831788F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831788FC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83178900: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 83178904: 4BFF9E45  bl 0x83172748
	ctx.lr = 0x83178908;
	sub_83172748(ctx, base);
	// 83178908: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317890C: 419A0018  beq cr6, 0x83178924
	if ctx.cr[6].eq {
	pc = 0x83178924; continue 'dispatch;
	}
	// 83178910: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83178914: 386BAA14  addi r3, r11, -0x55ec
	ctx.r[3].s64 = ctx.r[11].s64 + -21996;
	// 83178918: 48004829  bl 0x8317d140
	ctx.lr = 0x8317891C;
	sub_8317D140(ctx, base);
	// 8317891C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83178920: 4802F89C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83178924: 3FC08345  lis r30, -0x7cbb
	ctx.r[30].s64 = -2092630016;
	// 83178928: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 8317892C: 3BEBCB38  addi r31, r11, -0x34c8
	ctx.r[31].s64 = ctx.r[11].s64 + -13512;
	// 83178930: 807EA374  lwz r3, -0x5c8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83178934: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83178938: 419A0024  beq cr6, 0x8317895c
	if ctx.cr[6].eq {
	pc = 0x8317895C; continue 'dispatch;
	}
	// 8317893C: 3961008C  addi r11, r1, 0x8c
	ctx.r[11].s64 = ctx.r[1].s64 + 140;
	// 83178940: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 83178944: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 83178948: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 8317894C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83178950: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83178954: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83178958: 4E800421  bctrl
	ctx.lr = 0x8317895C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317895C: 8081008C  lwz r4, 0x8c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83178960: 807D0054  lwz r3, 0x54(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(84 as u32) ) } as u64;
	// 83178964: 4BFBD1A5  bl 0x83135b08
	ctx.lr = 0x83178968;
	sub_83135B08(ctx, base);
	// 83178968: 807EA374  lwz r3, -0x5c8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317896C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83178970: 419A0018  beq cr6, 0x83178988
	if ctx.cr[6].eq {
	pc = 0x83178988; continue 'dispatch;
	}
	// 83178974: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83178978: 389F006C  addi r4, r31, 0x6c
	ctx.r[4].s64 = ctx.r[31].s64 + 108;
	// 8317897C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83178980: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83178984: 4E800421  bctrl
	ctx.lr = 0x83178988;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83178988: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317898C: 4802F830  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83178990 size=164
    let mut pc: u32 = 0x83178990;
    'dispatch: loop {
        match pc {
            0x83178990 => {
    //   block [0x83178990..0x83178A34)
	// 83178990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83178994: 4802F7D5  bl 0x831a8168
	ctx.lr = 0x83178998;
	sub_831A8130(ctx, base);
	// 83178998: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317899C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831789A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831789A4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831789A8: 4BFF9DA1  bl 0x83172748
	ctx.lr = 0x831789AC;
	sub_83172748(ctx, base);
	// 831789AC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831789B0: 419A0018  beq cr6, 0x831789c8
	if ctx.cr[6].eq {
	pc = 0x831789C8; continue 'dispatch;
	}
	// 831789B4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831789B8: 386BAA74  addi r3, r11, -0x558c
	ctx.r[3].s64 = ctx.r[11].s64 + -21900;
	// 831789BC: 48004785  bl 0x8317d140
	ctx.lr = 0x831789C0;
	sub_8317D140(ctx, base);
	// 831789C0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 831789C4: 4802F7F4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 831789C8: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 831789CC: 839F0054  lwz r28, 0x54(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 831789D0: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 831789D4: 80BF0438  lwz r5, 0x438(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 831789D8: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 831789DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 831789E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831789E4: 4BFBEE4D  bl 0x83137830
	ctx.lr = 0x831789E8;
	sub_83137830(ctx, base);
	// 831789E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831789EC: 409A002C  bne cr6, 0x83178a18
	if !ctx.cr[6].eq {
	pc = 0x83178A18; continue 'dispatch;
	}
	// 831789F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831789F4: 4BFBED8D  bl 0x83137780
	ctx.lr = 0x831789F8;
	sub_83137780(ctx, base);
	// 831789F8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 831789FC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83178A00: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83178A04: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83178A08: 80A10058  lwz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83178A0C: 4BFBCE4D  bl 0x83135858
	ctx.lr = 0x83178A10;
	sub_83135858(ctx, base);
	// 83178A10: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83178A14: 4802F7A4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83178A18: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83178A1C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83178A20: 386BAA40  addi r3, r11, -0x55c0
	ctx.r[3].s64 = ctx.r[11].s64 + -21952;
	// 83178A24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83178A28: 48004719  bl 0x8317d140
	ctx.lr = 0x83178A2C;
	sub_8317D140(ctx, base);
	// 83178A2C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 83178A30: 4802F788  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83178A38 size=92
    let mut pc: u32 = 0x83178A38;
    'dispatch: loop {
        match pc {
            0x83178A38 => {
    //   block [0x83178A38..0x83178A94)
	// 83178A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83178A3C: 4802F72D  bl 0x831a8168
	ctx.lr = 0x83178A40;
	sub_831A8130(ctx, base);
	// 83178A40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83178A44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83178A48: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83178A4C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83178A50: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 83178A54: 4BFF9CF5  bl 0x83172748
	ctx.lr = 0x83178A58;
	sub_83172748(ctx, base);
	// 83178A58: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83178A5C: 419A0018  beq cr6, 0x83178a74
	if ctx.cr[6].eq {
	pc = 0x83178A74; continue 'dispatch;
	}
	// 83178A60: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83178A64: 386BAAA0  addi r3, r11, -0x5560
	ctx.r[3].s64 = ctx.r[11].s64 + -21856;
	// 83178A68: 480046D9  bl 0x8317d140
	ctx.lr = 0x83178A6C;
	sub_8317D140(ctx, base);
	// 83178A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83178A70: 4802F748  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83178A74: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 83178A78: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83178A7C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 83178A80: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83178A84: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83178A88: 4BFBCDD1  bl 0x83135858
	ctx.lr = 0x83178A8C;
	sub_83135858(ctx, base);
	// 83178A8C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83178A90: 4802F728  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83178A98 size=44
    let mut pc: u32 = 0x83178A98;
    'dispatch: loop {
        match pc {
            0x83178A98 => {
    //   block [0x83178A98..0x83178AC4)
	// 83178A98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83178A9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83178AA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83178AA4: 4BFBCF35  bl 0x831359d8
	ctx.lr = 0x83178AA8;
	sub_831359D8(ctx, base);
	// 83178AA8: 3963FFFD  addi r11, r3, -3
	ctx.r[11].s64 = ctx.r[3].s64 + -3;
	// 83178AAC: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 83178AB0: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 83178AB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83178AB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83178ABC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83178AC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83178AC8 size=12
    let mut pc: u32 = 0x83178AC8;
    'dispatch: loop {
        match pc {
            0x83178AC8 => {
    //   block [0x83178AC8..0x83178AD4)
	// 83178AC8: 80630054  lwz r3, 0x54(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 83178ACC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83178AD0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178AD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83178AD4 size=8
    let mut pc: u32 = 0x83178AD4;
    'dispatch: loop {
        match pc {
            0x83178AD4 => {
    //   block [0x83178AD4..0x83178ADC)
	// 83178AD4: 4BFBCFB4  b 0x83135a88
	sub_83135A88(ctx, base);
	return;
	// 83178AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83178AE0 size=12
    let mut pc: u32 = 0x83178AE0;
    'dispatch: loop {
        match pc {
            0x83178AE0 => {
    //   block [0x83178AE0..0x83178AEC)
	// 83178AE0: 80630054  lwz r3, 0x54(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 83178AE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83178AE8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178AEC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83178AEC size=8
    let mut pc: u32 = 0x83178AEC;
    'dispatch: loop {
        match pc {
            0x83178AEC => {
    //   block [0x83178AEC..0x83178AF4)
	// 83178AEC: 4BFBCE1C  b 0x83135908
	sub_83135908(ctx, base);
	return;
	// 83178AF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83178AF8 size=184
    let mut pc: u32 = 0x83178AF8;
    'dispatch: loop {
        match pc {
            0x83178AF8 => {
    //   block [0x83178AF8..0x83178BB0)
	// 83178AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83178AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83178B00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83178B04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83178B08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83178B0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83178B10: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83178B14: 4BFF9C35  bl 0x83172748
	ctx.lr = 0x83178B18;
	sub_83172748(ctx, base);
	// 83178B18: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83178B1C: 419A0014  beq cr6, 0x83178b30
	if ctx.cr[6].eq {
	pc = 0x83178B30; continue 'dispatch;
	}
	// 83178B20: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83178B24: 386BAB00  addi r3, r11, -0x5500
	ctx.r[3].s64 = ctx.r[11].s64 + -21760;
	// 83178B28: 48004619  bl 0x8317d140
	ctx.lr = 0x83178B2C;
	sub_8317D140(ctx, base);
	// 83178B2C: 4800006C  b 0x83178b98
	pc = 0x83178B98; continue 'dispatch;
	// 83178B30: 897F0080  lbz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 83178B34: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83178B38: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 83178B3C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83178B40: 409A0014  bne cr6, 0x83178b54
	if !ctx.cr[6].eq {
	pc = 0x83178B54; continue 'dispatch;
	}
	// 83178B44: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 83178B48: 409A000C  bne cr6, 0x83178b54
	if !ctx.cr[6].eq {
	pc = 0x83178B54; continue 'dispatch;
	}
	// 83178B4C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 83178B50: 995F0081  stb r10, 0x81(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(129 as u32), ctx.r[10].u8 ) };
	// 83178B54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83178B58: 409A003C  bne cr6, 0x83178b94
	if !ctx.cr[6].eq {
	pc = 0x83178B94; continue 'dispatch;
	}
	// 83178B5C: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 83178B60: 409A0034  bne cr6, 0x83178b94
	if !ctx.cr[6].eq {
	pc = 0x83178B94; continue 'dispatch;
	}
	// 83178B64: 48012E1D  bl 0x8318b980
	ctx.lr = 0x83178B68;
	sub_8318B980(ctx, base);
	// 83178B68: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83178B6C: 419A0010  beq cr6, 0x83178b7c
	if ctx.cr[6].eq {
	pc = 0x83178B7C; continue 'dispatch;
	}
	// 83178B70: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83178B74: 386BAAD4  addi r3, r11, -0x552c
	ctx.r[3].s64 = ctx.r[11].s64 + -21804;
	// 83178B78: 480045C9  bl 0x8317d140
	ctx.lr = 0x83178B7C;
	sub_8317D140(ctx, base);
	// 83178B7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83178B80: 387F04A0  addi r3, r31, 0x4a0
	ctx.r[3].s64 = ctx.r[31].s64 + 1184;
	// 83178B84: 4BFFA555  bl 0x831730d8
	ctx.lr = 0x83178B88;
	sub_831730D8(ctx, base);
	// 83178B88: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83178B8C: 387F04C4  addi r3, r31, 0x4c4
	ctx.r[3].s64 = ctx.r[31].s64 + 1220;
	// 83178B90: 4BFFA549  bl 0x831730d8
	ctx.lr = 0x83178B94;
	sub_831730D8(ctx, base);
	// 83178B94: 9BDF0080  stb r30, 0x80(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 83178B98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83178B9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83178BA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83178BA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83178BA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83178BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x83178BB0 size=164
    let mut pc: u32 = 0x83178BB0;
    'dispatch: loop {
        match pc {
            0x83178BB0 => {
    //   block [0x83178BB0..0x83178C54)
	// 83178BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83178BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83178BB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83178BBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83178BC0: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 83178BC4: 38800036  li r4, 0x36
	ctx.r[4].s64 = 54;
	// 83178BC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83178BCC: 4BFF9B15  bl 0x831726e0
	ctx.lr = 0x83178BD0;
	sub_831726E0(ctx, base);
	// 83178BD0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83178BD4: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83178BD8: 48003291  bl 0x8317be68
	ctx.lr = 0x83178BDC;
	sub_8317BE68(ctx, base);
	// 83178BDC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83178BE0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83178BE4: 41990030  bgt cr6, 0x83178c14
	if ctx.cr[6].gt {
	pc = 0x83178C14; continue 'dispatch;
	}
	// 83178BE8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83178BEC: 386BAB2C  addi r3, r11, -0x54d4
	ctx.r[3].s64 = ctx.r[11].s64 + -21716;
	// 83178BF0: 48004551  bl 0x8317d140
	ctx.lr = 0x83178BF4;
	sub_8317D140(ctx, base);
	// 83178BF4: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83178BF8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 83178BFC: 917F0520  stw r11, 0x520(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1312 as u32), ctx.r[11].u32 ) };
	// 83178C00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83178C04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83178C08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83178C0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83178C10: 4E800020  blr
	return;
	// 83178C14: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178C18: 4BFFF709  bl 0x83178320
	ctx.lr = 0x83178C1C;
	sub_83178320(ctx, base);
	// 83178C1C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83178C20: 1D6303E8  mulli r11, r3, 0x3e8
	ctx.r[11].s64 = ctx.r[3].s64 * 1000;
	// 83178C24: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 83178C28: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83178C2C: 1D6B0BB8  mulli r11, r11, 0xbb8
	ctx.r[11].s64 = ctx.r[11].s64 * 3000;
	// 83178C30: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 83178C34: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 83178C38: 7C6B5050  subf r3, r11, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 83178C3C: 907F0520  stw r3, 0x520(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1312 as u32), ctx.r[3].u32 ) };
	// 83178C40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83178C44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83178C48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83178C4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83178C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83178C58 size=220
    let mut pc: u32 = 0x83178C58;
    'dispatch: loop {
        match pc {
            0x83178C58 => {
    //   block [0x83178C58..0x83178D34)
	// 83178C58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83178C5C: 4802F511  bl 0x831a816c
	ctx.lr = 0x83178C60;
	sub_831A8130(ctx, base);
	// 83178C60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83178C64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83178C68: 4BFF9AE1  bl 0x83172748
	ctx.lr = 0x83178C6C;
	sub_83172748(ctx, base);
	// 83178C6C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83178C70: 419A0018  beq cr6, 0x83178c88
	if ctx.cr[6].eq {
	pc = 0x83178C88; continue 'dispatch;
	}
	// 83178C74: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83178C78: 386BAB58  addi r3, r11, -0x54a8
	ctx.r[3].s64 = ctx.r[11].s64 + -21672;
	// 83178C7C: 480044C5  bl 0x8317d140
	ctx.lr = 0x83178C80;
	sub_8317D140(ctx, base);
	// 83178C80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83178C84: 4802F538  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83178C88: 3FA08345  lis r29, -0x7cbb
	ctx.r[29].s64 = -2092630016;
	// 83178C8C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83178C90: 3BCBC988  addi r30, r11, -0x3678
	ctx.r[30].s64 = ctx.r[11].s64 + -13944;
	// 83178C94: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83178C98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83178C9C: 419A001C  beq cr6, 0x83178cb8
	if ctx.cr[6].eq {
	pc = 0x83178CB8; continue 'dispatch;
	}
	// 83178CA0: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83178CA4: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83178CA8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83178CAC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83178CB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83178CB4: 4E800421  bctrl
	ctx.lr = 0x83178CB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83178CB8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83178CBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178CC0: 4BFFFE39  bl 0x83178af8
	ctx.lr = 0x83178CC4;
	sub_83178AF8(ctx, base);
	// 83178CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178CC8: 809F0454  lwz r4, 0x454(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1108 as u32) ) } as u64;
	// 83178CCC: 4BFFDCF5  bl 0x831769c0
	ctx.lr = 0x83178CD0;
	sub_831769C0(ctx, base);
	// 83178CD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178CD4: 4BFFD835  bl 0x83176508
	ctx.lr = 0x83178CD8;
	sub_83176508(ctx, base);
	// 83178CD8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83178CDC: 4BFBCFD5  bl 0x83135cb0
	ctx.lr = 0x83178CE0;
	sub_83135CB0(ctx, base);
	// 83178CE0: 807F0450  lwz r3, 0x450(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1104 as u32) ) } as u64;
	// 83178CE4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83178CE8: 419A0014  beq cr6, 0x83178cfc
	if ctx.cr[6].eq {
	pc = 0x83178CFC; continue 'dispatch;
	}
	// 83178CEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83178CF0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 83178CF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83178CF8: 4E800421  bctrl
	ctx.lr = 0x83178CFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83178CFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178D00: 4BFFB329  bl 0x83174028
	ctx.lr = 0x83178D04;
	sub_83174028(ctx, base);
	// 83178D04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83178D08: 917F04F8  stw r11, 0x4f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1272 as u32), ctx.r[11].u32 ) };
	// 83178D0C: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83178D10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83178D14: 419A0018  beq cr6, 0x83178d2c
	if ctx.cr[6].eq {
	pc = 0x83178D2C; continue 'dispatch;
	}
	// 83178D18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83178D1C: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 83178D20: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83178D24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83178D28: 4E800421  bctrl
	ctx.lr = 0x83178D2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83178D2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83178D30: 4802F48C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83178D38 size=252
    let mut pc: u32 = 0x83178D38;
    'dispatch: loop {
        match pc {
            0x83178D38 => {
    //   block [0x83178D38..0x83178E34)
	// 83178D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83178D3C: 4802F42D  bl 0x831a8168
	ctx.lr = 0x83178D40;
	sub_831A8130(ctx, base);
	// 83178D40: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83178D44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83178D48: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83178D4C: 4BFF99FD  bl 0x83172748
	ctx.lr = 0x83178D50;
	sub_83172748(ctx, base);
	// 83178D50: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83178D54: 419A0018  beq cr6, 0x83178d6c
	if ctx.cr[6].eq {
	pc = 0x83178D6C; continue 'dispatch;
	}
	// 83178D58: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83178D5C: 386BABB4  addi r3, r11, -0x544c
	ctx.r[3].s64 = ctx.r[11].s64 + -21580;
	// 83178D60: 480043E1  bl 0x8317d140
	ctx.lr = 0x83178D64;
	sub_8317D140(ctx, base);
	// 83178D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83178D68: 4802F450  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83178D6C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83178D70: 409A0018  bne cr6, 0x83178d88
	if !ctx.cr[6].eq {
	pc = 0x83178D88; continue 'dispatch;
	}
	// 83178D74: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83178D78: 386BAB88  addi r3, r11, -0x5478
	ctx.r[3].s64 = ctx.r[11].s64 + -21624;
	// 83178D7C: 480043C5  bl 0x8317d140
	ctx.lr = 0x83178D80;
	sub_8317D140(ctx, base);
	// 83178D80: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83178D84: 4802F434  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 83178D88: 3F808345  lis r28, -0x7cbb
	ctx.r[28].s64 = -2092630016;
	// 83178D8C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83178D90: 3BCBC550  addi r30, r11, -0x3ab0
	ctx.r[30].s64 = ctx.r[11].s64 + -15024;
	// 83178D94: 807CA374  lwz r3, -0x5c8c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83178D98: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83178D9C: 419A0020  beq cr6, 0x83178dbc
	if ctx.cr[6].eq {
	pc = 0x83178DBC; continue 'dispatch;
	}
	// 83178DA0: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83178DA4: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83178DA8: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83178DAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83178DB0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83178DB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83178DB8: 4E800421  bctrl
	ctx.lr = 0x83178DBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83178DBC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83178DC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178DC4: 4BFFD7DD  bl 0x831765a0
	ctx.lr = 0x83178DC8;
	sub_831765A0(ctx, base);
	// 83178DC8: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83178DCC: 4BFBCDF5  bl 0x83135bc0
	ctx.lr = 0x83178DD0;
	sub_83135BC0(ctx, base);
	// 83178DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178DD4: 809F0438  lwz r4, 0x438(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 83178DD8: 4BFFFA31  bl 0x83178808
	ctx.lr = 0x83178DDC;
	sub_83178808(ctx, base);
	// 83178DDC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83178DE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178DE4: 4BFFFB0D  bl 0x831788f0
	ctx.lr = 0x83178DE8;
	sub_831788F0(ctx, base);
	// 83178DE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178DEC: 4BFFFE6D  bl 0x83178c58
	ctx.lr = 0x83178DF0;
	sub_83178C58(ctx, base);
	// 83178DF0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83178DF4: 815F0504  lwz r10, 0x504(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1284 as u32) ) } as u64;
	// 83178DF8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 83178DFC: 917F0508  stw r11, 0x508(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1288 as u32), ctx.r[11].u32 ) };
	// 83178E00: 917F0500  stw r11, 0x500(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1280 as u32), ctx.r[11].u32 ) };
	// 83178E04: 409A0008  bne cr6, 0x83178e0c
	if !ctx.cr[6].eq {
	pc = 0x83178E0C; continue 'dispatch;
	}
	// 83178E08: 917F050C  stw r11, 0x50c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1292 as u32), ctx.r[11].u32 ) };
	// 83178E0C: 807CA374  lwz r3, -0x5c8c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83178E10: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83178E14: 419A0018  beq cr6, 0x83178e2c
	if ctx.cr[6].eq {
	pc = 0x83178E2C; continue 'dispatch;
	}
	// 83178E18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83178E1C: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 83178E20: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83178E24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83178E28: 4E800421  bctrl
	ctx.lr = 0x83178E2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83178E2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83178E30: 4802F388  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83178E38 size=148
    let mut pc: u32 = 0x83178E38;
    'dispatch: loop {
        match pc {
            0x83178E38 => {
    //   block [0x83178E38..0x83178ECC)
	// 83178E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83178E3C: 4802F331  bl 0x831a816c
	ctx.lr = 0x83178E40;
	sub_831A8130(ctx, base);
	// 83178E40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83178E44: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83178E48: 4BFF9901  bl 0x83172748
	ctx.lr = 0x83178E4C;
	sub_83172748(ctx, base);
	// 83178E4C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83178E50: 419A0018  beq cr6, 0x83178e68
	if ctx.cr[6].eq {
	pc = 0x83178E68; continue 'dispatch;
	}
	// 83178E54: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83178E58: 386BABE4  addi r3, r11, -0x541c
	ctx.r[3].s64 = ctx.r[11].s64 + -21532;
	// 83178E5C: 480042E5  bl 0x8317d140
	ctx.lr = 0x83178E60;
	sub_8317D140(ctx, base);
	// 83178E60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83178E64: 4802F358  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83178E68: 3FC08345  lis r30, -0x7cbb
	ctx.r[30].s64 = -2092630016;
	// 83178E6C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83178E70: 3BEBCA60  addi r31, r11, -0x35a0
	ctx.r[31].s64 = ctx.r[11].s64 + -13728;
	// 83178E74: 807EA374  lwz r3, -0x5c8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83178E78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83178E7C: 419A001C  beq cr6, 0x83178e98
	if ctx.cr[6].eq {
	pc = 0x83178E98; continue 'dispatch;
	}
	// 83178E80: 93BF000C  stw r29, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[29].u32 ) };
	// 83178E84: 389F0004  addi r4, r31, 4
	ctx.r[4].s64 = ctx.r[31].s64 + 4;
	// 83178E88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83178E8C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83178E90: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83178E94: 4E800421  bctrl
	ctx.lr = 0x83178E98;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83178E98: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83178E9C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83178EA0: 4BFFFC59  bl 0x83178af8
	ctx.lr = 0x83178EA4;
	sub_83178AF8(ctx, base);
	// 83178EA4: 807EA374  lwz r3, -0x5c8c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83178EA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83178EAC: 419A0018  beq cr6, 0x83178ec4
	if ctx.cr[6].eq {
	pc = 0x83178EC4; continue 'dispatch;
	}
	// 83178EB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83178EB4: 389F006C  addi r4, r31, 0x6c
	ctx.r[4].s64 = ctx.r[31].s64 + 108;
	// 83178EB8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83178EBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83178EC0: 4E800421  bctrl
	ctx.lr = 0x83178EC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83178EC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83178EC8: 4802F2F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178ED0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83178ED0 size=252
    let mut pc: u32 = 0x83178ED0;
    'dispatch: loop {
        match pc {
            0x83178ED0 => {
    //   block [0x83178ED0..0x83178FCC)
	// 83178ED0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83178ED4: 4802F299  bl 0x831a816c
	ctx.lr = 0x83178ED8;
	sub_831A8130(ctx, base);
	// 83178ED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83178EDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83178EE0: 9081008C  stw r4, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[4].u32 ) };
	// 83178EE4: 90A10094  stw r5, 0x94(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(148 as u32), ctx.r[5].u32 ) };
	// 83178EE8: 4BFF9861  bl 0x83172748
	ctx.lr = 0x83178EEC;
	sub_83172748(ctx, base);
	// 83178EEC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83178EF0: 419A0018  beq cr6, 0x83178f08
	if ctx.cr[6].eq {
	pc = 0x83178F08; continue 'dispatch;
	}
	// 83178EF4: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83178EF8: 386BAC18  addi r3, r11, -0x53e8
	ctx.r[3].s64 = ctx.r[11].s64 + -21480;
	// 83178EFC: 48004245  bl 0x8317d140
	ctx.lr = 0x83178F00;
	sub_8317D140(ctx, base);
	// 83178F00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83178F04: 4802F2B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83178F08: 3FA08345  lis r29, -0x7cbb
	ctx.r[29].s64 = -2092630016;
	// 83178F0C: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83178F10: 3BCBC700  addi r30, r11, -0x3900
	ctx.r[30].s64 = ctx.r[11].s64 + -14592;
	// 83178F14: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83178F18: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83178F1C: 419A002C  beq cr6, 0x83178f48
	if ctx.cr[6].eq {
	pc = 0x83178F48; continue 'dispatch;
	}
	// 83178F20: 3961008C  addi r11, r1, 0x8c
	ctx.r[11].s64 = ctx.r[1].s64 + 140;
	// 83178F24: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 83178F28: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83178F2C: 917E0018  stw r11, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83178F30: 39610094  addi r11, r1, 0x94
	ctx.r[11].s64 = ctx.r[1].s64 + 148;
	// 83178F34: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83178F38: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83178F3C: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83178F40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83178F44: 4E800421  bctrl
	ctx.lr = 0x83178F48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83178F48: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83178F4C: 4BFBCC75  bl 0x83135bc0
	ctx.lr = 0x83178F50;
	sub_83135BC0(ctx, base);
	// 83178F50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178F54: 80A10094  lwz r5, 0x94(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 83178F58: 8081008C  lwz r4, 0x8c(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83178F5C: 4BFFFA35  bl 0x83178990
	ctx.lr = 0x83178F60;
	sub_83178990(ctx, base);
	// 83178F60: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83178F64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178F68: 4BFFF989  bl 0x831788f0
	ctx.lr = 0x83178F6C;
	sub_831788F0(ctx, base);
	// 83178F6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83178F70: 4BFFFCE9  bl 0x83178c58
	ctx.lr = 0x83178F74;
	sub_83178C58(ctx, base);
	// 83178F74: 8121008C  lwz r9, 0x8c(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 83178F78: 80E10094  lwz r7, 0x94(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 83178F7C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 83178F80: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83178F84: 811F0504  lwz r8, 0x504(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1284 as u32) ) } as u64;
	// 83178F88: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 83178F8C: 913F0510  stw r9, 0x510(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1296 as u32), ctx.r[9].u32 ) };
	// 83178F90: 915F0508  stw r10, 0x508(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1288 as u32), ctx.r[10].u32 ) };
	// 83178F94: 90FF0514  stw r7, 0x514(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1300 as u32), ctx.r[7].u32 ) };
	// 83178F98: 917F0500  stw r11, 0x500(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1280 as u32), ctx.r[11].u32 ) };
	// 83178F9C: 409A0008  bne cr6, 0x83178fa4
	if !ctx.cr[6].eq {
	pc = 0x83178FA4; continue 'dispatch;
	}
	// 83178FA0: 917F050C  stw r11, 0x50c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1292 as u32), ctx.r[11].u32 ) };
	// 83178FA4: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83178FA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83178FAC: 419A0018  beq cr6, 0x83178fc4
	if ctx.cr[6].eq {
	pc = 0x83178FC4; continue 'dispatch;
	}
	// 83178FB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83178FB4: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 83178FB8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83178FBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83178FC0: 4E800421  bctrl
	ctx.lr = 0x83178FC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83178FC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83178FC8: 4802F1F4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83178FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83178FD0 size=276
    let mut pc: u32 = 0x83178FD0;
    'dispatch: loop {
        match pc {
            0x83178FD0 => {
    //   block [0x83178FD0..0x831790E4)
	// 83178FD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83178FD4: 4802F195  bl 0x831a8168
	ctx.lr = 0x83178FD8;
	sub_831A8130(ctx, base);
	// 83178FD8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83178FDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83178FE0: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 83178FE4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83178FE8: 90C100AC  stw r6, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[6].u32 ) };
	// 83178FEC: 4BFF975D  bl 0x83172748
	ctx.lr = 0x83178FF0;
	sub_83172748(ctx, base);
	// 83178FF0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83178FF4: 419A0018  beq cr6, 0x8317900c
	if ctx.cr[6].eq {
	pc = 0x8317900C; continue 'dispatch;
	}
	// 83178FF8: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 83178FFC: 386BAC48  addi r3, r11, -0x53b8
	ctx.r[3].s64 = ctx.r[11].s64 + -21432;
	// 83179000: 48004141  bl 0x8317d140
	ctx.lr = 0x83179004;
	sub_8317D140(ctx, base);
	// 83179004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 83179008: 4802F1B0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8317900C: 3F808345  lis r28, -0x7cbb
	ctx.r[28].s64 = -2092630016;
	// 83179010: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 83179014: 3BCBC7D8  addi r30, r11, -0x3828
	ctx.r[30].s64 = ctx.r[11].s64 + -14376;
	// 83179018: 807CA374  lwz r3, -0x5c8c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 8317901C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83179020: 419A0030  beq cr6, 0x83179050
	if ctx.cr[6].eq {
	pc = 0x83179050; continue 'dispatch;
	}
	// 83179024: 396100A4  addi r11, r1, 0xa4
	ctx.r[11].s64 = ctx.r[1].s64 + 164;
	// 83179028: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 8317902C: 93BE0018  stw r29, 0x18(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 83179030: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 83179034: 917E0024  stw r11, 0x24(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83179038: 396100AC  addi r11, r1, 0xac
	ctx.r[11].s64 = ctx.r[1].s64 + 172;
	// 8317903C: 917E0030  stw r11, 0x30(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 83179040: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 83179044: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83179048: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317904C: 4E800421  bctrl
	ctx.lr = 0x83179050;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83179050: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83179054: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179058: 4BFFD549  bl 0x831765a0
	ctx.lr = 0x8317905C;
	sub_831765A0(ctx, base);
	// 8317905C: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 83179060: 4BFBCB61  bl 0x83135bc0
	ctx.lr = 0x83179064;
	sub_83135BC0(ctx, base);
	// 83179064: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 83179068: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317906C: 80C100AC  lwz r6, 0xac(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 83179070: 80A100A4  lwz r5, 0xa4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 83179074: 4BFFF9C5  bl 0x83178a38
	ctx.lr = 0x83179078;
	sub_83178A38(ctx, base);
	// 83179078: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8317907C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179080: 4BFFF871  bl 0x831788f0
	ctx.lr = 0x83179084;
	sub_831788F0(ctx, base);
	// 83179084: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179088: 4BFFFBD1  bl 0x83178c58
	ctx.lr = 0x8317908C;
	sub_83178C58(ctx, base);
	// 8317908C: 812100A4  lwz r9, 0xa4(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(164 as u32) ) } as u64;
	// 83179090: 80E100AC  lwz r7, 0xac(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(172 as u32) ) } as u64;
	// 83179094: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 83179098: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317909C: 811F0504  lwz r8, 0x504(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1284 as u32) ) } as u64;
	// 831790A0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 831790A4: 913F0518  stw r9, 0x518(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1304 as u32), ctx.r[9].u32 ) };
	// 831790A8: 915F0508  stw r10, 0x508(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1288 as u32), ctx.r[10].u32 ) };
	// 831790AC: 90FF051C  stw r7, 0x51c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1308 as u32), ctx.r[7].u32 ) };
	// 831790B0: 917F0500  stw r11, 0x500(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1280 as u32), ctx.r[11].u32 ) };
	// 831790B4: 409A0008  bne cr6, 0x831790bc
	if !ctx.cr[6].eq {
	pc = 0x831790BC; continue 'dispatch;
	}
	// 831790B8: 917F050C  stw r11, 0x50c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1292 as u32), ctx.r[11].u32 ) };
	// 831790BC: 807CA374  lwz r3, -0x5c8c(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 831790C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831790C4: 419A0018  beq cr6, 0x831790dc
	if ctx.cr[6].eq {
	pc = 0x831790DC; continue 'dispatch;
	}
	// 831790C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831790CC: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 831790D0: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 831790D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831790D8: 4E800421  bctrl
	ctx.lr = 0x831790DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831790DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831790E0: 4802F0D8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831790E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831790E8 size=164
    let mut pc: u32 = 0x831790E8;
    'dispatch: loop {
        match pc {
            0x831790E8 => {
    //   block [0x831790E8..0x8317918C)
	// 831790E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831790EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831790F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831790F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831790F8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 831790FC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83179100: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83179104: 4BFF98C5  bl 0x831729c8
	ctx.lr = 0x83179108;
	sub_831729C8(ctx, base);
	// 83179108: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8317910C: 817F0508  lwz r11, 0x508(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1288 as u32) ) } as u64;
	// 83179110: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 83179114: 915F0504  stw r10, 0x504(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1284 as u32), ctx.r[10].u32 ) };
	// 83179118: 419A0040  beq cr6, 0x83179158
	if ctx.cr[6].eq {
	pc = 0x83179158; continue 'dispatch;
	}
	// 8317911C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83179120: 419A0024  beq cr6, 0x83179144
	if ctx.cr[6].eq {
	pc = 0x83179144; continue 'dispatch;
	}
	// 83179124: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83179128: 409A003C  bne cr6, 0x83179164
	if !ctx.cr[6].eq {
	pc = 0x83179164; continue 'dispatch;
	}
	// 8317912C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179130: 80DF051C  lwz r6, 0x51c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1308 as u32) ) } as u64;
	// 83179134: 80BF0518  lwz r5, 0x518(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1304 as u32) ) } as u64;
	// 83179138: 809F0438  lwz r4, 0x438(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 8317913C: 4BFFFE95  bl 0x83178fd0
	ctx.lr = 0x83179140;
	sub_83178FD0(ctx, base);
	// 83179140: 48000024  b 0x83179164
	pc = 0x83179164; continue 'dispatch;
	// 83179144: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179148: 80BF0514  lwz r5, 0x514(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1300 as u32) ) } as u64;
	// 8317914C: 809F0510  lwz r4, 0x510(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1296 as u32) ) } as u64;
	// 83179150: 4BFFFD81  bl 0x83178ed0
	ctx.lr = 0x83179154;
	sub_83178ED0(ctx, base);
	// 83179154: 48000010  b 0x83179164
	pc = 0x83179164; continue 'dispatch;
	// 83179158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317915C: 809F0438  lwz r4, 0x438(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 83179160: 4BFFFBD9  bl 0x83178d38
	ctx.lr = 0x83179164;
	sub_83178D38(ctx, base);
	// 83179164: 817F0524  lwz r11, 0x524(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1316 as u32) ) } as u64;
	// 83179168: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8317916C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 83179170: 915F0504  stw r10, 0x504(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1284 as u32), ctx.r[10].u32 ) };
	// 83179174: 917F0524  stw r11, 0x524(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1316 as u32), ctx.r[11].u32 ) };
	// 83179178: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317917C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83179180: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83179184: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83179188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179190 size=164
    let mut pc: u32 = 0x83179190;
    'dispatch: loop {
        match pc {
            0x83179190 => {
    //   block [0x83179190..0x83179234)
	// 83179190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179194: 4802EFD9  bl 0x831a816c
	ctx.lr = 0x83179198;
	sub_831A8130(ctx, base);
	// 83179198: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317919C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831791A0: 4BFF95A9  bl 0x83172748
	ctx.lr = 0x831791A4;
	sub_83172748(ctx, base);
	// 831791A4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 831791A8: 419A0018  beq cr6, 0x831791c0
	if ctx.cr[6].eq {
	pc = 0x831791C0; continue 'dispatch;
	}
	// 831791AC: 3D60821A  lis r11, -0x7de6
	ctx.r[11].s64 = -2112225280;
	// 831791B0: 386BAC7C  addi r3, r11, -0x5384
	ctx.r[3].s64 = ctx.r[11].s64 + -21380;
	// 831791B4: 48003F8D  bl 0x8317d140
	ctx.lr = 0x831791B8;
	sub_8317D140(ctx, base);
	// 831791B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831791BC: 4802F000  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 831791C0: 3FA08345  lis r29, -0x7cbb
	ctx.r[29].s64 = -2092630016;
	// 831791C4: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 831791C8: 3BCBC628  addi r30, r11, -0x39d8
	ctx.r[30].s64 = ctx.r[11].s64 + -14808;
	// 831791CC: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 831791D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 831791D4: 419A001C  beq cr6, 0x831791f0
	if ctx.cr[6].eq {
	pc = 0x831791F0; continue 'dispatch;
	}
	// 831791D8: 93FE000C  stw r31, 0xc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 831791DC: 389E0004  addi r4, r30, 4
	ctx.r[4].s64 = ctx.r[30].s64 + 4;
	// 831791E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831791E4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 831791E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831791EC: 4E800421  bctrl
	ctx.lr = 0x831791F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831791F0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 831791F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831791F8: 4BFFF6F9  bl 0x831788f0
	ctx.lr = 0x831791FC;
	sub_831788F0(ctx, base);
	// 831791FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179200: 4BFFFC39  bl 0x83178e38
	ctx.lr = 0x83179204;
	sub_83178E38(ctx, base);
	// 83179204: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 83179208: 917F050C  stw r11, 0x50c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1292 as u32), ctx.r[11].u32 ) };
	// 8317920C: 807DA374  lwz r3, -0x5c8c(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(-23692 as u32) ) } as u64;
	// 83179210: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83179214: 419A0018  beq cr6, 0x8317922c
	if ctx.cr[6].eq {
	pc = 0x8317922C; continue 'dispatch;
	}
	// 83179218: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317921C: 389E006C  addi r4, r30, 0x6c
	ctx.r[4].s64 = ctx.r[30].s64 + 108;
	// 83179220: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 83179224: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83179228: 4E800421  bctrl
	ctx.lr = 0x8317922C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317922C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83179230: 4802EF8C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179238(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179238 size=112
    let mut pc: u32 = 0x83179238;
    'dispatch: loop {
        match pc {
            0x83179238 => {
    //   block [0x83179238..0x831792A8)
	// 83179238: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317923C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83179240: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83179244: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179248: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317924C: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 83179250: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83179254: 4BFF9775  bl 0x831729c8
	ctx.lr = 0x83179258;
	sub_831729C8(ctx, base);
	// 83179258: 817F0520  lwz r11, 0x520(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1312 as u32) ) } as u64;
	// 8317925C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83179260: 394003E8  li r10, 0x3e8
	ctx.r[10].s64 = 1000;
	// 83179264: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83179268: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 8317926C: 7D284BD6  divw r9, r8, r9
	ctx.r[9].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 83179270: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 83179274: 41990020  bgt cr6, 0x83179294
	if ctx.cr[6].gt {
	pc = 0x83179294; continue 'dispatch;
	}
	// 83179278: 817F050C  lwz r11, 0x50c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1292 as u32) ) } as u64;
	// 8317927C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 83179280: 409A0014  bne cr6, 0x83179294
	if !ctx.cr[6].eq {
	pc = 0x83179294; continue 'dispatch;
	}
	// 83179284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179288: 4BFFFF09  bl 0x83179190
	ctx.lr = 0x8317928C;
	sub_83179190(ctx, base);
	// 8317928C: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 83179290: 917F050C  stw r11, 0x50c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1292 as u32), ctx.r[11].u32 ) };
	// 83179294: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83179298: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317929C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831792A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 831792A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831792A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831792A8 size=164
    let mut pc: u32 = 0x831792A8;
    'dispatch: loop {
        match pc {
            0x831792A8 => {
    //   block [0x831792A8..0x8317934C)
	// 831792A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831792AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831792B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 831792B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831792B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831792BC: 817F04FC  lwz r11, 0x4fc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1276 as u32) ) } as u64;
	// 831792C0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831792C4: 419A0074  beq cr6, 0x83179338
	if ctx.cr[6].eq {
	pc = 0x83179338; continue 'dispatch;
	}
	// 831792C8: 817F0500  lwz r11, 0x500(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1280 as u32) ) } as u64;
	// 831792CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 831792D0: 419A0068  beq cr6, 0x83179338
	if ctx.cr[6].eq {
	pc = 0x83179338; continue 'dispatch;
	}
	// 831792D4: 817F050C  lwz r11, 0x50c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1292 as u32) ) } as u64;
	// 831792D8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 831792DC: 409A0028  bne cr6, 0x83179304
	if !ctx.cr[6].eq {
	pc = 0x83179304; continue 'dispatch;
	}
	// 831792E0: 4BFF9899  bl 0x83172b78
	ctx.lr = 0x831792E4;
	sub_83172B78(ctx, base);
	// 831792E4: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 831792E8: 409A001C  bne cr6, 0x83179304
	if !ctx.cr[6].eq {
	pc = 0x83179304; continue 'dispatch;
	}
	// 831792EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 831792F0: 4BFFF8C1  bl 0x83178bb0
	ctx.lr = 0x831792F4;
	sub_83178BB0(ctx, base);
	// 831792F4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831792F8: 41980040  blt cr6, 0x83179338
	if ctx.cr[6].lt {
	pc = 0x83179338; continue 'dispatch;
	}
	// 831792FC: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 83179300: 917F050C  stw r11, 0x50c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1292 as u32), ctx.r[11].u32 ) };
	// 83179304: 817F050C  lwz r11, 0x50c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1292 as u32) ) } as u64;
	// 83179308: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8317930C: 419A000C  beq cr6, 0x83179318
	if ctx.cr[6].eq {
	pc = 0x83179318; continue 'dispatch;
	}
	// 83179310: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 83179314: 409A0024  bne cr6, 0x83179338
	if !ctx.cr[6].eq {
	pc = 0x83179338; continue 'dispatch;
	}
	// 83179318: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317931C: 4BFF9445  bl 0x83172760
	ctx.lr = 0x83179320;
	sub_83172760(ctx, base);
	// 83179320: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 83179324: 409A000C  bne cr6, 0x83179330
	if !ctx.cr[6].eq {
	pc = 0x83179330; continue 'dispatch;
	}
	// 83179328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317932C: 4BFFFDBD  bl 0x831790e8
	ctx.lr = 0x83179330;
	sub_831790E8(ctx, base);
	// 83179330: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179334: 4BFFFF05  bl 0x83179238
	ctx.lr = 0x83179338;
	sub_83179238(ctx, base);
	// 83179338: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317933C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83179340: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83179344: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83179348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179350 size=72
    let mut pc: u32 = 0x83179350;
    'dispatch: loop {
        match pc {
            0x83179350 => {
    //   block [0x83179350..0x83179398)
	// 83179350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179354: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83179358: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317935C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179360: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83179364: 4800DF7D  bl 0x831872e0
	ctx.lr = 0x83179368;
	sub_831872E0(ctx, base);
	// 83179368: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317936C: 419A0014  beq cr6, 0x83179380
	if ctx.cr[6].eq {
	pc = 0x83179380; continue 'dispatch;
	}
	// 83179370: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83179374: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83179378: 60840111  ori r4, r4, 0x111
	ctx.r[4].u64 = ctx.r[4].u64 | 273;
	// 8317937C: 4800E17D  bl 0x831874f8
	ctx.lr = 0x83179380;
	sub_831874F8(ctx, base);
	// 83179380: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 83179384: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83179388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317938C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83179390: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83179394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179398 size=116
    let mut pc: u32 = 0x83179398;
    'dispatch: loop {
        match pc {
            0x83179398 => {
    //   block [0x83179398..0x8317940C)
	// 83179398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317939C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 831793A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831793A4: 2F040006  cmpwi cr6, r4, 6
	ctx.cr[6].compare_i32(ctx.r[4].s32, 6, &mut ctx.xer);
	// 831793A8: 409A002C  bne cr6, 0x831793d4
	if !ctx.cr[6].eq {
	pc = 0x831793D4; continue 'dispatch;
	}
	// 831793AC: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 831793B0: 409A0048  bne cr6, 0x831793f8
	if !ctx.cr[6].eq {
	pc = 0x831793F8; continue 'dispatch;
	}
	// 831793B4: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 831793B8: 48012AB1  bl 0x8318be68
	ctx.lr = 0x831793BC;
	sub_8318BE68(ctx, base);
	// 831793BC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831793C0: 409A0038  bne cr6, 0x831793f8
	if !ctx.cr[6].eq {
	pc = 0x831793F8; continue 'dispatch;
	}
	// 831793C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 831793C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831793CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831793D0: 4E800020  blr
	return;
	// 831793D4: 2F040005  cmpwi cr6, r4, 5
	ctx.cr[6].compare_i32(ctx.r[4].s32, 5, &mut ctx.xer);
	// 831793D8: 409A0020  bne cr6, 0x831793f8
	if !ctx.cr[6].eq {
	pc = 0x831793F8; continue 'dispatch;
	}
	// 831793DC: 2F050001  cmpwi cr6, r5, 1
	ctx.cr[6].compare_i32(ctx.r[5].s32, 1, &mut ctx.xer);
	// 831793E0: 409A0018  bne cr6, 0x831793f8
	if !ctx.cr[6].eq {
	pc = 0x831793F8; continue 'dispatch;
	}
	// 831793E4: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 831793E8: 48012A81  bl 0x8318be68
	ctx.lr = 0x831793EC;
	sub_8318BE68(ctx, base);
	// 831793EC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831793F0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831793F4: 419A0008  beq cr6, 0x831793fc
	if ctx.cr[6].eq {
	pc = 0x831793FC; continue 'dispatch;
	}
	// 831793F8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831793FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83179400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83179404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83179408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83179410 size=16
    let mut pc: u32 = 0x83179410;
    'dispatch: loop {
        match pc {
            0x83179410 => {
    //   block [0x83179410..0x83179420)
	// 83179410: 39640283  addi r11, r4, 0x283
	ctx.r[11].s64 = ctx.r[4].s64 + 643;
	// 83179414: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83179418: 7C6B182E  lwzx r3, r11, r3
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[3].u32)) } as u64;
	// 8317941C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83179420 size=12
    let mut pc: u32 = 0x83179420;
    'dispatch: loop {
        match pc {
            0x83179420 => {
    //   block [0x83179420..0x8317942C)
	// 83179420: 90A30D30  stw r5, 0xd30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(3376 as u32), ctx.r[5].u32 ) };
	// 83179424: 90830D2C  stw r4, 0xd2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(3372 as u32), ctx.r[4].u32 ) };
	// 83179428: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83179430 size=28
    let mut pc: u32 = 0x83179430;
    'dispatch: loop {
        match pc {
            0x83179430 => {
    //   block [0x83179430..0x8317944C)
	// 83179430: 1D640044  mulli r11, r4, 0x44
	ctx.r[11].s64 = ctx.r[4].s64 * 68;
	// 83179434: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 83179438: 816B1740  lwz r11, 0x1740(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(5952 as u32) ) } as u64;
	// 8317943C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 83179440: 409A000C  bne cr6, 0x8317944c
	if !ctx.cr[6].eq {
		sub_8317944C(ctx, base);
		return;
	}
	// 83179444: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83179448: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317944C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317944C size=12
    let mut pc: u32 = 0x8317944C;
    'dispatch: loop {
        match pc {
            0x8317944C => {
    //   block [0x8317944C..0x83179458)
	// 8317944C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83179450: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83179454: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179458 size=56
    let mut pc: u32 = 0x83179458;
    'dispatch: loop {
        match pc {
            0x83179458 => {
    //   block [0x83179458..0x83179490)
	// 83179458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317945C: 4802ED11  bl 0x831a816c
	ctx.lr = 0x83179460;
	sub_831A8130(ctx, base);
	// 83179460: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179464: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83179468: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317946C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83179470: 4BFFFF29  bl 0x83179398
	ctx.lr = 0x83179474;
	sub_83179398(ctx, base);
	// 83179474: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179478: 419A0010  beq cr6, 0x83179488
	if ctx.cr[6].eq {
	pc = 0x83179488; continue 'dispatch;
	}
	// 8317947C: 397E0283  addi r11, r30, 0x283
	ctx.r[11].s64 = ctx.r[30].s64 + 643;
	// 83179480: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83179484: 7FABF92E  stwx r29, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 83179488: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317948C: 4802ED30  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179490 size=56
    let mut pc: u32 = 0x83179490;
    'dispatch: loop {
        match pc {
            0x83179490 => {
    //   block [0x83179490..0x831794C8)
	// 83179490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179494: 4802ECD9  bl 0x831a816c
	ctx.lr = 0x83179498;
	sub_831A8130(ctx, base);
	// 83179498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317949C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831794A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831794A4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831794A8: 4BFFFEF1  bl 0x83179398
	ctx.lr = 0x831794AC;
	sub_83179398(ctx, base);
	// 831794AC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831794B0: 419A0010  beq cr6, 0x831794c0
	if ctx.cr[6].eq {
	pc = 0x831794C0; continue 'dispatch;
	}
	// 831794B4: 397E02E7  addi r11, r30, 0x2e7
	ctx.r[11].s64 = ctx.r[30].s64 + 743;
	// 831794B8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 831794BC: 7FABF92E  stwx r29, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[29].u32) };
	// 831794C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831794C4: 4802ECF8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831794C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831794C8 size=128
    let mut pc: u32 = 0x831794C8;
    'dispatch: loop {
        match pc {
            0x831794C8 => {
    //   block [0x831794C8..0x83179548)
	// 831794C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831794CC: 4802ECA1  bl 0x831a816c
	ctx.lr = 0x831794D0;
	sub_831A8130(ctx, base);
	// 831794D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831794D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831794D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831794DC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831794E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 831794E4: 409A0020  bne cr6, 0x83179504
	if !ctx.cr[6].eq {
	pc = 0x83179504; continue 'dispatch;
	}
	// 831794E8: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831794EC: 57CA103A  slwi r10, r30, 2
	ctx.r[10].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831794F0: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 831794F4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 831794F8: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 831794FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83179500: 4802ECBC  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83179504: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179508: 4800DDD9  bl 0x831872e0
	ctx.lr = 0x8317950C;
	sub_831872E0(ctx, base);
	// 8317950C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179510: 419A001C  beq cr6, 0x8317952c
	if ctx.cr[6].eq {
	pc = 0x8317952C; continue 'dispatch;
	}
	// 83179514: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83179518: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317951C: 60840113  ori r4, r4, 0x113
	ctx.r[4].u64 = ctx.r[4].u64 | 275;
	// 83179520: 4800DFD9  bl 0x831874f8
	ctx.lr = 0x83179524;
	sub_831874F8(ctx, base);
	// 83179524: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83179528: 4802EC94  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317952C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83179530: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179534: 4BFFFEDD  bl 0x83179410
	ctx.lr = 0x83179538;
	sub_83179410(ctx, base);
	// 83179538: 907D0000  stw r3, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 8317953C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83179540: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83179544: 4802EC78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179548 size=88
    let mut pc: u32 = 0x83179548;
    'dispatch: loop {
        match pc {
            0x83179548 => {
    //   block [0x83179548..0x831795A0)
	// 83179548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317954C: 4802EC21  bl 0x831a816c
	ctx.lr = 0x83179550;
	sub_831A8130(ctx, base);
	// 83179550: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179554: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83179558: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317955C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 83179560: 4800DD81  bl 0x831872e0
	ctx.lr = 0x83179564;
	sub_831872E0(ctx, base);
	// 83179564: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179568: 419A001C  beq cr6, 0x83179584
	if ctx.cr[6].eq {
	pc = 0x83179584; continue 'dispatch;
	}
	// 8317956C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 83179570: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83179574: 60840117  ori r4, r4, 0x117
	ctx.r[4].u64 = ctx.r[4].u64 | 279;
	// 83179578: 4800DF81  bl 0x831874f8
	ctx.lr = 0x8317957C;
	sub_831874F8(ctx, base);
	// 8317957C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83179580: 4802EC3C  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83179584: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83179588: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317958C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179590: 4BFFFEA1  bl 0x83179430
	ctx.lr = 0x83179594;
	sub_83179430(ctx, base);
	// 83179594: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83179598: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317959C: 4802EC20  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831795A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831795A0 size=92
    let mut pc: u32 = 0x831795A0;
    'dispatch: loop {
        match pc {
            0x831795A0 => {
    //   block [0x831795A0..0x831795FC)
	// 831795A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831795A4: 4802EBC1  bl 0x831a8164
	ctx.lr = 0x831795A8;
	sub_831A8130(ctx, base);
	// 831795A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831795AC: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 831795B0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 831795B4: 3BABA100  addi r29, r11, -0x5f00
	ctx.r[29].s64 = ctx.r[11].s64 + -24320;
	// 831795B8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 831795BC: 3BFD020C  addi r31, r29, 0x20c
	ctx.r[31].s64 = ctx.r[29].s64 + 524;
	// 831795C0: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 831795C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831795C8: 4800DD19  bl 0x831872e0
	ctx.lr = 0x831795CC;
	sub_831872E0(ctx, base);
	// 831795CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831795D0: 409A0014  bne cr6, 0x831795e4
	if !ctx.cr[6].eq {
	pc = 0x831795E4; continue 'dispatch;
	}
	// 831795D4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 831795D8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 831795DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 831795E0: 4BFFFE79  bl 0x83179458
	ctx.lr = 0x831795E4;
	sub_83179458(ctx, base);
	// 831795E4: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 831795E8: 397D022C  addi r11, r29, 0x22c
	ctx.r[11].s64 = ctx.r[29].s64 + 556;
	// 831795EC: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 831795F0: 4198FFD0  blt cr6, 0x831795c0
	if ctx.cr[6].lt {
	pc = 0x831795C0; continue 'dispatch;
	}
	// 831795F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831795F8: 4802EBBC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179600(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179600 size=156
    let mut pc: u32 = 0x83179600;
    'dispatch: loop {
        match pc {
            0x83179600 => {
    //   block [0x83179600..0x8317969C)
	// 83179600: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179604: 4802EB69  bl 0x831a816c
	ctx.lr = 0x83179608;
	sub_831A8130(ctx, base);
	// 83179608: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317960C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 83179610: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 83179614: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83179618: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8317961C: 409A002C  bne cr6, 0x83179648
	if !ctx.cr[6].eq {
	pc = 0x83179648; continue 'dispatch;
	}
	// 83179620: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83179624: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179628: 4BFFFF79  bl 0x831795a0
	ctx.lr = 0x8317962C;
	sub_831795A0(ctx, base);
	// 8317962C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83179630: 57EA103A  slwi r10, r31, 2
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 83179634: 396BA100  addi r11, r11, -0x5f00
	ctx.r[11].s64 = ctx.r[11].s64 + -24320;
	// 83179638: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317963C: 7FCA592E  stwx r30, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[30].u32) };
	// 83179640: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83179644: 4802EB78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83179648: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317964C: 4800DC95  bl 0x831872e0
	ctx.lr = 0x83179650;
	sub_831872E0(ctx, base);
	// 83179650: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179654: 419A001C  beq cr6, 0x83179670
	if ctx.cr[6].eq {
	pc = 0x83179670; continue 'dispatch;
	}
	// 83179658: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317965C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83179660: 60840112  ori r4, r4, 0x112
	ctx.r[4].u64 = ctx.r[4].u64 | 274;
	// 83179664: 4800DE95  bl 0x831874f8
	ctx.lr = 0x83179668;
	sub_831874F8(ctx, base);
	// 83179668: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317966C: 4802EB50  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 83179670: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83179674: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83179678: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317967C: 4BFFFDDD  bl 0x83179458
	ctx.lr = 0x83179680;
	sub_83179458(ctx, base);
	// 83179680: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83179684: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 83179688: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317968C: 4BFFFE05  bl 0x83179490
	ctx.lr = 0x83179690;
	sub_83179490(ctx, base);
	// 83179690: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83179694: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83179698: 4802EB24  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831796A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831796A0 size=112
    let mut pc: u32 = 0x831796A0;
    'dispatch: loop {
        match pc {
            0x831796A0 => {
    //   block [0x831796A0..0x83179710)
	// 831796A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831796A4: 4802EAC1  bl 0x831a8164
	ctx.lr = 0x831796A8;
	sub_831A8130(ctx, base);
	// 831796A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 831796AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 831796B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 831796B4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 831796B8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 831796BC: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 831796C0: 4800DC21  bl 0x831872e0
	ctx.lr = 0x831796C4;
	sub_831872E0(ctx, base);
	// 831796C4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831796C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831796CC: 419A0018  beq cr6, 0x831796e4
	if ctx.cr[6].eq {
	pc = 0x831796E4; continue 'dispatch;
	}
	// 831796D0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 831796D4: 60840171  ori r4, r4, 0x171
	ctx.r[4].u64 = ctx.r[4].u64 | 369;
	// 831796D8: 4800DE21  bl 0x831874f8
	ctx.lr = 0x831796DC;
	sub_831874F8(ctx, base);
	// 831796DC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 831796E0: 4802EAD4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 831796E4: 397EFF44  addi r11, r30, -0xbc
	ctx.r[11].s64 = ctx.r[30].s64 + -188;
	// 831796E8: 2B0B0043  cmplwi cr6, r11, 0x43
	ctx.cr[6].compare_u32(ctx.r[11].u32, 67 as u32, &mut ctx.xer);
	// 831796EC: 4199001C  bgt cr6, 0x83179708
	if ctx.cr[6].gt {
	pc = 0x83179708; continue 'dispatch;
	}
	// 831796F0: 395EFF54  addi r10, r30, -0xac
	ctx.r[10].s64 = ctx.r[30].s64 + -172;
	// 831796F4: 817F1784  lwz r11, 0x1784(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6020 as u32) ) } as u64;
	// 831796F8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 831796FC: 938B0150  stw r28, 0x150(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(336 as u32), ctx.r[28].u32 ) };
	// 83179700: 936B0154  stw r27, 0x154(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(340 as u32), ctx.r[27].u32 ) };
	// 83179704: 7FAA592E  stwx r29, r10, r11
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32), ctx.r[29].u32) };
	// 83179708: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317970C: 4802EAA8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179710 size=36
    let mut pc: u32 = 0x83179710;
    'dispatch: loop {
        match pc {
            0x83179710 => {
    //   block [0x83179710..0x83179734)
	// 83179710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179714: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83179718: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317971C: 48012EFD  bl 0x8318c618
	ctx.lr = 0x83179720;
	sub_8318C618(ctx, base);
	// 83179720: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83179724: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83179728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317972C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83179730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83179738 size=20
    let mut pc: u32 = 0x83179738;
    'dispatch: loop {
        match pc {
            0x83179738 => {
    //   block [0x83179738..0x8317974C)
	// 83179738: 81631784  lwz r11, 0x1784(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6020 as u32) ) } as u64;
	// 8317973C: 80A30D38  lwz r5, 0xd38(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3384 as u32) ) } as u64;
	// 83179740: 80830D34  lwz r4, 0xd34(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3380 as u32) ) } as u64;
	// 83179744: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 83179748: 48012FC8  b 0x8318c710
	sub_8318C710(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179750 size=164
    let mut pc: u32 = 0x83179750;
    'dispatch: loop {
        match pc {
            0x83179750 => {
    //   block [0x83179750..0x831797F4)
	// 83179750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179754: 4802EA15  bl 0x831a8168
	ctx.lr = 0x83179758;
	sub_831A8130(ctx, base);
	// 83179758: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317975C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83179760: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 83179764: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 83179768: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8317976C: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 83179770: 809F178C  lwz r4, 0x178c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6028 as u32) ) } as u64;
	// 83179774: 48014F0D  bl 0x8318e680
	ctx.lr = 0x83179778;
	sub_8318E680(ctx, base);
	// 83179778: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317977C: 409A0070  bne cr6, 0x831797ec
	if !ctx.cr[6].eq {
	pc = 0x831797EC; continue 'dispatch;
	}
	// 83179780: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83179784: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83179788: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317978C: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 83179790: 913C0000  stw r9, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 83179794: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83179798: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8317979C: 80BE0000  lwz r5, 0(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 831797A0: 2F050800  cmpwi cr6, r5, 0x800
	ctx.cr[6].compare_i32(ctx.r[5].s32, 2048, &mut ctx.xer);
	// 831797A4: 41980044  blt cr6, 0x831797e8
	if ctx.cr[6].lt {
	pc = 0x831797E8; continue 'dispatch;
	}
	// 831797A8: 809C0000  lwz r4, 0(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 831797AC: 817F1FAC  lwz r11, 0x1fac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8108 as u32) ) } as u64;
	// 831797B0: 7F045840  cmplw cr6, r4, r11
	ctx.cr[6].compare_u32(ctx.r[4].u32, ctx.r[11].u32, &mut ctx.xer);
	// 831797B4: 419A0034  beq cr6, 0x831797e8
	if ctx.cr[6].eq {
	pc = 0x831797E8; continue 'dispatch;
	}
	// 831797B8: 817F1FA8  lwz r11, 0x1fa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8104 as u32) ) } as u64;
	// 831797BC: 556B057E  clrlwi r11, r11, 0x15
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x000007FFu64;
	// 831797C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831797C4: 409A0024  bne cr6, 0x831797e8
	if !ctx.cr[6].eq {
	pc = 0x831797E8; continue 'dispatch;
	}
	// 831797C8: 817F1FA0  lwz r11, 0x1fa0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8096 as u32) ) } as u64;
	// 831797CC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 831797D0: 419A0010  beq cr6, 0x831797e0
	if ctx.cr[6].eq {
	pc = 0x831797E0; continue 'dispatch;
	}
	// 831797D4: 807F1FA4  lwz r3, 0x1fa4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8100 as u32) ) } as u64;
	// 831797D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 831797DC: 4E800421  bctrl
	ctx.lr = 0x831797E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831797E0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 831797E4: 917F1FAC  stw r11, 0x1fac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8108 as u32), ctx.r[11].u32 ) };
	// 831797E8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831797EC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 831797F0: 4802E9C8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831797F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x831797F8 size=84
    let mut pc: u32 = 0x831797F8;
    'dispatch: loop {
        match pc {
            0x831797F8 => {
    //   block [0x831797F8..0x8317984C)
	// 831797F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 831797FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83179800: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83179804: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83179808: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317980C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83179810: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83179814: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83179818: 809F178C  lwz r4, 0x178c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6028 as u32) ) } as u64;
	// 8317981C: 48015175  bl 0x8318e990
	ctx.lr = 0x83179820;
	sub_8318E990(ctx, base);
	// 83179820: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179824: 409A0010  bne cr6, 0x83179834
	if !ctx.cr[6].eq {
	pc = 0x83179834; continue 'dispatch;
	}
	// 83179828: 817F1FA8  lwz r11, 0x1fa8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8104 as u32) ) } as u64;
	// 8317982C: 7D7E5A14  add r11, r30, r11
	ctx.r[11].u64 = ctx.r[30].u64 + ctx.r[11].u64;
	// 83179830: 917F1FA8  stw r11, 0x1fa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8104 as u32), ctx.r[11].u32 ) };
	// 83179834: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83179838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317983C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83179840: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83179844: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83179848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83179850 size=8
    let mut pc: u32 = 0x83179850;
    'dispatch: loop {
        match pc {
            0x83179850 => {
    //   block [0x83179850..0x83179858)
	// 83179850: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 83179854: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179858(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83179858 size=12
    let mut pc: u32 = 0x83179858;
    'dispatch: loop {
        match pc {
            0x83179858 => {
    //   block [0x83179858..0x83179864)
	// 83179858: 90831FA0  stw r4, 0x1fa0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8096 as u32), ctx.r[4].u32 ) };
	// 8317985C: 90A31FA4  stw r5, 0x1fa4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8100 as u32), ctx.r[5].u32 ) };
	// 83179860: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179868 size=72
    let mut pc: u32 = 0x83179868;
    'dispatch: loop {
        match pc {
            0x83179868 => {
    //   block [0x83179868..0x831798B0)
	// 83179868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317986C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83179870: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179874: 81630D4C  lwz r11, 0xd4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3404 as u32) ) } as u64;
	// 83179878: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317987C: 419A0024  beq cr6, 0x831798a0
	if ctx.cr[6].eq {
	pc = 0x831798A0; continue 'dispatch;
	}
	// 83179880: E9430990  ld r10, 0x990(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(2448 as u32) ) };
	// 83179884: 98810050  stb r4, 0x50(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[4].u8 ) };
	// 83179888: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8317988C: 80630D50  lwz r3, 0xd50(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(3408 as u32) ) } as u64;
	// 83179890: F8A10058  std r5, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u64 ) };
	// 83179894: F9410060  std r10, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u64 ) };
	// 83179898: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317989C: 4E800421  bctrl
	ctx.lr = 0x831798A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 831798A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 831798A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 831798A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 831798AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831798B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831798B0 size=48
    let mut pc: u32 = 0x831798B0;
    'dispatch: loop {
        match pc {
            0x831798B0 => {
    //   block [0x831798B0..0x831798E0)
	// 831798B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 831798B4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 831798B8: 40990020  ble cr6, 0x831798d8
	if !ctx.cr[6].gt {
	pc = 0x831798D8; continue 'dispatch;
	}
	// 831798BC: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 831798C0: 38630001  addi r3, r3, 1
	ctx.r[3].s64 = ctx.r[3].s64 + 1;
	// 831798C4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 831798C8: 409A0018  bne cr6, 0x831798e0
	if !ctx.cr[6].eq {
		sub_831798E0(ctx, base);
		return;
	}
	// 831798CC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 831798D0: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 831798D4: 4198FFE8  blt cr6, 0x831798bc
	if ctx.cr[6].lt {
	pc = 0x831798BC; continue 'dispatch;
	}
	// 831798D8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 831798DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831798E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831798E0 size=8
    let mut pc: u32 = 0x831798E0;
    'dispatch: loop {
        match pc {
            0x831798E0 => {
    //   block [0x831798E0..0x831798E8)
	// 831798E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831798E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_831798E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x831798E8 size=20
    let mut pc: u32 = 0x831798E8;
    'dispatch: loop {
        match pc {
            0x831798E8 => {
    //   block [0x831798E8..0x831798FC)
	// 831798E8: 8163178C  lwz r11, 0x178c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6028 as u32) ) } as u64;
	// 831798EC: 1D6B0074  mulli r11, r11, 0x74
	ctx.r[11].s64 = ctx.r[11].s64 * 116;
	// 831798F0: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 831798F4: 386B13A8  addi r3, r11, 0x13a8
	ctx.r[3].s64 = ctx.r[11].s64 + 5032;
	// 831798F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179900 size=124
    let mut pc: u32 = 0x83179900;
    'dispatch: loop {
        match pc {
            0x83179900 => {
    //   block [0x83179900..0x8317997C)
	// 83179900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83179908: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317990C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83179910: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179914: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83179918: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317991C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83179920: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83179924: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83179928: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 8317992C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83179930: 4E800421  bctrl
	ctx.lr = 0x83179934;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83179934: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 83179938: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317993C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 83179940: 48015059  bl 0x8318e998
	ctx.lr = 0x83179944;
	sub_8318E998(ctx, base);
	// 83179944: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 83179948: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317994C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83179950: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179954: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83179958: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317995C: 4E800421  bctrl
	ctx.lr = 0x83179960;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83179960: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 83179964: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83179968: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317996C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83179970: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83179974: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83179978: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179980 size=328
    let mut pc: u32 = 0x83179980;
    'dispatch: loop {
        match pc {
            0x83179980 => {
    //   block [0x83179980..0x83179AC8)
	// 83179980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179984: 4802E7D1  bl 0x831a8154
	ctx.lr = 0x83179988;
	sub_831A8130(ctx, base);
	// 83179988: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317998C: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 83179990: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 83179994: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 83179998: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8317999C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 831799A0: 7CFB3B78  mr r27, r7
	ctx.r[27].u64 = ctx.r[7].u64;
	// 831799A4: 48014CD5  bl 0x8318e678
	ctx.lr = 0x831799A8;
	sub_8318E678(ctx, base);
	// 831799A8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 831799AC: 409A0114  bne cr6, 0x83179ac0
	if !ctx.cr[6].eq {
	pc = 0x83179AC0; continue 'dispatch;
	}
	// 831799B0: 81610084  lwz r11, 0x84(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(132 as u32) ) } as u64;
	// 831799B4: 8141008C  lwz r10, 0x8c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(140 as u32) ) } as u64;
	// 831799B8: 83A10080  lwz r29, 0x80(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(128 as u32) ) } as u64;
	// 831799BC: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	// 831799C0: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 831799C4: 83410088  lwz r26, 0x88(r1)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(136 as u32) ) } as u64;
	// 831799C8: 82E10094  lwz r23, 0x94(r1)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(148 as u32) ) } as u64;
	// 831799CC: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 831799D0: 40990010  ble cr6, 0x831799e0
	if !ctx.cr[6].gt {
	pc = 0x831799E0; continue 'dispatch;
	}
	// 831799D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 831799D8: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 831799DC: 4802E7C8  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 831799E0: 2F180001  cmpwi cr6, r24, 1
	ctx.cr[6].compare_i32(ctx.r[24].s32, 1, &mut ctx.xer);
	// 831799E4: 409A0050  bne cr6, 0x83179a34
	if !ctx.cr[6].eq {
	pc = 0x83179A34; continue 'dispatch;
	}
	// 831799E8: 2F3B0000  cmpdi cr6, r27, 0
	ctx.cr[6].compare_i64(ctx.r[27].s64, 0, &mut ctx.xer);
	// 831799EC: 41980084  blt cr6, 0x83179a70
	if ctx.cr[6].lt {
	pc = 0x83179A70; continue 'dispatch;
	}
	// 831799F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 831799F4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 831799F8: 48015181  bl 0x8318eb78
	ctx.lr = 0x831799FC;
	sub_8318EB78(ctx, base);
	// 831799FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179A00: 409AFFD4  bne cr6, 0x831799d4
	if !ctx.cr[6].eq {
	pc = 0x831799D4; continue 'dispatch;
	}
	// 83179A04: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 83179A08: FB610060  std r27, 0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[27].u64 ) };
	// 83179A0C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 83179A10: 93A10068  stw r29, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[29].u32 ) };
	// 83179A14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83179A18: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 83179A1C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83179A20: 480151F1  bl 0x8318ec10
	ctx.lr = 0x83179A24;
	sub_8318EC10(ctx, base);
	// 83179A24: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179A28: 419A0048  beq cr6, 0x83179a70
	if ctx.cr[6].eq {
	pc = 0x83179A70; continue 'dispatch;
	}
	// 83179A2C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 83179A30: 4802E774  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
	// 83179A34: 2F180002  cmpwi cr6, r24, 2
	ctx.cr[6].compare_i32(ctx.r[24].s32, 2, &mut ctx.xer);
	// 83179A38: 409A0038  bne cr6, 0x83179a70
	if !ctx.cr[6].eq {
	pc = 0x83179A70; continue 'dispatch;
	}
	// 83179A3C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 83179A40: 814BA340  lwz r10, -0x5cc0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23744 as u32) ) } as u64;
	// 83179A44: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 83179A48: 419A0028  beq cr6, 0x83179a70
	if ctx.cr[6].eq {
	pc = 0x83179A70; continue 'dispatch;
	}
	// 83179A4C: FB610070  std r27, 0x70(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[27].u64 ) };
	// 83179A50: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 83179A54: 93E10078  stw r31, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[31].u32 ) };
	// 83179A58: 387C1358  addi r3, r28, 0x1358
	ctx.r[3].s64 = ctx.r[28].s64 + 4952;
	// 83179A5C: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 83179A60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 83179A64: 4E800421  bctrl
	ctx.lr = 0x83179A68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 83179A68: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 83179A6C: 419AFF68  beq cr6, 0x831799d4
	if ctx.cr[6].eq {
	pc = 0x831799D4; continue 'dispatch;
	}
	// 83179A70: 7F1FF000  cmpw cr6, r31, r30
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[30].s32, &mut ctx.xer);
	// 83179A74: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 83179A78: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 83179A7C: 4199000C  bgt cr6, 0x83179a88
	if ctx.cr[6].gt {
	pc = 0x83179A88; continue 'dispatch;
	}
	// 83179A80: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83179A84: 48000018  b 0x83179a9c
	pc = 0x83179A9C; continue 'dispatch;
	// 83179A88: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83179A8C: 48014F0D  bl 0x8318e998
	ctx.lr = 0x83179A90;
	sub_8318E998(ctx, base);
	// 83179A90: 7CBEF850  subf r5, r30, r31
	ctx.r[5].s64 = ctx.r[31].s64 - ctx.r[30].s64;
	// 83179A94: 7C9ECA14  add r4, r30, r25
	ctx.r[4].u64 = ctx.r[30].u64 + ctx.r[25].u64;
	// 83179A98: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 83179A9C: 48014EFD  bl 0x8318e998
	ctx.lr = 0x83179AA0;
	sub_8318E998(ctx, base);
	// 83179AA0: 7EE6BB78  mr r6, r23
	ctx.r[6].u64 = ctx.r[23].u64;
	// 83179AA4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 83179AA8: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 83179AAC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 83179AB0: 48014ED9  bl 0x8318e988
	ctx.lr = 0x83179AB4;
	sub_8318E988(ctx, base);
	// 83179AB4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179AB8: 409A0008  bne cr6, 0x83179ac0
	if !ctx.cr[6].eq {
	pc = 0x83179AC0; continue 'dispatch;
	}
	// 83179ABC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83179AC0: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 83179AC4: 4802E6E0  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179AC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179AC8 size=124
    let mut pc: u32 = 0x83179AC8;
    'dispatch: loop {
        match pc {
            0x83179AC8 => {
    //   block [0x83179AC8..0x83179B44)
	// 83179AC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179ACC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83179AD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83179AD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179AD8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83179ADC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83179AE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83179AE4: 48012365  bl 0x8318be48
	ctx.lr = 0x83179AE8;
	sub_8318BE48(ctx, base);
	// 83179AE8: 809F1794  lwz r4, 0x1794(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6036 as u32) ) } as u64;
	// 83179AEC: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 83179AF0: 419A0010  beq cr6, 0x83179b00
	if ctx.cr[6].eq {
	pc = 0x83179B00; continue 'dispatch;
	}
	// 83179AF4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83179AF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179AFC: 480144C5  bl 0x8318dfc0
	ctx.lr = 0x83179B00;
	sub_8318DFC0(ctx, base);
	// 83179B00: 809F1790  lwz r4, 0x1790(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6032 as u32) ) } as u64;
	// 83179B04: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 83179B08: 419A0010  beq cr6, 0x83179b18
	if ctx.cr[6].eq {
	pc = 0x83179B18; continue 'dispatch;
	}
	// 83179B0C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83179B10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179B14: 480144AD  bl 0x8318dfc0
	ctx.lr = 0x83179B18;
	sub_8318DFC0(ctx, base);
	// 83179B18: 809F1798  lwz r4, 0x1798(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6040 as u32) ) } as u64;
	// 83179B1C: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 83179B20: 419A0010  beq cr6, 0x83179b30
	if ctx.cr[6].eq {
	pc = 0x83179B30; continue 'dispatch;
	}
	// 83179B24: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83179B28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179B2C: 48014495  bl 0x8318dfc0
	ctx.lr = 0x83179B30;
	sub_8318DFC0(ctx, base);
	// 83179B30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83179B34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83179B38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83179B3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83179B40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179B48 size=128
    let mut pc: u32 = 0x83179B48;
    'dispatch: loop {
        match pc {
            0x83179B48 => {
    //   block [0x83179B48..0x83179BC8)
	// 83179B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83179B50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83179B54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83179B58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179B5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83179B60: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 83179B64: 809F1794  lwz r4, 0x1794(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6036 as u32) ) } as u64;
	// 83179B68: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 83179B6C: 419A000C  beq cr6, 0x83179b78
	if ctx.cr[6].eq {
	pc = 0x83179B78; continue 'dispatch;
	}
	// 83179B70: 48014469  bl 0x8318dfd8
	ctx.lr = 0x83179B74;
	sub_8318DFD8(ctx, base);
	// 83179B74: 547E07FE  clrlwi r30, r3, 0x1f
	ctx.r[30].u64 = ctx.r[3].u32 as u64 & 0x00000001u64;
	// 83179B78: 809F1790  lwz r4, 0x1790(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6032 as u32) ) } as u64;
	// 83179B7C: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 83179B80: 419A0010  beq cr6, 0x83179b90
	if ctx.cr[6].eq {
	pc = 0x83179B90; continue 'dispatch;
	}
	// 83179B84: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179B88: 48014451  bl 0x8318dfd8
	ctx.lr = 0x83179B8C;
	sub_8318DFD8(ctx, base);
	// 83179B8C: 7C7EF038  and r30, r3, r30
	ctx.r[30].u64 = ctx.r[3].u64 & ctx.r[30].u64;
	// 83179B90: 809F1798  lwz r4, 0x1798(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6040 as u32) ) } as u64;
	// 83179B94: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 83179B98: 419A0014  beq cr6, 0x83179bac
	if ctx.cr[6].eq {
	pc = 0x83179BAC; continue 'dispatch;
	}
	// 83179B9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179BA0: 48014439  bl 0x8318dfd8
	ctx.lr = 0x83179BA4;
	sub_8318DFD8(ctx, base);
	// 83179BA4: 7C63F038  and r3, r3, r30
	ctx.r[3].u64 = ctx.r[3].u64 & ctx.r[30].u64;
	// 83179BA8: 48000008  b 0x83179bb0
	pc = 0x83179BB0; continue 'dispatch;
	// 83179BAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83179BB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83179BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83179BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83179BBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83179BC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83179BC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x83179BC8 size=20
    let mut pc: u32 = 0x83179BC8;
    'dispatch: loop {
        match pc {
            0x83179BC8 => {
    //   block [0x83179BC8..0x83179BDC)
	// 83179BC8: 81631784  lwz r11, 0x1784(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6020 as u32) ) } as u64;
	// 83179BCC: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 83179BD0: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 83179BD4: 914B0020  stw r10, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 83179BD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179BE0 size=88
    let mut pc: u32 = 0x83179BE0;
    'dispatch: loop {
        match pc {
            0x83179BE0 => {
    //   block [0x83179BE0..0x83179C38)
	// 83179BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83179BE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83179BEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179BF0: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 83179BF4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83179BF8: 7F045800  cmpw cr6, r4, r11
	ctx.cr[6].compare_i32(ctx.r[4].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83179BFC: 409A0024  bne cr6, 0x83179c20
	if !ctx.cr[6].eq {
	pc = 0x83179C20; continue 'dispatch;
	}
	// 83179C00: 48011DF1  bl 0x8318b9f0
	ctx.lr = 0x83179C04;
	sub_8318B9F0(ctx, base);
	// 83179C04: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179C08: 409A0018  bne cr6, 0x83179c20
	if !ctx.cr[6].eq {
	pc = 0x83179C20; continue 'dispatch;
	}
	// 83179C0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179C10: 48011E11  bl 0x8318ba20
	ctx.lr = 0x83179C14;
	sub_8318BA20(ctx, base);
	// 83179C14: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179C18: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83179C1C: 419A0008  beq cr6, 0x83179c24
	if ctx.cr[6].eq {
	pc = 0x83179C24; continue 'dispatch;
	}
	// 83179C20: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83179C24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83179C28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83179C2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83179C30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83179C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179C38 size=84
    let mut pc: u32 = 0x83179C38;
    'dispatch: loop {
        match pc {
            0x83179C38 => {
    //   block [0x83179C38..0x83179C8C)
	// 83179C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179C3C: 4802E531  bl 0x831a816c
	ctx.lr = 0x83179C40;
	sub_831A8130(ctx, base);
	// 83179C40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179C44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83179C48: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 83179C4C: 809F178C  lwz r4, 0x178c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6028 as u32) ) } as u64;
	// 83179C50: 83DF1784  lwz r30, 0x1784(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6020 as u32) ) } as u64;
	// 83179C54: 48014385  bl 0x8318dfd8
	ctx.lr = 0x83179C58;
	sub_8318DFD8(ctx, base);
	// 83179C58: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 83179C5C: 409A0018  bne cr6, 0x83179c74
	if !ctx.cr[6].eq {
	pc = 0x83179C74; continue 'dispatch;
	}
	// 83179C60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179C64: 809E003C  lwz r4, 0x3c(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(60 as u32) ) } as u64;
	// 83179C68: 4BFFFE61  bl 0x83179ac8
	ctx.lr = 0x83179C6C;
	sub_83179AC8(ctx, base);
	// 83179C6C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 83179C70: 48000008  b 0x83179c78
	pc = 0x83179C78; continue 'dispatch;
	// 83179C74: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 83179C78: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 83179C7C: 419A0008  beq cr6, 0x83179c84
	if ctx.cr[6].eq {
	pc = 0x83179C84; continue 'dispatch;
	}
	// 83179C80: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83179C84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83179C88: 4802E534  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179C90 size=128
    let mut pc: u32 = 0x83179C90;
    'dispatch: loop {
        match pc {
            0x83179C90 => {
    //   block [0x83179C90..0x83179D10)
	// 83179C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83179C98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83179C9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83179CA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179CA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83179CA8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 83179CAC: 809F1794  lwz r4, 0x1794(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6036 as u32) ) } as u64;
	// 83179CB0: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 83179CB4: 419A000C  beq cr6, 0x83179cc0
	if ctx.cr[6].eq {
	pc = 0x83179CC0; continue 'dispatch;
	}
	// 83179CB8: 480142E9  bl 0x8318dfa0
	ctx.lr = 0x83179CBC;
	sub_8318DFA0(ctx, base);
	// 83179CBC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 83179CC0: 809F1790  lwz r4, 0x1790(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6032 as u32) ) } as u64;
	// 83179CC4: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 83179CC8: 419A0010  beq cr6, 0x83179cd8
	if ctx.cr[6].eq {
	pc = 0x83179CD8; continue 'dispatch;
	}
	// 83179CCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179CD0: 480142D1  bl 0x8318dfa0
	ctx.lr = 0x83179CD4;
	sub_8318DFA0(ctx, base);
	// 83179CD4: 7C7EF378  or r30, r3, r30
	ctx.r[30].u64 = ctx.r[3].u64 | ctx.r[30].u64;
	// 83179CD8: 809F1798  lwz r4, 0x1798(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6040 as u32) ) } as u64;
	// 83179CDC: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 83179CE0: 419A0014  beq cr6, 0x83179cf4
	if ctx.cr[6].eq {
	pc = 0x83179CF4; continue 'dispatch;
	}
	// 83179CE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179CE8: 480142B9  bl 0x8318dfa0
	ctx.lr = 0x83179CEC;
	sub_8318DFA0(ctx, base);
	// 83179CEC: 7C63F378  or r3, r3, r30
	ctx.r[3].u64 = ctx.r[3].u64 | ctx.r[30].u64;
	// 83179CF0: 48000008  b 0x83179cf8
	pc = 0x83179CF8; continue 'dispatch;
	// 83179CF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83179CF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83179CFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83179D00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83179D04: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83179D08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83179D0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179D10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179D10 size=120
    let mut pc: u32 = 0x83179D10;
    'dispatch: loop {
        match pc {
            0x83179D10 => {
    //   block [0x83179D10..0x83179D88)
	// 83179D10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179D14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83179D18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83179D1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83179D20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179D24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83179D28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 83179D2C: 809F1794  lwz r4, 0x1794(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6036 as u32) ) } as u64;
	// 83179D30: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 83179D34: 419A000C  beq cr6, 0x83179d40
	if ctx.cr[6].eq {
	pc = 0x83179D40; continue 'dispatch;
	}
	// 83179D38: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83179D3C: 4801424D  bl 0x8318df88
	ctx.lr = 0x83179D40;
	sub_8318DF88(ctx, base);
	// 83179D40: 809F1790  lwz r4, 0x1790(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6032 as u32) ) } as u64;
	// 83179D44: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 83179D48: 419A0010  beq cr6, 0x83179d58
	if ctx.cr[6].eq {
	pc = 0x83179D58; continue 'dispatch;
	}
	// 83179D4C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83179D50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179D54: 48014235  bl 0x8318df88
	ctx.lr = 0x83179D58;
	sub_8318DF88(ctx, base);
	// 83179D58: 809F1798  lwz r4, 0x1798(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6040 as u32) ) } as u64;
	// 83179D5C: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 83179D60: 419A0010  beq cr6, 0x83179d70
	if ctx.cr[6].eq {
	pc = 0x83179D70; continue 'dispatch;
	}
	// 83179D64: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 83179D68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179D6C: 4801421D  bl 0x8318df88
	ctx.lr = 0x83179D70;
	sub_8318DF88(ctx, base);
	// 83179D70: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83179D74: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83179D78: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83179D7C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83179D80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83179D84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179D88 size=112
    let mut pc: u32 = 0x83179D88;
    'dispatch: loop {
        match pc {
            0x83179D88 => {
    //   block [0x83179D88..0x83179DF8)
	// 83179D88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179D8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83179D90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83179D94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179D98: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 83179D9C: 816A0008  lwz r11, 8(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 83179DA0: 83EA0A64  lwz r31, 0xa64(r10)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2660 as u32) ) } as u64;
	// 83179DA4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83179DA8: 41990014  bgt cr6, 0x83179dbc
	if ctx.cr[6].gt {
	pc = 0x83179DBC; continue 'dispatch;
	}
	// 83179DAC: 4BFFFB3D  bl 0x831798e8
	ctx.lr = 0x83179DB0;
	sub_831798E8(ctx, base);
	// 83179DB0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 83179DB4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83179DB8: 40990010  ble cr6, 0x83179dc8
	if !ctx.cr[6].gt {
	pc = 0x83179DC8; continue 'dispatch;
	}
	// 83179DBC: 7F0BF800  cmpw cr6, r11, r31
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[31].s32, &mut ctx.xer);
	// 83179DC0: 40980008  bge cr6, 0x83179dc8
	if !ctx.cr[6].lt {
	pc = 0x83179DC8; continue 'dispatch;
	}
	// 83179DC4: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 83179DC8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 83179DCC: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 83179DD0: 48014051  bl 0x8318de20
	ctx.lr = 0x83179DD4;
	sub_8318DE20(ctx, base);
	// 83179DD4: 7F03F800  cmpw cr6, r3, r31
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[31].s32, &mut ctx.xer);
	// 83179DD8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 83179DDC: 41980008  blt cr6, 0x83179de4
	if ctx.cr[6].lt {
	pc = 0x83179DE4; continue 'dispatch;
	}
	// 83179DE0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 83179DE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 83179DE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83179DEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83179DF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83179DF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179DF8 size=264
    let mut pc: u32 = 0x83179DF8;
    'dispatch: loop {
        match pc {
            0x83179DF8 => {
    //   block [0x83179DF8..0x83179F00)
	// 83179DF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179DFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 83179E00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 83179E04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 83179E08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179E0C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 83179E10: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83179E14: 83DF1784  lwz r30, 0x1784(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6020 as u32) ) } as u64;
	// 83179E18: 4BFFF5F9  bl 0x83179410
	ctx.lr = 0x83179E1C;
	sub_83179410(ctx, base);
	// 83179E1C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179E20: 419A005C  beq cr6, 0x83179e7c
	if ctx.cr[6].eq {
	pc = 0x83179E7C; continue 'dispatch;
	}
	// 83179E24: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 83179E28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179E2C: 4BFFF5E5  bl 0x83179410
	ctx.lr = 0x83179E30;
	sub_83179410(ctx, base);
	// 83179E30: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179E34: 419A0048  beq cr6, 0x83179e7c
	if ctx.cr[6].eq {
	pc = 0x83179E7C; continue 'dispatch;
	}
	// 83179E38: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 83179E3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179E40: 48013FE1  bl 0x8318de20
	ctx.lr = 0x83179E44;
	sub_8318DE20(ctx, base);
	// 83179E44: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179E48: 409A0034  bne cr6, 0x83179e7c
	if !ctx.cr[6].eq {
	pc = 0x83179E7C; continue 'dispatch;
	}
	// 83179E4C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 83179E50: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83179E54: 409A0028  bne cr6, 0x83179e7c
	if !ctx.cr[6].eq {
	pc = 0x83179E7C; continue 'dispatch;
	}
	// 83179E58: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83179E5C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179E60: 48011FD9  bl 0x8318be38
	ctx.lr = 0x83179E64;
	sub_8318BE38(ctx, base);
	// 83179E64: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179E68: 419A0014  beq cr6, 0x83179e7c
	if ctx.cr[6].eq {
	pc = 0x83179E7C; continue 'dispatch;
	}
	// 83179E6C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83179E70: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 83179E74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179E78: 4BFFF5E1  bl 0x83179458
	ctx.lr = 0x83179E7C;
	sub_83179458(ctx, base);
	// 83179E7C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 83179E80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179E84: 4BFFF58D  bl 0x83179410
	ctx.lr = 0x83179E88;
	sub_83179410(ctx, base);
	// 83179E88: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179E8C: 419A005C  beq cr6, 0x83179ee8
	if ctx.cr[6].eq {
	pc = 0x83179EE8; continue 'dispatch;
	}
	// 83179E90: 3880004F  li r4, 0x4f
	ctx.r[4].s64 = 79;
	// 83179E94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179E98: 4BFFF579  bl 0x83179410
	ctx.lr = 0x83179E9C;
	sub_83179410(ctx, base);
	// 83179E9C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179EA0: 419A0048  beq cr6, 0x83179ee8
	if ctx.cr[6].eq {
	pc = 0x83179EE8; continue 'dispatch;
	}
	// 83179EA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 83179EA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179EAC: 48013F75  bl 0x8318de20
	ctx.lr = 0x83179EB0;
	sub_8318DE20(ctx, base);
	// 83179EB0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179EB4: 409A0034  bne cr6, 0x83179ee8
	if !ctx.cr[6].eq {
	pc = 0x83179EE8; continue 'dispatch;
	}
	// 83179EB8: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83179EBC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83179EC0: 409A0028  bne cr6, 0x83179ee8
	if !ctx.cr[6].eq {
	pc = 0x83179EE8; continue 'dispatch;
	}
	// 83179EC4: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 83179EC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179ECC: 48011F6D  bl 0x8318be38
	ctx.lr = 0x83179ED0;
	sub_8318BE38(ctx, base);
	// 83179ED0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 83179ED4: 419A0014  beq cr6, 0x83179ee8
	if ctx.cr[6].eq {
	pc = 0x83179EE8; continue 'dispatch;
	}
	// 83179ED8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 83179EDC: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 83179EE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 83179EE4: 4BFFF575  bl 0x83179458
	ctx.lr = 0x83179EE8;
	sub_83179458(ctx, base);
	// 83179EE8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 83179EEC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 83179EF0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 83179EF4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 83179EF8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 83179EFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179F00 size=128
    let mut pc: u32 = 0x83179F00;
    'dispatch: loop {
        match pc {
            0x83179F00 => {
    //   block [0x83179F00..0x83179F80)
	// 83179F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179F04: 4802E259  bl 0x831a815c
	ctx.lr = 0x83179F08;
	sub_831A8130(ctx, base);
	// 83179F08: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179F0C: 83C31784  lwz r30, 0x1784(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6020 as u32) ) } as u64;
	// 83179F10: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 83179F14: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 83179F18: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 83179F1C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 83179F20: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 83179F24: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 83179F28: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 83179F2C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83179F30: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 83179F34: 48014ECD  bl 0x8318ee00
	ctx.lr = 0x83179F38;
	sub_8318EE00(ctx, base);
	// 83179F38: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 83179F3C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83179F40: 41990008  bgt cr6, 0x83179f48
	if ctx.cr[6].gt {
	pc = 0x83179F48; continue 'dispatch;
	}
	// 83179F44: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 83179F48: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83179F4C: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 83179F50: 41990008  bgt cr6, 0x83179f58
	if ctx.cr[6].gt {
	pc = 0x83179F58; continue 'dispatch;
	}
	// 83179F54: 7D7C5B78  mr r28, r11
	ctx.r[28].u64 = ctx.r[11].u64;
	// 83179F58: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 83179F5C: 2F1D0003  cmpwi cr6, r29, 3
	ctx.cr[6].compare_i32(ctx.r[29].s32, 3, &mut ctx.xer);
	// 83179F60: 4198FFC8  blt cr6, 0x83179f28
	if ctx.cr[6].lt {
	pc = 0x83179F28; continue 'dispatch;
	}
	// 83179F64: 939E0004  stw r28, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 83179F68: 93FE0008  stw r31, 8(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 83179F6C: 93FA0000  stw r31, 0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 83179F70: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 83179F74: 91790000  stw r11, 0(r25)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[25].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 83179F78: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 83179F7C: 4802E230  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_83179F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x83179F80 size=140
    let mut pc: u32 = 0x83179F80;
    'dispatch: loop {
        match pc {
            0x83179F80 => {
    //   block [0x83179F80..0x8317A00C)
	// 83179F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 83179F84: 4802E1E9  bl 0x831a816c
	ctx.lr = 0x83179F88;
	sub_831A8130(ctx, base);
	// 83179F88: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 83179F8C: 83A31784  lwz r29, 0x1784(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6020 as u32) ) } as u64;
	// 83179F90: 3BE30910  addi r31, r3, 0x910
	ctx.r[31].s64 = ctx.r[3].s64 + 2320;
	// 83179F94: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 83179F98: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 83179F9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83179FA0: 48014DF9  bl 0x8318ed98
	ctx.lr = 0x83179FA4;
	sub_8318ED98(ctx, base);
	// 83179FA4: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 83179FA8: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 83179FAC: 419A0010  beq cr6, 0x83179fbc
	if ctx.cr[6].eq {
	pc = 0x83179FBC; continue 'dispatch;
	}
	// 83179FB0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 83179FB4: 40990008  ble cr6, 0x83179fbc
	if !ctx.cr[6].gt {
	pc = 0x83179FBC; continue 'dispatch;
	}
	// 83179FB8: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 83179FBC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 83179FC0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 83179FC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 83179FC8: 48014E39  bl 0x8318ee00
	ctx.lr = 0x83179FCC;
	sub_8318EE00(ctx, base);
	// 83179FCC: 81610070  lwz r11, 0x70(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(112 as u32) ) } as u64;
	// 83179FD0: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 83179FD4: 419A0008  beq cr6, 0x83179fdc
	if ctx.cr[6].eq {
	pc = 0x83179FDC; continue 'dispatch;
	}
	// 83179FD8: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 83179FDC: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 83179FE0: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 83179FE4: 409A000C  bne cr6, 0x83179ff0
	if !ctx.cr[6].eq {
	pc = 0x83179FF0; continue 'dispatch;
	}
	// 83179FE8: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 83179FEC: 917F0024  stw r11, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 83179FF0: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 83179FF4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 83179FF8: 409A000C  bne cr6, 0x8317a004
	if !ctx.cr[6].eq {
	pc = 0x8317A004; continue 'dispatch;
	}
	// 83179FFC: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 8317A000: 917F0028  stw r11, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 8317A004: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8317A008: 4802E1B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317A010 size=80
    let mut pc: u32 = 0x8317A010;
    'dispatch: loop {
        match pc {
            0x8317A010 => {
    //   block [0x8317A010..0x8317A060)
	// 8317A010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317A014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317A018: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317A01C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317A020: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317A024: 807F13AC  lwz r3, 0x13ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5036 as u32) ) } as u64;
	// 8317A028: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317A02C: 419A0020  beq cr6, 0x8317a04c
	if ctx.cr[6].eq {
	pc = 0x8317A04C; continue 'dispatch;
	}
	// 8317A030: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8317A034: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8317A038: 48014471  bl 0x8318e4a8
	ctx.lr = 0x8317A03C;
	sub_8318E4A8(ctx, base);
	// 8317A03C: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317A040: E87F0988  ld r3, 0x988(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(2440 as u32) ) };
	// 8317A044: 480140DD  bl 0x8318e120
	ctx.lr = 0x8317A048;
	sub_8318E120(ctx, base);
	// 8317A048: F87F0988  std r3, 0x988(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(2440 as u32), ctx.r[3].u64 ) };
	// 8317A04C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317A050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317A054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317A058: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317A05C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317A060 size=132
    let mut pc: u32 = 0x8317A060;
    'dispatch: loop {
        match pc {
            0x8317A060 => {
    //   block [0x8317A060..0x8317A0E4)
	// 8317A060: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8317A064: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317A068: 794A0040  clrldi r10, r10, 1
	ctx.r[10].u64 = ctx.r[10].u64 & 0x7FFFFFFFFFFFFFFFu64;
	// 8317A06C: 39230040  addi r9, r3, 0x40
	ctx.r[9].s64 = ctx.r[3].s64 + 64;
	// 8317A070: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 8317A074: 7D475378  mr r7, r10
	ctx.r[7].u64 = ctx.r[10].u64;
	// 8317A078: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 8317A07C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317A080: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8317A084: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 8317A088: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 8317A08C: F9030010  std r8, 0x10(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[8].u64 ) };
	// 8317A090: 39000044  li r8, 0x44
	ctx.r[8].s64 = 68;
	// 8317A094: 7D465378  mr r6, r10
	ctx.r[6].u64 = ctx.r[10].u64;
	// 8317A098: F8E30018  std r7, 0x18(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), ctx.r[7].u64 ) };
	// 8317A09C: 7D455378  mr r5, r10
	ctx.r[5].u64 = ctx.r[10].u64;
	// 8317A0A0: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8317A0A4: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 8317A0A8: 9163003C  stw r11, 0x3c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8317A0AC: 90C30024  stw r6, 0x24(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(36 as u32), ctx.r[6].u32 ) };
	// 8317A0B0: 90A30028  stw r5, 0x28(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(40 as u32), ctx.r[5].u32 ) };
	// 8317A0B4: 9143002C  stw r10, 0x2c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(44 as u32), ctx.r[10].u32 ) };
	// 8317A0B8: 91430030  stw r10, 0x30(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 8317A0BC: 91430034  stw r10, 0x34(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(52 as u32), ctx.r[10].u32 ) };
	// 8317A0C0: 91430038  stw r10, 0x38(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 8317A0C4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 8317A0C8: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8317A0CC: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 8317A0D0: 4200FFF8  bdnz 0x8317a0c8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8317A0C8; continue 'dispatch;
	}
	// 8317A0D4: 91630150  stw r11, 0x150(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(336 as u32), ctx.r[11].u32 ) };
	// 8317A0D8: 91630154  stw r11, 0x154(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(340 as u32), ctx.r[11].u32 ) };
	// 8317A0DC: 91430158  stw r10, 0x158(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(344 as u32), ctx.r[10].u32 ) };
	// 8317A0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317A0E8 size=4
    let mut pc: u32 = 0x8317A0E8;
    'dispatch: loop {
        match pc {
            0x8317A0E8 => {
    //   block [0x8317A0E8..0x8317A0EC)
	// 8317A0E8: 4800D410  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A0F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317A0F0 size=4
    let mut pc: u32 = 0x8317A0F0;
    'dispatch: loop {
        match pc {
            0x8317A0F0 => {
    //   block [0x8317A0F0..0x8317A0F4)
	// 8317A0F0: 48012440  b 0x8318c530
	sub_8318C530(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A0F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317A0F8 size=12
    let mut pc: u32 = 0x8317A0F8;
    'dispatch: loop {
        match pc {
            0x8317A0F8 => {
    //   block [0x8317A0F8..0x8317A104)
	// 8317A0F8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317A0FC: 60840D0B  ori r4, r4, 0xd0b
	ctx.r[4].u64 = ctx.r[4].u64 | 3339;
	// 8317A100: 4800D3F8  b 0x831874f8
	sub_831874F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317A108 size=124
    let mut pc: u32 = 0x8317A108;
    'dispatch: loop {
        match pc {
            0x8317A108 => {
    //   block [0x8317A108..0x8317A184)
	// 8317A108: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317A10C: 4802E05D  bl 0x831a8168
	ctx.lr = 0x8317A110;
	sub_831A8130(ctx, base);
	// 8317A110: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317A114: 3BE50030  addi r31, r5, 0x30
	ctx.r[31].s64 = ctx.r[5].s64 + 48;
	// 8317A118: 83C40000  lwz r30, 0(r4)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317A11C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8317A120: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8317A124: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8317A128: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8317A12C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A130: 80BF0160  lwz r5, 0x160(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(352 as u32) ) } as u64;
	// 8317A134: 48012685  bl 0x8318c7b8
	ctx.lr = 0x8317A138;
	sub_8318C7B8(ctx, base);
	// 8317A138: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317A13C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8317A140: 80BF0164  lwz r5, 0x164(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(356 as u32) ) } as u64;
	// 8317A144: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 8317A148: 389F00B0  addi r4, r31, 0xb0
	ctx.r[4].s64 = ctx.r[31].s64 + 176;
	// 8317A14C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A150: 48012669  bl 0x8318c7b8
	ctx.lr = 0x8317A154;
	sub_8318C7B8(ctx, base);
	// 8317A154: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8317A158: 409A0014  bne cr6, 0x8317a16c
	if !ctx.cr[6].eq {
	pc = 0x8317A16C; continue 'dispatch;
	}
	// 8317A15C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317A160: 409A000C  bne cr6, 0x8317a16c
	if !ctx.cr[6].eq {
	pc = 0x8317A16C; continue 'dispatch;
	}
	// 8317A164: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317A168: 4802E050  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 8317A16C: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317A170: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8317A174: 60840D0D  ori r4, r4, 0xd0d
	ctx.r[4].u64 = ctx.r[4].u64 | 3341;
	// 8317A178: 4800D381  bl 0x831874f8
	ctx.lr = 0x8317A17C;
	sub_831874F8(ctx, base);
	// 8317A17C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317A180: 4802E038  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8317A188 size=12
    let mut pc: u32 = 0x8317A188;
    'dispatch: loop {
        match pc {
            0x8317A188 => {
    //   block [0x8317A188..0x8317A194)
	// 8317A188: 81631784  lwz r11, 0x1784(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(6020 as u32) ) } as u64;
	// 8317A18C: 806B0020  lwz r3, 0x20(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8317A190: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317A198 size=112
    let mut pc: u32 = 0x8317A198;
    'dispatch: loop {
        match pc {
            0x8317A198 => {
    //   block [0x8317A198..0x8317A208)
	// 8317A198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317A19C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317A1A0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317A1A4: 480042E5  bl 0x8317e488
	ctx.lr = 0x8317A1A8;
	sub_8317E488(ctx, base);
	// 8317A1A8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317A1AC: 419A0008  beq cr6, 0x8317a1b4
	if ctx.cr[6].eq {
	pc = 0x8317A1B4; continue 'dispatch;
	}
	// 8317A1B0: 48000000  b 0x8317a1b0
	pc = 0x8317A1B0; continue 'dispatch;
	// 8317A1B4: 3D60833A  lis r11, -0x7cc6
	ctx.r[11].s64 = -2093350912;
	// 8317A1B8: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 8317A1BC: 388B9C58  addi r4, r11, -0x63a8
	ctx.r[4].s64 = ctx.r[11].s64 + -25512;
	// 8317A1C0: 48012509  bl 0x8318c6c8
	ctx.lr = 0x8317A1C4;
	sub_8318C6C8(ctx, base);
	// 8317A1C4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317A1C8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317A1CC: 419A0020  beq cr6, 0x8317a1ec
	if ctx.cr[6].eq {
	pc = 0x8317A1EC; continue 'dispatch;
	}
	// 8317A1D0: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317A1D4: 60840D01  ori r4, r4, 0xd01
	ctx.r[4].u64 = ctx.r[4].u64 | 3329;
	// 8317A1D8: 4800D321  bl 0x831874f8
	ctx.lr = 0x8317A1DC;
	sub_831874F8(ctx, base);
	// 8317A1DC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317A1E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317A1E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317A1E8: 4E800020  blr
	return;
	// 8317A1EC: 3D408345  lis r10, -0x7cbb
	ctx.r[10].s64 = -2092630016;
	// 8317A1F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317A1F4: 916AA370  stw r11, -0x5c90(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23696 as u32), ctx.r[11].u32 ) };
	// 8317A1F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317A1FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317A200: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317A204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317A208 size=112
    let mut pc: u32 = 0x8317A208;
    'dispatch: loop {
        match pc {
            0x8317A208 => {
    //   block [0x8317A208..0x8317A278)
	// 8317A208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317A20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317A210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317A214: 4BFFF6D5  bl 0x831798e8
	ctx.lr = 0x8317A218;
	sub_831798E8(ctx, base);
	// 8317A218: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317A21C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317A220: 409A0030  bne cr6, 0x8317a250
	if !ctx.cr[6].eq {
	pc = 0x8317A250; continue 'dispatch;
	}
	// 8317A224: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317A228: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317A22C: 409A0010  bne cr6, 0x8317a23c
	if !ctx.cr[6].eq {
	pc = 0x8317A23C; continue 'dispatch;
	}
	// 8317A230: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 8317A234: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317A238: 419A0018  beq cr6, 0x8317a250
	if ctx.cr[6].eq {
	pc = 0x8317A250; continue 'dispatch;
	}
	// 8317A23C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317A240: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317A244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317A248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317A24C: 4E800020  blr
	return;
	// 8317A250: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8317A254: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317A258: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8317A25C: 7D645850  subf r11, r4, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[4].s64;
	// 8317A260: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8317A264: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317A268: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317A26C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317A270: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317A274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317A278 size=280
    let mut pc: u32 = 0x8317A278;
    'dispatch: loop {
        match pc {
            0x8317A278 => {
    //   block [0x8317A278..0x8317A390)
	// 8317A278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317A27C: 4802DEE1  bl 0x831a815c
	ctx.lr = 0x8317A280;
	sub_831A8130(ctx, base);
	// 8317A280: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317A284: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317A288: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8317A28C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8317A290: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8317A294: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8317A298: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8317A29C: 4BFFF175  bl 0x83179410
	ctx.lr = 0x8317A2A0;
	sub_83179410(ctx, base);
	// 8317A2A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317A2A4: 409A0010  bne cr6, 0x8317a2b4
	if !ctx.cr[6].eq {
	pc = 0x8317A2B4; continue 'dispatch;
	}
	// 8317A2A8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317A2AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317A2B0: 4802DEFC  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 8317A2B4: 83FB1784  lwz r31, 0x1784(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(6020 as u32) ) } as u64;
	// 8317A2B8: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8317A2BC: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8317A2C0: 409A0008  bne cr6, 0x8317a2c8
	if !ctx.cr[6].eq {
	pc = 0x8317A2C8; continue 'dispatch;
	}
	// 8317A2C4: 93DF0038  stw r30, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[30].u32 ) };
	// 8317A2C8: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8317A2CC: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8317A2D0: 409A0008  bne cr6, 0x8317a2d8
	if !ctx.cr[6].eq {
	pc = 0x8317A2D8; continue 'dispatch;
	}
	// 8317A2D4: 93DF0030  stw r30, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[30].u32 ) };
	// 8317A2D8: 3880001E  li r4, 0x1e
	ctx.r[4].s64 = 30;
	// 8317A2DC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8317A2E0: 4BFFF131  bl 0x83179410
	ctx.lr = 0x8317A2E4;
	sub_83179410(ctx, base);
	// 8317A2E4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8317A2E8: 2F1DFFFF  cmpwi cr6, r29, -1
	ctx.cr[6].compare_i32(ctx.r[29].s32, -1, &mut ctx.xer);
	// 8317A2EC: 419A0044  beq cr6, 0x8317a330
	if ctx.cr[6].eq {
	pc = 0x8317A330; continue 'dispatch;
	}
	// 8317A2F0: 38800037  li r4, 0x37
	ctx.r[4].s64 = 55;
	// 8317A2F4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8317A2F8: 4BFFF119  bl 0x83179410
	ctx.lr = 0x8317A2FC;
	sub_83179410(ctx, base);
	// 8317A2FC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317A300: 419A0014  beq cr6, 0x8317a314
	if ctx.cr[6].eq {
	pc = 0x8317A314; continue 'dispatch;
	}
	// 8317A304: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8317A308: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317A30C: 41980020  blt cr6, 0x8317a32c
	if ctx.cr[6].lt {
	pc = 0x8317A32C; continue 'dispatch;
	}
	// 8317A310: 48000020  b 0x8317a330
	pc = 0x8317A330; continue 'dispatch;
	// 8317A314: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 8317A318: 7D7E5850  subf r11, r30, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[30].s64;
	// 8317A31C: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8317A320: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317A324: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317A328: 419A0008  beq cr6, 0x8317a330
	if ctx.cr[6].eq {
	pc = 0x8317A330; continue 'dispatch;
	}
	// 8317A32C: 93BF0038  stw r29, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[29].u32 ) };
	// 8317A330: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 8317A334: 93DF0028  stw r30, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[30].u32 ) };
	// 8317A338: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8317A33C: 409AFF6C  bne cr6, 0x8317a2a8
	if !ctx.cr[6].eq {
	pc = 0x8317A2A8; continue 'dispatch;
	}
	// 8317A340: 2F3C0000  cmpdi cr6, r28, 0
	ctx.cr[6].compare_i64(ctx.r[28].s64, 0, &mut ctx.xer);
	// 8317A344: 4198002C  blt cr6, 0x8317a370
	if ctx.cr[6].lt {
	pc = 0x8317A370; continue 'dispatch;
	}
	// 8317A348: E95F0010  ld r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	// 8317A34C: 7F3C5000  cmpd cr6, r28, r10
	ctx.cr[6].compare_i64(ctx.r[28].s64, ctx.r[10].s64, &mut ctx.xer);
	// 8317A350: 40980008  bge cr6, 0x8317a358
	if !ctx.cr[6].lt {
	pc = 0x8317A358; continue 'dispatch;
	}
	// 8317A354: 7F8AE378  mr r10, r28
	ctx.r[10].u64 = ctx.r[28].u64;
	// 8317A358: E97F0018  ld r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	// 8317A35C: F95F0010  std r10, 0x10(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u64 ) };
	// 8317A360: 7F3C5800  cmpd cr6, r28, r11
	ctx.cr[6].compare_i64(ctx.r[28].s64, ctx.r[11].s64, &mut ctx.xer);
	// 8317A364: 40980008  bge cr6, 0x8317a36c
	if !ctx.cr[6].lt {
	pc = 0x8317A36C; continue 'dispatch;
	}
	// 8317A368: 7F8BE378  mr r11, r28
	ctx.r[11].u64 = ctx.r[28].u64;
	// 8317A36C: F97F0018  std r11, 0x18(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u64 ) };
	// 8317A370: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 8317A374: 809B1794  lwz r4, 0x1794(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(6036 as u32) ) } as u64;
	// 8317A378: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 8317A37C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8317A380: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8317A384: 4BFFF5FD  bl 0x83179980
	ctx.lr = 0x8317A388;
	sub_83179980(ctx, base);
	// 8317A388: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8317A38C: 4802DE20  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A390(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317A390 size=112
    let mut pc: u32 = 0x8317A390;
    'dispatch: loop {
        match pc {
            0x8317A390 => {
    //   block [0x8317A390..0x8317A400)
	// 8317A390: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317A394: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317A398: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317A39C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317A3A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317A3A4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317A3A8: 3880003B  li r4, 0x3b
	ctx.r[4].s64 = 59;
	// 8317A3AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317A3B0: 4BFFF061  bl 0x83179410
	ctx.lr = 0x8317A3B4;
	sub_83179410(ctx, base);
	// 8317A3B4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317A3B8: 419A002C  beq cr6, 0x8317a3e4
	if ctx.cr[6].eq {
	pc = 0x8317A3E4; continue 'dispatch;
	}
	// 8317A3BC: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 8317A3C0: 409A0024  bne cr6, 0x8317a3e4
	if !ctx.cr[6].eq {
	pc = 0x8317A3E4; continue 'dispatch;
	}
	// 8317A3C4: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8317A3C8: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 8317A3CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317A3D0: 4BFFFB31  bl 0x83179f00
	ctx.lr = 0x8317A3D4;
	sub_83179F00(ctx, base);
	// 8317A3D4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317A3D8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 8317A3DC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8317A3E0: 40980008  bge cr6, 0x8317a3e8
	if !ctx.cr[6].lt {
	pc = 0x8317A3E8; continue 'dispatch;
	}
	// 8317A3E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A3E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317A3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317A3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317A3F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317A3F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317A3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317A400 size=152
    let mut pc: u32 = 0x8317A400;
    'dispatch: loop {
        match pc {
            0x8317A400 => {
    //   block [0x8317A400..0x8317A498)
	// 8317A400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317A404: 4802DD69  bl 0x831a816c
	ctx.lr = 0x8317A408;
	sub_831A8130(ctx, base);
	// 8317A408: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317A40C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317A410: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8317A414: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8317A418: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317A41C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317A420: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317A424: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8317A428: 4E800421  bctrl
	ctx.lr = 0x8317A42C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317A42C: 7F03F800  cmpw cr6, r3, r31
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8317A430: 40980010  bge cr6, 0x8317a440
	if !ctx.cr[6].lt {
	pc = 0x8317A440; continue 'dispatch;
	}
	// 8317A434: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317A438: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317A43C: 4802DD80  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317A440: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8317A444: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8317A448: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A44C: 4BFFF4B5  bl 0x83179900
	ctx.lr = 0x8317A450;
	sub_83179900(ctx, base);
	// 8317A450: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317A454: 419AFFE0  beq cr6, 0x8317a434
	if ctx.cr[6].eq {
	pc = 0x8317A434; continue 'dispatch;
	}
	// 8317A458: 7FE3F850  subf r31, r3, r31
	ctx.r[31].s64 = ctx.r[31].s64 - ctx.r[3].s64;
	// 8317A45C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 8317A460: 4099002C  ble cr6, 0x8317a48c
	if !ctx.cr[6].gt {
	pc = 0x8317A48C; continue 'dispatch;
	}
	// 8317A464: 7C83EA14  add r4, r3, r29
	ctx.r[4].u64 = ctx.r[3].u64 + ctx.r[29].u64;
	// 8317A468: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8317A46C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A470: 4BFFF491  bl 0x83179900
	ctx.lr = 0x8317A474;
	sub_83179900(ctx, base);
	// 8317A474: 7F03F800  cmpw cr6, r3, r31
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[31].s32, &mut ctx.xer);
	// 8317A478: 419A0014  beq cr6, 0x8317a48c
	if ctx.cr[6].eq {
	pc = 0x8317A48C; continue 'dispatch;
	}
	// 8317A47C: 3D608345  lis r11, -0x7cbb
	ctx.r[11].s64 = -2092630016;
	// 8317A480: 814BA370  lwz r10, -0x5c90(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-23696 as u32) ) } as u64;
	// 8317A484: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8317A488: 914BA370  stw r10, -0x5c90(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-23696 as u32), ctx.r[10].u32 ) };
	// 8317A48C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317A490: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317A494: 4802DD28  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317A498 size=280
    let mut pc: u32 = 0x8317A498;
    'dispatch: loop {
        match pc {
            0x8317A498 => {
    //   block [0x8317A498..0x8317A5B0)
	// 8317A498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317A49C: 4802DCC1  bl 0x831a815c
	ctx.lr = 0x8317A4A0;
	sub_831A8130(ctx, base);
	// 8317A4A0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317A4A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317A4A8: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8317A4AC: 3BFE177C  addi r31, r30, 0x177c
	ctx.r[31].s64 = ctx.r[30].s64 + 6012;
	// 8317A4B0: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8317A4B4: 2F1A0004  cmpwi cr6, r26, 4
	ctx.cr[6].compare_i32(ctx.r[26].s32, 4, &mut ctx.xer);
	// 8317A4B8: 839F0008  lwz r28, 8(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8317A4BC: 83BF0010  lwz r29, 0x10(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8317A4C0: 4198005C  blt cr6, 0x8317a51c
	if ctx.cr[6].lt {
	pc = 0x8317A51C; continue 'dispatch;
	}
	// 8317A4C4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8317A4C8: 48014AB1  bl 0x8318ef78
	ctx.lr = 0x8317A4CC;
	sub_8318EF78(ctx, base);
	// 8317A4CC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8317A4D0: 3D600008  lis r11, 8
	ctx.r[11].s64 = 524288;
	// 8317A4D4: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317A4D8: 409A0030  bne cr6, 0x8317a508
	if !ctx.cr[6].eq {
	pc = 0x8317A508; continue 'dispatch;
	}
	// 8317A4DC: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 8317A4E0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317A4E4: 40980018  bge cr6, 0x8317a4fc
	if !ctx.cr[6].lt {
	pc = 0x8317A4FC; continue 'dispatch;
	}
	// 8317A4E8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8317A4EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A4F0: 48013921  bl 0x8318de10
	ctx.lr = 0x8317A4F4;
	sub_8318DE10(ctx, base);
	// 8317A4F4: 39630004  addi r11, r3, 4
	ctx.r[11].s64 = ctx.r[3].s64 + 4;
	// 8317A4F8: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 8317A4FC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8317A500: 917C003C  stw r11, 0x3c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8317A504: 4800001C  b 0x8317a520
	pc = 0x8317A520; continue 'dispatch;
	// 8317A508: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8317A50C: 419A0014  beq cr6, 0x8317a520
	if ctx.cr[6].eq {
	pc = 0x8317A520; continue 'dispatch;
	}
	// 8317A510: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317A514: 917C003C  stw r11, 0x3c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8317A518: 48000008  b 0x8317a520
	pc = 0x8317A520; continue 'dispatch;
	// 8317A51C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8317A520: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8317A524: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A528: 4BFFF6B9  bl 0x83179be0
	ctx.lr = 0x8317A52C;
	sub_83179BE0(ctx, base);
	// 8317A52C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317A530: 419A001C  beq cr6, 0x8317a54c
	if ctx.cr[6].eq {
	pc = 0x8317A54C; continue 'dispatch;
	}
	// 8317A534: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8317A538: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A53C: 4BFFF58D  bl 0x83179ac8
	ctx.lr = 0x8317A540;
	sub_83179AC8(ctx, base);
	// 8317A540: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317A544: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8317A548: 4802DC64  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 8317A54C: 2F190004  cmpwi cr6, r25, 4
	ctx.cr[6].compare_i32(ctx.r[25].s32, 4, &mut ctx.xer);
	// 8317A550: 4098001C  bge cr6, 0x8317a56c
	if !ctx.cr[6].lt {
	pc = 0x8317A56C; continue 'dispatch;
	}
	// 8317A554: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8317A558: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A55C: 4BFFF6DD  bl 0x83179c38
	ctx.lr = 0x8317A560;
	sub_83179C38(ctx, base);
	// 8317A560: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317A564: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317A568: 409AFFD8  bne cr6, 0x8317a540
	if !ctx.cr[6].eq {
	pc = 0x8317A540; continue 'dispatch;
	}
	// 8317A56C: 2F1A0040  cmpwi cr6, r26, 0x40
	ctx.cr[6].compare_i32(ctx.r[26].s32, 64, &mut ctx.xer);
	// 8317A570: 40980034  bge cr6, 0x8317a5a4
	if !ctx.cr[6].lt {
	pc = 0x8317A5A4; continue 'dispatch;
	}
	// 8317A574: 3D600001  lis r11, 1
	ctx.r[11].s64 = 65536;
	// 8317A578: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317A57C: 419A0010  beq cr6, 0x8317a58c
	if ctx.cr[6].eq {
	pc = 0x8317A58C; continue 'dispatch;
	}
	// 8317A580: 3D600004  lis r11, 4
	ctx.r[11].s64 = 262144;
	// 8317A584: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317A588: 409A001C  bne cr6, 0x8317a5a4
	if !ctx.cr[6].eq {
	pc = 0x8317A5A4; continue 'dispatch;
	}
	// 8317A58C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8317A590: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A594: 4BFFF6A5  bl 0x83179c38
	ctx.lr = 0x8317A598;
	sub_83179C38(ctx, base);
	// 8317A598: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317A59C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8317A5A0: 4802DC0C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 8317A5A4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317A5A8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8317A5AC: 4802DC00  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A5B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317A5B0 size=116
    let mut pc: u32 = 0x8317A5B0;
    'dispatch: loop {
        match pc {
            0x8317A5B0 => {
    //   block [0x8317A5B0..0x8317A624)
	// 8317A5B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317A5B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317A5B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8317A5BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317A5C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317A5C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317A5C8: 4BFFF6C9  bl 0x83179c90
	ctx.lr = 0x8317A5CC;
	sub_83179C90(ctx, base);
	// 8317A5CC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317A5D0: 419A003C  beq cr6, 0x8317a60c
	if ctx.cr[6].eq {
	pc = 0x8317A60C; continue 'dispatch;
	}
	// 8317A5D4: 83DF178C  lwz r30, 0x178c(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6028 as u32) ) } as u64;
	// 8317A5D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317A5DC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317A5E0: 480139C1  bl 0x8318dfa0
	ctx.lr = 0x8317A5E4;
	sub_8318DFA0(ctx, base);
	// 8317A5E4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8317A5E8: 409A0024  bne cr6, 0x8317a60c
	if !ctx.cr[6].eq {
	pc = 0x8317A60C; continue 'dispatch;
	}
	// 8317A5EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317A5F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317A5F4: 4BFFF795  bl 0x83179d88
	ctx.lr = 0x8317A5F8;
	sub_83179D88(ctx, base);
	// 8317A5F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317A5FC: 419A0010  beq cr6, 0x8317a60c
	if ctx.cr[6].eq {
	pc = 0x8317A60C; continue 'dispatch;
	}
	// 8317A600: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8317A604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317A608: 4BFFF709  bl 0x83179d10
	ctx.lr = 0x8317A60C;
	sub_83179D10(ctx, base);
	// 8317A60C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317A610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317A614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317A618: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8317A61C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317A620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317A628 size=164
    let mut pc: u32 = 0x8317A628;
    'dispatch: loop {
        match pc {
            0x8317A628 => {
    //   block [0x8317A628..0x8317A6CC)
	// 8317A628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317A62C: 4802DB41  bl 0x831a816c
	ctx.lr = 0x8317A630;
	sub_831A8130(ctx, base);
	// 8317A630: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317A634: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317A638: 3BBF19A0  addi r29, r31, 0x19a0
	ctx.r[29].s64 = ctx.r[31].s64 + 6560;
	// 8317A63C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317A640: 93BF1784  stw r29, 0x1784(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(6020 as u32), ctx.r[29].u32 ) };
	// 8317A644: 4BFFFA1D  bl 0x8317a060
	ctx.lr = 0x8317A648;
	sub_8317A060(ctx, base);
	// 8317A648: 48012041  bl 0x8318c688
	ctx.lr = 0x8317A64C;
	sub_8318C688(ctx, base);
	// 8317A64C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317A650: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8317A654: 409A0018  bne cr6, 0x8317a66c
	if !ctx.cr[6].eq {
	pc = 0x8317A66C; continue 'dispatch;
	}
	// 8317A658: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317A65C: 60840D08  ori r4, r4, 0xd08
	ctx.r[4].u64 = ctx.r[4].u64 | 3336;
	// 8317A660: 4800CE99  bl 0x831874f8
	ctx.lr = 0x8317A664;
	sub_831874F8(ctx, base);
	// 8317A664: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317A668: 4802DB54  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317A66C: 3D608318  lis r11, -0x7ce8
	ctx.r[11].s64 = -2095579136;
	// 8317A670: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 8317A674: 388BA0E8  addi r4, r11, -0x5f18
	ctx.r[4].s64 = ctx.r[11].s64 + -24344;
	// 8317A678: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A67C: 48011D9D  bl 0x8318c418
	ctx.lr = 0x8317A680;
	sub_8318C418(ctx, base);
	// 8317A680: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317A684: 419A0024  beq cr6, 0x8317a6a8
	if ctx.cr[6].eq {
	pc = 0x8317A6A8; continue 'dispatch;
	}
	// 8317A688: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A68C: 4BFFFA65  bl 0x8317a0f0
	ctx.lr = 0x8317A690;
	sub_8317A0F0(ctx, base);
	// 8317A690: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317A694: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317A698: 60840D09  ori r4, r4, 0xd09
	ctx.r[4].u64 = ctx.r[4].u64 | 3337;
	// 8317A69C: 4800CE5D  bl 0x831874f8
	ctx.lr = 0x8317A6A0;
	sub_831874F8(ctx, base);
	// 8317A6A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317A6A4: 4802DB18  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 8317A6A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8317A6AC: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 8317A6B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317A6B4: 917F1FA0  stw r11, 0x1fa0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8096 as u32), ctx.r[11].u32 ) };
	// 8317A6B8: 917F1FA4  stw r11, 0x1fa4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8100 as u32), ctx.r[11].u32 ) };
	// 8317A6BC: 917F1FA8  stw r11, 0x1fa8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8104 as u32), ctx.r[11].u32 ) };
	// 8317A6C0: 917F1FAC  stw r11, 0x1fac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8108 as u32), ctx.r[11].u32 ) };
	// 8317A6C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8317A6C8: 4802DAF4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317A6D0 size=100
    let mut pc: u32 = 0x8317A6D0;
    'dispatch: loop {
        match pc {
            0x8317A6D0 => {
    //   block [0x8317A6D0..0x8317A734)
	// 8317A6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317A6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317A6D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8317A6DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317A6E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317A6E4: 817F1784  lwz r11, 0x1784(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6020 as u32) ) } as u64;
	// 8317A6E8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317A6EC: 4BFFFA05  bl 0x8317a0f0
	ctx.lr = 0x8317A6F0;
	sub_8317A0F0(ctx, base);
	// 8317A6F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317A6F4: 419A0028  beq cr6, 0x8317a71c
	if ctx.cr[6].eq {
	pc = 0x8317A71C; continue 'dispatch;
	}
	// 8317A6F8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8317A6FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317A700: 60840D0A  ori r4, r4, 0xd0a
	ctx.r[4].u64 = ctx.r[4].u64 | 3338;
	// 8317A704: 4800CDF5  bl 0x831874f8
	ctx.lr = 0x8317A708;
	sub_831874F8(ctx, base);
	// 8317A708: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317A70C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317A710: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317A714: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317A718: 4E800020  blr
	return;
	// 8317A71C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317A720: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317A724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317A728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317A72C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8317A730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317A738 size=80
    let mut pc: u32 = 0x8317A738;
    'dispatch: loop {
        match pc {
            0x8317A738 => {
    //   block [0x8317A738..0x8317A788)
	// 8317A738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317A73C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8317A740: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317A744: 81431E28  lwz r10, 0x1e28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(7720 as u32) ) } as u64;
	// 8317A748: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8317A74C: 409A0018  bne cr6, 0x8317a764
	if !ctx.cr[6].eq {
	pc = 0x8317A764; continue 'dispatch;
	}
	// 8317A750: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317A754: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317A758: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317A75C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317A760: 4E800020  blr
	return;
	// 8317A764: 4BFFFA25  bl 0x8317a188
	ctx.lr = 0x8317A768;
	sub_8317A188(ctx, base);
	// 8317A768: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317A76C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8317A770: 41990008  bgt cr6, 0x8317a778
	if ctx.cr[6].gt {
	pc = 0x8317A778; continue 'dispatch;
	}
	// 8317A774: 386A08A0  addi r3, r10, 0x8a0
	ctx.r[3].s64 = ctx.r[10].s64 + 2208;
	// 8317A778: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8317A77C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8317A780: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8317A784: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317A788 size=176
    let mut pc: u32 = 0x8317A788;
    'dispatch: loop {
        match pc {
            0x8317A788 => {
    //   block [0x8317A788..0x8317A838)
	// 8317A788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317A78C: 4802D9D9  bl 0x831a8164
	ctx.lr = 0x8317A790;
	sub_831A8130(ctx, base);
	// 8317A790: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317A794: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8317A798: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 8317A79C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8317A7A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317A7A4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8317A7A8: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8317A7AC: 809C0028  lwz r4, 0x28(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(40 as u32) ) } as u64;
	// 8317A7B0: 39640003  addi r11, r4, 3
	ctx.r[11].s64 = ctx.r[4].s64 + 3;
	// 8317A7B4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317A7B8: 41980020  blt cr6, 0x8317a7d8
	if ctx.cr[6].lt {
	pc = 0x8317A7D8; continue 'dispatch;
	}
	// 8317A7BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A7C0: 4BFFF0F1  bl 0x831798b0
	ctx.lr = 0x8317A7C4;
	sub_831798B0(ctx, base);
	// 8317A7C4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317A7C8: 419A0010  beq cr6, 0x8317a7d8
	if ctx.cr[6].eq {
	pc = 0x8317A7D8; continue 'dispatch;
	}
	// 8317A7CC: 909B0000  stw r4, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 8317A7D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317A7D4: 4802D9E0  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
	// 8317A7D8: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 8317A7DC: 4198002C  blt cr6, 0x8317a808
	if ctx.cr[6].lt {
	pc = 0x8317A808; continue 'dispatch;
	}
	// 8317A7E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A7E4: 48014795  bl 0x8318ef78
	ctx.lr = 0x8317A7E8;
	sub_8318EF78(ctx, base);
	// 8317A7E8: 746B000D  andis. r11, r3, 0xd
	ctx.r[11].u64 = ctx.r[3].u64 & 851968;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317A7EC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317A7F0: 409A003C  bne cr6, 0x8317a82c
	if !ctx.cr[6].eq {
	pc = 0x8317A82C; continue 'dispatch;
	}
	// 8317A7F4: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 8317A7F8: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 8317A7FC: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 8317A800: 2F1F0004  cmpwi cr6, r31, 4
	ctx.cr[6].compare_i32(ctx.r[31].s32, 4, &mut ctx.xer);
	// 8317A804: 4098FFDC  bge cr6, 0x8317a7e0
	if !ctx.cr[6].lt {
	pc = 0x8317A7E0; continue 'dispatch;
	}
	// 8317A808: 397FFFFF  addi r11, r31, -1
	ctx.r[11].s64 = ctx.r[31].s64 + -1;
	// 8317A80C: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 8317A810: 4199001C  bgt cr6, 0x8317a82c
	if ctx.cr[6].gt {
	pc = 0x8317A82C; continue 'dispatch;
	}
	// 8317A814: 7C9EFA14  add r4, r30, r31
	ctx.r[4].u64 = ctx.r[30].u64 + ctx.r[31].u64;
	// 8317A818: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 8317A81C: 4BFFF9ED  bl 0x8317a208
	ctx.lr = 0x8317A820;
	sub_8317A208(ctx, base);
	// 8317A820: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317A824: 419A0008  beq cr6, 0x8317a82c
	if ctx.cr[6].eq {
	pc = 0x8317A82C; continue 'dispatch;
	}
	// 8317A828: 7FBDFA14  add r29, r29, r31
	ctx.r[29].u64 = ctx.r[29].u64 + ctx.r[31].u64;
	// 8317A82C: 93BB0000  stw r29, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 8317A830: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317A834: 4802D980  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317A838 size=84
    let mut pc: u32 = 0x8317A838;
    'dispatch: loop {
        match pc {
            0x8317A838 => {
    //   block [0x8317A838..0x8317A88C)
	// 8317A838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317A83C: 4802D92D  bl 0x831a8168
	ctx.lr = 0x8317A840;
	sub_831A8130(ctx, base);
	// 8317A840: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317A844: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317A848: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8317A84C: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 8317A850: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 8317A854: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8317A858: 4BFFFBA9  bl 0x8317a400
	ctx.lr = 0x8317A85C;
	sub_8317A400(ctx, base);
	// 8317A85C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317A860: 2F1F0001  cmpwi cr6, r31, 1
	ctx.cr[6].compare_i32(ctx.r[31].s32, 1, &mut ctx.xer);
	// 8317A864: 409A001C  bne cr6, 0x8317a880
	if !ctx.cr[6].eq {
	pc = 0x8317A880; continue 'dispatch;
	}
	// 8317A868: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8317A86C: 419A0014  beq cr6, 0x8317a880
	if ctx.cr[6].eq {
	pc = 0x8317A880; continue 'dispatch;
	}
	// 8317A870: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 8317A874: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8317A878: 7FC903A6  mtctr r30
	ctx.ctr.u64 = ctx.r[30].u64;
	// 8317A87C: 4E800421  bctrl
	ctx.lr = 0x8317A880;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317A880: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317A884: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8317A888: 4802D930  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317A890(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317A890 size=380
    let mut pc: u32 = 0x8317A890;
    'dispatch: loop {
        match pc {
            0x8317A890 => {
    //   block [0x8317A890..0x8317AA0C)
	// 8317A890: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317A894: 4802D8C9  bl 0x831a815c
	ctx.lr = 0x8317A898;
	sub_831A8130(ctx, base);
	// 8317A898: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317A89C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8317A8A0: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 8317A8A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8317A8A8: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8317A8AC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8317A8B0: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 8317A8B4: 4BFFEB5D  bl 0x83179410
	ctx.lr = 0x8317A8B8;
	sub_83179410(ctx, base);
	// 8317A8B8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317A8BC: 409A0010  bne cr6, 0x8317a8cc
	if !ctx.cr[6].eq {
	pc = 0x8317A8CC; continue 'dispatch;
	}
	// 8317A8C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317A8C4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8317A8C8: 4802D8E4  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 8317A8CC: 83FE1784  lwz r31, 0x1784(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(6020 as u32) ) } as u64;
	// 8317A8D0: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8317A8D4: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8317A8D8: 409A0044  bne cr6, 0x8317a91c
	if !ctx.cr[6].eq {
	pc = 0x8317A91C; continue 'dispatch;
	}
	// 8317A8DC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8317A8E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A8E4: 4BFFFAAD  bl 0x8317a390
	ctx.lr = 0x8317A8E8;
	sub_8317A390(ctx, base);
	// 8317A8E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8317A8EC: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8317A8F0: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317A8F4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8317A8F8: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8317A8FC: 48014505  bl 0x8318ee00
	ctx.lr = 0x8317A900;
	sub_8318EE00(ctx, base);
	// 8317A900: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8317A904: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8317A908: 409A0014  bne cr6, 0x8317a91c
	if !ctx.cr[6].eq {
	pc = 0x8317A91C; continue 'dispatch;
	}
	// 8317A90C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8317A910: 38800049  li r4, 0x49
	ctx.r[4].s64 = 73;
	// 8317A914: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A918: 4BFFEB41  bl 0x83179458
	ctx.lr = 0x8317A91C;
	sub_83179458(ctx, base);
	// 8317A91C: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8317A920: 2F0BFFFF  cmpwi cr6, r11, -1
	ctx.cr[6].compare_i32(ctx.r[11].s32, -1, &mut ctx.xer);
	// 8317A924: 409A0008  bne cr6, 0x8317a92c
	if !ctx.cr[6].eq {
	pc = 0x8317A92C; continue 'dispatch;
	}
	// 8317A928: 93BF002C  stw r29, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[29].u32 ) };
	// 8317A92C: 3880001D  li r4, 0x1d
	ctx.r[4].s64 = 29;
	// 8317A930: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A934: 4BFFEADD  bl 0x83179410
	ctx.lr = 0x8317A938;
	sub_83179410(ctx, base);
	// 8317A938: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8317A93C: 2F1CFFFF  cmpwi cr6, r28, -1
	ctx.cr[6].compare_i32(ctx.r[28].s32, -1, &mut ctx.xer);
	// 8317A940: 419A009C  beq cr6, 0x8317a9dc
	if ctx.cr[6].eq {
	pc = 0x8317A9DC; continue 'dispatch;
	}
	// 8317A944: 38800037  li r4, 0x37
	ctx.r[4].s64 = 55;
	// 8317A948: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317A94C: 4BFFEAC5  bl 0x83179410
	ctx.lr = 0x8317A950;
	sub_83179410(ctx, base);
	// 8317A950: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8317A954: 419A0014  beq cr6, 0x8317a968
	if ctx.cr[6].eq {
	pc = 0x8317A968; continue 'dispatch;
	}
	// 8317A958: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8317A95C: 7F1D5800  cmpw cr6, r29, r11
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[11].s32, &mut ctx.xer);
	// 8317A960: 41980020  blt cr6, 0x8317a980
	if ctx.cr[6].lt {
	pc = 0x8317A980; continue 'dispatch;
	}
	// 8317A964: 48000078  b 0x8317a9dc
	pc = 0x8317A9DC; continue 'dispatch;
	// 8317A968: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 8317A96C: 7D7D5850  subf r11, r29, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[29].s64;
	// 8317A970: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8317A974: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317A978: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317A97C: 419A0060  beq cr6, 0x8317a9dc
	if ctx.cr[6].eq {
	pc = 0x8317A9DC; continue 'dispatch;
	}
	// 8317A980: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8317A984: 7F0BE000  cmpw cr6, r11, r28
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[28].s32, &mut ctx.xer);
	// 8317A988: 419A0054  beq cr6, 0x8317a9dc
	if ctx.cr[6].eq {
	pc = 0x8317A9DC; continue 'dispatch;
	}
	// 8317A98C: 2F1A0004  cmpwi cr6, r26, 4
	ctx.cr[6].compare_i32(ctx.r[26].s32, 4, &mut ctx.xer);
	// 8317A990: 4198004C  blt cr6, 0x8317a9dc
	if ctx.cr[6].lt {
	pc = 0x8317A9DC; continue 'dispatch;
	}
	// 8317A994: 897B0000  lbz r11, 0(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8317A998: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317A99C: 409A0040  bne cr6, 0x8317a9dc
	if !ctx.cr[6].eq {
	pc = 0x8317A9DC; continue 'dispatch;
	}
	// 8317A9A0: 897B0001  lbz r11, 1(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(1 as u32) ) } as u64;
	// 8317A9A4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8317A9A8: 409A0034  bne cr6, 0x8317a9dc
	if !ctx.cr[6].eq {
	pc = 0x8317A9DC; continue 'dispatch;
	}
	// 8317A9AC: 897B0002  lbz r11, 2(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(2 as u32) ) } as u64;
	// 8317A9B0: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 8317A9B4: 409A0028  bne cr6, 0x8317a9dc
	if !ctx.cr[6].eq {
	pc = 0x8317A9DC; continue 'dispatch;
	}
	// 8317A9B8: 897B0003  lbz r11, 3(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(3 as u32) ) } as u64;
	// 8317A9BC: 2B0B00B3  cmplwi cr6, r11, 0xb3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 179 as u32, &mut ctx.xer);
	// 8317A9C0: 419A0018  beq cr6, 0x8317a9d8
	if ctx.cr[6].eq {
	pc = 0x8317A9D8; continue 'dispatch;
	}
	// 8317A9C4: 396BFF48  addi r11, r11, -0xb8
	ctx.r[11].s64 = ctx.r[11].s64 + -184;
	// 8317A9C8: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 8317A9CC: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8317A9D0: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8317A9D4: 419A0008  beq cr6, 0x8317a9dc
	if ctx.cr[6].eq {
	pc = 0x8317A9DC; continue 'dispatch;
	}
	// 8317A9D8: 939F0034  stw r28, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[28].u32 ) };
	// 8317A9DC: 817F0034  lwz r11, 0x34(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(52 as u32) ) } as u64;
	// 8317A9E0: 93BF0024  stw r29, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[29].u32 ) };
	// 8317A9E4: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 8317A9E8: 409AFED8  bne cr6, 0x8317a8c0
	if !ctx.cr[6].eq {
	pc = 0x8317A8C0; continue 'dispatch;
	}
	// 8317A9EC: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 8317A9F0: 809E1790  lwz r4, 0x1790(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(6032 as u32) ) } as u64;
	// 8317A9F4: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8317A9F8: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 8317A9FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8317AA00: 4BFFEF81  bl 0x83179980
	ctx.lr = 0x8317AA04;
	sub_83179980(ctx, base);
	// 8317AA04: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 8317AA08: 4802D7A4  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8317AA10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8317AA10 size=176
    let mut pc: u32 = 0x8317AA10;
    'dispatch: loop {
        match pc {
            0x8317AA10 => {
    //   block [0x8317AA10..0x8317AAC0)
	// 8317AA10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8317AA14: 4802D749  bl 0x831a815c
	ctx.lr = 0x8317AA18;
	sub_831A8130(ctx, base);
	// 8317AA18: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8317AA1C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8317AA20: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8317AA24: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8317AA28: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8317AA2C: 809F1798  lwz r4, 0x1798(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(6040 as u32) ) } as u64;
	// 8317AA30: 2F040008  cmpwi cr6, r4, 8
	ctx.cr[6].compare_i32(ctx.r[4].s32, 8, &mut ctx.xer);
	// 8317AA34: 409A0010  bne cr6, 0x8317aa44
	if !ctx.cr[6].eq {
	pc = 0x8317AA44; continue 'dispatch;
	}
	// 8317AA38: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8317AA3C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8317AA40: 4802D76C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
	// 8317AA44: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8317AA48: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8317AA4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317AA50: 480131A9  bl 0x8318dbf8
	ctx.lr = 0x8317AA54;
	sub_8318DBF8(ctx, base);
	// 8317AA54: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8317AA58: 83A10054  lwz r29, 0x54(r1)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8317AA5C: 83810058  lwz r28, 0x58(r1)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 8317AA60: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8317AA64: 8321005C  lwz r25, 0x5c(r1)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8317AA68: 419AFFD0  beq cr6, 0x8317aa38
	if ctx.cr[6].eq {
	pc = 0x8317AA38; continue 'dispatch;
	}
	// 8317AA6C: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 8317AA70: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8317AA74: 4BFFF98D  bl 0x8317a400
	ctx.lr = 0x8317AA78;
	sub_8317A400(ctx, base);
	// 8317AA78: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8317AA7C: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 8317AA80: 409A0034  bne cr6, 0x8317aab4
	if !ctx.cr[6].eq {
	pc = 0x8317AAB4; continue 'dispatch;
	}
	// 8317AA84: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8317AA88: 419A0014  beq cr6, 0x8317aa9c
	if ctx.cr[6].eq {
	pc = 0x8317AA9C; continue 'dispatch;
	}
	// 8317AA8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317AA90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8317AA94: 7FA903A6  mtctr r29
	ctx.ctr.u64 = ctx.r[29].u64;
	// 8317AA98: 4E800421  bctrl
	ctx.lr = 0x8317AA9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317AA9C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 8317AAA0: 419A0014  beq cr6, 0x8317aab4
	if ctx.cr[6].eq {
	pc = 0x8317AAB4; continue 'dispatch;
	}
	// 8317AAA4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8317AAA8: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 8317AAAC: 7F8903A6  mtctr r28
	ctx.ctr.u64 = ctx.r[28].u64;
	// 8317AAB0: 4E800421  bctrl
	ctx.lr = 0x8317AAB4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8317AAB4: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 8317AAB8: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8317AABC: 4802D6F0  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


