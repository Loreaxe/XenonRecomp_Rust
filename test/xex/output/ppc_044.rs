pub fn sub_824328D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824328D0 size=24
    let mut pc: u32 = 0x824328D0;
    'dispatch: loop {
        match pc {
            0x824328D0 => {
    //   block [0x824328D0..0x824328E8)
	// 824328D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824328D4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824328D8: 814B01F0  lwz r10, 0x1f0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(496 as u32) ) } as u64;
	// 824328DC: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 824328E0: 419A0008  beq cr6, 0x824328e8
	if ctx.cr[6].eq {
		sub_824328E8(ctx, base);
		return;
	}
	// 824328E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824328E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824328E8 size=84
    let mut pc: u32 = 0x824328E8;
    'dispatch: loop {
        match pc {
            0x824328E8 => {
    //   block [0x824328E8..0x82432910)
	// 824328E8: 814B0518  lwz r10, 0x518(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1304 as u32) ) } as u64;
	// 824328EC: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 824328F0: 409A0020  bne cr6, 0x82432910
	if !ctx.cr[6].eq {
	pc = 0x82432910; continue 'dispatch;
	}
	// 824328F4: 814B0534  lwz r10, 0x534(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1332 as u32) ) } as u64;
	// 824328F8: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824328FC: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82432900: 814A01F4  lwz r10, 0x1f4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(500 as u32) ) } as u64;
	// 82432904: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82432908: 409A0008  bne cr6, 0x82432910
	if !ctx.cr[6].eq {
	pc = 0x82432910; continue 'dispatch;
	}
	// 8243290C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82432910; continue 'dispatch;
            }
            0x82432910 => {
    //   block [0x82432910..0x8243293C)
	// 82432910: 814B053C  lwz r10, 0x53c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1340 as u32) ) } as u64;
	// 82432914: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82432918: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 8243291C: 814B0558  lwz r10, 0x558(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1368 as u32) ) } as u64;
	// 82432920: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82432924: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82432928: 816B01F4  lwz r11, 0x1f4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(500 as u32) ) } as u64;
	// 8243292C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82432930: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82432934: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82432938: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432940 size=284
    let mut pc: u32 = 0x82432940;
    'dispatch: loop {
        match pc {
            0x82432940 => {
    //   block [0x82432940..0x82432974)
	// 82432940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432944: 48102779  bl 0x825350bc
	ctx.lr = 0x82432948;
	sub_82535080(ctx, base);
	// 82432948: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243294C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432950: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82432954: 4800070D  bl 0x82433060
	ctx.lr = 0x82432958;
	sub_82433060(ctx, base);
	// 82432958: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243295C: 419A0018  beq cr6, 0x82432974
	if ctx.cr[6].eq {
	pc = 0x82432974; continue 'dispatch;
	}
	// 82432960: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432964: 386B49C0  addi r3, r11, 0x49c0
	ctx.r[3].s64 = ctx.r[11].s64 + 18880;
	// 82432968: 48004761  bl 0x824370c8
	ctx.lr = 0x8243296C;
	sub_824370C8(ctx, base);
	// 8243296C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82432970: 4810279C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82432974 => {
    //   block [0x82432974..0x82432990)
	// 82432974: 897F0082  lbz r11, 0x82(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(130 as u32) ) } as u64;
	// 82432978: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243297C: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 82432980: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82432984: 409A000C  bne cr6, 0x82432990
	if !ctx.cr[6].eq {
	pc = 0x82432990; continue 'dispatch;
	}
	// 82432988: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 8243298C: 419A00C8  beq cr6, 0x82432a54
	if ctx.cr[6].eq {
	pc = 0x82432A54; continue 'dispatch;
	}
	pc = 0x82432990; continue 'dispatch;
            }
            0x82432990 => {
    //   block [0x82432990..0x824329B0)
	// 82432990: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82432994: 409A001C  bne cr6, 0x824329b0
	if !ctx.cr[6].eq {
	pc = 0x824329B0; continue 'dispatch;
	}
	// 82432998: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 8243299C: 409A0014  bne cr6, 0x824329b0
	if !ctx.cr[6].eq {
	pc = 0x824329B0; continue 'dispatch;
	}
	// 824329A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824329A4: 4BFFFF2D  bl 0x824328d0
	ctx.lr = 0x824329A8;
	sub_824328D0(ctx, base);
	// 824329A8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824329AC: 409A00A8  bne cr6, 0x82432a54
	if !ctx.cr[6].eq {
	pc = 0x82432A54; continue 'dispatch;
	}
	pc = 0x824329B0; continue 'dispatch;
            }
            0x824329B0 => {
    //   block [0x824329B0..0x824329EC)
	// 824329B0: 48003F29  bl 0x824368d8
	ctx.lr = 0x824329B4;
	sub_824368D8(ctx, base);
	// 824329B4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824329B8: 409A003C  bne cr6, 0x824329f4
	if !ctx.cr[6].eq {
	pc = 0x824329F4; continue 'dispatch;
	}
	// 824329BC: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 824329C0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824329C4: 409A0030  bne cr6, 0x824329f4
	if !ctx.cr[6].eq {
	pc = 0x824329F4; continue 'dispatch;
	}
	// 824329C8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824329CC: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 824329D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824329D4: 48009DED  bl 0x8243c7c0
	ctx.lr = 0x824329D8;
	sub_8243C7C0(ctx, base);
	// 824329D8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824329DC: 409A0010  bne cr6, 0x824329ec
	if !ctx.cr[6].eq {
	pc = 0x824329EC; continue 'dispatch;
	}
	// 824329E0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824329E4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824329E8: 409A000C  bne cr6, 0x824329f4
	if !ctx.cr[6].eq {
	pc = 0x824329F4; continue 'dispatch;
	}
	pc = 0x824329EC; continue 'dispatch;
            }
            0x824329EC => {
    //   block [0x824329EC..0x824329F4)
	// 824329EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824329F0: 480099A1  bl 0x8243c390
	ctx.lr = 0x824329F4;
	sub_8243C390(ctx, base);
	pc = 0x824329F4; continue 'dispatch;
            }
            0x824329F4 => {
    //   block [0x824329F4..0x82432A24)
	// 824329F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824329F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824329FC: 48006865  bl 0x82439260
	ctx.lr = 0x82432A00;
	sub_82439260(ctx, base);
	// 82432A00: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82432A04: 419A0034  beq cr6, 0x82432a38
	if ctx.cr[6].eq {
	pc = 0x82432A38; continue 'dispatch;
	}
	// 82432A08: 3860FECA  li r3, -0x136
	ctx.r[3].s64 = -310;
	// 82432A0C: 48003FBD  bl 0x824369c8
	ctx.lr = 0x82432A10;
	sub_824369C8(ctx, base);
	// 82432A10: 2F1D0001  cmpwi cr6, r29, 1
	ctx.cr[6].compare_i32(ctx.r[29].s32, 1, &mut ctx.xer);
	// 82432A14: 409A0010  bne cr6, 0x82432a24
	if !ctx.cr[6].eq {
	pc = 0x82432A24; continue 'dispatch;
	}
	// 82432A18: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82432A1C: 388BB5DC  addi r4, r11, -0x4a24
	ctx.r[4].s64 = ctx.r[11].s64 + -18980;
	// 82432A20: 4800000C  b 0x82432a2c
	pc = 0x82432A2C; continue 'dispatch;
            }
            0x82432A24 => {
    //   block [0x82432A24..0x82432A2C)
	// 82432A24: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82432A28: 388BB5E0  addi r4, r11, -0x4a20
	ctx.r[4].s64 = ctx.r[11].s64 + -18976;
	pc = 0x82432A2C; continue 'dispatch;
            }
            0x82432A2C => {
    //   block [0x82432A2C..0x82432A38)
	// 82432A2C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432A30: 386B499C  addi r3, r11, 0x499c
	ctx.r[3].s64 = ctx.r[11].s64 + 18844;
	// 82432A34: 48004695  bl 0x824370c8
	ctx.lr = 0x82432A38;
	sub_824370C8(ctx, base);
	pc = 0x82432A38; continue 'dispatch;
            }
            0x82432A38 => {
    //   block [0x82432A38..0x82432A54)
	// 82432A38: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82432A3C: 387F0518  addi r3, r31, 0x518
	ctx.r[3].s64 = ctx.r[31].s64 + 1304;
	// 82432A40: 48008CE1  bl 0x8243b720
	ctx.lr = 0x82432A44;
	sub_8243B720(ctx, base);
	// 82432A44: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82432A48: 387F053C  addi r3, r31, 0x53c
	ctx.r[3].s64 = ctx.r[31].s64 + 1340;
	// 82432A4C: 48008CD5  bl 0x8243b720
	ctx.lr = 0x82432A50;
	sub_8243B720(ctx, base);
	// 82432A50: 9BBF0082  stb r29, 0x82(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(130 as u32), ctx.r[29].u8 ) };
	pc = 0x82432A54; continue 'dispatch;
            }
            0x82432A54 => {
    //   block [0x82432A54..0x82432A5C)
	// 82432A54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82432A58: 481026B4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82432A60 size=80
    let mut pc: u32 = 0x82432A60;
    'dispatch: loop {
        match pc {
            0x82432A60 => {
    //   block [0x82432A60..0x82432AB0)
	// 82432A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432A68: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432A6C: 80A3045C  lwz r5, 0x45c(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1116 as u32) ) } as u64;
	// 82432A70: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82432A74: 7CAB07B4  extsw r11, r5
	ctx.r[11].s64 = ctx.r[5].s32 as i64;
	// 82432A78: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 82432A7C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82432A80: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82432A84: FDA0069C  fcfid f13, f0
	ctx.f[13].f64 = (ctx.f[0].s64 as f64);
	// 82432A88: C80B2F58  lfd f0, 0x2f58(r11)
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(12120 as u32) ) };
	// 82432A8C: FC0D0032  fmul f0, f13, f0
	ctx.f[0].f64 = ctx.f[13].f64 * ctx.f[0].f64;
	// 82432A90: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82432A94: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 82432A98: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82432A9C: 4800057D  bl 0x82433018
	ctx.lr = 0x82432AA0;
	sub_82433018(ctx, base);
	// 82432AA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432AAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432AB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432AB0 size=72
    let mut pc: u32 = 0x82432AB0;
    'dispatch: loop {
        match pc {
            0x82432AB0 => {
    //   block [0x82432AB0..0x82432AE4)
	// 82432AB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432AB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432AB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432ABC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432AC0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432AC4: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82432AC8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82432ACC: 419A0018  beq cr6, 0x82432ae4
	if ctx.cr[6].eq {
	pc = 0x82432AE4; continue 'dispatch;
	}
	// 82432AD0: 48008949  bl 0x8243b418
	ctx.lr = 0x82432AD4;
	sub_8243B418(ctx, base);
	// 82432AD4: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82432AD8: 409A000C  bne cr6, 0x82432ae4
	if !ctx.cr[6].eq {
	pc = 0x82432AE4; continue 'dispatch;
	}
	// 82432ADC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432AE0: 4BFFFC61  bl 0x82432740
	ctx.lr = 0x82432AE4;
	sub_82432740(ctx, base);
	pc = 0x82432AE4; continue 'dispatch;
            }
            0x82432AE4 => {
    //   block [0x82432AE4..0x82432AF8)
	// 82432AE4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432AF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432AF8 size=164
    let mut pc: u32 = 0x82432AF8;
    'dispatch: loop {
        match pc {
            0x82432AF8 => {
    //   block [0x82432AF8..0x82432B1C)
	// 82432AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432B00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82432B04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432B08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432B0C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82432B10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432B14: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82432B18: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	pc = 0x82432B1C; continue 'dispatch;
            }
            0x82432B1C => {
    //   block [0x82432B1C..0x82432B64)
	// 82432B1C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82432B20: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82432B24: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82432B28: 409AFFF4  bne cr6, 0x82432b1c
	if !ctx.cr[6].eq {
	pc = 0x82432B1C; continue 'dispatch;
	}
	// 82432B2C: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82432B30: 815F043C  lwz r10, 0x43c(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1084 as u32) ) } as u64;
	// 82432B34: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82432B38: 556B003E  slwi r11, r11, 0
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82432B3C: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82432B40: 40990024  ble cr6, 0x82432b64
	if !ctx.cr[6].gt {
	pc = 0x82432B64; continue 'dispatch;
	}
	// 82432B44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432B48: 386B49E8  addi r3, r11, 0x49e8
	ctx.r[3].s64 = ctx.r[11].s64 + 18920;
	// 82432B4C: 4800457D  bl 0x824370c8
	ctx.lr = 0x82432B50;
	sub_824370C8(ctx, base);
	// 82432B50: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82432B54: 80BF043C  lwz r5, 0x43c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1084 as u32) ) } as u64;
	// 82432B58: 807F0438  lwz r3, 0x438(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 82432B5C: 48100065  bl 0x82532bc0
	ctx.lr = 0x82432B60;
	sub_82532BC0(ctx, base);
	// 82432B60: 48000024  b 0x82432b84
	pc = 0x82432B84; continue 'dispatch;
            }
            0x82432B64 => {
    //   block [0x82432B64..0x82432B6C)
	// 82432B64: 815F0438  lwz r10, 0x438(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 82432B68: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	pc = 0x82432B6C; continue 'dispatch;
            }
            0x82432B6C => {
    //   block [0x82432B6C..0x82432B84)
	// 82432B6C: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82432B70: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82432B74: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82432B78: 992A0000  stb r9, 0(r10)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82432B7C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82432B80: 409AFFEC  bne cr6, 0x82432b6c
	if !ctx.cr[6].eq {
	pc = 0x82432B6C; continue 'dispatch;
	}
	pc = 0x82432B84; continue 'dispatch;
            }
            0x82432B84 => {
    //   block [0x82432B84..0x82432B9C)
	// 82432B84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432B88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432B8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432B90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82432B94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432B98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432BA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432BA0 size=140
    let mut pc: u32 = 0x82432BA0;
    'dispatch: loop {
        match pc {
            0x82432BA0 => {
    //   block [0x82432BA0..0x82432C0C)
	// 82432BA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432BA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432BA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432BAC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432BB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432BB4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82432BB8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82432BBC: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 82432BC0: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82432BC4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82432BC8: 80BF0438  lwz r5, 0x438(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 82432BCC: 4BFE52DD  bl 0x82417ea8
	ctx.lr = 0x82432BD0;
	sub_82417EA8(ctx, base);
	// 82432BD0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82432BD4: 409A0038  bne cr6, 0x82432c0c
	if !ctx.cr[6].eq {
	pc = 0x82432C0C; continue 'dispatch;
	}
	// 82432BD8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82432BDC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82432BE0: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82432BE4: 81210058  lwz r9, 0x58(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82432BE8: 917F0444  stw r11, 0x444(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1092 as u32), ctx.r[11].u32 ) };
	// 82432BEC: 915F0448  stw r10, 0x448(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1096 as u32), ctx.r[10].u32 ) };
	// 82432BF0: 913F044C  stw r9, 0x44c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1100 as u32), ctx.r[9].u32 ) };
	// 82432BF4: 911F0440  stw r8, 0x440(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1088 as u32), ctx.r[8].u32 ) };
	// 82432BF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432BFC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432C00: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432C04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432C08: 4E800020  blr
	return;
            }
            0x82432C0C => {
    //   block [0x82432C0C..0x82432C2C)
	// 82432C0C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432C10: 386B4A04  addi r3, r11, 0x4a04
	ctx.r[3].s64 = ctx.r[11].s64 + 18948;
	// 82432C14: 480044B5  bl 0x824370c8
	ctx.lr = 0x82432C18;
	sub_824370C8(ctx, base);
	// 82432C18: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432C1C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432C20: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432C24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432C30 size=132
    let mut pc: u32 = 0x82432C30;
    'dispatch: loop {
        match pc {
            0x82432C30 => {
    //   block [0x82432C30..0x82432C70)
	// 82432C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432C38: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432C3C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432C40: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432C44: 4800041D  bl 0x82433060
	ctx.lr = 0x82432C48;
	sub_82433060(ctx, base);
	// 82432C48: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82432C4C: 419A0024  beq cr6, 0x82432c70
	if ctx.cr[6].eq {
	pc = 0x82432C70; continue 'dispatch;
	}
	// 82432C50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432C54: 386B4A2C  addi r3, r11, 0x4a2c
	ctx.r[3].s64 = ctx.r[11].s64 + 18988;
	// 82432C58: 48004471  bl 0x824370c8
	ctx.lr = 0x82432C5C;
	sub_824370C8(ctx, base);
	// 82432C5C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432C60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432C64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432C68: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432C6C: 4E800020  blr
	return;
            }
            0x82432C70 => {
    //   block [0x82432C70..0x82432CA0)
	// 82432C70: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82432C74: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82432C78: 419A0028  beq cr6, 0x82432ca0
	if ctx.cr[6].eq {
	pc = 0x82432CA0; continue 'dispatch;
	}
	// 82432C7C: 817F05AC  lwz r11, 0x5ac(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1452 as u32) ) } as u64;
	// 82432C80: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82432C84: 419A001C  beq cr6, 0x82432ca0
	if ctx.cr[6].eq {
	pc = 0x82432CA0; continue 'dispatch;
	}
	// 82432C88: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82432C8C: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82432C90: 917F05AC  stw r11, 0x5ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1452 as u32), ctx.r[11].u32 ) };
	// 82432C94: 480070C5  bl 0x82439d58
	ctx.lr = 0x82432C98;
	sub_82439D58(ctx, base);
	// 82432C98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432C9C: 480092A5  bl 0x8243bf40
	ctx.lr = 0x82432CA0;
	sub_8243BF40(ctx, base);
	pc = 0x82432CA0; continue 'dispatch;
            }
            0x82432CA0 => {
    //   block [0x82432CA0..0x82432CB4)
	// 82432CA0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432CA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432CA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432CAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432CB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432CB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432CB8 size=92
    let mut pc: u32 = 0x82432CB8;
    'dispatch: loop {
        match pc {
            0x82432CB8 => {
    //   block [0x82432CB8..0x82432CEC)
	// 82432CB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432CBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432CC0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432CC4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432CC8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432CCC: 480092B5  bl 0x8243bf80
	ctx.lr = 0x82432CD0;
	sub_8243BF80(ctx, base);
	// 82432CD0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82432CD4: 409A0018  bne cr6, 0x82432cec
	if !ctx.cr[6].eq {
	pc = 0x82432CEC; continue 'dispatch;
	}
	// 82432CD8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432CDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432CE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432CE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432CE8: 4E800020  blr
	return;
            }
            0x82432CEC => {
    //   block [0x82432CEC..0x82432D14)
	// 82432CEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432CF0: 48009171  bl 0x8243be60
	ctx.lr = 0x82432CF4;
	sub_8243BE60(ctx, base);
	// 82432CF4: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82432CF8: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82432CFC: 69630001  xori r3, r11, 1
	ctx.r[3].u64 = ctx.r[11].u64 ^ 1;
	// 82432D00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432D04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432D08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432D0C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432D10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432D18 size=300
    let mut pc: u32 = 0x82432D18;
    'dispatch: loop {
        match pc {
            0x82432D18 => {
    //   block [0x82432D18..0x82432D70)
	// 82432D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432D1C: 4810239D  bl 0x825350b8
	ctx.lr = 0x82432D20;
	sub_82535080(ctx, base);
	// 82432D20: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432D24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432D28: 48003A61  bl 0x82436788
	ctx.lr = 0x82432D2C;
	sub_82436788(ctx, base);
	// 82432D2C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82432D30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82432D34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432D38: 939F05AC  stw r28, 0x5ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1452 as u32), ctx.r[28].u32 ) };
	// 82432D3C: 48000A0D  bl 0x82433748
	ctx.lr = 0x82432D40;
	sub_82433748(ctx, base);
	// 82432D40: 817F0048  lwz r11, 0x48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82432D44: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82432D48: 419A007C  beq cr6, 0x82432dc4
	if ctx.cr[6].eq {
	pc = 0x82432DC4; continue 'dispatch;
	}
	// 82432D4C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432D50: 48002639  bl 0x82435388
	ctx.lr = 0x82432D54;
	sub_82435388(ctx, base);
	// 82432D54: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82432D58: 419A0018  beq cr6, 0x82432d70
	if ctx.cr[6].eq {
	pc = 0x82432D70; continue 'dispatch;
	}
	// 82432D5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432D60: 386B4A88  addi r3, r11, 0x4a88
	ctx.r[3].s64 = ctx.r[11].s64 + 19080;
	// 82432D64: 48004365  bl 0x824370c8
	ctx.lr = 0x82432D68;
	sub_824370C8(ctx, base);
	// 82432D68: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82432D6C: 4810239C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82432D70 => {
    //   block [0x82432D70..0x82432DB4)
	// 82432D70: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82432D74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432D78: 48008A49  bl 0x8243b7c0
	ctx.lr = 0x82432D7C;
	sub_8243B7C0(ctx, base);
	// 82432D7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82432D80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432D84: 48008A3D  bl 0x8243b7c0
	ctx.lr = 0x82432D88;
	sub_8243B7C0(ctx, base);
	// 82432D88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432D8C: 4BFFF35D  bl 0x824320e8
	ctx.lr = 0x82432D90;
	sub_824320E8(ctx, base);
	// 82432D90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432D94: 4BFFF2CD  bl 0x82432060
	ctx.lr = 0x82432D98;
	sub_82432060(ctx, base);
	// 82432D98: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82432D9C: 419A0018  beq cr6, 0x82432db4
	if ctx.cr[6].eq {
	pc = 0x82432DB4; continue 'dispatch;
	}
	// 82432DA0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432DA4: 386B4A5C  addi r3, r11, 0x4a5c
	ctx.r[3].s64 = ctx.r[11].s64 + 19036;
	// 82432DA8: 48004321  bl 0x824370c8
	ctx.lr = 0x82432DAC;
	sub_824370C8(ctx, base);
	// 82432DAC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82432DB0: 48102358  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82432DB4 => {
    //   block [0x82432DB4..0x82432DC4)
	// 82432DB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432DB8: 4BFFF311  bl 0x824320c8
	ctx.lr = 0x82432DBC;
	sub_824320C8(ctx, base);
	// 82432DBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432DC0: 48001651  bl 0x82434410
	ctx.lr = 0x82432DC4;
	sub_82434410(ctx, base);
	pc = 0x82432DC4; continue 'dispatch;
            }
            0x82432DC4 => {
    //   block [0x82432DC4..0x82432E44)
	// 82432DC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432DC8: 4BFFF9B1  bl 0x82432778
	ctx.lr = 0x82432DCC;
	sub_82432778(ctx, base);
	// 82432DCC: 4BFFF8C5  bl 0x82432690
	ctx.lr = 0x82432DD0;
	sub_82432690(ctx, base);
	// 82432DD0: 897F0082  lbz r11, 0x82(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(130 as u32) ) } as u64;
	// 82432DD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432DD8: 7D640774  extsb r4, r11
	ctx.r[4].s64 = ctx.r[11].s8 as i64;
	// 82432DDC: 4BFFFB65  bl 0x82432940
	ctx.lr = 0x82432DE0;
	sub_82432940(ctx, base);
	// 82432DE0: 3BDF0518  addi r30, r31, 0x518
	ctx.r[30].s64 = ctx.r[31].s64 + 1304;
	// 82432DE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82432DE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82432DEC: 48008935  bl 0x8243b720
	ctx.lr = 0x82432DF0;
	sub_8243B720(ctx, base);
	// 82432DF0: 3BBF053C  addi r29, r31, 0x53c
	ctx.r[29].s64 = ctx.r[31].s64 + 1340;
	// 82432DF4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82432DF8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82432DFC: 48008925  bl 0x8243b720
	ctx.lr = 0x82432E00;
	sub_8243B720(ctx, base);
	// 82432E00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82432E04: 480087FD  bl 0x8243b600
	ctx.lr = 0x82432E08;
	sub_8243B600(ctx, base);
	// 82432E08: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82432E0C: 480087F5  bl 0x8243b600
	ctx.lr = 0x82432E10;
	sub_8243B600(ctx, base);
	// 82432E10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432E14: 809F007C  lwz r4, 0x7c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 82432E18: 480001A9  bl 0x82432fc0
	ctx.lr = 0x82432E1C;
	sub_82432FC0(ctx, base);
	// 82432E1C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432E20: 809F0078  lwz r4, 0x78(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 82432E24: 480001AD  bl 0x82432fd0
	ctx.lr = 0x82432E28;
	sub_82432FD0(ctx, base);
	// 82432E28: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82432E2C: 939F0094  stw r28, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[28].u32 ) };
	// 82432E30: 9B9F0081  stb r28, 0x81(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(129 as u32), ctx.r[28].u8 ) };
	// 82432E34: 939F056C  stw r28, 0x56c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1388 as u32), ctx.r[28].u32 ) };
	// 82432E38: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82432E3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82432E40: 481022C8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432E48 size=140
    let mut pc: u32 = 0x82432E48;
    'dispatch: loop {
        match pc {
            0x82432E48 => {
    //   block [0x82432E48..0x82432E80)
	// 82432E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432E4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432E50: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82432E54: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432E58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432E60: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82432E64: 480001FD  bl 0x82433060
	ctx.lr = 0x82432E68;
	sub_82433060(ctx, base);
	// 82432E68: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82432E6C: 419A0014  beq cr6, 0x82432e80
	if ctx.cr[6].eq {
	pc = 0x82432E80; continue 'dispatch;
	}
	// 82432E70: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432E74: 386B4AB0  addi r3, r11, 0x4ab0
	ctx.r[3].s64 = ctx.r[11].s64 + 19120;
	// 82432E78: 48004251  bl 0x824370c8
	ctx.lr = 0x82432E7C;
	sub_824370C8(ctx, base);
	// 82432E7C: 48000040  b 0x82432ebc
	pc = 0x82432EBC; continue 'dispatch;
            }
            0x82432E80 => {
    //   block [0x82432E80..0x82432EBC)
	// 82432E80: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82432E84: 48009135  bl 0x8243bfb8
	ctx.lr = 0x82432E88;
	sub_8243BFB8(ctx, base);
	// 82432E88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432E8C: 4BFFF8FD  bl 0x82432788
	ctx.lr = 0x82432E90;
	sub_82432788(ctx, base);
	// 82432E90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82432E94: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82432E98: 93DF0450  stw r30, 0x450(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1104 as u32), ctx.r[30].u32 ) };
	// 82432E9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432EA0: 917F0468  stw r11, 0x468(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1128 as u32), ctx.r[11].u32 ) };
	// 82432EA4: 915F0464  stw r10, 0x464(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1124 as u32), ctx.r[10].u32 ) };
	// 82432EA8: 917F046C  stw r11, 0x46c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1132 as u32), ctx.r[11].u32 ) };
	// 82432EAC: 917F0470  stw r11, 0x470(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1136 as u32), ctx.r[11].u32 ) };
	// 82432EB0: 4BFFFE69  bl 0x82432d18
	ctx.lr = 0x82432EB4;
	sub_82432D18(ctx, base);
	// 82432EB4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82432EB8: 48009101  bl 0x8243bfb8
	ctx.lr = 0x82432EBC;
	sub_8243BFB8(ctx, base);
	pc = 0x82432EBC; continue 'dispatch;
            }
            0x82432EBC => {
    //   block [0x82432EBC..0x82432ED4)
	// 82432EBC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432EC0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432EC4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432EC8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82432ECC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432ED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432ED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432ED8 size=56
    let mut pc: u32 = 0x82432ED8;
    'dispatch: loop {
        match pc {
            0x82432ED8 => {
    //   block [0x82432ED8..0x82432F10)
	// 82432ED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432EDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432EE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432EE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432EE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432EEC: 480041B5  bl 0x824370a0
	ctx.lr = 0x82432EF0;
	sub_824370A0(ctx, base);
	// 82432EF0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432EF4: 4BFFF965  bl 0x82432858
	ctx.lr = 0x82432EF8;
	sub_82432858(ctx, base);
	// 82432EF8: 480041B9  bl 0x824370b0
	ctx.lr = 0x82432EFC;
	sub_824370B0(ctx, base);
	// 82432EFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82432F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432F08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432F0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432F10 size=72
    let mut pc: u32 = 0x82432F10;
    'dispatch: loop {
        match pc {
            0x82432F10 => {
    //   block [0x82432F10..0x82432F58)
	// 82432F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82432F18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82432F1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82432F20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432F24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432F28: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82432F2C: 48004175  bl 0x824370a0
	ctx.lr = 0x82432F30;
	sub_824370A0(ctx, base);
	// 82432F30: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82432F34: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432F38: 4BFFFF11  bl 0x82432e48
	ctx.lr = 0x82432F3C;
	sub_82432E48(ctx, base);
	// 82432F3C: 48004175  bl 0x824370b0
	ctx.lr = 0x82432F40;
	sub_824370B0(ctx, base);
	// 82432F40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432F44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82432F48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82432F4C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82432F50: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82432F54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82432F58 size=100
    let mut pc: u32 = 0x82432F58;
    'dispatch: loop {
        match pc {
            0x82432F58 => {
    //   block [0x82432F58..0x82432F90)
	// 82432F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82432F5C: 48102161  bl 0x825350bc
	ctx.lr = 0x82432F60;
	sub_82535080(ctx, base);
	// 82432F60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82432F64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82432F68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82432F6C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82432F70: 480000F1  bl 0x82433060
	ctx.lr = 0x82432F74;
	sub_82433060(ctx, base);
	// 82432F74: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82432F78: 419A0018  beq cr6, 0x82432f90
	if ctx.cr[6].eq {
	pc = 0x82432F90; continue 'dispatch;
	}
	// 82432F7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82432F80: 386B4ADC  addi r3, r11, 0x4adc
	ctx.r[3].s64 = ctx.r[11].s64 + 19164;
	// 82432F84: 48004145  bl 0x824370c8
	ctx.lr = 0x82432F88;
	sub_824370C8(ctx, base);
	// 82432F88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432F8C: 48102180  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82432F90 => {
    //   block [0x82432F90..0x82432FBC)
	// 82432F90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432F94: 480003ED  bl 0x82433380
	ctx.lr = 0x82432F98;
	sub_82433380(ctx, base);
	// 82432F98: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82432F9C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432FA0: 4BFFFF71  bl 0x82432f10
	ctx.lr = 0x82432FA4;
	sub_82432F10(ctx, base);
	// 82432FA4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82432FA8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82432FAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82432FB0: 4BFFFBF1  bl 0x82432ba0
	ctx.lr = 0x82432FB4;
	sub_82432BA0(ctx, base);
	// 82432FB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82432FB8: 48102154  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432FC0 size=16
    let mut pc: u32 = 0x82432FC0;
    'dispatch: loop {
        match pc {
            0x82432FC0 => {
    //   block [0x82432FC0..0x82432FD0)
	// 82432FC0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82432FC4: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82432FC8: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82432FCC: 4800992C  b 0x8243c8f8
	sub_8243C8F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432FD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432FD0 size=16
    let mut pc: u32 = 0x82432FD0;
    'dispatch: loop {
        match pc {
            0x82432FD0 => {
    //   block [0x82432FD0..0x82432FE0)
	// 82432FD0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82432FD4: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82432FD8: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82432FDC: 4800991C  b 0x8243c8f8
	sub_8243C8F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432FE0 size=24
    let mut pc: u32 = 0x82432FE0;
    'dispatch: loop {
        match pc {
            0x82432FE0 => {
    //   block [0x82432FE0..0x82432FF0)
	// 82432FE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82432FE4: 419A000C  beq cr6, 0x82432ff0
	if ctx.cr[6].eq {
	pc = 0x82432FF0; continue 'dispatch;
	}
	// 82432FE8: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82432FEC: 48000008  b 0x82432ff4
	pc = 0x82432FF4; continue 'dispatch;
            }
            0x82432FF0 => {
    //   block [0x82432FF0..0x82432FF4)
	// 82432FF0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82432FF4; continue 'dispatch;
            }
            0x82432FF4 => {
    //   block [0x82432FF4..0x82432FF8)
	// 82432FF4: 48009904  b 0x8243c8f8
	sub_8243C8F8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82432FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82432FF8 size=24
    let mut pc: u32 = 0x82432FF8;
    'dispatch: loop {
        match pc {
            0x82432FF8 => {
    //   block [0x82432FF8..0x82433008)
	// 82432FF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82432FFC: 419A000C  beq cr6, 0x82433008
	if ctx.cr[6].eq {
	pc = 0x82433008; continue 'dispatch;
	}
	// 82433000: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82433004: 48000008  b 0x8243300c
	pc = 0x8243300C; continue 'dispatch;
            }
            0x82433008 => {
    //   block [0x82433008..0x8243300C)
	// 82433008: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8243300C; continue 'dispatch;
            }
            0x8243300C => {
    //   block [0x8243300C..0x82433010)
	// 8243300C: 480097B4  b 0x8243c7c0
	sub_8243C7C0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433010 size=8
    let mut pc: u32 = 0x82433010;
    'dispatch: loop {
        match pc {
            0x82433010 => {
    //   block [0x82433010..0x82433018)
	// 82433010: 8063004C  lwz r3, 0x4c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82433014: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433018 size=72
    let mut pc: u32 = 0x82433018;
    'dispatch: loop {
        match pc {
            0x82433018 => {
    //   block [0x82433018..0x82433060)
	// 82433018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243301C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433020: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82433024: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243302C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433030: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82433034: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82433038: 48008321  bl 0x8243b358
	ctx.lr = 0x8243303C;
	sub_8243B358(ctx, base);
	// 8243303C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82433040: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433044: 480006ED  bl 0x82433730
	ctx.lr = 0x82433048;
	sub_82433730(ctx, base);
	// 82433048: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243304C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433050: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433054: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82433058: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243305C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433060(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433060 size=12
    let mut pc: u32 = 0x82433060;
    'dispatch: loop {
        match pc {
            0x82433060 => {
    //   block [0x82433060..0x8243306C)
	// 82433060: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82433064: 409A0008  bne cr6, 0x8243306c
	if !ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x8243306C);
		return;
	}
	// 82433068: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433078 size=172
    let mut pc: u32 = 0x82433078;
    'dispatch: loop {
        match pc {
            0x82433078 => {
    //   block [0x82433078..0x824330BC)
	// 82433078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243307C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433080: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433084: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82433088: 4BFFFFD9  bl 0x82433060
	ctx.lr = 0x8243308C;
	sub_82433060(ctx, base);
	// 8243308C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433090: 419A002C  beq cr6, 0x824330bc
	if ctx.cr[6].eq {
	pc = 0x824330BC; continue 'dispatch;
	}
	// 82433094: 3860FFF4  li r3, -0xc
	ctx.r[3].s64 = -12;
	// 82433098: 48003931  bl 0x824369c8
	ctx.lr = 0x8243309C;
	sub_824369C8(ctx, base);
	// 8243309C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824330A0: 386B4B08  addi r3, r11, 0x4b08
	ctx.r[3].s64 = ctx.r[11].s64 + 19208;
	// 824330A4: 48004025  bl 0x824370c8
	ctx.lr = 0x824330A8;
	sub_824370C8(ctx, base);
	// 824330A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824330AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824330B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824330B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824330B8: 4E800020  blr
	return;
            }
            0x824330BC => {
    //   block [0x824330BC..0x824330FC)
	// 824330BC: 806A0004  lwz r3, 4(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824330C0: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 824330C4: 409A0050  bne cr6, 0x82433114
	if !ctx.cr[6].eq {
	pc = 0x82433114; continue 'dispatch;
	}
	// 824330C8: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 824330CC: 4800957D  bl 0x8243c648
	ctx.lr = 0x824330D0;
	sub_8243C648(ctx, base);
	// 824330D0: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 824330D4: 419A003C  beq cr6, 0x82433110
	if ctx.cr[6].eq {
	pc = 0x82433110; continue 'dispatch;
	}
	// 824330D8: 2F030006  cmpwi cr6, r3, 6
	ctx.cr[6].compare_i32(ctx.r[3].s32, 6, &mut ctx.xer);
	// 824330DC: 419A0034  beq cr6, 0x82433110
	if ctx.cr[6].eq {
	pc = 0x82433110; continue 'dispatch;
	}
	// 824330E0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824330E4: 40980018  bge cr6, 0x824330fc
	if !ctx.cr[6].lt {
	pc = 0x824330FC; continue 'dispatch;
	}
	// 824330E8: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 824330EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824330F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824330F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824330F8: 4E800020  blr
	return;
            }
            0x824330FC => {
    //   block [0x824330FC..0x82433110)
	// 824330FC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82433100: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243310C: 4E800020  blr
	return;
            }
            0x82433110 => {
    //   block [0x82433110..0x82433114)
	// 82433110: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	pc = 0x82433114; continue 'dispatch;
            }
            0x82433114 => {
    //   block [0x82433114..0x82433124)
	// 82433114: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433118: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243311C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433120: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433128(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433128 size=124
    let mut pc: u32 = 0x82433128;
    'dispatch: loop {
        match pc {
            0x82433128 => {
    //   block [0x82433128..0x8243318C)
	// 82433128: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243312C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433130: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82433134: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433138: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243313C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433140: 817F05A0  lwz r11, 0x5a0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1440 as u32) ) } as u64;
	// 82433144: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82433148: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243314C: 409A0040  bne cr6, 0x8243318c
	if !ctx.cr[6].eq {
	pc = 0x8243318C; continue 'dispatch;
	}
	// 82433150: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82433154: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82433158: 4800BDA9  bl 0x8243ef00
	ctx.lr = 0x8243315C;
	sub_8243EF00(ctx, base);
	// 8243315C: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82433160: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82433164: 419A0028  beq cr6, 0x8243318c
	if ctx.cr[6].eq {
	pc = 0x8243318C; continue 'dispatch;
	}
	// 82433168: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8243316C: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82433170: 4800C251  bl 0x8243f3c0
	ctx.lr = 0x82433174;
	sub_8243F3C0(ctx, base);
	// 82433174: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82433178: 80A10054  lwz r5, 0x54(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243317C: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82433180: 4800C0C1  bl 0x8243f240
	ctx.lr = 0x82433184;
	sub_8243F240(ctx, base);
	// 82433184: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82433188: 917F05A0  stw r11, 0x5a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1440 as u32), ctx.r[11].u32 ) };
	pc = 0x8243318C; continue 'dispatch;
            }
            0x8243318C => {
    //   block [0x8243318C..0x824331A4)
	// 8243318C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82433190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433198: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243319C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824331A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824331A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824331A8 size=316
    let mut pc: u32 = 0x824331A8;
    'dispatch: loop {
        match pc {
            0x824331A8 => {
    //   block [0x824331A8..0x824331D8)
	// 824331A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824331AC: 48101F11  bl 0x825350bc
	ctx.lr = 0x824331B0;
	sub_82535080(ctx, base);
	// 824331B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824331B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824331B8: 4BFFFEA9  bl 0x82433060
	ctx.lr = 0x824331BC;
	sub_82433060(ctx, base);
	// 824331BC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824331C0: 419A0018  beq cr6, 0x824331d8
	if ctx.cr[6].eq {
	pc = 0x824331D8; continue 'dispatch;
	}
	// 824331C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824331C8: 386B4B50  addi r3, r11, 0x4b50
	ctx.r[3].s64 = ctx.r[11].s64 + 19280;
	// 824331CC: 48003EFD  bl 0x824370c8
	ctx.lr = 0x824331D0;
	sub_824370C8(ctx, base);
	// 824331D0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824331D4: 48101F38  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x824331D8 => {
    //   block [0x824331D8..0x824331E8)
	// 824331D8: 83BF0048  lwz r29, 0x48(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824331DC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824331E0: 409A0040  bne cr6, 0x82433220
	if !ctx.cr[6].eq {
	pc = 0x82433220; continue 'dispatch;
	}
	// 824331E4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	pc = 0x824331E8; continue 'dispatch;
            }
            0x824331E8 => {
    //   block [0x824331E8..0x824331F0)
	// 824331E8: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 824331EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	pc = 0x824331F0; continue 'dispatch;
            }
            0x824331F0 => {
    //   block [0x824331F0..0x824331F4)
	// 824331F0: 48009709  bl 0x8243c8f8
	ctx.lr = 0x824331F4;
	sub_8243C8F8(ctx, base);
	pc = 0x824331F4; continue 'dispatch;
            }
            0x824331F4 => {
    //   block [0x824331F4..0x82433204)
	// 824331F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824331F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 824331FC: 917F059C  stw r11, 0x59c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1436 as u32), ctx.r[11].u32 ) };
	// 82433200: 917F05A0  stw r11, 0x5a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1440 as u32), ctx.r[11].u32 ) };
	pc = 0x82433204; continue 'dispatch;
            }
            0x82433204 => {
    //   block [0x82433204..0x82433208)
	// 82433204: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	pc = 0x82433208; continue 'dispatch;
            }
            0x82433208 => {
    //   block [0x82433208..0x82433220)
	// 82433208: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243320C: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82433210: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433214: 4800C02D  bl 0x8243f240
	ctx.lr = 0x82433218;
	sub_8243F240(ctx, base);
	// 82433218: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243321C: 48101EF0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82433220 => {
    //   block [0x82433220..0x82433230)
	// 82433220: 2F040001  cmpwi cr6, r4, 1
	ctx.cr[6].compare_i32(ctx.r[4].s32, 1, &mut ctx.xer);
	// 82433224: 409A000C  bne cr6, 0x82433230
	if !ctx.cr[6].eq {
	pc = 0x82433230; continue 'dispatch;
	}
	// 82433228: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8243322C: 4BFFFFBC  b 0x824331e8
	pc = 0x824331E8; continue 'dispatch;
            }
            0x82433230 => {
    //   block [0x82433230..0x82433254)
	// 82433230: 2F040002  cmpwi cr6, r4, 2
	ctx.cr[6].compare_i32(ctx.r[4].s32, 2, &mut ctx.xer);
	// 82433234: 409A0028  bne cr6, 0x8243325c
	if !ctx.cr[6].eq {
	pc = 0x8243325C; continue 'dispatch;
	}
	// 82433238: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243323C: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82433240: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82433244: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82433248: 409A000C  bne cr6, 0x82433254
	if !ctx.cr[6].eq {
	pc = 0x82433254; continue 'dispatch;
	}
	// 8243324C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82433250: 4BFFFFA0  b 0x824331f0
	pc = 0x824331F0; continue 'dispatch;
            }
            0x82433254 => {
    //   block [0x82433254..0x8243325C)
	// 82433254: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82433258: 4BFFFF98  b 0x824331f0
	pc = 0x824331F0; continue 'dispatch;
            }
            0x8243325C => {
    //   block [0x8243325C..0x8243326C)
	// 8243325C: 2F040004  cmpwi cr6, r4, 4
	ctx.cr[6].compare_i32(ctx.r[4].s32, 4, &mut ctx.xer);
	// 82433260: 409A000C  bne cr6, 0x8243326c
	if !ctx.cr[6].eq {
	pc = 0x8243326C; continue 'dispatch;
	}
	// 82433264: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82433268: 4BFFFF80  b 0x824331e8
	pc = 0x824331E8; continue 'dispatch;
            }
            0x8243326C => {
    //   block [0x8243326C..0x8243327C)
	// 8243326C: 2F040005  cmpwi cr6, r4, 5
	ctx.cr[6].compare_i32(ctx.r[4].s32, 5, &mut ctx.xer);
	// 82433270: 409A000C  bne cr6, 0x8243327c
	if !ctx.cr[6].eq {
	pc = 0x8243327C; continue 'dispatch;
	}
	// 82433274: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82433278: 4BFFFF70  b 0x824331e8
	pc = 0x824331E8; continue 'dispatch;
            }
            0x8243327C => {
    //   block [0x8243327C..0x824332BC)
	// 8243327C: 2F040006  cmpwi cr6, r4, 6
	ctx.cr[6].compare_i32(ctx.r[4].s32, 6, &mut ctx.xer);
	// 82433280: 409A003C  bne cr6, 0x824332bc
	if !ctx.cr[6].eq {
	pc = 0x824332BC; continue 'dispatch;
	}
	// 82433284: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82433288: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 8243328C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82433290: 48009669  bl 0x8243c8f8
	ctx.lr = 0x82433294;
	sub_8243C8F8(ctx, base);
	// 82433294: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82433298: 817F05A8  lwz r11, 0x5a8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1448 as u32) ) } as u64;
	// 8243329C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824332A0: 93DF059C  stw r30, 0x59c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1436 as u32), ctx.r[30].u32 ) };
	// 824332A4: 419A0028  beq cr6, 0x824332cc
	if ctx.cr[6].eq {
	pc = 0x824332CC; continue 'dispatch;
	}
	// 824332A8: 813F05A4  lwz r9, 0x5a4(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1444 as u32) ) } as u64;
	// 824332AC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824332B0: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 824332B4: 915F05A0  stw r10, 0x5a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1440 as u32), ctx.r[10].u32 ) };
	// 824332B8: 4BFFFF4C  b 0x82433204
	pc = 0x82433204; continue 'dispatch;
            }
            0x824332BC => {
    //   block [0x824332BC..0x824332CC)
	// 824332BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824332C0: 386B4B30  addi r3, r11, 0x4b30
	ctx.r[3].s64 = ctx.r[11].s64 + 19248;
	// 824332C4: 48003E05  bl 0x824370c8
	ctx.lr = 0x824332C8;
	sub_824370C8(ctx, base);
	// 824332C8: 4BFFFF2C  b 0x824331f4
	pc = 0x824331F4; continue 'dispatch;
            }
            0x824332CC => {
    //   block [0x824332CC..0x824332E4)
	// 824332CC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824332D0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 824332D4: 38607530  li r3, 0x7530
	ctx.r[3].s64 = 30000;
	// 824332D8: 4800C0E9  bl 0x8243f3c0
	ctx.lr = 0x824332DC;
	sub_8243F3C0(ctx, base);
	// 824332DC: 93DF05A0  stw r30, 0x5a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1440 as u32), ctx.r[30].u32 ) };
	// 824332E0: 4BFFFF28  b 0x82433208
	pc = 0x82433208; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824332E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824332E8 size=152
    let mut pc: u32 = 0x824332E8;
    'dispatch: loop {
        match pc {
            0x824332E8 => {
    //   block [0x824332E8..0x82433330)
	// 824332E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824332EC: 48101DCD  bl 0x825350b8
	ctx.lr = 0x824332F0;
	sub_82535080(ctx, base);
	// 824332F0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824332F4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 824332F8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824332FC: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82433300: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 82433304: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82433308: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 8243330C: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82433310: 4BFFFD51  bl 0x82433060
	ctx.lr = 0x82433314;
	sub_82433060(ctx, base);
	// 82433314: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433318: 419A0018  beq cr6, 0x82433330
	if ctx.cr[6].eq {
	pc = 0x82433330; continue 'dispatch;
	}
	// 8243331C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433320: 386B4BA8  addi r3, r11, 0x4ba8
	ctx.r[3].s64 = ctx.r[11].s64 + 19368;
	// 82433324: 48003DA5  bl 0x824370c8
	ctx.lr = 0x82433328;
	sub_824370C8(ctx, base);
	// 82433328: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243332C: 48101DDC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82433330 => {
    //   block [0x82433330..0x82433364)
	// 82433330: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82433334: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82433338: 419A0040  beq cr6, 0x82433378
	if ctx.cr[6].eq {
	pc = 0x82433378; continue 'dispatch;
	}
	// 8243333C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82433340: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82433344: 4800CAE5  bl 0x8243fe28
	ctx.lr = 0x82433348;
	sub_8243FE28(ctx, base);
	// 82433348: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243334C: 419A0018  beq cr6, 0x82433364
	if ctx.cr[6].eq {
	pc = 0x82433364; continue 'dispatch;
	}
	// 82433350: 3860FECB  li r3, -0x135
	ctx.r[3].s64 = -309;
	// 82433354: 48003675  bl 0x824369c8
	ctx.lr = 0x82433358;
	sub_824369C8(ctx, base);
	// 82433358: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243335C: 386B4B80  addi r3, r11, 0x4b80
	ctx.r[3].s64 = ctx.r[11].s64 + 19328;
	// 82433360: 48003D69  bl 0x824370c8
	ctx.lr = 0x82433364;
	sub_824370C8(ctx, base);
	pc = 0x82433364; continue 'dispatch;
            }
            0x82433364 => {
    //   block [0x82433364..0x82433378)
	// 82433364: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82433368: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 8243336C: 4098000C  bge cr6, 0x82433378
	if !ctx.cr[6].lt {
	pc = 0x82433378; continue 'dispatch;
	}
	// 82433370: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82433374: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	pc = 0x82433378; continue 'dispatch;
            }
            0x82433378 => {
    //   block [0x82433378..0x82433380)
	// 82433378: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243337C: 48101D8C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433380 size=80
    let mut pc: u32 = 0x82433380;
    'dispatch: loop {
        match pc {
            0x82433380 => {
    //   block [0x82433380..0x824333BC)
	// 82433380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243338C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82433390: 4BFFFCD1  bl 0x82433060
	ctx.lr = 0x82433394;
	sub_82433060(ctx, base);
	// 82433394: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433398: 419A0024  beq cr6, 0x824333bc
	if ctx.cr[6].eq {
	pc = 0x824333BC; continue 'dispatch;
	}
	// 8243339C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824333A0: 386B4BD4  addi r3, r11, 0x4bd4
	ctx.r[3].s64 = ctx.r[11].s64 + 19412;
	// 824333A4: 48003D25  bl 0x824370c8
	ctx.lr = 0x824333A8;
	sub_824370C8(ctx, base);
	// 824333A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824333AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824333B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824333B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824333B8: 4E800020  blr
	return;
            }
            0x824333BC => {
    //   block [0x824333BC..0x824333D0)
	// 824333BC: 806A0454  lwz r3, 0x454(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(1108 as u32) ) } as u64;
	// 824333C0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824333C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824333C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824333CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824333D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824333D0 size=8
    let mut pc: u32 = 0x824333D0;
    'dispatch: loop {
        match pc {
            0x824333D0 => {
    //   block [0x824333D0..0x824333D8)
	// 824333D0: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 824333D4: 480095C4  b 0x8243c998
	sub_8243C998(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824333D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824333D8 size=88
    let mut pc: u32 = 0x824333D8;
    'dispatch: loop {
        match pc {
            0x824333D8 => {
    //   block [0x824333D8..0x82433414)
	// 824333D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824333DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824333E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824333E4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824333E8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 824333EC: 4BFFFC75  bl 0x82433060
	ctx.lr = 0x824333F0;
	sub_82433060(ctx, base);
	// 824333F0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824333F4: 419A0020  beq cr6, 0x82433414
	if ctx.cr[6].eq {
	pc = 0x82433414; continue 'dispatch;
	}
	// 824333F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824333FC: 386B4C04  addi r3, r11, 0x4c04
	ctx.r[3].s64 = ctx.r[11].s64 + 19460;
	// 82433400: 48003CC9  bl 0x824370c8
	ctx.lr = 0x82433404;
	sub_824370C8(ctx, base);
	// 82433404: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433408: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243340C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433410: 4E800020  blr
	return;
            }
            0x82433414 => {
    //   block [0x82433414..0x82433430)
	// 82433414: 38800047  li r4, 0x47
	ctx.r[4].s64 = 71;
	// 82433418: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243341C: 480094DD  bl 0x8243c8f8
	ctx.lr = 0x82433420;
	sub_8243C8F8(ctx, base);
	// 82433420: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433424: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433428: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243342C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433430 size=80
    let mut pc: u32 = 0x82433430;
    'dispatch: loop {
        match pc {
            0x82433430 => {
    //   block [0x82433430..0x8243346C)
	// 82433430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433438: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243343C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82433440: 4BFFFC21  bl 0x82433060
	ctx.lr = 0x82433444;
	sub_82433060(ctx, base);
	// 82433444: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433448: 419A0024  beq cr6, 0x8243346c
	if ctx.cr[6].eq {
	pc = 0x8243346C; continue 'dispatch;
	}
	// 8243344C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433450: 386B4C38  addi r3, r11, 0x4c38
	ctx.r[3].s64 = ctx.r[11].s64 + 19512;
	// 82433454: 48003C75  bl 0x824370c8
	ctx.lr = 0x82433458;
	sub_824370C8(ctx, base);
	// 82433458: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243345C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433468: 4E800020  blr
	return;
            }
            0x8243346C => {
    //   block [0x8243346C..0x82433480)
	// 8243346C: 806A0048  lwz r3, 0x48(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(72 as u32) ) } as u64;
	// 82433470: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433474: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433478: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243347C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433480 size=108
    let mut pc: u32 = 0x82433480;
    'dispatch: loop {
        match pc {
            0x82433480 => {
    //   block [0x82433480..0x824334C4)
	// 82433480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433488: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243348C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433490: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433494: 4BFFFBE5  bl 0x82433078
	ctx.lr = 0x82433498;
	sub_82433078(ctx, base);
	// 82433498: 817F0570  lwz r11, 0x570(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1392 as u32) ) } as u64;
	// 8243349C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824334A0: 409A0038  bne cr6, 0x824334d8
	if !ctx.cr[6].eq {
	pc = 0x824334D8; continue 'dispatch;
	}
	// 824334A4: 817F0574  lwz r11, 0x574(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1396 as u32) ) } as u64;
	// 824334A8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824334AC: 409A002C  bne cr6, 0x824334d8
	if !ctx.cr[6].eq {
	pc = 0x824334D8; continue 'dispatch;
	}
	// 824334B0: 817F0580  lwz r11, 0x580(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1408 as u32) ) } as u64;
	// 824334B4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824334B8: 419A000C  beq cr6, 0x824334c4
	if ctx.cr[6].eq {
	pc = 0x824334C4; continue 'dispatch;
	}
	// 824334BC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 824334C0: 409A0018  bne cr6, 0x824334d8
	if !ctx.cr[6].eq {
	pc = 0x824334D8; continue 'dispatch;
	}
	pc = 0x824334C4; continue 'dispatch;
            }
            0x824334C4 => {
    //   block [0x824334C4..0x824334D4)
	// 824334C4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824334C8: 419A000C  beq cr6, 0x824334d4
	if ctx.cr[6].eq {
	pc = 0x824334D4; continue 'dispatch;
	}
	// 824334CC: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 824334D0: 409A0008  bne cr6, 0x824334d8
	if !ctx.cr[6].eq {
	pc = 0x824334D8; continue 'dispatch;
	}
	pc = 0x824334D4; continue 'dispatch;
            }
            0x824334D4 => {
    //   block [0x824334D4..0x824334D8)
	// 824334D4: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	pc = 0x824334D8; continue 'dispatch;
            }
            0x824334D8 => {
    //   block [0x824334D8..0x824334EC)
	// 824334D8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824334DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824334E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824334E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824334E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824334F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824334F0 size=164
    let mut pc: u32 = 0x824334F0;
    'dispatch: loop {
        match pc {
            0x824334F0 => {
    //   block [0x824334F0..0x82433528)
	// 824334F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824334F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824334F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824334FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433508: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243350C: 4BFFFB55  bl 0x82433060
	ctx.lr = 0x82433510;
	sub_82433060(ctx, base);
	// 82433510: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433514: 419A0014  beq cr6, 0x82433528
	if ctx.cr[6].eq {
	pc = 0x82433528; continue 'dispatch;
	}
	// 82433518: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243351C: 386B4CBC  addi r3, r11, 0x4cbc
	ctx.r[3].s64 = ctx.r[11].s64 + 19644;
	// 82433520: 48003BA9  bl 0x824370c8
	ctx.lr = 0x82433524;
	sub_824370C8(ctx, base);
	// 82433524: 48000058  b 0x8243357c
	pc = 0x8243357C; continue 'dispatch;
            }
            0x82433528 => {
    //   block [0x82433528..0x82433540)
	// 82433528: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 8243352C: 409A0014  bne cr6, 0x82433540
	if !ctx.cr[6].eq {
	pc = 0x82433540; continue 'dispatch;
	}
	// 82433530: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433534: 386B4C94  addi r3, r11, 0x4c94
	ctx.r[3].s64 = ctx.r[11].s64 + 19604;
	// 82433538: 48003B91  bl 0x824370c8
	ctx.lr = 0x8243353C;
	sub_824370C8(ctx, base);
	// 8243353C: 48000040  b 0x8243357c
	pc = 0x8243357C; continue 'dispatch;
            }
            0x82433540 => {
    //   block [0x82433540..0x82433570)
	// 82433540: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82433544: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433548: 4BFF4499  bl 0x824279e0
	ctx.lr = 0x8243354C;
	sub_824279E0(ctx, base);
	// 8243354C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82433550: 40980020  bge cr6, 0x82433570
	if !ctx.cr[6].lt {
	pc = 0x82433570; continue 'dispatch;
	}
	// 82433554: 39400004  li r10, 4
	ctx.r[10].s64 = 4;
	// 82433558: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243355C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82433560: 386B4C64  addi r3, r11, 0x4c64
	ctx.r[3].s64 = ctx.r[11].s64 + 19556;
	// 82433564: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 82433568: 48003B61  bl 0x824370c8
	ctx.lr = 0x8243356C;
	sub_824370C8(ctx, base);
	// 8243356C: 48000010  b 0x8243357c
	pc = 0x8243357C; continue 'dispatch;
            }
            0x82433570 => {
    //   block [0x82433570..0x8243357C)
	// 82433570: 817F0084  lwz r11, 0x84(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(132 as u32) ) } as u64;
	// 82433574: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82433578: 917F0084  stw r11, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	pc = 0x8243357C; continue 'dispatch;
            }
            0x8243357C => {
    //   block [0x8243357C..0x82433594)
	// 8243357C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433580: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433584: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433588: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243358C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433590: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433598 size=92
    let mut pc: u32 = 0x82433598;
    'dispatch: loop {
        match pc {
            0x82433598 => {
    //   block [0x82433598..0x824335D0)
	// 82433598: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243359C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824335A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824335A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824335A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824335AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824335B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824335B4: 4BFFFAAD  bl 0x82433060
	ctx.lr = 0x824335B8;
	sub_82433060(ctx, base);
	// 824335B8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824335BC: 419A0014  beq cr6, 0x824335d0
	if ctx.cr[6].eq {
	pc = 0x824335D0; continue 'dispatch;
	}
	// 824335C0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824335C4: 386B4CEC  addi r3, r11, 0x4cec
	ctx.r[3].s64 = ctx.r[11].s64 + 19692;
	// 824335C8: 48003B01  bl 0x824370c8
	ctx.lr = 0x824335CC;
	sub_824370C8(ctx, base);
	// 824335CC: 48000010  b 0x824335dc
	pc = 0x824335DC; continue 'dispatch;
            }
            0x824335D0 => {
    //   block [0x824335D0..0x824335DC)
	// 824335D0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824335D4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824335D8: 4BFF43B1  bl 0x82427988
	ctx.lr = 0x824335DC;
	sub_82427988(ctx, base);
	pc = 0x824335DC; continue 'dispatch;
            }
            0x824335DC => {
    //   block [0x824335DC..0x824335F4)
	// 824335DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824335E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824335E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824335E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824335EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824335F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824335F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824335F8 size=164
    let mut pc: u32 = 0x824335F8;
    'dispatch: loop {
        match pc {
            0x824335F8 => {
    //   block [0x824335F8..0x82433630)
	// 824335F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824335FC: 48101ABD  bl 0x825350b8
	ctx.lr = 0x82433600;
	sub_82535080(ctx, base);
	// 82433600: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433604: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433608: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243360C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82433610: 4BFFFA51  bl 0x82433060
	ctx.lr = 0x82433614;
	sub_82433060(ctx, base);
	// 82433614: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433618: 419A0018  beq cr6, 0x82433630
	if ctx.cr[6].eq {
	pc = 0x82433630; continue 'dispatch;
	}
	// 8243361C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433620: 386B4D4C  addi r3, r11, 0x4d4c
	ctx.r[3].s64 = ctx.r[11].s64 + 19788;
	// 82433624: 48003AA5  bl 0x824370c8
	ctx.lr = 0x82433628;
	sub_824370C8(ctx, base);
	// 82433628: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8243362C: 48101ADC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82433630 => {
    //   block [0x82433630..0x82433680)
	// 82433630: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82433634: 839F0054  lwz r28, 0x54(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433638: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 8243363C: 80BF0438  lwz r5, 0x438(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 82433640: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82433644: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82433648: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243364C: 4BFE485D  bl 0x82417ea8
	ctx.lr = 0x82433650;
	sub_82417EA8(ctx, base);
	// 82433650: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82433654: 409A002C  bne cr6, 0x82433680
	if !ctx.cr[6].eq {
	pc = 0x82433680; continue 'dispatch;
	}
	// 82433658: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243365C: 4BFE411D  bl 0x82417778
	ctx.lr = 0x82433660;
	sub_82417778(ctx, base);
	// 82433660: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82433664: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82433668: 80E10050  lwz r7, 0x50(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243366C: 80C10054  lwz r6, 0x54(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433670: 80A10058  lwz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82433674: 4BFF4065  bl 0x824276d8
	ctx.lr = 0x82433678;
	sub_824276D8(ctx, base);
	// 82433678: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 8243367C: 48101A8C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82433680 => {
    //   block [0x82433680..0x8243369C)
	// 82433680: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433684: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82433688: 386B4D18  addi r3, r11, 0x4d18
	ctx.r[3].s64 = ctx.r[11].s64 + 19736;
	// 8243368C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82433690: 48003A39  bl 0x824370c8
	ctx.lr = 0x82433694;
	sub_824370C8(ctx, base);
	// 82433694: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82433698: 48101A70  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824336A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824336A0 size=92
    let mut pc: u32 = 0x824336A0;
    'dispatch: loop {
        match pc {
            0x824336A0 => {
    //   block [0x824336A0..0x824336DC)
	// 824336A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824336A4: 48101A15  bl 0x825350b8
	ctx.lr = 0x824336A8;
	sub_82535080(ctx, base);
	// 824336A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824336AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824336B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824336B4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824336B8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 824336BC: 4BFFF9A5  bl 0x82433060
	ctx.lr = 0x824336C0;
	sub_82433060(ctx, base);
	// 824336C0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824336C4: 419A0018  beq cr6, 0x824336dc
	if ctx.cr[6].eq {
	pc = 0x824336DC; continue 'dispatch;
	}
	// 824336C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824336CC: 386B4D78  addi r3, r11, 0x4d78
	ctx.r[3].s64 = ctx.r[11].s64 + 19832;
	// 824336D0: 480039F9  bl 0x824370c8
	ctx.lr = 0x824336D4;
	sub_824370C8(ctx, base);
	// 824336D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824336D8: 48101A30  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x824336DC => {
    //   block [0x824336DC..0x824336FC)
	// 824336DC: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 824336E0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824336E4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 824336E8: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824336EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824336F0: 4BFF3FE9  bl 0x824276d8
	ctx.lr = 0x824336F4;
	sub_824276D8(ctx, base);
	// 824336F4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824336F8: 48101A10  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433700 size=44
    let mut pc: u32 = 0x82433700;
    'dispatch: loop {
        match pc {
            0x82433700 => {
    //   block [0x82433700..0x8243372C)
	// 82433700: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433704: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433708: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243370C: 4BFF414D  bl 0x82427858
	ctx.lr = 0x82433710;
	sub_82427858(ctx, base);
	// 82433710: 3963FFFD  addi r11, r3, -3
	ctx.r[11].s64 = ctx.r[3].s64 + -3;
	// 82433714: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82433718: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 8243371C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433720: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433724: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433730 size=16
    let mut pc: u32 = 0x82433730;
    'dispatch: loop {
        match pc {
            0x82433730 => {
    //   block [0x82433730..0x82433740)
	// 82433730: 80630054  lwz r3, 0x54(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433734: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82433738: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8243373C: 4BFF41CC  b 0x82427908
	sub_82427908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433748(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433748 size=16
    let mut pc: u32 = 0x82433748;
    'dispatch: loop {
        match pc {
            0x82433748 => {
    //   block [0x82433748..0x82433758)
	// 82433748: 80630054  lwz r3, 0x54(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243374C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82433750: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82433754: 4BFF4034  b 0x82427788
	sub_82427788(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433760(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433760 size=184
    let mut pc: u32 = 0x82433760;
    'dispatch: loop {
        match pc {
            0x82433760 => {
    //   block [0x82433760..0x82433798)
	// 82433760: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433764: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433768: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243376C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433770: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433774: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433778: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243377C: 4BFFF8E5  bl 0x82433060
	ctx.lr = 0x82433780;
	sub_82433060(ctx, base);
	// 82433780: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433784: 419A0014  beq cr6, 0x82433798
	if ctx.cr[6].eq {
	pc = 0x82433798; continue 'dispatch;
	}
	// 82433788: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243378C: 386B4DD8  addi r3, r11, 0x4dd8
	ctx.r[3].s64 = ctx.r[11].s64 + 19928;
	// 82433790: 48003939  bl 0x824370c8
	ctx.lr = 0x82433794;
	sub_824370C8(ctx, base);
	// 82433794: 4800006C  b 0x82433800
	pc = 0x82433800; continue 'dispatch;
            }
            0x82433798 => {
    //   block [0x82433798..0x824337BC)
	// 82433798: 897F0080  lbz r11, 0x80(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(128 as u32) ) } as u64;
	// 8243379C: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824337A0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824337A4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824337A8: 409A0014  bne cr6, 0x824337bc
	if !ctx.cr[6].eq {
	pc = 0x824337BC; continue 'dispatch;
	}
	// 824337AC: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824337B0: 409A000C  bne cr6, 0x824337bc
	if !ctx.cr[6].eq {
	pc = 0x824337BC; continue 'dispatch;
	}
	// 824337B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 824337B8: 995F0081  stb r10, 0x81(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(129 as u32), ctx.r[10].u8 ) };
	pc = 0x824337BC; continue 'dispatch;
            }
            0x824337BC => {
    //   block [0x824337BC..0x824337E4)
	// 824337BC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824337C0: 409A003C  bne cr6, 0x824337fc
	if !ctx.cr[6].eq {
	pc = 0x824337FC; continue 'dispatch;
	}
	// 824337C4: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 824337C8: 409A0034  bne cr6, 0x824337fc
	if !ctx.cr[6].eq {
	pc = 0x824337FC; continue 'dispatch;
	}
	// 824337CC: 48010A95  bl 0x82444260
	ctx.lr = 0x824337D0;
	sub_82444260(ctx, base);
	// 824337D0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824337D4: 419A0010  beq cr6, 0x824337e4
	if ctx.cr[6].eq {
	pc = 0x824337E4; continue 'dispatch;
	}
	// 824337D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824337DC: 386B4DAC  addi r3, r11, 0x4dac
	ctx.r[3].s64 = ctx.r[11].s64 + 19884;
	// 824337E0: 480038E9  bl 0x824370c8
	ctx.lr = 0x824337E4;
	sub_824370C8(ctx, base);
	pc = 0x824337E4; continue 'dispatch;
            }
            0x824337E4 => {
    //   block [0x824337E4..0x824337FC)
	// 824337E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824337E8: 387F0518  addi r3, r31, 0x518
	ctx.r[3].s64 = ctx.r[31].s64 + 1304;
	// 824337EC: 48007F85  bl 0x8243b770
	ctx.lr = 0x824337F0;
	sub_8243B770(ctx, base);
	// 824337F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824337F4: 387F053C  addi r3, r31, 0x53c
	ctx.r[3].s64 = ctx.r[31].s64 + 1340;
	// 824337F8: 48007F79  bl 0x8243b770
	ctx.lr = 0x824337FC;
	sub_8243B770(ctx, base);
	pc = 0x824337FC; continue 'dispatch;
            }
            0x824337FC => {
    //   block [0x824337FC..0x82433800)
	// 824337FC: 9BDF0080  stb r30, 0x80(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	pc = 0x82433800; continue 'dispatch;
            }
            0x82433800 => {
    //   block [0x82433800..0x82433818)
	// 82433800: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433804: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433808: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243380C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82433810: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433818 size=8
    let mut pc: u32 = 0x82433818;
    'dispatch: loop {
        match pc {
            0x82433818 => {
    //   block [0x82433818..0x82433820)
	// 82433818: 90830570  stw r4, 0x570(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1392 as u32), ctx.r[4].u32 ) };
	// 8243381C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82433820 size=164
    let mut pc: u32 = 0x82433820;
    'dispatch: loop {
        match pc {
            0x82433820 => {
    //   block [0x82433820..0x82433884)
	// 82433820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433828: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243382C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433830: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82433834: 38800036  li r4, 0x36
	ctx.r[4].s64 = 54;
	// 82433838: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243383C: 4BFFF7BD  bl 0x82432ff8
	ctx.lr = 0x82433840;
	sub_82432FF8(ctx, base);
	// 82433840: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82433844: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82433848: 4800B6B9  bl 0x8243ef00
	ctx.lr = 0x8243384C;
	sub_8243EF00(ctx, base);
	// 8243384C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82433850: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82433854: 41990030  bgt cr6, 0x82433884
	if ctx.cr[6].gt {
	pc = 0x82433884; continue 'dispatch;
	}
	// 82433858: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243385C: 386B4E04  addi r3, r11, 0x4e04
	ctx.r[3].s64 = ctx.r[11].s64 + 19972;
	// 82433860: 48003869  bl 0x824370c8
	ctx.lr = 0x82433864;
	sub_824370C8(ctx, base);
	// 82433864: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433868: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 8243386C: 917F0594  stw r11, 0x594(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1428 as u32), ctx.r[11].u32 ) };
	// 82433870: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243387C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433880: 4E800020  blr
	return;
            }
            0x82433884 => {
    //   block [0x82433884..0x824338C4)
	// 82433884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433888: 48001139  bl 0x824349c0
	ctx.lr = 0x8243388C;
	sub_824349C0(ctx, base);
	// 8243388C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82433890: 1D6303E8  mulli r11, r3, 0x3e8
	ctx.r[11].s32 = ((ctx.r[3].s32 as i64 * 1000 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82433894: 7D6B53D6  divw r11, r11, r10
	ctx.r[11].s32 = ctx.r[11].s32 / ctx.r[10].s32;
	// 82433898: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243389C: 1D6B0BB8  mulli r11, r11, 0xbb8
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 3000 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824338A0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 824338A4: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 824338A8: 7C6B5050  subf r3, r11, r10
	ctx.r[3].s64 = ctx.r[10].s64 - ctx.r[11].s64;
	// 824338AC: 907F0594  stw r3, 0x594(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1428 as u32), ctx.r[3].u32 ) };
	// 824338B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824338B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824338B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824338BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824338C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824338C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824338C8 size=168
    let mut pc: u32 = 0x824338C8;
    'dispatch: loop {
        match pc {
            0x824338C8 => {
    //   block [0x824338C8..0x82433908)
	// 824338C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824338CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824338D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824338D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824338D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824338DC: 4BFFF785  bl 0x82433060
	ctx.lr = 0x824338E0;
	sub_82433060(ctx, base);
	// 824338E0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824338E4: 419A0024  beq cr6, 0x82433908
	if ctx.cr[6].eq {
	pc = 0x82433908; continue 'dispatch;
	}
	// 824338E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824338EC: 386B4E30  addi r3, r11, 0x4e30
	ctx.r[3].s64 = ctx.r[11].s64 + 20016;
	// 824338F0: 480037D9  bl 0x824370c8
	ctx.lr = 0x824338F4;
	sub_824370C8(ctx, base);
	// 824338F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824338F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824338FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433904: 4E800020  blr
	return;
            }
            0x82433908 => {
    //   block [0x82433908..0x8243394C)
	// 82433908: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243390C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433910: 4BFFFE51  bl 0x82433760
	ctx.lr = 0x82433914;
	sub_82433760(ctx, base);
	// 82433914: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433918: 809F0454  lwz r4, 0x454(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1108 as u32) ) } as u64;
	// 8243391C: 4BFFF5F5  bl 0x82432f10
	ctx.lr = 0x82433920;
	sub_82432F10(ctx, base);
	// 82433920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433924: 4BFFF13D  bl 0x82432a60
	ctx.lr = 0x82433928;
	sub_82432A60(ctx, base);
	// 82433928: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243392C: 4BFF4205  bl 0x82427b30
	ctx.lr = 0x82433930;
	sub_82427B30(ctx, base);
	// 82433930: 807F0450  lwz r3, 0x450(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1104 as u32) ) } as u64;
	// 82433934: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82433938: 419A0014  beq cr6, 0x8243394c
	if ctx.cr[6].eq {
	pc = 0x8243394C; continue 'dispatch;
	}
	// 8243393C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82433940: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82433944: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82433948: 4E800421  bctrl
	ctx.lr = 0x8243394C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8243394C => {
    //   block [0x8243394C..0x82433970)
	// 8243394C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433950: 480016E1  bl 0x82435030
	ctx.lr = 0x82433954;
	sub_82435030(ctx, base);
	// 82433954: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82433958: 917F056C  stw r11, 0x56c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1388 as u32), ctx.r[11].u32 ) };
	// 8243395C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433968: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243396C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433970 size=184
    let mut pc: u32 = 0x82433970;
    'dispatch: loop {
        match pc {
            0x82433970 => {
    //   block [0x82433970..0x824339A8)
	// 82433970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433978: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243397C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433980: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433984: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433988: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243398C: 4BFFF6D5  bl 0x82433060
	ctx.lr = 0x82433990;
	sub_82433060(ctx, base);
	// 82433990: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433994: 419A0014  beq cr6, 0x824339a8
	if ctx.cr[6].eq {
	pc = 0x824339A8; continue 'dispatch;
	}
	// 82433998: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243399C: 386B4E8C  addi r3, r11, 0x4e8c
	ctx.r[3].s64 = ctx.r[11].s64 + 20108;
	// 824339A0: 48003729  bl 0x824370c8
	ctx.lr = 0x824339A4;
	sub_824370C8(ctx, base);
	// 824339A4: 4800006C  b 0x82433a10
	pc = 0x82433A10; continue 'dispatch;
            }
            0x824339A8 => {
    //   block [0x824339A8..0x824339C0)
	// 824339A8: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 824339AC: 409A0014  bne cr6, 0x824339c0
	if !ctx.cr[6].eq {
	pc = 0x824339C0; continue 'dispatch;
	}
	// 824339B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824339B4: 386B4E60  addi r3, r11, 0x4e60
	ctx.r[3].s64 = ctx.r[11].s64 + 20064;
	// 824339B8: 48003711  bl 0x824370c8
	ctx.lr = 0x824339BC;
	sub_824370C8(ctx, base);
	// 824339BC: 48000054  b 0x82433a10
	pc = 0x82433A10; continue 'dispatch;
            }
            0x824339C0 => {
    //   block [0x824339C0..0x82433A10)
	// 824339C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824339C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824339C8: 4BFFF131  bl 0x82432af8
	ctx.lr = 0x824339CC;
	sub_82432AF8(ctx, base);
	// 824339CC: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824339D0: 4BFF4071  bl 0x82427a40
	ctx.lr = 0x824339D4;
	sub_82427A40(ctx, base);
	// 824339D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824339D8: 809F0438  lwz r4, 0x438(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 824339DC: 4BFFFB15  bl 0x824334f0
	ctx.lr = 0x824339E0;
	sub_824334F0(ctx, base);
	// 824339E0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824339E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824339E8: 4BFFFBB1  bl 0x82433598
	ctx.lr = 0x824339EC;
	sub_82433598(ctx, base);
	// 824339EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824339F0: 4BFFFED9  bl 0x824338c8
	ctx.lr = 0x824339F4;
	sub_824338C8(ctx, base);
	// 824339F4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824339F8: 815F0578  lwz r10, 0x578(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1400 as u32) ) } as u64;
	// 824339FC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82433A00: 917F057C  stw r11, 0x57c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1404 as u32), ctx.r[11].u32 ) };
	// 82433A04: 917F0574  stw r11, 0x574(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1396 as u32), ctx.r[11].u32 ) };
	// 82433A08: 409A0008  bne cr6, 0x82433a10
	if !ctx.cr[6].eq {
	pc = 0x82433A10; continue 'dispatch;
	}
	// 82433A0C: 917F0580  stw r11, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[11].u32 ) };
	pc = 0x82433A10; continue 'dispatch;
            }
            0x82433A10 => {
    //   block [0x82433A10..0x82433A28)
	// 82433A10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433A14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433A18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433A1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82433A20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433A24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433A28 size=96
    let mut pc: u32 = 0x82433A28;
    'dispatch: loop {
        match pc {
            0x82433A28 => {
    //   block [0x82433A28..0x82433A68)
	// 82433A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433A2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433A30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433A34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433A38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433A3C: 4BFFF625  bl 0x82433060
	ctx.lr = 0x82433A40;
	sub_82433060(ctx, base);
	// 82433A40: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433A44: 419A0024  beq cr6, 0x82433a68
	if ctx.cr[6].eq {
	pc = 0x82433A68; continue 'dispatch;
	}
	// 82433A48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433A4C: 386B4EBC  addi r3, r11, 0x4ebc
	ctx.r[3].s64 = ctx.r[11].s64 + 20156;
	// 82433A50: 48003679  bl 0x824370c8
	ctx.lr = 0x82433A54;
	sub_824370C8(ctx, base);
	// 82433A54: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433A58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433A5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433A60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433A64: 4E800020  blr
	return;
            }
            0x82433A68 => {
    //   block [0x82433A68..0x82433A88)
	// 82433A68: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82433A6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433A70: 4BFFFCF1  bl 0x82433760
	ctx.lr = 0x82433A74;
	sub_82433760(ctx, base);
	// 82433A74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433A78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433A7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433A80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433A84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433A88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433A88 size=148
    let mut pc: u32 = 0x82433A88;
    'dispatch: loop {
        match pc {
            0x82433A88 => {
    //   block [0x82433A88..0x82433AC0)
	// 82433A88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433A8C: 48101631  bl 0x825350bc
	ctx.lr = 0x82433A90;
	sub_82535080(ctx, base);
	// 82433A90: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433A94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433A98: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82433A9C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82433AA0: 4BFFF5C1  bl 0x82433060
	ctx.lr = 0x82433AA4;
	sub_82433060(ctx, base);
	// 82433AA4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433AA8: 419A0018  beq cr6, 0x82433ac0
	if ctx.cr[6].eq {
	pc = 0x82433AC0; continue 'dispatch;
	}
	// 82433AAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433AB0: 386B4EF0  addi r3, r11, 0x4ef0
	ctx.r[3].s64 = ctx.r[11].s64 + 20208;
	// 82433AB4: 48003615  bl 0x824370c8
	ctx.lr = 0x82433AB8;
	sub_824370C8(ctx, base);
	// 82433AB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433ABC: 48101650  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82433AC0 => {
    //   block [0x82433AC0..0x82433B14)
	// 82433AC0: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433AC4: 4BFF3F7D  bl 0x82427a40
	ctx.lr = 0x82433AC8;
	sub_82427A40(ctx, base);
	// 82433AC8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82433ACC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82433AD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433AD4: 4BFFFB25  bl 0x824335f8
	ctx.lr = 0x82433AD8;
	sub_824335F8(ctx, base);
	// 82433AD8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82433ADC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433AE0: 4BFFFAB9  bl 0x82433598
	ctx.lr = 0x82433AE4;
	sub_82433598(ctx, base);
	// 82433AE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433AE8: 4BFFFDE1  bl 0x824338c8
	ctx.lr = 0x82433AEC;
	sub_824338C8(ctx, base);
	// 82433AEC: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82433AF0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82433AF4: 813F0578  lwz r9, 0x578(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1400 as u32) ) } as u64;
	// 82433AF8: 93DF0584  stw r30, 0x584(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1412 as u32), ctx.r[30].u32 ) };
	// 82433AFC: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82433B00: 93BF0588  stw r29, 0x588(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1416 as u32), ctx.r[29].u32 ) };
	// 82433B04: 915F057C  stw r10, 0x57c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1404 as u32), ctx.r[10].u32 ) };
	// 82433B08: 917F0574  stw r11, 0x574(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1396 as u32), ctx.r[11].u32 ) };
	// 82433B0C: 409A0008  bne cr6, 0x82433b14
	if !ctx.cr[6].eq {
	pc = 0x82433B14; continue 'dispatch;
	}
	// 82433B10: 917F0580  stw r11, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[11].u32 ) };
	pc = 0x82433B14; continue 'dispatch;
            }
            0x82433B14 => {
    //   block [0x82433B14..0x82433B1C)
	// 82433B14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433B18: 481015F4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433B20 size=168
    let mut pc: u32 = 0x82433B20;
    'dispatch: loop {
        match pc {
            0x82433B20 => {
    //   block [0x82433B20..0x82433B5C)
	// 82433B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433B24: 48101595  bl 0x825350b8
	ctx.lr = 0x82433B28;
	sub_82535080(ctx, base);
	// 82433B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433B2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433B30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82433B34: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82433B38: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82433B3C: 4BFFF525  bl 0x82433060
	ctx.lr = 0x82433B40;
	sub_82433060(ctx, base);
	// 82433B40: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433B44: 419A0018  beq cr6, 0x82433b5c
	if ctx.cr[6].eq {
	pc = 0x82433B5C; continue 'dispatch;
	}
	// 82433B48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433B4C: 386B4F20  addi r3, r11, 0x4f20
	ctx.r[3].s64 = ctx.r[11].s64 + 20256;
	// 82433B50: 48003579  bl 0x824370c8
	ctx.lr = 0x82433B54;
	sub_824370C8(ctx, base);
	// 82433B54: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82433B58: 481015B0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82433B5C => {
    //   block [0x82433B5C..0x82433BC0)
	// 82433B5C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82433B60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433B64: 4BFFEF95  bl 0x82432af8
	ctx.lr = 0x82433B68;
	sub_82432AF8(ctx, base);
	// 82433B68: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433B6C: 4BFF3ED5  bl 0x82427a40
	ctx.lr = 0x82433B70;
	sub_82427A40(ctx, base);
	// 82433B70: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82433B74: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82433B78: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82433B7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433B80: 4BFFFB21  bl 0x824336a0
	ctx.lr = 0x82433B84;
	sub_824336A0(ctx, base);
	// 82433B84: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82433B88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433B8C: 4BFFFA0D  bl 0x82433598
	ctx.lr = 0x82433B90;
	sub_82433598(ctx, base);
	// 82433B90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433B94: 4BFFFD35  bl 0x824338c8
	ctx.lr = 0x82433B98;
	sub_824338C8(ctx, base);
	// 82433B98: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 82433B9C: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82433BA0: 813F0578  lwz r9, 0x578(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1400 as u32) ) } as u64;
	// 82433BA4: 93BF058C  stw r29, 0x58c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1420 as u32), ctx.r[29].u32 ) };
	// 82433BA8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82433BAC: 939F0590  stw r28, 0x590(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1424 as u32), ctx.r[28].u32 ) };
	// 82433BB0: 915F057C  stw r10, 0x57c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1404 as u32), ctx.r[10].u32 ) };
	// 82433BB4: 917F0574  stw r11, 0x574(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1396 as u32), ctx.r[11].u32 ) };
	// 82433BB8: 409A0008  bne cr6, 0x82433bc0
	if !ctx.cr[6].eq {
	pc = 0x82433BC0; continue 'dispatch;
	}
	// 82433BBC: 917F0580  stw r11, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[11].u32 ) };
	pc = 0x82433BC0; continue 'dispatch;
            }
            0x82433BC0 => {
    //   block [0x82433BC0..0x82433BC8)
	// 82433BC0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82433BC4: 48101544  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433BC8 size=164
    let mut pc: u32 = 0x82433BC8;
    'dispatch: loop {
        match pc {
            0x82433BC8 => {
    //   block [0x82433BC8..0x82433C24)
	// 82433BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433BD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433BD4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433BD8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82433BDC: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82433BE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433BE4: 4BFFF705  bl 0x824332e8
	ctx.lr = 0x82433BE8;
	sub_824332E8(ctx, base);
	// 82433BE8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82433BEC: 817F057C  lwz r11, 0x57c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1404 as u32) ) } as u64;
	// 82433BF0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82433BF4: 915F0578  stw r10, 0x578(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1400 as u32), ctx.r[10].u32 ) };
	// 82433BF8: 419A0040  beq cr6, 0x82433c38
	if ctx.cr[6].eq {
	pc = 0x82433C38; continue 'dispatch;
	}
	// 82433BFC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82433C00: 419A0024  beq cr6, 0x82433c24
	if ctx.cr[6].eq {
	pc = 0x82433C24; continue 'dispatch;
	}
	// 82433C04: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82433C08: 409A003C  bne cr6, 0x82433c44
	if !ctx.cr[6].eq {
	pc = 0x82433C44; continue 'dispatch;
	}
	// 82433C0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433C10: 80DF0590  lwz r6, 0x590(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1424 as u32) ) } as u64;
	// 82433C14: 80BF058C  lwz r5, 0x58c(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1420 as u32) ) } as u64;
	// 82433C18: 809F0438  lwz r4, 0x438(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 82433C1C: 4BFFFF05  bl 0x82433b20
	ctx.lr = 0x82433C20;
	sub_82433B20(ctx, base);
	// 82433C20: 48000024  b 0x82433c44
	pc = 0x82433C44; continue 'dispatch;
            }
            0x82433C24 => {
    //   block [0x82433C24..0x82433C38)
	// 82433C24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433C28: 80BF0588  lwz r5, 0x588(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1416 as u32) ) } as u64;
	// 82433C2C: 809F0584  lwz r4, 0x584(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1412 as u32) ) } as u64;
	// 82433C30: 4BFFFE59  bl 0x82433a88
	ctx.lr = 0x82433C34;
	sub_82433A88(ctx, base);
	// 82433C34: 48000010  b 0x82433c44
	pc = 0x82433C44; continue 'dispatch;
            }
            0x82433C38 => {
    //   block [0x82433C38..0x82433C44)
	// 82433C38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433C3C: 809F0438  lwz r4, 0x438(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1080 as u32) ) } as u64;
	// 82433C40: 4BFFFD31  bl 0x82433970
	ctx.lr = 0x82433C44;
	sub_82433970(ctx, base);
	pc = 0x82433C44; continue 'dispatch;
            }
            0x82433C44 => {
    //   block [0x82433C44..0x82433C6C)
	// 82433C44: 817F0598  lwz r11, 0x598(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1432 as u32) ) } as u64;
	// 82433C48: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82433C4C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82433C50: 915F0578  stw r10, 0x578(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1400 as u32), ctx.r[10].u32 ) };
	// 82433C54: 917F0598  stw r11, 0x598(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1432 as u32), ctx.r[11].u32 ) };
	// 82433C58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433C5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433C60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433C64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433C70 size=112
    let mut pc: u32 = 0x82433C70;
    'dispatch: loop {
        match pc {
            0x82433C70 => {
    //   block [0x82433C70..0x82433CB0)
	// 82433C70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433C74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433C78: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433C7C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433C80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433C84: 4BFFF3DD  bl 0x82433060
	ctx.lr = 0x82433C88;
	sub_82433060(ctx, base);
	// 82433C88: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433C8C: 419A0024  beq cr6, 0x82433cb0
	if ctx.cr[6].eq {
	pc = 0x82433CB0; continue 'dispatch;
	}
	// 82433C90: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82433C94: 386B4F54  addi r3, r11, 0x4f54
	ctx.r[3].s64 = ctx.r[11].s64 + 20308;
	// 82433C98: 48003431  bl 0x824370c8
	ctx.lr = 0x82433C9C;
	sub_824370C8(ctx, base);
	// 82433C9C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433CA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433CA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433CA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433CAC: 4E800020  blr
	return;
            }
            0x82433CB0 => {
    //   block [0x82433CB0..0x82433CE0)
	// 82433CB0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82433CB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433CB8: 4BFFF8E1  bl 0x82433598
	ctx.lr = 0x82433CBC;
	sub_82433598(ctx, base);
	// 82433CBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433CC0: 4BFFFD69  bl 0x82433a28
	ctx.lr = 0x82433CC4;
	sub_82433A28(ctx, base);
	// 82433CC4: 39600004  li r11, 4
	ctx.r[11].s64 = 4;
	// 82433CC8: 917F0580  stw r11, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[11].u32 ) };
	// 82433CCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433CD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433CD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433CD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433CE0 size=128
    let mut pc: u32 = 0x82433CE0;
    'dispatch: loop {
        match pc {
            0x82433CE0 => {
    //   block [0x82433CE0..0x82433D4C)
	// 82433CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433CE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433CEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433CF0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82433CF4: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82433CF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433CFC: 4BFFF5ED  bl 0x824332e8
	ctx.lr = 0x82433D00;
	sub_824332E8(ctx, base);
	// 82433D00: 3D401062  lis r10, 0x1062
	ctx.r[10].s64 = 274857984;
	// 82433D04: 817F0594  lwz r11, 0x594(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1428 as u32) ) } as u64;
	// 82433D08: 614A4DD3  ori r10, r10, 0x4dd3
	ctx.r[10].u64 = ctx.r[10].u64 | 19923;
	// 82433D0C: 81210050  lwz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82433D10: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433D14: 7D6B5096  mulhw r11, r11, r10
	ctx.r[11].s64 = ((ctx.r[11].s32 as i64 * ctx.r[10].s32 as i64) >> 32);
	// 82433D18: 7D6B3670  srawi r11, r11, 6
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 6) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 6) as i64;
	// 82433D1C: 7D284BD6  divw r9, r8, r9
	ctx.r[9].s32 = ctx.r[8].s32 / ctx.r[9].s32;
	// 82433D20: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82433D24: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82433D28: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82433D2C: 41990020  bgt cr6, 0x82433d4c
	if ctx.cr[6].gt {
	pc = 0x82433D4C; continue 'dispatch;
	}
	// 82433D30: 817F0580  lwz r11, 0x580(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1408 as u32) ) } as u64;
	// 82433D34: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82433D38: 409A0014  bne cr6, 0x82433d4c
	if !ctx.cr[6].eq {
	pc = 0x82433D4C; continue 'dispatch;
	}
	// 82433D3C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433D40: 4BFFFF31  bl 0x82433c70
	ctx.lr = 0x82433D44;
	sub_82433C70(ctx, base);
	// 82433D44: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82433D48: 917F0580  stw r11, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[11].u32 ) };
	pc = 0x82433D4C; continue 'dispatch;
            }
            0x82433D4C => {
    //   block [0x82433D4C..0x82433D60)
	// 82433D4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433D50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433D54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433D58: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433D5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433D60 size=164
    let mut pc: u32 = 0x82433D60;
    'dispatch: loop {
        match pc {
            0x82433D60 => {
    //   block [0x82433D60..0x82433DBC)
	// 82433D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433D68: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433D6C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433D70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433D74: 817F0570  lwz r11, 0x570(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1392 as u32) ) } as u64;
	// 82433D78: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82433D7C: 419A0074  beq cr6, 0x82433df0
	if ctx.cr[6].eq {
	pc = 0x82433DF0; continue 'dispatch;
	}
	// 82433D80: 817F0574  lwz r11, 0x574(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1396 as u32) ) } as u64;
	// 82433D84: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82433D88: 419A0068  beq cr6, 0x82433df0
	if ctx.cr[6].eq {
	pc = 0x82433DF0; continue 'dispatch;
	}
	// 82433D8C: 817F0580  lwz r11, 0x580(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1408 as u32) ) } as u64;
	// 82433D90: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82433D94: 409A0028  bne cr6, 0x82433dbc
	if !ctx.cr[6].eq {
	pc = 0x82433DBC; continue 'dispatch;
	}
	// 82433D98: 4BFFF6E9  bl 0x82433480
	ctx.lr = 0x82433D9C;
	sub_82433480(ctx, base);
	// 82433D9C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82433DA0: 409A001C  bne cr6, 0x82433dbc
	if !ctx.cr[6].eq {
	pc = 0x82433DBC; continue 'dispatch;
	}
	// 82433DA4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433DA8: 4BFFFA79  bl 0x82433820
	ctx.lr = 0x82433DAC;
	sub_82433820(ctx, base);
	// 82433DAC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82433DB0: 41980040  blt cr6, 0x82433df0
	if ctx.cr[6].lt {
	pc = 0x82433DF0; continue 'dispatch;
	}
	// 82433DB4: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82433DB8: 917F0580  stw r11, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[11].u32 ) };
	pc = 0x82433DBC; continue 'dispatch;
            }
            0x82433DBC => {
    //   block [0x82433DBC..0x82433DD0)
	// 82433DBC: 817F0580  lwz r11, 0x580(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1408 as u32) ) } as u64;
	// 82433DC0: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82433DC4: 419A000C  beq cr6, 0x82433dd0
	if ctx.cr[6].eq {
	pc = 0x82433DD0; continue 'dispatch;
	}
	// 82433DC8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82433DCC: 409A0024  bne cr6, 0x82433df0
	if !ctx.cr[6].eq {
	pc = 0x82433DF0; continue 'dispatch;
	}
	pc = 0x82433DD0; continue 'dispatch;
            }
            0x82433DD0 => {
    //   block [0x82433DD0..0x82433DE8)
	// 82433DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433DD4: 4BFFF2A5  bl 0x82433078
	ctx.lr = 0x82433DD8;
	sub_82433078(ctx, base);
	// 82433DD8: 2F030003  cmpwi cr6, r3, 3
	ctx.cr[6].compare_i32(ctx.r[3].s32, 3, &mut ctx.xer);
	// 82433DDC: 409A000C  bne cr6, 0x82433de8
	if !ctx.cr[6].eq {
	pc = 0x82433DE8; continue 'dispatch;
	}
	// 82433DE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433DE4: 4BFFFDE5  bl 0x82433bc8
	ctx.lr = 0x82433DE8;
	sub_82433BC8(ctx, base);
	pc = 0x82433DE8; continue 'dispatch;
            }
            0x82433DE8 => {
    //   block [0x82433DE8..0x82433DF0)
	// 82433DE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82433DEC: 4BFFFEF5  bl 0x82433ce0
	ctx.lr = 0x82433DF0;
	sub_82433CE0(ctx, base);
	pc = 0x82433DF0; continue 'dispatch;
            }
            0x82433DF0 => {
    //   block [0x82433DF0..0x82433E04)
	// 82433DF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433DF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433DF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433DFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433E00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433E08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433E08 size=112
    let mut pc: u32 = 0x82433E08;
    'dispatch: loop {
        match pc {
            0x82433E08 => {
    //   block [0x82433E08..0x82433E38)
	// 82433E08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433E0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433E10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82433E14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433E18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433E1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82433E20: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82433E24: 4BFFE45D  bl 0x82432280
	ctx.lr = 0x82433E28;
	sub_82432280(ctx, base);
	// 82433E28: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433E2C: 39600002  li r11, 2
	ctx.r[11].s64 = 2;
	// 82433E30: 419A0008  beq cr6, 0x82433e38
	if ctx.cr[6].eq {
	pc = 0x82433E38; continue 'dispatch;
	}
	// 82433E34: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	pc = 0x82433E38; continue 'dispatch;
            }
            0x82433E38 => {
    //   block [0x82433E38..0x82433E78)
	// 82433E38: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82433E3C: 917F0050  stw r11, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82433E40: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82433E44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82433E48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82433E4C: 4BFFE7FD  bl 0x82432648
	ctx.lr = 0x82433E50;
	sub_82432648(ctx, base);
	// 82433E50: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82433E54: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82433E58: 917F0054  stw r11, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82433E5C: 915F0058  stw r10, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82433E60: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433E64: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433E68: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433E6C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82433E70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433E74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82433E78 size=20
    let mut pc: u32 = 0x82433E78;
    'dispatch: loop {
        match pc {
            0x82433E78 => {
    //   block [0x82433E78..0x82433E8C)
	// 82433E78: 8163056C  lwz r11, 0x56c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1388 as u32) ) } as u64;
	// 82433E7C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82433E80: 409A000C  bne cr6, 0x82433e8c
	if !ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82433E8C);
		return;
	}
	// 82433E84: 9083056C  stw r4, 0x56c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1388 as u32), ctx.r[4].u32 ) };
	// 82433E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433EA0 size=80
    let mut pc: u32 = 0x82433EA0;
    'dispatch: loop {
        match pc {
            0x82433EA0 => {
    //   block [0x82433EA0..0x82433ED0)
	// 82433EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433EA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433EA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82433EAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82433EB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433EB4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433EB8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82433EBC: 817F0424  lwz r11, 0x424(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1060 as u32) ) } as u64;
	// 82433EC0: 815E0030  lwz r10, 0x30(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82433EC4: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82433EC8: 40980008  bge cr6, 0x82433ed0
	if !ctx.cr[6].lt {
	pc = 0x82433ED0; continue 'dispatch;
	}
	// 82433ECC: 4BFFE73D  bl 0x82432608
	ctx.lr = 0x82433ED0;
	sub_82432608(ctx, base);
	pc = 0x82433ED0; continue 'dispatch;
            }
            0x82433ED0 => {
    //   block [0x82433ED0..0x82433EF0)
	// 82433ED0: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 82433ED4: 917F0424  stw r11, 0x424(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1060 as u32), ctx.r[11].u32 ) };
	// 82433ED8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82433EDC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433EE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433EE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82433EE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82433EEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433EF0 size=120
    let mut pc: u32 = 0x82433EF0;
    'dispatch: loop {
        match pc {
            0x82433EF0 => {
    //   block [0x82433EF0..0x82433F20)
	// 82433EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433EF4: 481011C1  bl 0x825350b4
	ctx.lr = 0x82433EF8;
	sub_82535080(ctx, base);
	// 82433EF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433EFC: 81640038  lwz r11, 0x38(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 82433F00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82433F04: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82433F08: 837F040C  lwz r27, 0x40c(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1036 as u32) ) } as u64;
	// 82433F0C: 83AB0000  lwz r29, 0(r11)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82433F10: 838B0004  lwz r28, 4(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82433F14: 4800299D  bl 0x824368b0
	ctx.lr = 0x82433F18;
	sub_824368B0(ctx, base);
	// 82433F18: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433F1C: 419A0018  beq cr6, 0x82433f34
	if ctx.cr[6].eq {
	pc = 0x82433F34; continue 'dispatch;
	}
	pc = 0x82433F20; continue 'dispatch;
            }
            0x82433F20 => {
    //   block [0x82433F20..0x82433F34)
	// 82433F20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82433F24: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82433F28: 917E0048  stw r11, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82433F2C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82433F30: 481011D4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82433F34 => {
    //   block [0x82433F34..0x82433F58)
	// 82433F34: 397F03F4  addi r11, r31, 0x3f4
	ctx.r[11].s64 = ctx.r[31].s64 + 1012;
	// 82433F38: 7F1B5840  cmplw cr6, r27, r11
	ctx.cr[6].compare_u32(ctx.r[27].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82433F3C: 419AFFE4  beq cr6, 0x82433f20
	if ctx.cr[6].eq {
	pc = 0x82433F20; continue 'dispatch;
	}
	// 82433F40: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82433F44: 419A0014  beq cr6, 0x82433f58
	if ctx.cr[6].eq {
	pc = 0x82433F58; continue 'dispatch;
	}
	// 82433F48: 2F1C0004  cmpwi cr6, r28, 4
	ctx.cr[6].compare_i32(ctx.r[28].s32, 4, &mut ctx.xer);
	// 82433F4C: 4099000C  ble cr6, 0x82433f58
	if !ctx.cr[6].gt {
	pc = 0x82433F58; continue 'dispatch;
	}
	// 82433F50: 3BBD0004  addi r29, r29, 4
	ctx.r[29].s64 = ctx.r[29].s64 + 4;
	// 82433F54: 3B9CFFFC  addi r28, r28, -4
	ctx.r[28].s64 = ctx.r[28].s64 + -4;
	pc = 0x82433F58; continue 'dispatch;
            }
            0x82433F58 => {
    //   block [0x82433F58..0x82433F68)
	// 82433F58: 93BE0044  stw r29, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[29].u32 ) };
	// 82433F5C: 939E0048  stw r28, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[28].u32 ) };
	// 82433F60: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82433F64: 481011A0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82433F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82433F68 size=208
    let mut pc: u32 = 0x82433F68;
    'dispatch: loop {
        match pc {
            0x82433F68 => {
    //   block [0x82433F68..0x82433F88)
	// 82433F68: 1CA503E8  mulli r5, r5, 0x3e8
	ctx.r[5].s32 = ((ctx.r[5].s32 as i64 * 1000 as i64) as i32);
	ctx.r[5].s64 = ctx.r[5].s32 as i64;
	// 82433F6C: 4801060C  b 0x82444578
	sub_82444578(ctx, base);
	return;
	// 82433F70: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82433F74: 419A001C  beq cr6, 0x82433f90
	if ctx.cr[6].eq {
	pc = 0x82433F90; continue 'dispatch;
	}
	// 82433F78: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82433F7C: 419A000C  beq cr6, 0x82433f88
	if ctx.cr[6].eq {
	pc = 0x82433F88; continue 'dispatch;
	}
	// 82433F80: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82433F84: 4E800020  blr
	return;
            }
            0x82433F88 => {
    //   block [0x82433F88..0x82433F90)
	// 82433F88: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82433F8C: 4E800020  blr
	return;
            }
            0x82433F90 => {
    //   block [0x82433F90..0x82433FD8)
	// 82433F90: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82433F94: 4E800020  blr
	return;
	// 82433F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82433F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82433FA0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82433FA4: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 82433FA8: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 82433FAC: 41990068  bgt cr6, 0x82434014
	if ctx.cr[6].gt {
	pc = 0x82434014; continue 'dispatch;
	}
	// 82433FB0: 3D808243  lis r12, -0x7dbd
	ctx.r[12].s64 = -2109538304;
	// 82433FB4: 398C3FC8  addi r12, r12, 0x3fc8
	ctx.r[12].s64 = ctx.r[12].s64 + 16328;
	// 82433FB8: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82433FBC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82433FC0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82433FC4: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82434020; continue 'dispatch;
		},
		1 => {
	pc = 0x82433FD8; continue 'dispatch;
		},
		2 => {
	pc = 0x82433FEC; continue 'dispatch;
		},
		3 => {
	pc = 0x82434000; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82433FC8: 82434020  lwz r18, 0x4020(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16416 as u32) ) } as u64;
	// 82433FCC: 82433FD8  lwz r18, 0x3fd8(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16344 as u32) ) } as u64;
	// 82433FD0: 82433FEC  lwz r18, 0x3fec(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16364 as u32) ) } as u64;
	// 82433FD4: 82434000  lwz r18, 0x4000(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16384 as u32) ) } as u64;
            }
            0x82433FD8 => {
    //   block [0x82433FD8..0x82433FEC)
	// 82433FD8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82433FDC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433FE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433FE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433FE8: 4E800020  blr
	return;
            }
            0x82433FEC => {
    //   block [0x82433FEC..0x82434000)
	// 82433FEC: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82433FF0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82433FF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82433FF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82433FFC: 4E800020  blr
	return;
            }
            0x82434000 => {
    //   block [0x82434000..0x82434014)
	// 82434000: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82434004: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243400C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434010: 4E800020  blr
	return;
            }
            0x82434014 => {
    //   block [0x82434014..0x82434020)
	// 82434014: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82434018: 386B4FBC  addi r3, r11, 0x4fbc
	ctx.r[3].s64 = ctx.r[11].s64 + 20412;
	// 8243401C: 480030AD  bl 0x824370c8
	ctx.lr = 0x82434020;
	sub_824370C8(ctx, base);
	pc = 0x82434020; continue 'dispatch;
            }
            0x82434020 => {
    //   block [0x82434020..0x82434038)
	// 82434020: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82434024: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434028: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243402C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434038 size=132
    let mut pc: u32 = 0x82434038;
    'dispatch: loop {
        match pc {
            0x82434038 => {
    //   block [0x82434038..0x82434078)
	// 82434038: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243403C: 48101081  bl 0x825350bc
	ctx.lr = 0x82434040;
	sub_82535080(ctx, base);
	// 82434040: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434044: 81640058  lwz r11, 0x58(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(88 as u32) ) } as u64;
	// 82434048: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8243404C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82434050: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82434054: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82434058: 40990028  ble cr6, 0x82434080
	if !ctx.cr[6].gt {
	pc = 0x82434080; continue 'dispatch;
	}
	// 8243405C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82434060: 40990018  ble cr6, 0x82434078
	if !ctx.cr[6].gt {
	pc = 0x82434078; continue 'dispatch;
	}
	// 82434064: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82434068: 409A0018  bne cr6, 0x82434080
	if !ctx.cr[6].eq {
	pc = 0x82434080; continue 'dispatch;
	}
	// 8243406C: 8964006C  lbz r11, 0x6c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(108 as u32) ) } as u64;
	// 82434070: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82434074: 409A0018  bne cr6, 0x8243408c
	if !ctx.cr[6].eq {
	pc = 0x8243408C; continue 'dispatch;
	}
	pc = 0x82434078; continue 'dispatch;
            }
            0x82434078 => {
    //   block [0x82434078..0x82434080)
	// 82434078: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 8243407C: 48000010  b 0x8243408c
	pc = 0x8243408C; continue 'dispatch;
            }
            0x82434080 => {
    //   block [0x82434080..0x8243408C)
	// 82434080: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82434084: 386B4FEC  addi r3, r11, 0x4fec
	ctx.r[3].s64 = ctx.r[11].s64 + 20460;
	// 82434088: 48003041  bl 0x824370c8
	ctx.lr = 0x8243408C;
	sub_824370C8(ctx, base);
	pc = 0x8243408C; continue 'dispatch;
            }
            0x8243408C => {
    //   block [0x8243408C..0x824340B0)
	// 8243408C: 48002825  bl 0x824368b0
	ctx.lr = 0x82434090;
	sub_824368B0(ctx, base);
	// 82434090: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434094: 409A001C  bne cr6, 0x824340b0
	if !ctx.cr[6].eq {
	pc = 0x824340B0; continue 'dispatch;
	}
	// 82434098: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8243409C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824340A0: 48004411  bl 0x824384b0
	ctx.lr = 0x824340A4;
	sub_824384B0(ctx, base);
	// 824340A4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824340A8: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 824340AC: 419A0008  beq cr6, 0x824340b4
	if ctx.cr[6].eq {
	pc = 0x824340B4; continue 'dispatch;
	}
	pc = 0x824340B0; continue 'dispatch;
            }
            0x824340B0 => {
    //   block [0x824340B0..0x824340B4)
	// 824340B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	pc = 0x824340B4; continue 'dispatch;
            }
            0x824340B4 => {
    //   block [0x824340B4..0x824340BC)
	// 824340B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824340B8: 48101054  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824340C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824340C0 size=80
    let mut pc: u32 = 0x824340C0;
    'dispatch: loop {
        match pc {
            0x824340C0 => {
    //   block [0x824340C0..0x82434110)
	// 824340C0: 81640058  lwz r11, 0x58(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(88 as u32) ) } as u64;
	// 824340C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824340C8: 91630098  stw r11, 0x98(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 824340CC: 8164005C  lwz r11, 0x5c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(92 as u32) ) } as u64;
	// 824340D0: 9163009C  stw r11, 0x9c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 824340D4: 8964006C  lbz r11, 0x6c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(108 as u32) ) } as u64;
	// 824340D8: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824340DC: 916300A0  stw r11, 0xa0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(160 as u32), ctx.r[11].u32 ) };
	// 824340E0: 8964006D  lbz r11, 0x6d(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(109 as u32) ) } as u64;
	// 824340E4: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824340E8: 916300A4  stw r11, 0xa4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(164 as u32), ctx.r[11].u32 ) };
	// 824340EC: 8964006E  lbz r11, 0x6e(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(110 as u32) ) } as u64;
	// 824340F0: 7D6B0774  extsb r11, r11
	ctx.r[11].s64 = ctx.r[11].s8 as i64;
	// 824340F4: 916300A8  stw r11, 0xa8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 824340F8: 8164003C  lwz r11, 0x3c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(60 as u32) ) } as u64;
	// 824340FC: 916300AC  stw r11, 0xac(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(172 as u32), ctx.r[11].u32 ) };
	// 82434100: 81640040  lwz r11, 0x40(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(64 as u32) ) } as u64;
	// 82434104: 914300B4  stw r10, 0xb4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 82434108: 916300B0  stw r11, 0xb0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 8243410C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434110 size=140
    let mut pc: u32 = 0x82434110;
    'dispatch: loop {
        match pc {
            0x82434110 => {
    //   block [0x82434110..0x82434188)
	// 82434110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434118: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243411C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434120: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82434124: 81440038  lwz r10, 0x38(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(56 as u32) ) } as u64;
	// 82434128: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8243412C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82434130: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82434134: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82434138: 814A0004  lwz r10, 4(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243413C: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82434140: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82434144: 917F009C  stw r11, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 82434148: 419A0040  beq cr6, 0x82434188
	if ctx.cr[6].eq {
	pc = 0x82434188; continue 'dispatch;
	}
	// 8243414C: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 82434150: 40990038  ble cr6, 0x82434188
	if !ctx.cr[6].gt {
	pc = 0x82434188; continue 'dispatch;
	}
	// 82434154: 38C10054  addi r6, r1, 0x54
	ctx.r[6].s64 = ctx.r[1].s64 + 84;
	// 82434158: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8243415C: 388AFFFC  addi r4, r10, -4
	ctx.r[4].s64 = ctx.r[10].s64 + -4;
	// 82434160: 38690004  addi r3, r9, 4
	ctx.r[3].s64 = ctx.r[9].s64 + 4;
	// 82434164: 4800445D  bl 0x824385c0
	ctx.lr = 0x82434168;
	sub_824385C0(ctx, base);
	// 82434168: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243416C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82434170: 419A0018  beq cr6, 0x82434188
	if ctx.cr[6].eq {
	pc = 0x82434188; continue 'dispatch;
	}
	// 82434174: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82434178: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8243417C: 4099000C  ble cr6, 0x82434188
	if !ctx.cr[6].gt {
	pc = 0x82434188; continue 'dispatch;
	}
	// 82434180: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 82434184: 915F009C  stw r10, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[10].u32 ) };
	pc = 0x82434188; continue 'dispatch;
            }
            0x82434188 => {
    //   block [0x82434188..0x8243419C)
	// 82434188: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243418C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434190: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434194: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82434198: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824341A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824341A0 size=64
    let mut pc: u32 = 0x824341A0;
    'dispatch: loop {
        match pc {
            0x824341A0 => {
    //   block [0x824341A0..0x824341E0)
	// 824341A0: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 824341A4: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824341A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824341AC: 91640004  stw r11, 4(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 824341B0: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824341B4: 91640008  stw r11, 8(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 824341B8: A163000E  lhz r11, 0xe(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(14 as u32) ) } as u64;
	// 824341BC: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824341C0: 9164000C  stw r11, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 824341C4: A163000C  lhz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824341C8: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824341CC: 91640010  stw r11, 0x10(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824341D0: A163000C  lhz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824341D4: 7D6B0734  extsh r11, r11
	ctx.r[11].s64 = ctx.r[11].s16 as i64;
	// 824341D8: 91640014  stw r11, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824341DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824341E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824341E0 size=120
    let mut pc: u32 = 0x824341E0;
    'dispatch: loop {
        match pc {
            0x824341E0 => {
    //   block [0x824341E0..0x82434210)
	// 824341E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824341E4: 48100ED5  bl 0x825350b8
	ctx.lr = 0x824341E8;
	sub_82535080(ctx, base);
	// 824341E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824341EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824341F0: 4BFFEE71  bl 0x82433060
	ctx.lr = 0x824341F4;
	sub_82433060(ctx, base);
	// 824341F4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824341F8: 419A0018  beq cr6, 0x82434210
	if ctx.cr[6].eq {
	pc = 0x82434210; continue 'dispatch;
	}
	// 824341FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82434200: 386B5020  addi r3, r11, 0x5020
	ctx.r[3].s64 = ctx.r[11].s64 + 20512;
	// 82434204: 48002EC5  bl 0x824370c8
	ctx.lr = 0x82434208;
	sub_824370C8(ctx, base);
	// 82434208: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243420C: 48100EFC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82434210 => {
    //   block [0x82434210..0x82434250)
	// 82434210: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82434214: 83DF0088  lwz r30, 0x88(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(136 as u32) ) } as u64;
	// 82434218: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243421C: 83BF008C  lwz r29, 0x8c(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82434220: 839F0090  lwz r28, 0x90(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82434224: 4BFFFC55  bl 0x82433e78
	ctx.lr = 0x82434228;
	sub_82433E78(ctx, base);
	// 82434228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243422C: 4BFFF205  bl 0x82433430
	ctx.lr = 0x82434230;
	sub_82433430(ctx, base);
	// 82434230: 7F1DE000  cmpw cr6, r29, r28
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82434234: 4099001C  ble cr6, 0x82434250
	if !ctx.cr[6].gt {
	pc = 0x82434250; continue 'dispatch;
	}
	// 82434238: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243423C: 480064E5  bl 0x8243a720
	ctx.lr = 0x82434240;
	sub_8243A720(ctx, base);
	// 82434240: 817F0090  lwz r11, 0x90(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(144 as u32) ) } as u64;
	// 82434244: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82434248: 917F0090  stw r11, 0x90(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(144 as u32), ctx.r[11].u32 ) };
	// 8243424C: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	pc = 0x82434250; continue 'dispatch;
            }
            0x82434250 => {
    //   block [0x82434250..0x82434258)
	// 82434250: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82434254: 48100EB4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434258 size=108
    let mut pc: u32 = 0x82434258;
    'dispatch: loop {
        match pc {
            0x82434258 => {
    //   block [0x82434258..0x82434284)
	// 82434258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243425C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434260: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82434264: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434268: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243426C: 4BFFEDF5  bl 0x82433060
	ctx.lr = 0x82434270;
	sub_82433060(ctx, base);
	// 82434270: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434274: 419A0028  beq cr6, 0x8243429c
	if ctx.cr[6].eq {
	pc = 0x8243429C; continue 'dispatch;
	}
	// 82434278: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243427C: 386B504C  addi r3, r11, 0x504c
	ctx.r[3].s64 = ctx.r[11].s64 + 20556;
	// 82434280: 48002E49  bl 0x824370c8
	ctx.lr = 0x82434284;
	sub_824370C8(ctx, base);
	pc = 0x82434284; continue 'dispatch;
            }
            0x82434284 => {
    //   block [0x82434284..0x8243429C)
	// 82434284: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82434288: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243428C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434290: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434294: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82434298: 4E800020  blr
	return;
            }
            0x8243429C => {
    //   block [0x8243429C..0x824342C4)
	// 8243429C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824342A0: 4BFFF191  bl 0x82433430
	ctx.lr = 0x824342A4;
	sub_82433430(ctx, base);
	// 824342A4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824342A8: 419AFFDC  beq cr6, 0x82434284
	if ctx.cr[6].eq {
	pc = 0x82434284; continue 'dispatch;
	}
	// 824342AC: 48010FFD  bl 0x824452a8
	ctx.lr = 0x824342B0;
	sub_824452A8(ctx, base);
	// 824342B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824342B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824342B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824342BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824342C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824342C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824342C8 size=52
    let mut pc: u32 = 0x824342C8;
    'dispatch: loop {
        match pc {
            0x824342C8 => {
    //   block [0x824342C8..0x824342EC)
	// 824342C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824342CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824342D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824342D4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824342D8: 48011539  bl 0x82445810
	ctx.lr = 0x824342DC;
	sub_82445810(ctx, base);
	// 824342DC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824342E0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824342E4: 409A0008  bne cr6, 0x824342ec
	if !ctx.cr[6].eq {
	pc = 0x824342EC; continue 'dispatch;
	}
	// 824342E8: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	pc = 0x824342EC; continue 'dispatch;
            }
            0x824342EC => {
    //   block [0x824342EC..0x824342FC)
	// 824342EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824342F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824342F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824342F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434300 size=52
    let mut pc: u32 = 0x82434300;
    'dispatch: loop {
        match pc {
            0x82434300 => {
    //   block [0x82434300..0x82434324)
	// 82434300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434308: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243430C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82434310: 48011529  bl 0x82445838
	ctx.lr = 0x82434314;
	sub_82445838(ctx, base);
	// 82434314: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434318: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243431C: 409A0008  bne cr6, 0x82434324
	if !ctx.cr[6].eq {
	pc = 0x82434324; continue 'dispatch;
	}
	// 82434320: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	pc = 0x82434324; continue 'dispatch;
            }
            0x82434324 => {
    //   block [0x82434324..0x82434334)
	// 82434324: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434328: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243432C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434338 size=72
    let mut pc: u32 = 0x82434338;
    'dispatch: loop {
        match pc {
            0x82434338 => {
    //   block [0x82434338..0x82434368)
	// 82434338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243433C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434340: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434344: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82434348: 48011451  bl 0x82445798
	ctx.lr = 0x8243434C;
	sub_82445798(ctx, base);
	// 8243434C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434350: 419A0018  beq cr6, 0x82434368
	if ctx.cr[6].eq {
	pc = 0x82434368; continue 'dispatch;
	}
	// 82434354: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82434358: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243435C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434360: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434364: 4E800020  blr
	return;
            }
            0x82434368 => {
    //   block [0x82434368..0x82434380)
	// 82434368: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243436C: 55631838  slwi r3, r11, 3
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82434370: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434374: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434378: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243437C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434380 size=140
    let mut pc: u32 = 0x82434380;
    'dispatch: loop {
        match pc {
            0x82434380 => {
    //   block [0x82434380..0x824343A0)
	// 82434380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434388: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243438C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82434390: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 82434394: 48011725  bl 0x82445ab8
	ctx.lr = 0x82434398;
	sub_82445AB8(ctx, base);
	// 82434398: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243439C: 419A0018  beq cr6, 0x824343b4
	if ctx.cr[6].eq {
	pc = 0x824343B4; continue 'dispatch;
	}
	pc = 0x824343A0; continue 'dispatch;
            }
            0x824343A0 => {
    //   block [0x824343A0..0x824343B4)
	// 824343A0: 38600011  li r3, 0x11
	ctx.r[3].s64 = 17;
	// 824343A4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824343A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824343AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824343B0: 4E800020  blr
	return;
            }
            0x824343B4 => {
    //   block [0x824343B4..0x824343E4)
	// 824343B4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824343B8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824343BC: 419A003C  beq cr6, 0x824343f8
	if ctx.cr[6].eq {
	pc = 0x824343F8; continue 'dispatch;
	}
	// 824343C0: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 824343C4: 419A0020  beq cr6, 0x824343e4
	if ctx.cr[6].eq {
	pc = 0x824343E4; continue 'dispatch;
	}
	// 824343C8: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 824343CC: 409AFFD4  bne cr6, 0x824343a0
	if !ctx.cr[6].eq {
	pc = 0x824343A0; continue 'dispatch;
	}
	// 824343D0: 38600061  li r3, 0x61
	ctx.r[3].s64 = 97;
	// 824343D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824343D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824343DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824343E0: 4E800020  blr
	return;
            }
            0x824343E4 => {
    //   block [0x824343E4..0x824343F8)
	// 824343E4: 38600051  li r3, 0x51
	ctx.r[3].s64 = 81;
	// 824343E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824343EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824343F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824343F4: 4E800020  blr
	return;
            }
            0x824343F8 => {
    //   block [0x824343F8..0x8243440C)
	// 824343F8: 38600021  li r3, 0x21
	ctx.r[3].s64 = 33;
	// 824343FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434408: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82434410 size=112
    let mut pc: u32 = 0x82434410;
    'dispatch: loop {
        match pc {
            0x82434410 => {
    //   block [0x82434410..0x8243442C)
	// 82434410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82434414: 394300D4  addi r10, r3, 0xd4
	ctx.r[10].s64 = ctx.r[3].s64 + 212;
	// 82434418: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8243441C: 39000011  li r8, 0x11
	ctx.r[8].s64 = 17;
	// 82434420: 916300C4  stw r11, 0xc4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 82434424: 916300C8  stw r11, 0xc8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 82434428: 916300CC  stw r11, 0xcc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	pc = 0x8243442C; continue 'dispatch;
            }
            0x8243442C => {
    //   block [0x8243442C..0x8243445C)
	// 8243442C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82434430: 916AFFFC  stw r11, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82434434: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434438: 916A000C  stw r11, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8243443C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82434440: 916A0010  stw r11, 0x10(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82434444: 910A0014  stw r8, 0x14(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(20 as u32), ctx.r[8].u32 ) };
	// 82434448: 394A0024  addi r10, r10, 0x24
	ctx.r[10].s64 = ctx.r[10].s64 + 36;
	// 8243444C: 409AFFE0  bne cr6, 0x8243442c
	if !ctx.cr[6].eq {
	pc = 0x8243442C; continue 'dispatch;
	}
	// 82434450: 394301F8  addi r10, r3, 0x1f8
	ctx.r[10].s64 = ctx.r[3].s64 + 504;
	// 82434454: 916301F0  stw r11, 0x1f0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(496 as u32), ctx.r[11].u32 ) };
	// 82434458: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	pc = 0x8243445C; continue 'dispatch;
            }
            0x8243445C => {
    //   block [0x8243445C..0x82434480)
	// 8243445C: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82434460: 916AFFFC  stw r11, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[11].u32 ) };
	// 82434464: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434468: 916A0004  stw r11, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8243446C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82434470: 916A0008  stw r11, 8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82434474: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82434478: 409AFFE4  bne cr6, 0x8243445c
	if !ctx.cr[6].eq {
	pc = 0x8243445C; continue 'dispatch;
	}
	// 8243447C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82434480 size=108
    let mut pc: u32 = 0x82434480;
    'dispatch: loop {
        match pc {
            0x82434480 => {
    //   block [0x82434480..0x824344C4)
	// 82434480: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 82434484: 2B0B0007  cmplwi cr6, r11, 7
	ctx.cr[6].compare_u32(ctx.r[11].u32, 7 as u32, &mut ctx.xer);
	// 82434488: 41990064  bgt cr6, 0x824344ec
	if ctx.cr[6].gt {
		sub_824344EC(ctx, base);
		return;
	}
	// 8243448C: 3D808243  lis r12, -0x7dbd
	ctx.r[12].s64 = -2109538304;
	// 82434490: 398C44A4  addi r12, r12, 0x44a4
	ctx.r[12].s64 = ctx.r[12].s64 + 17572;
	// 82434494: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82434498: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8243449C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 824344A0: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x824344C4; continue 'dispatch;
		},
		1 => {
	pc = 0x824344CC; continue 'dispatch;
		},
		2 => {
	pc = 0x824344CC; continue 'dispatch;
		},
		3 => {
	pc = 0x824344CC; continue 'dispatch;
		},
		4 => {
	pc = 0x824344CC; continue 'dispatch;
		},
		5 => {
	pc = 0x824344D4; continue 'dispatch;
		},
		6 => {
	pc = 0x824344DC; continue 'dispatch;
		},
		7 => {
	pc = 0x824344E4; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 824344A4: 824344C4  lwz r18, 0x44c4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17604 as u32) ) } as u64;
	// 824344A8: 824344CC  lwz r18, 0x44cc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17612 as u32) ) } as u64;
	// 824344AC: 824344CC  lwz r18, 0x44cc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17612 as u32) ) } as u64;
	// 824344B0: 824344CC  lwz r18, 0x44cc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17612 as u32) ) } as u64;
	// 824344B4: 824344CC  lwz r18, 0x44cc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17612 as u32) ) } as u64;
	// 824344B8: 824344D4  lwz r18, 0x44d4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17620 as u32) ) } as u64;
	// 824344BC: 824344DC  lwz r18, 0x44dc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17628 as u32) ) } as u64;
	// 824344C0: 824344E4  lwz r18, 0x44e4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(17636 as u32) ) } as u64;
            }
            0x824344C4 => {
    //   block [0x824344C4..0x824344CC)
	// 824344C4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824344C8: 4E800020  blr
	return;
            }
            0x824344CC => {
    //   block [0x824344CC..0x824344D4)
	// 824344CC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 824344D0: 4E800020  blr
	return;
            }
            0x824344D4 => {
    //   block [0x824344D4..0x824344DC)
	// 824344D4: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 824344D8: 4E800020  blr
	return;
            }
            0x824344DC => {
    //   block [0x824344DC..0x824344E4)
	// 824344DC: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 824344E0: 4E800020  blr
	return;
            }
            0x824344E4 => {
    //   block [0x824344E4..0x824344EC)
	// 824344E4: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 824344E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824344EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824344EC size=8
    let mut pc: u32 = 0x824344EC;
    'dispatch: loop {
        match pc {
            0x824344EC => {
    //   block [0x824344EC..0x824344F4)
	// 824344EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824344F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824344F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824344F8 size=500
    let mut pc: u32 = 0x824344F8;
    'dispatch: loop {
        match pc {
            0x824344F8 => {
    //   block [0x824344F8..0x82434554)
	// 824344F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824344FC: 48100BB9  bl 0x825350b4
	ctx.lr = 0x82434500;
	sub_82535080(ctx, base);
	// 82434500: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434504: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82434508: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8243450C: 3BA0FFFF  li r29, -1
	ctx.r[29].s64 = -1;
	// 82434510: 817F0518  lwz r11, 0x518(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1304 as u32) ) } as u64;
	// 82434514: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434518: 409A0050  bne cr6, 0x82434568
	if !ctx.cr[6].eq {
	pc = 0x82434568; continue 'dispatch;
	}
	// 8243451C: 817F0530  lwz r11, 0x530(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1328 as u32) ) } as u64;
	// 82434520: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434524: 409A0044  bne cr6, 0x82434568
	if !ctx.cr[6].eq {
	pc = 0x82434568; continue 'dispatch;
	}
	// 82434528: 83BF0534  lwz r29, 0x534(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1332 as u32) ) } as u64;
	// 8243452C: 57AB2036  slwi r11, r29, 4
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82434530: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82434534: 814B01F4  lwz r10, 0x1f4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(500 as u32) ) } as u64;
	// 82434538: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 8243453C: 409A002C  bne cr6, 0x82434568
	if !ctx.cr[6].eq {
	pc = 0x82434568; continue 'dispatch;
	}
	// 82434540: 816B01F8  lwz r11, 0x1f8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(504 as u32) ) } as u64;
	// 82434544: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 82434548: 419A000C  beq cr6, 0x82434554
	if ctx.cr[6].eq {
	pc = 0x82434554; continue 'dispatch;
	}
	// 8243454C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82434550: 409A0018  bne cr6, 0x82434568
	if !ctx.cr[6].eq {
	pc = 0x82434568; continue 'dispatch;
	}
	pc = 0x82434554; continue 'dispatch;
            }
            0x82434554 => {
    //   block [0x82434554..0x82434568)
	// 82434554: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82434558: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243455C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434560: 48006F91  bl 0x8243b4f0
	ctx.lr = 0x82434564;
	sub_8243B4F0(ctx, base);
	// 82434564: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	pc = 0x82434568; continue 'dispatch;
            }
            0x82434568 => {
    //   block [0x82434568..0x824345B4)
	// 82434568: 817F053C  lwz r11, 0x53c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1340 as u32) ) } as u64;
	// 8243456C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82434570: 3BC0FFFF  li r30, -1
	ctx.r[30].s64 = -1;
	// 82434574: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434578: 409A003C  bne cr6, 0x824345b4
	if !ctx.cr[6].eq {
	pc = 0x824345B4; continue 'dispatch;
	}
	// 8243457C: 817F0554  lwz r11, 0x554(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1364 as u32) ) } as u64;
	// 82434580: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434584: 409A0030  bne cr6, 0x824345b4
	if !ctx.cr[6].eq {
	pc = 0x824345B4; continue 'dispatch;
	}
	// 82434588: 83DF0558  lwz r30, 0x558(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1368 as u32) ) } as u64;
	// 8243458C: 57CB2036  slwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82434590: 7D6BFA14  add r11, r11, r31
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82434594: 816B01F4  lwz r11, 0x1f4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(500 as u32) ) } as u64;
	// 82434598: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243459C: 409A0018  bne cr6, 0x824345b4
	if !ctx.cr[6].eq {
	pc = 0x824345B4; continue 'dispatch;
	}
	// 824345A0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824345A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824345A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824345AC: 48006F45  bl 0x8243b4f0
	ctx.lr = 0x824345B0;
	sub_8243B4F0(ctx, base);
	// 824345B0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	pc = 0x824345B4; continue 'dispatch;
            }
            0x824345B4 => {
    //   block [0x824345B4..0x824345C0)
	// 824345B4: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824345B8: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 824345BC: 393F01F8  addi r9, r31, 0x1f8
	ctx.r[9].s64 = ctx.r[31].s64 + 504;
	pc = 0x824345C0; continue 'dispatch;
            }
            0x824345C0 => {
    //   block [0x824345C0..0x824345F0)
	// 824345C0: 8169FFFC  lwz r11, -4(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(-4 as u32) ) } as u64;
	// 824345C4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824345C8: 409A0028  bne cr6, 0x824345f0
	if !ctx.cr[6].eq {
	pc = 0x824345F0; continue 'dispatch;
	}
	// 824345CC: 396AFFFE  addi r11, r10, -2
	ctx.r[11].s64 = ctx.r[10].s64 + -2;
	// 824345D0: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 824345D4: 419A001C  beq cr6, 0x824345f0
	if ctx.cr[6].eq {
	pc = 0x824345F0; continue 'dispatch;
	}
	// 824345D8: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 824345DC: 419A0014  beq cr6, 0x824345f0
	if ctx.cr[6].eq {
	pc = 0x824345F0; continue 'dispatch;
	}
	// 824345E0: 81690000  lwz r11, 0(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 824345E4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824345E8: 409A0008  bne cr6, 0x824345f0
	if !ctx.cr[6].eq {
	pc = 0x824345F0; continue 'dispatch;
	}
	// 824345EC: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	pc = 0x824345F0; continue 'dispatch;
            }
            0x824345F0 => {
    //   block [0x824345F0..0x82434620)
	// 824345F0: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 824345F4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824345F8: 409A0028  bne cr6, 0x82434620
	if !ctx.cr[6].eq {
	pc = 0x82434620; continue 'dispatch;
	}
	// 824345FC: 396AFFFF  addi r11, r10, -1
	ctx.r[11].s64 = ctx.r[10].s64 + -1;
	// 82434600: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82434604: 419A001C  beq cr6, 0x82434620
	if ctx.cr[6].eq {
	pc = 0x82434620; continue 'dispatch;
	}
	// 82434608: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 8243460C: 419A0014  beq cr6, 0x82434620
	if ctx.cr[6].eq {
	pc = 0x82434620; continue 'dispatch;
	}
	// 82434610: 81690010  lwz r11, 0x10(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82434614: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434618: 409A0008  bne cr6, 0x82434620
	if !ctx.cr[6].eq {
	pc = 0x82434620; continue 'dispatch;
	}
	// 8243461C: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	pc = 0x82434620; continue 'dispatch;
            }
            0x82434620 => {
    //   block [0x82434620..0x8243464C)
	// 82434620: 8169001C  lwz r11, 0x1c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(28 as u32) ) } as u64;
	// 82434624: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434628: 409A0024  bne cr6, 0x8243464c
	if !ctx.cr[6].eq {
	pc = 0x8243464C; continue 'dispatch;
	}
	// 8243462C: 7F0AE800  cmpw cr6, r10, r29
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82434630: 419A001C  beq cr6, 0x8243464c
	if ctx.cr[6].eq {
	pc = 0x8243464C; continue 'dispatch;
	}
	// 82434634: 7F0AF000  cmpw cr6, r10, r30
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82434638: 419A0014  beq cr6, 0x8243464c
	if ctx.cr[6].eq {
	pc = 0x8243464C; continue 'dispatch;
	}
	// 8243463C: 81690020  lwz r11, 0x20(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 82434640: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434644: 409A0008  bne cr6, 0x8243464c
	if !ctx.cr[6].eq {
	pc = 0x8243464C; continue 'dispatch;
	}
	// 82434648: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	pc = 0x8243464C; continue 'dispatch;
            }
            0x8243464C => {
    //   block [0x8243464C..0x8243467C)
	// 8243464C: 8169002C  lwz r11, 0x2c(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(44 as u32) ) } as u64;
	// 82434650: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434654: 409A0028  bne cr6, 0x8243467c
	if !ctx.cr[6].eq {
	pc = 0x8243467C; continue 'dispatch;
	}
	// 82434658: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 8243465C: 7F0BE800  cmpw cr6, r11, r29
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[29].s32, &mut ctx.xer);
	// 82434660: 419A001C  beq cr6, 0x8243467c
	if ctx.cr[6].eq {
	pc = 0x8243467C; continue 'dispatch;
	}
	// 82434664: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82434668: 419A0014  beq cr6, 0x8243467c
	if ctx.cr[6].eq {
	pc = 0x8243467C; continue 'dispatch;
	}
	// 8243466C: 81690030  lwz r11, 0x30(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(48 as u32) ) } as u64;
	// 82434670: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434674: 409A0008  bne cr6, 0x8243467c
	if !ctx.cr[6].eq {
	pc = 0x8243467C; continue 'dispatch;
	}
	// 82434678: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	pc = 0x8243467C; continue 'dispatch;
            }
            0x8243467C => {
    //   block [0x8243467C..0x824346A0)
	// 8243467C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82434680: 39290040  addi r9, r9, 0x40
	ctx.r[9].s64 = ctx.r[9].s64 + 64;
	// 82434684: 396AFFFE  addi r11, r10, -2
	ctx.r[11].s64 = ctx.r[10].s64 + -2;
	// 82434688: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 8243468C: 4198FF34  blt cr6, 0x824345c0
	if ctx.cr[6].lt {
	pc = 0x824345C0; continue 'dispatch;
	}
	// 82434690: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 82434694: 419A000C  beq cr6, 0x824346a0
	if ctx.cr[6].eq {
	pc = 0x824346A0; continue 'dispatch;
	}
	// 82434698: 2F080001  cmpwi cr6, r8, 1
	ctx.cr[6].compare_i32(ctx.r[8].s32, 1, &mut ctx.xer);
	// 8243469C: 409A0014  bne cr6, 0x824346b0
	if !ctx.cr[6].eq {
	pc = 0x824346B0; continue 'dispatch;
	}
	pc = 0x824346A0; continue 'dispatch;
            }
            0x824346A0 => {
    //   block [0x824346A0..0x824346B0)
	// 824346A0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824346A4: 38800043  li r4, 0x43
	ctx.r[4].s64 = 67;
	// 824346A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824346AC: 4BFFE935  bl 0x82432fe0
	ctx.lr = 0x824346B0;
	sub_82432FE0(ctx, base);
	pc = 0x824346B0; continue 'dispatch;
            }
            0x824346B0 => {
    //   block [0x824346B0..0x824346C4)
	// 824346B0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824346B4: 409A0010  bne cr6, 0x824346c4
	if !ctx.cr[6].eq {
	pc = 0x824346C4; continue 'dispatch;
	}
	// 824346B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824346BC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824346C0: 4BFFE901  bl 0x82432fc0
	ctx.lr = 0x824346C4;
	sub_82432FC0(ctx, base);
	pc = 0x824346C4; continue 'dispatch;
            }
            0x824346C4 => {
    //   block [0x824346C4..0x824346E4)
	// 824346C4: 2F1B0001  cmpwi cr6, r27, 1
	ctx.cr[6].compare_i32(ctx.r[27].s32, 1, &mut ctx.xer);
	// 824346C8: 409A001C  bne cr6, 0x824346e4
	if !ctx.cr[6].eq {
	pc = 0x824346E4; continue 'dispatch;
	}
	// 824346CC: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 824346D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824346D4: 4BFFEAD5  bl 0x824331a8
	ctx.lr = 0x824346D8;
	sub_824331A8(ctx, base);
	// 824346D8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824346DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824346E0: 4BFFE8E1  bl 0x82432fc0
	ctx.lr = 0x824346E4;
	sub_82432FC0(ctx, base);
	pc = 0x824346E4; continue 'dispatch;
            }
            0x824346E4 => {
    //   block [0x824346E4..0x824346EC)
	// 824346E4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824346E8: 48100A1C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824346F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824346F0 size=120
    let mut pc: u32 = 0x824346F0;
    'dispatch: loop {
        match pc {
            0x824346F0 => {
    //   block [0x824346F0..0x82434760)
	// 824346F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824346F4: 481009C9  bl 0x825350bc
	ctx.lr = 0x824346F8;
	sub_82535080(ctx, base);
	// 824346F8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824346FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82434700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82434704: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82434708: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 8243470C: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 82434710: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82434714: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434718: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243471C: 48010EC5  bl 0x824455e0
	ctx.lr = 0x82434720;
	sub_824455E0(ctx, base);
	// 82434720: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434724: 409A003C  bne cr6, 0x82434760
	if !ctx.cr[6].eq {
	pc = 0x82434760; continue 'dispatch;
	}
	// 82434728: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243472C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434730: 409A0030  bne cr6, 0x82434760
	if !ctx.cr[6].eq {
	pc = 0x82434760; continue 'dispatch;
	}
	// 82434734: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82434738: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 8243473C: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 82434740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434744: 4801120D  bl 0x82445950
	ctx.lr = 0x82434748;
	sub_82445950(ctx, base);
	// 82434748: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243474C: 409A0014  bne cr6, 0x82434760
	if !ctx.cr[6].eq {
	pc = 0x82434760; continue 'dispatch;
	}
	// 82434750: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82434754: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82434758: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243475C: 915D0000  stw r10, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	pc = 0x82434760; continue 'dispatch;
            }
            0x82434760 => {
    //   block [0x82434760..0x82434768)
	// 82434760: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82434764: 481009A8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434768(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434768 size=136
    let mut pc: u32 = 0x82434768;
    'dispatch: loop {
        match pc {
            0x82434768 => {
    //   block [0x82434768..0x824347CC)
	// 82434768: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243476C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434770: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82434774: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82434778: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243477C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82434780: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 82434784: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82434788: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8243478C: 48010E55  bl 0x824455e0
	ctx.lr = 0x82434790;
	sub_824455E0(ctx, base);
	// 82434790: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434794: 409A0040  bne cr6, 0x824347d4
	if !ctx.cr[6].eq {
	pc = 0x824347D4; continue 'dispatch;
	}
	// 82434798: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243479C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824347A0: 409A0034  bne cr6, 0x824347d4
	if !ctx.cr[6].eq {
	pc = 0x824347D4; continue 'dispatch;
	}
	// 824347A4: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 824347A8: 388000E0  li r4, 0xe0
	ctx.r[4].s64 = 224;
	// 824347AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824347B0: 480111F1  bl 0x824459a0
	ctx.lr = 0x824347B4;
	sub_824459A0(ctx, base);
	// 824347B4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824347B8: 409A001C  bne cr6, 0x824347d4
	if !ctx.cr[6].eq {
	pc = 0x824347D4; continue 'dispatch;
	}
	// 824347BC: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824347C0: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 824347C4: 409A0008  bne cr6, 0x824347cc
	if !ctx.cr[6].eq {
	pc = 0x824347CC; continue 'dispatch;
	}
	// 824347C8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	pc = 0x824347CC; continue 'dispatch;
            }
            0x824347CC => {
    //   block [0x824347CC..0x824347D4)
	// 824347CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824347D0: 48000008  b 0x824347d8
	pc = 0x824347D8; continue 'dispatch;
            }
            0x824347D4 => {
    //   block [0x824347D4..0x824347D8)
	// 824347D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x824347D8; continue 'dispatch;
            }
            0x824347D8 => {
    //   block [0x824347D8..0x824347F0)
	// 824347D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824347DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824347E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824347E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824347E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824347EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824347F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824347F0 size=60
    let mut pc: u32 = 0x824347F0;
    'dispatch: loop {
        match pc {
            0x824347F0 => {
    //   block [0x824347F0..0x8243482C)
	// 824347F0: 816300C8  lwz r11, 0xc8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(200 as u32) ) } as u64;
	// 824347F4: 7D6A1E70  srawi r10, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 824347F8: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 824347FC: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82434800: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82434804: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82434808: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243480C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82434810: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82434814: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82434818: 814B00D0  lwz r10, 0xd0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(208 as u32) ) } as u64;
	// 8243481C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82434820: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82434824: 806B00E4  lwz r3, 0xe4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(228 as u32) ) } as u64;
	// 82434828: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434830 size=336
    let mut pc: u32 = 0x82434830;
    'dispatch: loop {
        match pc {
            0x82434830 => {
    //   block [0x82434830..0x824348B0)
	// 82434830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434834: 48100851  bl 0x82535084
	ctx.lr = 0x82434838;
	sub_82535080(ctx, base);
	// 82434838: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243483C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82434840: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82434844: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82434848: 835D0048  lwz r26, 0x48(r29)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243484C: 831E0020  lwz r24, 0x20(r30)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(32 as u32) ) } as u64;
	// 82434850: 807E001C  lwz r3, 0x1c(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(28 as u32) ) } as u64;
	// 82434854: 4BFFF71D  bl 0x82433f70
	ctx.lr = 0x82434858;
	sub_82433F68(ctx, base);
	// 82434858: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 8243485C: 82DE0000  lwz r22, 0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82434860: 82BE0004  lwz r21, 4(r30)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82434864: 829E0008  lwz r20, 8(r30)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82434868: 827E000C  lwz r19, 0xc(r30)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243486C: 807E0010  lwz r3, 0x10(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82434870: 4BFFF729  bl 0x82433f98
	ctx.lr = 0x82434874;
	sub_82433F68(ctx, base);
	// 82434874: 7C721B78  mr r18, r3
	ctx.r[18].u64 = ctx.r[3].u64;
	// 82434878: 837E0034  lwz r27, 0x34(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(52 as u32) ) } as u64;
	// 8243487C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82434880: 839E0018  lwz r28, 0x18(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 82434884: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82434888: 835E0030  lwz r26, 0x30(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 8243488C: 823E002C  lwz r17, 0x2c(r30)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(44 as u32) ) } as u64;
	// 82434890: 821E0024  lwz r16, 0x24(r30)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(36 as u32) ) } as u64;
	// 82434894: 81FE0028  lwz r15, 0x28(r30)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(40 as u32) ) } as u64;
	// 82434898: 4800A669  bl 0x8243ef00
	ctx.lr = 0x8243489C;
	sub_8243EF00(ctx, base);
	// 8243489C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824348A0: 419A0010  beq cr6, 0x824348b0
	if ctx.cr[6].eq {
	pc = 0x824348B0; continue 'dispatch;
	}
	// 824348A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824348A8: 386B5080  addi r3, r11, 0x5080
	ctx.r[3].s64 = ctx.r[11].s64 + 20608;
	// 824348AC: 4800281D  bl 0x824370c8
	ctx.lr = 0x824348B0;
	sub_824370C8(ctx, base);
	pc = 0x824348B0; continue 'dispatch;
            }
            0x824348B0 => {
    //   block [0x824348B0..0x82434964)
	// 824348B0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824348B4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824348B8: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824348BC: 4BFFF6AD  bl 0x82433f68
	ctx.lr = 0x824348C0;
	sub_82433F68(ctx, base);
	// 824348C0: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 824348C4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824348C8: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824348CC: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824348D0: 4BFFF699  bl 0x82433f68
	ctx.lr = 0x824348D4;
	sub_82433F68(ctx, base);
	// 824348D4: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824348D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824348DC: 931F0000  stw r24, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[24].u32 ) };
	// 824348E0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824348E4: 92FF0008  stw r23, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 824348E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824348EC: 92DF000C  stw r22, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[22].u32 ) };
	// 824348F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824348F4: 92BF0010  stw r21, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[21].u32 ) };
	// 824348F8: 915F0020  stw r10, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[10].u32 ) };
	// 824348FC: 929F0014  stw r20, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[20].u32 ) };
	// 82434900: 927F0018  stw r19, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[19].u32 ) };
	// 82434904: 925F001C  stw r18, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[18].u32 ) };
	// 82434908: 933F0024  stw r25, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[25].u32 ) };
	// 8243490C: 937F0028  stw r27, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[27].u32 ) };
	// 82434910: 939F002C  stw r28, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[28].u32 ) };
	// 82434914: 923F0030  stw r17, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[17].u32 ) };
	// 82434918: 935F0038  stw r26, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[26].u32 ) };
	// 8243491C: 921F003C  stw r16, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[16].u32 ) };
	// 82434920: 91FF0040  stw r15, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[15].u32 ) };
	// 82434924: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 82434928: 4BFFF5C9  bl 0x82433ef0
	ctx.lr = 0x8243492C;
	sub_82433EF0(ctx, base);
	// 8243492C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82434930: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82434934: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82434938: 4BFFF7D9  bl 0x82434110
	ctx.lr = 0x8243493C;
	sub_82434110(ctx, base);
	// 8243493C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82434940: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82434944: 80DF009C  lwz r6, 0x9c(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 82434948: 80BF0098  lwz r5, 0x98(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(152 as u32) ) } as u64;
	// 8243494C: 4BFFF6ED  bl 0x82434038
	ctx.lr = 0x82434950;
	sub_82434038(ctx, base);
	// 82434950: 397F0060  addi r11, r31, 0x60
	ctx.r[11].s64 = ctx.r[31].s64 + 96;
	// 82434954: 393E0048  addi r9, r30, 0x48
	ctx.r[9].s64 = ctx.r[30].s64 + 72;
	// 82434958: 907F004C  stw r3, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[3].u32 ) };
	// 8243495C: 39400007  li r10, 7
	ctx.r[10].s64 = 7;
	// 82434960: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x82434964; continue 'dispatch;
            }
            0x82434964 => {
    //   block [0x82434964..0x82434980)
	// 82434964: E9490000  ld r10, 0(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82434968: 39290008  addi r9, r9, 8
	ctx.r[9].s64 = ctx.r[9].s64 + 8;
	// 8243496C: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 82434970: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82434974: 4200FFF0  bdnz 0x82434964
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82434964; continue 'dispatch;
	}
	// 82434978: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 8243497C: 48100758  b 0x825350d4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434980 size=60
    let mut pc: u32 = 0x82434980;
    'dispatch: loop {
        match pc {
            0x82434980 => {
    //   block [0x82434980..0x824349BC)
	// 82434980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434984: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434988: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243498C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434990: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82434994: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82434998: 4800FEC9  bl 0x82444860
	ctx.lr = 0x8243499C;
	sub_82444860(ctx, base);
	// 8243499C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824349A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 824349A4: 4BFFF7FD  bl 0x824341a0
	ctx.lr = 0x824349A8;
	sub_824341A0(ctx, base);
	// 824349A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824349AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824349B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824349B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824349B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824349C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824349C0 size=4
    let mut pc: u32 = 0x824349C0;
    'dispatch: loop {
        match pc {
            0x824349C0 => {
    //   block [0x824349C0..0x824349C4)
	// 824349C0: 4BFFFE30  b 0x824347f0
	sub_824347F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824349C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824349C8 size=260
    let mut pc: u32 = 0x824349C8;
    'dispatch: loop {
        match pc {
            0x824349C8 => {
    //   block [0x824349C8..0x824349F4)
	// 824349C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824349CC: 481006DD  bl 0x825350a8
	ctx.lr = 0x824349D0;
	sub_82535080(ctx, base);
	// 824349D0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824349D4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824349D8: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 824349DC: 817A01F0  lwz r11, 0x1f0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(496 as u32) ) } as u64;
	// 824349E0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824349E4: 419A00E0  beq cr6, 0x82434ac4
	if ctx.cr[6].eq {
	pc = 0x82434AC4; continue 'dispatch;
	}
	// 824349E8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 824349EC: 3B000001  li r24, 1
	ctx.r[24].s64 = 1;
	// 824349F0: 7F9DE378  mr r29, r28
	ctx.r[29].u64 = ctx.r[28].u64;
	pc = 0x824349F4; continue 'dispatch;
            }
            0x824349F4 => {
    //   block [0x824349F4..0x82434A68)
	// 824349F4: 395D0020  addi r10, r29, 0x20
	ctx.r[10].s64 = ctx.r[29].s64 + 32;
	// 824349F8: 57AB2036  slwi r11, r29, 4
	ctx.r[11].u32 = ctx.r[29].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824349FC: 393D00C0  addi r9, r29, 0xc0
	ctx.r[9].s64 = ctx.r[29].s64 + 192;
	// 82434A00: 7FEBD214  add r31, r11, r26
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[26].u64;
	// 82434A04: 55592036  slwi r25, r10, 4
	ctx.r[25].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[25].u64 = ctx.r[25].u32 as u64;
	// 82434A08: 553E063E  clrlwi r30, r9, 0x18
	ctx.r[30].u64 = ctx.r[9].u32 as u64 & 0x000000FFu64;
	// 82434A0C: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82434A10: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82434A14: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82434A18: 939F01F4  stw r28, 0x1f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[28].u32 ) };
	// 82434A1C: 939F01F8  stw r28, 0x1f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(504 as u32), ctx.r[28].u32 ) };
	// 82434A20: 939F01FC  stw r28, 0x1fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[28].u32 ) };
	// 82434A24: 7F99D12E  stwx r28, r25, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32), ctx.r[28].u32) };
	// 82434A28: 48010BB9  bl 0x824455e0
	ctx.lr = 0x82434A2C;
	sub_824455E0(ctx, base);
	// 82434A2C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434A30: 409A0078  bne cr6, 0x82434aa8
	if !ctx.cr[6].eq {
	pc = 0x82434AA8; continue 'dispatch;
	}
	// 82434A34: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82434A38: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434A3C: 409A006C  bne cr6, 0x82434aa8
	if !ctx.cr[6].eq {
	pc = 0x82434AA8; continue 'dispatch;
	}
	// 82434A40: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82434A44: 931F01F4  stw r24, 0x1f4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(500 as u32), ctx.r[24].u32 ) };
	// 82434A48: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82434A4C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82434A50: 48010E11  bl 0x82445860
	ctx.lr = 0x82434A54;
	sub_82445860(ctx, base);
	// 82434A54: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434A58: 409A0010  bne cr6, 0x82434a68
	if !ctx.cr[6].eq {
	pc = 0x82434A68; continue 'dispatch;
	}
	// 82434A5C: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82434A60: 4BFFFA21  bl 0x82434480
	ctx.lr = 0x82434A64;
	sub_82434480(ctx, base);
	// 82434A64: 907F01F8  stw r3, 0x1f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(504 as u32), ctx.r[3].u32 ) };
	pc = 0x82434A68; continue 'dispatch;
            }
            0x82434A68 => {
    //   block [0x82434A68..0x82434A88)
	// 82434A68: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82434A6C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82434A70: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82434A74: 48010E3D  bl 0x824458b0
	ctx.lr = 0x82434A78;
	sub_824458B0(ctx, base);
	// 82434A78: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434A7C: 409A000C  bne cr6, 0x82434a88
	if !ctx.cr[6].eq {
	pc = 0x82434A88; continue 'dispatch;
	}
	// 82434A80: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82434A84: 917F01FC  stw r11, 0x1fc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(508 as u32), ctx.r[11].u32 ) };
	pc = 0x82434A88; continue 'dispatch;
            }
            0x82434A88 => {
    //   block [0x82434A88..0x82434AA8)
	// 82434A88: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 82434A8C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82434A90: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82434A94: 48010E45  bl 0x824458d8
	ctx.lr = 0x82434A98;
	sub_824458D8(ctx, base);
	// 82434A98: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434A9C: 409A000C  bne cr6, 0x82434aa8
	if !ctx.cr[6].eq {
	pc = 0x82434AA8; continue 'dispatch;
	}
	// 82434AA0: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82434AA4: 7D79D12E  stwx r11, r25, r26
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32), ctx.r[11].u32) };
	pc = 0x82434AA8; continue 'dispatch;
            }
            0x82434AA8 => {
    //   block [0x82434AA8..0x82434AC4)
	// 82434AA8: 397D0001  addi r11, r29, 1
	ctx.r[11].s64 = ctx.r[29].s64 + 1;
	// 82434AAC: 557D063E  clrlwi r29, r11, 0x18
	ctx.r[29].u64 = ctx.r[11].u32 as u64 & 0x000000FFu64;
	// 82434AB0: 2B1D0020  cmplwi cr6, r29, 0x20
	ctx.cr[6].compare_u32(ctx.r[29].u32, 32 as u32, &mut ctx.xer);
	// 82434AB4: 4198FF40  blt cr6, 0x824349f4
	if ctx.cr[6].lt {
	pc = 0x824349F4; continue 'dispatch;
	}
	// 82434AB8: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82434ABC: 931A01F0  stw r24, 0x1f0(r26)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[26].u32.wrapping_add(496 as u32), ctx.r[24].u32 ) };
	// 82434AC0: 4BFFFA39  bl 0x824344f8
	ctx.lr = 0x82434AC4;
	sub_824344F8(ctx, base);
	pc = 0x82434AC4; continue 'dispatch;
            }
            0x82434AC4 => {
    //   block [0x82434AC4..0x82434ACC)
	// 82434AC4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82434AC8: 48100630  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434AD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434AD0 size=316
    let mut pc: u32 = 0x82434AD0;
    'dispatch: loop {
        match pc {
            0x82434AD0 => {
    //   block [0x82434AD0..0x82434AFC)
	// 82434AD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434AD4: 481005E1  bl 0x825350b4
	ctx.lr = 0x82434AD8;
	sub_82535080(ctx, base);
	// 82434AD8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434ADC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82434AE0: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82434AE4: 4BFFE57D  bl 0x82433060
	ctx.lr = 0x82434AE8;
	sub_82433060(ctx, base);
	// 82434AE8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434AEC: 419A0030  beq cr6, 0x82434b1c
	if ctx.cr[6].eq {
	pc = 0x82434B1C; continue 'dispatch;
	}
	// 82434AF0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82434AF4: 386B50A0  addi r3, r11, 0x50a0
	ctx.r[3].s64 = ctx.r[11].s64 + 20640;
	// 82434AF8: 480025D1  bl 0x824370c8
	ctx.lr = 0x82434AFC;
	sub_824370C8(ctx, base);
	pc = 0x82434AFC; continue 'dispatch;
            }
            0x82434AFC => {
    //   block [0x82434AFC..0x82434B0C)
	// 82434AFC: 38A000A0  li r5, 0xa0
	ctx.r[5].s64 = 160;
	// 82434B00: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82434B04: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82434B08: 481006C9  bl 0x825351d0
	ctx.lr = 0x82434B0C;
	sub_825351D0(ctx, base);
	pc = 0x82434B0C; continue 'dispatch;
            }
            0x82434B0C => {
    //   block [0x82434B0C..0x82434B1C)
	// 82434B0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82434B10: 917B0000  stw r11, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434B14: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82434B18: 481005EC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82434B1C => {
    //   block [0x82434B1C..0x82434B6C)
	// 82434B1C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82434B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434B24: 4BFFF355  bl 0x82433e78
	ctx.lr = 0x82434B28;
	sub_82433E78(ctx, base);
	// 82434B28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434B2C: 4BFFE905  bl 0x82433430
	ctx.lr = 0x82434B30;
	sub_82433430(ctx, base);
	// 82434B30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82434B34: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82434B38: 419AFFC4  beq cr6, 0x82434afc
	if ctx.cr[6].eq {
	pc = 0x82434AFC; continue 'dispatch;
	}
	// 82434B3C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82434B40: 48005B09  bl 0x8243a648
	ctx.lr = 0x82434B44;
	sub_8243A648(ctx, base);
	// 82434B44: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82434B48: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82434B4C: 419AFFC0  beq cr6, 0x82434b0c
	if ctx.cr[6].eq {
	pc = 0x82434B0C; continue 'dispatch;
	}
	// 82434B50: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82434B54: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82434B58: 409A0058  bne cr6, 0x82434bb0
	if !ctx.cr[6].eq {
	pc = 0x82434BB0; continue 'dispatch;
	}
	// 82434B5C: 839F0018  lwz r28, 0x18(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82434B60: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82434B64: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82434B68: 40990048  ble cr6, 0x82434bb0
	if !ctx.cr[6].gt {
	pc = 0x82434BB0; continue 'dispatch;
	}
	pc = 0x82434B6C; continue 'dispatch;
            }
            0x82434B6C => {
    //   block [0x82434B6C..0x82434BAC)
	// 82434B6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434B70: 4BFFF6E9  bl 0x82434258
	ctx.lr = 0x82434B74;
	sub_82434258(ctx, base);
	// 82434B74: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434B78: 409A0034  bne cr6, 0x82434bac
	if !ctx.cr[6].eq {
	pc = 0x82434BAC; continue 'dispatch;
	}
	// 82434B7C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82434B80: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82434B84: 48005B9D  bl 0x8243a720
	ctx.lr = 0x82434B88;
	sub_8243A720(ctx, base);
	// 82434B88: 817F0094  lwz r11, 0x94(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(148 as u32) ) } as u64;
	// 82434B8C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82434B90: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82434B94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82434B98: 917F0094  stw r11, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[11].u32 ) };
	// 82434B9C: 48005AAD  bl 0x8243a648
	ctx.lr = 0x82434BA0;
	sub_8243A648(ctx, base);
	// 82434BA0: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82434BA4: 7F1EE000  cmpw cr6, r30, r28
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[28].s32, &mut ctx.xer);
	// 82434BA8: 4198FFC4  blt cr6, 0x82434b6c
	if ctx.cr[6].lt {
	pc = 0x82434B6C; continue 'dispatch;
	}
	pc = 0x82434BAC; continue 'dispatch;
            }
            0x82434BAC => {
    //   block [0x82434BAC..0x82434BB0)
	// 82434BAC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	pc = 0x82434BB0; continue 'dispatch;
            }
            0x82434BB0 => {
    //   block [0x82434BB0..0x82434C0C)
	// 82434BB0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82434BB4: 419AFF58  beq cr6, 0x82434b0c
	if ctx.cr[6].eq {
	pc = 0x82434B0C; continue 'dispatch;
	}
	// 82434BB8: 817F008C  lwz r11, 0x8c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 82434BBC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434BC0: 909F0088  stw r4, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[4].u32 ) };
	// 82434BC4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82434BC8: 917F008C  stw r11, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[11].u32 ) };
	// 82434BCC: 4BFFF4F5  bl 0x824340c0
	ctx.lr = 0x82434BD0;
	sub_824340C0(ctx, base);
	// 82434BD0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82434BD4: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82434BD8: 4BFFFC59  bl 0x82434830
	ctx.lr = 0x82434BDC;
	sub_82434830(ctx, base);
	// 82434BDC: 817B0024  lwz r11, 0x24(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82434BE0: 815B0030  lwz r10, 0x30(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(48 as u32) ) } as u64;
	// 82434BE4: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82434BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434BEC: 917B0004  stw r11, 4(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82434BF0: 915F00C8  stw r10, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[10].u32 ) };
	// 82434BF4: 4BFFF2AD  bl 0x82433ea0
	ctx.lr = 0x82434BF8;
	sub_82433EA0(ctx, base);
	// 82434BF8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82434BFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434C00: 4BFFF209  bl 0x82433e08
	ctx.lr = 0x82434C04;
	sub_82433E08(ctx, base);
	// 82434C04: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82434C08: 481004FC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82434C10 size=320
    let mut pc: u32 = 0x82434C10;
    'dispatch: loop {
        match pc {
            0x82434C10 => {
    //   block [0x82434C10..0x82434D40)
	// 82434C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434C14: 48100499  bl 0x825350ac
	ctx.lr = 0x82434C18;
	sub_82535080(ctx, base);
	// 82434C18: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434C1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82434C20: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82434C24: 2F050800  cmpwi cr6, r5, 0x800
	ctx.cr[6].compare_i32(ctx.r[5].s32, 2048, &mut ctx.xer);
	// 82434C28: 817E00C4  lwz r11, 0xc4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82434C2C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82434C30: 917E00C4  stw r11, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[11].u32 ) };
	// 82434C34: 41980114  blt cr6, 0x82434d48
	if ctx.cr[6].lt {
	pc = 0x82434D48; continue 'dispatch;
	}
	// 82434C38: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82434C3C: 419A010C  beq cr6, 0x82434d48
	if ctx.cr[6].eq {
	pc = 0x82434D48; continue 'dispatch;
	}
	// 82434C40: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82434C44: 48010745  bl 0x82445388
	ctx.lr = 0x82434C48;
	sub_82445388(ctx, base);
	// 82434C48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82434C4C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82434C50: 419A00F8  beq cr6, 0x82434d48
	if ctx.cr[6].eq {
	pc = 0x82434D48; continue 'dispatch;
	}
	// 82434C54: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82434C58: 48010801  bl 0x82445458
	ctx.lr = 0x82434C5C;
	sub_82445458(ctx, base);
	// 82434C5C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434C60: 409A00E0  bne cr6, 0x82434d40
	if !ctx.cr[6].eq {
	pc = 0x82434D40; continue 'dispatch;
	}
	// 82434C64: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82434C68: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82434C6C: 409A00D4  bne cr6, 0x82434d40
	if !ctx.cr[6].eq {
	pc = 0x82434D40; continue 'dispatch;
	}
	// 82434C70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434C74: 4BFFFAF5  bl 0x82434768
	ctx.lr = 0x82434C78;
	sub_82434768(ctx, base);
	// 82434C78: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82434C7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434C80: 4BFFF649  bl 0x824342c8
	ctx.lr = 0x82434C84;
	sub_824342C8(ctx, base);
	// 82434C84: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82434C88: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434C8C: 4BFFF6F5  bl 0x82434380
	ctx.lr = 0x82434C90;
	sub_82434380(ctx, base);
	// 82434C90: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82434C94: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434C98: 4BFFF6A1  bl 0x82434338
	ctx.lr = 0x82434C9C;
	sub_82434338(ctx, base);
	// 82434C9C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82434CA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434CA4: 4BFFF65D  bl 0x82434300
	ctx.lr = 0x82434CA8;
	sub_82434300(ctx, base);
	// 82434CA8: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82434CAC: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82434CB0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82434CB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434CB8: 4BFFFA39  bl 0x824346f0
	ctx.lr = 0x82434CBC;
	sub_824346F0(ctx, base);
	// 82434CBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82434CC0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82434CC4: 4BFFFD05  bl 0x824349c8
	ctx.lr = 0x82434CC8;
	sub_824349C8(ctx, base);
	// 82434CC8: 817E00CC  lwz r11, 0xcc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 82434CCC: 815E00C4  lwz r10, 0xc4(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(196 as u32) ) } as u64;
	// 82434CD0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82434CD4: 55691838  slwi r9, r11, 3
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82434CD8: 80E10054  lwz r7, 0x54(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82434CDC: 390AFFFF  addi r8, r10, -1
	ctx.r[8].s64 = ctx.r[10].s64 + -1;
	// 82434CE0: 80C10058  lwz r6, 0x58(r1)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82434CE4: 394B0006  addi r10, r11, 6
	ctx.r[10].s64 = ctx.r[11].s64 + 6;
	// 82434CE8: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82434CEC: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82434CF0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82434CF4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82434CF8: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82434CFC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82434D00: 910B00D4  stw r8, 0xd4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(212 as u32), ctx.r[8].u32 ) };
	// 82434D04: 7CEAF12E  stwx r7, r10, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[7].u32) };
	// 82434D08: 93AB00E0  stw r29, 0xe0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(224 as u32), ctx.r[29].u32 ) };
	// 82434D0C: 938B00E4  stw r28, 0xe4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(228 as u32), ctx.r[28].u32 ) };
	// 82434D10: 936B00E8  stw r27, 0xe8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(232 as u32), ctx.r[27].u32 ) };
	// 82434D14: 934B00EC  stw r26, 0xec(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(236 as u32), ctx.r[26].u32 ) };
	// 82434D18: 932B00F0  stw r25, 0xf0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(240 as u32), ctx.r[25].u32 ) };
	// 82434D1C: 90CB00DC  stw r6, 0xdc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(220 as u32), ctx.r[6].u32 ) };
	// 82434D20: 90AB00D0  stw r5, 0xd0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(208 as u32), ctx.r[5].u32 ) };
	// 82434D24: 817E00CC  lwz r11, 0xcc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(204 as u32) ) } as u64;
	// 82434D28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82434D2C: 7D6A1E70  srawi r10, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82434D30: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82434D34: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82434D38: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82434D3C: 917E00CC  stw r11, 0xcc(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	pc = 0x82434D40; continue 'dispatch;
            }
            0x82434D40 => {
    //   block [0x82434D40..0x82434D48)
	// 82434D40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434D44: 480106DD  bl 0x82445420
	ctx.lr = 0x82434D48;
	sub_82445420(ctx, base);
	pc = 0x82434D48; continue 'dispatch;
            }
            0x82434D48 => {
    //   block [0x82434D48..0x82434D50)
	// 82434D48: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82434D4C: 481003B0  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82434D50 size=24
    let mut pc: u32 = 0x82434D50;
    'dispatch: loop {
        match pc {
            0x82434D50 => {
    //   block [0x82434D50..0x82434D68)
	// 82434D50: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 82434D54: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 82434D58: 388B4C10  addi r4, r11, 0x4c10
	ctx.r[4].s64 = ctx.r[11].s64 + 19472;
	// 82434D5C: 80650048  lwz r3, 0x48(r5)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(72 as u32) ) } as u64;
	// 82434D60: 480079B8  b 0x8243c718
	crate::recompiler::externs::call(ctx, base, 0x8243C718);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82434D68 size=280
    let mut pc: u32 = 0x82434D68;
    'dispatch: loop {
        match pc {
            0x82434D68 => {
    //   block [0x82434D68..0x82434D88)
	// 82434D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434D6C: 4810034D  bl 0x825350b8
	ctx.lr = 0x82434D70;
	sub_82535080(ctx, base);
	// 82434D70: 81430014  lwz r10, 0x14(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82434D74: 83E30000  lwz r31, 0(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82434D78: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82434D7C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82434D80: 41990008  bgt cr6, 0x82434d88
	if ctx.cr[6].gt {
	pc = 0x82434D88; continue 'dispatch;
	}
	// 82434D84: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	pc = 0x82434D88; continue 'dispatch;
            }
            0x82434D88 => {
    //   block [0x82434D88..0x82434E00)
	// 82434D88: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 82434D8C: 419A00BC  beq cr6, 0x82434e48
	if ctx.cr[6].eq {
	pc = 0x82434E48; continue 'dispatch;
	}
	// 82434D90: 2F1F0006  cmpwi cr6, r31, 6
	ctx.cr[6].compare_i32(ctx.r[31].s32, 6, &mut ctx.xer);
	// 82434D94: 419A00B4  beq cr6, 0x82434e48
	if ctx.cr[6].eq {
	pc = 0x82434E48; continue 'dispatch;
	}
	// 82434D98: 2F1F0003  cmpwi cr6, r31, 3
	ctx.cr[6].compare_i32(ctx.r[31].s32, 3, &mut ctx.xer);
	// 82434D9C: 419A0064  beq cr6, 0x82434e00
	if ctx.cr[6].eq {
	pc = 0x82434E00; continue 'dispatch;
	}
	// 82434DA0: 2F1F0007  cmpwi cr6, r31, 7
	ctx.cr[6].compare_i32(ctx.r[31].s32, 7, &mut ctx.xer);
	// 82434DA4: 419A005C  beq cr6, 0x82434e00
	if ctx.cr[6].eq {
	pc = 0x82434E00; continue 'dispatch;
	}
	// 82434DA8: 7D6B1E70  srawi r11, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82434DAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82434DB0: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82434DB4: 3FA00000  lis r29, 0
	ctx.r[29].s64 = 0;
	// 82434DB8: 7D6B5E70  srawi r11, r11, 0xb
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 11) as i64;
	// 82434DBC: 3BE05DCC  li r31, 0x5dcc
	ctx.r[31].s64 = 24012;
	// 82434DC0: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82434DC4: 3BC05F0C  li r30, 0x5f0c
	ctx.r[30].s64 = 24332;
	// 82434DC8: 557C5828  slwi r28, r11, 0xb
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82434DCC: 7D6B51D6  mullw r11, r11, r10
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[10].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82434DD0: 7F8A0E70  srawi r10, r28, 1
	ctx.xer.ca = (ctx.r[28].s32 < 0) && ((ctx.r[28].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[28].s32 >> 1) as i64;
	// 82434DD4: 557C5828  slwi r28, r11, 0xb
	ctx.r[28].u32 = ctx.r[11].u32.wrapping_shl(11);
	ctx.r[28].u64 = ctx.r[28].u32 as u64;
	// 82434DD8: 7D6A0194  addze r11, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82434DDC: 63BD81C0  ori r29, r29, 0x81c0
	ctx.r[29].u64 = ctx.r[29].u64 | 33216;
	// 82434DE0: 396B0800  addi r11, r11, 0x800
	ctx.r[11].s64 = ctx.r[11].s64 + 2048;
	// 82434DE4: 93840000  stw r28, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82434DE8: 90650000  stw r3, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82434DEC: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434DF0: 93E70000  stw r31, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82434DF4: 93C80000  stw r30, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82434DF8: 93A90000  stw r29, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 82434DFC: 4810030C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82434E00 => {
    //   block [0x82434E00..0x82434E48)
	// 82434E00: 7D631E70  srawi r3, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82434E04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82434E08: 7C630194  addze r3, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[3].s64 = tmp.s64;
	// 82434E0C: 7C635E70  srawi r3, r3, 0xb
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[3].s32 >> 11) as i64;
	// 82434E10: 7C630194  addze r3, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[3].s64 = tmp.s64;
	// 82434E14: 547F5828  slwi r31, r3, 0xb
	ctx.r[31].u32 = ctx.r[3].u32.wrapping_shl(11);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82434E18: 7D4351D6  mullw r10, r3, r10
	ctx.r[10].s32 = ((ctx.r[3].s32 as i64 * ctx.r[10].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82434E1C: 7FE30E70  srawi r3, r31, 1
	ctx.xer.ca = (ctx.r[31].s32 < 0) && ((ctx.r[31].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[31].s32 >> 1) as i64;
	// 82434E20: 555F5828  slwi r31, r10, 0xb
	ctx.r[31].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82434E24: 7D430194  addze r10, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82434E28: 394A0800  addi r10, r10, 0x800
	ctx.r[10].s64 = ctx.r[10].s64 + 2048;
	// 82434E2C: 93E40000  stw r31, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 82434E30: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E34: 91460000  stw r10, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82434E38: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E3C: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E40: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E44: 481002C4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82434E48 => {
    //   block [0x82434E48..0x82434E80)
	// 82434E48: 7D631E70  srawi r3, r11, 3
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 3) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[11].s32 >> 3) as i64;
	// 82434E4C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82434E50: 7C630194  addze r3, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[3].s64 = tmp.s64;
	// 82434E54: 7C635E70  srawi r3, r3, 0xb
	ctx.xer.ca = (ctx.r[3].s32 < 0) && ((ctx.r[3].u32 & ((1u32 << 11) - 1)) != 0);
	ctx.r[3].s64 = (ctx.r[3].s32 >> 11) as i64;
	// 82434E58: 7C630194  addze r3, r3
	tmp.s64 = ctx.r[3].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[3].u32);
	ctx.r[3].s64 = tmp.s64;
	// 82434E5C: 7D4351D6  mullw r10, r3, r10
	ctx.r[10].s32 = ((ctx.r[3].s32 as i64 * ctx.r[10].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82434E60: 554A5828  slwi r10, r10, 0xb
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(11);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82434E64: 91440000  stw r10, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82434E68: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E6C: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E70: 91670000  stw r11, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E74: 91680000  stw r11, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E78: 91690000  stw r11, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434E7C: 4810028C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82434E80 size=60
    let mut pc: u32 = 0x82434E80;
    'dispatch: loop {
        match pc {
            0x82434E80 => {
    //   block [0x82434E80..0x82434EBC)
	// 82434E80: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82434E84: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82434E88: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82434E8C: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82434E90: 3929001F  addi r9, r9, 0x1f
	ctx.r[9].s64 = ctx.r[9].s64 + 31;
	// 82434E94: 7D292E70  srawi r9, r9, 5
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 5) as i64;
	// 82434E98: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82434E9C: 7D480E70  srawi r8, r10, 1
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[10].s32 >> 1) as i64;
	// 82434EA0: 55292834  slwi r9, r9, 5
	ctx.r[9].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82434EA4: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82434EA8: 91260000  stw r9, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82434EAC: 91070000  stw r8, 0(r7)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82434EB0: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434EB4: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82434EB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434EC0 size=132
    let mut pc: u32 = 0x82434EC0;
    'dispatch: loop {
        match pc {
            0x82434EC0 => {
    //   block [0x82434EC0..0x82434EFC)
	// 82434EC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434EC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434EC8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434ECC: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82434ED0: 41990054  bgt cr6, 0x82434f24
	if ctx.cr[6].gt {
	pc = 0x82434F24; continue 'dispatch;
	}
	// 82434ED4: 3D808243  lis r12, -0x7dbd
	ctx.r[12].s64 = -2109538304;
	// 82434ED8: 398C4EEC  addi r12, r12, 0x4eec
	ctx.r[12].s64 = ctx.r[12].s64 + 20204;
	// 82434EDC: 5460103A  slwi r0, r3, 2
	ctx.r[0].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82434EE0: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82434EE4: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82434EE8: 4E800420  bctr
	match ctx.r[3].u64 {
		0 => {
	pc = 0x82434F30; continue 'dispatch;
		},
		1 => {
	pc = 0x82434EFC; continue 'dispatch;
		},
		2 => {
	pc = 0x82434F10; continue 'dispatch;
		},
		3 => {
	pc = 0x82434F30; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82434EEC: 82434F30  lwz r18, 0x4f30(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20272 as u32) ) } as u64;
	// 82434EF0: 82434EFC  lwz r18, 0x4efc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20220 as u32) ) } as u64;
	// 82434EF4: 82434F10  lwz r18, 0x4f10(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20240 as u32) ) } as u64;
	// 82434EF8: 82434F30  lwz r18, 0x4f30(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20272 as u32) ) } as u64;
            }
            0x82434EFC => {
    //   block [0x82434EFC..0x82434F10)
	// 82434EFC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82434F00: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434F04: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434F08: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434F0C: 4E800020  blr
	return;
            }
            0x82434F10 => {
    //   block [0x82434F10..0x82434F24)
	// 82434F10: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82434F14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434F18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434F1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434F20: 4E800020  blr
	return;
            }
            0x82434F24 => {
    //   block [0x82434F24..0x82434F30)
	// 82434F24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82434F28: 386B5138  addi r3, r11, 0x5138
	ctx.r[3].s64 = ctx.r[11].s64 + 20792;
	// 82434F2C: 4800219D  bl 0x824370c8
	ctx.lr = 0x82434F30;
	sub_824370C8(ctx, base);
	pc = 0x82434F30; continue 'dispatch;
            }
            0x82434F30 => {
    //   block [0x82434F30..0x82434F44)
	// 82434F30: 38600003  li r3, 3
	ctx.r[3].s64 = 3;
	// 82434F34: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82434F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434F40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82434F48 size=28
    let mut pc: u32 = 0x82434F48;
    'dispatch: loop {
        match pc {
            0x82434F48 => {
    //   block [0x82434F48..0x82434F64)
	// 82434F48: 39604000  li r11, 0x4000
	ctx.r[11].s64 = 16384;
	// 82434F4C: 39400700  li r10, 0x700
	ctx.r[10].s64 = 1792;
	// 82434F50: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82434F54: 91450000  stw r10, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82434F58: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82434F5C: 386B0700  addi r3, r11, 0x700
	ctx.r[3].s64 = ctx.r[11].s64 + 1792;
	// 82434F60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82434F68 size=8
    let mut pc: u32 = 0x82434F68;
    'dispatch: loop {
        match pc {
            0x82434F68 => {
    //   block [0x82434F68..0x82434F70)
	// 82434F68: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 82434F6C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82434F70 size=88
    let mut pc: u32 = 0x82434F70;
    'dispatch: loop {
        match pc {
            0x82434F70 => {
    //   block [0x82434F70..0x82434FB0)
	// 82434F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82434F74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82434F78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82434F7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82434F80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82434F84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82434F88: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82434F8C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82434F90: 4BFFCC41  bl 0x82431bd0
	ctx.lr = 0x82434F94;
	sub_82431BD0(ctx, base);
	// 82434F94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82434F98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82434F9C: 4BFFD015  bl 0x82431fb0
	ctx.lr = 0x82434FA0;
	sub_82431FB0(ctx, base);
	// 82434FA0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82434FA4: 3C7E0002  addis r3, r30, 2
	ctx.r[3].s64 = ctx.r[30].s64 + 131072;
	// 82434FA8: 419A0008  beq cr6, 0x82434fb0
	if ctx.cr[6].eq {
	pc = 0x82434FB0; continue 'dispatch;
	}
	// 82434FAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	pc = 0x82434FB0; continue 'dispatch;
            }
            0x82434FB0 => {
    //   block [0x82434FB0..0x82434FC8)
	// 82434FB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82434FB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82434FB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82434FBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82434FC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82434FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82434FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82434FC8 size=100
    let mut pc: u32 = 0x82434FC8;
    'dispatch: loop {
        match pc {
            0x82434FC8 => {
    //   block [0x82434FC8..0x8243502C)
	// 82434FC8: 3963000F  addi r11, r3, 0xf
	ctx.r[11].s64 = ctx.r[3].s64 + 15;
	// 82434FCC: 3944000F  addi r10, r4, 0xf
	ctx.r[10].s64 = ctx.r[4].s64 + 15;
	// 82434FD0: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82434FD4: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82434FD8: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 82434FDC: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82434FE0: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82434FE4: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82434FE8: 55482036  slwi r8, r10, 4
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82434FEC: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82434FF0: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 82434FF4: 3929007F  addi r9, r9, 0x7f
	ctx.r[9].s64 = ctx.r[9].s64 + 127;
	// 82434FF8: 7D293E70  srawi r9, r9, 7
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 7) as i64;
	// 82434FFC: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82435000: 7D080E70  srawi r8, r8, 1
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 1) as i64;
	// 82435004: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82435008: 7D673E70  srawi r7, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 8243500C: 7D6941D6  mullw r11, r9, r8
	ctx.r[11].s32 = ((ctx.r[9].s32 as i64 * ctx.r[8].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82435010: 7D270194  addze r9, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[9].s64 = tmp.s64;
	// 82435014: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s32 = ((ctx.r[9].s32 as i64 * ctx.r[10].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82435018: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243501C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435020: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82435024: 386B0080  addi r3, r11, 0x80
	ctx.r[3].s64 = ctx.r[11].s64 + 128;
	// 82435028: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435030 size=224
    let mut pc: u32 = 0x82435030;
    'dispatch: loop {
        match pc {
            0x82435030 => {
    //   block [0x82435030..0x82435088)
	// 82435030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435038: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243503C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82435040: 814B0450  lwz r10, 0x450(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1104 as u32) ) } as u64;
	// 82435044: 806B0048  lwz r3, 0x48(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82435048: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8243504C: 419A00B4  beq cr6, 0x82435100
	if ctx.cr[6].eq {
	pc = 0x82435100; continue 'dispatch;
	}
	// 82435050: 812B0474  lwz r9, 0x474(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1140 as u32) ) } as u64;
	// 82435054: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82435058: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8243505C: 409A002C  bne cr6, 0x82435088
	if !ctx.cr[6].eq {
	pc = 0x82435088; continue 'dispatch;
	}
	// 82435060: 812B0478  lwz r9, 0x478(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1144 as u32) ) } as u64;
	// 82435064: 816B047C  lwz r11, 0x47c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1148 as u32) ) } as u64;
	// 82435068: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 8243506C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82435070: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82435074: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82435078: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 8243507C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82435080: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82435084: 48000058  b 0x824350dc
	pc = 0x824350DC; continue 'dispatch;
            }
            0x82435088 => {
    //   block [0x82435088..0x824350B4)
	// 82435088: 812B0454  lwz r9, 0x454(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1108 as u32) ) } as u64;
	// 8243508C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82435090: 409A0024  bne cr6, 0x824350b4
	if !ctx.cr[6].eq {
	pc = 0x824350B4; continue 'dispatch;
	}
	// 82435094: 812B0458  lwz r9, 0x458(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1112 as u32) ) } as u64;
	// 82435098: 91210058  stw r9, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[9].u32 ) };
	// 8243509C: 812B045C  lwz r9, 0x45c(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1116 as u32) ) } as u64;
	// 824350A0: 816B0460  lwz r11, 0x460(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1120 as u32) ) } as u64;
	// 824350A4: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 824350A8: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824350AC: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 824350B0: 48000024  b 0x824350d4
	pc = 0x824350D4; continue 'dispatch;
            }
            0x824350B4 => {
    //   block [0x824350B4..0x824350D4)
	// 824350B4: 814B0468  lwz r10, 0x468(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1128 as u32) ) } as u64;
	// 824350B8: 812B0464  lwz r9, 0x464(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1124 as u32) ) } as u64;
	// 824350BC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 824350C0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824350C4: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 824350C8: 814B046C  lwz r10, 0x46c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1132 as u32) ) } as u64;
	// 824350CC: 816B0470  lwz r11, 0x470(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1136 as u32) ) } as u64;
	// 824350D0: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	pc = 0x824350D4; continue 'dispatch;
            }
            0x824350D4 => {
    //   block [0x824350D4..0x824350DC)
	// 824350D4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 824350D8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	pc = 0x824350DC; continue 'dispatch;
            }
            0x824350DC => {
    //   block [0x824350DC..0x82435100)
	// 824350DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 824350E0: 48004F69  bl 0x8243a048
	ctx.lr = 0x824350E4;
	sub_8243A048(ctx, base);
	// 824350E4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824350E8: 419A0018  beq cr6, 0x82435100
	if ctx.cr[6].eq {
	pc = 0x82435100; continue 'dispatch;
	}
	// 824350EC: 3860FEC8  li r3, -0x138
	ctx.r[3].s64 = -312;
	// 824350F0: 480018D9  bl 0x824369c8
	ctx.lr = 0x824350F4;
	sub_824369C8(ctx, base);
	// 824350F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824350F8: 386B5160  addi r3, r11, 0x5160
	ctx.r[3].s64 = ctx.r[11].s64 + 20832;
	// 824350FC: 48001FCD  bl 0x824370c8
	ctx.lr = 0x82435100;
	sub_824370C8(ctx, base);
	pc = 0x82435100; continue 'dispatch;
            }
            0x82435100 => {
    //   block [0x82435100..0x82435110)
	// 82435100: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82435104: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435108: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243510C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435110 size=72
    let mut pc: u32 = 0x82435110;
    'dispatch: loop {
        match pc {
            0x82435110 => {
    //   block [0x82435110..0x8243514C)
	// 82435110: 3963FFFE  addi r11, r3, -2
	ctx.r[11].s64 = ctx.r[3].s64 + -2;
	// 82435114: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 82435118: 4199003C  bgt cr6, 0x82435154
	if ctx.cr[6].gt {
	pc = 0x82435154; continue 'dispatch;
	}
	// 8243511C: 3D808243  lis r12, -0x7dbd
	ctx.r[12].s64 = -2109538304;
	// 82435120: 398C5134  addi r12, r12, 0x5134
	ctx.r[12].s64 = ctx.r[12].s64 + 20788;
	// 82435124: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82435128: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8243512C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82435130: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x8243514C; continue 'dispatch;
		},
		1 => {
	pc = 0x8243514C; continue 'dispatch;
		},
		2 => {
	pc = 0x82435154; continue 'dispatch;
		},
		3 => {
	pc = 0x82435154; continue 'dispatch;
		},
		4 => {
	pc = 0x8243514C; continue 'dispatch;
		},
		5 => {
	pc = 0x8243514C; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82435134: 8243514C  lwz r18, 0x514c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20812 as u32) ) } as u64;
	// 82435138: 8243514C  lwz r18, 0x514c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20812 as u32) ) } as u64;
	// 8243513C: 82435154  lwz r18, 0x5154(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20820 as u32) ) } as u64;
	// 82435140: 82435154  lwz r18, 0x5154(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20820 as u32) ) } as u64;
	// 82435144: 8243514C  lwz r18, 0x514c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20812 as u32) ) } as u64;
	// 82435148: 8243514C  lwz r18, 0x514c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20812 as u32) ) } as u64;
            }
            0x8243514C => {
    //   block [0x8243514C..0x82435154)
	// 8243514C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82435150: 4E800020  blr
	return;
            }
            0x82435154 => {
    //   block [0x82435154..0x82435158)
	// 82435154: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435160 size=24
    let mut pc: u32 = 0x82435160;
    'dispatch: loop {
        match pc {
            0x82435160 => {
    //   block [0x82435160..0x82435178)
	// 82435160: 396303F4  addi r11, r3, 0x3f4
	ctx.r[11].s64 = ctx.r[3].s64 + 1012;
	// 82435164: 908B0000  stw r4, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82435168: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 8243516C: 90CB0008  stw r6, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82435170: 9163040C  stw r11, 0x40c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1036 as u32), ctx.r[11].u32 ) };
	// 82435174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435178 size=140
    let mut pc: u32 = 0x82435178;
    'dispatch: loop {
        match pc {
            0x82435178 => {
    //   block [0x82435178..0x824351A8)
	// 82435178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243517C: 480FFF3D  bl 0x825350b8
	ctx.lr = 0x82435180;
	sub_82535080(ctx, base);
	// 82435180: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435184: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82435188: 817D040C  lwz r11, 0x40c(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(1036 as u32) ) } as u64;
	// 8243518C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82435190: 409A0018  bne cr6, 0x824351a8
	if !ctx.cr[6].eq {
	pc = 0x824351A8; continue 'dispatch;
	}
	// 82435194: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82435198: 386B51BC  addi r3, r11, 0x51bc
	ctx.r[3].s64 = ctx.r[11].s64 + 20924;
	// 8243519C: 48001F2D  bl 0x824370c8
	ctx.lr = 0x824351A0;
	sub_824370C8(ctx, base);
	// 824351A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824351A4: 480FFF64  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x824351A8 => {
    //   block [0x824351A8..0x824351DC)
	// 824351A8: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 824351AC: 83EB0008  lwz r31, 8(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 824351B0: 3BCA0003  addi r30, r10, 3
	ctx.r[30].s64 = ctx.r[10].s64 + 3;
	// 824351B4: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 824351B8: 838B0000  lwz r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 824351BC: 7D7EF9D6  mullw r11, r30, r31
	ctx.r[11].s32 = ((ctx.r[30].s32 as i64 * ctx.r[31].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824351C0: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 824351C4: 40980018  bge cr6, 0x824351dc
	if !ctx.cr[6].lt {
	pc = 0x824351DC; continue 'dispatch;
	}
	// 824351C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824351CC: 386B5180  addi r3, r11, 0x5180
	ctx.r[3].s64 = ctx.r[11].s64 + 20864;
	// 824351D0: 48001EF9  bl 0x824370c8
	ctx.lr = 0x824351D4;
	sub_824370C8(ctx, base);
	// 824351D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824351D8: 480FFF30  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x824351DC => {
    //   block [0x824351DC..0x824351FC)
	// 824351DC: 480016D5  bl 0x824368b0
	ctx.lr = 0x824351E0;
	sub_824368B0(ctx, base);
	// 824351E0: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824351E4: 409A0018  bne cr6, 0x824351fc
	if !ctx.cr[6].eq {
	pc = 0x824351FC; continue 'dispatch;
	}
	// 824351E8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 824351EC: 807D0048  lwz r3, 0x48(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(72 as u32) ) } as u64;
	// 824351F0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824351F4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 824351F8: 4800FE89  bl 0x82445080
	ctx.lr = 0x824351FC;
	sub_82445080(ctx, base);
	pc = 0x824351FC; continue 'dispatch;
            }
            0x824351FC => {
    //   block [0x824351FC..0x82435204)
	// 824351FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82435200: 480FFF08  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435208 size=60
    let mut pc: u32 = 0x82435208;
    'dispatch: loop {
        match pc {
            0x82435208 => {
    //   block [0x82435208..0x82435234)
	// 82435208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243520C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435210: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435214: 48005F9D  bl 0x8243b1b0
	ctx.lr = 0x82435218;
	sub_8243B1B0(ctx, base);
	// 82435218: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243521C: 419A0018  beq cr6, 0x82435234
	if ctx.cr[6].eq {
	pc = 0x82435234; continue 'dispatch;
	}
	// 82435220: 3860FECE  li r3, -0x132
	ctx.r[3].s64 = -306;
	// 82435224: 480017A5  bl 0x824369c8
	ctx.lr = 0x82435228;
	sub_824369C8(ctx, base);
	// 82435228: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243522C: 386B51F4  addi r3, r11, 0x51f4
	ctx.r[3].s64 = ctx.r[11].s64 + 20980;
	// 82435230: 48001E99  bl 0x824370c8
	ctx.lr = 0x82435234;
	sub_824370C8(ctx, base);
	pc = 0x82435234; continue 'dispatch;
            }
            0x82435234 => {
    //   block [0x82435234..0x82435244)
	// 82435234: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82435238: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243523C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82435240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82435248 size=316
    let mut pc: u32 = 0x82435248;
    'dispatch: loop {
        match pc {
            0x82435248 => {
    //   block [0x82435248..0x824352FC)
	// 82435248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243524C: 480FFE71  bl 0x825350bc
	ctx.lr = 0x82435250;
	sub_82535080(ctx, base);
	// 82435250: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435254: 83E30048  lwz r31, 0x48(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82435258: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8243525C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82435260: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82435264: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435268: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8243526C: 4800768D  bl 0x8243c8f8
	ctx.lr = 0x82435270;
	sub_8243C8F8(ctx, base);
	// 82435270: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82435274: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82435278: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243527C: 4800767D  bl 0x8243c8f8
	ctx.lr = 0x82435280;
	sub_8243C8F8(ctx, base);
	// 82435280: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82435284: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82435288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243528C: 4800766D  bl 0x8243c8f8
	ctx.lr = 0x82435290;
	sub_8243C8F8(ctx, base);
	// 82435290: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82435294: 38800017  li r4, 0x17
	ctx.r[4].s64 = 23;
	// 82435298: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243529C: 4800765D  bl 0x8243c8f8
	ctx.lr = 0x824352A0;
	sub_8243C8F8(ctx, base);
	// 824352A0: 7D7EE9D6  mullw r11, r30, r29
	ctx.r[11].s32 = ((ctx.r[30].s32 as i64 * ctx.r[29].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824352A4: 1D6B03E8  mulli r11, r11, 0x3e8
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 1000 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824352A8: 7D6B07B4  extsw r11, r11
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824352AC: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 824352B0: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 824352B4: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 824352B8: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824352BC: FC00069C  fcfid f0, f0
	ctx.f[0].f64 = (ctx.f[0].s64 as f64);
	// 824352C0: FDA00018  frsp f13, f0
	ctx.f[13].f64 = (ctx.f[0].f64 as f32) as f64;
	// 824352C4: C00BBFFC  lfs f0, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824352C8: EC0D002A  fadds f0, f13, f0
	ctx.f[0].f64 = ((ctx.f[13].f64 + ctx.f[0].f64) as f32) as f64;
	// 824352CC: FDA0001E  fctiwz f13, f0
	ctx.f[13].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 824352D0: 7DA057AE  stfiwx f13, 0, r10
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 824352D4: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824352D8: 7D6A07B4  extsw r10, r11
	ctx.r[10].s64 = ctx.r[11].s32 as i64;
	// 824352DC: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 824352E0: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 824352E4: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 824352E8: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 824352EC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 824352F0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 824352F4: 41990008  bgt cr6, 0x824352fc
	if ctx.cr[6].gt {
	pc = 0x824352FC; continue 'dispatch;
	}
	// 824352F8: 7D7E5B78  mr r30, r11
	ctx.r[30].u64 = ctx.r[11].u64;
	pc = 0x824352FC; continue 'dispatch;
            }
            0x824352FC => {
    //   block [0x824352FC..0x82435384)
	// 824352FC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82435300: 3880002D  li r4, 0x2d
	ctx.r[4].s64 = 45;
	// 82435304: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435308: 480075F1  bl 0x8243c8f8
	ctx.lr = 0x8243530C;
	sub_8243C8F8(ctx, base);
	// 8243530C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82435310: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 82435314: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435318: 480075E1  bl 0x8243c8f8
	ctx.lr = 0x8243531C;
	sub_8243C8F8(ctx, base);
	// 8243531C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82435320: 3880002A  li r4, 0x2a
	ctx.r[4].s64 = 42;
	// 82435324: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435328: 480075D1  bl 0x8243c8f8
	ctx.lr = 0x8243532C;
	sub_8243C8F8(ctx, base);
	// 8243532C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82435330: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82435334: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435338: 480075C1  bl 0x8243c8f8
	ctx.lr = 0x8243533C;
	sub_8243C8F8(ctx, base);
	// 8243533C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82435340: 38800033  li r4, 0x33
	ctx.r[4].s64 = 51;
	// 82435344: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435348: 480075B1  bl 0x8243c8f8
	ctx.lr = 0x8243534C;
	sub_8243C8F8(ctx, base);
	// 8243534C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82435350: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 82435354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435358: 480075A1  bl 0x8243c8f8
	ctx.lr = 0x8243535C;
	sub_8243C8F8(ctx, base);
	// 8243535C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82435360: 3880001C  li r4, 0x1c
	ctx.r[4].s64 = 28;
	// 82435364: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435368: 48007591  bl 0x8243c8f8
	ctx.lr = 0x8243536C;
	sub_8243C8F8(ctx, base);
	// 8243536C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82435370: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82435374: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435378: 4800AC61  bl 0x8243ffd8
	ctx.lr = 0x8243537C;
	sub_8243FFD8(ctx, base);
	// 8243537C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82435380: 480FFD8C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435388 size=168
    let mut pc: u32 = 0x82435388;
    'dispatch: loop {
        match pc {
            0x82435388 => {
    //   block [0x82435388..0x824353D0)
	// 82435388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243538C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435390: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82435394: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82435398: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243539C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824353A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824353A4: 83DF0048  lwz r30, 0x48(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 824353A8: 917F0048  stw r11, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 824353AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824353B0: 48005EA9  bl 0x8243b258
	ctx.lr = 0x824353B4;
	sub_8243B258(ctx, base);
	// 824353B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824353B8: 419A0018  beq cr6, 0x824353d0
	if ctx.cr[6].eq {
	pc = 0x824353D0; continue 'dispatch;
	}
	// 824353BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824353C0: 386B5248  addi r3, r11, 0x5248
	ctx.r[3].s64 = ctx.r[11].s64 + 21064;
	// 824353C4: 48001D05  bl 0x824370c8
	ctx.lr = 0x824353C8;
	sub_824370C8(ctx, base);
	// 824353C8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824353CC: 4800004C  b 0x82435418
	pc = 0x82435418; continue 'dispatch;
            }
            0x824353D0 => {
    //   block [0x824353D0..0x8243540C)
	// 824353D0: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 824353D4: 93DF0048  stw r30, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[30].u32 ) };
	// 824353D8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 824353DC: 388B69F8  addi r4, r11, 0x69f8
	ctx.r[4].s64 = ctx.r[11].s64 + 27128;
	// 824353E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824353E4: 480125AD  bl 0x82447990
	ctx.lr = 0x824353E8;
	sub_82447990(ctx, base);
	// 824353E8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824353EC: 419A0020  beq cr6, 0x8243540c
	if ctx.cr[6].eq {
	pc = 0x8243540C; continue 'dispatch;
	}
	// 824353F0: 3860FED1  li r3, -0x12f
	ctx.r[3].s64 = -303;
	// 824353F4: 480015D5  bl 0x824369c8
	ctx.lr = 0x824353F8;
	sub_824369C8(ctx, base);
	// 824353F8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824353FC: 386B5214  addi r3, r11, 0x5214
	ctx.r[3].s64 = ctx.r[11].s64 + 21012;
	// 82435400: 48001CC9  bl 0x824370c8
	ctx.lr = 0x82435404;
	sub_824370C8(ctx, base);
	// 82435404: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82435408: 48000010  b 0x82435418
	pc = 0x82435418; continue 'dispatch;
            }
            0x8243540C => {
    //   block [0x8243540C..0x82435418)
	// 8243540C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435410: 4BFFFD69  bl 0x82435178
	ctx.lr = 0x82435414;
	sub_82435178(ctx, base);
	// 82435414: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82435418; continue 'dispatch;
            }
            0x82435418 => {
    //   block [0x82435418..0x82435430)
	// 82435418: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243541C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435420: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82435424: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82435428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243542C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435430 size=16
    let mut pc: u32 = 0x82435430;
    'dispatch: loop {
        match pc {
            0x82435430 => {
    //   block [0x82435430..0x82435440)
	// 82435430: 80A30460  lwz r5, 0x460(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1120 as u32) ) } as u64;
	// 82435434: 8083045C  lwz r4, 0x45c(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1116 as u32) ) } as u64;
	// 82435438: 80630458  lwz r3, 0x458(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1112 as u32) ) } as u64;
	// 8243543C: 4BFEE00C  b 0x82423448
	sub_82423448(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435440 size=96
    let mut pc: u32 = 0x82435440;
    'dispatch: loop {
        match pc {
            0x82435440 => {
    //   block [0x82435440..0x8243547C)
	// 82435440: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435444: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435448: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243544C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435450: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82435454: 48001335  bl 0x82436788
	ctx.lr = 0x82435458;
	sub_82436788(ctx, base);
	// 82435458: 815F0018  lwz r10, 0x18(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 8243545C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82435460: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82435464: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82435468: 409A0024  bne cr6, 0x8243548c
	if !ctx.cr[6].eq {
	pc = 0x8243548C; continue 'dispatch;
	}
	// 8243546C: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82435470: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82435474: 409A0008  bne cr6, 0x8243547c
	if !ctx.cr[6].eq {
	pc = 0x8243547C; continue 'dispatch;
	}
	// 82435478: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x8243547C; continue 'dispatch;
            }
            0x8243547C => {
    //   block [0x8243547C..0x8243548C)
	// 8243547C: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82435480: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82435484: 409A0008  bne cr6, 0x8243548c
	if !ctx.cr[6].eq {
	pc = 0x8243548C; continue 'dispatch;
	}
	// 82435488: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x8243548C; continue 'dispatch;
            }
            0x8243548C => {
    //   block [0x8243548C..0x824354A0)
	// 8243548C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82435490: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435494: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82435498: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243549C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824354A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824354A0 size=64
    let mut pc: u32 = 0x824354A0;
    'dispatch: loop {
        match pc {
            0x824354A0 => {
    //   block [0x824354A0..0x824354D0)
	// 824354A0: 81040018  lwz r8, 0x18(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 824354A4: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824354A8: 39630498  addi r11, r3, 0x498
	ctx.r[11].s64 = ctx.r[3].s64 + 1176;
	// 824354AC: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 824354B0: 91030480  stw r8, 0x480(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1152 as u32), ctx.r[8].u32 ) };
	// 824354B4: 8104001C  lwz r8, 0x1c(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 824354B8: 91030484  stw r8, 0x484(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1156 as u32), ctx.r[8].u32 ) };
	// 824354BC: 81040018  lwz r8, 0x18(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 824354C0: 9123048C  stw r9, 0x48c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1164 as u32), ctx.r[9].u32 ) };
	// 824354C4: 91030488  stw r8, 0x488(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1160 as u32), ctx.r[8].u32 ) };
	// 824354C8: 91230494  stw r9, 0x494(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1172 as u32), ctx.r[9].u32 ) };
	// 824354CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x824354D0; continue 'dispatch;
            }
            0x824354D0 => {
    //   block [0x824354D0..0x824354E0)
	// 824354D0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824354D4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824354D8: 4200FFF8  bdnz 0x824354d0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824354D0; continue 'dispatch;
	}
	// 824354DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824354E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824354E0 size=108
    let mut pc: u32 = 0x824354E0;
    'dispatch: loop {
        match pc {
            0x824354E0 => {
    //   block [0x824354E0..0x8243551C)
	// 824354E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824354E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824354E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824354EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824354F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824354F4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824354F8: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 824354FC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82435500: 419A001C  beq cr6, 0x8243551c
	if ctx.cr[6].eq {
	pc = 0x8243551C; continue 'dispatch;
	}
	// 82435504: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82435508: 419A0014  beq cr6, 0x8243551c
	if ctx.cr[6].eq {
	pc = 0x8243551C; continue 'dispatch;
	}
	// 8243550C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82435510: 386B52A4  addi r3, r11, 0x52a4
	ctx.r[3].s64 = ctx.r[11].s64 + 21156;
	// 82435514: 48001BB5  bl 0x824370c8
	ctx.lr = 0x82435518;
	sub_824370C8(ctx, base);
	// 82435518: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x8243551C; continue 'dispatch;
            }
            0x8243551C => {
    //   block [0x8243551C..0x82435538)
	// 8243551C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82435520: 2F0B000E  cmpwi cr6, r11, 0xe
	ctx.cr[6].compare_i32(ctx.r[11].s32, 14, &mut ctx.xer);
	// 82435524: 40990014  ble cr6, 0x82435538
	if !ctx.cr[6].gt {
	pc = 0x82435538; continue 'dispatch;
	}
	// 82435528: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243552C: 386B5278  addi r3, r11, 0x5278
	ctx.r[3].s64 = ctx.r[11].s64 + 21112;
	// 82435530: 48001B99  bl 0x824370c8
	ctx.lr = 0x82435534;
	sub_824370C8(ctx, base);
	// 82435534: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82435538; continue 'dispatch;
            }
            0x82435538 => {
    //   block [0x82435538..0x8243554C)
	// 82435538: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243553C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82435544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82435548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435550 size=32
    let mut pc: u32 = 0x82435550;
    'dispatch: loop {
        match pc {
            0x82435550 => {
    //   block [0x82435550..0x82435570)
	// 82435550: 39630480  addi r11, r3, 0x480
	ctx.r[11].s64 = ctx.r[3].s64 + 1152;
	// 82435554: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82435558: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243555C: 7D4A2214  add r10, r10, r4
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[4].u64;
	// 82435560: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82435564: 4099000C  ble cr6, 0x82435570
	if !ctx.cr[6].gt {
		crate::recompiler::externs::call(ctx, base, 0x82435570);
		return;
	}
	// 82435568: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243556C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435588 size=68
    let mut pc: u32 = 0x82435588;
    'dispatch: loop {
        match pc {
            0x82435588 => {
    //   block [0x82435588..0x824355CC)
	// 82435588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243558C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435590: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82435594: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435598: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243559C: 480011ED  bl 0x82436788
	ctx.lr = 0x824355A0;
	sub_82436788(ctx, base);
	// 824355A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824355A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824355A8: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 824355AC: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 824355B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824355B4: 4E800421  bctrl
	ctx.lr = 0x824355B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824355B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824355BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824355C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824355C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824355C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824355D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824355D0 size=68
    let mut pc: u32 = 0x824355D0;
    'dispatch: loop {
        match pc {
            0x824355D0 => {
    //   block [0x824355D0..0x82435614)
	// 824355D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824355D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824355D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824355DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824355E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824355E4: 480011A5  bl 0x82436788
	ctx.lr = 0x824355E8;
	sub_82436788(ctx, base);
	// 824355E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824355EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824355F0: 806B0030  lwz r3, 0x30(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 824355F4: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 824355F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824355FC: 4E800421  bctrl
	ctx.lr = 0x82435600;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82435600: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82435604: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435608: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243560C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82435610: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435618 size=16
    let mut pc: u32 = 0x82435618;
    'dispatch: loop {
        match pc {
            0x82435618 => {
    //   block [0x82435618..0x82435628)
	// 82435618: 81630494  lwz r11, 0x494(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1172 as u32) ) } as u64;
	// 8243561C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82435620: 91630494  stw r11, 0x494(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1172 as u32), ctx.r[11].u32 ) };
	// 82435624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435628 size=16
    let mut pc: u32 = 0x82435628;
    'dispatch: loop {
        match pc {
            0x82435628 => {
    //   block [0x82435628..0x82435638)
	// 82435628: 81630494  lwz r11, 0x494(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1172 as u32) ) } as u64;
	// 8243562C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82435630: 91630494  stw r11, 0x494(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1172 as u32), ctx.r[11].u32 ) };
	// 82435634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82435638 size=8
    let mut pc: u32 = 0x82435638;
    'dispatch: loop {
        match pc {
            0x82435638 => {
    //   block [0x82435638..0x82435640)
	// 82435638: 80630494  lwz r3, 0x494(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1172 as u32) ) } as u64;
	// 8243563C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435640(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435640 size=144
    let mut pc: u32 = 0x82435640;
    'dispatch: loop {
        match pc {
            0x82435640 => {
    //   block [0x82435640..0x82435664)
	// 82435640: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435644: 480FFA71  bl 0x825350b4
	ctx.lr = 0x82435648;
	sub_82535080(ctx, base);
	// 82435648: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243564C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82435650: 3B600001  li r27, 1
	ctx.r[27].s64 = 1;
	// 82435654: 3BAB9B78  addi r29, r11, -0x6488
	ctx.r[29].s64 = ctx.r[11].s64 + -25736;
	// 82435658: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8243565C: 3BDDFFD8  addi r30, r29, -0x28
	ctx.r[30].s64 = ctx.r[29].s64 + -40;
	// 82435660: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	pc = 0x82435664; continue 'dispatch;
            }
            0x82435664 => {
    //   block [0x82435664..0x824356A0)
	// 82435664: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82435668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243566C: 4BFF77ED  bl 0x8242ce58
	ctx.lr = 0x82435670;
	sub_8242CE58(ctx, base);
	// 82435670: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82435674: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82435678: 419A0028  beq cr6, 0x824356a0
	if ctx.cr[6].eq {
	pc = 0x824356A0; continue 'dispatch;
	}
	// 8243567C: 3BFF0020  addi r31, r31, 0x20
	ctx.r[31].s64 = ctx.r[31].s64 + 32;
	// 82435680: 397D0100  addi r11, r29, 0x100
	ctx.r[11].s64 = ctx.r[29].s64 + 256;
	// 82435684: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82435688: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 8243568C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82435690: 4198FFD4  blt cr6, 0x82435664
	if ctx.cr[6].lt {
	pc = 0x82435664; continue 'dispatch;
	}
	// 82435694: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82435698: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243569C: 480FFA68  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x824356A0 => {
    //   block [0x824356A0..0x824356AC)
	// 824356A0: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 824356A4: 40990020  ble cr6, 0x824356c4
	if !ctx.cr[6].gt {
	pc = 0x824356C4; continue 'dispatch;
	}
	// 824356A8: 3BFDFFD8  addi r31, r29, -0x28
	ctx.r[31].s64 = ctx.r[29].s64 + -40;
	pc = 0x824356AC; continue 'dispatch;
            }
            0x824356AC => {
    //   block [0x824356AC..0x824356C4)
	// 824356AC: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824356B0: 4BFF7889  bl 0x8242cf38
	ctx.lr = 0x824356B4;
	sub_8242CF38(ctx, base);
	// 824356B4: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 824356B8: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 824356BC: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 824356C0: 409AFFEC  bne cr6, 0x824356ac
	if !ctx.cr[6].eq {
	pc = 0x824356AC; continue 'dispatch;
	}
	pc = 0x824356C4; continue 'dispatch;
            }
            0x824356C4 => {
    //   block [0x824356C4..0x824356D0)
	// 824356C4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824356C8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824356CC: 480FFA38  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824356D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824356D0 size=24
    let mut pc: u32 = 0x824356D0;
    'dispatch: loop {
        match pc {
            0x824356D0 => {
    //   block [0x824356D0..0x824356E8)
	// 824356D0: 816305B0  lwz r11, 0x5b0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1456 as u32) ) } as u64;
	// 824356D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824356D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824356DC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824356E0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 824356E4: 4BFF78E4  b 0x8242cfc8
	sub_8242CFC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824356F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824356F0 size=24
    let mut pc: u32 = 0x824356F0;
    'dispatch: loop {
        match pc {
            0x824356F0 => {
    //   block [0x824356F0..0x82435708)
	// 824356F0: 816305B0  lwz r11, 0x5b0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1456 as u32) ) } as u64;
	// 824356F4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824356F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824356FC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82435700: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82435704: 4BFF795C  b 0x8242d060
	sub_8242D060(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435710 size=208
    let mut pc: u32 = 0x82435710;
    'dispatch: loop {
        match pc {
            0x82435710 => {
    //   block [0x82435710..0x82435758)
	// 82435710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435714: 480FF9A1  bl 0x825350b4
	ctx.lr = 0x82435718;
	sub_82535080(ctx, base);
	// 82435718: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243571C: 83E30028  lwz r31, 0x28(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82435720: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82435724: 83A3002C  lwz r29, 0x2c(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82435728: 83C30030  lwz r30, 0x30(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 8243572C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82435730: 83830008  lwz r28, 8(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82435734: 8363000C  lwz r27, 0xc(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82435738: 41990020  bgt cr6, 0x82435758
	if ctx.cr[6].gt {
	pc = 0x82435758; continue 'dispatch;
	}
	// 8243573C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82435740: 41990018  bgt cr6, 0x82435758
	if ctx.cr[6].gt {
	pc = 0x82435758; continue 'dispatch;
	}
	// 82435744: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82435748: 409A0010  bne cr6, 0x82435758
	if !ctx.cr[6].eq {
	pc = 0x82435758; continue 'dispatch;
	}
	// 8243574C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82435750: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82435754: 480FF9B0  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82435758 => {
    //   block [0x82435758..0x82435778)
	// 82435758: 2F1F0002  cmpwi cr6, r31, 2
	ctx.cr[6].compare_i32(ctx.r[31].s32, 2, &mut ctx.xer);
	// 8243575C: 4199001C  bgt cr6, 0x82435778
	if ctx.cr[6].gt {
	pc = 0x82435778; continue 'dispatch;
	}
	// 82435760: 2F1F0010  cmpwi cr6, r31, 0x10
	ctx.cr[6].compare_i32(ctx.r[31].s32, 16, &mut ctx.xer);
	// 82435764: 40990014  ble cr6, 0x82435778
	if !ctx.cr[6].gt {
	pc = 0x82435778; continue 'dispatch;
	}
	// 82435768: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243576C: 386B5310  addi r3, r11, 0x5310
	ctx.r[3].s64 = ctx.r[11].s64 + 21264;
	// 82435770: 48001959  bl 0x824370c8
	ctx.lr = 0x82435774;
	sub_824370C8(ctx, base);
	// 82435774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	pc = 0x82435778; continue 'dispatch;
            }
            0x82435778 => {
    //   block [0x82435778..0x8243579C)
	// 82435778: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 8243577C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435780: 4BFFF849  bl 0x82434fc8
	ctx.lr = 0x82435784;
	sub_82434FC8(ctx, base);
	// 82435784: 7F1D1800  cmpw cr6, r29, r3
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82435788: 40980014  bge cr6, 0x8243579c
	if !ctx.cr[6].lt {
	pc = 0x8243579C; continue 'dispatch;
	}
	// 8243578C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82435790: 386B52EC  addi r3, r11, 0x52ec
	ctx.r[3].s64 = ctx.r[11].s64 + 21228;
	// 82435794: 48001935  bl 0x824370c8
	ctx.lr = 0x82435798;
	sub_824370C8(ctx, base);
	// 82435798: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	pc = 0x8243579C; continue 'dispatch;
            }
            0x8243579C => {
    //   block [0x8243579C..0x824357AC)
	// 8243579C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824357A0: 40990034  ble cr6, 0x824357d4
	if !ctx.cr[6].gt {
	pc = 0x824357D4; continue 'dispatch;
	}
	// 824357A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824357A8: 3BAB52C8  addi r29, r11, 0x52c8
	ctx.r[29].s64 = ctx.r[11].s64 + 21192;
	pc = 0x824357AC; continue 'dispatch;
            }
            0x824357AC => {
    //   block [0x824357AC..0x824357C4)
	// 824357AC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 824357B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824357B4: 409A0010  bne cr6, 0x824357c4
	if !ctx.cr[6].eq {
	pc = 0x824357C4; continue 'dispatch;
	}
	// 824357B8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824357BC: 4800190D  bl 0x824370c8
	ctx.lr = 0x824357C0;
	sub_824370C8(ctx, base);
	// 824357C0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	pc = 0x824357C4; continue 'dispatch;
            }
            0x824357C4 => {
    //   block [0x824357C4..0x824357D4)
	// 824357C4: 3BFFFFFF  addi r31, r31, -1
	ctx.r[31].s64 = ctx.r[31].s64 + -1;
	// 824357C8: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 824357CC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824357D0: 409AFFDC  bne cr6, 0x824357ac
	if !ctx.cr[6].eq {
	pc = 0x824357AC; continue 'dispatch;
	}
	pc = 0x824357D4; continue 'dispatch;
            }
            0x824357D4 => {
    //   block [0x824357D4..0x824357E0)
	// 824357D4: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 824357D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824357DC: 480FF928  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824357E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824357E0 size=168
    let mut pc: u32 = 0x824357E0;
    'dispatch: loop {
        match pc {
            0x824357E0 => {
    //   block [0x824357E0..0x8243580C)
	// 824357E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824357E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824357E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824357EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824357F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824357F4: 817F0494  lwz r11, 0x494(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1172 as u32) ) } as u64;
	// 824357F8: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 824357FC: 41980028  blt cr6, 0x82435824
	if ctx.cr[6].lt {
	pc = 0x82435824; continue 'dispatch;
	}
	// 82435800: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82435804: 386B5334  addi r3, r11, 0x5334
	ctx.r[3].s64 = ctx.r[11].s64 + 21300;
	// 82435808: 480018C1  bl 0x824370c8
	ctx.lr = 0x8243580C;
	sub_824370C8(ctx, base);
	pc = 0x8243580C; continue 'dispatch;
            }
            0x8243580C => {
    //   block [0x8243580C..0x82435824)
	// 8243580C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82435810: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82435814: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435818: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243581C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82435820: 4E800020  blr
	return;
            }
            0x82435824 => {
    //   block [0x82435824..0x82435844)
	// 82435824: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82435828: 4198FFE4  blt cr6, 0x8243580c
	if ctx.cr[6].lt {
	pc = 0x8243580C; continue 'dispatch;
	}
	// 8243582C: 817F0480  lwz r11, 0x480(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1152 as u32) ) } as u64;
	// 82435830: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82435834: 419A0010  beq cr6, 0x82435844
	if ctx.cr[6].eq {
	pc = 0x82435844; continue 'dispatch;
	}
	// 82435838: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243583C: 4BFFFD15  bl 0x82435550
	ctx.lr = 0x82435840;
	sub_82435550(ctx, base);
	// 82435840: 4800000C  b 0x8243584c
	pc = 0x8243584C; continue 'dispatch;
            }
            0x82435844 => {
    //   block [0x82435844..0x8243584C)
	// 82435844: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82435848: 4BFFFD41  bl 0x82435588
	ctx.lr = 0x8243584C;
	sub_82435588(ctx, base);
	pc = 0x8243584C; continue 'dispatch;
            }
            0x8243584C => {
    //   block [0x8243584C..0x82435870)
	// 8243584C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82435850: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82435854: 419A001C  beq cr6, 0x82435870
	if ctx.cr[6].eq {
	pc = 0x82435870; continue 'dispatch;
	}
	// 82435858: 817F0494  lwz r11, 0x494(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1172 as u32) ) } as u64;
	// 8243585C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435860: 396B0126  addi r11, r11, 0x126
	ctx.r[11].s64 = ctx.r[11].s64 + 294;
	// 82435864: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82435868: 7D4BF92E  stwx r10, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u32) };
	// 8243586C: 4BFFFDAD  bl 0x82435618
	ctx.lr = 0x82435870;
	sub_82435618(ctx, base);
	pc = 0x82435870; continue 'dispatch;
            }
            0x82435870 => {
    //   block [0x82435870..0x82435888)
	// 82435870: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82435874: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82435878: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243587C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82435880: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82435884: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435888 size=68
    let mut pc: u32 = 0x82435888;
    'dispatch: loop {
        match pc {
            0x82435888 => {
    //   block [0x82435888..0x824358B0)
	// 82435888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243588C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435890: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82435894: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435898: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243589C: 817F0480  lwz r11, 0x480(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1152 as u32) ) } as u64;
	// 824358A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824358A4: 409A000C  bne cr6, 0x824358b0
	if !ctx.cr[6].eq {
	pc = 0x824358B0; continue 'dispatch;
	}
	// 824358A8: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 824358AC: 4BFFFD25  bl 0x824355d0
	ctx.lr = 0x824358B0;
	sub_824355D0(ctx, base);
	pc = 0x824358B0; continue 'dispatch;
            }
            0x824358B0 => {
    //   block [0x824358B0..0x824358CC)
	// 824358B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824358B4: 4BFFFD75  bl 0x82435628
	ctx.lr = 0x824358B8;
	sub_82435628(ctx, base);
	// 824358B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824358BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824358C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824358C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824358C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824358D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824358D0 size=76
    let mut pc: u32 = 0x824358D0;
    'dispatch: loop {
        match pc {
            0x824358D0 => {
    //   block [0x824358D0..0x824358EC)
	// 824358D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824358D4: 480FF7E5  bl 0x825350b8
	ctx.lr = 0x824358D8;
	sub_82535080(ctx, base);
	// 824358D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824358DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824358E0: 3BA00020  li r29, 0x20
	ctx.r[29].s64 = 32;
	// 824358E4: 3BFE0514  addi r31, r30, 0x514
	ctx.r[31].s64 = ctx.r[30].s64 + 1300;
	// 824358E8: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	pc = 0x824358EC; continue 'dispatch;
            }
            0x824358EC => {
    //   block [0x824358EC..0x82435904)
	// 824358EC: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 824358F0: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 824358F4: 419A0010  beq cr6, 0x82435904
	if ctx.cr[6].eq {
	pc = 0x82435904; continue 'dispatch;
	}
	// 824358F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824358FC: 4BFFFF8D  bl 0x82435888
	ctx.lr = 0x82435900;
	sub_82435888(ctx, base);
	// 82435900: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	pc = 0x82435904; continue 'dispatch;
            }
            0x82435904 => {
    //   block [0x82435904..0x8243591C)
	// 82435904: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 82435908: 3BFFFFFC  addi r31, r31, -4
	ctx.r[31].s64 = ctx.r[31].s64 + -4;
	// 8243590C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82435910: 409AFFDC  bne cr6, 0x824358ec
	if !ctx.cr[6].eq {
	pc = 0x824358EC; continue 'dispatch;
	}
	// 82435914: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82435918: 480FF7F0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82435920 size=224
    let mut pc: u32 = 0x82435920;
    'dispatch: loop {
        match pc {
            0x82435920 => {
    //   block [0x82435920..0x82435958)
	// 82435920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435924: 480FF795  bl 0x825350b8
	ctx.lr = 0x82435928;
	sub_82535080(ctx, base);
	// 82435928: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243592C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82435930: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82435934: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82435938: 4BFFFDD9  bl 0x82435710
	ctx.lr = 0x8243593C;
	sub_82435710(ctx, base);
	// 8243593C: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82435940: 409A0018  bne cr6, 0x82435958
	if !ctx.cr[6].eq {
	pc = 0x82435958; continue 'dispatch;
	}
	// 82435944: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82435948: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243594C: 917C0000  stw r11, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82435950: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82435954: 480FF7B4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82435958 => {
    //   block [0x82435958..0x82435A00)
	// 82435958: 38E10058  addi r7, r1, 0x58
	ctx.r[7].s64 = ctx.r[1].s64 + 88;
	// 8243595C: 83DF0010  lwz r30, 0x10(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82435960: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82435964: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82435968: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8243596C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435970: 4BFFF511  bl 0x82434e80
	ctx.lr = 0x82435974;
	sub_82434E80(ctx, base);
	// 82435974: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82435978: 4BFFF549  bl 0x82434ec0
	ctx.lr = 0x8243597C;
	sub_82434EC0(ctx, base);
	// 8243597C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82435980: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82435984: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82435988: 394A000F  addi r10, r10, 0xf
	ctx.r[10].s64 = ctx.r[10].s64 + 15;
	// 8243598C: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82435990: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82435994: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 82435998: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243599C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 824359A0: 7D690E70  srawi r9, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 824359A4: 55482036  slwi r8, r10, 4
	ctx.r[8].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 824359A8: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824359AC: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 824359B0: 3929007F  addi r9, r9, 0x7f
	ctx.r[9].s64 = ctx.r[9].s64 + 127;
	// 824359B4: 7D293E70  srawi r9, r9, 7
	ctx.xer.ca = (ctx.r[9].s32 < 0) && ((ctx.r[9].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[9].s64 = (ctx.r[9].s32 >> 7) as i64;
	// 824359B8: 7D290194  addze r9, r9
	tmp.s64 = ctx.r[9].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[9].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824359BC: 7D080E70  srawi r8, r8, 1
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 1) as i64;
	// 824359C0: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 824359C4: 7D673E70  srawi r7, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 824359C8: 7D6941D6  mullw r11, r9, r8
	ctx.r[11].s32 = ((ctx.r[9].s32 as i64 * ctx.r[8].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824359CC: 7D270194  addze r9, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[9].s64 = tmp.s64;
	// 824359D0: 7D4951D6  mullw r10, r9, r10
	ctx.r[10].s32 = ((ctx.r[9].s32 as i64 * ctx.r[10].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 824359D4: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824359D8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824359DC: 556A402E  slwi r10, r11, 8
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824359E0: 556B482C  slwi r11, r11, 9
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(9);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824359E4: 394A0080  addi r10, r10, 0x80
	ctx.r[10].s64 = ctx.r[10].s64 + 128;
	// 824359E8: 396B0100  addi r11, r11, 0x100
	ctx.r[11].s64 = ctx.r[11].s64 + 256;
	// 824359EC: 7D4AF1D6  mullw r10, r10, r30
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * ctx.r[30].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 824359F0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824359F4: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824359F8: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824359FC: 480FF70C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435A00 size=180
    let mut pc: u32 = 0x82435A00;
    'dispatch: loop {
        match pc {
            0x82435A00 => {
    //   block [0x82435A00..0x82435AB4)
	// 82435A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435A04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435A08: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82435A0C: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435A10: 3921006C  addi r9, r1, 0x6c
	ctx.r[9].s64 = ctx.r[1].s64 + 108;
	// 82435A14: 39010068  addi r8, r1, 0x68
	ctx.r[8].s64 = ctx.r[1].s64 + 104;
	// 82435A18: 38E10064  addi r7, r1, 0x64
	ctx.r[7].s64 = ctx.r[1].s64 + 100;
	// 82435A1C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82435A20: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 82435A24: 38810058  addi r4, r1, 0x58
	ctx.r[4].s64 = ctx.r[1].s64 + 88;
	// 82435A28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82435A2C: 4BFFF33D  bl 0x82434d68
	ctx.lr = 0x82435A30;
	sub_82434D68(ctx, base);
	// 82435A30: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82435A34: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82435A38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435A3C: 4BFFFEE5  bl 0x82435920
	ctx.lr = 0x82435A40;
	sub_82435920(ctx, base);
	// 82435A40: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82435A44: 38810074  addi r4, r1, 0x74
	ctx.r[4].s64 = ctx.r[1].s64 + 116;
	// 82435A48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435A4C: 4BFFF4FD  bl 0x82434f48
	ctx.lr = 0x82435A50;
	sub_82434F48(ctx, base);
	// 82435A50: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82435A54: 4BFFF515  bl 0x82434f68
	ctx.lr = 0x82435A58;
	sub_82434F68(ctx, base);
	// 82435A58: 7D6A1A14  add r11, r10, r3
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82435A5C: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82435A60: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A64: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82435A68: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A6C: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82435A70: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A74: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82435A78: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A7C: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82435A80: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A84: 81410064  lwz r10, 0x64(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82435A88: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A8C: 81410068  lwz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82435A90: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A94: 8141006C  lwz r10, 0x6c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 82435A98: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435A9C: 386B08C0  addi r3, r11, 0x8c0
	ctx.r[3].s64 = ctx.r[11].s64 + 2240;
	// 82435AA0: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82435AA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435AA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82435AAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82435AB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435AB8 size=100
    let mut pc: u32 = 0x82435AB8;
    'dispatch: loop {
        match pc {
            0x82435AB8 => {
    //   block [0x82435AB8..0x82435AEC)
	// 82435AB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435ABC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82435AC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82435AC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82435AC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435ACC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82435AD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82435AD4: 409A0018  bne cr6, 0x82435aec
	if !ctx.cr[6].eq {
	pc = 0x82435AEC; continue 'dispatch;
	}
	// 82435AD8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82435ADC: 386B5358  addi r3, r11, 0x5358
	ctx.r[3].s64 = ctx.r[11].s64 + 21336;
	// 82435AE0: 480015E9  bl 0x824370c8
	ctx.lr = 0x82435AE4;
	sub_824370C8(ctx, base);
	// 82435AE4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82435AE8: 4800001C  b 0x82435b04
	pc = 0x82435B04; continue 'dispatch;
            }
            0x82435AEC => {
    //   block [0x82435AEC..0x82435B04)
	// 82435AEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435AF0: 4BFFFF11  bl 0x82435a00
	ctx.lr = 0x82435AF4;
	sub_82435A00(ctx, base);
	// 82435AF4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82435AF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82435AFC: 4BFFF475  bl 0x82434f70
	ctx.lr = 0x82435B00;
	sub_82434F70(ctx, base);
	// 82435B00: 7C63F214  add r3, r3, r30
	ctx.r[3].u64 = ctx.r[3].u64 + ctx.r[30].u64;
	pc = 0x82435B04; continue 'dispatch;
            }
            0x82435B04 => {
    //   block [0x82435B04..0x82435B1C)
	// 82435B04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82435B08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82435B0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82435B10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82435B14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82435B18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82435B20 size=292
    let mut pc: u32 = 0x82435B20;
    'dispatch: loop {
        match pc {
            0x82435B20 => {
    //   block [0x82435B20..0x82435BDC)
	// 82435B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435B24: 480FF595  bl 0x825350b8
	ctx.lr = 0x82435B28;
	sub_82535080(ctx, base);
	// 82435B28: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435B2C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82435B30: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82435B34: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82435B38: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82435B3C: 396B000F  addi r11, r11, 0xf
	ctx.r[11].s64 = ctx.r[11].s64 + 15;
	// 82435B40: 81240028  lwz r9, 0x28(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82435B44: 394A000F  addi r10, r10, 0xf
	ctx.r[10].s64 = ctx.r[10].s64 + 15;
	// 82435B48: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82435B4C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82435B50: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82435B54: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 82435B58: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82435B5C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82435B60: 7D680E70  srawi r8, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82435B64: 55472036  slwi r7, r10, 4
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82435B68: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82435B6C: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 82435B70: 3908007F  addi r8, r8, 0x7f
	ctx.r[8].s64 = ctx.r[8].s64 + 127;
	// 82435B74: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82435B78: 7D083E70  srawi r8, r8, 7
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 7) as i64;
	// 82435B7C: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82435B80: 7CE70E70  srawi r7, r7, 1
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 1) as i64;
	// 82435B84: 7CE70194  addze r7, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[7].s64 = tmp.s64;
	// 82435B88: 7D663E70  srawi r6, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 82435B8C: 7D6839D6  mullw r11, r8, r7
	ctx.r[11].s32 = ((ctx.r[8].s32 as i64 * ctx.r[7].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82435B90: 7D060194  addze r8, r6
	tmp.s64 = ctx.r[6].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[6].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82435B94: 7D4851D6  mullw r10, r8, r10
	ctx.r[10].s32 = ((ctx.r[8].s32 as i64 * ctx.r[10].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82435B98: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82435B9C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435BA0: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82435BA4: 3BCB0080  addi r30, r11, 0x80
	ctx.r[30].s64 = ctx.r[11].s64 + 128;
	// 82435BA8: 419A0048  beq cr6, 0x82435bf0
	if ctx.cr[6].eq {
	pc = 0x82435BF0; continue 'dispatch;
	}
	// 82435BAC: 2F090002  cmpwi cr6, r9, 2
	ctx.cr[6].compare_i32(ctx.r[9].s32, 2, &mut ctx.xer);
	// 82435BB0: 4198002C  blt cr6, 0x82435bdc
	if ctx.cr[6].lt {
	pc = 0x82435BDC; continue 'dispatch;
	}
	// 82435BB4: 8164002C  lwz r11, 0x2c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 82435BB8: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82435BBC: 41980020  blt cr6, 0x82435bdc
	if ctx.cr[6].lt {
	pc = 0x82435BDC; continue 'dispatch;
	}
	// 82435BC0: 81640030  lwz r11, 0x30(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82435BC4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82435BC8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82435BCC: 81640030  lwz r11, 0x30(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(48 as u32) ) } as u64;
	// 82435BD0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82435BD4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82435BD8: 4800003C  b 0x82435c14
	pc = 0x82435C14; continue 'dispatch;
            }
            0x82435BDC => {
    //   block [0x82435BDC..0x82435BF0)
	// 82435BDC: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82435BE0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82435BE4: 939F0004  stw r28, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[28].u32 ) };
	// 82435BE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82435BEC: 480FF51C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82435BF0 => {
    //   block [0x82435BF0..0x82435C14)
	// 82435BF0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82435BF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82435BF8: 4BFFFBE9  bl 0x824357e0
	ctx.lr = 0x82435BFC;
	sub_824357E0(ctx, base);
	// 82435BFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82435C00: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82435C04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82435C08: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82435C0C: 4BFFFBD5  bl 0x824357e0
	ctx.lr = 0x82435C10;
	sub_824357E0(ctx, base);
	// 82435C10: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	pc = 0x82435C14; continue 'dispatch;
            }
            0x82435C14 => {
    //   block [0x82435C14..0x82435C2C)
	// 82435C14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82435C18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82435C1C: 419A0010  beq cr6, 0x82435c2c
	if ctx.cr[6].eq {
	pc = 0x82435C2C; continue 'dispatch;
	}
	// 82435C20: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82435C24: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82435C28: 409A0010  bne cr6, 0x82435c38
	if !ctx.cr[6].eq {
	pc = 0x82435C38; continue 'dispatch;
	}
	pc = 0x82435C2C; continue 'dispatch;
            }
            0x82435C2C => {
    //   block [0x82435C2C..0x82435C38)
	// 82435C2C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82435C30: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82435C34: 480FF4D4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82435C38 => {
    //   block [0x82435C38..0x82435C44)
	// 82435C38: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435C3C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82435C40: 480FF4C8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82435C48 size=328
    let mut pc: u32 = 0x82435C48;
    'dispatch: loop {
        match pc {
            0x82435C48 => {
    //   block [0x82435C48..0x82435D0C)
	// 82435C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435C4C: 480FF461  bl 0x825350ac
	ctx.lr = 0x82435C50;
	sub_82535080(ctx, base);
	// 82435C50: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435C54: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82435C58: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82435C5C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82435C60: 3B200000  li r25, 0
	ctx.r[25].s64 = 0;
	// 82435C64: 839F0010  lwz r28, 0x10(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82435C68: 83DF0008  lwz r30, 8(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82435C6C: 83BF000C  lwz r29, 0xc(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82435C70: 807F0024  lwz r3, 0x24(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82435C74: 4BFFF24D  bl 0x82434ec0
	ctx.lr = 0x82435C78;
	sub_82434EC0(ctx, base);
	// 82435C78: 397E000F  addi r11, r30, 0xf
	ctx.r[11].s64 = ctx.r[30].s64 + 15;
	// 82435C7C: 813F0028  lwz r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82435C80: 395D000F  addi r10, r29, 0xf
	ctx.r[10].s64 = ctx.r[29].s64 + 15;
	// 82435C84: 7D6B2670  srawi r11, r11, 4
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 4) as i64;
	// 82435C88: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82435C8C: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82435C90: 7D4A2670  srawi r10, r10, 4
	ctx.xer.ca = (ctx.r[10].s32 < 0) && ((ctx.r[10].u32 & ((1u32 << 4) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[10].s32 >> 4) as i64;
	// 82435C94: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82435C98: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82435C9C: 7D680E70  srawi r8, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82435CA0: 55472036  slwi r7, r10, 4
	ctx.r[7].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82435CA4: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82435CA8: 396B007F  addi r11, r11, 0x7f
	ctx.r[11].s64 = ctx.r[11].s64 + 127;
	// 82435CAC: 3908007F  addi r8, r8, 0x7f
	ctx.r[8].s64 = ctx.r[8].s64 + 127;
	// 82435CB0: 7D083E70  srawi r8, r8, 7
	ctx.xer.ca = (ctx.r[8].s32 < 0) && ((ctx.r[8].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[8].s64 = (ctx.r[8].s32 >> 7) as i64;
	// 82435CB4: 7D080194  addze r8, r8
	tmp.s64 = ctx.r[8].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[8].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82435CB8: 7CE70E70  srawi r7, r7, 1
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[7].s64 = (ctx.r[7].s32 >> 1) as i64;
	// 82435CBC: 7CE70194  addze r7, r7
	tmp.s64 = ctx.r[7].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[7].u32);
	ctx.r[7].s64 = tmp.s64;
	// 82435CC0: 7D663E70  srawi r6, r11, 7
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 7) - 1)) != 0);
	ctx.r[6].s64 = (ctx.r[11].s32 >> 7) as i64;
	// 82435CC4: 7D6839D6  mullw r11, r8, r7
	ctx.r[11].s32 = ((ctx.r[8].s32 as i64 * ctx.r[7].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82435CC8: 7D060194  addze r8, r6
	tmp.s64 = ctx.r[6].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[6].u32);
	ctx.r[8].s64 = tmp.s64;
	// 82435CCC: 7D4851D6  mullw r10, r8, r10
	ctx.r[10].s32 = ((ctx.r[8].s32 as i64 * ctx.r[10].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82435CD0: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82435CD4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435CD8: 556B402E  slwi r11, r11, 8
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(8);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82435CDC: 3BCB0080  addi r30, r11, 0x80
	ctx.r[30].s64 = ctx.r[11].s64 + 128;
	// 82435CE0: 419A0070  beq cr6, 0x82435d50
	if ctx.cr[6].eq {
	pc = 0x82435D50; continue 'dispatch;
	}
	// 82435CE4: 397C0002  addi r11, r28, 2
	ctx.r[11].s64 = ctx.r[28].s64 + 2;
	// 82435CE8: 7F095800  cmpw cr6, r9, r11
	ctx.cr[6].compare_i32(ctx.r[9].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82435CEC: 41980058  blt cr6, 0x82435d44
	if ctx.cr[6].lt {
	pc = 0x82435D44; continue 'dispatch;
	}
	// 82435CF0: 817F002C  lwz r11, 0x2c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) } as u64;
	// 82435CF4: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82435CF8: 4198004C  blt cr6, 0x82435d44
	if ctx.cr[6].lt {
	pc = 0x82435D44; continue 'dispatch;
	}
	// 82435CFC: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82435D00: 40990084  ble cr6, 0x82435d84
	if !ctx.cr[6].gt {
	pc = 0x82435D84; continue 'dispatch;
	}
	// 82435D04: 7F6ADB78  mr r10, r27
	ctx.r[10].u64 = ctx.r[27].u64;
	// 82435D08: 213B0008  subfic r9, r27, 8
	ctx.xer.ca = ctx.r[27].u32 <= 8 as u32;
	ctx.r[9].s64 = (8 as i64) - ctx.r[27].s64;
	pc = 0x82435D0C; continue 'dispatch;
            }
            0x82435D0C => {
    //   block [0x82435D0C..0x82435D28)
	// 82435D0C: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 82435D10: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82435D14: 7D6B502E  lwzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82435D18: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82435D1C: 916A0000  stw r11, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82435D20: 409A0008  bne cr6, 0x82435d28
	if !ctx.cr[6].eq {
	pc = 0x82435D28; continue 'dispatch;
	}
	// 82435D24: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	pc = 0x82435D28; continue 'dispatch;
            }
            0x82435D28 => {
    //   block [0x82435D28..0x82435D44)
	// 82435D28: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82435D2C: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82435D30: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82435D34: 409AFFD8  bne cr6, 0x82435d0c
	if !ctx.cr[6].eq {
	pc = 0x82435D0C; continue 'dispatch;
	}
	// 82435D38: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82435D3C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82435D40: 480FF3BC  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            0x82435D44 => {
    //   block [0x82435D44..0x82435D50)
	// 82435D44: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82435D48: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82435D4C: 480FF3B0  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            0x82435D50 => {
    //   block [0x82435D50..0x82435D58)
	// 82435D50: 2F1C0000  cmpwi cr6, r28, 0
	ctx.cr[6].compare_i32(ctx.r[28].s32, 0, &mut ctx.xer);
	// 82435D54: 40990030  ble cr6, 0x82435d84
	if !ctx.cr[6].gt {
	pc = 0x82435D84; continue 'dispatch;
	}
	pc = 0x82435D58; continue 'dispatch;
            }
            0x82435D58 => {
    //   block [0x82435D58..0x82435D74)
	// 82435D58: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82435D5C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82435D60: 4BFFFA81  bl 0x824357e0
	ctx.lr = 0x82435D64;
	sub_824357E0(ctx, base);
	// 82435D64: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82435D68: 907B0000  stw r3, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82435D6C: 409A0008  bne cr6, 0x82435d74
	if !ctx.cr[6].eq {
	pc = 0x82435D74; continue 'dispatch;
	}
	// 82435D70: 3B20FFFF  li r25, -1
	ctx.r[25].s64 = -1;
	pc = 0x82435D74; continue 'dispatch;
            }
            0x82435D74 => {
    //   block [0x82435D74..0x82435D84)
	// 82435D74: 3B9CFFFF  addi r28, r28, -1
	ctx.r[28].s64 = ctx.r[28].s64 + -1;
	// 82435D78: 3B7B0004  addi r27, r27, 4
	ctx.r[27].s64 = ctx.r[27].s64 + 4;
	// 82435D7C: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 82435D80: 409AFFD8  bne cr6, 0x82435d58
	if !ctx.cr[6].eq {
	pc = 0x82435D58; continue 'dispatch;
	}
	pc = 0x82435D84; continue 'dispatch;
            }
            0x82435D84 => {
    //   block [0x82435D84..0x82435D90)
	// 82435D84: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82435D88: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82435D8C: 480FF370  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82435D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82435D90 size=1008
    let mut pc: u32 = 0x82435D90;
    'dispatch: loop {
        match pc {
            0x82435D90 => {
    //   block [0x82435D90..0x82435E9C)
	// 82435D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82435D94: 480FF2ED  bl 0x82535080
	ctx.lr = 0x82435D98;
	sub_82535080(ctx, base);
	// 82435D98: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82435D9C: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82435DA0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82435DA4: 3BEB9B48  addi r31, r11, -0x64b8
	ctx.r[31].s64 = ctx.r[11].s64 + -25784;
	// 82435DA8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82435DAC: 393F0140  addi r9, r31, 0x140
	ctx.r[9].s64 = ctx.r[31].s64 + 320;
	// 82435DB0: 391F013C  addi r8, r31, 0x13c
	ctx.r[8].s64 = ctx.r[31].s64 + 316;
	// 82435DB4: 38FF0004  addi r7, r31, 4
	ctx.r[7].s64 = ctx.r[31].s64 + 4;
	// 82435DB8: 833D0000  lwz r25, 0(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82435DBC: 38DF0028  addi r6, r31, 0x28
	ctx.r[6].s64 = ctx.r[31].s64 + 40;
	// 82435DC0: 829D0010  lwz r20, 0x10(r29)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82435DC4: 38BF0134  addi r5, r31, 0x134
	ctx.r[5].s64 = ctx.r[31].s64 + 308;
	// 82435DC8: 821D0008  lwz r16, 8(r29)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82435DCC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82435DD0: 81FD000C  lwz r15, 0xc(r29)
	ctx.r[15].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82435DD4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82435DD8: 4BFFEF91  bl 0x82434d68
	ctx.lr = 0x82435DDC;
	sub_82434D68(ctx, base);
	// 82435DDC: 38BF002C  addi r5, r31, 0x2c
	ctx.r[5].s64 = ctx.r[31].s64 + 44;
	// 82435DE0: 389F0138  addi r4, r31, 0x138
	ctx.r[4].s64 = ctx.r[31].s64 + 312;
	// 82435DE4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82435DE8: 4BFFFB39  bl 0x82435920
	ctx.lr = 0x82435DEC;
	sub_82435920(ctx, base);
	// 82435DEC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82435DF0: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82435DF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82435DF8: 4BFFF151  bl 0x82434f48
	ctx.lr = 0x82435DFC;
	sub_82434F48(ctx, base);
	// 82435DFC: 4BFFF16D  bl 0x82434f68
	ctx.lr = 0x82435E00;
	sub_82434F68(ctx, base);
	// 82435E00: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82435E04: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82435E08: 7C6E1B78  mr r14, r3
	ctx.r[14].u64 = ctx.r[3].u64;
	// 82435E0C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435E10: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82435E14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82435E18: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82435E1C: 388B0080  addi r4, r11, 0x80
	ctx.r[4].s64 = ctx.r[11].s64 + 128;
	// 82435E20: 4BFFF9C1  bl 0x824357e0
	ctx.lr = 0x82435E24;
	sub_824357E0(ctx, base);
	// 82435E24: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82435E28: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 82435E2C: 388B0040  addi r4, r11, 0x40
	ctx.r[4].s64 = ctx.r[11].s64 + 64;
	// 82435E30: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435E34: 4BFFF9AD  bl 0x824357e0
	ctx.lr = 0x82435E38;
	sub_824357E0(ctx, base);
	// 82435E38: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82435E3C: 38A10068  addi r5, r1, 0x68
	ctx.r[5].s64 = ctx.r[1].s64 + 104;
	// 82435E40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82435E44: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435E48: 4BFFFCD9  bl 0x82435b20
	ctx.lr = 0x82435E4C;
	sub_82435B20(ctx, base);
	// 82435E4C: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82435E50: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 82435E54: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82435E58: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435E5C: 4BFFFDED  bl 0x82435c48
	ctx.lr = 0x82435E60;
	sub_82435C48(ctx, base);
	// 82435E60: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82435E64: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82435E68: 4BFFF2A9  bl 0x82435110
	ctx.lr = 0x82435E6C;
	sub_82435110(ctx, base);
	// 82435E6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82435E70: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82435E74: 409A0028  bne cr6, 0x82435e9c
	if !ctx.cr[6].eq {
	pc = 0x82435E9C; continue 'dispatch;
	}
	// 82435E78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435E7C: 809F013C  lwz r4, 0x13c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(316 as u32) ) } as u64;
	// 82435E80: 4BFFF961  bl 0x824357e0
	ctx.lr = 0x82435E84;
	sub_824357E0(ctx, base);
	// 82435E84: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82435E88: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435E8C: 809F0140  lwz r4, 0x140(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(320 as u32) ) } as u64;
	// 82435E90: 4BFFF951  bl 0x824357e0
	ctx.lr = 0x82435E94;
	sub_824357E0(ctx, base);
	// 82435E94: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82435E98: 4800000C  b 0x82435ea4
	pc = 0x82435EA4; continue 'dispatch;
            }
            0x82435E9C => {
    //   block [0x82435E9C..0x82435EA4)
	// 82435E9C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82435EA0: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	pc = 0x82435EA4; continue 'dispatch;
            }
            0x82435EA4 => {
    //   block [0x82435EA4..0x82435F44)
	// 82435EA4: 38800800  li r4, 0x800
	ctx.r[4].s64 = 2048;
	// 82435EA8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435EAC: 4BFFF935  bl 0x824357e0
	ctx.lr = 0x82435EB0;
	sub_824357E0(ctx, base);
	// 82435EB0: 82210054  lwz r17, 0x54(r1)
	ctx.r[17].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82435EB4: 90610058  stw r3, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[3].u32 ) };
	// 82435EB8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435EBC: 7E248B78  mr r4, r17
	ctx.r[4].u64 = ctx.r[17].u64;
	// 82435EC0: 4BFFF921  bl 0x824357e0
	ctx.lr = 0x82435EC4;
	sub_824357E0(ctx, base);
	// 82435EC4: 7C721B78  mr r18, r3
	ctx.r[18].u64 = ctx.r[3].u64;
	// 82435EC8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435ECC: 80810050  lwz r4, 0x50(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82435ED0: 4BFFF911  bl 0x824357e0
	ctx.lr = 0x82435ED4;
	sub_824357E0(ctx, base);
	// 82435ED4: 7C751B78  mr r21, r3
	ctx.r[21].u64 = ctx.r[3].u64;
	// 82435ED8: 7DC47378  mr r4, r14
	ctx.r[4].u64 = ctx.r[14].u64;
	// 82435EDC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82435EE0: 4BFFF901  bl 0x824357e0
	ctx.lr = 0x82435EE4;
	sub_824357E0(ctx, base);
	// 82435EE4: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82435EE8: 2B130000  cmplwi cr6, r19, 0
	ctx.cr[6].compare_u32(ctx.r[19].u32, 0 as u32, &mut ctx.xer);
	// 82435EEC: 419A0274  beq cr6, 0x82436160
	if ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435EF0: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 82435EF4: 419A026C  beq cr6, 0x82436160
	if ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435EF8: 2F170000  cmpwi cr6, r23, 0
	ctx.cr[6].compare_i32(ctx.r[23].s32, 0, &mut ctx.xer);
	// 82435EFC: 409A0264  bne cr6, 0x82436160
	if !ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435F00: 2F160000  cmpwi cr6, r22, 0
	ctx.cr[6].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82435F04: 409A025C  bne cr6, 0x82436160
	if !ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435F08: 2B120000  cmplwi cr6, r18, 0
	ctx.cr[6].compare_u32(ctx.r[18].u32, 0 as u32, &mut ctx.xer);
	// 82435F0C: 419A0254  beq cr6, 0x82436160
	if ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435F10: 82C10058  lwz r22, 0x58(r1)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82435F14: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82435F18: 419A0248  beq cr6, 0x82436160
	if ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435F1C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82435F20: 419A0240  beq cr6, 0x82436160
	if ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435F24: 2B150000  cmplwi cr6, r21, 0
	ctx.cr[6].compare_u32(ctx.r[21].u32, 0 as u32, &mut ctx.xer);
	// 82435F28: 419A0238  beq cr6, 0x82436160
	if ctx.cr[6].eq {
	pc = 0x82436160; continue 'dispatch;
	}
	// 82435F2C: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82435F30: 409A0020  bne cr6, 0x82435f50
	if !ctx.cr[6].eq {
	pc = 0x82435F50; continue 'dispatch;
	}
	// 82435F34: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 82435F38: 419A000C  beq cr6, 0x82435f44
	if ctx.cr[6].eq {
	pc = 0x82435F44; continue 'dispatch;
	}
	// 82435F3C: 2B180000  cmplwi cr6, r24, 0
	ctx.cr[6].compare_u32(ctx.r[24].u32, 0 as u32, &mut ctx.xer);
	// 82435F40: 409A0010  bne cr6, 0x82435f50
	if !ctx.cr[6].eq {
	pc = 0x82435F50; continue 'dispatch;
	}
	pc = 0x82435F44; continue 'dispatch;
            }
            0x82435F44 => {
    //   block [0x82435F44..0x82435F50)
	// 82435F44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82435F48: 386B53F8  addi r3, r11, 0x53f8
	ctx.r[3].s64 = ctx.r[11].s64 + 21496;
	// 82435F4C: 4800021C  b 0x82436168
	pc = 0x82436168; continue 'dispatch;
            }
            0x82435F50 => {
    //   block [0x82435F50..0x82435FE8)
	// 82435F50: 397B003F  addi r11, r27, 0x3f
	ctx.r[11].s64 = ctx.r[27].s64 + 63;
	// 82435F54: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82435F58: 557B0032  rlwinm r27, r11, 0, 0, 0x19
	ctx.r[27].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82435F5C: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82435F60: 38A1005C  addi r5, r1, 0x5c
	ctx.r[5].s64 = ctx.r[1].s64 + 92;
	// 82435F64: 38810054  addi r4, r1, 0x54
	ctx.r[4].s64 = ctx.r[1].s64 + 84;
	// 82435F68: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82435F6C: 937F0130  stw r27, 0x130(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(304 as u32), ctx.r[27].u32 ) };
	// 82435F70: 4BFFEF11  bl 0x82434e80
	ctx.lr = 0x82435F74;
	sub_82434E80(ctx, base);
	// 82435F74: 3D608273  lis r11, -0x7d8d
	ctx.r[11].s64 = -2106392576;
	// 82435F78: 3AE00000  li r23, 0
	ctx.r[23].s64 = 0;
	// 82435F7C: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82435F80: 3BCB4130  addi r30, r11, 0x4130
	ctx.r[30].s64 = ctx.r[11].s64 + 16688;
	// 82435F84: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82435F88: 7EE9BB78  mr r9, r23
	ctx.r[9].u64 = ctx.r[23].u64;
	// 82435F8C: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 82435F90: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82435F94: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82435F98: 935E00B4  stw r26, 0xb4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(180 as u32), ctx.r[26].u32 ) };
	// 82435F9C: 835E010C  lwz r26, 0x10c(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(268 as u32) ) } as u64;
	// 82435FA0: 931E00C4  stw r24, 0xc4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(196 as u32), ctx.r[24].u32 ) };
	// 82435FA4: 917E0048  stw r11, 0x48(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82435FA8: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82435FAC: 917E004C  stw r11, 0x4c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(76 as u32), ctx.r[11].u32 ) };
	// 82435FB0: 915E0050  stw r10, 0x50(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82435FB4: 913E0054  stw r9, 0x54(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82435FB8: 917E0058  stw r11, 0x58(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82435FBC: 7EEBBB78  mr r11, r23
	ctx.r[11].u64 = ctx.r[23].u64;
	// 82435FC0: 915E005C  stw r10, 0x5c(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82435FC4: 929E0060  stw r20, 0x60(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(96 as u32), ctx.r[20].u32 ) };
	// 82435FC8: 917E0064  stw r11, 0x64(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82435FCC: 419A0044  beq cr6, 0x82436010
	if ctx.cr[6].eq {
	pc = 0x82436010; continue 'dispatch;
	}
	// 82435FD0: 2F190002  cmpwi cr6, r25, 2
	ctx.cr[6].compare_i32(ctx.r[25].s32, 2, &mut ctx.xer);
	// 82435FD4: 419A0014  beq cr6, 0x82435fe8
	if ctx.cr[6].eq {
	pc = 0x82435FE8; continue 'dispatch;
	}
	// 82435FD8: 2F190003  cmpwi cr6, r25, 3
	ctx.cr[6].compare_i32(ctx.r[25].s32, 3, &mut ctx.xer);
	// 82435FDC: 409A0058  bne cr6, 0x82436034
	if !ctx.cr[6].eq {
	pc = 0x82436034; continue 'dispatch;
	}
	// 82435FE0: 389E00C8  addi r4, r30, 0xc8
	ctx.r[4].s64 = ctx.r[30].s64 + 200;
	// 82435FE4: 48000030  b 0x82436014
	pc = 0x82436014; continue 'dispatch;
            }
            0x82435FE8 => {
    //   block [0x82435FE8..0x82436010)
	// 82435FE8: 389E0068  addi r4, r30, 0x68
	ctx.r[4].s64 = ctx.r[30].s64 + 104;
	// 82435FEC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82435FF0: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 82435FF4: 480FEB5D  bl 0x82534b50
	ctx.lr = 0x82435FF8;
	sub_82534B50(ctx, base);
	// 82435FF8: 39400800  li r10, 0x800
	ctx.r[10].s64 = 2048;
	// 82435FFC: 937C0458  stw r27, 0x458(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1112 as u32), ctx.r[27].u32 ) };
	// 82436000: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82436004: 396BF800  addi r11, r11, -0x800
	ctx.r[11].s64 = ctx.r[11].s64 + -2048;
	// 82436008: 915C0460  stw r10, 0x460(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1120 as u32), ctx.r[10].u32 ) };
	// 8243600C: 48000024  b 0x82436030
	pc = 0x82436030; continue 'dispatch;
            }
            0x82436010 => {
    //   block [0x82436010..0x82436014)
	// 82436010: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	pc = 0x82436014; continue 'dispatch;
            }
            0x82436014 => {
    //   block [0x82436014..0x82436030)
	// 82436014: 38A00044  li r5, 0x44
	ctx.r[5].s64 = 68;
	// 82436018: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8243601C: 480FEB35  bl 0x82534b50
	ctx.lr = 0x82436020;
	sub_82534B50(ctx, base);
	// 82436020: 937C0458  stw r27, 0x458(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1112 as u32), ctx.r[27].u32 ) };
	// 82436024: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82436028: 935C0460  stw r26, 0x460(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1120 as u32), ctx.r[26].u32 ) };
	// 8243602C: 7D7A5850  subf r11, r26, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[26].s64;
	pc = 0x82436030; continue 'dispatch;
            }
            0x82436030 => {
    //   block [0x82436030..0x82436034)
	// 82436030: 917C045C  stw r11, 0x45c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1116 as u32), ctx.r[11].u32 ) };
	pc = 0x82436034; continue 'dispatch;
            }
            0x82436034 => {
    //   block [0x82436034..0x82436058)
	// 82436034: 817F0134  lwz r11, 0x134(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82436038: 93410098  stw r26, 0x98(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(152 as u32), ctx.r[26].u32 ) };
	// 8243603C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82436040: 419A0018  beq cr6, 0x82436058
	if ctx.cr[6].eq {
	pc = 0x82436058; continue 'dispatch;
	}
	// 82436044: 7D4BD3D6  divw r10, r11, r26
	ctx.r[10].s32 = ctx.r[11].s32 / ctx.r[26].s32;
	// 82436048: 7D4AD1D6  mullw r10, r10, r26
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * ctx.r[26].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 8243604C: 7D4A5850  subf r10, r10, r11
	ctx.r[10].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82436050: 7D6A5850  subf r11, r10, r11
	ctx.r[11].s64 = ctx.r[11].s64 - ctx.r[10].s64;
	// 82436054: 917F0134  stw r11, 0x134(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	pc = 0x82436058; continue 'dispatch;
            }
            0x82436058 => {
    //   block [0x82436058..0x824360B8)
	// 82436058: 807D0024  lwz r3, 0x24(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243605C: 4BFFEE65  bl 0x82434ec0
	ctx.lr = 0x82436060;
	sub_82434EC0(ctx, base);
	// 82436060: 815F0134  lwz r10, 0x134(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(308 as u32) ) } as u64;
	// 82436064: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82436068: 387E0044  addi r3, r30, 0x44
	ctx.r[3].s64 = ctx.r[30].s64 + 68;
	// 8243606C: 92610074  stw r19, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[19].u32 ) };
	// 82436070: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 82436074: 9281009C  stw r20, 0x9c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(156 as u32), ctx.r[20].u32 ) };
	// 82436078: 38810068  addi r4, r1, 0x68
	ctx.r[4].s64 = ctx.r[1].s64 + 104;
	// 8243607C: 920100A0  stw r16, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[16].u32 ) };
	// 82436080: 91E100A4  stw r15, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[15].u32 ) };
	// 82436084: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82436088: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 8243608C: 924100AC  stw r18, 0xac(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(172 as u32), ctx.r[18].u32 ) };
	// 82436090: 922100B0  stw r17, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[17].u32 ) };
	// 82436094: 916100A8  stw r11, 0xa8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(168 as u32), ctx.r[11].u32 ) };
	// 82436098: 9141007C  stw r10, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[10].u32 ) };
	// 8243609C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824360A0: 91410080  stw r10, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 824360A4: 4800E55D  bl 0x82444600
	ctx.lr = 0x824360A8;
	sub_82444600(ctx, base);
	// 824360A8: 2F190001  cmpwi cr6, r25, 1
	ctx.cr[6].compare_i32(ctx.r[25].s32, 1, &mut ctx.xer);
	// 824360AC: 409A000C  bne cr6, 0x824360b8
	if !ctx.cr[6].eq {
	pc = 0x824360B8; continue 'dispatch;
	}
	// 824360B0: 387E00AC  addi r3, r30, 0xac
	ctx.r[3].s64 = ctx.r[30].s64 + 172;
	// 824360B4: 48010125  bl 0x824461d8
	ctx.lr = 0x824360B8;
	sub_824461D8(ctx, base);
	pc = 0x824360B8; continue 'dispatch;
            }
            0x824360B8 => {
    //   block [0x824360B8..0x824360F0)
	// 824360B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824360BC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824360C0: 48004EE9  bl 0x8243afa8
	ctx.lr = 0x824360C4;
	sub_8243AFA8(ctx, base);
	// 824360C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824360C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824360CC: 409A0024  bne cr6, 0x824360f0
	if !ctx.cr[6].eq {
	pc = 0x824360F0; continue 'dispatch;
	}
	// 824360D0: 3860FECF  li r3, -0x131
	ctx.r[3].s64 = -305;
	// 824360D4: 480008F5  bl 0x824369c8
	ctx.lr = 0x824360D8;
	sub_824369C8(ctx, base);
	// 824360D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824360DC: 386B53CC  addi r3, r11, 0x53cc
	ctx.r[3].s64 = ctx.r[11].s64 + 21452;
	// 824360E0: 48000FE9  bl 0x824370c8
	ctx.lr = 0x824360E4;
	sub_824370C8(ctx, base);
	// 824360E4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824360E8: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 824360EC: 480FEFE4  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
            }
            0x824360F0 => {
    //   block [0x824360F0..0x8243612C)
	// 824360F0: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 824360F4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824360F8: 388B69F8  addi r4, r11, 0x69f8
	ctx.r[4].s64 = ctx.r[11].s64 + 27128;
	// 824360FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436100: 48011891  bl 0x82447990
	ctx.lr = 0x82436104;
	sub_82447990(ctx, base);
	// 82436104: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82436108: 419A0024  beq cr6, 0x8243612c
	if ctx.cr[6].eq {
	pc = 0x8243612C; continue 'dispatch;
	}
	// 8243610C: 3860FED1  li r3, -0x12f
	ctx.r[3].s64 = -303;
	// 82436110: 480008B9  bl 0x824369c8
	ctx.lr = 0x82436114;
	sub_824369C8(ctx, base);
	// 82436114: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436118: 386B53A4  addi r3, r11, 0x53a4
	ctx.r[3].s64 = ctx.r[11].s64 + 21412;
	// 8243611C: 48000FAD  bl 0x824370c8
	ctx.lr = 0x82436120;
	sub_824370C8(ctx, base);
	// 82436120: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436124: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 82436128: 480FEFA8  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
            }
            0x8243612C => {
    //   block [0x8243612C..0x82436160)
	// 8243612C: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82436130: 38C00040  li r6, 0x40
	ctx.r[6].s64 = 64;
	// 82436134: 38A00800  li r5, 0x800
	ctx.r[5].s64 = 2048;
	// 82436138: 91DC043C  stw r14, 0x43c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1084 as u32), ctx.r[14].u32 ) };
	// 8243613C: 7EC4B378  mr r4, r22
	ctx.r[4].u64 = ctx.r[22].u64;
	// 82436140: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82436144: 917C0438  stw r11, 0x438(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1080 as u32), ctx.r[11].u32 ) };
	// 82436148: 4BFFF019  bl 0x82435160
	ctx.lr = 0x8243614C;
	sub_82435160(ctx, base);
	// 8243614C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436150: 92FC056C  stw r23, 0x56c(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1388 as u32), ctx.r[23].u32 ) };
	// 82436154: 92FC05AC  stw r23, 0x5ac(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(1452 as u32), ctx.r[23].u32 ) };
	// 82436158: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 8243615C: 480FEF74  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
            }
            0x82436160 => {
    //   block [0x82436160..0x82436168)
	// 82436160: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436164: 386B5388  addi r3, r11, 0x5388
	ctx.r[3].s64 = ctx.r[11].s64 + 21384;
	pc = 0x82436168; continue 'dispatch;
            }
            0x82436168 => {
    //   block [0x82436168..0x82436180)
	// 82436168: 48000F61  bl 0x824370c8
	ctx.lr = 0x8243616C;
	sub_824370C8(ctx, base);
	// 8243616C: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82436170: 4BFFF761  bl 0x824358d0
	ctx.lr = 0x82436174;
	sub_824358D0(ctx, base);
	// 82436174: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436178: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 8243617C: 480FEF54  b 0x825350d0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436180(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436180 size=216
    let mut pc: u32 = 0x82436180;
    'dispatch: loop {
        match pc {
            0x82436180 => {
    //   block [0x82436180..0x824361DC)
	// 82436180: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436184: 480FEF39  bl 0x825350bc
	ctx.lr = 0x82436188;
	sub_82535080(ctx, base);
	// 82436188: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243618C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82436190: 3BDF0008  addi r30, r31, 8
	ctx.r[30].s64 = ctx.r[31].s64 + 8;
	// 82436194: 809E000C  lwz r4, 0xc(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82436198: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243619C: 4BFFBA35  bl 0x82431bd0
	ctx.lr = 0x824361A0;
	sub_82431BD0(ctx, base);
	// 824361A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824361A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824361A8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824361AC: 4BFFF635  bl 0x824357e0
	ctx.lr = 0x824361B0;
	sub_824357E0(ctx, base);
	// 824361B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824361B4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824361B8: 409A0024  bne cr6, 0x824361dc
	if !ctx.cr[6].eq {
	pc = 0x824361DC; continue 'dispatch;
	}
	// 824361BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824361C0: 386B543C  addi r3, r11, 0x543c
	ctx.r[3].s64 = ctx.r[11].s64 + 21564;
	// 824361C4: 48000F05  bl 0x824370c8
	ctx.lr = 0x824361C8;
	sub_824370C8(ctx, base);
	// 824361C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824361CC: 4BFFF705  bl 0x824358d0
	ctx.lr = 0x824361D0;
	sub_824358D0(ctx, base);
	// 824361D0: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 824361D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824361D8: 480FEF34  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x824361DC => {
    //   block [0x824361DC..0x82436228)
	// 824361DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824361E0: 917F00BC  stw r11, 0xbc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(188 as u32), ctx.r[11].u32 ) };
	// 824361E4: 93BF00C0  stw r29, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[29].u32 ) };
	// 824361E8: 4BFFBDC9  bl 0x82431fb0
	ctx.lr = 0x824361EC;
	sub_82431FB0(ctx, base);
	// 824361EC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824361F0: 409A0050  bne cr6, 0x82436240
	if !ctx.cr[6].eq {
	pc = 0x82436240; continue 'dispatch;
	}
	// 824361F4: 3C800002  lis r4, 2
	ctx.r[4].s64 = 131072;
	// 824361F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824361FC: 4BFFF5E5  bl 0x824357e0
	ctx.lr = 0x82436200;
	sub_824357E0(ctx, base);
	// 82436200: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82436204: 409A0024  bne cr6, 0x82436228
	if !ctx.cr[6].eq {
	pc = 0x82436228; continue 'dispatch;
	}
	// 82436208: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243620C: 386B5414  addi r3, r11, 0x5414
	ctx.r[3].s64 = ctx.r[11].s64 + 21524;
	// 82436210: 48000EB9  bl 0x824370c8
	ctx.lr = 0x82436214;
	sub_824370C8(ctx, base);
	// 82436214: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436218: 4BFFF6B9  bl 0x824358d0
	ctx.lr = 0x8243621C;
	sub_824358D0(ctx, base);
	// 8243621C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82436220: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82436224: 480FEEE8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82436228 => {
    //   block [0x82436228..0x82436240)
	// 82436228: 3D600002  lis r11, 2
	ctx.r[11].s64 = 131072;
	// 8243622C: 907F0414  stw r3, 0x414(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1044 as u32), ctx.r[3].u32 ) };
	// 82436230: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436234: 917F0418  stw r11, 0x418(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1048 as u32), ctx.r[11].u32 ) };
	// 82436238: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243623C: 480FEED0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82436240 => {
    //   block [0x82436240..0x82436258)
	// 82436240: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82436244: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436248: 917F0414  stw r11, 0x414(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1044 as u32), ctx.r[11].u32 ) };
	// 8243624C: 917F0418  stw r11, 0x418(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1048 as u32), ctx.r[11].u32 ) };
	// 82436250: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82436254: 480FEEB8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436258 size=332
    let mut pc: u32 = 0x82436258;
    'dispatch: loop {
        match pc {
            0x82436258 => {
    //   block [0x82436258..0x824362B4)
	// 82436258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243625C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436260: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82436264: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82436268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243626C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82436270: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82436274: 419A0118  beq cr6, 0x8243638c
	if ctx.cr[6].eq {
	pc = 0x8243638C; continue 'dispatch;
	}
	// 82436278: 4BFFF459  bl 0x824356d0
	ctx.lr = 0x8243627C;
	sub_824356D0(ctx, base);
	// 8243627C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82436280: 48005D39  bl 0x8243bfb8
	ctx.lr = 0x82436284;
	sub_8243BFB8(ctx, base);
	// 82436284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436288: 4BFFC501  bl 0x82432788
	ctx.lr = 0x8243628C;
	sub_82432788(ctx, base);
	// 8243628C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82436290: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436294: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82436298: 48005D21  bl 0x8243bfb8
	ctx.lr = 0x8243629C;
	sub_8243BFB8(ctx, base);
	// 8243629C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824362A0: 4BFFBD99  bl 0x82432038
	ctx.lr = 0x824362A4;
	sub_82432038(ctx, base);
	// 824362A4: 807F00B8  lwz r3, 0xb8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(184 as u32) ) } as u64;
	// 824362A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824362AC: 419A0008  beq cr6, 0x824362b4
	if ctx.cr[6].eq {
	pc = 0x824362B4; continue 'dispatch;
	}
	// 824362B0: 4BFFB931  bl 0x82431be0
	ctx.lr = 0x824362B4;
	sub_82431BE0(ctx, base);
	pc = 0x824362B4; continue 'dispatch;
            }
            0x824362B4 => {
    //   block [0x824362B4..0x824362C4)
	// 824362B4: 807F0054  lwz r3, 0x54(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) } as u64;
	// 824362B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824362BC: 419A0008  beq cr6, 0x824362c4
	if ctx.cr[6].eq {
	pc = 0x824362C4; continue 'dispatch;
	}
	// 824362C0: 4BFF1819  bl 0x82427ad8
	ctx.lr = 0x824362C4;
	sub_82427AD8(ctx, base);
	pc = 0x824362C4; continue 'dispatch;
            }
            0x824362C4 => {
    //   block [0x824362C4..0x824362D4)
	// 824362C4: 807F004C  lwz r3, 0x4c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 824362C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824362CC: 419A0008  beq cr6, 0x824362d4
	if ctx.cr[6].eq {
	pc = 0x824362D4; continue 'dispatch;
	}
	// 824362D0: 480050A9  bl 0x8243b378
	ctx.lr = 0x824362D4;
	sub_8243B378(ctx, base);
	pc = 0x824362D4; continue 'dispatch;
            }
            0x824362D4 => {
    //   block [0x824362D4..0x824362F0)
	// 824362D4: 807F0454  lwz r3, 0x454(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1108 as u32) ) } as u64;
	// 824362D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824362DC: 419A0014  beq cr6, 0x824362f0
	if ctx.cr[6].eq {
	pc = 0x824362F0; continue 'dispatch;
	}
	// 824362E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824362E4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 824362E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824362EC: 4E800421  bctrl
	ctx.lr = 0x824362F0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824362F0 => {
    //   block [0x824362F0..0x8243630C)
	// 824362F0: 807F0474  lwz r3, 0x474(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1140 as u32) ) } as u64;
	// 824362F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824362F8: 419A0014  beq cr6, 0x8243630c
	if ctx.cr[6].eq {
	pc = 0x8243630C; continue 'dispatch;
	}
	// 824362FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82436300: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82436304: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82436308: 4E800421  bctrl
	ctx.lr = 0x8243630C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8243630C => {
    //   block [0x8243630C..0x8243631C)
	// 8243630C: 807F0048  lwz r3, 0x48(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(72 as u32) ) } as u64;
	// 82436310: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82436314: 419A0008  beq cr6, 0x8243631c
	if ctx.cr[6].eq {
	pc = 0x8243631C; continue 'dispatch;
	}
	// 82436318: 4BFFEEF1  bl 0x82435208
	ctx.lr = 0x8243631C;
	sub_82435208(ctx, base);
	pc = 0x8243631C; continue 'dispatch;
            }
            0x8243631C => {
    //   block [0x8243631C..0x82436350)
	// 8243631C: 387F0518  addi r3, r31, 0x518
	ctx.r[3].s64 = ctx.r[31].s64 + 1304;
	// 82436320: 48005509  bl 0x8243b828
	ctx.lr = 0x82436324;
	sub_8243B828(ctx, base);
	// 82436324: 387F053C  addi r3, r31, 0x53c
	ctx.r[3].s64 = ctx.r[31].s64 + 1340;
	// 82436328: 48005501  bl 0x8243b828
	ctx.lr = 0x8243632C;
	sub_8243B828(ctx, base);
	// 8243632C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436330: 4BFFF5A1  bl 0x824358d0
	ctx.lr = 0x82436334;
	sub_824358D0(ctx, base);
	// 82436334: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436338: 4BFFF301  bl 0x82435638
	ctx.lr = 0x8243633C;
	sub_82435638(ctx, base);
	// 8243633C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82436340: 419A0010  beq cr6, 0x82436350
	if ctx.cr[6].eq {
	pc = 0x82436350; continue 'dispatch;
	}
	// 82436344: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436348: 386B5460  addi r3, r11, 0x5460
	ctx.r[3].s64 = ctx.r[11].s64 + 21600;
	// 8243634C: 48000D7D  bl 0x824370c8
	ctx.lr = 0x82436350;
	sub_824370C8(ctx, base);
	pc = 0x82436350; continue 'dispatch;
            }
            0x82436350 => {
    //   block [0x82436350..0x8243638C)
	// 82436350: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436354: 93DF0000  stw r30, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 82436358: 4BFFF399  bl 0x824356f0
	ctx.lr = 0x8243635C;
	sub_824356F0(ctx, base);
	// 8243635C: 807F05B0  lwz r3, 0x5b0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82436360: 4BFF6C69  bl 0x8242cfc8
	ctx.lr = 0x82436364;
	sub_8242CFC8(ctx, base);
	// 82436364: 83DF05B0  lwz r30, 0x5b0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1456 as u32) ) } as u64;
	// 82436368: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243636C: 41980020  blt cr6, 0x8243638c
	if ctx.cr[6].lt {
	pc = 0x8243638C; continue 'dispatch;
	}
	// 82436370: 38A005B4  li r5, 0x5b4
	ctx.r[5].s64 = 1460;
	// 82436374: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82436378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243637C: 480FEE55  bl 0x825351d0
	ctx.lr = 0x82436380;
	sub_825351D0(ctx, base);
	// 82436380: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82436384: 93DF05B0  stw r30, 0x5b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1456 as u32), ctx.r[30].u32 ) };
	// 82436388: 4BFF6CD9  bl 0x8242d060
	ctx.lr = 0x8243638C;
	sub_8242D060(ctx, base);
	pc = 0x8243638C; continue 'dispatch;
            }
            0x8243638C => {
    //   block [0x8243638C..0x824363A4)
	// 8243638C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82436390: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436394: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436398: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243639C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824363A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824363A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824363A8 size=4
    let mut pc: u32 = 0x824363A8;
    'dispatch: loop {
        match pc {
            0x824363A8 => {
    //   block [0x824363A8..0x824363AC)
	// 824363A8: 4BFFFEB0  b 0x82436258
	sub_82436258(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824363B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824363B0 size=984
    let mut pc: u32 = 0x824363B0;
    'dispatch: loop {
        match pc {
            0x824363B0 => {
    //   block [0x824363B0..0x824363E0)
	// 824363B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824363B4: 480FECF5  bl 0x825350a8
	ctx.lr = 0x824363B8;
	sub_82535080(ctx, base);
	// 824363B8: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824363BC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 824363C0: 2B1B0000  cmplwi cr6, r27, 0
	ctx.cr[6].compare_u32(ctx.r[27].u32, 0 as u32, &mut ctx.xer);
	// 824363C4: 409A001C  bne cr6, 0x824363e0
	if !ctx.cr[6].eq {
	pc = 0x824363E0; continue 'dispatch;
	}
	// 824363C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824363CC: 386B55B8  addi r3, r11, 0x55b8
	ctx.r[3].s64 = ctx.r[11].s64 + 21944;
	// 824363D0: 48000CF9  bl 0x824370c8
	ctx.lr = 0x824363D4;
	sub_824370C8(ctx, base);
	// 824363D4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824363D8: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 824363DC: 480FED1C  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            0x824363E0 => {
    //   block [0x824363E0..0x82436404)
	// 824363E0: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824363E4: 4BFFF0FD  bl 0x824354e0
	ctx.lr = 0x824363E8;
	sub_824354E0(ctx, base);
	// 824363E8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824363EC: 409A0390  bne cr6, 0x8243677c
	if !ctx.cr[6].eq {
	pc = 0x8243677C; continue 'dispatch;
	}
	// 824363F0: 48000399  bl 0x82436788
	ctx.lr = 0x824363F4;
	sub_82436788(ctx, base);
	// 824363F4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824363F8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 824363FC: 397C006C  addi r11, r28, 0x6c
	ctx.r[11].s64 = ctx.r[28].s64 + 108;
	// 82436400: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	pc = 0x82436404; continue 'dispatch;
            }
            0x82436404 => {
    //   block [0x82436404..0x82436424)
	// 82436404: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82436408: 7D7F5B78  mr r31, r11
	ctx.r[31].u64 = ctx.r[11].u64;
	// 8243640C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82436410: 409A0014  bne cr6, 0x82436424
	if !ctx.cr[6].eq {
	pc = 0x82436424; continue 'dispatch;
	}
	// 82436414: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82436418: 396B05B4  addi r11, r11, 0x5b4
	ctx.r[11].s64 = ctx.r[11].s64 + 1460;
	// 8243641C: 2F1D0008  cmpwi cr6, r29, 8
	ctx.cr[6].compare_i32(ctx.r[29].s32, 8, &mut ctx.xer);
	// 82436420: 4198FFE4  blt cr6, 0x82436404
	if ctx.cr[6].lt {
	pc = 0x82436404; continue 'dispatch;
	}
	pc = 0x82436424; continue 'dispatch;
            }
            0x82436424 => {
    //   block [0x82436424..0x8243644C)
	// 82436424: 2F1D0008  cmpwi cr6, r29, 8
	ctx.cr[6].compare_i32(ctx.r[29].s32, 8, &mut ctx.xer);
	// 82436428: 409A0024  bne cr6, 0x8243644c
	if !ctx.cr[6].eq {
	pc = 0x8243644C; continue 'dispatch;
	}
	// 8243642C: 3860FFF5  li r3, -0xb
	ctx.r[3].s64 = -11;
	// 82436430: 48000599  bl 0x824369c8
	ctx.lr = 0x82436434;
	sub_824369C8(ctx, base);
	// 82436434: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436438: 386B5568  addi r3, r11, 0x5568
	ctx.r[3].s64 = ctx.r[11].s64 + 21864;
	// 8243643C: 48000C8D  bl 0x824370c8
	ctx.lr = 0x82436440;
	sub_824370C8(ctx, base);
	// 82436440: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436444: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82436448: 480FECB0  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            0x8243644C => {
    //   block [0x8243644C..0x82436474)
	// 8243644C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82436450: 4BFFEFF1  bl 0x82435440
	ctx.lr = 0x82436454;
	sub_82435440(ctx, base);
	// 82436454: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82436458: 409A001C  bne cr6, 0x82436474
	if !ctx.cr[6].eq {
	pc = 0x82436474; continue 'dispatch;
	}
	// 8243645C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436460: 386B5528  addi r3, r11, 0x5528
	ctx.r[3].s64 = ctx.r[11].s64 + 21800;
	// 82436464: 48000C65  bl 0x824370c8
	ctx.lr = 0x82436468;
	sub_824370C8(ctx, base);
	// 82436468: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243646C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82436470: 480FEC88  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            0x82436474 => {
    //   block [0x82436474..0x824364D8)
	// 82436474: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82436478: 57B8103A  slwi r24, r29, 2
	ctx.r[24].u32 = ctx.r[29].u32.wrapping_shl(2);
	ctx.r[24].u64 = ctx.r[24].u32 as u64;
	// 8243647C: 3B2B9B50  addi r25, r11, -0x64b0
	ctx.r[25].s64 = ctx.r[11].s64 + -25776;
	// 82436480: 7C78C82E  lwzx r3, r24, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82436484: 4BFF6B45  bl 0x8242cfc8
	ctx.lr = 0x82436488;
	sub_8242CFC8(ctx, base);
	// 82436488: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243648C: 419802F0  blt cr6, 0x8243677c
	if ctx.cr[6].lt {
	pc = 0x8243677C; continue 'dispatch;
	}
	// 82436490: 38A005B4  li r5, 0x5b4
	ctx.r[5].s64 = 1460;
	// 82436494: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82436498: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243649C: 480FED35  bl 0x825351d0
	ctx.lr = 0x824364A0;
	sub_825351D0(ctx, base);
	// 824364A0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 824364A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824364A8: 4BFFEFF9  bl 0x824354a0
	ctx.lr = 0x824364AC;
	sub_824354A0(ctx, base);
	// 824364AC: 3BBF0008  addi r29, r31, 8
	ctx.r[29].s64 = ctx.r[31].s64 + 8;
	// 824364B0: 38A0003C  li r5, 0x3c
	ctx.r[5].s64 = 60;
	// 824364B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824364B8: 480FE699  bl 0x82534b50
	ctx.lr = 0x824364BC;
	sub_82534B50(ctx, base);
	// 824364BC: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 824364C0: 4BFFF251  bl 0x82435710
	ctx.lr = 0x824364C4;
	sub_82435710(ctx, base);
	// 824364C4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824364C8: 409A0010  bne cr6, 0x824364d8
	if !ctx.cr[6].eq {
	pc = 0x824364D8; continue 'dispatch;
	}
	// 824364CC: 817F0030  lwz r11, 0x30(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) } as u64;
	// 824364D0: 396BFFFE  addi r11, r11, -2
	ctx.r[11].s64 = ctx.r[11].s64 + -2;
	// 824364D4: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	pc = 0x824364D8; continue 'dispatch;
            }
            0x824364D8 => {
    //   block [0x824364D8..0x82436500)
	// 824364D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824364DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824364E0: 4BFFF8B1  bl 0x82435d90
	ctx.lr = 0x824364E4;
	sub_82435D90(ctx, base);
	// 824364E4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 824364E8: 2B1A0000  cmplwi cr6, r26, 0
	ctx.cr[6].compare_u32(ctx.r[26].u32, 0 as u32, &mut ctx.xer);
	// 824364EC: 935F0048  stw r26, 0x48(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(72 as u32), ctx.r[26].u32 ) };
	// 824364F0: 409A0024  bne cr6, 0x82436514
	if !ctx.cr[6].eq {
	pc = 0x82436514; continue 'dispatch;
	}
	// 824364F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824364F8: 386B5504  addi r3, r11, 0x5504
	ctx.r[3].s64 = ctx.r[11].s64 + 21764;
	// 824364FC: 48000BCD  bl 0x824370c8
	ctx.lr = 0x82436500;
	sub_824370C8(ctx, base);
	pc = 0x82436500; continue 'dispatch;
            }
            0x82436500 => {
    //   block [0x82436500..0x82436514)
	// 82436500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436504: 4BFFFEA5  bl 0x824363a8
	ctx.lr = 0x82436508;
	sub_824363A8(ctx, base);
	// 82436508: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243650C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82436510: 480FEBE8  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            0x82436514 => {
    //   block [0x82436514..0x82436584)
	// 82436514: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436518: 4BFFEC61  bl 0x82435178
	ctx.lr = 0x8243651C;
	sub_82435178(ctx, base);
	// 8243651C: 39210054  addi r9, r1, 0x54
	ctx.r[9].s64 = ctx.r[1].s64 + 84;
	// 82436520: 39010058  addi r8, r1, 0x58
	ctx.r[8].s64 = ctx.r[1].s64 + 88;
	// 82436524: 38E1005C  addi r7, r1, 0x5c
	ctx.r[7].s64 = ctx.r[1].s64 + 92;
	// 82436528: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 8243652C: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 82436530: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82436534: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82436538: 4BFFE831  bl 0x82434d68
	ctx.lr = 0x8243653C;
	sub_82434D68(ctx, base);
	// 8243653C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82436540: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436544: 80DC000C  lwz r6, 0xc(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(12 as u32) ) } as u64;
	// 82436548: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243654C: 4BFFECFD  bl 0x82435248
	ctx.lr = 0x82436550;
	sub_82435248(ctx, base);
	// 82436550: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436554: 4BFFEEDD  bl 0x82435430
	ctx.lr = 0x82436558;
	sub_82435430(ctx, base);
	// 82436558: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243655C: 907F0454  stw r3, 0x454(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1108 as u32), ctx.r[3].u32 ) };
	// 82436560: 409A0024  bne cr6, 0x82436584
	if !ctx.cr[6].eq {
	pc = 0x82436584; continue 'dispatch;
	}
	// 82436564: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436568: 386B54E0  addi r3, r11, 0x54e0
	ctx.r[3].s64 = ctx.r[11].s64 + 21728;
	// 8243656C: 48000B5D  bl 0x824370c8
	ctx.lr = 0x82436570;
	sub_824370C8(ctx, base);
	// 82436570: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436574: 4BFFFE35  bl 0x824363a8
	ctx.lr = 0x82436578;
	sub_824363A8(ctx, base);
	// 82436578: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243657C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82436580: 480FEB78  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            0x82436584 => {
    //   block [0x82436584..0x824365BC)
	// 82436584: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82436588: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243658C: 4BFEF0AD  bl 0x82425638
	ctx.lr = 0x82436590;
	sub_82425638(ctx, base);
	// 82436590: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82436594: 907F0474  stw r3, 0x474(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1140 as u32), ctx.r[3].u32 ) };
	// 82436598: 409A0024  bne cr6, 0x824365bc
	if !ctx.cr[6].eq {
	pc = 0x824365BC; continue 'dispatch;
	}
	// 8243659C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824365A0: 386B54BC  addi r3, r11, 0x54bc
	ctx.r[3].s64 = ctx.r[11].s64 + 21692;
	// 824365A4: 48000B25  bl 0x824370c8
	ctx.lr = 0x824365A8;
	sub_824370C8(ctx, base);
	// 824365A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824365AC: 4BFFFDFD  bl 0x824363a8
	ctx.lr = 0x824365B0;
	sub_824363A8(ctx, base);
	// 824365B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824365B4: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 824365B8: 480FEB40  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            0x824365BC => {
    //   block [0x824365BC..0x82436604)
	// 824365BC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824365C0: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 824365C4: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 824365C8: 3BBF0050  addi r29, r31, 0x50
	ctx.r[29].s64 = ctx.r[31].s64 + 80;
	// 824365CC: 93DF0004  stw r30, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 824365D0: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 824365D4: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 824365D8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824365DC: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 824365E0: 939F0044  stw r28, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[28].u32 ) };
	// 824365E4: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 824365E8: 917F0058  stw r11, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 824365EC: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 824365F0: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 824365F4: 4800624D  bl 0x8243c840
	ctx.lr = 0x824365F8;
	sub_8243C840(ctx, base);
	// 824365F8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824365FC: 419A0008  beq cr6, 0x82436604
	if ctx.cr[6].eq {
	pc = 0x82436604; continue 'dispatch;
	}
	// 82436600: 93DD0000  stw r30, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	pc = 0x82436604; continue 'dispatch;
            }
            0x82436604 => {
    //   block [0x82436604..0x824366C4)
	// 82436604: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82436608: 939F0078  stw r28, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[28].u32 ) };
	// 8243660C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436610: 939F007C  stw r28, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[28].u32 ) };
	// 82436614: 9BDF0080  stb r30, 0x80(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82436618: 9BDF0081  stb r30, 0x81(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 8243661C: 9BDF0082  stb r30, 0x82(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(130 as u32), ctx.r[30].u8 ) };
	// 82436620: 9BDF0083  stb r30, 0x83(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(131 as u32), ctx.r[30].u8 ) };
	// 82436624: 93DF0068  stw r30, 0x68(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(104 as u32), ctx.r[30].u32 ) };
	// 82436628: 48001769  bl 0x82437d90
	ctx.lr = 0x8243662C;
	sub_82437D90(ctx, base);
	// 8243662C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82436630: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436634: 48001795  bl 0x82437dc8
	ctx.lr = 0x82436638;
	sub_82437DC8(ctx, base);
	// 82436638: 807F0454  lwz r3, 0x454(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1108 as u32) ) } as u64;
	// 8243663C: 939F0060  stw r28, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[28].u32 ) };
	// 82436640: 93DF0094  stw r30, 0x94(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(148 as u32), ctx.r[30].u32 ) };
	// 82436644: 48004D2D  bl 0x8243b370
	ctx.lr = 0x82436648;
	sub_8243B370(ctx, base);
	// 82436648: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243664C: 907F004C  stw r3, 0x4c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(76 as u32), ctx.r[3].u32 ) };
	// 82436650: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436654: 419A0124  beq cr6, 0x82436778
	if ctx.cr[6].eq {
	pc = 0x82436778; continue 'dispatch;
	}
	// 82436658: 4BFFC409  bl 0x82432a60
	ctx.lr = 0x8243665C;
	sub_82432A60(ctx, base);
	// 8243665C: 807F0454  lwz r3, 0x454(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1108 as u32) ) } as u64;
	// 82436660: 4BFF0FF9  bl 0x82427658
	ctx.lr = 0x82436664;
	sub_82427658(ctx, base);
	// 82436664: 809F004C  lwz r4, 0x4c(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82436668: 93DF0084  stw r30, 0x84(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(132 as u32), ctx.r[30].u32 ) };
	// 8243666C: 907F0054  stw r3, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[3].u32 ) };
	// 82436670: 4BFF1029  bl 0x82427698
	ctx.lr = 0x82436674;
	sub_82427698(ctx, base);
	// 82436674: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436678: 4BFFFB09  bl 0x82436180
	ctx.lr = 0x8243667C;
	sub_82436180(ctx, base);
	// 8243667C: 2F03FFFF  cmpwi cr6, r3, -1
	ctx.cr[6].compare_i32(ctx.r[3].s32, -1, &mut ctx.xer);
	// 82436680: 419AFE80  beq cr6, 0x82436500
	if ctx.cr[6].eq {
	pc = 0x82436500; continue 'dispatch;
	}
	// 82436684: 80DB000C  lwz r6, 0xc(r27)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(12 as u32) ) } as u64;
	// 82436688: 80BB0008  lwz r5, 8(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8243668C: 809F00C0  lwz r4, 0xc0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(192 as u32) ) } as u64;
	// 82436690: 807F00BC  lwz r3, 0xbc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(188 as u32) ) } as u64;
	// 82436694: 4BFFB545  bl 0x82431bd8
	ctx.lr = 0x82436698;
	sub_82431BD8(ctx, base);
	// 82436698: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8243669C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824366A0: 409A0024  bne cr6, 0x824366c4
	if !ctx.cr[6].eq {
	pc = 0x824366C4; continue 'dispatch;
	}
	// 824366A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824366A8: 386B54A0  addi r3, r11, 0x54a0
	ctx.r[3].s64 = ctx.r[11].s64 + 21664;
	// 824366AC: 48000A1D  bl 0x824370c8
	ctx.lr = 0x824366B0;
	sub_824370C8(ctx, base);
	// 824366B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824366B4: 4BFFFCF5  bl 0x824363a8
	ctx.lr = 0x824366B8;
	sub_824363A8(ctx, base);
	// 824366B8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824366BC: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 824366C0: 480FEA38  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            0x824366C4 => {
    //   block [0x824366C4..0x82436714)
	// 824366C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824366C8: 809F0028  lwz r4, 0x28(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 824366CC: 917F00B8  stw r11, 0xb8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(184 as u32), ctx.r[11].u32 ) };
	// 824366D0: 4BFFB831  bl 0x82431f00
	ctx.lr = 0x824366D4;
	sub_82431F00(ctx, base);
	// 824366D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824366D8: 4BFFB8F9  bl 0x82431fd0
	ctx.lr = 0x824366DC;
	sub_82431FD0(ctx, base);
	// 824366DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824366E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824366E4: 917F0410  stw r11, 0x410(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1040 as u32), ctx.r[11].u32 ) };
	// 824366E8: 4BFFB979  bl 0x82432060
	ctx.lr = 0x824366EC;
	sub_82432060(ctx, base);
	// 824366EC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824366F0: 419A0024  beq cr6, 0x82436714
	if ctx.cr[6].eq {
	pc = 0x82436714; continue 'dispatch;
	}
	// 824366F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824366F8: 386B5478  addi r3, r11, 0x5478
	ctx.r[3].s64 = ctx.r[11].s64 + 21624;
	// 824366FC: 480009CD  bl 0x824370c8
	ctx.lr = 0x82436700;
	sub_824370C8(ctx, base);
	// 82436700: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436704: 4BFFFCA5  bl 0x824363a8
	ctx.lr = 0x82436708;
	sub_824363A8(ctx, base);
	// 82436708: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243670C: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82436710: 480FE9E8  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            0x82436714 => {
    //   block [0x82436714..0x82436778)
	// 82436714: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436718: 4BFFB9B1  bl 0x824320c8
	ctx.lr = 0x8243671C;
	sub_824320C8(ctx, base);
	// 8243671C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436720: 4BFFDCF1  bl 0x82434410
	ctx.lr = 0x82436724;
	sub_82434410(ctx, base);
	// 82436724: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436728: 4BFFE629  bl 0x82434d50
	ctx.lr = 0x8243672C;
	sub_82434D50(ctx, base);
	// 8243672C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82436730: 939F0000  stw r28, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82436734: 93DF0580  stw r30, 0x580(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1408 as u32), ctx.r[30].u32 ) };
	// 82436738: 93DF0570  stw r30, 0x570(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1392 as u32), ctx.r[30].u32 ) };
	// 8243673C: 93DF0574  stw r30, 0x574(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1396 as u32), ctx.r[30].u32 ) };
	// 82436740: 93DF0578  stw r30, 0x578(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1400 as u32), ctx.r[30].u32 ) };
	// 82436744: 93DF0598  stw r30, 0x598(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1432 as u32), ctx.r[30].u32 ) };
	// 82436748: 917F0594  stw r11, 0x594(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1428 as u32), ctx.r[11].u32 ) };
	// 8243674C: 93DF059C  stw r30, 0x59c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1436 as u32), ctx.r[30].u32 ) };
	// 82436750: 93DF05A0  stw r30, 0x5a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1440 as u32), ctx.r[30].u32 ) };
	// 82436754: 93DF05A4  stw r30, 0x5a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1444 as u32), ctx.r[30].u32 ) };
	// 82436758: 93DF05A8  stw r30, 0x5a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1448 as u32), ctx.r[30].u32 ) };
	// 8243675C: 7D78C82E  lwzx r11, r24, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82436760: 917F05B0  stw r11, 0x5b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1456 as u32), ctx.r[11].u32 ) };
	// 82436764: 7C78C82E  lwzx r3, r24, r25
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82436768: 4BFF68F9  bl 0x8242d060
	ctx.lr = 0x8243676C;
	sub_8242D060(ctx, base);
	// 8243676C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82436770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436774: 4098000C  bge cr6, 0x82436780
	if !ctx.cr[6].lt {
	pc = 0x82436780; continue 'dispatch;
	}
	pc = 0x82436778; continue 'dispatch;
            }
            0x82436778 => {
    //   block [0x82436778..0x8243677C)
	// 82436778: 4BFFFC31  bl 0x824363a8
	ctx.lr = 0x8243677C;
	sub_824363A8(ctx, base);
	pc = 0x8243677C; continue 'dispatch;
            }
            0x8243677C => {
    //   block [0x8243677C..0x82436780)
	// 8243677C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82436780; continue 'dispatch;
            }
            0x82436780 => {
    //   block [0x82436780..0x82436788)
	// 82436780: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82436784: 480FE974  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82436788 size=12
    let mut pc: u32 = 0x82436788;
    'dispatch: loop {
        match pc {
            0x82436788 => {
    //   block [0x82436788..0x82436794)
	// 82436788: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243678C: 386B0EC0  addi r3, r11, 0xec0
	ctx.r[3].s64 = ctx.r[11].s64 + 3776;
	// 82436790: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824367A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824367A0 size=104
    let mut pc: u32 = 0x824367A0;
    'dispatch: loop {
        match pc {
            0x824367A0 => {
    //   block [0x824367A0..0x82436808)
	// 824367A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824367A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824367A8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824367AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824367B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824367B4: 38CB56B0  addi r6, r11, 0x56b0
	ctx.r[6].s64 = ctx.r[11].s64 + 22192;
	// 824367B8: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 824367BC: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 824367C0: 388BC2C0  addi r4, r11, -0x3d40
	ctx.r[4].s64 = ctx.r[11].s64 + -15680;
	// 824367C4: 48000825  bl 0x82436fe8
	ctx.lr = 0x824367C8;
	sub_82436FE8(ctx, base);
	// 824367C8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824367CC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824367D0: 38AB5698  addi r5, r11, 0x5698
	ctx.r[5].s64 = ctx.r[11].s64 + 22168;
	// 824367D4: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 824367D8: 386BC598  addi r3, r11, -0x3a68
	ctx.r[3].s64 = ctx.r[11].s64 + -14952;
	// 824367DC: 4800088D  bl 0x82437068
	ctx.lr = 0x824367E0;
	sub_82437068(ctx, base);
	// 824367E0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824367E4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824367E8: 38AB5680  addi r5, r11, 0x5680
	ctx.r[5].s64 = ctx.r[11].s64 + 22144;
	// 824367EC: 3D608244  lis r11, -0x7dbc
	ctx.r[11].s64 = -2109472768;
	// 824367F0: 386BC5F8  addi r3, r11, -0x3a08
	ctx.r[3].s64 = ctx.r[11].s64 + -14856;
	// 824367F4: 4800083D  bl 0x82437030
	ctx.lr = 0x824367F8;
	sub_82437030(ctx, base);
	// 824367F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824367FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436804: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82436808 size=168
    let mut pc: u32 = 0x82436808;
    'dispatch: loop {
        match pc {
            0x82436808 => {
    //   block [0x82436808..0x82436874)
	// 82436808: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243680C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436810: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82436814: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82436818: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243681C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82436820: 4BFFFF69  bl 0x82436788
	ctx.lr = 0x82436824;
	sub_82436788(ctx, base);
	// 82436824: 38A02E10  li r5, 0x2e10
	ctx.r[5].s64 = 11792;
	// 82436828: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243682C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82436830: 480FE9A1  bl 0x825351d0
	ctx.lr = 0x82436834;
	sub_825351D0(ctx, base);
	// 82436834: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436838: 480055F1  bl 0x8243be28
	ctx.lr = 0x8243683C;
	sub_8243BE28(ctx, base);
	// 8243683C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82436840: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82436844: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82436848: 917F005C  stw r11, 0x5c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8243684C: 419A0028  beq cr6, 0x82436874
	if ctx.cr[6].eq {
	pc = 0x82436874; continue 'dispatch;
	}
	// 82436850: C01E0000  lfs f0, 0(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82436854: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82436858: 813E0004  lwz r9, 4(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243685C: 913F0008  stw r9, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 82436860: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82436864: 913F000C  stw r9, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82436868: 813E000C  lwz r9, 0xc(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243686C: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82436870: 4800001C  b 0x8243688c
	pc = 0x8243688C; continue 'dispatch;
            }
            0x82436874 => {
    //   block [0x82436874..0x8243688C)
	// 82436874: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82436878: 915F0008  stw r10, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8243687C: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82436880: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82436884: C0091FF4  lfs f0, 0x1ff4(r9)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8180 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82436888: D01F0004  stfs f0, 4(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), tmp.u32 ) };
	pc = 0x8243688C; continue 'dispatch;
            }
            0x8243688C => {
    //   block [0x8243688C..0x824368B0)
	// 8243688C: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82436890: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 82436894: 917F2E0C  stw r11, 0x2e0c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(11788 as u32), ctx.r[11].u32 ) };
	// 82436898: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243689C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824368A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824368A4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824368A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824368AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824368B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824368B0 size=36
    let mut pc: u32 = 0x824368B0;
    'dispatch: loop {
        match pc {
            0x824368B0 => {
    //   block [0x824368B0..0x824368D4)
	// 824368B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824368B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824368B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824368BC: 4BFFFECD  bl 0x82436788
	ctx.lr = 0x824368C0;
	sub_82436788(ctx, base);
	// 824368C0: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 824368C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824368C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824368CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824368D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824368D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824368D8 size=36
    let mut pc: u32 = 0x824368D8;
    'dispatch: loop {
        match pc {
            0x824368D8 => {
    //   block [0x824368D8..0x824368FC)
	// 824368D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824368DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824368E0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824368E4: 4BFFFEA5  bl 0x82436788
	ctx.lr = 0x824368E8;
	sub_82436788(ctx, base);
	// 824368E8: 8063003C  lwz r3, 0x3c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) } as u64;
	// 824368EC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824368F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824368F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824368F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436900 size=196
    let mut pc: u32 = 0x82436900;
    'dispatch: loop {
        match pc {
            0x82436900 => {
    //   block [0x82436900..0x82436948)
	// 82436900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436904: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436908: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243690C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82436910: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436914: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82436918: 4BF8B361  bl 0x823c1c78
	ctx.lr = 0x8243691C;
	sub_823C1C78(ctx, base);
	// 8243691C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82436920: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82436924: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82436928: 4BFE5B89  bl 0x8241c4b0
	ctx.lr = 0x8243692C;
	sub_8241C4B0(ctx, base);
	// 8243692C: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82436930: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82436934: 419A0014  beq cr6, 0x82436948
	if ctx.cr[6].eq {
	pc = 0x82436948; continue 'dispatch;
	}
	// 82436938: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243693C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82436940: 419A0008  beq cr6, 0x82436948
	if ctx.cr[6].eq {
	pc = 0x82436948; continue 'dispatch;
	}
	// 82436944: 83E10074  lwz r31, 0x74(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	pc = 0x82436948; continue 'dispatch;
            }
            0x82436948 => {
    //   block [0x82436948..0x82436980)
	// 82436948: 2B1F0005  cmplwi cr6, r31, 5
	ctx.cr[6].compare_u32(ctx.r[31].u32, 5 as u32, &mut ctx.xer);
	// 8243694C: 4199005C  bgt cr6, 0x824369a8
	if ctx.cr[6].gt {
	pc = 0x824369A8; continue 'dispatch;
	}
	// 82436950: 3D808243  lis r12, -0x7dbd
	ctx.r[12].s64 = -2109538304;
	// 82436954: 398C6968  addi r12, r12, 0x6968
	ctx.r[12].s64 = ctx.r[12].s64 + 26984;
	// 82436958: 57E0103A  slwi r0, r31, 2
	ctx.r[0].u32 = ctx.r[31].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 8243695C: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82436960: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82436964: 4E800420  bctr
	match ctx.r[31].u64 {
		0 => {
	pc = 0x824369A8; continue 'dispatch;
		},
		1 => {
	pc = 0x82436980; continue 'dispatch;
		},
		2 => {
	pc = 0x82436988; continue 'dispatch;
		},
		3 => {
	pc = 0x82436990; continue 'dispatch;
		},
		4 => {
	pc = 0x82436998; continue 'dispatch;
		},
		5 => {
	pc = 0x824369A0; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82436968: 824369A8  lwz r18, 0x69a8(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(27048 as u32) ) } as u64;
	// 8243696C: 82436980  lwz r18, 0x6980(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(27008 as u32) ) } as u64;
	// 82436970: 82436988  lwz r18, 0x6988(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(27016 as u32) ) } as u64;
	// 82436974: 82436990  lwz r18, 0x6990(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(27024 as u32) ) } as u64;
	// 82436978: 82436998  lwz r18, 0x6998(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(27032 as u32) ) } as u64;
	// 8243697C: 824369A0  lwz r18, 0x69a0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(27040 as u32) ) } as u64;
            }
            0x82436980 => {
    //   block [0x82436980..0x82436988)
	// 82436980: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82436984: 48000028  b 0x824369ac
	pc = 0x824369AC; continue 'dispatch;
            }
            0x82436988 => {
    //   block [0x82436988..0x82436990)
	// 82436988: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 8243698C: 48000020  b 0x824369ac
	pc = 0x824369AC; continue 'dispatch;
            }
            0x82436990 => {
    //   block [0x82436990..0x82436998)
	// 82436990: 38600008  li r3, 8
	ctx.r[3].s64 = 8;
	// 82436994: 48000018  b 0x824369ac
	pc = 0x824369AC; continue 'dispatch;
            }
            0x82436998 => {
    //   block [0x82436998..0x824369A0)
	// 82436998: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 8243699C: 48000010  b 0x824369ac
	pc = 0x824369AC; continue 'dispatch;
            }
            0x824369A0 => {
    //   block [0x824369A0..0x824369A8)
	// 824369A0: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 824369A4: 48000008  b 0x824369ac
	pc = 0x824369AC; continue 'dispatch;
            }
            0x824369A8 => {
    //   block [0x824369A8..0x824369AC)
	// 824369A8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x824369AC; continue 'dispatch;
            }
            0x824369AC => {
    //   block [0x824369AC..0x824369C4)
	// 824369AC: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 824369B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824369B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824369B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824369BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824369C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824369C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824369C8 size=48
    let mut pc: u32 = 0x824369C8;
    'dispatch: loop {
        match pc {
            0x824369C8 => {
    //   block [0x824369C8..0x824369F8)
	// 824369C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824369CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824369D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824369D4: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 824369D8: 4BFFFDB1  bl 0x82436788
	ctx.lr = 0x824369DC;
	sub_82436788(ctx, base);
	// 824369DC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824369E0: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 824369E4: 914B0068  stw r10, 0x68(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 824369E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824369EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824369F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824369F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824369F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824369F8 size=488
    let mut pc: u32 = 0x824369F8;
    'dispatch: loop {
        match pc {
            0x824369F8 => {
    //   block [0x824369F8..0x82436A30)
	// 824369F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824369FC: 480FE6C1  bl 0x825350bc
	ctx.lr = 0x82436A00;
	sub_82535080(ctx, base);
	// 82436A00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436A04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82436A08: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82436A0C: 2B1E0000  cmplwi cr6, r30, 0
	ctx.cr[6].compare_u32(ctx.r[30].u32, 0 as u32, &mut ctx.xer);
	// 82436A10: 419A0020  beq cr6, 0x82436a30
	if ctx.cr[6].eq {
	pc = 0x82436A30; continue 'dispatch;
	}
	// 82436A14: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 82436A18: 4BFFCA19  bl 0x82433430
	ctx.lr = 0x82436A1C;
	sub_82433430(ctx, base);
	// 82436A1C: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82436A20: 93CB0EA8  stw r30, 0xea8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(3752 as u32), ctx.r[30].u32 ) };
	// 82436A24: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82436A28: 906B3CD0  stw r3, 0x3cd0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(15568 as u32), ctx.r[3].u32 ) };
	// 82436A2C: 4800001C  b 0x82436a48
	pc = 0x82436A48; continue 'dispatch;
            }
            0x82436A30 => {
    //   block [0x82436A30..0x82436A48)
	// 82436A30: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 82436A34: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82436A38: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82436A3C: 916A0EA8  stw r11, 0xea8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(3752 as u32), ctx.r[11].u32 ) };
	// 82436A40: 3D408313  lis r10, -0x7ced
	ctx.r[10].s64 = -2095906816;
	// 82436A44: 916A3CD0  stw r11, 0x3cd0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(15568 as u32), ctx.r[11].u32 ) };
	pc = 0x82436A48; continue 'dispatch;
            }
            0x82436A48 => {
    //   block [0x82436A48..0x82436A74)
	// 82436A48: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82436A4C: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82436A50: 3BCB9C90  addi r30, r11, -0x6370
	ctx.r[30].s64 = ctx.r[11].s64 + -25456;
	// 82436A54: 419A0020  beq cr6, 0x82436a74
	if ctx.cr[6].eq {
	pc = 0x82436A74; continue 'dispatch;
	}
	// 82436A58: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82436A5C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82436A60: 2F0B000F  cmpwi cr6, r11, 0xf
	ctx.cr[6].compare_i32(ctx.r[11].s32, 15, &mut ctx.xer);
	// 82436A64: 7FEAF12E  stwx r31, r10, r30
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32), ctx.r[31].u32) };
	// 82436A68: 4098000C  bge cr6, 0x82436a74
	if !ctx.cr[6].lt {
	pc = 0x82436A74; continue 'dispatch;
	}
	// 82436A6C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82436A70: 917E0044  stw r11, 0x44(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	pc = 0x82436A74; continue 'dispatch;
            }
            0x82436A74 => {
    //   block [0x82436A74..0x82436AB4)
	// 82436A74: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436A78: 61650F15  ori r5, r11, 0xf15
	ctx.r[5].u64 = ctx.r[11].u64 | 3861;
	// 82436A7C: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82436A80: 4199008C  bgt cr6, 0x82436b0c
	if ctx.cr[6].gt {
	pc = 0x82436B0C; continue 'dispatch;
	}
	// 82436A84: 419A007C  beq cr6, 0x82436b00
	if ctx.cr[6].eq {
	pc = 0x82436B00; continue 'dispatch;
	}
	// 82436A88: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436A8C: 61650C04  ori r5, r11, 0xc04
	ctx.r[5].u64 = ctx.r[11].u64 | 3076;
	// 82436A90: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82436A94: 41990050  bgt cr6, 0x82436ae4
	if ctx.cr[6].gt {
	pc = 0x82436AE4; continue 'dispatch;
	}
	// 82436A98: 419A0040  beq cr6, 0x82436ad8
	if ctx.cr[6].eq {
	pc = 0x82436AD8; continue 'dispatch;
	}
	// 82436A9C: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436AA0: 616B0408  ori r11, r11, 0x408
	ctx.r[11].u64 = ctx.r[11].u64 | 1032;
	// 82436AA4: 7D6BF851  subf. r11, r11, r31
	ctx.r[11].s64 = ctx.r[31].s64 - ctx.r[11].s64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82436AA8: 418200A8  beq 0x82436b50
	if ctx.cr[0].eq {
	pc = 0x82436B50; continue 'dispatch;
	}
	// 82436AAC: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 82436AB0: 419A00A0  beq cr6, 0x82436b50
	if ctx.cr[6].eq {
	pc = 0x82436B50; continue 'dispatch;
	}
	pc = 0x82436AB4; continue 'dispatch;
            }
            0x82436AB4 => {
    //   block [0x82436AB4..0x82436ABC)
	// 82436AB4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436AB8: 388B599C  addi r4, r11, 0x599c
	ctx.r[4].s64 = ctx.r[11].s64 + 22940;
	pc = 0x82436ABC; continue 'dispatch;
            }
            0x82436ABC => {
    //   block [0x82436ABC..0x82436AC0)
	// 82436ABC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	pc = 0x82436AC0; continue 'dispatch;
            }
            0x82436AC0 => {
    //   block [0x82436AC0..0x82436AD8)
	// 82436AC0: 387E0048  addi r3, r30, 0x48
	ctx.r[3].s64 = ctx.r[30].s64 + 72;
	// 82436AC4: 480FC15D  bl 0x82532c20
	ctx.lr = 0x82436AC8;
	sub_82532C20(ctx, base);
	// 82436AC8: 387E0048  addi r3, r30, 0x48
	ctx.r[3].s64 = ctx.r[30].s64 + 72;
	// 82436ACC: 480005FD  bl 0x824370c8
	ctx.lr = 0x82436AD0;
	sub_824370C8(ctx, base);
	// 82436AD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82436AD4: 480FE638  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82436AD8 => {
    //   block [0x82436AD8..0x82436AE4)
	// 82436AD8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436ADC: 388B5918  addi r4, r11, 0x5918
	ctx.r[4].s64 = ctx.r[11].s64 + 22808;
	// 82436AE0: 4BFFFFE0  b 0x82436ac0
	pc = 0x82436AC0; continue 'dispatch;
            }
            0x82436AE4 => {
    //   block [0x82436AE4..0x82436B00)
	// 82436AE4: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436AE8: 61650F04  ori r5, r11, 0xf04
	ctx.r[5].u64 = ctx.r[11].u64 | 3844;
	// 82436AEC: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82436AF0: 409AFFC4  bne cr6, 0x82436ab4
	if !ctx.cr[6].eq {
	pc = 0x82436AB4; continue 'dispatch;
	}
	// 82436AF4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436AF8: 388B58B8  addi r4, r11, 0x58b8
	ctx.r[4].s64 = ctx.r[11].s64 + 22712;
	// 82436AFC: 4BFFFFC4  b 0x82436ac0
	pc = 0x82436AC0; continue 'dispatch;
            }
            0x82436B00 => {
    //   block [0x82436B00..0x82436B0C)
	// 82436B00: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436B04: 388B5848  addi r4, r11, 0x5848
	ctx.r[4].s64 = ctx.r[11].s64 + 22600;
	// 82436B08: 4BFFFFB8  b 0x82436ac0
	pc = 0x82436AC0; continue 'dispatch;
            }
            0x82436B0C => {
    //   block [0x82436B0C..0x82436B50)
	// 82436B0C: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436B10: 61650F1F  ori r5, r11, 0xf1f
	ctx.r[5].u64 = ctx.r[11].u64 | 3871;
	// 82436B14: 7F1F2800  cmpw cr6, r31, r5
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[5].s32, &mut ctx.xer);
	// 82436B18: 419900AC  bgt cr6, 0x82436bc4
	if ctx.cr[6].gt {
	pc = 0x82436BC4; continue 'dispatch;
	}
	// 82436B1C: 419A009C  beq cr6, 0x82436bb8
	if ctx.cr[6].eq {
	pc = 0x82436BB8; continue 'dispatch;
	}
	// 82436B20: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436B24: 616B0F17  ori r11, r11, 0xf17
	ctx.r[11].u64 = ctx.r[11].u64 | 3863;
	// 82436B28: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82436B2C: 4198FF88  blt cr6, 0x82436ab4
	if ctx.cr[6].lt {
	pc = 0x82436AB4; continue 'dispatch;
	}
	// 82436B30: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436B34: 616B0F18  ori r11, r11, 0xf18
	ctx.r[11].u64 = ctx.r[11].u64 | 3864;
	// 82436B38: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82436B3C: 40990020  ble cr6, 0x82436b5c
	if !ctx.cr[6].gt {
	pc = 0x82436B5C; continue 'dispatch;
	}
	// 82436B40: 3D60FF00  lis r11, -0x100
	ctx.r[11].s64 = -16777216;
	// 82436B44: 616B0F1C  ori r11, r11, 0xf1c
	ctx.r[11].u64 = ctx.r[11].u64 | 3868;
	// 82436B48: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82436B4C: 409AFF68  bne cr6, 0x82436ab4
	if !ctx.cr[6].eq {
	pc = 0x82436AB4; continue 'dispatch;
	}
	pc = 0x82436B50; continue 'dispatch;
            }
            0x82436B50 => {
    //   block [0x82436B50..0x82436B5C)
	// 82436B50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436B54: 388B57F0  addi r4, r11, 0x57f0
	ctx.r[4].s64 = ctx.r[11].s64 + 22512;
	// 82436B58: 4BFFFF64  b 0x82436abc
	pc = 0x82436ABC; continue 'dispatch;
            }
            0x82436B5C => {
    //   block [0x82436B5C..0x82436B88)
	// 82436B5C: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 82436B60: 419A0028  beq cr6, 0x82436b88
	if ctx.cr[6].eq {
	pc = 0x82436B88; continue 'dispatch;
	}
	// 82436B64: 817D00D0  lwz r11, 0xd0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(208 as u32) ) } as u64;
	// 82436B68: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82436B6C: 409A001C  bne cr6, 0x82436b88
	if !ctx.cr[6].eq {
	pc = 0x82436B88; continue 'dispatch;
	}
	// 82436B70: 80DD00D8  lwz r6, 0xd8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(216 as u32) ) } as u64;
	// 82436B74: 80FD00DC  lwz r7, 0xdc(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(220 as u32) ) } as u64;
	// 82436B78: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82436B7C: 4099000C  ble cr6, 0x82436b88
	if !ctx.cr[6].gt {
	pc = 0x82436B88; continue 'dispatch;
	}
	// 82436B80: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82436B84: 41990010  bgt cr6, 0x82436b94
	if ctx.cr[6].gt {
	pc = 0x82436B94; continue 'dispatch;
	}
	pc = 0x82436B88; continue 'dispatch;
            }
            0x82436B88 => {
    //   block [0x82436B88..0x82436B94)
	// 82436B88: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436B8C: 388B5790  addi r4, r11, 0x5790
	ctx.r[4].s64 = ctx.r[11].s64 + 22416;
	// 82436B90: 4BFFFF2C  b 0x82436abc
	pc = 0x82436ABC; continue 'dispatch;
            }
            0x82436B94 => {
    //   block [0x82436B94..0x82436BB8)
	// 82436B94: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436B98: 387E0048  addi r3, r30, 0x48
	ctx.r[3].s64 = ctx.r[30].s64 + 72;
	// 82436B9C: 388B5728  addi r4, r11, 0x5728
	ctx.r[4].s64 = ctx.r[11].s64 + 22312;
	// 82436BA0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82436BA4: 480FC07D  bl 0x82532c20
	ctx.lr = 0x82436BA8;
	sub_82532C20(ctx, base);
	// 82436BA8: 387E0048  addi r3, r30, 0x48
	ctx.r[3].s64 = ctx.r[30].s64 + 72;
	// 82436BAC: 4800051D  bl 0x824370c8
	ctx.lr = 0x82436BB0;
	sub_824370C8(ctx, base);
	// 82436BB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82436BB4: 480FE558  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82436BB8 => {
    //   block [0x82436BB8..0x82436BC4)
	// 82436BB8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436BBC: 388B56E0  addi r4, r11, 0x56e0
	ctx.r[4].s64 = ctx.r[11].s64 + 22240;
	// 82436BC0: 4BFFFF00  b 0x82436ac0
	pc = 0x82436AC0; continue 'dispatch;
            }
            0x82436BC4 => {
    //   block [0x82436BC4..0x82436BE0)
	// 82436BC4: 2F1FFFFD  cmpwi cr6, r31, -3
	ctx.cr[6].compare_i32(ctx.r[31].s32, -3, &mut ctx.xer);
	// 82436BC8: 4198FEEC  blt cr6, 0x82436ab4
	if ctx.cr[6].lt {
	pc = 0x82436AB4; continue 'dispatch;
	}
	// 82436BCC: 2F1FFFFE  cmpwi cr6, r31, -2
	ctx.cr[6].compare_i32(ctx.r[31].s32, -2, &mut ctx.xer);
	// 82436BD0: 4199FEE4  bgt cr6, 0x82436ab4
	if ctx.cr[6].gt {
	pc = 0x82436AB4; continue 'dispatch;
	}
	// 82436BD4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436BD8: 388B56C8  addi r4, r11, 0x56c8
	ctx.r[4].s64 = ctx.r[11].s64 + 22216;
	// 82436BDC: 4BFFFEE0  b 0x82436abc
	pc = 0x82436ABC; continue 'dispatch;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436BE0 size=68
    let mut pc: u32 = 0x82436BE0;
    'dispatch: loop {
        match pc {
            0x82436BE0 => {
    //   block [0x82436BE0..0x82436C0C)
	// 82436BE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436BE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436BE8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436BEC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82436BF0: 419A001C  beq cr6, 0x82436c0c
	if ctx.cr[6].eq {
	pc = 0x82436C0C; continue 'dispatch;
	}
	// 82436BF4: 4BFFC83D  bl 0x82433430
	ctx.lr = 0x82436BF8;
	sub_82433430(ctx, base);
	// 82436BF8: 48009379  bl 0x8243ff70
	ctx.lr = 0x82436BFC;
	sub_8243FF70(ctx, base);
	// 82436BFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82436C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436C08: 4E800020  blr
	return;
            }
            0x82436C0C => {
    //   block [0x82436C0C..0x82436C24)
	// 82436C0C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436C10: 48009361  bl 0x8243ff70
	ctx.lr = 0x82436C14;
	sub_8243FF70(ctx, base);
	// 82436C14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82436C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436C20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82436C28 size=160
    let mut pc: u32 = 0x82436C28;
    'dispatch: loop {
        match pc {
            0x82436C28 => {
    //   block [0x82436C28..0x82436C9C)
	// 82436C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436C30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82436C34: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436C38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82436C3C: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82436C40: 3880001B  li r4, 0x1b
	ctx.r[4].s64 = 27;
	// 82436C44: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436C48: C01F0000  lfs f0, 0(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82436C4C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82436C50: 7C005FAE  stfiwx f0, 0, r11
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32, tmp.u32) };
	// 82436C54: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82436C58: 4BFFC389  bl 0x82432fe0
	ctx.lr = 0x82436C5C;
	sub_82432FE0(ctx, base);
	// 82436C5C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82436C60: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82436C64: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436C68: 4BFFC379  bl 0x82432fe0
	ctx.lr = 0x82436C6C;
	sub_82432FE0(ctx, base);
	// 82436C6C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436C70: 4BFFFC91  bl 0x82436900
	ctx.lr = 0x82436C74;
	sub_82436900(ctx, base);
	// 82436C74: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82436C78: 419A002C  beq cr6, 0x82436ca4
	if ctx.cr[6].eq {
	pc = 0x82436CA4; continue 'dispatch;
	}
	// 82436C7C: 2F030002  cmpwi cr6, r3, 2
	ctx.cr[6].compare_i32(ctx.r[3].s32, 2, &mut ctx.xer);
	// 82436C80: 419A0024  beq cr6, 0x82436ca4
	if ctx.cr[6].eq {
	pc = 0x82436CA4; continue 'dispatch;
	}
	// 82436C84: 2F030004  cmpwi cr6, r3, 4
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4, &mut ctx.xer);
	// 82436C88: 419A0014  beq cr6, 0x82436c9c
	if ctx.cr[6].eq {
	pc = 0x82436C9C; continue 'dispatch;
	}
	// 82436C8C: 2F030008  cmpwi cr6, r3, 8
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8, &mut ctx.xer);
	// 82436C90: 419A000C  beq cr6, 0x82436c9c
	if ctx.cr[6].eq {
	pc = 0x82436C9C; continue 'dispatch;
	}
	// 82436C94: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82436C98: 48000010  b 0x82436ca8
	pc = 0x82436CA8; continue 'dispatch;
            }
            0x82436C9C => {
    //   block [0x82436C9C..0x82436CA4)
	// 82436C9C: 38A00020  li r5, 0x20
	ctx.r[5].s64 = 32;
	// 82436CA0: 48000008  b 0x82436ca8
	pc = 0x82436CA8; continue 'dispatch;
            }
            0x82436CA4 => {
    //   block [0x82436CA4..0x82436CA8)
	// 82436CA4: 38A00028  li r5, 0x28
	ctx.r[5].s64 = 40;
	pc = 0x82436CA8; continue 'dispatch;
            }
            0x82436CA8 => {
    //   block [0x82436CA8..0x82436CC8)
	// 82436CA8: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 82436CAC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436CB0: 48009329  bl 0x8243ffd8
	ctx.lr = 0x82436CB4;
	sub_8243FFD8(ctx, base);
	// 82436CB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82436CB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436CBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436CC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82436CC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436CC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436CC8 size=200
    let mut pc: u32 = 0x82436CC8;
    'dispatch: loop {
        match pc {
            0x82436CC8 => {
    //   block [0x82436CC8..0x82436D20)
	// 82436CC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436CCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436CD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436CD4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82436CD8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82436CDC: 386A59E0  addi r3, r10, 0x59e0
	ctx.r[3].s64 = ctx.r[10].s64 + 23008;
	// 82436CE0: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82436CE4: 38803880  li r4, 0x3880
	ctx.r[4].s64 = 14464;
	// 82436CE8: E94A5674  ld r10, 0x5674(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(22132 as u32) ) };
	// 82436CEC: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 82436CF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82436CF4: 48010AF5  bl 0x824477e8
	ctx.lr = 0x82436CF8;
	sub_824477E8(ctx, base);
	// 82436CF8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82436CFC: 419A0024  beq cr6, 0x82436d20
	if ctx.cr[6].eq {
	pc = 0x82436D20; continue 'dispatch;
	}
	// 82436D00: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436D04: 386B59AC  addi r3, r11, 0x59ac
	ctx.r[3].s64 = ctx.r[11].s64 + 22956;
	// 82436D08: 480003C1  bl 0x824370c8
	ctx.lr = 0x82436D0C;
	sub_824370C8(ctx, base);
	// 82436D0C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82436D10: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82436D14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436D18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436D1C: 4E800020  blr
	return;
            }
            0x82436D20 => {
    //   block [0x82436D20..0x82436D48)
	// 82436D20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82436D24: 48010D9D  bl 0x82447ac0
	ctx.lr = 0x82436D28;
	sub_82447AC0(ctx, base);
	// 82436D28: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82436D2C: 419A001C  beq cr6, 0x82436d48
	if ctx.cr[6].eq {
	pc = 0x82436D48; continue 'dispatch;
	}
	// 82436D30: 3860FED3  li r3, -0x12d
	ctx.r[3].s64 = -301;
	// 82436D34: 4BFFFC95  bl 0x824369c8
	ctx.lr = 0x82436D38;
	sub_824369C8(ctx, base);
	// 82436D38: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82436D3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436D40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436D44: 4E800020  blr
	return;
            }
            0x82436D48 => {
    //   block [0x82436D48..0x82436D7C)
	// 82436D48: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 82436D4C: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82436D50: 388B69F8  addi r4, r11, 0x69f8
	ctx.r[4].s64 = ctx.r[11].s64 + 27128;
	// 82436D54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436D58: 48010C39  bl 0x82447990
	ctx.lr = 0x82436D5C;
	sub_82447990(ctx, base);
	// 82436D5C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82436D60: 419A001C  beq cr6, 0x82436d7c
	if ctx.cr[6].eq {
	pc = 0x82436D7C; continue 'dispatch;
	}
	// 82436D64: 3860FED1  li r3, -0x12f
	ctx.r[3].s64 = -303;
	// 82436D68: 4BFFFC61  bl 0x824369c8
	ctx.lr = 0x82436D6C;
	sub_824369C8(ctx, base);
	// 82436D6C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82436D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436D78: 4E800020  blr
	return;
            }
            0x82436D7C => {
    //   block [0x82436D7C..0x82436D90)
	// 82436D7C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82436D80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82436D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82436D90 size=392
    let mut pc: u32 = 0x82436D90;
    'dispatch: loop {
        match pc {
            0x82436D90 => {
    //   block [0x82436D90..0x82436DC0)
	// 82436D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436D98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82436D9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82436DA0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82436DA4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436DA8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82436DAC: 409A0014  bne cr6, 0x82436dc0
	if !ctx.cr[6].eq {
	pc = 0x82436DC0; continue 'dispatch;
	}
	// 82436DB0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436DB4: 386B5A2C  addi r3, r11, 0x5a2c
	ctx.r[3].s64 = ctx.r[11].s64 + 23084;
	// 82436DB8: 48000311  bl 0x824370c8
	ctx.lr = 0x82436DBC;
	sub_824370C8(ctx, base);
	// 82436DBC: 48000140  b 0x82436efc
	pc = 0x82436EFC; continue 'dispatch;
            }
            0x82436DC0 => {
    //   block [0x82436DC0..0x82436E70)
	// 82436DC0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82436DC4: C3E30000  lfs f31, 0(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82436DC8: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82436DCC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82436DD0: 3D20828A  lis r9, -0x7d76
	ctx.r[9].s64 = -2104885248;
	// 82436DD4: 394A55E8  addi r10, r10, 0x55e8
	ctx.r[10].s64 = ctx.r[10].s64 + 21992;
	// 82436DD8: FBEB0000  std r31, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[31].u64 ) };
	// 82436DDC: FBEB0008  std r31, 8(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 82436DE0: FBEB0010  std r31, 0x10(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[31].u64 ) };
	// 82436DE4: FBEB0018  std r31, 0x18(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[31].u64 ) };
	// 82436DE8: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82436DEC: D3E10060  stfs f31, 0x60(r1)
	tmp.f32 = (ctx.f[31].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82436DF0: 91499CD0  stw r10, -0x6330(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-25392 as u32), ctx.r[10].u32 ) };
	// 82436DF4: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82436DF8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82436DFC: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82436E00: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82436E04: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82436E08: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82436E0C: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82436E10: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
	// 82436E14: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82436E18: 8163001C  lwz r11, 0x1c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82436E1C: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82436E20: 48000179  bl 0x82436f98
	ctx.lr = 0x82436E24;
	sub_82436F98(ctx, base);
	// 82436E24: 3FC0828A  lis r30, -0x7d76
	ctx.r[30].s64 = -2104885248;
	// 82436E28: 93E10068  stw r31, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[31].u32 ) };
	// 82436E2C: 817E9DD8  lwz r11, -0x6228(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-25128 as u32) ) } as u64;
	// 82436E30: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82436E34: 409A00C0  bne cr6, 0x82436ef4
	if !ctx.cr[6].eq {
	pc = 0x82436EF4; continue 'dispatch;
	}
	// 82436E38: 48000319  bl 0x82437150
	ctx.lr = 0x82436E3C;
	sub_82437150(ctx, base);
	// 82436E3C: 4BFFE805  bl 0x82435640
	ctx.lr = 0x82436E40;
	sub_82435640(ctx, base);
	// 82436E40: 4BFE4EE1  bl 0x8241bd20
	ctx.lr = 0x82436E44;
	sub_8241BD20(ctx, base);
	// 82436E44: 4BFEBF25  bl 0x82422d68
	ctx.lr = 0x82436E48;
	sub_82422D68(ctx, base);
	// 82436E48: 4BFEE159  bl 0x82424fa0
	ctx.lr = 0x82436E4C;
	sub_82424FA0(ctx, base);
	// 82436E4C: 4BFF4565  bl 0x8242b3b0
	ctx.lr = 0x82436E50;
	sub_8242B3B0(ctx, base);
	// 82436E50: 480008B9  bl 0x82437708
	ctx.lr = 0x82436E54;
	sub_82437708(ctx, base);
	// 82436E54: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82436E58: 419A0018  beq cr6, 0x82436e70
	if ctx.cr[6].eq {
	pc = 0x82436E70; continue 'dispatch;
	}
	// 82436E5C: 3860FF9B  li r3, -0x65
	ctx.r[3].s64 = -101;
	// 82436E60: 4BFFFB69  bl 0x824369c8
	ctx.lr = 0x82436E64;
	sub_824369C8(ctx, base);
	// 82436E64: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436E68: 386B5A04  addi r3, r11, 0x5a04
	ctx.r[3].s64 = ctx.r[11].s64 + 23044;
	// 82436E6C: 4800025D  bl 0x824370c8
	ctx.lr = 0x82436E70;
	sub_824370C8(ctx, base);
	pc = 0x82436E70; continue 'dispatch;
            }
            0x82436E70 => {
    //   block [0x82436E70..0x82436EBC)
	// 82436E70: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82436E74: 4BFFF995  bl 0x82436808
	ctx.lr = 0x82436E78;
	sub_82436808(ctx, base);
	// 82436E78: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82436E7C: C00B204C  lfs f0, 0x204c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8268 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82436E80: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82436E84: 93EB06FC  stw r31, 0x6fc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1788 as u32), ctx.r[31].u32 ) };
	// 82436E88: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82436E8C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82436E90: C1ABBFFC  lfs f13, -0x4004(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-16388 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82436E94: EC1F683A  fmadds f0, f31, f0, f13
	ctx.f[0].f64 = (((ctx.f[31].f64 * ctx.f[0].f64 + ctx.f[13].f64) as f32) as f64);
	// 82436E98: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 82436E9C: 7C0057AE  stfiwx f0, 0, r10
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32, tmp.u32) };
	// 82436EA0: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82436EA4: 4BFFFE25  bl 0x82436cc8
	ctx.lr = 0x82436EA8;
	sub_82436CC8(ctx, base);
	// 82436EA8: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82436EAC: 419A0010  beq cr6, 0x82436ebc
	if ctx.cr[6].eq {
	pc = 0x82436EBC; continue 'dispatch;
	}
	// 82436EB0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82436EB4: 386B59E8  addi r3, r11, 0x59e8
	ctx.r[3].s64 = ctx.r[11].s64 + 23016;
	// 82436EB8: 48000211  bl 0x824370c8
	ctx.lr = 0x82436EBC;
	sub_824370C8(ctx, base);
	pc = 0x82436EBC; continue 'dispatch;
            }
            0x82436EBC => {
    //   block [0x82436EBC..0x82436EF4)
	// 82436EBC: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 82436EC0: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82436EC4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82436EC8: 916A9DDC  stw r11, -0x6224(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-25124 as u32), ctx.r[11].u32 ) };
	// 82436ECC: 4BFFFD5D  bl 0x82436c28
	ctx.lr = 0x82436ED0;
	sub_82436C28(ctx, base);
	// 82436ED0: 4BFF3F61  bl 0x8242ae30
	ctx.lr = 0x82436ED4;
	sub_8242AE30(ctx, base);
	// 82436ED4: 3D608243  lis r11, -0x7dbd
	ctx.r[11].s64 = -2109538304;
	// 82436ED8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82436EDC: 386B6798  addi r3, r11, 0x6798
	ctx.r[3].s64 = ctx.r[11].s64 + 26520;
	// 82436EE0: 4BFF3E89  bl 0x8242ad68
	ctx.lr = 0x82436EE4;
	sub_8242AD68(ctx, base);
	// 82436EE4: 4BFFB30D  bl 0x824321f0
	ctx.lr = 0x82436EE8;
	sub_824321F0(ctx, base);
	// 82436EE8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82436EEC: 4BFFF8B5  bl 0x824367a0
	ctx.lr = 0x82436EF0;
	sub_824367A0(ctx, base);
	// 82436EF0: 817E9DD8  lwz r11, -0x6228(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(-25128 as u32) ) } as u64;
	pc = 0x82436EF4; continue 'dispatch;
            }
            0x82436EF4 => {
    //   block [0x82436EF4..0x82436EFC)
	// 82436EF4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82436EF8: 917E9DD8  stw r11, -0x6228(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(-25128 as u32), ctx.r[11].u32 ) };
	pc = 0x82436EFC; continue 'dispatch;
            }
            0x82436EFC => {
    //   block [0x82436EFC..0x82436F18)
	// 82436EFC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82436F00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436F04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436F08: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82436F0C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82436F10: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82436F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436F18 size=104
    let mut pc: u32 = 0x82436F18;
    'dispatch: loop {
        match pc {
            0x82436F18 => {
    //   block [0x82436F18..0x82436F80)
	// 82436F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436F1C: 480FE195  bl 0x825350b0
	ctx.lr = 0x82436F20;
	sub_82535080(ctx, base);
	// 82436F20: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436F24: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82436F28: 83E300B8  lwz r31, 0xb8(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(184 as u32) ) } as u64;
	// 82436F2C: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 82436F30: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82436F34: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82436F38: 7D1B4378  mr r27, r8
	ctx.r[27].u64 = ctx.r[8].u64;
	// 82436F3C: 7D3A4B78  mr r26, r9
	ctx.r[26].u64 = ctx.r[9].u64;
	// 82436F40: 4BFFB619  bl 0x82432558
	ctx.lr = 0x82436F44;
	sub_82432558(ctx, base);
	// 82436F44: 7F4AD378  mr r10, r26
	ctx.r[10].u64 = ctx.r[26].u64;
	// 82436F48: 7F69DB78  mr r9, r27
	ctx.r[9].u64 = ctx.r[27].u64;
	// 82436F4C: 7F88E378  mr r8, r28
	ctx.r[8].u64 = ctx.r[28].u64;
	// 82436F50: 7FA7EB78  mr r7, r29
	ctx.r[7].u64 = ctx.r[29].u64;
	// 82436F54: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82436F58: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82436F5C: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82436F60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436F64: 480011BD  bl 0x82438120
	ctx.lr = 0x82436F68;
	sub_82438120(ctx, base);
	// 82436F68: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82436F6C: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82436F70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82436F74: 48010BB5  bl 0x82447b28
	ctx.lr = 0x82436F78;
	sub_82447B28(ctx, base);
	// 82436F78: 38210190  addi r1, r1, 0x190
	ctx.r[1].s64 = ctx.r[1].s64 + 400;
	// 82436F7C: 480FE184  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82436F80 size=20
    let mut pc: u32 = 0x82436F80;
    'dispatch: loop {
        match pc {
            0x82436F80 => {
    //   block [0x82436F80..0x82436F94)
	// 82436F80: 81240010  lwz r9, 0x10(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82436F84: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82436F88: 8104000C  lwz r8, 0xc(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82436F8C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82436F90: 4BFFFF88  b 0x82436f18
	sub_82436F18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436F98 size=80
    let mut pc: u32 = 0x82436F98;
    'dispatch: loop {
        match pc {
            0x82436F98 => {
    //   block [0x82436F98..0x82436FE8)
	// 82436F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436FA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82436FA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436FA8: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82436FAC: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82436FB0: 3BEB9F28  addi r31, r11, -0x60d8
	ctx.r[31].s64 = ctx.r[11].s64 + -24792;
	// 82436FB4: 387FFEDC  addi r3, r31, -0x124
	ctx.r[3].s64 = ctx.r[31].s64 + -292;
	// 82436FB8: 4BFF5EA1  bl 0x8242ce58
	ctx.lr = 0x82436FBC;
	sub_8242CE58(ctx, base);
	// 82436FBC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82436FC0: 907F0008  stw r3, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 82436FC4: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82436FC8: 917FFEFC  stw r11, -0x104(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-260 as u32), ctx.r[11].u32 ) };
	// 82436FCC: 917FFED8  stw r11, -0x128(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-296 as u32), ctx.r[11].u32 ) };
	// 82436FD0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82436FD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82436FD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82436FDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82436FE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82436FE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82436FE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82436FE8 size=72
    let mut pc: u32 = 0x82436FE8;
    'dispatch: loop {
        match pc {
            0x82436FE8 => {
    //   block [0x82436FE8..0x82437030)
	// 82436FE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82436FEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82436FF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82436FF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82436FF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82436FFC: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 82437000: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82437004: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82437008: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8243700C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82437010: 4BFEEF51  bl 0x82425f60
	ctx.lr = 0x82437014;
	sub_82425F60(ctx, base);
	// 82437014: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82437018: 93EB9E24  stw r31, -0x61dc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-25052 as u32), ctx.r[31].u32 ) };
	// 8243701C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437020: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437024: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437028: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243702C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437030 size=56
    let mut pc: u32 = 0x82437030;
    'dispatch: loop {
        match pc {
            0x82437030 => {
    //   block [0x82437030..0x82437068)
	// 82437030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437034: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437038: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243703C: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82437040: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82437044: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82437048: 38600006  li r3, 6
	ctx.r[3].s64 = 6;
	// 8243704C: 4BFEED8D  bl 0x82425dd8
	ctx.lr = 0x82437050;
	sub_82425DD8(ctx, base);
	// 82437050: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82437054: 906B9E00  stw r3, -0x6200(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-25088 as u32), ctx.r[3].u32 ) };
	// 82437058: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243705C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437060: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437064: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437068 size=56
    let mut pc: u32 = 0x82437068;
    'dispatch: loop {
        match pc {
            0x82437068 => {
    //   block [0x82437068..0x824370A0)
	// 82437068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243706C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437074: 7CA62B78  mr r6, r5
	ctx.r[6].u64 = ctx.r[5].u64;
	// 82437078: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8243707C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82437080: 38600005  li r3, 5
	ctx.r[3].s64 = 5;
	// 82437084: 4BFEED55  bl 0x82425dd8
	ctx.lr = 0x82437088;
	sub_82425DD8(ctx, base);
	// 82437088: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 8243708C: 906B9F28  stw r3, -0x60d8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-24792 as u32), ctx.r[3].u32 ) };
	// 82437090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243709C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824370A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824370A0 size=12
    let mut pc: u32 = 0x824370A0;
    'dispatch: loop {
        match pc {
            0x824370A0 => {
    //   block [0x824370A0..0x824370AC)
	// 824370A0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824370A4: 806B9F34  lwz r3, -0x60cc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24780 as u32) ) } as u64;
	// 824370A8: 4BFF5F20  b 0x8242cfc8
	sub_8242CFC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824370B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824370B0 size=12
    let mut pc: u32 = 0x824370B0;
    'dispatch: loop {
        match pc {
            0x824370B0 => {
    //   block [0x824370B0..0x824370BC)
	// 824370B0: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824370B4: 806B9F34  lwz r3, -0x60cc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-24780 as u32) ) } as u64;
	// 824370B8: 4BFF5FA8  b 0x8242d060
	sub_8242D060(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824370C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824370C0 size=4
    let mut pc: u32 = 0x824370C0;
    'dispatch: loop {
        match pc {
            0x824370C0 => {
    //   block [0x824370C0..0x824370C4)
	// 824370C0: 4BFEEC98  b 0x82425d58
	sub_82425D58(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824370C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824370C8 size=136
    let mut pc: u32 = 0x824370C8;
    'dispatch: loop {
        match pc {
            0x824370C8 => {
    //   block [0x824370C8..0x82437150)
	// 824370C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824370CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824370D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824370D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824370D8: F8810018  std r4, 0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(24 as u32), ctx.r[4].u64 ) };
	// 824370DC: F8A10020  std r5, 0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(32 as u32), ctx.r[5].u64 ) };
	// 824370E0: F8C10028  std r6, 0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(40 as u32), ctx.r[6].u64 ) };
	// 824370E4: F8E10030  std r7, 0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(48 as u32), ctx.r[7].u64 ) };
	// 824370E8: F9010038  std r8, 0x38(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(56 as u32), ctx.r[8].u64 ) };
	// 824370EC: F9210040  std r9, 0x40(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(64 as u32), ctx.r[9].u64 ) };
	// 824370F0: F9410048  std r10, 0x48(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(72 as u32), ctx.r[10].u64 ) };
	// 824370F4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824370F8: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 824370FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82437100: 3BEB9E28  addi r31, r11, -0x61d8
	ctx.r[31].s64 = ctx.r[11].s64 + -25048;
	// 82437104: 38A00100  li r5, 0x100
	ctx.r[5].s64 = 256;
	// 82437108: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 8243710C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437110: 480FE0C1  bl 0x825351d0
	ctx.lr = 0x82437114;
	sub_825351D0(ctx, base);
	// 82437114: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82437118: 39410088  addi r10, r1, 0x88
	ctx.r[10].s64 = ctx.r[1].s64 + 136;
	// 8243711C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82437120: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437124: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82437128: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243712C: 480FBCED  bl 0x82532e18
	ctx.lr = 0x82437130;
	sub_82532E18(ctx, base);
	// 82437130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437134: 4BFF6645  bl 0x8242d778
	ctx.lr = 0x82437138;
	sub_8242D778(ctx, base);
	// 82437138: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243713C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437144: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82437148: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243714C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437150 size=80
    let mut pc: u32 = 0x82437150;
    'dispatch: loop {
        match pc {
            0x82437150 => {
    //   block [0x82437150..0x8243718C)
	// 82437150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437158: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243715C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437160: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82437164: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82437168: 3BEB9DE0  addi r31, r11, -0x6220
	ctx.r[31].s64 = ctx.r[11].s64 + -25120;
	// 8243716C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437170: 4BFF5CE9  bl 0x8242ce58
	ctx.lr = 0x82437174;
	sub_8242CE58(ctx, base);
	// 82437174: 907F0154  stw r3, 0x154(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(340 as u32), ctx.r[3].u32 ) };
	// 82437178: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 8243717C: 409A0010  bne cr6, 0x8243718c
	if !ctx.cr[6].eq {
	pc = 0x8243718C; continue 'dispatch;
	}
	// 82437180: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82437184: 386B5A54  addi r3, r11, 0x5a54
	ctx.r[3].s64 = ctx.r[11].s64 + 23124;
	// 82437188: 4BFFFF41  bl 0x824370c8
	ctx.lr = 0x8243718C;
	sub_824370C8(ctx, base);
	pc = 0x8243718C; continue 'dispatch;
            }
            0x8243718C => {
    //   block [0x8243718C..0x824371A0)
	// 8243718C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82437190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437198: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243719C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824371A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824371A0 size=12
    let mut pc: u32 = 0x824371A0;
    'dispatch: loop {
        match pc {
            0x824371A0 => {
    //   block [0x824371A0..0x824371AC)
	// 824371A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824371A4: 386B5A84  addi r3, r11, 0x5a84
	ctx.r[3].s64 = ctx.r[11].s64 + 23172;
	// 824371A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824371B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824371B0 size=20
    let mut pc: u32 = 0x824371B0;
    'dispatch: loop {
        match pc {
            0x824371B0 => {
    //   block [0x824371B0..0x824371C4)
	// 824371B0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824371B4: 396B09A0  addi r11, r11, 0x9a0
	ctx.r[11].s64 = ctx.r[11].s64 + 2464;
	// 824371B8: 906B0008  stw r3, 8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(8 as u32), ctx.r[3].u32 ) };
	// 824371BC: 908B000C  stw r4, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 824371C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824371C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824371C8 size=64
    let mut pc: u32 = 0x824371C8;
    'dispatch: loop {
        match pc {
            0x824371C8 => {
    //   block [0x824371C8..0x824371E4)
	// 824371C8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824371CC: 394B09A0  addi r10, r11, 0x9a0
	ctx.r[10].s64 = ctx.r[11].s64 + 2464;
	// 824371D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824371D4: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 824371D8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824371DC: 40990024  ble cr6, 0x82437200
	if !ctx.cr[6].gt {
	pc = 0x82437200; continue 'dispatch;
	}
	// 824371E0: 386A0018  addi r3, r10, 0x18
	ctx.r[3].s64 = ctx.r[10].s64 + 24;
	pc = 0x824371E4; continue 'dispatch;
            }
            0x824371E4 => {
    //   block [0x824371E4..0x82437200)
	// 824371E4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824371E8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824371EC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824371F0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824371F4: 3863009C  addi r3, r3, 0x9c
	ctx.r[3].s64 = ctx.r[3].s64 + 156;
	// 824371F8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824371FC: 4198FFE8  blt cr6, 0x824371e4
	if ctx.cr[6].lt {
	pc = 0x824371E4; continue 'dispatch;
	}
	pc = 0x82437200; continue 'dispatch;
            }
            0x82437200 => {
    //   block [0x82437200..0x82437208)
	// 82437200: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437204: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437208 size=136
    let mut pc: u32 = 0x82437208;
    'dispatch: loop {
        match pc {
            0x82437208 => {
    //   block [0x82437208..0x82437290)
	// 82437208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243720C: 480FDEB1  bl 0x825350bc
	ctx.lr = 0x82437210;
	sub_82535080(ctx, base);
	// 82437210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437214: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82437218: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8243721C: 38A0009C  li r5, 0x9c
	ctx.r[5].s64 = 156;
	// 82437220: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82437224: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82437228: 480FDFA9  bl 0x825351d0
	ctx.lr = 0x8243722C;
	sub_825351D0(ctx, base);
	// 8243722C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82437230: 395E001F  addi r10, r30, 0x1f
	ctx.r[10].s64 = ctx.r[30].s64 + 31;
	// 82437234: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 82437238: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8243723C: 93BF0054  stw r29, 0x54(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82437240: 554A0034  rlwinm r10, r10, 0, 0, 0x1a
	ctx.r[10].u64 = ctx.r[10].u32 as u64 & 0xFFFFFFFFu64;
	// 82437244: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82437248: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 8243724C: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82437250: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82437254: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82437258: 917F0034  stw r11, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[11].u32 ) };
	// 8243725C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82437260: 396A0400  addi r11, r10, 0x400
	ctx.r[11].s64 = ctx.r[10].s64 + 1024;
	// 82437264: 915F0038  stw r10, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[10].u32 ) };
	// 82437268: 394B0400  addi r10, r11, 0x400
	ctx.r[10].s64 = ctx.r[11].s64 + 1024;
	// 8243726C: 913F0028  stw r9, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[9].u32 ) };
	// 82437270: 911F0058  stw r8, 0x58(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82437274: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82437278: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 8243727C: 396A0400  addi r11, r10, 0x400
	ctx.r[11].s64 = ctx.r[10].s64 + 1024;
	// 82437280: 915F0040  stw r10, 0x40(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(64 as u32), ctx.r[10].u32 ) };
	// 82437284: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 82437288: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243728C: 480FDE80  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437290 size=20
    let mut pc: u32 = 0x82437290;
    'dispatch: loop {
        match pc {
            0x82437290 => {
    //   block [0x82437290..0x824372A4)
	// 82437290: 2F03201F  cmpwi cr6, r3, 0x201f
	ctx.cr[6].compare_i32(ctx.r[3].s32, 8223, &mut ctx.xer);
	// 82437294: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82437298: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
	// 8243729C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824372A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824372A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824372A8 size=92
    let mut pc: u32 = 0x824372A8;
    'dispatch: loop {
        match pc {
            0x824372A8 => {
    //   block [0x824372A8..0x824372F0)
	// 824372A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824372AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824372B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824372B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824372B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824372BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824372C0: 419A0030  beq cr6, 0x824372f0
	if ctx.cr[6].eq {
	pc = 0x824372F0; continue 'dispatch;
	}
	// 824372C4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 824372C8: 83EB0030  lwz r31, 0x30(r11)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 824372CC: 806B0024  lwz r3, 0x24(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 824372D0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824372D4: 4800143D  bl 0x82438710
	ctx.lr = 0x824372D8;
	sub_82438710(ctx, base);
	// 824372D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824372DC: 480109C5  bl 0x82447ca0
	ctx.lr = 0x824372E0;
	sub_82447CA0(ctx, base);
	// 824372E0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824372E4: 814B09A0  lwz r10, 0x9a0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 824372E8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824372EC: 914B09A0  stw r10, 0x9a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2464 as u32), ctx.r[10].u32 ) };
	pc = 0x824372F0; continue 'dispatch;
            }
            0x824372F0 => {
    //   block [0x824372F0..0x82437304)
	// 824372F0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824372F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824372F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824372FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82437300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437308 size=52
    let mut pc: u32 = 0x82437308;
    'dispatch: loop {
        match pc {
            0x82437308 => {
    //   block [0x82437308..0x8243733C)
	// 82437308: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 8243730C: 396B09A0  addi r11, r11, 0x9a0
	ctx.r[11].s64 = ctx.r[11].s64 + 2464;
	// 82437310: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82437314: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82437318: 914B0010  stw r10, 0x10(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 8243731C: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82437320: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82437324: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82437328: 806B000C  lwz r3, 0xc(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243732C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82437330: 554B003E  slwi r11, r10, 0
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82437334: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82437338: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437340 size=16
    let mut pc: u32 = 0x82437340;
    'dispatch: loop {
        match pc {
            0x82437340 => {
    //   block [0x82437340..0x82437350)
	// 82437340: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82437344: 396B09A0  addi r11, r11, 0x9a0
	ctx.r[11].s64 = ctx.r[11].s64 + 2464;
	// 82437348: 906B0014  stw r3, 0x14(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(20 as u32), ctx.r[3].u32 ) };
	// 8243734C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437350 size=16
    let mut pc: u32 = 0x82437350;
    'dispatch: loop {
        match pc {
            0x82437350 => {
    //   block [0x82437350..0x82437360)
	// 82437350: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82437354: 396B09A0  addi r11, r11, 0x9a0
	ctx.r[11].s64 = ctx.r[11].s64 + 2464;
	// 82437358: 806B0014  lwz r3, 0x14(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8243735C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437360 size=80
    let mut pc: u32 = 0x82437360;
    'dispatch: loop {
        match pc {
            0x82437360 => {
    //   block [0x82437360..0x824373B0)
	// 82437360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437368: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8243736C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437370: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82437374: 38A00508  li r5, 0x508
	ctx.r[5].s64 = 1288;
	// 82437378: 3BEB09A0  addi r31, r11, 0x9a0
	ctx.r[31].s64 = ctx.r[11].s64 + 2464;
	// 8243737C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82437380: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437384: 480FDE4D  bl 0x825351d0
	ctx.lr = 0x82437388;
	sub_825351D0(ctx, base);
	// 82437388: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 8243738C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82437390: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82437394: 4BFFFFAD  bl 0x82437340
	ctx.lr = 0x82437398;
	sub_82437340(ctx, base);
	// 82437398: 48010829  bl 0x82447bc0
	ctx.lr = 0x8243739C;
	sub_82447BC0(ctx, base);
	// 8243739C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824373A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824373A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824373A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824373AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824373B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824373B0 size=296
    let mut pc: u32 = 0x824373B0;
    'dispatch: loop {
        match pc {
            0x824373B0 => {
    //   block [0x824373B0..0x82437404)
	// 824373B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824373B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824373B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824373BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824373C0: 7C882378  mr r8, r4
	ctx.r[8].u64 = ctx.r[4].u64;
	// 824373C4: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 824373C8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 824373CC: 4BFFFDFD  bl 0x824371c8
	ctx.lr = 0x824373D0;
	sub_824371C8(ctx, base);
	// 824373D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824373D4: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 824373D8: 419A002C  beq cr6, 0x82437404
	if ctx.cr[6].eq {
	pc = 0x82437404; continue 'dispatch;
	}
	// 824373DC: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 824373E0: 7D034378  mr r3, r8
	ctx.r[3].u64 = ctx.r[8].u64;
	// 824373E4: 4BFFFEAD  bl 0x82437290
	ctx.lr = 0x824373E8;
	sub_82437290(ctx, base);
	// 824373E8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824373EC: 419A0030  beq cr6, 0x8243741c
	if ctx.cr[6].eq {
	pc = 0x8243741C; continue 'dispatch;
	}
	// 824373F0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824373F4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824373F8: 38AB5AFC  addi r5, r11, 0x5afc
	ctx.r[5].s64 = ctx.r[11].s64 + 23292;
	// 824373FC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437400: 4BFFFF09  bl 0x82437308
	ctx.lr = 0x82437404;
	sub_82437308(ctx, base);
	pc = 0x82437404; continue 'dispatch;
            }
            0x82437404 => {
    //   block [0x82437404..0x8243741C)
	// 82437404: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437408: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243740C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437410: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437414: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82437418: 4E800020  blr
	return;
            }
            0x8243741C => {
    //   block [0x8243741C..0x82437468)
	// 8243741C: 7D054378  mr r5, r8
	ctx.r[5].u64 = ctx.r[8].u64;
	// 82437420: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 82437424: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437428: 4BFFFDE1  bl 0x82437208
	ctx.lr = 0x8243742C;
	sub_82437208(ctx, base);
	// 8243742C: 4800179D  bl 0x82438bc8
	ctx.lr = 0x82437430;
	sub_82438BC8(ctx, base);
	// 82437430: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82437434: 409A0034  bne cr6, 0x82437468
	if !ctx.cr[6].eq {
	pc = 0x82437468; continue 'dispatch;
	}
	// 82437438: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243743C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82437440: 38AB5ADC  addi r5, r11, 0x5adc
	ctx.r[5].s64 = ctx.r[11].s64 + 23260;
	// 82437444: 4BFFFEC5  bl 0x82437308
	ctx.lr = 0x82437448;
	sub_82437308(ctx, base);
	// 82437448: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243744C: 4BFFFE5D  bl 0x824372a8
	ctx.lr = 0x82437450;
	sub_824372A8(ctx, base);
	// 82437450: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437454: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437458: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243745C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82437464: 4E800020  blr
	return;
            }
            0x82437468 => {
    //   block [0x82437468..0x824374AC)
	// 82437468: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 8243746C: 4801091D  bl 0x82447d88
	ctx.lr = 0x82437470;
	sub_82447D88(ctx, base);
	// 82437470: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82437474: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82437478: 409A0034  bne cr6, 0x824374ac
	if !ctx.cr[6].eq {
	pc = 0x824374AC; continue 'dispatch;
	}
	// 8243747C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82437480: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82437484: 38AB5ABC  addi r5, r11, 0x5abc
	ctx.r[5].s64 = ctx.r[11].s64 + 23228;
	// 82437488: 4BFFFE81  bl 0x82437308
	ctx.lr = 0x8243748C;
	sub_82437308(ctx, base);
	// 8243748C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437490: 4BFFFE19  bl 0x824372a8
	ctx.lr = 0x82437494;
	sub_824372A8(ctx, base);
	// 82437494: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437498: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243749C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824374A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824374A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824374A8: 4E800020  blr
	return;
            }
            0x824374AC => {
    //   block [0x824374AC..0x824374D8)
	// 824374AC: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 824374B0: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824374B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824374B8: 814B09A0  lwz r10, 0x9a0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 824374BC: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 824374C0: 914B09A0  stw r10, 0x9a0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(2464 as u32), ctx.r[10].u32 ) };
	// 824374C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824374C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824374CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824374D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824374D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824374D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824374D8 size=104
    let mut pc: u32 = 0x824374D8;
    'dispatch: loop {
        match pc {
            0x824374D8 => {
    //   block [0x824374D8..0x8243752C)
	// 824374D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824374DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824374E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824374E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824374E8: 3FE0828A  lis r31, -0x7d76
	ctx.r[31].s64 = -2104885248;
	// 824374EC: 817F9F3C  lwz r11, -0x60c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-24772 as u32) ) } as u64;
	// 824374F0: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824374F4: 40980038  bge cr6, 0x8243752c
	if !ctx.cr[6].lt {
	pc = 0x8243752C; continue 'dispatch;
	}
	// 824374F8: 4BFFFCA9  bl 0x824371a0
	ctx.lr = 0x824374FC;
	sub_824371A0(ctx, base);
	// 824374FC: 3D60828A  lis r11, -0x7d76
	ctx.r[11].s64 = -2104885248;
	// 82437500: 906B9F38  stw r3, -0x60c8(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(-24776 as u32), ctx.r[3].u32 ) };
	// 82437504: 4BFFFE5D  bl 0x82437360
	ctx.lr = 0x82437508;
	sub_82437360(ctx, base);
	// 82437508: 480106D1  bl 0x82447bd8
	ctx.lr = 0x8243750C;
	sub_82447BD8(ctx, base);
	// 8243750C: 48001A5D  bl 0x82438f68
	ctx.lr = 0x82437510;
	sub_82438F68(ctx, base);
	// 82437510: 48010871  bl 0x82447d80
	ctx.lr = 0x82437514;
	sub_82447D80(ctx, base);
	// 82437514: 817F9F3C  lwz r11, -0x60c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(-24772 as u32) ) } as u64;
	// 82437518: 3D40828A  lis r10, -0x7d76
	ctx.r[10].s64 = -2104885248;
	// 8243751C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82437520: 917F9F3C  stw r11, -0x60c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(-24772 as u32), ctx.r[11].u32 ) };
	// 82437524: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82437528: 916A9F40  stw r11, -0x60c0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-24768 as u32), ctx.r[11].u32 ) };
	pc = 0x8243752C; continue 'dispatch;
            }
            0x8243752C => {
    //   block [0x8243752C..0x82437540)
	// 8243752C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437530: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437534: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437538: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243753C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437540 size=12
    let mut pc: u32 = 0x82437540;
    'dispatch: loop {
        match pc {
            0x82437540 => {
    //   block [0x82437540..0x8243754C)
	// 82437540: 90830008  stw r4, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[4].u32 ) };
	// 82437544: 90A3000C  stw r5, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u32 ) };
	// 82437548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437550 size=8
    let mut pc: u32 = 0x82437550;
    'dispatch: loop {
        match pc {
            0x82437550 => {
    //   block [0x82437550..0x82437558)
	// 82437550: 90830010  stw r4, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[4].u32 ) };
	// 82437554: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437558 size=148
    let mut pc: u32 = 0x82437558;
    'dispatch: loop {
        match pc {
            0x82437558 => {
    //   block [0x82437558..0x824375C0)
	// 82437558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243755C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437560: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82437564: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82437568: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243756C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82437570: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82437574: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82437578: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8243757C: 38A948DC  addi r5, r9, 0x48dc
	ctx.r[5].s64 = ctx.r[9].s64 + 18652;
	// 82437580: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82437584: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82437588: 83DF0024  lwz r30, 0x24(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243758C: 38895B28  addi r4, r9, 0x5b28
	ctx.r[4].s64 = ctx.r[9].s64 + 23336;
	// 82437590: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82437594: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82437598: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8243759C: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 824375A0: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 824375A4: 4BFF1A6D  bl 0x82429010
	ctx.lr = 0x824375A8;
	sub_82429010(ctx, base);
	// 824375A8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824375AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824375B0: 409A0010  bne cr6, 0x824375c0
	if !ctx.cr[6].eq {
	pc = 0x824375C0; continue 'dispatch;
	}
	// 824375B4: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824375B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824375BC: 4800000C  b 0x824375c8
	pc = 0x824375C8; continue 'dispatch;
            }
            0x824375C0 => {
    //   block [0x824375C0..0x824375C8)
	// 824375C0: 80A1005C  lwz r5, 0x5c(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 824375C4: 80810058  lwz r4, 0x58(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	pc = 0x824375C8; continue 'dispatch;
            }
            0x824375C8 => {
    //   block [0x824375C8..0x824375EC)
	// 824375C8: 48001659  bl 0x82438c20
	ctx.lr = 0x824375CC;
	sub_82438C20(ctx, base);
	// 824375CC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824375D0: 917F0014  stw r11, 0x14(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 824375D4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824375D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824375DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824375E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824375E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824375E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824375F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824375F0 size=28
    let mut pc: u32 = 0x824375F0;
    'dispatch: loop {
        match pc {
            0x824375F0 => {
    //   block [0x824375F0..0x8243760C)
	// 824375F0: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 824375F4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824375F8: 419A0014  beq cr6, 0x8243760c
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x8243760C);
		return;
	}
	// 824375FC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82437600: 91640000  stw r11, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82437604: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82437608: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437620(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437620 size=16
    let mut pc: u32 = 0x82437620;
    'dispatch: loop {
        match pc {
            0x82437620 => {
    //   block [0x82437620..0x82437630)
	// 82437620: 8084004C  lwz r4, 0x4c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82437624: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82437628: 48001610  b 0x82438c38
	sub_82438C38(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437630 size=8
    let mut pc: u32 = 0x82437630;
    'dispatch: loop {
        match pc {
            0x82437630 => {
    //   block [0x82437630..0x82437638)
	// 82437630: 80630064  lwz r3, 0x64(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(100 as u32) ) } as u64;
	// 82437634: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437638 size=40
    let mut pc: u32 = 0x82437638;
    'dispatch: loop {
        match pc {
            0x82437638 => {
    //   block [0x82437638..0x82437660)
	// 82437638: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 8243763C: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82437640: 7D6B21D6  mullw r11, r11, r4
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * ctx.r[4].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82437644: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82437648: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243764C: 7D444850  subf r10, r4, r9
	ctx.r[10].s64 = ctx.r[9].s64 - ctx.r[4].s64;
	// 82437650: 7D6B2A14  add r11, r11, r5
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[5].u64;
	// 82437654: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82437658: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8243765C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437660(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82437660 size=132
    let mut pc: u32 = 0x82437660;
    'dispatch: loop {
        match pc {
            0x82437660 => {
    //   block [0x82437660..0x824376D4)
	// 82437660: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437664: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437668: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243766C: 7CAB0E70  srawi r11, r5, 1
	ctx.xer.ca = (ctx.r[5].s32 < 0) && ((ctx.r[5].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[5].s32 >> 1) as i64;
	// 82437670: 7C681B78  mr r8, r3
	ctx.r[8].u64 = ctx.r[3].u64;
	// 82437674: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82437678: 7C8A0E70  srawi r10, r4, 1
	ctx.xer.ca = (ctx.r[4].s32 < 0) && ((ctx.r[4].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[4].s32 >> 1) as i64;
	// 8243767C: 5567083C  slwi r7, r11, 1
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82437680: 7D6A0194  addze r11, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82437684: 38680004  addi r3, r8, 4
	ctx.r[3].s64 = ctx.r[8].s64 + 4;
	// 82437688: 5566083C  slwi r6, r11, 1
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 8243768C: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 82437690: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82437694: 4BFFFFA5  bl 0x82437638
	ctx.lr = 0x82437698;
	sub_82437638(ctx, base);
	// 82437698: 7CCB0E70  srawi r11, r6, 1
	ctx.xer.ca = (ctx.r[6].s32 < 0) && ((ctx.r[6].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[6].s32 >> 1) as i64;
	// 8243769C: 38680014  addi r3, r8, 0x14
	ctx.r[3].s64 = ctx.r[8].s64 + 20;
	// 824376A0: 7CAB0194  addze r5, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[5].s64 = tmp.s64;
	// 824376A4: 7CEB0E70  srawi r11, r7, 1
	ctx.xer.ca = (ctx.r[7].s32 < 0) && ((ctx.r[7].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[7].s32 >> 1) as i64;
	// 824376A8: 7C8B0194  addze r4, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[4].s64 = tmp.s64;
	// 824376AC: 4BFFFF8D  bl 0x82437638
	ctx.lr = 0x824376B0;
	sub_82437638(ctx, base);
	// 824376B0: 38680024  addi r3, r8, 0x24
	ctx.r[3].s64 = ctx.r[8].s64 + 36;
	// 824376B4: 4BFFFF85  bl 0x82437638
	ctx.lr = 0x824376B8;
	sub_82437638(ctx, base);
	// 824376B8: 81680044  lwz r11, 0x44(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(68 as u32) ) } as u64;
	// 824376BC: 38680044  addi r3, r8, 0x44
	ctx.r[3].s64 = ctx.r[8].s64 + 68;
	// 824376C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824376C4: 419A0010  beq cr6, 0x824376d4
	if ctx.cr[6].eq {
	pc = 0x824376D4; continue 'dispatch;
	}
	// 824376C8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 824376CC: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 824376D0: 4BFFFF69  bl 0x82437638
	ctx.lr = 0x824376D4;
	sub_82437638(ctx, base);
	pc = 0x824376D4; continue 'dispatch;
            }
            0x824376D4 => {
    //   block [0x824376D4..0x824376E4)
	// 824376D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824376D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824376DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824376E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824376E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824376E8 size=8
    let mut pc: u32 = 0x824376E8;
    'dispatch: loop {
        match pc {
            0x824376E8 => {
    //   block [0x824376E8..0x824376F0)
	// 824376E8: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824376EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824376F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824376F0 size=20
    let mut pc: u32 = 0x824376F0;
    'dispatch: loop {
        match pc {
            0x824376F0 => {
    //   block [0x824376F0..0x82437704)
	// 824376F0: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 824376F4: 90A30004  stw r5, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 824376F8: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 824376FC: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82437700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437708 size=8
    let mut pc: u32 = 0x82437708;
    'dispatch: loop {
        match pc {
            0x82437708 => {
    //   block [0x82437708..0x82437710)
	// 82437708: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243770C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437720 size=168
    let mut pc: u32 = 0x82437720;
    'dispatch: loop {
        match pc {
            0x82437720 => {
    //   block [0x82437720..0x82437760)
	// 82437720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437724: 480FD995  bl 0x825350b8
	ctx.lr = 0x82437728;
	sub_82535080(ctx, base);
	// 82437728: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243772C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82437730: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82437734: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82437738: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8243773C: 48010105  bl 0x82447840
	ctx.lr = 0x82437740;
	sub_82447840(ctx, base);
	// 82437740: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82437744: 419A001C  beq cr6, 0x82437760
	if ctx.cr[6].eq {
	pc = 0x82437760; continue 'dispatch;
	}
	// 82437748: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243774C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437750: 60840191  ori r4, r4, 0x191
	ctx.r[4].u64 = ctx.r[4].u64 | 401;
	// 82437754: 480101B5  bl 0x82447908
	ctx.lr = 0x82437758;
	sub_82447908(ctx, base);
	// 82437758: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243775C: 480FD9AC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82437760 => {
    //   block [0x82437760..0x82437788)
	// 82437760: 815F21E8  lwz r10, 0x21e8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8680 as u32) ) } as u64;
	// 82437764: 813F21E0  lwz r9, 0x21e0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8672 as u32) ) } as u64;
	// 82437768: 2F0A0008  cmpwi cr6, r10, 8
	ctx.cr[6].compare_i32(ctx.r[10].s32, 8, &mut ctx.xer);
	// 8243776C: 409A001C  bne cr6, 0x82437788
	if !ctx.cr[6].eq {
	pc = 0x82437788; continue 'dispatch;
	}
	// 82437770: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82437774: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437778: 60840602  ori r4, r4, 0x602
	ctx.r[4].u64 = ctx.r[4].u64 | 1538;
	// 8243777C: 4801018D  bl 0x82447908
	ctx.lr = 0x82437780;
	sub_82447908(ctx, base);
	// 82437780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82437784: 480FD984  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82437788 => {
    //   block [0x82437788..0x824377C8)
	// 82437788: 57CB2036  slwi r11, r30, 4
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8243778C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82437790: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82437794: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82437798: 386B0004  addi r3, r11, 4
	ctx.r[3].s64 = ctx.r[11].s64 + 4;
	// 8243779C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 824377A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824377A4: 4BFFFF4D  bl 0x824376f0
	ctx.lr = 0x824377A8;
	sub_824376F0(ctx, base);
	// 824377A8: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 824377AC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824377B0: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 824377B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824377B8: 48010831  bl 0x82447fe8
	ctx.lr = 0x824377BC;
	sub_82447FE8(ctx, base);
	// 824377BC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824377C0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824377C4: 480FD944  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824377C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824377C8 size=108
    let mut pc: u32 = 0x824377C8;
    'dispatch: loop {
        match pc {
            0x824377C8 => {
    //   block [0x824377C8..0x82437820)
	// 824377C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824377CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824377D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824377D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824377D8: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 824377DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824377E0: 48011789  bl 0x82448f68
	ctx.lr = 0x824377E4;
	sub_82448F68(ctx, base);
	// 824377E4: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824377E8: 419A0038  beq cr6, 0x82437820
	if ctx.cr[6].eq {
	pc = 0x82437820; continue 'dispatch;
	}
	// 824377EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824377F0: 809F21E8  lwz r4, 0x21e8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8680 as u32) ) } as u64;
	// 824377F4: 48010C0D  bl 0x82448400
	ctx.lr = 0x824377F8;
	sub_82448400(ctx, base);
	// 824377F8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824377FC: 409A0024  bne cr6, 0x82437820
	if !ctx.cr[6].eq {
	pc = 0x82437820; continue 'dispatch;
	}
	// 82437800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437804: 4BFFFEE5  bl 0x824376e8
	ctx.lr = 0x82437808;
	sub_824376E8(ctx, base);
	// 82437808: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243780C: 419A0014  beq cr6, 0x82437820
	if ctx.cr[6].eq {
	pc = 0x82437820; continue 'dispatch;
	}
	// 82437810: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82437814: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82437818: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243781C: 4801173D  bl 0x82448f58
	ctx.lr = 0x82437820;
	sub_82448F58(ctx, base);
	pc = 0x82437820; continue 'dispatch;
            }
            0x82437820 => {
    //   block [0x82437820..0x82437834)
	// 82437820: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437824: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437828: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243782C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82437830: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437838 size=108
    let mut pc: u32 = 0x82437838;
    'dispatch: loop {
        match pc {
            0x82437838 => {
    //   block [0x82437838..0x82437890)
	// 82437838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243783C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437840: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82437844: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437848: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 8243784C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82437850: 480116F9  bl 0x82448f48
	ctx.lr = 0x82437854;
	sub_82448F48(ctx, base);
	// 82437854: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82437858: 419A0038  beq cr6, 0x82437890
	if ctx.cr[6].eq {
	pc = 0x82437890; continue 'dispatch;
	}
	// 8243785C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437860: 809F21E8  lwz r4, 0x21e8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8680 as u32) ) } as u64;
	// 82437864: 48010B65  bl 0x824483c8
	ctx.lr = 0x82437868;
	sub_824483C8(ctx, base);
	// 82437868: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243786C: 409A0024  bne cr6, 0x82437890
	if !ctx.cr[6].eq {
	pc = 0x82437890; continue 'dispatch;
	}
	// 82437870: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437874: 4BFFFE75  bl 0x824376e8
	ctx.lr = 0x82437878;
	sub_824376E8(ctx, base);
	// 82437878: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243787C: 419A0014  beq cr6, 0x82437890
	if ctx.cr[6].eq {
	pc = 0x82437890; continue 'dispatch;
	}
	// 82437880: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82437884: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82437888: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243788C: 480116AD  bl 0x82448f38
	ctx.lr = 0x82437890;
	sub_82448F38(ctx, base);
	pc = 0x82437890; continue 'dispatch;
            }
            0x82437890 => {
    //   block [0x82437890..0x824378A4)
	// 82437890: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437894: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437898: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243789C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824378A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824378A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824378A8 size=20
    let mut pc: u32 = 0x824378A8;
    'dispatch: loop {
        match pc {
            0x824378A8 => {
    //   block [0x824378A8..0x824378BC)
	// 824378A8: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 824378AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824378B0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 824378B4: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824378B8: 4BFFFE38  b 0x824376f0
	sub_824376F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824378C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824378C0 size=60
    let mut pc: u32 = 0x824378C0;
    'dispatch: loop {
        match pc {
            0x824378C0 => {
    //   block [0x824378C0..0x824378FC)
	// 824378C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824378C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824378C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824378CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824378D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824378D4: 4BFFFEF5  bl 0x824377c8
	ctx.lr = 0x824378D8;
	sub_824377C8(ctx, base);
	// 824378D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824378DC: 4BFFFF5D  bl 0x82437838
	ctx.lr = 0x824378E0;
	sub_82437838(ctx, base);
	// 824378E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824378E4: 4BFFFE25  bl 0x82437708
	ctx.lr = 0x824378E8;
	sub_82437708(ctx, base);
	// 824378E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824378EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824378F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824378F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824378F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437900 size=84
    let mut pc: u32 = 0x82437900;
    'dispatch: loop {
        match pc {
            0x82437900 => {
    //   block [0x82437900..0x82437920)
	// 82437900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437904: 480FD7B5  bl 0x825350b8
	ctx.lr = 0x82437908;
	sub_82535080(ctx, base);
	// 82437908: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243790C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82437910: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82437914: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82437918: 3BC40004  addi r30, r4, 4
	ctx.r[30].s64 = ctx.r[4].s64 + 4;
	// 8243791C: 93E40000  stw r31, 0(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	pc = 0x82437920; continue 'dispatch;
            }
            0x82437920 => {
    //   block [0x82437920..0x82437954)
	// 82437920: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82437924: 4BFFFF85  bl 0x824378a8
	ctx.lr = 0x82437928;
	sub_824378A8(ctx, base);
	// 82437928: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8243792C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82437930: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82437934: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82437938: 480106B1  bl 0x82447fe8
	ctx.lr = 0x8243793C;
	sub_82447FE8(ctx, base);
	// 8243793C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82437940: 3BDE0010  addi r30, r30, 0x10
	ctx.r[30].s64 = ctx.r[30].s64 + 16;
	// 82437944: 2F1F0003  cmpwi cr6, r31, 3
	ctx.cr[6].compare_i32(ctx.r[31].s32, 3, &mut ctx.xer);
	// 82437948: 4198FFD8  blt cr6, 0x82437920
	if ctx.cr[6].lt {
	pc = 0x82437920; continue 'dispatch;
	}
	// 8243794C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82437950: 480FD7B8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437958(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437958 size=48
    let mut pc: u32 = 0x82437958;
    'dispatch: loop {
        match pc {
            0x82437958 => {
    //   block [0x82437958..0x82437988)
	// 82437958: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243795C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437960: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437964: 38832620  addi r4, r3, 0x2620
	ctx.r[4].s64 = ctx.r[3].s64 + 9760;
	// 82437968: 80A321E8  lwz r5, 0x21e8(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8680 as u32) ) } as u64;
	// 8243796C: 908321E0  stw r4, 0x21e0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8672 as u32), ctx.r[4].u32 ) };
	// 82437970: 4BFFFF91  bl 0x82437900
	ctx.lr = 0x82437974;
	sub_82437900(ctx, base);
	// 82437974: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437978: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243797C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437984: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437988(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437988 size=24
    let mut pc: u32 = 0x82437988;
    'dispatch: loop {
        match pc {
            0x82437988 => {
    //   block [0x82437988..0x824379A0)
	// 82437988: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8243798C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82437990: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82437994: 90E30044  stw r7, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[7].u32 ) };
	// 82437998: 91030048  stw r8, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[8].u32 ) };
	// 8243799C: 4BFFFCC4  b 0x82437660
	sub_82437660(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824379A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824379A0 size=40
    let mut pc: u32 = 0x824379A0;
    'dispatch: loop {
        match pc {
            0x824379A0 => {
    //   block [0x824379A0..0x824379C8)
	// 824379A0: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 824379A4: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 824379A8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 824379AC: 81230004  lwz r9, 4(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824379B0: 7D4A59D6  mullw r10, r10, r11
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * ctx.r[11].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 824379B4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 824379B8: 7D6B00D0  neg r11, r11
	ctx.r[11].s64 = -ctx.r[11].s64;
	// 824379BC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 824379C0: 91630010  stw r11, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 824379C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824379C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824379C8 size=428
    let mut pc: u32 = 0x824379C8;
    'dispatch: loop {
        match pc {
            0x824379C8 => {
    //   block [0x824379C8..0x82437B0C)
	// 824379C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824379CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824379D0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824379D4: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 824379D8: 2F0B0061  cmpwi cr6, r11, 0x61
	ctx.cr[6].compare_i32(ctx.r[11].s32, 97, &mut ctx.xer);
	// 824379DC: 41990130  bgt cr6, 0x82437b0c
	if ctx.cr[6].gt {
	pc = 0x82437B0C; continue 'dispatch;
	}
	// 824379E0: 419A0180  beq cr6, 0x82437b60
	if ctx.cr[6].eq {
	pc = 0x82437B60; continue 'dispatch;
	}
	// 824379E4: 396BFFEF  addi r11, r11, -0x11
	ctx.r[11].s64 = ctx.r[11].s64 + -17;
	// 824379E8: 2B0B0040  cmplwi cr6, r11, 0x40
	ctx.cr[6].compare_u32(ctx.r[11].u32, 64 as u32, &mut ctx.xer);
	// 824379EC: 41990160  bgt cr6, 0x82437b4c
	if ctx.cr[6].gt {
	pc = 0x82437B4C; continue 'dispatch;
	}
	// 824379F0: 3D808243  lis r12, -0x7dbd
	ctx.r[12].s64 = -2109538304;
	// 824379F4: 398C7A08  addi r12, r12, 0x7a08
	ctx.r[12].s64 = ctx.r[12].s64 + 31240;
	// 824379F8: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 824379FC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82437A00: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82437A04: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82437B60; continue 'dispatch;
		},
		1 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		2 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		3 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		4 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		5 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		6 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		7 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		8 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		9 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		10 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		11 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		12 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		13 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		14 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		15 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		16 => {
	pc = 0x82437B28; continue 'dispatch;
		},
		17 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		18 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		19 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		20 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		21 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		22 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		23 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		24 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		25 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		26 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		27 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		28 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		29 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		30 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		31 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		32 => {
	pc = 0x82437B60; continue 'dispatch;
		},
		33 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		34 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		35 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		36 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		37 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		38 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		39 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		40 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		41 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		42 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		43 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		44 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		45 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		46 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		47 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		48 => {
	pc = 0x82437B60; continue 'dispatch;
		},
		49 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		50 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		51 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		52 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		53 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		54 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		55 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		56 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		57 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		58 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		59 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		60 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		61 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		62 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		63 => {
	pc = 0x82437B4C; continue 'dispatch;
		},
		64 => {
	pc = 0x82437B60; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82437A08: 82437B60  lwz r18, 0x7b60(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31584 as u32) ) } as u64;
	// 82437A0C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A10: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A14: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A18: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A1C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A20: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A24: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A28: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A2C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A30: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A34: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A38: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A3C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A40: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A44: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A48: 82437B28  lwz r18, 0x7b28(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31528 as u32) ) } as u64;
	// 82437A4C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A50: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A54: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A58: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A5C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A60: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A64: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A68: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A6C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A70: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A74: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A78: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A7C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A80: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A84: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A88: 82437B60  lwz r18, 0x7b60(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31584 as u32) ) } as u64;
	// 82437A8C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A90: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A94: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A98: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437A9C: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AA0: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AA4: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AA8: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AAC: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AB0: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AB4: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AB8: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437ABC: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AC0: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AC4: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AC8: 82437B60  lwz r18, 0x7b60(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31584 as u32) ) } as u64;
	// 82437ACC: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AD0: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AD4: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AD8: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437ADC: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AE0: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AE4: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AE8: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AEC: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AF0: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AF4: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AF8: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437AFC: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437B00: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437B04: 82437B4C  lwz r18, 0x7b4c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31564 as u32) ) } as u64;
	// 82437B08: 82437B60  lwz r18, 0x7b60(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31584 as u32) ) } as u64;
            }
            0x82437B0C => {
    //   block [0x82437B0C..0x82437B28)
	// 82437B0C: 2F0B0101  cmpwi cr6, r11, 0x101
	ctx.cr[6].compare_i32(ctx.r[11].s32, 257, &mut ctx.xer);
	// 82437B10: 4199002C  bgt cr6, 0x82437b3c
	if ctx.cr[6].gt {
	pc = 0x82437B3C; continue 'dispatch;
	}
	// 82437B14: 419A0014  beq cr6, 0x82437b28
	if ctx.cr[6].eq {
	pc = 0x82437B28; continue 'dispatch;
	}
	// 82437B18: 2F0B0071  cmpwi cr6, r11, 0x71
	ctx.cr[6].compare_i32(ctx.r[11].s32, 113, &mut ctx.xer);
	// 82437B1C: 419A0044  beq cr6, 0x82437b60
	if ctx.cr[6].eq {
	pc = 0x82437B60; continue 'dispatch;
	}
	// 82437B20: 2F0B00F1  cmpwi cr6, r11, 0xf1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 241, &mut ctx.xer);
	// 82437B24: 48000024  b 0x82437b48
	pc = 0x82437B48; continue 'dispatch;
            }
            0x82437B28 => {
    //   block [0x82437B28..0x82437B3C)
	// 82437B28: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82437B2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437B30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437B34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437B38: 4E800020  blr
	return;
            }
            0x82437B3C => {
    //   block [0x82437B3C..0x82437B48)
	// 82437B3C: 2F0B0111  cmpwi cr6, r11, 0x111
	ctx.cr[6].compare_i32(ctx.r[11].s32, 273, &mut ctx.xer);
	// 82437B40: 419A0020  beq cr6, 0x82437b60
	if ctx.cr[6].eq {
	pc = 0x82437B60; continue 'dispatch;
	}
	// 82437B44: 2F0B1001  cmpwi cr6, r11, 0x1001
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4097, &mut ctx.xer);
	pc = 0x82437B48; continue 'dispatch;
            }
            0x82437B48 => {
    //   block [0x82437B48..0x82437B4C)
	// 82437B48: 419A0018  beq cr6, 0x82437b60
	if ctx.cr[6].eq {
	pc = 0x82437B60; continue 'dispatch;
	}
	pc = 0x82437B4C; continue 'dispatch;
            }
            0x82437B4C => {
    //   block [0x82437B4C..0x82437B60)
	// 82437B4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82437B50: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82437B54: 38AB5B68  addi r5, r11, 0x5b68
	ctx.r[5].s64 = ctx.r[11].s64 + 23400;
	// 82437B58: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437B5C: 4BFFF7AD  bl 0x82437308
	ctx.lr = 0x82437B60;
	sub_82437308(ctx, base);
	pc = 0x82437B60; continue 'dispatch;
            }
            0x82437B60 => {
    //   block [0x82437B60..0x82437B74)
	// 82437B60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437B64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437B68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437B6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437B70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437B78 size=20
    let mut pc: u32 = 0x82437B78;
    'dispatch: loop {
        match pc {
            0x82437B78 => {
    //   block [0x82437B78..0x82437B8C)
	// 82437B78: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82437B7C: 2F0B0051  cmpwi cr6, r11, 0x51
	ctx.cr[6].compare_i32(ctx.r[11].s32, 81, &mut ctx.xer);
	// 82437B80: 409A000C  bne cr6, 0x82437b8c
	if !ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82437B8C);
		return;
	}
	// 82437B84: 38600004  li r3, 4
	ctx.r[3].s64 = 4;
	// 82437B88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82437BB0 size=228
    let mut pc: u32 = 0x82437BB0;
    'dispatch: loop {
        match pc {
            0x82437BB0 => {
    //   block [0x82437BB0..0x82437BE8)
	// 82437BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437BB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82437BB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82437BBC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437BC0: 81630034  lwz r11, 0x34(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(52 as u32) ) } as u64;
	// 82437BC4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82437BC8: 2F0B0064  cmpwi cr6, r11, 0x64
	ctx.cr[6].compare_i32(ctx.r[11].s32, 100, &mut ctx.xer);
	// 82437BCC: 409A001C  bne cr6, 0x82437be8
	if !ctx.cr[6].eq {
	pc = 0x82437BE8; continue 'dispatch;
	}
	// 82437BD0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82437BD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437BD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437BDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437BE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82437BE4: 4E800020  blr
	return;
            }
            0x82437BE8 => {
    //   block [0x82437BE8..0x82437C68)
	// 82437BE8: 7F0B2000  cmpw cr6, r11, r4
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[4].s32, &mut ctx.xer);
	// 82437BEC: 409A0090  bne cr6, 0x82437c7c
	if !ctx.cr[6].eq {
	pc = 0x82437C7C; continue 'dispatch;
	}
	// 82437BF0: 3964FFFF  addi r11, r4, -1
	ctx.r[11].s64 = ctx.r[4].s64 + -1;
	// 82437BF4: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 82437BF8: 41990084  bgt cr6, 0x82437c7c
	if ctx.cr[6].gt {
	pc = 0x82437C7C; continue 'dispatch;
	}
	// 82437BFC: 3D808243  lis r12, -0x7dbd
	ctx.r[12].s64 = -2109538304;
	// 82437C00: 398C7C14  addi r12, r12, 0x7c14
	ctx.r[12].s64 = ctx.r[12].s64 + 31764;
	// 82437C04: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82437C08: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82437C0C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82437C10: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82437C78; continue 'dispatch;
		},
		1 => {
	pc = 0x82437C68; continue 'dispatch;
		},
		2 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		3 => {
	pc = 0x82437C78; continue 'dispatch;
		},
		4 => {
	pc = 0x82437C78; continue 'dispatch;
		},
		5 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		6 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		7 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		8 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		9 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		10 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		11 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		12 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		13 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		14 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		15 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		16 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		17 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		18 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		19 => {
	pc = 0x82437C7C; continue 'dispatch;
		},
		20 => {
	pc = 0x82437C78; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82437C14: 82437C78  lwz r18, 0x7c78(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31864 as u32) ) } as u64;
	// 82437C18: 82437C68  lwz r18, 0x7c68(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31848 as u32) ) } as u64;
	// 82437C1C: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C20: 82437C78  lwz r18, 0x7c78(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31864 as u32) ) } as u64;
	// 82437C24: 82437C78  lwz r18, 0x7c78(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31864 as u32) ) } as u64;
	// 82437C28: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C2C: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C30: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C34: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C38: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C3C: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C40: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C44: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C48: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C4C: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C50: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C54: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C58: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C5C: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C60: 82437C7C  lwz r18, 0x7c7c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31868 as u32) ) } as u64;
	// 82437C64: 82437C78  lwz r18, 0x7c78(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(31864 as u32) ) } as u64;
            }
            0x82437C68 => {
    //   block [0x82437C68..0x82437C78)
	// 82437C68: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82437C6C: 4BFF67BD  bl 0x8242e428
	ctx.lr = 0x82437C70;
	sub_8242E428(ctx, base);
	// 82437C70: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82437C74: 419A0008  beq cr6, 0x82437c7c
	if ctx.cr[6].eq {
	pc = 0x82437C7C; continue 'dispatch;
	}
	pc = 0x82437C78; continue 'dispatch;
            }
            0x82437C78 => {
    //   block [0x82437C78..0x82437C7C)
	// 82437C78: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x82437C7C; continue 'dispatch;
            }
            0x82437C7C => {
    //   block [0x82437C7C..0x82437C94)
	// 82437C7C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82437C80: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82437C84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82437C88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82437C8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82437C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82437C98 size=160
    let mut pc: u32 = 0x82437C98;
    'dispatch: loop {
        match pc {
            0x82437C98 => {
    //   block [0x82437C98..0x82437CB8)
	// 82437C98: 80A30038  lwz r5, 0x38(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82437C9C: 8084004C  lwz r4, 0x4c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82437CA0: 80630024  lwz r3, 0x24(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(36 as u32) ) } as u64;
	// 82437CA4: 4800136C  b 0x82439010
	sub_82439010(ctx, base);
	return;
	// 82437CA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82437CAC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82437CB0: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82437CB4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
            }
            0x82437CB8 => {
    //   block [0x82437CB8..0x82437CD8)
	// 82437CB8: 994B0000  stb r10, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 82437CBC: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82437CC0: 4200FFF8  bdnz 0x82437cb8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82437CB8; continue 'dispatch;
	}
	// 82437CC4: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82437CC8: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82437CCC: 2123FFF0  subfic r9, r3, -0x10
	ctx.xer.ca = ctx.r[3].u32 <= -16 as u32;
	ctx.r[9].s64 = (-16 as i64) - ctx.r[3].s64;
	// 82437CD0: 394000DC  li r10, 0xdc
	ctx.r[10].s64 = 220;
	// 82437CD4: C0085B98  lfs f0, 0x5b98(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(23448 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x82437CD8; continue 'dispatch;
            }
            0x82437CD8 => {
    //   block [0x82437CD8..0x82437D24)
	// 82437CD8: 7D095A14  add r8, r9, r11
	ctx.r[8].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82437CDC: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82437CE0: 7D0807B4  extsw r8, r8
	ctx.r[8].s64 = ctx.r[8].s32 as i64;
	// 82437CE4: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82437CE8: F901FFF0  std r8, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[8].u64 ) };
	// 82437CEC: C9A1FFF0  lfd f13, -0x10(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82437CF0: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 82437CF4: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 82437CF8: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 82437CFC: FDA06E5E  fctidz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[13].f64.trunc() as i64 };
	// 82437D00: D9A1FFF8  stfd f13, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.f[13].u64 ) };
	// 82437D04: 8901FFFF  lbz r8, -1(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82437D08: 990B0000  stb r8, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82437D0C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82437D10: 409AFFC8  bne cr6, 0x82437cd8
	if !ctx.cr[6].eq {
	pc = 0x82437CD8; continue 'dispatch;
	}
	// 82437D14: 396300EC  addi r11, r3, 0xec
	ctx.r[11].s64 = ctx.r[3].s64 + 236;
	// 82437D18: 392000FF  li r9, 0xff
	ctx.r[9].s64 = 255;
	// 82437D1C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82437D20: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x82437D24; continue 'dispatch;
            }
            0x82437D24 => {
    //   block [0x82437D24..0x82437D38)
	// 82437D24: 992B0000  stb r9, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u8 ) };
	// 82437D28: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82437D2C: 4200FFF8  bdnz 0x82437d24
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82437D24; continue 'dispatch;
	}
	// 82437D30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D38 size=16
    let mut pc: u32 = 0x82437D38;
    'dispatch: loop {
        match pc {
            0x82437D38 => {
    //   block [0x82437D38..0x82437D48)
	// 82437D38: 80A30038  lwz r5, 0x38(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82437D3C: 8084004C  lwz r4, 0x4c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82437D40: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82437D44: 4800FF84  b 0x82447cc8
	sub_82447CC8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D48 size=16
    let mut pc: u32 = 0x82437D48;
    'dispatch: loop {
        match pc {
            0x82437D48 => {
    //   block [0x82437D48..0x82437D58)
	// 82437D48: 80A30038  lwz r5, 0x38(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82437D4C: 8084004C  lwz r4, 0x4c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82437D50: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82437D54: 4800FFCC  b 0x82447d20
	sub_82447D20(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D58 size=16
    let mut pc: u32 = 0x82437D58;
    'dispatch: loop {
        match pc {
            0x82437D58 => {
    //   block [0x82437D58..0x82437D68)
	// 82437D58: 80A30038  lwz r5, 0x38(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82437D5C: 8084004C  lwz r4, 0x4c(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(76 as u32) ) } as u64;
	// 82437D60: 80630030  lwz r3, 0x30(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82437D64: 4800FFEC  b 0x82447d50
	sub_82447D50(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D68 size=24
    let mut pc: u32 = 0x82437D68;
    'dispatch: loop {
        match pc {
            0x82437D68 => {
    //   block [0x82437D68..0x82437D80)
	// 82437D68: 81630070  lwz r11, 0x70(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(112 as u32) ) } as u64;
	// 82437D6C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82437D70: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82437D74: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 82437D78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82437D7C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D88 size=8
    let mut pc: u32 = 0x82437D88;
    'dispatch: loop {
        match pc {
            0x82437D88 => {
    //   block [0x82437D88..0x82437D90)
	// 82437D88: 90830068  stw r4, 0x68(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(104 as u32), ctx.r[4].u32 ) };
	// 82437D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D90 size=8
    let mut pc: u32 = 0x82437D90;
    'dispatch: loop {
        match pc {
            0x82437D90 => {
    //   block [0x82437D90..0x82437D98)
	// 82437D90: 9083006C  stw r4, 0x6c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(108 as u32), ctx.r[4].u32 ) };
	// 82437D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437D98 size=12
    let mut pc: u32 = 0x82437D98;
    'dispatch: loop {
        match pc {
            0x82437D98 => {
    //   block [0x82437D98..0x82437DA4)
	// 82437D98: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82437D9C: 908B0018  stw r4, 0x18(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(24 as u32), ctx.r[4].u32 ) };
	// 82437DA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437DA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437DA8 size=12
    let mut pc: u32 = 0x82437DA8;
    'dispatch: loop {
        match pc {
            0x82437DA8 => {
    //   block [0x82437DA8..0x82437DB4)
	// 82437DA8: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82437DAC: 908B0020  stw r4, 0x20(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(32 as u32), ctx.r[4].u32 ) };
	// 82437DB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437DB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437DB8 size=12
    let mut pc: u32 = 0x82437DB8;
    'dispatch: loop {
        match pc {
            0x82437DB8 => {
    //   block [0x82437DB8..0x82437DC4)
	// 82437DB8: 81630030  lwz r11, 0x30(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82437DBC: 908B001C  stw r4, 0x1c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(28 as u32), ctx.r[4].u32 ) };
	// 82437DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437DC8 size=8
    let mut pc: u32 = 0x82437DC8;
    'dispatch: loop {
        match pc {
            0x82437DC8 => {
    //   block [0x82437DC8..0x82437DD0)
	// 82437DC8: 90830070  stw r4, 0x70(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(112 as u32), ctx.r[4].u32 ) };
	// 82437DCC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437DD0 size=40
    let mut pc: u32 = 0x82437DD0;
    'dispatch: loop {
        match pc {
            0x82437DD0 => {
    //   block [0x82437DD0..0x82437DF8)
	// 82437DD0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82437DD4: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82437DD8: 419A009C  beq cr6, 0x82437e74
	if ctx.cr[6].eq {
		sub_82437E74(ctx, base);
		return;
	}
	// 82437DDC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82437DE0: 419A0094  beq cr6, 0x82437e74
	if ctx.cr[6].eq {
		sub_82437E74(ctx, base);
		return;
	}
	// 82437DE4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82437DE8: 419A0010  beq cr6, 0x82437df8
	if ctx.cr[6].eq {
		sub_82437DF8(ctx, base);
		return;
	}
	// 82437DEC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82437DF0: 38AB5B9C  addi r5, r11, 0x5b9c
	ctx.r[5].s64 = ctx.r[11].s64 + 23452;
	// 82437DF4: 4BFFF514  b 0x82437308
	sub_82437308(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437DF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82437DF8 size=124
    let mut pc: u32 = 0x82437DF8;
    'dispatch: loop {
        match pc {
            0x82437DF8 => {
    //   block [0x82437DF8..0x82437E74)
	// 82437DF8: 39600003  li r11, 3
	ctx.r[11].s64 = 3;
	// 82437DFC: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82437E00: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82437E04: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82437E08: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 82437E0C: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82437E10: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82437E14: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82437E18: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82437E1C: 91650010  stw r11, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82437E20: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82437E24: 91650014  stw r11, 0x14(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82437E28: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 82437E2C: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82437E30: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82437E34: 91650018  stw r11, 0x18(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82437E38: 8164001C  lwz r11, 0x1c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) } as u64;
	// 82437E3C: 9165001C  stw r11, 0x1c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82437E40: 81640018  lwz r11, 0x18(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) } as u64;
	// 82437E44: 91650020  stw r11, 0x20(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82437E48: 81640024  lwz r11, 0x24(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) } as u64;
	// 82437E4C: 91650024  stw r11, 0x24(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(36 as u32), ctx.r[11].u32 ) };
	// 82437E50: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 82437E54: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82437E58: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82437E5C: 91650028  stw r11, 0x28(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(40 as u32), ctx.r[11].u32 ) };
	// 82437E60: 8164002C  lwz r11, 0x2c(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(44 as u32) ) } as u64;
	// 82437E64: 9165002C  stw r11, 0x2c(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82437E68: 81640028  lwz r11, 0x28(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(40 as u32) ) } as u64;
	// 82437E6C: 91650030  stw r11, 0x30(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 82437E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437E74(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82437E74 size=44
    let mut pc: u32 = 0x82437E74;
    'dispatch: loop {
        match pc {
            0x82437E74 => {
    //   block [0x82437E74..0x82437EA0)
	// 82437E74: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82437E78: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82437E7C: 81640004  lwz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82437E80: 91650004  stw r11, 4(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82437E84: 81640044  lwz r11, 0x44(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(68 as u32) ) } as u64;
	// 82437E88: 91650008  stw r11, 8(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82437E8C: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82437E90: 9165000C  stw r11, 0xc(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82437E94: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82437E98: 91650010  stw r11, 0x10(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82437E9C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82437EA0 size=280
    let mut pc: u32 = 0x82437EA0;
    'dispatch: loop {
        match pc {
            0x82437EA0 => {
    //   block [0x82437EA0..0x82437F58)
	// 82437EA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437EA4: 480FD219  bl 0x825350bc
	ctx.lr = 0x82437EA8;
	sub_82535080(ctx, base);
	// 82437EA8: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437EAC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82437EB0: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82437EB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82437EB8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82437EBC: 4BFFFF15  bl 0x82437dd0
	ctx.lr = 0x82437EC0;
	sub_82437DD0(ctx, base);
	// 82437EC0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82437EC4: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82437EC8: 40990098  ble cr6, 0x82437f60
	if !ctx.cr[6].gt {
	pc = 0x82437F60; continue 'dispatch;
	}
	// 82437ECC: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82437ED0: 409900A4  ble cr6, 0x82437f74
	if !ctx.cr[6].gt {
	pc = 0x82437F74; continue 'dispatch;
	}
	// 82437ED4: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82437ED8: 409A0088  bne cr6, 0x82437f60
	if !ctx.cr[6].eq {
	pc = 0x82437F60; continue 'dispatch;
	}
	// 82437EDC: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82437EE0: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82437EE4: 81010054  lwz r8, 0x54(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82437EE8: 7D6A59D6  mullw r11, r10, r11
	ctx.r[11].s32 = ((ctx.r[10].s32 as i64 * ctx.r[11].s32 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 82437EEC: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82437EF0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82437EF4: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82437EF8: 914100E4  stw r10, 0xe4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(228 as u32), ctx.r[10].u32 ) };
	// 82437EFC: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82437F00: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82437F04: 914100E8  stw r10, 0xe8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(232 as u32), ctx.r[10].u32 ) };
	// 82437F08: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	// 82437F0C: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82437F10: 914100EC  stw r10, 0xec(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), ctx.r[10].u32 ) };
	// 82437F14: 7D6A0E70  srawi r10, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[10].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82437F18: 7D685A14  add r11, r8, r11
	ctx.r[11].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82437F1C: 7D4A0194  addze r10, r10
	tmp.s64 = ctx.r[10].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[10].u32);
	ctx.r[10].s64 = tmp.s64;
	// 82437F20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82437F24: 81610064  lwz r11, 0x64(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82437F28: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82437F2C: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82437F30: 81610074  lwz r11, 0x74(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82437F34: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82437F38: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 82437F3C: 409A001C  bne cr6, 0x82437f58
	if !ctx.cr[6].eq {
	pc = 0x82437F58; continue 'dispatch;
	}
	// 82437F40: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82437F44: 38AB5C1C  addi r5, r11, 0x5c1c
	ctx.r[5].s64 = ctx.r[11].s64 + 23580;
	// 82437F48: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82437F4C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82437F50: 916100F0  stw r11, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[11].u32 ) };
	// 82437F54: 48000014  b 0x82437f68
	pc = 0x82437F68; continue 'dispatch;
            }
            0x82437F58 => {
    //   block [0x82437F58..0x82437F60)
	// 82437F58: 912100F0  stw r9, 0xf0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(240 as u32), ctx.r[9].u32 ) };
	// 82437F5C: 48000018  b 0x82437f74
	pc = 0x82437F74; continue 'dispatch;
            }
            0x82437F60 => {
    //   block [0x82437F60..0x82437F68)
	// 82437F60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82437F64: 38AB5BD8  addi r5, r11, 0x5bd8
	ctx.r[5].s64 = ctx.r[11].s64 + 23512;
	pc = 0x82437F68; continue 'dispatch;
            }
            0x82437F68 => {
    //   block [0x82437F68..0x82437F74)
	// 82437F68: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82437F6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82437F70: 4BFFF399  bl 0x82437308
	ctx.lr = 0x82437F74;
	sub_82437308(ctx, base);
	pc = 0x82437F74; continue 'dispatch;
            }
            0x82437F74 => {
    //   block [0x82437F74..0x82437F88)
	// 82437F74: 4BFFF3DD  bl 0x82437350
	ctx.lr = 0x82437F78;
	sub_82437350(ctx, base);
	// 82437F78: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82437F7C: 409A000C  bne cr6, 0x82437f88
	if !ctx.cr[6].eq {
	pc = 0x82437F88; continue 'dispatch;
	}
	// 82437F80: 817E0038  lwz r11, 0x38(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(56 as u32) ) } as u64;
	// 82437F84: 48000008  b 0x82437f8c
	pc = 0x82437F8C; continue 'dispatch;
            }
            0x82437F88 => {
    //   block [0x82437F88..0x82437F8C)
	// 82437F88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82437F8C; continue 'dispatch;
            }
            0x82437F8C => {
    //   block [0x82437F8C..0x82437FB0)
	// 82437F8C: 916100C0  stw r11, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[11].u32 ) };
	// 82437F90: 817E006C  lwz r11, 0x6c(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(108 as u32) ) } as u64;
	// 82437F94: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82437F98: 419A0018  beq cr6, 0x82437fb0
	if ctx.cr[6].eq {
	pc = 0x82437FB0; continue 'dispatch;
	}
	// 82437F9C: 38A100C0  addi r5, r1, 0xc0
	ctx.r[5].s64 = ctx.r[1].s64 + 192;
	// 82437FA0: 388100E0  addi r4, r1, 0xe0
	ctx.r[4].s64 = ctx.r[1].s64 + 224;
	// 82437FA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82437FA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82437FAC: 4E800421  bctrl
	ctx.lr = 0x82437FB0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82437FB0 => {
    //   block [0x82437FB0..0x82437FB8)
	// 82437FB0: 38210170  addi r1, r1, 0x170
	ctx.r[1].s64 = ctx.r[1].s64 + 368;
	// 82437FB4: 480FD158  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82437FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82437FB8 size=128
    let mut pc: u32 = 0x82437FB8;
    'dispatch: loop {
        match pc {
            0x82437FB8 => {
    //   block [0x82437FB8..0x82437FF8)
	// 82437FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82437FBC: 480FD101  bl 0x825350bc
	ctx.lr = 0x82437FC0;
	sub_82535080(ctx, base);
	// 82437FC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82437FC4: 54EB2036  slwi r11, r7, 4
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82437FC8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82437FCC: 7FEB3214  add r31, r11, r6
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[6].u64;
	// 82437FD0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82437FD4: 90BF0004  stw r5, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82437FD8: 817E0044  lwz r11, 0x44(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82437FDC: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82437FE0: 4BFFF9E9  bl 0x824379c8
	ctx.lr = 0x82437FE4;
	sub_824379C8(ctx, base);
	// 82437FE4: 817E0048  lwz r11, 0x48(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(72 as u32) ) } as u64;
	// 82437FE8: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82437FEC: 409A000C  bne cr6, 0x82437ff8
	if !ctx.cr[6].eq {
	pc = 0x82437FF8; continue 'dispatch;
	}
	// 82437FF0: 7D6B0E70  srawi r11, r11, 1
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 1) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 1) as i64;
	// 82437FF4: 7D6B0194  addze r11, r11
	tmp.s64 = ctx.r[11].s64 + ctx.xer.ca as i64;
	ctx.xer.ca = (tmp.u32 < ctx.r[11].u32);
	ctx.r[11].s64 = tmp.s64;
	pc = 0x82437FF8; continue 'dispatch;
            }
            0x82437FF8 => {
    //   block [0x82437FF8..0x8243802C)
	// 82437FF8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82437FFC: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82438000: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82438004: 409A0028  bne cr6, 0x8243802c
	if !ctx.cr[6].eq {
	pc = 0x8243802C; continue 'dispatch;
	}
	// 82438008: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8243800C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438010: 83DE0044  lwz r30, 0x44(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(68 as u32) ) } as u64;
	// 82438014: 38AB5C5C  addi r5, r11, 0x5c5c
	ctx.r[5].s64 = ctx.r[11].s64 + 23644;
	// 82438018: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 8243801C: 4BFFF2ED  bl 0x82437308
	ctx.lr = 0x82438020;
	sub_82437308(ctx, base);
	// 82438020: 93DF0010  stw r30, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[30].u32 ) };
	// 82438024: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438028: 480FD0E4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8243802C => {
    //   block [0x8243802C..0x82438038)
	// 8243802C: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
	// 82438030: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438034: 480FD0D8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438038 size=8
    let mut pc: u32 = 0x82438038;
    'dispatch: loop {
        match pc {
            0x82438038 => {
    //   block [0x82438038..0x82438040)
	// 82438038: 80630038  lwz r3, 0x38(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(56 as u32) ) } as u64;
	// 8243803C: 4BFFFC6C  b 0x82437ca8
	sub_82437C98(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438040 size=164
    let mut pc: u32 = 0x82438040;
    'dispatch: loop {
        match pc {
            0x82438040 => {
    //   block [0x82438040..0x82438068)
	// 82438040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438044: 480FD079  bl 0x825350bc
	ctx.lr = 0x82438048;
	sub_82535080(ctx, base);
	// 82438048: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243804C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82438050: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82438054: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82438058: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 8243805C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82438060: 3920000C  li r9, 0xc
	ctx.r[9].s64 = 12;
	// 82438064: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82438068; continue 'dispatch;
            }
            0x82438068 => {
    //   block [0x82438068..0x82438094)
	// 82438068: F94B0000  std r10, 0(r11)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u64 ) };
	// 8243806C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82438070: 4200FFF8  bdnz 0x82438068
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438068; continue 'dispatch;
	}
	// 82438074: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82438078: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8243807C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438080: 4BFFFD51  bl 0x82437dd0
	ctx.lr = 0x82438084;
	sub_82437DD0(ctx, base);
	// 82438084: 2F1E0001  cmpwi cr6, r30, 1
	ctx.cr[6].compare_i32(ctx.r[30].s32, 1, &mut ctx.xer);
	// 82438088: 409A000C  bne cr6, 0x82438094
	if !ctx.cr[6].eq {
	pc = 0x82438094; continue 'dispatch;
	}
	// 8243808C: 817F0038  lwz r11, 0x38(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 82438090: 48000008  b 0x82438098
	pc = 0x82438098; continue 'dispatch;
            }
            0x82438094 => {
    //   block [0x82438094..0x82438098)
	// 82438094: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	pc = 0x82438098; continue 'dispatch;
            }
            0x82438098 => {
    //   block [0x82438098..0x824380BC)
	// 82438098: 815F0050  lwz r10, 0x50(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243809C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824380A0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 824380A4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 824380A8: 4BFFF589  bl 0x82437630
	ctx.lr = 0x824380AC;
	sub_82437630(ctx, base);
	// 824380AC: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 824380B0: 409A000C  bne cr6, 0x824380bc
	if !ctx.cr[6].eq {
	pc = 0x824380BC; continue 'dispatch;
	}
	// 824380B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 824380B8: 4BFFF8E9  bl 0x824379a0
	ctx.lr = 0x824380BC;
	sub_824379A0(ctx, base);
	pc = 0x824380BC; continue 'dispatch;
            }
            0x824380BC => {
    //   block [0x824380BC..0x824380DC)
	// 824380BC: 817F0068  lwz r11, 0x68(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(104 as u32) ) } as u64;
	// 824380C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 824380C4: 419A0018  beq cr6, 0x824380dc
	if ctx.cr[6].eq {
	pc = 0x824380DC; continue 'dispatch;
	}
	// 824380C8: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 824380CC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 824380D0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 824380D4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824380D8: 4E800421  bctrl
	ctx.lr = 0x824380DC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x824380DC => {
    //   block [0x824380DC..0x824380E4)
	// 824380DC: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 824380E0: 480FD02C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824380E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824380E8 size=56
    let mut pc: u32 = 0x824380E8;
    'dispatch: loop {
        match pc {
            0x824380E8 => {
    //   block [0x824380E8..0x82438120)
	// 824380E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824380EC: 480FCFD1  bl 0x825350bc
	ctx.lr = 0x824380F0;
	sub_82535080(ctx, base);
	// 824380F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824380F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824380F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824380FC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82438100: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82438104: 4BFFFF3D  bl 0x82438040
	ctx.lr = 0x82438108;
	sub_82438040(ctx, base);
	// 82438108: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8243810C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438110: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438114: 4BFFFD8D  bl 0x82437ea0
	ctx.lr = 0x82438118;
	sub_82437EA0(ctx, base);
	// 82438118: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243811C: 480FCFF0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438120 size=88
    let mut pc: u32 = 0x82438120;
    'dispatch: loop {
        match pc {
            0x82438120 => {
    //   block [0x82438120..0x82438178)
	// 82438120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438124: 480FCF95  bl 0x825350b8
	ctx.lr = 0x82438128;
	sub_82535080(ctx, base);
	// 82438128: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243812C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82438130: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82438134: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	// 82438138: 7D064378  mr r6, r8
	ctx.r[6].u64 = ctx.r[8].u64;
	// 8243813C: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 82438140: 7D485378  mr r8, r10
	ctx.r[8].u64 = ctx.r[10].u64;
	// 82438144: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82438148: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243814C: 4BFFF83D  bl 0x82437988
	ctx.lr = 0x82438150;
	sub_82437988(ctx, base);
	// 82438150: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82438154: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82438158: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8243815C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438160: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438164: 4BFFFE55  bl 0x82437fb8
	ctx.lr = 0x82438168;
	sub_82437FB8(ctx, base);
	// 82438168: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8243816C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82438170: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82438174: 480FCF94  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438178 size=312
    let mut pc: u32 = 0x82438178;
    'dispatch: loop {
        match pc {
            0x82438178 => {
    //   block [0x82438178..0x8243821C)
	// 82438178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243817C: 480FCF41  bl 0x825350bc
	ctx.lr = 0x82438180;
	sub_82535080(ctx, base);
	// 82438180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438184: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82438188: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243818C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82438190: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82438194: 4BFFFA1D  bl 0x82437bb0
	ctx.lr = 0x82438198;
	sub_82437BB0(ctx, base);
	// 82438198: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 8243819C: 409A010C  bne cr6, 0x824382a8
	if !ctx.cr[6].eq {
	pc = 0x824382A8; continue 'dispatch;
	}
	// 824381A0: 397DFFFF  addi r11, r29, -1
	ctx.r[11].s64 = ctx.r[29].s64 + -1;
	// 824381A4: 93BF0034  stw r29, 0x34(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), ctx.r[29].u32 ) };
	// 824381A8: 2B0B0014  cmplwi cr6, r11, 0x14
	ctx.cr[6].compare_u32(ctx.r[11].u32, 20 as u32, &mut ctx.xer);
	// 824381AC: 419900E8  bgt cr6, 0x82438294
	if ctx.cr[6].gt {
	pc = 0x82438294; continue 'dispatch;
	}
	// 824381B0: 3D808244  lis r12, -0x7dbc
	ctx.r[12].s64 = -2109472768;
	// 824381B4: 398C81C8  addi r12, r12, -0x7e38
	ctx.r[12].s64 = ctx.r[12].s64 + -32312;
	// 824381B8: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 824381BC: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 824381C0: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 824381C4: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x82438280; continue 'dispatch;
		},
		1 => {
	pc = 0x82438230; continue 'dispatch;
		},
		2 => {
	pc = 0x82438294; continue 'dispatch;
		},
		3 => {
	pc = 0x82438244; continue 'dispatch;
		},
		4 => {
	pc = 0x82438258; continue 'dispatch;
		},
		5 => {
	pc = 0x82438294; continue 'dispatch;
		},
		6 => {
	pc = 0x82438294; continue 'dispatch;
		},
		7 => {
	pc = 0x82438294; continue 'dispatch;
		},
		8 => {
	pc = 0x82438294; continue 'dispatch;
		},
		9 => {
	pc = 0x82438294; continue 'dispatch;
		},
		10 => {
	pc = 0x8243821C; continue 'dispatch;
		},
		11 => {
	pc = 0x82438294; continue 'dispatch;
		},
		12 => {
	pc = 0x8243821C; continue 'dispatch;
		},
		13 => {
	pc = 0x82438294; continue 'dispatch;
		},
		14 => {
	pc = 0x82438294; continue 'dispatch;
		},
		15 => {
	pc = 0x82438294; continue 'dispatch;
		},
		16 => {
	pc = 0x82438294; continue 'dispatch;
		},
		17 => {
	pc = 0x82438294; continue 'dispatch;
		},
		18 => {
	pc = 0x82438294; continue 'dispatch;
		},
		19 => {
	pc = 0x82438294; continue 'dispatch;
		},
		20 => {
	pc = 0x8243826C; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 824381C8: 82438280  lwz r18, -0x7d80(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32128 as u32) ) } as u64;
	// 824381CC: 82438230  lwz r18, -0x7dd0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32208 as u32) ) } as u64;
	// 824381D0: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 824381D4: 82438244  lwz r18, -0x7dbc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32188 as u32) ) } as u64;
	// 824381D8: 82438258  lwz r18, -0x7da8(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32168 as u32) ) } as u64;
	// 824381DC: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 824381E0: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 824381E4: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 824381E8: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 824381EC: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 824381F0: 8243821C  lwz r18, -0x7de4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32228 as u32) ) } as u64;
	// 824381F4: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 824381F8: 8243821C  lwz r18, -0x7de4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32228 as u32) ) } as u64;
	// 824381FC: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 82438200: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 82438204: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 82438208: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 8243820C: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 82438210: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 82438214: 82438294  lwz r18, -0x7d6c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32108 as u32) ) } as u64;
	// 82438218: 8243826C  lwz r18, -0x7d94(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-32148 as u32) ) } as u64;
            }
            0x8243821C => {
    //   block [0x8243821C..0x82438230)
	// 8243821C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438224: 4BFFFA75  bl 0x82437c98
	ctx.lr = 0x82438228;
	sub_82437C98(ctx, base);
	// 82438228: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243822C: 480FCEE0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82438230 => {
    //   block [0x82438230..0x82438244)
	// 82438230: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438238: 4BFFFB01  bl 0x82437d38
	ctx.lr = 0x8243823C;
	sub_82437D38(ctx, base);
	// 8243823C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438240: 480FCECC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82438244 => {
    //   block [0x82438244..0x82438258)
	// 82438244: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438248: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243824C: 4BFFFAFD  bl 0x82437d48
	ctx.lr = 0x82438250;
	sub_82437D48(ctx, base);
	// 82438250: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438254: 480FCEB8  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82438258 => {
    //   block [0x82438258..0x8243826C)
	// 82438258: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243825C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438260: 4BFFFAF9  bl 0x82437d58
	ctx.lr = 0x82438264;
	sub_82437D58(ctx, base);
	// 82438264: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438268: 480FCEA4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8243826C => {
    //   block [0x8243826C..0x82438280)
	// 8243826C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438274: 4BFFFAF5  bl 0x82437d68
	ctx.lr = 0x82438278;
	sub_82437D68(ctx, base);
	// 82438278: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243827C: 480FCE90  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82438280 => {
    //   block [0x82438280..0x82438294)
	// 82438280: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438284: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438288: 4BFFFDB1  bl 0x82438038
	ctx.lr = 0x8243828C;
	sub_82438038(ctx, base);
	// 8243828C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438290: 480FCE7C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82438294 => {
    //   block [0x82438294..0x824382A8)
	// 82438294: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438298: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243829C: 38AB5C9C  addi r5, r11, 0x5c9c
	ctx.r[5].s64 = ctx.r[11].s64 + 23708;
	// 824382A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824382A4: 4BFFF065  bl 0x82437308
	ctx.lr = 0x824382A8;
	sub_82437308(ctx, base);
	pc = 0x824382A8; continue 'dispatch;
            }
            0x824382A8 => {
    //   block [0x824382A8..0x824382B0)
	// 824382A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824382AC: 480FCE60  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824382B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824382B0 size=4
    let mut pc: u32 = 0x824382B0;
    'dispatch: loop {
        match pc {
            0x824382B0 => {
    //   block [0x824382B0..0x824382B4)
	// 824382B0: 4BFFFEC8  b 0x82438178
	sub_82438178(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824382B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824382B8 size=484
    let mut pc: u32 = 0x824382B8;
    'dispatch: loop {
        match pc {
            0x824382B8 => {
    //   block [0x824382B8..0x824382F0)
	// 824382B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824382BC: 480FCE01  bl 0x825350bc
	ctx.lr = 0x824382C0;
	sub_82535080(ctx, base);
	// 824382C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824382C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824382C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824382CC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824382D0: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 824382D4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824382D8: 409A0018  bne cr6, 0x824382f0
	if !ctx.cr[6].eq {
	pc = 0x824382F0; continue 'dispatch;
	}
	// 824382DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824382E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824382E4: 388B5D0C  addi r4, r11, 0x5d0c
	ctx.r[4].s64 = ctx.r[11].s64 + 23820;
	// 824382E8: 480110C1  bl 0x824493a8
	ctx.lr = 0x824382EC;
	sub_824493A8(ctx, base);
	// 824382EC: 907F0004  stw r3, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	pc = 0x824382F0; continue 'dispatch;
            }
            0x824382F0 => {
    //   block [0x824382F0..0x824383E4)
	// 824382F0: 2F030051  cmpwi cr6, r3, 0x51
	ctx.cr[6].compare_i32(ctx.r[3].s32, 81, &mut ctx.xer);
	// 824382F4: 41990134  bgt cr6, 0x82438428
	if ctx.cr[6].gt {
	pc = 0x82438428; continue 'dispatch;
	}
	// 824382F8: 419A016C  beq cr6, 0x82438464
	if ctx.cr[6].eq {
	pc = 0x82438464; continue 'dispatch;
	}
	// 824382FC: 3963FFEF  addi r11, r3, -0x11
	ctx.r[11].s64 = ctx.r[3].s64 + -17;
	// 82438300: 2B0B0030  cmplwi cr6, r11, 0x30
	ctx.cr[6].compare_u32(ctx.r[11].u32, 48 as u32, &mut ctx.xer);
	// 82438304: 4199013C  bgt cr6, 0x82438440
	if ctx.cr[6].gt {
	pc = 0x82438440; continue 'dispatch;
	}
	// 82438308: 3D808244  lis r12, -0x7dbc
	ctx.r[12].s64 = -2109472768;
	// 8243830C: 398C8320  addi r12, r12, -0x7ce0
	ctx.r[12].s64 = ctx.r[12].s64 + -31968;
	// 82438310: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82438314: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82438318: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 8243831C: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x824383E4; continue 'dispatch;
		},
		1 => {
	pc = 0x82438440; continue 'dispatch;
		},
		2 => {
	pc = 0x82438440; continue 'dispatch;
		},
		3 => {
	pc = 0x82438440; continue 'dispatch;
		},
		4 => {
	pc = 0x82438440; continue 'dispatch;
		},
		5 => {
	pc = 0x82438440; continue 'dispatch;
		},
		6 => {
	pc = 0x82438440; continue 'dispatch;
		},
		7 => {
	pc = 0x82438440; continue 'dispatch;
		},
		8 => {
	pc = 0x82438440; continue 'dispatch;
		},
		9 => {
	pc = 0x82438440; continue 'dispatch;
		},
		10 => {
	pc = 0x82438440; continue 'dispatch;
		},
		11 => {
	pc = 0x82438440; continue 'dispatch;
		},
		12 => {
	pc = 0x82438440; continue 'dispatch;
		},
		13 => {
	pc = 0x82438440; continue 'dispatch;
		},
		14 => {
	pc = 0x82438440; continue 'dispatch;
		},
		15 => {
	pc = 0x82438440; continue 'dispatch;
		},
		16 => {
	pc = 0x824383F8; continue 'dispatch;
		},
		17 => {
	pc = 0x82438440; continue 'dispatch;
		},
		18 => {
	pc = 0x82438440; continue 'dispatch;
		},
		19 => {
	pc = 0x82438440; continue 'dispatch;
		},
		20 => {
	pc = 0x82438440; continue 'dispatch;
		},
		21 => {
	pc = 0x82438440; continue 'dispatch;
		},
		22 => {
	pc = 0x82438440; continue 'dispatch;
		},
		23 => {
	pc = 0x82438440; continue 'dispatch;
		},
		24 => {
	pc = 0x82438440; continue 'dispatch;
		},
		25 => {
	pc = 0x82438440; continue 'dispatch;
		},
		26 => {
	pc = 0x82438440; continue 'dispatch;
		},
		27 => {
	pc = 0x82438440; continue 'dispatch;
		},
		28 => {
	pc = 0x82438440; continue 'dispatch;
		},
		29 => {
	pc = 0x82438440; continue 'dispatch;
		},
		30 => {
	pc = 0x82438440; continue 'dispatch;
		},
		31 => {
	pc = 0x82438440; continue 'dispatch;
		},
		32 => {
	pc = 0x82438420; continue 'dispatch;
		},
		33 => {
	pc = 0x82438440; continue 'dispatch;
		},
		34 => {
	pc = 0x82438440; continue 'dispatch;
		},
		35 => {
	pc = 0x82438440; continue 'dispatch;
		},
		36 => {
	pc = 0x82438440; continue 'dispatch;
		},
		37 => {
	pc = 0x82438440; continue 'dispatch;
		},
		38 => {
	pc = 0x82438440; continue 'dispatch;
		},
		39 => {
	pc = 0x82438440; continue 'dispatch;
		},
		40 => {
	pc = 0x82438440; continue 'dispatch;
		},
		41 => {
	pc = 0x82438440; continue 'dispatch;
		},
		42 => {
	pc = 0x82438440; continue 'dispatch;
		},
		43 => {
	pc = 0x82438440; continue 'dispatch;
		},
		44 => {
	pc = 0x82438440; continue 'dispatch;
		},
		45 => {
	pc = 0x82438440; continue 'dispatch;
		},
		46 => {
	pc = 0x82438440; continue 'dispatch;
		},
		47 => {
	pc = 0x82438440; continue 'dispatch;
		},
		48 => {
	pc = 0x82438464; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82438320: 824383E4  lwz r18, -0x7c1c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31772 as u32) ) } as u64;
	// 82438324: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438328: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243832C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438330: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438334: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438338: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243833C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438340: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438344: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438348: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243834C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438350: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438354: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438358: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243835C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438360: 824383F8  lwz r18, -0x7c08(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31752 as u32) ) } as u64;
	// 82438364: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438368: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243836C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438370: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438374: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438378: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243837C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438380: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438384: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438388: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243838C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438390: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438394: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 82438398: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 8243839C: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383A0: 82438420  lwz r18, -0x7be0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31712 as u32) ) } as u64;
	// 824383A4: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383A8: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383AC: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383B0: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383B4: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383B8: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383BC: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383C0: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383C4: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383C8: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383CC: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383D0: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383D4: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383D8: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383DC: 82438440  lwz r18, -0x7bc0(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31680 as u32) ) } as u64;
	// 824383E0: 82438464  lwz r18, -0x7b9c(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-31644 as u32) ) } as u64;
            }
            0x824383E4 => {
    //   block [0x824383E4..0x824383F0)
	// 824383E4: 817E0090  lwz r11, 0x90(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(144 as u32) ) } as u64;
	// 824383E8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824383EC: 419A0070  beq cr6, 0x8243845c
	if ctx.cr[6].eq {
	pc = 0x8243845C; continue 'dispatch;
	}
	pc = 0x824383F0; continue 'dispatch;
            }
            0x824383F0 => {
    //   block [0x824383F0..0x824383F8)
	// 824383F0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 824383F4: 48000090  b 0x82438484
	pc = 0x82438484; continue 'dispatch;
            }
            0x824383F8 => {
    //   block [0x824383F8..0x82438420)
	// 824383F8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824383FC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438404: 4BFFFEAD  bl 0x824382b0
	ctx.lr = 0x82438408;
	sub_824382B0(ctx, base);
	// 82438408: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8243840C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438414: 4BFFFCD5  bl 0x824380e8
	ctx.lr = 0x82438418;
	sub_824380E8(ctx, base);
	// 82438418: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243841C: 480FCCF0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x82438420 => {
    //   block [0x82438420..0x82438428)
	// 82438420: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82438424: 48000050  b 0x82438474
	pc = 0x82438474; continue 'dispatch;
            }
            0x82438428 => {
    //   block [0x82438428..0x82438440)
	// 82438428: 2F030061  cmpwi cr6, r3, 0x61
	ctx.cr[6].compare_i32(ctx.r[3].s32, 97, &mut ctx.xer);
	// 8243842C: 419A0038  beq cr6, 0x82438464
	if ctx.cr[6].eq {
	pc = 0x82438464; continue 'dispatch;
	}
	// 82438430: 2F030101  cmpwi cr6, r3, 0x101
	ctx.cr[6].compare_i32(ctx.r[3].s32, 257, &mut ctx.xer);
	// 82438434: 419AFFBC  beq cr6, 0x824383f0
	if ctx.cr[6].eq {
	pc = 0x824383F0; continue 'dispatch;
	}
	// 82438438: 2F031001  cmpwi cr6, r3, 0x1001
	ctx.cr[6].compare_i32(ctx.r[3].s32, 4097, &mut ctx.xer);
	// 8243843C: 419A0020  beq cr6, 0x8243845c
	if ctx.cr[6].eq {
	pc = 0x8243845C; continue 'dispatch;
	}
	pc = 0x82438440; continue 'dispatch;
            }
            0x82438440 => {
    //   block [0x82438440..0x8243845C)
	// 82438440: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438444: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438448: 38AB5CD0  addi r5, r11, 0x5cd0
	ctx.r[5].s64 = ctx.r[11].s64 + 23760;
	// 8243844C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438450: 4BFFEEB9  bl 0x82437308
	ctx.lr = 0x82438454;
	sub_82437308(ctx, base);
	// 82438454: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438458: 480FCCB4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8243845C => {
    //   block [0x8243845C..0x82438464)
	// 8243845C: 38A00015  li r5, 0x15
	ctx.r[5].s64 = 21;
	// 82438460: 48000014  b 0x82438474
	pc = 0x82438474; continue 'dispatch;
            }
            0x82438464 => {
    //   block [0x82438464..0x82438474)
	// 82438464: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243846C: 4BFFF70D  bl 0x82437b78
	ctx.lr = 0x82438470;
	sub_82437B78(ctx, base);
	// 82438470: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	pc = 0x82438474; continue 'dispatch;
            }
            0x82438474 => {
    //   block [0x82438474..0x82438484)
	// 82438474: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243847C: 4BFFFE35  bl 0x824382b0
	ctx.lr = 0x82438480;
	sub_824382B0(ctx, base);
	// 82438480: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	pc = 0x82438484; continue 'dispatch;
            }
            0x82438484 => {
    //   block [0x82438484..0x8243849C)
	// 82438484: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82438488: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243848C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438490: 4BFFFBB1  bl 0x82438040
	ctx.lr = 0x82438494;
	sub_82438040(ctx, base);
	// 82438494: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438498: 480FCC74  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824384A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824384A0 size=12
    let mut pc: u32 = 0x824384A0;
    'dispatch: loop {
        match pc {
            0x824384A0 => {
    //   block [0x824384A0..0x824384AC)
	// 824384A0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824384A4: 386B5D14  addi r3, r11, 0x5d14
	ctx.r[3].s64 = ctx.r[11].s64 + 23828;
	// 824384A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824384B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824384B0 size=92
    let mut pc: u32 = 0x824384B0;
    'dispatch: loop {
        match pc {
            0x824384B0 => {
    //   block [0x824384B0..0x824384F8)
	// 824384B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824384B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824384B8: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824384BC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824384C0: 419A0038  beq cr6, 0x824384f8
	if ctx.cr[6].eq {
	pc = 0x824384F8; continue 'dispatch;
	}
	// 824384C4: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824384C8: 41980030  blt cr6, 0x824384f8
	if ctx.cr[6].lt {
	pc = 0x824384F8; continue 'dispatch;
	}
	// 824384CC: 3D60820C  lis r11, -0x7df4
	ctx.r[11].s64 = -2113142784;
	// 824384D0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 824384D4: 388B4EB8  addi r4, r11, 0x4eb8
	ctx.r[4].s64 = ctx.r[11].s64 + 20152;
	// 824384D8: 38630012  addi r3, r3, 0x12
	ctx.r[3].s64 = ctx.r[3].s64 + 18;
	// 824384DC: 480FACB5  bl 0x82533190
	ctx.lr = 0x824384E0;
	sub_82533190(ctx, base);
	// 824384E0: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 824384E4: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824384E8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824384EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824384F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824384F4: 4E800020  blr
	return;
            }
            0x824384F8 => {
    //   block [0x824384F8..0x8243850C)
	// 824384F8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824384FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82438500: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438504: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438508: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438510 size=92
    let mut pc: u32 = 0x82438510;
    'dispatch: loop {
        match pc {
            0x82438510 => {
    //   block [0x82438510..0x82438558)
	// 82438510: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438514: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82438518: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243851C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82438520: 419A0038  beq cr6, 0x82438558
	if ctx.cr[6].eq {
	pc = 0x82438558; continue 'dispatch;
	}
	// 82438524: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82438528: 41980030  blt cr6, 0x82438558
	if ctx.cr[6].lt {
	pc = 0x82438558; continue 'dispatch;
	}
	// 8243852C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438530: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82438534: 388B5D54  addi r4, r11, 0x5d54
	ctx.r[4].s64 = ctx.r[11].s64 + 23892;
	// 82438538: 38630013  addi r3, r3, 0x13
	ctx.r[3].s64 = ctx.r[3].s64 + 19;
	// 8243853C: 480FAC55  bl 0x82533190
	ctx.lr = 0x82438540;
	sub_82533190(ctx, base);
	// 82438540: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82438544: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 82438548: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8243854C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438550: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438554: 4E800020  blr
	return;
            }
            0x82438558 => {
    //   block [0x82438558..0x8243856C)
	// 82438558: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243855C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82438560: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438564: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82438570 size=16
    let mut pc: u32 = 0x82438570;
    'dispatch: loop {
        match pc {
            0x82438570 => {
    //   block [0x82438570..0x82438580)
	// 82438570: 21630000  subfic r11, r3, 0
	ctx.xer.ca = ctx.r[3].u32 <= 0 as u32;
	ctx.r[11].s64 = (0 as i64) - ctx.r[3].s64;
	// 82438574: 7D6B5910  subfe r11, r11, r11
	let x = (!ctx.r[11].u32);
	let y = ctx.r[11].u32;
	let s = x.wrapping_add(y);
	let res = s.wrapping_add(ctx.xer.ca as u32);
	tmp.u8 = (s < x) as u8 | (res < s) as u8;
	ctx.r[11].u32 = res;
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	ctx.xer.ca = (tmp.u8 != 0);
	// 82438578: 71630023  andi. r3, r11, 0x23
	ctx.r[3].u64 = ctx.r[11].u64 & 35;
	ctx.cr[0].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243857C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438580 size=64
    let mut pc: u32 = 0x82438580;
    'dispatch: loop {
        match pc {
            0x82438580 => {
    //   block [0x82438580..0x824385B0)
	// 82438580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82438588: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243858C: 3D00828A  lis r8, -0x7d76
	ctx.r[8].s64 = -2104885248;
	// 82438590: 81489F48  lwz r10, -0x60b8(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-24760 as u32) ) } as u64;
	// 82438594: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 82438598: 40980018  bge cr6, 0x824385b0
	if !ctx.cr[6].lt {
	pc = 0x824385B0; continue 'dispatch;
	}
	// 8243859C: 4BFFFF05  bl 0x824384a0
	ctx.lr = 0x824385A0;
	sub_824384A0(ctx, base);
	// 824385A0: 3D20828A  lis r9, -0x7d76
	ctx.r[9].s64 = -2104885248;
	// 824385A4: 396A0001  addi r11, r10, 1
	ctx.r[11].s64 = ctx.r[10].s64 + 1;
	// 824385A8: 90699F44  stw r3, -0x60bc(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(-24764 as u32), ctx.r[3].u32 ) };
	// 824385AC: 91689F48  stw r11, -0x60b8(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(-24760 as u32), ctx.r[11].u32 ) };
	pc = 0x824385B0; continue 'dispatch;
            }
            0x824385B0 => {
    //   block [0x824385B0..0x824385C0)
	// 824385B0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824385B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824385B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824385BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824385C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824385C0 size=228
    let mut pc: u32 = 0x824385C0;
    'dispatch: loop {
        match pc {
            0x824385C0 => {
    //   block [0x824385C0..0x82438604)
	// 824385C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824385C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824385C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 824385CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824385D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824385D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824385D8: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 824385DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 824385E0: 91650000  stw r11, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824385E4: 91660000  stw r11, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 824385E8: 419A00A4  beq cr6, 0x8243868c
	if ctx.cr[6].eq {
	pc = 0x8243868C; continue 'dispatch;
	}
	// 824385EC: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 824385F0: 4099009C  ble cr6, 0x8243868c
	if !ctx.cr[6].gt {
	pc = 0x8243868C; continue 'dispatch;
	}
	// 824385F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824385F8: 3BEB5D60  addi r31, r11, 0x5d60
	ctx.r[31].s64 = ctx.r[11].s64 + 23904;
	// 824385FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438600: 3BCB5D6C  addi r30, r11, 0x5d6c
	ctx.r[30].s64 = ctx.r[11].s64 + 23916;
	pc = 0x82438604; continue 'dispatch;
            }
            0x82438604 => {
    //   block [0x82438604..0x82438610)
	// 82438604: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 82438608: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
	// 8243860C: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	pc = 0x82438610; continue 'dispatch;
            }
            0x82438610 => {
    //   block [0x82438610..0x82438630)
	// 82438610: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438614: 886A0000  lbz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438618: 7D234851  subf. r9, r3, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 8243861C: 40820014  bne 0x82438630
	if !ctx.cr[0].eq {
	pc = 0x82438630; continue 'dispatch;
	}
	// 82438620: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82438624: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82438628: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 8243862C: 409AFFE4  bne cr6, 0x82438610
	if !ctx.cr[6].eq {
	pc = 0x82438610; continue 'dispatch;
	}
	pc = 0x82438630; continue 'dispatch;
            }
            0x82438630 => {
    //   block [0x82438630..0x82438644)
	// 82438630: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82438634: 409A0048  bne cr6, 0x8243867c
	if !ctx.cr[6].eq {
	pc = 0x8243867C; continue 'dispatch;
	}
	// 82438638: 7CEB3B78  mr r11, r7
	ctx.r[11].u64 = ctx.r[7].u64;
	// 8243863C: 7FEAFB78  mr r10, r31
	ctx.r[10].u64 = ctx.r[31].u64;
	// 82438640: 390B0008  addi r8, r11, 8
	ctx.r[8].s64 = ctx.r[11].s64 + 8;
	pc = 0x82438644; continue 'dispatch;
            }
            0x82438644 => {
    //   block [0x82438644..0x82438664)
	// 82438644: 892B0000  lbz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438648: 886A0000  lbz r3, 0(r10)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 8243864C: 7D234851  subf. r9, r3, r9
	ctx.r[9].s64 = ctx.r[9].s64 - ctx.r[3].s64;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82438650: 40820014  bne 0x82438664
	if !ctx.cr[0].eq {
	pc = 0x82438664; continue 'dispatch;
	}
	// 82438654: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82438658: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 8243865C: 7F0B4000  cmpw cr6, r11, r8
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82438660: 409AFFE4  bne cr6, 0x82438644
	if !ctx.cr[6].eq {
	pc = 0x82438644; continue 'dispatch;
	}
	pc = 0x82438664; continue 'dispatch;
            }
            0x82438664 => {
    //   block [0x82438664..0x8243867C)
	// 82438664: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82438668: 409A0014  bne cr6, 0x8243867c
	if !ctx.cr[6].eq {
	pc = 0x8243867C; continue 'dispatch;
	}
	// 8243866C: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 82438670: 90E50000  stw r7, 0(r5)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82438674: 4BFFFEFD  bl 0x82438570
	ctx.lr = 0x82438678;
	sub_82438570(ctx, base);
	// 82438678: 90660000  stw r3, 0(r6)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	pc = 0x8243867C; continue 'dispatch;
            }
            0x8243867C => {
    //   block [0x8243867C..0x8243868C)
	// 8243867C: 3884FFFF  addi r4, r4, -1
	ctx.r[4].s64 = ctx.r[4].s64 + -1;
	// 82438680: 38E70001  addi r7, r7, 1
	ctx.r[7].s64 = ctx.r[7].s64 + 1;
	// 82438684: 2B040000  cmplwi cr6, r4, 0
	ctx.cr[6].compare_u32(ctx.r[4].u32, 0 as u32, &mut ctx.xer);
	// 82438688: 409AFF7C  bne cr6, 0x82438604
	if !ctx.cr[6].eq {
	pc = 0x82438604; continue 'dispatch;
	}
	pc = 0x8243868C; continue 'dispatch;
            }
            0x8243868C => {
    //   block [0x8243868C..0x824386A4)
	// 8243868C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438690: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438694: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438698: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243869C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824386A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824386A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824386A8 size=64
    let mut pc: u32 = 0x824386A8;
    'dispatch: loop {
        match pc {
            0x824386A8 => {
    //   block [0x824386A8..0x824386C4)
	// 824386A8: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 824386AC: 394B0720  addi r10, r11, 0x720
	ctx.r[10].s64 = ctx.r[11].s64 + 1824;
	// 824386B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824386B4: 812A0008  lwz r9, 8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) } as u64;
	// 824386B8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 824386BC: 40990024  ble cr6, 0x824386e0
	if !ctx.cr[6].gt {
	pc = 0x824386E0; continue 'dispatch;
	}
	// 824386C0: 386A000C  addi r3, r10, 0xc
	ctx.r[3].s64 = ctx.r[10].s64 + 12;
	pc = 0x824386C4; continue 'dispatch;
            }
            0x824386C4 => {
    //   block [0x824386C4..0x824386E0)
	// 824386C4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824386C8: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 824386CC: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 824386D0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824386D4: 3863004C  addi r3, r3, 0x4c
	ctx.r[3].s64 = ctx.r[3].s64 + 76;
	// 824386D8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 824386DC: 4198FFE8  blt cr6, 0x824386c4
	if ctx.cr[6].lt {
	pc = 0x824386C4; continue 'dispatch;
	}
	pc = 0x824386E0; continue 'dispatch;
            }
            0x824386E0 => {
    //   block [0x824386E0..0x824386E8)
	// 824386E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824386E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824386E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x824386E8 size=36
    let mut pc: u32 = 0x824386E8;
    'dispatch: loop {
        match pc {
            0x824386E8 => {
    //   block [0x824386E8..0x8243870C)
	// 824386E8: 3D40820D  lis r10, -0x7df3
	ctx.r[10].s64 = -2113077248;
	// 824386EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 824386F0: C00A1FF8  lfs f0, 0x1ff8(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(8184 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 824386F4: D003003C  stfs f0, 0x3c(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 824386F8: 91630044  stw r11, 0x44(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	// 824386FC: D0030040  stfs f0, 0x40(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(64 as u32), tmp.u32 ) };
	// 82438700: 91630048  stw r11, 0x48(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(72 as u32), ctx.r[11].u32 ) };
	// 82438704: 91630004  stw r11, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82438708: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438710 size=36
    let mut pc: u32 = 0x82438710;
    'dispatch: loop {
        match pc {
            0x82438710 => {
    //   block [0x82438710..0x82438734)
	// 82438710: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82438714: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82438718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243871C: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82438720: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82438724: 814B0720  lwz r10, 0x720(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1824 as u32) ) } as u64;
	// 82438728: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 8243872C: 914B0720  stw r10, 0x720(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1824 as u32), ctx.r[10].u32 ) };
	// 82438730: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438738 size=188
    let mut pc: u32 = 0x82438738;
    'dispatch: loop {
        match pc {
            0x82438738 => {
    //   block [0x82438738..0x8243877C)
	// 82438738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243873C: 480FC981  bl 0x825350bc
	ctx.lr = 0x82438740;
	sub_82535080(ctx, base);
	// 82438740: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438744: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82438748: 815F000C  lwz r10, 0xc(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8243874C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82438750: 409A002C  bne cr6, 0x8243877c
	if !ctx.cr[6].eq {
	pc = 0x8243877C; continue 'dispatch;
	}
	// 82438754: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82438758: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 8243875C: 917F001C  stw r11, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82438760: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 82438764: 917F0020  stw r11, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82438768: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 8243876C: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 82438770: 93BF0028  stw r29, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 82438774: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82438778: 480FC994  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x8243877C => {
    //   block [0x8243877C..0x824387F4)
	// 8243877C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438780: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82438784: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82438788: 3BCB48DC  addi r30, r11, 0x48dc
	ctx.r[30].s64 = ctx.r[11].s64 + 18652;
	// 8243878C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82438790: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 82438794: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82438798: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8243879C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824387A0: 388B5D78  addi r4, r11, 0x5d78
	ctx.r[4].s64 = ctx.r[11].s64 + 23928;
	// 824387A4: 4BFF086D  bl 0x82429010
	ctx.lr = 0x824387A8;
	sub_82429010(ctx, base);
	// 824387A8: 81410050  lwz r10, 0x50(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824387AC: 81210054  lwz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824387B0: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 824387B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 824387B8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 824387BC: 388B5D70  addi r4, r11, 0x5d70
	ctx.r[4].s64 = ctx.r[11].s64 + 23920;
	// 824387C0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 824387C4: 915F001C  stw r10, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 824387C8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 824387CC: 93BF0018  stw r29, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[29].u32 ) };
	// 824387D0: 913F0020  stw r9, 0x20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(32 as u32), ctx.r[9].u32 ) };
	// 824387D4: 4BFF083D  bl 0x82429010
	ctx.lr = 0x824387D8;
	sub_82429010(ctx, base);
	// 824387D8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 824387DC: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 824387E0: 93BF0028  stw r29, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[29].u32 ) };
	// 824387E4: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 824387E8: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 824387EC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824387F0: 480FC91C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824387F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824387F8 size=132
    let mut pc: u32 = 0x824387F8;
    'dispatch: loop {
        match pc {
            0x824387F8 => {
    //   block [0x824387F8..0x82438858)
	// 824387F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824387FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82438800: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82438804: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82438808: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243880C: 81630028  lwz r11, 0x28(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82438810: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82438814: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82438818: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243881C: 409A003C  bne cr6, 0x82438858
	if !ctx.cr[6].eq {
	pc = 0x82438858; continue 'dispatch;
	}
	// 82438820: 8163002C  lwz r11, 0x2c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) } as u64;
	// 82438824: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82438828: 419A0030  beq cr6, 0x82438858
	if ctx.cr[6].eq {
	pc = 0x82438858; continue 'dispatch;
	}
	// 8243882C: 81430030  lwz r10, 0x30(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(48 as u32) ) } as u64;
	// 82438830: 38C10058  addi r6, r1, 0x58
	ctx.r[6].s64 = ctx.r[1].s64 + 88;
	// 82438834: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82438838: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8243883C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82438840: 4BFF07D1  bl 0x82429010
	ctx.lr = 0x82438844;
	sub_82429010(ctx, base);
	// 82438844: 81610058  lwz r11, 0x58(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82438848: 8141005C  lwz r10, 0x5c(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 8243884C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82438850: 915E0000  stw r10, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82438854: 48000010  b 0x82438864
	pc = 0x82438864; continue 'dispatch;
            }
            0x82438858 => {
    //   block [0x82438858..0x82438864)
	// 82438858: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243885C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82438860: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	pc = 0x82438864; continue 'dispatch;
            }
            0x82438864 => {
    //   block [0x82438864..0x8243887C)
	// 82438864: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82438868: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243886C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438870: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82438874: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82438878: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438880 size=240
    let mut pc: u32 = 0x82438880;
    'dispatch: loop {
        match pc {
            0x82438880 => {
    //   block [0x82438880..0x8243889C)
	// 82438880: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82438884: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 82438888: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8243888C: 6127FFFF  ori r7, r9, 0xffff
	ctx.r[7].u64 = ctx.r[9].u64 | 65535;
	// 82438890: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82438894: 409A0008  bne cr6, 0x8243889c
	if !ctx.cr[6].eq {
	pc = 0x8243889C; continue 'dispatch;
	}
	// 82438898: 7CE53B78  mr r5, r7
	ctx.r[5].u64 = ctx.r[7].u64;
	pc = 0x8243889C; continue 'dispatch;
            }
            0x8243889C => {
    //   block [0x8243889C..0x824388A8)
	// 8243889C: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 824388A0: 409A0008  bne cr6, 0x824388a8
	if !ctx.cr[6].eq {
	pc = 0x824388A8; continue 'dispatch;
	}
	// 824388A4: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	pc = 0x824388A8; continue 'dispatch;
            }
            0x824388A8 => {
    //   block [0x824388A8..0x824388B8)
	// 824388A8: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 824388AC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 824388B0: 39000009  li r8, 9
	ctx.r[8].s64 = 9;
	// 824388B4: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	pc = 0x824388B8; continue 'dispatch;
            }
            0x824388B8 => {
    //   block [0x824388B8..0x824388D0)
	// 824388B8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 824388BC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824388C0: 4200FFF8  bdnz 0x824388b8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824388B8; continue 'dispatch;
	}
	// 824388C4: 39660024  addi r11, r6, 0x24
	ctx.r[11].s64 = ctx.r[6].s64 + 36;
	// 824388C8: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 824388CC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x824388D0; continue 'dispatch;
            }
            0x824388D0 => {
    //   block [0x824388D0..0x824388F0)
	// 824388D0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824388D4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824388D8: 4200FFF8  bdnz 0x824388d0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824388D0; continue 'dispatch;
	}
	// 824388DC: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 824388E0: 409A0020  bne cr6, 0x82438900
	if !ctx.cr[6].eq {
	pc = 0x82438900; continue 'dispatch;
	}
	// 824388E4: 39660044  addi r11, r6, 0x44
	ctx.r[11].s64 = ctx.r[6].s64 + 68;
	// 824388E8: 392000CF  li r9, 0xcf
	ctx.r[9].s64 = 207;
	// 824388EC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x824388F0; continue 'dispatch;
            }
            0x824388F0 => {
    //   block [0x824388F0..0x82438900)
	// 824388F0: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 824388F4: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 824388F8: 4200FFF8  bdnz 0x824388f0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824388F0; continue 'dispatch;
	}
	// 824388FC: 4800003C  b 0x82438938
	pc = 0x82438938; continue 'dispatch;
            }
            0x82438900 => {
    //   block [0x82438900..0x8243891C)
	// 82438900: 3D202793  lis r9, 0x2793
	ctx.r[9].s64 = 663945216;
	// 82438904: 7D6A2850  subf r11, r10, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 82438908: 61282B49  ori r8, r9, 0x2b49
	ctx.r[8].u64 = ctx.r[9].u64 | 11081;
	// 8243890C: 39260044  addi r9, r6, 0x44
	ctx.r[9].s64 = ctx.r[6].s64 + 68;
	// 82438910: 7D6B4016  mulhwu r11, r11, r8
	ctx.r[11].u64 = ((ctx.r[11].u32 as u64 * ctx.r[8].u32 as u64) >> 32);
	// 82438914: 5568D97E  srwi r8, r11, 5
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(5);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82438918: 396000CF  li r11, 0xcf
	ctx.r[11].s64 = 207;
	pc = 0x8243891C; continue 'dispatch;
            }
            0x8243891C => {
    //   block [0x8243891C..0x82438938)
	// 8243891C: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82438920: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82438924: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82438928: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243892C: 90890000  stw r4, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82438930: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82438934: 409AFFE8  bne cr6, 0x8243891c
	if !ctx.cr[6].eq {
	pc = 0x8243891C; continue 'dispatch;
	}
	pc = 0x82438938; continue 'dispatch;
            }
            0x82438938 => {
    //   block [0x82438938..0x82438944)
	// 82438938: 39660380  addi r11, r6, 0x380
	ctx.r[11].s64 = ctx.r[6].s64 + 896;
	// 8243893C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82438940: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x82438944; continue 'dispatch;
            }
            0x82438944 => {
    //   block [0x82438944..0x82438960)
	// 82438944: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82438948: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 8243894C: 4200FFF8  bdnz 0x82438944
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438944; continue 'dispatch;
	}
	// 82438950: 396603C0  addi r11, r6, 0x3c0
	ctx.r[11].s64 = ctx.r[6].s64 + 960;
	// 82438954: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 82438958: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8243895C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x82438960; continue 'dispatch;
            }
            0x82438960 => {
    //   block [0x82438960..0x82438970)
	// 82438960: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82438964: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438968: 4200FFF8  bdnz 0x82438960
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438960; continue 'dispatch;
	}
	// 8243896C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82438970 size=488
    let mut pc: u32 = 0x82438970;
    'dispatch: loop {
        match pc {
            0x82438970 => {
    //   block [0x82438970..0x82438990)
	// 82438970: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82438974: 39060400  addi r8, r6, 0x400
	ctx.r[8].s64 = ctx.r[6].s64 + 1024;
	// 82438978: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 8243897C: 3BE80400  addi r31, r8, 0x400
	ctx.r[31].s64 = ctx.r[8].s64 + 1024;
	// 82438980: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82438984: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 82438988: 7FEBFB78  mr r11, r31
	ctx.r[11].u64 = ctx.r[31].u64;
	// 8243898C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82438990; continue 'dispatch;
            }
            0x82438990 => {
    //   block [0x82438990..0x824389B0)
	// 82438990: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82438994: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82438998: 4200FFF8  bdnz 0x82438990
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438990; continue 'dispatch;
	}
	// 8243899C: 3C608201  lis r3, -0x7dff
	ctx.r[3].s64 = -2113863680;
	// 824389A0: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	// 824389A4: 20FFFFF0  subfic r7, r31, -0x10
	ctx.xer.ca = ctx.r[31].u32 <= -16 as u32;
	ctx.r[7].s64 = (-16 as i64) - ctx.r[31].s64;
	// 824389A8: 392000DC  li r9, 0xdc
	ctx.r[9].s64 = 220;
	// 824389AC: C0035B98  lfs f0, 0x5b98(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(23448 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	pc = 0x824389B0; continue 'dispatch;
            }
            0x824389B0 => {
    //   block [0x824389B0..0x824389FC)
	// 824389B0: 7C875A14  add r4, r7, r11
	ctx.r[4].u64 = ctx.r[7].u64 + ctx.r[11].u64;
	// 824389B4: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 824389B8: 7C8407B4  extsw r4, r4
	ctx.r[4].s64 = ctx.r[4].s32 as i64;
	// 824389BC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 824389C0: F881FFE0  std r4, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.r[4].u64 ) };
	// 824389C4: C9A1FFE0  lfd f13, -0x20(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 824389C8: FDA06E9C  fcfid f13, f13
	ctx.f[13].f64 = (ctx.f[13].s64 as f64);
	// 824389CC: FDA06818  frsp f13, f13
	ctx.f[13].f64 = (ctx.f[13].f64 as f32) as f64;
	// 824389D0: EDAD0032  fmuls f13, f13, f0
	ctx.f[13].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 824389D4: FDA06E5E  fctidz f13, f13
	ctx.f[13].s64 = if ctx.f[13].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[13].f64.trunc() as i64 };
	// 824389D8: D9A1FFE8  stfd f13, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.f[13].u64 ) };
	// 824389DC: 8881FFEF  lbz r4, -0x11(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(-17 as u32) ) } as u64;
	// 824389E0: 988B0000  stb r4, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[4].u8 ) };
	// 824389E4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 824389E8: 409AFFC8  bne cr6, 0x824389b0
	if !ctx.cr[6].eq {
	pc = 0x824389B0; continue 'dispatch;
	}
	// 824389EC: 397F00EC  addi r11, r31, 0xec
	ctx.r[11].s64 = ctx.r[31].s64 + 236;
	// 824389F0: 38E000FF  li r7, 0xff
	ctx.r[7].s64 = 255;
	// 824389F4: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 824389F8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x824389FC; continue 'dispatch;
            }
            0x824389FC => {
    //   block [0x824389FC..0x82438A20)
	// 824389FC: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82438A00: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82438A04: 4200FFF8  bdnz 0x824389fc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x824389FC; continue 'dispatch;
	}
	// 82438A08: 3D608000  lis r11, -0x8000
	ctx.r[11].s64 = -2147483648;
	// 82438A0C: 3D207FFF  lis r9, 0x7fff
	ctx.r[9].s64 = 2147418112;
	// 82438A10: 7F055840  cmplw cr6, r5, r11
	ctx.cr[6].compare_u32(ctx.r[5].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82438A14: 6124FFFF  ori r4, r9, 0xffff
	ctx.r[4].u64 = ctx.r[9].u64 | 65535;
	// 82438A18: 409A0008  bne cr6, 0x82438a20
	if !ctx.cr[6].eq {
	pc = 0x82438A20; continue 'dispatch;
	}
	// 82438A1C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	pc = 0x82438A20; continue 'dispatch;
            }
            0x82438A20 => {
    //   block [0x82438A20..0x82438A2C)
	// 82438A20: 7F0A5840  cmplw cr6, r10, r11
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82438A24: 409A0008  bne cr6, 0x82438a2c
	if !ctx.cr[6].eq {
	pc = 0x82438A2C; continue 'dispatch;
	}
	// 82438A28: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	pc = 0x82438A2C; continue 'dispatch;
            }
            0x82438A2C => {
    //   block [0x82438A2C..0x82438A3C)
	// 82438A2C: 7D0B4378  mr r11, r8
	ctx.r[11].u64 = ctx.r[8].u64;
	// 82438A30: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82438A34: 39200009  li r9, 9
	ctx.r[9].s64 = 9;
	// 82438A38: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82438A3C; continue 'dispatch;
            }
            0x82438A3C => {
    //   block [0x82438A3C..0x82438A54)
	// 82438A3C: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82438A40: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438A44: 4200FFF8  bdnz 0x82438a3c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438A3C; continue 'dispatch;
	}
	// 82438A48: 39680024  addi r11, r8, 0x24
	ctx.r[11].s64 = ctx.r[8].s64 + 36;
	// 82438A4C: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 82438A50: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82438A54; continue 'dispatch;
            }
            0x82438A54 => {
    //   block [0x82438A54..0x82438A74)
	// 82438A54: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82438A58: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438A5C: 4200FFF8  bdnz 0x82438a54
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438A54; continue 'dispatch;
	}
	// 82438A60: 7F0A2840  cmplw cr6, r10, r5
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[5].u32, &mut ctx.xer);
	// 82438A64: 409A0020  bne cr6, 0x82438a84
	if !ctx.cr[6].eq {
	pc = 0x82438A84; continue 'dispatch;
	}
	// 82438A68: 39680044  addi r11, r8, 0x44
	ctx.r[11].s64 = ctx.r[8].s64 + 68;
	// 82438A6C: 392000CF  li r9, 0xcf
	ctx.r[9].s64 = 207;
	// 82438A70: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82438A74; continue 'dispatch;
            }
            0x82438A74 => {
    //   block [0x82438A74..0x82438A84)
	// 82438A74: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82438A78: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438A7C: 4200FFF8  bdnz 0x82438a74
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438A74; continue 'dispatch;
	}
	// 82438A80: 4800003C  b 0x82438abc
	pc = 0x82438ABC; continue 'dispatch;
            }
            0x82438A84 => {
    //   block [0x82438A84..0x82438AA0)
	// 82438A84: 3D202793  lis r9, 0x2793
	ctx.r[9].s64 = 663945216;
	// 82438A88: 7D6A2850  subf r11, r10, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[10].s64;
	// 82438A8C: 61272B49  ori r7, r9, 0x2b49
	ctx.r[7].u64 = ctx.r[9].u64 | 11081;
	// 82438A90: 39280044  addi r9, r8, 0x44
	ctx.r[9].s64 = ctx.r[8].s64 + 68;
	// 82438A94: 7D6B3816  mulhwu r11, r11, r7
	ctx.r[11].u64 = ((ctx.r[11].u32 as u64 * ctx.r[7].u32 as u64) >> 32);
	// 82438A98: 5567D97E  srwi r7, r11, 5
	ctx.r[7].u32 = ctx.r[11].u32.wrapping_shr(5);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82438A9C: 396000CF  li r11, 0xcf
	ctx.r[11].s64 = 207;
	pc = 0x82438AA0; continue 'dispatch;
            }
            0x82438AA0 => {
    //   block [0x82438AA0..0x82438ABC)
	// 82438AA0: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82438AA4: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82438AA8: 7D4A3A14  add r10, r10, r7
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[7].u64;
	// 82438AAC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82438AB0: 90690000  stw r3, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82438AB4: 39290004  addi r9, r9, 4
	ctx.r[9].s64 = ctx.r[9].s64 + 4;
	// 82438AB8: 409AFFE8  bne cr6, 0x82438aa0
	if !ctx.cr[6].eq {
	pc = 0x82438AA0; continue 'dispatch;
	}
	pc = 0x82438ABC; continue 'dispatch;
            }
            0x82438ABC => {
    //   block [0x82438ABC..0x82438AC8)
	// 82438ABC: 39680380  addi r11, r8, 0x380
	ctx.r[11].s64 = ctx.r[8].s64 + 896;
	// 82438AC0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82438AC4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x82438AC8; continue 'dispatch;
            }
            0x82438AC8 => {
    //   block [0x82438AC8..0x82438AE4)
	// 82438AC8: 90AB0000  stw r5, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82438ACC: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438AD0: 4200FFF8  bdnz 0x82438ac8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438AC8; continue 'dispatch;
	}
	// 82438AD4: 396803C0  addi r11, r8, 0x3c0
	ctx.r[11].s64 = ctx.r[8].s64 + 960;
	// 82438AD8: 7C892378  mr r9, r4
	ctx.r[9].u64 = ctx.r[4].u64;
	// 82438ADC: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82438AE0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	pc = 0x82438AE4; continue 'dispatch;
            }
            0x82438AE4 => {
    //   block [0x82438AE4..0x82438AFC)
	// 82438AE4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82438AE8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438AEC: 4200FFF8  bdnz 0x82438ae4
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82438AE4; continue 'dispatch;
	}
	// 82438AF0: 397F0001  addi r11, r31, 1
	ctx.r[11].s64 = ctx.r[31].s64 + 1;
	// 82438AF4: 39460008  addi r10, r6, 8
	ctx.r[10].s64 = ctx.r[6].s64 + 8;
	// 82438AF8: 39200040  li r9, 0x40
	ctx.r[9].s64 = 64;
	pc = 0x82438AFC; continue 'dispatch;
            }
            0x82438AFC => {
    //   block [0x82438AFC..0x82438B58)
	// 82438AFC: 88EBFFFF  lbz r7, -1(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(-1 as u32) ) } as u64;
	// 82438B00: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82438B04: 54E7103E  rotlwi r7, r7, 2
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 82438B08: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82438B0C: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82438B10: 90EAFFF8  stw r7, -8(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-8 as u32), ctx.r[7].u32 ) };
	// 82438B14: 88EB0000  lbz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438B18: 54E7103E  rotlwi r7, r7, 2
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 82438B1C: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82438B20: 90EAFFFC  stw r7, -4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-4 as u32), ctx.r[7].u32 ) };
	// 82438B24: 88EB0001  lbz r7, 1(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82438B28: 54E7103E  rotlwi r7, r7, 2
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 82438B2C: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82438B30: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82438B34: 88EB0002  lbz r7, 2(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82438B38: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438B3C: 54E7103E  rotlwi r7, r7, 2
	ctx.r[7].u64 = ((ctx.r[7].u32).rotate_left(2)) as u64;
	// 82438B40: 7CE7402E  lwzx r7, r7, r8
	ctx.r[7].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82438B44: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82438B48: 394A0010  addi r10, r10, 0x10
	ctx.r[10].s64 = ctx.r[10].s64 + 16;
	// 82438B4C: 409AFFB0  bne cr6, 0x82438afc
	if !ctx.cr[6].eq {
	pc = 0x82438AFC; continue 'dispatch;
	}
	// 82438B50: EBE1FFF8  ld r31, -8(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) };
	// 82438B54: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438B58 size=16
    let mut pc: u32 = 0x82438B58;
    'dispatch: loop {
        match pc {
            0x82438B58 => {
    //   block [0x82438B58..0x82438B68)
	// 82438B58: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82438B5C: 396B0720  addi r11, r11, 0x720
	ctx.r[11].s64 = ctx.r[11].s64 + 1824;
	// 82438B60: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82438B64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438B68 size=16
    let mut pc: u32 = 0x82438B68;
    'dispatch: loop {
        match pc {
            0x82438B68 => {
    //   block [0x82438B68..0x82438B78)
	// 82438B68: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82438B6C: 396B0720  addi r11, r11, 0x720
	ctx.r[11].s64 = ctx.r[11].s64 + 1824;
	// 82438B70: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82438B74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438B78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438B78 size=76
    let mut pc: u32 = 0x82438B78;
    'dispatch: loop {
        match pc {
            0x82438B78 => {
    //   block [0x82438B78..0x82438BC4)
	// 82438B78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438B7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82438B80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82438B84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438B88: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82438B8C: 38A0026C  li r5, 0x26c
	ctx.r[5].s64 = 620;
	// 82438B90: 3BEB0720  addi r31, r11, 0x720
	ctx.r[31].s64 = ctx.r[11].s64 + 1824;
	// 82438B94: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82438B98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438B9C: 480FC635  bl 0x825351d0
	ctx.lr = 0x82438BA0;
	sub_825351D0(ctx, base);
	// 82438BA0: 39600008  li r11, 8
	ctx.r[11].s64 = 8;
	// 82438BA4: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82438BA8: 917F0008  stw r11, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82438BAC: 4BFFFFAD  bl 0x82438b58
	ctx.lr = 0x82438BB0;
	sub_82438B58(ctx, base);
	// 82438BB0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82438BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438BBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82438BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438BC8 size=84
    let mut pc: u32 = 0x82438BC8;
    'dispatch: loop {
        match pc {
            0x82438BC8 => {
    //   block [0x82438BC8..0x82438BF0)
	// 82438BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82438BD0: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438BD4: 4BFFFAD5  bl 0x824386a8
	ctx.lr = 0x82438BD8;
	sub_824386A8(ctx, base);
	// 82438BD8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82438BDC: 409A0014  bne cr6, 0x82438bf0
	if !ctx.cr[6].eq {
	pc = 0x82438BF0; continue 'dispatch;
	}
	// 82438BE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82438BE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438BE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438BEC: 4E800020  blr
	return;
            }
            0x82438BF0 => {
    //   block [0x82438BF0..0x82438C1C)
	// 82438BF0: 4BFFFAF9  bl 0x824386e8
	ctx.lr = 0x82438BF4;
	sub_824386E8(ctx, base);
	// 82438BF4: 3D608313  lis r11, -0x7ced
	ctx.r[11].s64 = -2095906816;
	// 82438BF8: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82438BFC: 814B0720  lwz r10, 0x720(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1824 as u32) ) } as u64;
	// 82438C00: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82438C04: 914B0720  stw r10, 0x720(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(1824 as u32), ctx.r[10].u32 ) };
	// 82438C08: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82438C0C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82438C10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438C14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438C18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438C20 size=20
    let mut pc: u32 = 0x82438C20;
    'dispatch: loop {
        match pc {
            0x82438C20 => {
    //   block [0x82438C20..0x82438C34)
	// 82438C20: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82438C24: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82438C28: 90A30010  stw r5, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82438C2C: 91630008  stw r11, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[11].u32 ) };
	// 82438C30: 4BFFFB08  b 0x82438738
	sub_82438738(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438C38 size=236
    let mut pc: u32 = 0x82438C38;
    'dispatch: loop {
        match pc {
            0x82438C38 => {
    //   block [0x82438C38..0x82438C9C)
	// 82438C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438C3C: 480FC479  bl 0x825350b4
	ctx.lr = 0x82438C40;
	sub_82535080(ctx, base);
	// 82438C40: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438C44: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438C48: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 82438C4C: 3BCB48DC  addi r30, r11, 0x48dc
	ctx.r[30].s64 = ctx.r[11].s64 + 18652;
	// 82438C50: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438C54: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82438C58: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82438C5C: 388B5D98  addi r4, r11, 0x5d98
	ctx.r[4].s64 = ctx.r[11].s64 + 23960;
	// 82438C60: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82438C64: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82438C68: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82438C6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82438C70: 4BFFFB89  bl 0x824387f8
	ctx.lr = 0x82438C74;
	sub_824387F8(ctx, base);
	// 82438C74: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82438C78: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82438C7C: 409A0020  bne cr6, 0x82438c9c
	if !ctx.cr[6].eq {
	pc = 0x82438C9C; continue 'dispatch;
	}
	// 82438C80: 3D407FFF  lis r10, 0x7fff
	ctx.r[10].s64 = 2147418112;
	// 82438C84: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82438C88: 614AFFFF  ori r10, r10, 0xffff
	ctx.r[10].u64 = ctx.r[10].u64 | 65535;
	// 82438C8C: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82438C90: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82438C94: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82438C98: 480FC46C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82438C9C => {
    //   block [0x82438C9C..0x82438CE8)
	// 82438C9C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438CA0: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82438CA4: 388B5D94  addi r4, r11, 0x5d94
	ctx.r[4].s64 = ctx.r[11].s64 + 23956;
	// 82438CA8: 480FEF59  bl 0x82537c00
	ctx.lr = 0x82438CAC;
	sub_82537C00(ctx, base);
	// 82438CAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82438CB0: 38E10054  addi r7, r1, 0x54
	ctx.r[7].s64 = ctx.r[1].s64 + 84;
	// 82438CB4: 388B5D8C  addi r4, r11, 0x5d8c
	ctx.r[4].s64 = ctx.r[11].s64 + 23948;
	// 82438CB8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82438CBC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82438CC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438CC4: 4BFFFB35  bl 0x824387f8
	ctx.lr = 0x82438CC8;
	sub_824387F8(ctx, base);
	// 82438CC8: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82438CCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82438CD0: 409A0018  bne cr6, 0x82438ce8
	if !ctx.cr[6].eq {
	pc = 0x82438CE8; continue 'dispatch;
	}
	// 82438CD4: 3D607FFF  lis r11, 0x7fff
	ctx.r[11].s64 = 2147418112;
	// 82438CD8: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 82438CDC: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82438CE0: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82438CE4: 480FC420  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82438CE8 => {
    //   block [0x82438CE8..0x82438D24)
	// 82438CE8: 81410058  lwz r10, 0x58(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82438CEC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82438CF0: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82438CF4: 7D4AD9D6  mullw r10, r10, r27
	ctx.r[10].s32 = ((ctx.r[10].s32 as i64 * ctx.r[27].s32 as i64) as i32);
	ctx.r[10].s64 = ctx.r[10].s32 as i64;
	// 82438CF8: 38895D80  addi r4, r9, 0x5d80
	ctx.r[4].s64 = ctx.r[9].s64 + 23936;
	// 82438CFC: 38C1005C  addi r6, r1, 0x5c
	ctx.r[6].s64 = ctx.r[1].s64 + 92;
	// 82438D00: 38A10064  addi r5, r1, 0x64
	ctx.r[5].s64 = ctx.r[1].s64 + 100;
	// 82438D04: 7C6A5A14  add r3, r10, r11
	ctx.r[3].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82438D08: 480FEEF9  bl 0x82537c00
	ctx.lr = 0x82438D0C;
	sub_82537C00(ctx, base);
	// 82438D0C: 8161005C  lwz r11, 0x5c(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 82438D10: 81410060  lwz r10, 0x60(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82438D14: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82438D18: 915C0000  stw r10, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82438D1C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82438D20: 480FC3E4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438D28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82438D28 size=80
    let mut pc: u32 = 0x82438D28;
    'dispatch: loop {
        match pc {
            0x82438D28 => {
    //   block [0x82438D28..0x82438D6C)
	// 82438D28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438D2C: 480FC38D  bl 0x825350b8
	ctx.lr = 0x82438D30;
	sub_82535080(ctx, base);
	// 82438D30: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438D34: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82438D38: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82438D3C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82438D40: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82438D44: 4BFFE60D  bl 0x82437350
	ctx.lr = 0x82438D48;
	sub_82437350(ctx, base);
	// 82438D48: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82438D4C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82438D50: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82438D54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82438D58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438D5C: 409A0010  bne cr6, 0x82438d6c
	if !ctx.cr[6].eq {
	pc = 0x82438D6C; continue 'dispatch;
	}
	// 82438D60: 4BFFFC11  bl 0x82438970
	ctx.lr = 0x82438D64;
	sub_82438970(ctx, base);
	// 82438D64: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82438D68: 480FC3A0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x82438D6C => {
    //   block [0x82438D6C..0x82438D78)
	// 82438D6C: 4BFFFB15  bl 0x82438880
	ctx.lr = 0x82438D70;
	sub_82438880(ctx, base);
	// 82438D70: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82438D74: 480FC394  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82438D78 size=252
    let mut pc: u32 = 0x82438D78;
    'dispatch: loop {
        match pc {
            0x82438D78 => {
    //   block [0x82438D78..0x82438DA4)
	// 82438D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438D7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82438D80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438D84: C1A3003C  lfs f13, 0x3c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82438D88: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82438D8C: C0030040  lfs f0, 0x40(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82438D90: 7CA92B78  mr r9, r5
	ctx.r[9].u64 = ctx.r[5].u64;
	// 82438D94: 4BFFFDD5  bl 0x82438b68
	ctx.lr = 0x82438D98;
	sub_82438B68(ctx, base);
	// 82438D98: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82438D9C: 409A0038  bne cr6, 0x82438dd4
	if !ctx.cr[6].eq {
	pc = 0x82438DD4; continue 'dispatch;
	}
	// 82438DA0: 39600100  li r11, 0x100
	ctx.r[11].s64 = 256;
	pc = 0x82438DA4; continue 'dispatch;
            }
            0x82438DA4 => {
    //   block [0x82438DA4..0x82438DD4)
	// 82438DA4: 810A0000  lwz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438DA8: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82438DAC: 394A0004  addi r10, r10, 4
	ctx.r[10].s64 = ctx.r[10].s64 + 4;
	// 82438DB0: 55088BFE  srwi r8, r8, 0xf
	ctx.r[8].u32 = ctx.r[8].u32.wrapping_shr(15);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82438DB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82438DB8: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82438DBC: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82438DC0: 409AFFE4  bne cr6, 0x82438da4
	if !ctx.cr[6].eq {
	pc = 0x82438DA4; continue 'dispatch;
	}
	// 82438DC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438DC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438DCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438DD0: 4E800020  blr
	return;
            }
            0x82438DD4 => {
    //   block [0x82438DD4..0x82438E10)
	// 82438DD4: FD606828  fsub f11, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 82438DD8: 3D00820D  lis r8, -0x7df3
	ctx.r[8].s64 = -2113077248;
	// 82438DDC: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82438DE0: 39400100  li r10, 0x100
	ctx.r[10].s64 = 256;
	// 82438DE4: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82438DE8: C9882000  lfd f12, 0x2000(r8)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(8192 as u32) ) };
	// 82438DEC: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82438DF0: FD8C5824  fdiv f12, f12, f11
	ctx.f[12].f64 = ctx.f[12].f64 / ctx.f[11].f64;
	// 82438DF4: FD8C0032  fmul f12, f12, f0
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64;
	// 82438DF8: FD6C0372  fmul f11, f12, f13
	ctx.f[11].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 82438DFC: C9A85DA8  lfd f13, 0x5da8(r8)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(23976 as u32) ) };
	// 82438E00: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82438E04: FD8C0372  fmul f12, f12, f13
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 82438E08: FDAB0372  fmul f13, f11, f13
	ctx.f[13].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 82438E0C: C9685DA0  lfd f11, 0x5da0(r8)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[8].u32.wrapping_add(23968 as u32) ) };
	pc = 0x82438E10; continue 'dispatch;
            }
            0x82438E10 => {
    //   block [0x82438E10..0x82438E20)
	// 82438E10: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438E14: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82438E18: 409A0008  bne cr6, 0x82438e20
	if !ctx.cr[6].eq {
	pc = 0x82438E20; continue 'dispatch;
	}
	// 82438E1C: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	pc = 0x82438E20; continue 'dispatch;
            }
            0x82438E20 => {
    //   block [0x82438E20..0x82438E74)
	// 82438E20: 810B0000  lwz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438E24: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82438E28: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438E2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82438E30: F9010050  std r8, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u64 ) };
	// 82438E34: C9410050  lfd f10, 0x50(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82438E38: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 82438E3C: FD4A0032  fmul f10, f10, f0
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64;
	// 82438E40: FD4A02F2  fmul f10, f10, f11
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[11].f64;
	// 82438E44: FD4D5024  fdiv f10, f13, f10
	ctx.f[10].f64 = ctx.f[13].f64 / ctx.f[10].f64;
	// 82438E48: FD4C5028  fsub f10, f12, f10
	ctx.f[10].f64 = ctx.f[12].f64 - ctx.f[10].f64;
	// 82438E4C: FD40565E  fctidz f10, f10
	ctx.f[10].s64 = if ctx.f[10].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[10].f64.trunc() as i64 };
	// 82438E50: D9410058  stfd f10, 0x58(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.f[10].u64 ) };
	// 82438E54: A101005E  lhz r8, 0x5e(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[1].u32.wrapping_add(94 as u32) ) } as u64;
	// 82438E58: B1090000  sth r8, 0(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 82438E5C: 39290002  addi r9, r9, 2
	ctx.r[9].s64 = ctx.r[9].s64 + 2;
	// 82438E60: 409AFFB0  bne cr6, 0x82438e10
	if !ctx.cr[6].eq {
	pc = 0x82438E10; continue 'dispatch;
	}
	// 82438E64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82438E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438E70: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82438E78 size=240
    let mut pc: u32 = 0x82438E78;
    'dispatch: loop {
        match pc {
            0x82438E78 => {
    //   block [0x82438E78..0x82438EA4)
	// 82438E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82438E80: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438E84: C1A3003C  lfs f13, 0x3c(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(60 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82438E88: C0030040  lfs f0, 0x40(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(64 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82438E8C: 4BFFFCDD  bl 0x82438b68
	ctx.lr = 0x82438E90;
	sub_82438B68(ctx, base);
	// 82438E90: 2F030001  cmpwi cr6, r3, 1
	ctx.cr[6].compare_i32(ctx.r[3].s32, 1, &mut ctx.xer);
	// 82438E94: 39400100  li r10, 0x100
	ctx.r[10].s64 = 256;
	// 82438E98: 409A0038  bne cr6, 0x82438ed0
	if !ctx.cr[6].eq {
	pc = 0x82438ED0; continue 'dispatch;
	}
	// 82438E9C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82438EA0: 7D252050  subf r9, r5, r4
	ctx.r[9].s64 = ctx.r[4].s64 - ctx.r[5].s64;
	pc = 0x82438EA4; continue 'dispatch;
            }
            0x82438EA4 => {
    //   block [0x82438EA4..0x82438ED0)
	// 82438EA4: 7D09582E  lwzx r8, r9, r11
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82438EA8: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82438EAC: 5508082E  rlwinm r8, r8, 1, 0, 0x17
	ctx.r[8].u64 = ctx.r[8].u32 as u64 & 0x7FFFFFFFu64;
	// 82438EB0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82438EB4: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82438EB8: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438EBC: 409AFFE8  bne cr6, 0x82438ea4
	if !ctx.cr[6].eq {
	pc = 0x82438EA4; continue 'dispatch;
	}
	// 82438EC0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82438EC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438EC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438ECC: 4E800020  blr
	return;
            }
            0x82438ED0 => {
    //   block [0x82438ED0..0x82438F0C)
	// 82438ED0: FD606828  fsub f11, f0, f13
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[11].f64 = ctx.f[0].f64 - ctx.f[13].f64;
	// 82438ED4: 3D20820D  lis r9, -0x7df3
	ctx.r[9].s64 = -2113077248;
	// 82438ED8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82438EDC: 7D042850  subf r8, r4, r5
	ctx.r[8].s64 = ctx.r[5].s64 - ctx.r[4].s64;
	// 82438EE0: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82438EE4: C9892000  lfd f12, 0x2000(r9)
	ctx.f[12].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8192 as u32) ) };
	// 82438EE8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82438EEC: FD8C5824  fdiv f12, f12, f11
	ctx.f[12].f64 = ctx.f[12].f64 / ctx.f[11].f64;
	// 82438EF0: FD8C0032  fmul f12, f12, f0
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[0].f64;
	// 82438EF4: FD6C0372  fmul f11, f12, f13
	ctx.f[11].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 82438EF8: C9A95DB0  lfd f13, 0x5db0(r9)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(23984 as u32) ) };
	// 82438EFC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82438F00: FD8C0372  fmul f12, f12, f13
	ctx.f[12].f64 = ctx.f[12].f64 * ctx.f[13].f64;
	// 82438F04: FDAB0372  fmul f13, f11, f13
	ctx.f[13].f64 = ctx.f[11].f64 * ctx.f[13].f64;
	// 82438F08: C9695DA0  lfd f11, 0x5da0(r9)
	ctx.f[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(23968 as u32) ) };
	pc = 0x82438F0C; continue 'dispatch;
            }
            0x82438F0C => {
    //   block [0x82438F0C..0x82438F1C)
	// 82438F0C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438F10: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82438F14: 409A0008  bne cr6, 0x82438f1c
	if !ctx.cr[6].eq {
	pc = 0x82438F1C; continue 'dispatch;
	}
	// 82438F18: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	pc = 0x82438F1C; continue 'dispatch;
            }
            0x82438F1C => {
    //   block [0x82438F1C..0x82438F68)
	// 82438F1C: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82438F20: 7CC85A14  add r6, r8, r11
	ctx.r[6].u64 = ctx.r[8].u64 + ctx.r[11].u64;
	// 82438F24: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82438F28: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 82438F2C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82438F30: F9210050  std r9, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u64 ) };
	// 82438F34: C9410050  lfd f10, 0x50(r1)
	ctx.f[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 82438F38: FD40569C  fcfid f10, f10
	ctx.f[10].f64 = (ctx.f[10].s64 as f64);
	// 82438F3C: FD4A0032  fmul f10, f10, f0
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[0].f64;
	// 82438F40: FD4A02F2  fmul f10, f10, f11
	ctx.f[10].f64 = ctx.f[10].f64 * ctx.f[11].f64;
	// 82438F44: FD4D5024  fdiv f10, f13, f10
	ctx.f[10].f64 = ctx.f[13].f64 / ctx.f[10].f64;
	// 82438F48: FD4C5028  fsub f10, f12, f10
	ctx.f[10].f64 = ctx.f[12].f64 - ctx.f[10].f64;
	// 82438F4C: FD40565E  fctidz f10, f10
	ctx.f[10].s64 = if ctx.f[10].f64 > (i64::MAX as f64) { i64::MAX } else { ctx.f[10].f64.trunc() as i64 };
	// 82438F50: 7D4037AE  stfiwx f10, 0, r6
	tmp.f32 = (ctx.f[10].f64 as f32);
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[6].u32, tmp.u32) };
	// 82438F54: 409AFFB8  bne cr6, 0x82438f0c
	if !ctx.cr[6].eq {
	pc = 0x82438F0C; continue 'dispatch;
	}
	// 82438F58: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82438F5C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82438F60: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82438F64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438F68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82438F68 size=4
    let mut pc: u32 = 0x82438F68;
    'dispatch: loop {
        match pc {
            0x82438F68 => {
    //   block [0x82438F68..0x82438F6C)
	// 82438F68: 4BFFFC10  b 0x82438b78
	sub_82438B78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82438F70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82438F70 size=160
    let mut pc: u32 = 0x82438F70;
    'dispatch: loop {
        match pc {
            0x82438F70 => {
    //   block [0x82438F70..0x82438FE4)
	// 82438F70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82438F74: 480FC141  bl 0x825350b4
	ctx.lr = 0x82438F78;
	sub_82535080(ctx, base);
	// 82438F78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82438F7C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82438F80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82438F84: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82438F88: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82438F8C: 38A00400  li r5, 0x400
	ctx.r[5].s64 = 1024;
	// 82438F90: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82438F94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82438F98: 3BBE0400  addi r29, r30, 0x400
	ctx.r[29].s64 = ctx.r[30].s64 + 1024;
	// 82438F9C: 480FC235  bl 0x825351d0
	ctx.lr = 0x82438FA0;
	sub_825351D0(ctx, base);
	// 82438FA0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82438FA4: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82438FA8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82438FAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438FB0: 4BFFFD79  bl 0x82438d28
	ctx.lr = 0x82438FB4;
	sub_82438D28(ctx, base);
	// 82438FB4: 817F0044  lwz r11, 0x44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(68 as u32) ) } as u64;
	// 82438FB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82438FBC: 409A0034  bne cr6, 0x82438ff0
	if !ctx.cr[6].eq {
	pc = 0x82438FF0; continue 'dispatch;
	}
	// 82438FC0: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82438FC4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82438FC8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82438FCC: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 82438FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82438FD4: 409A0010  bne cr6, 0x82438fe4
	if !ctx.cr[6].eq {
	pc = 0x82438FE4; continue 'dispatch;
	}
	// 82438FD8: 4BFFFDA1  bl 0x82438d78
	ctx.lr = 0x82438FDC;
	sub_82438D78(ctx, base);
	// 82438FDC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82438FE0: 480FC124  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82438FE4 => {
    //   block [0x82438FE4..0x82438FF0)
	// 82438FE4: 4BFFFE95  bl 0x82438e78
	ctx.lr = 0x82438FE8;
	sub_82438E78(ctx, base);
	// 82438FE8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82438FEC: 480FC118  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x82438FF0 => {
    //   block [0x82438FF0..0x82439010)
	// 82438FF0: C05F0040  lfs f2, 0x40(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(64 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 82438FF4: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82438FF8: C03F003C  lfs f1, 0x3c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(60 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82438FFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82439000: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82439004: 4E800421  bctrl
	ctx.lr = 0x82439008;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82439008: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8243900C: 480FC0F8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439010(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439010 size=84
    let mut pc: u32 = 0x82439010;
    'dispatch: loop {
        match pc {
            0x82439010 => {
    //   block [0x82439010..0x82439064)
	// 82439010: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439014: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439018: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243901C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439020: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439024: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82439028: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8243902C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82439030: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439034: 4BFFFC05  bl 0x82438c38
	ctx.lr = 0x82439038;
	sub_82438C38(ctx, base);
	// 82439038: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 8243903C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439040: 80A10050  lwz r5, 0x50(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82439044: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82439048: 4BFFFF29  bl 0x82438f70
	ctx.lr = 0x8243904C;
	sub_82438F70(ctx, base);
	// 8243904C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439058: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243905C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439068 size=56
    let mut pc: u32 = 0x82439068;
    'dispatch: loop {
        match pc {
            0x82439068 => {
    //   block [0x82439068..0x824390A0)
	// 82439068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243906C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439070: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439074: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82439078: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 8243907C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82439080: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82439084: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82439088: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8243908C: 4800FE6D  bl 0x82448ef8
	ctx.lr = 0x82439090;
	sub_82448EF8(ctx, base);
	// 82439090: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82439094: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439098: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243909C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824390A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824390A0 size=20
    let mut pc: u32 = 0x824390A0;
    'dispatch: loop {
        match pc {
            0x824390A0 => {
    //   block [0x824390A0..0x824390B4)
	// 824390A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824390A4: 39400003  li r10, 3
	ctx.r[10].s64 = 3;
	// 824390A8: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824390AC: 914B004C  stw r10, 0x4c(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(76 as u32), ctx.r[10].u32 ) };
	// 824390B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824390B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824390B8 size=104
    let mut pc: u32 = 0x824390B8;
    'dispatch: loop {
        match pc {
            0x824390B8 => {
    //   block [0x824390B8..0x824390F4)
	// 824390B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824390BC: 480FC001  bl 0x825350bc
	ctx.lr = 0x824390C0;
	sub_82535080(ctx, base);
	// 824390C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824390C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824390C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 824390CC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 824390D0: 4800E771  bl 0x82447840
	ctx.lr = 0x824390D4;
	sub_82447840(ctx, base);
	// 824390D4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824390D8: 419A001C  beq cr6, 0x824390f4
	if ctx.cr[6].eq {
	pc = 0x824390F4; continue 'dispatch;
	}
	// 824390DC: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824390E0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824390E4: 60840144  ori r4, r4, 0x144
	ctx.r[4].u64 = ctx.r[4].u64 | 324;
	// 824390E8: 4800E821  bl 0x82447908
	ctx.lr = 0x824390EC;
	sub_82447908(ctx, base);
	// 824390EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824390F0: 480FC01C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            0x824390F4 => {
    //   block [0x824390F4..0x82439120)
	// 824390F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 824390F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 824390FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439100: 48006119  bl 0x8243f218
	ctx.lr = 0x82439104;
	sub_8243F218(ctx, base);
	// 82439104: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82439108: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243910C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439110: 4800C9D1  bl 0x82445ae0
	ctx.lr = 0x82439114;
	sub_82445AE0(ctx, base);
	// 82439114: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439118: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8243911C: 480FBFF0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439120 size=104
    let mut pc: u32 = 0x82439120;
    'dispatch: loop {
        match pc {
            0x82439120 => {
    //   block [0x82439120..0x82439158)
	// 82439120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439124: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439128: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243912C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439130: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439134: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439138: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243913C: 817F004C  lwz r11, 0x4c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(76 as u32) ) } as u64;
	// 82439140: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82439144: 419A0014  beq cr6, 0x82439158
	if ctx.cr[6].eq {
	pc = 0x82439158; continue 'dispatch;
	}
	// 82439148: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 8243914C: 419A000C  beq cr6, 0x82439158
	if ctx.cr[6].eq {
	pc = 0x82439158; continue 'dispatch;
	}
	// 82439150: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439154: 4800001C  b 0x82439170
	pc = 0x82439170; continue 'dispatch;
            }
            0x82439158 => {
    //   block [0x82439158..0x82439170)
	// 82439158: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8243915C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439160: 48006789  bl 0x8243f8e8
	ctx.lr = 0x82439164;
	sub_8243F8E8(ctx, base);
	// 82439164: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82439168: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243916C: 4BFFFEFD  bl 0x82439068
	ctx.lr = 0x82439170;
	sub_82439068(ctx, base);
	pc = 0x82439170; continue 'dispatch;
            }
            0x82439170 => {
    //   block [0x82439170..0x82439188)
	// 82439170: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439174: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439178: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243917C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82439180: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439184: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439188 size=96
    let mut pc: u32 = 0x82439188;
    'dispatch: loop {
        match pc {
            0x82439188 => {
    //   block [0x82439188..0x824391CC)
	// 82439188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243918C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439190: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439194: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439198: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243919C: 4800E6A5  bl 0x82447840
	ctx.lr = 0x824391A0;
	sub_82447840(ctx, base);
	// 824391A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824391A4: 419A0028  beq cr6, 0x824391cc
	if ctx.cr[6].eq {
	pc = 0x824391CC; continue 'dispatch;
	}
	// 824391A8: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 824391AC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824391B0: 60840143  ori r4, r4, 0x143
	ctx.r[4].u64 = ctx.r[4].u64 | 323;
	// 824391B4: 4800E755  bl 0x82447908
	ctx.lr = 0x824391B8;
	sub_82447908(ctx, base);
	// 824391B8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824391BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824391C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824391C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824391C8: 4E800020  blr
	return;
            }
            0x824391CC => {
    //   block [0x824391CC..0x824391E8)
	// 824391CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824391D0: 4BFFFED1  bl 0x824390a0
	ctx.lr = 0x824391D4;
	sub_824390A0(ctx, base);
	// 824391D4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 824391D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824391DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824391E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824391E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824391E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x824391E8 size=52
    let mut pc: u32 = 0x824391E8;
    'dispatch: loop {
        match pc {
            0x824391E8 => {
    //   block [0x824391E8..0x8243921C)
	// 824391E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 824391EC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824391F0: 2B040001  cmplwi cr6, r4, 1
	ctx.cr[6].compare_u32(ctx.r[4].u32, 1 as u32, &mut ctx.xer);
	// 824391F4: 41980048  blt cr6, 0x8243923c
	if ctx.cr[6].lt {
		sub_8243923C(ctx, base);
		return;
	}
	// 824391F8: 419A0024  beq cr6, 0x8243921c
	if ctx.cr[6].eq {
		sub_8243921C(ctx, base);
		return;
	}
	// 824391FC: 2B040003  cmplwi cr6, r4, 3
	ctx.cr[6].compare_u32(ctx.r[4].u32, 3 as u32, &mut ctx.xer);
	// 82439200: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
	// 82439204: 814B0048  lwz r10, 0x48(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(72 as u32) ) } as u64;
	// 82439208: 2F0A0004  cmpwi cr6, r10, 4
	ctx.cr[6].compare_i32(ctx.r[10].s32, 4, &mut ctx.xer);
	// 8243920C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82439210: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82439214: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82439218: 4BFFFF08  b 0x82439120
	sub_82439120(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243921C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243921C size=32
    let mut pc: u32 = 0x8243921C;
    'dispatch: loop {
        match pc {
            0x8243921C => {
    //   block [0x8243921C..0x8243923C)
	// 8243921C: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82439220: 392A0001  addi r9, r10, 1
	ctx.r[9].s64 = ctx.r[10].s64 + 1;
	// 82439224: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82439228: 912B0054  stw r9, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8243922C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	// 82439230: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82439234: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82439238: 4BFFFEE8  b 0x82439120
	sub_82439120(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8243923C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8243923C size=28
    let mut pc: u32 = 0x8243923C;
    'dispatch: loop {
        match pc {
            0x8243923C => {
    //   block [0x8243923C..0x82439258)
	// 8243923C: 814B0054  lwz r10, 0x54(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 82439240: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82439244: 914B0054  stw r10, 0x54(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82439248: 4C820020  bnelr
	if !ctx.cr[0].eq { return; }
	// 8243924C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82439250: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82439254: 4BFFFECC  b 0x82439120
	sub_82439120(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439258 size=4
    let mut pc: u32 = 0x82439258;
    'dispatch: loop {
        match pc {
            0x82439258 => {
    //   block [0x82439258..0x8243925C)
	// 82439258: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439260 size=156
    let mut pc: u32 = 0x82439260;
    'dispatch: loop {
        match pc {
            0x82439260 => {
    //   block [0x82439260..0x8243929C)
	// 82439260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439268: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243926C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439270: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439274: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439278: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8243927C: 4800E5C5  bl 0x82447840
	ctx.lr = 0x82439280;
	sub_82447840(ctx, base);
	// 82439280: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439284: 419A0018  beq cr6, 0x8243929c
	if ctx.cr[6].eq {
	pc = 0x8243929C; continue 'dispatch;
	}
	// 82439288: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 8243928C: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439290: 60840142  ori r4, r4, 0x142
	ctx.r[4].u64 = ctx.r[4].u64 | 322;
	// 82439294: 4800E675  bl 0x82447908
	ctx.lr = 0x82439298;
	sub_82447908(ctx, base);
	// 82439298: 4800004C  b 0x824392e4
	pc = 0x824392E4; continue 'dispatch;
            }
            0x8243929C => {
    //   block [0x8243929C..0x824392B8)
	// 8243929C: 817F0050  lwz r11, 0x50(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) } as u64;
	// 824392A0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 824392A4: 409A001C  bne cr6, 0x824392c0
	if !ctx.cr[6].eq {
	pc = 0x824392C0; continue 'dispatch;
	}
	// 824392A8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824392AC: 409A000C  bne cr6, 0x824392b8
	if !ctx.cr[6].eq {
	pc = 0x824392B8; continue 'dispatch;
	}
	// 824392B0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 824392B4: 48000030  b 0x824392e4
	pc = 0x824392E4; continue 'dispatch;
            }
            0x824392B8 => {
    //   block [0x824392B8..0x824392C0)
	// 824392B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 824392BC: 48000014  b 0x824392d0
	pc = 0x824392D0; continue 'dispatch;
            }
            0x824392C0 => {
    //   block [0x824392C0..0x824392D0)
	// 824392C0: 7D6B0034  cntlzw r11, r11
	ctx.r[11].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 824392C4: 556BDFFE  rlwinm r11, r11, 0x1b, 0x1f, 0x1f
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	// 824392C8: 696B0001  xori r11, r11, 1
	ctx.r[11].u64 = ctx.r[11].u64 ^ 1;
	// 824392CC: 388B0001  addi r4, r11, 1
	ctx.r[4].s64 = ctx.r[11].s64 + 1;
	pc = 0x824392D0; continue 'dispatch;
            }
            0x824392D0 => {
    //   block [0x824392D0..0x824392E4)
	// 824392D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824392D4: 93DF0050  stw r30, 0x50(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 824392D8: 4BFFFF11  bl 0x824391e8
	ctx.lr = 0x824392DC;
	sub_824391E8(ctx, base);
	// 824392DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824392E0: 917F0044  stw r11, 0x44(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(68 as u32), ctx.r[11].u32 ) };
	pc = 0x824392E4; continue 'dispatch;
            }
            0x824392E4 => {
    //   block [0x824392E4..0x824392FC)
	// 824392E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824392E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824392EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824392F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824392F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824392F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439300 size=16
    let mut pc: u32 = 0x82439300;
    'dispatch: loop {
        match pc {
            0x82439300 => {
    //   block [0x82439300..0x82439310)
	// 82439300: 3C80FF00  lis r4, -0x100
	ctx.r[4].s64 = -16777216;
	// 82439304: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439308: 60840201  ori r4, r4, 0x201
	ctx.r[4].u64 = ctx.r[4].u64 | 513;
	// 8243930C: 4800E5FC  b 0x82447908
	sub_82447908(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439310 size=8
    let mut pc: u32 = 0x82439310;
    'dispatch: loop {
        match pc {
            0x82439310 => {
    //   block [0x82439310..0x82439318)
	// 82439310: 48006C08  b 0x8243ff18
	sub_8243FF18(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439318(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439318 size=44
    let mut pc: u32 = 0x82439318;
    'dispatch: loop {
        match pc {
            0x82439318 => {
    //   block [0x82439318..0x82439344)
	// 82439318: 81630048  lwz r11, 0x48(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243931C: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82439320: 419A0024  beq cr6, 0x82439344
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82439344);
		return;
	}
	// 82439324: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82439328: 419A001C  beq cr6, 0x82439344
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82439344);
		return;
	}
	// 8243932C: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 82439330: 419A0014  beq cr6, 0x82439344
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82439344);
		return;
	}
	// 82439334: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82439338: 419A000C  beq cr6, 0x82439344
	if ctx.cr[6].eq {
		crate::recompiler::externs::call(ctx, base, 0x82439344);
		return;
	}
	// 8243933C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82439340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439358 size=8
    let mut pc: u32 = 0x82439358;
    'dispatch: loop {
        match pc {
            0x82439358 => {
    //   block [0x82439358..0x82439360)
	// 82439358: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 8243935C: 4800FB2C  b 0x82448e88
	sub_82448E88(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439360 size=40
    let mut pc: u32 = 0x82439360;
    'dispatch: loop {
        match pc {
            0x82439360 => {
    //   block [0x82439360..0x82439380)
	// 82439360: 8163004C  lwz r11, 0x4c(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(76 as u32) ) } as u64;
	// 82439364: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 82439368: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 8243936C: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
	// 82439370: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82439374: 4099000C  ble cr6, 0x82439380
	if !ctx.cr[6].gt {
	pc = 0x82439380; continue 'dispatch;
	}
	// 82439378: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 8243937C: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
	pc = 0x82439380; continue 'dispatch;
            }
            0x82439380 => {
    //   block [0x82439380..0x82439388)
	// 82439380: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82439384: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439388 size=164
    let mut pc: u32 = 0x82439388;
    'dispatch: loop {
        match pc {
            0x82439388 => {
    //   block [0x82439388..0x824393B0)
	// 82439388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243938C: 480FBD31  bl 0x825350bc
	ctx.lr = 0x82439390;
	sub_82535080(ctx, base);
	// 82439390: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439394: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82439398: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8243939C: 4800336D  bl 0x8243c708
	ctx.lr = 0x824393A0;
	sub_8243C708(ctx, base);
	// 824393A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824393A4: 409A000C  bne cr6, 0x824393b0
	if !ctx.cr[6].eq {
	pc = 0x824393B0; continue 'dispatch;
	}
	// 824393A8: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 824393AC: 48000024  b 0x824393d0
	pc = 0x824393D0; continue 'dispatch;
            }
            0x824393B0 => {
    //   block [0x824393B0..0x824393D0)
	// 824393B0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 824393B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824393B8: 4800FB91  bl 0x82448f48
	ctx.lr = 0x824393BC;
	sub_82448F48(ctx, base);
	// 824393BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824393C0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 824393C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824393C8: 4800FBA1  bl 0x82448f68
	ctx.lr = 0x824393CC;
	sub_82448F68(ctx, base);
	// 824393CC: 7C7DF378  or r29, r3, r30
	ctx.r[29].u64 = ctx.r[3].u64 | ctx.r[30].u64;
	pc = 0x824393D0; continue 'dispatch;
            }
            0x824393D0 => {
    //   block [0x824393D0..0x824393EC)
	// 824393D0: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 824393D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824393D8: 48003331  bl 0x8243c708
	ctx.lr = 0x824393DC;
	sub_8243C708(ctx, base);
	// 824393DC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824393E0: 409A000C  bne cr6, 0x824393ec
	if !ctx.cr[6].eq {
	pc = 0x824393EC; continue 'dispatch;
	}
	// 824393E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 824393E8: 48000024  b 0x8243940c
	pc = 0x8243940C; continue 'dispatch;
            }
            0x824393EC => {
    //   block [0x824393EC..0x8243940C)
	// 824393EC: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 824393F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824393F4: 4800FB55  bl 0x82448f48
	ctx.lr = 0x824393F8;
	sub_82448F48(ctx, base);
	// 824393F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824393FC: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82439400: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439404: 4800FB65  bl 0x82448f68
	ctx.lr = 0x82439408;
	sub_82448F68(ctx, base);
	// 82439408: 7C6BF378  or r11, r3, r30
	ctx.r[11].u64 = ctx.r[3].u64 | ctx.r[30].u64;
	pc = 0x8243940C; continue 'dispatch;
            }
            0x8243940C => {
    //   block [0x8243940C..0x82439420)
	// 8243940C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82439410: 419A0010  beq cr6, 0x82439420
	if ctx.cr[6].eq {
	pc = 0x82439420; continue 'dispatch;
	}
	// 82439414: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82439418: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8243941C: 409A0008  bne cr6, 0x82439424
	if !ctx.cr[6].eq {
	pc = 0x82439424; continue 'dispatch;
	}
	pc = 0x82439420; continue 'dispatch;
            }
            0x82439420 => {
    //   block [0x82439420..0x82439424)
	// 82439420: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	pc = 0x82439424; continue 'dispatch;
            }
            0x82439424 => {
    //   block [0x82439424..0x8243942C)
	// 82439424: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439428: 480FBCE4  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439430 size=176
    let mut pc: u32 = 0x82439430;
    'dispatch: loop {
        match pc {
            0x82439430 => {
    //   block [0x82439430..0x82439480)
	// 82439430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439438: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243943C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439444: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439448: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8243944C: 817F0A20  lwz r11, 0xa20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2592 as u32) ) } as u64;
	// 82439450: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82439454: 409A002C  bne cr6, 0x82439480
	if !ctx.cr[6].eq {
	pc = 0x82439480; continue 'dispatch;
	}
	// 82439458: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243945C: 4800EDED  bl 0x82448248
	ctx.lr = 0x82439460;
	sub_82448248(ctx, base);
	// 82439460: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439464: 409A001C  bne cr6, 0x82439480
	if !ctx.cr[6].eq {
	pc = 0x82439480; continue 'dispatch;
	}
	// 82439468: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243946C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439470: 4800EDC9  bl 0x82448238
	ctx.lr = 0x82439474;
	sub_82448238(ctx, base);
	// 82439474: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439478: 409A0008  bne cr6, 0x82439480
	if !ctx.cr[6].eq {
	pc = 0x82439480; continue 'dispatch;
	}
	// 8243947C: 93DF0A20  stw r30, 0xa20(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2592 as u32), ctx.r[30].u32 ) };
	pc = 0x82439480; continue 'dispatch;
            }
            0x82439480 => {
    //   block [0x82439480..0x824394B8)
	// 82439480: 817F0A24  lwz r11, 0xa24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 82439484: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82439488: 409A0030  bne cr6, 0x824394b8
	if !ctx.cr[6].eq {
	pc = 0x824394B8; continue 'dispatch;
	}
	// 8243948C: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82439490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439494: 4800EDB5  bl 0x82448248
	ctx.lr = 0x82439498;
	sub_82448248(ctx, base);
	// 82439498: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 8243949C: 409A001C  bne cr6, 0x824394b8
	if !ctx.cr[6].eq {
	pc = 0x824394B8; continue 'dispatch;
	}
	// 824394A0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 824394A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824394A8: 4800ED91  bl 0x82448238
	ctx.lr = 0x824394AC;
	sub_82448238(ctx, base);
	// 824394AC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824394B0: 409A0008  bne cr6, 0x824394b8
	if !ctx.cr[6].eq {
	pc = 0x824394B8; continue 'dispatch;
	}
	// 824394B4: 93DF0A24  stw r30, 0xa24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2596 as u32), ctx.r[30].u32 ) };
	pc = 0x824394B8; continue 'dispatch;
            }
            0x824394B8 => {
    //   block [0x824394B8..0x824394E0)
	// 824394B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 824394BC: 80BF0A24  lwz r5, 0xa24(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 824394C0: 809F0A20  lwz r4, 0xa20(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2592 as u32) ) } as u64;
	// 824394C4: 4800FF65  bl 0x82449428
	ctx.lr = 0x824394C8;
	sub_82449428(ctx, base);
	// 824394C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824394CC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824394D0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824394D4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824394D8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824394DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824394E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824394E0 size=160
    let mut pc: u32 = 0x824394E0;
    'dispatch: loop {
        match pc {
            0x824394E0 => {
    //   block [0x824394E0..0x82439518)
	// 824394E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824394E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824394E8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824394EC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824394F0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824394F4: 817F0A24  lwz r11, 0xa24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2596 as u32) ) } as u64;
	// 824394F8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824394FC: 409A001C  bne cr6, 0x82439518
	if !ctx.cr[6].eq {
	pc = 0x82439518; continue 'dispatch;
	}
	// 82439500: 817F0A48  lwz r11, 0xa48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2632 as u32) ) } as u64;
	// 82439504: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 82439508: 409A0010  bne cr6, 0x82439518
	if !ctx.cr[6].eq {
	pc = 0x82439518; continue 'dispatch;
	}
	// 8243950C: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82439510: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82439514: 4800323D  bl 0x8243c750
	ctx.lr = 0x82439518;
	sub_8243C750(ctx, base);
	pc = 0x82439518; continue 'dispatch;
            }
            0x82439518 => {
    //   block [0x82439518..0x82439540)
	// 82439518: 817F0A20  lwz r11, 0xa20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2592 as u32) ) } as u64;
	// 8243951C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82439520: 409A0020  bne cr6, 0x82439540
	if !ctx.cr[6].eq {
	pc = 0x82439540; continue 'dispatch;
	}
	// 82439524: 817F0A48  lwz r11, 0xa48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2632 as u32) ) } as u64;
	// 82439528: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 8243952C: 409A0014  bne cr6, 0x82439540
	if !ctx.cr[6].eq {
	pc = 0x82439540; continue 'dispatch;
	}
	// 82439530: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82439534: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82439538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8243953C: 48003215  bl 0x8243c750
	ctx.lr = 0x82439540;
	sub_8243C750(ctx, base);
	pc = 0x82439540; continue 'dispatch;
            }
            0x82439540 => {
    //   block [0x82439540..0x82439568)
	// 82439540: 817F0A48  lwz r11, 0xa48(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2632 as u32) ) } as u64;
	// 82439544: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82439548: 409A0024  bne cr6, 0x8243956c
	if !ctx.cr[6].eq {
	pc = 0x8243956C; continue 'dispatch;
	}
	// 8243954C: 817F106C  lwz r11, 0x106c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4204 as u32) ) } as u64;
	// 82439550: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 82439554: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439558: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8243955C: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82439560: 409A0008  bne cr6, 0x82439568
	if !ctx.cr[6].eq {
	pc = 0x82439568; continue 'dispatch;
	}
	// 82439564: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	pc = 0x82439568; continue 'dispatch;
            }
            0x82439568 => {
    //   block [0x82439568..0x8243956C)
	// 82439568: 480031E9  bl 0x8243c750
	ctx.lr = 0x8243956C;
	sub_8243C750(ctx, base);
	pc = 0x8243956C; continue 'dispatch;
            }
            0x8243956C => {
    //   block [0x8243956C..0x82439580)
	// 8243956C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82439570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439578: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243957C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439580(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439580 size=200
    let mut pc: u32 = 0x82439580;
    'dispatch: loop {
        match pc {
            0x82439580 => {
    //   block [0x82439580..0x824395AC)
	// 82439580: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439584: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439588: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243958C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439594: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82439598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8243959C: 815E0A24  lwz r10, 0xa24(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2596 as u32) ) } as u64;
	// 824395A0: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 824395A4: 409A0008  bne cr6, 0x824395ac
	if !ctx.cr[6].eq {
	pc = 0x824395AC; continue 'dispatch;
	}
	// 824395A8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	pc = 0x824395AC; continue 'dispatch;
            }
            0x824395AC => {
    //   block [0x824395AC..0x824395BC)
	// 824395AC: 815E0A20  lwz r10, 0xa20(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2592 as u32) ) } as u64;
	// 824395B0: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 824395B4: 409A0008  bne cr6, 0x824395bc
	if !ctx.cr[6].eq {
	pc = 0x824395BC; continue 'dispatch;
	}
	// 824395B8: 616B0002  ori r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u64 | 2;
	pc = 0x824395BC; continue 'dispatch;
            }
            0x824395BC => {
    //   block [0x824395BC..0x8243960C)
	// 824395BC: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 824395C0: 419A005C  beq cr6, 0x8243961c
	if ctx.cr[6].eq {
	pc = 0x8243961C; continue 'dispatch;
	}
	// 824395C4: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 824395C8: 419A004C  beq cr6, 0x82439614
	if ctx.cr[6].eq {
	pc = 0x82439614; continue 'dispatch;
	}
	// 824395CC: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 824395D0: 409A003C  bne cr6, 0x8243960c
	if !ctx.cr[6].eq {
	pc = 0x8243960C; continue 'dispatch;
	}
	// 824395D4: 38800019  li r4, 0x19
	ctx.r[4].s64 = 25;
	// 824395D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824395DC: 4800312D  bl 0x8243c708
	ctx.lr = 0x824395E0;
	sub_8243C708(ctx, base);
	// 824395E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 824395E4: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 824395E8: 409A0038  bne cr6, 0x82439620
	if !ctx.cr[6].eq {
	pc = 0x82439620; continue 'dispatch;
	}
	// 824395EC: 480104CD  bl 0x82449ab8
	ctx.lr = 0x824395F0;
	sub_82449AB8(ctx, base);
	// 824395F0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824395F4: 409A0018  bne cr6, 0x8243960c
	if !ctx.cr[6].eq {
	pc = 0x8243960C; continue 'dispatch;
	}
	// 824395F8: 38800048  li r4, 0x48
	ctx.r[4].s64 = 72;
	// 824395FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82439600: 48003109  bl 0x8243c708
	ctx.lr = 0x82439604;
	sub_8243C708(ctx, base);
	// 82439604: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439608: 409A0018  bne cr6, 0x82439620
	if !ctx.cr[6].eq {
	pc = 0x82439620; continue 'dispatch;
	}
	pc = 0x8243960C; continue 'dispatch;
            }
            0x8243960C => {
    //   block [0x8243960C..0x82439614)
	// 8243960C: 3BE00003  li r31, 3
	ctx.r[31].s64 = 3;
	// 82439610: 48000010  b 0x82439620
	pc = 0x82439620; continue 'dispatch;
            }
            0x82439614 => {
    //   block [0x82439614..0x8243961C)
	// 82439614: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 82439618: 48000008  b 0x82439620
	pc = 0x82439620; continue 'dispatch;
            }
            0x8243961C => {
    //   block [0x8243961C..0x82439620)
	// 8243961C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	pc = 0x82439620; continue 'dispatch;
            }
            0x82439620 => {
    //   block [0x82439620..0x82439648)
	// 82439620: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82439624: 38800019  li r4, 0x19
	ctx.r[4].s64 = 25;
	// 82439628: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243962C: 48003125  bl 0x8243c750
	ctx.lr = 0x82439630;
	sub_8243C750(ctx, base);
	// 82439630: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439634: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439638: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8243963C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82439640: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439644: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439648(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82439648 size=8
    let mut pc: u32 = 0x82439648;
    'dispatch: loop {
        match pc {
            0x82439648 => {
    //   block [0x82439648..0x82439650)
	// 82439648: 80630048  lwz r3, 0x48(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(72 as u32) ) } as u64;
	// 8243964C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439650 size=168
    let mut pc: u32 = 0x82439650;
    'dispatch: loop {
        match pc {
            0x82439650 => {
    //   block [0x82439650..0x8243968C)
	// 82439650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439658: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243965C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439664: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82439668: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8243966C: 4800309D  bl 0x8243c708
	ctx.lr = 0x82439670;
	sub_8243C708(ctx, base);
	// 82439670: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439674: 419A0020  beq cr6, 0x82439694
	if ctx.cr[6].eq {
	pc = 0x82439694; continue 'dispatch;
	}
	// 82439678: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 8243967C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82439680: 4800F8E9  bl 0x82448f68
	ctx.lr = 0x82439684;
	sub_82448F68(ctx, base);
	// 82439684: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439688: 419A000C  beq cr6, 0x82439694
	if ctx.cr[6].eq {
	pc = 0x82439694; continue 'dispatch;
	}
	pc = 0x8243968C; continue 'dispatch;
            }
            0x8243968C => {
    //   block [0x8243968C..0x82439694)
	// 8243968C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82439690: 48000050  b 0x824396e0
	pc = 0x824396E0; continue 'dispatch;
            }
            0x82439694 => {
    //   block [0x82439694..0x824396BC)
	// 82439694: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82439698: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8243969C: 4800306D  bl 0x8243c708
	ctx.lr = 0x824396A0;
	sub_8243C708(ctx, base);
	// 824396A0: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824396A4: 419A0018  beq cr6, 0x824396bc
	if ctx.cr[6].eq {
	pc = 0x824396BC; continue 'dispatch;
	}
	// 824396A8: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 824396AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824396B0: 4800F8B9  bl 0x82448f68
	ctx.lr = 0x824396B4;
	sub_82448F68(ctx, base);
	// 824396B4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824396B8: 409AFFD4  bne cr6, 0x8243968c
	if !ctx.cr[6].eq {
	pc = 0x8243968C; continue 'dispatch;
	}
	pc = 0x824396BC; continue 'dispatch;
            }
            0x824396BC => {
    //   block [0x824396BC..0x824396C0)
	// 824396BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	pc = 0x824396C0; continue 'dispatch;
            }
            0x824396C0 => {
    //   block [0x824396C0..0x824396E0)
	// 824396C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 824396C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824396C8: 4800ED39  bl 0x82448400
	ctx.lr = 0x824396CC;
	sub_82448400(ctx, base);
	// 824396CC: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 824396D0: 409AFFBC  bne cr6, 0x8243968c
	if !ctx.cr[6].eq {
	pc = 0x8243968C; continue 'dispatch;
	}
	// 824396D4: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 824396D8: 2F1F0008  cmpwi cr6, r31, 8
	ctx.cr[6].compare_i32(ctx.r[31].s32, 8, &mut ctx.xer);
	// 824396DC: 4198FFE4  blt cr6, 0x824396c0
	if ctx.cr[6].lt {
	pc = 0x824396C0; continue 'dispatch;
	}
	pc = 0x824396E0; continue 'dispatch;
            }
            0x824396E0 => {
    //   block [0x824396E0..0x824396F8)
	// 824396E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 824396E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 824396E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 824396EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 824396F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 824396F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824396F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824396F8 size=164
    let mut pc: u32 = 0x824396F8;
    'dispatch: loop {
        match pc {
            0x824396F8 => {
    //   block [0x824396F8..0x82439780)
	// 824396F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824396FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439700: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82439704: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439708: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8243970C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82439710: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82439714: 817F2050  lwz r11, 0x2050(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8272 as u32) ) } as u64;
	// 82439718: 1D6B0074  mulli r11, r11, 0x74
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 116 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 8243971C: 7FCBFA14  add r30, r11, r31
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82439720: 807E13AC  lwz r3, 0x13ac(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(5036 as u32) ) } as u64;
	// 82439724: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82439728: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 8243972C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82439730: 4E800421  bctrl
	ctx.lr = 0x82439734;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82439734: 3D4051EB  lis r10, 0x51eb
	ctx.r[10].s64 = 1374355456;
	// 82439738: 817E13B4  lwz r11, 0x13b4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(5044 as u32) ) } as u64;
	// 8243973C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82439740: 6149851F  ori r9, r10, 0x851f
	ctx.r[9].u64 = ctx.r[10].u64 | 34079;
	// 82439744: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82439748: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8243974C: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82439750: 7D6B4896  mulhw r11, r11, r9
	ctx.r[11].s64 = ((ctx.r[11].s32 as i64 * ctx.r[9].s32 as i64) >> 32);
	// 82439754: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 82439758: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8243975C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82439760: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82439764: 4098001C  bge cr6, 0x82439780
	if !ctx.cr[6].lt {
	pc = 0x82439780; continue 'dispatch;
	}
	// 82439768: 38800046  li r4, 0x46
	ctx.r[4].s64 = 70;
	// 8243976C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439770: 48002F99  bl 0x8243c708
	ctx.lr = 0x82439774;
	sub_8243C708(ctx, base);
	// 82439774: 7F1E1800  cmpw cr6, r30, r3
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[3].s32, &mut ctx.xer);
	// 82439778: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 8243977C: 41980008  blt cr6, 0x82439784
	if ctx.cr[6].lt {
	pc = 0x82439784; continue 'dispatch;
	}
            }
            0x82439780 => {
    //   block [0x82439780..0x82439784)
	// 82439780: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	pc = 0x82439784; continue 'dispatch;
            }
            0x82439784 => {
    //   block [0x82439784..0x8243979C)
	// 82439784: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439788: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8243978C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439790: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82439794: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439798: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_824397A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x824397A0 size=128
    let mut pc: u32 = 0x824397A0;
    'dispatch: loop {
        match pc {
            0x824397A0 => {
    //   block [0x824397A0..0x8243980C)
	// 824397A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 824397A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 824397A8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 824397AC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824397B0: 81632094  lwz r11, 0x2094(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8340 as u32) ) } as u64;
	// 824397B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824397B8: 1D6B0074  mulli r11, r11, 0x74
	ctx.r[11].s32 = ((ctx.r[11].s32 as i64 * 116 as i64) as i32);
	ctx.r[11].s64 = ctx.r[11].s32 as i64;
	// 824397BC: 7FEB1A14  add r31, r11, r3
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 824397C0: 807F13AC  lwz r3, 0x13ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5036 as u32) ) } as u64;
	// 824397C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 824397C8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 824397CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 824397D0: 4E800421  bctrl
	ctx.lr = 0x824397D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 824397D4: 3D4051EB  lis r10, 0x51eb
	ctx.r[10].s64 = 1374355456;
	// 824397D8: 817F13B4  lwz r11, 0x13b4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(5044 as u32) ) } as u64;
	// 824397DC: 6149851F  ori r9, r10, 0x851f
	ctx.r[9].u64 = ctx.r[10].u64 | 34079;
	// 824397E0: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824397E4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824397E8: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 824397EC: 7D6B4896  mulhw r11, r11, r9
	ctx.r[11].s64 = ((ctx.r[11].s32 as i64 * ctx.r[9].s32 as i64) >> 32);
	// 824397F0: 7D6B2E70  srawi r11, r11, 5
	ctx.xer.ca = (ctx.r[11].s32 < 0) && ((ctx.r[11].u32 & ((1u32 << 5) - 1)) != 0);
	ctx.r[11].s64 = (ctx.r[11].s32 >> 5) as i64;
	// 824397F4: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 824397F8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 824397FC: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82439800: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82439804: 40980008  bge cr6, 0x8243980c
	if !ctx.cr[6].lt {
	pc = 0x8243980C; continue 'dispatch;
	}
	// 82439808: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
            }
            0x8243980C => {
    //   block [0x8243980C..0x82439820)
	// 8243980C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82439810: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439814: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439818: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8243981C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439820(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439820 size=116
    let mut pc: u32 = 0x82439820;
    'dispatch: loop {
        match pc {
            0x82439820 => {
    //   block [0x82439820..0x82439844)
	// 82439820: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82439824: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82439828: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8243982C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82439830: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82439834: 83E30A5C  lwz r31, 0xa5c(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2652 as u32) ) } as u64;
	// 82439838: 83C30A60  lwz r30, 0xa60(r3)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(2656 as u32) ) } as u64;
	// 8243983C: 2F1FFFFC  cmpwi cr6, r31, -4
	ctx.cr[6].compare_i32(ctx.r[31].s32, -4, &mut ctx.xer);
	// 82439840: 409A000C  bne cr6, 0x8243984c
	if !ctx.cr[6].eq {
	pc = 0x8243984C; continue 'dispatch;
	}
	pc = 0x82439844; continue 'dispatch;
            }
            0x82439844 => {
    //   block [0x82439844..0x8243984C)
	// 82439844: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82439848: 48000034  b 0x8243987c
	pc = 0x8243987C; continue 'dispatch;
            }
            0x8243984C => {
    //   block [0x8243984C..0x8243987C)
	// 8243984C: 38A10054  addi r5, r1, 0x54
	ctx.r[5].s64 = ctx.r[1].s64 + 84;
	// 82439850: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82439854: 4800503D  bl 0x8243e890
	ctx.lr = 0x82439858;
	sub_8243E890(ctx, base);
	// 82439858: 80610050  lwz r3, 0x50(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 8243985C: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439860: 4198FFE4  blt cr6, 0x82439844
	if ctx.cr[6].lt {
	pc = 0x82439844; continue 'dispatch;
	}
	// 82439864: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82439868: 80810054  lwz r4, 0x54(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 8243986C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82439870: 48010351  bl 0x82449bc0
	ctx.lr = 0x82439874;
	sub_82449BC0(ctx, base);
	// 82439874: 7C6B0034  cntlzw r11, r3
	ctx.r[11].u64 = if ctx.r[3].u32 == 0 { 32 } else { ctx.r[3].u32.leading_zeros() as u64 };
	// 82439878: 5563DFFE  rlwinm r3, r11, 0x1b, 0x1f, 0x1f
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000001Fu64;
	pc = 0x8243987C; continue 'dispatch;
            }
            0x8243987C => {
    //   block [0x8243987C..0x82439894)
	// 8243987C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82439880: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82439884: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82439888: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8243988C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82439890: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82439898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82439898 size=228
    let mut pc: u32 = 0x82439898;
    'dispatch: loop {
        match pc {
            0x82439898 => {
    //   block [0x82439898..0x824398CC)
	// 82439898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8243989C: 480FB819  bl 0x825350b4
	ctx.lr = 0x824398A0;
	sub_82535080(ctx, base);
	// 824398A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 824398A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 824398A8: 817E0A24  lwz r11, 0xa24(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2596 as u32) ) } as u64;
	// 824398AC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824398B0: 409A001C  bne cr6, 0x824398cc
	if !ctx.cr[6].eq {
	pc = 0x824398CC; continue 'dispatch;
	}
	// 824398B4: 817E0A20  lwz r11, 0xa20(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(2592 as u32) ) } as u64;
	// 824398B8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 824398BC: 409A0010  bne cr6, 0x824398cc
	if !ctx.cr[6].eq {
	pc = 0x824398CC; continue 'dispatch;
	}
	// 824398C0: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 824398C4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 824398C8: 480FB83C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            0x824398CC => {
    //   block [0x824398CC..0x8243993C)
	// 824398CC: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 824398D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824398D4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 824398D8: 4800F691  bl 0x82448f68
	ctx.lr = 0x824398DC;
	sub_82448F68(ctx, base);
	// 824398DC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 824398E0: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 824398E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824398E8: 4800F681  bl 0x82448f68
	ctx.lr = 0x824398EC;
	sub_82448F68(ctx, base);
	// 824398EC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 824398F0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 824398F4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 824398F8: 4800F671  bl 0x82448f68
	ctx.lr = 0x824398FC;
	sub_82448F68(ctx, base);
	// 824398FC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82439900: 38800019  li r4, 0x19
	ctx.r[4].s64 = 25;
	// 82439904: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82439908: 48002E01  bl 0x8243c708
	ctx.lr = 0x8243990C;
	sub_8243C708(ctx, base);
	// 8243990C: 2B030003  cmplwi cr6, r3, 3
	ctx.cr[6].compare_u32(ctx.r[3].u32, 3 as u32, &mut ctx.xer);
	// 82439910: 41990048  bgt cr6, 0x82439958
	if ctx.cr[6].gt {
	pc = 0x82439958; continue 'dispatch;
	}
	// 82439914: 3D808244  lis r12, -0x7dbc
	ctx.r[12].s64 = -2109472768;
	// 82439918: 398C992C  addi r12, r12, -0x66d4
	ctx.r[12].s64 = ctx.r[12].s64 + -26324;
	// 8243991C: 5460103A  slwi r0, r3, 2
	ctx.r[0].u32 = ctx.r[3].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82439920: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 82439924: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82439928: 4E800420  bctr
	match ctx.r[3].u64 {
		0 => {
	pc = 0x82439954; continue 'dispatch;
		},
		1 => {
	pc = 0x8243993C; continue 'dispatch;
		},
		2 => {
	pc = 0x82439944; continue 'dispatch;
		},
		3 => {
	pc = 0x8243994C; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 8243992C: 82439954  lwz r18, -0x66ac(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-26284 as u32) ) } as u64;
	// 82439930: 8243993C  lwz r18, -0x66c4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-26308 as u32) ) } as u64;
	// 82439934: 82439944  lwz r18, -0x66bc(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-26300 as u32) ) } as u64;
	// 82439938: 8243994C  lwz r18, -0x66b4(r3)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(-26292 as u32) ) } as u64;
            }
            0x8243993C => {
    //   block [0x8243993C..0x82439944)
	// 8243993C: 7F9FE378  mr r31, r28
	ctx.r[31].u64 = ctx.r[28].u64;
	// 82439940: 48000018  b 0x82439958
	pc = 0x82439958; continue 'dispatch;
            }
            0x82439944 => {
    //   block [0x82439944..0x8243994C)
	// 82439944: 7FBFEB78  mr r31, r29
	ctx.r[31].u64 = ctx.r[29].u64;
	// 82439948: 48000010  b 0x82439958
	pc = 0x82439958; continue 'dispatch;
            }
            0x8243994C => {
    //   block [0x8243994C..0x82439954)
	// 8243994C: 7F9FEB78  or r31, r28, r29
	ctx.r[31].u64 = ctx.r[28].u64 | ctx.r[29].u64;
	// 82439950: 48000008  b 0x82439958
	pc = 0x82439958; continue 'dispatch;
            }
            0x82439954 => {
    //   block [0x82439954..0x82439958)
	// 82439954: 7F9FE838  and r31, r28, r29
	ctx.r[31].u64 = ctx.r[28].u64 & ctx.r[29].u64;
	pc = 0x82439958; continue 'dispatch;
            }
            0x82439958 => {
    //   block [0x82439958..0x82439970)
	// 82439958: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8243995C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82439960: 4800F619  bl 0x82448f78
	ctx.lr = 0x82439964;
	sub_82448F78(ctx, base);
	// 82439964: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82439968: 419A0008  beq cr6, 0x82439970
	if ctx.cr[6].eq {
	pc = 0x82439970; continue 'dispatch;
	}
	// 8243996C: 7F7FF838  and r31, r27, r31
	ctx.r[31].u64 = ctx.r[27].u64 & ctx.r[31].u64;
	pc = 0x82439970; continue 'dispatch;
            }
            0x82439970 => {
    //   block [0x82439970..0x8243997C)
	// 82439970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82439974: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82439978: 480FB78C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


