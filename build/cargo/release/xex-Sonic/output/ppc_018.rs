pub fn sub_823B5880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B5880 size=136
    let mut pc: u32 = 0x823B5880;
    'dispatch: loop {
        match pc {
            0x823B5880 => {
    //   block [0x823B5880..0x823B5908)
	// 823B5880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B5888: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B588C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B5890: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5894: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B5898: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B589C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B58A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B58A4: 388BF828  addi r4, r11, -0x7d8
	ctx.r[4].s64 = ctx.r[11].s64 + -2008;
	// 823B58A8: 48A3E161  bl 0x82df3a08
	ctx.lr = 0x823B58AC;
	sub_82DF3A08(ctx, base);
	// 823B58AC: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823B58B0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823B58B4: 809E0234  lwz r4, 0x234(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(564 as u32) ) } as u64;
	// 823B58B8: 48A5F779  bl 0x82e15030
	ctx.lr = 0x823B58BC;
	sub_82E15030(ctx, base);
	// 823B58BC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B58C0: 395F0018  addi r10, r31, 0x18
	ctx.r[10].s64 = ctx.r[31].s64 + 24;
	// 823B58C4: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 823B58C8: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 823B58CC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B58D0: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 823B58D4: 4BF0EB8D  bl 0x822c4460
	ctx.lr = 0x823B58D8;
	sub_822C4460(ctx, base);
	// 823B58D8: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823B58DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823B58E0: 419A0008  beq cr6, 0x823b58e8
	if ctx.cr[6].eq {
	pc = 0x823B58E8; continue 'dispatch;
	}
	// 823B58E4: 4BF0AFAD  bl 0x822c0890
	ctx.lr = 0x823B58E8;
	sub_822C0890(ctx, base);
	// 823B58E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B58EC: 48A3DB3D  bl 0x82df3428
	ctx.lr = 0x823B58F0;
	sub_82DF3428(ctx, base);
	// 823B58F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823B58F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B58F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B58FC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B5900: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B5904: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B5908 size=136
    let mut pc: u32 = 0x823B5908;
    'dispatch: loop {
        match pc {
            0x823B5908 => {
    //   block [0x823B5908..0x823B5990)
	// 823B5908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B590C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B5910: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B5914: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B5918: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B591C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823B5920: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B5924: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B5928: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B592C: 388BC604  addi r4, r11, -0x39fc
	ctx.r[4].s64 = ctx.r[11].s64 + -14844;
	// 823B5930: 48A3E0D9  bl 0x82df3a08
	ctx.lr = 0x823B5934;
	sub_82DF3A08(ctx, base);
	// 823B5934: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823B5938: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823B593C: 809E0234  lwz r4, 0x234(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(564 as u32) ) } as u64;
	// 823B5940: 48A5F6F1  bl 0x82e15030
	ctx.lr = 0x823B5944;
	sub_82E15030(ctx, base);
	// 823B5944: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B5948: 395F0018  addi r10, r31, 0x18
	ctx.r[10].s64 = ctx.r[31].s64 + 24;
	// 823B594C: 388B0004  addi r4, r11, 4
	ctx.r[4].s64 = ctx.r[11].s64 + 4;
	// 823B5950: 386A0004  addi r3, r10, 4
	ctx.r[3].s64 = ctx.r[10].s64 + 4;
	// 823B5954: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5958: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 823B595C: 4BF0EB05  bl 0x822c4460
	ctx.lr = 0x823B5960;
	sub_822C4460(ctx, base);
	// 823B5960: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823B5964: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823B5968: 419A0008  beq cr6, 0x823b5970
	if ctx.cr[6].eq {
	pc = 0x823B5970; continue 'dispatch;
	}
	// 823B596C: 4BF0AF25  bl 0x822c0890
	ctx.lr = 0x823B5970;
	sub_822C0890(ctx, base);
	// 823B5970: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5974: 48A3DAB5  bl 0x82df3428
	ctx.lr = 0x823B5978;
	sub_82DF3428(ctx, base);
	// 823B5978: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823B597C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B5980: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B5984: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B5988: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B598C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B5990 size=232
    let mut pc: u32 = 0x823B5990;
    'dispatch: loop {
        match pc {
            0x823B5990 => {
    //   block [0x823B5990..0x823B5A78)
	// 823B5990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5994: 48DF27D9  bl 0x831a816c
	ctx.lr = 0x823B5998;
	sub_831A8130(ctx, base);
	// 823B5998: DBE1FFD8  stfd f31, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 823B599C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B59A0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B59A4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 823B59A8: 481D1509  bl 0x82586eb0
	ctx.lr = 0x823B59AC;
	sub_82586EB0(ctx, base);
	// 823B59AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823B59B0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823B59B4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B59B8: C01F0800  lfs f0, 0x800(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2048 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B59BC: D01D0018  stfs f0, 0x18(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 823B59C0: 817F0820  lwz r11, 0x820(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2080 as u32) ) } as u64;
	// 823B59C4: 917D001C  stw r11, 0x1c(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 823B59C8: 48159B01  bl 0x8250f4c8
	ctx.lr = 0x823B59CC;
	sub_8250F4C8(ctx, base);
	// 823B59CC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B59D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B59D4: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 823B59D8: 409A0008  bne cr6, 0x823b59e0
	if !ctx.cr[6].eq {
	pc = 0x823B59E0; continue 'dispatch;
	}
	// 823B59DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823B59E0: C3FF066C  lfs f31, 0x66c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1644 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 823B59E4: 48152B35  bl 0x82508518
	ctx.lr = 0x823B59E8;
	sub_82508518(ctx, base);
	// 823B59E8: EDBF0828  fsubs f13, f31, f1
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[13].f64 = (((ctx.f[31].f64 - ctx.f[1].f64) as f32) as f64);
	// 823B59EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823B59F0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 823B59F4: C00B964C  lfs f0, -0x69b4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27060 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B59F8: FDA06A10  fabs f13, f13
	ctx.f[13].u64 = ctx.f[13].u64 & !0x8000_0000_0000_0000u64;
	// 823B59FC: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 823B5A00: 41980008  blt cr6, 0x823b5a08
	if ctx.cr[6].lt {
	pc = 0x823B5A08; continue 'dispatch;
	}
	// 823B5A04: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 823B5A08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5A0C: 48A3C285  bl 0x82df1c90
	ctx.lr = 0x823B5A10;
	sub_82DF1C90(ctx, base);
	// 823B5A10: 57CB063F  clrlwi. r11, r30, 0x18
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B5A14: 41820058  beq 0x823b5a6c
	if ctx.cr[0].eq {
	pc = 0x823B5A6C; continue 'dispatch;
	}
	// 823B5A18: 817F0398  lwz r11, 0x398(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(920 as u32) ) } as u64;
	// 823B5A1C: 894B005C  lbz r10, 0x5c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) } as u64;
	// 823B5A20: 280A0000  cmplwi r10, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B5A24: 40820048  bne 0x823b5a6c
	if !ctx.cr[0].eq {
	pc = 0x823B5A6C; continue 'dispatch;
	}
	// 823B5A28: 815F0820  lwz r10, 0x820(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2080 as u32) ) } as u64;
	// 823B5A2C: 2F0A0001  cmpwi cr6, r10, 1
	ctx.cr[6].compare_i32(ctx.r[10].s32, 1, &mut ctx.xer);
	// 823B5A30: 409A003C  bne cr6, 0x823b5a6c
	if !ctx.cr[6].eq {
	pc = 0x823B5A6C; continue 'dispatch;
	}
	// 823B5A34: 816B0054  lwz r11, 0x54(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(84 as u32) ) } as u64;
	// 823B5A38: 2B0B0004  cmplwi cr6, r11, 4
	ctx.cr[6].compare_u32(ctx.r[11].u32, 4 as u32, &mut ctx.xer);
	// 823B5A3C: 419A0024  beq cr6, 0x823b5a60
	if ctx.cr[6].eq {
	pc = 0x823B5A60; continue 'dispatch;
	}
	// 823B5A40: 2B0B0001  cmplwi cr6, r11, 1
	ctx.cr[6].compare_u32(ctx.r[11].u32, 1 as u32, &mut ctx.xer);
	// 823B5A44: 419A001C  beq cr6, 0x823b5a60
	if ctx.cr[6].eq {
	pc = 0x823B5A60; continue 'dispatch;
	}
	// 823B5A48: 2B0B0002  cmplwi cr6, r11, 2
	ctx.cr[6].compare_u32(ctx.r[11].u32, 2 as u32, &mut ctx.xer);
	// 823B5A4C: 419A0014  beq cr6, 0x823b5a60
	if ctx.cr[6].eq {
	pc = 0x823B5A60; continue 'dispatch;
	}
	// 823B5A50: 2B0B0005  cmplwi cr6, r11, 5
	ctx.cr[6].compare_u32(ctx.r[11].u32, 5 as u32, &mut ctx.xer);
	// 823B5A54: 419A000C  beq cr6, 0x823b5a60
	if ctx.cr[6].eq {
	pc = 0x823B5A60; continue 'dispatch;
	}
	// 823B5A58: 2B0B0003  cmplwi cr6, r11, 3
	ctx.cr[6].compare_u32(ctx.r[11].u32, 3 as u32, &mut ctx.xer);
	// 823B5A5C: 409A0010  bne cr6, 0x823b5a6c
	if !ctx.cr[6].eq {
	pc = 0x823B5A6C; continue 'dispatch;
	}
	// 823B5A60: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823B5A64: C00B0790  lfs f0, 0x790(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(1936 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B5A68: D01D0018  stfs f0, 0x18(r29)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 823B5A6C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823B5A70: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 823B5A74: 48DF2748  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5A78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B5A78 size=224
    let mut pc: u32 = 0x823B5A78;
    'dispatch: loop {
        match pc {
            0x823B5A78 => {
    //   block [0x823B5A78..0x823B5B58)
	// 823B5A78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5A7C: 48DF26F1  bl 0x831a816c
	ctx.lr = 0x823B5A80;
	sub_831A8130(ctx, base);
	// 823B5A80: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5A84: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823B5A88: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823B5A8C: 481D1425  bl 0x82586eb0
	ctx.lr = 0x823B5A90;
	sub_82586EB0(ctx, base);
	// 823B5A90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823B5A94: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823B5A98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5A9C: 48159A2D  bl 0x8250f4c8
	ctx.lr = 0x823B5AA0;
	sub_8250F4C8(ctx, base);
	// 823B5AA0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5AA4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5AA8: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 823B5AAC: 409A0008  bne cr6, 0x823b5ab4
	if !ctx.cr[6].eq {
	pc = 0x823B5AB4; continue 'dispatch;
	}
	// 823B5AB0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823B5AB4: 48152A65  bl 0x82508518
	ctx.lr = 0x823B5AB8;
	sub_82508518(ctx, base);
	// 823B5AB8: D03F066C  stfs f1, 0x66c(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1644 as u32), tmp.u32 ) };
	// 823B5ABC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5AC0: 48A3C1D1  bl 0x82df1c90
	ctx.lr = 0x823B5AC4;
	sub_82DF1C90(ctx, base);
	// 823B5AC4: 817E0018  lwz r11, 0x18(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) } as u64;
	// 823B5AC8: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B5ACC: 409A000C  bne cr6, 0x823b5ad8
	if !ctx.cr[6].eq {
	pc = 0x823B5AD8; continue 'dispatch;
	}
	// 823B5AD0: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 823B5AD4: 48000070  b 0x823b5b44
	pc = 0x823B5B44; continue 'dispatch;
	// 823B5AD8: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 823B5ADC: 409A000C  bne cr6, 0x823b5ae8
	if !ctx.cr[6].eq {
	pc = 0x823B5AE8; continue 'dispatch;
	}
	// 823B5AE0: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 823B5AE4: 48000060  b 0x823b5b44
	pc = 0x823B5B44; continue 'dispatch;
	// 823B5AE8: 2F0B0003  cmpwi cr6, r11, 3
	ctx.cr[6].compare_i32(ctx.r[11].s32, 3, &mut ctx.xer);
	// 823B5AEC: 409A000C  bne cr6, 0x823b5af8
	if !ctx.cr[6].eq {
	pc = 0x823B5AF8; continue 'dispatch;
	}
	// 823B5AF0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823B5AF4: 48000050  b 0x823b5b44
	pc = 0x823B5B44; continue 'dispatch;
	// 823B5AF8: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 823B5AFC: 409A000C  bne cr6, 0x823b5b08
	if !ctx.cr[6].eq {
	pc = 0x823B5B08; continue 'dispatch;
	}
	// 823B5B00: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B5B04: 48000040  b 0x823b5b44
	pc = 0x823B5B44; continue 'dispatch;
	// 823B5B08: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 823B5B0C: 409A000C  bne cr6, 0x823b5b18
	if !ctx.cr[6].eq {
	pc = 0x823B5B18; continue 'dispatch;
	}
	// 823B5B10: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 823B5B14: 48000030  b 0x823b5b44
	pc = 0x823B5B44; continue 'dispatch;
	// 823B5B18: 2F0B0005  cmpwi cr6, r11, 5
	ctx.cr[6].compare_i32(ctx.r[11].s32, 5, &mut ctx.xer);
	// 823B5B1C: 409A000C  bne cr6, 0x823b5b28
	if !ctx.cr[6].eq {
	pc = 0x823B5B28; continue 'dispatch;
	}
	// 823B5B20: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 823B5B24: 48000020  b 0x823b5b44
	pc = 0x823B5B44; continue 'dispatch;
	// 823B5B28: 2F0B0006  cmpwi cr6, r11, 6
	ctx.cr[6].compare_i32(ctx.r[11].s32, 6, &mut ctx.xer);
	// 823B5B2C: 409A000C  bne cr6, 0x823b5b38
	if !ctx.cr[6].eq {
	pc = 0x823B5B38; continue 'dispatch;
	}
	// 823B5B30: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 823B5B34: 48000010  b 0x823b5b44
	pc = 0x823B5B44; continue 'dispatch;
	// 823B5B38: 2F0B0007  cmpwi cr6, r11, 7
	ctx.cr[6].compare_i32(ctx.r[11].s32, 7, &mut ctx.xer);
	// 823B5B3C: 409A0014  bne cr6, 0x823b5b50
	if !ctx.cr[6].eq {
	pc = 0x823B5B50; continue 'dispatch;
	}
	// 823B5B40: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 823B5B44: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823B5B48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B5B4C: 4BFEC6FD  bl 0x823a2248
	ctx.lr = 0x823B5B50;
	sub_823A2248(ctx, base);
	// 823B5B50: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823B5B54: 48DF2668  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B5B58 size=164
    let mut pc: u32 = 0x823B5B58;
    'dispatch: loop {
        match pc {
            0x823B5B58 => {
    //   block [0x823B5B58..0x823B5BFC)
	// 823B5B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5B5C: 48DF260D  bl 0x831a8168
	ctx.lr = 0x823B5B60;
	sub_831A8130(ctx, base);
	// 823B5B60: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5B64: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823B5B68: 481D1349  bl 0x82586eb0
	ctx.lr = 0x823B5B6C;
	sub_82586EB0(ctx, base);
	// 823B5B6C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823B5B70: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 823B5B74: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 823B5B78: 817F07C4  lwz r11, 0x7c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1988 as u32) ) } as u64;
	// 823B5B7C: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 823B5B80: 419A0060  beq cr6, 0x823b5be0
	if ctx.cr[6].eq {
	pc = 0x823B5BE0; continue 'dispatch;
	}
	// 823B5B84: 817E0040  lwz r11, 0x40(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(64 as u32) ) } as u64;
	// 823B5B88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B5B8C: 419A000C  beq cr6, 0x823b5b98
	if ctx.cr[6].eq {
	pc = 0x823B5B98; continue 'dispatch;
	}
	// 823B5B90: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 823B5B94: 409A004C  bne cr6, 0x823b5be0
	if !ctx.cr[6].eq {
	pc = 0x823B5BE0; continue 'dispatch;
	}
	// 823B5B98: 9B9F08C9  stb r28, 0x8c9(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2249 as u32), ctx.r[28].u8 ) };
	// 823B5B9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823B5BA0: 9BBF08CA  stb r29, 0x8ca(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2250 as u32), ctx.r[29].u8 ) };
	// 823B5BA4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5BA8: 4BF4FEC9  bl 0x82305a70
	ctx.lr = 0x823B5BAC;
	sub_82305A70(ctx, base);
	// 823B5BAC: 396008D0  li r11, 0x8d0
	ctx.r[11].s64 = 2256;
	// 823B5BB0: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 823B5BB4: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823B5BB8: 39200BB0  li r9, 0xbb0
	ctx.r[9].s64 = 2992;
	// 823B5BBC: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 823B5BC0: 38E00BC0  li r7, 0xbc0
	ctx.r[7].s64 = 3008;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5C00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B5C00 size=44
    let mut pc: u32 = 0x823B5C00;
    'dispatch: loop {
        match pc {
            0x823B5C00 => {
    //   block [0x823B5C00..0x823B5C2C)
	// 823B5C00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5C04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B5C08: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5C0C: 481D12A5  bl 0x82586eb0
	ctx.lr = 0x823B5C10;
	sub_82586EB0(ctx, base);
	// 823B5C10: 81630398  lwz r11, 0x398(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(920 as u32) ) } as u64;
	// 823B5C14: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823B5C18: 994B005C  stb r10, 0x5c(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(92 as u32), ctx.r[10].u8 ) };
	// 823B5C1C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823B5C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B5C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B5C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B5C30 size=128
    let mut pc: u32 = 0x823B5C30;
    'dispatch: loop {
        match pc {
            0x823B5C30 => {
    //   block [0x823B5C30..0x823B5CB0)
	// 823B5C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B5C38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B5C3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B5C40: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5C44: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B5C48: 481D1269  bl 0x82586eb0
	ctx.lr = 0x823B5C4C;
	sub_82586EB0(ctx, base);
	// 823B5C4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B5C50: 389F0020  addi r4, r31, 0x20
	ctx.r[4].s64 = ctx.r[31].s64 + 32;
	// 823B5C54: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 823B5C58: 48AC6241  bl 0x82e7be98
	ctx.lr = 0x823B5C5C;
	sub_82E7BE98(ctx, base);
	// 823B5C5C: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823B5C60: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B5C64: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823B5C68: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 823B5C6C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823B5C70: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B5C74: C1AA08A8  lfs f13, 0x8a8(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2216 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823B5C78: D0010050  stfs f0, 0x50(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), tmp.u32 ) };
	// 823B5C7C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 823B5C80: D1A10058  stfs f13, 0x58(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), tmp.u32 ) };
	// 823B5C84: D001005C  stfs f0, 0x5c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 823B5C88: 48AC60C1  bl 0x82e7bd48
	ctx.lr = 0x823B5C8C;
	sub_82E7BD48(ctx, base);
	// 823B5C8C: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823B5C90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B5C94: 4BFEF9FD  bl 0x823a5690
	ctx.lr = 0x823B5C98;
	sub_823A5690(ctx, base);
	// 823B5C98: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 823B5C9C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B5CA0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B5CA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B5CA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B5CAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5CB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B5CB0 size=68
    let mut pc: u32 = 0x823B5CB0;
    'dispatch: loop {
        match pc {
            0x823B5CB0 => {
    //   block [0x823B5CB0..0x823B5CF4)
	// 823B5CB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5CB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B5CB8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B5CBC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5CC0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B5CC4: 481D11ED  bl 0x82586eb0
	ctx.lr = 0x823B5CC8;
	sub_82586EB0(ctx, base);
	// 823B5CC8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823B5CCC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5CD0: 4BFEFD79  bl 0x823a5a48
	ctx.lr = 0x823B5CD4;
	sub_823A5A48(ctx, base);
	// 823B5CD4: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 823B5CD8: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B5CF8 size=160
    let mut pc: u32 = 0x823B5CF8;
    'dispatch: loop {
        match pc {
            0x823B5CF8 => {
    //   block [0x823B5CF8..0x823B5D98)
	// 823B5CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B5D00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B5D04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B5D08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5D0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B5D10: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B5D14: 481D119D  bl 0x82586eb0
	ctx.lr = 0x823B5D18;
	sub_82586EB0(ctx, base);
	// 823B5D18: 817F0018  lwz r11, 0x18(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 823B5D1C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B5D20: 419A0024  beq cr6, 0x823b5d44
	if ctx.cr[6].eq {
	pc = 0x823B5D44; continue 'dispatch;
	}
	// 823B5D24: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 823B5D28: 419A001C  beq cr6, 0x823b5d44
	if ctx.cr[6].eq {
	pc = 0x823B5D44; continue 'dispatch;
	}
	// 823B5D2C: 2F0B0002  cmpwi cr6, r11, 2
	ctx.cr[6].compare_i32(ctx.r[11].s32, 2, &mut ctx.xer);
	// 823B5D30: 409A0014  bne cr6, 0x823b5d44
	if !ctx.cr[6].eq {
	pc = 0x823B5D44; continue 'dispatch;
	}
	// 823B5D34: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 823B5D38: 81430678  lwz r10, 0x678(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1656 as u32) ) } as u64;
	// 823B5D3C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 823B5D40: 91630678  stw r11, 0x678(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(1656 as u32), ctx.r[11].u32 ) };
	// 823B5D44: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823B5D48: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5D4C: 4815977D  bl 0x8250f4c8
	ctx.lr = 0x823B5D50;
	sub_8250F4C8(ctx, base);
	// 823B5D50: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B5D54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B5D58: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 823B5D5C: 409A0008  bne cr6, 0x823b5d64
	if !ctx.cr[6].eq {
	pc = 0x823B5D64; continue 'dispatch;
	}
	// 823B5D60: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823B5D64: 83FF001C  lwz r31, 0x1c(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 823B5D68: 48152971  bl 0x825086d8
	ctx.lr = 0x823B5D6C;
	sub_825086D8(ctx, base);
	// 823B5D6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823B5D70: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 823B5D74: 480A640D  bl 0x8245c180
	ctx.lr = 0x823B5D78;
	sub_8245C180(ctx, base);
	// 823B5D78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5D7C: 48A3BF15  bl 0x82df1c90
	ctx.lr = 0x823B5D80;
	sub_82DF1C90(ctx, base);
	// 823B5D80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B5D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B5D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B5D8C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B5D90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B5D94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5D98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B5D98 size=204
    let mut pc: u32 = 0x823B5D98;
    'dispatch: loop {
        match pc {
            0x823B5D98 => {
    //   block [0x823B5D98..0x823B5E64)
	// 823B5D98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5D9C: 48DF23CD  bl 0x831a8168
	ctx.lr = 0x823B5DA0;
	sub_831A8130(ctx, base);
	// 823B5DA0: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5DA4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823B5DA8: 481D1109  bl 0x82586eb0
	ctx.lr = 0x823B5DAC;
	sub_82586EB0(ctx, base);
	// 823B5DAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823B5DB0: 817F07C4  lwz r11, 0x7c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1988 as u32) ) } as u64;
	// 823B5DB4: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 823B5DB8: 419A00A4  beq cr6, 0x823b5e5c
	if ctx.cr[6].eq {
	pc = 0x823B5E5C; continue 'dispatch;
	}
	// 823B5DBC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 823B5DC0: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 823B5DC4: 9BBF08C9  stb r29, 0x8c9(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2249 as u32), ctx.r[29].u8 ) };
	// 823B5DC8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823B5DCC: 9B9F08CA  stb r28, 0x8ca(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2250 as u32), ctx.r[28].u8 ) };
	// 823B5DD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5DD4: 4BF4FC9D  bl 0x82305a70
	ctx.lr = 0x823B5DD8;
	sub_82305A70(ctx, base);
	// 823B5DD8: 392008D0  li r9, 0x8d0
	ctx.r[9].s64 = 2256;
	// 823B5DDC: 397E0020  addi r11, r30, 0x20
	ctx.r[11].s64 = ctx.r[30].s64 + 32;
	// 823B5DE0: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823B5DE4: 39000BB0  li r8, 0xbb0
	ctx.r[8].s64 = 2992;
	// 823B5DE8: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
	// 823B5DEC: 395F0BC0  addi r10, r31, 0xbc0
	ctx.r[10].s64 = ctx.r[31].s64 + 3008;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B5E68 size=164
    let mut pc: u32 = 0x823B5E68;
    'dispatch: loop {
        match pc {
            0x823B5E68 => {
    //   block [0x823B5E68..0x823B5F0C)
	// 823B5E68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5E6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B5E70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B5E74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B5E78: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5E7C: 481D1035  bl 0x82586eb0
	ctx.lr = 0x823B5E80;
	sub_82586EB0(ctx, base);
	// 823B5E80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823B5E84: 817F08CC  lwz r11, 0x8cc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2252 as u32) ) } as u64;
	// 823B5E88: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823B5E8C: 409A005C  bne cr6, 0x823b5ee8
	if !ctx.cr[6].eq {
	pc = 0x823B5EE8; continue 'dispatch;
	}
	// 823B5E90: 817F07C4  lwz r11, 0x7c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1988 as u32) ) } as u64;
	// 823B5E94: 2F0B0020  cmpwi cr6, r11, 0x20
	ctx.cr[6].compare_i32(ctx.r[11].s32, 32, &mut ctx.xer);
	// 823B5E98: 419A0050  beq cr6, 0x823b5ee8
	if ctx.cr[6].eq {
	pc = 0x823B5EE8; continue 'dispatch;
	}
	// 823B5E9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B5EA0: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 823B5EA4: 997F08C9  stb r11, 0x8c9(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2249 as u32), ctx.r[11].u8 ) };
	// 823B5EA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823B5EAC: 9BDF08CA  stb r30, 0x8ca(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2250 as u32), ctx.r[30].u8 ) };
	// 823B5EB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5EB4: 4BF4FBBD  bl 0x82305a70
	ctx.lr = 0x823B5EB8;
	sub_82305A70(ctx, base);
	// 823B5EB8: 3D608338  lis r11, -0x7cc8
	ctx.r[11].s64 = -2093481984;
	// 823B5EBC: 394008D0  li r10, 0x8d0
	ctx.r[10].s64 = 2256;
	// 823B5EC0: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
	// 823B5EC4: 396B6910  addi r11, r11, 0x6910
	ctx.r[11].s64 = ctx.r[11].s64 + 26896;
	// 823B5EC8: 39200BB0  li r9, 0xbb0
	ctx.r[9].s64 = 2992;
	// 823B5ECC: 39000BC0  li r8, 0xbc0
	ctx.r[8].s64 = 3008;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B5F10 size=140
    let mut pc: u32 = 0x823B5F10;
    'dispatch: loop {
        match pc {
            0x823B5F10 => {
    //   block [0x823B5F10..0x823B5F9C)
	// 823B5F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B5F18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B5F1C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5F20: 481D0F91  bl 0x82586eb0
	ctx.lr = 0x823B5F24;
	sub_82586EB0(ctx, base);
	// 823B5F24: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823B5F28: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823B5F2C: C1BF0860  lfs f13, 0x860(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2144 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823B5F30: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B5F34: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 823B5F38: 40990050  ble cr6, 0x823b5f88
	if !ctx.cr[6].gt {
	pc = 0x823B5F88; continue 'dispatch;
	}
	// 823B5F3C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823B5F40: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823B5F44: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 823B5F48: 995F085C  stb r10, 0x85c(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2140 as u32), ctx.r[10].u8 ) };
	// 823B5F4C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5F50: 3889D064  addi r4, r9, -0x2f9c
	ctx.r[4].s64 = ctx.r[9].s64 + -12188;
	// 823B5F54: C00B9534  lfs f0, -0x6acc(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823B5F58: D01F0860  stfs f0, 0x860(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2144 as u32), tmp.u32 ) };
	// 823B5F5C: 48A3DAAD  bl 0x82df3a08
	ctx.lr = 0x823B5F60;
	sub_82DF3A08(ctx, base);
	// 823B5F60: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 823B5F64: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823B5F68: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823B5F6C: 4BF52DCD  bl 0x82308d38
	ctx.lr = 0x823B5F70;
	sub_82308D38(ctx, base);
	// 823B5F70: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823B5F74: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823B5F78: 419A0008  beq cr6, 0x823b5f80
	if ctx.cr[6].eq {
	pc = 0x823B5F80; continue 'dispatch;
	}
	// 823B5F7C: 4BF0A915  bl 0x822c0890
	ctx.lr = 0x823B5F80;
	sub_822C0890(ctx, base);
	// 823B5F80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B5F84: 48A3D4A5  bl 0x82df3428
	ctx.lr = 0x823B5F88;
	sub_82DF3428(ctx, base);
	// 823B5F88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B5F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B5F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B5F94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B5F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5FA0 size=12
    let mut pc: u32 = 0x823B5FA0;
    'dispatch: loop {
        match pc {
            0x823B5FA0 => {
    //   block [0x823B5FA0..0x823B5FAC)
	// 823B5FA0: 806303E8  lwz r3, 0x3e8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1000 as u32) ) } as u64;
	// 823B5FA4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823B5FA8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5FAC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5FAC size=20
    let mut pc: u32 = 0x823B5FAC;
    'dispatch: loop {
        match pc {
            0x823B5FAC => {
    //   block [0x823B5FAC..0x823B5FC0)
	// 823B5FAC: C0840024  lfs f4, 0x24(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(36 as u32) ) };
	ctx.f[4].f64 = (tmp.f32 as f64);
	// 823B5FB0: C0640020  lfs f3, 0x20(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(32 as u32) ) };
	ctx.f[3].f64 = (tmp.f32 as f64);
	// 823B5FB4: C044001C  lfs f2, 0x1c(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[2].f64 = (tmp.f32 as f64);
	// 823B5FB8: C0240018  lfs f1, 0x18(r4)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823B5FBC: 480CEEE4  b 0x82484ea0
	sub_82484EA0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5FC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5FC0 size=4
    let mut pc: u32 = 0x823B5FC0;
    'dispatch: loop {
        match pc {
            0x823B5FC0 => {
    //   block [0x823B5FC0..0x823B5FC4)
	// 823B5FC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5FC8 size=12
    let mut pc: u32 = 0x823B5FC8;
    'dispatch: loop {
        match pc {
            0x823B5FC8 => {
    //   block [0x823B5FC8..0x823B5FD4)
	// 823B5FC8: 806303E8  lwz r3, 0x3e8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(1000 as u32) ) } as u64;
	// 823B5FCC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823B5FD0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5FD4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x823B5FD4 size=8
    let mut pc: u32 = 0x823B5FD4;
    'dispatch: loop {
        match pc {
            0x823B5FD4 => {
    //   block [0x823B5FD4..0x823B5FDC)
	// 823B5FD4: C0240018  lfs f1, 0x18(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(24 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823B5FD8: 480CEE58  b 0x82484e30
	sub_82484E30(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5FDC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823B5FDC size=4
    let mut pc: u32 = 0x823B5FDC;
    'dispatch: loop {
        match pc {
            0x823B5FDC => {
    //   block [0x823B5FDC..0x823B5FE0)
	// 823B5FDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B5FE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823B5FE0 size=68
    let mut pc: u32 = 0x823B5FE0;
    'dispatch: loop {
        match pc {
            0x823B5FE0 => {
    //   block [0x823B5FE0..0x823B6024)
	// 823B5FE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B5FE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B5FE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B5FEC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B5FF0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B5FF4: 481D0EBD  bl 0x82586eb0
	ctx.lr = 0x823B5FF8;
	sub_82586EB0(ctx, base);
	// 823B5FF8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823B5FFC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B6000: 4BF51CC1  bl 0x82307cc0
	ctx.lr = 0x823B6004;
	sub_82307CC0(ctx, base);
	// 823B6004: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 823B6008: 13E018C7  vcmpequd (lvx128) v31, v0, v3
	tmp.u32 = ctx.r[3].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6028 size=196
    let mut pc: u32 = 0x823B6028;
    'dispatch: loop {
        match pc {
            0x823B6028 => {
    //   block [0x823B6028..0x823B60EC)
	// 823B6028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B602C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6030: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B6034: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6038: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B603C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6040: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6044: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6048: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B604C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6050: 4BF0A8E9  bl 0x822c0938
	ctx.lr = 0x823B6054;
	sub_822C0938(ctx, base);
	// 823B6054: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B6058: 41820028  beq 0x823b6080
	if ctx.cr[0].eq {
	pc = 0x823B6080; continue 'dispatch;
	}
	// 823B605C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B6060: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B6064: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6068: 392BF14C  addi r9, r11, -0xeb4
	ctx.r[9].s64 = ctx.r[11].s64 + -3764;
	// 823B606C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6070: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B6074: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B6078: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B607C: 48000008  b 0x823b6084
	pc = 0x823B6084; continue 'dispatch;
	// 823B6080: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6084: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6088: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B608C: 409A0044  bne cr6, 0x823b60d0
	if !ctx.cr[6].eq {
	pc = 0x823B60D0; continue 'dispatch;
	}
	// 823B6090: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B6094: 419A001C  beq cr6, 0x823b60b0
	if ctx.cr[6].eq {
	pc = 0x823B60B0; continue 'dispatch;
	}
	// 823B6098: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B609C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B60A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B60A4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B60A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B60AC: 4E800421  bctrl
	ctx.lr = 0x823B60B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B60B0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B60B4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B60B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B60BC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B60C0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B60C4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B60C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B60CC: 4BF09F35  bl 0x822c0000
	ctx.lr = 0x823B60D0;
	sub_822C0000(ctx, base);
	// 823B60D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B60D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B60D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B60DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B60E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B60E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B60E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B60F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B60F0 size=196
    let mut pc: u32 = 0x823B60F0;
    'dispatch: loop {
        match pc {
            0x823B60F0 => {
    //   block [0x823B60F0..0x823B61B4)
	// 823B60F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B60F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B60F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B60FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6100: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6104: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6108: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B610C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6110: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B6114: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6118: 4BF0A821  bl 0x822c0938
	ctx.lr = 0x823B611C;
	sub_822C0938(ctx, base);
	// 823B611C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B6120: 41820028  beq 0x823b6148
	if ctx.cr[0].eq {
	pc = 0x823B6148; continue 'dispatch;
	}
	// 823B6124: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B6128: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B612C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6130: 392BF160  addi r9, r11, -0xea0
	ctx.r[9].s64 = ctx.r[11].s64 + -3744;
	// 823B6134: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6138: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B613C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B6140: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B6144: 48000008  b 0x823b614c
	pc = 0x823B614C; continue 'dispatch;
	// 823B6148: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B614C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6150: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6154: 409A0044  bne cr6, 0x823b6198
	if !ctx.cr[6].eq {
	pc = 0x823B6198; continue 'dispatch;
	}
	// 823B6158: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B615C: 419A001C  beq cr6, 0x823b6178
	if ctx.cr[6].eq {
	pc = 0x823B6178; continue 'dispatch;
	}
	// 823B6160: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6164: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B6168: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B616C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6170: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B6174: 4E800421  bctrl
	ctx.lr = 0x823B6178;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B6178: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B617C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B6180: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B6184: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6188: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B618C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6190: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B6194: 4BF09E6D  bl 0x822c0000
	ctx.lr = 0x823B6198;
	sub_822C0000(ctx, base);
	// 823B6198: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B619C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B61A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B61A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B61A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B61AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B61B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B61B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B61B8 size=196
    let mut pc: u32 = 0x823B61B8;
    'dispatch: loop {
        match pc {
            0x823B61B8 => {
    //   block [0x823B61B8..0x823B627C)
	// 823B61B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B61BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B61C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B61C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B61C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B61CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B61D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B61D4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B61D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B61DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B61E0: 4BF0A759  bl 0x822c0938
	ctx.lr = 0x823B61E4;
	sub_822C0938(ctx, base);
	// 823B61E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B61E8: 41820028  beq 0x823b6210
	if ctx.cr[0].eq {
	pc = 0x823B6210; continue 'dispatch;
	}
	// 823B61EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B61F0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B61F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B61F8: 392BF174  addi r9, r11, -0xe8c
	ctx.r[9].s64 = ctx.r[11].s64 + -3724;
	// 823B61FC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6200: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B6204: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B6208: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B620C: 48000008  b 0x823b6214
	pc = 0x823B6214; continue 'dispatch;
	// 823B6210: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6214: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6218: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B621C: 409A0044  bne cr6, 0x823b6260
	if !ctx.cr[6].eq {
	pc = 0x823B6260; continue 'dispatch;
	}
	// 823B6220: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B6224: 419A001C  beq cr6, 0x823b6240
	if ctx.cr[6].eq {
	pc = 0x823B6240; continue 'dispatch;
	}
	// 823B6228: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B622C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B6230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6234: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6238: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B623C: 4E800421  bctrl
	ctx.lr = 0x823B6240;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B6240: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B6244: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B6248: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B624C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6250: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B6254: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6258: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B625C: 4BF09DA5  bl 0x822c0000
	ctx.lr = 0x823B6260;
	sub_822C0000(ctx, base);
	// 823B6260: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B6264: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B626C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6270: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B6274: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6278: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6280(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6280 size=172
    let mut pc: u32 = 0x823B6280;
    'dispatch: loop {
        match pc {
            0x823B6280 => {
    //   block [0x823B6280..0x823B632C)
	// 823B6280: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B6284: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6288: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B628C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6290: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6294: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6298: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B629C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B62A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B62A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B62A8: 4BF0A691  bl 0x822c0938
	ctx.lr = 0x823B62AC;
	sub_822C0938(ctx, base);
	// 823B62AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B62B0: 41820028  beq 0x823b62d8
	if ctx.cr[0].eq {
	pc = 0x823B62D8; continue 'dispatch;
	}
	// 823B62B4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B62B8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B62BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B62C0: 392BF188  addi r9, r11, -0xe78
	ctx.r[9].s64 = ctx.r[11].s64 + -3704;
	// 823B62C4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B62C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B62CC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B62D0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B62D4: 48000008  b 0x823b62dc
	pc = 0x823B62DC; continue 'dispatch;
	// 823B62D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B62DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B62E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B62E4: 409A002C  bne cr6, 0x823b6310
	if !ctx.cr[6].eq {
	pc = 0x823B6310; continue 'dispatch;
	}
	// 823B62E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B62EC: 4BF09F7D  bl 0x822c0268
	ctx.lr = 0x823B62F0;
	sub_822C0268(ctx, base);
	// 823B62F0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B62F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B62F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B62FC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6300: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6304: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B6308: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B630C: 4BF09CF5  bl 0x822c0000
	ctx.lr = 0x823B6310;
	sub_822C0000(ctx, base);
	// 823B6310: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B6314: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6318: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B631C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6320: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B6324: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6328: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6330(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6330 size=196
    let mut pc: u32 = 0x823B6330;
    'dispatch: loop {
        match pc {
            0x823B6330 => {
    //   block [0x823B6330..0x823B63F4)
	// 823B6330: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B6334: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6338: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B633C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6340: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6344: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6348: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B634C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6350: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B6354: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6358: 4BF0A5E1  bl 0x822c0938
	ctx.lr = 0x823B635C;
	sub_822C0938(ctx, base);
	// 823B635C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B6360: 41820028  beq 0x823b6388
	if ctx.cr[0].eq {
	pc = 0x823B6388; continue 'dispatch;
	}
	// 823B6364: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B6368: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B636C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6370: 392BF19C  addi r9, r11, -0xe64
	ctx.r[9].s64 = ctx.r[11].s64 + -3684;
	// 823B6374: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6378: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B637C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B6380: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B6384: 48000008  b 0x823b638c
	pc = 0x823B638C; continue 'dispatch;
	// 823B6388: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B638C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6390: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6394: 409A0044  bne cr6, 0x823b63d8
	if !ctx.cr[6].eq {
	pc = 0x823B63D8; continue 'dispatch;
	}
	// 823B6398: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B639C: 419A001C  beq cr6, 0x823b63b8
	if ctx.cr[6].eq {
	pc = 0x823B63B8; continue 'dispatch;
	}
	// 823B63A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B63A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B63A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B63AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B63B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B63B4: 4E800421  bctrl
	ctx.lr = 0x823B63B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B63B8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B63BC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B63C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B63C4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B63C8: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B63CC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B63D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B63D4: 4BF09C2D  bl 0x822c0000
	ctx.lr = 0x823B63D8;
	sub_822C0000(ctx, base);
	// 823B63D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B63DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B63E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B63E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B63E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B63EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B63F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B63F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B63F8 size=196
    let mut pc: u32 = 0x823B63F8;
    'dispatch: loop {
        match pc {
            0x823B63F8 => {
    //   block [0x823B63F8..0x823B64BC)
	// 823B63F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B63FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6400: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B6404: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6408: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B640C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6410: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6414: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6418: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B641C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6420: 4BF0A519  bl 0x822c0938
	ctx.lr = 0x823B6424;
	sub_822C0938(ctx, base);
	// 823B6424: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B6428: 41820028  beq 0x823b6450
	if ctx.cr[0].eq {
	pc = 0x823B6450; continue 'dispatch;
	}
	// 823B642C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B6430: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B6434: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6438: 392BF1B0  addi r9, r11, -0xe50
	ctx.r[9].s64 = ctx.r[11].s64 + -3664;
	// 823B643C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6440: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B6444: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B6448: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B644C: 48000008  b 0x823b6454
	pc = 0x823B6454; continue 'dispatch;
	// 823B6450: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6454: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6458: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B645C: 409A0044  bne cr6, 0x823b64a0
	if !ctx.cr[6].eq {
	pc = 0x823B64A0; continue 'dispatch;
	}
	// 823B6460: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B6464: 419A001C  beq cr6, 0x823b6480
	if ctx.cr[6].eq {
	pc = 0x823B6480; continue 'dispatch;
	}
	// 823B6468: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B646C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B6470: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6474: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6478: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B647C: 4E800421  bctrl
	ctx.lr = 0x823B6480;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B6480: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B6484: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B6488: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B648C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6490: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B6494: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6498: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B649C: 4BF09B65  bl 0x822c0000
	ctx.lr = 0x823B64A0;
	sub_822C0000(ctx, base);
	// 823B64A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B64A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B64A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B64AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B64B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B64B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B64B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B64C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B64C0 size=196
    let mut pc: u32 = 0x823B64C0;
    'dispatch: loop {
        match pc {
            0x823B64C0 => {
    //   block [0x823B64C0..0x823B6584)
	// 823B64C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B64C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B64C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B64CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B64D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B64D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B64D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B64DC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B64E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B64E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B64E8: 4BF0A451  bl 0x822c0938
	ctx.lr = 0x823B64EC;
	sub_822C0938(ctx, base);
	// 823B64EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B64F0: 41820028  beq 0x823b6518
	if ctx.cr[0].eq {
	pc = 0x823B6518; continue 'dispatch;
	}
	// 823B64F4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B64F8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B64FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6500: 392BF1C4  addi r9, r11, -0xe3c
	ctx.r[9].s64 = ctx.r[11].s64 + -3644;
	// 823B6504: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6508: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B650C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B6510: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B6514: 48000008  b 0x823b651c
	pc = 0x823B651C; continue 'dispatch;
	// 823B6518: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B651C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6520: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6524: 409A0044  bne cr6, 0x823b6568
	if !ctx.cr[6].eq {
	pc = 0x823B6568; continue 'dispatch;
	}
	// 823B6528: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B652C: 419A001C  beq cr6, 0x823b6548
	if ctx.cr[6].eq {
	pc = 0x823B6548; continue 'dispatch;
	}
	// 823B6530: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6534: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B6538: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B653C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6540: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B6544: 4E800421  bctrl
	ctx.lr = 0x823B6548;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B6548: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B654C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B6550: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B6554: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6558: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B655C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6560: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B6564: 4BF09A9D  bl 0x822c0000
	ctx.lr = 0x823B6568;
	sub_822C0000(ctx, base);
	// 823B6568: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B656C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6570: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B6574: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6578: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B657C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6580: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6588 size=196
    let mut pc: u32 = 0x823B6588;
    'dispatch: loop {
        match pc {
            0x823B6588 => {
    //   block [0x823B6588..0x823B664C)
	// 823B6588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B658C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6590: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B6594: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6598: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B659C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B65A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B65A4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B65A8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B65AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B65B0: 4BF0A389  bl 0x822c0938
	ctx.lr = 0x823B65B4;
	sub_822C0938(ctx, base);
	// 823B65B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B65B8: 41820028  beq 0x823b65e0
	if ctx.cr[0].eq {
	pc = 0x823B65E0; continue 'dispatch;
	}
	// 823B65BC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B65C0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B65C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B65C8: 392BF1D8  addi r9, r11, -0xe28
	ctx.r[9].s64 = ctx.r[11].s64 + -3624;
	// 823B65CC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B65D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B65D4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B65D8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B65DC: 48000008  b 0x823b65e4
	pc = 0x823B65E4; continue 'dispatch;
	// 823B65E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B65E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B65E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B65EC: 409A0044  bne cr6, 0x823b6630
	if !ctx.cr[6].eq {
	pc = 0x823B6630; continue 'dispatch;
	}
	// 823B65F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B65F4: 419A001C  beq cr6, 0x823b6610
	if ctx.cr[6].eq {
	pc = 0x823B6610; continue 'dispatch;
	}
	// 823B65F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B65FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B6600: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6604: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6608: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B660C: 4E800421  bctrl
	ctx.lr = 0x823B6610;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B6610: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B6614: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B6618: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B661C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6620: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B6624: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6628: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B662C: 4BF099D5  bl 0x822c0000
	ctx.lr = 0x823B6630;
	sub_822C0000(ctx, base);
	// 823B6630: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B6634: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6638: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B663C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6640: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B6644: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6648: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6650(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6650 size=196
    let mut pc: u32 = 0x823B6650;
    'dispatch: loop {
        match pc {
            0x823B6650 => {
    //   block [0x823B6650..0x823B6714)
	// 823B6650: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B6654: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6658: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B665C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6664: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6668: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B666C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6670: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B6674: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6678: 4BF0A2C1  bl 0x822c0938
	ctx.lr = 0x823B667C;
	sub_822C0938(ctx, base);
	// 823B667C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B6680: 41820028  beq 0x823b66a8
	if ctx.cr[0].eq {
	pc = 0x823B66A8; continue 'dispatch;
	}
	// 823B6684: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B6688: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B668C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6690: 392BF1EC  addi r9, r11, -0xe14
	ctx.r[9].s64 = ctx.r[11].s64 + -3604;
	// 823B6694: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6698: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B669C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B66A0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B66A4: 48000008  b 0x823b66ac
	pc = 0x823B66AC; continue 'dispatch;
	// 823B66A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B66AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B66B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B66B4: 409A0044  bne cr6, 0x823b66f8
	if !ctx.cr[6].eq {
	pc = 0x823B66F8; continue 'dispatch;
	}
	// 823B66B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B66BC: 419A001C  beq cr6, 0x823b66d8
	if ctx.cr[6].eq {
	pc = 0x823B66D8; continue 'dispatch;
	}
	// 823B66C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B66C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B66C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B66CC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B66D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B66D4: 4E800421  bctrl
	ctx.lr = 0x823B66D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B66D8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B66DC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B66E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B66E4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B66E8: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B66EC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B66F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B66F4: 4BF0990D  bl 0x822c0000
	ctx.lr = 0x823B66F8;
	sub_822C0000(ctx, base);
	// 823B66F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B66FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6700: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B6704: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6708: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B670C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6710: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6718 size=196
    let mut pc: u32 = 0x823B6718;
    'dispatch: loop {
        match pc {
            0x823B6718 => {
    //   block [0x823B6718..0x823B67DC)
	// 823B6718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B671C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6720: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B6724: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B672C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6730: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6734: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6738: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B673C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6740: 4BF0A1F9  bl 0x822c0938
	ctx.lr = 0x823B6744;
	sub_822C0938(ctx, base);
	// 823B6744: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B6748: 41820028  beq 0x823b6770
	if ctx.cr[0].eq {
	pc = 0x823B6770; continue 'dispatch;
	}
	// 823B674C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B6750: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B6754: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6758: 392BF200  addi r9, r11, -0xe00
	ctx.r[9].s64 = ctx.r[11].s64 + -3584;
	// 823B675C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6760: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B6764: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B6768: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B676C: 48000008  b 0x823b6774
	pc = 0x823B6774; continue 'dispatch;
	// 823B6770: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6774: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6778: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B677C: 409A0044  bne cr6, 0x823b67c0
	if !ctx.cr[6].eq {
	pc = 0x823B67C0; continue 'dispatch;
	}
	// 823B6780: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B6784: 419A001C  beq cr6, 0x823b67a0
	if ctx.cr[6].eq {
	pc = 0x823B67A0; continue 'dispatch;
	}
	// 823B6788: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B678C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B6790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6794: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6798: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B679C: 4E800421  bctrl
	ctx.lr = 0x823B67A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B67A0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B67A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B67A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B67AC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B67B0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B67B4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B67B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B67BC: 4BF09845  bl 0x822c0000
	ctx.lr = 0x823B67C0;
	sub_822C0000(ctx, base);
	// 823B67C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B67C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B67C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B67CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B67D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B67D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B67D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B67E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B67E0 size=196
    let mut pc: u32 = 0x823B67E0;
    'dispatch: loop {
        match pc {
            0x823B67E0 => {
    //   block [0x823B67E0..0x823B68A4)
	// 823B67E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B67E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B67E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B67EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B67F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B67F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B67F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B67FC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6800: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B6804: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6808: 4BF0A131  bl 0x822c0938
	ctx.lr = 0x823B680C;
	sub_822C0938(ctx, base);
	// 823B680C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B6810: 41820028  beq 0x823b6838
	if ctx.cr[0].eq {
	pc = 0x823B6838; continue 'dispatch;
	}
	// 823B6814: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B6818: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B681C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6820: 392BF214  addi r9, r11, -0xdec
	ctx.r[9].s64 = ctx.r[11].s64 + -3564;
	// 823B6824: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6828: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B682C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B6830: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B6834: 48000008  b 0x823b683c
	pc = 0x823B683C; continue 'dispatch;
	// 823B6838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B683C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6840: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6844: 409A0044  bne cr6, 0x823b6888
	if !ctx.cr[6].eq {
	pc = 0x823B6888; continue 'dispatch;
	}
	// 823B6848: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B684C: 419A001C  beq cr6, 0x823b6868
	if ctx.cr[6].eq {
	pc = 0x823B6868; continue 'dispatch;
	}
	// 823B6850: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6854: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B6858: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B685C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6860: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B6864: 4E800421  bctrl
	ctx.lr = 0x823B6868;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B6868: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B686C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B6870: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B6874: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6878: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B687C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6880: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B6884: 4BF0977D  bl 0x822c0000
	ctx.lr = 0x823B6888;
	sub_822C0000(ctx, base);
	// 823B6888: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B688C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B6894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6898: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B689C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B68A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B68A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B68A8 size=196
    let mut pc: u32 = 0x823B68A8;
    'dispatch: loop {
        match pc {
            0x823B68A8 => {
    //   block [0x823B68A8..0x823B696C)
	// 823B68A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B68AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B68B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B68B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B68B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B68BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B68C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B68C4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B68C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B68CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B68D0: 4BF0A069  bl 0x822c0938
	ctx.lr = 0x823B68D4;
	sub_822C0938(ctx, base);
	// 823B68D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B68D8: 41820028  beq 0x823b6900
	if ctx.cr[0].eq {
	pc = 0x823B6900; continue 'dispatch;
	}
	// 823B68DC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B68E0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B68E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B68E8: 392BF228  addi r9, r11, -0xdd8
	ctx.r[9].s64 = ctx.r[11].s64 + -3544;
	// 823B68EC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B68F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B68F4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B68F8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B68FC: 48000008  b 0x823b6904
	pc = 0x823B6904; continue 'dispatch;
	// 823B6900: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6904: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6908: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B690C: 409A0044  bne cr6, 0x823b6950
	if !ctx.cr[6].eq {
	pc = 0x823B6950; continue 'dispatch;
	}
	// 823B6910: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B6914: 419A001C  beq cr6, 0x823b6930
	if ctx.cr[6].eq {
	pc = 0x823B6930; continue 'dispatch;
	}
	// 823B6918: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B691C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B6920: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6924: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6928: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B692C: 4E800421  bctrl
	ctx.lr = 0x823B6930;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B6930: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B6934: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B6938: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B693C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6940: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B6944: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6948: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B694C: 4BF096B5  bl 0x822c0000
	ctx.lr = 0x823B6950;
	sub_822C0000(ctx, base);
	// 823B6950: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B6954: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6958: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B695C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6960: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B6964: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6968: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6970 size=196
    let mut pc: u32 = 0x823B6970;
    'dispatch: loop {
        match pc {
            0x823B6970 => {
    //   block [0x823B6970..0x823B6A34)
	// 823B6970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B6974: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6978: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B697C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6980: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6984: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6988: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B698C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6990: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B6994: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6998: 4BF09FA1  bl 0x822c0938
	ctx.lr = 0x823B699C;
	sub_822C0938(ctx, base);
	// 823B699C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B69A0: 41820028  beq 0x823b69c8
	if ctx.cr[0].eq {
	pc = 0x823B69C8; continue 'dispatch;
	}
	// 823B69A4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B69A8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B69AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B69B0: 392BF23C  addi r9, r11, -0xdc4
	ctx.r[9].s64 = ctx.r[11].s64 + -3524;
	// 823B69B4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B69B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B69BC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B69C0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B69C4: 48000008  b 0x823b69cc
	pc = 0x823B69CC; continue 'dispatch;
	// 823B69C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B69CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B69D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B69D4: 409A0044  bne cr6, 0x823b6a18
	if !ctx.cr[6].eq {
	pc = 0x823B6A18; continue 'dispatch;
	}
	// 823B69D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B69DC: 419A001C  beq cr6, 0x823b69f8
	if ctx.cr[6].eq {
	pc = 0x823B69F8; continue 'dispatch;
	}
	// 823B69E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B69E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B69E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B69EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B69F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B69F4: 4E800421  bctrl
	ctx.lr = 0x823B69F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B69F8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B69FC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B6A00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B6A04: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6A08: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B6A0C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6A10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B6A14: 4BF095ED  bl 0x822c0000
	ctx.lr = 0x823B6A18;
	sub_822C0000(ctx, base);
	// 823B6A18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B6A1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6A20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B6A24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6A28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B6A2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6A30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6A38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6A38 size=196
    let mut pc: u32 = 0x823B6A38;
    'dispatch: loop {
        match pc {
            0x823B6A38 => {
    //   block [0x823B6A38..0x823B6AFC)
	// 823B6A38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B6A3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6A40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B6A44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6A48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6A4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6A50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6A54: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6A58: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B6A5C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6A60: 4BF09ED9  bl 0x822c0938
	ctx.lr = 0x823B6A64;
	sub_822C0938(ctx, base);
	// 823B6A64: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B6A68: 41820028  beq 0x823b6a90
	if ctx.cr[0].eq {
	pc = 0x823B6A90; continue 'dispatch;
	}
	// 823B6A6C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B6A70: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B6A74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6A78: 392BF250  addi r9, r11, -0xdb0
	ctx.r[9].s64 = ctx.r[11].s64 + -3504;
	// 823B6A7C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6A80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B6A84: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B6A88: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B6A8C: 48000008  b 0x823b6a94
	pc = 0x823B6A94; continue 'dispatch;
	// 823B6A90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6A94: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6A98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6A9C: 409A0044  bne cr6, 0x823b6ae0
	if !ctx.cr[6].eq {
	pc = 0x823B6AE0; continue 'dispatch;
	}
	// 823B6AA0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B6AA4: 419A001C  beq cr6, 0x823b6ac0
	if ctx.cr[6].eq {
	pc = 0x823B6AC0; continue 'dispatch;
	}
	// 823B6AA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6AAC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B6AB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6AB4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6AB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B6ABC: 4E800421  bctrl
	ctx.lr = 0x823B6AC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B6AC0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B6AC4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B6AC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B6ACC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6AD0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B6AD4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6AD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B6ADC: 4BF09525  bl 0x822c0000
	ctx.lr = 0x823B6AE0;
	sub_822C0000(ctx, base);
	// 823B6AE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B6AE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6AE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B6AEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6AF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B6AF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6AF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6B00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6B00 size=196
    let mut pc: u32 = 0x823B6B00;
    'dispatch: loop {
        match pc {
            0x823B6B00 => {
    //   block [0x823B6B00..0x823B6BC4)
	// 823B6B00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B6B04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6B08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B6B0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6B10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6B14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6B18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6B1C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6B20: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B6B24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6B28: 4BF09E11  bl 0x822c0938
	ctx.lr = 0x823B6B2C;
	sub_822C0938(ctx, base);
	// 823B6B2C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B6B30: 41820028  beq 0x823b6b58
	if ctx.cr[0].eq {
	pc = 0x823B6B58; continue 'dispatch;
	}
	// 823B6B34: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B6B38: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B6B3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6B40: 392BF264  addi r9, r11, -0xd9c
	ctx.r[9].s64 = ctx.r[11].s64 + -3484;
	// 823B6B44: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6B48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B6B4C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B6B50: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B6B54: 48000008  b 0x823b6b5c
	pc = 0x823B6B5C; continue 'dispatch;
	// 823B6B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6B5C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6B60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6B64: 409A0044  bne cr6, 0x823b6ba8
	if !ctx.cr[6].eq {
	pc = 0x823B6BA8; continue 'dispatch;
	}
	// 823B6B68: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B6B6C: 419A001C  beq cr6, 0x823b6b88
	if ctx.cr[6].eq {
	pc = 0x823B6B88; continue 'dispatch;
	}
	// 823B6B70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6B74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B6B78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6B7C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6B80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B6B84: 4E800421  bctrl
	ctx.lr = 0x823B6B88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B6B88: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B6B8C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B6B90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B6B94: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6B98: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B6B9C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6BA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B6BA4: 4BF0945D  bl 0x822c0000
	ctx.lr = 0x823B6BA8;
	sub_822C0000(ctx, base);
	// 823B6BA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B6BAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6BB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B6BB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6BB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B6BBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6BC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6BC8 size=196
    let mut pc: u32 = 0x823B6BC8;
    'dispatch: loop {
        match pc {
            0x823B6BC8 => {
    //   block [0x823B6BC8..0x823B6C8C)
	// 823B6BC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B6BCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6BD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B6BD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6BD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6BDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6BE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6BE4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6BE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B6BEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6BF0: 4BF09D49  bl 0x822c0938
	ctx.lr = 0x823B6BF4;
	sub_822C0938(ctx, base);
	// 823B6BF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B6BF8: 41820028  beq 0x823b6c20
	if ctx.cr[0].eq {
	pc = 0x823B6C20; continue 'dispatch;
	}
	// 823B6BFC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B6C00: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B6C04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6C08: 392BF278  addi r9, r11, -0xd88
	ctx.r[9].s64 = ctx.r[11].s64 + -3464;
	// 823B6C0C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6C10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B6C14: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B6C18: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B6C1C: 48000008  b 0x823b6c24
	pc = 0x823B6C24; continue 'dispatch;
	// 823B6C20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6C24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6C28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6C2C: 409A0044  bne cr6, 0x823b6c70
	if !ctx.cr[6].eq {
	pc = 0x823B6C70; continue 'dispatch;
	}
	// 823B6C30: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B6C34: 419A001C  beq cr6, 0x823b6c50
	if ctx.cr[6].eq {
	pc = 0x823B6C50; continue 'dispatch;
	}
	// 823B6C38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6C3C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B6C40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6C44: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6C48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B6C4C: 4E800421  bctrl
	ctx.lr = 0x823B6C50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B6C50: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B6C54: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B6C58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B6C5C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6C60: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B6C64: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6C68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B6C6C: 4BF09395  bl 0x822c0000
	ctx.lr = 0x823B6C70;
	sub_822C0000(ctx, base);
	// 823B6C70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B6C74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B6C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6C80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B6C84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6C88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6C90 size=196
    let mut pc: u32 = 0x823B6C90;
    'dispatch: loop {
        match pc {
            0x823B6C90 => {
    //   block [0x823B6C90..0x823B6D54)
	// 823B6C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B6C94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6C98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B6C9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6CA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6CA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6CA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6CAC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6CB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B6CB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6CB8: 4BF09C81  bl 0x822c0938
	ctx.lr = 0x823B6CBC;
	sub_822C0938(ctx, base);
	// 823B6CBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B6CC0: 41820028  beq 0x823b6ce8
	if ctx.cr[0].eq {
	pc = 0x823B6CE8; continue 'dispatch;
	}
	// 823B6CC4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B6CC8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B6CCC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6CD0: 392BF28C  addi r9, r11, -0xd74
	ctx.r[9].s64 = ctx.r[11].s64 + -3444;
	// 823B6CD4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6CD8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B6CDC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B6CE0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B6CE4: 48000008  b 0x823b6cec
	pc = 0x823B6CEC; continue 'dispatch;
	// 823B6CE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6CEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6CF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6CF4: 409A0044  bne cr6, 0x823b6d38
	if !ctx.cr[6].eq {
	pc = 0x823B6D38; continue 'dispatch;
	}
	// 823B6CF8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B6CFC: 419A001C  beq cr6, 0x823b6d18
	if ctx.cr[6].eq {
	pc = 0x823B6D18; continue 'dispatch;
	}
	// 823B6D00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6D04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B6D08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6D0C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6D10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B6D14: 4E800421  bctrl
	ctx.lr = 0x823B6D18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B6D18: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B6D1C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B6D20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B6D24: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6D28: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B6D2C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6D30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B6D34: 4BF092CD  bl 0x822c0000
	ctx.lr = 0x823B6D38;
	sub_822C0000(ctx, base);
	// 823B6D38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B6D3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6D40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B6D44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6D48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B6D4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6D58 size=196
    let mut pc: u32 = 0x823B6D58;
    'dispatch: loop {
        match pc {
            0x823B6D58 => {
    //   block [0x823B6D58..0x823B6E1C)
	// 823B6D58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B6D5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6D60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B6D64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6D68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6D6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6D70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6D74: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6D78: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B6D7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6D80: 4BF09BB9  bl 0x822c0938
	ctx.lr = 0x823B6D84;
	sub_822C0938(ctx, base);
	// 823B6D84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B6D88: 41820028  beq 0x823b6db0
	if ctx.cr[0].eq {
	pc = 0x823B6DB0; continue 'dispatch;
	}
	// 823B6D8C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B6D90: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B6D94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6D98: 392BF2A0  addi r9, r11, -0xd60
	ctx.r[9].s64 = ctx.r[11].s64 + -3424;
	// 823B6D9C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6DA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B6DA4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B6DA8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B6DAC: 48000008  b 0x823b6db4
	pc = 0x823B6DB4; continue 'dispatch;
	// 823B6DB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6DB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6DB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6DBC: 409A0044  bne cr6, 0x823b6e00
	if !ctx.cr[6].eq {
	pc = 0x823B6E00; continue 'dispatch;
	}
	// 823B6DC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B6DC4: 419A001C  beq cr6, 0x823b6de0
	if ctx.cr[6].eq {
	pc = 0x823B6DE0; continue 'dispatch;
	}
	// 823B6DC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6DCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B6DD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6DD4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6DD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B6DDC: 4E800421  bctrl
	ctx.lr = 0x823B6DE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B6DE0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B6DE4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B6DE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B6DEC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6DF0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B6DF4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6DF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B6DFC: 4BF09205  bl 0x822c0000
	ctx.lr = 0x823B6E00;
	sub_822C0000(ctx, base);
	// 823B6E00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B6E04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6E08: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B6E0C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6E10: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B6E14: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6E18: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6E20 size=196
    let mut pc: u32 = 0x823B6E20;
    'dispatch: loop {
        match pc {
            0x823B6E20 => {
    //   block [0x823B6E20..0x823B6EE4)
	// 823B6E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B6E24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6E28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B6E2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6E30: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6E34: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6E38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6E3C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6E40: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B6E44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6E48: 4BF09AF1  bl 0x822c0938
	ctx.lr = 0x823B6E4C;
	sub_822C0938(ctx, base);
	// 823B6E4C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B6E50: 41820028  beq 0x823b6e78
	if ctx.cr[0].eq {
	pc = 0x823B6E78; continue 'dispatch;
	}
	// 823B6E54: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B6E58: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B6E5C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6E60: 392BF2B4  addi r9, r11, -0xd4c
	ctx.r[9].s64 = ctx.r[11].s64 + -3404;
	// 823B6E64: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6E68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B6E6C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B6E70: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B6E74: 48000008  b 0x823b6e7c
	pc = 0x823B6E7C; continue 'dispatch;
	// 823B6E78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6E7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6E80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6E84: 409A0044  bne cr6, 0x823b6ec8
	if !ctx.cr[6].eq {
	pc = 0x823B6EC8; continue 'dispatch;
	}
	// 823B6E88: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B6E8C: 419A001C  beq cr6, 0x823b6ea8
	if ctx.cr[6].eq {
	pc = 0x823B6EA8; continue 'dispatch;
	}
	// 823B6E90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6E94: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B6E98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6E9C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6EA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B6EA4: 4E800421  bctrl
	ctx.lr = 0x823B6EA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B6EA8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B6EAC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B6EB0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B6EB4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6EB8: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B6EBC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6EC0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B6EC4: 4BF0913D  bl 0x822c0000
	ctx.lr = 0x823B6EC8;
	sub_822C0000(ctx, base);
	// 823B6EC8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B6ECC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6ED0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B6ED4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6ED8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B6EDC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6EE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6EE8 size=196
    let mut pc: u32 = 0x823B6EE8;
    'dispatch: loop {
        match pc {
            0x823B6EE8 => {
    //   block [0x823B6EE8..0x823B6FAC)
	// 823B6EE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B6EEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6EF0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B6EF4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6EF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6EFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6F00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6F04: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6F08: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B6F0C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6F10: 4BF09A29  bl 0x822c0938
	ctx.lr = 0x823B6F14;
	sub_822C0938(ctx, base);
	// 823B6F14: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B6F18: 41820028  beq 0x823b6f40
	if ctx.cr[0].eq {
	pc = 0x823B6F40; continue 'dispatch;
	}
	// 823B6F1C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B6F20: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B6F24: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6F28: 392BF2C8  addi r9, r11, -0xd38
	ctx.r[9].s64 = ctx.r[11].s64 + -3384;
	// 823B6F2C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6F30: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B6F34: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B6F38: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B6F3C: 48000008  b 0x823b6f44
	pc = 0x823B6F44; continue 'dispatch;
	// 823B6F40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6F44: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6F48: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B6F4C: 409A0044  bne cr6, 0x823b6f90
	if !ctx.cr[6].eq {
	pc = 0x823B6F90; continue 'dispatch;
	}
	// 823B6F50: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B6F54: 419A001C  beq cr6, 0x823b6f70
	if ctx.cr[6].eq {
	pc = 0x823B6F70; continue 'dispatch;
	}
	// 823B6F58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B6F5C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B6F60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B6F64: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B6F68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B6F6C: 4E800421  bctrl
	ctx.lr = 0x823B6F70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B6F70: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B6F74: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B6F78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B6F7C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B6F80: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B6F84: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B6F88: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B6F8C: 4BF09075  bl 0x822c0000
	ctx.lr = 0x823B6F90;
	sub_822C0000(ctx, base);
	// 823B6F90: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B6F94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B6F98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B6F9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B6FA0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B6FA4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B6FA8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B6FB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B6FB0 size=196
    let mut pc: u32 = 0x823B6FB0;
    'dispatch: loop {
        match pc {
            0x823B6FB0 => {
    //   block [0x823B6FB0..0x823B7074)
	// 823B6FB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B6FB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B6FB8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B6FBC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B6FC0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B6FC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B6FC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B6FCC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B6FD0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B6FD4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B6FD8: 4BF09961  bl 0x822c0938
	ctx.lr = 0x823B6FDC;
	sub_822C0938(ctx, base);
	// 823B6FDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B6FE0: 41820028  beq 0x823b7008
	if ctx.cr[0].eq {
	pc = 0x823B7008; continue 'dispatch;
	}
	// 823B6FE4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B6FE8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B6FEC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B6FF0: 392BF2DC  addi r9, r11, -0xd24
	ctx.r[9].s64 = ctx.r[11].s64 + -3364;
	// 823B6FF4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B6FF8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B6FFC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7000: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B7004: 48000008  b 0x823b700c
	pc = 0x823B700C; continue 'dispatch;
	// 823B7008: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B700C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7010: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7014: 409A0044  bne cr6, 0x823b7058
	if !ctx.cr[6].eq {
	pc = 0x823B7058; continue 'dispatch;
	}
	// 823B7018: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B701C: 419A001C  beq cr6, 0x823b7038
	if ctx.cr[6].eq {
	pc = 0x823B7038; continue 'dispatch;
	}
	// 823B7020: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7024: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B7028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B702C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7030: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B7034: 4E800421  bctrl
	ctx.lr = 0x823B7038;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7038: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B703C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7040: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B7044: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B7048: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B704C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7050: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B7054: 4BF08FAD  bl 0x822c0000
	ctx.lr = 0x823B7058;
	sub_822C0000(ctx, base);
	// 823B7058: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B705C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7060: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B7064: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7068: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B706C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7070: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7078 size=196
    let mut pc: u32 = 0x823B7078;
    'dispatch: loop {
        match pc {
            0x823B7078 => {
    //   block [0x823B7078..0x823B713C)
	// 823B7078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B707C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7080: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B7084: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7088: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B708C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7090: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7094: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B7098: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B709C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B70A0: 4BF09899  bl 0x822c0938
	ctx.lr = 0x823B70A4;
	sub_822C0938(ctx, base);
	// 823B70A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B70A8: 41820028  beq 0x823b70d0
	if ctx.cr[0].eq {
	pc = 0x823B70D0; continue 'dispatch;
	}
	// 823B70AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B70B0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B70B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B70B8: 392BF2F0  addi r9, r11, -0xd10
	ctx.r[9].s64 = ctx.r[11].s64 + -3344;
	// 823B70BC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B70C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B70C4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B70C8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B70CC: 48000008  b 0x823b70d4
	pc = 0x823B70D4; continue 'dispatch;
	// 823B70D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B70D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B70D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B70DC: 409A0044  bne cr6, 0x823b7120
	if !ctx.cr[6].eq {
	pc = 0x823B7120; continue 'dispatch;
	}
	// 823B70E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B70E4: 419A001C  beq cr6, 0x823b7100
	if ctx.cr[6].eq {
	pc = 0x823B7100; continue 'dispatch;
	}
	// 823B70E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B70EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B70F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B70F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B70F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B70FC: 4E800421  bctrl
	ctx.lr = 0x823B7100;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7100: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B7104: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7108: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B710C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B7110: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B7114: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7118: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B711C: 4BF08EE5  bl 0x822c0000
	ctx.lr = 0x823B7120;
	sub_822C0000(ctx, base);
	// 823B7120: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B7124: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B712C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7130: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B7134: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7138: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7140 size=196
    let mut pc: u32 = 0x823B7140;
    'dispatch: loop {
        match pc {
            0x823B7140 => {
    //   block [0x823B7140..0x823B7204)
	// 823B7140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7148: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B714C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7150: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7154: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7158: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B715C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B7160: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B7164: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7168: 4BF097D1  bl 0x822c0938
	ctx.lr = 0x823B716C;
	sub_822C0938(ctx, base);
	// 823B716C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7170: 41820028  beq 0x823b7198
	if ctx.cr[0].eq {
	pc = 0x823B7198; continue 'dispatch;
	}
	// 823B7174: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7178: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B717C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7180: 392BF304  addi r9, r11, -0xcfc
	ctx.r[9].s64 = ctx.r[11].s64 + -3324;
	// 823B7184: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7188: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B718C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7190: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B7194: 48000008  b 0x823b719c
	pc = 0x823B719C; continue 'dispatch;
	// 823B7198: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B719C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B71A0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B71A4: 409A0044  bne cr6, 0x823b71e8
	if !ctx.cr[6].eq {
	pc = 0x823B71E8; continue 'dispatch;
	}
	// 823B71A8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B71AC: 419A001C  beq cr6, 0x823b71c8
	if ctx.cr[6].eq {
	pc = 0x823B71C8; continue 'dispatch;
	}
	// 823B71B0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B71B4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B71B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B71BC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B71C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B71C4: 4E800421  bctrl
	ctx.lr = 0x823B71C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B71C8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B71CC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B71D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B71D4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B71D8: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B71DC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B71E0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B71E4: 4BF08E1D  bl 0x822c0000
	ctx.lr = 0x823B71E8;
	sub_822C0000(ctx, base);
	// 823B71E8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B71EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B71F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B71F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B71F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B71FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7208 size=196
    let mut pc: u32 = 0x823B7208;
    'dispatch: loop {
        match pc {
            0x823B7208 => {
    //   block [0x823B7208..0x823B72CC)
	// 823B7208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B720C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7210: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B7214: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7218: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B721C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7220: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7224: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B7228: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B722C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7230: 4BF09709  bl 0x822c0938
	ctx.lr = 0x823B7234;
	sub_822C0938(ctx, base);
	// 823B7234: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7238: 41820028  beq 0x823b7260
	if ctx.cr[0].eq {
	pc = 0x823B7260; continue 'dispatch;
	}
	// 823B723C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7240: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B7244: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7248: 392BF318  addi r9, r11, -0xce8
	ctx.r[9].s64 = ctx.r[11].s64 + -3304;
	// 823B724C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7250: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B7254: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7258: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B725C: 48000008  b 0x823b7264
	pc = 0x823B7264; continue 'dispatch;
	// 823B7260: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7264: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7268: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B726C: 409A0044  bne cr6, 0x823b72b0
	if !ctx.cr[6].eq {
	pc = 0x823B72B0; continue 'dispatch;
	}
	// 823B7270: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B7274: 419A001C  beq cr6, 0x823b7290
	if ctx.cr[6].eq {
	pc = 0x823B7290; continue 'dispatch;
	}
	// 823B7278: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B727C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B7280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B7284: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7288: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B728C: 4E800421  bctrl
	ctx.lr = 0x823B7290;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7290: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B7294: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7298: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B729C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B72A0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B72A4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B72A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B72AC: 4BF08D55  bl 0x822c0000
	ctx.lr = 0x823B72B0;
	sub_822C0000(ctx, base);
	// 823B72B0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B72B4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B72B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B72BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B72C0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B72C4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B72C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B72D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B72D0 size=196
    let mut pc: u32 = 0x823B72D0;
    'dispatch: loop {
        match pc {
            0x823B72D0 => {
    //   block [0x823B72D0..0x823B7394)
	// 823B72D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B72D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B72D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B72DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B72E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B72E4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B72E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B72EC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B72F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B72F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B72F8: 4BF09641  bl 0x822c0938
	ctx.lr = 0x823B72FC;
	sub_822C0938(ctx, base);
	// 823B72FC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7300: 41820028  beq 0x823b7328
	if ctx.cr[0].eq {
	pc = 0x823B7328; continue 'dispatch;
	}
	// 823B7304: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7308: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B730C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7310: 392BF32C  addi r9, r11, -0xcd4
	ctx.r[9].s64 = ctx.r[11].s64 + -3284;
	// 823B7314: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7318: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B731C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7320: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B7324: 48000008  b 0x823b732c
	pc = 0x823B732C; continue 'dispatch;
	// 823B7328: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B732C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7330: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7334: 409A0044  bne cr6, 0x823b7378
	if !ctx.cr[6].eq {
	pc = 0x823B7378; continue 'dispatch;
	}
	// 823B7338: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B733C: 419A001C  beq cr6, 0x823b7358
	if ctx.cr[6].eq {
	pc = 0x823B7358; continue 'dispatch;
	}
	// 823B7340: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7344: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B7348: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B734C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7350: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B7354: 4E800421  bctrl
	ctx.lr = 0x823B7358;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7358: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B735C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7360: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B7364: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B7368: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B736C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7370: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B7374: 4BF08C8D  bl 0x822c0000
	ctx.lr = 0x823B7378;
	sub_822C0000(ctx, base);
	// 823B7378: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B737C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B7384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7388: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B738C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7398 size=196
    let mut pc: u32 = 0x823B7398;
    'dispatch: loop {
        match pc {
            0x823B7398 => {
    //   block [0x823B7398..0x823B745C)
	// 823B7398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B739C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B73A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B73A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B73A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B73AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B73B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B73B4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B73B8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B73BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B73C0: 4BF09579  bl 0x822c0938
	ctx.lr = 0x823B73C4;
	sub_822C0938(ctx, base);
	// 823B73C4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B73C8: 41820028  beq 0x823b73f0
	if ctx.cr[0].eq {
	pc = 0x823B73F0; continue 'dispatch;
	}
	// 823B73CC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B73D0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B73D4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B73D8: 392BF340  addi r9, r11, -0xcc0
	ctx.r[9].s64 = ctx.r[11].s64 + -3264;
	// 823B73DC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B73E0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B73E4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B73E8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B73EC: 48000008  b 0x823b73f4
	pc = 0x823B73F4; continue 'dispatch;
	// 823B73F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B73F4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B73F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B73FC: 409A0044  bne cr6, 0x823b7440
	if !ctx.cr[6].eq {
	pc = 0x823B7440; continue 'dispatch;
	}
	// 823B7400: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B7404: 419A001C  beq cr6, 0x823b7420
	if ctx.cr[6].eq {
	pc = 0x823B7420; continue 'dispatch;
	}
	// 823B7408: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B740C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B7410: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B7414: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7418: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B741C: 4E800421  bctrl
	ctx.lr = 0x823B7420;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7420: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B7424: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7428: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B742C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B7430: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B7434: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7438: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B743C: 4BF08BC5  bl 0x822c0000
	ctx.lr = 0x823B7440;
	sub_822C0000(ctx, base);
	// 823B7440: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B7444: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7448: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B744C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7450: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B7454: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7458: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7460(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7460 size=196
    let mut pc: u32 = 0x823B7460;
    'dispatch: loop {
        match pc {
            0x823B7460 => {
    //   block [0x823B7460..0x823B7524)
	// 823B7460: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7464: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7468: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B746C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7470: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7474: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7478: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B747C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B7480: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B7484: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7488: 4BF094B1  bl 0x822c0938
	ctx.lr = 0x823B748C;
	sub_822C0938(ctx, base);
	// 823B748C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7490: 41820028  beq 0x823b74b8
	if ctx.cr[0].eq {
	pc = 0x823B74B8; continue 'dispatch;
	}
	// 823B7494: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7498: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B749C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B74A0: 392BF354  addi r9, r11, -0xcac
	ctx.r[9].s64 = ctx.r[11].s64 + -3244;
	// 823B74A4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B74A8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B74AC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B74B0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B74B4: 48000008  b 0x823b74bc
	pc = 0x823B74BC; continue 'dispatch;
	// 823B74B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B74BC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B74C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B74C4: 409A0044  bne cr6, 0x823b7508
	if !ctx.cr[6].eq {
	pc = 0x823B7508; continue 'dispatch;
	}
	// 823B74C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B74CC: 419A001C  beq cr6, 0x823b74e8
	if ctx.cr[6].eq {
	pc = 0x823B74E8; continue 'dispatch;
	}
	// 823B74D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B74D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B74D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B74DC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B74E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B74E4: 4E800421  bctrl
	ctx.lr = 0x823B74E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B74E8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B74EC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B74F0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B74F4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B74F8: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B74FC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7500: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B7504: 4BF08AFD  bl 0x822c0000
	ctx.lr = 0x823B7508;
	sub_822C0000(ctx, base);
	// 823B7508: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B750C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7510: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B7514: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7518: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B751C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7528 size=196
    let mut pc: u32 = 0x823B7528;
    'dispatch: loop {
        match pc {
            0x823B7528 => {
    //   block [0x823B7528..0x823B75EC)
	// 823B7528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B752C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7530: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B7534: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7538: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B753C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7540: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7544: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B7548: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B754C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7550: 4BF093E9  bl 0x822c0938
	ctx.lr = 0x823B7554;
	sub_822C0938(ctx, base);
	// 823B7554: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7558: 41820028  beq 0x823b7580
	if ctx.cr[0].eq {
	pc = 0x823B7580; continue 'dispatch;
	}
	// 823B755C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7560: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B7564: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7568: 392BF368  addi r9, r11, -0xc98
	ctx.r[9].s64 = ctx.r[11].s64 + -3224;
	// 823B756C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7570: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B7574: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7578: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B757C: 48000008  b 0x823b7584
	pc = 0x823B7584; continue 'dispatch;
	// 823B7580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7584: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7588: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B758C: 409A0044  bne cr6, 0x823b75d0
	if !ctx.cr[6].eq {
	pc = 0x823B75D0; continue 'dispatch;
	}
	// 823B7590: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B7594: 419A001C  beq cr6, 0x823b75b0
	if ctx.cr[6].eq {
	pc = 0x823B75B0; continue 'dispatch;
	}
	// 823B7598: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B759C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B75A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B75A4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B75A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B75AC: 4E800421  bctrl
	ctx.lr = 0x823B75B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B75B0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B75B4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B75B8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B75BC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B75C0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B75C4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B75C8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B75CC: 4BF08A35  bl 0x822c0000
	ctx.lr = 0x823B75D0;
	sub_822C0000(ctx, base);
	// 823B75D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B75D4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B75D8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B75DC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B75E0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B75E4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B75E8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B75F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B75F0 size=196
    let mut pc: u32 = 0x823B75F0;
    'dispatch: loop {
        match pc {
            0x823B75F0 => {
    //   block [0x823B75F0..0x823B76B4)
	// 823B75F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B75F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B75F8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B75FC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7600: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7604: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B760C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B7610: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B7614: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7618: 4BF09321  bl 0x822c0938
	ctx.lr = 0x823B761C;
	sub_822C0938(ctx, base);
	// 823B761C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7620: 41820028  beq 0x823b7648
	if ctx.cr[0].eq {
	pc = 0x823B7648; continue 'dispatch;
	}
	// 823B7624: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7628: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B762C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7630: 392BF37C  addi r9, r11, -0xc84
	ctx.r[9].s64 = ctx.r[11].s64 + -3204;
	// 823B7634: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7638: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B763C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7640: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B7644: 48000008  b 0x823b764c
	pc = 0x823B764C; continue 'dispatch;
	// 823B7648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B764C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7650: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7654: 409A0044  bne cr6, 0x823b7698
	if !ctx.cr[6].eq {
	pc = 0x823B7698; continue 'dispatch;
	}
	// 823B7658: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B765C: 419A001C  beq cr6, 0x823b7678
	if ctx.cr[6].eq {
	pc = 0x823B7678; continue 'dispatch;
	}
	// 823B7660: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7664: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B7668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B766C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7670: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B7674: 4E800421  bctrl
	ctx.lr = 0x823B7678;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7678: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B767C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7680: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B7684: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B7688: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B768C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7690: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B7694: 4BF0896D  bl 0x822c0000
	ctx.lr = 0x823B7698;
	sub_822C0000(ctx, base);
	// 823B7698: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B769C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B76A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B76A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B76A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B76AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B76B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B76B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B76B8 size=196
    let mut pc: u32 = 0x823B76B8;
    'dispatch: loop {
        match pc {
            0x823B76B8 => {
    //   block [0x823B76B8..0x823B777C)
	// 823B76B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B76BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B76C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B76C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B76C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B76CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B76D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B76D4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B76D8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B76DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B76E0: 4BF09259  bl 0x822c0938
	ctx.lr = 0x823B76E4;
	sub_822C0938(ctx, base);
	// 823B76E4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B76E8: 41820028  beq 0x823b7710
	if ctx.cr[0].eq {
	pc = 0x823B7710; continue 'dispatch;
	}
	// 823B76EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B76F0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B76F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B76F8: 392BF390  addi r9, r11, -0xc70
	ctx.r[9].s64 = ctx.r[11].s64 + -3184;
	// 823B76FC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7700: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B7704: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7708: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B770C: 48000008  b 0x823b7714
	pc = 0x823B7714; continue 'dispatch;
	// 823B7710: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7714: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7718: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B771C: 409A0044  bne cr6, 0x823b7760
	if !ctx.cr[6].eq {
	pc = 0x823B7760; continue 'dispatch;
	}
	// 823B7720: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B7724: 419A001C  beq cr6, 0x823b7740
	if ctx.cr[6].eq {
	pc = 0x823B7740; continue 'dispatch;
	}
	// 823B7728: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B772C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B7730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B7734: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7738: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B773C: 4E800421  bctrl
	ctx.lr = 0x823B7740;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7740: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B7744: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7748: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B774C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B7750: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B7754: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7758: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B775C: 4BF088A5  bl 0x822c0000
	ctx.lr = 0x823B7760;
	sub_822C0000(ctx, base);
	// 823B7760: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B7764: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7768: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B776C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7770: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B7774: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7780 size=196
    let mut pc: u32 = 0x823B7780;
    'dispatch: loop {
        match pc {
            0x823B7780 => {
    //   block [0x823B7780..0x823B7844)
	// 823B7780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7788: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B778C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7790: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7794: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B779C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B77A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B77A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B77A8: 4BF09191  bl 0x822c0938
	ctx.lr = 0x823B77AC;
	sub_822C0938(ctx, base);
	// 823B77AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B77B0: 41820028  beq 0x823b77d8
	if ctx.cr[0].eq {
	pc = 0x823B77D8; continue 'dispatch;
	}
	// 823B77B4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B77B8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B77BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B77C0: 392BF3A4  addi r9, r11, -0xc5c
	ctx.r[9].s64 = ctx.r[11].s64 + -3164;
	// 823B77C4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B77C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B77CC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B77D0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B77D4: 48000008  b 0x823b77dc
	pc = 0x823B77DC; continue 'dispatch;
	// 823B77D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B77DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B77E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B77E4: 409A0044  bne cr6, 0x823b7828
	if !ctx.cr[6].eq {
	pc = 0x823B7828; continue 'dispatch;
	}
	// 823B77E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B77EC: 419A001C  beq cr6, 0x823b7808
	if ctx.cr[6].eq {
	pc = 0x823B7808; continue 'dispatch;
	}
	// 823B77F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B77F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B77F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B77FC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7800: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B7804: 4E800421  bctrl
	ctx.lr = 0x823B7808;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7808: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B780C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7810: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B7814: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B7818: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B781C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7820: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B7824: 4BF087DD  bl 0x822c0000
	ctx.lr = 0x823B7828;
	sub_822C0000(ctx, base);
	// 823B7828: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B782C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7830: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B7834: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7838: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B783C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7840: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7848 size=196
    let mut pc: u32 = 0x823B7848;
    'dispatch: loop {
        match pc {
            0x823B7848 => {
    //   block [0x823B7848..0x823B790C)
	// 823B7848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B784C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7850: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B7854: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7858: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B785C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7860: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7864: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B7868: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B786C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7870: 4BF090C9  bl 0x822c0938
	ctx.lr = 0x823B7874;
	sub_822C0938(ctx, base);
	// 823B7874: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7878: 41820028  beq 0x823b78a0
	if ctx.cr[0].eq {
	pc = 0x823B78A0; continue 'dispatch;
	}
	// 823B787C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7880: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B7884: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7888: 392BF3B8  addi r9, r11, -0xc48
	ctx.r[9].s64 = ctx.r[11].s64 + -3144;
	// 823B788C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7890: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B7894: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7898: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B789C: 48000008  b 0x823b78a4
	pc = 0x823B78A4; continue 'dispatch;
	// 823B78A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B78A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B78A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B78AC: 409A0044  bne cr6, 0x823b78f0
	if !ctx.cr[6].eq {
	pc = 0x823B78F0; continue 'dispatch;
	}
	// 823B78B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B78B4: 419A001C  beq cr6, 0x823b78d0
	if ctx.cr[6].eq {
	pc = 0x823B78D0; continue 'dispatch;
	}
	// 823B78B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B78BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B78C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B78C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B78C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B78CC: 4E800421  bctrl
	ctx.lr = 0x823B78D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B78D0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B78D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B78D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B78DC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B78E0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B78E4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B78E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B78EC: 4BF08715  bl 0x822c0000
	ctx.lr = 0x823B78F0;
	sub_822C0000(ctx, base);
	// 823B78F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B78F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B78F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B78FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7900: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B7904: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7908: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7910 size=196
    let mut pc: u32 = 0x823B7910;
    'dispatch: loop {
        match pc {
            0x823B7910 => {
    //   block [0x823B7910..0x823B79D4)
	// 823B7910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7914: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7918: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B791C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7920: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7924: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7928: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B792C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B7930: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B7934: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7938: 4BF09001  bl 0x822c0938
	ctx.lr = 0x823B793C;
	sub_822C0938(ctx, base);
	// 823B793C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7940: 41820028  beq 0x823b7968
	if ctx.cr[0].eq {
	pc = 0x823B7968; continue 'dispatch;
	}
	// 823B7944: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7948: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B794C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7950: 392BF3CC  addi r9, r11, -0xc34
	ctx.r[9].s64 = ctx.r[11].s64 + -3124;
	// 823B7954: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7958: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B795C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7960: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B7964: 48000008  b 0x823b796c
	pc = 0x823B796C; continue 'dispatch;
	// 823B7968: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B796C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7970: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7974: 409A0044  bne cr6, 0x823b79b8
	if !ctx.cr[6].eq {
	pc = 0x823B79B8; continue 'dispatch;
	}
	// 823B7978: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B797C: 419A001C  beq cr6, 0x823b7998
	if ctx.cr[6].eq {
	pc = 0x823B7998; continue 'dispatch;
	}
	// 823B7980: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7984: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B7988: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B798C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7990: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B7994: 4E800421  bctrl
	ctx.lr = 0x823B7998;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7998: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B799C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B79A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B79A4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B79A8: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B79AC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B79B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B79B4: 4BF0864D  bl 0x822c0000
	ctx.lr = 0x823B79B8;
	sub_822C0000(ctx, base);
	// 823B79B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B79BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B79C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B79C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B79C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B79CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B79D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B79D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B79D8 size=196
    let mut pc: u32 = 0x823B79D8;
    'dispatch: loop {
        match pc {
            0x823B79D8 => {
    //   block [0x823B79D8..0x823B7A9C)
	// 823B79D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B79DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B79E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B79E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B79E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B79EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B79F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B79F4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B79F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B79FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7A00: 4BF08F39  bl 0x822c0938
	ctx.lr = 0x823B7A04;
	sub_822C0938(ctx, base);
	// 823B7A04: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7A08: 41820028  beq 0x823b7a30
	if ctx.cr[0].eq {
	pc = 0x823B7A30; continue 'dispatch;
	}
	// 823B7A0C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7A10: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B7A14: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7A18: 392BF3E0  addi r9, r11, -0xc20
	ctx.r[9].s64 = ctx.r[11].s64 + -3104;
	// 823B7A1C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7A20: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B7A24: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7A28: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B7A2C: 48000008  b 0x823b7a34
	pc = 0x823B7A34; continue 'dispatch;
	// 823B7A30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7A34: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7A38: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7A3C: 409A0044  bne cr6, 0x823b7a80
	if !ctx.cr[6].eq {
	pc = 0x823B7A80; continue 'dispatch;
	}
	// 823B7A40: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B7A44: 419A001C  beq cr6, 0x823b7a60
	if ctx.cr[6].eq {
	pc = 0x823B7A60; continue 'dispatch;
	}
	// 823B7A48: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7A4C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B7A50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B7A54: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7A58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B7A5C: 4E800421  bctrl
	ctx.lr = 0x823B7A60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7A60: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B7A64: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7A68: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B7A6C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B7A70: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B7A74: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7A78: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B7A7C: 4BF08585  bl 0x822c0000
	ctx.lr = 0x823B7A80;
	sub_822C0000(ctx, base);
	// 823B7A80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B7A84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7A88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B7A8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7A90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B7A94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7AA0 size=196
    let mut pc: u32 = 0x823B7AA0;
    'dispatch: loop {
        match pc {
            0x823B7AA0 => {
    //   block [0x823B7AA0..0x823B7B64)
	// 823B7AA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7AA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7AA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B7AAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7AB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7AB4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7AB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7ABC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B7AC0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B7AC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7AC8: 4BF08E71  bl 0x822c0938
	ctx.lr = 0x823B7ACC;
	sub_822C0938(ctx, base);
	// 823B7ACC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7AD0: 41820028  beq 0x823b7af8
	if ctx.cr[0].eq {
	pc = 0x823B7AF8; continue 'dispatch;
	}
	// 823B7AD4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7AD8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B7ADC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7AE0: 392BF3F4  addi r9, r11, -0xc0c
	ctx.r[9].s64 = ctx.r[11].s64 + -3084;
	// 823B7AE4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7AE8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B7AEC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7AF0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B7AF4: 48000008  b 0x823b7afc
	pc = 0x823B7AFC; continue 'dispatch;
	// 823B7AF8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7AFC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7B00: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7B04: 409A0044  bne cr6, 0x823b7b48
	if !ctx.cr[6].eq {
	pc = 0x823B7B48; continue 'dispatch;
	}
	// 823B7B08: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B7B0C: 419A001C  beq cr6, 0x823b7b28
	if ctx.cr[6].eq {
	pc = 0x823B7B28; continue 'dispatch;
	}
	// 823B7B10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7B14: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B7B18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B7B1C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7B20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B7B24: 4E800421  bctrl
	ctx.lr = 0x823B7B28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7B28: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B7B2C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7B30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B7B34: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B7B38: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B7B3C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7B40: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B7B44: 4BF084BD  bl 0x822c0000
	ctx.lr = 0x823B7B48;
	sub_822C0000(ctx, base);
	// 823B7B48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B7B4C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7B50: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B7B54: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7B58: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B7B5C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7B60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7B68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7B68 size=196
    let mut pc: u32 = 0x823B7B68;
    'dispatch: loop {
        match pc {
            0x823B7B68 => {
    //   block [0x823B7B68..0x823B7C2C)
	// 823B7B68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7B6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7B70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B7B74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7B78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7B7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7B80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7B84: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B7B88: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B7B8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7B90: 4BF08DA9  bl 0x822c0938
	ctx.lr = 0x823B7B94;
	sub_822C0938(ctx, base);
	// 823B7B94: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7B98: 41820028  beq 0x823b7bc0
	if ctx.cr[0].eq {
	pc = 0x823B7BC0; continue 'dispatch;
	}
	// 823B7B9C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7BA0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B7BA4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7BA8: 392BF408  addi r9, r11, -0xbf8
	ctx.r[9].s64 = ctx.r[11].s64 + -3064;
	// 823B7BAC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7BB0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B7BB4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7BB8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B7BBC: 48000008  b 0x823b7bc4
	pc = 0x823B7BC4; continue 'dispatch;
	// 823B7BC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7BC4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7BC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7BCC: 409A0044  bne cr6, 0x823b7c10
	if !ctx.cr[6].eq {
	pc = 0x823B7C10; continue 'dispatch;
	}
	// 823B7BD0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B7BD4: 419A001C  beq cr6, 0x823b7bf0
	if ctx.cr[6].eq {
	pc = 0x823B7BF0; continue 'dispatch;
	}
	// 823B7BD8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7BDC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B7BE0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B7BE4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7BE8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B7BEC: 4E800421  bctrl
	ctx.lr = 0x823B7BF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7BF0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B7BF4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7BF8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B7BFC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B7C00: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B7C04: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7C08: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B7C0C: 4BF083F5  bl 0x822c0000
	ctx.lr = 0x823B7C10;
	sub_822C0000(ctx, base);
	// 823B7C10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B7C14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7C18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B7C1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7C20: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B7C24: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7C28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7C30 size=196
    let mut pc: u32 = 0x823B7C30;
    'dispatch: loop {
        match pc {
            0x823B7C30 => {
    //   block [0x823B7C30..0x823B7CF4)
	// 823B7C30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7C34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7C38: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B7C3C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7C40: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7C44: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7C48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7C4C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B7C50: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B7C54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7C58: 4BF08CE1  bl 0x822c0938
	ctx.lr = 0x823B7C5C;
	sub_822C0938(ctx, base);
	// 823B7C5C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7C60: 41820028  beq 0x823b7c88
	if ctx.cr[0].eq {
	pc = 0x823B7C88; continue 'dispatch;
	}
	// 823B7C64: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7C68: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B7C6C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7C70: 392BF41C  addi r9, r11, -0xbe4
	ctx.r[9].s64 = ctx.r[11].s64 + -3044;
	// 823B7C74: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7C78: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B7C7C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7C80: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B7C84: 48000008  b 0x823b7c8c
	pc = 0x823B7C8C; continue 'dispatch;
	// 823B7C88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7C8C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7C90: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7C94: 409A0044  bne cr6, 0x823b7cd8
	if !ctx.cr[6].eq {
	pc = 0x823B7CD8; continue 'dispatch;
	}
	// 823B7C98: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B7C9C: 419A001C  beq cr6, 0x823b7cb8
	if ctx.cr[6].eq {
	pc = 0x823B7CB8; continue 'dispatch;
	}
	// 823B7CA0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7CA4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B7CA8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B7CAC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7CB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B7CB4: 4E800421  bctrl
	ctx.lr = 0x823B7CB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7CB8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B7CBC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7CC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B7CC4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B7CC8: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B7CCC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7CD0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B7CD4: 4BF0832D  bl 0x822c0000
	ctx.lr = 0x823B7CD8;
	sub_822C0000(ctx, base);
	// 823B7CD8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B7CDC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7CE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B7CE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7CE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B7CEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7CF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7CF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7CF8 size=196
    let mut pc: u32 = 0x823B7CF8;
    'dispatch: loop {
        match pc {
            0x823B7CF8 => {
    //   block [0x823B7CF8..0x823B7DBC)
	// 823B7CF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7CFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7D00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B7D04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7D08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7D0C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7D10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7D14: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B7D18: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B7D1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7D20: 4BF08C19  bl 0x822c0938
	ctx.lr = 0x823B7D24;
	sub_822C0938(ctx, base);
	// 823B7D24: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7D28: 41820028  beq 0x823b7d50
	if ctx.cr[0].eq {
	pc = 0x823B7D50; continue 'dispatch;
	}
	// 823B7D2C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7D30: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B7D34: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7D38: 392BF430  addi r9, r11, -0xbd0
	ctx.r[9].s64 = ctx.r[11].s64 + -3024;
	// 823B7D3C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7D40: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B7D44: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7D48: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B7D4C: 48000008  b 0x823b7d54
	pc = 0x823B7D54; continue 'dispatch;
	// 823B7D50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7D54: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7D58: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7D5C: 409A0044  bne cr6, 0x823b7da0
	if !ctx.cr[6].eq {
	pc = 0x823B7DA0; continue 'dispatch;
	}
	// 823B7D60: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B7D64: 419A001C  beq cr6, 0x823b7d80
	if ctx.cr[6].eq {
	pc = 0x823B7D80; continue 'dispatch;
	}
	// 823B7D68: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7D6C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B7D70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B7D74: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7D78: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B7D7C: 4E800421  bctrl
	ctx.lr = 0x823B7D80;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7D80: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B7D84: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7D88: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B7D8C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B7D90: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B7D94: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7D98: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B7D9C: 4BF08265  bl 0x822c0000
	ctx.lr = 0x823B7DA0;
	sub_822C0000(ctx, base);
	// 823B7DA0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B7DA4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7DA8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B7DAC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7DB0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B7DB4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7DB8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7DC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7DC0 size=196
    let mut pc: u32 = 0x823B7DC0;
    'dispatch: loop {
        match pc {
            0x823B7DC0 => {
    //   block [0x823B7DC0..0x823B7E84)
	// 823B7DC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7DC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7DC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B7DCC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7DD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7DD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7DD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7DDC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B7DE0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B7DE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7DE8: 4BF08B51  bl 0x822c0938
	ctx.lr = 0x823B7DEC;
	sub_822C0938(ctx, base);
	// 823B7DEC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7DF0: 41820028  beq 0x823b7e18
	if ctx.cr[0].eq {
	pc = 0x823B7E18; continue 'dispatch;
	}
	// 823B7DF4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7DF8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B7DFC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7E00: 392BF444  addi r9, r11, -0xbbc
	ctx.r[9].s64 = ctx.r[11].s64 + -3004;
	// 823B7E04: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7E08: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B7E0C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7E10: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B7E14: 48000008  b 0x823b7e1c
	pc = 0x823B7E1C; continue 'dispatch;
	// 823B7E18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7E1C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7E20: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7E24: 409A0044  bne cr6, 0x823b7e68
	if !ctx.cr[6].eq {
	pc = 0x823B7E68; continue 'dispatch;
	}
	// 823B7E28: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B7E2C: 419A001C  beq cr6, 0x823b7e48
	if ctx.cr[6].eq {
	pc = 0x823B7E48; continue 'dispatch;
	}
	// 823B7E30: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7E34: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B7E38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B7E3C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7E40: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B7E44: 4E800421  bctrl
	ctx.lr = 0x823B7E48;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7E48: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B7E4C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7E50: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B7E54: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B7E58: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B7E5C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7E60: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B7E64: 4BF0819D  bl 0x822c0000
	ctx.lr = 0x823B7E68;
	sub_822C0000(ctx, base);
	// 823B7E68: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B7E6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7E70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B7E74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7E78: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B7E7C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7E80: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7E88 size=196
    let mut pc: u32 = 0x823B7E88;
    'dispatch: loop {
        match pc {
            0x823B7E88 => {
    //   block [0x823B7E88..0x823B7F4C)
	// 823B7E88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7E8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7E90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B7E94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7E98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7E9C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7EA0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7EA4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B7EA8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B7EAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7EB0: 4BF08A89  bl 0x822c0938
	ctx.lr = 0x823B7EB4;
	sub_822C0938(ctx, base);
	// 823B7EB4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7EB8: 41820028  beq 0x823b7ee0
	if ctx.cr[0].eq {
	pc = 0x823B7EE0; continue 'dispatch;
	}
	// 823B7EBC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7EC0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B7EC4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7EC8: 392BF458  addi r9, r11, -0xba8
	ctx.r[9].s64 = ctx.r[11].s64 + -2984;
	// 823B7ECC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7ED0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B7ED4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7ED8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B7EDC: 48000008  b 0x823b7ee4
	pc = 0x823B7EE4; continue 'dispatch;
	// 823B7EE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7EE4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7EE8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7EEC: 409A0044  bne cr6, 0x823b7f30
	if !ctx.cr[6].eq {
	pc = 0x823B7F30; continue 'dispatch;
	}
	// 823B7EF0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B7EF4: 419A001C  beq cr6, 0x823b7f10
	if ctx.cr[6].eq {
	pc = 0x823B7F10; continue 'dispatch;
	}
	// 823B7EF8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7EFC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B7F00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B7F04: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7F08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B7F0C: 4E800421  bctrl
	ctx.lr = 0x823B7F10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7F10: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B7F14: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7F18: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B7F1C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B7F20: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B7F24: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7F28: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B7F2C: 4BF080D5  bl 0x822c0000
	ctx.lr = 0x823B7F30;
	sub_822C0000(ctx, base);
	// 823B7F30: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B7F34: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B7F38: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B7F3C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B7F40: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B7F44: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B7F48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B7F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B7F50 size=196
    let mut pc: u32 = 0x823B7F50;
    'dispatch: loop {
        match pc {
            0x823B7F50 => {
    //   block [0x823B7F50..0x823B8014)
	// 823B7F50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B7F54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B7F58: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B7F5C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B7F60: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B7F64: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B7F68: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7F6C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B7F70: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B7F74: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7F78: 4BF089C1  bl 0x822c0938
	ctx.lr = 0x823B7F7C;
	sub_822C0938(ctx, base);
	// 823B7F7C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B7F80: 41820028  beq 0x823b7fa8
	if ctx.cr[0].eq {
	pc = 0x823B7FA8; continue 'dispatch;
	}
	// 823B7F84: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B7F88: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B7F8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B7F90: 392BF46C  addi r9, r11, -0xb94
	ctx.r[9].s64 = ctx.r[11].s64 + -2964;
	// 823B7F94: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B7F98: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B7F9C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B7FA0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B7FA4: 48000008  b 0x823b7fac
	pc = 0x823B7FAC; continue 'dispatch;
	// 823B7FA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B7FAC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B7FB0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B7FB4: 409A0044  bne cr6, 0x823b7ff8
	if !ctx.cr[6].eq {
	pc = 0x823B7FF8; continue 'dispatch;
	}
	// 823B7FB8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B7FBC: 419A001C  beq cr6, 0x823b7fd8
	if ctx.cr[6].eq {
	pc = 0x823B7FD8; continue 'dispatch;
	}
	// 823B7FC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B7FC4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B7FC8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B7FCC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B7FD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B7FD4: 4E800421  bctrl
	ctx.lr = 0x823B7FD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B7FD8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B7FDC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B7FE0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B7FE4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B7FE8: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B7FEC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B7FF0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B7FF4: 4BF0800D  bl 0x822c0000
	ctx.lr = 0x823B7FF8;
	sub_822C0000(ctx, base);
	// 823B7FF8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B7FFC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8000: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8004: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8008: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B800C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8018 size=196
    let mut pc: u32 = 0x823B8018;
    'dispatch: loop {
        match pc {
            0x823B8018 => {
    //   block [0x823B8018..0x823B80DC)
	// 823B8018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B801C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8020: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B8024: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8028: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B802C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8030: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8034: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8038: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B803C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8040: 4BF088F9  bl 0x822c0938
	ctx.lr = 0x823B8044;
	sub_822C0938(ctx, base);
	// 823B8044: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B8048: 41820028  beq 0x823b8070
	if ctx.cr[0].eq {
	pc = 0x823B8070; continue 'dispatch;
	}
	// 823B804C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8050: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B8054: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8058: 392BF480  addi r9, r11, -0xb80
	ctx.r[9].s64 = ctx.r[11].s64 + -2944;
	// 823B805C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B8060: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B8064: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B8068: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B806C: 48000008  b 0x823b8074
	pc = 0x823B8074; continue 'dispatch;
	// 823B8070: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8074: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8078: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B807C: 409A0044  bne cr6, 0x823b80c0
	if !ctx.cr[6].eq {
	pc = 0x823B80C0; continue 'dispatch;
	}
	// 823B8080: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B8084: 419A001C  beq cr6, 0x823b80a0
	if ctx.cr[6].eq {
	pc = 0x823B80A0; continue 'dispatch;
	}
	// 823B8088: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B808C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8090: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8094: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8098: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B809C: 4E800421  bctrl
	ctx.lr = 0x823B80A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B80A0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B80A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B80A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B80AC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B80B0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B80B4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B80B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B80BC: 4BF07F45  bl 0x822c0000
	ctx.lr = 0x823B80C0;
	sub_822C0000(ctx, base);
	// 823B80C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B80C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B80C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B80CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B80D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B80D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B80D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B80E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B80E0 size=196
    let mut pc: u32 = 0x823B80E0;
    'dispatch: loop {
        match pc {
            0x823B80E0 => {
    //   block [0x823B80E0..0x823B81A4)
	// 823B80E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B80E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B80E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B80EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B80F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B80F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B80F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B80FC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8100: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B8104: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8108: 4BF08831  bl 0x822c0938
	ctx.lr = 0x823B810C;
	sub_822C0938(ctx, base);
	// 823B810C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B8110: 41820028  beq 0x823b8138
	if ctx.cr[0].eq {
	pc = 0x823B8138; continue 'dispatch;
	}
	// 823B8114: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8118: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B811C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8120: 392BF494  addi r9, r11, -0xb6c
	ctx.r[9].s64 = ctx.r[11].s64 + -2924;
	// 823B8124: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B8128: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B812C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B8130: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B8134: 48000008  b 0x823b813c
	pc = 0x823B813C; continue 'dispatch;
	// 823B8138: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B813C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8140: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8144: 409A0044  bne cr6, 0x823b8188
	if !ctx.cr[6].eq {
	pc = 0x823B8188; continue 'dispatch;
	}
	// 823B8148: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B814C: 419A001C  beq cr6, 0x823b8168
	if ctx.cr[6].eq {
	pc = 0x823B8168; continue 'dispatch;
	}
	// 823B8150: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8154: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8158: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B815C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8160: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B8164: 4E800421  bctrl
	ctx.lr = 0x823B8168;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B8168: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B816C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8170: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8174: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8178: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B817C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B8180: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B8184: 4BF07E7D  bl 0x822c0000
	ctx.lr = 0x823B8188;
	sub_822C0000(ctx, base);
	// 823B8188: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B818C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8190: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8194: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8198: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B819C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B81A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B81A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B81A8 size=196
    let mut pc: u32 = 0x823B81A8;
    'dispatch: loop {
        match pc {
            0x823B81A8 => {
    //   block [0x823B81A8..0x823B826C)
	// 823B81A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B81AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B81B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B81B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B81B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B81BC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B81C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B81C4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B81C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B81CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B81D0: 4BF08769  bl 0x822c0938
	ctx.lr = 0x823B81D4;
	sub_822C0938(ctx, base);
	// 823B81D4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B81D8: 41820028  beq 0x823b8200
	if ctx.cr[0].eq {
	pc = 0x823B8200; continue 'dispatch;
	}
	// 823B81DC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B81E0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B81E4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B81E8: 392BF4A8  addi r9, r11, -0xb58
	ctx.r[9].s64 = ctx.r[11].s64 + -2904;
	// 823B81EC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B81F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B81F4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B81F8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B81FC: 48000008  b 0x823b8204
	pc = 0x823B8204; continue 'dispatch;
	// 823B8200: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8204: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8208: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B820C: 409A0044  bne cr6, 0x823b8250
	if !ctx.cr[6].eq {
	pc = 0x823B8250; continue 'dispatch;
	}
	// 823B8210: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B8214: 419A001C  beq cr6, 0x823b8230
	if ctx.cr[6].eq {
	pc = 0x823B8230; continue 'dispatch;
	}
	// 823B8218: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B821C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8220: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8224: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8228: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B822C: 4E800421  bctrl
	ctx.lr = 0x823B8230;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B8230: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B8234: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8238: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B823C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8240: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B8244: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B8248: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B824C: 4BF07DB5  bl 0x822c0000
	ctx.lr = 0x823B8250;
	sub_822C0000(ctx, base);
	// 823B8250: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B8254: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8258: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B825C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8260: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B8264: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8268: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8270 size=196
    let mut pc: u32 = 0x823B8270;
    'dispatch: loop {
        match pc {
            0x823B8270 => {
    //   block [0x823B8270..0x823B8334)
	// 823B8270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8278: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B827C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8280: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B8284: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8288: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B828C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8290: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B8294: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8298: 4BF086A1  bl 0x822c0938
	ctx.lr = 0x823B829C;
	sub_822C0938(ctx, base);
	// 823B829C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B82A0: 41820028  beq 0x823b82c8
	if ctx.cr[0].eq {
	pc = 0x823B82C8; continue 'dispatch;
	}
	// 823B82A4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B82A8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B82AC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B82B0: 392BF4BC  addi r9, r11, -0xb44
	ctx.r[9].s64 = ctx.r[11].s64 + -2884;
	// 823B82B4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B82B8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B82BC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B82C0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B82C4: 48000008  b 0x823b82cc
	pc = 0x823B82CC; continue 'dispatch;
	// 823B82C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B82CC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B82D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B82D4: 409A0044  bne cr6, 0x823b8318
	if !ctx.cr[6].eq {
	pc = 0x823B8318; continue 'dispatch;
	}
	// 823B82D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B82DC: 419A001C  beq cr6, 0x823b82f8
	if ctx.cr[6].eq {
	pc = 0x823B82F8; continue 'dispatch;
	}
	// 823B82E0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B82E4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B82E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B82EC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B82F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B82F4: 4E800421  bctrl
	ctx.lr = 0x823B82F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B82F8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B82FC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8300: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8304: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8308: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B830C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B8310: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B8314: 4BF07CED  bl 0x822c0000
	ctx.lr = 0x823B8318;
	sub_822C0000(ctx, base);
	// 823B8318: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B831C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8320: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8324: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8328: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B832C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8330: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8338(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8338 size=196
    let mut pc: u32 = 0x823B8338;
    'dispatch: loop {
        match pc {
            0x823B8338 => {
    //   block [0x823B8338..0x823B83FC)
	// 823B8338: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B833C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8340: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B8344: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8348: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B834C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8350: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8354: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8358: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B835C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8360: 4BF085D9  bl 0x822c0938
	ctx.lr = 0x823B8364;
	sub_822C0938(ctx, base);
	// 823B8364: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B8368: 41820028  beq 0x823b8390
	if ctx.cr[0].eq {
	pc = 0x823B8390; continue 'dispatch;
	}
	// 823B836C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8370: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B8374: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8378: 392BF4D0  addi r9, r11, -0xb30
	ctx.r[9].s64 = ctx.r[11].s64 + -2864;
	// 823B837C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B8380: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B8384: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B8388: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B838C: 48000008  b 0x823b8394
	pc = 0x823B8394; continue 'dispatch;
	// 823B8390: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8394: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8398: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B839C: 409A0044  bne cr6, 0x823b83e0
	if !ctx.cr[6].eq {
	pc = 0x823B83E0; continue 'dispatch;
	}
	// 823B83A0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B83A4: 419A001C  beq cr6, 0x823b83c0
	if ctx.cr[6].eq {
	pc = 0x823B83C0; continue 'dispatch;
	}
	// 823B83A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B83AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B83B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B83B4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B83B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B83BC: 4E800421  bctrl
	ctx.lr = 0x823B83C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B83C0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B83C4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B83C8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B83CC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B83D0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B83D4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B83D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B83DC: 4BF07C25  bl 0x822c0000
	ctx.lr = 0x823B83E0;
	sub_822C0000(ctx, base);
	// 823B83E0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B83E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B83E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B83EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B83F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B83F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B83F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8400 size=196
    let mut pc: u32 = 0x823B8400;
    'dispatch: loop {
        match pc {
            0x823B8400 => {
    //   block [0x823B8400..0x823B84C4)
	// 823B8400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8408: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B840C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8410: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B8414: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8418: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B841C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8420: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B8424: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8428: 4BF08511  bl 0x822c0938
	ctx.lr = 0x823B842C;
	sub_822C0938(ctx, base);
	// 823B842C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B8430: 41820028  beq 0x823b8458
	if ctx.cr[0].eq {
	pc = 0x823B8458; continue 'dispatch;
	}
	// 823B8434: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8438: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B843C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8440: 392BF4E4  addi r9, r11, -0xb1c
	ctx.r[9].s64 = ctx.r[11].s64 + -2844;
	// 823B8444: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B8448: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B844C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B8450: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B8454: 48000008  b 0x823b845c
	pc = 0x823B845C; continue 'dispatch;
	// 823B8458: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B845C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8460: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8464: 409A0044  bne cr6, 0x823b84a8
	if !ctx.cr[6].eq {
	pc = 0x823B84A8; continue 'dispatch;
	}
	// 823B8468: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B846C: 419A001C  beq cr6, 0x823b8488
	if ctx.cr[6].eq {
	pc = 0x823B8488; continue 'dispatch;
	}
	// 823B8470: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8474: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8478: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B847C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8480: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B8484: 4E800421  bctrl
	ctx.lr = 0x823B8488;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B8488: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B848C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8490: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8494: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8498: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B849C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B84A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B84A4: 4BF07B5D  bl 0x822c0000
	ctx.lr = 0x823B84A8;
	sub_822C0000(ctx, base);
	// 823B84A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B84AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B84B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B84B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B84B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B84BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B84C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B84C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B84C8 size=196
    let mut pc: u32 = 0x823B84C8;
    'dispatch: loop {
        match pc {
            0x823B84C8 => {
    //   block [0x823B84C8..0x823B858C)
	// 823B84C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B84CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B84D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B84D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B84D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B84DC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B84E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B84E4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B84E8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B84EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B84F0: 4BF08449  bl 0x822c0938
	ctx.lr = 0x823B84F4;
	sub_822C0938(ctx, base);
	// 823B84F4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B84F8: 41820028  beq 0x823b8520
	if ctx.cr[0].eq {
	pc = 0x823B8520; continue 'dispatch;
	}
	// 823B84FC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8500: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B8504: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8508: 392BF4F8  addi r9, r11, -0xb08
	ctx.r[9].s64 = ctx.r[11].s64 + -2824;
	// 823B850C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B8510: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B8514: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B8518: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B851C: 48000008  b 0x823b8524
	pc = 0x823B8524; continue 'dispatch;
	// 823B8520: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8524: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8528: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B852C: 409A0044  bne cr6, 0x823b8570
	if !ctx.cr[6].eq {
	pc = 0x823B8570; continue 'dispatch;
	}
	// 823B8530: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B8534: 419A001C  beq cr6, 0x823b8550
	if ctx.cr[6].eq {
	pc = 0x823B8550; continue 'dispatch;
	}
	// 823B8538: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B853C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8540: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8544: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8548: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B854C: 4E800421  bctrl
	ctx.lr = 0x823B8550;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B8550: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B8554: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8558: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B855C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8560: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B8564: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B8568: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B856C: 4BF07A95  bl 0x822c0000
	ctx.lr = 0x823B8570;
	sub_822C0000(ctx, base);
	// 823B8570: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B8574: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8578: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B857C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8580: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B8584: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8588: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8590(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8590 size=196
    let mut pc: u32 = 0x823B8590;
    'dispatch: loop {
        match pc {
            0x823B8590 => {
    //   block [0x823B8590..0x823B8654)
	// 823B8590: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8594: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8598: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B859C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B85A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B85A4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B85A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B85AC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B85B0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B85B4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B85B8: 4BF08381  bl 0x822c0938
	ctx.lr = 0x823B85BC;
	sub_822C0938(ctx, base);
	// 823B85BC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B85C0: 41820028  beq 0x823b85e8
	if ctx.cr[0].eq {
	pc = 0x823B85E8; continue 'dispatch;
	}
	// 823B85C4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B85C8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B85CC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B85D0: 392BF50C  addi r9, r11, -0xaf4
	ctx.r[9].s64 = ctx.r[11].s64 + -2804;
	// 823B85D4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B85D8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B85DC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B85E0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B85E4: 48000008  b 0x823b85ec
	pc = 0x823B85EC; continue 'dispatch;
	// 823B85E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B85EC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B85F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B85F4: 409A0044  bne cr6, 0x823b8638
	if !ctx.cr[6].eq {
	pc = 0x823B8638; continue 'dispatch;
	}
	// 823B85F8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B85FC: 419A001C  beq cr6, 0x823b8618
	if ctx.cr[6].eq {
	pc = 0x823B8618; continue 'dispatch;
	}
	// 823B8600: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8604: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8608: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B860C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8610: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B8614: 4E800421  bctrl
	ctx.lr = 0x823B8618;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B8618: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B861C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8620: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8624: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8628: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B862C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B8630: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B8634: 4BF079CD  bl 0x822c0000
	ctx.lr = 0x823B8638;
	sub_822C0000(ctx, base);
	// 823B8638: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B863C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8640: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8644: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8648: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B864C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8650: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8658 size=196
    let mut pc: u32 = 0x823B8658;
    'dispatch: loop {
        match pc {
            0x823B8658 => {
    //   block [0x823B8658..0x823B871C)
	// 823B8658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B865C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8660: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B8664: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8668: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B866C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8670: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8674: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8678: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B867C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8680: 4BF082B9  bl 0x822c0938
	ctx.lr = 0x823B8684;
	sub_822C0938(ctx, base);
	// 823B8684: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B8688: 41820028  beq 0x823b86b0
	if ctx.cr[0].eq {
	pc = 0x823B86B0; continue 'dispatch;
	}
	// 823B868C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8690: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B8694: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8698: 392BF520  addi r9, r11, -0xae0
	ctx.r[9].s64 = ctx.r[11].s64 + -2784;
	// 823B869C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B86A0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B86A4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B86A8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B86AC: 48000008  b 0x823b86b4
	pc = 0x823B86B4; continue 'dispatch;
	// 823B86B0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B86B4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B86B8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B86BC: 409A0044  bne cr6, 0x823b8700
	if !ctx.cr[6].eq {
	pc = 0x823B8700; continue 'dispatch;
	}
	// 823B86C0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B86C4: 419A001C  beq cr6, 0x823b86e0
	if ctx.cr[6].eq {
	pc = 0x823B86E0; continue 'dispatch;
	}
	// 823B86C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B86CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B86D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B86D4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B86D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B86DC: 4E800421  bctrl
	ctx.lr = 0x823B86E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B86E0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B86E4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B86E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B86EC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B86F0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B86F4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B86F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B86FC: 4BF07905  bl 0x822c0000
	ctx.lr = 0x823B8700;
	sub_822C0000(ctx, base);
	// 823B8700: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B8704: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8708: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B870C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8710: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B8714: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8720 size=196
    let mut pc: u32 = 0x823B8720;
    'dispatch: loop {
        match pc {
            0x823B8720 => {
    //   block [0x823B8720..0x823B87E4)
	// 823B8720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8728: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B872C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8730: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B8734: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8738: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B873C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8740: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B8744: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8748: 4BF081F1  bl 0x822c0938
	ctx.lr = 0x823B874C;
	sub_822C0938(ctx, base);
	// 823B874C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B8750: 41820028  beq 0x823b8778
	if ctx.cr[0].eq {
	pc = 0x823B8778; continue 'dispatch;
	}
	// 823B8754: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8758: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B875C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8760: 392BF534  addi r9, r11, -0xacc
	ctx.r[9].s64 = ctx.r[11].s64 + -2764;
	// 823B8764: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B8768: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B876C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B8770: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B8774: 48000008  b 0x823b877c
	pc = 0x823B877C; continue 'dispatch;
	// 823B8778: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B877C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8780: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8784: 409A0044  bne cr6, 0x823b87c8
	if !ctx.cr[6].eq {
	pc = 0x823B87C8; continue 'dispatch;
	}
	// 823B8788: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B878C: 419A001C  beq cr6, 0x823b87a8
	if ctx.cr[6].eq {
	pc = 0x823B87A8; continue 'dispatch;
	}
	// 823B8790: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8794: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8798: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B879C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B87A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B87A4: 4E800421  bctrl
	ctx.lr = 0x823B87A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B87A8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B87AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B87B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B87B4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B87B8: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B87BC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B87C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B87C4: 4BF0783D  bl 0x822c0000
	ctx.lr = 0x823B87C8;
	sub_822C0000(ctx, base);
	// 823B87C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B87CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B87D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B87D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B87D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B87DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B87E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B87E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B87E8 size=196
    let mut pc: u32 = 0x823B87E8;
    'dispatch: loop {
        match pc {
            0x823B87E8 => {
    //   block [0x823B87E8..0x823B88AC)
	// 823B87E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B87EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B87F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B87F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B87F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B87FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8804: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8808: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B880C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8810: 4BF08129  bl 0x822c0938
	ctx.lr = 0x823B8814;
	sub_822C0938(ctx, base);
	// 823B8814: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B8818: 41820028  beq 0x823b8840
	if ctx.cr[0].eq {
	pc = 0x823B8840; continue 'dispatch;
	}
	// 823B881C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8820: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B8824: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8828: 392BF548  addi r9, r11, -0xab8
	ctx.r[9].s64 = ctx.r[11].s64 + -2744;
	// 823B882C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B8830: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B8834: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B8838: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B883C: 48000008  b 0x823b8844
	pc = 0x823B8844; continue 'dispatch;
	// 823B8840: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8844: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8848: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B884C: 409A0044  bne cr6, 0x823b8890
	if !ctx.cr[6].eq {
	pc = 0x823B8890; continue 'dispatch;
	}
	// 823B8850: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B8854: 419A001C  beq cr6, 0x823b8870
	if ctx.cr[6].eq {
	pc = 0x823B8870; continue 'dispatch;
	}
	// 823B8858: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B885C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8864: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8868: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B886C: 4E800421  bctrl
	ctx.lr = 0x823B8870;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B8870: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B8874: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8878: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B887C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8880: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B8884: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B8888: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B888C: 4BF07775  bl 0x822c0000
	ctx.lr = 0x823B8890;
	sub_822C0000(ctx, base);
	// 823B8890: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B8894: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B889C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B88A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B88A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B88A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B88B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B88B0 size=196
    let mut pc: u32 = 0x823B88B0;
    'dispatch: loop {
        match pc {
            0x823B88B0 => {
    //   block [0x823B88B0..0x823B8974)
	// 823B88B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B88B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B88B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B88BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B88C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B88C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B88C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B88CC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B88D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B88D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B88D8: 4BF08061  bl 0x822c0938
	ctx.lr = 0x823B88DC;
	sub_822C0938(ctx, base);
	// 823B88DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B88E0: 41820028  beq 0x823b8908
	if ctx.cr[0].eq {
	pc = 0x823B8908; continue 'dispatch;
	}
	// 823B88E4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B88E8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B88EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B88F0: 392BF55C  addi r9, r11, -0xaa4
	ctx.r[9].s64 = ctx.r[11].s64 + -2724;
	// 823B88F4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B88F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B88FC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B8900: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B8904: 48000008  b 0x823b890c
	pc = 0x823B890C; continue 'dispatch;
	// 823B8908: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B890C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8910: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8914: 409A0044  bne cr6, 0x823b8958
	if !ctx.cr[6].eq {
	pc = 0x823B8958; continue 'dispatch;
	}
	// 823B8918: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B891C: 419A001C  beq cr6, 0x823b8938
	if ctx.cr[6].eq {
	pc = 0x823B8938; continue 'dispatch;
	}
	// 823B8920: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8924: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B892C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8930: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B8934: 4E800421  bctrl
	ctx.lr = 0x823B8938;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B8938: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B893C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8940: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8944: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8948: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B894C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B8950: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B8954: 4BF076AD  bl 0x822c0000
	ctx.lr = 0x823B8958;
	sub_822C0000(ctx, base);
	// 823B8958: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B895C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8960: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8964: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8968: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B896C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8970: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8978(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8978 size=196
    let mut pc: u32 = 0x823B8978;
    'dispatch: loop {
        match pc {
            0x823B8978 => {
    //   block [0x823B8978..0x823B8A3C)
	// 823B8978: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B897C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8980: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B8984: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8988: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B898C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8990: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8994: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8998: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B899C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B89A0: 4BF07F99  bl 0x822c0938
	ctx.lr = 0x823B89A4;
	sub_822C0938(ctx, base);
	// 823B89A4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B89A8: 41820028  beq 0x823b89d0
	if ctx.cr[0].eq {
	pc = 0x823B89D0; continue 'dispatch;
	}
	// 823B89AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B89B0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B89B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B89B8: 392BF570  addi r9, r11, -0xa90
	ctx.r[9].s64 = ctx.r[11].s64 + -2704;
	// 823B89BC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B89C0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B89C4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B89C8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B89CC: 48000008  b 0x823b89d4
	pc = 0x823B89D4; continue 'dispatch;
	// 823B89D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B89D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B89D8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B89DC: 409A0044  bne cr6, 0x823b8a20
	if !ctx.cr[6].eq {
	pc = 0x823B8A20; continue 'dispatch;
	}
	// 823B89E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B89E4: 419A001C  beq cr6, 0x823b8a00
	if ctx.cr[6].eq {
	pc = 0x823B8A00; continue 'dispatch;
	}
	// 823B89E8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B89EC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B89F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B89F4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B89F8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B89FC: 4E800421  bctrl
	ctx.lr = 0x823B8A00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B8A00: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B8A04: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8A08: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8A0C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8A10: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B8A14: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B8A18: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B8A1C: 4BF075E5  bl 0x822c0000
	ctx.lr = 0x823B8A20;
	sub_822C0000(ctx, base);
	// 823B8A20: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B8A24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8A28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8A2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8A30: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B8A34: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8A38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8A40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8A40 size=196
    let mut pc: u32 = 0x823B8A40;
    'dispatch: loop {
        match pc {
            0x823B8A40 => {
    //   block [0x823B8A40..0x823B8B04)
	// 823B8A40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8A44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8A48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B8A4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8A50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B8A54: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8A58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8A5C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8A60: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B8A64: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8A68: 4BF07ED1  bl 0x822c0938
	ctx.lr = 0x823B8A6C;
	sub_822C0938(ctx, base);
	// 823B8A6C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B8A70: 41820028  beq 0x823b8a98
	if ctx.cr[0].eq {
	pc = 0x823B8A98; continue 'dispatch;
	}
	// 823B8A74: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8A78: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B8A7C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8A80: 392BF584  addi r9, r11, -0xa7c
	ctx.r[9].s64 = ctx.r[11].s64 + -2684;
	// 823B8A84: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B8A88: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B8A8C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B8A90: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B8A94: 48000008  b 0x823b8a9c
	pc = 0x823B8A9C; continue 'dispatch;
	// 823B8A98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8A9C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8AA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8AA4: 409A0044  bne cr6, 0x823b8ae8
	if !ctx.cr[6].eq {
	pc = 0x823B8AE8; continue 'dispatch;
	}
	// 823B8AA8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B8AAC: 419A001C  beq cr6, 0x823b8ac8
	if ctx.cr[6].eq {
	pc = 0x823B8AC8; continue 'dispatch;
	}
	// 823B8AB0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8AB4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8AB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8ABC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8AC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B8AC4: 4E800421  bctrl
	ctx.lr = 0x823B8AC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B8AC8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B8ACC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8AD0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8AD4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8AD8: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B8ADC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B8AE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B8AE4: 4BF0751D  bl 0x822c0000
	ctx.lr = 0x823B8AE8;
	sub_822C0000(ctx, base);
	// 823B8AE8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B8AEC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8AF0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8AF4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8AF8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B8AFC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8B00: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8B08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8B08 size=196
    let mut pc: u32 = 0x823B8B08;
    'dispatch: loop {
        match pc {
            0x823B8B08 => {
    //   block [0x823B8B08..0x823B8BCC)
	// 823B8B08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8B0C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8B10: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B8B14: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8B18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B8B1C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8B20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8B24: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8B28: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B8B2C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8B30: 4BF07E09  bl 0x822c0938
	ctx.lr = 0x823B8B34;
	sub_822C0938(ctx, base);
	// 823B8B34: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B8B38: 41820028  beq 0x823b8b60
	if ctx.cr[0].eq {
	pc = 0x823B8B60; continue 'dispatch;
	}
	// 823B8B3C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8B40: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B8B44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8B48: 392BF598  addi r9, r11, -0xa68
	ctx.r[9].s64 = ctx.r[11].s64 + -2664;
	// 823B8B4C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B8B50: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B8B54: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B8B58: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B8B5C: 48000008  b 0x823b8b64
	pc = 0x823B8B64; continue 'dispatch;
	// 823B8B60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8B64: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8B68: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8B6C: 409A0044  bne cr6, 0x823b8bb0
	if !ctx.cr[6].eq {
	pc = 0x823B8BB0; continue 'dispatch;
	}
	// 823B8B70: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B8B74: 419A001C  beq cr6, 0x823b8b90
	if ctx.cr[6].eq {
	pc = 0x823B8B90; continue 'dispatch;
	}
	// 823B8B78: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8B7C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8B80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8B84: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8B88: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B8B8C: 4E800421  bctrl
	ctx.lr = 0x823B8B90;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B8B90: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B8B94: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8B98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8B9C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8BA0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B8BA4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B8BA8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B8BAC: 4BF07455  bl 0x822c0000
	ctx.lr = 0x823B8BB0;
	sub_822C0000(ctx, base);
	// 823B8BB0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B8BB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8BB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8BBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8BC0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B8BC4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8BC8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8BD0 size=196
    let mut pc: u32 = 0x823B8BD0;
    'dispatch: loop {
        match pc {
            0x823B8BD0 => {
    //   block [0x823B8BD0..0x823B8C94)
	// 823B8BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8BD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8BD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B8BDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8BE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B8BE4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8BE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8BEC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8BF0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B8BF4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8BF8: 4BF07D41  bl 0x822c0938
	ctx.lr = 0x823B8BFC;
	sub_822C0938(ctx, base);
	// 823B8BFC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B8C00: 41820028  beq 0x823b8c28
	if ctx.cr[0].eq {
	pc = 0x823B8C28; continue 'dispatch;
	}
	// 823B8C04: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8C08: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B8C0C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8C10: 392BF5AC  addi r9, r11, -0xa54
	ctx.r[9].s64 = ctx.r[11].s64 + -2644;
	// 823B8C14: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B8C18: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B8C1C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B8C20: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B8C24: 48000008  b 0x823b8c2c
	pc = 0x823B8C2C; continue 'dispatch;
	// 823B8C28: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8C2C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8C30: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8C34: 409A0044  bne cr6, 0x823b8c78
	if !ctx.cr[6].eq {
	pc = 0x823B8C78; continue 'dispatch;
	}
	// 823B8C38: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B8C3C: 419A001C  beq cr6, 0x823b8c58
	if ctx.cr[6].eq {
	pc = 0x823B8C58; continue 'dispatch;
	}
	// 823B8C40: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8C44: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8C48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8C4C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8C50: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B8C54: 4E800421  bctrl
	ctx.lr = 0x823B8C58;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B8C58: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B8C5C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8C60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8C64: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8C68: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B8C6C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B8C70: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B8C74: 4BF0738D  bl 0x822c0000
	ctx.lr = 0x823B8C78;
	sub_822C0000(ctx, base);
	// 823B8C78: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B8C7C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8C80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8C84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8C88: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B8C8C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8C90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8C98 size=196
    let mut pc: u32 = 0x823B8C98;
    'dispatch: loop {
        match pc {
            0x823B8C98 => {
    //   block [0x823B8C98..0x823B8D5C)
	// 823B8C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8C9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8CA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B8CA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8CA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B8CAC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8CB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8CB4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8CB8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B8CBC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8CC0: 4BF07C79  bl 0x822c0938
	ctx.lr = 0x823B8CC4;
	sub_822C0938(ctx, base);
	// 823B8CC4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B8CC8: 41820028  beq 0x823b8cf0
	if ctx.cr[0].eq {
	pc = 0x823B8CF0; continue 'dispatch;
	}
	// 823B8CCC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8CD0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B8CD4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8CD8: 392BF5C0  addi r9, r11, -0xa40
	ctx.r[9].s64 = ctx.r[11].s64 + -2624;
	// 823B8CDC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B8CE0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B8CE4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B8CE8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B8CEC: 48000008  b 0x823b8cf4
	pc = 0x823B8CF4; continue 'dispatch;
	// 823B8CF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8CF4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8CF8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8CFC: 409A0044  bne cr6, 0x823b8d40
	if !ctx.cr[6].eq {
	pc = 0x823B8D40; continue 'dispatch;
	}
	// 823B8D00: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B8D04: 419A001C  beq cr6, 0x823b8d20
	if ctx.cr[6].eq {
	pc = 0x823B8D20; continue 'dispatch;
	}
	// 823B8D08: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8D0C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8D10: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8D14: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8D18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B8D1C: 4E800421  bctrl
	ctx.lr = 0x823B8D20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B8D20: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B8D24: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8D28: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8D2C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8D30: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B8D34: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B8D38: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B8D3C: 4BF072C5  bl 0x822c0000
	ctx.lr = 0x823B8D40;
	sub_822C0000(ctx, base);
	// 823B8D40: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B8D44: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8D48: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8D4C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8D50: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B8D54: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8D60 size=196
    let mut pc: u32 = 0x823B8D60;
    'dispatch: loop {
        match pc {
            0x823B8D60 => {
    //   block [0x823B8D60..0x823B8E24)
	// 823B8D60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8D64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8D68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B8D6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8D70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B8D74: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8D78: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8D7C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8D80: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B8D84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8D88: 4BF07BB1  bl 0x822c0938
	ctx.lr = 0x823B8D8C;
	sub_822C0938(ctx, base);
	// 823B8D8C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B8D90: 41820028  beq 0x823b8db8
	if ctx.cr[0].eq {
	pc = 0x823B8DB8; continue 'dispatch;
	}
	// 823B8D94: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8D98: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B8D9C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8DA0: 392BF5D4  addi r9, r11, -0xa2c
	ctx.r[9].s64 = ctx.r[11].s64 + -2604;
	// 823B8DA4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B8DA8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B8DAC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B8DB0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B8DB4: 48000008  b 0x823b8dbc
	pc = 0x823B8DBC; continue 'dispatch;
	// 823B8DB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8DBC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8DC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8DC4: 409A0044  bne cr6, 0x823b8e08
	if !ctx.cr[6].eq {
	pc = 0x823B8E08; continue 'dispatch;
	}
	// 823B8DC8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B8DCC: 419A001C  beq cr6, 0x823b8de8
	if ctx.cr[6].eq {
	pc = 0x823B8DE8; continue 'dispatch;
	}
	// 823B8DD0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8DD4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8DD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8DDC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8DE0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B8DE4: 4E800421  bctrl
	ctx.lr = 0x823B8DE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B8DE8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B8DEC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8DF0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8DF4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8DF8: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B8DFC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B8E00: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B8E04: 4BF071FD  bl 0x822c0000
	ctx.lr = 0x823B8E08;
	sub_822C0000(ctx, base);
	// 823B8E08: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B8E0C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8E10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8E14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8E18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B8E1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8E20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8E28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8E28 size=196
    let mut pc: u32 = 0x823B8E28;
    'dispatch: loop {
        match pc {
            0x823B8E28 => {
    //   block [0x823B8E28..0x823B8EEC)
	// 823B8E28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8E2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8E30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B8E34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8E38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B8E3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8E40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8E44: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8E48: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B8E4C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8E50: 4BF07AE9  bl 0x822c0938
	ctx.lr = 0x823B8E54;
	sub_822C0938(ctx, base);
	// 823B8E54: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B8E58: 41820028  beq 0x823b8e80
	if ctx.cr[0].eq {
	pc = 0x823B8E80; continue 'dispatch;
	}
	// 823B8E5C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8E60: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B8E64: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8E68: 392BF5E8  addi r9, r11, -0xa18
	ctx.r[9].s64 = ctx.r[11].s64 + -2584;
	// 823B8E6C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B8E70: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B8E74: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B8E78: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B8E7C: 48000008  b 0x823b8e84
	pc = 0x823B8E84; continue 'dispatch;
	// 823B8E80: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8E84: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8E88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8E8C: 409A0044  bne cr6, 0x823b8ed0
	if !ctx.cr[6].eq {
	pc = 0x823B8ED0; continue 'dispatch;
	}
	// 823B8E90: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B8E94: 419A001C  beq cr6, 0x823b8eb0
	if ctx.cr[6].eq {
	pc = 0x823B8EB0; continue 'dispatch;
	}
	// 823B8E98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8E9C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8EA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8EA4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8EA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B8EAC: 4E800421  bctrl
	ctx.lr = 0x823B8EB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B8EB0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B8EB4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8EB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8EBC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8EC0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B8EC4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B8EC8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B8ECC: 4BF07135  bl 0x822c0000
	ctx.lr = 0x823B8ED0;
	sub_822C0000(ctx, base);
	// 823B8ED0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B8ED4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8ED8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8EDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8EE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B8EE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8EE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8EF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8EF0 size=196
    let mut pc: u32 = 0x823B8EF0;
    'dispatch: loop {
        match pc {
            0x823B8EF0 => {
    //   block [0x823B8EF0..0x823B8FB4)
	// 823B8EF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8EF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8EF8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B8EFC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8F00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B8F04: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8F08: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8F0C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8F10: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B8F14: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8F18: 4BF07A21  bl 0x822c0938
	ctx.lr = 0x823B8F1C;
	sub_822C0938(ctx, base);
	// 823B8F1C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B8F20: 41820028  beq 0x823b8f48
	if ctx.cr[0].eq {
	pc = 0x823B8F48; continue 'dispatch;
	}
	// 823B8F24: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8F28: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B8F2C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8F30: 392BF5FC  addi r9, r11, -0xa04
	ctx.r[9].s64 = ctx.r[11].s64 + -2564;
	// 823B8F34: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B8F38: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B8F3C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B8F40: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B8F44: 48000008  b 0x823b8f4c
	pc = 0x823B8F4C; continue 'dispatch;
	// 823B8F48: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8F4C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8F50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B8F54: 409A0044  bne cr6, 0x823b8f98
	if !ctx.cr[6].eq {
	pc = 0x823B8F98; continue 'dispatch;
	}
	// 823B8F58: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B8F5C: 419A001C  beq cr6, 0x823b8f78
	if ctx.cr[6].eq {
	pc = 0x823B8F78; continue 'dispatch;
	}
	// 823B8F60: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B8F64: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B8F68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B8F6C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B8F70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B8F74: 4E800421  bctrl
	ctx.lr = 0x823B8F78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B8F78: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B8F7C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B8F80: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B8F84: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B8F88: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B8F8C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B8F90: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B8F94: 4BF0706D  bl 0x822c0000
	ctx.lr = 0x823B8F98;
	sub_822C0000(ctx, base);
	// 823B8F98: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B8F9C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B8FA0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B8FA4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B8FA8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B8FAC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B8FB0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B8FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B8FB8 size=196
    let mut pc: u32 = 0x823B8FB8;
    'dispatch: loop {
        match pc {
            0x823B8FB8 => {
    //   block [0x823B8FB8..0x823B907C)
	// 823B8FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B8FBC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B8FC0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B8FC4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B8FC8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B8FCC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B8FD0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B8FD4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B8FD8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B8FDC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B8FE0: 4BF07959  bl 0x822c0938
	ctx.lr = 0x823B8FE4;
	sub_822C0938(ctx, base);
	// 823B8FE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B8FE8: 41820028  beq 0x823b9010
	if ctx.cr[0].eq {
	pc = 0x823B9010; continue 'dispatch;
	}
	// 823B8FEC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B8FF0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B8FF4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B8FF8: 392BF610  addi r9, r11, -0x9f0
	ctx.r[9].s64 = ctx.r[11].s64 + -2544;
	// 823B8FFC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9000: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9004: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9008: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B900C: 48000008  b 0x823b9014
	pc = 0x823B9014; continue 'dispatch;
	// 823B9010: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9014: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9018: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B901C: 409A0044  bne cr6, 0x823b9060
	if !ctx.cr[6].eq {
	pc = 0x823B9060; continue 'dispatch;
	}
	// 823B9020: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B9024: 419A001C  beq cr6, 0x823b9040
	if ctx.cr[6].eq {
	pc = 0x823B9040; continue 'dispatch;
	}
	// 823B9028: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B902C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9030: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9034: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9038: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B903C: 4E800421  bctrl
	ctx.lr = 0x823B9040;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9040: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B9044: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B9048: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B904C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9050: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B9054: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9058: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B905C: 4BF06FA5  bl 0x822c0000
	ctx.lr = 0x823B9060;
	sub_822C0000(ctx, base);
	// 823B9060: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B9064: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B906C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9070: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B9074: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9078: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9080 size=196
    let mut pc: u32 = 0x823B9080;
    'dispatch: loop {
        match pc {
            0x823B9080 => {
    //   block [0x823B9080..0x823B9144)
	// 823B9080: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9084: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9088: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B908C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9090: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9094: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B9098: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B909C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B90A0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B90A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B90A8: 4BF07891  bl 0x822c0938
	ctx.lr = 0x823B90AC;
	sub_822C0938(ctx, base);
	// 823B90AC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B90B0: 41820028  beq 0x823b90d8
	if ctx.cr[0].eq {
	pc = 0x823B90D8; continue 'dispatch;
	}
	// 823B90B4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B90B8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B90BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B90C0: 392BF624  addi r9, r11, -0x9dc
	ctx.r[9].s64 = ctx.r[11].s64 + -2524;
	// 823B90C4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B90C8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B90CC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B90D0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B90D4: 48000008  b 0x823b90dc
	pc = 0x823B90DC; continue 'dispatch;
	// 823B90D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B90DC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B90E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B90E4: 409A0044  bne cr6, 0x823b9128
	if !ctx.cr[6].eq {
	pc = 0x823B9128; continue 'dispatch;
	}
	// 823B90E8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B90EC: 419A001C  beq cr6, 0x823b9108
	if ctx.cr[6].eq {
	pc = 0x823B9108; continue 'dispatch;
	}
	// 823B90F0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B90F4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B90F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B90FC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9100: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B9104: 4E800421  bctrl
	ctx.lr = 0x823B9108;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9108: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B910C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B9110: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B9114: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9118: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B911C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9120: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B9124: 4BF06EDD  bl 0x822c0000
	ctx.lr = 0x823B9128;
	sub_822C0000(ctx, base);
	// 823B9128: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B912C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9130: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B9134: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9138: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B913C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9140: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9148(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9148 size=196
    let mut pc: u32 = 0x823B9148;
    'dispatch: loop {
        match pc {
            0x823B9148 => {
    //   block [0x823B9148..0x823B920C)
	// 823B9148: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B914C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9150: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B9154: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9158: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B915C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B9160: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9164: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B9168: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B916C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9170: 4BF077C9  bl 0x822c0938
	ctx.lr = 0x823B9174;
	sub_822C0938(ctx, base);
	// 823B9174: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9178: 41820028  beq 0x823b91a0
	if ctx.cr[0].eq {
	pc = 0x823B91A0; continue 'dispatch;
	}
	// 823B917C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B9180: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B9184: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9188: 392BF638  addi r9, r11, -0x9c8
	ctx.r[9].s64 = ctx.r[11].s64 + -2504;
	// 823B918C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9190: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9194: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9198: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B919C: 48000008  b 0x823b91a4
	pc = 0x823B91A4; continue 'dispatch;
	// 823B91A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B91A4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B91A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B91AC: 409A0044  bne cr6, 0x823b91f0
	if !ctx.cr[6].eq {
	pc = 0x823B91F0; continue 'dispatch;
	}
	// 823B91B0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B91B4: 419A001C  beq cr6, 0x823b91d0
	if ctx.cr[6].eq {
	pc = 0x823B91D0; continue 'dispatch;
	}
	// 823B91B8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B91BC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B91C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B91C4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B91C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B91CC: 4E800421  bctrl
	ctx.lr = 0x823B91D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B91D0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B91D4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B91D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B91DC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B91E0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B91E4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B91E8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B91EC: 4BF06E15  bl 0x822c0000
	ctx.lr = 0x823B91F0;
	sub_822C0000(ctx, base);
	// 823B91F0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B91F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B91F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B91FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9200: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B9204: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9208: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9210 size=196
    let mut pc: u32 = 0x823B9210;
    'dispatch: loop {
        match pc {
            0x823B9210 => {
    //   block [0x823B9210..0x823B92D4)
	// 823B9210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9218: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B921C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9220: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9224: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B9228: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B922C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B9230: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B9234: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9238: 4BF07701  bl 0x822c0938
	ctx.lr = 0x823B923C;
	sub_822C0938(ctx, base);
	// 823B923C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9240: 41820028  beq 0x823b9268
	if ctx.cr[0].eq {
	pc = 0x823B9268; continue 'dispatch;
	}
	// 823B9244: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B9248: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B924C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9250: 392BF64C  addi r9, r11, -0x9b4
	ctx.r[9].s64 = ctx.r[11].s64 + -2484;
	// 823B9254: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9258: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B925C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9260: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B9264: 48000008  b 0x823b926c
	pc = 0x823B926C; continue 'dispatch;
	// 823B9268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B926C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9270: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9274: 409A0044  bne cr6, 0x823b92b8
	if !ctx.cr[6].eq {
	pc = 0x823B92B8; continue 'dispatch;
	}
	// 823B9278: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B927C: 419A001C  beq cr6, 0x823b9298
	if ctx.cr[6].eq {
	pc = 0x823B9298; continue 'dispatch;
	}
	// 823B9280: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9284: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B928C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9290: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B9294: 4E800421  bctrl
	ctx.lr = 0x823B9298;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9298: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B929C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B92A0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B92A4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B92A8: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B92AC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B92B0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B92B4: 4BF06D4D  bl 0x822c0000
	ctx.lr = 0x823B92B8;
	sub_822C0000(ctx, base);
	// 823B92B8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B92BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B92C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B92C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B92C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B92CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B92D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B92D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B92D8 size=196
    let mut pc: u32 = 0x823B92D8;
    'dispatch: loop {
        match pc {
            0x823B92D8 => {
    //   block [0x823B92D8..0x823B939C)
	// 823B92D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B92DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B92E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B92E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B92E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B92EC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B92F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B92F4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B92F8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B92FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9300: 4BF07639  bl 0x822c0938
	ctx.lr = 0x823B9304;
	sub_822C0938(ctx, base);
	// 823B9304: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9308: 41820028  beq 0x823b9330
	if ctx.cr[0].eq {
	pc = 0x823B9330; continue 'dispatch;
	}
	// 823B930C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B9310: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B9314: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9318: 392BF660  addi r9, r11, -0x9a0
	ctx.r[9].s64 = ctx.r[11].s64 + -2464;
	// 823B931C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9320: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9324: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9328: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B932C: 48000008  b 0x823b9334
	pc = 0x823B9334; continue 'dispatch;
	// 823B9330: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9334: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9338: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B933C: 409A0044  bne cr6, 0x823b9380
	if !ctx.cr[6].eq {
	pc = 0x823B9380; continue 'dispatch;
	}
	// 823B9340: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B9344: 419A001C  beq cr6, 0x823b9360
	if ctx.cr[6].eq {
	pc = 0x823B9360; continue 'dispatch;
	}
	// 823B9348: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B934C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9350: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9354: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9358: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B935C: 4E800421  bctrl
	ctx.lr = 0x823B9360;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9360: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B9364: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B9368: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B936C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9370: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B9374: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9378: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B937C: 4BF06C85  bl 0x822c0000
	ctx.lr = 0x823B9380;
	sub_822C0000(ctx, base);
	// 823B9380: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B9384: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9388: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B938C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9390: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B9394: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9398: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B93A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B93A0 size=196
    let mut pc: u32 = 0x823B93A0;
    'dispatch: loop {
        match pc {
            0x823B93A0 => {
    //   block [0x823B93A0..0x823B9464)
	// 823B93A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B93A4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B93A8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B93AC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B93B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B93B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B93B8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B93BC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B93C0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B93C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B93C8: 4BF07571  bl 0x822c0938
	ctx.lr = 0x823B93CC;
	sub_822C0938(ctx, base);
	// 823B93CC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B93D0: 41820028  beq 0x823b93f8
	if ctx.cr[0].eq {
	pc = 0x823B93F8; continue 'dispatch;
	}
	// 823B93D4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B93D8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B93DC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B93E0: 392BF674  addi r9, r11, -0x98c
	ctx.r[9].s64 = ctx.r[11].s64 + -2444;
	// 823B93E4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B93E8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B93EC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B93F0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B93F4: 48000008  b 0x823b93fc
	pc = 0x823B93FC; continue 'dispatch;
	// 823B93F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B93FC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9400: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9404: 409A0044  bne cr6, 0x823b9448
	if !ctx.cr[6].eq {
	pc = 0x823B9448; continue 'dispatch;
	}
	// 823B9408: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B940C: 419A001C  beq cr6, 0x823b9428
	if ctx.cr[6].eq {
	pc = 0x823B9428; continue 'dispatch;
	}
	// 823B9410: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9414: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9418: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B941C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9420: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B9424: 4E800421  bctrl
	ctx.lr = 0x823B9428;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9428: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B942C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B9430: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B9434: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9438: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B943C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9440: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B9444: 4BF06BBD  bl 0x822c0000
	ctx.lr = 0x823B9448;
	sub_822C0000(ctx, base);
	// 823B9448: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B944C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9450: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B9454: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9458: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B945C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9468 size=196
    let mut pc: u32 = 0x823B9468;
    'dispatch: loop {
        match pc {
            0x823B9468 => {
    //   block [0x823B9468..0x823B952C)
	// 823B9468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B946C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9470: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B9474: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9478: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B947C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B9480: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9484: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B9488: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B948C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9490: 4BF074A9  bl 0x822c0938
	ctx.lr = 0x823B9494;
	sub_822C0938(ctx, base);
	// 823B9494: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9498: 41820028  beq 0x823b94c0
	if ctx.cr[0].eq {
	pc = 0x823B94C0; continue 'dispatch;
	}
	// 823B949C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B94A0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B94A4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B94A8: 392BF688  addi r9, r11, -0x978
	ctx.r[9].s64 = ctx.r[11].s64 + -2424;
	// 823B94AC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B94B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B94B4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B94B8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B94BC: 48000008  b 0x823b94c4
	pc = 0x823B94C4; continue 'dispatch;
	// 823B94C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B94C4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B94C8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B94CC: 409A0044  bne cr6, 0x823b9510
	if !ctx.cr[6].eq {
	pc = 0x823B9510; continue 'dispatch;
	}
	// 823B94D0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B94D4: 419A001C  beq cr6, 0x823b94f0
	if ctx.cr[6].eq {
	pc = 0x823B94F0; continue 'dispatch;
	}
	// 823B94D8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B94DC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B94E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B94E4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B94E8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B94EC: 4E800421  bctrl
	ctx.lr = 0x823B94F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B94F0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B94F4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B94F8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B94FC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9500: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B9504: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9508: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B950C: 4BF06AF5  bl 0x822c0000
	ctx.lr = 0x823B9510;
	sub_822C0000(ctx, base);
	// 823B9510: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B9514: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9518: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B951C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9520: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B9524: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9528: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9530 size=196
    let mut pc: u32 = 0x823B9530;
    'dispatch: loop {
        match pc {
            0x823B9530 => {
    //   block [0x823B9530..0x823B95F4)
	// 823B9530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9538: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B953C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9540: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9544: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B9548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B954C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B9550: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B9554: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9558: 4BF073E1  bl 0x822c0938
	ctx.lr = 0x823B955C;
	sub_822C0938(ctx, base);
	// 823B955C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9560: 41820028  beq 0x823b9588
	if ctx.cr[0].eq {
	pc = 0x823B9588; continue 'dispatch;
	}
	// 823B9564: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B9568: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B956C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9570: 392BF69C  addi r9, r11, -0x964
	ctx.r[9].s64 = ctx.r[11].s64 + -2404;
	// 823B9574: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9578: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B957C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9580: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B9584: 48000008  b 0x823b958c
	pc = 0x823B958C; continue 'dispatch;
	// 823B9588: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B958C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9590: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9594: 409A0044  bne cr6, 0x823b95d8
	if !ctx.cr[6].eq {
	pc = 0x823B95D8; continue 'dispatch;
	}
	// 823B9598: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B959C: 419A001C  beq cr6, 0x823b95b8
	if ctx.cr[6].eq {
	pc = 0x823B95B8; continue 'dispatch;
	}
	// 823B95A0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B95A4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B95A8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B95AC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B95B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B95B4: 4E800421  bctrl
	ctx.lr = 0x823B95B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B95B8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B95BC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B95C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B95C4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B95C8: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B95CC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B95D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B95D4: 4BF06A2D  bl 0x822c0000
	ctx.lr = 0x823B95D8;
	sub_822C0000(ctx, base);
	// 823B95D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B95DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B95E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B95E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B95E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B95EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B95F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B95F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B95F8 size=196
    let mut pc: u32 = 0x823B95F8;
    'dispatch: loop {
        match pc {
            0x823B95F8 => {
    //   block [0x823B95F8..0x823B96BC)
	// 823B95F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B95FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B9604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9608: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B960C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B9610: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9614: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B9618: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B961C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9620: 4BF07319  bl 0x822c0938
	ctx.lr = 0x823B9624;
	sub_822C0938(ctx, base);
	// 823B9624: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9628: 41820028  beq 0x823b9650
	if ctx.cr[0].eq {
	pc = 0x823B9650; continue 'dispatch;
	}
	// 823B962C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B9630: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B9634: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9638: 392BF6B0  addi r9, r11, -0x950
	ctx.r[9].s64 = ctx.r[11].s64 + -2384;
	// 823B963C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9640: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9644: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9648: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B964C: 48000008  b 0x823b9654
	pc = 0x823B9654; continue 'dispatch;
	// 823B9650: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9654: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9658: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B965C: 409A0044  bne cr6, 0x823b96a0
	if !ctx.cr[6].eq {
	pc = 0x823B96A0; continue 'dispatch;
	}
	// 823B9660: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B9664: 419A001C  beq cr6, 0x823b9680
	if ctx.cr[6].eq {
	pc = 0x823B9680; continue 'dispatch;
	}
	// 823B9668: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B966C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9670: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9674: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9678: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B967C: 4E800421  bctrl
	ctx.lr = 0x823B9680;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9680: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B9684: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B9688: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B968C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9690: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B9694: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9698: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B969C: 4BF06965  bl 0x822c0000
	ctx.lr = 0x823B96A0;
	sub_822C0000(ctx, base);
	// 823B96A0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B96A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B96A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B96AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B96B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B96B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B96B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B96C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B96C0 size=196
    let mut pc: u32 = 0x823B96C0;
    'dispatch: loop {
        match pc {
            0x823B96C0 => {
    //   block [0x823B96C0..0x823B9784)
	// 823B96C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B96C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B96C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B96CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B96D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B96D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B96D8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B96DC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B96E0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B96E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B96E8: 4BF07251  bl 0x822c0938
	ctx.lr = 0x823B96EC;
	sub_822C0938(ctx, base);
	// 823B96EC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B96F0: 41820028  beq 0x823b9718
	if ctx.cr[0].eq {
	pc = 0x823B9718; continue 'dispatch;
	}
	// 823B96F4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B96F8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B96FC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9700: 392BF6C4  addi r9, r11, -0x93c
	ctx.r[9].s64 = ctx.r[11].s64 + -2364;
	// 823B9704: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9708: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B970C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9710: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B9714: 48000008  b 0x823b971c
	pc = 0x823B971C; continue 'dispatch;
	// 823B9718: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B971C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9720: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9724: 409A0044  bne cr6, 0x823b9768
	if !ctx.cr[6].eq {
	pc = 0x823B9768; continue 'dispatch;
	}
	// 823B9728: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B972C: 419A001C  beq cr6, 0x823b9748
	if ctx.cr[6].eq {
	pc = 0x823B9748; continue 'dispatch;
	}
	// 823B9730: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9734: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9738: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B973C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9740: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B9744: 4E800421  bctrl
	ctx.lr = 0x823B9748;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9748: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B974C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B9750: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B9754: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9758: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B975C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9760: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B9764: 4BF0689D  bl 0x822c0000
	ctx.lr = 0x823B9768;
	sub_822C0000(ctx, base);
	// 823B9768: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B976C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B9774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9778: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B977C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9788 size=196
    let mut pc: u32 = 0x823B9788;
    'dispatch: loop {
        match pc {
            0x823B9788 => {
    //   block [0x823B9788..0x823B984C)
	// 823B9788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B978C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9790: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B9794: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9798: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B979C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B97A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B97A4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B97A8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B97AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B97B0: 4BF07189  bl 0x822c0938
	ctx.lr = 0x823B97B4;
	sub_822C0938(ctx, base);
	// 823B97B4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B97B8: 41820028  beq 0x823b97e0
	if ctx.cr[0].eq {
	pc = 0x823B97E0; continue 'dispatch;
	}
	// 823B97BC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B97C0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B97C4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B97C8: 392BF6D8  addi r9, r11, -0x928
	ctx.r[9].s64 = ctx.r[11].s64 + -2344;
	// 823B97CC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B97D0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B97D4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B97D8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B97DC: 48000008  b 0x823b97e4
	pc = 0x823B97E4; continue 'dispatch;
	// 823B97E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B97E4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B97E8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B97EC: 409A0044  bne cr6, 0x823b9830
	if !ctx.cr[6].eq {
	pc = 0x823B9830; continue 'dispatch;
	}
	// 823B97F0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B97F4: 419A001C  beq cr6, 0x823b9810
	if ctx.cr[6].eq {
	pc = 0x823B9810; continue 'dispatch;
	}
	// 823B97F8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B97FC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9800: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9804: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9808: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B980C: 4E800421  bctrl
	ctx.lr = 0x823B9810;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9810: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B9814: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B9818: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B981C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9820: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B9824: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9828: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B982C: 4BF067D5  bl 0x822c0000
	ctx.lr = 0x823B9830;
	sub_822C0000(ctx, base);
	// 823B9830: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B9834: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9838: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B983C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9840: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B9844: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9848: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9850(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9850 size=196
    let mut pc: u32 = 0x823B9850;
    'dispatch: loop {
        match pc {
            0x823B9850 => {
    //   block [0x823B9850..0x823B9914)
	// 823B9850: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9854: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9858: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B985C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9860: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9864: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B9868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B986C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B9870: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B9874: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9878: 4BF070C1  bl 0x822c0938
	ctx.lr = 0x823B987C;
	sub_822C0938(ctx, base);
	// 823B987C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9880: 41820028  beq 0x823b98a8
	if ctx.cr[0].eq {
	pc = 0x823B98A8; continue 'dispatch;
	}
	// 823B9884: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B9888: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B988C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9890: 392BF6EC  addi r9, r11, -0x914
	ctx.r[9].s64 = ctx.r[11].s64 + -2324;
	// 823B9894: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9898: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B989C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B98A0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B98A4: 48000008  b 0x823b98ac
	pc = 0x823B98AC; continue 'dispatch;
	// 823B98A8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B98AC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B98B0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B98B4: 409A0044  bne cr6, 0x823b98f8
	if !ctx.cr[6].eq {
	pc = 0x823B98F8; continue 'dispatch;
	}
	// 823B98B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B98BC: 419A001C  beq cr6, 0x823b98d8
	if ctx.cr[6].eq {
	pc = 0x823B98D8; continue 'dispatch;
	}
	// 823B98C0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B98C4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B98C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B98CC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B98D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B98D4: 4E800421  bctrl
	ctx.lr = 0x823B98D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B98D8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B98DC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B98E0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B98E4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B98E8: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B98EC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B98F0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B98F4: 4BF0670D  bl 0x822c0000
	ctx.lr = 0x823B98F8;
	sub_822C0000(ctx, base);
	// 823B98F8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B98FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9900: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B9904: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9908: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B990C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9910: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9918(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9918 size=196
    let mut pc: u32 = 0x823B9918;
    'dispatch: loop {
        match pc {
            0x823B9918 => {
    //   block [0x823B9918..0x823B99DC)
	// 823B9918: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B991C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9920: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B9924: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9928: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B992C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B9930: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9934: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B9938: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B993C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9940: 4BF06FF9  bl 0x822c0938
	ctx.lr = 0x823B9944;
	sub_822C0938(ctx, base);
	// 823B9944: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9948: 41820028  beq 0x823b9970
	if ctx.cr[0].eq {
	pc = 0x823B9970; continue 'dispatch;
	}
	// 823B994C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B9950: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B9954: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9958: 392BF700  addi r9, r11, -0x900
	ctx.r[9].s64 = ctx.r[11].s64 + -2304;
	// 823B995C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9960: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9964: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9968: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B996C: 48000008  b 0x823b9974
	pc = 0x823B9974; continue 'dispatch;
	// 823B9970: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9974: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9978: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B997C: 409A0044  bne cr6, 0x823b99c0
	if !ctx.cr[6].eq {
	pc = 0x823B99C0; continue 'dispatch;
	}
	// 823B9980: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B9984: 419A001C  beq cr6, 0x823b99a0
	if ctx.cr[6].eq {
	pc = 0x823B99A0; continue 'dispatch;
	}
	// 823B9988: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B998C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9990: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9994: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9998: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B999C: 4E800421  bctrl
	ctx.lr = 0x823B99A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B99A0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B99A4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B99A8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B99AC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B99B0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B99B4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B99B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B99BC: 4BF06645  bl 0x822c0000
	ctx.lr = 0x823B99C0;
	sub_822C0000(ctx, base);
	// 823B99C0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B99C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B99C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B99CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B99D0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B99D4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B99D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B99E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B99E0 size=196
    let mut pc: u32 = 0x823B99E0;
    'dispatch: loop {
        match pc {
            0x823B99E0 => {
    //   block [0x823B99E0..0x823B9AA4)
	// 823B99E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B99E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B99E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B99EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B99F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B99F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B99F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B99FC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B9A00: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B9A04: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9A08: 4BF06F31  bl 0x822c0938
	ctx.lr = 0x823B9A0C;
	sub_822C0938(ctx, base);
	// 823B9A0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9A10: 41820028  beq 0x823b9a38
	if ctx.cr[0].eq {
	pc = 0x823B9A38; continue 'dispatch;
	}
	// 823B9A14: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B9A18: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B9A1C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9A20: 392BF714  addi r9, r11, -0x8ec
	ctx.r[9].s64 = ctx.r[11].s64 + -2284;
	// 823B9A24: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9A28: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9A2C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9A30: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B9A34: 48000008  b 0x823b9a3c
	pc = 0x823B9A3C; continue 'dispatch;
	// 823B9A38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9A3C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9A40: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9A44: 409A0044  bne cr6, 0x823b9a88
	if !ctx.cr[6].eq {
	pc = 0x823B9A88; continue 'dispatch;
	}
	// 823B9A48: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B9A4C: 419A001C  beq cr6, 0x823b9a68
	if ctx.cr[6].eq {
	pc = 0x823B9A68; continue 'dispatch;
	}
	// 823B9A50: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9A54: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9A58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9A5C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9A60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B9A64: 4E800421  bctrl
	ctx.lr = 0x823B9A68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9A68: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B9A6C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B9A70: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B9A74: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9A78: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B9A7C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9A80: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B9A84: 4BF0657D  bl 0x822c0000
	ctx.lr = 0x823B9A88;
	sub_822C0000(ctx, base);
	// 823B9A88: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B9A8C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9A90: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B9A94: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9A98: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B9A9C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9AA8 size=196
    let mut pc: u32 = 0x823B9AA8;
    'dispatch: loop {
        match pc {
            0x823B9AA8 => {
    //   block [0x823B9AA8..0x823B9B6C)
	// 823B9AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9AAC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9AB0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B9AB4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9AB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9ABC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B9AC0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9AC4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B9AC8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B9ACC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9AD0: 4BF06E69  bl 0x822c0938
	ctx.lr = 0x823B9AD4;
	sub_822C0938(ctx, base);
	// 823B9AD4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9AD8: 41820028  beq 0x823b9b00
	if ctx.cr[0].eq {
	pc = 0x823B9B00; continue 'dispatch;
	}
	// 823B9ADC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B9AE0: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B9AE4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9AE8: 392BF728  addi r9, r11, -0x8d8
	ctx.r[9].s64 = ctx.r[11].s64 + -2264;
	// 823B9AEC: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9AF0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9AF4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9AF8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B9AFC: 48000008  b 0x823b9b04
	pc = 0x823B9B04; continue 'dispatch;
	// 823B9B00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9B04: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9B08: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9B0C: 409A0044  bne cr6, 0x823b9b50
	if !ctx.cr[6].eq {
	pc = 0x823B9B50; continue 'dispatch;
	}
	// 823B9B10: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B9B14: 419A001C  beq cr6, 0x823b9b30
	if ctx.cr[6].eq {
	pc = 0x823B9B30; continue 'dispatch;
	}
	// 823B9B18: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9B1C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9B20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9B24: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9B28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B9B2C: 4E800421  bctrl
	ctx.lr = 0x823B9B30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9B30: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B9B34: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B9B38: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B9B3C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9B40: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B9B44: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9B48: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B9B4C: 4BF064B5  bl 0x822c0000
	ctx.lr = 0x823B9B50;
	sub_822C0000(ctx, base);
	// 823B9B50: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B9B54: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9B58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B9B5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9B60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B9B64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9B70 size=196
    let mut pc: u32 = 0x823B9B70;
    'dispatch: loop {
        match pc {
            0x823B9B70 => {
    //   block [0x823B9B70..0x823B9C34)
	// 823B9B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9B78: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B9B7C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9B80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9B84: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B9B88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9B8C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B9B90: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B9B94: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9B98: 4BF06DA1  bl 0x822c0938
	ctx.lr = 0x823B9B9C;
	sub_822C0938(ctx, base);
	// 823B9B9C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9BA0: 41820028  beq 0x823b9bc8
	if ctx.cr[0].eq {
	pc = 0x823B9BC8; continue 'dispatch;
	}
	// 823B9BA4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B9BA8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B9BAC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9BB0: 392BF73C  addi r9, r11, -0x8c4
	ctx.r[9].s64 = ctx.r[11].s64 + -2244;
	// 823B9BB4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9BB8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9BBC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9BC0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B9BC4: 48000008  b 0x823b9bcc
	pc = 0x823B9BCC; continue 'dispatch;
	// 823B9BC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9BCC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9BD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9BD4: 409A0044  bne cr6, 0x823b9c18
	if !ctx.cr[6].eq {
	pc = 0x823B9C18; continue 'dispatch;
	}
	// 823B9BD8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B9BDC: 419A001C  beq cr6, 0x823b9bf8
	if ctx.cr[6].eq {
	pc = 0x823B9BF8; continue 'dispatch;
	}
	// 823B9BE0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9BE4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9BE8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9BEC: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9BF0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B9BF4: 4E800421  bctrl
	ctx.lr = 0x823B9BF8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9BF8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B9BFC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B9C00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B9C04: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9C08: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B9C0C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9C10: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B9C14: 4BF063ED  bl 0x822c0000
	ctx.lr = 0x823B9C18;
	sub_822C0000(ctx, base);
	// 823B9C18: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B9C1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9C20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B9C24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9C28: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B9C2C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9C30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9C38 size=196
    let mut pc: u32 = 0x823B9C38;
    'dispatch: loop {
        match pc {
            0x823B9C38 => {
    //   block [0x823B9C38..0x823B9CFC)
	// 823B9C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9C40: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B9C44: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9C48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9C4C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B9C50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9C54: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B9C58: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B9C5C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9C60: 4BF06CD9  bl 0x822c0938
	ctx.lr = 0x823B9C64;
	sub_822C0938(ctx, base);
	// 823B9C64: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9C68: 41820028  beq 0x823b9c90
	if ctx.cr[0].eq {
	pc = 0x823B9C90; continue 'dispatch;
	}
	// 823B9C6C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B9C70: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B9C74: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9C78: 392BF750  addi r9, r11, -0x8b0
	ctx.r[9].s64 = ctx.r[11].s64 + -2224;
	// 823B9C7C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9C80: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9C84: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9C88: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B9C8C: 48000008  b 0x823b9c94
	pc = 0x823B9C94; continue 'dispatch;
	// 823B9C90: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9C94: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9C98: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9C9C: 409A0044  bne cr6, 0x823b9ce0
	if !ctx.cr[6].eq {
	pc = 0x823B9CE0; continue 'dispatch;
	}
	// 823B9CA0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B9CA4: 419A001C  beq cr6, 0x823b9cc0
	if ctx.cr[6].eq {
	pc = 0x823B9CC0; continue 'dispatch;
	}
	// 823B9CA8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9CAC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9CB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9CB4: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9CB8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B9CBC: 4E800421  bctrl
	ctx.lr = 0x823B9CC0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9CC0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B9CC4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B9CC8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B9CCC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9CD0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B9CD4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9CD8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B9CDC: 4BF06325  bl 0x822c0000
	ctx.lr = 0x823B9CE0;
	sub_822C0000(ctx, base);
	// 823B9CE0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B9CE4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9CE8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B9CEC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9CF0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B9CF4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9CF8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9D00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9D00 size=196
    let mut pc: u32 = 0x823B9D00;
    'dispatch: loop {
        match pc {
            0x823B9D00 => {
    //   block [0x823B9D00..0x823B9DC4)
	// 823B9D00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9D04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9D08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B9D0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9D10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9D14: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B9D18: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9D1C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B9D20: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B9D24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9D28: 4BF06C11  bl 0x822c0938
	ctx.lr = 0x823B9D2C;
	sub_822C0938(ctx, base);
	// 823B9D2C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9D30: 41820028  beq 0x823b9d58
	if ctx.cr[0].eq {
	pc = 0x823B9D58; continue 'dispatch;
	}
	// 823B9D34: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B9D38: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B9D3C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9D40: 392BF764  addi r9, r11, -0x89c
	ctx.r[9].s64 = ctx.r[11].s64 + -2204;
	// 823B9D44: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9D48: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9D4C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9D50: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B9D54: 48000008  b 0x823b9d5c
	pc = 0x823B9D5C; continue 'dispatch;
	// 823B9D58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9D5C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9D60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9D64: 409A0044  bne cr6, 0x823b9da8
	if !ctx.cr[6].eq {
	pc = 0x823B9DA8; continue 'dispatch;
	}
	// 823B9D68: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B9D6C: 419A001C  beq cr6, 0x823b9d88
	if ctx.cr[6].eq {
	pc = 0x823B9D88; continue 'dispatch;
	}
	// 823B9D70: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9D74: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9D78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9D7C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 823B9D80: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B9D84: 4E800421  bctrl
	ctx.lr = 0x823B9D88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9D88: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B9D8C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B9D90: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B9D94: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9D98: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B9D9C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9DA0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B9DA4: 4BF0625D  bl 0x822c0000
	ctx.lr = 0x823B9DA8;
	sub_822C0000(ctx, base);
	// 823B9DA8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B9DAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9DB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B9DB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9DB8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B9DBC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9DC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9DC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9DC8 size=196
    let mut pc: u32 = 0x823B9DC8;
    'dispatch: loop {
        match pc {
            0x823B9DC8 => {
    //   block [0x823B9DC8..0x823B9E8C)
	// 823B9DC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9DCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9DD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B9DD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9DD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9DDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B9DE0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9DE4: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B9DE8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B9DEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9DF0: 4BF06B49  bl 0x822c0938
	ctx.lr = 0x823B9DF4;
	sub_822C0938(ctx, base);
	// 823B9DF4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9DF8: 41820028  beq 0x823b9e20
	if ctx.cr[0].eq {
	pc = 0x823B9E20; continue 'dispatch;
	}
	// 823B9DFC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B9E00: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B9E04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9E08: 392BF778  addi r9, r11, -0x888
	ctx.r[9].s64 = ctx.r[11].s64 + -2184;
	// 823B9E0C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9E10: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9E14: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9E18: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B9E1C: 48000008  b 0x823b9e24
	pc = 0x823B9E24; continue 'dispatch;
	// 823B9E20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9E24: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9E28: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9E2C: 409A0044  bne cr6, 0x823b9e70
	if !ctx.cr[6].eq {
	pc = 0x823B9E70; continue 'dispatch;
	}
	// 823B9E30: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B9E34: 419A001C  beq cr6, 0x823b9e50
	if ctx.cr[6].eq {
	pc = 0x823B9E50; continue 'dispatch;
	}
	// 823B9E38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9E3C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9E40: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9E44: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9E48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B9E4C: 4E800421  bctrl
	ctx.lr = 0x823B9E50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9E50: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B9E54: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B9E58: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B9E5C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9E60: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B9E64: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9E68: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B9E6C: 4BF06195  bl 0x822c0000
	ctx.lr = 0x823B9E70;
	sub_822C0000(ctx, base);
	// 823B9E70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B9E74: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9E78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B9E7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9E80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B9E84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9E88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9E90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9E90 size=196
    let mut pc: u32 = 0x823B9E90;
    'dispatch: loop {
        match pc {
            0x823B9E90 => {
    //   block [0x823B9E90..0x823B9F54)
	// 823B9E90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9E94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9E98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B9E9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9EA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9EA4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B9EA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9EAC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B9EB0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B9EB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9EB8: 4BF06A81  bl 0x822c0938
	ctx.lr = 0x823B9EBC;
	sub_822C0938(ctx, base);
	// 823B9EBC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9EC0: 41820028  beq 0x823b9ee8
	if ctx.cr[0].eq {
	pc = 0x823B9EE8; continue 'dispatch;
	}
	// 823B9EC4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B9EC8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B9ECC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9ED0: 392BF7A0  addi r9, r11, -0x860
	ctx.r[9].s64 = ctx.r[11].s64 + -2144;
	// 823B9ED4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9ED8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9EDC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9EE0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B9EE4: 48000008  b 0x823b9eec
	pc = 0x823B9EEC; continue 'dispatch;
	// 823B9EE8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9EEC: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9EF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9EF4: 409A0044  bne cr6, 0x823b9f38
	if !ctx.cr[6].eq {
	pc = 0x823B9F38; continue 'dispatch;
	}
	// 823B9EF8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B9EFC: 419A001C  beq cr6, 0x823b9f18
	if ctx.cr[6].eq {
	pc = 0x823B9F18; continue 'dispatch;
	}
	// 823B9F00: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9F04: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9F08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9F0C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9F10: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B9F14: 4E800421  bctrl
	ctx.lr = 0x823B9F18;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9F18: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B9F1C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B9F20: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B9F24: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9F28: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B9F2C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9F30: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B9F34: 4BF060CD  bl 0x822c0000
	ctx.lr = 0x823B9F38;
	sub_822C0000(ctx, base);
	// 823B9F38: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823B9F3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823B9F40: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823B9F44: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823B9F48: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823B9F4C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823B9F50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823B9F58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823B9F58 size=196
    let mut pc: u32 = 0x823B9F58;
    'dispatch: loop {
        match pc {
            0x823B9F58 => {
    //   block [0x823B9F58..0x823BA01C)
	// 823B9F58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823B9F5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823B9F60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823B9F64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823B9F68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823B9F6C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823B9F70: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9F74: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823B9F78: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823B9F7C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9F80: 4BF069B9  bl 0x822c0938
	ctx.lr = 0x823B9F84;
	sub_822C0938(ctx, base);
	// 823B9F84: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823B9F88: 41820028  beq 0x823b9fb0
	if ctx.cr[0].eq {
	pc = 0x823B9FB0; continue 'dispatch;
	}
	// 823B9F8C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823B9F90: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823B9F94: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823B9F98: 392BF7B4  addi r9, r11, -0x84c
	ctx.r[9].s64 = ctx.r[11].s64 + -2124;
	// 823B9F9C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823B9FA0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823B9FA4: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823B9FA8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823B9FAC: 48000008  b 0x823b9fb4
	pc = 0x823B9FB4; continue 'dispatch;
	// 823B9FB0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823B9FB4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823B9FB8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823B9FBC: 409A0044  bne cr6, 0x823ba000
	if !ctx.cr[6].eq {
	pc = 0x823BA000; continue 'dispatch;
	}
	// 823B9FC0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823B9FC4: 419A001C  beq cr6, 0x823b9fe0
	if ctx.cr[6].eq {
	pc = 0x823B9FE0; continue 'dispatch;
	}
	// 823B9FC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9FCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823B9FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823B9FD4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823B9FD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823B9FDC: 4E800421  bctrl
	ctx.lr = 0x823B9FE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823B9FE0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823B9FE4: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823B9FE8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823B9FEC: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823B9FF0: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823B9FF4: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823B9FF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823B9FFC: 4BF06005  bl 0x822c0000
	ctx.lr = 0x823BA000;
	sub_822C0000(ctx, base);
	// 823BA000: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BA004: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BA008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA00C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA010: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BA014: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BA018: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA020 size=196
    let mut pc: u32 = 0x823BA020;
    'dispatch: loop {
        match pc {
            0x823BA020 => {
    //   block [0x823BA020..0x823BA0E4)
	// 823BA020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA028: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BA02C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BA030: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA034: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823BA038: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BA03C: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823BA040: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823BA044: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BA048: 4BF068F1  bl 0x822c0938
	ctx.lr = 0x823BA04C;
	sub_822C0938(ctx, base);
	// 823BA04C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BA050: 41820028  beq 0x823ba078
	if ctx.cr[0].eq {
	pc = 0x823BA078; continue 'dispatch;
	}
	// 823BA054: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823BA058: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823BA05C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823BA060: 392BF7C8  addi r9, r11, -0x838
	ctx.r[9].s64 = ctx.r[11].s64 + -2104;
	// 823BA064: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823BA068: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823BA06C: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823BA070: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823BA074: 48000008  b 0x823ba07c
	pc = 0x823BA07C; continue 'dispatch;
	// 823BA078: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BA07C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BA080: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA084: 409A0044  bne cr6, 0x823ba0c8
	if !ctx.cr[6].eq {
	pc = 0x823BA0C8; continue 'dispatch;
	}
	// 823BA088: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823BA08C: 419A001C  beq cr6, 0x823ba0a8
	if ctx.cr[6].eq {
	pc = 0x823BA0A8; continue 'dispatch;
	}
	// 823BA090: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA094: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823BA098: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BA09C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA0A0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823BA0A4: 4E800421  bctrl
	ctx.lr = 0x823BA0A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823BA0A8: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823BA0AC: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823BA0B0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BA0B4: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823BA0B8: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823BA0BC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823BA0C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823BA0C4: 4BF05F3D  bl 0x822c0000
	ctx.lr = 0x823BA0C8;
	sub_822C0000(ctx, base);
	// 823BA0C8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BA0CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BA0D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA0D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA0D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BA0DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BA0E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA0E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA0E8 size=196
    let mut pc: u32 = 0x823BA0E8;
    'dispatch: loop {
        match pc {
            0x823BA0E8 => {
    //   block [0x823BA0E8..0x823BA1AC)
	// 823BA0E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA0EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA0F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BA0F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BA0F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA0FC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823BA100: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BA104: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823BA108: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823BA10C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BA110: 4BF06829  bl 0x822c0938
	ctx.lr = 0x823BA114;
	sub_822C0938(ctx, base);
	// 823BA114: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BA118: 41820028  beq 0x823ba140
	if ctx.cr[0].eq {
	pc = 0x823BA140; continue 'dispatch;
	}
	// 823BA11C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823BA120: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823BA124: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823BA128: 392BF7DC  addi r9, r11, -0x824
	ctx.r[9].s64 = ctx.r[11].s64 + -2084;
	// 823BA12C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823BA130: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823BA134: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823BA138: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823BA13C: 48000008  b 0x823ba144
	pc = 0x823BA144; continue 'dispatch;
	// 823BA140: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BA144: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BA148: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA14C: 409A0044  bne cr6, 0x823ba190
	if !ctx.cr[6].eq {
	pc = 0x823BA190; continue 'dispatch;
	}
	// 823BA150: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823BA154: 419A001C  beq cr6, 0x823ba170
	if ctx.cr[6].eq {
	pc = 0x823BA170; continue 'dispatch;
	}
	// 823BA158: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA15C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823BA160: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BA164: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA168: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823BA16C: 4E800421  bctrl
	ctx.lr = 0x823BA170;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823BA170: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823BA174: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823BA178: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BA17C: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823BA180: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823BA184: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823BA188: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823BA18C: 4BF05E75  bl 0x822c0000
	ctx.lr = 0x823BA190;
	sub_822C0000(ctx, base);
	// 823BA190: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BA194: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BA198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA19C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA1A0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BA1A4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BA1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA1B0 size=196
    let mut pc: u32 = 0x823BA1B0;
    'dispatch: loop {
        match pc {
            0x823BA1B0 => {
    //   block [0x823BA1B0..0x823BA274)
	// 823BA1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA1B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BA1BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BA1C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA1C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823BA1C8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BA1CC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823BA1D0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823BA1D4: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BA1D8: 4BF06761  bl 0x822c0938
	ctx.lr = 0x823BA1DC;
	sub_822C0938(ctx, base);
	// 823BA1DC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BA1E0: 41820028  beq 0x823ba208
	if ctx.cr[0].eq {
	pc = 0x823BA208; continue 'dispatch;
	}
	// 823BA1E4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823BA1E8: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823BA1EC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823BA1F0: 392BF7F0  addi r9, r11, -0x810
	ctx.r[9].s64 = ctx.r[11].s64 + -2064;
	// 823BA1F4: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823BA1F8: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823BA1FC: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823BA200: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823BA204: 48000008  b 0x823ba20c
	pc = 0x823BA20C; continue 'dispatch;
	// 823BA208: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BA20C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BA210: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA214: 409A0044  bne cr6, 0x823ba258
	if !ctx.cr[6].eq {
	pc = 0x823BA258; continue 'dispatch;
	}
	// 823BA218: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823BA21C: 419A001C  beq cr6, 0x823ba238
	if ctx.cr[6].eq {
	pc = 0x823BA238; continue 'dispatch;
	}
	// 823BA220: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA224: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823BA228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BA22C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA230: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823BA234: 4E800421  bctrl
	ctx.lr = 0x823BA238;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823BA238: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823BA23C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823BA240: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BA244: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823BA248: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823BA24C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823BA250: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823BA254: 4BF05DAD  bl 0x822c0000
	ctx.lr = 0x823BA258;
	sub_822C0000(ctx, base);
	// 823BA258: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BA25C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BA260: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA264: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA268: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BA26C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BA270: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA278 size=548
    let mut pc: u32 = 0x823BA278;
    'dispatch: loop {
        match pc {
            0x823BA278 => {
    //   block [0x823BA278..0x823BA49C)
	// 823BA278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA27C: 48DEDEF1  bl 0x831a816c
	ctx.lr = 0x823BA280;
	sub_831A8130(ctx, base);
	// 823BA280: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA284: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BA288: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823BA28C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BA290: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 823BA294: 396BF85C  addi r11, r11, -0x7a4
	ctx.r[11].s64 = ctx.r[11].s64 + -1956;
	// 823BA298: 394AF844  addi r10, r10, -0x7bc
	ctx.r[10].s64 = ctx.r[10].s64 + -1980;
	// 823BA29C: 3929F834  addi r9, r9, -0x7cc
	ctx.r[9].s64 = ctx.r[9].s64 + -1996;
	// 823BA2A0: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BA2A4: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 823BA2A8: 913F00C0  stw r9, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[9].u32 ) };
	// 823BA2AC: 807F03FC  lwz r3, 0x3fc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1020 as u32) ) } as u64;
	// 823BA2B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA2B4: 419A0008  beq cr6, 0x823ba2bc
	if ctx.cr[6].eq {
	pc = 0x823BA2BC; continue 'dispatch;
	}
	// 823BA2B8: 4BF065D9  bl 0x822c0890
	ctx.lr = 0x823BA2BC;
	sub_822C0890(ctx, base);
	// 823BA2BC: 807F03F4  lwz r3, 0x3f4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1012 as u32) ) } as u64;
	// 823BA2C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA2C4: 419A0008  beq cr6, 0x823ba2cc
	if ctx.cr[6].eq {
	pc = 0x823BA2CC; continue 'dispatch;
	}
	// 823BA2C8: 4BF065C9  bl 0x822c0890
	ctx.lr = 0x823BA2CC;
	sub_822C0890(ctx, base);
	// 823BA2CC: 807F03EC  lwz r3, 0x3ec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1004 as u32) ) } as u64;
	// 823BA2D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA2D4: 419A0008  beq cr6, 0x823ba2dc
	if ctx.cr[6].eq {
	pc = 0x823BA2DC; continue 'dispatch;
	}
	// 823BA2D8: 4BF065B9  bl 0x822c0890
	ctx.lr = 0x823BA2DC;
	sub_822C0890(ctx, base);
	// 823BA2DC: 807F03E0  lwz r3, 0x3e0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(992 as u32) ) } as u64;
	// 823BA2E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA2E4: 419A0008  beq cr6, 0x823ba2ec
	if ctx.cr[6].eq {
	pc = 0x823BA2EC; continue 'dispatch;
	}
	// 823BA2E8: 4BF065A9  bl 0x822c0890
	ctx.lr = 0x823BA2EC;
	sub_822C0890(ctx, base);
	// 823BA2EC: 807F03D8  lwz r3, 0x3d8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(984 as u32) ) } as u64;
	// 823BA2F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA2F4: 419A0008  beq cr6, 0x823ba2fc
	if ctx.cr[6].eq {
	pc = 0x823BA2FC; continue 'dispatch;
	}
	// 823BA2F8: 4BF06599  bl 0x822c0890
	ctx.lr = 0x823BA2FC;
	sub_822C0890(ctx, base);
	// 823BA2FC: 807F03CC  lwz r3, 0x3cc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(972 as u32) ) } as u64;
	// 823BA300: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA304: 419A0008  beq cr6, 0x823ba30c
	if ctx.cr[6].eq {
	pc = 0x823BA30C; continue 'dispatch;
	}
	// 823BA308: 4BF06589  bl 0x822c0890
	ctx.lr = 0x823BA30C;
	sub_822C0890(ctx, base);
	// 823BA30C: 807F03C4  lwz r3, 0x3c4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(964 as u32) ) } as u64;
	// 823BA310: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA314: 419A0008  beq cr6, 0x823ba31c
	if ctx.cr[6].eq {
	pc = 0x823BA31C; continue 'dispatch;
	}
	// 823BA318: 4BF06579  bl 0x822c0890
	ctx.lr = 0x823BA31C;
	sub_822C0890(ctx, base);
	// 823BA31C: 807F03BC  lwz r3, 0x3bc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(956 as u32) ) } as u64;
	// 823BA320: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA324: 419A0008  beq cr6, 0x823ba32c
	if ctx.cr[6].eq {
	pc = 0x823BA32C; continue 'dispatch;
	}
	// 823BA328: 4BF06569  bl 0x822c0890
	ctx.lr = 0x823BA32C;
	sub_822C0890(ctx, base);
	// 823BA32C: 807F03B4  lwz r3, 0x3b4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(948 as u32) ) } as u64;
	// 823BA330: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA334: 419A0008  beq cr6, 0x823ba33c
	if ctx.cr[6].eq {
	pc = 0x823BA33C; continue 'dispatch;
	}
	// 823BA338: 4BF06559  bl 0x822c0890
	ctx.lr = 0x823BA33C;
	sub_822C0890(ctx, base);
	// 823BA33C: 807F03AC  lwz r3, 0x3ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(940 as u32) ) } as u64;
	// 823BA340: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA344: 419A0008  beq cr6, 0x823ba34c
	if ctx.cr[6].eq {
	pc = 0x823BA34C; continue 'dispatch;
	}
	// 823BA348: 4BF06549  bl 0x822c0890
	ctx.lr = 0x823BA34C;
	sub_822C0890(ctx, base);
	// 823BA34C: 397F03A8  addi r11, r31, 0x3a8
	ctx.r[11].s64 = ctx.r[31].s64 + 936;
	// 823BA350: 3BA0000B  li r29, 0xb
	ctx.r[29].s64 = 11;
	// 823BA354: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 823BA358: 3BDEFFF8  addi r30, r30, -8
	ctx.r[30].s64 = ctx.r[30].s64 + -8;
	// 823BA35C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA360: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA364: 419A0008  beq cr6, 0x823ba36c
	if ctx.cr[6].eq {
	pc = 0x823BA36C; continue 'dispatch;
	}
	// 823BA368: 4BF06529  bl 0x822c0890
	ctx.lr = 0x823BA36C;
	sub_822C0890(ctx, base);
	// 823BA36C: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 823BA370: 4080FFE8  bge 0x823ba358
	if !ctx.cr[0].lt {
	pc = 0x823BA358; continue 'dispatch;
	}
	// 823BA374: 397F0348  addi r11, r31, 0x348
	ctx.r[11].s64 = ctx.r[31].s64 + 840;
	// 823BA378: 3BA0000B  li r29, 0xb
	ctx.r[29].s64 = 11;
	// 823BA37C: 3BCB0004  addi r30, r11, 4
	ctx.r[30].s64 = ctx.r[11].s64 + 4;
	// 823BA380: 3BDEFFF8  addi r30, r30, -8
	ctx.r[30].s64 = ctx.r[30].s64 + -8;
	// 823BA384: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA388: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA38C: 419A0008  beq cr6, 0x823ba394
	if ctx.cr[6].eq {
	pc = 0x823BA394; continue 'dispatch;
	}
	// 823BA390: 4BF06501  bl 0x822c0890
	ctx.lr = 0x823BA394;
	sub_822C0890(ctx, base);
	// 823BA394: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 823BA398: 4080FFE8  bge 0x823ba380
	if !ctx.cr[0].lt {
	pc = 0x823BA380; continue 'dispatch;
	}
	// 823BA39C: 807F02E4  lwz r3, 0x2e4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(740 as u32) ) } as u64;
	// 823BA3A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA3A4: 419A0008  beq cr6, 0x823ba3ac
	if ctx.cr[6].eq {
	pc = 0x823BA3AC; continue 'dispatch;
	}
	// 823BA3A8: 4BF064E9  bl 0x822c0890
	ctx.lr = 0x823BA3AC;
	sub_822C0890(ctx, base);
	// 823BA3AC: 807F02DC  lwz r3, 0x2dc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(732 as u32) ) } as u64;
	// 823BA3B0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA3B4: 419A0008  beq cr6, 0x823ba3bc
	if ctx.cr[6].eq {
	pc = 0x823BA3BC; continue 'dispatch;
	}
	// 823BA3B8: 4BF064D9  bl 0x822c0890
	ctx.lr = 0x823BA3BC;
	sub_822C0890(ctx, base);
	// 823BA3BC: 807F02D4  lwz r3, 0x2d4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(724 as u32) ) } as u64;
	// 823BA3C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA3C4: 419A0008  beq cr6, 0x823ba3cc
	if ctx.cr[6].eq {
	pc = 0x823BA3CC; continue 'dispatch;
	}
	// 823BA3C8: 4BF064C9  bl 0x822c0890
	ctx.lr = 0x823BA3CC;
	sub_822C0890(ctx, base);
	// 823BA3CC: 807F02CC  lwz r3, 0x2cc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(716 as u32) ) } as u64;
	// 823BA3D0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA3D4: 419A0008  beq cr6, 0x823ba3dc
	if ctx.cr[6].eq {
	pc = 0x823BA3DC; continue 'dispatch;
	}
	// 823BA3D8: 4BF064B9  bl 0x822c0890
	ctx.lr = 0x823BA3DC;
	sub_822C0890(ctx, base);
	// 823BA3DC: 807F02C4  lwz r3, 0x2c4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(708 as u32) ) } as u64;
	// 823BA3E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA3E4: 419A0008  beq cr6, 0x823ba3ec
	if ctx.cr[6].eq {
	pc = 0x823BA3EC; continue 'dispatch;
	}
	// 823BA3E8: 4BF064A9  bl 0x822c0890
	ctx.lr = 0x823BA3EC;
	sub_822C0890(ctx, base);
	// 823BA3EC: 807F02BC  lwz r3, 0x2bc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(700 as u32) ) } as u64;
	// 823BA3F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA3F4: 419A0008  beq cr6, 0x823ba3fc
	if ctx.cr[6].eq {
	pc = 0x823BA3FC; continue 'dispatch;
	}
	// 823BA3F8: 4BF06499  bl 0x822c0890
	ctx.lr = 0x823BA3FC;
	sub_822C0890(ctx, base);
	// 823BA3FC: 807F02B4  lwz r3, 0x2b4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(692 as u32) ) } as u64;
	// 823BA400: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA404: 419A0008  beq cr6, 0x823ba40c
	if ctx.cr[6].eq {
	pc = 0x823BA40C; continue 'dispatch;
	}
	// 823BA408: 4BF06489  bl 0x822c0890
	ctx.lr = 0x823BA40C;
	sub_822C0890(ctx, base);
	// 823BA40C: 807F02AC  lwz r3, 0x2ac(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(684 as u32) ) } as u64;
	// 823BA410: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA414: 419A0008  beq cr6, 0x823ba41c
	if ctx.cr[6].eq {
	pc = 0x823BA41C; continue 'dispatch;
	}
	// 823BA418: 4BF06479  bl 0x822c0890
	ctx.lr = 0x823BA41C;
	sub_822C0890(ctx, base);
	// 823BA41C: 807F02A4  lwz r3, 0x2a4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(676 as u32) ) } as u64;
	// 823BA420: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA424: 419A0008  beq cr6, 0x823ba42c
	if ctx.cr[6].eq {
	pc = 0x823BA42C; continue 'dispatch;
	}
	// 823BA428: 4BF06469  bl 0x822c0890
	ctx.lr = 0x823BA42C;
	sub_822C0890(ctx, base);
	// 823BA42C: 807F029C  lwz r3, 0x29c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(668 as u32) ) } as u64;
	// 823BA430: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA434: 419A0008  beq cr6, 0x823ba43c
	if ctx.cr[6].eq {
	pc = 0x823BA43C; continue 'dispatch;
	}
	// 823BA438: 4BF06459  bl 0x822c0890
	ctx.lr = 0x823BA43C;
	sub_822C0890(ctx, base);
	// 823BA43C: 807F0294  lwz r3, 0x294(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(660 as u32) ) } as u64;
	// 823BA440: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA444: 419A0008  beq cr6, 0x823ba44c
	if ctx.cr[6].eq {
	pc = 0x823BA44C; continue 'dispatch;
	}
	// 823BA448: 4BF06449  bl 0x822c0890
	ctx.lr = 0x823BA44C;
	sub_822C0890(ctx, base);
	// 823BA44C: 807F028C  lwz r3, 0x28c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(652 as u32) ) } as u64;
	// 823BA450: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA454: 419A0008  beq cr6, 0x823ba45c
	if ctx.cr[6].eq {
	pc = 0x823BA45C; continue 'dispatch;
	}
	// 823BA458: 4BF06439  bl 0x822c0890
	ctx.lr = 0x823BA45C;
	sub_822C0890(ctx, base);
	// 823BA45C: 807F0284  lwz r3, 0x284(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(644 as u32) ) } as u64;
	// 823BA460: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA464: 419A0008  beq cr6, 0x823ba46c
	if ctx.cr[6].eq {
	pc = 0x823BA46C; continue 'dispatch;
	}
	// 823BA468: 4BF06429  bl 0x822c0890
	ctx.lr = 0x823BA46C;
	sub_822C0890(ctx, base);
	// 823BA46C: 807F027C  lwz r3, 0x27c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(636 as u32) ) } as u64;
	// 823BA470: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA474: 419A0008  beq cr6, 0x823ba47c
	if ctx.cr[6].eq {
	pc = 0x823BA47C; continue 'dispatch;
	}
	// 823BA478: 4BF06419  bl 0x822c0890
	ctx.lr = 0x823BA47C;
	sub_822C0890(ctx, base);
	// 823BA47C: 807F0274  lwz r3, 0x274(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(628 as u32) ) } as u64;
	// 823BA480: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BA484: 419A0008  beq cr6, 0x823ba48c
	if ctx.cr[6].eq {
	pc = 0x823BA48C; continue 'dispatch;
	}
	// 823BA488: 4BF06409  bl 0x822c0890
	ctx.lr = 0x823BA48C;
	sub_822C0890(ctx, base);
	// 823BA48C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BA490: 4BF496C1  bl 0x82303b50
	ctx.lr = 0x823BA494;
	sub_82303B50(ctx, base);
	// 823BA494: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BA498: 48DEDD24  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA4A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BA4A0 size=8
    let mut pc: u32 = 0x823BA4A0;
    'dispatch: loop {
        match pc {
            0x823BA4A0 => {
    //   block [0x823BA4A0..0x823BA4A8)
	// 823BA4A0: 3863FFD8  addi r3, r3, -0x28
	ctx.r[3].s64 = ctx.r[3].s64 + -40;
	// 823BA4A4: 48001264  b 0x823bb708
	sub_823BB708(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA4A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BA4A8 size=280
    let mut pc: u32 = 0x823BA4A8;
    'dispatch: loop {
        match pc {
            0x823BA4A8 => {
    //   block [0x823BA4A8..0x823BA5C0)
	// 823BA4A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA4AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA4B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BA4B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BA4B8: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BA5C0 size=284
    let mut pc: u32 = 0x823BA5C0;
    'dispatch: loop {
        match pc {
            0x823BA5C0 => {
    //   block [0x823BA5C0..0x823BA6DC)
	// 823BA5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA5C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BA5CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BA5D0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 823BA5D4: 3980FFD0  li r12, -0x30
	ctx.r[12].s64 = -48;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA6E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823BA6E0 size=360
    let mut pc: u32 = 0x823BA6E0;
    'dispatch: loop {
        match pc {
            0x823BA6E0 => {
    //   block [0x823BA6E0..0x823BA848)
	// 823BA6E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA6E4: 48DEDA81  bl 0x831a8164
	ctx.lr = 0x823BA6E8;
	sub_831A8130(ctx, base);
	// 823BA6E8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA6EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BA6F0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823BA6F4: 481CC7BD  bl 0x82586eb0
	ctx.lr = 0x823BA6F8;
	sub_82586EB0(ctx, base);
	// 823BA6F8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823BA6FC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823BA700: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823BA704: 48154C2D  bl 0x8250f330
	ctx.lr = 0x823BA708;
	sub_8250F330(ctx, base);
	// 823BA708: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823BA70C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BA710: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA714: 48130BAD  bl 0x824eb2c0
	ctx.lr = 0x823BA718;
	sub_824EB2C0(ctx, base);
	// 823BA718: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823BA71C: 83830000  lwz r28, 0(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA720: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA724: 419A000C  beq cr6, 0x823ba730
	if ctx.cr[6].eq {
	pc = 0x823BA730; continue 'dispatch;
	}
	// 823BA728: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823BA72C: 4BF06165  bl 0x822c0890
	ctx.lr = 0x823BA730;
	sub_822C0890(ctx, base);
	// 823BA730: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823BA734: 48A3755D  bl 0x82df1c90
	ctx.lr = 0x823BA738;
	sub_82DF1C90(ctx, base);
	// 823BA738: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 823BA73C: 4815C585  bl 0x82516cc0
	ctx.lr = 0x823BA740;
	sub_82516CC0(ctx, base);
	// 823BA740: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823BA744: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823BA748: 48154D81  bl 0x8250f4c8
	ctx.lr = 0x823BA74C;
	sub_8250F4C8(ctx, base);
	// 823BA74C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA750: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 823BA754: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA758: 386BFFFC  addi r3, r11, -4
	ctx.r[3].s64 = ctx.r[11].s64 + -4;
	// 823BA75C: 409A0008  bne cr6, 0x823ba764
	if !ctx.cr[6].eq {
	pc = 0x823BA764; continue 'dispatch;
	}
	// 823BA760: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 823BA764: 4814DF75  bl 0x825086d8
	ctx.lr = 0x823BA768;
	sub_825086D8(ctx, base);
	// 823BA768: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BA76C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823BA770: 48A37521  bl 0x82df1c90
	ctx.lr = 0x823BA774;
	sub_82DF1C90(ctx, base);
	// 823BA774: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BA778: 3B9F0018  addi r28, r31, 0x18
	ctx.r[28].s64 = ctx.r[31].s64 + 24;
	// 823BA77C: 4BF4B72D  bl 0x82305ea8
	ctx.lr = 0x823BA780;
	sub_82305EA8(ctx, base);
	// 823BA780: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BA784: 480A1BB5  bl 0x8245c338
	ctx.lr = 0x823BA788;
	sub_8245C338(ctx, base);
	// 823BA788: 907F0018  stw r3, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[3].u32 ) };
	// 823BA78C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BA790: 4BFE6D71  bl 0x823a1500
	ctx.lr = 0x823BA794;
	sub_823A1500(ctx, base);
	// 823BA794: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823BA798: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BA79C: 480A18CD  bl 0x8245c068
	ctx.lr = 0x823BA7A0;
	sub_8245C068(ctx, base);
	// 823BA7A0: 907F001C  stw r3, 0x1c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(28 as u32), ctx.r[3].u32 ) };
	// 823BA7A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BA7A8: 480A1A01  bl 0x8245c1a8
	ctx.lr = 0x823BA7AC;
	sub_8245C1A8(ctx, base);
	// 823BA7AC: 907F0024  stw r3, 0x24(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(36 as u32), ctx.r[3].u32 ) };
	// 823BA7B0: 817E0678  lwz r11, 0x678(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1656 as u32) ) } as u64;
	// 823BA7B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BA7B8: 917F002C  stw r11, 0x2c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(44 as u32), ctx.r[11].u32 ) };
	// 823BA7BC: 817E0674  lwz r11, 0x674(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(1652 as u32) ) } as u64;
	// 823BA7C0: 917F0030  stw r11, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[11].u32 ) };
	// 823BA7C4: 4BF4B6E5  bl 0x82305ea8
	ctx.lr = 0x823BA7C8;
	sub_82305EA8(ctx, base);
	// 823BA7C8: D03F0034  stfs f1, 0x34(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(52 as u32), tmp.u32 ) };
	// 823BA7CC: 937F0038  stw r27, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[27].u32 ) };
	// 823BA7D0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 823BA7D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BA7D8: 480A2979  bl 0x8245d150
	ctx.lr = 0x823BA7DC;
	sub_8245D150(ctx, base);
	// 823BA7DC: 546B063F  clrlwi. r11, r3, 0x18
	ctx.r[11].u64 = ctx.r[3].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BA7E0: 41820014  beq 0x823ba7f4
	if ctx.cr[0].eq {
	pc = 0x823BA7F4; continue 'dispatch;
	}
	// 823BA7E4: 815F0038  lwz r10, 0x38(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 823BA7E8: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA7EC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 823BA7F0: 917F0038  stw r11, 0x38(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(56 as u32), ctx.r[11].u32 ) };
	// 823BA7F4: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 823BA7F8: 3B9C0004  addi r28, r28, 4
	ctx.r[28].s64 = ctx.r[28].s64 + 4;
	// 823BA7FC: 2F1B0007  cmpwi cr6, r27, 7
	ctx.cr[6].compare_i32(ctx.r[27].s32, 7, &mut ctx.xer);
	// 823BA800: 4198FFD0  blt cr6, 0x823ba7d0
	if ctx.cr[6].lt {
	pc = 0x823BA7D0; continue 'dispatch;
	}
	// 823BA804: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 823BA808: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 823BA80C: 480A069D  bl 0x8245aea8
	ctx.lr = 0x823BA810;
	sub_8245AEA8(ctx, base);
	// 823BA810: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BA814: 40820018  bne 0x823ba82c
	if !ctx.cr[0].eq {
	pc = 0x823BA82C; continue 'dispatch;
	}
	// 823BA818: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BA81C: 809F0038  lwz r4, 0x38(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) } as u64;
	// 823BA820: 480A1A81  bl 0x8245c2a0
	ctx.lr = 0x823BA824;
	sub_8245C2A0(ctx, base);
	// 823BA824: 907F003C  stw r3, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[3].u32 ) };
	// 823BA828: 48000018  b 0x823ba840
	pc = 0x823BA840; continue 'dispatch;
	// 823BA82C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 823BA830: 807E01FC  lwz r3, 0x1fc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(508 as u32) ) } as u64;
	// 823BA834: 480A0675  bl 0x8245aea8
	ctx.lr = 0x823BA838;
	sub_8245AEA8(ctx, base);
	// 823BA838: 3963FFFF  addi r11, r3, -1
	ctx.r[11].s64 = ctx.r[3].s64 + -1;
	// 823BA83C: 917F003C  stw r11, 0x3c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(60 as u32), ctx.r[11].u32 ) };
	// 823BA840: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 823BA844: 48DED970  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA848 size=164
    let mut pc: u32 = 0x823BA848;
    'dispatch: loop {
        match pc {
            0x823BA848 => {
    //   block [0x823BA848..0x823BA8EC)
	// 823BA848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA84C: 48DED919  bl 0x831a8164
	ctx.lr = 0x823BA850;
	sub_831A8130(ctx, base);
	// 823BA850: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA854: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BA858: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 823BA85C: 481CC655  bl 0x82586eb0
	ctx.lr = 0x823BA860;
	sub_82586EB0(ctx, base);
	// 823BA860: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 823BA864: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BA868: 481CC649  bl 0x82586eb0
	ctx.lr = 0x823BA86C;
	sub_82586EB0(ctx, base);
	// 823BA86C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 823BA870: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BA874: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BA878: 48154C51  bl 0x8250f4c8
	ctx.lr = 0x823BA87C;
	sub_8250F4C8(ctx, base);
	// 823BA87C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BA880: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BA884: 3BCBFFFC  addi r30, r11, -4
	ctx.r[30].s64 = ctx.r[11].s64 + -4;
	// 823BA888: 409A0008  bne cr6, 0x823ba890
	if !ctx.cr[6].eq {
	pc = 0x823BA890; continue 'dispatch;
	}
	// 823BA88C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 823BA890: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 823BA894: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823BA898: 4BF4B1D9  bl 0x82305a70
	ctx.lr = 0x823BA89C;
	sub_82305A70(ctx, base);
	// 823BA89C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 823BA8A0: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 823BA8A4: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 823BA8A8: 4BF4B211  bl 0x82305ab8
	ctx.lr = 0x823BA8AC;
	sub_82305AB8(ctx, base);
	// 823BA8AC: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 823BA8B0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 823BA8B4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 823BA8B8: 4BF4CEC1  bl 0x82307778
	ctx.lr = 0x823BA8BC;
	sub_82307778(ctx, base);
	// 823BA8BC: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823BA8C0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823BA8C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BA8C8: 4814E579  bl 0x82508e40
	ctx.lr = 0x823BA8CC;
	sub_82508E40(ctx, base);
	// 823BA8CC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BA8D0: 48A373C1  bl 0x82df1c90
	ctx.lr = 0x823BA8D4;
	sub_82DF1C90(ctx, base);
	// 823BA8D4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 823BA8D8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823BA8DC: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 823BA8E0: 4BF4A101  bl 0x823049e0
	ctx.lr = 0x823BA8E4;
	sub_823049E0(ctx, base);
	// 823BA8E4: 38210110  addi r1, r1, 0x110
	ctx.r[1].s64 = ctx.r[1].s64 + 272;
	// 823BA8E8: 48DED8CC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA8F0 size=80
    let mut pc: u32 = 0x823BA8F0;
    'dispatch: loop {
        match pc {
            0x823BA8F0 => {
    //   block [0x823BA8F0..0x823BA940)
	// 823BA8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA8F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA8F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BA8FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA900: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BA904: 48AA2025  bl 0x82e5c928
	ctx.lr = 0x823BA908;
	sub_82E5C928(ctx, base);
	// 823BA908: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BA90C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BA910: 394AF8B4  addi r10, r10, -0x74c
	ctx.r[10].s64 = ctx.r[10].s64 + -1868;
	// 823BA914: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BA918: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BA91C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BA920: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BA924: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 823BA928: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 823BA92C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BA930: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA934: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA938: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BA93C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA940 size=80
    let mut pc: u32 = 0x823BA940;
    'dispatch: loop {
        match pc {
            0x823BA940 => {
    //   block [0x823BA940..0x823BA990)
	// 823BA940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA944: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA948: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BA94C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA950: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BA954: 48AA1FD5  bl 0x82e5c928
	ctx.lr = 0x823BA958;
	sub_82E5C928(ctx, base);
	// 823BA958: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BA95C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BA960: 394AF8DC  addi r10, r10, -0x724
	ctx.r[10].s64 = ctx.r[10].s64 + -1828;
	// 823BA964: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BA968: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 823BA96C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BA970: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BA974: 48A3877D  bl 0x82df30f0
	ctx.lr = 0x823BA978;
	sub_82DF30F0(ctx, base);
	// 823BA978: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BA97C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BA980: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA984: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA988: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BA98C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA990(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA990 size=96
    let mut pc: u32 = 0x823BA990;
    'dispatch: loop {
        match pc {
            0x823BA990 => {
    //   block [0x823BA990..0x823BA9F0)
	// 823BA990: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA994: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA998: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BA99C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BA9A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BA9A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BA9A8: 48AA1F81  bl 0x82e5c928
	ctx.lr = 0x823BA9AC;
	sub_82E5C928(ctx, base);
	// 823BA9AC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823BA9B0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 823BA9B4: 396BF92C  addi r11, r11, -0x6d4
	ctx.r[11].s64 = ctx.r[11].s64 + -1748;
	// 823BA9B8: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 823BA9BC: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 823BA9C0: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 823BA9C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BA9C8: 48A38729  bl 0x82df30f0
	ctx.lr = 0x823BA9CC;
	sub_82DF30F0(ctx, base);
	// 823BA9CC: 93DF0088  stw r30, 0x88(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(136 as u32), ctx.r[30].u32 ) };
	// 823BA9D0: 93DF008C  stw r30, 0x8c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 823BA9D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BA9D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BA9DC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BA9E0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BA9E4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BA9E8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BA9EC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BA9F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BA9F0 size=80
    let mut pc: u32 = 0x823BA9F0;
    'dispatch: loop {
        match pc {
            0x823BA9F0 => {
    //   block [0x823BA9F0..0x823BAA40)
	// 823BA9F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BA9F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BA9F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BA9FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAA00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BAA04: 48AA1F25  bl 0x82e5c928
	ctx.lr = 0x823BAA08;
	sub_82E5C928(ctx, base);
	// 823BAA08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BAA0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BAA10: 394AFA94  addi r10, r10, -0x56c
	ctx.r[10].s64 = ctx.r[10].s64 + -1388;
	// 823BAA14: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BAA18: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 823BAA1C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BAA20: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BAA24: 48A386CD  bl 0x82df30f0
	ctx.lr = 0x823BAA28;
	sub_82DF30F0(ctx, base);
	// 823BAA28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BAA2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BAA30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAA34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAA38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAA3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAA40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAA40 size=80
    let mut pc: u32 = 0x823BAA40;
    'dispatch: loop {
        match pc {
            0x823BAA40 => {
    //   block [0x823BAA40..0x823BAA90)
	// 823BAA40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAA44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAA48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAA4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAA50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BAA54: 48AA1ED5  bl 0x82e5c928
	ctx.lr = 0x823BAA58;
	sub_82E5C928(ctx, base);
	// 823BAA58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BAA5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BAA60: 394AFABC  addi r10, r10, -0x544
	ctx.r[10].s64 = ctx.r[10].s64 + -1348;
	// 823BAA64: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BAA68: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 823BAA6C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BAA70: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BAA74: 48A3867D  bl 0x82df30f0
	ctx.lr = 0x823BAA78;
	sub_82DF30F0(ctx, base);
	// 823BAA78: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BAA7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BAA80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAA84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAA88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAA8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAA90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAA90 size=96
    let mut pc: u32 = 0x823BAA90;
    'dispatch: loop {
        match pc {
            0x823BAA90 => {
    //   block [0x823BAA90..0x823BAAF0)
	// 823BAA90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAA94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAA98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BAA9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAAA0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAAA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BAAA8: 48AA1E81  bl 0x82e5c928
	ctx.lr = 0x823BAAAC;
	sub_82E5C928(ctx, base);
	// 823BAAAC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823BAAB0: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 823BAAB4: 396BFB34  addi r11, r11, -0x4cc
	ctx.r[11].s64 = ctx.r[11].s64 + -1228;
	// 823BAAB8: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 823BAABC: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 823BAAC0: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 823BAAC4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BAAC8: 48A38629  bl 0x82df30f0
	ctx.lr = 0x823BAACC;
	sub_82DF30F0(ctx, base);
	// 823BAACC: 93DF0074  stw r30, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[30].u32 ) };
	// 823BAAD0: 93DF0078  stw r30, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[30].u32 ) };
	// 823BAAD4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BAAD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BAADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAAE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAAE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BAAE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAAEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAAF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAAF0 size=80
    let mut pc: u32 = 0x823BAAF0;
    'dispatch: loop {
        match pc {
            0x823BAAF0 => {
    //   block [0x823BAAF0..0x823BAB40)
	// 823BAAF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAAF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAAF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAAFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAB00: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BAB04: 48AA1E25  bl 0x82e5c928
	ctx.lr = 0x823BAB08;
	sub_82E5C928(ctx, base);
	// 823BAB08: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BAB0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BAB10: 394AFB5C  addi r10, r10, -0x4a4
	ctx.r[10].s64 = ctx.r[10].s64 + -1188;
	// 823BAB14: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BAB18: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 823BAB1C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BAB20: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BAB24: 48A385CD  bl 0x82df30f0
	ctx.lr = 0x823BAB28;
	sub_82DF30F0(ctx, base);
	// 823BAB28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BAB2C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BAB30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAB34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAB38: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAB3C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAB40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAB40 size=80
    let mut pc: u32 = 0x823BAB40;
    'dispatch: loop {
        match pc {
            0x823BAB40 => {
    //   block [0x823BAB40..0x823BAB90)
	// 823BAB40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAB44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAB48: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAB4C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAB50: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BAB54: 48AA1DD5  bl 0x82e5c928
	ctx.lr = 0x823BAB58;
	sub_82E5C928(ctx, base);
	// 823BAB58: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BAB5C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BAB60: 394AFB84  addi r10, r10, -0x47c
	ctx.r[10].s64 = ctx.r[10].s64 + -1148;
	// 823BAB64: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BAB68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BAB6C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BAB70: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BAB74: 917F0098  stw r11, 0x98(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(152 as u32), ctx.r[11].u32 ) };
	// 823BAB78: 917F009C  stw r11, 0x9c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(156 as u32), ctx.r[11].u32 ) };
	// 823BAB7C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BAB80: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAB84: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAB88: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAB8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAB90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAB90 size=80
    let mut pc: u32 = 0x823BAB90;
    'dispatch: loop {
        match pc {
            0x823BAB90 => {
    //   block [0x823BAB90..0x823BABE0)
	// 823BAB90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAB94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAB98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAB9C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BABA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BABA4: 48AA1D85  bl 0x82e5c928
	ctx.lr = 0x823BABA8;
	sub_82E5C928(ctx, base);
	// 823BABA8: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BABAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BABB0: 394AFBAC  addi r10, r10, -0x454
	ctx.r[10].s64 = ctx.r[10].s64 + -1108;
	// 823BABB4: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BABB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BABBC: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BABC0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BABC4: 917F00B0  stw r11, 0xb0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(176 as u32), ctx.r[11].u32 ) };
	// 823BABC8: 917F00B4  stw r11, 0xb4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(180 as u32), ctx.r[11].u32 ) };
	// 823BABCC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BABD0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BABD4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BABD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BABDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BABE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BABE0 size=100
    let mut pc: u32 = 0x823BABE0;
    'dispatch: loop {
        match pc {
            0x823BABE0 => {
    //   block [0x823BABE0..0x823BAC44)
	// 823BABE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BABE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BABE8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BABEC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BABF0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BABF4: 48AA1D35  bl 0x82e5c928
	ctx.lr = 0x823BABF8;
	sub_82E5C928(ctx, base);
	// 823BABF8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823BABFC: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 823BAC00: 390BFBD4  addi r8, r11, -0x42c
	ctx.r[8].s64 = ctx.r[11].s64 + -1068;
	// 823BAC04: 915F0060  stw r10, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 823BAC08: 397F00D4  addi r11, r31, 0xd4
	ctx.r[11].s64 = ctx.r[31].s64 + 212;
	// 823BAC0C: 915F0064  stw r10, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 823BAC10: 3920004F  li r9, 0x4f
	ctx.r[9].s64 = 79;
	// 823BAC14: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 823BAC18: 914B0000  stw r10, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BAC1C: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 823BAC20: 914B0004  stw r10, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823BAC24: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 823BAC28: 4080FFF0  bge 0x823bac18
	if !ctx.cr[0].lt {
	pc = 0x823BAC18; continue 'dispatch;
	}
	// 823BAC2C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BAC30: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BAC34: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAC38: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAC3C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAC40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAC48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAC48 size=80
    let mut pc: u32 = 0x823BAC48;
    'dispatch: loop {
        match pc {
            0x823BAC48 => {
    //   block [0x823BAC48..0x823BAC98)
	// 823BAC48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAC4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAC50: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAC54: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAC58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BAC5C: 48AA1CCD  bl 0x82e5c928
	ctx.lr = 0x823BAC60;
	sub_82E5C928(ctx, base);
	// 823BAC60: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BAC64: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BAC68: 394AFC24  addi r10, r10, -0x3dc
	ctx.r[10].s64 = ctx.r[10].s64 + -988;
	// 823BAC6C: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BAC70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BAC74: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BAC78: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BAC7C: 917F00C8  stw r11, 0xc8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(200 as u32), ctx.r[11].u32 ) };
	// 823BAC80: 917F00CC  stw r11, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[11].u32 ) };
	// 823BAC84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BAC88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAC8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAC90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAC94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAC98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAC98 size=80
    let mut pc: u32 = 0x823BAC98;
    'dispatch: loop {
        match pc {
            0x823BAC98 => {
    //   block [0x823BAC98..0x823BACE8)
	// 823BAC98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAC9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BACA0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BACA4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BACA8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BACAC: 48AA1C7D  bl 0x82e5c928
	ctx.lr = 0x823BACB0;
	sub_82E5C928(ctx, base);
	// 823BACB0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BACB4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BACB8: 394AFC4C  addi r10, r10, -0x3b4
	ctx.r[10].s64 = ctx.r[10].s64 + -948;
	// 823BACBC: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BACC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BACC4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BACC8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BACCC: 917F00E8  stw r11, 0xe8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(232 as u32), ctx.r[11].u32 ) };
	// 823BACD0: 917F00EC  stw r11, 0xec(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(236 as u32), ctx.r[11].u32 ) };
	// 823BACD4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BACD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BACDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BACE0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BACE4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BACE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BACE8 size=80
    let mut pc: u32 = 0x823BACE8;
    'dispatch: loop {
        match pc {
            0x823BACE8 => {
    //   block [0x823BACE8..0x823BAD38)
	// 823BACE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BACEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BACF0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BACF4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BACF8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BACFC: 48AA1C2D  bl 0x82e5c928
	ctx.lr = 0x823BAD00;
	sub_82E5C928(ctx, base);
	// 823BAD00: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BAD04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BAD08: 394AFC74  addi r10, r10, -0x38c
	ctx.r[10].s64 = ctx.r[10].s64 + -908;
	// 823BAD0C: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BAD10: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 823BAD14: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BAD18: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BAD1C: 48A383D5  bl 0x82df30f0
	ctx.lr = 0x823BAD20;
	sub_82DF30F0(ctx, base);
	// 823BAD20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BAD24: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BAD28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAD2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAD30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAD34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAD38 size=80
    let mut pc: u32 = 0x823BAD38;
    'dispatch: loop {
        match pc {
            0x823BAD38 => {
    //   block [0x823BAD38..0x823BAD88)
	// 823BAD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAD3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAD40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAD44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAD48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BAD4C: 48AA1BDD  bl 0x82e5c928
	ctx.lr = 0x823BAD50;
	sub_82E5C928(ctx, base);
	// 823BAD50: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BAD54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BAD58: 394AFCC4  addi r10, r10, -0x33c
	ctx.r[10].s64 = ctx.r[10].s64 + -828;
	// 823BAD5C: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BAD60: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 823BAD64: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BAD68: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BAD6C: 48A38385  bl 0x82df30f0
	ctx.lr = 0x823BAD70;
	sub_82DF30F0(ctx, base);
	// 823BAD70: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BAD74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BAD78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAD7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAD80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAD84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAD88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAD88 size=80
    let mut pc: u32 = 0x823BAD88;
    'dispatch: loop {
        match pc {
            0x823BAD88 => {
    //   block [0x823BAD88..0x823BADD8)
	// 823BAD88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAD8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAD90: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAD94: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAD98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BAD9C: 48AA1B8D  bl 0x82e5c928
	ctx.lr = 0x823BADA0;
	sub_82E5C928(ctx, base);
	// 823BADA0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BADA4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BADA8: 394AFCEC  addi r10, r10, -0x314
	ctx.r[10].s64 = ctx.r[10].s64 + -788;
	// 823BADAC: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BADB0: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 823BADB4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BADB8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BADBC: 48A38335  bl 0x82df30f0
	ctx.lr = 0x823BADC0;
	sub_82DF30F0(ctx, base);
	// 823BADC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BADC4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BADC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BADCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BADD0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BADD4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BADD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BADD8 size=80
    let mut pc: u32 = 0x823BADD8;
    'dispatch: loop {
        match pc {
            0x823BADD8 => {
    //   block [0x823BADD8..0x823BAE28)
	// 823BADD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BADDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BADE0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BADE4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BADE8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BADEC: 48AA1B3D  bl 0x82e5c928
	ctx.lr = 0x823BADF0;
	sub_82E5C928(ctx, base);
	// 823BADF0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BADF4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BADF8: 394A00AC  addi r10, r10, 0xac
	ctx.r[10].s64 = ctx.r[10].s64 + 172;
	// 823BADFC: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BAE00: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BAE04: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BAE08: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BAE0C: 917F00D4  stw r11, 0xd4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(212 as u32), ctx.r[11].u32 ) };
	// 823BAE10: 917F00D8  stw r11, 0xd8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(216 as u32), ctx.r[11].u32 ) };
	// 823BAE14: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BAE18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAE1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAE20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAE24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAE28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAE28 size=80
    let mut pc: u32 = 0x823BAE28;
    'dispatch: loop {
        match pc {
            0x823BAE28 => {
    //   block [0x823BAE28..0x823BAE78)
	// 823BAE28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAE2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAE30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAE34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAE38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BAE3C: 48AA1AED  bl 0x82e5c928
	ctx.lr = 0x823BAE40;
	sub_82E5C928(ctx, base);
	// 823BAE40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BAE44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BAE48: 394A0124  addi r10, r10, 0x124
	ctx.r[10].s64 = ctx.r[10].s64 + 292;
	// 823BAE4C: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BAE50: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BAE54: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BAE58: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BAE5C: 917F0078  stw r11, 0x78(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 823BAE60: 917F007C  stw r11, 0x7c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 823BAE64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BAE68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAE6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAE70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAE74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAE78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAE78 size=80
    let mut pc: u32 = 0x823BAE78;
    'dispatch: loop {
        match pc {
            0x823BAE78 => {
    //   block [0x823BAE78..0x823BAEC8)
	// 823BAE78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAE7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAE80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAE84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAE88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BAE8C: 48AA1A9D  bl 0x82e5c928
	ctx.lr = 0x823BAE90;
	sub_82E5C928(ctx, base);
	// 823BAE90: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BAE94: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BAE98: 394A023C  addi r10, r10, 0x23c
	ctx.r[10].s64 = ctx.r[10].s64 + 572;
	// 823BAE9C: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BAEA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BAEA4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BAEA8: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BAEAC: 917F0070  stw r11, 0x70(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 823BAEB0: 917F0074  stw r11, 0x74(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 823BAEB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BAEB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAEBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAEC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAEC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAEC8 size=96
    let mut pc: u32 = 0x823BAEC8;
    'dispatch: loop {
        match pc {
            0x823BAEC8 => {
    //   block [0x823BAEC8..0x823BAF28)
	// 823BAEC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAED0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BAED4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAED8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAEDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BAEE0: 48AA1A49  bl 0x82e5c928
	ctx.lr = 0x823BAEE4;
	sub_82E5C928(ctx, base);
	// 823BAEE4: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823BAEE8: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 823BAEEC: 396B02B4  addi r11, r11, 0x2b4
	ctx.r[11].s64 = ctx.r[11].s64 + 692;
	// 823BAEF0: 93DF0060  stw r30, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[30].u32 ) };
	// 823BAEF4: 387F00C8  addi r3, r31, 0xc8
	ctx.r[3].s64 = ctx.r[31].s64 + 200;
	// 823BAEF8: 93DF0064  stw r30, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 823BAEFC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BAF00: 48A381F1  bl 0x82df30f0
	ctx.lr = 0x823BAF04;
	sub_82DF30F0(ctx, base);
	// 823BAF04: 93DF00CC  stw r30, 0xcc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(204 as u32), ctx.r[30].u32 ) };
	// 823BAF08: 93DF00D0  stw r30, 0xd0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(208 as u32), ctx.r[30].u32 ) };
	// 823BAF0C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BAF10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BAF14: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAF18: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAF1C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BAF20: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAF24: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAF28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAF28 size=80
    let mut pc: u32 = 0x823BAF28;
    'dispatch: loop {
        match pc {
            0x823BAF28 => {
    //   block [0x823BAF28..0x823BAF78)
	// 823BAF28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAF2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAF30: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAF34: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAF38: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BAF3C: 48AA19ED  bl 0x82e5c928
	ctx.lr = 0x823BAF40;
	sub_82E5C928(ctx, base);
	// 823BAF40: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BAF44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BAF48: 394A0304  addi r10, r10, 0x304
	ctx.r[10].s64 = ctx.r[10].s64 + 772;
	// 823BAF4C: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BAF50: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 823BAF54: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BAF58: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 823BAF5C: 48A38195  bl 0x82df30f0
	ctx.lr = 0x823BAF60;
	sub_82DF30F0(ctx, base);
	// 823BAF60: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BAF64: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BAF68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAF6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAF70: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAF74: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAF78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAF78 size=80
    let mut pc: u32 = 0x823BAF78;
    'dispatch: loop {
        match pc {
            0x823BAF78 => {
    //   block [0x823BAF78..0x823BAFC8)
	// 823BAF78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAF7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAF80: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAF84: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAF88: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BAF8C: 807F0074  lwz r3, 0x74(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(116 as u32) ) } as u64;
	// 823BAF90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BAF94: 419A0008  beq cr6, 0x823baf9c
	if ctx.cr[6].eq {
	pc = 0x823BAF9C; continue 'dispatch;
	}
	// 823BAF98: 4BF058F9  bl 0x822c0890
	ctx.lr = 0x823BAF9C;
	sub_822C0890(ctx, base);
	// 823BAF9C: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BAFA0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BAFA4: 419A0008  beq cr6, 0x823bafac
	if ctx.cr[6].eq {
	pc = 0x823BAFAC; continue 'dispatch;
	}
	// 823BAFA8: 4BF058E9  bl 0x822c0890
	ctx.lr = 0x823BAFAC;
	sub_822C0890(ctx, base);
	// 823BAFAC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BAFB0: 48AA15D9  bl 0x82e5c588
	ctx.lr = 0x823BAFB4;
	sub_82E5C588(ctx, base);
	// 823BAFB4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BAFB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BAFBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BAFC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BAFC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BAFC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BAFC8 size=88
    let mut pc: u32 = 0x823BAFC8;
    'dispatch: loop {
        match pc {
            0x823BAFC8 => {
    //   block [0x823BAFC8..0x823BB020)
	// 823BAFC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BAFCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BAFD0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BAFD4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BAFD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BAFDC: 807F008C  lwz r3, 0x8c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(140 as u32) ) } as u64;
	// 823BAFE0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BAFE4: 419A0008  beq cr6, 0x823bafec
	if ctx.cr[6].eq {
	pc = 0x823BAFEC; continue 'dispatch;
	}
	// 823BAFE8: 4BF058A9  bl 0x822c0890
	ctx.lr = 0x823BAFEC;
	sub_822C0890(ctx, base);
	// 823BAFEC: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 823BAFF0: 48A38439  bl 0x82df3428
	ctx.lr = 0x823BAFF4;
	sub_82DF3428(ctx, base);
	// 823BAFF4: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BAFF8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BAFFC: 419A0008  beq cr6, 0x823bb004
	if ctx.cr[6].eq {
	pc = 0x823BB004; continue 'dispatch;
	}
	// 823BB000: 4BF05891  bl 0x822c0890
	ctx.lr = 0x823BB004;
	sub_822C0890(ctx, base);
	// 823BB004: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB008: 48AA1581  bl 0x82e5c588
	ctx.lr = 0x823BB00C;
	sub_82E5C588(ctx, base);
	// 823BB00C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BB010: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB014: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB018: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB01C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB020 size=88
    let mut pc: u32 = 0x823BB020;
    'dispatch: loop {
        match pc {
            0x823BB020 => {
    //   block [0x823BB020..0x823BB078)
	// 823BB020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB024: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB028: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB02C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB030: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BB034: 807F0078  lwz r3, 0x78(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(120 as u32) ) } as u64;
	// 823BB038: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB03C: 419A0008  beq cr6, 0x823bb044
	if ctx.cr[6].eq {
	pc = 0x823BB044; continue 'dispatch;
	}
	// 823BB040: 4BF05851  bl 0x822c0890
	ctx.lr = 0x823BB044;
	sub_822C0890(ctx, base);
	// 823BB044: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 823BB048: 48A383E1  bl 0x82df3428
	ctx.lr = 0x823BB04C;
	sub_82DF3428(ctx, base);
	// 823BB04C: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BB050: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB054: 419A0008  beq cr6, 0x823bb05c
	if ctx.cr[6].eq {
	pc = 0x823BB05C; continue 'dispatch;
	}
	// 823BB058: 4BF05839  bl 0x822c0890
	ctx.lr = 0x823BB05C;
	sub_822C0890(ctx, base);
	// 823BB05C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB060: 48AA1529  bl 0x82e5c588
	ctx.lr = 0x823BB064;
	sub_82E5C588(ctx, base);
	// 823BB064: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BB068: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB06C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB070: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB078 size=80
    let mut pc: u32 = 0x823BB078;
    'dispatch: loop {
        match pc {
            0x823BB078 => {
    //   block [0x823BB078..0x823BB0C8)
	// 823BB078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB07C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB080: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB084: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB088: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BB08C: 807F009C  lwz r3, 0x9c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(156 as u32) ) } as u64;
	// 823BB090: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB094: 419A0008  beq cr6, 0x823bb09c
	if ctx.cr[6].eq {
	pc = 0x823BB09C; continue 'dispatch;
	}
	// 823BB098: 4BF057F9  bl 0x822c0890
	ctx.lr = 0x823BB09C;
	sub_822C0890(ctx, base);
	// 823BB09C: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BB0A0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB0A4: 419A0008  beq cr6, 0x823bb0ac
	if ctx.cr[6].eq {
	pc = 0x823BB0AC; continue 'dispatch;
	}
	// 823BB0A8: 4BF057E9  bl 0x822c0890
	ctx.lr = 0x823BB0AC;
	sub_822C0890(ctx, base);
	// 823BB0AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB0B0: 48AA14D9  bl 0x82e5c588
	ctx.lr = 0x823BB0B4;
	sub_82E5C588(ctx, base);
	// 823BB0B4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BB0B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB0BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB0C0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB0C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB0C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB0C8 size=80
    let mut pc: u32 = 0x823BB0C8;
    'dispatch: loop {
        match pc {
            0x823BB0C8 => {
    //   block [0x823BB0C8..0x823BB118)
	// 823BB0C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB0CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB0D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB0D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB0D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BB0DC: 807F00B4  lwz r3, 0xb4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(180 as u32) ) } as u64;
	// 823BB0E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB0E4: 419A0008  beq cr6, 0x823bb0ec
	if ctx.cr[6].eq {
	pc = 0x823BB0EC; continue 'dispatch;
	}
	// 823BB0E8: 4BF057A9  bl 0x822c0890
	ctx.lr = 0x823BB0EC;
	sub_822C0890(ctx, base);
	// 823BB0EC: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BB0F0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB0F4: 419A0008  beq cr6, 0x823bb0fc
	if ctx.cr[6].eq {
	pc = 0x823BB0FC; continue 'dispatch;
	}
	// 823BB0F8: 4BF05799  bl 0x822c0890
	ctx.lr = 0x823BB0FC;
	sub_822C0890(ctx, base);
	// 823BB0FC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB100: 48AA1489  bl 0x82e5c588
	ctx.lr = 0x823BB104;
	sub_82E5C588(ctx, base);
	// 823BB104: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BB108: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB10C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB110: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB114: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB118(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB118 size=88
    let mut pc: u32 = 0x823BB118;
    'dispatch: loop {
        match pc {
            0x823BB118 => {
    //   block [0x823BB118..0x823BB170)
	// 823BB118: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB11C: 48DED051  bl 0x831a816c
	ctx.lr = 0x823BB120;
	sub_831A8130(ctx, base);
	// 823BB120: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB124: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823BB128: 3BA0004F  li r29, 0x4f
	ctx.r[29].s64 = 79;
	// 823BB12C: 397E0354  addi r11, r30, 0x354
	ctx.r[11].s64 = ctx.r[30].s64 + 852;
	// 823BB130: 3BEB0004  addi r31, r11, 4
	ctx.r[31].s64 = ctx.r[11].s64 + 4;
	// 823BB134: 3BFFFFF8  addi r31, r31, -8
	ctx.r[31].s64 = ctx.r[31].s64 + -8;
	// 823BB138: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB13C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB140: 419A0008  beq cr6, 0x823bb148
	if ctx.cr[6].eq {
	pc = 0x823BB148; continue 'dispatch;
	}
	// 823BB144: 4BF0574D  bl 0x822c0890
	ctx.lr = 0x823BB148;
	sub_822C0890(ctx, base);
	// 823BB148: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 823BB14C: 4080FFE8  bge 0x823bb134
	if !ctx.cr[0].lt {
	pc = 0x823BB134; continue 'dispatch;
	}
	// 823BB150: 807E0064  lwz r3, 0x64(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BB154: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB158: 419A0008  beq cr6, 0x823bb160
	if ctx.cr[6].eq {
	pc = 0x823BB160; continue 'dispatch;
	}
	// 823BB15C: 4BF05735  bl 0x822c0890
	ctx.lr = 0x823BB160;
	sub_822C0890(ctx, base);
	// 823BB160: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BB164: 48AA1425  bl 0x82e5c588
	ctx.lr = 0x823BB168;
	sub_82E5C588(ctx, base);
	// 823BB168: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BB16C: 48DED050  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB170(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB170 size=80
    let mut pc: u32 = 0x823BB170;
    'dispatch: loop {
        match pc {
            0x823BB170 => {
    //   block [0x823BB170..0x823BB1C0)
	// 823BB170: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB174: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB178: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB17C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB180: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BB184: 807F00CC  lwz r3, 0xcc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(204 as u32) ) } as u64;
	// 823BB188: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB18C: 419A0008  beq cr6, 0x823bb194
	if ctx.cr[6].eq {
	pc = 0x823BB194; continue 'dispatch;
	}
	// 823BB190: 4BF05701  bl 0x822c0890
	ctx.lr = 0x823BB194;
	sub_822C0890(ctx, base);
	// 823BB194: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BB198: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB19C: 419A0008  beq cr6, 0x823bb1a4
	if ctx.cr[6].eq {
	pc = 0x823BB1A4; continue 'dispatch;
	}
	// 823BB1A0: 4BF056F1  bl 0x822c0890
	ctx.lr = 0x823BB1A4;
	sub_822C0890(ctx, base);
	// 823BB1A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB1A8: 48AA13E1  bl 0x82e5c588
	ctx.lr = 0x823BB1AC;
	sub_82E5C588(ctx, base);
	// 823BB1AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BB1B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB1B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB1B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB1BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB1C0 size=80
    let mut pc: u32 = 0x823BB1C0;
    'dispatch: loop {
        match pc {
            0x823BB1C0 => {
    //   block [0x823BB1C0..0x823BB210)
	// 823BB1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB1C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB1C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB1CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB1D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BB1D4: 807F00EC  lwz r3, 0xec(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(236 as u32) ) } as u64;
	// 823BB1D8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB1DC: 419A0008  beq cr6, 0x823bb1e4
	if ctx.cr[6].eq {
	pc = 0x823BB1E4; continue 'dispatch;
	}
	// 823BB1E0: 4BF056B1  bl 0x822c0890
	ctx.lr = 0x823BB1E4;
	sub_822C0890(ctx, base);
	// 823BB1E4: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BB1E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB1EC: 419A0008  beq cr6, 0x823bb1f4
	if ctx.cr[6].eq {
	pc = 0x823BB1F4; continue 'dispatch;
	}
	// 823BB1F0: 4BF056A1  bl 0x822c0890
	ctx.lr = 0x823BB1F4;
	sub_822C0890(ctx, base);
	// 823BB1F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB1F8: 48AA1391  bl 0x82e5c588
	ctx.lr = 0x823BB1FC;
	sub_82E5C588(ctx, base);
	// 823BB1FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BB200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB208: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB20C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB210 size=80
    let mut pc: u32 = 0x823BB210;
    'dispatch: loop {
        match pc {
            0x823BB210 => {
    //   block [0x823BB210..0x823BB260)
	// 823BB210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB218: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB21C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB220: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BB224: 807F00D8  lwz r3, 0xd8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(216 as u32) ) } as u64;
	// 823BB228: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB22C: 419A0008  beq cr6, 0x823bb234
	if ctx.cr[6].eq {
	pc = 0x823BB234; continue 'dispatch;
	}
	// 823BB230: 4BF05661  bl 0x822c0890
	ctx.lr = 0x823BB234;
	sub_822C0890(ctx, base);
	// 823BB234: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BB238: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB23C: 419A0008  beq cr6, 0x823bb244
	if ctx.cr[6].eq {
	pc = 0x823BB244; continue 'dispatch;
	}
	// 823BB240: 4BF05651  bl 0x822c0890
	ctx.lr = 0x823BB244;
	sub_822C0890(ctx, base);
	// 823BB244: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB248: 48AA1341  bl 0x82e5c588
	ctx.lr = 0x823BB24C;
	sub_82E5C588(ctx, base);
	// 823BB24C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BB250: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB254: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB258: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB25C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB260 size=80
    let mut pc: u32 = 0x823BB260;
    'dispatch: loop {
        match pc {
            0x823BB260 => {
    //   block [0x823BB260..0x823BB2B0)
	// 823BB260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB268: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB26C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB270: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BB274: 807F007C  lwz r3, 0x7c(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(124 as u32) ) } as u64;
	// 823BB278: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB27C: 419A0008  beq cr6, 0x823bb284
	if ctx.cr[6].eq {
	pc = 0x823BB284; continue 'dispatch;
	}
	// 823BB280: 4BF05611  bl 0x822c0890
	ctx.lr = 0x823BB284;
	sub_822C0890(ctx, base);
	// 823BB284: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BB288: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB28C: 419A0008  beq cr6, 0x823bb294
	if ctx.cr[6].eq {
	pc = 0x823BB294; continue 'dispatch;
	}
	// 823BB290: 4BF05601  bl 0x822c0890
	ctx.lr = 0x823BB294;
	sub_822C0890(ctx, base);
	// 823BB294: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB298: 48AA12F1  bl 0x82e5c588
	ctx.lr = 0x823BB29C;
	sub_82E5C588(ctx, base);
	// 823BB29C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BB2A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB2A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB2A8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB2AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB2B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB2B0 size=88
    let mut pc: u32 = 0x823BB2B0;
    'dispatch: loop {
        match pc {
            0x823BB2B0 => {
    //   block [0x823BB2B0..0x823BB308)
	// 823BB2B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB2B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB2B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB2BC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB2C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BB2C4: 807F00D0  lwz r3, 0xd0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(208 as u32) ) } as u64;
	// 823BB2C8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB2CC: 419A0008  beq cr6, 0x823bb2d4
	if ctx.cr[6].eq {
	pc = 0x823BB2D4; continue 'dispatch;
	}
	// 823BB2D0: 4BF055C1  bl 0x822c0890
	ctx.lr = 0x823BB2D4;
	sub_822C0890(ctx, base);
	// 823BB2D4: 387F00C8  addi r3, r31, 0xc8
	ctx.r[3].s64 = ctx.r[31].s64 + 200;
	// 823BB2D8: 48A38151  bl 0x82df3428
	ctx.lr = 0x823BB2DC;
	sub_82DF3428(ctx, base);
	// 823BB2DC: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BB2E0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BB2E4: 419A0008  beq cr6, 0x823bb2ec
	if ctx.cr[6].eq {
	pc = 0x823BB2EC; continue 'dispatch;
	}
	// 823BB2E8: 4BF055A9  bl 0x822c0890
	ctx.lr = 0x823BB2EC;
	sub_822C0890(ctx, base);
	// 823BB2EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB2F0: 48AA1299  bl 0x82e5c588
	ctx.lr = 0x823BB2F4;
	sub_82E5C588(ctx, base);
	// 823BB2F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BB2F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB2FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB300: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB304: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB308 size=188
    let mut pc: u32 = 0x823BB308;
    'dispatch: loop {
        match pc {
            0x823BB308 => {
    //   block [0x823BB308..0x823BB3C4)
	// 823BB308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB30C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB310: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BB314: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB318: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB31C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823BB320: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BB324: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 823BB328: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823BB32C: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BB330: 4BF05609  bl 0x822c0938
	ctx.lr = 0x823BB334;
	sub_822C0938(ctx, base);
	// 823BB334: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BB338: 41820028  beq 0x823bb360
	if ctx.cr[0].eq {
	pc = 0x823BB360; continue 'dispatch;
	}
	// 823BB33C: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823BB340: 93E3000C  stw r31, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[31].u32 ) };
	// 823BB344: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 823BB348: 392BF78C  addi r9, r11, -0x874
	ctx.r[9].s64 = ctx.r[11].s64 + -2164;
	// 823BB34C: 91430004  stw r10, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823BB350: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823BB354: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 823BB358: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 823BB35C: 48000008  b 0x823bb364
	pc = 0x823BB364; continue 'dispatch;
	// 823BB360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BB364: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BB368: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BB36C: 409A003C  bne cr6, 0x823bb3a8
	if !ctx.cr[6].eq {
	pc = 0x823BB3A8; continue 'dispatch;
	}
	// 823BB370: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823BB374: 419A0014  beq cr6, 0x823bb388
	if ctx.cr[6].eq {
	pc = 0x823BB388; continue 'dispatch;
	}
	// 823BB378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB37C: 4BFF6235  bl 0x823b15b0
	ctx.lr = 0x823BB380;
	sub_823B15B0(ctx, base);
	// 823BB380: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB384: 4BF04EE5  bl 0x822c0268
	ctx.lr = 0x823BB388;
	sub_822C0268(ctx, base);
	// 823BB388: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823BB38C: 3D408200  lis r10, -0x7e00
	ctx.r[10].s64 = -2113929216;
	// 823BB390: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BB394: 394A0828  addi r10, r10, 0x828
	ctx.r[10].s64 = ctx.r[10].s64 + 2088;
	// 823BB398: 816B7E20  lwz r11, 0x7e20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32288 as u32) ) } as u64;
	// 823BB39C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823BB3A0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823BB3A4: 4BF04C5D  bl 0x822c0000
	ctx.lr = 0x823BB3A8;
	sub_822C0000(ctx, base);
	// 823BB3A8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BB3AC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BB3B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB3B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB3B8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BB3BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB3C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB3C8 size=64
    let mut pc: u32 = 0x823BB3C8;
    'dispatch: loop {
        match pc {
            0x823BB3C8 => {
    //   block [0x823BB3C8..0x823BB408)
	// 823BB3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB3CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB3D0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB3D4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB3D8: 83E3000C  lwz r31, 0xc(r3)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 823BB3DC: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823BB3E0: 419A0014  beq cr6, 0x823bb3f4
	if ctx.cr[6].eq {
	pc = 0x823BB3F4; continue 'dispatch;
	}
	// 823BB3E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB3E8: 4BFF61C9  bl 0x823b15b0
	ctx.lr = 0x823BB3EC;
	sub_823B15B0(ctx, base);
	// 823BB3EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB3F0: 4BF04E79  bl 0x822c0268
	ctx.lr = 0x823BB3F4;
	sub_822C0268(ctx, base);
	// 823BB3F4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 823BB3F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB3FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB400: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB404: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB408 size=104
    let mut pc: u32 = 0x823BB408;
    'dispatch: loop {
        match pc {
            0x823BB408 => {
    //   block [0x823BB408..0x823BB470)
	// 823BB408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB40C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB410: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB414: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB418: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823BB41C: 4812F66D  bl 0x824eaa88
	ctx.lr = 0x823BB420;
	sub_824EAA88(ctx, base);
	// 823BB420: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823BB424: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BB428: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB42C: 4812FE95  bl 0x824eb2c0
	ctx.lr = 0x823BB430;
	sub_824EB2C0(ctx, base);
	// 823BB430: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB434: 4815B585  bl 0x825169b8
	ctx.lr = 0x823BB438;
	sub_825169B8(ctx, base);
	// 823BB438: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823BB43C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BB440: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BB444: 419A000C  beq cr6, 0x823bb450
	if ctx.cr[6].eq {
	pc = 0x823BB450; continue 'dispatch;
	}
	// 823BB448: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823BB44C: 4BF05445  bl 0x822c0890
	ctx.lr = 0x823BB450;
	sub_822C0890(ctx, base);
	// 823BB450: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823BB454: 48A3683D  bl 0x82df1c90
	ctx.lr = 0x823BB458;
	sub_82DF1C90(ctx, base);
	// 823BB458: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB45C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BB460: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB464: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB468: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB46C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB470(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB470 size=104
    let mut pc: u32 = 0x823BB470;
    'dispatch: loop {
        match pc {
            0x823BB470 => {
    //   block [0x823BB470..0x823BB4D8)
	// 823BB470: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB474: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB478: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB47C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB480: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823BB484: 4812F605  bl 0x824eaa88
	ctx.lr = 0x823BB488;
	sub_824EAA88(ctx, base);
	// 823BB488: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 823BB48C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BB490: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB494: 4812FE2D  bl 0x824eb2c0
	ctx.lr = 0x823BB498;
	sub_824EB2C0(ctx, base);
	// 823BB498: 80630000  lwz r3, 0(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BB49C: 4815B59D  bl 0x82516a38
	ctx.lr = 0x823BB4A0;
	sub_82516A38(ctx, base);
	// 823BB4A0: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823BB4A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BB4A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BB4AC: 419A000C  beq cr6, 0x823bb4b8
	if ctx.cr[6].eq {
	pc = 0x823BB4B8; continue 'dispatch;
	}
	// 823BB4B0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 823BB4B4: 4BF053DD  bl 0x822c0890
	ctx.lr = 0x823BB4B8;
	sub_822C0890(ctx, base);
	// 823BB4B8: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823BB4BC: 48A367D5  bl 0x82df1c90
	ctx.lr = 0x823BB4C0;
	sub_82DF1C90(ctx, base);
	// 823BB4C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB4C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BB4C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB4CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB4D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB4D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB4D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB4D8 size=556
    let mut pc: u32 = 0x823BB4D8;
    'dispatch: loop {
        match pc {
            0x823BB4D8 => {
    //   block [0x823BB4D8..0x823BB704)
	// 823BB4D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB4DC: 48DECC89  bl 0x831a8164
	ctx.lr = 0x823BB4E0;
	sub_831A8130(ctx, base);
	// 823BB4E0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB4E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BB4E8: 4BF48811  bl 0x82303cf8
	ctx.lr = 0x823BB4EC;
	sub_82303CF8(ctx, base);
	// 823BB4EC: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823BB4F0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BB4F4: 3D208202  lis r9, -0x7dfe
	ctx.r[9].s64 = -2113798144;
	// 823BB4F8: 396BF85C  addi r11, r11, -0x7a4
	ctx.r[11].s64 = ctx.r[11].s64 + -1956;
	// 823BB4FC: 394AF844  addi r10, r10, -0x7bc
	ctx.r[10].s64 = ctx.r[10].s64 + -1980;
	// 823BB500: 3929F834  addi r9, r9, -0x7cc
	ctx.r[9].s64 = ctx.r[9].s64 + -1996;
	// 823BB504: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BB508: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 823BB50C: 915F0028  stw r10, 0x28(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(40 as u32), ctx.r[10].u32 ) };
	// 823BB510: 913F00C0  stw r9, 0xc0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(192 as u32), ctx.r[9].u32 ) };
	// 823BB514: 397F02E8  addi r11, r31, 0x2e8
	ctx.r[11].s64 = ctx.r[31].s64 + 744;
	// 823BB518: 93DF0270  stw r30, 0x270(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(624 as u32), ctx.r[30].u32 ) };
	// 823BB51C: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 823BB520: 93DF0274  stw r30, 0x274(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(628 as u32), ctx.r[30].u32 ) };
	// 823BB524: 93DF0278  stw r30, 0x278(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(632 as u32), ctx.r[30].u32 ) };
	// 823BB528: 93DF027C  stw r30, 0x27c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(636 as u32), ctx.r[30].u32 ) };
	// 823BB52C: 93DF0280  stw r30, 0x280(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(640 as u32), ctx.r[30].u32 ) };
	// 823BB530: 93DF0284  stw r30, 0x284(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(644 as u32), ctx.r[30].u32 ) };
	// 823BB534: 93DF0288  stw r30, 0x288(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(648 as u32), ctx.r[30].u32 ) };
	// 823BB538: 93DF028C  stw r30, 0x28c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(652 as u32), ctx.r[30].u32 ) };
	// 823BB53C: 93DF0290  stw r30, 0x290(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(656 as u32), ctx.r[30].u32 ) };
	// 823BB540: 93DF0294  stw r30, 0x294(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(660 as u32), ctx.r[30].u32 ) };
	// 823BB544: 93DF0298  stw r30, 0x298(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(664 as u32), ctx.r[30].u32 ) };
	// 823BB548: 93DF029C  stw r30, 0x29c(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(668 as u32), ctx.r[30].u32 ) };
	// 823BB54C: 93DF02A0  stw r30, 0x2a0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(672 as u32), ctx.r[30].u32 ) };
	// 823BB550: 93DF02A4  stw r30, 0x2a4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(676 as u32), ctx.r[30].u32 ) };
	// 823BB554: 93DF02A8  stw r30, 0x2a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(680 as u32), ctx.r[30].u32 ) };
	// 823BB558: 93DF02AC  stw r30, 0x2ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(684 as u32), ctx.r[30].u32 ) };
	// 823BB55C: 93DF02B0  stw r30, 0x2b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(688 as u32), ctx.r[30].u32 ) };
	// 823BB560: 93DF02B4  stw r30, 0x2b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(692 as u32), ctx.r[30].u32 ) };
	// 823BB564: 93DF02B8  stw r30, 0x2b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(696 as u32), ctx.r[30].u32 ) };
	// 823BB568: 93DF02BC  stw r30, 0x2bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(700 as u32), ctx.r[30].u32 ) };
	// 823BB56C: 93DF02C0  stw r30, 0x2c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(704 as u32), ctx.r[30].u32 ) };
	// 823BB570: 93DF02C4  stw r30, 0x2c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(708 as u32), ctx.r[30].u32 ) };
	// 823BB574: 93DF02C8  stw r30, 0x2c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(712 as u32), ctx.r[30].u32 ) };
	// 823BB578: 93DF02CC  stw r30, 0x2cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(716 as u32), ctx.r[30].u32 ) };
	// 823BB57C: 93DF02D0  stw r30, 0x2d0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(720 as u32), ctx.r[30].u32 ) };
	// 823BB580: 93DF02D4  stw r30, 0x2d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(724 as u32), ctx.r[30].u32 ) };
	// 823BB584: 93DF02D8  stw r30, 0x2d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(728 as u32), ctx.r[30].u32 ) };
	// 823BB588: 93DF02DC  stw r30, 0x2dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(732 as u32), ctx.r[30].u32 ) };
	// 823BB58C: 93DF02E0  stw r30, 0x2e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(736 as u32), ctx.r[30].u32 ) };
	// 823BB590: 93DF02E4  stw r30, 0x2e4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(740 as u32), ctx.r[30].u32 ) };
	// 823BB594: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 823BB598: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823BB59C: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 823BB5A0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 823BB5A4: 4080FFF0  bge 0x823bb594
	if !ctx.cr[0].lt {
	pc = 0x823BB594; continue 'dispatch;
	}
	// 823BB5A8: 397F0348  addi r11, r31, 0x348
	ctx.r[11].s64 = ctx.r[31].s64 + 840;
	// 823BB5AC: 3940000B  li r10, 0xb
	ctx.r[10].s64 = 11;
	// 823BB5B0: 93CB0000  stw r30, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[30].u32 ) };
	// 823BB5B4: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 823BB5B8: 93CB0004  stw r30, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[30].u32 ) };
	// 823BB5BC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 823BB5C0: 4080FFF0  bge 0x823bb5b0
	if !ctx.cr[0].lt {
	pc = 0x823BB5B0; continue 'dispatch;
	}
	// 823BB5C4: 93DF03A8  stw r30, 0x3a8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(936 as u32), ctx.r[30].u32 ) };
	// 823BB5C8: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823BB5CC: 93DF03AC  stw r30, 0x3ac(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(940 as u32), ctx.r[30].u32 ) };
	// 823BB5D0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BB5D4: 93DF03B0  stw r30, 0x3b0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(944 as u32), ctx.r[30].u32 ) };
	// 823BB5D8: 3B6B03C8  addi r27, r11, 0x3c8
	ctx.r[27].s64 = ctx.r[11].s64 + 968;
	// 823BB5DC: 93DF03B4  stw r30, 0x3b4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(948 as u32), ctx.r[30].u32 ) };
	// 823BB5E0: 38A0014D  li r5, 0x14d
	ctx.r[5].s64 = 333;
	// 823BB5E4: 93DF03B8  stw r30, 0x3b8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(952 as u32), ctx.r[30].u32 ) };
	// 823BB5E8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 823BB5EC: 93DF03BC  stw r30, 0x3bc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(956 as u32), ctx.r[30].u32 ) };
	// 823BB5F0: 38600088  li r3, 0x88
	ctx.r[3].s64 = 136;
	// 823BB5F4: 93DF03C0  stw r30, 0x3c0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(960 as u32), ctx.r[30].u32 ) };
	// 823BB5F8: 93DF03C4  stw r30, 0x3c4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(964 as u32), ctx.r[30].u32 ) };
	// 823BB5FC: 93DF03C8  stw r30, 0x3c8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(968 as u32), ctx.r[30].u32 ) };
	// 823BB600: 93DF03CC  stw r30, 0x3cc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(972 as u32), ctx.r[30].u32 ) };
	// 823BB604: 93DF03D4  stw r30, 0x3d4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(980 as u32), ctx.r[30].u32 ) };
	// 823BB608: 93DF03D8  stw r30, 0x3d8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(984 as u32), ctx.r[30].u32 ) };
	// 823BB60C: 93DF03DC  stw r30, 0x3dc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(988 as u32), ctx.r[30].u32 ) };
	// 823BB610: 93DF03E0  stw r30, 0x3e0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(992 as u32), ctx.r[30].u32 ) };
	// 823BB614: 48A36DD5  bl 0x82df23e8
	ctx.lr = 0x823BB618;
	sub_82DF23E8(ctx, base);
	// 823BB618: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BB61C: 41820010  beq 0x823bb62c
	if ctx.cr[0].eq {
	pc = 0x823BB62C; continue 'dispatch;
	}
	// 823BB620: 480C9B29  bl 0x82485148
	ctx.lr = 0x823BB624;
	sub_82485148(ctx, base);
	// 823BB624: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BB628: 48000008  b 0x823bb630
	pc = 0x823BB630; continue 'dispatch;
	// 823BB62C: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 823BB630: 93BF03E8  stw r29, 0x3e8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1000 as u32), ctx.r[29].u32 ) };
	// 823BB634: 397F03E8  addi r11, r31, 0x3e8
	ctx.r[11].s64 = ctx.r[31].s64 + 1000;
	// 823BB638: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823BB63C: 3B8B0004  addi r28, r11, 4
	ctx.r[28].s64 = ctx.r[11].s64 + 4;
	// 823BB640: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 823BB644: 4BFFA9E5  bl 0x823b6028
	ctx.lr = 0x823BB648;
	sub_823B6028(ctx, base);
	// 823BB648: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 823BB64C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823BB650: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 823BB654: 4BF049AD  bl 0x822c0000
	ctx.lr = 0x823BB658;
	sub_822C0000(ctx, base);
	// 823BB658: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 823BB65C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BB660: 38A0014B  li r5, 0x14b
	ctx.r[5].s64 = 331;
	// 823BB664: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 823BB668: 48A36D81  bl 0x82df23e8
	ctx.lr = 0x823BB66C;
	sub_82DF23E8(ctx, base);
	// 823BB66C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BB670: 41820010  beq 0x823bb680
	if ctx.cr[0].eq {
	pc = 0x823BB680; continue 'dispatch;
	}
	// 823BB674: 481CF625  bl 0x8258ac98
	ctx.lr = 0x823BB678;
	sub_8258AC98(ctx, base);
	// 823BB678: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BB67C: 48000008  b 0x823bb684
	pc = 0x823BB684; continue 'dispatch;
	// 823BB680: 7FDDF378  mr r29, r30
	ctx.r[29].u64 = ctx.r[30].u64;
	// 823BB684: 93BF03F0  stw r29, 0x3f0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1008 as u32), ctx.r[29].u32 ) };
	// 823BB688: 397F03F0  addi r11, r31, 0x3f0
	ctx.r[11].s64 = ctx.r[31].s64 + 1008;
	// 823BB68C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823BB690: 3B8B0004  addi r28, r11, 4
	ctx.r[28].s64 = ctx.r[11].s64 + 4;
	// 823BB694: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 823BB698: 4BF4B2F9  bl 0x82306990
	ctx.lr = 0x823BB69C;
	sub_82306990(ctx, base);
	// 823BB69C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 823BB6A0: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823BB6A4: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 823BB6A8: 4BF04959  bl 0x822c0000
	ctx.lr = 0x823BB6AC;
	sub_822C0000(ctx, base);
	// 823BB6AC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 823BB6B0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BB6B4: 38A0014C  li r5, 0x14c
	ctx.r[5].s64 = 332;
	// 823BB6B8: 38600100  li r3, 0x100
	ctx.r[3].s64 = 256;
	// 823BB6BC: 48A36D2D  bl 0x82df23e8
	ctx.lr = 0x823BB6C0;
	sub_82DF23E8(ctx, base);
	// 823BB6C0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BB6C4: 4182000C  beq 0x823bb6d0
	if ctx.cr[0].eq {
	pc = 0x823BB6D0; continue 'dispatch;
	}
	// 823BB6C8: 481CF5D1  bl 0x8258ac98
	ctx.lr = 0x823BB6CC;
	sub_8258AC98(ctx, base);
	// 823BB6CC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823BB6D0: 93DF03F8  stw r30, 0x3f8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(1016 as u32), ctx.r[30].u32 ) };
	// 823BB6D4: 397F03F8  addi r11, r31, 0x3f8
	ctx.r[11].s64 = ctx.r[31].s64 + 1016;
	// 823BB6D8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823BB6DC: 3BAB0004  addi r29, r11, 4
	ctx.r[29].s64 = ctx.r[11].s64 + 4;
	// 823BB6E0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BB6E4: 4BF4B2AD  bl 0x82306990
	ctx.lr = 0x823BB6E8;
	sub_82306990(ctx, base);
	// 823BB6E8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823BB6EC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823BB6F0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BB6F4: 4BF0490D  bl 0x822c0000
	ctx.lr = 0x823BB6F8;
	sub_822C0000(ctx, base);
	// 823BB6F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB6FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823BB700: 48DECAB4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BB708 size=76
    let mut pc: u32 = 0x823BB708;
    'dispatch: loop {
        match pc {
            0x823BB708 => {
    //   block [0x823BB708..0x823BB754)
	// 823BB708: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB70C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BB710: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BB714: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BB718: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BB71C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BB720: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BB724: 4BFFEB55  bl 0x823ba278
	ctx.lr = 0x823BB728;
	sub_823BA278(ctx, base);
	// 823BB728: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BB72C: 4182000C  beq 0x823bb738
	if ctx.cr[0].eq {
	pc = 0x823BB738; continue 'dispatch;
	}
	// 823BB730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB734: 48A36CA5  bl 0x82df23d8
	ctx.lr = 0x823BB738;
	sub_82DF23D8(ctx, base);
	// 823BB738: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BB73C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BB740: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BB744: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BB748: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BB74C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BB750: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BB758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x823BB758 size=3416
    let mut pc: u32 = 0x823BB758;
    'dispatch: loop {
        match pc {
            0x823BB758 => {
    //   block [0x823BB758..0x823BC4B0)
	// 823BB758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BB75C: 48DEC9E9  bl 0x831a8144
	ctx.lr = 0x823BB760;
	sub_831A8130(ctx, base);
	// 823BB760: 3981FF90  addi r12, r1, -0x70
	ctx.r[12].s64 = ctx.r[1].s64 + -112;
	// 823BB764: 48DED309  bl 0x831a8a6c
	ctx.lr = 0x823BB768;
	sub_831A8A40(ctx, base);
	// 823BB768: 3980FF40  li r12, -0xc0
	ctx.r[12].s64 = -192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BC4B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823BC4B0 size=508
    let mut pc: u32 = 0x823BC4B0;
    'dispatch: loop {
        match pc {
            0x823BC4B0 => {
    //   block [0x823BC4B0..0x823BC6AC)
	// 823BC4B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BC4B4: 48DEBCB1  bl 0x831a8164
	ctx.lr = 0x823BC4B8;
	sub_831A8130(ctx, base);
	// 823BC4B8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BC4BC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 823BC4C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BC4C4: 481CA9ED  bl 0x82586eb0
	ctx.lr = 0x823BC4C8;
	sub_82586EB0(ctx, base);
	// 823BC4C8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BC4CC: 897F0284  lbz r11, 0x284(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(644 as u32) ) } as u64;
	// 823BC4D0: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BC4D4: 408201D0  bne 0x823bc6a4
	if !ctx.cr[0].eq {
	pc = 0x823BC6A4; continue 'dispatch;
	}
	// 823BC4D8: 897F0710  lbz r11, 0x710(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(1808 as u32) ) } as u64;
	// 823BC4DC: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BC4E0: 4182006C  beq 0x823bc54c
	if ctx.cr[0].eq {
	pc = 0x823BC54C; continue 'dispatch;
	}
	// 823BC4E4: 4BFE507D  bl 0x823a1560
	ctx.lr = 0x823BC4E8;
	sub_823A1560(ctx, base);
	// 823BC4E8: 817E0030  lwz r11, 0x30(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(48 as u32) ) } as u64;
	// 823BC4EC: 7C6A07B4  extsw r10, r3
	ctx.r[10].s64 = ctx.r[3].s32 as i64;
	// 823BC4F0: F9410050  std r10, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u64 ) };
	// 823BC4F4: C8010050  lfd f0, 0x50(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[0].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823BC4F8: FD80069C  fcfid f12, f0
	ctx.f[12].f64 = (ctx.f[0].s64 as f64);
	// 823BC4FC: F9610050  std r11, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u64 ) };
	// 823BC500: C9A10050  lfd f13, 0x50(r1)
	ctx.f[13].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) };
	// 823BC504: FD606E9C  fcfid f11, f13
	ctx.f[11].f64 = (ctx.f[13].s64 as f64);
	// 823BC508: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823BC50C: FD806018  frsp f12, f12
	ctx.f[12].f64 = (ctx.f[12].f64 as f32) as f64;
	// 823BC510: C00B6218  lfs f0, 0x6218(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(25112 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823BC514: FD605818  frsp f11, f11
	ctx.f[11].f64 = (ctx.f[11].f64 as f32) as f64;
	// 823BC518: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BC51C: C1AB9528  lfs f13, -0x6ad8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-27352 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823BC520: ED8B603C  fnmsubs f12, f11, f0, f12
	ctx.f[12].f64 = -(((ctx.f[11].f64 * ctx.f[0].f64 - ctx.f[12].f64) as f32) as f64);
	// 823BC524: EDAC0372  fmuls f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 823BC528: EC0D0032  fmuls f0, f13, f0
	ctx.f[0].f64 = (((ctx.f[13].f64 * ctx.f[0].f64) as f32) as f64);
	// 823BC52C: FC00001E  fctiwz f0, f0
	ctx.f[0].s64 = if ctx.f[0].f64 > (i32::MAX as f64) { i32::MAX as i64 } else { ctx.f[0].f64.trunc() as i32 as i64 };
	// 823BC530: D8010050  stfd f0, 0x50(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.f[0].u64 ) };
	// 823BC534: 81610054  lwz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823BC538: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BC53C: 40980090  bge cr6, 0x823bc5cc
	if !ctx.cr[6].lt {
	pc = 0x823BC5CC; continue 'dispatch;
	}
	// 823BC540: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823BC544: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BC548: 4BFF0429  bl 0x823ac970
	ctx.lr = 0x823BC54C;
	sub_823AC970(ctx, base);
	// 823BC54C: 897F08C0  lbz r11, 0x8c0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2240 as u32) ) } as u64;
	// 823BC550: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BC554: 40820150  bne 0x823bc6a4
	if !ctx.cr[0].eq {
	pc = 0x823BC6A4; continue 'dispatch;
	}
	// 823BC558: 897F0B44  lbz r11, 0xb44(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(2884 as u32) ) } as u64;
	// 823BC55C: 280B0000  cmplwi r11, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BC560: 41820010  beq 0x823bc570
	if ctx.cr[0].eq {
	pc = 0x823BC570; continue 'dispatch;
	}
	// 823BC564: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823BC568: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BC56C: 4BFF2EB5  bl 0x823af420
	ctx.lr = 0x823BC570;
	sub_823AF420(ctx, base);
	// 823BC570: 3FA08326  lis r29, -0x7cda
	ctx.r[29].s64 = -2094661632;
	// 823BC574: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BC578: 809D7DDC  lwz r4, 0x7ddc(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(32220 as u32) ) } as u64;
	// 823BC57C: 48A3748D  bl 0x82df3a08
	ctx.lr = 0x823BC580;
	sub_82DF3A08(ctx, base);
	// 823BC580: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 823BC584: 3B610050  addi r27, r1, 0x50
	ctx.r[27].s64 = ctx.r[1].s64 + 80;
	// 823BC588: 4BF45249  bl 0x823017d0
	ctx.lr = 0x823BC58C;
	sub_823017D0(ctx, base);
	// 823BC58C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 823BC590: 48A36D79  bl 0x82df3308
	ctx.lr = 0x823BC594;
	sub_82DF3308(ctx, base);
	// 823BC594: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 823BC598: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BC59C: 48A36E8D  bl 0x82df3428
	ctx.lr = 0x823BC5A0;
	sub_82DF3428(ctx, base);
	// 823BC5A0: 576B063F  clrlwi. r11, r27, 0x18
	ctx.r[11].u64 = ctx.r[27].u32 as u64 & 0x000000FFu64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BC5A4: 418200B4  beq 0x823bc658
	if ctx.cr[0].eq {
	pc = 0x823BC658; continue 'dispatch;
	}
	// 823BC5A8: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 823BC5AC: C01E0018  lfs f0, 0x18(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823BC5B0: 394008B0  li r10, 0x8b0
	ctx.r[10].s64 = 2224;
	// 823BC5B4: D01F0894  stfs f0, 0x894(r31)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(2196 as u32), tmp.u32 ) };
	// 823BC5B8: 13FE58C7  vcmpequd (lvx128) v31, v30, v11
	tmp.u32 = ctx.r[30].u32 + ctx.r[11].u32;
	tmp.u32 &= !0xFu32;
	// load 16B at tmp.u32 into ctx.v[63] using VectorMaskL[(tmp.u32 & 0xF)]
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BC6B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BC6B0 size=216
    let mut pc: u32 = 0x823BC6B0;
    'dispatch: loop {
        match pc {
            0x823BC6B0 => {
    //   block [0x823BC6B0..0x823BC788)
	// 823BC6B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BC6B4: 48DEBAB5  bl 0x831a8168
	ctx.lr = 0x823BC6B8;
	sub_831A8130(ctx, base);
	// 823BC6B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BC6BC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 823BC6C0: 481CA7F1  bl 0x82586eb0
	ctx.lr = 0x823BC6C4;
	sub_82586EB0(ctx, base);
	// 823BC6C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BC6C8: 4BFEA1D9  bl 0x823a68a0
	ctx.lr = 0x823BC6CC;
	sub_823A68A0(ctx, base);
	// 823BC6CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BC6D0: 4BFEA151  bl 0x823a6820
	ctx.lr = 0x823BC6D4;
	sub_823A6820(ctx, base);
	// 823BC6D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BC6D8: 4BFEA331  bl 0x823a6a08
	ctx.lr = 0x823BC6DC;
	sub_823A6A08(ctx, base);
	// 823BC6DC: 807F0B58  lwz r3, 0xb58(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(2904 as u32) ) } as u64;
	// 823BC6E0: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 823BC6E4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 823BC6E8: 3BDF0B58  addi r30, r31, 0xb58
	ctx.r[30].s64 = ctx.r[31].s64 + 2904;
	// 823BC6EC: 9BBF08C1  stb r29, 0x8c1(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2241 as u32), ctx.r[29].u8 ) };
	// 823BC6F0: 9BBF0B43  stb r29, 0xb43(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2883 as u32), ctx.r[29].u8 ) };
	// 823BC6F4: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BC6F8: 997F0286  stb r11, 0x286(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(646 as u32), ctx.r[11].u8 ) };
	// 823BC6FC: 9BBF08C9  stb r29, 0x8c9(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2249 as u32), ctx.r[29].u8 ) };
	// 823BC700: 419A0068  beq cr6, 0x823bc768
	if ctx.cr[6].eq {
	pc = 0x823BC768; continue 'dispatch;
	}
	// 823BC704: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 823BC708: 48AC4F09  bl 0x82e81610
	ctx.lr = 0x823BC70C;
	sub_82E81610(ctx, base);
	// 823BC70C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BC710: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BC714: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BC718: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823BC71C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823BC720: 419A0024  beq cr6, 0x823bc744
	if ctx.cr[6].eq {
	pc = 0x823BC744; continue 'dispatch;
	}
	// 823BC724: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 823BC728: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823BC72C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823BC730: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823BC734: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 823BC738: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823BC73C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823BC740: 4082FFE8  bne 0x823bc728
	if !ctx.cr[0].eq {
	pc = 0x823BC728; continue 'dispatch;
	}
	// 823BC744: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823BC748: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BC74C: 4BF49D85  bl 0x823064d0
	ctx.lr = 0x823BC750;
	sub_823064D0(ctx, base);
	// 823BC750: 93BE0000  stw r29, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[29].u32 ) };
	// 823BC754: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BC758: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BC75C: 93BE0004  stw r29, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[29].u32 ) };
	// 823BC760: 419A0008  beq cr6, 0x823bc768
	if ctx.cr[6].eq {
	pc = 0x823BC768; continue 'dispatch;
	}
	// 823BC764: 4BF0412D  bl 0x822c0890
	ctx.lr = 0x823BC768;
	sub_822C0890(ctx, base);
	// 823BC768: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BC76C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 823BC770: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 823BC774: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 823BC778: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823BC77C: 4E800421  bctrl
	ctx.lr = 0x823BC780;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823BC780: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823BC784: 48DEBA34  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BC788(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BC788 size=112
    let mut pc: u32 = 0x823BC788;
    'dispatch: loop {
        match pc {
            0x823BC788 => {
    //   block [0x823BC788..0x823BC7F8)
	// 823BC788: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BC78C: 48DEB9E1  bl 0x831a816c
	ctx.lr = 0x823BC790;
	sub_831A8130(ctx, base);
	// 823BC790: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BC794: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BC798: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BC79C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BC7A0: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BC7A4: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BC7A8: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 823BC7AC: 48A35C3D  bl 0x82df23e8
	ctx.lr = 0x823BC7B0;
	sub_82DF23E8(ctx, base);
	// 823BC7B0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BC7B4: 41820010  beq 0x823bc7c4
	if ctx.cr[0].eq {
	pc = 0x823BC7C4; continue 'dispatch;
	}
	// 823BC7B8: 4BFFE139  bl 0x823ba8f0
	ctx.lr = 0x823BC7BC;
	sub_823BA8F0(ctx, base);
	// 823BC7BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BC7C0: 48000008  b 0x823bc7c8
	pc = 0x823BC7C8; continue 'dispatch;
	// 823BC7C4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BC7C8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BC7CC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BC7D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BC7D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BC7D8: 4BFF9B59  bl 0x823b6330
	ctx.lr = 0x823BC7DC;
	sub_823B6330(ctx, base);
	// 823BC7DC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BC7E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BC7E4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BC7E8: 4BF03819  bl 0x822c0000
	ctx.lr = 0x823BC7EC;
	sub_822C0000(ctx, base);
	// 823BC7EC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BC7F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BC7F4: 48DEB9C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BC7F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BC7F8 size=112
    let mut pc: u32 = 0x823BC7F8;
    'dispatch: loop {
        match pc {
            0x823BC7F8 => {
    //   block [0x823BC7F8..0x823BC868)
	// 823BC7F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BC7FC: 48DEB971  bl 0x831a816c
	ctx.lr = 0x823BC800;
	sub_831A8130(ctx, base);
	// 823BC800: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BC804: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BC808: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BC80C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BC810: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BC814: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BC818: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BC81C: 48A35BCD  bl 0x82df23e8
	ctx.lr = 0x823BC820;
	sub_82DF23E8(ctx, base);
	// 823BC820: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BC824: 41820010  beq 0x823bc834
	if ctx.cr[0].eq {
	pc = 0x823BC834; continue 'dispatch;
	}
	// 823BC828: 4BFFE119  bl 0x823ba940
	ctx.lr = 0x823BC82C;
	sub_823BA940(ctx, base);
	// 823BC82C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BC830: 48000008  b 0x823bc838
	pc = 0x823BC838; continue 'dispatch;
	// 823BC834: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BC838: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BC83C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BC840: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BC844: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BC848: 4BFF9BB1  bl 0x823b63f8
	ctx.lr = 0x823BC84C;
	sub_823B63F8(ctx, base);
	// 823BC84C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BC850: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BC854: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BC858: 4BF037A9  bl 0x822c0000
	ctx.lr = 0x823BC85C;
	sub_822C0000(ctx, base);
	// 823BC85C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BC860: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BC864: 48DEB958  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BC868(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BC868 size=136
    let mut pc: u32 = 0x823BC868;
    'dispatch: loop {
        match pc {
            0x823BC868 => {
    //   block [0x823BC868..0x823BC8F0)
	// 823BC868: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BC86C: 48DEB901  bl 0x831a816c
	ctx.lr = 0x823BC870;
	sub_831A8130(ctx, base);
	// 823BC870: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BC874: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BC878: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BC87C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BC880: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BC884: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BC888: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 823BC88C: 48A35B5D  bl 0x82df23e8
	ctx.lr = 0x823BC890;
	sub_82DF23E8(ctx, base);
	// 823BC890: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BC894: 41820028  beq 0x823bc8bc
	if ctx.cr[0].eq {
	pc = 0x823BC8BC; continue 'dispatch;
	}
	// 823BC898: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BC89C: 48AA008D  bl 0x82e5c928
	ctx.lr = 0x823BC8A0;
	sub_82E5C928(ctx, base);
	// 823BC8A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BC8A4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BC8A8: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BC8AC: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BC8B0: 396AF904  addi r11, r10, -0x6fc
	ctx.r[11].s64 = ctx.r[10].s64 + -1788;
	// 823BC8B4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BC8B8: 48000008  b 0x823bc8c0
	pc = 0x823BC8C0; continue 'dispatch;
	// 823BC8BC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BC8C0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BC8C4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BC8C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BC8CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BC8D0: 4BFF9BF1  bl 0x823b64c0
	ctx.lr = 0x823BC8D4;
	sub_823B64C0(ctx, base);
	// 823BC8D4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BC8D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BC8DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BC8E0: 4BF03721  bl 0x822c0000
	ctx.lr = 0x823BC8E4;
	sub_822C0000(ctx, base);
	// 823BC8E4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BC8E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BC8EC: 48DEB8D0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BC8F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BC8F0 size=112
    let mut pc: u32 = 0x823BC8F0;
    'dispatch: loop {
        match pc {
            0x823BC8F0 => {
    //   block [0x823BC8F0..0x823BC960)
	// 823BC8F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BC8F4: 48DEB879  bl 0x831a816c
	ctx.lr = 0x823BC8F8;
	sub_831A8130(ctx, base);
	// 823BC8F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BC8FC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BC900: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BC904: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BC908: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BC90C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BC910: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 823BC914: 48A35AD5  bl 0x82df23e8
	ctx.lr = 0x823BC918;
	sub_82DF23E8(ctx, base);
	// 823BC918: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BC91C: 41820010  beq 0x823bc92c
	if ctx.cr[0].eq {
	pc = 0x823BC92C; continue 'dispatch;
	}
	// 823BC920: 4BFFE071  bl 0x823ba990
	ctx.lr = 0x823BC924;
	sub_823BA990(ctx, base);
	// 823BC924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BC928: 48000008  b 0x823bc930
	pc = 0x823BC930; continue 'dispatch;
	// 823BC92C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BC930: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BC934: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BC938: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BC93C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BC940: 4BFF9C49  bl 0x823b6588
	ctx.lr = 0x823BC944;
	sub_823B6588(ctx, base);
	// 823BC944: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BC948: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BC94C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BC950: 4BF036B1  bl 0x822c0000
	ctx.lr = 0x823BC954;
	sub_822C0000(ctx, base);
	// 823BC954: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BC958: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BC95C: 48DEB860  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BC960(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BC960 size=136
    let mut pc: u32 = 0x823BC960;
    'dispatch: loop {
        match pc {
            0x823BC960 => {
    //   block [0x823BC960..0x823BC9E8)
	// 823BC960: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BC964: 48DEB809  bl 0x831a816c
	ctx.lr = 0x823BC968;
	sub_831A8130(ctx, base);
	// 823BC968: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BC96C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BC970: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BC974: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BC978: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BC97C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BC980: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BC984: 48A35A65  bl 0x82df23e8
	ctx.lr = 0x823BC988;
	sub_82DF23E8(ctx, base);
	// 823BC988: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BC98C: 41820028  beq 0x823bc9b4
	if ctx.cr[0].eq {
	pc = 0x823BC9B4; continue 'dispatch;
	}
	// 823BC990: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BC994: 48A9FF95  bl 0x82e5c928
	ctx.lr = 0x823BC998;
	sub_82E5C928(ctx, base);
	// 823BC998: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BC99C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BC9A0: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BC9A4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BC9A8: 396AF954  addi r11, r10, -0x6ac
	ctx.r[11].s64 = ctx.r[10].s64 + -1708;
	// 823BC9AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BC9B0: 48000008  b 0x823bc9b8
	pc = 0x823BC9B8; continue 'dispatch;
	// 823BC9B4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BC9B8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BC9BC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BC9C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BC9C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BC9C8: 4BFF9C89  bl 0x823b6650
	ctx.lr = 0x823BC9CC;
	sub_823B6650(ctx, base);
	// 823BC9CC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BC9D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BC9D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BC9D8: 4BF03629  bl 0x822c0000
	ctx.lr = 0x823BC9DC;
	sub_822C0000(ctx, base);
	// 823BC9DC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BC9E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BC9E4: 48DEB7D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BC9E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BC9E8 size=136
    let mut pc: u32 = 0x823BC9E8;
    'dispatch: loop {
        match pc {
            0x823BC9E8 => {
    //   block [0x823BC9E8..0x823BCA70)
	// 823BC9E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BC9EC: 48DEB781  bl 0x831a816c
	ctx.lr = 0x823BC9F0;
	sub_831A8130(ctx, base);
	// 823BC9F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BC9F4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BC9F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BC9FC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BCA00: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BCA04: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BCA08: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BCA0C: 48A359DD  bl 0x82df23e8
	ctx.lr = 0x823BCA10;
	sub_82DF23E8(ctx, base);
	// 823BCA10: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BCA14: 41820028  beq 0x823bca3c
	if ctx.cr[0].eq {
	pc = 0x823BCA3C; continue 'dispatch;
	}
	// 823BCA18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BCA1C: 48A9FF0D  bl 0x82e5c928
	ctx.lr = 0x823BCA20;
	sub_82E5C928(ctx, base);
	// 823BCA20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BCA24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BCA28: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BCA2C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BCA30: 396AF97C  addi r11, r10, -0x684
	ctx.r[11].s64 = ctx.r[10].s64 + -1668;
	// 823BCA34: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BCA38: 48000008  b 0x823bca40
	pc = 0x823BCA40; continue 'dispatch;
	// 823BCA3C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BCA40: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BCA44: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BCA48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCA4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCA50: 4BFF9CC9  bl 0x823b6718
	ctx.lr = 0x823BCA54;
	sub_823B6718(ctx, base);
	// 823BCA54: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BCA58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCA5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCA60: 4BF035A1  bl 0x822c0000
	ctx.lr = 0x823BCA64;
	sub_822C0000(ctx, base);
	// 823BCA64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BCA68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BCA6C: 48DEB750  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BCA70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BCA70 size=136
    let mut pc: u32 = 0x823BCA70;
    'dispatch: loop {
        match pc {
            0x823BCA70 => {
    //   block [0x823BCA70..0x823BCAF8)
	// 823BCA70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BCA74: 48DEB6F9  bl 0x831a816c
	ctx.lr = 0x823BCA78;
	sub_831A8130(ctx, base);
	// 823BCA78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BCA7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BCA80: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BCA84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BCA88: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BCA8C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BCA90: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 823BCA94: 48A35955  bl 0x82df23e8
	ctx.lr = 0x823BCA98;
	sub_82DF23E8(ctx, base);
	// 823BCA98: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BCA9C: 41820028  beq 0x823bcac4
	if ctx.cr[0].eq {
	pc = 0x823BCAC4; continue 'dispatch;
	}
	// 823BCAA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BCAA4: 48A9FE85  bl 0x82e5c928
	ctx.lr = 0x823BCAA8;
	sub_82E5C928(ctx, base);
	// 823BCAA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BCAAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BCAB0: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BCAB4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BCAB8: 396AF9A4  addi r11, r10, -0x65c
	ctx.r[11].s64 = ctx.r[10].s64 + -1628;
	// 823BCABC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BCAC0: 48000008  b 0x823bcac8
	pc = 0x823BCAC8; continue 'dispatch;
	// 823BCAC4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BCAC8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BCACC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BCAD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCAD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCAD8: 4BFF9D09  bl 0x823b67e0
	ctx.lr = 0x823BCADC;
	sub_823B67E0(ctx, base);
	// 823BCADC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BCAE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCAE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCAE8: 4BF03519  bl 0x822c0000
	ctx.lr = 0x823BCAEC;
	sub_822C0000(ctx, base);
	// 823BCAEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BCAF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BCAF4: 48DEB6C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BCAF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BCAF8 size=136
    let mut pc: u32 = 0x823BCAF8;
    'dispatch: loop {
        match pc {
            0x823BCAF8 => {
    //   block [0x823BCAF8..0x823BCB80)
	// 823BCAF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BCAFC: 48DEB671  bl 0x831a816c
	ctx.lr = 0x823BCB00;
	sub_831A8130(ctx, base);
	// 823BCB00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BCB04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BCB08: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BCB0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BCB10: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BCB14: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BCB18: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BCB1C: 48A358CD  bl 0x82df23e8
	ctx.lr = 0x823BCB20;
	sub_82DF23E8(ctx, base);
	// 823BCB20: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BCB24: 41820028  beq 0x823bcb4c
	if ctx.cr[0].eq {
	pc = 0x823BCB4C; continue 'dispatch;
	}
	// 823BCB28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BCB2C: 48A9FDFD  bl 0x82e5c928
	ctx.lr = 0x823BCB30;
	sub_82E5C928(ctx, base);
	// 823BCB30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BCB34: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BCB38: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BCB3C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BCB40: 396AF9CC  addi r11, r10, -0x634
	ctx.r[11].s64 = ctx.r[10].s64 + -1588;
	// 823BCB44: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BCB48: 48000008  b 0x823bcb50
	pc = 0x823BCB50; continue 'dispatch;
	// 823BCB4C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BCB50: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BCB54: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BCB58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCB5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCB60: 4BFF9D49  bl 0x823b68a8
	ctx.lr = 0x823BCB64;
	sub_823B68A8(ctx, base);
	// 823BCB64: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BCB68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCB6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCB70: 4BF03491  bl 0x822c0000
	ctx.lr = 0x823BCB74;
	sub_822C0000(ctx, base);
	// 823BCB74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BCB78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BCB7C: 48DEB640  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BCB80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BCB80 size=136
    let mut pc: u32 = 0x823BCB80;
    'dispatch: loop {
        match pc {
            0x823BCB80 => {
    //   block [0x823BCB80..0x823BCC08)
	// 823BCB80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BCB84: 48DEB5E9  bl 0x831a816c
	ctx.lr = 0x823BCB88;
	sub_831A8130(ctx, base);
	// 823BCB88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BCB8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BCB90: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BCB94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BCB98: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BCB9C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BCBA0: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BCBA4: 48A35845  bl 0x82df23e8
	ctx.lr = 0x823BCBA8;
	sub_82DF23E8(ctx, base);
	// 823BCBA8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BCBAC: 41820028  beq 0x823bcbd4
	if ctx.cr[0].eq {
	pc = 0x823BCBD4; continue 'dispatch;
	}
	// 823BCBB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BCBB4: 48A9FD75  bl 0x82e5c928
	ctx.lr = 0x823BCBB8;
	sub_82E5C928(ctx, base);
	// 823BCBB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BCBBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BCBC0: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BCBC4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BCBC8: 396AF9F4  addi r11, r10, -0x60c
	ctx.r[11].s64 = ctx.r[10].s64 + -1548;
	// 823BCBCC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BCBD0: 48000008  b 0x823bcbd8
	pc = 0x823BCBD8; continue 'dispatch;
	// 823BCBD4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BCBD8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BCBDC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BCBE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCBE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCBE8: 4BFF9D89  bl 0x823b6970
	ctx.lr = 0x823BCBEC;
	sub_823B6970(ctx, base);
	// 823BCBEC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BCBF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCBF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCBF8: 4BF03409  bl 0x822c0000
	ctx.lr = 0x823BCBFC;
	sub_822C0000(ctx, base);
	// 823BCBFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BCC00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BCC04: 48DEB5B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BCC08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BCC08 size=136
    let mut pc: u32 = 0x823BCC08;
    'dispatch: loop {
        match pc {
            0x823BCC08 => {
    //   block [0x823BCC08..0x823BCC90)
	// 823BCC08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BCC0C: 48DEB561  bl 0x831a816c
	ctx.lr = 0x823BCC10;
	sub_831A8130(ctx, base);
	// 823BCC10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BCC14: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BCC18: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BCC1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BCC20: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BCC24: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BCC28: 38600088  li r3, 0x88
	ctx.r[3].s64 = 136;
	// 823BCC2C: 48A357BD  bl 0x82df23e8
	ctx.lr = 0x823BCC30;
	sub_82DF23E8(ctx, base);
	// 823BCC30: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BCC34: 41820028  beq 0x823bcc5c
	if ctx.cr[0].eq {
	pc = 0x823BCC5C; continue 'dispatch;
	}
	// 823BCC38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BCC3C: 48A9FCED  bl 0x82e5c928
	ctx.lr = 0x823BCC40;
	sub_82E5C928(ctx, base);
	// 823BCC40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BCC44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BCC48: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BCC4C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BCC50: 396AFA1C  addi r11, r10, -0x5e4
	ctx.r[11].s64 = ctx.r[10].s64 + -1508;
	// 823BCC54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BCC58: 48000008  b 0x823bcc60
	pc = 0x823BCC60; continue 'dispatch;
	// 823BCC5C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BCC60: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BCC64: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BCC68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCC6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCC70: 4BFF9DC9  bl 0x823b6a38
	ctx.lr = 0x823BCC74;
	sub_823B6A38(ctx, base);
	// 823BCC74: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BCC78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCC7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCC80: 4BF03381  bl 0x822c0000
	ctx.lr = 0x823BCC84;
	sub_822C0000(ctx, base);
	// 823BCC84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BCC88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BCC8C: 48DEB530  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BCC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BCC90 size=136
    let mut pc: u32 = 0x823BCC90;
    'dispatch: loop {
        match pc {
            0x823BCC90 => {
    //   block [0x823BCC90..0x823BCD18)
	// 823BCC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BCC94: 48DEB4D9  bl 0x831a816c
	ctx.lr = 0x823BCC98;
	sub_831A8130(ctx, base);
	// 823BCC98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BCC9C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BCCA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BCCA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BCCA8: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BCCAC: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BCCB0: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 823BCCB4: 48A35735  bl 0x82df23e8
	ctx.lr = 0x823BCCB8;
	sub_82DF23E8(ctx, base);
	// 823BCCB8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BCCBC: 41820028  beq 0x823bcce4
	if ctx.cr[0].eq {
	pc = 0x823BCCE4; continue 'dispatch;
	}
	// 823BCCC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BCCC4: 48A9FC65  bl 0x82e5c928
	ctx.lr = 0x823BCCC8;
	sub_82E5C928(ctx, base);
	// 823BCCC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BCCCC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BCCD0: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BCCD4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BCCD8: 396AFA44  addi r11, r10, -0x5bc
	ctx.r[11].s64 = ctx.r[10].s64 + -1468;
	// 823BCCDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BCCE0: 48000008  b 0x823bcce8
	pc = 0x823BCCE8; continue 'dispatch;
	// 823BCCE4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BCCE8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BCCEC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BCCF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCCF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCCF8: 4BFF9E09  bl 0x823b6b00
	ctx.lr = 0x823BCCFC;
	sub_823B6B00(ctx, base);
	// 823BCCFC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BCD00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCD04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCD08: 4BF032F9  bl 0x822c0000
	ctx.lr = 0x823BCD0C;
	sub_822C0000(ctx, base);
	// 823BCD0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BCD10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BCD14: 48DEB4A8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BCD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BCD18 size=136
    let mut pc: u32 = 0x823BCD18;
    'dispatch: loop {
        match pc {
            0x823BCD18 => {
    //   block [0x823BCD18..0x823BCDA0)
	// 823BCD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BCD1C: 48DEB451  bl 0x831a816c
	ctx.lr = 0x823BCD20;
	sub_831A8130(ctx, base);
	// 823BCD20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BCD24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BCD28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BCD2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BCD30: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BCD34: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BCD38: 38600078  li r3, 0x78
	ctx.r[3].s64 = 120;
	// 823BCD3C: 48A356AD  bl 0x82df23e8
	ctx.lr = 0x823BCD40;
	sub_82DF23E8(ctx, base);
	// 823BCD40: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BCD44: 41820028  beq 0x823bcd6c
	if ctx.cr[0].eq {
	pc = 0x823BCD6C; continue 'dispatch;
	}
	// 823BCD48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BCD4C: 48A9FBDD  bl 0x82e5c928
	ctx.lr = 0x823BCD50;
	sub_82E5C928(ctx, base);
	// 823BCD50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BCD54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BCD58: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BCD5C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BCD60: 396AFA6C  addi r11, r10, -0x594
	ctx.r[11].s64 = ctx.r[10].s64 + -1428;
	// 823BCD64: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BCD68: 48000008  b 0x823bcd70
	pc = 0x823BCD70; continue 'dispatch;
	// 823BCD6C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BCD70: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BCD74: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BCD78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCD7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCD80: 4BFF9E49  bl 0x823b6bc8
	ctx.lr = 0x823BCD84;
	sub_823B6BC8(ctx, base);
	// 823BCD84: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BCD88: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCD8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCD90: 4BF03271  bl 0x822c0000
	ctx.lr = 0x823BCD94;
	sub_822C0000(ctx, base);
	// 823BCD94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BCD98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BCD9C: 48DEB420  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BCDA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BCDA0 size=112
    let mut pc: u32 = 0x823BCDA0;
    'dispatch: loop {
        match pc {
            0x823BCDA0 => {
    //   block [0x823BCDA0..0x823BCE10)
	// 823BCDA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BCDA4: 48DEB3C9  bl 0x831a816c
	ctx.lr = 0x823BCDA8;
	sub_831A8130(ctx, base);
	// 823BCDA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BCDAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BCDB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BCDB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BCDB8: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BCDBC: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BCDC0: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BCDC4: 48A35625  bl 0x82df23e8
	ctx.lr = 0x823BCDC8;
	sub_82DF23E8(ctx, base);
	// 823BCDC8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BCDCC: 41820010  beq 0x823bcddc
	if ctx.cr[0].eq {
	pc = 0x823BCDDC; continue 'dispatch;
	}
	// 823BCDD0: 4BFFDC21  bl 0x823ba9f0
	ctx.lr = 0x823BCDD4;
	sub_823BA9F0(ctx, base);
	// 823BCDD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BCDD8: 48000008  b 0x823bcde0
	pc = 0x823BCDE0; continue 'dispatch;
	// 823BCDDC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BCDE0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BCDE4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BCDE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCDEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCDF0: 4BFF9EA1  bl 0x823b6c90
	ctx.lr = 0x823BCDF4;
	sub_823B6C90(ctx, base);
	// 823BCDF4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BCDF8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCDFC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCE00: 4BF03201  bl 0x822c0000
	ctx.lr = 0x823BCE04;
	sub_822C0000(ctx, base);
	// 823BCE04: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BCE08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BCE0C: 48DEB3B0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BCE10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BCE10 size=112
    let mut pc: u32 = 0x823BCE10;
    'dispatch: loop {
        match pc {
            0x823BCE10 => {
    //   block [0x823BCE10..0x823BCE80)
	// 823BCE10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BCE14: 48DEB359  bl 0x831a816c
	ctx.lr = 0x823BCE18;
	sub_831A8130(ctx, base);
	// 823BCE18: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BCE1C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BCE20: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BCE24: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BCE28: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BCE2C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BCE30: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BCE34: 48A355B5  bl 0x82df23e8
	ctx.lr = 0x823BCE38;
	sub_82DF23E8(ctx, base);
	// 823BCE38: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BCE3C: 41820010  beq 0x823bce4c
	if ctx.cr[0].eq {
	pc = 0x823BCE4C; continue 'dispatch;
	}
	// 823BCE40: 4BFFDC01  bl 0x823baa40
	ctx.lr = 0x823BCE44;
	sub_823BAA40(ctx, base);
	// 823BCE44: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BCE48: 48000008  b 0x823bce50
	pc = 0x823BCE50; continue 'dispatch;
	// 823BCE4C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BCE50: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BCE54: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BCE58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCE5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCE60: 4BFF9EF9  bl 0x823b6d58
	ctx.lr = 0x823BCE64;
	sub_823B6D58(ctx, base);
	// 823BCE64: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BCE68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCE6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCE70: 4BF03191  bl 0x822c0000
	ctx.lr = 0x823BCE74;
	sub_822C0000(ctx, base);
	// 823BCE74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BCE78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BCE7C: 48DEB340  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BCE80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BCE80 size=136
    let mut pc: u32 = 0x823BCE80;
    'dispatch: loop {
        match pc {
            0x823BCE80 => {
    //   block [0x823BCE80..0x823BCF08)
	// 823BCE80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BCE84: 48DEB2E9  bl 0x831a816c
	ctx.lr = 0x823BCE88;
	sub_831A8130(ctx, base);
	// 823BCE88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BCE8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BCE90: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BCE94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BCE98: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BCE9C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BCEA0: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BCEA4: 48A35545  bl 0x82df23e8
	ctx.lr = 0x823BCEA8;
	sub_82DF23E8(ctx, base);
	// 823BCEA8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BCEAC: 41820028  beq 0x823bced4
	if ctx.cr[0].eq {
	pc = 0x823BCED4; continue 'dispatch;
	}
	// 823BCEB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BCEB4: 48A9FA75  bl 0x82e5c928
	ctx.lr = 0x823BCEB8;
	sub_82E5C928(ctx, base);
	// 823BCEB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BCEBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BCEC0: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BCEC4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BCEC8: 396AFAE4  addi r11, r10, -0x51c
	ctx.r[11].s64 = ctx.r[10].s64 + -1308;
	// 823BCECC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BCED0: 48000008  b 0x823bced8
	pc = 0x823BCED8; continue 'dispatch;
	// 823BCED4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BCED8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BCEDC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BCEE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCEE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCEE8: 4BFF9F39  bl 0x823b6e20
	ctx.lr = 0x823BCEEC;
	sub_823B6E20(ctx, base);
	// 823BCEEC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BCEF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCEF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCEF8: 4BF03109  bl 0x822c0000
	ctx.lr = 0x823BCEFC;
	sub_822C0000(ctx, base);
	// 823BCEFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BCF00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BCF04: 48DEB2B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BCF08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BCF08 size=136
    let mut pc: u32 = 0x823BCF08;
    'dispatch: loop {
        match pc {
            0x823BCF08 => {
    //   block [0x823BCF08..0x823BCF90)
	// 823BCF08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BCF0C: 48DEB261  bl 0x831a816c
	ctx.lr = 0x823BCF10;
	sub_831A8130(ctx, base);
	// 823BCF10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BCF14: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BCF18: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BCF1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BCF20: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BCF24: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BCF28: 38600078  li r3, 0x78
	ctx.r[3].s64 = 120;
	// 823BCF2C: 48A354BD  bl 0x82df23e8
	ctx.lr = 0x823BCF30;
	sub_82DF23E8(ctx, base);
	// 823BCF30: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BCF34: 41820028  beq 0x823bcf5c
	if ctx.cr[0].eq {
	pc = 0x823BCF5C; continue 'dispatch;
	}
	// 823BCF38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BCF3C: 48A9F9ED  bl 0x82e5c928
	ctx.lr = 0x823BCF40;
	sub_82E5C928(ctx, base);
	// 823BCF40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BCF44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BCF48: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BCF4C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BCF50: 396AFB0C  addi r11, r10, -0x4f4
	ctx.r[11].s64 = ctx.r[10].s64 + -1268;
	// 823BCF54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BCF58: 48000008  b 0x823bcf60
	pc = 0x823BCF60; continue 'dispatch;
	// 823BCF5C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BCF60: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BCF64: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BCF68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCF6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCF70: 4BFF9F79  bl 0x823b6ee8
	ctx.lr = 0x823BCF74;
	sub_823B6EE8(ctx, base);
	// 823BCF74: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BCF78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCF7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCF80: 4BF03081  bl 0x822c0000
	ctx.lr = 0x823BCF84;
	sub_822C0000(ctx, base);
	// 823BCF84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BCF88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BCF8C: 48DEB230  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BCF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BCF90 size=112
    let mut pc: u32 = 0x823BCF90;
    'dispatch: loop {
        match pc {
            0x823BCF90 => {
    //   block [0x823BCF90..0x823BD000)
	// 823BCF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BCF94: 48DEB1D9  bl 0x831a816c
	ctx.lr = 0x823BCF98;
	sub_831A8130(ctx, base);
	// 823BCF98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BCF9C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BCFA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BCFA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BCFA8: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BCFAC: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BCFB0: 3860007C  li r3, 0x7c
	ctx.r[3].s64 = 124;
	// 823BCFB4: 48A35435  bl 0x82df23e8
	ctx.lr = 0x823BCFB8;
	sub_82DF23E8(ctx, base);
	// 823BCFB8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BCFBC: 41820010  beq 0x823bcfcc
	if ctx.cr[0].eq {
	pc = 0x823BCFCC; continue 'dispatch;
	}
	// 823BCFC0: 4BFFDAD1  bl 0x823baa90
	ctx.lr = 0x823BCFC4;
	sub_823BAA90(ctx, base);
	// 823BCFC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BCFC8: 48000008  b 0x823bcfd0
	pc = 0x823BCFD0; continue 'dispatch;
	// 823BCFCC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BCFD0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BCFD4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BCFD8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCFDC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCFE0: 4BFF9FD1  bl 0x823b6fb0
	ctx.lr = 0x823BCFE4;
	sub_823B6FB0(ctx, base);
	// 823BCFE4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BCFE8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BCFEC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BCFF0: 4BF03011  bl 0x822c0000
	ctx.lr = 0x823BCFF4;
	sub_822C0000(ctx, base);
	// 823BCFF4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BCFF8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BCFFC: 48DEB1C0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD000 size=112
    let mut pc: u32 = 0x823BD000;
    'dispatch: loop {
        match pc {
            0x823BD000 => {
    //   block [0x823BD000..0x823BD070)
	// 823BD000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD004: 48DEB169  bl 0x831a816c
	ctx.lr = 0x823BD008;
	sub_831A8130(ctx, base);
	// 823BD008: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD00C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD010: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD014: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD018: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD01C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD020: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BD024: 48A353C5  bl 0x82df23e8
	ctx.lr = 0x823BD028;
	sub_82DF23E8(ctx, base);
	// 823BD028: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BD02C: 41820010  beq 0x823bd03c
	if ctx.cr[0].eq {
	pc = 0x823BD03C; continue 'dispatch;
	}
	// 823BD030: 4BFFDAC1  bl 0x823baaf0
	ctx.lr = 0x823BD034;
	sub_823BAAF0(ctx, base);
	// 823BD034: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BD038: 48000008  b 0x823bd040
	pc = 0x823BD040; continue 'dispatch;
	// 823BD03C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD040: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD044: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD048: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD04C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD050: 4BFFA029  bl 0x823b7078
	ctx.lr = 0x823BD054;
	sub_823B7078(ctx, base);
	// 823BD054: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD058: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD05C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD060: 4BF02FA1  bl 0x822c0000
	ctx.lr = 0x823BD064;
	sub_822C0000(ctx, base);
	// 823BD064: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD068: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD06C: 48DEB150  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD070 size=112
    let mut pc: u32 = 0x823BD070;
    'dispatch: loop {
        match pc {
            0x823BD070 => {
    //   block [0x823BD070..0x823BD0E0)
	// 823BD070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD074: 48DEB0F9  bl 0x831a816c
	ctx.lr = 0x823BD078;
	sub_831A8130(ctx, base);
	// 823BD078: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD07C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD080: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD084: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD088: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD08C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD090: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 823BD094: 48A35355  bl 0x82df23e8
	ctx.lr = 0x823BD098;
	sub_82DF23E8(ctx, base);
	// 823BD098: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BD09C: 41820010  beq 0x823bd0ac
	if ctx.cr[0].eq {
	pc = 0x823BD0AC; continue 'dispatch;
	}
	// 823BD0A0: 4BFFDAA1  bl 0x823bab40
	ctx.lr = 0x823BD0A4;
	sub_823BAB40(ctx, base);
	// 823BD0A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BD0A8: 48000008  b 0x823bd0b0
	pc = 0x823BD0B0; continue 'dispatch;
	// 823BD0AC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD0B0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD0B4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD0B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD0BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD0C0: 4BFFA081  bl 0x823b7140
	ctx.lr = 0x823BD0C4;
	sub_823B7140(ctx, base);
	// 823BD0C4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD0C8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD0CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD0D0: 4BF02F31  bl 0x822c0000
	ctx.lr = 0x823BD0D4;
	sub_822C0000(ctx, base);
	// 823BD0D4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD0D8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD0DC: 48DEB0E0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD0E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD0E0 size=112
    let mut pc: u32 = 0x823BD0E0;
    'dispatch: loop {
        match pc {
            0x823BD0E0 => {
    //   block [0x823BD0E0..0x823BD150)
	// 823BD0E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD0E4: 48DEB089  bl 0x831a816c
	ctx.lr = 0x823BD0E8;
	sub_831A8130(ctx, base);
	// 823BD0E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD0EC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD0F0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD0F4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD0F8: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD0FC: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD100: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 823BD104: 48A352E5  bl 0x82df23e8
	ctx.lr = 0x823BD108;
	sub_82DF23E8(ctx, base);
	// 823BD108: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BD10C: 41820010  beq 0x823bd11c
	if ctx.cr[0].eq {
	pc = 0x823BD11C; continue 'dispatch;
	}
	// 823BD110: 4BFFDA81  bl 0x823bab90
	ctx.lr = 0x823BD114;
	sub_823BAB90(ctx, base);
	// 823BD114: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BD118: 48000008  b 0x823bd120
	pc = 0x823BD120; continue 'dispatch;
	// 823BD11C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD120: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD124: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD128: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD12C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD130: 4BFFA0D9  bl 0x823b7208
	ctx.lr = 0x823BD134;
	sub_823B7208(ctx, base);
	// 823BD134: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD138: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD13C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD140: 4BF02EC1  bl 0x822c0000
	ctx.lr = 0x823BD144;
	sub_822C0000(ctx, base);
	// 823BD144: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD148: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD14C: 48DEB070  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD150 size=112
    let mut pc: u32 = 0x823BD150;
    'dispatch: loop {
        match pc {
            0x823BD150 => {
    //   block [0x823BD150..0x823BD1C0)
	// 823BD150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD154: 48DEB019  bl 0x831a816c
	ctx.lr = 0x823BD158;
	sub_831A8130(ctx, base);
	// 823BD158: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD15C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD160: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD164: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD168: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD16C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD170: 38600400  li r3, 0x400
	ctx.r[3].s64 = 1024;
	// 823BD174: 48A35275  bl 0x82df23e8
	ctx.lr = 0x823BD178;
	sub_82DF23E8(ctx, base);
	// 823BD178: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BD17C: 41820010  beq 0x823bd18c
	if ctx.cr[0].eq {
	pc = 0x823BD18C; continue 'dispatch;
	}
	// 823BD180: 4BFFDA61  bl 0x823babe0
	ctx.lr = 0x823BD184;
	sub_823BABE0(ctx, base);
	// 823BD184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BD188: 48000008  b 0x823bd190
	pc = 0x823BD190; continue 'dispatch;
	// 823BD18C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD190: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD194: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD198: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD19C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD1A0: 4BFFA131  bl 0x823b72d0
	ctx.lr = 0x823BD1A4;
	sub_823B72D0(ctx, base);
	// 823BD1A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD1A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD1AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD1B0: 4BF02E51  bl 0x822c0000
	ctx.lr = 0x823BD1B4;
	sub_822C0000(ctx, base);
	// 823BD1B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD1B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD1BC: 48DEB000  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD1C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD1C0 size=136
    let mut pc: u32 = 0x823BD1C0;
    'dispatch: loop {
        match pc {
            0x823BD1C0 => {
    //   block [0x823BD1C0..0x823BD248)
	// 823BD1C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD1C4: 48DEAFA9  bl 0x831a816c
	ctx.lr = 0x823BD1C8;
	sub_831A8130(ctx, base);
	// 823BD1C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD1CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD1D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD1D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD1D8: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD1DC: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD1E0: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 823BD1E4: 48A35205  bl 0x82df23e8
	ctx.lr = 0x823BD1E8;
	sub_82DF23E8(ctx, base);
	// 823BD1E8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BD1EC: 41820028  beq 0x823bd214
	if ctx.cr[0].eq {
	pc = 0x823BD214; continue 'dispatch;
	}
	// 823BD1F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BD1F4: 48A9F735  bl 0x82e5c928
	ctx.lr = 0x823BD1F8;
	sub_82E5C928(ctx, base);
	// 823BD1F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BD1FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BD200: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BD204: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BD208: 396AFBFC  addi r11, r10, -0x404
	ctx.r[11].s64 = ctx.r[10].s64 + -1028;
	// 823BD20C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BD210: 48000008  b 0x823bd218
	pc = 0x823BD218; continue 'dispatch;
	// 823BD214: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD218: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD21C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD220: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD224: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD228: 4BFFA171  bl 0x823b7398
	ctx.lr = 0x823BD22C;
	sub_823B7398(ctx, base);
	// 823BD22C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD230: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD234: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD238: 4BF02DC9  bl 0x822c0000
	ctx.lr = 0x823BD23C;
	sub_822C0000(ctx, base);
	// 823BD23C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD240: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD244: 48DEAF78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD248 size=112
    let mut pc: u32 = 0x823BD248;
    'dispatch: loop {
        match pc {
            0x823BD248 => {
    //   block [0x823BD248..0x823BD2B8)
	// 823BD248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD24C: 48DEAF21  bl 0x831a816c
	ctx.lr = 0x823BD250;
	sub_831A8130(ctx, base);
	// 823BD250: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD254: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD258: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD25C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD260: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD264: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD268: 386000D0  li r3, 0xd0
	ctx.r[3].s64 = 208;
	// 823BD26C: 48A3517D  bl 0x82df23e8
	ctx.lr = 0x823BD270;
	sub_82DF23E8(ctx, base);
	// 823BD270: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BD274: 41820010  beq 0x823bd284
	if ctx.cr[0].eq {
	pc = 0x823BD284; continue 'dispatch;
	}
	// 823BD278: 4BFFD9D1  bl 0x823bac48
	ctx.lr = 0x823BD27C;
	sub_823BAC48(ctx, base);
	// 823BD27C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BD280: 48000008  b 0x823bd288
	pc = 0x823BD288; continue 'dispatch;
	// 823BD284: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD288: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD28C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD290: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD294: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD298: 4BFFA1C9  bl 0x823b7460
	ctx.lr = 0x823BD29C;
	sub_823B7460(ctx, base);
	// 823BD29C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD2A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD2A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD2A8: 4BF02D59  bl 0x822c0000
	ctx.lr = 0x823BD2AC;
	sub_822C0000(ctx, base);
	// 823BD2AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD2B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD2B4: 48DEAF08  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD2B8 size=112
    let mut pc: u32 = 0x823BD2B8;
    'dispatch: loop {
        match pc {
            0x823BD2B8 => {
    //   block [0x823BD2B8..0x823BD328)
	// 823BD2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD2BC: 48DEAEB1  bl 0x831a816c
	ctx.lr = 0x823BD2C0;
	sub_831A8130(ctx, base);
	// 823BD2C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD2C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD2C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD2D0: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD2D4: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD2D8: 386000F0  li r3, 0xf0
	ctx.r[3].s64 = 240;
	// 823BD2DC: 48A3510D  bl 0x82df23e8
	ctx.lr = 0x823BD2E0;
	sub_82DF23E8(ctx, base);
	// 823BD2E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BD2E4: 41820010  beq 0x823bd2f4
	if ctx.cr[0].eq {
	pc = 0x823BD2F4; continue 'dispatch;
	}
	// 823BD2E8: 4BFFD9B1  bl 0x823bac98
	ctx.lr = 0x823BD2EC;
	sub_823BAC98(ctx, base);
	// 823BD2EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BD2F0: 48000008  b 0x823bd2f8
	pc = 0x823BD2F8; continue 'dispatch;
	// 823BD2F4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD2F8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD2FC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD300: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD304: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD308: 4BFFA221  bl 0x823b7528
	ctx.lr = 0x823BD30C;
	sub_823B7528(ctx, base);
	// 823BD30C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD310: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD314: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD318: 4BF02CE9  bl 0x822c0000
	ctx.lr = 0x823BD31C;
	sub_822C0000(ctx, base);
	// 823BD31C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD320: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD324: 48DEAE98  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD328 size=112
    let mut pc: u32 = 0x823BD328;
    'dispatch: loop {
        match pc {
            0x823BD328 => {
    //   block [0x823BD328..0x823BD398)
	// 823BD328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD32C: 48DEAE41  bl 0x831a816c
	ctx.lr = 0x823BD330;
	sub_831A8130(ctx, base);
	// 823BD330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD334: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD338: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD33C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD340: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD344: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD348: 3860008C  li r3, 0x8c
	ctx.r[3].s64 = 140;
	// 823BD34C: 48A3509D  bl 0x82df23e8
	ctx.lr = 0x823BD350;
	sub_82DF23E8(ctx, base);
	// 823BD350: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BD354: 41820010  beq 0x823bd364
	if ctx.cr[0].eq {
	pc = 0x823BD364; continue 'dispatch;
	}
	// 823BD358: 4BFFD991  bl 0x823bace8
	ctx.lr = 0x823BD35C;
	sub_823BACE8(ctx, base);
	// 823BD35C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BD360: 48000008  b 0x823bd368
	pc = 0x823BD368; continue 'dispatch;
	// 823BD364: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD368: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD36C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD370: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD374: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD378: 4BFFA279  bl 0x823b75f0
	ctx.lr = 0x823BD37C;
	sub_823B75F0(ctx, base);
	// 823BD37C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD380: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD384: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD388: 4BF02C79  bl 0x822c0000
	ctx.lr = 0x823BD38C;
	sub_822C0000(ctx, base);
	// 823BD38C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD390: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD394: 48DEAE28  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD398 size=136
    let mut pc: u32 = 0x823BD398;
    'dispatch: loop {
        match pc {
            0x823BD398 => {
    //   block [0x823BD398..0x823BD420)
	// 823BD398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD39C: 48DEADD1  bl 0x831a816c
	ctx.lr = 0x823BD3A0;
	sub_831A8130(ctx, base);
	// 823BD3A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD3A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD3A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD3AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD3B0: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD3B4: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD3B8: 3860007C  li r3, 0x7c
	ctx.r[3].s64 = 124;
	// 823BD3BC: 48A3502D  bl 0x82df23e8
	ctx.lr = 0x823BD3C0;
	sub_82DF23E8(ctx, base);
	// 823BD3C0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BD3C4: 41820028  beq 0x823bd3ec
	if ctx.cr[0].eq {
	pc = 0x823BD3EC; continue 'dispatch;
	}
	// 823BD3C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BD3CC: 48A9F55D  bl 0x82e5c928
	ctx.lr = 0x823BD3D0;
	sub_82E5C928(ctx, base);
	// 823BD3D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BD3D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BD3D8: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BD3DC: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BD3E0: 396AFC9C  addi r11, r10, -0x364
	ctx.r[11].s64 = ctx.r[10].s64 + -868;
	// 823BD3E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BD3E8: 48000008  b 0x823bd3f0
	pc = 0x823BD3F0; continue 'dispatch;
	// 823BD3EC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD3F0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD3F4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD3F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD3FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD400: 4BFFA2B9  bl 0x823b76b8
	ctx.lr = 0x823BD404;
	sub_823B76B8(ctx, base);
	// 823BD404: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD408: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD40C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD410: 4BF02BF1  bl 0x822c0000
	ctx.lr = 0x823BD414;
	sub_822C0000(ctx, base);
	// 823BD414: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD418: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD41C: 48DEADA0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD420(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD420 size=112
    let mut pc: u32 = 0x823BD420;
    'dispatch: loop {
        match pc {
            0x823BD420 => {
    //   block [0x823BD420..0x823BD490)
	// 823BD420: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD424: 48DEAD49  bl 0x831a816c
	ctx.lr = 0x823BD428;
	sub_831A8130(ctx, base);
	// 823BD428: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD42C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD430: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD434: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD438: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD43C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD440: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BD444: 48A34FA5  bl 0x82df23e8
	ctx.lr = 0x823BD448;
	sub_82DF23E8(ctx, base);
	// 823BD448: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BD44C: 41820010  beq 0x823bd45c
	if ctx.cr[0].eq {
	pc = 0x823BD45C; continue 'dispatch;
	}
	// 823BD450: 4BFFD8E9  bl 0x823bad38
	ctx.lr = 0x823BD454;
	sub_823BAD38(ctx, base);
	// 823BD454: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BD458: 48000008  b 0x823bd460
	pc = 0x823BD460; continue 'dispatch;
	// 823BD45C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD460: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD464: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD468: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD46C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD470: 4BFFA311  bl 0x823b7780
	ctx.lr = 0x823BD474;
	sub_823B7780(ctx, base);
	// 823BD474: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD478: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD47C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD480: 4BF02B81  bl 0x822c0000
	ctx.lr = 0x823BD484;
	sub_822C0000(ctx, base);
	// 823BD484: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD488: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD48C: 48DEAD30  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD490(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD490 size=112
    let mut pc: u32 = 0x823BD490;
    'dispatch: loop {
        match pc {
            0x823BD490 => {
    //   block [0x823BD490..0x823BD500)
	// 823BD490: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD494: 48DEACD9  bl 0x831a816c
	ctx.lr = 0x823BD498;
	sub_831A8130(ctx, base);
	// 823BD498: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD49C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD4A0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD4A4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD4A8: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD4AC: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD4B0: 3860008C  li r3, 0x8c
	ctx.r[3].s64 = 140;
	// 823BD4B4: 48A34F35  bl 0x82df23e8
	ctx.lr = 0x823BD4B8;
	sub_82DF23E8(ctx, base);
	// 823BD4B8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BD4BC: 41820010  beq 0x823bd4cc
	if ctx.cr[0].eq {
	pc = 0x823BD4CC; continue 'dispatch;
	}
	// 823BD4C0: 4BFFD8C9  bl 0x823bad88
	ctx.lr = 0x823BD4C4;
	sub_823BAD88(ctx, base);
	// 823BD4C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BD4C8: 48000008  b 0x823bd4d0
	pc = 0x823BD4D0; continue 'dispatch;
	// 823BD4CC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD4D0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD4D4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD4D8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD4DC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD4E0: 4BFFA369  bl 0x823b7848
	ctx.lr = 0x823BD4E4;
	sub_823B7848(ctx, base);
	// 823BD4E4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD4E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD4EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD4F0: 4BF02B11  bl 0x822c0000
	ctx.lr = 0x823BD4F4;
	sub_822C0000(ctx, base);
	// 823BD4F4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD4F8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD4FC: 48DEACC0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD500(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD500 size=136
    let mut pc: u32 = 0x823BD500;
    'dispatch: loop {
        match pc {
            0x823BD500 => {
    //   block [0x823BD500..0x823BD588)
	// 823BD500: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD504: 48DEAC69  bl 0x831a816c
	ctx.lr = 0x823BD508;
	sub_831A8130(ctx, base);
	// 823BD508: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD50C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD510: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD514: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD518: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD51C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD520: 38600078  li r3, 0x78
	ctx.r[3].s64 = 120;
	// 823BD524: 48A34EC5  bl 0x82df23e8
	ctx.lr = 0x823BD528;
	sub_82DF23E8(ctx, base);
	// 823BD528: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BD52C: 41820028  beq 0x823bd554
	if ctx.cr[0].eq {
	pc = 0x823BD554; continue 'dispatch;
	}
	// 823BD530: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BD534: 48A9F3F5  bl 0x82e5c928
	ctx.lr = 0x823BD538;
	sub_82E5C928(ctx, base);
	// 823BD538: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BD53C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BD540: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BD544: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BD548: 396AFD14  addi r11, r10, -0x2ec
	ctx.r[11].s64 = ctx.r[10].s64 + -748;
	// 823BD54C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BD550: 48000008  b 0x823bd558
	pc = 0x823BD558; continue 'dispatch;
	// 823BD554: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD558: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD55C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD560: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD564: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD568: 4BFFA3A9  bl 0x823b7910
	ctx.lr = 0x823BD56C;
	sub_823B7910(ctx, base);
	// 823BD56C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD570: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD574: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD578: 4BF02A89  bl 0x822c0000
	ctx.lr = 0x823BD57C;
	sub_822C0000(ctx, base);
	// 823BD57C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD580: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD584: 48DEAC38  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD588 size=136
    let mut pc: u32 = 0x823BD588;
    'dispatch: loop {
        match pc {
            0x823BD588 => {
    //   block [0x823BD588..0x823BD610)
	// 823BD588: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD58C: 48DEABE1  bl 0x831a816c
	ctx.lr = 0x823BD590;
	sub_831A8130(ctx, base);
	// 823BD590: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD594: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD598: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD59C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD5A0: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD5A4: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD5A8: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 823BD5AC: 48A34E3D  bl 0x82df23e8
	ctx.lr = 0x823BD5B0;
	sub_82DF23E8(ctx, base);
	// 823BD5B0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BD5B4: 41820028  beq 0x823bd5dc
	if ctx.cr[0].eq {
	pc = 0x823BD5DC; continue 'dispatch;
	}
	// 823BD5B8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BD5BC: 48A9F36D  bl 0x82e5c928
	ctx.lr = 0x823BD5C0;
	sub_82E5C928(ctx, base);
	// 823BD5C0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BD5C4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BD5C8: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BD5CC: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BD5D0: 396AFD3C  addi r11, r10, -0x2c4
	ctx.r[11].s64 = ctx.r[10].s64 + -708;
	// 823BD5D4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BD5D8: 48000008  b 0x823bd5e0
	pc = 0x823BD5E0; continue 'dispatch;
	// 823BD5DC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD5E0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD5E4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD5E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD5EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD5F0: 4BFFA3E9  bl 0x823b79d8
	ctx.lr = 0x823BD5F4;
	sub_823B79D8(ctx, base);
	// 823BD5F4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD5F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD5FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD600: 4BF02A01  bl 0x822c0000
	ctx.lr = 0x823BD604;
	sub_822C0000(ctx, base);
	// 823BD604: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD608: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD60C: 48DEABB0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD610 size=136
    let mut pc: u32 = 0x823BD610;
    'dispatch: loop {
        match pc {
            0x823BD610 => {
    //   block [0x823BD610..0x823BD698)
	// 823BD610: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD614: 48DEAB59  bl 0x831a816c
	ctx.lr = 0x823BD618;
	sub_831A8130(ctx, base);
	// 823BD618: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD61C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD620: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD624: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD628: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD62C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD630: 38600078  li r3, 0x78
	ctx.r[3].s64 = 120;
	// 823BD634: 48A34DB5  bl 0x82df23e8
	ctx.lr = 0x823BD638;
	sub_82DF23E8(ctx, base);
	// 823BD638: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BD63C: 41820028  beq 0x823bd664
	if ctx.cr[0].eq {
	pc = 0x823BD664; continue 'dispatch;
	}
	// 823BD640: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BD644: 48A9F2E5  bl 0x82e5c928
	ctx.lr = 0x823BD648;
	sub_82E5C928(ctx, base);
	// 823BD648: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BD64C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BD650: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BD654: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BD658: 396AFD64  addi r11, r10, -0x29c
	ctx.r[11].s64 = ctx.r[10].s64 + -668;
	// 823BD65C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BD660: 48000008  b 0x823bd668
	pc = 0x823BD668; continue 'dispatch;
	// 823BD664: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD668: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD66C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD670: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD674: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD678: 4BFFA429  bl 0x823b7aa0
	ctx.lr = 0x823BD67C;
	sub_823B7AA0(ctx, base);
	// 823BD67C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD680: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD684: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD688: 4BF02979  bl 0x822c0000
	ctx.lr = 0x823BD68C;
	sub_822C0000(ctx, base);
	// 823BD68C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD690: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD694: 48DEAB28  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD698(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD698 size=136
    let mut pc: u32 = 0x823BD698;
    'dispatch: loop {
        match pc {
            0x823BD698 => {
    //   block [0x823BD698..0x823BD720)
	// 823BD698: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD69C: 48DEAAD1  bl 0x831a816c
	ctx.lr = 0x823BD6A0;
	sub_831A8130(ctx, base);
	// 823BD6A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD6A4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD6A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD6AC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD6B0: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD6B4: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD6B8: 38600080  li r3, 0x80
	ctx.r[3].s64 = 128;
	// 823BD6BC: 48A34D2D  bl 0x82df23e8
	ctx.lr = 0x823BD6C0;
	sub_82DF23E8(ctx, base);
	// 823BD6C0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BD6C4: 41820028  beq 0x823bd6ec
	if ctx.cr[0].eq {
	pc = 0x823BD6EC; continue 'dispatch;
	}
	// 823BD6C8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BD6CC: 48A9F25D  bl 0x82e5c928
	ctx.lr = 0x823BD6D0;
	sub_82E5C928(ctx, base);
	// 823BD6D0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BD6D4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BD6D8: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BD6DC: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BD6E0: 396AFD8C  addi r11, r10, -0x274
	ctx.r[11].s64 = ctx.r[10].s64 + -628;
	// 823BD6E4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BD6E8: 48000008  b 0x823bd6f0
	pc = 0x823BD6F0; continue 'dispatch;
	// 823BD6EC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD6F0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD6F4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD6F8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD6FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD700: 4BFFA469  bl 0x823b7b68
	ctx.lr = 0x823BD704;
	sub_823B7B68(ctx, base);
	// 823BD704: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD708: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD70C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD710: 4BF028F1  bl 0x822c0000
	ctx.lr = 0x823BD714;
	sub_822C0000(ctx, base);
	// 823BD714: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD718: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD71C: 48DEAAA0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD720 size=136
    let mut pc: u32 = 0x823BD720;
    'dispatch: loop {
        match pc {
            0x823BD720 => {
    //   block [0x823BD720..0x823BD7A8)
	// 823BD720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD724: 48DEAA49  bl 0x831a816c
	ctx.lr = 0x823BD728;
	sub_831A8130(ctx, base);
	// 823BD728: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD72C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD730: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD734: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD738: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD73C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD740: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 823BD744: 48A34CA5  bl 0x82df23e8
	ctx.lr = 0x823BD748;
	sub_82DF23E8(ctx, base);
	// 823BD748: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BD74C: 41820028  beq 0x823bd774
	if ctx.cr[0].eq {
	pc = 0x823BD774; continue 'dispatch;
	}
	// 823BD750: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BD754: 48A9F1D5  bl 0x82e5c928
	ctx.lr = 0x823BD758;
	sub_82E5C928(ctx, base);
	// 823BD758: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BD75C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BD760: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BD764: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BD768: 396AFDB4  addi r11, r10, -0x24c
	ctx.r[11].s64 = ctx.r[10].s64 + -588;
	// 823BD76C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BD770: 48000008  b 0x823bd778
	pc = 0x823BD778; continue 'dispatch;
	// 823BD774: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD778: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD77C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD780: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD784: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD788: 4BFFA4A9  bl 0x823b7c30
	ctx.lr = 0x823BD78C;
	sub_823B7C30(ctx, base);
	// 823BD78C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD790: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD794: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD798: 4BF02869  bl 0x822c0000
	ctx.lr = 0x823BD79C;
	sub_822C0000(ctx, base);
	// 823BD79C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD7A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD7A4: 48DEAA18  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD7A8 size=136
    let mut pc: u32 = 0x823BD7A8;
    'dispatch: loop {
        match pc {
            0x823BD7A8 => {
    //   block [0x823BD7A8..0x823BD830)
	// 823BD7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD7AC: 48DEA9C1  bl 0x831a816c
	ctx.lr = 0x823BD7B0;
	sub_831A8130(ctx, base);
	// 823BD7B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD7B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD7B8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD7BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD7C0: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD7C4: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD7C8: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 823BD7CC: 48A34C1D  bl 0x82df23e8
	ctx.lr = 0x823BD7D0;
	sub_82DF23E8(ctx, base);
	// 823BD7D0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BD7D4: 41820028  beq 0x823bd7fc
	if ctx.cr[0].eq {
	pc = 0x823BD7FC; continue 'dispatch;
	}
	// 823BD7D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BD7DC: 48A9F14D  bl 0x82e5c928
	ctx.lr = 0x823BD7E0;
	sub_82E5C928(ctx, base);
	// 823BD7E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BD7E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BD7E8: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BD7EC: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BD7F0: 396AFDDC  addi r11, r10, -0x224
	ctx.r[11].s64 = ctx.r[10].s64 + -548;
	// 823BD7F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BD7F8: 48000008  b 0x823bd800
	pc = 0x823BD800; continue 'dispatch;
	// 823BD7FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD800: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD804: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD808: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD80C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD810: 4BFFA4E9  bl 0x823b7cf8
	ctx.lr = 0x823BD814;
	sub_823B7CF8(ctx, base);
	// 823BD814: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD818: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD81C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD820: 4BF027E1  bl 0x822c0000
	ctx.lr = 0x823BD824;
	sub_822C0000(ctx, base);
	// 823BD824: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD828: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD82C: 48DEA990  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD830(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD830 size=136
    let mut pc: u32 = 0x823BD830;
    'dispatch: loop {
        match pc {
            0x823BD830 => {
    //   block [0x823BD830..0x823BD8B8)
	// 823BD830: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD834: 48DEA939  bl 0x831a816c
	ctx.lr = 0x823BD838;
	sub_831A8130(ctx, base);
	// 823BD838: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD83C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD840: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD844: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD848: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD84C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD850: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 823BD854: 48A34B95  bl 0x82df23e8
	ctx.lr = 0x823BD858;
	sub_82DF23E8(ctx, base);
	// 823BD858: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BD85C: 41820028  beq 0x823bd884
	if ctx.cr[0].eq {
	pc = 0x823BD884; continue 'dispatch;
	}
	// 823BD860: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BD864: 48A9F0C5  bl 0x82e5c928
	ctx.lr = 0x823BD868;
	sub_82E5C928(ctx, base);
	// 823BD868: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BD86C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BD870: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BD874: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BD878: 396AFE04  addi r11, r10, -0x1fc
	ctx.r[11].s64 = ctx.r[10].s64 + -508;
	// 823BD87C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BD880: 48000008  b 0x823bd888
	pc = 0x823BD888; continue 'dispatch;
	// 823BD884: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD888: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD88C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD890: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD894: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD898: 4BFFA529  bl 0x823b7dc0
	ctx.lr = 0x823BD89C;
	sub_823B7DC0(ctx, base);
	// 823BD89C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD8A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD8A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD8A8: 4BF02759  bl 0x822c0000
	ctx.lr = 0x823BD8AC;
	sub_822C0000(ctx, base);
	// 823BD8AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD8B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD8B4: 48DEA908  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD8B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD8B8 size=136
    let mut pc: u32 = 0x823BD8B8;
    'dispatch: loop {
        match pc {
            0x823BD8B8 => {
    //   block [0x823BD8B8..0x823BD940)
	// 823BD8B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD8BC: 48DEA8B1  bl 0x831a816c
	ctx.lr = 0x823BD8C0;
	sub_831A8130(ctx, base);
	// 823BD8C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD8C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD8C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD8CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD8D0: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD8D4: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD8D8: 38600078  li r3, 0x78
	ctx.r[3].s64 = 120;
	// 823BD8DC: 48A34B0D  bl 0x82df23e8
	ctx.lr = 0x823BD8E0;
	sub_82DF23E8(ctx, base);
	// 823BD8E0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BD8E4: 41820028  beq 0x823bd90c
	if ctx.cr[0].eq {
	pc = 0x823BD90C; continue 'dispatch;
	}
	// 823BD8E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BD8EC: 48A9F03D  bl 0x82e5c928
	ctx.lr = 0x823BD8F0;
	sub_82E5C928(ctx, base);
	// 823BD8F0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BD8F4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BD8F8: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BD8FC: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BD900: 396AFE2C  addi r11, r10, -0x1d4
	ctx.r[11].s64 = ctx.r[10].s64 + -468;
	// 823BD904: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BD908: 48000008  b 0x823bd910
	pc = 0x823BD910; continue 'dispatch;
	// 823BD90C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD910: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD914: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD918: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD91C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD920: 4BFFA569  bl 0x823b7e88
	ctx.lr = 0x823BD924;
	sub_823B7E88(ctx, base);
	// 823BD924: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD928: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD92C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD930: 4BF026D1  bl 0x822c0000
	ctx.lr = 0x823BD934;
	sub_822C0000(ctx, base);
	// 823BD934: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD938: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD93C: 48DEA880  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD940 size=136
    let mut pc: u32 = 0x823BD940;
    'dispatch: loop {
        match pc {
            0x823BD940 => {
    //   block [0x823BD940..0x823BD9C8)
	// 823BD940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD944: 48DEA829  bl 0x831a816c
	ctx.lr = 0x823BD948;
	sub_831A8130(ctx, base);
	// 823BD948: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD94C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD950: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD958: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD95C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD960: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 823BD964: 48A34A85  bl 0x82df23e8
	ctx.lr = 0x823BD968;
	sub_82DF23E8(ctx, base);
	// 823BD968: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BD96C: 41820028  beq 0x823bd994
	if ctx.cr[0].eq {
	pc = 0x823BD994; continue 'dispatch;
	}
	// 823BD970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BD974: 48A9EFB5  bl 0x82e5c928
	ctx.lr = 0x823BD978;
	sub_82E5C928(ctx, base);
	// 823BD978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BD97C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BD980: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BD984: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BD988: 396AFE54  addi r11, r10, -0x1ac
	ctx.r[11].s64 = ctx.r[10].s64 + -428;
	// 823BD98C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BD990: 48000008  b 0x823bd998
	pc = 0x823BD998; continue 'dispatch;
	// 823BD994: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BD998: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BD99C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BD9A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD9A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD9A8: 4BFFA5A9  bl 0x823b7f50
	ctx.lr = 0x823BD9AC;
	sub_823B7F50(ctx, base);
	// 823BD9AC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BD9B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BD9B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BD9B8: 4BF02649  bl 0x822c0000
	ctx.lr = 0x823BD9BC;
	sub_822C0000(ctx, base);
	// 823BD9BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BD9C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BD9C4: 48DEA7F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BD9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BD9C8 size=136
    let mut pc: u32 = 0x823BD9C8;
    'dispatch: loop {
        match pc {
            0x823BD9C8 => {
    //   block [0x823BD9C8..0x823BDA50)
	// 823BD9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BD9CC: 48DEA7A1  bl 0x831a816c
	ctx.lr = 0x823BD9D0;
	sub_831A8130(ctx, base);
	// 823BD9D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BD9D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BD9D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BD9DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BD9E0: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BD9E4: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BD9E8: 38600084  li r3, 0x84
	ctx.r[3].s64 = 132;
	// 823BD9EC: 48A349FD  bl 0x82df23e8
	ctx.lr = 0x823BD9F0;
	sub_82DF23E8(ctx, base);
	// 823BD9F0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BD9F4: 41820028  beq 0x823bda1c
	if ctx.cr[0].eq {
	pc = 0x823BDA1C; continue 'dispatch;
	}
	// 823BD9F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BD9FC: 48A9EF2D  bl 0x82e5c928
	ctx.lr = 0x823BDA00;
	sub_82E5C928(ctx, base);
	// 823BDA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BDA04: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BDA08: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BDA0C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BDA10: 396AFE7C  addi r11, r10, -0x184
	ctx.r[11].s64 = ctx.r[10].s64 + -388;
	// 823BDA14: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BDA18: 48000008  b 0x823bda20
	pc = 0x823BDA20; continue 'dispatch;
	// 823BDA1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BDA20: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BDA24: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BDA28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDA2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDA30: 4BFFA5E9  bl 0x823b8018
	ctx.lr = 0x823BDA34;
	sub_823B8018(ctx, base);
	// 823BDA34: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BDA38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDA3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDA40: 4BF025C1  bl 0x822c0000
	ctx.lr = 0x823BDA44;
	sub_822C0000(ctx, base);
	// 823BDA44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BDA48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BDA4C: 48DEA770  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BDA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BDA50 size=136
    let mut pc: u32 = 0x823BDA50;
    'dispatch: loop {
        match pc {
            0x823BDA50 => {
    //   block [0x823BDA50..0x823BDAD8)
	// 823BDA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BDA54: 48DEA719  bl 0x831a816c
	ctx.lr = 0x823BDA58;
	sub_831A8130(ctx, base);
	// 823BDA58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BDA5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BDA60: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BDA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BDA68: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BDA6C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BDA70: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 823BDA74: 48A34975  bl 0x82df23e8
	ctx.lr = 0x823BDA78;
	sub_82DF23E8(ctx, base);
	// 823BDA78: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BDA7C: 41820028  beq 0x823bdaa4
	if ctx.cr[0].eq {
	pc = 0x823BDAA4; continue 'dispatch;
	}
	// 823BDA80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BDA84: 48A9EEA5  bl 0x82e5c928
	ctx.lr = 0x823BDA88;
	sub_82E5C928(ctx, base);
	// 823BDA88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BDA8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BDA90: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BDA94: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BDA98: 396AFEA4  addi r11, r10, -0x15c
	ctx.r[11].s64 = ctx.r[10].s64 + -348;
	// 823BDA9C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BDAA0: 48000008  b 0x823bdaa8
	pc = 0x823BDAA8; continue 'dispatch;
	// 823BDAA4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BDAA8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BDAAC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BDAB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDAB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDAB8: 4BFFA629  bl 0x823b80e0
	ctx.lr = 0x823BDABC;
	sub_823B80E0(ctx, base);
	// 823BDABC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BDAC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDAC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDAC8: 4BF02539  bl 0x822c0000
	ctx.lr = 0x823BDACC;
	sub_822C0000(ctx, base);
	// 823BDACC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BDAD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BDAD4: 48DEA6E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BDAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BDAD8 size=136
    let mut pc: u32 = 0x823BDAD8;
    'dispatch: loop {
        match pc {
            0x823BDAD8 => {
    //   block [0x823BDAD8..0x823BDB60)
	// 823BDAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BDADC: 48DEA691  bl 0x831a816c
	ctx.lr = 0x823BDAE0;
	sub_831A8130(ctx, base);
	// 823BDAE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BDAE4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BDAE8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BDAEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BDAF0: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BDAF4: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BDAF8: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 823BDAFC: 48A348ED  bl 0x82df23e8
	ctx.lr = 0x823BDB00;
	sub_82DF23E8(ctx, base);
	// 823BDB00: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BDB04: 41820028  beq 0x823bdb2c
	if ctx.cr[0].eq {
	pc = 0x823BDB2C; continue 'dispatch;
	}
	// 823BDB08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BDB0C: 48A9EE1D  bl 0x82e5c928
	ctx.lr = 0x823BDB10;
	sub_82E5C928(ctx, base);
	// 823BDB10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BDB14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BDB18: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BDB1C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BDB20: 396AFECC  addi r11, r10, -0x134
	ctx.r[11].s64 = ctx.r[10].s64 + -308;
	// 823BDB24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BDB28: 48000008  b 0x823bdb30
	pc = 0x823BDB30; continue 'dispatch;
	// 823BDB2C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BDB30: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BDB34: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BDB38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDB3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDB40: 4BFFA669  bl 0x823b81a8
	ctx.lr = 0x823BDB44;
	sub_823B81A8(ctx, base);
	// 823BDB44: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BDB48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDB4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDB50: 4BF024B1  bl 0x822c0000
	ctx.lr = 0x823BDB54;
	sub_822C0000(ctx, base);
	// 823BDB54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BDB58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BDB5C: 48DEA660  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BDB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BDB60 size=136
    let mut pc: u32 = 0x823BDB60;
    'dispatch: loop {
        match pc {
            0x823BDB60 => {
    //   block [0x823BDB60..0x823BDBE8)
	// 823BDB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BDB64: 48DEA609  bl 0x831a816c
	ctx.lr = 0x823BDB68;
	sub_831A8130(ctx, base);
	// 823BDB68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BDB6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BDB70: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BDB74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BDB78: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BDB7C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BDB80: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 823BDB84: 48A34865  bl 0x82df23e8
	ctx.lr = 0x823BDB88;
	sub_82DF23E8(ctx, base);
	// 823BDB88: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BDB8C: 41820028  beq 0x823bdbb4
	if ctx.cr[0].eq {
	pc = 0x823BDBB4; continue 'dispatch;
	}
	// 823BDB90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BDB94: 48A9ED95  bl 0x82e5c928
	ctx.lr = 0x823BDB98;
	sub_82E5C928(ctx, base);
	// 823BDB98: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BDB9C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BDBA0: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BDBA4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BDBA8: 396AFEF4  addi r11, r10, -0x10c
	ctx.r[11].s64 = ctx.r[10].s64 + -268;
	// 823BDBAC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BDBB0: 48000008  b 0x823bdbb8
	pc = 0x823BDBB8; continue 'dispatch;
	// 823BDBB4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BDBB8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BDBBC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BDBC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDBC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDBC8: 4BFFA6A9  bl 0x823b8270
	ctx.lr = 0x823BDBCC;
	sub_823B8270(ctx, base);
	// 823BDBCC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BDBD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDBD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDBD8: 4BF02429  bl 0x822c0000
	ctx.lr = 0x823BDBDC;
	sub_822C0000(ctx, base);
	// 823BDBDC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BDBE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BDBE4: 48DEA5D8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BDBE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BDBE8 size=136
    let mut pc: u32 = 0x823BDBE8;
    'dispatch: loop {
        match pc {
            0x823BDBE8 => {
    //   block [0x823BDBE8..0x823BDC70)
	// 823BDBE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BDBEC: 48DEA581  bl 0x831a816c
	ctx.lr = 0x823BDBF0;
	sub_831A8130(ctx, base);
	// 823BDBF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BDBF4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BDBF8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BDBFC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BDC00: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BDC04: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BDC08: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BDC0C: 48A347DD  bl 0x82df23e8
	ctx.lr = 0x823BDC10;
	sub_82DF23E8(ctx, base);
	// 823BDC10: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BDC14: 41820028  beq 0x823bdc3c
	if ctx.cr[0].eq {
	pc = 0x823BDC3C; continue 'dispatch;
	}
	// 823BDC18: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BDC1C: 48A9ED0D  bl 0x82e5c928
	ctx.lr = 0x823BDC20;
	sub_82E5C928(ctx, base);
	// 823BDC20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BDC24: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BDC28: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BDC2C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BDC30: 396AFF1C  addi r11, r10, -0xe4
	ctx.r[11].s64 = ctx.r[10].s64 + -228;
	// 823BDC34: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BDC38: 48000008  b 0x823bdc40
	pc = 0x823BDC40; continue 'dispatch;
	// 823BDC3C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BDC40: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BDC44: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BDC48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDC4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDC50: 4BFFA6E9  bl 0x823b8338
	ctx.lr = 0x823BDC54;
	sub_823B8338(ctx, base);
	// 823BDC54: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BDC58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDC5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDC60: 4BF023A1  bl 0x822c0000
	ctx.lr = 0x823BDC64;
	sub_822C0000(ctx, base);
	// 823BDC64: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BDC68: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BDC6C: 48DEA550  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BDC70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BDC70 size=136
    let mut pc: u32 = 0x823BDC70;
    'dispatch: loop {
        match pc {
            0x823BDC70 => {
    //   block [0x823BDC70..0x823BDCF8)
	// 823BDC70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BDC74: 48DEA4F9  bl 0x831a816c
	ctx.lr = 0x823BDC78;
	sub_831A8130(ctx, base);
	// 823BDC78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BDC7C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BDC80: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BDC84: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BDC88: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BDC8C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BDC90: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BDC94: 48A34755  bl 0x82df23e8
	ctx.lr = 0x823BDC98;
	sub_82DF23E8(ctx, base);
	// 823BDC98: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BDC9C: 41820028  beq 0x823bdcc4
	if ctx.cr[0].eq {
	pc = 0x823BDCC4; continue 'dispatch;
	}
	// 823BDCA0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BDCA4: 48A9EC85  bl 0x82e5c928
	ctx.lr = 0x823BDCA8;
	sub_82E5C928(ctx, base);
	// 823BDCA8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BDCAC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BDCB0: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BDCB4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BDCB8: 396AFF44  addi r11, r10, -0xbc
	ctx.r[11].s64 = ctx.r[10].s64 + -188;
	// 823BDCBC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BDCC0: 48000008  b 0x823bdcc8
	pc = 0x823BDCC8; continue 'dispatch;
	// 823BDCC4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BDCC8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BDCCC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BDCD0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDCD4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDCD8: 4BFFA729  bl 0x823b8400
	ctx.lr = 0x823BDCDC;
	sub_823B8400(ctx, base);
	// 823BDCDC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BDCE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDCE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDCE8: 4BF02319  bl 0x822c0000
	ctx.lr = 0x823BDCEC;
	sub_822C0000(ctx, base);
	// 823BDCEC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BDCF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BDCF4: 48DEA4C8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BDCF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BDCF8 size=136
    let mut pc: u32 = 0x823BDCF8;
    'dispatch: loop {
        match pc {
            0x823BDCF8 => {
    //   block [0x823BDCF8..0x823BDD80)
	// 823BDCF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BDCFC: 48DEA471  bl 0x831a816c
	ctx.lr = 0x823BDD00;
	sub_831A8130(ctx, base);
	// 823BDD00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BDD04: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BDD08: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BDD0C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BDD10: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BDD14: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BDD18: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 823BDD1C: 48A346CD  bl 0x82df23e8
	ctx.lr = 0x823BDD20;
	sub_82DF23E8(ctx, base);
	// 823BDD20: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BDD24: 41820028  beq 0x823bdd4c
	if ctx.cr[0].eq {
	pc = 0x823BDD4C; continue 'dispatch;
	}
	// 823BDD28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BDD2C: 48A9EBFD  bl 0x82e5c928
	ctx.lr = 0x823BDD30;
	sub_82E5C928(ctx, base);
	// 823BDD30: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BDD34: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BDD38: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BDD3C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BDD40: 396AFF6C  addi r11, r10, -0x94
	ctx.r[11].s64 = ctx.r[10].s64 + -148;
	// 823BDD44: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BDD48: 48000008  b 0x823bdd50
	pc = 0x823BDD50; continue 'dispatch;
	// 823BDD4C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BDD50: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BDD54: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BDD58: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDD5C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDD60: 4BFFA769  bl 0x823b84c8
	ctx.lr = 0x823BDD64;
	sub_823B84C8(ctx, base);
	// 823BDD64: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BDD68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDD6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDD70: 4BF02291  bl 0x822c0000
	ctx.lr = 0x823BDD74;
	sub_822C0000(ctx, base);
	// 823BDD74: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BDD78: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BDD7C: 48DEA440  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BDD80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BDD80 size=136
    let mut pc: u32 = 0x823BDD80;
    'dispatch: loop {
        match pc {
            0x823BDD80 => {
    //   block [0x823BDD80..0x823BDE08)
	// 823BDD80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BDD84: 48DEA3E9  bl 0x831a816c
	ctx.lr = 0x823BDD88;
	sub_831A8130(ctx, base);
	// 823BDD88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BDD8C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BDD90: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BDD94: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BDD98: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BDD9C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BDDA0: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 823BDDA4: 48A34645  bl 0x82df23e8
	ctx.lr = 0x823BDDA8;
	sub_82DF23E8(ctx, base);
	// 823BDDA8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BDDAC: 41820028  beq 0x823bddd4
	if ctx.cr[0].eq {
	pc = 0x823BDDD4; continue 'dispatch;
	}
	// 823BDDB0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BDDB4: 48A9EB75  bl 0x82e5c928
	ctx.lr = 0x823BDDB8;
	sub_82E5C928(ctx, base);
	// 823BDDB8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BDDBC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BDDC0: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BDDC4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BDDC8: 396AFF94  addi r11, r10, -0x6c
	ctx.r[11].s64 = ctx.r[10].s64 + -108;
	// 823BDDCC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BDDD0: 48000008  b 0x823bddd8
	pc = 0x823BDDD8; continue 'dispatch;
	// 823BDDD4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BDDD8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BDDDC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BDDE0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDDE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDDE8: 4BFFA7A9  bl 0x823b8590
	ctx.lr = 0x823BDDEC;
	sub_823B8590(ctx, base);
	// 823BDDEC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BDDF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDDF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDDF8: 4BF02209  bl 0x822c0000
	ctx.lr = 0x823BDDFC;
	sub_822C0000(ctx, base);
	// 823BDDFC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BDE00: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BDE04: 48DEA3B8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BDE08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BDE08 size=136
    let mut pc: u32 = 0x823BDE08;
    'dispatch: loop {
        match pc {
            0x823BDE08 => {
    //   block [0x823BDE08..0x823BDE90)
	// 823BDE08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BDE0C: 48DEA361  bl 0x831a816c
	ctx.lr = 0x823BDE10;
	sub_831A8130(ctx, base);
	// 823BDE10: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BDE14: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BDE18: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BDE1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BDE20: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BDE24: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BDE28: 38600068  li r3, 0x68
	ctx.r[3].s64 = 104;
	// 823BDE2C: 48A345BD  bl 0x82df23e8
	ctx.lr = 0x823BDE30;
	sub_82DF23E8(ctx, base);
	// 823BDE30: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BDE34: 41820028  beq 0x823bde5c
	if ctx.cr[0].eq {
	pc = 0x823BDE5C; continue 'dispatch;
	}
	// 823BDE38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BDE3C: 48A9EAED  bl 0x82e5c928
	ctx.lr = 0x823BDE40;
	sub_82E5C928(ctx, base);
	// 823BDE40: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BDE44: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BDE48: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BDE4C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BDE50: 396AFFBC  addi r11, r10, -0x44
	ctx.r[11].s64 = ctx.r[10].s64 + -68;
	// 823BDE54: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BDE58: 48000008  b 0x823bde60
	pc = 0x823BDE60; continue 'dispatch;
	// 823BDE5C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BDE60: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BDE64: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BDE68: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDE6C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDE70: 4BFFA7E9  bl 0x823b8658
	ctx.lr = 0x823BDE74;
	sub_823B8658(ctx, base);
	// 823BDE74: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BDE78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDE7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDE80: 4BF02181  bl 0x822c0000
	ctx.lr = 0x823BDE84;
	sub_822C0000(ctx, base);
	// 823BDE84: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BDE88: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BDE8C: 48DEA330  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BDE90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BDE90 size=136
    let mut pc: u32 = 0x823BDE90;
    'dispatch: loop {
        match pc {
            0x823BDE90 => {
    //   block [0x823BDE90..0x823BDF18)
	// 823BDE90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BDE94: 48DEA2D9  bl 0x831a816c
	ctx.lr = 0x823BDE98;
	sub_831A8130(ctx, base);
	// 823BDE98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BDE9C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BDEA0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BDEA4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BDEA8: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BDEAC: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BDEB0: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 823BDEB4: 48A34535  bl 0x82df23e8
	ctx.lr = 0x823BDEB8;
	sub_82DF23E8(ctx, base);
	// 823BDEB8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BDEBC: 41820028  beq 0x823bdee4
	if ctx.cr[0].eq {
	pc = 0x823BDEE4; continue 'dispatch;
	}
	// 823BDEC0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BDEC4: 48A9EA65  bl 0x82e5c928
	ctx.lr = 0x823BDEC8;
	sub_82E5C928(ctx, base);
	// 823BDEC8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BDECC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BDED0: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BDED4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BDED8: 396AFFE4  addi r11, r10, -0x1c
	ctx.r[11].s64 = ctx.r[10].s64 + -28;
	// 823BDEDC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BDEE0: 48000008  b 0x823bdee8
	pc = 0x823BDEE8; continue 'dispatch;
	// 823BDEE4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BDEE8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BDEEC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BDEF0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDEF4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDEF8: 4BFFA829  bl 0x823b8720
	ctx.lr = 0x823BDEFC;
	sub_823B8720(ctx, base);
	// 823BDEFC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BDF00: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDF04: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDF08: 4BF020F9  bl 0x822c0000
	ctx.lr = 0x823BDF0C;
	sub_822C0000(ctx, base);
	// 823BDF0C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BDF10: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BDF14: 48DEA2A8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BDF18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BDF18 size=136
    let mut pc: u32 = 0x823BDF18;
    'dispatch: loop {
        match pc {
            0x823BDF18 => {
    //   block [0x823BDF18..0x823BDFA0)
	// 823BDF18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BDF1C: 48DEA251  bl 0x831a816c
	ctx.lr = 0x823BDF20;
	sub_831A8130(ctx, base);
	// 823BDF20: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BDF24: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BDF28: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BDF2C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BDF30: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BDF34: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BDF38: 38600078  li r3, 0x78
	ctx.r[3].s64 = 120;
	// 823BDF3C: 48A344AD  bl 0x82df23e8
	ctx.lr = 0x823BDF40;
	sub_82DF23E8(ctx, base);
	// 823BDF40: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BDF44: 41820028  beq 0x823bdf6c
	if ctx.cr[0].eq {
	pc = 0x823BDF6C; continue 'dispatch;
	}
	// 823BDF48: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BDF4C: 48A9E9DD  bl 0x82e5c928
	ctx.lr = 0x823BDF50;
	sub_82E5C928(ctx, base);
	// 823BDF50: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BDF54: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BDF58: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BDF5C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BDF60: 396A000C  addi r11, r10, 0xc
	ctx.r[11].s64 = ctx.r[10].s64 + 12;
	// 823BDF64: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BDF68: 48000008  b 0x823bdf70
	pc = 0x823BDF70; continue 'dispatch;
	// 823BDF6C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BDF70: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BDF74: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BDF78: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDF7C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDF80: 4BFFA869  bl 0x823b87e8
	ctx.lr = 0x823BDF84;
	sub_823B87E8(ctx, base);
	// 823BDF84: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BDF88: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BDF8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BDF90: 4BF02071  bl 0x822c0000
	ctx.lr = 0x823BDF94;
	sub_822C0000(ctx, base);
	// 823BDF94: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BDF98: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BDF9C: 48DEA220  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BDFA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BDFA0 size=136
    let mut pc: u32 = 0x823BDFA0;
    'dispatch: loop {
        match pc {
            0x823BDFA0 => {
    //   block [0x823BDFA0..0x823BE028)
	// 823BDFA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BDFA4: 48DEA1C9  bl 0x831a816c
	ctx.lr = 0x823BDFA8;
	sub_831A8130(ctx, base);
	// 823BDFA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BDFAC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BDFB0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BDFB4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BDFB8: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BDFBC: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BDFC0: 38600068  li r3, 0x68
	ctx.r[3].s64 = 104;
	// 823BDFC4: 48A34425  bl 0x82df23e8
	ctx.lr = 0x823BDFC8;
	sub_82DF23E8(ctx, base);
	// 823BDFC8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BDFCC: 41820028  beq 0x823bdff4
	if ctx.cr[0].eq {
	pc = 0x823BDFF4; continue 'dispatch;
	}
	// 823BDFD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BDFD4: 48A9E955  bl 0x82e5c928
	ctx.lr = 0x823BDFD8;
	sub_82E5C928(ctx, base);
	// 823BDFD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BDFDC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BDFE0: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BDFE4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BDFE8: 396A0034  addi r11, r10, 0x34
	ctx.r[11].s64 = ctx.r[10].s64 + 52;
	// 823BDFEC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BDFF0: 48000008  b 0x823bdff8
	pc = 0x823BDFF8; continue 'dispatch;
	// 823BDFF4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BDFF8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BDFFC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE000: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE004: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE008: 4BFFA8A9  bl 0x823b88b0
	ctx.lr = 0x823BE00C;
	sub_823B88B0(ctx, base);
	// 823BE00C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE010: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE014: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE018: 4BF01FE9  bl 0x822c0000
	ctx.lr = 0x823BE01C;
	sub_822C0000(ctx, base);
	// 823BE01C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE020: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE024: 48DEA198  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE028 size=136
    let mut pc: u32 = 0x823BE028;
    'dispatch: loop {
        match pc {
            0x823BE028 => {
    //   block [0x823BE028..0x823BE0B0)
	// 823BE028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE02C: 48DEA141  bl 0x831a816c
	ctx.lr = 0x823BE030;
	sub_831A8130(ctx, base);
	// 823BE030: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE034: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE038: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE040: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE044: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE048: 38600068  li r3, 0x68
	ctx.r[3].s64 = 104;
	// 823BE04C: 48A3439D  bl 0x82df23e8
	ctx.lr = 0x823BE050;
	sub_82DF23E8(ctx, base);
	// 823BE050: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BE054: 41820028  beq 0x823be07c
	if ctx.cr[0].eq {
	pc = 0x823BE07C; continue 'dispatch;
	}
	// 823BE058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BE05C: 48A9E8CD  bl 0x82e5c928
	ctx.lr = 0x823BE060;
	sub_82E5C928(ctx, base);
	// 823BE060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BE064: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BE068: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BE06C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BE070: 396A005C  addi r11, r10, 0x5c
	ctx.r[11].s64 = ctx.r[10].s64 + 92;
	// 823BE074: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BE078: 48000008  b 0x823be080
	pc = 0x823BE080; continue 'dispatch;
	// 823BE07C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE080: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE084: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE088: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE08C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE090: 4BFFA8E9  bl 0x823b8978
	ctx.lr = 0x823BE094;
	sub_823B8978(ctx, base);
	// 823BE094: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE098: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE09C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE0A0: 4BF01F61  bl 0x822c0000
	ctx.lr = 0x823BE0A4;
	sub_822C0000(ctx, base);
	// 823BE0A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE0A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE0AC: 48DEA110  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE0B0 size=136
    let mut pc: u32 = 0x823BE0B0;
    'dispatch: loop {
        match pc {
            0x823BE0B0 => {
    //   block [0x823BE0B0..0x823BE138)
	// 823BE0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE0B4: 48DEA0B9  bl 0x831a816c
	ctx.lr = 0x823BE0B8;
	sub_831A8130(ctx, base);
	// 823BE0B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE0BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE0C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE0C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE0C8: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE0CC: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE0D0: 386000A0  li r3, 0xa0
	ctx.r[3].s64 = 160;
	// 823BE0D4: 48A34315  bl 0x82df23e8
	ctx.lr = 0x823BE0D8;
	sub_82DF23E8(ctx, base);
	// 823BE0D8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BE0DC: 41820028  beq 0x823be104
	if ctx.cr[0].eq {
	pc = 0x823BE104; continue 'dispatch;
	}
	// 823BE0E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BE0E4: 48A9E845  bl 0x82e5c928
	ctx.lr = 0x823BE0E8;
	sub_82E5C928(ctx, base);
	// 823BE0E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BE0EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BE0F0: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BE0F4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BE0F8: 396A0084  addi r11, r10, 0x84
	ctx.r[11].s64 = ctx.r[10].s64 + 132;
	// 823BE0FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BE100: 48000008  b 0x823be108
	pc = 0x823BE108; continue 'dispatch;
	// 823BE104: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE108: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE10C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE110: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE114: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE118: 4BFFA929  bl 0x823b8a40
	ctx.lr = 0x823BE11C;
	sub_823B8A40(ctx, base);
	// 823BE11C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE120: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE124: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE128: 4BF01ED9  bl 0x822c0000
	ctx.lr = 0x823BE12C;
	sub_822C0000(ctx, base);
	// 823BE12C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE130: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE134: 48DEA088  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE138 size=112
    let mut pc: u32 = 0x823BE138;
    'dispatch: loop {
        match pc {
            0x823BE138 => {
    //   block [0x823BE138..0x823BE1A8)
	// 823BE138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE13C: 48DEA031  bl 0x831a816c
	ctx.lr = 0x823BE140;
	sub_831A8130(ctx, base);
	// 823BE140: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE144: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE148: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE14C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE150: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE154: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE158: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 823BE15C: 48A3428D  bl 0x82df23e8
	ctx.lr = 0x823BE160;
	sub_82DF23E8(ctx, base);
	// 823BE160: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BE164: 41820010  beq 0x823be174
	if ctx.cr[0].eq {
	pc = 0x823BE174; continue 'dispatch;
	}
	// 823BE168: 4BFFCC71  bl 0x823badd8
	ctx.lr = 0x823BE16C;
	sub_823BADD8(ctx, base);
	// 823BE16C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BE170: 48000008  b 0x823be178
	pc = 0x823BE178; continue 'dispatch;
	// 823BE174: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE178: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE17C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE180: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE184: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE188: 4BFFA981  bl 0x823b8b08
	ctx.lr = 0x823BE18C;
	sub_823B8B08(ctx, base);
	// 823BE18C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE190: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE194: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE198: 4BF01E69  bl 0x822c0000
	ctx.lr = 0x823BE19C;
	sub_822C0000(ctx, base);
	// 823BE19C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE1A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE1A4: 48DEA018  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE1A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE1A8 size=136
    let mut pc: u32 = 0x823BE1A8;
    'dispatch: loop {
        match pc {
            0x823BE1A8 => {
    //   block [0x823BE1A8..0x823BE230)
	// 823BE1A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE1AC: 48DE9FC1  bl 0x831a816c
	ctx.lr = 0x823BE1B0;
	sub_831A8130(ctx, base);
	// 823BE1B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE1B4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE1B8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE1BC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE1C0: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE1C4: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE1C8: 386000F0  li r3, 0xf0
	ctx.r[3].s64 = 240;
	// 823BE1CC: 48A3421D  bl 0x82df23e8
	ctx.lr = 0x823BE1D0;
	sub_82DF23E8(ctx, base);
	// 823BE1D0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BE1D4: 41820028  beq 0x823be1fc
	if ctx.cr[0].eq {
	pc = 0x823BE1FC; continue 'dispatch;
	}
	// 823BE1D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BE1DC: 48A9E74D  bl 0x82e5c928
	ctx.lr = 0x823BE1E0;
	sub_82E5C928(ctx, base);
	// 823BE1E0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BE1E4: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BE1E8: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BE1EC: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BE1F0: 396A00D4  addi r11, r10, 0xd4
	ctx.r[11].s64 = ctx.r[10].s64 + 212;
	// 823BE1F4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BE1F8: 48000008  b 0x823be200
	pc = 0x823BE200; continue 'dispatch;
	// 823BE1FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE200: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE204: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE208: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE20C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE210: 4BFFA9C1  bl 0x823b8bd0
	ctx.lr = 0x823BE214;
	sub_823B8BD0(ctx, base);
	// 823BE214: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE218: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE21C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE220: 4BF01DE1  bl 0x822c0000
	ctx.lr = 0x823BE224;
	sub_822C0000(ctx, base);
	// 823BE224: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE228: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE22C: 48DE9F90  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE230(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE230 size=136
    let mut pc: u32 = 0x823BE230;
    'dispatch: loop {
        match pc {
            0x823BE230 => {
    //   block [0x823BE230..0x823BE2B8)
	// 823BE230: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE234: 48DE9F39  bl 0x831a816c
	ctx.lr = 0x823BE238;
	sub_831A8130(ctx, base);
	// 823BE238: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE23C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE240: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE244: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE248: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE24C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE250: 38600068  li r3, 0x68
	ctx.r[3].s64 = 104;
	// 823BE254: 48A34195  bl 0x82df23e8
	ctx.lr = 0x823BE258;
	sub_82DF23E8(ctx, base);
	// 823BE258: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BE25C: 41820028  beq 0x823be284
	if ctx.cr[0].eq {
	pc = 0x823BE284; continue 'dispatch;
	}
	// 823BE260: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BE264: 48A9E6C5  bl 0x82e5c928
	ctx.lr = 0x823BE268;
	sub_82E5C928(ctx, base);
	// 823BE268: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BE26C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BE270: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BE274: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BE278: 396A00FC  addi r11, r10, 0xfc
	ctx.r[11].s64 = ctx.r[10].s64 + 252;
	// 823BE27C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BE280: 48000008  b 0x823be288
	pc = 0x823BE288; continue 'dispatch;
	// 823BE284: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE288: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE28C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE290: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE294: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE298: 4BFFAA01  bl 0x823b8c98
	ctx.lr = 0x823BE29C;
	sub_823B8C98(ctx, base);
	// 823BE29C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE2A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE2A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE2A8: 4BF01D59  bl 0x822c0000
	ctx.lr = 0x823BE2AC;
	sub_822C0000(ctx, base);
	// 823BE2AC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE2B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE2B4: 48DE9F08  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE2B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE2B8 size=112
    let mut pc: u32 = 0x823BE2B8;
    'dispatch: loop {
        match pc {
            0x823BE2B8 => {
    //   block [0x823BE2B8..0x823BE328)
	// 823BE2B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE2BC: 48DE9EB1  bl 0x831a816c
	ctx.lr = 0x823BE2C0;
	sub_831A8130(ctx, base);
	// 823BE2C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE2C4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE2C8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE2CC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE2D0: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE2D4: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE2D8: 38600084  li r3, 0x84
	ctx.r[3].s64 = 132;
	// 823BE2DC: 48A3410D  bl 0x82df23e8
	ctx.lr = 0x823BE2E0;
	sub_82DF23E8(ctx, base);
	// 823BE2E0: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BE2E4: 41820010  beq 0x823be2f4
	if ctx.cr[0].eq {
	pc = 0x823BE2F4; continue 'dispatch;
	}
	// 823BE2E8: 4BFFCB41  bl 0x823bae28
	ctx.lr = 0x823BE2EC;
	sub_823BAE28(ctx, base);
	// 823BE2EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BE2F0: 48000008  b 0x823be2f8
	pc = 0x823BE2F8; continue 'dispatch;
	// 823BE2F4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE2F8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE2FC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE300: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE304: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE308: 4BFFAA59  bl 0x823b8d60
	ctx.lr = 0x823BE30C;
	sub_823B8D60(ctx, base);
	// 823BE30C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE310: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE314: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE318: 4BF01CE9  bl 0x822c0000
	ctx.lr = 0x823BE31C;
	sub_822C0000(ctx, base);
	// 823BE31C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE320: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE324: 48DE9E98  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE328 size=136
    let mut pc: u32 = 0x823BE328;
    'dispatch: loop {
        match pc {
            0x823BE328 => {
    //   block [0x823BE328..0x823BE3B0)
	// 823BE328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE32C: 48DE9E41  bl 0x831a816c
	ctx.lr = 0x823BE330;
	sub_831A8130(ctx, base);
	// 823BE330: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE334: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE338: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE33C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE340: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE344: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE348: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BE34C: 48A3409D  bl 0x82df23e8
	ctx.lr = 0x823BE350;
	sub_82DF23E8(ctx, base);
	// 823BE350: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BE354: 41820028  beq 0x823be37c
	if ctx.cr[0].eq {
	pc = 0x823BE37C; continue 'dispatch;
	}
	// 823BE358: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BE35C: 48A9E5CD  bl 0x82e5c928
	ctx.lr = 0x823BE360;
	sub_82E5C928(ctx, base);
	// 823BE360: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BE364: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BE368: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BE36C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BE370: 396A014C  addi r11, r10, 0x14c
	ctx.r[11].s64 = ctx.r[10].s64 + 332;
	// 823BE374: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BE378: 48000008  b 0x823be380
	pc = 0x823BE380; continue 'dispatch;
	// 823BE37C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE380: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE384: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE388: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE38C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE390: 4BFFAA99  bl 0x823b8e28
	ctx.lr = 0x823BE394;
	sub_823B8E28(ctx, base);
	// 823BE394: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE398: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE39C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE3A0: 4BF01C61  bl 0x822c0000
	ctx.lr = 0x823BE3A4;
	sub_822C0000(ctx, base);
	// 823BE3A4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE3A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE3AC: 48DE9E10  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE3B0 size=136
    let mut pc: u32 = 0x823BE3B0;
    'dispatch: loop {
        match pc {
            0x823BE3B0 => {
    //   block [0x823BE3B0..0x823BE438)
	// 823BE3B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE3B4: 48DE9DB9  bl 0x831a816c
	ctx.lr = 0x823BE3B8;
	sub_831A8130(ctx, base);
	// 823BE3B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE3BC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE3C0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE3C4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE3C8: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE3CC: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE3D0: 38600078  li r3, 0x78
	ctx.r[3].s64 = 120;
	// 823BE3D4: 48A34015  bl 0x82df23e8
	ctx.lr = 0x823BE3D8;
	sub_82DF23E8(ctx, base);
	// 823BE3D8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BE3DC: 41820028  beq 0x823be404
	if ctx.cr[0].eq {
	pc = 0x823BE404; continue 'dispatch;
	}
	// 823BE3E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BE3E4: 48A9E545  bl 0x82e5c928
	ctx.lr = 0x823BE3E8;
	sub_82E5C928(ctx, base);
	// 823BE3E8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BE3EC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BE3F0: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BE3F4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BE3F8: 396A0174  addi r11, r10, 0x174
	ctx.r[11].s64 = ctx.r[10].s64 + 372;
	// 823BE3FC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BE400: 48000008  b 0x823be408
	pc = 0x823BE408; continue 'dispatch;
	// 823BE404: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE408: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE40C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE410: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE414: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE418: 4BFFAAD9  bl 0x823b8ef0
	ctx.lr = 0x823BE41C;
	sub_823B8EF0(ctx, base);
	// 823BE41C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE420: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE424: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE428: 4BF01BD9  bl 0x822c0000
	ctx.lr = 0x823BE42C;
	sub_822C0000(ctx, base);
	// 823BE42C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE430: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE434: 48DE9D88  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE438(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE438 size=136
    let mut pc: u32 = 0x823BE438;
    'dispatch: loop {
        match pc {
            0x823BE438 => {
    //   block [0x823BE438..0x823BE4C0)
	// 823BE438: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE43C: 48DE9D31  bl 0x831a816c
	ctx.lr = 0x823BE440;
	sub_831A8130(ctx, base);
	// 823BE440: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE444: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE448: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE44C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE450: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE454: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE458: 386000B0  li r3, 0xb0
	ctx.r[3].s64 = 176;
	// 823BE45C: 48A33F8D  bl 0x82df23e8
	ctx.lr = 0x823BE460;
	sub_82DF23E8(ctx, base);
	// 823BE460: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BE464: 41820028  beq 0x823be48c
	if ctx.cr[0].eq {
	pc = 0x823BE48C; continue 'dispatch;
	}
	// 823BE468: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BE46C: 48A9E4BD  bl 0x82e5c928
	ctx.lr = 0x823BE470;
	sub_82E5C928(ctx, base);
	// 823BE470: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BE474: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BE478: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BE47C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BE480: 396A019C  addi r11, r10, 0x19c
	ctx.r[11].s64 = ctx.r[10].s64 + 412;
	// 823BE484: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BE488: 48000008  b 0x823be490
	pc = 0x823BE490; continue 'dispatch;
	// 823BE48C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE490: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE494: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE498: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE49C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE4A0: 4BFFAB19  bl 0x823b8fb8
	ctx.lr = 0x823BE4A4;
	sub_823B8FB8(ctx, base);
	// 823BE4A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE4A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE4AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE4B0: 4BF01B51  bl 0x822c0000
	ctx.lr = 0x823BE4B4;
	sub_822C0000(ctx, base);
	// 823BE4B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE4B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE4BC: 48DE9D00  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE4C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE4C0 size=136
    let mut pc: u32 = 0x823BE4C0;
    'dispatch: loop {
        match pc {
            0x823BE4C0 => {
    //   block [0x823BE4C0..0x823BE548)
	// 823BE4C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE4C4: 48DE9CA9  bl 0x831a816c
	ctx.lr = 0x823BE4C8;
	sub_831A8130(ctx, base);
	// 823BE4C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE4CC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE4D0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE4D4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE4D8: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE4DC: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE4E0: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BE4E4: 48A33F05  bl 0x82df23e8
	ctx.lr = 0x823BE4E8;
	sub_82DF23E8(ctx, base);
	// 823BE4E8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BE4EC: 41820028  beq 0x823be514
	if ctx.cr[0].eq {
	pc = 0x823BE514; continue 'dispatch;
	}
	// 823BE4F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BE4F4: 48A9E435  bl 0x82e5c928
	ctx.lr = 0x823BE4F8;
	sub_82E5C928(ctx, base);
	// 823BE4F8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BE4FC: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BE500: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BE504: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BE508: 396A01C4  addi r11, r10, 0x1c4
	ctx.r[11].s64 = ctx.r[10].s64 + 452;
	// 823BE50C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BE510: 48000008  b 0x823be518
	pc = 0x823BE518; continue 'dispatch;
	// 823BE514: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE518: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE51C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE520: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE524: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE528: 4BFFAB59  bl 0x823b9080
	ctx.lr = 0x823BE52C;
	sub_823B9080(ctx, base);
	// 823BE52C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE530: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE534: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE538: 4BF01AC9  bl 0x822c0000
	ctx.lr = 0x823BE53C;
	sub_822C0000(ctx, base);
	// 823BE53C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE540: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE544: 48DE9C78  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE548 size=136
    let mut pc: u32 = 0x823BE548;
    'dispatch: loop {
        match pc {
            0x823BE548 => {
    //   block [0x823BE548..0x823BE5D0)
	// 823BE548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE54C: 48DE9C21  bl 0x831a816c
	ctx.lr = 0x823BE550;
	sub_831A8130(ctx, base);
	// 823BE550: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE554: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE558: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE55C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE560: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE564: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE568: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BE56C: 48A33E7D  bl 0x82df23e8
	ctx.lr = 0x823BE570;
	sub_82DF23E8(ctx, base);
	// 823BE570: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BE574: 41820028  beq 0x823be59c
	if ctx.cr[0].eq {
	pc = 0x823BE59C; continue 'dispatch;
	}
	// 823BE578: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BE57C: 48A9E3AD  bl 0x82e5c928
	ctx.lr = 0x823BE580;
	sub_82E5C928(ctx, base);
	// 823BE580: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BE584: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BE588: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BE58C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BE590: 396A01EC  addi r11, r10, 0x1ec
	ctx.r[11].s64 = ctx.r[10].s64 + 492;
	// 823BE594: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BE598: 48000008  b 0x823be5a0
	pc = 0x823BE5A0; continue 'dispatch;
	// 823BE59C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE5A0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE5A4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE5A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE5AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE5B0: 4BFFAB99  bl 0x823b9148
	ctx.lr = 0x823BE5B4;
	sub_823B9148(ctx, base);
	// 823BE5B4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE5B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE5BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE5C0: 4BF01A41  bl 0x822c0000
	ctx.lr = 0x823BE5C4;
	sub_822C0000(ctx, base);
	// 823BE5C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE5C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE5CC: 48DE9BF0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE5D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE5D0 size=136
    let mut pc: u32 = 0x823BE5D0;
    'dispatch: loop {
        match pc {
            0x823BE5D0 => {
    //   block [0x823BE5D0..0x823BE658)
	// 823BE5D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE5D4: 48DE9B99  bl 0x831a816c
	ctx.lr = 0x823BE5D8;
	sub_831A8130(ctx, base);
	// 823BE5D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE5DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE5E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE5E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE5E8: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE5EC: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE5F0: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 823BE5F4: 48A33DF5  bl 0x82df23e8
	ctx.lr = 0x823BE5F8;
	sub_82DF23E8(ctx, base);
	// 823BE5F8: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BE5FC: 41820028  beq 0x823be624
	if ctx.cr[0].eq {
	pc = 0x823BE624; continue 'dispatch;
	}
	// 823BE600: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BE604: 48A9E325  bl 0x82e5c928
	ctx.lr = 0x823BE608;
	sub_82E5C928(ctx, base);
	// 823BE608: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BE60C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BE610: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BE614: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BE618: 396A0214  addi r11, r10, 0x214
	ctx.r[11].s64 = ctx.r[10].s64 + 532;
	// 823BE61C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BE620: 48000008  b 0x823be628
	pc = 0x823BE628; continue 'dispatch;
	// 823BE624: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE628: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE62C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE630: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE634: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE638: 4BFFABD9  bl 0x823b9210
	ctx.lr = 0x823BE63C;
	sub_823B9210(ctx, base);
	// 823BE63C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE640: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE644: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE648: 4BF019B9  bl 0x822c0000
	ctx.lr = 0x823BE64C;
	sub_822C0000(ctx, base);
	// 823BE64C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE650: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE654: 48DE9B68  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE658 size=112
    let mut pc: u32 = 0x823BE658;
    'dispatch: loop {
        match pc {
            0x823BE658 => {
    //   block [0x823BE658..0x823BE6C8)
	// 823BE658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE65C: 48DE9B11  bl 0x831a816c
	ctx.lr = 0x823BE660;
	sub_831A8130(ctx, base);
	// 823BE660: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE664: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE668: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE66C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE670: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE674: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE678: 38600078  li r3, 0x78
	ctx.r[3].s64 = 120;
	// 823BE67C: 48A33D6D  bl 0x82df23e8
	ctx.lr = 0x823BE680;
	sub_82DF23E8(ctx, base);
	// 823BE680: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BE684: 41820010  beq 0x823be694
	if ctx.cr[0].eq {
	pc = 0x823BE694; continue 'dispatch;
	}
	// 823BE688: 4BFFC7F1  bl 0x823bae78
	ctx.lr = 0x823BE68C;
	sub_823BAE78(ctx, base);
	// 823BE68C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BE690: 48000008  b 0x823be698
	pc = 0x823BE698; continue 'dispatch;
	// 823BE694: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE698: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE69C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE6A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE6A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE6A8: 4BFFAC31  bl 0x823b92d8
	ctx.lr = 0x823BE6AC;
	sub_823B92D8(ctx, base);
	// 823BE6AC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE6B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE6B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE6B8: 4BF01949  bl 0x822c0000
	ctx.lr = 0x823BE6BC;
	sub_822C0000(ctx, base);
	// 823BE6BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE6C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE6C4: 48DE9AF8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE6C8 size=136
    let mut pc: u32 = 0x823BE6C8;
    'dispatch: loop {
        match pc {
            0x823BE6C8 => {
    //   block [0x823BE6C8..0x823BE750)
	// 823BE6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE6CC: 48DE9AA1  bl 0x831a816c
	ctx.lr = 0x823BE6D0;
	sub_831A8130(ctx, base);
	// 823BE6D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE6D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE6D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE6DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE6E0: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE6E4: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE6E8: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 823BE6EC: 48A33CFD  bl 0x82df23e8
	ctx.lr = 0x823BE6F0;
	sub_82DF23E8(ctx, base);
	// 823BE6F0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BE6F4: 41820028  beq 0x823be71c
	if ctx.cr[0].eq {
	pc = 0x823BE71C; continue 'dispatch;
	}
	// 823BE6F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BE6FC: 48A9E22D  bl 0x82e5c928
	ctx.lr = 0x823BE700;
	sub_82E5C928(ctx, base);
	// 823BE700: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BE704: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BE708: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BE70C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BE710: 396A0264  addi r11, r10, 0x264
	ctx.r[11].s64 = ctx.r[10].s64 + 612;
	// 823BE714: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BE718: 48000008  b 0x823be720
	pc = 0x823BE720; continue 'dispatch;
	// 823BE71C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE720: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE724: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE728: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE72C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE730: 4BFFAC71  bl 0x823b93a0
	ctx.lr = 0x823BE734;
	sub_823B93A0(ctx, base);
	// 823BE734: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE738: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE73C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE740: 4BF018C1  bl 0x822c0000
	ctx.lr = 0x823BE744;
	sub_822C0000(ctx, base);
	// 823BE744: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE748: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE74C: 48DE9A70  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE750 size=136
    let mut pc: u32 = 0x823BE750;
    'dispatch: loop {
        match pc {
            0x823BE750 => {
    //   block [0x823BE750..0x823BE7D8)
	// 823BE750: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE754: 48DE9A19  bl 0x831a816c
	ctx.lr = 0x823BE758;
	sub_831A8130(ctx, base);
	// 823BE758: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE75C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE760: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE764: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE768: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE76C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE770: 386000C0  li r3, 0xc0
	ctx.r[3].s64 = 192;
	// 823BE774: 48A33C75  bl 0x82df23e8
	ctx.lr = 0x823BE778;
	sub_82DF23E8(ctx, base);
	// 823BE778: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BE77C: 41820028  beq 0x823be7a4
	if ctx.cr[0].eq {
	pc = 0x823BE7A4; continue 'dispatch;
	}
	// 823BE780: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BE784: 48A9E1A5  bl 0x82e5c928
	ctx.lr = 0x823BE788;
	sub_82E5C928(ctx, base);
	// 823BE788: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BE78C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BE790: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BE794: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BE798: 396A028C  addi r11, r10, 0x28c
	ctx.r[11].s64 = ctx.r[10].s64 + 652;
	// 823BE79C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BE7A0: 48000008  b 0x823be7a8
	pc = 0x823BE7A8; continue 'dispatch;
	// 823BE7A4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE7A8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE7AC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE7B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE7B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE7B8: 4BFFACB1  bl 0x823b9468
	ctx.lr = 0x823BE7BC;
	sub_823B9468(ctx, base);
	// 823BE7BC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE7C0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE7C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE7C8: 4BF01839  bl 0x822c0000
	ctx.lr = 0x823BE7CC;
	sub_822C0000(ctx, base);
	// 823BE7CC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE7D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE7D4: 48DE99E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE7D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE7D8 size=112
    let mut pc: u32 = 0x823BE7D8;
    'dispatch: loop {
        match pc {
            0x823BE7D8 => {
    //   block [0x823BE7D8..0x823BE848)
	// 823BE7D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE7DC: 48DE9991  bl 0x831a816c
	ctx.lr = 0x823BE7E0;
	sub_831A8130(ctx, base);
	// 823BE7E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE7E4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE7E8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE7EC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE7F0: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE7F4: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE7F8: 386000E0  li r3, 0xe0
	ctx.r[3].s64 = 224;
	// 823BE7FC: 48A33BED  bl 0x82df23e8
	ctx.lr = 0x823BE800;
	sub_82DF23E8(ctx, base);
	// 823BE800: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BE804: 41820010  beq 0x823be814
	if ctx.cr[0].eq {
	pc = 0x823BE814; continue 'dispatch;
	}
	// 823BE808: 4BFFC6C1  bl 0x823baec8
	ctx.lr = 0x823BE80C;
	sub_823BAEC8(ctx, base);
	// 823BE80C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BE810: 48000008  b 0x823be818
	pc = 0x823BE818; continue 'dispatch;
	// 823BE814: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE818: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE81C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE820: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE824: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE828: 4BFFAD09  bl 0x823b9530
	ctx.lr = 0x823BE82C;
	sub_823B9530(ctx, base);
	// 823BE82C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE830: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE834: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE838: 4BF017C9  bl 0x822c0000
	ctx.lr = 0x823BE83C;
	sub_822C0000(ctx, base);
	// 823BE83C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE840: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE844: 48DE9978  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE848(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE848 size=136
    let mut pc: u32 = 0x823BE848;
    'dispatch: loop {
        match pc {
            0x823BE848 => {
    //   block [0x823BE848..0x823BE8D0)
	// 823BE848: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE84C: 48DE9921  bl 0x831a816c
	ctx.lr = 0x823BE850;
	sub_831A8130(ctx, base);
	// 823BE850: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE854: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE858: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE85C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE860: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE864: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE868: 38600070  li r3, 0x70
	ctx.r[3].s64 = 112;
	// 823BE86C: 48A33B7D  bl 0x82df23e8
	ctx.lr = 0x823BE870;
	sub_82DF23E8(ctx, base);
	// 823BE870: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BE874: 41820028  beq 0x823be89c
	if ctx.cr[0].eq {
	pc = 0x823BE89C; continue 'dispatch;
	}
	// 823BE878: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BE87C: 48A9E0AD  bl 0x82e5c928
	ctx.lr = 0x823BE880;
	sub_82E5C928(ctx, base);
	// 823BE880: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BE884: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BE888: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BE88C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BE890: 396A02DC  addi r11, r10, 0x2dc
	ctx.r[11].s64 = ctx.r[10].s64 + 732;
	// 823BE894: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BE898: 48000008  b 0x823be8a0
	pc = 0x823BE8A0; continue 'dispatch;
	// 823BE89C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE8A0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE8A4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE8A8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE8AC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE8B0: 4BFFAE11  bl 0x823b96c0
	ctx.lr = 0x823BE8B4;
	sub_823B96C0(ctx, base);
	// 823BE8B4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE8B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE8BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE8C0: 4BF01741  bl 0x822c0000
	ctx.lr = 0x823BE8C4;
	sub_822C0000(ctx, base);
	// 823BE8C4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE8C8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE8CC: 48DE98F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE8D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE8D0 size=112
    let mut pc: u32 = 0x823BE8D0;
    'dispatch: loop {
        match pc {
            0x823BE8D0 => {
    //   block [0x823BE8D0..0x823BE940)
	// 823BE8D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE8D4: 48DE9899  bl 0x831a816c
	ctx.lr = 0x823BE8D8;
	sub_831A8130(ctx, base);
	// 823BE8D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE8DC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE8E0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE8E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE8E8: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE8EC: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE8F0: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BE8F4: 48A33AF5  bl 0x82df23e8
	ctx.lr = 0x823BE8F8;
	sub_82DF23E8(ctx, base);
	// 823BE8F8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BE8FC: 41820010  beq 0x823be90c
	if ctx.cr[0].eq {
	pc = 0x823BE90C; continue 'dispatch;
	}
	// 823BE900: 4BFFC629  bl 0x823baf28
	ctx.lr = 0x823BE904;
	sub_823BAF28(ctx, base);
	// 823BE904: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BE908: 48000008  b 0x823be910
	pc = 0x823BE910; continue 'dispatch;
	// 823BE90C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE910: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE914: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE918: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE91C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE920: 4BFFAE69  bl 0x823b9788
	ctx.lr = 0x823BE924;
	sub_823B9788(ctx, base);
	// 823BE924: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE928: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE92C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE930: 4BF016D1  bl 0x822c0000
	ctx.lr = 0x823BE934;
	sub_822C0000(ctx, base);
	// 823BE934: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE938: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE93C: 48DE9880  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE940 size=136
    let mut pc: u32 = 0x823BE940;
    'dispatch: loop {
        match pc {
            0x823BE940 => {
    //   block [0x823BE940..0x823BE9C8)
	// 823BE940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE944: 48DE9829  bl 0x831a816c
	ctx.lr = 0x823BE948;
	sub_831A8130(ctx, base);
	// 823BE948: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE94C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE950: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE954: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE958: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE95C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE960: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BE964: 48A33A85  bl 0x82df23e8
	ctx.lr = 0x823BE968;
	sub_82DF23E8(ctx, base);
	// 823BE968: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BE96C: 41820028  beq 0x823be994
	if ctx.cr[0].eq {
	pc = 0x823BE994; continue 'dispatch;
	}
	// 823BE970: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BE974: 48A9DFB5  bl 0x82e5c928
	ctx.lr = 0x823BE978;
	sub_82E5C928(ctx, base);
	// 823BE978: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BE97C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BE980: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BE984: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BE988: 396A032C  addi r11, r10, 0x32c
	ctx.r[11].s64 = ctx.r[10].s64 + 812;
	// 823BE98C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BE990: 48000008  b 0x823be998
	pc = 0x823BE998; continue 'dispatch;
	// 823BE994: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BE998: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BE99C: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BE9A0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE9A4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE9A8: 4BFFAEA9  bl 0x823b9850
	ctx.lr = 0x823BE9AC;
	sub_823B9850(ctx, base);
	// 823BE9AC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BE9B0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BE9B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BE9B8: 4BF01649  bl 0x822c0000
	ctx.lr = 0x823BE9BC;
	sub_822C0000(ctx, base);
	// 823BE9BC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BE9C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BE9C4: 48DE97F8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BE9C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BE9C8 size=136
    let mut pc: u32 = 0x823BE9C8;
    'dispatch: loop {
        match pc {
            0x823BE9C8 => {
    //   block [0x823BE9C8..0x823BEA50)
	// 823BE9C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BE9CC: 48DE97A1  bl 0x831a816c
	ctx.lr = 0x823BE9D0;
	sub_831A8130(ctx, base);
	// 823BE9D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BE9D4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BE9D8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BE9DC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BE9E0: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BE9E4: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BE9E8: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 823BE9EC: 48A339FD  bl 0x82df23e8
	ctx.lr = 0x823BE9F0;
	sub_82DF23E8(ctx, base);
	// 823BE9F0: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BE9F4: 41820028  beq 0x823bea1c
	if ctx.cr[0].eq {
	pc = 0x823BEA1C; continue 'dispatch;
	}
	// 823BE9F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BE9FC: 48A9DF2D  bl 0x82e5c928
	ctx.lr = 0x823BEA00;
	sub_82E5C928(ctx, base);
	// 823BEA00: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BEA04: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BEA08: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BEA0C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BEA10: 396A0354  addi r11, r10, 0x354
	ctx.r[11].s64 = ctx.r[10].s64 + 852;
	// 823BEA14: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BEA18: 48000008  b 0x823bea20
	pc = 0x823BEA20; continue 'dispatch;
	// 823BEA1C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BEA20: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BEA24: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BEA28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BEA2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BEA30: 4BFFAEE9  bl 0x823b9918
	ctx.lr = 0x823BEA34;
	sub_823B9918(ctx, base);
	// 823BEA34: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BEA38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BEA3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BEA40: 4BF015C1  bl 0x822c0000
	ctx.lr = 0x823BEA44;
	sub_822C0000(ctx, base);
	// 823BEA44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BEA48: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BEA4C: 48DE9770  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BEA50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BEA50 size=136
    let mut pc: u32 = 0x823BEA50;
    'dispatch: loop {
        match pc {
            0x823BEA50 => {
    //   block [0x823BEA50..0x823BEAD8)
	// 823BEA50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BEA54: 48DE9719  bl 0x831a816c
	ctx.lr = 0x823BEA58;
	sub_831A8130(ctx, base);
	// 823BEA58: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BEA5C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BEA60: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BEA64: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BEA68: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BEA6C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BEA70: 3860006C  li r3, 0x6c
	ctx.r[3].s64 = 108;
	// 823BEA74: 48A33975  bl 0x82df23e8
	ctx.lr = 0x823BEA78;
	sub_82DF23E8(ctx, base);
	// 823BEA78: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BEA7C: 41820028  beq 0x823beaa4
	if ctx.cr[0].eq {
	pc = 0x823BEAA4; continue 'dispatch;
	}
	// 823BEA80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BEA84: 48A9DEA5  bl 0x82e5c928
	ctx.lr = 0x823BEA88;
	sub_82E5C928(ctx, base);
	// 823BEA88: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BEA8C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BEA90: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BEA94: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BEA98: 396A037C  addi r11, r10, 0x37c
	ctx.r[11].s64 = ctx.r[10].s64 + 892;
	// 823BEA9C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BEAA0: 48000008  b 0x823beaa8
	pc = 0x823BEAA8; continue 'dispatch;
	// 823BEAA4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BEAA8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BEAAC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BEAB0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BEAB4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BEAB8: 4BFFAF29  bl 0x823b99e0
	ctx.lr = 0x823BEABC;
	sub_823B99E0(ctx, base);
	// 823BEABC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BEAC0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BEAC4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BEAC8: 4BF01539  bl 0x822c0000
	ctx.lr = 0x823BEACC;
	sub_822C0000(ctx, base);
	// 823BEACC: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BEAD0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BEAD4: 48DE96E8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BEAD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BEAD8 size=136
    let mut pc: u32 = 0x823BEAD8;
    'dispatch: loop {
        match pc {
            0x823BEAD8 => {
    //   block [0x823BEAD8..0x823BEB60)
	// 823BEAD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BEADC: 48DE9691  bl 0x831a816c
	ctx.lr = 0x823BEAE0;
	sub_831A8130(ctx, base);
	// 823BEAE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BEAE4: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BEAE8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BEAEC: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BEAF0: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BEAF4: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BEAF8: 38600074  li r3, 0x74
	ctx.r[3].s64 = 116;
	// 823BEAFC: 48A338ED  bl 0x82df23e8
	ctx.lr = 0x823BEB00;
	sub_82DF23E8(ctx, base);
	// 823BEB00: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BEB04: 41820028  beq 0x823beb2c
	if ctx.cr[0].eq {
	pc = 0x823BEB2C; continue 'dispatch;
	}
	// 823BEB08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BEB0C: 48A9DE1D  bl 0x82e5c928
	ctx.lr = 0x823BEB10;
	sub_82E5C928(ctx, base);
	// 823BEB10: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BEB14: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BEB18: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BEB1C: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BEB20: 396A03A4  addi r11, r10, 0x3a4
	ctx.r[11].s64 = ctx.r[10].s64 + 932;
	// 823BEB24: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BEB28: 48000008  b 0x823beb30
	pc = 0x823BEB30; continue 'dispatch;
	// 823BEB2C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BEB30: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BEB34: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BEB38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BEB3C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BEB40: 4BFFAF69  bl 0x823b9aa8
	ctx.lr = 0x823BEB44;
	sub_823B9AA8(ctx, base);
	// 823BEB44: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BEB48: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BEB4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BEB50: 4BF014B1  bl 0x822c0000
	ctx.lr = 0x823BEB54;
	sub_822C0000(ctx, base);
	// 823BEB54: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BEB58: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BEB5C: 48DE9660  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BEB60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BEB60 size=112
    let mut pc: u32 = 0x823BEB60;
    'dispatch: loop {
        match pc {
            0x823BEB60 => {
    //   block [0x823BEB60..0x823BEBD0)
	// 823BEB60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BEB64: 48DE9609  bl 0x831a816c
	ctx.lr = 0x823BEB68;
	sub_831A8130(ctx, base);
	// 823BEB68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BEB6C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BEB70: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BEB74: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BEB78: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BEB7C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BEB80: 38600090  li r3, 0x90
	ctx.r[3].s64 = 144;
	// 823BEB84: 48A33865  bl 0x82df23e8
	ctx.lr = 0x823BEB88;
	sub_82DF23E8(ctx, base);
	// 823BEB88: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BEB8C: 41820010  beq 0x823beb9c
	if ctx.cr[0].eq {
	pc = 0x823BEB9C; continue 'dispatch;
	}
	// 823BEB90: 48007EC9  bl 0x823c6a58
	ctx.lr = 0x823BEB94;
	sub_823C6A58(ctx, base);
	// 823BEB94: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BEB98: 48000008  b 0x823beba0
	pc = 0x823BEBA0; continue 'dispatch;
	// 823BEB9C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BEBA0: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BEBA4: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BEBA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BEBAC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BEBB0: 4BFFAFC1  bl 0x823b9b70
	ctx.lr = 0x823BEBB4;
	sub_823B9B70(ctx, base);
	// 823BEBB4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BEBB8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BEBBC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BEBC0: 4BF01441  bl 0x822c0000
	ctx.lr = 0x823BEBC4;
	sub_822C0000(ctx, base);
	// 823BEBC4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BEBC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BEBCC: 48DE95F0  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BEBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BEBD0 size=112
    let mut pc: u32 = 0x823BEBD0;
    'dispatch: loop {
        match pc {
            0x823BEBD0 => {
    //   block [0x823BEBD0..0x823BEC40)
	// 823BEBD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BEBD4: 48DE9599  bl 0x831a816c
	ctx.lr = 0x823BEBD8;
	sub_831A8130(ctx, base);
	// 823BEBD8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BEBDC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BEBE0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BEBE4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BEBE8: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BEBEC: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BEBF0: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 823BEBF4: 48A337F5  bl 0x82df23e8
	ctx.lr = 0x823BEBF8;
	sub_82DF23E8(ctx, base);
	// 823BEBF8: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BEBFC: 41820010  beq 0x823bec0c
	if ctx.cr[0].eq {
	pc = 0x823BEC0C; continue 'dispatch;
	}
	// 823BEC00: 480066E1  bl 0x823c52e0
	ctx.lr = 0x823BEC04;
	sub_823C52E0(ctx, base);
	// 823BEC04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BEC08: 48000008  b 0x823bec10
	pc = 0x823BEC10; continue 'dispatch;
	// 823BEC0C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BEC10: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BEC14: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BEC18: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BEC1C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BEC20: 4BFFB019  bl 0x823b9c38
	ctx.lr = 0x823BEC24;
	sub_823B9C38(ctx, base);
	// 823BEC24: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BEC28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BEC2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BEC30: 4BF013D1  bl 0x822c0000
	ctx.lr = 0x823BEC34;
	sub_822C0000(ctx, base);
	// 823BEC34: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BEC38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BEC3C: 48DE9580  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BEC40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BEC40 size=112
    let mut pc: u32 = 0x823BEC40;
    'dispatch: loop {
        match pc {
            0x823BEC40 => {
    //   block [0x823BEC40..0x823BECB0)
	// 823BEC40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BEC44: 48DE9529  bl 0x831a816c
	ctx.lr = 0x823BEC48;
	sub_831A8130(ctx, base);
	// 823BEC48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BEC4C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BEC50: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BEC54: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BEC58: 388B5444  addi r4, r11, 0x5444
	ctx.r[4].s64 = ctx.r[11].s64 + 21572;
	// 823BEC5C: 38A00053  li r5, 0x53
	ctx.r[5].s64 = 83;
	// 823BEC60: 38600060  li r3, 0x60
	ctx.r[3].s64 = 96;
	// 823BEC64: 48A33785  bl 0x82df23e8
	ctx.lr = 0x823BEC68;
	sub_82DF23E8(ctx, base);
	// 823BEC68: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BEC6C: 41820010  beq 0x823bec7c
	if ctx.cr[0].eq {
	pc = 0x823BEC7C; continue 'dispatch;
	}
	// 823BEC70: 4800BDD1  bl 0x823caa40
	ctx.lr = 0x823BEC74;
	sub_823CAA40(ctx, base);
	// 823BEC74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BEC78: 48000008  b 0x823bec80
	pc = 0x823BEC80; continue 'dispatch;
	// 823BEC7C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BEC80: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BEC84: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BEC88: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BEC8C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BEC90: 4BFFB071  bl 0x823b9d00
	ctx.lr = 0x823BEC94;
	sub_823B9D00(ctx, base);
	// 823BEC94: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BEC98: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BEC9C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BECA0: 4BF01361  bl 0x822c0000
	ctx.lr = 0x823BECA4;
	sub_822C0000(ctx, base);
	// 823BECA4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BECA8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BECAC: 48DE9510  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BECB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BECB0 size=212
    let mut pc: u32 = 0x823BECB0;
    'dispatch: loop {
        match pc {
            0x823BECB0 => {
    //   block [0x823BECB0..0x823BED84)
	// 823BECB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BECB4: 48DE94B5  bl 0x831a8168
	ctx.lr = 0x823BECB8;
	sub_831A8130(ctx, base);
	// 823BECB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BECBC: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 823BECC0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BECC4: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 823BECC8: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 823BECCC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823BECD0: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BECD4: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 823BECD8: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 823BECDC: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 823BECE0: 48A33709  bl 0x82df23e8
	ctx.lr = 0x823BECE4;
	sub_82DF23E8(ctx, base);
	// 823BECE4: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BECE8: 41820050  beq 0x823bed38
	if ctx.cr[0].eq {
	pc = 0x823BED38; continue 'dispatch;
	}
	// 823BECEC: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BECF0: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BECF4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BECF8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823BECFC: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823BED00: 419A0024  beq cr6, 0x823bed24
	if ctx.cr[6].eq {
	pc = 0x823BED24; continue 'dispatch;
	}
	// 823BED04: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 823BED08: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823BED0C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823BED10: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823BED14: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 823BED18: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823BED1C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823BED20: 4082FFE8  bne 0x823bed08
	if !ctx.cr[0].eq {
	pc = 0x823BED08; continue 'dispatch;
	}
	// 823BED24: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823BED28: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 823BED2C: 4879287D  bl 0x82b515a8
	ctx.lr = 0x823BED30;
	sub_82B515A8(ctx, base);
	// 823BED30: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BED34: 48000008  b 0x823bed3c
	pc = 0x823BED3C; continue 'dispatch;
	// 823BED38: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BED3C: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BED40: 3BDC0004  addi r30, r28, 4
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	// 823BED44: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BED48: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BED4C: 4BFFB07D  bl 0x823b9dc8
	ctx.lr = 0x823BED50;
	sub_823B9DC8(ctx, base);
	// 823BED50: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BED54: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BED58: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BED5C: 4BF012A5  bl 0x822c0000
	ctx.lr = 0x823BED60;
	sub_822C0000(ctx, base);
	// 823BED60: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BED64: 41820014  beq 0x823bed78
	if ctx.cr[0].eq {
	pc = 0x823BED78; continue 'dispatch;
	}
	// 823BED68: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823BED6C: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BED70: 419A0008  beq cr6, 0x823bed78
	if ctx.cr[6].eq {
	pc = 0x823BED78; continue 'dispatch;
	}
	// 823BED74: 4BF01B1D  bl 0x822c0890
	ctx.lr = 0x823BED78;
	sub_822C0890(ctx, base);
	// 823BED78: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 823BED7C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823BED80: 48DE9438  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BED88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BED88 size=112
    let mut pc: u32 = 0x823BED88;
    'dispatch: loop {
        match pc {
            0x823BED88 => {
    //   block [0x823BED88..0x823BEDF8)
	// 823BED88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BED8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BED90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BED94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BED98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BED9C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BEDA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BEDA4: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 823BEDA8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823BEDAC: 4BFFB0E5  bl 0x823b9e90
	ctx.lr = 0x823BEDB0;
	sub_823B9E90(ctx, base);
	// 823BEDB0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823BEDB4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823BEDB8: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823BEDBC: 4BF01245  bl 0x822c0000
	ctx.lr = 0x823BEDC0;
	sub_822C0000(ctx, base);
	// 823BEDC0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823BEDC4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823BEDC8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BEDCC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BEDD0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BEDD4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823BEDD8: 419A0008  beq cr6, 0x823bede0
	if ctx.cr[6].eq {
	pc = 0x823BEDE0; continue 'dispatch;
	}
	// 823BEDDC: 4BF01AB5  bl 0x822c0890
	ctx.lr = 0x823BEDE0;
	sub_822C0890(ctx, base);
	// 823BEDE0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BEDE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BEDE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BEDEC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BEDF0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BEDF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BEDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BEDF8 size=112
    let mut pc: u32 = 0x823BEDF8;
    'dispatch: loop {
        match pc {
            0x823BEDF8 => {
    //   block [0x823BEDF8..0x823BEE68)
	// 823BEDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BEDFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BEE00: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BEE04: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BEE08: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BEE0C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BEE10: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BEE14: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 823BEE18: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823BEE1C: 4BFFB13D  bl 0x823b9f58
	ctx.lr = 0x823BEE20;
	sub_823B9F58(ctx, base);
	// 823BEE20: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823BEE24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823BEE28: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823BEE2C: 4BF011D5  bl 0x822c0000
	ctx.lr = 0x823BEE30;
	sub_822C0000(ctx, base);
	// 823BEE30: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823BEE34: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823BEE38: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BEE3C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BEE40: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BEE44: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823BEE48: 419A0008  beq cr6, 0x823bee50
	if ctx.cr[6].eq {
	pc = 0x823BEE50; continue 'dispatch;
	}
	// 823BEE4C: 4BF01A45  bl 0x822c0890
	ctx.lr = 0x823BEE50;
	sub_822C0890(ctx, base);
	// 823BEE50: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BEE54: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BEE58: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BEE5C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BEE60: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BEE64: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BEE68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BEE68 size=112
    let mut pc: u32 = 0x823BEE68;
    'dispatch: loop {
        match pc {
            0x823BEE68 => {
    //   block [0x823BEE68..0x823BEED8)
	// 823BEE68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BEE6C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BEE70: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BEE74: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BEE78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BEE7C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BEE80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BEE84: 93C10050  stw r30, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[30].u32 ) };
	// 823BEE88: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823BEE8C: 4BFFB195  bl 0x823ba020
	ctx.lr = 0x823BEE90;
	sub_823BA020(ctx, base);
	// 823BEE90: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 823BEE94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823BEE98: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823BEE9C: 4BF01165  bl 0x822c0000
	ctx.lr = 0x823BEEA0;
	sub_822C0000(ctx, base);
	// 823BEEA0: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823BEEA4: 81410054  lwz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823BEEA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BEEAC: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BEEB0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BEEB4: 915F0004  stw r10, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 823BEEB8: 419A0008  beq cr6, 0x823beec0
	if ctx.cr[6].eq {
	pc = 0x823BEEC0; continue 'dispatch;
	}
	// 823BEEBC: 4BF019D5  bl 0x822c0890
	ctx.lr = 0x823BEEC0;
	sub_822C0890(ctx, base);
	// 823BEEC0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BEEC4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BEEC8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BEECC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BEED0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BEED4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BEED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BEED8 size=212
    let mut pc: u32 = 0x823BEED8;
    'dispatch: loop {
        match pc {
            0x823BEED8 => {
    //   block [0x823BEED8..0x823BEFAC)
	// 823BEED8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BEEDC: 48DE928D  bl 0x831a8168
	ctx.lr = 0x823BEEE0;
	sub_831A8130(ctx, base);
	// 823BEEE0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BEEE4: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 823BEEE8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BEEEC: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 823BEEF0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 823BEEF4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823BEEF8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BEEFC: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 823BEF00: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 823BEF04: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 823BEF08: 48A334E1  bl 0x82df23e8
	ctx.lr = 0x823BEF0C;
	sub_82DF23E8(ctx, base);
	// 823BEF0C: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BEF10: 41820050  beq 0x823bef60
	if ctx.cr[0].eq {
	pc = 0x823BEF60; continue 'dispatch;
	}
	// 823BEF14: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BEF18: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BEF1C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BEF20: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823BEF24: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 823BEF28: 419A0024  beq cr6, 0x823bef4c
	if ctx.cr[6].eq {
	pc = 0x823BEF4C; continue 'dispatch;
	}
	// 823BEF2C: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 823BEF30: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823BEF34: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823BEF38: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823BEF3C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 823BEF40: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823BEF44: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823BEF48: 4082FFE8  bne 0x823bef30
	if !ctx.cr[0].eq {
	pc = 0x823BEF30; continue 'dispatch;
	}
	// 823BEF4C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823BEF50: 3BA00001  li r29, 1
	ctx.r[29].s64 = 1;
	// 823BEF54: 48792C55  bl 0x82b51ba8
	ctx.lr = 0x823BEF58;
	sub_82B51BA8(ctx, base);
	// 823BEF58: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BEF5C: 48000008  b 0x823bef64
	pc = 0x823BEF64; continue 'dispatch;
	// 823BEF60: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BEF64: 93FC0000  stw r31, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BEF68: 3BDC0004  addi r30, r28, 4
	ctx.r[30].s64 = ctx.r[28].s64 + 4;
	// 823BEF6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BEF70: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BEF74: 4BFFB175  bl 0x823ba0e8
	ctx.lr = 0x823BEF78;
	sub_823BA0E8(ctx, base);
	// 823BEF78: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BEF7C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BEF80: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BEF84: 4BF0107D  bl 0x822c0000
	ctx.lr = 0x823BEF88;
	sub_822C0000(ctx, base);
	// 823BEF88: 57AB07FF  clrlwi. r11, r29, 0x1f
	ctx.r[11].u64 = ctx.r[29].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BEF8C: 41820014  beq 0x823befa0
	if ctx.cr[0].eq {
	pc = 0x823BEFA0; continue 'dispatch;
	}
	// 823BEF90: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823BEF94: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BEF98: 419A0008  beq cr6, 0x823befa0
	if ctx.cr[6].eq {
	pc = 0x823BEFA0; continue 'dispatch;
	}
	// 823BEF9C: 4BF018F5  bl 0x822c0890
	ctx.lr = 0x823BEFA0;
	sub_822C0890(ctx, base);
	// 823BEFA0: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 823BEFA4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823BEFA8: 48DE9210  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BEFB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BEFB0 size=120
    let mut pc: u32 = 0x823BEFB0;
    'dispatch: loop {
        match pc {
            0x823BEFB0 => {
    //   block [0x823BEFB0..0x823BF028)
	// 823BEFB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BEFB4: 48DE91B9  bl 0x831a816c
	ctx.lr = 0x823BEFB8;
	sub_831A8130(ctx, base);
	// 823BEFB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BEFBC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BEFC0: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BEFC4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 823BEFC8: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BEFCC: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 823BEFD0: 38A00089  li r5, 0x89
	ctx.r[5].s64 = 137;
	// 823BEFD4: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 823BEFD8: 48A33411  bl 0x82df23e8
	ctx.lr = 0x823BEFDC;
	sub_82DF23E8(ctx, base);
	// 823BEFDC: 28030000  cmplwi r3, 0
	ctx.cr[0].compare_u32(ctx.r[0].u32, 0 as u32, &mut ctx.xer);
	// 823BEFE0: 41820014  beq 0x823beff4
	if ctx.cr[0].eq {
	pc = 0x823BEFF4; continue 'dispatch;
	}
	// 823BEFE4: 809F0000  lwz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BEFE8: 48556809  bl 0x829157f0
	ctx.lr = 0x823BEFEC;
	sub_829157F0(ctx, base);
	// 823BEFEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BEFF0: 48000008  b 0x823beff8
	pc = 0x823BEFF8; continue 'dispatch;
	// 823BEFF4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BEFF8: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BEFFC: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BF000: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BF004: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BF008: 4BFFB1A9  bl 0x823ba1b0
	ctx.lr = 0x823BF00C;
	sub_823BA1B0(ctx, base);
	// 823BF00C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BF010: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BF014: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BF018: 4BF00FE9  bl 0x822c0000
	ctx.lr = 0x823BF01C;
	sub_822C0000(ctx, base);
	// 823BF01C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BF020: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF024: 48DE9198  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF028 size=132
    let mut pc: u32 = 0x823BF028;
    'dispatch: loop {
        match pc {
            0x823BF028 => {
    //   block [0x823BF028..0x823BF0AC)
	// 823BF028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF02C: 48DE9141  bl 0x831a816c
	ctx.lr = 0x823BF030;
	sub_831A8130(ctx, base);
	// 823BF030: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF034: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 823BF038: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 823BF03C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BF040: 388BA66C  addi r4, r11, -0x5994
	ctx.r[4].s64 = ctx.r[11].s64 + -22932;
	// 823BF044: 38A00073  li r5, 0x73
	ctx.r[5].s64 = 115;
	// 823BF048: 3860001C  li r3, 0x1c
	ctx.r[3].s64 = 28;
	// 823BF04C: 48A3339D  bl 0x82df23e8
	ctx.lr = 0x823BF050;
	sub_82DF23E8(ctx, base);
	// 823BF050: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BF054: 41820024  beq 0x823bf078
	if ctx.cr[0].eq {
	pc = 0x823BF078; continue 'dispatch;
	}
	// 823BF058: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF05C: 48A9A625  bl 0x82e59680
	ctx.lr = 0x823BF060;
	sub_82E59680(ctx, base);
	// 823BF060: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BF064: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BF068: 997F0018  stb r11, 0x18(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u8 ) };
	// 823BF06C: 396ABF74  addi r11, r10, -0x408c
	ctx.r[11].s64 = ctx.r[10].s64 + -16524;
	// 823BF070: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BF074: 48000008  b 0x823bf07c
	pc = 0x823BF07C; continue 'dispatch;
	// 823BF078: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BF07C: 93FD0000  stw r31, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[31].u32 ) };
	// 823BF080: 3BDD0004  addi r30, r29, 4
	ctx.r[30].s64 = ctx.r[29].s64 + 4;
	// 823BF084: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BF088: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BF08C: 4BFE4C6D  bl 0x823a3cf8
	ctx.lr = 0x823BF090;
	sub_823A3CF8(ctx, base);
	// 823BF090: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BF094: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BF098: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BF09C: 4BF00F65  bl 0x822c0000
	ctx.lr = 0x823BF0A0;
	sub_822C0000(ctx, base);
	// 823BF0A0: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BF0A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF0A8: 48DE9114  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF0B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF0B0 size=76
    let mut pc: u32 = 0x823BF0B0;
    'dispatch: loop {
        match pc {
            0x823BF0B0 => {
    //   block [0x823BF0B0..0x823BF0FC)
	// 823BF0B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF0B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF0B8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF0BC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF0C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF0C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF0C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF0CC: 4BFFBEAD  bl 0x823baf78
	ctx.lr = 0x823BF0D0;
	sub_823BAF78(ctx, base);
	// 823BF0D0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF0D4: 4182000C  beq 0x823bf0e0
	if ctx.cr[0].eq {
	pc = 0x823BF0E0; continue 'dispatch;
	}
	// 823BF0D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF0DC: 48A332FD  bl 0x82df23d8
	ctx.lr = 0x823BF0E0;
	sub_82DF23D8(ctx, base);
	// 823BF0E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF0E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF0E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF0EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF0F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF0F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF0F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF100 size=76
    let mut pc: u32 = 0x823BF100;
    'dispatch: loop {
        match pc {
            0x823BF100 => {
    //   block [0x823BF100..0x823BF14C)
	// 823BF100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF104: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF108: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF10C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF110: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF114: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF118: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF11C: 4BFFBEAD  bl 0x823bafc8
	ctx.lr = 0x823BF120;
	sub_823BAFC8(ctx, base);
	// 823BF120: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF124: 4182000C  beq 0x823bf130
	if ctx.cr[0].eq {
	pc = 0x823BF130; continue 'dispatch;
	}
	// 823BF128: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF12C: 48A332AD  bl 0x82df23d8
	ctx.lr = 0x823BF130;
	sub_82DF23D8(ctx, base);
	// 823BF130: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF134: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF138: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF13C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF140: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF144: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF148: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF150 size=104
    let mut pc: u32 = 0x823BF150;
    'dispatch: loop {
        match pc {
            0x823BF150 => {
    //   block [0x823BF150..0x823BF1B8)
	// 823BF150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF158: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF15C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF160: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF164: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF168: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF16C: 387F0068  addi r3, r31, 0x68
	ctx.r[3].s64 = ctx.r[31].s64 + 104;
	// 823BF170: 48A342B9  bl 0x82df3428
	ctx.lr = 0x823BF174;
	sub_82DF3428(ctx, base);
	// 823BF174: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BF178: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BF17C: 419A0008  beq cr6, 0x823bf184
	if ctx.cr[6].eq {
	pc = 0x823BF184; continue 'dispatch;
	}
	// 823BF180: 4BF01711  bl 0x822c0890
	ctx.lr = 0x823BF184;
	sub_822C0890(ctx, base);
	// 823BF184: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF188: 48A9D401  bl 0x82e5c588
	ctx.lr = 0x823BF18C;
	sub_82E5C588(ctx, base);
	// 823BF18C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF190: 4182000C  beq 0x823bf19c
	if ctx.cr[0].eq {
	pc = 0x823BF19C; continue 'dispatch;
	}
	// 823BF194: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF198: 48A33241  bl 0x82df23d8
	ctx.lr = 0x823BF19C;
	sub_82DF23D8(ctx, base);
	// 823BF19C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF1A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF1A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF1A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF1AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF1B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF1B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF1B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF1B8 size=76
    let mut pc: u32 = 0x823BF1B8;
    'dispatch: loop {
        match pc {
            0x823BF1B8 => {
    //   block [0x823BF1B8..0x823BF204)
	// 823BF1B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF1BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF1C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF1C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF1C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF1CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF1D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF1D4: 4BFFBE4D  bl 0x823bb020
	ctx.lr = 0x823BF1D8;
	sub_823BB020(ctx, base);
	// 823BF1D8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF1DC: 4182000C  beq 0x823bf1e8
	if ctx.cr[0].eq {
	pc = 0x823BF1E8; continue 'dispatch;
	}
	// 823BF1E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF1E4: 48A331F5  bl 0x82df23d8
	ctx.lr = 0x823BF1E8;
	sub_82DF23D8(ctx, base);
	// 823BF1E8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF1EC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF1F0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF1F4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF1F8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF1FC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF200: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF208 size=76
    let mut pc: u32 = 0x823BF208;
    'dispatch: loop {
        match pc {
            0x823BF208 => {
    //   block [0x823BF208..0x823BF254)
	// 823BF208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF20C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF210: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF214: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF218: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF21C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF220: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF224: 4BFFBE55  bl 0x823bb078
	ctx.lr = 0x823BF228;
	sub_823BB078(ctx, base);
	// 823BF228: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF22C: 4182000C  beq 0x823bf238
	if ctx.cr[0].eq {
	pc = 0x823BF238; continue 'dispatch;
	}
	// 823BF230: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF234: 48A331A5  bl 0x82df23d8
	ctx.lr = 0x823BF238;
	sub_82DF23D8(ctx, base);
	// 823BF238: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF23C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF240: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF244: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF248: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF24C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF250: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF258 size=76
    let mut pc: u32 = 0x823BF258;
    'dispatch: loop {
        match pc {
            0x823BF258 => {
    //   block [0x823BF258..0x823BF2A4)
	// 823BF258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF25C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF260: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF264: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF26C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF270: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF274: 4BFFBE55  bl 0x823bb0c8
	ctx.lr = 0x823BF278;
	sub_823BB0C8(ctx, base);
	// 823BF278: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF27C: 4182000C  beq 0x823bf288
	if ctx.cr[0].eq {
	pc = 0x823BF288; continue 'dispatch;
	}
	// 823BF280: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF284: 48A33155  bl 0x82df23d8
	ctx.lr = 0x823BF288;
	sub_82DF23D8(ctx, base);
	// 823BF288: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF28C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF290: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF294: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF298: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF29C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF2A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF2A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF2A8 size=76
    let mut pc: u32 = 0x823BF2A8;
    'dispatch: loop {
        match pc {
            0x823BF2A8 => {
    //   block [0x823BF2A8..0x823BF2F4)
	// 823BF2A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF2AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF2B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF2B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF2B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF2BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF2C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF2C4: 4BFFBE55  bl 0x823bb118
	ctx.lr = 0x823BF2C8;
	sub_823BB118(ctx, base);
	// 823BF2C8: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF2CC: 4182000C  beq 0x823bf2d8
	if ctx.cr[0].eq {
	pc = 0x823BF2D8; continue 'dispatch;
	}
	// 823BF2D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF2D4: 48A33105  bl 0x82df23d8
	ctx.lr = 0x823BF2D8;
	sub_82DF23D8(ctx, base);
	// 823BF2D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF2DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF2E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF2E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF2E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF2EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF2F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF2F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF2F8 size=76
    let mut pc: u32 = 0x823BF2F8;
    'dispatch: loop {
        match pc {
            0x823BF2F8 => {
    //   block [0x823BF2F8..0x823BF344)
	// 823BF2F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF2FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF300: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF304: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF308: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF30C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF310: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF314: 4BFFBE5D  bl 0x823bb170
	ctx.lr = 0x823BF318;
	sub_823BB170(ctx, base);
	// 823BF318: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF31C: 4182000C  beq 0x823bf328
	if ctx.cr[0].eq {
	pc = 0x823BF328; continue 'dispatch;
	}
	// 823BF320: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF324: 48A330B5  bl 0x82df23d8
	ctx.lr = 0x823BF328;
	sub_82DF23D8(ctx, base);
	// 823BF328: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF32C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF338: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF33C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF340: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF348 size=76
    let mut pc: u32 = 0x823BF348;
    'dispatch: loop {
        match pc {
            0x823BF348 => {
    //   block [0x823BF348..0x823BF394)
	// 823BF348: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF34C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF350: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF354: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF358: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF35C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF360: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF364: 4BFFBE5D  bl 0x823bb1c0
	ctx.lr = 0x823BF368;
	sub_823BB1C0(ctx, base);
	// 823BF368: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF36C: 4182000C  beq 0x823bf378
	if ctx.cr[0].eq {
	pc = 0x823BF378; continue 'dispatch;
	}
	// 823BF370: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF374: 48A33065  bl 0x82df23d8
	ctx.lr = 0x823BF378;
	sub_82DF23D8(ctx, base);
	// 823BF378: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF37C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF380: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF384: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF388: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF38C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF390: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF398 size=104
    let mut pc: u32 = 0x823BF398;
    'dispatch: loop {
        match pc {
            0x823BF398 => {
    //   block [0x823BF398..0x823BF400)
	// 823BF398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF39C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF3A0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF3A4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF3A8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF3AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF3B0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF3B4: 387F0088  addi r3, r31, 0x88
	ctx.r[3].s64 = ctx.r[31].s64 + 136;
	// 823BF3B8: 48A34071  bl 0x82df3428
	ctx.lr = 0x823BF3BC;
	sub_82DF3428(ctx, base);
	// 823BF3BC: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BF3C0: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BF3C4: 419A0008  beq cr6, 0x823bf3cc
	if ctx.cr[6].eq {
	pc = 0x823BF3CC; continue 'dispatch;
	}
	// 823BF3C8: 4BF014C9  bl 0x822c0890
	ctx.lr = 0x823BF3CC;
	sub_822C0890(ctx, base);
	// 823BF3CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF3D0: 48A9D1B9  bl 0x82e5c588
	ctx.lr = 0x823BF3D4;
	sub_82E5C588(ctx, base);
	// 823BF3D4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF3D8: 4182000C  beq 0x823bf3e4
	if ctx.cr[0].eq {
	pc = 0x823BF3E4; continue 'dispatch;
	}
	// 823BF3DC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF3E0: 48A32FF9  bl 0x82df23d8
	ctx.lr = 0x823BF3E4;
	sub_82DF23D8(ctx, base);
	// 823BF3E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF3E8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF3EC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF3F0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF3F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF3F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF3FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF400 size=104
    let mut pc: u32 = 0x823BF400;
    'dispatch: loop {
        match pc {
            0x823BF400 => {
    //   block [0x823BF400..0x823BF468)
	// 823BF400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF408: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF40C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF410: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF414: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF418: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF41C: 387F0070  addi r3, r31, 0x70
	ctx.r[3].s64 = ctx.r[31].s64 + 112;
	// 823BF420: 48A34009  bl 0x82df3428
	ctx.lr = 0x823BF424;
	sub_82DF3428(ctx, base);
	// 823BF424: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BF428: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BF42C: 419A0008  beq cr6, 0x823bf434
	if ctx.cr[6].eq {
	pc = 0x823BF434; continue 'dispatch;
	}
	// 823BF430: 4BF01461  bl 0x822c0890
	ctx.lr = 0x823BF434;
	sub_822C0890(ctx, base);
	// 823BF434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF438: 48A9D151  bl 0x82e5c588
	ctx.lr = 0x823BF43C;
	sub_82E5C588(ctx, base);
	// 823BF43C: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF440: 4182000C  beq 0x823bf44c
	if ctx.cr[0].eq {
	pc = 0x823BF44C; continue 'dispatch;
	}
	// 823BF444: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF448: 48A32F91  bl 0x82df23d8
	ctx.lr = 0x823BF44C;
	sub_82DF23D8(ctx, base);
	// 823BF44C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF450: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF45C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF460: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF464: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF468 size=104
    let mut pc: u32 = 0x823BF468;
    'dispatch: loop {
        match pc {
            0x823BF468 => {
    //   block [0x823BF468..0x823BF4D0)
	// 823BF468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF46C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF470: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF474: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF478: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF47C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF480: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF484: 387F0080  addi r3, r31, 0x80
	ctx.r[3].s64 = ctx.r[31].s64 + 128;
	// 823BF488: 48A33FA1  bl 0x82df3428
	ctx.lr = 0x823BF48C;
	sub_82DF3428(ctx, base);
	// 823BF48C: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BF490: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BF494: 419A0008  beq cr6, 0x823bf49c
	if ctx.cr[6].eq {
	pc = 0x823BF49C; continue 'dispatch;
	}
	// 823BF498: 4BF013F9  bl 0x822c0890
	ctx.lr = 0x823BF49C;
	sub_822C0890(ctx, base);
	// 823BF49C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF4A0: 48A9D0E9  bl 0x82e5c588
	ctx.lr = 0x823BF4A4;
	sub_82E5C588(ctx, base);
	// 823BF4A4: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF4A8: 4182000C  beq 0x823bf4b4
	if ctx.cr[0].eq {
	pc = 0x823BF4B4; continue 'dispatch;
	}
	// 823BF4AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF4B0: 48A32F29  bl 0x82df23d8
	ctx.lr = 0x823BF4B4;
	sub_82DF23D8(ctx, base);
	// 823BF4B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF4B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF4BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF4C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF4C4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF4C8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF4CC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF4D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF4D0 size=76
    let mut pc: u32 = 0x823BF4D0;
    'dispatch: loop {
        match pc {
            0x823BF4D0 => {
    //   block [0x823BF4D0..0x823BF51C)
	// 823BF4D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF4D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF4D8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF4DC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF4E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF4E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF4E8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF4EC: 4BFFBD25  bl 0x823bb210
	ctx.lr = 0x823BF4F0;
	sub_823BB210(ctx, base);
	// 823BF4F0: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF4F4: 4182000C  beq 0x823bf500
	if ctx.cr[0].eq {
	pc = 0x823BF500; continue 'dispatch;
	}
	// 823BF4F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF4FC: 48A32EDD  bl 0x82df23d8
	ctx.lr = 0x823BF500;
	sub_82DF23D8(ctx, base);
	// 823BF500: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF504: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF50C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF510: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF514: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF520 size=76
    let mut pc: u32 = 0x823BF520;
    'dispatch: loop {
        match pc {
            0x823BF520 => {
    //   block [0x823BF520..0x823BF56C)
	// 823BF520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF524: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF528: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF52C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF530: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF534: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF538: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF53C: 4BFFBD25  bl 0x823bb260
	ctx.lr = 0x823BF540;
	sub_823BB260(ctx, base);
	// 823BF540: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF544: 4182000C  beq 0x823bf550
	if ctx.cr[0].eq {
	pc = 0x823BF550; continue 'dispatch;
	}
	// 823BF548: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF54C: 48A32E8D  bl 0x82df23d8
	ctx.lr = 0x823BF550;
	sub_82DF23D8(ctx, base);
	// 823BF550: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF554: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF558: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF55C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF560: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF564: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF568: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF570 size=76
    let mut pc: u32 = 0x823BF570;
    'dispatch: loop {
        match pc {
            0x823BF570 => {
    //   block [0x823BF570..0x823BF5BC)
	// 823BF570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF574: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF578: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF57C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF580: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF584: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF588: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF58C: 4BFFBD25  bl 0x823bb2b0
	ctx.lr = 0x823BF590;
	sub_823BB2B0(ctx, base);
	// 823BF590: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF594: 4182000C  beq 0x823bf5a0
	if ctx.cr[0].eq {
	pc = 0x823BF5A0; continue 'dispatch;
	}
	// 823BF598: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF59C: 48A32E3D  bl 0x82df23d8
	ctx.lr = 0x823BF5A0;
	sub_82DF23D8(ctx, base);
	// 823BF5A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF5A4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF5A8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF5AC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF5B0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF5B4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF5B8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF5C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x823BF5C0 size=104
    let mut pc: u32 = 0x823BF5C0;
    'dispatch: loop {
        match pc {
            0x823BF5C0 => {
    //   block [0x823BF5C0..0x823BF628)
	// 823BF5C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF5C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF5C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF5CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF5D0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF5D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF5D8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 823BF5DC: 387F006C  addi r3, r31, 0x6c
	ctx.r[3].s64 = ctx.r[31].s64 + 108;
	// 823BF5E0: 48A33E49  bl 0x82df3428
	ctx.lr = 0x823BF5E4;
	sub_82DF3428(ctx, base);
	// 823BF5E4: 807F0064  lwz r3, 0x64(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BF5E8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BF5EC: 419A0008  beq cr6, 0x823bf5f4
	if ctx.cr[6].eq {
	pc = 0x823BF5F4; continue 'dispatch;
	}
	// 823BF5F0: 4BF012A1  bl 0x822c0890
	ctx.lr = 0x823BF5F4;
	sub_822C0890(ctx, base);
	// 823BF5F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF5F8: 48A9CF91  bl 0x82e5c588
	ctx.lr = 0x823BF5FC;
	sub_82E5C588(ctx, base);
	// 823BF5FC: 57CB07FF  clrlwi. r11, r30, 0x1f
	ctx.r[11].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	ctx.cr[0].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 823BF600: 4182000C  beq 0x823bf60c
	if ctx.cr[0].eq {
	pc = 0x823BF60C; continue 'dispatch;
	}
	// 823BF604: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF608: 48A32DD1  bl 0x82df23d8
	ctx.lr = 0x823BF60C;
	sub_82DF23D8(ctx, base);
	// 823BF60C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF610: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 823BF614: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF618: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF61C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF620: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF624: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823BF628 size=240
    let mut pc: u32 = 0x823BF628;
    'dispatch: loop {
        match pc {
            0x823BF628 => {
    //   block [0x823BF628..0x823BF718)
	// 823BF628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF62C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 823BF630: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 823BF634: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 823BF638: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF63C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF640: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823BF644: 389F03E8  addi r4, r31, 0x3e8
	ctx.r[4].s64 = ctx.r[31].s64 + 1000;
	// 823BF648: 4BFFF669  bl 0x823becb0
	ctx.lr = 0x823BF64C;
	sub_823BECB0(ctx, base);
	// 823BF64C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BF650: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 823BF654: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 823BF658: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BF65C: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 823BF660: 419A0024  beq cr6, 0x823bf684
	if ctx.cr[6].eq {
	pc = 0x823BF684; continue 'dispatch;
	}
	// 823BF664: 396B0004  addi r11, r11, 4
	ctx.r[11].s64 = ctx.r[11].s64 + 4;
	// 823BF668: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823BF66C: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823BF670: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823BF674: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 823BF678: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823BF67C: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823BF680: 4082FFE8  bne 0x823bf668
	if !ctx.cr[0].eq {
	pc = 0x823BF668; continue 'dispatch;
	}
	// 823BF684: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BF688: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823BF68C: 4814FE8D  bl 0x8250f518
	ctx.lr = 0x823BF690;
	sub_8250F518(ctx, base);
	// 823BF690: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BF694: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 823BF698: 386BFF6C  addi r3, r11, -0x94
	ctx.r[3].s64 = ctx.r[11].s64 + -148;
	// 823BF69C: 409A0008  bne cr6, 0x823bf6a4
	if !ctx.cr[6].eq {
	pc = 0x823BF6A4; continue 'dispatch;
	}
	// 823BF6A0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 823BF6A4: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 823BF6A8: 481689A9  bl 0x82528050
	ctx.lr = 0x823BF6AC;
	sub_82528050(ctx, base);
	// 823BF6AC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823BF6B0: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BF6B4: 7C661B78  mr r6, r3
	ctx.r[6].u64 = ctx.r[3].u64;
	// 823BF6B8: 388A03C8  addi r4, r10, 0x3c8
	ctx.r[4].s64 = ctx.r[10].s64 + 968;
	// 823BF6BC: 38A00212  li r5, 0x212
	ctx.r[5].s64 = 530;
	// 823BF6C0: 387F0028  addi r3, r31, 0x28
	ctx.r[3].s64 = ctx.r[31].s64 + 40;
	// 823BF6C4: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 823BF6C8: 7FC7F378  mr r7, r30
	ctx.r[7].u64 = ctx.r[30].u64;
	// 823BF6CC: 48A99375  bl 0x82e58a40
	ctx.lr = 0x823BF6D0;
	sub_82E58A40(ctx, base);
	// 823BF6D0: 38610058  addi r3, r1, 0x58
	ctx.r[3].s64 = ctx.r[1].s64 + 88;
	// 823BF6D4: 48A325BD  bl 0x82df1c90
	ctx.lr = 0x823BF6D8;
	sub_82DF1C90(ctx, base);
	// 823BF6D8: 80610054  lwz r3, 0x54(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823BF6DC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BF6E0: 419A0008  beq cr6, 0x823bf6e8
	if ctx.cr[6].eq {
	pc = 0x823BF6E8; continue 'dispatch;
	}
	// 823BF6E4: 4BF011AD  bl 0x822c0890
	ctx.lr = 0x823BF6E8;
	sub_822C0890(ctx, base);
	// 823BF6E8: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BF6EC: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BF6F0: 419A0008  beq cr6, 0x823bf6f8
	if ctx.cr[6].eq {
	pc = 0x823BF6F8; continue 'dispatch;
	}
	// 823BF6F4: 4BF0119D  bl 0x822c0890
	ctx.lr = 0x823BF6F8;
	sub_822C0890(ctx, base);
	// 823BF6F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF6FC: 4BF00905  bl 0x822c0000
	ctx.lr = 0x823BF700;
	sub_822C0000(ctx, base);
	// 823BF700: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 823BF704: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 823BF708: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 823BF70C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 823BF710: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 823BF714: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF718(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823BF718 size=500
    let mut pc: u32 = 0x823BF718;
    'dispatch: loop {
        match pc {
            0x823BF718 => {
    //   block [0x823BF718..0x823BF90C)
	// 823BF718: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF71C: 48DE8A51  bl 0x831a816c
	ctx.lr = 0x823BF720;
	sub_831A8130(ctx, base);
	// 823BF720: DBC1FFD0  stfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[30].u64 ) };
	// 823BF724: DBE1FFD8  stfd f31, -0x28(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[31].u64 ) };
	// 823BF728: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF72C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 823BF730: FFC00890  fmr f30, f1
	ctx.f[30].f64 = ctx.f[1].f64;
	// 823BF734: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 823BF738: 3BBE00CC  addi r29, r30, 0xcc
	ctx.r[29].s64 = ctx.r[30].s64 + 204;
	// 823BF73C: 388B3F88  addi r4, r11, 0x3f88
	ctx.r[4].s64 = ctx.r[11].s64 + 16264;
	// 823BF740: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BF744: 48A9CD2D  bl 0x82e5c470
	ctx.lr = 0x823BF748;
	sub_82E5C470(ctx, base);
	// 823BF748: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 823BF74C: 387E013C  addi r3, r30, 0x13c
	ctx.r[3].s64 = ctx.r[30].s64 + 316;
	// 823BF750: 388B4B18  addi r4, r11, 0x4b18
	ctx.r[4].s64 = ctx.r[11].s64 + 19224;
	// 823BF754: 48A9CD1D  bl 0x82e5c470
	ctx.lr = 0x823BF758;
	sub_82E5C470(ctx, base);
	// 823BF758: 3D608335  lis r11, -0x7ccb
	ctx.r[11].s64 = -2093678592;
	// 823BF75C: 387E01B0  addi r3, r30, 0x1b0
	ctx.r[3].s64 = ctx.r[30].s64 + 432;
	// 823BF760: 388B4B90  addi r4, r11, 0x4b90
	ctx.r[4].s64 = ctx.r[11].s64 + 19344;
	// 823BF764: 48A9CD0D  bl 0x82e5c470
	ctx.lr = 0x823BF768;
	sub_82E5C470(ctx, base);
	// 823BF768: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 823BF76C: 48A9CF2D  bl 0x82e5c698
	ctx.lr = 0x823BF770;
	sub_82E5C698(ctx, base);
	// 823BF770: 3D608202  lis r11, -0x7dfe
	ctx.r[11].s64 = -2113798144;
	// 823BF774: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BF778: 388B03C8  addi r4, r11, 0x3c8
	ctx.r[4].s64 = ctx.r[11].s64 + 968;
	// 823BF77C: 38A0034C  li r5, 0x34c
	ctx.r[5].s64 = 844;
	// 823BF780: 38600068  li r3, 0x68
	ctx.r[3].s64 = 104;
	// 823BF784: 48A32C65  bl 0x82df23e8
	ctx.lr = 0x823BF788;
	sub_82DF23E8(ctx, base);
	// 823BF788: 7C7F1B79  or. r31, r3, r3
	ctx.r[31].u64 = ctx.r[3].u64 | ctx.r[3].u64;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 823BF78C: 41820028  beq 0x823bf7b4
	if ctx.cr[0].eq {
	pc = 0x823BF7B4; continue 'dispatch;
	}
	// 823BF790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF794: 48A9D195  bl 0x82e5c928
	ctx.lr = 0x823BF798;
	sub_82E5C928(ctx, base);
	// 823BF798: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 823BF79C: 3D408202  lis r10, -0x7dfe
	ctx.r[10].s64 = -2113798144;
	// 823BF7A0: 917F0060  stw r11, 0x60(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 823BF7A4: 917F0064  stw r11, 0x64(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 823BF7A8: 396A0420  addi r11, r10, 0x420
	ctx.r[11].s64 = ctx.r[10].s64 + 1056;
	// 823BF7AC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 823BF7B0: 48000008  b 0x823bf7b8
	pc = 0x823BF7B8; continue 'dispatch;
	// 823BF7B4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 823BF7B8: 93E10050  stw r31, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[31].u32 ) };
	// 823BF7BC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BF7C0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823BF7C4: 4BFF69F5  bl 0x823b61b8
	ctx.lr = 0x823BF7C8;
	sub_823B61B8(ctx, base);
	// 823BF7C8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 823BF7CC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BF7D0: 38610054  addi r3, r1, 0x54
	ctx.r[3].s64 = ctx.r[1].s64 + 84;
	// 823BF7D4: 4BF0082D  bl 0x822c0000
	ctx.lr = 0x823BF7D8;
	sub_822C0000(ctx, base);
	// 823BF7D8: 83E10054  lwz r31, 0x54(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 823BF7DC: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823BF7E0: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823BF7E4: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 823BF7E8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 823BF7EC: 419A0024  beq cr6, 0x823bf810
	if ctx.cr[6].eq {
	pc = 0x823BF810; continue 'dispatch;
	}
	// 823BF7F0: 397F0004  addi r11, r31, 4
	ctx.r[11].s64 = ctx.r[31].s64 + 4;
	// 823BF7F4: 7D2000A6  mfmsr r9
	ctx.r[9].u64 = ctx.msr;
	// 823BF7F8: 7DA10164  mtmsrd r13, 1
	ctx.msr = (ctx.r[13].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823BF7FC: 7D405828  lwarx r10, 0, r11
	// lwarx
	let ea = ctx.r[11].u32;
	ctx.reserved.u32 = unsafe { crate::rt::load_u32(base as *const u8, ea) };
	ctx.r[10].u64 = ctx.reserved.u32 as u64;
	// 823BF800: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 823BF804: 7D40592D  stwcx. r10, 0, r11
	// stwcx.
	let addr = ctx.r[11].u32;
	ctx.cr[0].lt = false;
	ctx.cr[0].gt = false;
	let ok = unsafe { crate::rt::stwcx32(base as *mut u8, addr, ctx.reserved.u32, ctx.r[10].u32) };
	ctx.cr[0].eq = ok;
	ctx.cr[0].so = ctx.xer.so;
	// 823BF808: 7D210164  mtmsrd r9, 1
	ctx.msr = (ctx.r[9].u32 & 0x8020) | (ctx.msr & !0x8020);
	// 823BF80C: 4082FFE8  bne 0x823bf7f4
	if !ctx.cr[0].eq {
	pc = 0x823BF7F4; continue 'dispatch;
	}
	// 823BF810: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823BF814: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823BF818: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BF81C: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 823BF820: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823BF824: C3EB08A4  lfs f31, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 823BF828: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 823BF82C: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 823BF830: 48A9EE79  bl 0x82e5e6a8
	ctx.lr = 0x823BF834;
	sub_82E5E6A8(ctx, base);
	// 823BF834: 80610064  lwz r3, 0x64(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 823BF838: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BF83C: 419A0008  beq cr6, 0x823bf844
	if ctx.cr[6].eq {
	pc = 0x823BF844; continue 'dispatch;
	}
	// 823BF840: 4BF01051  bl 0x822c0890
	ctx.lr = 0x823BF844;
	sub_822C0890(ctx, base);
	// 823BF844: 8061005C  lwz r3, 0x5c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) } as u64;
	// 823BF848: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BF84C: 419A0008  beq cr6, 0x823bf854
	if ctx.cr[6].eq {
	pc = 0x823BF854; continue 'dispatch;
	}
	// 823BF850: 4BF01041  bl 0x822c0890
	ctx.lr = 0x823BF854;
	sub_822C0890(ctx, base);
	// 823BF854: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 823BF858: 419A000C  beq cr6, 0x823bf864
	if ctx.cr[6].eq {
	pc = 0x823BF864; continue 'dispatch;
	}
	// 823BF85C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF860: 4BF01031  bl 0x822c0890
	ctx.lr = 0x823BF864;
	sub_822C0890(ctx, base);
	// 823BF864: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823BF868: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BF86C: 808B7D78  lwz r4, 0x7d78(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32120 as u32) ) } as u64;
	// 823BF870: 48A34199  bl 0x82df3a08
	ctx.lr = 0x823BF874;
	sub_82DF3A08(ctx, base);
	// 823BF874: 7C651B78  mr r5, r3
	ctx.r[5].u64 = ctx.r[3].u64;
	// 823BF878: FC20F890  fmr f1, f31
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[31].f64;
	// 823BF87C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823BF880: 38610068  addi r3, r1, 0x68
	ctx.r[3].s64 = ctx.r[1].s64 + 104;
	// 823BF884: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 823BF888: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 823BF88C: 48A9F16D  bl 0x82e5e9f8
	ctx.lr = 0x823BF890;
	sub_82E5E9F8(ctx, base);
	// 823BF890: 8061006C  lwz r3, 0x6c(r1)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) } as u64;
	// 823BF894: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 823BF898: 419A0008  beq cr6, 0x823bf8a0
	if ctx.cr[6].eq {
	pc = 0x823BF8A0; continue 'dispatch;
	}
	// 823BF89C: 4BF00FF5  bl 0x822c0890
	ctx.lr = 0x823BF8A0;
	sub_822C0890(ctx, base);
	// 823BF8A0: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823BF8A4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BF8A8: 808B7E1C  lwz r4, 0x7e1c(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32284 as u32) ) } as u64;
	// 823BF8AC: 48A3415D  bl 0x82df3a08
	ctx.lr = 0x823BF8B0;
	sub_82DF3A08(ctx, base);
	// 823BF8B0: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 823BF8B4: FC20F090  fmr f1, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[1].f64 = ctx.f[30].f64;
	// 823BF8B8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823BF8BC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BF8C0: 4BF44DC1  bl 0x82304680
	ctx.lr = 0x823BF8C4;
	sub_82304680(ctx, base);
	// 823BF8C4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BF8C8: 48A33B61  bl 0x82df3428
	ctx.lr = 0x823BF8CC;
	sub_82DF3428(ctx, base);
	// 823BF8CC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BF8D0: 4BF44399  bl 0x82303c68
	ctx.lr = 0x823BF8D4;
	sub_82303C68(ctx, base);
	// 823BF8D4: 3D608326  lis r11, -0x7cda
	ctx.r[11].s64 = -2094661632;
	// 823BF8D8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BF8DC: 808B7E14  lwz r4, 0x7e14(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32276 as u32) ) } as u64;
	// 823BF8E0: 48A34129  bl 0x82df3a08
	ctx.lr = 0x823BF8E4;
	sub_82DF3A08(ctx, base);
	// 823BF8E4: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 823BF8E8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 823BF8EC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 823BF8F0: 4BF44B81  bl 0x82304470
	ctx.lr = 0x823BF8F4;
	sub_82304470(ctx, base);
	// 823BF8F4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BF8F8: 48A33B31  bl 0x82df3428
	ctx.lr = 0x823BF8FC;
	sub_82DF3428(ctx, base);
	// 823BF8FC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 823BF900: CBC1FFD0  lfd f30, -0x30(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 823BF904: CBE1FFD8  lfd f31, -0x28(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 823BF908: 48DE88B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_823BF910(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x823BF910 size=296
    let mut pc: u32 = 0x823BF910;
    'dispatch: loop {
        match pc {
            0x823BF910 => {
    //   block [0x823BF910..0x823BFA38)
	// 823BF910: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 823BF914: 48DE8859  bl 0x831a816c
	ctx.lr = 0x823BF918;
	sub_831A8130(ctx, base);
	// 823BF918: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 823BF91C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 823BF920: 481C7591  bl 0x82586eb0
	ctx.lr = 0x823BF924;
	sub_82586EB0(ctx, base);
	// 823BF924: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 823BF928: 83DF03C0  lwz r30, 0x3c0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(960 as u32) ) } as u64;
	// 823BF92C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BF930: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 823BF934: 48000028  b 0x823bf95c
	pc = 0x823BF95C; continue 'dispatch;
	// 823BF938: 806B0010  lwz r3, 0x10(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 823BF93C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 823BF940: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 823BF944: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 823BF948: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 823BF94C: 4E800421  bctrl
	ctx.lr = 0x823BF950;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 823BF950: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 823BF954: 4BFE1D35  bl 0x823a1688
	ctx.lr = 0x823BF958;
	sub_823A1688(ctx, base);
	// 823BF958: 81610050  lwz r11, 0x50(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 823BF95C: 7F0BF040  cmplw cr6, r11, r30
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[30].u32, &mut ctx.xer);
	// 823BF960: 409AFFD8  bne cr6, 0x823bf938
	if !ctx.cr[6].eq {
	pc = 0x823BF938; continue 'dispatch;
	}
	// 823BF964: 817F07C4  lwz r11, 0x7c4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(1988 as u32) ) } as u64;
	// 823BF968: 2F0B0010  cmpwi cr6, r11, 0x10
	ctx.cr[6].compare_i32(ctx.r[11].s32, 16, &mut ctx.xer);
	// 823BF96C: 419A0064  beq cr6, 0x823bf9d0
	if ctx.cr[6].eq {
	pc = 0x823BF9D0; continue 'dispatch;
	}
	// 823BF970: 2F0B0011  cmpwi cr6, r11, 0x11
	ctx.cr[6].compare_i32(ctx.r[11].s32, 17, &mut ctx.xer);
	// 823BF974: 419A005C  beq cr6, 0x823bf9d0
	if ctx.cr[6].eq {
	pc = 0x823BF9D0; continue 'dispatch;
	}
	// 823BF978: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF97C: 4BF461DD  bl 0x82305b58
	ctx.lr = 0x823BF980;
	sub_82305B58(ctx, base);
	// 823BF980: 48723F11  bl 0x82ae3890
	ctx.lr = 0x823BF984;
	sub_82AE3890(ctx, base);
	// 823BF984: 817F02E0  lwz r11, 0x2e0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(736 as u32) ) } as u64;
	// 823BF988: 3BDF02E0  addi r30, r31, 0x2e0
	ctx.r[30].s64 = ctx.r[31].s64 + 736;
	// 823BF98C: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823BF990: 419A00A0  beq cr6, 0x823bfa30
	if ctx.cr[6].eq {
	pc = 0x823BFA30; continue 'dispatch;
	}
	// 823BF994: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF998: 4BF461C1  bl 0x82305b58
	ctx.lr = 0x823BF99C;
	sub_82305B58(ctx, base);
	// 823BF99C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823BF9A0: 48725301  bl 0x82ae4ca0
	ctx.lr = 0x823BF9A4;
	sub_82AE4CA0(ctx, base);
	// 823BF9A4: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823BF9A8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 823BF9AC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF9B0: 3BE10060  addi r31, r1, 0x60
	ctx.r[31].s64 = ctx.r[1].s64 + 96;
	// 823BF9B4: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823BF9B8: C1AAC664  lfs f13, -0x399c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-14748 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823BF9BC: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 823BF9C0: D1A10064  stfs f13, 0x64(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 823BF9C4: D0010068  stfs f0, 0x68(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), tmp.u32 ) };
	// 823BF9C8: D001006C  stfs f0, 0x6c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), tmp.u32 ) };
	// 823BF9CC: 48000058  b 0x823bfa24
	pc = 0x823BFA24; continue 'dispatch;
	// 823BF9D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF9D4: 4BF46185  bl 0x82305b58
	ctx.lr = 0x823BF9D8;
	sub_82305B58(ctx, base);
	// 823BF9D8: 48723EB9  bl 0x82ae3890
	ctx.lr = 0x823BF9DC;
	sub_82AE3890(ctx, base);
	// 823BF9DC: 817F02E4  lwz r11, 0x2e4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(740 as u32) ) } as u64;
	// 823BF9E0: 3BDF02E4  addi r30, r31, 0x2e4
	ctx.r[30].s64 = ctx.r[31].s64 + 740;
	// 823BF9E4: 7F035840  cmplw cr6, r3, r11
	ctx.cr[6].compare_u32(ctx.r[3].u32, ctx.r[11].u32, &mut ctx.xer);
	// 823BF9E8: 419A0048  beq cr6, 0x823bfa30
	if ctx.cr[6].eq {
	pc = 0x823BFA30; continue 'dispatch;
	}
	// 823BF9EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BF9F0: 4BF46169  bl 0x82305b58
	ctx.lr = 0x823BF9F4;
	sub_82305B58(ctx, base);
	// 823BF9F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 823BF9F8: 487252A9  bl 0x82ae4ca0
	ctx.lr = 0x823BF9FC;
	sub_82AE4CA0(ctx, base);
	// 823BF9FC: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 823BFA00: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 823BFA04: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 823BFA08: 3BE10070  addi r31, r1, 0x70
	ctx.r[31].s64 = ctx.r[1].s64 + 112;
	// 823BFA0C: C00B08A4  lfs f0, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 823BFA10: C1AAFD2C  lfs f13, -0x2d4(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-724 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 823BFA14: D0010070  stfs f0, 0x70(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), tmp.u32 ) };
	// 823BFA18: D1A10074  stfs f13, 0x74(r1)
	tmp.f32 = (ctx.f[13].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), tmp.u32 ) };
	// 823BFA1C: D0010078  stfs f0, 0x78(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), tmp.u32 ) };
	// 823BFA20: D001007C  stfs f0, 0x7c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), tmp.u32 ) };
	// 823BFA24: 4BF46135  bl 0x82305b58
	ctx.lr = 0x823BFA28;
	sub_82305B58(ctx, base);
	// 823BFA28: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 823BFA2C: 48723ACD  bl 0x82ae34f8
	ctx.lr = 0x823BFA30;
	sub_82AE34F8(ctx, base);
	// 823BFA30: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 823BFA34: 48DE8788  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


