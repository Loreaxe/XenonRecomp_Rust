pub fn sub_8251B638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251B638 size=176
    let mut pc: u32 = 0x8251B638;
    'dispatch: loop {
        match pc {
            0x8251B638 => {
    //   block [0x8251B638..0x8251B6B8)
	// 8251B638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B640: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251B644: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251B648: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8251B64C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B650: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8251B654: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8251B658: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251B65C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8251B660: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B664: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8251B668: 4BFFFC61  bl 0x8251b2c8
	ctx.lr = 0x8251B66C;
	sub_8251B2C8(ctx, base);
	// 8251B66C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B670: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251B674: 40980044  bge cr6, 0x8251b6b8
	if !ctx.cr[6].lt {
	pc = 0x8251B6B8; continue 'dispatch;
	}
	// 8251B678: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251B67C: 394B08F0  addi r10, r11, 0x8f0
	ctx.r[10].s64 = ctx.r[11].s64 + 2288;
	// 8251B680: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x8251B6B8; continue 'dispatch;
            }
            0x8251B6B8 => {
    //   block [0x8251B6B8..0x8251B6E8)
	// 8251B6B8: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251B6BC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8251B6C0: 419A000C  beq cr6, 0x8251b6cc
	if ctx.cr[6].eq {
	pc = 0x8251B6CC; continue 'dispatch;
	}
	// 8251B6C4: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 8251B6C8: 4BFF7D79  bl 0x82513440
	ctx.lr = 0x8251B6CC;
	sub_82513440(ctx, base);
	// 8251B6CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251B6D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B6D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B6D8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8251B6DC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251B6E0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251B6E4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B6E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B6E8 size=152
    let mut pc: u32 = 0x8251B6E8;
    'dispatch: loop {
        match pc {
            0x8251B6E8 => {
    //   block [0x8251B6E8..0x8251B730)
	// 8251B6E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B6EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B6F0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251B6F4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251B6F8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B6FC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251B700: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251B704: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251B708: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 8251B70C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8251B710: 409A0020  bne cr6, 0x8251b730
	if !ctx.cr[6].eq {
	pc = 0x8251B730; continue 'dispatch;
	}
	// 8251B714: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B718: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 8251B71C: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 8251B720: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251B724: 5565087C  rlwinm r5, r11, 1, 1, 0x1e
	ctx.r[5].u64 = ctx.r[11].u32 as u64 & 0x7FFFFFFFu64;
	// 8251B728: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 8251B72C: 4BF4898D  bl 0x824640b8
	ctx.lr = 0x8251B730;
	sub_824640B8(ctx, base);
	pc = 0x8251B730; continue 'dispatch;
            }
            0x8251B730 => {
    //   block [0x8251B730..0x8251B764)
	// 8251B730: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 8251B734: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 8251B738: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8251B73C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8251B740: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251B744: 419A0020  beq cr6, 0x8251b764
	if ctx.cr[6].eq {
	pc = 0x8251B764; continue 'dispatch;
	}
	// 8251B748: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B74C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251B750: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 8251B754: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251B758: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8251B75C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251B760: 4BF48959  bl 0x824640b8
	ctx.lr = 0x8251B764;
	sub_824640B8(ctx, base);
	pc = 0x8251B764; continue 'dispatch;
            }
            0x8251B764 => {
    //   block [0x8251B764..0x8251B780)
	// 8251B764: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B768: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251B76C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251B770: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251B774: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251B778: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251B77C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B780 size=240
    let mut pc: u32 = 0x8251B780;
    'dispatch: loop {
        match pc {
            0x8251B780 => {
    //   block [0x8251B780..0x8251B7B0)
	// 8251B780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B784: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251B788: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251B78C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251B790: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B794: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251B798: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251B79C: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 8251B7A0: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 8251B7A4: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 8251B7A8: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 8251B7AC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x8251B7B0; continue 'dispatch;
            }
            0x8251B7B0 => {
    //   block [0x8251B7B0..0x8251B870)
	// 8251B7B0: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251B7B4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8251B7B8: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8251B7BC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8251B7C0: 4200FFF0  bdnz 0x8251b7b0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8251B7B0; continue 'dispatch;
	}
	// 8251B7C4: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251B7C8: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 8251B7CC: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 8251B7D0: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8251B7D4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8251B7D8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B870 size=136
    let mut pc: u32 = 0x8251B870;
    'dispatch: loop {
        match pc {
            0x8251B870 => {
    //   block [0x8251B870..0x8251B8D0)
	// 8251B870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B874: 48019849  bl 0x825350bc
	ctx.lr = 0x8251B878;
	sub_82535080(ctx, base);
	// 8251B878: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B87C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B880: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251B884: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8251B888: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8251B88C: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 8251B890: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8251B894: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251B898: 4BF487A1  bl 0x82464038
	ctx.lr = 0x8251B89C;
	sub_82464038(ctx, base);
	// 8251B89C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251B8A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251B8A4: 394B51C4  addi r10, r11, 0x51c4
	ctx.r[10].s64 = ctx.r[11].s64 + 20932;
	// 8251B8A8: 38E0002C  li r7, 0x2c
	ctx.r[7].s64 = 44;
	// 8251B8AC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8251B8B0: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8251B8B4: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8251B8B8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8251B8BC: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 8251B8C0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8251B8C4: B0FF0004  sth r7, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 8251B8C8: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 8251B8CC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x8251B8D0; continue 'dispatch;
            }
            0x8251B8D0 => {
    //   block [0x8251B8D0..0x8251B8F8)
	// 8251B8D0: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8251B8D4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8251B8D8: 4200FFF8  bdnz 0x8251b8d0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8251B8D0; continue 'dispatch;
	}
	// 8251B8DC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B8E0: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 8251B8E4: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 8251B8E8: 480AD2D1  bl 0x825c8bb8
	ctx.lr = 0x8251B8EC;
	sub_825C8BB8(ctx, base);
	// 8251B8EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B8F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251B8F4: 48019818  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B8F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B8F8 size=120
    let mut pc: u32 = 0x8251B8F8;
    'dispatch: loop {
        match pc {
            0x8251B8F8 => {
    //   block [0x8251B8F8..0x8251B914)
	// 8251B8F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B8FC: 480197BD  bl 0x825350b8
	ctx.lr = 0x8251B900;
	sub_82535080(ctx, base);
	// 8251B900: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B904: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251B908: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8251B90C: 3BDF001C  addi r30, r31, 0x1c
	ctx.r[30].s64 = ctx.r[31].s64 + 28;
	// 8251B910: 3BA00008  li r29, 8
	ctx.r[29].s64 = 8;
	pc = 0x8251B914; continue 'dispatch;
            }
            0x8251B914 => {
    //   block [0x8251B914..0x8251B938)
	// 8251B914: A09E0000  lhz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B918: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 8251B91C: 419A001C  beq cr6, 0x8251b938
	if ctx.cr[6].eq {
	pc = 0x8251B938; continue 'dispatch;
	}
	// 8251B920: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251B924: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251B928: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B92C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251B930: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251B934: 4E800421  bctrl
	ctx.lr = 0x8251B938;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8251B938 => {
    //   block [0x8251B938..0x8251B968)
	// 8251B938: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 8251B93C: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 8251B940: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8251B944: 409AFFD0  bne cr6, 0x8251b914
	if !ctx.cr[6].eq {
	pc = 0x8251B914; continue 'dispatch;
	}
	// 8251B948: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251B94C: 419A001C  beq cr6, 0x8251b968
	if ctx.cr[6].eq {
	pc = 0x8251B968; continue 'dispatch;
	}
	// 8251B950: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B954: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8251B958: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B95C: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B960: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251B964: 4E800421  bctrl
	ctx.lr = 0x8251B968;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8251B968 => {
    //   block [0x8251B968..0x8251B970)
	// 8251B968: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251B96C: 4801979C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251B970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251B970 size=148
    let mut pc: u32 = 0x8251B970;
    'dispatch: loop {
        match pc {
            0x8251B970 => {
    //   block [0x8251B970..0x8251B9D0)
	// 8251B970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251B974: 48019749  bl 0x825350bc
	ctx.lr = 0x8251B978;
	sub_82535080(ctx, base);
	// 8251B978: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251B97C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B980: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251B984: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8251B988: 3880002C  li r4, 0x2c
	ctx.r[4].s64 = 44;
	// 8251B98C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8251B990: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 8251B994: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251B998: 4BF486A1  bl 0x82464038
	ctx.lr = 0x8251B99C;
	sub_82464038(ctx, base);
	// 8251B99C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251B9A0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251B9A4: 394B51C4  addi r10, r11, 0x51c4
	ctx.r[10].s64 = ctx.r[11].s64 + 20932;
	// 8251B9A8: 38E0002C  li r7, 0x2c
	ctx.r[7].s64 = 44;
	// 8251B9AC: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8251B9B0: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8251B9B4: 39200008  li r9, 8
	ctx.r[9].s64 = 8;
	// 8251B9B8: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 8251B9BC: 397F001C  addi r11, r31, 0x1c
	ctx.r[11].s64 = ctx.r[31].s64 + 28;
	// 8251B9C0: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8251B9C4: B0FF0004  sth r7, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u16 ) };
	// 8251B9C8: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 8251B9CC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x8251B9D0; continue 'dispatch;
            }
            0x8251B9D0 => {
    //   block [0x8251B9D0..0x8251BA04)
	// 8251B9D0: B10B0000  sth r8, 0(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u16 ) };
	// 8251B9D4: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8251B9D8: 4200FFF8  bdnz 0x8251b9d0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8251B9D0; continue 'dispatch;
	}
	// 8251B9DC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251B9E0: 389F000C  addi r4, r31, 0xc
	ctx.r[4].s64 = ctx.r[31].s64 + 12;
	// 8251B9E4: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 8251B9E8: 480AD1D1  bl 0x825c8bb8
	ctx.lr = 0x8251B9EC;
	sub_825C8BB8(ctx, base);
	// 8251B9EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251B9F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251B9F4: 396B5200  addi r11, r11, 0x5200
	ctx.r[11].s64 = ctx.r[11].s64 + 20992;
	// 8251B9F8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251B9FC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251BA00: 4801970C  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BA08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251BA08 size=584
    let mut pc: u32 = 0x8251BA08;
    'dispatch: loop {
        match pc {
            0x8251BA08 => {
    //   block [0x8251BA08..0x8251BC50)
	// 8251BA08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251BA0C: 48019685  bl 0x82535090
	ctx.lr = 0x8251BA10;
	sub_82535080(ctx, base);
	// 8251BA10: DBE1FF80  stfd f31, -0x80(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 8251BA14: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251BA18: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8251BA1C: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 8251BA20: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8251BA24: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8251BA28: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 8251BA2C: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251BA30: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 8251BA34: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 8251BA38: 82F90000  lwz r23, 0(r25)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BA3C: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 8251BA40: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 8251BA44: 83DB0000  lwz r30, 0(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BA48: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 8251BA4C: EB4B0000  ld r26, 0(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251BA50: 3BF70020  addi r31, r23, 0x20
	ctx.r[31].s64 = ctx.r[23].s64 + 32;
	// 8251BA54: EA6B0008  ld r19, 8(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251BA58: 3B8100A0  addi r28, r1, 0xa0
	ctx.r[28].s64 = ctx.r[1].s64 + 160;
	// 8251BA5C: EB060000  ld r24, 0(r6)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8251BA60: 3BA00002  li r29, 2
	ctx.r[29].s64 = 2;
	// 8251BA64: EA460008  ld r18, 8(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 8251BA68: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 8251BA6C: EAA50000  ld r21, 0(r5)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 8251BA70: 7CDFE050  subf r6, r31, r28
	ctx.r[6].s64 = ctx.r[28].s64 - ctx.r[31].s64;
	// 8251BA74: FB4A0000  std r26, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[26].u64 ) };
	// 8251BA78: FA6A0008  std r19, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[19].u64 ) };
	// 8251BA7C: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 8251BA80: FB090000  std r24, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
	// 8251BA84: FA490008  std r18, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[18].u64 ) };
	// 8251BA88: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 8251BA8C: FAA80000  std r21, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BC50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251BC50 size=588
    let mut pc: u32 = 0x8251BC50;
    'dispatch: loop {
        match pc {
            0x8251BC50 => {
    //   block [0x8251BC50..0x8251BE9C)
	// 8251BC50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251BC54: 48019441  bl 0x82535094
	ctx.lr = 0x8251BC58;
	sub_82535080(ctx, base);
	// 8251BC58: DBE1FF88  stfd f31, -0x78(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-120 as u32), ctx.f[31].u64 ) };
	// 8251BC5C: 9421FE30  stwu r1, -0x1d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-464 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251BC60: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8251BC64: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 8251BC68: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 8251BC6C: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8251BC70: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 8251BC74: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251BC78: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251BC7C: 83190000  lwz r24, 0(r25)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BC80: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8251BC84: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 8251BC88: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 8251BC8C: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 8251BC90: EB8B0000  ld r28, 0(r11)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251BC94: 38780020  addi r3, r24, 0x20
	ctx.r[3].s64 = ctx.r[24].s64 + 32;
	// 8251BC98: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251BC9C: 3BA100A0  addi r29, r1, 0xa0
	ctx.r[29].s64 = ctx.r[1].s64 + 160;
	// 8251BCA0: EB460000  ld r26, 0(r6)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8251BCA4: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8251BCA8: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 8251BCAC: EAC50000  ld r22, 0(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 8251BCB0: FB8A0000  std r28, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[28].u64 ) };
	// 8251BCB4: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251BCB8: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 8251BCBC: FB490000  std r26, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[26].u64 ) };
	// 8251BCC0: 7D43E850  subf r10, r3, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[3].s64;
	// 8251BCC4: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 8251BCC8: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 8251BCCC: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 8251BCD0: FAC80000  std r22, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 8251BCD4: EAA40000  ld r21, 0(r4)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 8251BCD8: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251BEA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251BEA0 size=744
    let mut pc: u32 = 0x8251BEA0;
    'dispatch: loop {
        match pc {
            0x8251BEA0 => {
    //   block [0x8251BEA0..0x8251BEF0)
	// 8251BEA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251BEA4: 480191E1  bl 0x82535084
	ctx.lr = 0x8251BEA8;
	sub_82535080(ctx, base);
	// 8251BEA8: DBE1FF68  stfd f31, -0x98(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), ctx.f[31].u64 ) };
	// 8251BEAC: 9421FDC0  stwu r1, -0x240(r1)
	ea = ctx.r[1].u32.wrapping_add(-576 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251BEB0: 82AD0000  lwz r21, 0(r13)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BEB4: 3AC00014  li r22, 0x14
	ctx.r[22].s64 = 20;
	// 8251BEB8: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 8251BEBC: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 8251BEC0: 7D76A82E  lwzx r11, r22, r21
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 8251BEC4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251BEC8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251BECC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251BED0: 40980020  bge cr6, 0x8251bef0
	if !ctx.cr[6].lt {
	pc = 0x8251BEF0; continue 'dispatch;
	}
	// 8251BED4: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251BED8: 3929524C  addi r9, r9, 0x524c
	ctx.r[9].s64 = ctx.r[9].s64 + 21068;
	// 8251BEDC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251BEE0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251BEE4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251BEE8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251BEEC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251BEF0; continue 'dispatch;
            }
            0x8251BEF0 => {
    //   block [0x8251BEF0..0x8251C188)
	// 8251BEF0: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251BEF4: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 8251BEF8: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 8251BEFC: 83250000  lwz r25, 0(r5)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BF00: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 8251BF04: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251BF08: 3BEB0020  addi r31, r11, 0x20
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	// 8251BF0C: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 8251BF10: EA8B0000  ld r20, 0(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251BF14: 3BCB0030  addi r30, r11, 0x30
	ctx.r[30].s64 = ctx.r[11].s64 + 48;
	// 8251BF18: EA0B0008  ld r16, 8(r11)
	ctx.r[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251BF1C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 8251BF20: EA660000  ld r19, 0(r6)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8251BF24: 3BB90020  addi r29, r25, 0x20
	ctx.r[29].s64 = ctx.r[25].s64 + 32;
	// 8251BF28: E9E60008  ld r15, 8(r6)
	ctx.r[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 8251BF2C: 3B4100F0  addi r26, r1, 0xf0
	ctx.r[26].s64 = ctx.r[1].s64 + 240;
	// 8251BF30: EA5F0000  ld r18, 0(r31)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 8251BF34: 3B600002  li r27, 2
	ctx.r[27].s64 = 2;
	// 8251BF38: FA8A0000  std r20, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 8251BF3C: 397D0020  addi r11, r29, 0x20
	ctx.r[11].s64 = ctx.r[29].s64 + 32;
	// 8251BF40: FA0A0008  std r16, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[16].u64 ) };
	// 8251BF44: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 8251BF48: FA690000  std r19, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 8251BF4C: 7CDDD050  subf r6, r29, r26
	ctx.r[6].s64 = ctx.r[26].s64 - ctx.r[29].s64;
	// 8251BF50: F9E90008  std r15, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[15].u64 ) };
	// 8251BF54: EBFF0008  ld r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 8251BF58: FA480000  std r18, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251C188 size=756
    let mut pc: u32 = 0x8251C188;
    'dispatch: loop {
        match pc {
            0x8251C188 => {
    //   block [0x8251C188..0x8251C1E0)
	// 8251C188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251C18C: 48018EFD  bl 0x82535088
	ctx.lr = 0x8251C190;
	sub_82535080(ctx, base);
	// 8251C190: DBE1FF70  stfd f31, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[31].u64 ) };
	// 8251C194: 9421FDC0  stwu r1, -0x240(r1)
	ea = ctx.r[1].u32.wrapping_add(-576 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251C198: 82AD0000  lwz r21, 0(r13)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C19C: 3AC00014  li r22, 0x14
	ctx.r[22].s64 = 20;
	// 8251C1A0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8251C1A4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8251C1A8: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 8251C1AC: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 8251C1B0: 7D76A82E  lwzx r11, r22, r21
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[21].u32)) } as u64;
	// 8251C1B4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C1B8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251C1BC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251C1C0: 40980020  bge cr6, 0x8251c1e0
	if !ctx.cr[6].lt {
	pc = 0x8251C1E0; continue 'dispatch;
	}
	// 8251C1C4: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251C1C8: 3929524C  addi r9, r9, 0x524c
	ctx.r[9].s64 = ctx.r[9].s64 + 21068;
	// 8251C1CC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251C1D0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251C1D4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251C1D8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251C1DC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251C1E0; continue 'dispatch;
            }
            0x8251C1E0 => {
    //   block [0x8251C1E0..0x8251C47C)
	// 8251C1E0: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C1E4: 39410090  addi r10, r1, 0x90
	ctx.r[10].s64 = ctx.r[1].s64 + 144;
	// 8251C1E8: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 8251C1EC: 833A0000  lwz r25, 0(r26)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C1F0: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 8251C1F4: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C1F8: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 8251C1FC: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 8251C200: EB6B0000  ld r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251C204: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 8251C208: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251C20C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 8251C210: EA860000  ld r20, 0(r6)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8251C214: 38790020  addi r3, r25, 0x20
	ctx.r[3].s64 = ctx.r[25].s64 + 32;
	// 8251C218: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 8251C21C: 3BA10100  addi r29, r1, 0x100
	ctx.r[29].s64 = ctx.r[1].s64 + 256;
	// 8251C220: EA650000  ld r19, 0(r5)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 8251C224: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8251C228: FB6A0000  std r27, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 8251C22C: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251C230: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 8251C234: FA890000  std r20, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 8251C238: 7D43E850  subf r10, r3, r29
	ctx.r[10].s64 = ctx.r[29].s64 - ctx.r[3].s64;
	// 8251C23C: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 8251C240: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 8251C244: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 8251C248: FA680000  std r19, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 8251C24C: EA440000  ld r18, 0(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 8251C250: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C480(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251C480 size=204
    let mut pc: u32 = 0x8251C480;
    'dispatch: loop {
        match pc {
            0x8251C480 => {
    //   block [0x8251C480..0x8251C54C)
	// 8251C480: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251C484: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251C488: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251C48C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251C490: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251C494: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 8251C498: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251C49C: 396BC550  addi r11, r11, -0x3ab0
	ctx.r[11].s64 = ctx.r[11].s64 + -15024;
	// 8251C4A0: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251C4A4: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251C4A8: 3908B970  addi r8, r8, -0x4690
	ctx.r[8].s64 = ctx.r[8].s64 + -18064;
	// 8251C4AC: 3929C688  addi r9, r9, -0x3978
	ctx.r[9].s64 = ctx.r[9].s64 + -14712;
	// 8251C4B0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8251C4B4: 394ACAB0  addi r10, r10, -0x3550
	ctx.r[10].s64 = ctx.r[10].s64 + -13648;
	// 8251C4B8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8251C4BC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8251C4C0: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 8251C4C4: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 8251C4C8: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8251C4CC: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8251C4D0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251C4D4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251C4D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251C4DC: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 8251C4E0: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 8251C4E4: 4BFE38A5  bl 0x824ffd88
	ctx.lr = 0x8251C4E8;
	sub_824FFD88(ctx, base);
	// 8251C4E8: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251C4EC: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 8251C4F0: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251C4F4: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 8251C4F8: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251C4FC: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 8251C500: 3908B870  addi r8, r8, -0x4790
	ctx.r[8].s64 = ctx.r[8].s64 + -18320;
	// 8251C504: 3929BC50  addi r9, r9, -0x43b0
	ctx.r[9].s64 = ctx.r[9].s64 + -17328;
	// 8251C508: 394AC188  addi r10, r10, -0x3e78
	ctx.r[10].s64 = ctx.r[10].s64 + -15992;
	// 8251C50C: 396BBEF8  addi r11, r11, -0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + -16648;
	// 8251C510: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8251C514: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 8251C518: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 8251C51C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8251C520: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8251C524: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251C528: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 8251C52C: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8251C530: 4BFE3859  bl 0x824ffd88
	ctx.lr = 0x8251C534;
	sub_824FFD88(ctx, base);
	// 8251C534: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8251C538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251C53C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251C540: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251C544: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251C548: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C550(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251C550 size=232
    let mut pc: u32 = 0x8251C550;
    'dispatch: loop {
        match pc {
            0x8251C550 => {
    //   block [0x8251C550..0x8251C57C)
	// 8251C550: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251C554: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251C558: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251C55C: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251C560: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251C564: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251C568: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 8251C56C: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 8251C570: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 8251C574: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 8251C578: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x8251C57C; continue 'dispatch;
            }
            0x8251C57C => {
    //   block [0x8251C57C..0x8251C638)
	// 8251C57C: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251C580: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 8251C584: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 8251C588: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 8251C58C: 4200FFF0  bdnz 0x8251c57c
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x8251C57C; continue 'dispatch;
	}
	// 8251C590: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251C594: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 8251C598: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 8251C59C: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 8251C5A0: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8251C5A4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251C638 size=76
    let mut pc: u32 = 0x8251C638;
    'dispatch: loop {
        match pc {
            0x8251C638 => {
    //   block [0x8251C638..0x8251C684)
	// 8251C638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251C63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251C640: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251C644: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8251C648: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251C64C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251C650: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8251C654: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8251C658: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 8251C65C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251C660: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251C664: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251C668: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8251C66C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 8251C670: 4BFFF399  bl 0x8251ba08
	ctx.lr = 0x8251C674;
	sub_8251BA08(ctx, base);
	// 8251C674: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251C678: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251C67C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251C680: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C688(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251C688 size=76
    let mut pc: u32 = 0x8251C688;
    'dispatch: loop {
        match pc {
            0x8251C688 => {
    //   block [0x8251C688..0x8251C6D4)
	// 8251C688: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251C68C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251C690: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251C694: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251C698: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251C69C: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251C6A0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8251C6A4: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 8251C6A8: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 8251C6AC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251C6B0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8251C6B4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251C6B8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8251C6BC: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 8251C6C0: 4BFFF591  bl 0x8251bc50
	ctx.lr = 0x8251C6C4;
	sub_8251BC50(ctx, base);
	// 8251C6C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251C6C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251C6CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251C6D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251C6D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251C6D8 size=904
    let mut pc: u32 = 0x8251C6D8;
    'dispatch: loop {
        match pc {
            0x8251C6D8 => {
    //   block [0x8251C6D8..0x8251C734)
	// 8251C6D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251C6DC: 480189A9  bl 0x82535084
	ctx.lr = 0x8251C6E0;
	sub_82535080(ctx, base);
	// 8251C6E0: DBE1FF68  stfd f31, -0x98(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-152 as u32), ctx.f[31].u64 ) };
	// 8251C6E4: 9421FDF0  stwu r1, -0x210(r1)
	ea = ctx.r[1].u32.wrapping_add(-528 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251C6E8: 820D0000  lwz r16, 0(r13)
	ctx.r[16].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C6EC: 3A200014  li r17, 0x14
	ctx.r[17].s64 = 20;
	// 8251C6F0: 7C741B78  mr r20, r3
	ctx.r[20].u64 = ctx.r[3].u64;
	// 8251C6F4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 8251C6F8: 7CB62B78  mr r22, r5
	ctx.r[22].u64 = ctx.r[5].u64;
	// 8251C6FC: 7CD23378  mr r18, r6
	ctx.r[18].u64 = ctx.r[6].u64;
	// 8251C700: 7D71802E  lwzx r11, r17, r16
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[17].u32.wrapping_add(ctx.r[16].u32)) } as u64;
	// 8251C704: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 8251C708: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251C70C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251C710: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251C714: 40980020  bge cr6, 0x8251c734
	if !ctx.cr[6].lt {
	pc = 0x8251C734; continue 'dispatch;
	}
	// 8251C718: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251C71C: 39295238  addi r9, r9, 0x5238
	ctx.r[9].s64 = ctx.r[9].s64 + 21048;
	// 8251C720: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251C724: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251C728: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251C72C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251C730: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251C734; continue 'dispatch;
            }
            0x8251C734 => {
    //   block [0x8251C734..0x8251CA60)
	// 8251C734: 81760008  lwz r11, 8(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251C738: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 8251C73C: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 8251C740: 82F60000  lwz r23, 0(r22)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C744: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 8251C748: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251C74C: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 8251C750: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 8251C754: EB6B0000  ld r27, 0(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251C758: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 8251C75C: EA6B0008  ld r19, 8(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251C760: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 8251C764: EB260000  ld r25, 0(r6)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8251C768: 38770020  addi r3, r23, 0x20
	ctx.r[3].s64 = ctx.r[23].s64 + 32;
	// 8251C76C: E9E60008  ld r15, 8(r6)
	ctx.r[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 8251C770: 3BA100C0  addi r29, r1, 0xc0
	ctx.r[29].s64 = ctx.r[1].s64 + 192;
	// 8251C774: EB050000  ld r24, 0(r5)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 8251C778: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 8251C77C: FB6A0000  std r27, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[27].u64 ) };
	// 8251C780: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 8251C784: FA6A0008  std r19, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[19].u64 ) };
	// 8251C788: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 8251C78C: FB290000  std r25, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
	// 8251C790: 7CC3E850  subf r6, r3, r29
	ctx.r[6].s64 = ctx.r[29].s64 - ctx.r[3].s64;
	// 8251C794: F9E90008  std r15, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[15].u64 ) };
	// 8251C798: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 8251C79C: FB080000  std r24, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251CA60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251CA60 size=80
    let mut pc: u32 = 0x8251CA60;
    'dispatch: loop {
        match pc {
            0x8251CA60 => {
    //   block [0x8251CA60..0x8251CAB0)
	// 8251CA60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251CA64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251CA68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251CA6C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251CA70: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8251CA74: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8251CA78: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 8251CA7C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8251CA80: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251CA84: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251CA88: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251CA8C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8251CA90: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8251CA94: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8251CA98: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251CA9C: 4BFFF405  bl 0x8251bea0
	ctx.lr = 0x8251CAA0;
	sub_8251BEA0(ctx, base);
	// 8251CAA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251CAA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251CAA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251CAAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251CAB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251CAB0 size=80
    let mut pc: u32 = 0x8251CAB0;
    'dispatch: loop {
        match pc {
            0x8251CAB0 => {
    //   block [0x8251CAB0..0x8251CB00)
	// 8251CAB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251CAB4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251CAB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251CABC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251CAC0: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8251CAC4: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 8251CAC8: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 8251CACC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251CAD0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251CAD4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8251CAD8: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251CADC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8251CAE0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8251CAE4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8251CAE8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251CAEC: 4BFFF69D  bl 0x8251c188
	ctx.lr = 0x8251CAF0;
	sub_8251C188(ctx, base);
	// 8251CAF0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251CAF4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251CAF8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251CAFC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251CB00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251CB00 size=176
    let mut pc: u32 = 0x8251CB00;
    'dispatch: loop {
        match pc {
            0x8251CB00 => {
    //   block [0x8251CB00..0x8251CB80)
	// 8251CB00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251CB04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251CB08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251CB0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251CB10: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8251CB14: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251CB18: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8251CB1C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8251CB20: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251CB24: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8251CB28: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CB2C: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8251CB30: 4BFFFBA9  bl 0x8251c6d8
	ctx.lr = 0x8251CB34;
	sub_8251C6D8(ctx, base);
	// 8251CB34: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CB38: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251CB3C: 40980044  bge cr6, 0x8251cb80
	if !ctx.cr[6].lt {
	pc = 0x8251CB80; continue 'dispatch;
	}
	// 8251CB40: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251CB44: 394B08F0  addi r10, r11, 0x8f0
	ctx.r[10].s64 = ctx.r[11].s64 + 2288;
	// 8251CB48: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x8251CB80; continue 'dispatch;
            }
            0x8251CB80 => {
    //   block [0x8251CB80..0x8251CBB0)
	// 8251CB80: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251CB84: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8251CB88: 419A000C  beq cr6, 0x8251cb94
	if ctx.cr[6].eq {
	pc = 0x8251CB94; continue 'dispatch;
	}
	// 8251CB8C: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 8251CB90: 4BFF68B1  bl 0x82513440
	ctx.lr = 0x8251CB94;
	sub_82513440(ctx, base);
	// 8251CB94: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251CB98: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251CB9C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251CBA0: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8251CBA4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251CBA8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251CBAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251CBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251CBB0 size=128
    let mut pc: u32 = 0x8251CBB0;
    'dispatch: loop {
        match pc {
            0x8251CBB0 => {
    //   block [0x8251CBB0..0x8251CC30)
	// 8251CBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251CBB4: 48018509  bl 0x825350bc
	ctx.lr = 0x8251CBB8;
	sub_82535080(ctx, base);
	// 8251CBB8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251CBBC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CBC0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251CBC4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251CBC8: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8251CBCC: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 8251CBD0: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8251CBD4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251CBD8: 4BF47461  bl 0x82464038
	ctx.lr = 0x8251CBDC;
	sub_82464038(ctx, base);
	// 8251CBDC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251CBE0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251CBE4: 394B5284  addi r10, r11, 0x5284
	ctx.r[10].s64 = ctx.r[11].s64 + 21124;
	// 8251CBE8: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 8251CBEC: 39200028  li r9, 0x28
	ctx.r[9].s64 = 40;
	// 8251CBF0: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8251CBF4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8251CBF8: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8251CBFC: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8251CC00: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 8251CC04: B13F0004  sth r9, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 8251CC08: B17F000C  sth r11, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 8251CC0C: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8251CC10: B17F000E  sth r11, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 8251CC14: B17F0010  sth r11, 0x10(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 8251CC18: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CC1C: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 8251CC20: 480AC071  bl 0x825c8c90
	ctx.lr = 0x8251CC24;
	sub_825C8C90(ctx, base);
	// 8251CC24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251CC28: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251CC2C: 480184E0  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251CC30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251CC30 size=120
    let mut pc: u32 = 0x8251CC30;
    'dispatch: loop {
        match pc {
            0x8251CC30 => {
    //   block [0x8251CC30..0x8251CC4C)
	// 8251CC30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251CC34: 48018485  bl 0x825350b8
	ctx.lr = 0x8251CC38;
	sub_82535080(ctx, base);
	// 8251CC38: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251CC3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251CC40: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8251CC44: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 8251CC48: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	pc = 0x8251CC4C; continue 'dispatch;
            }
            0x8251CC4C => {
    //   block [0x8251CC4C..0x8251CC70)
	// 8251CC4C: A09E0000  lhz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CC50: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 8251CC54: 419A001C  beq cr6, 0x8251cc70
	if ctx.cr[6].eq {
	pc = 0x8251CC70; continue 'dispatch;
	}
	// 8251CC58: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251CC5C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251CC60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CC64: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251CC68: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251CC6C: 4E800421  bctrl
	ctx.lr = 0x8251CC70;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8251CC70 => {
    //   block [0x8251CC70..0x8251CCA0)
	// 8251CC70: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 8251CC74: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 8251CC78: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8251CC7C: 409AFFD0  bne cr6, 0x8251cc4c
	if !ctx.cr[6].eq {
	pc = 0x8251CC4C; continue 'dispatch;
	}
	// 8251CC80: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251CC84: 419A001C  beq cr6, 0x8251cca0
	if ctx.cr[6].eq {
	pc = 0x8251CCA0; continue 'dispatch;
	}
	// 8251CC88: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CC8C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8251CC90: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251CC94: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CC98: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251CC9C: 4E800421  bctrl
	ctx.lr = 0x8251CCA0;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8251CCA0 => {
    //   block [0x8251CCA0..0x8251CCA8)
	// 8251CCA0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251CCA4: 48018464  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251CCA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251CCA8 size=140
    let mut pc: u32 = 0x8251CCA8;
    'dispatch: loop {
        match pc {
            0x8251CCA8 => {
    //   block [0x8251CCA8..0x8251CD34)
	// 8251CCA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251CCAC: 48018411  bl 0x825350bc
	ctx.lr = 0x8251CCB0;
	sub_82535080(ctx, base);
	// 8251CCB0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251CCB4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CCB8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251CCBC: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8251CCC0: 38800028  li r4, 0x28
	ctx.r[4].s64 = 40;
	// 8251CCC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251CCC8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8251CCCC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251CCD0: 4BF47369  bl 0x82464038
	ctx.lr = 0x8251CCD4;
	sub_82464038(ctx, base);
	// 8251CCD4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251CCD8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251CCDC: 394B5284  addi r10, r11, 0x5284
	ctx.r[10].s64 = ctx.r[11].s64 + 21124;
	// 8251CCE0: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 8251CCE4: 39200028  li r9, 0x28
	ctx.r[9].s64 = 40;
	// 8251CCE8: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8251CCEC: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8251CCF0: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 8251CCF4: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8251CCF8: 389F0014  addi r4, r31, 0x14
	ctx.r[4].s64 = ctx.r[31].s64 + 20;
	// 8251CCFC: B13F0004  sth r9, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 8251CD00: B17F000C  sth r11, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 8251CD04: B11F0006  sth r8, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8251CD08: B17F000E  sth r11, 0xe(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 8251CD0C: B17F0010  sth r11, 0x10(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 8251CD10: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CD14: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 8251CD18: 480ABF79  bl 0x825c8c90
	ctx.lr = 0x8251CD1C;
	sub_825C8C90(ctx, base);
	// 8251CD1C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251CD20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251CD24: 396B52C0  addi r11, r11, 0x52c0
	ctx.r[11].s64 = ctx.r[11].s64 + 21184;
	// 8251CD28: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251CD2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251CD30: 480183DC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251CD38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251CD38 size=456
    let mut pc: u32 = 0x8251CD38;
    'dispatch: loop {
        match pc {
            0x8251CD38 => {
    //   block [0x8251CD38..0x8251CF00)
	// 8251CD38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251CD3C: 48018359  bl 0x82535094
	ctx.lr = 0x8251CD40;
	sub_82535080(ctx, base);
	// 8251CD40: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251CD44: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251CD48: 7D0A4378  mr r10, r8
	ctx.r[10].u64 = ctx.r[8].u64;
	// 8251CD4C: 3BE10080  addi r31, r1, 0x80
	ctx.r[31].s64 = ctx.r[1].s64 + 128;
	// 8251CD50: 82A30000  lwz r21, 0(r3)
	ctx.r[21].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CD54: 3B8B0010  addi r28, r11, 0x10
	ctx.r[28].s64 = ctx.r[11].s64 + 16;
	// 8251CD58: 82C40000  lwz r22, 0(r4)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CD5C: 3B6B0020  addi r27, r11, 0x20
	ctx.r[27].s64 = ctx.r[11].s64 + 32;
	// 8251CD60: 3B4B0030  addi r26, r11, 0x30
	ctx.r[26].s64 = ctx.r[11].s64 + 48;
	// 8251CD64: E90B0000  ld r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251CD68: 7CE93B78  mr r9, r7
	ctx.r[9].u64 = ctx.r[7].u64;
	// 8251CD6C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251CD70: 3BC10070  addi r30, r1, 0x70
	ctx.r[30].s64 = ctx.r[1].s64 + 112;
	// 8251CD74: E8FC0000  ld r7, 0(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) };
	// 8251CD78: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8251CD7C: EB9C0008  ld r28, 8(r28)
	ctx.r[28].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) };
	// 8251CD80: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 8251CD84: EA9B0000  ld r20, 0(r27)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) };
	// 8251CD88: 3B200001  li r25, 1
	ctx.r[25].s64 = 1;
	// 8251CD8C: F91F0000  std r8, 0(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u64 ) };
	// 8251CD90: 3AE100A0  addi r23, r1, 0xa0
	ctx.r[23].s64 = ctx.r[1].s64 + 160;
	// 8251CD94: F97F0008  std r11, 8(r31)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251CD98: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 8251CD9C: F8FE0000  std r7, 0(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[7].u64 ) };
	// 8251CDA0: 3B150030  addi r24, r21, 0x30
	ctx.r[24].s64 = ctx.r[21].s64 + 48;
	// 8251CDA4: FB9E0008  std r28, 8(r30)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[30].u32.wrapping_add(8 as u32), ctx.r[28].u64 ) };
	// 8251CDA8: EB7B0008  ld r27, 8(r27)
	ctx.r[27].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) };
	// 8251CDAC: FA830000  std r20, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251CF00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251CF00 size=588
    let mut pc: u32 = 0x8251CF00;
    'dispatch: loop {
        match pc {
            0x8251CF00 => {
    //   block [0x8251CF00..0x8251D14C)
	// 8251CF00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251CF04: 48018195  bl 0x82535098
	ctx.lr = 0x8251CF08;
	sub_82535080(ctx, base);
	// 8251CF08: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251CF0C: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251CF10: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8251CF14: 83430000  lwz r26, 0(r3)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CF18: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8251CF1C: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 8251CF20: 83640000  lwz r27, 0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251CF24: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 8251CF28: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 8251CF2C: EAEB0000  ld r23, 0(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251CF30: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 8251CF34: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251CF38: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 8251CF3C: EAC50000  ld r22, 0(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 8251CF40: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 8251CF44: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 8251CF48: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251CF4C: EAA30000  ld r21, 0(r3)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8251CF50: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8251CF54: FAEA0000  std r23, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 8251CF58: 3B810100  addi r28, r1, 0x100
	ctx.r[28].s64 = ctx.r[1].s64 + 256;
	// 8251CF5C: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251CF60: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8251CF64: FAC90000  std r22, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 8251CF68: 3BBA0030  addi r29, r26, 0x30
	ctx.r[29].s64 = ctx.r[26].s64 + 48;
	// 8251CF6C: F8A90008  std r5, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[5].u64 ) };
	// 8251CF70: E8630008  ld r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 8251CF74: FAA80000  std r21, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251D150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251D150 size=688
    let mut pc: u32 = 0x8251D150;
    'dispatch: loop {
        match pc {
            0x8251D150 => {
    //   block [0x8251D150..0x8251D19C)
	// 8251D150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251D154: 48017F3D  bl 0x82535090
	ctx.lr = 0x8251D158;
	sub_82535080(ctx, base);
	// 8251D158: 9421FE10  stwu r1, -0x1f0(r1)
	ea = ctx.r[1].u32.wrapping_add(-496 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251D15C: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D160: 3B000014  li r24, 0x14
	ctx.r[24].s64 = 20;
	// 8251D164: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8251D168: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 8251D16C: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 8251D170: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251D174: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251D178: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251D17C: 40980020  bge cr6, 0x8251d19c
	if !ctx.cr[6].lt {
	pc = 0x8251D19C; continue 'dispatch;
	}
	// 8251D180: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251D184: 39295308  addi r9, r9, 0x5308
	ctx.r[9].s64 = ctx.r[9].s64 + 21256;
	// 8251D188: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251D18C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251D190: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251D194: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251D198: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251D19C; continue 'dispatch;
            }
            0x8251D19C => {
    //   block [0x8251D19C..0x8251D400)
	// 8251D19C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251D1A0: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8251D1A4: 908100B0  stw r4, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[4].u32 ) };
	// 8251D1A8: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 8251D1AC: 90A100B4  stw r5, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[5].u32 ) };
	// 8251D1B0: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 8251D1B4: 83440000  lwz r26, 0(r4)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D1B8: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 8251D1BC: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 8251D1C0: 83650000  lwz r27, 0(r5)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D1C4: EAAB0000  ld r21, 0(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251D1C8: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 8251D1CC: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251D1D0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251D1D4: EA860000  ld r20, 0(r6)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8251D1D8: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8251D1DC: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 8251D1E0: 3B810130  addi r28, r1, 0x130
	ctx.r[28].s64 = ctx.r[1].s64 + 304;
	// 8251D1E4: EA640000  ld r19, 0(r4)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 8251D1E8: 3BBA0030  addi r29, r26, 0x30
	ctx.r[29].s64 = ctx.r[26].s64 + 48;
	// 8251D1EC: FAAA0000  std r21, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 8251D1F0: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251D1F4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8251D1F8: FA890000  std r20, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 8251D1FC: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 8251D200: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 8251D204: FA680000  std r19, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251D400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251D400 size=712
    let mut pc: u32 = 0x8251D400;
    'dispatch: loop {
        match pc {
            0x8251D400 => {
    //   block [0x8251D400..0x8251D454)
	// 8251D400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251D404: 48017C91  bl 0x82535094
	ctx.lr = 0x8251D408;
	sub_82535080(ctx, base);
	// 8251D408: 9421FE00  stwu r1, -0x200(r1)
	ea = ctx.r[1].u32.wrapping_add(-512 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251D40C: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D410: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 8251D414: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251D418: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251D41C: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 8251D420: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 8251D424: 7D7AC82E  lwzx r11, r26, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 8251D428: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251D42C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251D430: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251D434: 40980020  bge cr6, 0x8251d454
	if !ctx.cr[6].lt {
	pc = 0x8251D454; continue 'dispatch;
	}
	// 8251D438: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251D43C: 39295308  addi r9, r9, 0x5308
	ctx.r[9].s64 = ctx.r[9].s64 + 21256;
	// 8251D440: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251D444: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251D448: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251D44C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251D450: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251D454; continue 'dispatch;
            }
            0x8251D454 => {
    //   block [0x8251D454..0x8251D6C8)
	// 8251D454: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D458: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 8251D45C: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 8251D460: 480AB831  bl 0x825c8c90
	ctx.lr = 0x8251D464;
	sub_825C8C90(ctx, base);
	// 8251D464: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251D468: 93C100B0  stw r30, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[30].u32 ) };
	// 8251D46C: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 8251D470: 93E100B4  stw r31, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[31].u32 ) };
	// 8251D474: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 8251D478: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 8251D47C: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D480: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 8251D484: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D488: EACB0000  ld r22, 0(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251D48C: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8251D490: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251D494: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 8251D498: EAA60000  ld r21, 0(r6)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8251D49C: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 8251D4A0: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 8251D4A4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8251D4A8: EA850000  ld r20, 0(r5)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 8251D4AC: 3BA10150  addi r29, r1, 0x150
	ctx.r[29].s64 = ctx.r[1].s64 + 336;
	// 8251D4B0: FACA0000  std r22, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 8251D4B4: 3BDB0030  addi r30, r27, 0x30
	ctx.r[30].s64 = ctx.r[27].s64 + 48;
	// 8251D4B8: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251D4BC: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 8251D4C0: FAA90000  std r21, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 8251D4C4: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 8251D4C8: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 8251D4CC: FA880000  std r20, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251D6C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251D6C8 size=632
    let mut pc: u32 = 0x8251D6C8;
    'dispatch: loop {
        match pc {
            0x8251D6C8 => {
    //   block [0x8251D6C8..0x8251D718)
	// 8251D6C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251D6CC: 480179C1  bl 0x8253508c
	ctx.lr = 0x8251D6D0;
	sub_82535080(ctx, base);
	// 8251D6D0: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251D6D4: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D6D8: 3AE00014  li r23, 0x14
	ctx.r[23].s64 = 20;
	// 8251D6DC: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8251D6E0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 8251D6E4: 7CF53B78  mr r21, r7
	ctx.r[21].u64 = ctx.r[7].u64;
	// 8251D6E8: 7D77B02E  lwzx r11, r23, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 8251D6EC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251D6F0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251D6F4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251D6F8: 40980020  bge cr6, 0x8251d718
	if !ctx.cr[6].lt {
	pc = 0x8251D718; continue 'dispatch;
	}
	// 8251D6FC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251D700: 39295308  addi r9, r9, 0x5308
	ctx.r[9].s64 = ctx.r[9].s64 + 21256;
	// 8251D704: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251D708: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251D70C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251D710: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251D714: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251D718; continue 'dispatch;
            }
            0x8251D718 => {
    //   block [0x8251D718..0x8251D940)
	// 8251D718: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251D71C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8251D720: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 8251D724: 83380000  lwz r25, 0(r24)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D728: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 8251D72C: 837A0000  lwz r27, 0(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D730: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 8251D734: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 8251D738: EA8B0000  ld r20, 0(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251D73C: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 8251D740: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251D744: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251D748: EA650000  ld r19, 0(r5)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 8251D74C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8251D750: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 8251D754: 3B8100A0  addi r28, r1, 0xa0
	ctx.r[28].s64 = ctx.r[1].s64 + 160;
	// 8251D758: EA440000  ld r18, 0(r4)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 8251D75C: 3BB90030  addi r29, r25, 0x30
	ctx.r[29].s64 = ctx.r[25].s64 + 48;
	// 8251D760: FA8A0000  std r20, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 8251D764: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251D768: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8251D76C: FA690000  std r19, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 8251D770: F8A90008  std r5, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[5].u64 ) };
	// 8251D774: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 8251D778: FA480000  std r18, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251D940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251D940 size=652
    let mut pc: u32 = 0x8251D940;
    'dispatch: loop {
        match pc {
            0x8251D940 => {
    //   block [0x8251D940..0x8251D994)
	// 8251D940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251D944: 4801774D  bl 0x82535090
	ctx.lr = 0x8251D948;
	sub_82535080(ctx, base);
	// 8251D948: 9421FE20  stwu r1, -0x1e0(r1)
	ea = ctx.r[1].u32.wrapping_add(-480 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251D94C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D950: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 8251D954: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 8251D958: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251D95C: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 8251D960: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 8251D964: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8251D968: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251D96C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251D970: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251D974: 40980020  bge cr6, 0x8251d994
	if !ctx.cr[6].lt {
	pc = 0x8251D994; continue 'dispatch;
	}
	// 8251D978: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251D97C: 39295308  addi r9, r9, 0x5308
	ctx.r[9].s64 = ctx.r[9].s64 + 21256;
	// 8251D980: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251D984: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251D988: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251D98C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251D990: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251D994; continue 'dispatch;
            }
            0x8251D994 => {
    //   block [0x8251D994..0x8251DBCC)
	// 8251D994: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D998: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 8251D99C: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 8251D9A0: 480AB2F1  bl 0x825c8c90
	ctx.lr = 0x8251D9A4;
	sub_825C8C90(ctx, base);
	// 8251D9A4: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251D9A8: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 8251D9AC: 837A0000  lwz r27, 0(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D9B0: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 8251D9B4: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251D9B8: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 8251D9BC: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 8251D9C0: EAAB0000  ld r21, 0(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251D9C4: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 8251D9C8: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251D9CC: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 8251D9D0: EA860000  ld r20, 0(r6)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8251D9D4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251D9D8: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 8251D9DC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8251D9E0: EA650000  ld r19, 0(r5)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 8251D9E4: 3BA100C0  addi r29, r1, 0xc0
	ctx.r[29].s64 = ctx.r[1].s64 + 192;
	// 8251D9E8: FAAA0000  std r21, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 8251D9EC: 3BDB0030  addi r30, r27, 0x30
	ctx.r[30].s64 = ctx.r[27].s64 + 48;
	// 8251D9F0: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251D9F4: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 8251D9F8: FA890000  std r20, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 8251D9FC: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 8251DA00: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 8251DA04: FA680000  std r19, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251DBD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251DBD0 size=64
    let mut pc: u32 = 0x8251DBD0;
    'dispatch: loop {
        match pc {
            0x8251DBD0 => {
    //   block [0x8251DBD0..0x8251DC10)
	// 8251DBD0: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 8251DBD4: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251DBD8: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251DBDC: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251DBE0: 38EBBEF8  addi r7, r11, -0x4108
	ctx.r[7].s64 = ctx.r[11].s64 + -16648;
	// 8251DBE4: 3908CBB0  addi r8, r8, -0x3450
	ctx.r[8].s64 = ctx.r[8].s64 + -13392;
	// 8251DBE8: 3929D940  addi r9, r9, -0x26c0
	ctx.r[9].s64 = ctx.r[9].s64 + -9920;
	// 8251DBEC: 394AD400  addi r10, r10, -0x2c00
	ctx.r[10].s64 = ctx.r[10].s64 + -11264;
	// 8251DBF0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251DBF4: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 8251DBF8: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8251DBFC: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251DC00: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8251DC04: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 8251DC08: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 8251DC0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251DC10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251DC10 size=68
    let mut pc: u32 = 0x8251DC10;
    'dispatch: loop {
        match pc {
            0x8251DC10 => {
    //   block [0x8251DC10..0x8251DC54)
	// 8251DC10: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251DC14: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251DC18: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251DC1C: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 8251DC20: 3908CCA8  addi r8, r8, -0x3358
	ctx.r[8].s64 = ctx.r[8].s64 + -13144;
	// 8251DC24: 3929E160  addi r9, r9, -0x1ea0
	ctx.r[9].s64 = ctx.r[9].s64 + -7840;
	// 8251DC28: 394AE200  addi r10, r10, -0x1e00
	ctx.r[10].s64 = ctx.r[10].s64 + -7680;
	// 8251DC2C: 396BC550  addi r11, r11, -0x3ab0
	ctx.r[11].s64 = ctx.r[11].s64 + -15024;
	// 8251DC30: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 8251DC34: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8251DC38: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 8251DC3C: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251DC40: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8251DC44: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 8251DC48: 98E30010  stb r7, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u8 ) };
	// 8251DC4C: 98C30011  stb r6, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[6].u8 ) };
	// 8251DC50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251DC58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251DC58 size=204
    let mut pc: u32 = 0x8251DC58;
    'dispatch: loop {
        match pc {
            0x8251DC58 => {
    //   block [0x8251DC58..0x8251DD24)
	// 8251DC58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251DC5C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251DC60: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251DC64: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251DC68: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251DC6C: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 8251DC70: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251DC74: 396BC550  addi r11, r11, -0x3ab0
	ctx.r[11].s64 = ctx.r[11].s64 + -15024;
	// 8251DC78: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251DC7C: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251DC80: 3908CCA8  addi r8, r8, -0x3358
	ctx.r[8].s64 = ctx.r[8].s64 + -13144;
	// 8251DC84: 3929E160  addi r9, r9, -0x1ea0
	ctx.r[9].s64 = ctx.r[9].s64 + -7840;
	// 8251DC88: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8251DC8C: 394AE200  addi r10, r10, -0x1e00
	ctx.r[10].s64 = ctx.r[10].s64 + -7680;
	// 8251DC90: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8251DC94: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8251DC98: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8251DC9C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 8251DCA0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8251DCA4: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8251DCA8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251DCAC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251DCB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251DCB4: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 8251DCB8: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 8251DCBC: 4BFE20CD  bl 0x824ffd88
	ctx.lr = 0x8251DCC0;
	sub_824FFD88(ctx, base);
	// 8251DCC0: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251DCC4: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 8251DCC8: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251DCCC: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 8251DCD0: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251DCD4: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 8251DCD8: 3908CBB0  addi r8, r8, -0x3450
	ctx.r[8].s64 = ctx.r[8].s64 + -13392;
	// 8251DCDC: 3929D940  addi r9, r9, -0x26c0
	ctx.r[9].s64 = ctx.r[9].s64 + -9920;
	// 8251DCE0: 394AD400  addi r10, r10, -0x2c00
	ctx.r[10].s64 = ctx.r[10].s64 + -11264;
	// 8251DCE4: 396BBEF8  addi r11, r11, -0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + -16648;
	// 8251DCE8: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8251DCEC: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 8251DCF0: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 8251DCF4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8251DCF8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8251DCFC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251DD00: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 8251DD04: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8251DD08: 4BFE2081  bl 0x824ffd88
	ctx.lr = 0x8251DD0C;
	sub_824FFD88(ctx, base);
	// 8251DD0C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8251DD10: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251DD14: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251DD18: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251DD1C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251DD20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251DD28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251DD28 size=204
    let mut pc: u32 = 0x8251DD28;
    'dispatch: loop {
        match pc {
            0x8251DD28 => {
    //   block [0x8251DD28..0x8251DDF4)
	// 8251DD28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251DD2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251DD30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251DD34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251DD38: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251DD3C: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 8251DD40: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251DD44: 396BC550  addi r11, r11, -0x3ab0
	ctx.r[11].s64 = ctx.r[11].s64 + -15024;
	// 8251DD48: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251DD4C: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251DD50: 3908CCA8  addi r8, r8, -0x3358
	ctx.r[8].s64 = ctx.r[8].s64 + -13144;
	// 8251DD54: 3929E160  addi r9, r9, -0x1ea0
	ctx.r[9].s64 = ctx.r[9].s64 + -7840;
	// 8251DD58: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8251DD5C: 394AE200  addi r10, r10, -0x1e00
	ctx.r[10].s64 = ctx.r[10].s64 + -7680;
	// 8251DD60: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8251DD64: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8251DD68: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8251DD6C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 8251DD70: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 8251DD74: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8251DD78: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251DD7C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251DD80: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251DD84: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 8251DD88: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 8251DD8C: 4BFE2115  bl 0x824ffea0
	ctx.lr = 0x8251DD90;
	sub_824FFEA0(ctx, base);
	// 8251DD90: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251DD94: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 8251DD98: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251DD9C: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 8251DDA0: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251DDA4: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 8251DDA8: 3908CBB0  addi r8, r8, -0x3450
	ctx.r[8].s64 = ctx.r[8].s64 + -13392;
	// 8251DDAC: 3929D940  addi r9, r9, -0x26c0
	ctx.r[9].s64 = ctx.r[9].s64 + -9920;
	// 8251DDB0: 394AD400  addi r10, r10, -0x2c00
	ctx.r[10].s64 = ctx.r[10].s64 + -11264;
	// 8251DDB4: 396BBEF8  addi r11, r11, -0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + -16648;
	// 8251DDB8: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 8251DDBC: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 8251DDC0: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 8251DDC4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8251DDC8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8251DDCC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251DDD0: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 8251DDD4: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8251DDD8: 4BFE20C9  bl 0x824ffea0
	ctx.lr = 0x8251DDDC;
	sub_824FFEA0(ctx, base);
	// 8251DDDC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8251DDE0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251DDE4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251DDE8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251DDEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251DDF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251DDF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251DDF8 size=792
    let mut pc: u32 = 0x8251DDF8;
    'dispatch: loop {
        match pc {
            0x8251DDF8 => {
    //   block [0x8251DDF8..0x8251DE50)
	// 8251DDF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251DDFC: 48017291  bl 0x8253508c
	ctx.lr = 0x8251DE00;
	sub_82535080(ctx, base);
	// 8251DE00: 9421FE40  stwu r1, -0x1c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-448 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251DE04: 828D0000  lwz r20, 0(r13)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DE08: 3AA00014  li r21, 0x14
	ctx.r[21].s64 = 20;
	// 8251DE0C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8251DE10: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 8251DE14: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8251DE18: 7CD63378  mr r22, r6
	ctx.r[22].u64 = ctx.r[6].u64;
	// 8251DE1C: 7D75A02E  lwzx r11, r21, r20
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 8251DE20: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8251DE24: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251DE28: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251DE2C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251DE30: 40980020  bge cr6, 0x8251de50
	if !ctx.cr[6].lt {
	pc = 0x8251DE50; continue 'dispatch;
	}
	// 8251DE34: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251DE38: 392952F8  addi r9, r9, 0x52f8
	ctx.r[9].s64 = ctx.r[9].s64 + 21240;
	// 8251DE3C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251DE40: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251DE44: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251DE48: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251DE4C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251DE50; continue 'dispatch;
            }
            0x8251DE50 => {
    //   block [0x8251DE50..0x8251E110)
	// 8251DE50: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251DE54: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8251DE58: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 8251DE5C: 83980000  lwz r28, 0(r24)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DE60: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 8251DE64: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251DE68: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 8251DE6C: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 8251DE70: EAEB0000  ld r23, 0(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251DE74: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 8251DE78: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251DE7C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251DE80: EA660000  ld r19, 0(r6)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8251DE84: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8251DE88: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 8251DE8C: 3BC100A0  addi r30, r1, 0xa0
	ctx.r[30].s64 = ctx.r[1].s64 + 160;
	// 8251DE90: EA450000  ld r18, 0(r5)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 8251DE94: 3BFC0030  addi r31, r28, 0x30
	ctx.r[31].s64 = ctx.r[28].s64 + 48;
	// 8251DE98: FAEA0000  std r23, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 8251DE9C: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251DEA0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8251DEA4: FA690000  std r19, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 8251DEA8: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 8251DEAC: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 8251DEB0: FA480000  std r18, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E110(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E110 size=76
    let mut pc: u32 = 0x8251E110;
    'dispatch: loop {
        match pc {
            0x8251E110 => {
    //   block [0x8251E110..0x8251E15C)
	// 8251E110: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E114: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251E118: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E11C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8251E120: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251E124: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251E128: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8251E12C: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8251E130: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 8251E134: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251E138: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251E13C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251E140: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8251E144: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 8251E148: 4BFFF581  bl 0x8251d6c8
	ctx.lr = 0x8251E14C;
	sub_8251D6C8(ctx, base);
	// 8251E14C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251E150: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251E154: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251E158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E160 size=76
    let mut pc: u32 = 0x8251E160;
    'dispatch: loop {
        match pc {
            0x8251E160 => {
    //   block [0x8251E160..0x8251E1AC)
	// 8251E160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E164: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251E168: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E16C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251E170: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251E174: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251E178: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8251E17C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 8251E180: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 8251E184: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251E188: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8251E18C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251E190: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8251E194: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 8251E198: 4BFFF7A9  bl 0x8251d940
	ctx.lr = 0x8251E19C;
	sub_8251D940(ctx, base);
	// 8251E19C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251E1A0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251E1A4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251E1A8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E1B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251E1B0 size=80
    let mut pc: u32 = 0x8251E1B0;
    'dispatch: loop {
        match pc {
            0x8251E1B0 => {
    //   block [0x8251E1B0..0x8251E200)
	// 8251E1B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E1B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251E1B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E1BC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251E1C0: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8251E1C4: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 8251E1C8: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 8251E1CC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8251E1D0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251E1D4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251E1D8: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251E1DC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8251E1E0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8251E1E4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8251E1E8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251E1EC: 4BFFEF65  bl 0x8251d150
	ctx.lr = 0x8251E1F0;
	sub_8251D150(ctx, base);
	// 8251E1F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251E1F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251E1F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251E1FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251E200 size=80
    let mut pc: u32 = 0x8251E200;
    'dispatch: loop {
        match pc {
            0x8251E200 => {
    //   block [0x8251E200..0x8251E250)
	// 8251E200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E204: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251E208: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E20C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251E210: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8251E214: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 8251E218: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 8251E21C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251E220: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251E224: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8251E228: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251E22C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8251E230: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8251E234: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8251E238: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251E23C: 4BFFF1C5  bl 0x8251d400
	ctx.lr = 0x8251E240;
	sub_8251D400(ctx, base);
	// 8251E240: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251E244: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251E248: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251E24C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E250(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251E250 size=176
    let mut pc: u32 = 0x8251E250;
    'dispatch: loop {
        match pc {
            0x8251E250 => {
    //   block [0x8251E250..0x8251E2D0)
	// 8251E250: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E254: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251E258: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251E25C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251E260: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8251E264: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E268: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 8251E26C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8251E270: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251E274: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 8251E278: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E27C: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 8251E280: 4BFFFB79  bl 0x8251ddf8
	ctx.lr = 0x8251E284;
	sub_8251DDF8(ctx, base);
	// 8251E284: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E288: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 8251E28C: 40980044  bge cr6, 0x8251e2d0
	if !ctx.cr[6].lt {
	pc = 0x8251E2D0; continue 'dispatch;
	}
	// 8251E290: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251E294: 394B08F0  addi r10, r11, 0x8f0
	ctx.r[10].s64 = ctx.r[11].s64 + 2288;
	// 8251E298: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x8251E2D0; continue 'dispatch;
            }
            0x8251E2D0 => {
    //   block [0x8251E2D0..0x8251E300)
	// 8251E2D0: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251E2D4: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 8251E2D8: 419A000C  beq cr6, 0x8251e2e4
	if ctx.cr[6].eq {
	pc = 0x8251E2E4; continue 'dispatch;
	}
	// 8251E2DC: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 8251E2E0: 4BFF5161  bl 0x82513440
	ctx.lr = 0x8251E2E4;
	sub_82513440(ctx, base);
	// 8251E2E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251E2E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251E2EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251E2F0: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8251E2F4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251E2F8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251E2FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E300 size=116
    let mut pc: u32 = 0x8251E300;
    'dispatch: loop {
        match pc {
            0x8251E300 => {
    //   block [0x8251E300..0x8251E374)
	// 8251E300: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E304: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251E308: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251E30C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E310: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E314: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251E318: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8251E31C: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 8251E320: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8251E324: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251E328: 4BF45D11  bl 0x82464038
	ctx.lr = 0x8251E32C;
	sub_82464038(ctx, base);
	// 8251E32C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251E330: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8251E334: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 8251E338: 394B5334  addi r10, r11, 0x5334
	ctx.r[10].s64 = ctx.r[11].s64 + 21300;
	// 8251E33C: 3D600000  lis r11, 0
	ctx.r[11].s64 = 0;
	// 8251E340: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 8251E344: 616BFFFF  ori r11, r11, 0xffff
	ctx.r[11].u64 = ctx.r[11].u64 | 65535;
	// 8251E348: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 8251E34C: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 8251E350: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 8251E354: B163000C  sth r11, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
	// 8251E358: B163000E  sth r11, 0xe(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 8251E35C: B1630010  sth r11, 0x10(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u16 ) };
	// 8251E360: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251E364: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251E368: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251E36C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251E370: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E378 size=120
    let mut pc: u32 = 0x8251E378;
    'dispatch: loop {
        match pc {
            0x8251E378 => {
    //   block [0x8251E378..0x8251E394)
	// 8251E378: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E37C: 48016D3D  bl 0x825350b8
	ctx.lr = 0x8251E380;
	sub_82535080(ctx, base);
	// 8251E380: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E384: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251E388: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8251E38C: 3BDF000C  addi r30, r31, 0xc
	ctx.r[30].s64 = ctx.r[31].s64 + 12;
	// 8251E390: 3BA00003  li r29, 3
	ctx.r[29].s64 = 3;
	pc = 0x8251E394; continue 'dispatch;
            }
            0x8251E394 => {
    //   block [0x8251E394..0x8251E3B8)
	// 8251E394: A09E0000  lhz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E398: 2B04FFFF  cmplwi cr6, r4, 0xffff
	ctx.cr[6].compare_u32(ctx.r[4].u32, 65535 as u32, &mut ctx.xer);
	// 8251E39C: 419A001C  beq cr6, 0x8251e3b8
	if ctx.cr[6].eq {
	pc = 0x8251E3B8; continue 'dispatch;
	}
	// 8251E3A0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251E3A4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251E3A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E3AC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251E3B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251E3B4: 4E800421  bctrl
	ctx.lr = 0x8251E3B8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8251E3B8 => {
    //   block [0x8251E3B8..0x8251E3E8)
	// 8251E3B8: 3BBDFFFF  addi r29, r29, -1
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	// 8251E3BC: 3BDE0002  addi r30, r30, 2
	ctx.r[30].s64 = ctx.r[30].s64 + 2;
	// 8251E3C0: 2B1D0000  cmplwi cr6, r29, 0
	ctx.cr[6].compare_u32(ctx.r[29].u32, 0 as u32, &mut ctx.xer);
	// 8251E3C4: 409AFFD0  bne cr6, 0x8251e394
	if !ctx.cr[6].eq {
	pc = 0x8251E394; continue 'dispatch;
	}
	// 8251E3C8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 8251E3CC: 419A001C  beq cr6, 0x8251e3e8
	if ctx.cr[6].eq {
	pc = 0x8251E3E8; continue 'dispatch;
	}
	// 8251E3D0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E3D4: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8251E3D8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251E3DC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E3E0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251E3E4: 4E800421  bctrl
	ctx.lr = 0x8251E3E8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8251E3E8 => {
    //   block [0x8251E3E8..0x8251E3F0)
	// 8251E3E8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8251E3EC: 48016D1C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E3F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E3F0 size=672
    let mut pc: u32 = 0x8251E3F0;
    'dispatch: loop {
        match pc {
            0x8251E3F0 => {
    //   block [0x8251E3F0..0x8251E440)
	// 8251E3F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E3F4: 48016CA1  bl 0x82535094
	ctx.lr = 0x8251E3F8;
	sub_82535080(ctx, base);
	// 8251E3F8: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E3FC: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E400: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 8251E404: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 8251E408: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8251E40C: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 8251E410: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8251E414: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251E418: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251E41C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251E420: 40980020  bge cr6, 0x8251e440
	if !ctx.cr[6].lt {
	pc = 0x8251E440; continue 'dispatch;
	}
	// 8251E424: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251E428: 3929536C  addi r9, r9, 0x536c
	ctx.r[9].s64 = ctx.r[9].s64 + 21356;
	// 8251E42C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251E430: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251E434: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8251E438: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251E43C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251E440; continue 'dispatch;
            }
            0x8251E440 => {
    //   block [0x8251E440..0x8251E690)
	// 8251E440: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251E444: 39410070  addi r10, r1, 0x70
	ctx.r[10].s64 = ctx.r[1].s64 + 112;
	// 8251E448: 39210050  addi r9, r1, 0x50
	ctx.r[9].s64 = ctx.r[1].s64 + 80;
	// 8251E44C: 835B0000  lwz r26, 0(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E450: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 8251E454: 839D0000  lwz r28, 0(r29)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E458: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 8251E45C: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 8251E460: EACB0000  ld r22, 0(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251E464: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 8251E468: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251E46C: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 8251E470: EAA60000  ld r21, 0(r6)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8251E474: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8251E478: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 8251E47C: 3BC100D0  addi r30, r1, 0xd0
	ctx.r[30].s64 = ctx.r[1].s64 + 208;
	// 8251E480: EA850000  ld r20, 0(r5)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 8251E484: 3BFA0030  addi r31, r26, 0x30
	ctx.r[31].s64 = ctx.r[26].s64 + 48;
	// 8251E488: FACA0000  std r22, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 8251E48C: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251E490: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 8251E494: FAA90000  std r21, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 8251E498: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 8251E49C: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 8251E4A0: FA880000  std r20, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E690(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251E690 size=20
    let mut pc: u32 = 0x8251E690;
    'dispatch: loop {
        match pc {
            0x8251E690 => {
    //   block [0x8251E690..0x8251E6A4)
	// 8251E690: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251E694: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251E698: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8251E69C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8251E6A0: 4BFFFD50  b 0x8251e3f0
	sub_8251E3F0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E6A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251E6A8 size=460
    let mut pc: u32 = 0x8251E6A8;
    'dispatch: loop {
        match pc {
            0x8251E6A8 => {
    //   block [0x8251E6A8..0x8251E874)
	// 8251E6A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E6AC: 480169F5  bl 0x825350a0
	ctx.lr = 0x8251E6B0;
	sub_82535080(ctx, base);
	// 8251E6B0: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E6B4: 7CC73378  mr r7, r6
	ctx.r[7].u64 = ctx.r[6].u64;
	// 8251E6B8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251E6BC: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251E6C0: 83430000  lwz r26, 0(r3)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E6C4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 8251E6C8: 83640000  lwz r27, 0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E6CC: 3BEB0020  addi r31, r11, 0x20
	ctx.r[31].s64 = ctx.r[11].s64 + 32;
	// 8251E6D0: 3BCB0030  addi r30, r11, 0x30
	ctx.r[30].s64 = ctx.r[11].s64 + 48;
	// 8251E6D4: D007001C  stfs f0, 0x1c(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(28 as u32), tmp.u32 ) };
	// 8251E6D8: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8251E6DC: D007003C  stfs f0, 0x3c(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(60 as u32), tmp.u32 ) };
	// 8251E6E0: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8251E6E4: D007005C  stfs f0, 0x5c(r7)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[7].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 8251E6E8: EB2B0000  ld r25, 0(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251E6EC: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251E6F0: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 8251E6F4: EB030000  ld r24, 0(r3)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8251E6F8: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 8251E6FC: E8630008  ld r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 8251E700: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 8251E704: EAFF0000  ld r23, 0(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 8251E708: 3B8100B0  addi r28, r1, 0xb0
	ctx.r[28].s64 = ctx.r[1].s64 + 176;
	// 8251E70C: FB2A0000  std r25, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
	// 8251E710: 3BBA0030  addi r29, r26, 0x30
	ctx.r[29].s64 = ctx.r[26].s64 + 48;
	// 8251E714: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251E718: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8251E71C: FB090000  std r24, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
	// 8251E720: F8690008  std r3, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[3].u64 ) };
	// 8251E724: EBFF0008  ld r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 8251E728: FAE80000  std r23, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251E878(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251E878 size=448
    let mut pc: u32 = 0x8251E878;
    'dispatch: loop {
        match pc {
            0x8251E878 => {
    //   block [0x8251E878..0x8251EA38)
	// 8251E878: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251E87C: 48016821  bl 0x8253509c
	ctx.lr = 0x8251E880;
	sub_82535080(ctx, base);
	// 8251E880: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251E884: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251E888: 7CC83378  mr r8, r6
	ctx.r[8].u64 = ctx.r[6].u64;
	// 8251E88C: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 8251E890: 83030000  lwz r24, 0(r3)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E894: 3BEB0010  addi r31, r11, 0x10
	ctx.r[31].s64 = ctx.r[11].s64 + 16;
	// 8251E898: 83240000  lwz r25, 0(r4)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251E89C: 3BCB0020  addi r30, r11, 0x20
	ctx.r[30].s64 = ctx.r[11].s64 + 32;
	// 8251E8A0: 3BAB0030  addi r29, r11, 0x30
	ctx.r[29].s64 = ctx.r[11].s64 + 48;
	// 8251E8A4: E8CB0000  ld r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251E8A8: 39210060  addi r9, r1, 0x60
	ctx.r[9].s64 = ctx.r[1].s64 + 96;
	// 8251E8AC: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251E8B0: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 8251E8B4: EAFF0000  ld r23, 0(r31)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 8251E8B8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8251E8BC: EBFF0008  ld r31, 8(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) };
	// 8251E8C0: 3B800001  li r28, 1
	ctx.r[28].s64 = 1;
	// 8251E8C4: EADE0000  ld r22, 0(r30)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 8251E8C8: 3B4100B0  addi r26, r1, 0xb0
	ctx.r[26].s64 = ctx.r[1].s64 + 176;
	// 8251E8CC: F8CA0000  std r6, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[6].u64 ) };
	// 8251E8D0: 3B780030  addi r27, r24, 0x30
	ctx.r[27].s64 = ctx.r[24].s64 + 48;
	// 8251E8D4: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251E8D8: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 8251E8DC: FAE90000  std r23, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 8251E8E0: FBE90008  std r31, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[31].u64 ) };
	// 8251E8E4: EBDE0008  ld r30, 8(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 8251E8E8: FAC70000  std r22, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251EA38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251EA38 size=596
    let mut pc: u32 = 0x8251EA38;
    'dispatch: loop {
        match pc {
            0x8251EA38 => {
    //   block [0x8251EA38..0x8251EA80)
	// 8251EA38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251EA3C: 48016659  bl 0x82535094
	ctx.lr = 0x8251EA40;
	sub_82535080(ctx, base);
	// 8251EA40: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251EA44: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251EA48: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 8251EA4C: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 8251EA50: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8251EA54: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251EA58: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251EA5C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251EA60: 40980020  bge cr6, 0x8251ea80
	if !ctx.cr[6].lt {
	pc = 0x8251EA80; continue 'dispatch;
	}
	// 8251EA64: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251EA68: 3929536C  addi r9, r9, 0x536c
	ctx.r[9].s64 = ctx.r[9].s64 + 21356;
	// 8251EA6C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251EA70: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251EA74: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251EA78: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251EA7C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251EA80; continue 'dispatch;
            }
            0x8251EA80 => {
    //   block [0x8251EA80..0x8251EC8C)
	// 8251EA80: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251EA84: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8251EA88: 908100B0  stw r4, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[4].u32 ) };
	// 8251EA8C: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 8251EA90: 90A100B4  stw r5, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[5].u32 ) };
	// 8251EA94: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 8251EA98: 83440000  lwz r26, 0(r4)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251EA9C: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 8251EAA0: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 8251EAA4: 83650000  lwz r27, 0(r5)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251EAA8: EACB0000  ld r22, 0(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251EAAC: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 8251EAB0: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251EAB4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251EAB8: EA830000  ld r20, 0(r3)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8251EABC: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8251EAC0: E8630008  ld r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 8251EAC4: 3B8100F0  addi r28, r1, 0xf0
	ctx.r[28].s64 = ctx.r[1].s64 + 240;
	// 8251EAC8: EAA40000  ld r21, 0(r4)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 8251EACC: 3BBA0030  addi r29, r26, 0x30
	ctx.r[29].s64 = ctx.r[26].s64 + 48;
	// 8251EAD0: FACA0000  std r22, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 8251EAD4: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251EAD8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8251EADC: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 8251EAE0: FA880000  std r20, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 8251EAE4: FAA90000  std r21, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 8251EAE8: EA7F0000  ld r19, 0(r31)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251EC90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251EC90 size=596
    let mut pc: u32 = 0x8251EC90;
    'dispatch: loop {
        match pc {
            0x8251EC90 => {
    //   block [0x8251EC90..0x8251ECD8)
	// 8251EC90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251EC94: 48016401  bl 0x82535094
	ctx.lr = 0x8251EC98;
	sub_82535080(ctx, base);
	// 8251EC98: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251EC9C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251ECA0: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 8251ECA4: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 8251ECA8: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8251ECAC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251ECB0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251ECB4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251ECB8: 40980020  bge cr6, 0x8251ecd8
	if !ctx.cr[6].lt {
	pc = 0x8251ECD8; continue 'dispatch;
	}
	// 8251ECBC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251ECC0: 3929536C  addi r9, r9, 0x536c
	ctx.r[9].s64 = ctx.r[9].s64 + 21356;
	// 8251ECC4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251ECC8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251ECCC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251ECD0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251ECD4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251ECD8; continue 'dispatch;
            }
            0x8251ECD8 => {
    //   block [0x8251ECD8..0x8251EEE4)
	// 8251ECD8: 81630008  lwz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251ECDC: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8251ECE0: 906100B0  stw r3, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[3].u32 ) };
	// 8251ECE4: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 8251ECE8: 908100B4  stw r4, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[4].u32 ) };
	// 8251ECEC: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 8251ECF0: 83430000  lwz r26, 0(r3)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251ECF4: 3BEB0030  addi r31, r11, 0x30
	ctx.r[31].s64 = ctx.r[11].s64 + 48;
	// 8251ECF8: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 8251ECFC: 83640000  lwz r27, 0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251ED00: EACB0000  ld r22, 0(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251ED04: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 8251ED08: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251ED0C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251ED10: EAA60000  ld r21, 0(r6)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8251ED14: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 8251ED18: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 8251ED1C: 3B8100F0  addi r28, r1, 0xf0
	ctx.r[28].s64 = ctx.r[1].s64 + 240;
	// 8251ED20: EA830000  ld r20, 0(r3)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
	// 8251ED24: 3BBA0030  addi r29, r26, 0x30
	ctx.r[29].s64 = ctx.r[26].s64 + 48;
	// 8251ED28: FACA0000  std r22, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 8251ED2C: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251ED30: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8251ED34: FAA90000  std r21, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 8251ED38: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 8251ED3C: E8630008  ld r3, 8(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) };
	// 8251ED40: FA880000  std r20, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251EEE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251EEE8 size=104
    let mut pc: u32 = 0x8251EEE8;
    'dispatch: loop {
        match pc {
            0x8251EEE8 => {
    //   block [0x8251EEE8..0x8251EF50)
	// 8251EEE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251EEEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251EEF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251EEF4: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 8251EEF8: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251EEFC: 396BBEF8  addi r11, r11, -0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + -16648;
	// 8251EF00: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251EF04: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251EF08: 3908E300  addi r8, r8, -0x1d00
	ctx.r[8].s64 = ctx.r[8].s64 + -7424;
	// 8251EF0C: 3929E3F0  addi r9, r9, -0x1c10
	ctx.r[9].s64 = ctx.r[9].s64 + -7184;
	// 8251EF10: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8251EF14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8251EF18: 394AEC90  addi r10, r10, -0x1370
	ctx.r[10].s64 = ctx.r[10].s64 + -4976;
	// 8251EF1C: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 8251EF20: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 8251EF24: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 8251EF28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251EF2C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8251EF30: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 8251EF34: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251EF38: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 8251EF3C: 4BFE0E4D  bl 0x824ffd88
	ctx.lr = 0x8251EF40;
	sub_824FFD88(ctx, base);
	// 8251EF40: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251EF44: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251EF48: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251EF4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251EF50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x8251EF50 size=796
    let mut pc: u32 = 0x8251EF50;
    'dispatch: loop {
        match pc {
            0x8251EF50 => {
    //   block [0x8251EF50..0x8251EFA8)
	// 8251EF50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251EF54: 48016139  bl 0x8253508c
	ctx.lr = 0x8251EF58;
	sub_82535080(ctx, base);
	// 8251EF58: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251EF5C: 828D0000  lwz r20, 0(r13)
	ctx.r[20].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251EF60: 3AA00014  li r21, 0x14
	ctx.r[21].s64 = 20;
	// 8251EF64: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 8251EF68: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 8251EF6C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8251EF70: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 8251EF74: 7D75A02E  lwzx r11, r21, r20
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[20].u32)) } as u64;
	// 8251EF78: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8251EF7C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251EF80: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251EF84: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251EF88: 40980020  bge cr6, 0x8251efa8
	if !ctx.cr[6].lt {
	pc = 0x8251EFA8; continue 'dispatch;
	}
	// 8251EF8C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251EF90: 3929536C  addi r9, r9, 0x536c
	ctx.r[9].s64 = ctx.r[9].s64 + 21356;
	// 8251EF94: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251EF98: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251EF9C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251EFA0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251EFA4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251EFA8; continue 'dispatch;
            }
            0x8251EFA8 => {
    //   block [0x8251EFA8..0x8251F26C)
	// 8251EFA8: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251EFAC: C0170004  lfs f0, 4(r23)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8251EFB0: D00100EC  stfs f0, 0xec(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(236 as u32), tmp.u32 ) };
	// 8251EFB4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8251EFB8: D001010C  stfs f0, 0x10c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(268 as u32), tmp.u32 ) };
	// 8251EFBC: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 8251EFC0: D001012C  stfs f0, 0x12c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(300 as u32), tmp.u32 ) };
	// 8251EFC4: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 8251EFC8: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 8251EFCC: 83990000  lwz r28, 0(r25)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251EFD0: EB0B0000  ld r24, 0(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8251EFD4: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 8251EFD8: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8251EFDC: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 8251EFE0: EA660000  ld r19, 0(r6)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8251EFE4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 8251EFE8: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 8251EFEC: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 8251EFF0: EA450000  ld r18, 0(r5)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 8251EFF4: 3BC100C0  addi r30, r1, 0xc0
	ctx.r[30].s64 = ctx.r[1].s64 + 192;
	// 8251EFF8: FB0A0000  std r24, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
	// 8251EFFC: 3BFC0030  addi r31, r28, 0x30
	ctx.r[31].s64 = ctx.r[28].s64 + 48;
	// 8251F000: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8251F004: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8251F008: FA690000  std r19, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 8251F00C: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 8251F010: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 8251F014: FA480000  std r18, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251F270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251F270 size=104
    let mut pc: u32 = 0x8251F270;
    'dispatch: loop {
        match pc {
            0x8251F270 => {
    //   block [0x8251F270..0x8251F2D8)
	// 8251F270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251F274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251F278: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251F27C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251F280: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F284: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251F288: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8251F28C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8251F290: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8251F294: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251F298: 4BF44DA1  bl 0x82464038
	ctx.lr = 0x8251F29C;
	sub_82464038(ctx, base);
	// 8251F29C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251F2A0: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8251F2A4: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251F2A8: 396B5394  addi r11, r11, 0x5394
	ctx.r[11].s64 = ctx.r[11].s64 + 21396;
	// 8251F2AC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8251F2B0: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8251F2B4: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 8251F2B8: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251F2BC: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8251F2C0: B103000C  sth r8, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 8251F2C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251F2C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251F2CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251F2D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251F2D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251F2D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251F2D8 size=104
    let mut pc: u32 = 0x8251F2D8;
    'dispatch: loop {
        match pc {
            0x8251F2D8 => {
    //   block [0x8251F2D8..0x8251F314)
	// 8251F2D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251F2DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251F2E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251F2E4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251F2E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251F2EC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 8251F2F0: A17F000C  lhz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251F2F4: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 8251F2F8: 419A001C  beq cr6, 0x8251f314
	if ctx.cr[6].eq {
	pc = 0x8251F314; continue 'dispatch;
	}
	// 8251F2FC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251F300: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8251F304: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F308: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8251F30C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251F310: 4E800421  bctrl
	ctx.lr = 0x8251F314;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8251F314 => {
    //   block [0x8251F314..0x8251F340)
	// 8251F314: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F318: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8251F31C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251F320: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F324: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8251F328: 4E800421  bctrl
	ctx.lr = 0x8251F32C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8251F32C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251F330: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251F334: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251F338: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251F33C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251F340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251F340 size=104
    let mut pc: u32 = 0x8251F340;
    'dispatch: loop {
        match pc {
            0x8251F340 => {
    //   block [0x8251F340..0x8251F3A8)
	// 8251F340: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251F344: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251F348: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251F34C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251F350: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F354: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251F358: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8251F35C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8251F360: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8251F364: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8251F368: 4BF44CD1  bl 0x82464038
	ctx.lr = 0x8251F36C;
	sub_82464038(ctx, base);
	// 8251F36C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251F370: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8251F374: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8251F378: 396B53D0  addi r11, r11, 0x53d0
	ctx.r[11].s64 = ctx.r[11].s64 + 21456;
	// 8251F37C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 8251F380: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8251F384: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 8251F388: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8251F38C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8251F390: B103000C  sth r8, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 8251F394: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 8251F398: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251F39C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251F3A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251F3A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251F3A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251F3A8 size=360
    let mut pc: u32 = 0x8251F3A8;
    'dispatch: loop {
        match pc {
            0x8251F3A8 => {
    //   block [0x8251F3A8..0x8251F3FC)
	// 8251F3A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251F3AC: 48015D01  bl 0x825350ac
	ctx.lr = 0x8251F3B0;
	sub_82535080(ctx, base);
	// 8251F3B0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251F3B4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F3B8: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 8251F3BC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 8251F3C0: 7F4B5214  add r26, r11, r10
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8251F3C4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251F3C8: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8251F3CC: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F3D0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251F3D4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251F3D8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251F3DC: 40980020  bge cr6, 0x8251f3fc
	if !ctx.cr[6].lt {
	pc = 0x8251F3FC; continue 'dispatch;
	}
	// 8251F3E0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251F3E4: 39295408  addi r9, r9, 0x5408
	ctx.r[9].s64 = ctx.r[9].s64 + 21512;
	// 8251F3E8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251F3EC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251F3F0: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251F3F4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251F3F8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251F3FC; continue 'dispatch;
            }
            0x8251F3FC => {
    //   block [0x8251F3FC..0x8251F510)
	// 8251F3FC: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251F400: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8251F404: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F408: 3BCB0030  addi r30, r11, 0x30
	ctx.r[30].s64 = ctx.r[11].s64 + 48;
	// 8251F40C: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F410: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251F414: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8251F418: 4808C739  bl 0x825abb50
	ctx.lr = 0x8251F41C;
	sub_825ABB50(ctx, base);
	// 8251F41C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251F420: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251F510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x8251F510 size=20
    let mut pc: u32 = 0x8251F510;
    'dispatch: loop {
        match pc {
            0x8251F510 => {
    //   block [0x8251F510..0x8251F524)
	// 8251F510: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251F514: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8251F518: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8251F51C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 8251F520: 4BFFFE88  b 0x8251f3a8
	sub_8251F3A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251F528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251F528 size=888
    let mut pc: u32 = 0x8251F528;
    'dispatch: loop {
        match pc {
            0x8251F528 => {
    //   block [0x8251F528..0x8251F584)
	// 8251F528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251F52C: 48015B79  bl 0x825350a4
	ctx.lr = 0x8251F530;
	sub_82535080(ctx, base);
	// 8251F530: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251F534: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F538: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 8251F53C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 8251F540: 7EEB5214  add r23, r11, r10
	ctx.r[23].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8251F544: 7C9B2378  mr r27, r4
	ctx.r[27].u64 = ctx.r[4].u64;
	// 8251F548: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8251F54C: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 8251F550: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 8251F554: 81770000  lwz r11, 0(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F558: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251F55C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251F560: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251F564: 40980020  bge cr6, 0x8251f584
	if !ctx.cr[6].lt {
	pc = 0x8251F584; continue 'dispatch;
	}
	// 8251F568: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251F56C: 39295408  addi r9, r9, 0x5408
	ctx.r[9].s64 = ctx.r[9].s64 + 21512;
	// 8251F570: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251F574: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251F578: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251F57C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251F580: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251F584; continue 'dispatch;
            }
            0x8251F584 => {
    //   block [0x8251F584..0x8251F8A0)
	// 8251F584: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251F588: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251F58C: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F590: 3BDD0008  addi r30, r29, 8
	ctx.r[30].s64 = ctx.r[29].s64 + 8;
	// 8251F594: 3B8B0030  addi r28, r11, 0x30
	ctx.r[28].s64 = ctx.r[11].s64 + 48;
	// 8251F598: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251F59C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251F5A0: 4808C5B1  bl 0x825abb50
	ctx.lr = 0x8251F5A4;
	sub_825ABB50(ctx, base);
	// 8251F5A4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251F5A8: 394000E0  li r10, 0xe0
	ctx.r[10].s64 = 224;
	// 8251F5AC: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F5B0: 396B0E30  addi r11, r11, 0xe30
	ctx.r[11].s64 = ctx.r[11].s64 + 3632;
	// 8251F5B4: 11A0038C  vspltisw v13, 0
	for i in 0..4 {
		ctx.v[13].u32[i] = 0;
	}
	// 8251F5B8: 811B0000  lwz r8, 0(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251F8A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251F8A0 size=784
    let mut pc: u32 = 0x8251F8A0;
    'dispatch: loop {
        match pc {
            0x8251F8A0 => {
    //   block [0x8251F8A0..0x8251F8F8)
	// 8251F8A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251F8A4: 48015809  bl 0x825350ac
	ctx.lr = 0x8251F8A8;
	sub_82535080(ctx, base);
	// 8251F8A8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251F8AC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F8B0: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 8251F8B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8251F8B8: 7F4B5214  add r26, r11, r10
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8251F8BC: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8251F8C0: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8251F8C4: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 8251F8C8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F8CC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251F8D0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251F8D4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251F8D8: 40980020  bge cr6, 0x8251f8f8
	if !ctx.cr[6].lt {
	pc = 0x8251F8F8; continue 'dispatch;
	}
	// 8251F8DC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251F8E0: 39295408  addi r9, r9, 0x5408
	ctx.r[9].s64 = ctx.r[9].s64 + 21512;
	// 8251F8E4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251F8E8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251F8EC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251F8F0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251F8F4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251F8F8; continue 'dispatch;
            }
            0x8251F8F8 => {
    //   block [0x8251F8F8..0x8251FBB0)
	// 8251F8F8: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251F8FC: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251F900: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251F904: 3BBF0008  addi r29, r31, 8
	ctx.r[29].s64 = ctx.r[31].s64 + 8;
	// 8251F908: 3B8B0030  addi r28, r11, 0x30
	ctx.r[28].s64 = ctx.r[11].s64 + 48;
	// 8251F90C: 93C100A0  stw r30, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[30].u32 ) };
	// 8251F910: 93E100A4  stw r31, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[31].u32 ) };
	// 8251F914: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251F918: 4808C239  bl 0x825abb50
	ctx.lr = 0x8251F91C;
	sub_825ABB50(ctx, base);
	// 8251F91C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251F920: 394000E0  li r10, 0xe0
	ctx.r[10].s64 = 224;
	// 8251F924: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251F928: 396B0E30  addi r11, r11, 0xe30
	ctx.r[11].s64 = ctx.r[11].s64 + 3632;
	// 8251F92C: 11A0038C  vspltisw v13, 0
	for i in 0..4 {
		ctx.v[13].u32[i] = 0;
	}
	// 8251F930: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251FBB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251FBB0 size=784
    let mut pc: u32 = 0x8251FBB0;
    'dispatch: loop {
        match pc {
            0x8251FBB0 => {
    //   block [0x8251FBB0..0x8251FC08)
	// 8251FBB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251FBB4: 480154F9  bl 0x825350ac
	ctx.lr = 0x8251FBB8;
	sub_82535080(ctx, base);
	// 8251FBB8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251FBBC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251FBC0: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 8251FBC4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 8251FBC8: 7F4B5214  add r26, r11, r10
	ctx.r[26].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8251FBCC: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8251FBD0: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 8251FBD4: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8251FBD8: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251FBDC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8251FBE0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8251FBE4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8251FBE8: 40980020  bge cr6, 0x8251fc08
	if !ctx.cr[6].lt {
	pc = 0x8251FC08; continue 'dispatch;
	}
	// 8251FBEC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251FBF0: 39295408  addi r9, r9, 0x5408
	ctx.r[9].s64 = ctx.r[9].s64 + 21512;
	// 8251FBF4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8251FBF8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8251FBFC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8251FC00: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8251FC04: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8251FC08; continue 'dispatch;
            }
            0x8251FC08 => {
    //   block [0x8251FC08..0x8251FEC0)
	// 8251FC08: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251FC0C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 8251FC10: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8251FC14: 3BBF0008  addi r29, r31, 8
	ctx.r[29].s64 = ctx.r[31].s64 + 8;
	// 8251FC18: 3B8B0030  addi r28, r11, 0x30
	ctx.r[28].s64 = ctx.r[11].s64 + 48;
	// 8251FC1C: 93C100A0  stw r30, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[30].u32 ) };
	// 8251FC20: 93E100A4  stw r31, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[31].u32 ) };
	// 8251FC24: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8251FC28: 4808BF29  bl 0x825abb50
	ctx.lr = 0x8251FC2C;
	sub_825ABB50(ctx, base);
	// 8251FC2C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251FC30: 394000E0  li r10, 0xe0
	ctx.r[10].s64 = 224;
	// 8251FC34: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8251FC38: 396B0E30  addi r11, r11, 0xe30
	ctx.r[11].s64 = ctx.r[11].s64 + 3632;
	// 8251FC3C: 11A0038C  vspltisw v13, 0
	for i in 0..4 {
		ctx.v[13].u32[i] = 0;
	}
	// 8251FC40: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251FEC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251FEC0 size=204
    let mut pc: u32 = 0x8251FEC0;
    'dispatch: loop {
        match pc {
            0x8251FEC0 => {
    //   block [0x8251FEC0..0x8251FF8C)
	// 8251FEC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251FEC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251FEC8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8251FECC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8251FED0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251FED4: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 8251FED8: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251FEDC: 396BC550  addi r11, r11, -0x3ab0
	ctx.r[11].s64 = ctx.r[11].s64 + -15024;
	// 8251FEE0: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251FEE4: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251FEE8: 3908F340  addi r8, r8, -0xcc0
	ctx.r[8].s64 = ctx.r[8].s64 + -3264;
	// 8251FEEC: 3929FFD8  addi r9, r9, -0x28
	ctx.r[9].s64 = ctx.r[9].s64 + -40;
	// 8251FEF0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 8251FEF4: 394A0078  addi r10, r10, 0x78
	ctx.r[10].s64 = ctx.r[10].s64 + 120;
	// 8251FEF8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 8251FEFC: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 8251FF00: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 8251FF04: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 8251FF08: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 8251FF0C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 8251FF10: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8251FF14: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8251FF18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8251FF1C: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 8251FF20: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 8251FF24: 4BFDFE65  bl 0x824ffd88
	ctx.lr = 0x8251FF28;
	sub_824FFD88(ctx, base);
	// 8251FF28: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8251FF2C: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 8251FF30: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8251FF34: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 8251FF38: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 8251FF3C: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 8251FF40: 3908F270  addi r8, r8, -0xd90
	ctx.r[8].s64 = ctx.r[8].s64 + -3472;
	// 8251FF44: 3929F3A8  addi r9, r9, -0xc58
	ctx.r[9].s64 = ctx.r[9].s64 + -3160;
	// 8251FF48: 394AFBB0  addi r10, r10, -0x450
	ctx.r[10].s64 = ctx.r[10].s64 + -1104;
	// 8251FF4C: 396BBEF8  addi r11, r11, -0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + -16648;
	// 8251FF50: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 8251FF54: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 8251FF58: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 8251FF5C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8251FF60: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 8251FF64: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8251FF68: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 8251FF6C: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 8251FF70: 4BFDFE19  bl 0x824ffd88
	ctx.lr = 0x8251FF74;
	sub_824FFD88(ctx, base);
	// 8251FF74: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 8251FF78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251FF7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251FF80: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8251FF84: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8251FF88: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251FF90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251FF90 size=68
    let mut pc: u32 = 0x8251FF90;
    'dispatch: loop {
        match pc {
            0x8251FF90 => {
    //   block [0x8251FF90..0x8251FFD4)
	// 8251FF90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251FF94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251FF98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251FF9C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8251FFA0: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 8251FFA4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 8251FFA8: 396B4BAC  addi r11, r11, 0x4bac
	ctx.r[11].s64 = ctx.r[11].s64 + 19372;
	// 8251FFAC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 8251FFB0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8251FFB4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8251FFB8: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 8251FFBC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8251FFC0: 4BFFF3E9  bl 0x8251f3a8
	ctx.lr = 0x8251FFC4;
	sub_8251F3A8(ctx, base);
	// 8251FFC4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8251FFC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8251FFCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8251FFD0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_8251FFD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x8251FFD8 size=76
    let mut pc: u32 = 0x8251FFD8;
    'dispatch: loop {
        match pc {
            0x8251FFD8 => {
    //   block [0x8251FFD8..0x82520024)
	// 8251FFD8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8251FFDC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 8251FFE0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8251FFE4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 8251FFE8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8251FFEC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8251FFF0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8251FFF4: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 8251FFF8: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 8251FFFC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82520000: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82520004: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82520008: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 8252000C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82520010: 4BFFF399  bl 0x8251f3a8
	ctx.lr = 0x82520014;
	sub_8251F3A8(ctx, base);
	// 82520014: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82520018: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252001C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82520020: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82520028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82520028 size=80
    let mut pc: u32 = 0x82520028;
    'dispatch: loop {
        match pc {
            0x82520028 => {
    //   block [0x82520028..0x82520078)
	// 82520028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252002C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82520030: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82520034: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82520038: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8252003C: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82520040: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 82520044: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82520048: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8252004C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82520050: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82520054: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82520058: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8252005C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82520060: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82520064: 4BFFF83D  bl 0x8251f8a0
	ctx.lr = 0x82520068;
	sub_8251F8A0(ctx, base);
	// 82520068: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252006C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82520070: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82520074: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82520078(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82520078 size=80
    let mut pc: u32 = 0x82520078;
    'dispatch: loop {
        match pc {
            0x82520078 => {
    //   block [0x82520078..0x825200C8)
	// 82520078: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252007C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82520080: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82520084: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82520088: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8252008C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82520090: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 82520094: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82520098: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8252009C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 825200A0: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825200A4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825200A8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 825200AC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 825200B0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825200B4: 4BFFFAFD  bl 0x8251fbb0
	ctx.lr = 0x825200B8;
	sub_8251FBB0(ctx, base);
	// 825200B8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825200BC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825200C0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825200C4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825200C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825200C8 size=176
    let mut pc: u32 = 0x825200C8;
    'dispatch: loop {
        match pc {
            0x825200C8 => {
    //   block [0x825200C8..0x82520148)
	// 825200C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825200CC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825200D0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825200D4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825200D8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 825200DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825200E0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 825200E4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825200E8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825200EC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 825200F0: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825200F4: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825200F8: 4BFFF431  bl 0x8251f528
	ctx.lr = 0x825200FC;
	sub_8251F528(ctx, base);
	// 825200FC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520100: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82520104: 40980044  bge cr6, 0x82520148
	if !ctx.cr[6].lt {
	pc = 0x82520148; continue 'dispatch;
	}
	// 82520108: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252010C: 394B08F0  addi r10, r11, 0x8f0
	ctx.r[10].s64 = ctx.r[11].s64 + 2288;
	// 82520110: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82520148; continue 'dispatch;
            }
            0x82520148 => {
    //   block [0x82520148..0x82520178)
	// 82520148: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252014C: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82520150: 419A000C  beq cr6, 0x8252015c
	if ctx.cr[6].eq {
	pc = 0x8252015C; continue 'dispatch;
	}
	// 82520154: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82520158: 4BFF32E9  bl 0x82513440
	ctx.lr = 0x8252015C;
	sub_82513440(ctx, base);
	// 8252015C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82520160: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82520164: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82520168: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8252016C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82520170: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82520174: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82520178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82520178 size=116
    let mut pc: u32 = 0x82520178;
    'dispatch: loop {
        match pc {
            0x82520178 => {
    //   block [0x82520178..0x825201EC)
	// 82520178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252017C: 48014F41  bl 0x825350bc
	ctx.lr = 0x82520180;
	sub_82535080(ctx, base);
	// 82520180: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82520184: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520188: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252018C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82520190: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82520194: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82520198: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8252019C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825201A0: 4BF43E99  bl 0x82464038
	ctx.lr = 0x825201A4;
	sub_82464038(ctx, base);
	// 825201A4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825201A8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825201AC: 396B5434  addi r11, r11, 0x5434
	ctx.r[11].s64 = ctx.r[11].s64 + 21556;
	// 825201B0: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 825201B4: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 825201B8: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 825201BC: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 825201C0: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 825201C4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825201C8: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 825201CC: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 825201D0: B11F000C  sth r8, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 825201D4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825201D8: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 825201DC: 480A89DD  bl 0x825c8bb8
	ctx.lr = 0x825201E0;
	sub_825C8BB8(ctx, base);
	// 825201E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825201E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825201E8: 48014F24  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825201F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825201F0 size=104
    let mut pc: u32 = 0x825201F0;
    'dispatch: loop {
        match pc {
            0x825201F0 => {
    //   block [0x825201F0..0x8252022C)
	// 825201F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825201F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825201F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825201FC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82520200: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82520204: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82520208: A17F000C  lhz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252020C: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 82520210: 419A001C  beq cr6, 0x8252022c
	if ctx.cr[6].eq {
	pc = 0x8252022C; continue 'dispatch;
	}
	// 82520214: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82520218: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8252021C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520220: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82520224: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82520228: 4E800421  bctrl
	ctx.lr = 0x8252022C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252022C => {
    //   block [0x8252022C..0x82520258)
	// 8252022C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520230: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82520234: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82520238: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252023C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82520240: 4E800421  bctrl
	ctx.lr = 0x82520244;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82520244: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82520248: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252024C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82520250: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82520254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82520258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82520258 size=128
    let mut pc: u32 = 0x82520258;
    'dispatch: loop {
        match pc {
            0x82520258 => {
    //   block [0x82520258..0x825202D8)
	// 82520258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252025C: 48014E61  bl 0x825350bc
	ctx.lr = 0x82520260;
	sub_82535080(ctx, base);
	// 82520260: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82520264: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520268: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252026C: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82520270: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82520274: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82520278: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 8252027C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82520280: 4BF43DB9  bl 0x82464038
	ctx.lr = 0x82520284;
	sub_82464038(ctx, base);
	// 82520284: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82520288: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252028C: 396B5434  addi r11, r11, 0x5434
	ctx.r[11].s64 = ctx.r[11].s64 + 21556;
	// 82520290: 39400020  li r10, 0x20
	ctx.r[10].s64 = 32;
	// 82520294: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82520298: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8252029C: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 825202A0: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 825202A4: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825202A8: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 825202AC: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 825202B0: B11F000C  sth r8, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 825202B4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825202B8: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 825202BC: 480A88FD  bl 0x825c8bb8
	ctx.lr = 0x825202C0;
	sub_825C8BB8(ctx, base);
	// 825202C0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825202C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825202C8: 396B5470  addi r11, r11, 0x5470
	ctx.r[11].s64 = ctx.r[11].s64 + 21616;
	// 825202CC: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825202D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825202D4: 48014E38  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825202D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825202D8 size=304
    let mut pc: u32 = 0x825202D8;
    'dispatch: loop {
        match pc {
            0x825202D8 => {
    //   block [0x825202D8..0x82520328)
	// 825202D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825202DC: 48014DD1  bl 0x825350ac
	ctx.lr = 0x825202E0;
	sub_82535080(ctx, base);
	// 825202E0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825202E4: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825202E8: 3B600014  li r27, 0x14
	ctx.r[27].s64 = 20;
	// 825202EC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825202F0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825202F4: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 825202F8: 7D7BD02E  lwzx r11, r27, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 825202FC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82520300: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82520304: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82520308: 40980020  bge cr6, 0x82520328
	if !ctx.cr[6].lt {
	pc = 0x82520328; continue 'dispatch;
	}
	// 8252030C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82520310: 392954A8  addi r9, r9, 0x54a8
	ctx.r[9].s64 = ctx.r[9].s64 + 21672;
	// 82520314: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82520318: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252031C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82520320: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82520324: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82520328; continue 'dispatch;
            }
            0x82520328 => {
    //   block [0x82520328..0x82520408)
	// 82520328: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252032C: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82520330: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82520334: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
	// 82520338: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 8252033C: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520340: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82520344: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520348: 38A30010  addi r5, r3, 0x10
	ctx.r[5].s64 = ctx.r[3].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82520408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82520408 size=320
    let mut pc: u32 = 0x82520408;
    'dispatch: loop {
        match pc {
            0x82520408 => {
    //   block [0x82520408..0x82520458)
	// 82520408: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252040C: 48014C9D  bl 0x825350a8
	ctx.lr = 0x82520410;
	sub_82535080(ctx, base);
	// 82520410: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82520414: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520418: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 8252041C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82520420: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82520424: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82520428: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 8252042C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82520430: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82520434: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82520438: 40980020  bge cr6, 0x82520458
	if !ctx.cr[6].lt {
	pc = 0x82520458; continue 'dispatch;
	}
	// 8252043C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82520440: 392954A8  addi r9, r9, 0x54a8
	ctx.r[9].s64 = ctx.r[9].s64 + 21672;
	// 82520444: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82520448: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252044C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82520450: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82520454: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82520458; continue 'dispatch;
            }
            0x82520458 => {
    //   block [0x82520458..0x82520548)
	// 82520458: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252045C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82520460: 837D0000  lwz r27, 0(r29)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520464: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82520468: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 8252046C: 480A874D  bl 0x825c8bb8
	ctx.lr = 0x82520470;
	sub_825C8BB8(ctx, base);
	// 82520470: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82520474: 811D0008  lwz r8, 8(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82520478: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 8252047C: 3D40820A  lis r10, -0x7df6
	ctx.r[10].s64 = -2113273856;
	// 82520480: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82520484: 394A9F50  addi r10, r10, -0x60b0
	ctx.r[10].s64 = ctx.r[10].s64 + -24752;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82520548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82520548 size=376
    let mut pc: u32 = 0x82520548;
    'dispatch: loop {
        match pc {
            0x82520548 => {
    //   block [0x82520548..0x825206C0)
	// 82520548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252054C: 48014B4D  bl 0x82535098
	ctx.lr = 0x82520550;
	sub_82535080(ctx, base);
	// 82520550: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82520554: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82520558: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 8252055C: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82520560: 80C30008  lwz r6, 8(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82520564: 83E40000  lwz r31, 0(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520568: 388B0020  addi r4, r11, 0x20
	ctx.r[4].s64 = ctx.r[11].s64 + 32;
	// 8252056C: 83030000  lwz r24, 0(r3)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520570: 3B660030  addi r27, r6, 0x30
	ctx.r[27].s64 = ctx.r[6].s64 + 48;
	// 82520574: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82520578: EAEB0000  ld r23, 0(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8252057C: 386B0030  addi r3, r11, 0x30
	ctx.r[3].s64 = ctx.r[11].s64 + 48;
	// 82520580: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82520584: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82520588: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 8252058C: EAA40000  ld r21, 0(r4)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 82520590: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82520594: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 82520598: EAC60000  ld r22, 0(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8252059C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 825205A0: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 825205A4: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 825205A8: FAEA0000  std r23, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 825205AC: 3BC00002  li r30, 2
	ctx.r[30].s64 = 2;
	// 825205B0: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 825205B4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 825205B8: FAA80000  std r21, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 825205BC: 3B8100D0  addi r28, r1, 0xd0
	ctx.r[28].s64 = ctx.r[1].s64 + 208;
	// 825205C0: FAC90000  std r22, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 825205C4: 3BBF0040  addi r29, r31, 0x40
	ctx.r[29].s64 = ctx.r[31].s64 + 64;
	// 825205C8: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 825205CC: EA830000  ld r20, 0(r3)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825206C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825206C0 size=492
    let mut pc: u32 = 0x825206C0;
    'dispatch: loop {
        match pc {
            0x825206C0 => {
    //   block [0x825206C0..0x8252070C)
	// 825206C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825206C4: 480149CD  bl 0x82535090
	ctx.lr = 0x825206C8;
	sub_82535080(ctx, base);
	// 825206C8: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825206CC: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825206D0: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 825206D4: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 825206D8: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 825206DC: 7D79C02E  lwzx r11, r25, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 825206E0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825206E4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825206E8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825206EC: 40980020  bge cr6, 0x8252070c
	if !ctx.cr[6].lt {
	pc = 0x8252070C; continue 'dispatch;
	}
	// 825206F0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825206F4: 392954A8  addi r9, r9, 0x54a8
	ctx.r[9].s64 = ctx.r[9].s64 + 21672;
	// 825206F8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825206FC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82520700: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82520704: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82520708: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252070C; continue 'dispatch;
            }
            0x8252070C => {
    //   block [0x8252070C..0x825208AC)
	// 8252070C: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82520710: 394100C0  addi r10, r1, 0xc0
	ctx.r[10].s64 = ctx.r[1].s64 + 192;
	// 82520714: 80C40008  lwz r6, 8(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82520718: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 8252071C: 908100B0  stw r4, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[4].u32 ) };
	// 82520720: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82520724: 90A100B4  stw r5, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[5].u32 ) };
	// 82520728: 3B660030  addi r27, r6, 0x30
	ctx.r[27].s64 = ctx.r[6].s64 + 48;
	// 8252072C: 83850000  lwz r28, 0(r5)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520730: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82520734: 83440000  lwz r26, 0(r4)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520738: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 8252073C: EAAB0000  ld r21, 0(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82520740: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82520744: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82520748: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 8252074C: 3BE00002  li r31, 2
	ctx.r[31].s64 = 2;
	// 82520750: EA860000  ld r20, 0(r6)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82520754: 3BA10110  addi r29, r1, 0x110
	ctx.r[29].s64 = ctx.r[1].s64 + 272;
	// 82520758: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 8252075C: 3BDC0040  addi r30, r28, 0x40
	ctx.r[30].s64 = ctx.r[28].s64 + 64;
	// 82520760: FAAA0000  std r21, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82520764: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82520768: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 8252076C: EA650000  ld r19, 0(r5)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82520770: FA890000  std r20, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82520774: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 82520778: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825208B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825208B0 size=516
    let mut pc: u32 = 0x825208B0;
    'dispatch: loop {
        match pc {
            0x825208B0 => {
    //   block [0x825208B0..0x82520904)
	// 825208B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825208B4: 480147E1  bl 0x82535094
	ctx.lr = 0x825208B8;
	sub_82535080(ctx, base);
	// 825208B8: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825208BC: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825208C0: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 825208C4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825208C8: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825208CC: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 825208D0: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 825208D4: 7D7AC82E  lwzx r11, r26, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 825208D8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825208DC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825208E0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825208E4: 40980020  bge cr6, 0x82520904
	if !ctx.cr[6].lt {
	pc = 0x82520904; continue 'dispatch;
	}
	// 825208E8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825208EC: 392954A8  addi r9, r9, 0x54a8
	ctx.r[9].s64 = ctx.r[9].s64 + 21672;
	// 825208F0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825208F4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825208F8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 825208FC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82520900: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82520904; continue 'dispatch;
            }
            0x82520904 => {
    //   block [0x82520904..0x82520AB4)
	// 82520904: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520908: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 8252090C: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82520910: 480A82A9  bl 0x825c8bb8
	ctx.lr = 0x82520914;
	sub_825C8BB8(ctx, base);
	// 82520914: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82520918: 80DE0008  lwz r6, 8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252091C: 394100D0  addi r10, r1, 0xd0
	ctx.r[10].s64 = ctx.r[1].s64 + 208;
	// 82520920: 93C100C0  stw r30, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 82520924: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82520928: 93E100C4  stw r31, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[31].u32 ) };
	// 8252092C: 3B860030  addi r28, r6, 0x30
	ctx.r[28].s64 = ctx.r[6].s64 + 48;
	// 82520930: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82520934: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520938: EACB0000  ld r22, 0(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8252093C: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82520940: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82520944: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82520948: EA850000  ld r20, 0(r5)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 8252094C: 39010090  addi r8, r1, 0x90
	ctx.r[8].s64 = ctx.r[1].s64 + 144;
	// 82520950: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82520954: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82520958: EAA60000  ld r21, 0(r6)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 8252095C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82520960: FACA0000  std r22, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82520964: 3BFD0040  addi r31, r29, 0x40
	ctx.r[31].s64 = ctx.r[29].s64 + 64;
	// 82520968: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8252096C: 396100D0  addi r11, r1, 0xd0
	ctx.r[11].s64 = ctx.r[1].s64 + 208;
	// 82520970: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82520974: FA880000  std r20, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82520978: FAA90000  std r21, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 8252097C: EA640000  ld r19, 0(r4)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82520AB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82520AB8 size=64
    let mut pc: u32 = 0x82520AB8;
    'dispatch: loop {
        match pc {
            0x82520AB8 => {
    //   block [0x82520AB8..0x82520AF8)
	// 82520AB8: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 82520ABC: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82520AC0: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82520AC4: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82520AC8: 38EBBEF8  addi r7, r11, -0x4108
	ctx.r[7].s64 = ctx.r[11].s64 + -16648;
	// 82520ACC: 39080178  addi r8, r8, 0x178
	ctx.r[8].s64 = ctx.r[8].s64 + 376;
	// 82520AD0: 39290408  addi r9, r9, 0x408
	ctx.r[9].s64 = ctx.r[9].s64 + 1032;
	// 82520AD4: 394A08B0  addi r10, r10, 0x8b0
	ctx.r[10].s64 = ctx.r[10].s64 + 2224;
	// 82520AD8: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82520ADC: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82520AE0: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82520AE4: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82520AE8: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82520AEC: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82520AF0: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82520AF4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82520AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82520AF8 size=68
    let mut pc: u32 = 0x82520AF8;
    'dispatch: loop {
        match pc {
            0x82520AF8 => {
    //   block [0x82520AF8..0x82520B3C)
	// 82520AF8: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82520AFC: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82520B00: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82520B04: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 82520B08: 39080258  addi r8, r8, 0x258
	ctx.r[8].s64 = ctx.r[8].s64 + 600;
	// 82520B0C: 39290D30  addi r9, r9, 0xd30
	ctx.r[9].s64 = ctx.r[9].s64 + 3376;
	// 82520B10: 394A1040  addi r10, r10, 0x1040
	ctx.r[10].s64 = ctx.r[10].s64 + 4160;
	// 82520B14: 396BC550  addi r11, r11, -0x3ab0
	ctx.r[11].s64 = ctx.r[11].s64 + -15024;
	// 82520B18: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82520B1C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82520B20: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82520B24: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82520B28: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82520B2C: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82520B30: 98E30010  stb r7, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[7].u8 ) };
	// 82520B34: 98C30011  stb r6, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[6].u8 ) };
	// 82520B38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82520B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82520B40 size=204
    let mut pc: u32 = 0x82520B40;
    'dispatch: loop {
        match pc {
            0x82520B40 => {
    //   block [0x82520B40..0x82520C0C)
	// 82520B40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82520B44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82520B48: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82520B4C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82520B50: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82520B54: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 82520B58: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82520B5C: 396BC550  addi r11, r11, -0x3ab0
	ctx.r[11].s64 = ctx.r[11].s64 + -15024;
	// 82520B60: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82520B64: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82520B68: 39080258  addi r8, r8, 0x258
	ctx.r[8].s64 = ctx.r[8].s64 + 600;
	// 82520B6C: 39290D30  addi r9, r9, 0xd30
	ctx.r[9].s64 = ctx.r[9].s64 + 3376;
	// 82520B70: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82520B74: 394A1040  addi r10, r10, 0x1040
	ctx.r[10].s64 = ctx.r[10].s64 + 4160;
	// 82520B78: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82520B7C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82520B80: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82520B84: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82520B88: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82520B8C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82520B90: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82520B94: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82520B98: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82520B9C: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82520BA0: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82520BA4: 4BFDF1E5  bl 0x824ffd88
	ctx.lr = 0x82520BA8;
	sub_824FFD88(ctx, base);
	// 82520BA8: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82520BAC: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82520BB0: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82520BB4: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82520BB8: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82520BBC: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 82520BC0: 39080178  addi r8, r8, 0x178
	ctx.r[8].s64 = ctx.r[8].s64 + 376;
	// 82520BC4: 39290408  addi r9, r9, 0x408
	ctx.r[9].s64 = ctx.r[9].s64 + 1032;
	// 82520BC8: 394A08B0  addi r10, r10, 0x8b0
	ctx.r[10].s64 = ctx.r[10].s64 + 2224;
	// 82520BCC: 396BBEF8  addi r11, r11, -0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + -16648;
	// 82520BD0: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82520BD4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82520BD8: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82520BDC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82520BE0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82520BE4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82520BE8: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82520BEC: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82520BF0: 4BFDF199  bl 0x824ffd88
	ctx.lr = 0x82520BF4;
	sub_824FFD88(ctx, base);
	// 82520BF4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82520BF8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82520BFC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82520C00: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82520C04: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82520C08: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82520C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82520C10 size=204
    let mut pc: u32 = 0x82520C10;
    'dispatch: loop {
        match pc {
            0x82520C10 => {
    //   block [0x82520C10..0x82520CDC)
	// 82520C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82520C14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82520C18: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82520C1C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82520C20: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82520C24: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 82520C28: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82520C2C: 396BC550  addi r11, r11, -0x3ab0
	ctx.r[11].s64 = ctx.r[11].s64 + -15024;
	// 82520C30: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82520C34: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82520C38: 39080258  addi r8, r8, 0x258
	ctx.r[8].s64 = ctx.r[8].s64 + 600;
	// 82520C3C: 39290D30  addi r9, r9, 0xd30
	ctx.r[9].s64 = ctx.r[9].s64 + 3376;
	// 82520C40: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82520C44: 394A1040  addi r10, r10, 0x1040
	ctx.r[10].s64 = ctx.r[10].s64 + 4160;
	// 82520C48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82520C4C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82520C50: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82520C54: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82520C58: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82520C5C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82520C60: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82520C64: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82520C68: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82520C6C: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82520C70: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82520C74: 4BFDF22D  bl 0x824ffea0
	ctx.lr = 0x82520C78;
	sub_824FFEA0(ctx, base);
	// 82520C78: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82520C7C: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82520C80: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82520C84: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82520C88: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82520C8C: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 82520C90: 39080178  addi r8, r8, 0x178
	ctx.r[8].s64 = ctx.r[8].s64 + 376;
	// 82520C94: 39290408  addi r9, r9, 0x408
	ctx.r[9].s64 = ctx.r[9].s64 + 1032;
	// 82520C98: 394A08B0  addi r10, r10, 0x8b0
	ctx.r[10].s64 = ctx.r[10].s64 + 2224;
	// 82520C9C: 396BBEF8  addi r11, r11, -0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + -16648;
	// 82520CA0: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82520CA4: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82520CA8: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82520CAC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82520CB0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82520CB4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82520CB8: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82520CBC: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82520CC0: 4BFDF1E1  bl 0x824ffea0
	ctx.lr = 0x82520CC4;
	sub_824FFEA0(ctx, base);
	// 82520CC4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82520CC8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82520CCC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82520CD0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82520CD4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82520CD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82520CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82520CE0 size=76
    let mut pc: u32 = 0x82520CE0;
    'dispatch: loop {
        match pc {
            0x82520CE0 => {
    //   block [0x82520CE0..0x82520D2C)
	// 82520CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82520CE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82520CE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82520CEC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82520CF0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82520CF4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82520CF8: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82520CFC: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82520D00: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 82520D04: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82520D08: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82520D0C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82520D10: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82520D14: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82520D18: 4BFFF5C1  bl 0x825202d8
	ctx.lr = 0x82520D1C;
	sub_825202D8(ctx, base);
	// 82520D1C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82520D20: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82520D24: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82520D28: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82520D30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82520D30 size=76
    let mut pc: u32 = 0x82520D30;
    'dispatch: loop {
        match pc {
            0x82520D30 => {
    //   block [0x82520D30..0x82520D7C)
	// 82520D30: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82520D34: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82520D38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82520D3C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82520D40: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82520D44: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82520D48: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82520D4C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82520D50: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 82520D54: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82520D58: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82520D5C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82520D60: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82520D64: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82520D68: 4BFFF6A1  bl 0x82520408
	ctx.lr = 0x82520D6C;
	sub_82520408(ctx, base);
	// 82520D6C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82520D70: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82520D74: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82520D78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82520D80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82520D80 size=620
    let mut pc: u32 = 0x82520D80;
    'dispatch: loop {
        match pc {
            0x82520D80 => {
    //   block [0x82520D80..0x82520DD8)
	// 82520D80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82520D84: 48014301  bl 0x82535084
	ctx.lr = 0x82520D88;
	sub_82535080(ctx, base);
	// 82520D88: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82520D8C: 826D0000  lwz r19, 0(r13)
	ctx.r[19].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520D90: 3A800014  li r20, 0x14
	ctx.r[20].s64 = 20;
	// 82520D94: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82520D98: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82520D9C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82520DA0: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82520DA4: 7D74982E  lwzx r11, r20, r19
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[20].u32.wrapping_add(ctx.r[19].u32)) } as u64;
	// 82520DA8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82520DAC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82520DB0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82520DB4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82520DB8: 40980020  bge cr6, 0x82520dd8
	if !ctx.cr[6].lt {
	pc = 0x82520DD8; continue 'dispatch;
	}
	// 82520DBC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82520DC0: 392954A8  addi r9, r9, 0x54a8
	ctx.r[9].s64 = ctx.r[9].s64 + 21672;
	// 82520DC4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82520DC8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82520DCC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82520DD0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82520DD4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82520DD8; continue 'dispatch;
            }
            0x82520DD8 => {
    //   block [0x82520DD8..0x82520FEC)
	// 82520DD8: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82520DDC: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82520DE0: 80DC0008  lwz r6, 8(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82520DE4: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 82520DE8: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82520DEC: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520DF0: 3B060030  addi r24, r6, 0x30
	ctx.r[24].s64 = ctx.r[6].s64 + 48;
	// 82520DF4: 83590000  lwz r26, 0(r25)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520DF8: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82520DFC: 82FC0000  lwz r23, 0(r28)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82520E00: EA4B0000  ld r18, 0(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82520E04: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82520E08: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82520E0C: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82520E10: EA050000  ld r16, 0(r5)
	ctx.r[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82520E14: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 82520E18: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82520E1C: 38600002  li r3, 2
	ctx.r[3].s64 = 2;
	// 82520E20: EA260000  ld r17, 0(r6)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82520E24: 3BC100E0  addi r30, r1, 0xe0
	ctx.r[30].s64 = ctx.r[1].s64 + 224;
	// 82520E28: FA4A0000  std r18, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
	// 82520E2C: 3BFD0040  addi r31, r29, 0x40
	ctx.r[31].s64 = ctx.r[29].s64 + 64;
	// 82520E30: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82520E34: 396100B0  addi r11, r1, 0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + 176;
	// 82520E38: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82520E3C: FA080000  std r16, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[16].u64 ) };
	// 82520E40: FA290000  std r17, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[17].u64 ) };
	// 82520E44: E9E40000  ld r15, 0(r4)
	ctx.r[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82520FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82520FF0 size=80
    let mut pc: u32 = 0x82520FF0;
    'dispatch: loop {
        match pc {
            0x82520FF0 => {
    //   block [0x82520FF0..0x82521040)
	// 82520FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82520FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82520FF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82520FFC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82521000: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82521004: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82521008: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 8252100C: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82521010: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82521014: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82521018: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252101C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82521020: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82521024: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82521028: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8252102C: 4BFFF695  bl 0x825206c0
	ctx.lr = 0x82521030;
	sub_825206C0(ctx, base);
	// 82521030: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82521034: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82521038: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252103C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82521040(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82521040 size=80
    let mut pc: u32 = 0x82521040;
    'dispatch: loop {
        match pc {
            0x82521040 => {
    //   block [0x82521040..0x82521090)
	// 82521040: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82521044: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82521048: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252104C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82521050: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82521054: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82521058: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 8252105C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82521060: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82521064: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82521068: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252106C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82521070: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82521074: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82521078: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8252107C: 4BFFF835  bl 0x825208b0
	ctx.lr = 0x82521080;
	sub_825208B0(ctx, base);
	// 82521080: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82521084: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82521088: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252108C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82521090(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82521090 size=176
    let mut pc: u32 = 0x82521090;
    'dispatch: loop {
        match pc {
            0x82521090 => {
    //   block [0x82521090..0x82521110)
	// 82521090: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82521094: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82521098: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252109C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825210A0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 825210A4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825210A8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 825210AC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825210B0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825210B4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 825210B8: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825210BC: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825210C0: 4BFFFCC1  bl 0x82520d80
	ctx.lr = 0x825210C4;
	sub_82520D80(ctx, base);
	// 825210C4: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825210C8: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825210CC: 40980044  bge cr6, 0x82521110
	if !ctx.cr[6].lt {
	pc = 0x82521110; continue 'dispatch;
	}
	// 825210D0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825210D4: 394B08F0  addi r10, r11, 0x8f0
	ctx.r[10].s64 = ctx.r[11].s64 + 2288;
	// 825210D8: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82521110; continue 'dispatch;
            }
            0x82521110 => {
    //   block [0x82521110..0x82521140)
	// 82521110: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82521114: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82521118: 419A000C  beq cr6, 0x82521124
	if ctx.cr[6].eq {
	pc = 0x82521124; continue 'dispatch;
	}
	// 8252111C: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82521120: 4BFF2321  bl 0x82513440
	ctx.lr = 0x82521124;
	sub_82513440(ctx, base);
	// 82521124: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82521128: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252112C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82521130: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82521134: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82521138: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252113C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82521140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82521140 size=104
    let mut pc: u32 = 0x82521140;
    'dispatch: loop {
        match pc {
            0x82521140 => {
    //   block [0x82521140..0x825211A8)
	// 82521140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82521144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82521148: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252114C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82521150: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82521154: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82521158: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8252115C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82521160: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82521164: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82521168: 4BF42ED1  bl 0x82464038
	ctx.lr = 0x8252116C;
	sub_82464038(ctx, base);
	// 8252116C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82521170: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82521174: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82521178: 396B54D4  addi r11, r11, 0x54d4
	ctx.r[11].s64 = ctx.r[11].s64 + 21716;
	// 8252117C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82521180: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82521184: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82521188: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252118C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82521190: B103000C  sth r8, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 82521194: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82521198: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252119C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825211A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825211A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825211A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825211A8 size=104
    let mut pc: u32 = 0x825211A8;
    'dispatch: loop {
        match pc {
            0x825211A8 => {
    //   block [0x825211A8..0x825211E4)
	// 825211A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825211AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825211B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825211B4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825211B8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825211BC: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 825211C0: A17F000C  lhz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825211C4: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 825211C8: 419A001C  beq cr6, 0x825211e4
	if ctx.cr[6].eq {
	pc = 0x825211E4; continue 'dispatch;
	}
	// 825211CC: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825211D0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825211D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825211D8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825211DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825211E0: 4E800421  bctrl
	ctx.lr = 0x825211E4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x825211E4 => {
    //   block [0x825211E4..0x82521210)
	// 825211E4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825211E8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825211EC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825211F0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825211F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825211F8: 4E800421  bctrl
	ctx.lr = 0x825211FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825211FC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82521200: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82521204: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82521208: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252120C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82521210(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82521210 size=104
    let mut pc: u32 = 0x82521210;
    'dispatch: loop {
        match pc {
            0x82521210 => {
    //   block [0x82521210..0x82521278)
	// 82521210: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82521214: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82521218: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252121C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82521220: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82521224: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82521228: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 8252122C: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82521230: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82521234: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82521238: 4BF42E01  bl 0x82464038
	ctx.lr = 0x8252123C;
	sub_82464038(ctx, base);
	// 8252123C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82521240: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82521244: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82521248: 396B5510  addi r11, r11, 0x5510
	ctx.r[11].s64 = ctx.r[11].s64 + 21776;
	// 8252124C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82521250: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82521254: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82521258: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252125C: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82521260: B103000C  sth r8, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 82521264: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82521268: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252126C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82521270: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82521274: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82521278(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82521278 size=456
    let mut pc: u32 = 0x82521278;
    'dispatch: loop {
        match pc {
            0x82521278 => {
    //   block [0x82521278..0x825212CC)
	// 82521278: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252127C: 48013E19  bl 0x82535094
	ctx.lr = 0x82521280;
	sub_82535080(ctx, base);
	// 82521280: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82521284: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82521288: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 8252128C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82521290: 7F0B5214  add r24, r11, r10
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82521294: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82521298: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 8252129C: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 825212A0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825212A4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825212A8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825212AC: 40980020  bge cr6, 0x825212cc
	if !ctx.cr[6].lt {
	pc = 0x825212CC; continue 'dispatch;
	}
	// 825212B0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825212B4: 39295548  addi r9, r9, 0x5548
	ctx.r[9].s64 = ctx.r[9].s64 + 21832;
	// 825212B8: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825212BC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825212C0: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 825212C4: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825212C8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x825212CC; continue 'dispatch;
            }
            0x825212CC => {
    //   block [0x825212CC..0x82521440)
	// 825212CC: 817C0008  lwz r11, 8(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 825212D0: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 825212D4: 80DD0008  lwz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825212D8: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 825212DC: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 825212E0: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825212E4: 3B460030  addi r26, r6, 0x30
	ctx.r[26].s64 = ctx.r[6].s64 + 48;
	// 825212E8: 833D0000  lwz r25, 0(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825212EC: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 825212F0: EACB0000  ld r22, 0(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 825212F4: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 825212F8: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 825212FC: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82521300: EA850000  ld r20, 0(r5)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82521304: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 82521308: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 8252130C: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82521310: EAA60000  ld r21, 0(r6)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82521314: 3BC100C0  addi r30, r1, 0xc0
	ctx.r[30].s64 = ctx.r[1].s64 + 192;
	// 82521318: FACA0000  std r22, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 8252131C: 3BFB0030  addi r31, r27, 0x30
	ctx.r[31].s64 = ctx.r[27].s64 + 48;
	// 82521320: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82521324: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82521328: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 8252132C: FA880000  std r20, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82521330: FAA90000  std r21, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82521334: EA640000  ld r19, 0(r4)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82521440(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82521440 size=20
    let mut pc: u32 = 0x82521440;
    'dispatch: loop {
        match pc {
            0x82521440 => {
    //   block [0x82521440..0x82521454)
	// 82521440: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82521444: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82521448: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8252144C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82521450: 4BFFFE28  b 0x82521278
	sub_82521278(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82521458(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82521458 size=632
    let mut pc: u32 = 0x82521458;
    'dispatch: loop {
        match pc {
            0x82521458 => {
    //   block [0x82521458..0x825216D0)
	// 82521458: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252145C: 48013C41  bl 0x8253509c
	ctx.lr = 0x82521460;
	sub_82535080(ctx, base);
	// 82521460: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82521464: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82521468: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 8252146C: 80C30008  lwz r6, 8(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82521470: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82521474: 83E40000  lwz r31, 0(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82521478: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 8252147C: 3B260030  addi r25, r6, 0x30
	ctx.r[25].s64 = ctx.r[6].s64 + 48;
	// 82521480: 83430000  lwz r26, 0(r3)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82521484: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82521488: EB0B0000  ld r24, 0(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8252148C: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82521490: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82521494: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82521498: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 8252149C: EAC50000  ld r22, 0(r5)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 825214A0: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 825214A4: 39010090  addi r8, r1, 0x90
	ctx.r[8].s64 = ctx.r[1].s64 + 144;
	// 825214A8: EAE60000  ld r23, 0(r6)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 825214AC: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
	// 825214B0: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 825214B4: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 825214B8: FB0A0000  std r24, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
	// 825214BC: 3BA10080  addi r29, r1, 0x80
	ctx.r[29].s64 = ctx.r[1].s64 + 128;
	// 825214C0: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 825214C4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 825214C8: EAA40000  ld r21, 0(r4)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
	// 825214CC: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 825214D0: E8840008  ld r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) };
	// 825214D4: FAE90000  std r23, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 825214D8: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825216D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825216D0 size=768
    let mut pc: u32 = 0x825216D0;
    'dispatch: loop {
        match pc {
            0x825216D0 => {
    //   block [0x825216D0..0x82521720)
	// 825216D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825216D4: 480139C5  bl 0x82535098
	ctx.lr = 0x825216D8;
	sub_82535080(ctx, base);
	// 825216D8: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825216DC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825216E0: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 825216E4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 825216E8: 7F0B5214  add r24, r11, r10
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825216EC: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 825216F0: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 825216F4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825216F8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825216FC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82521700: 40980020  bge cr6, 0x82521720
	if !ctx.cr[6].lt {
	pc = 0x82521720; continue 'dispatch;
	}
	// 82521704: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82521708: 39295548  addi r9, r9, 0x5548
	ctx.r[9].s64 = ctx.r[9].s64 + 21832;
	// 8252170C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82521710: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82521714: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82521718: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252171C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82521720; continue 'dispatch;
            }
            0x82521720 => {
    //   block [0x82521720..0x825219D0)
	// 82521720: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82521724: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82521728: 80C40008  lwz r6, 8(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252172C: 392100E0  addi r9, r1, 0xe0
	ctx.r[9].s64 = ctx.r[1].s64 + 224;
	// 82521730: 908100A0  stw r4, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[4].u32 ) };
	// 82521734: 390100D0  addi r8, r1, 0xd0
	ctx.r[8].s64 = ctx.r[1].s64 + 208;
	// 82521738: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 8252173C: 3B660030  addi r27, r6, 0x30
	ctx.r[27].s64 = ctx.r[6].s64 + 48;
	// 82521740: 83E50000  lwz r31, 0(r5)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82521744: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82521748: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252174C: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82521750: EAEB0000  ld r23, 0(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82521754: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82521758: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 8252175C: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82521760: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82521764: EAC60000  ld r22, 0(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82521768: 3BA100C0  addi r29, r1, 0xc0
	ctx.r[29].s64 = ctx.r[1].s64 + 192;
	// 8252176C: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82521770: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82521774: FAEA0000  std r23, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82521778: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 8252177C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82521780: EAA50000  ld r21, 0(r5)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82521784: FAC90000  std r22, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82521788: F8C90008  std r6, 8(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(8 as u32), ctx.r[6].u64 ) };
	// 8252178C: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825219D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825219D0 size=768
    let mut pc: u32 = 0x825219D0;
    'dispatch: loop {
        match pc {
            0x825219D0 => {
    //   block [0x825219D0..0x82521A20)
	// 825219D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825219D4: 480136C5  bl 0x82535098
	ctx.lr = 0x825219D8;
	sub_82535080(ctx, base);
	// 825219D8: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825219DC: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825219E0: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 825219E4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 825219E8: 7F0B5214  add r24, r11, r10
	ctx.r[24].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825219EC: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 825219F0: 81780000  lwz r11, 0(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 825219F4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825219F8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825219FC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82521A00: 40980020  bge cr6, 0x82521a20
	if !ctx.cr[6].lt {
	pc = 0x82521A20; continue 'dispatch;
	}
	// 82521A04: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82521A08: 39295548  addi r9, r9, 0x5548
	ctx.r[9].s64 = ctx.r[9].s64 + 21832;
	// 82521A0C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82521A10: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82521A14: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82521A18: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82521A1C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82521A20; continue 'dispatch;
            }
            0x82521A20 => {
    //   block [0x82521A20..0x82521CD0)
	// 82521A20: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82521A24: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82521A28: 80C30008  lwz r6, 8(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82521A2C: 392100E0  addi r9, r1, 0xe0
	ctx.r[9].s64 = ctx.r[1].s64 + 224;
	// 82521A30: 906100A0  stw r3, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[3].u32 ) };
	// 82521A34: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82521A38: 908100A4  stw r4, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[4].u32 ) };
	// 82521A3C: 3B660030  addi r27, r6, 0x30
	ctx.r[27].s64 = ctx.r[6].s64 + 48;
	// 82521A40: 83E40000  lwz r31, 0(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82521A44: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82521A48: EAEB0000  ld r23, 0(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82521A4C: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82521A50: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82521A54: 390100D0  addi r8, r1, 0xd0
	ctx.r[8].s64 = ctx.r[1].s64 + 208;
	// 82521A58: EAA50000  ld r21, 0(r5)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82521A5C: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82521A60: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82521A64: 3BA100C0  addi r29, r1, 0xc0
	ctx.r[29].s64 = ctx.r[1].s64 + 192;
	// 82521A68: EAC60000  ld r22, 0(r6)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82521A6C: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82521A70: FAEA0000  std r23, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82521A74: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82521A78: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82521A7C: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82521A80: FAA80000  std r21, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82521A84: FAC90000  std r22, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
	// 82521A88: EA840000  ld r20, 0(r4)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82521CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82521CD0 size=204
    let mut pc: u32 = 0x82521CD0;
    'dispatch: loop {
        match pc {
            0x82521CD0 => {
    //   block [0x82521CD0..0x82521D9C)
	// 82521CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82521CD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82521CD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82521CDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82521CE0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82521CE4: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 82521CE8: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82521CEC: 396BC550  addi r11, r11, -0x3ab0
	ctx.r[11].s64 = ctx.r[11].s64 + -15024;
	// 82521CF0: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82521CF4: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82521CF8: 39081210  addi r8, r8, 0x1210
	ctx.r[8].s64 = ctx.r[8].s64 + 4624;
	// 82521CFC: 39291DE8  addi r9, r9, 0x1de8
	ctx.r[9].s64 = ctx.r[9].s64 + 7656;
	// 82521D00: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82521D04: 394A2258  addi r10, r10, 0x2258
	ctx.r[10].s64 = ctx.r[10].s64 + 8792;
	// 82521D08: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82521D0C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82521D10: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82521D14: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82521D18: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82521D1C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82521D20: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82521D24: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82521D28: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82521D2C: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82521D30: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82521D34: 4BFDE055  bl 0x824ffd88
	ctx.lr = 0x82521D38;
	sub_824FFD88(ctx, base);
	// 82521D38: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82521D3C: 9BC10080  stb r30, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[30].u8 ) };
	// 82521D40: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82521D44: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82521D48: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82521D4C: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 82521D50: 39081140  addi r8, r8, 0x1140
	ctx.r[8].s64 = ctx.r[8].s64 + 4416;
	// 82521D54: 39291278  addi r9, r9, 0x1278
	ctx.r[9].s64 = ctx.r[9].s64 + 4728;
	// 82521D58: 394A19D0  addi r10, r10, 0x19d0
	ctx.r[10].s64 = ctx.r[10].s64 + 6608;
	// 82521D5C: 396BBEF8  addi r11, r11, -0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + -16648;
	// 82521D60: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82521D64: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82521D68: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82521D6C: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82521D70: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82521D74: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82521D78: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82521D7C: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82521D80: 4BFDE009  bl 0x824ffd88
	ctx.lr = 0x82521D84;
	sub_824FFD88(ctx, base);
	// 82521D84: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82521D88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82521D8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82521D90: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82521D94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82521D98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82521DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82521DA0 size=68
    let mut pc: u32 = 0x82521DA0;
    'dispatch: loop {
        match pc {
            0x82521DA0 => {
    //   block [0x82521DA0..0x82521DE4)
	// 82521DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82521DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82521DA8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82521DAC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82521DB0: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82521DB4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82521DB8: 396B4BAC  addi r11, r11, 0x4bac
	ctx.r[11].s64 = ctx.r[11].s64 + 19372;
	// 82521DBC: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82521DC0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82521DC4: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82521DC8: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82521DCC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82521DD0: 4BFFF4A9  bl 0x82521278
	ctx.lr = 0x82521DD4;
	sub_82521278(ctx, base);
	// 82521DD4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82521DD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82521DDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82521DE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82521DE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82521DE8 size=76
    let mut pc: u32 = 0x82521DE8;
    'dispatch: loop {
        match pc {
            0x82521DE8 => {
    //   block [0x82521DE8..0x82521E34)
	// 82521DE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82521DEC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82521DF0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82521DF4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82521DF8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82521DFC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82521E00: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82521E04: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82521E08: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 82521E0C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82521E10: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82521E14: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82521E18: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82521E1C: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82521E20: 4BFFF459  bl 0x82521278
	ctx.lr = 0x82521E24;
	sub_82521278(ctx, base);
	// 82521E24: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82521E28: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82521E2C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82521E30: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82521E38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82521E38 size=96
    let mut pc: u32 = 0x82521E38;
    'dispatch: loop {
        match pc {
            0x82521E38 => {
    //   block [0x82521E38..0x82521E80)
	// 82521E38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82521E3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82521E40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82521E44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82521E48: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82521E4C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82521E50: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 82521E54: 548A07FE  clrlwi r10, r4, 0x1f
	ctx.r[10].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82521E58: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82521E5C: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82521E60: 419A0020  beq cr6, 0x82521e80
	if ctx.cr[6].eq {
	pc = 0x82521E80; continue 'dispatch;
	}
	// 82521E64: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82521E68: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82521E6C: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82521E70: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82521E74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82521E78: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82521E7C: 4BF4223D  bl 0x824640b8
	ctx.lr = 0x82521E80;
	sub_824640B8(ctx, base);
	pc = 0x82521E80; continue 'dispatch;
            }
            0x82521E80 => {
    //   block [0x82521E80..0x82521E98)
	// 82521E80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82521E84: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82521E88: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82521E8C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82521E90: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82521E94: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82521E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82521E98 size=876
    let mut pc: u32 = 0x82521E98;
    'dispatch: loop {
        match pc {
            0x82521E98 => {
    //   block [0x82521E98..0x82521EF4)
	// 82521E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82521E9C: 480131ED  bl 0x82535088
	ctx.lr = 0x82521EA0;
	sub_82535080(ctx, base);
	// 82521EA0: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82521EA4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82521EA8: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82521EAC: 7C761B78  mr r22, r3
	ctx.r[22].u64 = ctx.r[3].u64;
	// 82521EB0: 7E8B5214  add r20, r11, r10
	ctx.r[20].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82521EB4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82521EB8: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82521EBC: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82521EC0: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82521EC4: 81740000  lwz r11, 0(r20)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[20].u32.wrapping_add(0 as u32) ) } as u64;
	// 82521EC8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82521ECC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82521ED0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82521ED4: 40980020  bge cr6, 0x82521ef4
	if !ctx.cr[6].lt {
	pc = 0x82521EF4; continue 'dispatch;
	}
	// 82521ED8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82521EDC: 39295548  addi r9, r9, 0x5548
	ctx.r[9].s64 = ctx.r[9].s64 + 21832;
	// 82521EE0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82521EE4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82521EE8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82521EEC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82521EF0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82521EF4; continue 'dispatch;
            }
            0x82521EF4 => {
    //   block [0x82521EF4..0x82522204)
	// 82521EF4: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82521EF8: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82521EFC: 80DA0008  lwz r6, 8(r26)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82521F00: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82521F04: 38AB0020  addi r5, r11, 0x20
	ctx.r[5].s64 = ctx.r[11].s64 + 32;
	// 82521F08: 83F90000  lwz r31, 0(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82521F0C: 3B060030  addi r24, r6, 0x30
	ctx.r[24].s64 = ctx.r[6].s64 + 48;
	// 82521F10: 83970000  lwz r28, 0(r23)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82521F14: 38CB0010  addi r6, r11, 0x10
	ctx.r[6].s64 = ctx.r[11].s64 + 16;
	// 82521F18: 837A0000  lwz r27, 0(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82521F1C: EA6B0000  ld r19, 0(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82521F20: 388B0030  addi r4, r11, 0x30
	ctx.r[4].s64 = ctx.r[11].s64 + 48;
	// 82521F24: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82521F28: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82521F2C: EA250000  ld r17, 0(r5)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	// 82521F30: 38E10090  addi r7, r1, 0x90
	ctx.r[7].s64 = ctx.r[1].s64 + 144;
	// 82521F34: E8A50008  ld r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) };
	// 82521F38: 38600001  li r3, 1
	ctx.r[3].s64 = 1;
	// 82521F3C: EA460000  ld r18, 0(r6)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) };
	// 82521F40: 3BA100C0  addi r29, r1, 0xc0
	ctx.r[29].s64 = ctx.r[1].s64 + 192;
	// 82521F44: FA6A0000  std r19, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82521F48: 3BDF0030  addi r30, r31, 0x30
	ctx.r[30].s64 = ctx.r[31].s64 + 48;
	// 82521F4C: F96A0008  std r11, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82521F50: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82521F54: E8C60008  ld r6, 8(r6)
	ctx.r[6].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) };
	// 82521F58: FA280000  std r17, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[17].u64 ) };
	// 82521F5C: FA490000  std r18, 0(r9)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
	// 82521F60: EA040000  ld r16, 0(r4)
	ctx.r[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82522208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82522208 size=80
    let mut pc: u32 = 0x82522208;
    'dispatch: loop {
        match pc {
            0x82522208 => {
    //   block [0x82522208..0x82522258)
	// 82522208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252220C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82522210: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82522214: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82522218: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8252221C: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82522220: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 82522224: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82522228: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8252222C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82522230: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82522234: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82522238: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8252223C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82522240: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82522244: 4BFFF48D  bl 0x825216d0
	ctx.lr = 0x82522248;
	sub_825216D0(ctx, base);
	// 82522248: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252224C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82522250: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82522254: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82522258(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82522258 size=80
    let mut pc: u32 = 0x82522258;
    'dispatch: loop {
        match pc {
            0x82522258 => {
    //   block [0x82522258..0x825222A8)
	// 82522258: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252225C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82522260: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82522264: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82522268: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8252226C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82522270: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 82522274: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82522278: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8252227C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82522280: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82522284: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82522288: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8252228C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82522290: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82522294: 4BFFF73D  bl 0x825219d0
	ctx.lr = 0x82522298;
	sub_825219D0(ctx, base);
	// 82522298: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252229C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825222A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825222A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825222A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825222A8 size=176
    let mut pc: u32 = 0x825222A8;
    'dispatch: loop {
        match pc {
            0x825222A8 => {
    //   block [0x825222A8..0x82522328)
	// 825222A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825222AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825222B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825222B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825222B8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 825222BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825222C0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 825222C4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825222C8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825222CC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 825222D0: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825222D4: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 825222D8: 4BFFFBC1  bl 0x82521e98
	ctx.lr = 0x825222DC;
	sub_82521E98(ctx, base);
	// 825222DC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825222E0: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 825222E4: 40980044  bge cr6, 0x82522328
	if !ctx.cr[6].lt {
	pc = 0x82522328; continue 'dispatch;
	}
	// 825222E8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825222EC: 394B08F0  addi r10, r11, 0x8f0
	ctx.r[10].s64 = ctx.r[11].s64 + 2288;
	// 825222F0: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82522328; continue 'dispatch;
            }
            0x82522328 => {
    //   block [0x82522328..0x82522358)
	// 82522328: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252232C: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82522330: 419A000C  beq cr6, 0x8252233c
	if ctx.cr[6].eq {
	pc = 0x8252233C; continue 'dispatch;
	}
	// 82522334: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82522338: 4BFF1109  bl 0x82513440
	ctx.lr = 0x8252233C;
	sub_82513440(ctx, base);
	// 8252233C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82522340: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82522344: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82522348: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 8252234C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82522350: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82522354: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82522358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82522358 size=104
    let mut pc: u32 = 0x82522358;
    'dispatch: loop {
        match pc {
            0x82522358 => {
    //   block [0x82522358..0x825223C0)
	// 82522358: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252235C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82522360: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82522364: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82522368: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252236C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82522370: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82522374: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82522378: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 8252237C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82522380: 4BF41CB9  bl 0x82464038
	ctx.lr = 0x82522384;
	sub_82464038(ctx, base);
	// 82522384: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82522388: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 8252238C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82522390: 396B555C  addi r11, r11, 0x555c
	ctx.r[11].s64 = ctx.r[11].s64 + 21852;
	// 82522394: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82522398: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 8252239C: B1430004  sth r10, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 825223A0: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825223A4: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 825223A8: B103000C  sth r8, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[8].u16 ) };
	// 825223AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825223B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825223B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825223B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825223BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825223C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825223C0 size=112
    let mut pc: u32 = 0x825223C0;
    'dispatch: loop {
        match pc {
            0x825223C0 => {
    //   block [0x825223C0..0x82522404)
	// 825223C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825223C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825223C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825223CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825223D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825223D4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 825223D8: A17F000C  lhz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825223DC: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 825223E0: 419A0024  beq cr6, 0x82522404
	if ctx.cr[6].eq {
	pc = 0x82522404; continue 'dispatch;
	}
	// 825223E4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825223E8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825223EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825223F0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825223F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825223F8: 4E800421  bctrl
	ctx.lr = 0x825223FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825223FC: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82522400: B17F000C  sth r11, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u16 ) };
            }
            0x82522404 => {
    //   block [0x82522404..0x82522430)
	// 82522404: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522408: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252240C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82522410: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522414: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82522418: 4E800421  bctrl
	ctx.lr = 0x8252241C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252241C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82522420: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82522424: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82522428: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252242C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82522430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82522430 size=248
    let mut pc: u32 = 0x82522430;
    'dispatch: loop {
        match pc {
            0x82522430 => {
    //   block [0x82522430..0x82522484)
	// 82522430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82522434: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82522438: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252243C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82522440: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522444: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82522448: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 8252244C: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82522450: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82522454: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522458: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252245C: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82522460: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82522464: 40980020  bge cr6, 0x82522484
	if !ctx.cr[6].lt {
	pc = 0x82522484; continue 'dispatch;
	}
	// 82522468: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 8252246C: 39085594  addi r8, r8, 0x5594
	ctx.r[8].s64 = ctx.r[8].s64 + 21908;
	// 82522470: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82522474: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 82522478: 38EA000C  addi r7, r10, 0xc
	ctx.r[7].s64 = ctx.r[10].s64 + 12;
	// 8252247C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82522480: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	pc = 0x82522484; continue 'dispatch;
            }
            0x82522484 => {
    //   block [0x82522484..0x82522528)
	// 82522484: 81490008  lwz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82522488: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 8252248C: 81050008  lwz r8, 8(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82522490: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82522528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82522528 size=504
    let mut pc: u32 = 0x82522528;
    'dispatch: loop {
        match pc {
            0x82522528 => {
    //   block [0x82522528..0x82522578)
	// 82522528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252252C: 48012B8D  bl 0x825350b8
	ctx.lr = 0x82522530;
	sub_82535080(ctx, base);
	// 82522530: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82522534: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522538: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 8252253C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82522540: 7F8B5214  add r28, r11, r10
	ctx.r[28].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82522544: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82522548: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252254C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82522550: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82522554: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82522558: 40980020  bge cr6, 0x82522578
	if !ctx.cr[6].lt {
	pc = 0x82522578; continue 'dispatch;
	}
	// 8252255C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82522560: 39295594  addi r9, r9, 0x5594
	ctx.r[9].s64 = ctx.r[9].s64 + 21908;
	// 82522564: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82522568: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252256C: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82522570: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82522574: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82522578; continue 'dispatch;
            }
            0x82522578 => {
    //   block [0x82522578..0x82522720)
	// 82522578: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252257C: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82522580: 81040008  lwz r8, 8(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82522584: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82522588: 38EB0030  addi r7, r11, 0x30
	ctx.r[7].s64 = ctx.r[11].s64 + 48;
	// 8252258C: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522590: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82522720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82522720 size=436
    let mut pc: u32 = 0x82522720;
    'dispatch: loop {
        match pc {
            0x82522720 => {
    //   block [0x82522720..0x8252276C)
	// 82522720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82522724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82522728: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252272C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82522730: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522734: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82522738: 7FEB5214  add r31, r11, r10
	ctx.r[31].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252273C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522740: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82522744: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82522748: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252274C: 40980020  bge cr6, 0x8252276c
	if !ctx.cr[6].lt {
	pc = 0x8252276C; continue 'dispatch;
	}
	// 82522750: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82522754: 39295594  addi r9, r9, 0x5594
	ctx.r[9].s64 = ctx.r[9].s64 + 21908;
	// 82522758: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252275C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82522760: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82522764: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82522768: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252276C; continue 'dispatch;
            }
            0x8252276C => {
    //   block [0x8252276C..0x825228D4)
	// 8252276C: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82522770: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82522774: 80640008  lwz r3, 8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82522778: 392B0030  addi r9, r11, 0x30
	ctx.r[9].s64 = ctx.r[11].s64 + 48;
	// 8252277C: 90810080  stw r4, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u32 ) };
	// 82522780: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82522784: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522788: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8252278C: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825228D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825228D8 size=440
    let mut pc: u32 = 0x825228D8;
    'dispatch: loop {
        match pc {
            0x825228D8 => {
    //   block [0x825228D8..0x82522928)
	// 825228D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825228DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825228E0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825228E4: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825228E8: 812D0000  lwz r9, 0(r13)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825228EC: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 825228F0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825228F4: 7FEA4A14  add r31, r10, r9
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 825228F8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825228FC: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82522900: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82522904: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82522908: 40980020  bge cr6, 0x82522928
	if !ctx.cr[6].lt {
	pc = 0x82522928; continue 'dispatch;
	}
	// 8252290C: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82522910: 39085594  addi r8, r8, 0x5594
	ctx.r[8].s64 = ctx.r[8].s64 + 21908;
	// 82522914: 91090000  stw r8, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82522918: 7D0C42E6  mftb r8, 0x10c
	ctx.r[8].u64 = crate::rt::rdtsc_u64();
	// 8252291C: 38E9000C  addi r7, r9, 0xc
	ctx.r[7].s64 = ctx.r[9].s64 + 12;
	// 82522920: 91090004  stw r8, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82522924: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	pc = 0x82522928; continue 'dispatch;
            }
            0x82522928 => {
    //   block [0x82522928..0x82522A90)
	// 82522928: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252292C: 39000030  li r8, 0x30
	ctx.r[8].s64 = 48;
	// 82522930: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82522934: 392A0030  addi r9, r10, 0x30
	ctx.r[9].s64 = ctx.r[10].s64 + 48;
	// 82522938: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 8252293C: 90810084  stw r4, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[4].u32 ) };
	// 82522940: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522944: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82522948: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82522A90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82522A90 size=20
    let mut pc: u32 = 0x82522A90;
    'dispatch: loop {
        match pc {
            0x82522A90 => {
    //   block [0x82522A90..0x82522AA4)
	// 82522A90: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82522A94: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82522A98: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82522A9C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82522AA0: 4BFFF990  b 0x82522430
	sub_82522430(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82522AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82522AA8 size=52
    let mut pc: u32 = 0x82522AA8;
    'dispatch: loop {
        match pc {
            0x82522AA8 => {
    //   block [0x82522AA8..0x82522ADC)
	// 82522AA8: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82522AAC: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82522AB0: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82522AB4: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 82522AB8: 39082358  addi r8, r8, 0x2358
	ctx.r[8].s64 = ctx.r[8].s64 + 9048;
	// 82522ABC: 39292430  addi r9, r9, 0x2430
	ctx.r[9].s64 = ctx.r[9].s64 + 9264;
	// 82522AC0: 394A28D8  addi r10, r10, 0x28d8
	ctx.r[10].s64 = ctx.r[10].s64 + 10456;
	// 82522AC4: 396BBEF8  addi r11, r11, -0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + -16648;
	// 82522AC8: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82522ACC: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82522AD0: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 82522AD4: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82522AD8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82522AE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82522AE0 size=104
    let mut pc: u32 = 0x82522AE0;
    'dispatch: loop {
        match pc {
            0x82522AE0 => {
    //   block [0x82522AE0..0x82522B48)
	// 82522AE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82522AE4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82522AE8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82522AEC: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82522AF0: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82522AF4: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82522AF8: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 82522AFC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82522B00: 39082358  addi r8, r8, 0x2358
	ctx.r[8].s64 = ctx.r[8].s64 + 9048;
	// 82522B04: 39292430  addi r9, r9, 0x2430
	ctx.r[9].s64 = ctx.r[9].s64 + 9264;
	// 82522B08: 394A28D8  addi r10, r10, 0x28d8
	ctx.r[10].s64 = ctx.r[10].s64 + 10456;
	// 82522B0C: 396BBEF8  addi r11, r11, -0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + -16648;
	// 82522B10: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82522B14: 98E10060  stb r7, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u8 ) };
	// 82522B18: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82522B1C: 98E10061  stb r7, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[7].u8 ) };
	// 82522B20: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82522B24: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82522B28: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82522B2C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82522B30: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82522B34: 4BFDD255  bl 0x824ffd88
	ctx.lr = 0x82522B38;
	sub_824FFD88(ctx, base);
	// 82522B38: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82522B3C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82522B40: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82522B44: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82522B48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82522B48 size=104
    let mut pc: u32 = 0x82522B48;
    'dispatch: loop {
        match pc {
            0x82522B48 => {
    //   block [0x82522B48..0x82522BB0)
	// 82522B48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82522B4C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82522B50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82522B54: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82522B58: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82522B5C: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82522B60: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 82522B64: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82522B68: 39082358  addi r8, r8, 0x2358
	ctx.r[8].s64 = ctx.r[8].s64 + 9048;
	// 82522B6C: 39292430  addi r9, r9, 0x2430
	ctx.r[9].s64 = ctx.r[9].s64 + 9264;
	// 82522B70: 394A28D8  addi r10, r10, 0x28d8
	ctx.r[10].s64 = ctx.r[10].s64 + 10456;
	// 82522B74: 396BBEF8  addi r11, r11, -0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + -16648;
	// 82522B78: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82522B7C: 98E10060  stb r7, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u8 ) };
	// 82522B80: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82522B84: 98E10061  stb r7, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[7].u8 ) };
	// 82522B88: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82522B8C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82522B90: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82522B94: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82522B98: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82522B9C: 4BFDD305  bl 0x824ffea0
	ctx.lr = 0x82522BA0;
	sub_824FFEA0(ctx, base);
	// 82522BA0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82522BA4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82522BA8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82522BAC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82522BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82522BB0 size=120
    let mut pc: u32 = 0x82522BB0;
    'dispatch: loop {
        match pc {
            0x82522BB0 => {
    //   block [0x82522BB0..0x82522BD8)
	// 82522BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82522BB4: 48012505  bl 0x825350b8
	ctx.lr = 0x82522BB8;
	sub_82535080(ctx, base);
	// 82522BB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82522BBC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82522BC0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82522BC4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82522BC8: 897D0031  lbz r11, 0x31(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(49 as u32) ) } as u64;
	// 82522BCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82522BD0: 419A0038  beq cr6, 0x82522c08
	if ctx.cr[6].eq {
	pc = 0x82522C08; continue 'dispatch;
	}
	// 82522BD4: 3BFD0012  addi r31, r29, 0x12
	ctx.r[31].s64 = ctx.r[29].s64 + 18;
	pc = 0x82522BD8; continue 'dispatch;
            }
            0x82522BD8 => {
    //   block [0x82522BD8..0x82522C08)
	// 82522BD8: 807D0008  lwz r3, 8(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82522BDC: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82522BE0: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522BE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522BE8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82522BEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82522BF0: 4E800421  bctrl
	ctx.lr = 0x82522BF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82522BF4: 897D0031  lbz r11, 0x31(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[29].u32.wrapping_add(49 as u32) ) } as u64;
	// 82522BF8: 3BDE0001  addi r30, r30, 1
	ctx.r[30].s64 = ctx.r[30].s64 + 1;
	// 82522BFC: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82522C00: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82522C04: 4198FFD4  blt cr6, 0x82522bd8
	if ctx.cr[6].lt {
	pc = 0x82522BD8; continue 'dispatch;
	}
            }
            0x82522C08 => {
    //   block [0x82522C08..0x82522C28)
	// 82522C08: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522C0C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82522C10: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82522C14: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522C18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82522C1C: 4E800421  bctrl
	ctx.lr = 0x82522C20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82522C20: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82522C24: 480124E4  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82522C28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82522C28 size=184
    let mut pc: u32 = 0x82522C28;
    'dispatch: loop {
        match pc {
            0x82522C28 => {
    //   block [0x82522C28..0x82522CC0)
	// 82522C28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82522C2C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82522C30: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82522C34: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82522C38: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82522C3C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522C40: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82522C44: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82522C48: C18B002C  lfs f12, 0x2c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82522C4C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82522C50: C1AB55E0  lfs f13, 0x55e0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(21984 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82522C54: ED8C0372  fmuls f12, f12, f13
	ctx.f[12].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82522C58: FF006000  fcmpu cr6, f0, f12
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[12].f64);
	// 82522C5C: 40980064  bge cr6, 0x82522cc0
	if !ctx.cr[6].lt {
	pc = 0x82522CC0; continue 'dispatch;
	}
	// 82522C60: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522C64: C18B002C  lfs f12, 0x2c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82522C68: EDAC0372  fmuls f13, f12, f13
	ctx.f[13].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82522C6C: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82522C70: 40980050  bge cr6, 0x82522cc0
	if !ctx.cr[6].lt {
	pc = 0x82522CC0; continue 'dispatch;
	}
	// 82522C74: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82522C78: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82522C7C: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82522C80: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 82522C84: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82522C88: 4BF413B1  bl 0x82464038
	ctx.lr = 0x82522C8C;
	sub_82464038(ctx, base);
	// 82522C8C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82522C90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82522C94: 396B55A8  addi r11, r11, 0x55a8
	ctx.r[11].s64 = ctx.r[11].s64 + 21928;
	// 82522C98: 39400050  li r10, 0x50
	ctx.r[10].s64 = 80;
	// 82522C9C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82522CA0: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82522CA4: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82522CA8: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82522CAC: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82522CB0: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82522CB4: 4800961D  bl 0x8252c2d0
	ctx.lr = 0x82522CB8;
	sub_8252C2D0(ctx, base);
	// 82522CB8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82522CBC: 4800000C  b 0x82522cc8
	pc = 0x82522CC8; continue 'dispatch;
            }
            0x82522CC0 => {
    //   block [0x82522CC0..0x82522CC8)
	// 82522CC0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82522CC4: 48008B85  bl 0x8252b848
	ctx.lr = 0x82522CC8;
	sub_8252B848(ctx, base);
	pc = 0x82522CC8; continue 'dispatch;
            }
            0x82522CC8 => {
    //   block [0x82522CC8..0x82522CE0)
	// 82522CC8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82522CCC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82522CD0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82522CD4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82522CD8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82522CDC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82522CE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82522CE0 size=508
    let mut pc: u32 = 0x82522CE0;
    'dispatch: loop {
        match pc {
            0x82522CE0 => {
    //   block [0x82522CE0..0x82522EDC)
	// 82522CE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82522CE4: 480123C9  bl 0x825350ac
	ctx.lr = 0x82522CE8;
	sub_82535080(ctx, base);
	// 82522CE8: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82522EE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82522EE0 size=552
    let mut pc: u32 = 0x82522EE0;
    'dispatch: loop {
        match pc {
            0x82522EE0 => {
    //   block [0x82522EE0..0x82523108)
	// 82522EE0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82522EE4: 480121CD  bl 0x825350b0
	ctx.lr = 0x82522EE8;
	sub_82535080(ctx, base);
	// 82522EE8: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523108(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82523108 size=20
    let mut pc: u32 = 0x82523108;
    'dispatch: loop {
        match pc {
            0x82523108 => {
    //   block [0x82523108..0x8252311C)
	// 82523108: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 8252310C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82523110: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82523114: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82523118: 4BFFFDC8  b 0x82522ee0
	sub_82522EE0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523120(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82523120 size=544
    let mut pc: u32 = 0x82523120;
    'dispatch: loop {
        match pc {
            0x82523120 => {
    //   block [0x82523120..0x82523340)
	// 82523120: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82523124: 48011F8D  bl 0x825350b0
	ctx.lr = 0x82523128;
	sub_82535080(ctx, base);
	// 82523128: 3980FFA0  li r12, -0x60
	ctx.r[12].s64 = -96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523340(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82523340 size=20
    let mut pc: u32 = 0x82523340;
    'dispatch: loop {
        match pc {
            0x82523340 => {
    //   block [0x82523340..0x82523354)
	// 82523340: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82523344: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82523348: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 8252334C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82523350: 4BFFFDD0  b 0x82523120
	sub_82523120(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523358(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82523358 size=64
    let mut pc: u32 = 0x82523358;
    'dispatch: loop {
        match pc {
            0x82523358 => {
    //   block [0x82523358..0x82523398)
	// 82523358: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 8252335C: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82523360: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82523364: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82523368: 38EBBEF8  addi r7, r11, -0x4108
	ctx.r[7].s64 = ctx.r[11].s64 + -16648;
	// 8252336C: 39082C28  addi r8, r8, 0x2c28
	ctx.r[8].s64 = ctx.r[8].s64 + 11304;
	// 82523370: 39293120  addi r9, r9, 0x3120
	ctx.r[9].s64 = ctx.r[9].s64 + 12576;
	// 82523374: 394A2EE0  addi r10, r10, 0x2ee0
	ctx.r[10].s64 = ctx.r[10].s64 + 12000;
	// 82523378: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 8252337C: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82523380: 91030000  stw r8, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82523384: 91230004  stw r9, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82523388: 91430008  stw r10, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[10].u32 ) };
	// 8252338C: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82523390: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82523394: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523398(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82523398 size=104
    let mut pc: u32 = 0x82523398;
    'dispatch: loop {
        match pc {
            0x82523398 => {
    //   block [0x82523398..0x82523400)
	// 82523398: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252339C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825233A0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825233A4: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 825233A8: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 825233AC: 396BBEF8  addi r11, r11, -0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + -16648;
	// 825233B0: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 825233B4: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 825233B8: 39082C28  addi r8, r8, 0x2c28
	ctx.r[8].s64 = ctx.r[8].s64 + 11304;
	// 825233BC: 39293120  addi r9, r9, 0x3120
	ctx.r[9].s64 = ctx.r[9].s64 + 12576;
	// 825233C0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 825233C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825233C8: 394A2EE0  addi r10, r10, 0x2ee0
	ctx.r[10].s64 = ctx.r[10].s64 + 12000;
	// 825233CC: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 825233D0: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 825233D4: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 825233D8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825233DC: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 825233E0: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 825233E4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825233E8: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 825233EC: 4BFDC99D  bl 0x824ffd88
	ctx.lr = 0x825233F0;
	sub_824FFD88(ctx, base);
	// 825233F0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825233F4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825233F8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825233FC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523400(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82523400 size=160
    let mut pc: u32 = 0x82523400;
    'dispatch: loop {
        match pc {
            0x82523400 => {
    //   block [0x82523400..0x825234A0)
	// 82523400: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82523404: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82523408: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 8252340C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82523410: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82523414: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82523418: 39200050  li r9, 0x50
	ctx.r[9].s64 = 80;
	// 8252341C: 394300C0  addi r10, r3, 0xc0
	ctx.r[10].s64 = ctx.r[3].s64 + 192;
	// 82523420: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82523424: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82523428: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825234A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825234A0 size=148
    let mut pc: u32 = 0x825234A0;
    'dispatch: loop {
        match pc {
            0x825234A0 => {
    //   block [0x825234A0..0x8252350C)
	// 825234A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825234A4: 48011C15  bl 0x825350b8
	ctx.lr = 0x825234A8;
	sub_82535080(ctx, base);
	// 825234A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825234AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825234B0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825234B4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825234B8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825234BC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825234C0: 2B1C0000  cmplwi cr6, r28, 0
	ctx.cr[6].compare_u32(ctx.r[28].u32, 0 as u32, &mut ctx.xer);
	// 825234C4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825234C8: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 825234CC: 419A0040  beq cr6, 0x8252350c
	if ctx.cr[6].eq {
	pc = 0x8252350C; continue 'dispatch;
	}
	// 825234D0: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 825234D4: 4BF40B65  bl 0x82464038
	ctx.lr = 0x825234D8;
	sub_82464038(ctx, base);
	// 825234D8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825234DC: 39600080  li r11, 0x80
	ctx.r[11].s64 = 128;
	// 825234E0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 825234E4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825234E8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825234EC: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 825234F0: 48008311  bl 0x8252b800
	ctx.lr = 0x825234F4;
	sub_8252B800(ctx, base);
	// 825234F4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825234F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825234FC: 396B4F14  addi r11, r11, 0x4f14
	ctx.r[11].s64 = ctx.r[11].s64 + 20244;
	// 82523500: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82523504: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82523508: 48011C00  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            0x8252350C => {
    //   block [0x8252350C..0x82523534)
	// 8252350C: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82523510: 4BF40B29  bl 0x82464038
	ctx.lr = 0x82523514;
	sub_82464038(ctx, base);
	// 82523514: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82523518: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 8252351C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82523520: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82523524: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82523528: 48007631  bl 0x8252ab58
	ctx.lr = 0x8252352C;
	sub_8252AB58(ctx, base);
	// 8252352C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82523530: 48011BD8  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82523538 size=108
    let mut pc: u32 = 0x82523538;
    'dispatch: loop {
        match pc {
            0x82523538 => {
    //   block [0x82523538..0x825235A4)
	// 82523538: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252353C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82523540: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82523544: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 82523548: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 8252354C: 396BBEF8  addi r11, r11, -0x4108
	ctx.r[11].s64 = ctx.r[11].s64 + -16648;
	// 82523550: 3D208253  lis r9, -0x7dad
	ctx.r[9].s64 = -2108489728;
	// 82523554: 3D408253  lis r10, -0x7dad
	ctx.r[10].s64 = -2108489728;
	// 82523558: 390834A0  addi r8, r8, 0x34a0
	ctx.r[8].s64 = ctx.r[8].s64 + 13472;
	// 8252355C: 3929AF20  addi r9, r9, -0x50e0
	ctx.r[9].s64 = ctx.r[9].s64 + -20704;
	// 82523560: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82523564: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82523568: 394AB0C8  addi r10, r10, -0x4f38
	ctx.r[10].s64 = ctx.r[10].s64 + -20280;
	// 8252356C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82523570: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82523574: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82523578: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8252357C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82523580: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82523584: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82523588: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8252358C: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 82523590: 4BFDC7F9  bl 0x824ffd88
	ctx.lr = 0x82523594;
	sub_824FFD88(ctx, base);
	// 82523594: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82523598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252359C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825235A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825235A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825235A8 size=1496
    let mut pc: u32 = 0x825235A8;
    'dispatch: loop {
        match pc {
            0x825235A8 => {
    //   block [0x825235A8..0x82523604)
	// 825235A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825235AC: 48011AE5  bl 0x82535090
	ctx.lr = 0x825235B0;
	sub_82535080(ctx, base);
	// 825235B0: 9421FB30  stwu r1, -0x4d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1232 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825235B4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825235B8: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 825235BC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 825235C0: 7ECB5214  add r22, r11, r10
	ctx.r[22].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825235C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825235C8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825235CC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825235D0: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 825235D4: 81760000  lwz r11, 0(r22)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 825235D8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825235DC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825235E0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825235E4: 40980020  bge cr6, 0x82523604
	if !ctx.cr[6].lt {
	pc = 0x82523604; continue 'dispatch;
	}
	// 825235E8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825235EC: 39295600  addi r9, r9, 0x5600
	ctx.r[9].s64 = ctx.r[9].s64 + 22016;
	// 825235F0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825235F4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825235F8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 825235FC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82523600: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82523604; continue 'dispatch;
            }
            0x82523604 => {
    //   block [0x82523604..0x82523674)
	// 82523604: C03C0050  lfs f1, 0x50(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82523608: C01B0018  lfs f0, 0x18(r27)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252360C: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 82523610: 419A0178  beq cr6, 0x82523788
	if ctx.cr[6].eq {
	pc = 0x82523788; continue 'dispatch;
	}
	// 82523614: 817C0060  lwz r11, 0x60(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(96 as u32) ) } as u64;
	// 82523618: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252361C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82523620: 409A005C  bne cr6, 0x8252367c
	if !ctx.cr[6].eq {
	pc = 0x8252367C; continue 'dispatch;
	}
	// 82523624: C01C0054  lfs f0, 0x54(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82523628: D01B0018  stfs f0, 0x18(r27)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 8252362C: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 82523630: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82523634: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82523638: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8252363C: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82523640: 48008351  bl 0x8252b990
	ctx.lr = 0x82523644;
	sub_8252B990(ctx, base);
	// 82523644: 81560000  lwz r10, 0(r22)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[22].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523648: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252364C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523650: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82523654: 40980020  bge cr6, 0x82523674
	if !ctx.cr[6].lt {
	pc = 0x82523674; continue 'dispatch;
	}
	// 82523658: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8252365C: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 82523660: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82523664: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82523668: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8252366C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82523670: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82523674; continue 'dispatch;
            }
            0x82523674 => {
    //   block [0x82523674..0x8252367C)
	// 82523674: 382104D0  addi r1, r1, 0x4d0
	ctx.r[1].s64 = ctx.r[1].s64 + 1232;
	// 82523678: 48011A68  b 0x825350e0
	sub_825350D0(ctx, base);
	return;
            }
            0x8252367C => {
    //   block [0x8252367C..0x82523788)
	// 8252367C: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82523680: 38A10410  addi r5, r1, 0x410
	ctx.r[5].s64 = ctx.r[1].s64 + 1040;
	// 82523684: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523688: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 8252368C: 817E0004  lwz r11, 4(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82523690: 93C1008C  stw r30, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[30].u32 ) };
	// 82523694: 93A1007C  stw r29, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[29].u32 ) };
	// 82523698: 91410080  stw r10, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[10].u32 ) };
	// 8252369C: 91610084  stw r11, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[11].u32 ) };
	// 825236A0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825236A4: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 825236A8: 39610410  addi r11, r1, 0x410
	ctx.r[11].s64 = ctx.r[1].s64 + 1040;
	// 825236AC: 91610088  stw r11, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[11].u32 ) };
	// 825236B0: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 825236B4: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825236B8: 396102B0  addi r11, r1, 0x2b0
	ctx.r[11].s64 = ctx.r[1].s64 + 688;
	// 825236BC: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 825236C0: 48085CE9  bl 0x825a93a8
	ctx.lr = 0x825236C4;
	sub_825A93A8(ctx, base);
	// 825236C4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825236C8: 38A102B0  addi r5, r1, 0x2b0
	ctx.r[5].s64 = ctx.r[1].s64 + 688;
	// 825236CC: C03C0050  lfs f1, 0x50(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825236D0: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 825236D4: 48085CD5  bl 0x825a93a8
	ctx.lr = 0x825236D8;
	sub_825A93A8(ctx, base);
	// 825236D8: 3BFB000C  addi r31, r27, 0xc
	ctx.r[31].s64 = ctx.r[27].s64 + 12;
	// 825236DC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825236E0: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825236E4: 835D0000  lwz r26, 0(r29)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825236E8: 38C10150  addi r6, r1, 0x150
	ctx.r[6].s64 = ctx.r[1].s64 + 336;
	// 825236EC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825236F0: 895F0009  lbz r10, 9(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 825236F4: 91610140  stw r11, 0x140(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[11].u32 ) };
	// 825236F8: 91610144  stw r11, 0x144(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(324 as u32), ctx.r[11].u32 ) };
	// 825236FC: 897F000A  lbz r11, 0xa(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 82523700: 88BF0008  lbz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82523704: 91410134  stw r10, 0x134(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), ctx.r[10].u32 ) };
	// 82523708: 556AE13E  srwi r10, r11, 4
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252370C: 556B073E  clrlwi r11, r11, 0x1c
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82523710: 90A10130  stw r5, 0x130(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(304 as u32), ctx.r[5].u32 ) };
	// 82523714: 91410138  stw r10, 0x138(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(312 as u32), ctx.r[10].u32 ) };
	// 82523718: 9161013C  stw r11, 0x13c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(316 as u32), ctx.r[11].u32 ) };
	// 8252371C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523720: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82523724: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82523728: 4E800421  bctrl
	ctx.lr = 0x8252372C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252372C: 81610130  lwz r11, 0x130(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(304 as u32) ) } as u64;
	// 82523730: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523734: 38C10210  addi r6, r1, 0x210
	ctx.r[6].s64 = ctx.r[1].s64 + 528;
	// 82523738: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252373C: 80A10134  lwz r5, 0x134(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82523740: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82523744: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82523748: 816A0034  lwz r11, 0x34(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 8252374C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82523750: 4E800421  bctrl
	ctx.lr = 0x82523754;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82523754: 817C0060  lwz r11, 0x60(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(96 as u32) ) } as u64;
	// 82523758: 38FB0020  addi r7, r27, 0x20
	ctx.r[7].s64 = ctx.r[27].s64 + 32;
	// 8252375C: 38C10130  addi r6, r1, 0x130
	ctx.r[6].s64 = ctx.r[1].s64 + 304;
	// 82523760: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82523764: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82523768: C02B0000  lfs f1, 0(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 8252376C: 480074DD  bl 0x8252ac48
	ctx.lr = 0x82523770;
	sub_8252AC48(ctx, base);
	// 82523770: 81610144  lwz r11, 0x144(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(324 as u32) ) } as u64;
	// 82523774: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82523778: 419A0010  beq cr6, 0x82523788
	if ctx.cr[6].eq {
	pc = 0x82523788; continue 'dispatch;
	}
	// 8252377C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82523780: 38610130  addi r3, r1, 0x130
	ctx.r[3].s64 = ctx.r[1].s64 + 304;
	// 82523784: 480A2B3D  bl 0x825c62c0
	ctx.lr = 0x82523788;
	sub_825C62C0(ctx, base);
            }
            0x82523788 => {
    //   block [0x82523788..0x82523B80)
	// 82523788: C01C0054  lfs f0, 0x54(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252378C: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82523790: D01B0018  stfs f0, 0x18(r27)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82523794: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82523798: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252379C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825237A0: 392B0040  addi r9, r11, 0x40
	ctx.r[9].s64 = ctx.r[11].s64 + 64;
	// 825237A4: C01C0058  lfs f0, 0x58(r28)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825237A8: 38EA0040  addi r7, r10, 0x40
	ctx.r[7].s64 = ctx.r[10].s64 + 64;
	// 825237AC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825237B0: C1AB00A0  lfs f13, 0xa0(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(160 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 825237B4: 391B0020  addi r8, r27, 0x20
	ctx.r[8].s64 = ctx.r[27].s64 + 32;
	// 825237B8: C18B009C  lfs f12, 0x9c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(156 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 825237BC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 825237C0: ED8D0332  fmuls f12, f13, f12
	ctx.f[12].f64 = (((ctx.f[13].f64 * ctx.f[12].f64) as f32) as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523B80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82523B80 size=144
    let mut pc: u32 = 0x82523B80;
    'dispatch: loop {
        match pc {
            0x82523B80 => {
    //   block [0x82523B80..0x82523BBC)
	// 82523B80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82523B84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82523B88: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82523B8C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82523B90: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82523B94: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82523B98: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82523B9C: 409A0020  bne cr6, 0x82523bbc
	if !ctx.cr[6].eq {
	pc = 0x82523BBC; continue 'dispatch;
	}
	// 82523BA0: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82523BA4: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82523BA8: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523BAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523BB0: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82523BB4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82523BB8: 4E800421  bctrl
	ctx.lr = 0x82523BBC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82523BBC => {
    //   block [0x82523BBC..0x82523BE4)
	// 82523BBC: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82523BC0: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82523BC4: 409A0020  bne cr6, 0x82523be4
	if !ctx.cr[6].eq {
	pc = 0x82523BE4; continue 'dispatch;
	}
	// 82523BC8: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82523BCC: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523BD0: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82523BD4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523BD8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82523BDC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82523BE0: 4E800421  bctrl
	ctx.lr = 0x82523BE4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82523BE4 => {
    //   block [0x82523BE4..0x82523C10)
	// 82523BE4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523BE8: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82523BEC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82523BF0: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523BF4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82523BF8: 4E800421  bctrl
	ctx.lr = 0x82523BFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82523BFC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82523C00: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82523C04: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82523C08: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82523C0C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82523C10 size=4
    let mut pc: u32 = 0x82523C10;
    'dispatch: loop {
        match pc {
            0x82523C10 => {
    //   block [0x82523C10..0x82523C14)
	// 82523C10: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523C18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82523C18 size=20
    let mut pc: u32 = 0x82523C18;
    'dispatch: loop {
        match pc {
            0x82523C18 => {
    //   block [0x82523C18..0x82523C2C)
	// 82523C18: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523C1C: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 82523C20: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82523C24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82523C28: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523C30(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82523C30 size=28
    let mut pc: u32 = 0x82523C30;
    'dispatch: loop {
        match pc {
            0x82523C30 => {
    //   block [0x82523C30..0x82523C4C)
	// 82523C30: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523C34: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82523C38: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82523C3C: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82523C40: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82523C44: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82523C48: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82523C50 size=4
    let mut pc: u32 = 0x82523C50;
    'dispatch: loop {
        match pc {
            0x82523C50 => {
    //   block [0x82523C50..0x82523C54)
	// 82523C50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523C58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82523C58 size=4
    let mut pc: u32 = 0x82523C58;
    'dispatch: loop {
        match pc {
            0x82523C58 => {
    //   block [0x82523C58..0x82523C5C)
	// 82523C58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523C60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82523C60 size=4
    let mut pc: u32 = 0x82523C60;
    'dispatch: loop {
        match pc {
            0x82523C60 => {
    //   block [0x82523C60..0x82523C64)
	// 82523C60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523C68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82523C68 size=4
    let mut pc: u32 = 0x82523C68;
    'dispatch: loop {
        match pc {
            0x82523C68 => {
    //   block [0x82523C68..0x82523C6C)
	// 82523C68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523C70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82523C70 size=124
    let mut pc: u32 = 0x82523C70;
    'dispatch: loop {
        match pc {
            0x82523C70 => {
    //   block [0x82523C70..0x82523C98)
	// 82523C70: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82523C74: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82523C78: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82523C7C: 394B560C  addi r10, r11, 0x560c
	ctx.r[10].s64 = ctx.r[11].s64 + 22028;
	// 82523C80: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82523C84: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82523C88: 91430000  stw r10, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82523C8C: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523C90: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82523C94: 419A0014  beq cr6, 0x82523ca8
	if ctx.cr[6].eq {
	pc = 0x82523CA8; continue 'dispatch;
	}
	pc = 0x82523C98; continue 'dispatch;
            }
            0x82523C98 => {
    //   block [0x82523C98..0x82523CA8)
	// 82523C98: 7D4B5378  mr r11, r10
	ctx.r[11].u64 = ctx.r[10].u64;
	// 82523C9C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523CA0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82523CA4: 409AFFF4  bne cr6, 0x82523c98
	if !ctx.cr[6].eq {
	pc = 0x82523C98; continue 'dispatch;
	}
	pc = 0x82523CA8; continue 'dispatch;
            }
            0x82523CA8 => {
    //   block [0x82523CA8..0x82523CBC)
	// 82523CA8: 9163000C  stw r11, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82523CAC: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82523CB0: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523CB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523CB8: 419A0014  beq cr6, 0x82523ccc
	if ctx.cr[6].eq {
	pc = 0x82523CCC; continue 'dispatch;
	}
	pc = 0x82523CBC; continue 'dispatch;
            }
            0x82523CBC => {
    //   block [0x82523CBC..0x82523CCC)
	// 82523CBC: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82523CC0: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523CC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523CC8: 409AFFF4  bne cr6, 0x82523cbc
	if !ctx.cr[6].eq {
	pc = 0x82523CBC; continue 'dispatch;
	}
	pc = 0x82523CCC; continue 'dispatch;
            }
            0x82523CCC => {
    //   block [0x82523CCC..0x82523CEC)
	// 82523CCC: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82523CD0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523CD4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523CD8: 9163001C  stw r11, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[11].u32 ) };
	// 82523CDC: 81650000  lwz r11, 0(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523CE0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523CE4: 91630020  stw r11, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u32 ) };
	// 82523CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82523CF0 size=304
    let mut pc: u32 = 0x82523CF0;
    'dispatch: loop {
        match pc {
            0x82523CF0 => {
    //   block [0x82523CF0..0x82523D5C)
	// 82523CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82523CF4: 480113C1  bl 0x825350b4
	ctx.lr = 0x82523CF8;
	sub_82535080(ctx, base);
	// 82523CF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82523CFC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523D00: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82523D04: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82523D08: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82523D0C: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82523D10: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82523D14: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82523D18: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82523D1C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82523D20: 4BF40319  bl 0x82464038
	ctx.lr = 0x82523D24;
	sub_82464038(ctx, base);
	// 82523D24: 39600024  li r11, 0x24
	ctx.r[11].s64 = 36;
	// 82523D28: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82523D2C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82523D30: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82523D34: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82523D38: 4BFFFF39  bl 0x82523c70
	ctx.lr = 0x82523D3C;
	sub_82523C70(ctx, base);
	// 82523D3C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82523D40: 817C001C  lwz r11, 0x1c(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82523D44: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82523D48: 409A0060  bne cr6, 0x82523da8
	if !ctx.cr[6].eq {
	pc = 0x82523DA8; continue 'dispatch;
	}
	// 82523D4C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523D50: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523D54: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523D58: 419A0014  beq cr6, 0x82523d6c
	if ctx.cr[6].eq {
	pc = 0x82523D6C; continue 'dispatch;
	}
	pc = 0x82523D5C; continue 'dispatch;
            }
            0x82523D5C => {
    //   block [0x82523D5C..0x82523D6C)
	// 82523D5C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82523D60: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523D64: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523D68: 409AFFF4  bne cr6, 0x82523d5c
	if !ctx.cr[6].eq {
	pc = 0x82523D5C; continue 'dispatch;
	}
	pc = 0x82523D6C; continue 'dispatch;
            }
            0x82523D6C => {
    //   block [0x82523D6C..0x82523D7C)
	// 82523D6C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523D70: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82523D74: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523D78: 419A0014  beq cr6, 0x82523d8c
	if ctx.cr[6].eq {
	pc = 0x82523D8C; continue 'dispatch;
	}
	pc = 0x82523D7C; continue 'dispatch;
            }
            0x82523D7C => {
    //   block [0x82523D7C..0x82523D8C)
	// 82523D7C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82523D80: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523D84: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523D88: 409AFFF4  bne cr6, 0x82523d7c
	if !ctx.cr[6].eq {
	pc = 0x82523D7C; continue 'dispatch;
	}
	pc = 0x82523D8C; continue 'dispatch;
            }
            0x82523D8C => {
    //   block [0x82523D8C..0x82523DA8)
	// 82523D8C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523D90: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82523D94: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82523D98: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82523D9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82523DA0: 4E800421  bctrl
	ctx.lr = 0x82523DA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82523DA4: 93DC0014  stw r30, 0x14(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
            }
            0x82523DA8 => {
    //   block [0x82523DA8..0x82523DC8)
	// 82523DA8: 817C0020  lwz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82523DAC: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82523DB0: 409A0064  bne cr6, 0x82523e14
	if !ctx.cr[6].eq {
	pc = 0x82523E14; continue 'dispatch;
	}
	// 82523DB4: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523DB8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82523DBC: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523DC0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523DC4: 419A0014  beq cr6, 0x82523dd8
	if ctx.cr[6].eq {
	pc = 0x82523DD8; continue 'dispatch;
	}
	pc = 0x82523DC8; continue 'dispatch;
            }
            0x82523DC8 => {
    //   block [0x82523DC8..0x82523DD8)
	// 82523DC8: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82523DCC: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523DD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523DD4: 409AFFF4  bne cr6, 0x82523dc8
	if !ctx.cr[6].eq {
	pc = 0x82523DC8; continue 'dispatch;
	}
	pc = 0x82523DD8; continue 'dispatch;
            }
            0x82523DD8 => {
    //   block [0x82523DD8..0x82523DE8)
	// 82523DD8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523DDC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82523DE0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523DE4: 419A0014  beq cr6, 0x82523df8
	if ctx.cr[6].eq {
	pc = 0x82523DF8; continue 'dispatch;
	}
	pc = 0x82523DE8; continue 'dispatch;
            }
            0x82523DE8 => {
    //   block [0x82523DE8..0x82523DF8)
	// 82523DE8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82523DEC: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523DF0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523DF4: 409AFFF4  bne cr6, 0x82523de8
	if !ctx.cr[6].eq {
	pc = 0x82523DE8; continue 'dispatch;
	}
	pc = 0x82523DF8; continue 'dispatch;
            }
            0x82523DF8 => {
    //   block [0x82523DF8..0x82523E14)
	// 82523DF8: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523DFC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82523E00: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82523E04: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82523E08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82523E0C: 4E800421  bctrl
	ctx.lr = 0x82523E10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82523E10: 93DC0018  stw r30, 0x18(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[30].u32 ) };
            }
            0x82523E14 => {
    //   block [0x82523E14..0x82523E20)
	// 82523E14: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82523E18: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82523E1C: 480112E8  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523E20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82523E20 size=236
    let mut pc: u32 = 0x82523E20;
    'dispatch: loop {
        match pc {
            0x82523E20 => {
    //   block [0x82523E20..0x82523E58)
	// 82523E20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82523E24: 48011299  bl 0x825350bc
	ctx.lr = 0x82523E28;
	sub_82535080(ctx, base);
	// 82523E28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82523E2C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82523E30: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82523E34: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82523E38: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523E3C: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523E40: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82523E44: 409A0058  bne cr6, 0x82523e9c
	if !ctx.cr[6].eq {
	pc = 0x82523E9C; continue 'dispatch;
	}
	// 82523E48: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523E4C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82523E50: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523E54: 419A0014  beq cr6, 0x82523e68
	if ctx.cr[6].eq {
	pc = 0x82523E68; continue 'dispatch;
	}
	pc = 0x82523E58; continue 'dispatch;
            }
            0x82523E58 => {
    //   block [0x82523E58..0x82523E68)
	// 82523E58: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82523E5C: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523E60: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523E64: 409AFFF4  bne cr6, 0x82523e58
	if !ctx.cr[6].eq {
	pc = 0x82523E58; continue 'dispatch;
	}
	pc = 0x82523E68; continue 'dispatch;
            }
            0x82523E68 => {
    //   block [0x82523E68..0x82523E78)
	// 82523E68: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523E6C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82523E70: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523E74: 419A0014  beq cr6, 0x82523e88
	if ctx.cr[6].eq {
	pc = 0x82523E88; continue 'dispatch;
	}
	pc = 0x82523E78; continue 'dispatch;
            }
            0x82523E78 => {
    //   block [0x82523E78..0x82523E88)
	// 82523E78: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82523E7C: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523E80: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523E84: 409AFFF4  bne cr6, 0x82523e78
	if !ctx.cr[6].eq {
	pc = 0x82523E78; continue 'dispatch;
	}
	pc = 0x82523E88; continue 'dispatch;
            }
            0x82523E88 => {
    //   block [0x82523E88..0x82523E9C)
	// 82523E88: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523E8C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82523E90: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82523E94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82523E98: 4E800421  bctrl
	ctx.lr = 0x82523E9C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82523E9C => {
    //   block [0x82523E9C..0x82523EBC)
	// 82523E9C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523EA0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523EA4: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82523EA8: 409A0058  bne cr6, 0x82523f00
	if !ctx.cr[6].eq {
	pc = 0x82523F00; continue 'dispatch;
	}
	// 82523EAC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523EB0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82523EB4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523EB8: 419A0014  beq cr6, 0x82523ecc
	if ctx.cr[6].eq {
	pc = 0x82523ECC; continue 'dispatch;
	}
	pc = 0x82523EBC; continue 'dispatch;
            }
            0x82523EBC => {
    //   block [0x82523EBC..0x82523ECC)
	// 82523EBC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82523EC0: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523EC4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523EC8: 409AFFF4  bne cr6, 0x82523ebc
	if !ctx.cr[6].eq {
	pc = 0x82523EBC; continue 'dispatch;
	}
	pc = 0x82523ECC; continue 'dispatch;
            }
            0x82523ECC => {
    //   block [0x82523ECC..0x82523EDC)
	// 82523ECC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523ED0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82523ED4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523ED8: 419A0014  beq cr6, 0x82523eec
	if ctx.cr[6].eq {
	pc = 0x82523EEC; continue 'dispatch;
	}
	pc = 0x82523EDC; continue 'dispatch;
            }
            0x82523EDC => {
    //   block [0x82523EDC..0x82523EEC)
	// 82523EDC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82523EE0: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523EE4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82523EE8: 409AFFF4  bne cr6, 0x82523edc
	if !ctx.cr[6].eq {
	pc = 0x82523EDC; continue 'dispatch;
	}
	pc = 0x82523EEC; continue 'dispatch;
            }
            0x82523EEC => {
    //   block [0x82523EEC..0x82523F00)
	// 82523EEC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523EF0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82523EF4: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82523EF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82523EFC: 4E800421  bctrl
	ctx.lr = 0x82523F00;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82523F00 => {
    //   block [0x82523F00..0x82523F0C)
	// 82523F00: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82523F04: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82523F08: 48011204  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82523F10 size=140
    let mut pc: u32 = 0x82523F10;
    'dispatch: loop {
        match pc {
            0x82523F10 => {
    //   block [0x82523F10..0x82523F9C)
	// 82523F10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82523F14: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82523F18: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82523F1C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82523F20: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 82523F24: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82523F28: 396B3C68  addi r11, r11, 0x3c68
	ctx.r[11].s64 = ctx.r[11].s64 + 15464;
	// 82523F2C: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82523F30: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82523F34: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82523F38: 39083CF0  addi r8, r8, 0x3cf0
	ctx.r[8].s64 = ctx.r[8].s64 + 15600;
	// 82523F3C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82523F40: 39293C30  addi r9, r9, 0x3c30
	ctx.r[9].s64 = ctx.r[9].s64 + 15408;
	// 82523F44: 394A3C58  addi r10, r10, 0x3c58
	ctx.r[10].s64 = ctx.r[10].s64 + 15448;
	// 82523F48: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82523F4C: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82523F50: 98E10060  stb r7, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u8 ) };
	// 82523F54: 38A0001D  li r5, 0x1d
	ctx.r[5].s64 = 29;
	// 82523F58: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82523F5C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82523F60: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82523F64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82523F68: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82523F6C: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 82523F70: 4BFDBE19  bl 0x824ffd88
	ctx.lr = 0x82523F74;
	sub_824FFD88(ctx, base);
	// 82523F74: 38C0001D  li r6, 0x1d
	ctx.r[6].s64 = 29;
	// 82523F78: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82523F7C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82523F80: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82523F84: 4BFDBE05  bl 0x824ffd88
	ctx.lr = 0x82523F88;
	sub_824FFD88(ctx, base);
	// 82523F88: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82523F8C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82523F90: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82523F94: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82523F98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523FA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82523FA0 size=84
    let mut pc: u32 = 0x82523FA0;
    'dispatch: loop {
        match pc {
            0x82523FA0 => {
    //   block [0x82523FA0..0x82523FF4)
	// 82523FA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82523FA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82523FA8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82523FAC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82523FB0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82523FB4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82523FB8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523FBC: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82523FC0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82523FC4: 4E800421  bctrl
	ctx.lr = 0x82523FC8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82523FC8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523FCC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82523FD0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82523FD4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82523FD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82523FDC: 4E800421  bctrl
	ctx.lr = 0x82523FE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82523FE0: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82523FE4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82523FE8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82523FEC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82523FF0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82523FF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82523FF8 size=144
    let mut pc: u32 = 0x82523FF8;
    'dispatch: loop {
        match pc {
            0x82523FF8 => {
    //   block [0x82523FF8..0x82524088)
	// 82523FF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82523FFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82524000: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82524004: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82524008: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252400C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82524010: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82524014: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82524018: 388B566C  addi r4, r11, 0x566c
	ctx.r[4].s64 = ctx.r[11].s64 + 22124;
	// 8252401C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82524020: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524024: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82524028: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252402C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82524030: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82524034: 4E800421  bctrl
	ctx.lr = 0x82524038;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82524038: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252403C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82524040: 80DE000C  lwz r6, 0xc(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82524044: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82524048: 388B5660  addi r4, r11, 0x5660
	ctx.r[4].s64 = ctx.r[11].s64 + 22112;
	// 8252404C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82524050: 816A000C  lwz r11, 0xc(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82524054: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82524058: 4E800421  bctrl
	ctx.lr = 0x8252405C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252405C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524060: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82524064: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82524068: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252406C: 4E800421  bctrl
	ctx.lr = 0x82524070;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82524070: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82524074: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82524078: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 8252407C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82524080: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82524084: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82524088 size=20
    let mut pc: u32 = 0x82524088;
    'dispatch: loop {
        match pc {
            0x82524088 => {
    //   block [0x82524088..0x8252409C)
	// 82524088: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252408C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524090: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82524094: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82524098: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825240A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825240A0 size=20
    let mut pc: u32 = 0x825240A0;
    'dispatch: loop {
        match pc {
            0x825240A0 => {
    //   block [0x825240A0..0x825240B4)
	// 825240A0: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825240A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825240A8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825240AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825240B0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825240B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825240B8 size=20
    let mut pc: u32 = 0x825240B8;
    'dispatch: loop {
        match pc {
            0x825240B8 => {
    //   block [0x825240B8..0x825240CC)
	// 825240B8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825240BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825240C0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825240C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825240C8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825240D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825240D0 size=20
    let mut pc: u32 = 0x825240D0;
    'dispatch: loop {
        match pc {
            0x825240D0 => {
    //   block [0x825240D0..0x825240E4)
	// 825240D0: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825240D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825240D8: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 825240DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825240E0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825240E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825240E8 size=20
    let mut pc: u32 = 0x825240E8;
    'dispatch: loop {
        match pc {
            0x825240E8 => {
    //   block [0x825240E8..0x825240FC)
	// 825240E8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825240EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825240F0: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 825240F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825240F8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524100(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82524100 size=264
    let mut pc: u32 = 0x82524100;
    'dispatch: loop {
        match pc {
            0x82524100 => {
    //   block [0x82524100..0x825241BC)
	// 82524100: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82524104: 48010FA5  bl 0x825350a8
	ctx.lr = 0x82524108;
	sub_82535080(ctx, base);
	// 82524108: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252410C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524110: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82524114: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82524118: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8252411C: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82524120: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82524124: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82524128: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 8252412C: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82524130: 4BF3FF09  bl 0x82464038
	ctx.lr = 0x82524134;
	sub_82464038(ctx, base);
	// 82524134: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82524138: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252413C: 396B5680  addi r11, r11, 0x5680
	ctx.r[11].s64 = ctx.r[11].s64 + 22144;
	// 82524140: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82524144: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82524148: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8252414C: 931F0008  stw r24, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 82524150: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82524154: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82524158: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 8252415C: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524160: 839E0008  lwz r28, 8(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82524164: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82524168: 835B0014  lwz r26, 0x14(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252416C: 48000895  bl 0x82524a00
	ctx.lr = 0x82524170;
	sub_82524A00(ctx, base);
	// 82524170: 38BB0030  addi r5, r27, 0x30
	ctx.r[5].s64 = ctx.r[27].s64 + 48;
	// 82524174: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82524178: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 8252417C: 48084F1D  bl 0x825a9098
	ctx.lr = 0x82524180;
	sub_825A9098(ctx, base);
	// 82524180: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82524184: 81390000  lwz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524188: 811D0010  lwz r8, 0x10(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252418C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524190: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82524194: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82524198: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8252419C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825241A0: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 825241A4: 390A05A0  addi r8, r10, 0x5a0
	ctx.r[8].s64 = ctx.r[10].s64 + 1440;
	// 825241A8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825241AC: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 825241B0: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 825241B4: 409A0008  bne cr6, 0x825241bc
	if !ctx.cr[6].eq {
	pc = 0x825241BC; continue 'dispatch;
	}
	// 825241B8: 390A01A0  addi r8, r10, 0x1a0
	ctx.r[8].s64 = ctx.r[10].s64 + 416;
	pc = 0x825241BC; continue 'dispatch;
            }
            0x825241BC => {
    //   block [0x825241BC..0x82524208)
	// 825241BC: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825241C0: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 825241C4: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 825241C8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825241CC: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 825241D0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825241D4: 7D6B48AE  lbzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 825241D8: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 825241DC: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825241E0: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825241E4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 825241E8: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 825241EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825241F0: 4E800421  bctrl
	ctx.lr = 0x825241F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825241F4: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825241F8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825241FC: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 82524200: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82524204: 48010EF4  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524208(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82524208 size=544
    let mut pc: u32 = 0x82524208;
    'dispatch: loop {
        match pc {
            0x82524208 => {
    //   block [0x82524208..0x82524260)
	// 82524208: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252420C: 48010E91  bl 0x8253509c
	ctx.lr = 0x82524210;
	sub_82535080(ctx, base);
	// 82524210: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524214: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524218: 3B600014  li r27, 0x14
	ctx.r[27].s64 = 20;
	// 8252421C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82524220: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82524224: 7CB82B78  mr r24, r5
	ctx.r[24].u64 = ctx.r[5].u64;
	// 82524228: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 8252422C: 7D7BD02E  lwzx r11, r27, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82524230: 7CF63B78  mr r22, r7
	ctx.r[22].u64 = ctx.r[7].u64;
	// 82524234: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82524238: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252423C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82524240: 40980020  bge cr6, 0x82524260
	if !ctx.cr[6].lt {
	pc = 0x82524260; continue 'dispatch;
	}
	// 82524244: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82524248: 392956B8  addi r9, r9, 0x56b8
	ctx.r[9].s64 = ctx.r[9].s64 + 22200;
	// 8252424C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82524250: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82524254: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82524258: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252425C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82524260; continue 'dispatch;
            }
            0x82524260 => {
    //   block [0x82524260..0x82524428)
	// 82524260: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524264: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82524268: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252426C: 3BFD0030  addi r31, r29, 0x30
	ctx.r[31].s64 = ctx.r[29].s64 + 48;
	// 82524270: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82524274: 48084E25  bl 0x825a9098
	ctx.lr = 0x82524278;
	sub_825A9098(ctx, base);
	// 82524278: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252427C: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
	// 82524280: 8079000C  lwz r3, 0xc(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(12 as u32) ) } as u64;
	// 82524284: 396A0040  addi r11, r10, 0x40
	ctx.r[11].s64 = ctx.r[10].s64 + 64;
	// 82524288: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 8252428C: 393D0020  addi r9, r29, 0x20
	ctx.r[9].s64 = ctx.r[29].s64 + 32;
	// 82524290: 390B0020  addi r8, r11, 0x20
	ctx.r[8].s64 = ctx.r[11].s64 + 32;
	// 82524294: 7F05C378  mr r5, r24
	ctx.r[5].u64 = ctx.r[24].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524428(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82524428 size=132
    let mut pc: u32 = 0x82524428;
    'dispatch: loop {
        match pc {
            0x82524428 => {
    //   block [0x82524428..0x825244AC)
	// 82524428: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252442C: 48010C81  bl 0x825350ac
	ctx.lr = 0x82524430;
	sub_82535080(ctx, base);
	// 82524430: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524434: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82524438: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8252443C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82524440: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82524444: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82524448: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252444C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82524450: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82524454: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 82524458: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 8252445C: 48084C3D  bl 0x825a9098
	ctx.lr = 0x82524460;
	sub_825A9098(ctx, base);
	// 82524460: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82524464: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82524468: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 8252446C: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82524470: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82524474: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82524478: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 8252447C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82524480: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82524484: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82524488: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8252448C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82524490: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82524494: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524498: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252449C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825244A0: 4E800421  bctrl
	ctx.lr = 0x825244A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825244A4: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 825244A8: 48010C54  b 0x825350fc
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825244B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825244B0 size=168
    let mut pc: u32 = 0x825244B0;
    'dispatch: loop {
        match pc {
            0x825244B0 => {
    //   block [0x825244B0..0x82524558)
	// 825244B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825244B4: 48010BFD  bl 0x825350b0
	ctx.lr = 0x825244B8;
	sub_82535080(ctx, base);
	// 825244B8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825244BC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825244C0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825244C4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825244C8: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825244CC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 825244D0: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825244D4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 825244D8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825244DC: 38BC0030  addi r5, r28, 0x30
	ctx.r[5].s64 = ctx.r[28].s64 + 48;
	// 825244E0: 48084BB9  bl 0x825a9098
	ctx.lr = 0x825244E4;
	sub_825A9098(ctx, base);
	// 825244E4: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 825244E8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 825244EC: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 825244F0: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825244F4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 825244F8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825244FC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82524500: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82524504: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82524508: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8252450C: 815C0014  lwz r10, 0x14(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82524510: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82524514: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82524518: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8252451C: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82524520: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82524524: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82524528: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252452C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82524530: 7D4A58AE  lbzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82524534: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82524538: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8252453C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82524540: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82524544: 816B09AC  lwz r11, 0x9ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2476 as u32) ) } as u64;
	// 82524548: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252454C: 4E800421  bctrl
	ctx.lr = 0x82524550;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82524550: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82524554: 48010BAC  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82524558 size=124
    let mut pc: u32 = 0x82524558;
    'dispatch: loop {
        match pc {
            0x82524558 => {
    //   block [0x82524558..0x825245D4)
	// 82524558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252455C: 48010B55  bl 0x825350b0
	ctx.lr = 0x82524560;
	sub_82535080(ctx, base);
	// 82524560: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524564: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82524568: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8252456C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82524570: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82524574: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82524578: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252457C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82524580: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82524584: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 82524588: 48084B11  bl 0x825a9098
	ctx.lr = 0x8252458C;
	sub_825A9098(ctx, base);
	// 8252458C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82524590: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82524594: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82524598: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252459C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 825245A0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825245A4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825245A8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825245AC: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 825245B0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825245B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825245B8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825245BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825245C0: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 825245C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825245C8: 4E800421  bctrl
	ctx.lr = 0x825245CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825245CC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 825245D0: 48010B30  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825245D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825245D8 size=160
    let mut pc: u32 = 0x825245D8;
    'dispatch: loop {
        match pc {
            0x825245D8 => {
    //   block [0x825245D8..0x82524678)
	// 825245D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825245DC: 48010AD9  bl 0x825350b4
	ctx.lr = 0x825245E0;
	sub_82535080(ctx, base);
	// 825245E0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825245E4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825245E8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825245EC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825245F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825245F4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 825245F8: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825245FC: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82524600: 38BC0030  addi r5, r28, 0x30
	ctx.r[5].s64 = ctx.r[28].s64 + 48;
	// 82524604: 48084A95  bl 0x825a9098
	ctx.lr = 0x82524608;
	sub_825A9098(ctx, base);
	// 82524608: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8252460C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82524610: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82524614: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524618: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8252461C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524620: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82524624: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82524628: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8252462C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82524630: 815C0014  lwz r10, 0x14(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82524634: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82524638: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8252463C: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82524640: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82524644: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82524648: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252464C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82524650: 7D4A58AE  lbzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82524654: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82524658: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8252465C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82524660: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82524664: 816B09A8  lwz r11, 0x9a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2472 as u32) ) } as u64;
	// 82524668: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252466C: 4E800421  bctrl
	ctx.lr = 0x82524670;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82524670: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82524674: 48010A90  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82524678 size=124
    let mut pc: u32 = 0x82524678;
    'dispatch: loop {
        match pc {
            0x82524678 => {
    //   block [0x82524678..0x825246F4)
	// 82524678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252467C: 48010A35  bl 0x825350b0
	ctx.lr = 0x82524680;
	sub_82535080(ctx, base);
	// 82524680: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524684: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82524688: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 8252468C: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82524690: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82524694: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82524698: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252469C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 825246A0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825246A4: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 825246A8: 480849F1  bl 0x825a9098
	ctx.lr = 0x825246AC;
	sub_825A9098(ctx, base);
	// 825246AC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 825246B0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 825246B4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 825246B8: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 825246BC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 825246C0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825246C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825246C8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825246CC: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 825246D0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825246D4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825246D8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825246DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825246E0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825246E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825246E8: 4E800421  bctrl
	ctx.lr = 0x825246EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825246EC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 825246F0: 48010A10  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825246F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825246F8 size=160
    let mut pc: u32 = 0x825246F8;
    'dispatch: loop {
        match pc {
            0x825246F8 => {
    //   block [0x825246F8..0x82524798)
	// 825246F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825246FC: 480109B9  bl 0x825350b4
	ctx.lr = 0x82524700;
	sub_82535080(ctx, base);
	// 82524700: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524704: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82524708: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8252470C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82524710: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82524714: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82524718: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252471C: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82524720: 38BC0030  addi r5, r28, 0x30
	ctx.r[5].s64 = ctx.r[28].s64 + 48;
	// 82524724: 48084975  bl 0x825a9098
	ctx.lr = 0x82524728;
	sub_825A9098(ctx, base);
	// 82524728: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 8252472C: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82524730: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82524734: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524738: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 8252473C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524740: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82524744: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82524748: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8252474C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82524750: 815C0014  lwz r10, 0x14(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82524754: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82524758: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 8252475C: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82524760: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82524764: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82524768: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 8252476C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82524770: 7D4A58AE  lbzx r10, r10, r11
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82524774: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82524778: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 8252477C: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82524780: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82524784: 816B09A4  lwz r11, 0x9a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82524788: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252478C: 4E800421  bctrl
	ctx.lr = 0x82524790;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82524790: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82524794: 48010970  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524798(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82524798 size=124
    let mut pc: u32 = 0x82524798;
    'dispatch: loop {
        match pc {
            0x82524798 => {
    //   block [0x82524798..0x82524814)
	// 82524798: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252479C: 48010915  bl 0x825350b0
	ctx.lr = 0x825247A0;
	sub_82535080(ctx, base);
	// 825247A0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825247A4: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 825247A8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825247AC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825247B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825247B4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 825247B8: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825247BC: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 825247C0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 825247C4: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 825247C8: 480848D1  bl 0x825a9098
	ctx.lr = 0x825247CC;
	sub_825A9098(ctx, base);
	// 825247CC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 825247D0: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 825247D4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 825247D8: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 825247DC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 825247E0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 825247E4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825247E8: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825247EC: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 825247F0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825247F4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825247F8: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825247FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524800: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82524804: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82524808: 4E800421  bctrl
	ctx.lr = 0x8252480C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252480C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82524810: 480108F0  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82524818 size=276
    let mut pc: u32 = 0x82524818;
    'dispatch: loop {
        match pc {
            0x82524818 => {
    //   block [0x82524818..0x825248D4)
	// 82524818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252481C: 4801088D  bl 0x825350a8
	ctx.lr = 0x82524820;
	sub_82535080(ctx, base);
	// 82524820: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524824: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524828: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252482C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82524830: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82524834: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82524838: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 8252483C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82524840: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82524844: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82524848: 4BF3F7F1  bl 0x82464038
	ctx.lr = 0x8252484C;
	sub_82464038(ctx, base);
	// 8252484C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82524850: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82524854: 396B5680  addi r11, r11, 0x5680
	ctx.r[11].s64 = ctx.r[11].s64 + 22144;
	// 82524858: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 8252485C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82524860: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82524864: 931F0008  stw r24, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[24].u32 ) };
	// 82524868: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252486C: B15F0004  sth r10, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[10].u16 ) };
	// 82524870: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82524874: 837E0000  lwz r27, 0(r30)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524878: 839E0008  lwz r28, 8(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252487C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82524880: 835B0014  lwz r26, 0x14(r27)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82524884: 4800017D  bl 0x82524a00
	ctx.lr = 0x82524888;
	sub_82524A00(ctx, base);
	// 82524888: 38BB0030  addi r5, r27, 0x30
	ctx.r[5].s64 = ctx.r[27].s64 + 48;
	// 8252488C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82524890: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82524894: 48084805  bl 0x825a9098
	ctx.lr = 0x82524898;
	sub_825A9098(ctx, base);
	// 82524898: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 8252489C: 81390000  lwz r9, 0(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 825248A0: 811D0010  lwz r8, 0x10(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 825248A4: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825248A8: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 825248AC: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 825248B0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825248B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825248B8: 93410050  stw r26, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[26].u32 ) };
	// 825248BC: 390A05A0  addi r8, r10, 0x5a0
	ctx.r[8].s64 = ctx.r[10].s64 + 1440;
	// 825248C0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825248C4: 817A000C  lwz r11, 0xc(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(12 as u32) ) } as u64;
	// 825248C8: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 825248CC: 409A0008  bne cr6, 0x825248d4
	if !ctx.cr[6].eq {
	pc = 0x825248D4; continue 'dispatch;
	}
	// 825248D0: 390A01A0  addi r8, r10, 0x1a0
	ctx.r[8].s64 = ctx.r[10].s64 + 416;
	pc = 0x825248D4; continue 'dispatch;
            }
            0x825248D4 => {
    //   block [0x825248D4..0x8252492C)
	// 825248D4: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825248D8: 7F06C378  mr r6, r24
	ctx.r[6].u64 = ctx.r[24].u64;
	// 825248DC: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 825248E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825248E4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 825248E8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 825248EC: 7D6B48AE  lbzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 825248F0: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 825248F4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825248F8: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825248FC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82524900: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82524904: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82524908: 4E800421  bctrl
	ctx.lr = 0x8252490C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252490C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82524910: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82524914: 396B56C8  addi r11, r11, 0x56c8
	ctx.r[11].s64 = ctx.r[11].s64 + 22216;
	// 82524918: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252491C: 915F000C  stw r10, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82524920: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82524924: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82524928: 480107D0  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524930(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82524930 size=204
    let mut pc: u32 = 0x82524930;
    'dispatch: loop {
        match pc {
            0x82524930 => {
    //   block [0x82524930..0x825249FC)
	// 82524930: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82524934: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82524938: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252493C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82524940: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524944: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82524948: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 8252494C: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82524950: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 82524954: 39084818  addi r8, r8, 0x4818
	ctx.r[8].s64 = ctx.r[8].s64 + 18456;
	// 82524958: 39294B20  addi r9, r9, 0x4b20
	ctx.r[9].s64 = ctx.r[9].s64 + 19232;
	// 8252495C: 394A4B70  addi r10, r10, 0x4b70
	ctx.r[10].s64 = ctx.r[10].s64 + 19312;
	// 82524960: 396B4BC0  addi r11, r11, 0x4bc0
	ctx.r[11].s64 = ctx.r[11].s64 + 19392;
	// 82524964: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82524968: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 8252496C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82524970: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82524974: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82524978: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8252497C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82524980: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82524984: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82524988: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 8252498C: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82524990: 4BFDB3F9  bl 0x824ffd88
	ctx.lr = 0x82524994;
	sub_824FFD88(ctx, base);
	// 82524994: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 82524998: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 8252499C: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 825249A0: 396B44B0  addi r11, r11, 0x44b0
	ctx.r[11].s64 = ctx.r[11].s64 + 17584;
	// 825249A4: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 825249A8: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 825249AC: 39084100  addi r8, r8, 0x4100
	ctx.r[8].s64 = ctx.r[8].s64 + 16640;
	// 825249B0: 392946F8  addi r9, r9, 0x46f8
	ctx.r[9].s64 = ctx.r[9].s64 + 18168;
	// 825249B4: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 825249B8: 394A45D8  addi r10, r10, 0x45d8
	ctx.r[10].s64 = ctx.r[10].s64 + 17880;
	// 825249BC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825249C0: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 825249C4: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 825249C8: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 825249CC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 825249D0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825249D4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825249D8: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 825249DC: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 825249E0: 4BFDB3A9  bl 0x824ffd88
	ctx.lr = 0x825249E4;
	sub_824FFD88(ctx, base);
	// 825249E4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825249E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825249EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825249F0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825249F4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825249F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524A00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82524A00 size=288
    let mut pc: u32 = 0x82524A00;
    'dispatch: loop {
        match pc {
            0x82524A00 => {
    //   block [0x82524A00..0x82524B20)
	// 82524A00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82524A04: 48010699  bl 0x8253509c
	ctx.lr = 0x82524A08;
	sub_82535080(ctx, base);
	// 82524A08: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524A0C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82524A10: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82524A14: 397E0010  addi r11, r30, 0x10
	ctx.r[11].s64 = ctx.r[30].s64 + 16;
	// 82524A18: 395E0020  addi r10, r30, 0x20
	ctx.r[10].s64 = ctx.r[30].s64 + 32;
	// 82524A1C: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82524A20: 393E0030  addi r9, r30, 0x30
	ctx.r[9].s64 = ctx.r[30].s64 + 48;
	// 82524A24: EB3E0000  ld r25, 0(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) };
	// 82524A28: 39010050  addi r8, r1, 0x50
	ctx.r[8].s64 = ctx.r[1].s64 + 80;
	// 82524A2C: EABE0008  ld r21, 8(r30)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) };
	// 82524A30: EB0B0000  ld r24, 0(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82524A34: 3BA10080  addi r29, r1, 0x80
	ctx.r[29].s64 = ctx.r[1].s64 + 128;
	// 82524A38: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82524A3C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82524A40: EAEA0000  ld r23, 0(r10)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) };
	// 82524A44: 3B800010  li r28, 0x10
	ctx.r[28].s64 = 16;
	// 82524A48: E94A0008  ld r10, 8(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[10].u32.wrapping_add(8 as u32) ) };
	// 82524A4C: 3B600020  li r27, 0x20
	ctx.r[27].s64 = 32;
	// 82524A50: EAC90000  ld r22, 0(r9)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) };
	// 82524A54: 3B400030  li r26, 0x30
	ctx.r[26].s64 = 48;
	// 82524A58: FB070000  std r24, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
	// 82524A5C: 389E0040  addi r4, r30, 0x40
	ctx.r[4].s64 = ctx.r[30].s64 + 64;
	// 82524A60: F9670008  std r11, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82524A64: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82524A68: FAE60000  std r23, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82524A6C: 387F0040  addi r3, r31, 0x40
	ctx.r[3].s64 = ctx.r[31].s64 + 64;
	// 82524A70: F9460008  std r10, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[10].u64 ) };
	// 82524A74: 38A00050  li r5, 0x50
	ctx.r[5].s64 = 80;
	// 82524A78: E9290008  ld r9, 8(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) };
	// 82524A7C: FB280000  std r25, 0(r8)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524B20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82524B20 size=76
    let mut pc: u32 = 0x82524B20;
    'dispatch: loop {
        match pc {
            0x82524B20 => {
    //   block [0x82524B20..0x82524B6C)
	// 82524B20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82524B24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82524B28: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524B2C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82524B30: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82524B34: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82524B38: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82524B3C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82524B40: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 82524B44: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82524B48: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82524B4C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82524B50: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82524B54: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82524B58: 4BFFFBA1  bl 0x825246f8
	ctx.lr = 0x82524B5C;
	sub_825246F8(ctx, base);
	// 82524B5C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82524B60: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82524B64: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82524B68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82524B70 size=80
    let mut pc: u32 = 0x82524B70;
    'dispatch: loop {
        match pc {
            0x82524B70 => {
    //   block [0x82524B70..0x82524BC0)
	// 82524B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82524B74: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82524B78: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524B7C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82524B80: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82524B84: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82524B88: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 82524B8C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82524B90: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82524B94: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82524B98: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82524B9C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82524BA0: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82524BA4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82524BA8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82524BAC: 4BFFFA2D  bl 0x825245d8
	ctx.lr = 0x82524BB0;
	sub_825245D8(ctx, base);
	// 82524BB0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82524BB4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82524BB8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82524BBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524BC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82524BC0 size=232
    let mut pc: u32 = 0x82524BC0;
    'dispatch: loop {
        match pc {
            0x82524BC0 => {
    //   block [0x82524BC0..0x82524BEC)
	// 82524BC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82524BC4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82524BC8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82524BCC: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524BD0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82524BD4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82524BD8: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82524BDC: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82524BE0: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82524BE4: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82524BE8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82524BEC; continue 'dispatch;
            }
            0x82524BEC => {
    //   block [0x82524BEC..0x82524CA8)
	// 82524BEC: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82524BF0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82524BF4: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82524BF8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82524BFC: 4200FFF0  bdnz 0x82524bec
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82524BEC; continue 'dispatch;
	}
	// 82524C00: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82524C04: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82524C08: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 82524C0C: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82524C10: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82524C14: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524CA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82524CA8 size=140
    let mut pc: u32 = 0x82524CA8;
    'dispatch: loop {
        match pc {
            0x82524CA8 => {
    //   block [0x82524CA8..0x82524D34)
	// 82524CA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82524CAC: 48010405  bl 0x825350b0
	ctx.lr = 0x82524CB0;
	sub_82535080(ctx, base);
	// 82524CB0: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524CB4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82524CB8: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82524CBC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82524CC0: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82524CC4: 396B4BAC  addi r11, r11, 0x4bac
	ctx.r[11].s64 = ctx.r[11].s64 + 19372;
	// 82524CC8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82524CCC: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524CD0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82524CD4: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82524CD8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82524CDC: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 82524CE0: 9B410054  stb r26, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[26].u8 ) };
	// 82524CE4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82524CE8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82524CEC: 480843AD  bl 0x825a9098
	ctx.lr = 0x82524CF0;
	sub_825A9098(ctx, base);
	// 82524CF0: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82524CF4: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82524CF8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82524CFC: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82524D00: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82524D04: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82524D08: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82524D0C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82524D10: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82524D14: 93410064  stw r26, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 82524D18: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82524D1C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524D20: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82524D24: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82524D28: 4E800421  bctrl
	ctx.lr = 0x82524D2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82524D2C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82524D30: 480103D0  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524D38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82524D38 size=148
    let mut pc: u32 = 0x82524D38;
    'dispatch: loop {
        match pc {
            0x82524D38 => {
    //   block [0x82524D38..0x82524DCC)
	// 82524D38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82524D3C: 48010379  bl 0x825350b4
	ctx.lr = 0x82524D40;
	sub_82535080(ctx, base);
	// 82524D40: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524D44: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82524D48: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82524D4C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82524D50: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82524D54: 396B4B94  addi r11, r11, 0x4b94
	ctx.r[11].s64 = ctx.r[11].s64 + 19348;
	// 82524D58: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82524D5C: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524D60: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82524D64: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82524D68: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82524D6C: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 82524D70: C00A8CB4  lfs f0, -0x734c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82524D74: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82524D78: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82524D7C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82524D80: 48084319  bl 0x825a9098
	ctx.lr = 0x82524D84;
	sub_825A9098(ctx, base);
	// 82524D84: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82524D88: 93E1006C  stw r31, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[31].u32 ) };
	// 82524D8C: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82524D90: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82524D94: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82524D98: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82524D9C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82524DA0: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82524DA4: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82524DA8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82524DAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82524DB0: 91610064  stw r11, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[11].u32 ) };
	// 82524DB4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524DB8: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82524DBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82524DC0: 4E800421  bctrl
	ctx.lr = 0x82524DC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82524DC4: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82524DC8: 4801033C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524DD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82524DD0 size=176
    let mut pc: u32 = 0x82524DD0;
    'dispatch: loop {
        match pc {
            0x82524DD0 => {
    //   block [0x82524DD0..0x82524E50)
	// 82524DD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82524DD4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82524DD8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82524DDC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82524DE0: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82524DE4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524DE8: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82524DEC: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82524DF0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82524DF4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82524DF8: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524DFC: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82524E00: 4BFFF409  bl 0x82524208
	ctx.lr = 0x82524E04;
	sub_82524208(ctx, base);
	// 82524E04: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524E08: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82524E0C: 40980044  bge cr6, 0x82524e50
	if !ctx.cr[6].lt {
	pc = 0x82524E50; continue 'dispatch;
	}
	// 82524E10: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82524E14: 394B08F0  addi r10, r11, 0x8f0
	ctx.r[10].s64 = ctx.r[11].s64 + 2288;
	// 82524E18: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82524E50; continue 'dispatch;
            }
            0x82524E50 => {
    //   block [0x82524E50..0x82524E80)
	// 82524E50: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82524E54: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82524E58: 419A000C  beq cr6, 0x82524e64
	if ctx.cr[6].eq {
	pc = 0x82524E64; continue 'dispatch;
	}
	// 82524E5C: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82524E60: 4BFEE5E1  bl 0x82513440
	ctx.lr = 0x82524E64;
	sub_82513440(ctx, base);
	// 82524E64: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82524E68: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82524E6C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82524E70: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82524E74: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82524E78: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82524E7C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82524E80 size=124
    let mut pc: u32 = 0x82524E80;
    'dispatch: loop {
        match pc {
            0x82524E80 => {
    //   block [0x82524E80..0x82524EFC)
	// 82524E80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82524E84: 4801022D  bl 0x825350b0
	ctx.lr = 0x82524E88;
	sub_82535080(ctx, base);
	// 82524E88: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524E8C: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82524E90: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82524E94: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82524E98: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82524E9C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82524EA0: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524EA4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82524EA8: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82524EAC: 38BE0030  addi r5, r30, 0x30
	ctx.r[5].s64 = ctx.r[30].s64 + 48;
	// 82524EB0: 480841E9  bl 0x825a9098
	ctx.lr = 0x82524EB4;
	sub_825A9098(ctx, base);
	// 82524EB4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82524EB8: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 82524EBC: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82524EC0: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82524EC4: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82524EC8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82524ECC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82524ED0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82524ED4: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82524ED8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82524EDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82524EE0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82524EE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82524EE8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82524EEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82524EF0: 4E800421  bctrl
	ctx.lr = 0x82524EF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82524EF4: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82524EF8: 48010208  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524F00(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82524F00 size=240
    let mut pc: u32 = 0x82524F00;
    'dispatch: loop {
        match pc {
            0x82524F00 => {
    //   block [0x82524F00..0x82524F30)
	// 82524F00: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82524F04: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82524F08: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82524F0C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82524F10: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524F14: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82524F18: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82524F1C: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82524F20: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82524F24: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82524F28: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82524F2C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82524F30; continue 'dispatch;
            }
            0x82524F30 => {
    //   block [0x82524F30..0x82524FF0)
	// 82524F30: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82524F34: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82524F38: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82524F3C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82524F40: 4200FFF0  bdnz 0x82524f30
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82524F30; continue 'dispatch;
	}
	// 82524F44: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82524F48: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82524F4C: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 82524F50: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82524F54: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82524F58: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82524FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82524FF0 size=124
    let mut pc: u32 = 0x82524FF0;
    'dispatch: loop {
        match pc {
            0x82524FF0 => {
    //   block [0x82524FF0..0x82525018)
	// 82524FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82524FF4: 480100C5  bl 0x825350b8
	ctx.lr = 0x82524FF8;
	sub_82535080(ctx, base);
	// 82524FF8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82524FFC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82525000: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82525004: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82525008: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252500C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82525010: 4099003C  ble cr6, 0x8252504c
	if !ctx.cr[6].gt {
	pc = 0x8252504C; continue 'dispatch;
	}
	// 82525014: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82525018; continue 'dispatch;
            }
            0x82525018 => {
    //   block [0x82525018..0x8252504C)
	// 82525018: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252501C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82525020: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82525024: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525028: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252502C: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82525030: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525034: 4E800421  bctrl
	ctx.lr = 0x82525038;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525038: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252503C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82525040: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82525044: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82525048: 4198FFD0  blt cr6, 0x82525018
	if ctx.cr[6].lt {
	pc = 0x82525018; continue 'dispatch;
	}
            }
            0x8252504C => {
    //   block [0x8252504C..0x8252506C)
	// 8252504C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525050: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82525054: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82525058: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252505C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525060: 4E800421  bctrl
	ctx.lr = 0x82525064;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525064: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82525068: 480100A0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82525070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82525070 size=100
    let mut pc: u32 = 0x82525070;
    'dispatch: loop {
        match pc {
            0x82525070 => {
    //   block [0x82525070..0x82525098)
	// 82525070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82525074: 48010045  bl 0x825350b8
	ctx.lr = 0x82525078;
	sub_82535080(ctx, base);
	// 82525078: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252507C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82525080: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82525084: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82525088: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252508C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82525090: 4099003C  ble cr6, 0x825250cc
	if !ctx.cr[6].gt {
	pc = 0x825250CC; continue 'dispatch;
	}
	// 82525094: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82525098; continue 'dispatch;
            }
            0x82525098 => {
    //   block [0x82525098..0x825250CC)
	// 82525098: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252509C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825250A0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 825250A4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825250A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825250AC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 825250B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825250B4: 4E800421  bctrl
	ctx.lr = 0x825250B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825250B8: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 825250BC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 825250C0: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 825250C4: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825250C8: 4198FFD0  blt cr6, 0x82525098
	if ctx.cr[6].lt {
	pc = 0x82525098; continue 'dispatch;
	}
            }
            0x825250CC => {
    //   block [0x825250CC..0x825250D4)
	// 825250CC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825250D0: 48010038  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825250D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825250D8 size=132
    let mut pc: u32 = 0x825250D8;
    'dispatch: loop {
        match pc {
            0x825250D8 => {
    //   block [0x825250D8..0x82525110)
	// 825250D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825250DC: 4800FFDD  bl 0x825350b8
	ctx.lr = 0x825250E0;
	sub_82535080(ctx, base);
	// 825250E0: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 825250E4: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 825250E8: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825250EC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825250F0: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 825250F4: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 825250F8: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825250FC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82525100: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82525104: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82525108: 40990044  ble cr6, 0x8252514c
	if !ctx.cr[6].gt {
	pc = 0x8252514C; continue 'dispatch;
	}
	// 8252510C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82525110; continue 'dispatch;
            }
            0x82525110 => {
    //   block [0x82525110..0x8252514C)
	// 82525110: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525114: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82525118: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8252511C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82525120: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82525124: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525128: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252512C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82525130: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525134: 4E800421  bctrl
	ctx.lr = 0x82525138;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525138: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252513C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82525140: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82525144: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82525148: 4198FFC8  blt cr6, 0x82525110
	if ctx.cr[6].lt {
	pc = 0x82525110; continue 'dispatch;
	}
            }
            0x8252514C => {
    //   block [0x8252514C..0x8252515C)
	// 8252514C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82525150: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82525154: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82525158: 4800FFB0  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82525160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82525160 size=292
    let mut pc: u32 = 0x82525160;
    'dispatch: loop {
        match pc {
            0x82525160 => {
    //   block [0x82525160..0x825251B8)
	// 82525160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82525164: 4800FF41  bl 0x825350a4
	ctx.lr = 0x82525168;
	sub_82535080(ctx, base);
	// 82525168: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252516C: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525170: 3B000014  li r24, 0x14
	ctx.r[24].s64 = 20;
	// 82525174: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82525178: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 8252517C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82525180: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82525184: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82525188: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 8252518C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525190: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525194: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82525198: 40980020  bge cr6, 0x825251b8
	if !ctx.cr[6].lt {
	pc = 0x825251B8; continue 'dispatch;
	}
	// 8252519C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825251A0: 39295700  addi r9, r9, 0x5700
	ctx.r[9].s64 = ctx.r[9].s64 + 22272;
	// 825251A4: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825251A8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825251AC: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 825251B0: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825251B4: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x825251B8; continue 'dispatch;
            }
            0x825251B8 => {
    //   block [0x825251B8..0x825251F0)
	// 825251B8: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825251BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825251C0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825251C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825251C8: 4E800421  bctrl
	ctx.lr = 0x825251CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825251CC: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825251D0: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825251D4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825251D8: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 825251DC: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825251E0: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 825251E4: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825251E8: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825251EC: 41980060  blt cr6, 0x8252524c
	if ctx.cr[6].lt {
	pc = 0x8252524C; continue 'dispatch;
	}
            }
            0x825251F0 => {
    //   block [0x825251F0..0x8252524C)
	// 825251F0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825251F4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 825251F8: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825251FC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82525200: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82525204: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82525208: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252520C: 4E800421  bctrl
	ctx.lr = 0x82525210;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525210: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82525214: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82525218: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 8252521C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525220: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82525224: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82525228: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8252522C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525230: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82525234: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525238: 4E800421  bctrl
	ctx.lr = 0x8252523C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252523C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82525240: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82525244: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82525248: 4098FFA8  bge cr6, 0x825251f0
	if !ctx.cr[6].lt {
	pc = 0x825251F0; continue 'dispatch;
	}
            }
            0x8252524C => {
    //   block [0x8252524C..0x8252527C)
	// 8252524C: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82525250: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525254: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525258: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252525C: 40980020  bge cr6, 0x8252527c
	if !ctx.cr[6].lt {
	pc = 0x8252527C; continue 'dispatch;
	}
	// 82525260: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82525264: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 82525268: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252526C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82525270: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82525274: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82525278: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252527C; continue 'dispatch;
            }
            0x8252527C => {
    //   block [0x8252527C..0x82525284)
	// 8252527C: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82525280: 4800FE74  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82525288(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82525288 size=292
    let mut pc: u32 = 0x82525288;
    'dispatch: loop {
        match pc {
            0x82525288 => {
    //   block [0x82525288..0x825252E0)
	// 82525288: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252528C: 4800FE19  bl 0x825350a4
	ctx.lr = 0x82525290;
	sub_82535080(ctx, base);
	// 82525290: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82525294: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525298: 3B000014  li r24, 0x14
	ctx.r[24].s64 = 20;
	// 8252529C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825252A0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825252A4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 825252A8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 825252AC: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 825252B0: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 825252B4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825252B8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825252BC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825252C0: 40980020  bge cr6, 0x825252e0
	if !ctx.cr[6].lt {
	pc = 0x825252E0; continue 'dispatch;
	}
	// 825252C4: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825252C8: 39295700  addi r9, r9, 0x5700
	ctx.r[9].s64 = ctx.r[9].s64 + 22272;
	// 825252CC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825252D0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825252D4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 825252D8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825252DC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x825252E0; continue 'dispatch;
            }
            0x825252E0 => {
    //   block [0x825252E0..0x82525318)
	// 825252E0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825252E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825252E8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825252EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825252F0: 4E800421  bctrl
	ctx.lr = 0x825252F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825252F4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825252F8: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825252FC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82525300: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 82525304: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525308: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8252530C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82525310: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82525314: 41980060  blt cr6, 0x82525374
	if ctx.cr[6].lt {
	pc = 0x82525374; continue 'dispatch;
	}
            }
            0x82525318 => {
    //   block [0x82525318..0x82525374)
	// 82525318: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252531C: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82525320: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525324: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82525328: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8252532C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82525330: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525334: 4E800421  bctrl
	ctx.lr = 0x82525338;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525338: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 8252533C: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82525340: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82525344: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525348: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 8252534C: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82525350: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82525354: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525358: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252535C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525360: 4E800421  bctrl
	ctx.lr = 0x82525364;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525364: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82525368: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 8252536C: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82525370: 4098FFA8  bge cr6, 0x82525318
	if !ctx.cr[6].lt {
	pc = 0x82525318; continue 'dispatch;
	}
            }
            0x82525374 => {
    //   block [0x82525374..0x825253A4)
	// 82525374: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82525378: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252537C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525380: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82525384: 40980020  bge cr6, 0x825253a4
	if !ctx.cr[6].lt {
	pc = 0x825253A4; continue 'dispatch;
	}
	// 82525388: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 8252538C: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 82525390: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82525394: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82525398: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8252539C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825253A0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x825253A4; continue 'dispatch;
            }
            0x825253A4 => {
    //   block [0x825253A4..0x825253AC)
	// 825253A4: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 825253A8: 4800FD4C  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825253B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825253B0 size=408
    let mut pc: u32 = 0x825253B0;
    'dispatch: loop {
        match pc {
            0x825253B0 => {
    //   block [0x825253B0..0x82525404)
	// 825253B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825253B4: 4800FCF1  bl 0x825350a4
	ctx.lr = 0x825253B8;
	sub_82535080(ctx, base);
	// 825253B8: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825253BC: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825253C0: 3B000014  li r24, 0x14
	ctx.r[24].s64 = 20;
	// 825253C4: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 825253C8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 825253CC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825253D0: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 825253D4: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 825253D8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825253DC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825253E0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825253E4: 40980020  bge cr6, 0x82525404
	if !ctx.cr[6].lt {
	pc = 0x82525404; continue 'dispatch;
	}
	// 825253E8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825253EC: 39295700  addi r9, r9, 0x5700
	ctx.r[9].s64 = ctx.r[9].s64 + 22272;
	// 825253F0: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825253F4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825253F8: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 825253FC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82525400: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82525404; continue 'dispatch;
            }
            0x82525404 => {
    //   block [0x82525404..0x8252544C)
	// 82525404: 807A0000  lwz r3, 0(r26)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525408: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252540C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82525410: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525414: 4E800421  bctrl
	ctx.lr = 0x82525418;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525418: 817A0008  lwz r11, 8(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252541C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82525420: 9341006C  stw r26, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[26].u32 ) };
	// 82525424: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82525428: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252542C: 836B000C  lwz r27, 0xc(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525430: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525434: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82525438: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252543C: 4E800421  bctrl
	ctx.lr = 0x82525440;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525440: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82525444: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82525448: 419A00C8  beq cr6, 0x82525510
	if ctx.cr[6].eq {
	pc = 0x82525510; continue 'dispatch;
	}
            }
            0x8252544C => {
    //   block [0x8252544C..0x825254EC)
	// 8252544C: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82525450: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82525454: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82525458: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 8252545C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82525460: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82525464: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525468: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8252546C: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525470: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525474: 4E800421  bctrl
	ctx.lr = 0x82525478;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525478: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252547C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82525480: 419A006C  beq cr6, 0x825254ec
	if ctx.cr[6].eq {
	pc = 0x825254EC; continue 'dispatch;
	}
	// 82525484: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525488: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 8252548C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82525490: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82525494: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82525498: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252549C: 4E800421  bctrl
	ctx.lr = 0x825254A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825254A0: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 825254A4: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 825254A8: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 825254AC: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 825254B0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825254B4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825254B8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 825254BC: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 825254C0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825254C4: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825254C8: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 825254CC: 7D4AD8AE  lbzx r10, r10, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 825254D0: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 825254D4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 825254D8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825254DC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 825254E0: 816B09A8  lwz r11, 0x9a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2472 as u32) ) } as u64;
	// 825254E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825254E8: 4E800421  bctrl
	ctx.lr = 0x825254EC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x825254EC => {
    //   block [0x825254EC..0x82525510)
	// 825254EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825254F0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825254F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825254F8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825254FC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525500: 4E800421  bctrl
	ctx.lr = 0x82525504;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525504: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82525508: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 8252550C: 409AFF40  bne cr6, 0x8252544c
	if !ctx.cr[6].eq {
	pc = 0x8252544C; continue 'dispatch;
	}
            }
            0x82525510 => {
    //   block [0x82525510..0x82525540)
	// 82525510: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82525514: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525518: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252551C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82525520: 40980020  bge cr6, 0x82525540
	if !ctx.cr[6].lt {
	pc = 0x82525540; continue 'dispatch;
	}
	// 82525524: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82525528: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 8252552C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82525530: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82525534: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82525538: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252553C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82525540; continue 'dispatch;
            }
            0x82525540 => {
    //   block [0x82525540..0x82525548)
	// 82525540: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82525544: 4800FBB0  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82525548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82525548 size=300
    let mut pc: u32 = 0x82525548;
    'dispatch: loop {
        match pc {
            0x82525548 => {
    //   block [0x82525548..0x825255A4)
	// 82525548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252554C: 4800FB55  bl 0x825350a0
	ctx.lr = 0x82525550;
	sub_82535080(ctx, base);
	// 82525550: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82525554: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525558: 3AE00014  li r23, 0x14
	ctx.r[23].s64 = 20;
	// 8252555C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82525560: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82525564: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82525568: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8252556C: 7D77B02E  lwzx r11, r23, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82525570: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82525574: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82525578: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252557C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525580: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82525584: 40980020  bge cr6, 0x825255a4
	if !ctx.cr[6].lt {
	pc = 0x825255A4; continue 'dispatch;
	}
	// 82525588: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252558C: 39295700  addi r9, r9, 0x5700
	ctx.r[9].s64 = ctx.r[9].s64 + 22272;
	// 82525590: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82525594: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82525598: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 8252559C: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825255A0: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x825255A4; continue 'dispatch;
            }
            0x825255A4 => {
    //   block [0x825255A4..0x825255DC)
	// 825255A4: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825255A8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825255AC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825255B0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825255B4: 4E800421  bctrl
	ctx.lr = 0x825255B8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825255B8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825255BC: 815D0008  lwz r10, 8(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825255C0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825255C4: 3BCBFFFF  addi r30, r11, -1
	ctx.r[30].s64 = ctx.r[11].s64 + -1;
	// 825255C8: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825255CC: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 825255D0: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 825255D4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825255D8: 41980064  blt cr6, 0x8252563c
	if ctx.cr[6].lt {
	pc = 0x8252563C; continue 'dispatch;
	}
            }
            0x825255DC => {
    //   block [0x825255DC..0x8252563C)
	// 825255DC: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825255E0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 825255E4: 83BF0000  lwz r29, 0(r31)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825255E8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 825255EC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 825255F0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825255F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825255F8: 4E800421  bctrl
	ctx.lr = 0x825255FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825255FC: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82525600: 93A10054  stw r29, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[29].u32 ) };
	// 82525604: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82525608: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252560C: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82525610: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82525614: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82525618: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8252561C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525620: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82525624: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525628: 4E800421  bctrl
	ctx.lr = 0x8252562C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252562C: 3BDEFFFF  addi r30, r30, -1
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	// 82525630: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82525634: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82525638: 4098FFA4  bge cr6, 0x825255dc
	if !ctx.cr[6].lt {
	pc = 0x825255DC; continue 'dispatch;
	}
            }
            0x8252563C => {
    //   block [0x8252563C..0x8252566C)
	// 8252563C: 7D57B02E  lwzx r10, r23, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82525640: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525644: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525648: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252564C: 40980020  bge cr6, 0x8252566c
	if !ctx.cr[6].lt {
	pc = 0x8252566C; continue 'dispatch;
	}
	// 82525650: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82525654: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 82525658: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252565C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82525660: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82525664: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82525668: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252566C; continue 'dispatch;
            }
            0x8252566C => {
    //   block [0x8252566C..0x82525674)
	// 8252566C: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82525670: 4800FA80  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82525678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82525678 size=416
    let mut pc: u32 = 0x82525678;
    'dispatch: loop {
        match pc {
            0x82525678 => {
    //   block [0x82525678..0x825256D0)
	// 82525678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252567C: 4800FA25  bl 0x825350a0
	ctx.lr = 0x82525680;
	sub_82535080(ctx, base);
	// 82525680: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82525684: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525688: 3AE00014  li r23, 0x14
	ctx.r[23].s64 = 20;
	// 8252568C: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82525690: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82525694: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82525698: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 8252569C: 7D77B02E  lwzx r11, r23, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 825256A0: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 825256A4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 825256A8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825256AC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825256B0: 40980020  bge cr6, 0x825256d0
	if !ctx.cr[6].lt {
	pc = 0x825256D0; continue 'dispatch;
	}
	// 825256B4: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825256B8: 39295700  addi r9, r9, 0x5700
	ctx.r[9].s64 = ctx.r[9].s64 + 22272;
	// 825256BC: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825256C0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825256C4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 825256C8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825256CC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x825256D0; continue 'dispatch;
            }
            0x825256D0 => {
    //   block [0x825256D0..0x82525718)
	// 825256D0: 80780000  lwz r3, 0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 825256D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825256D8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825256DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825256E0: 4E800421  bctrl
	ctx.lr = 0x825256E4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825256E4: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 825256E8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825256EC: 9301006C  stw r24, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[24].u32 ) };
	// 825256F0: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825256F4: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825256F8: 836B000C  lwz r27, 0xc(r11)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825256FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525700: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82525704: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525708: 4E800421  bctrl
	ctx.lr = 0x8252570C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252570C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82525710: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82525714: 419A00CC  beq cr6, 0x825257e0
	if ctx.cr[6].eq {
	pc = 0x825257E0; continue 'dispatch;
	}
            }
            0x82525718 => {
    //   block [0x82525718..0x825257BC)
	// 82525718: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252571C: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82525720: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82525724: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82525728: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 8252572C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82525730: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525734: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82525738: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252573C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525740: 4E800421  bctrl
	ctx.lr = 0x82525744;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525744: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525748: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 8252574C: 419A0070  beq cr6, 0x825257bc
	if ctx.cr[6].eq {
	pc = 0x825257BC; continue 'dispatch;
	}
	// 82525750: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525754: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82525758: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8252575C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82525760: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82525764: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525768: 4E800421  bctrl
	ctx.lr = 0x8252576C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252576C: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82525770: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82525774: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82525778: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252577C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82525780: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525784: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82525788: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 8252578C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82525790: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82525794: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82525798: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8252579C: 7D4AD8AE  lbzx r10, r10, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 825257A0: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 825257A4: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 825257A8: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825257AC: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 825257B0: 816B09AC  lwz r11, 0x9ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2476 as u32) ) } as u64;
	// 825257B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825257B8: 4E800421  bctrl
	ctx.lr = 0x825257BC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x825257BC => {
    //   block [0x825257BC..0x825257E0)
	// 825257BC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825257C0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825257C4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825257C8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825257CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825257D0: 4E800421  bctrl
	ctx.lr = 0x825257D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825257D4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825257D8: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 825257DC: 409AFF3C  bne cr6, 0x82525718
	if !ctx.cr[6].eq {
	pc = 0x82525718; continue 'dispatch;
	}
            }
            0x825257E0 => {
    //   block [0x825257E0..0x82525810)
	// 825257E0: 7D57B02E  lwzx r10, r23, r22
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 825257E4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825257E8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825257EC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825257F0: 40980020  bge cr6, 0x82525810
	if !ctx.cr[6].lt {
	pc = 0x82525810; continue 'dispatch;
	}
	// 825257F4: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 825257F8: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 825257FC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82525800: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82525804: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82525808: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252580C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82525810; continue 'dispatch;
            }
            0x82525810 => {
    //   block [0x82525810..0x82525818)
	// 82525810: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82525814: 4800F8DC  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82525818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82525818 size=304
    let mut pc: u32 = 0x82525818;
    'dispatch: loop {
        match pc {
            0x82525818 => {
    //   block [0x82525818..0x82525870)
	// 82525818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252581C: 4800F889  bl 0x825350a4
	ctx.lr = 0x82525820;
	sub_82535080(ctx, base);
	// 82525820: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82525824: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525828: 3B000014  li r24, 0x14
	ctx.r[24].s64 = 20;
	// 8252582C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82525830: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82525834: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82525838: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 8252583C: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82525840: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82525844: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525848: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252584C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82525850: 40980020  bge cr6, 0x82525870
	if !ctx.cr[6].lt {
	pc = 0x82525870; continue 'dispatch;
	}
	// 82525854: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82525858: 39295700  addi r9, r9, 0x5700
	ctx.r[9].s64 = ctx.r[9].s64 + 22272;
	// 8252585C: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82525860: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82525864: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82525868: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252586C: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82525870; continue 'dispatch;
            }
            0x82525870 => {
    //   block [0x82525870..0x825258A8)
	// 82525870: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525874: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525878: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 8252587C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525880: 4E800421  bctrl
	ctx.lr = 0x82525884;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525884: 817E0010  lwz r11, 0x10(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82525888: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252588C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82525890: 3B6BFFFF  addi r27, r11, -1
	ctx.r[27].s64 = ctx.r[11].s64 + -1;
	// 82525894: 83DE000C  lwz r30, 0xc(r30)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525898: 93E1005C  stw r31, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[31].u32 ) };
	// 8252589C: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 825258A0: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825258A4: 4198006C  blt cr6, 0x82525910
	if ctx.cr[6].lt {
	pc = 0x82525910; continue 'dispatch;
	}
            }
            0x825258A8 => {
    //   block [0x825258A8..0x82525910)
	// 825258A8: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825258AC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 825258B0: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825258B4: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825258B8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 825258BC: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825258C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825258C4: 4E800421  bctrl
	ctx.lr = 0x825258C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825258C8: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 825258CC: 93E10054  stw r31, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[31].u32 ) };
	// 825258D0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 825258D4: 807E0004  lwz r3, 4(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 825258D8: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 825258DC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 825258E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825258E4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825258E8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825258EC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825258F0: 4E800421  bctrl
	ctx.lr = 0x825258F4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825258F4: 897C0004  lbz r11, 4(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 825258F8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825258FC: 409A0014  bne cr6, 0x82525910
	if !ctx.cr[6].eq {
	pc = 0x82525910; continue 'dispatch;
	}
	// 82525900: 3B7BFFFF  addi r27, r27, -1
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	// 82525904: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82525908: 2F1B0000  cmpwi cr6, r27, 0
	ctx.cr[6].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 8252590C: 4098FF9C  bge cr6, 0x825258a8
	if !ctx.cr[6].lt {
	pc = 0x825258A8; continue 'dispatch;
	}
            }
            0x82525910 => {
    //   block [0x82525910..0x82525940)
	// 82525910: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82525914: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525918: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252591C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82525920: 40980020  bge cr6, 0x82525940
	if !ctx.cr[6].lt {
	pc = 0x82525940; continue 'dispatch;
	}
	// 82525924: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82525928: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 8252592C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82525930: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82525934: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82525938: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252593C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82525940; continue 'dispatch;
            }
            0x82525940 => {
    //   block [0x82525940..0x82525948)
	// 82525940: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82525944: 4800F7B0  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82525948(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82525948 size=420
    let mut pc: u32 = 0x82525948;
    'dispatch: loop {
        match pc {
            0x82525948 => {
    //   block [0x82525948..0x8252599C)
	// 82525948: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252594C: 4800F759  bl 0x825350a4
	ctx.lr = 0x82525950;
	sub_82535080(ctx, base);
	// 82525950: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82525954: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525958: 3B000014  li r24, 0x14
	ctx.r[24].s64 = 20;
	// 8252595C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82525960: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82525964: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82525968: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 8252596C: 7D78B82E  lwzx r11, r24, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82525970: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525974: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525978: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252597C: 40980020  bge cr6, 0x8252599c
	if !ctx.cr[6].lt {
	pc = 0x8252599C; continue 'dispatch;
	}
	// 82525980: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82525984: 39295700  addi r9, r9, 0x5700
	ctx.r[9].s64 = ctx.r[9].s64 + 22272;
	// 82525988: 912A0000  stw r9, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252598C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82525990: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82525994: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82525998: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252599C; continue 'dispatch;
            }
            0x8252599C => {
    //   block [0x8252599C..0x825259E4)
	// 8252599C: 80790000  lwz r3, 0(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 825259A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825259A4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825259A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825259AC: 4E800421  bctrl
	ctx.lr = 0x825259B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825259B0: 81790008  lwz r11, 8(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 825259B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825259B8: 9321006C  stw r25, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[25].u32 ) };
	// 825259BC: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825259C0: 817C0000  lwz r11, 0(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825259C4: 834B000C  lwz r26, 0xc(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825259C8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825259CC: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 825259D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825259D4: 4E800421  bctrl
	ctx.lr = 0x825259D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825259D8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825259DC: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 825259E0: 419A00D4  beq cr6, 0x82525ab4
	if ctx.cr[6].eq {
	pc = 0x82525AB4; continue 'dispatch;
	}
            }
            0x825259E4 => {
    //   block [0x825259E4..0x82525A90)
	// 825259E4: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825259E8: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 825259EC: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 825259F0: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 825259F4: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 825259F8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825259FC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525A00: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82525A04: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525A08: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525A0C: 4E800421  bctrl
	ctx.lr = 0x82525A10;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525A10: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525A14: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82525A18: 419A0078  beq cr6, 0x82525a90
	if ctx.cr[6].eq {
	pc = 0x82525A90; continue 'dispatch;
	}
	// 82525A1C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525A20: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82525A24: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82525A28: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82525A2C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82525A30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525A34: 4E800421  bctrl
	ctx.lr = 0x82525A38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525A38: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82525A3C: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82525A40: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82525A44: 8143000C  lwz r10, 0xc(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525A48: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82525A4C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525A50: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82525A54: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82525A58: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82525A5C: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82525A60: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82525A64: 7D4AD0AE  lbzx r10, r10, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82525A68: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82525A6C: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82525A70: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82525A74: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82525A78: 816B09A4  lwz r11, 0x9a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82525A7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525A80: 4E800421  bctrl
	ctx.lr = 0x82525A84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525A84: 897B0004  lbz r11, 4(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525A88: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82525A8C: 409A0028  bne cr6, 0x82525ab4
	if !ctx.cr[6].eq {
	pc = 0x82525AB4; continue 'dispatch;
	}
            }
            0x82525A90 => {
    //   block [0x82525A90..0x82525AB4)
	// 82525A90: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525A94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82525A98: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82525A9C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525AA0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525AA4: 4E800421  bctrl
	ctx.lr = 0x82525AA8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525AA8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82525AAC: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82525AB0: 409AFF34  bne cr6, 0x825259e4
	if !ctx.cr[6].eq {
	pc = 0x825259E4; continue 'dispatch;
	}
            }
            0x82525AB4 => {
    //   block [0x82525AB4..0x82525AE4)
	// 82525AB4: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82525AB8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525ABC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525AC0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82525AC4: 40980020  bge cr6, 0x82525ae4
	if !ctx.cr[6].lt {
	pc = 0x82525AE4; continue 'dispatch;
	}
	// 82525AC8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82525ACC: 392974DC  addi r9, r9, 0x74dc
	ctx.r[9].s64 = ctx.r[9].s64 + 29916;
	// 82525AD0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82525AD4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82525AD8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82525ADC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82525AE0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82525AE4; continue 'dispatch;
            }
            0x82525AE4 => {
    //   block [0x82525AE4..0x82525AEC)
	// 82525AE4: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82525AE8: 4800F60C  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82525AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82525AF0 size=632
    let mut pc: u32 = 0x82525AF0;
    'dispatch: loop {
        match pc {
            0x82525AF0 => {
    //   block [0x82525AF0..0x82525B6C)
	// 82525AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82525AF4: 4800F5AD  bl 0x825350a0
	ctx.lr = 0x82525AF8;
	sub_82535080(ctx, base);
	// 82525AF8: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82525AFC: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82525B00: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82525B04: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82525B08: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82525B0C: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82525B10: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82525B14: 80770000  lwz r3, 0(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525B18: 92E1006C  stw r23, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[23].u32 ) };
	// 82525B1C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82525B20: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525B24: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82525B28: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525B2C: 4E800421  bctrl
	ctx.lr = 0x82525B30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525B30: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82525B34: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525B38: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525B3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525B40: 4E800421  bctrl
	ctx.lr = 0x82525B44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525B44: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525B48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82525B4C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82525B50: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82525B54: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525B58: 4E800421  bctrl
	ctx.lr = 0x82525B5C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525B5C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82525B60: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82525B64: 409901FC  ble cr6, 0x82525d60
	if !ctx.cr[6].gt {
	pc = 0x82525D60; continue 'dispatch;
	}
	// 82525B68: 7FF6FB78  mr r22, r31
	ctx.r[22].u64 = ctx.r[31].u64;
            }
            0x82525B6C => {
    //   block [0x82525B6C..0x82525B80)
	// 82525B6C: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82525B70: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82525B74: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82525B78: 40990024  ble cr6, 0x82525b9c
	if !ctx.cr[6].gt {
	pc = 0x82525B9C; continue 'dispatch;
	}
	// 82525B7C: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	pc = 0x82525B80; continue 'dispatch;
            }
            0x82525B80 => {
    //   block [0x82525B80..0x82525B9C)
	// 82525B80: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525B84: 7F09D040  cmplw cr6, r9, r26
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82525B88: 419A0018  beq cr6, 0x82525ba0
	if ctx.cr[6].eq {
	pc = 0x82525BA0; continue 'dispatch;
	}
	// 82525B8C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82525B90: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82525B94: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82525B98: 4198FFE8  blt cr6, 0x82525b80
	if ctx.cr[6].lt {
	pc = 0x82525B80; continue 'dispatch;
	}
	pc = 0x82525B9C; continue 'dispatch;
            }
            0x82525B9C => {
    //   block [0x82525B9C..0x82525BA0)
	// 82525B9C: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	pc = 0x82525BA0; continue 'dispatch;
            }
            0x82525BA0 => {
    //   block [0x82525BA0..0x82525C28)
	// 82525BA0: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82525BA4: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82525BA8: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82525BAC: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 82525BB0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82525BB4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82525BB8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525BBC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82525BC0: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525BC4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525BC8: 4E800421  bctrl
	ctx.lr = 0x82525BCC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525BCC: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525BD0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82525BD4: 419A010C  beq cr6, 0x82525ce0
	if ctx.cr[6].eq {
	pc = 0x82525CE0; continue 'dispatch;
	}
	// 82525BD8: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525BDC: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82525BE0: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82525BE4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82525BE8: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82525BEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525BF0: 4E800421  bctrl
	ctx.lr = 0x82525BF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525BF4: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82525BF8: 93410064  stw r26, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 82525BFC: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82525C00: 409A00AC  bne cr6, 0x82525cac
	if !ctx.cr[6].eq {
	pc = 0x82525CAC; continue 'dispatch;
	}
	// 82525C04: 3BFD000C  addi r31, r29, 0xc
	ctx.r[31].s64 = ctx.r[29].s64 + 12;
	// 82525C08: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82525C0C: 815F0004  lwz r10, 4(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525C10: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82525C14: 7F0A5800  cmpw cr6, r10, r11
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82525C18: 409A0010  bne cr6, 0x82525c28
	if !ctx.cr[6].eq {
	pc = 0x82525C28; continue 'dispatch;
	}
	// 82525C1C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82525C20: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82525C24: 4BF4872D  bl 0x8246e350
	ctx.lr = 0x82525C28;
	sub_8246E350(ctx, base);
            }
            0x82525C28 => {
    //   block [0x82525C28..0x82525C70)
	// 82525C28: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525C2C: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525C30: 556A1838  slwi r10, r11, 3
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82525C34: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82525C38: 7FCA4A14  add r30, r10, r9
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82525C3C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82525C40: 935E0000  stw r26, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[26].u32 ) };
	// 82525C44: 817C0010  lwz r11, 0x10(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82525C48: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525C4C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82525C50: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82525C54: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525C58: 80DD0008  lwz r6, 8(r29)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82525C5C: 390A05A0  addi r8, r10, 0x5a0
	ctx.r[8].s64 = ctx.r[10].s64 + 1440;
	// 82525C60: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525C64: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525C68: 409A0008  bne cr6, 0x82525c70
	if !ctx.cr[6].eq {
	pc = 0x82525C70; continue 'dispatch;
	}
	// 82525C6C: 390A01A0  addi r8, r10, 0x1a0
	ctx.r[8].s64 = ctx.r[10].s64 + 416;
	pc = 0x82525C70; continue 'dispatch;
            }
            0x82525C70 => {
    //   block [0x82525C70..0x82525CAC)
	// 82525C70: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82525C74: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82525C78: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82525C7C: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82525C80: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82525C84: 7D6B48AE  lbzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82525C88: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82525C8C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82525C90: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82525C94: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82525C98: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82525C9C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525CA0: 4E800421  bctrl
	ctx.lr = 0x82525CA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525CA4: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82525CA8: 48000090  b 0x82525d38
	pc = 0x82525D38; continue 'dispatch;
            }
            0x82525CAC => {
    //   block [0x82525CAC..0x82525CE0)
	// 82525CAC: 815D000C  lwz r10, 0xc(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525CB0: 57EB1838  slwi r11, r31, 3
	ctx.r[11].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82525CB4: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82525CB8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82525CBC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82525CC0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82525CC4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82525CC8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525CCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525CD0: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82525CD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525CD8: 4E800421  bctrl
	ctx.lr = 0x82525CDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525CDC: 4800005C  b 0x82525d38
	pc = 0x82525D38; continue 'dispatch;
            }
            0x82525CE0 => {
    //   block [0x82525CE0..0x82525D38)
	// 82525CE0: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82525CE4: 419A0054  beq cr6, 0x82525d38
	if ctx.cr[6].eq {
	pc = 0x82525D38; continue 'dispatch;
	}
	// 82525CE8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525CEC: 57FF1838  slwi r31, r31, 3
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82525CF0: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82525CF4: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82525CF8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525CFC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525D00: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82525D04: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525D08: 4E800421  bctrl
	ctx.lr = 0x82525D0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525D0C: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82525D10: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525D14: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82525D18: 7D0BFA14  add r8, r11, r31
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82525D1C: 55491838  slwi r9, r10, 3
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82525D20: 7D695A14  add r11, r9, r11
	ctx.r[11].u64 = ctx.r[9].u64 + ctx.r[11].u64;
	// 82525D24: 915D0010  stw r10, 0x10(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82525D28: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525D2C: 91480000  stw r10, 0(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82525D30: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525D34: 91680004  stw r11, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
            }
            0x82525D38 => {
    //   block [0x82525D38..0x82525D60)
	// 82525D38: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525D3C: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82525D40: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82525D44: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525D48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525D4C: 4E800421  bctrl
	ctx.lr = 0x82525D50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525D50: 3AD6FFFF  addi r22, r22, -1
	ctx.r[22].s64 = ctx.r[22].s64 + -1;
	// 82525D54: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82525D58: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82525D5C: 409AFE10  bne cr6, 0x82525b6c
	if !ctx.cr[6].eq {
	pc = 0x82525B6C; continue 'dispatch;
	}
            }
            0x82525D60 => {
    //   block [0x82525D60..0x82525D68)
	// 82525D60: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82525D64: 4800F38C  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82525D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82525D68 size=500
    let mut pc: u32 = 0x82525D68;
    'dispatch: loop {
        match pc {
            0x82525D68 => {
    //   block [0x82525D68..0x82525E18)
	// 82525D68: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82525D6C: 4800F335  bl 0x825350a0
	ctx.lr = 0x82525D70;
	sub_82535080(ctx, base);
	// 82525D70: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82525D74: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82525D78: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82525D7C: 396B5718  addi r11, r11, 0x5718
	ctx.r[11].s64 = ctx.r[11].s64 + 22296;
	// 82525D80: 3BB7000C  addi r29, r23, 0xc
	ctx.r[29].s64 = ctx.r[23].s64 + 12;
	// 82525D84: 3D008000  lis r8, -0x8000
	ctx.r[8].s64 = -2147483648;
	// 82525D88: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82525D8C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82525D90: 91770000  stw r11, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82525D94: 397D000C  addi r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 + 12;
	// 82525D98: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82525D9C: 61080004  ori r8, r8, 4
	ctx.r[8].u64 = ctx.r[8].u64 | 4;
	// 82525DA0: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82525DA4: 93370008  stw r25, 8(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82525DA8: B1570006  sth r10, 6(r23)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[23].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82525DAC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82525DB0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82525DB4: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82525DB8: 913D0004  stw r9, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82525DBC: 911D0008  stw r8, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82525DC0: 81780008  lwz r11, 8(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82525DC4: 80780000  lwz r3, 0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525DC8: 9301006C  stw r24, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[24].u32 ) };
	// 82525DCC: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82525DD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525DD4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82525DD8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525DDC: 4E800421  bctrl
	ctx.lr = 0x82525DE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525DE0: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82525DE4: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525DE8: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525DEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525DF0: 4E800421  bctrl
	ctx.lr = 0x82525DF4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525DF4: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82525DF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82525DFC: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82525E00: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82525E04: 40980024  bge cr6, 0x82525e28
	if !ctx.cr[6].lt {
	pc = 0x82525E28; continue 'dispatch;
	}
	// 82525E08: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82525E0C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82525E10: 41980008  blt cr6, 0x82525e18
	if ctx.cr[6].lt {
	pc = 0x82525E18; continue 'dispatch;
	}
	// 82525E14: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
            }
            0x82525E18 => {
    //   block [0x82525E18..0x82525E28)
	// 82525E18: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82525E1C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82525E20: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82525E24: 4BF484A5  bl 0x8246e2c8
	ctx.lr = 0x82525E28;
	sub_8246E2C8(ctx, base);
	pc = 0x82525E28; continue 'dispatch;
            }
            0x82525E28 => {
    //   block [0x82525E28..0x82525E4C)
	// 82525E28: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525E2C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82525E30: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82525E34: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525E38: 4E800421  bctrl
	ctx.lr = 0x82525E3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525E3C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82525E40: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82525E44: 4099010C  ble cr6, 0x82525f50
	if !ctx.cr[6].gt {
	pc = 0x82525F50; continue 'dispatch;
	}
	// 82525E48: 7FD6F378  mr r22, r30
	ctx.r[22].u64 = ctx.r[30].u64;
            }
            0x82525E4C => {
    //   block [0x82525E4C..0x82525EE8)
	// 82525E4C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525E50: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82525E54: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82525E58: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82525E5C: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82525E60: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525E64: 4E800421  bctrl
	ctx.lr = 0x82525E68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525E68: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82525E6C: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82525E70: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82525E74: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82525E78: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82525E7C: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82525E80: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82525E84: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525E88: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82525E8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82525E90: 816B0004  lwz r11, 4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525E94: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525E98: 4E800421  bctrl
	ctx.lr = 0x82525E9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525E9C: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525EA0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82525EA4: 419A0084  beq cr6, 0x82525f28
	if ctx.cr[6].eq {
	pc = 0x82525F28; continue 'dispatch;
	}
	// 82525EA8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82525EAC: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525EB0: 392B0001  addi r9, r11, 1
	ctx.r[9].s64 = ctx.r[11].s64 + 1;
	// 82525EB4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82525EB8: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82525EBC: 81610060  lwz r11, 0x60(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82525EC0: 913D0004  stw r9, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82525EC4: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525EC8: 811F0010  lwz r8, 0x10(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82525ECC: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525ED0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82525ED4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525ED8: 390A05A0  addi r8, r10, 0x5a0
	ctx.r[8].s64 = ctx.r[10].s64 + 1440;
	// 82525EDC: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525EE0: 409A0008  bne cr6, 0x82525ee8
	if !ctx.cr[6].eq {
	pc = 0x82525EE8; continue 'dispatch;
	}
	// 82525EE4: 390A01A0  addi r8, r10, 0x1a0
	ctx.r[8].s64 = ctx.r[10].s64 + 416;
            }
            0x82525EE8 => {
    //   block [0x82525EE8..0x82525F28)
	// 82525EE8: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82525EEC: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82525EF0: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82525EF4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82525EF8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82525EFC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82525F00: 7D6B48AE  lbzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82525F04: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82525F08: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82525F0C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82525F10: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82525F14: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82525F18: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525F1C: 4E800421  bctrl
	ctx.lr = 0x82525F20;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525F20: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82525F24: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
            }
            0x82525F28 => {
    //   block [0x82525F28..0x82525F50)
	// 82525F28: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525F2C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82525F30: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82525F34: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82525F38: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82525F3C: 4E800421  bctrl
	ctx.lr = 0x82525F40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82525F40: 3AD6FFFF  addi r22, r22, -1
	ctx.r[22].s64 = ctx.r[22].s64 + -1;
	// 82525F44: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82525F48: 2B160000  cmplwi cr6, r22, 0
	ctx.cr[6].compare_u32(ctx.r[22].u32, 0 as u32, &mut ctx.xer);
	// 82525F4C: 409AFF00  bne cr6, 0x82525e4c
	if !ctx.cr[6].eq {
	pc = 0x82525E4C; continue 'dispatch;
	}
            }
            0x82525F50 => {
    //   block [0x82525F50..0x82525F5C)
	// 82525F50: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82525F54: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82525F58: 4800F198  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82525F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82525F60 size=88
    let mut pc: u32 = 0x82525F60;
    'dispatch: loop {
        match pc {
            0x82525F60 => {
    //   block [0x82525F60..0x82525FB8)
	// 82525F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82525F64: 4800F155  bl 0x825350b8
	ctx.lr = 0x82525F68;
	sub_82535080(ctx, base);
	// 82525F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82525F6C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525F70: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82525F74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82525F78: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82525F7C: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82525F80: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82525F84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82525F88: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82525F8C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82525F90: 4BF3E0A9  bl 0x82464038
	ctx.lr = 0x82525F94;
	sub_82464038(ctx, base);
	// 82525F94: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82525F98: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82525F9C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82525FA0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82525FA4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82525FA8: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82525FAC: 4BFFFDBD  bl 0x82525d68
	ctx.lr = 0x82525FB0;
	sub_82525D68(ctx, base);
	// 82525FB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82525FB4: 4800F154  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82525FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82525FB8 size=108
    let mut pc: u32 = 0x82525FB8;
    'dispatch: loop {
        match pc {
            0x82525FB8 => {
    //   block [0x82525FB8..0x82526024)
	// 82525FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82525FBC: 4800F0F9  bl 0x825350b4
	ctx.lr = 0x82525FC0;
	sub_82535080(ctx, base);
	// 82525FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82525FC4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82525FC8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82525FCC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82525FD0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82525FD4: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82525FD8: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82525FDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82525FE0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82525FE4: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82525FE8: 4BF3E051  bl 0x82464038
	ctx.lr = 0x82525FEC;
	sub_82464038(ctx, base);
	// 82525FEC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82525FF0: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82525FF4: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82525FF8: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82525FFC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82526000: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82526004: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82526008: 4BFFFD61  bl 0x82525d68
	ctx.lr = 0x8252600C;
	sub_82525D68(ctx, base);
	// 8252600C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82526010: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82526014: 396B4C60  addi r11, r11, 0x4c60
	ctx.r[11].s64 = ctx.r[11].s64 + 19552;
	// 82526018: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252601C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82526020: 4800F0E4  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82526028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82526028 size=204
    let mut pc: u32 = 0x82526028;
    'dispatch: loop {
        match pc {
            0x82526028 => {
    //   block [0x82526028..0x825260F4)
	// 82526028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252602C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82526030: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82526034: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82526038: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252603C: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82526040: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82526044: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82526048: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 8252604C: 39085FB8  addi r8, r8, 0x5fb8
	ctx.r[8].s64 = ctx.r[8].s64 + 24504;
	// 82526050: 392992E0  addi r9, r9, -0x6d20
	ctx.r[9].s64 = ctx.r[9].s64 + -27936;
	// 82526054: 394A9330  addi r10, r10, -0x6cd0
	ctx.r[10].s64 = ctx.r[10].s64 + -27856;
	// 82526058: 396B9380  addi r11, r11, -0x6c80
	ctx.r[11].s64 = ctx.r[11].s64 + -27776;
	// 8252605C: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82526060: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82526064: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82526068: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 8252606C: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82526070: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82526074: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82526078: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252607C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82526080: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82526084: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82526088: 4BFD9D01  bl 0x824ffd88
	ctx.lr = 0x8252608C;
	sub_824FFD88(ctx, base);
	// 8252608C: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 82526090: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82526094: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82526098: 396B5678  addi r11, r11, 0x5678
	ctx.r[11].s64 = ctx.r[11].s64 + 22136;
	// 8252609C: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 825260A0: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 825260A4: 39085F60  addi r8, r8, 0x5f60
	ctx.r[8].s64 = ctx.r[8].s64 + 24416;
	// 825260A8: 39295948  addi r9, r9, 0x5948
	ctx.r[9].s64 = ctx.r[9].s64 + 22856;
	// 825260AC: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 825260B0: 394A53B0  addi r10, r10, 0x53b0
	ctx.r[10].s64 = ctx.r[10].s64 + 21424;
	// 825260B4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825260B8: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 825260BC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 825260C0: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 825260C4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 825260C8: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825260CC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825260D0: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 825260D4: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 825260D8: 4BFD9CB1  bl 0x824ffd88
	ctx.lr = 0x825260DC;
	sub_824FFD88(ctx, base);
	// 825260DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825260E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825260E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825260E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825260EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825260F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825260F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825260F8 size=124
    let mut pc: u32 = 0x825260F8;
    'dispatch: loop {
        match pc {
            0x825260F8 => {
    //   block [0x825260F8..0x82526120)
	// 825260F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825260FC: 4800EFBD  bl 0x825350b8
	ctx.lr = 0x82526100;
	sub_82535080(ctx, base);
	// 82526100: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82526104: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82526108: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 8252610C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82526110: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82526114: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82526118: 4099003C  ble cr6, 0x82526154
	if !ctx.cr[6].gt {
	pc = 0x82526154; continue 'dispatch;
	}
	// 8252611C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	pc = 0x82526120; continue 'dispatch;
            }
            0x82526120 => {
    //   block [0x82526120..0x82526154)
	// 82526120: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82526124: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82526128: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8252612C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82526130: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82526134: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82526138: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252613C: 4E800421  bctrl
	ctx.lr = 0x82526140;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82526140: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82526144: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82526148: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 8252614C: 7F1F5800  cmpw cr6, r31, r11
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82526150: 4198FFD0  blt cr6, 0x82526120
	if ctx.cr[6].lt {
	pc = 0x82526120; continue 'dispatch;
	}
            }
            0x82526154 => {
    //   block [0x82526154..0x82526174)
	// 82526154: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82526158: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 8252615C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82526160: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82526164: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82526168: 4E800421  bctrl
	ctx.lr = 0x8252616C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252616C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82526170: 4800EF98  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82526178(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82526178 size=308
    let mut pc: u32 = 0x82526178;
    'dispatch: loop {
        match pc {
            0x82526178 => {
    //   block [0x82526178..0x825262A4)
	// 82526178: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252617C: 4800EF25  bl 0x825350a0
	ctx.lr = 0x82526180;
	sub_82535080(ctx, base);
	// 82526180: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82526184: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82526188: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252618C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82526190: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82526194: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82526198: 833D0000  lwz r25, 0(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252619C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 825261A0: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825261A4: 4BFFE85D  bl 0x82524a00
	ctx.lr = 0x825261A8;
	sub_82524A00(ctx, base);
	// 825261A8: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825261AC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825261B0: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825261B4: 4BFD49BD  bl 0x824fab70
	ctx.lr = 0x825261B8;
	sub_824FAB70(ctx, base);
	// 825261B8: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825261BC: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825261C0: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 825261C4: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 825261C8: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 825261CC: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 825261D0: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 825261D4: 419800D0  blt cr6, 0x825262a4
	if ctx.cr[6].lt {
	pc = 0x825262A4; continue 'dispatch;
	}
	// 825261D8: 3AC00030  li r22, 0x30
	ctx.r[22].s64 = 48;
	// 825261DC: 3AE00040  li r23, 0x40
	ctx.r[23].s64 = 64;
	// 825261E0: 3B000050  li r24, 0x50
	ctx.r[24].s64 = 80;
	// 825261E4: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 825261E8: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 825261EC: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 825261F0: 394A0002  addi r10, r10, 2
	ctx.r[10].s64 = ctx.r[10].s64 + 2;
	// 825261F4: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 825261F8: 554A2036  slwi r10, r10, 4
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	pc = 0x825262A4; continue 'dispatch;
            }
            0x825262A4 => {
    //   block [0x825262A4..0x825262AC)
	// 825262A4: 38210190  addi r1, r1, 0x190
	ctx.r[1].s64 = ctx.r[1].s64 + 400;
	// 825262A8: 4800EE48  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825262B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825262B0 size=268
    let mut pc: u32 = 0x825262B0;
    'dispatch: loop {
        match pc {
            0x825262B0 => {
    //   block [0x825262B0..0x825263B4)
	// 825262B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825262B4: 4800EDF5  bl 0x825350a8
	ctx.lr = 0x825262B8;
	sub_82535080(ctx, base);
	// 825262B8: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825262BC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825262C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825262C4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825262C8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825262CC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 825262D0: 833D0000  lwz r25, 0(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 825262D4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 825262D8: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 825262DC: 4BFFE725  bl 0x82524a00
	ctx.lr = 0x825262E0;
	sub_82524A00(ctx, base);
	// 825262E0: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825262E4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825262E8: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825262EC: 4BFD4885  bl 0x824fab70
	ctx.lr = 0x825262F0;
	sub_824FAB70(ctx, base);
	// 825262F0: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825262F4: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825262F8: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 825262FC: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82526300: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82526304: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82526308: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 8252630C: 419800A8  blt cr6, 0x825263b4
	if ctx.cr[6].lt {
	pc = 0x825263B4; continue 'dispatch;
	}
	// 82526310: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 82526314: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82526318: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 8252631C: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 82526320: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 82526324: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82526328: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	pc = 0x825263B4; continue 'dispatch;
            }
            0x825263B4 => {
    //   block [0x825263B4..0x825263BC)
	// 825263B4: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 825263B8: 4800ED40  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825263C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825263C0 size=292
    let mut pc: u32 = 0x825263C0;
    'dispatch: loop {
        match pc {
            0x825263C0 => {
    //   block [0x825263C0..0x825264DC)
	// 825263C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825263C4: 4800ECE1  bl 0x825350a4
	ctx.lr = 0x825263C8;
	sub_82535080(ctx, base);
	// 825263C8: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825263CC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 825263D0: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 825263D4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 825263D8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825263DC: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 825263E0: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 825263E4: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 825263E8: 4BFFE619  bl 0x82524a00
	ctx.lr = 0x825263EC;
	sub_82524A00(ctx, base);
	// 825263EC: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 825263F0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825263F4: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 825263F8: 4BFD4779  bl 0x824fab70
	ctx.lr = 0x825263FC;
	sub_824FAB70(ctx, base);
	// 825263FC: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82526400: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82526404: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82526408: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 8252640C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82526410: 815B0010  lwz r10, 0x10(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82526414: 830B000C  lwz r24, 0xc(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82526418: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252641C: 409900C0  ble cr6, 0x825264dc
	if !ctx.cr[6].gt {
	pc = 0x825264DC; continue 'dispatch;
	}
	// 82526420: 3BDB0020  addi r30, r27, 0x20
	ctx.r[30].s64 = ctx.r[27].s64 + 32;
	// 82526424: 3AE00030  li r23, 0x30
	ctx.r[23].s64 = 48;
	// 82526428: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	pc = 0x825264DC; continue 'dispatch;
            }
            0x825264DC => {
    //   block [0x825264DC..0x825264E4)
	// 825264DC: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 825264E0: 4800EC14  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825264E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825264E8 size=272
    let mut pc: u32 = 0x825264E8;
    'dispatch: loop {
        match pc {
            0x825264E8 => {
    //   block [0x825264E8..0x825265F0)
	// 825264E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825264EC: 4800EBB9  bl 0x825350a4
	ctx.lr = 0x825264F0;
	sub_82535080(ctx, base);
	// 825264F0: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825264F4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825264F8: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825264FC: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82526500: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82526504: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82526508: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252650C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82526510: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82526514: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 82526518: 4BFFE4E9  bl 0x82524a00
	ctx.lr = 0x8252651C;
	sub_82524A00(ctx, base);
	// 8252651C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82526520: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82526524: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82526528: 4BFD4649  bl 0x824fab70
	ctx.lr = 0x8252652C;
	sub_824FAB70(ctx, base);
	// 8252652C: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82526530: 831F000C  lwz r24, 0xc(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82526534: 3BEBFFFF  addi r31, r11, -1
	ctx.r[31].s64 = ctx.r[11].s64 + -1;
	// 82526538: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 8252653C: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82526540: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82526544: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82526548: 419800A8  blt cr6, 0x825265f0
	if ctx.cr[6].lt {
	pc = 0x825265F0; continue 'dispatch;
	}
	// 8252654C: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82526550: 3AE00030  li r23, 0x30
	ctx.r[23].s64 = 48;
	// 82526554: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82526558: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 8252655C: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	pc = 0x825265F0; continue 'dispatch;
            }
            0x825265F0 => {
    //   block [0x825265F0..0x825265F8)
	// 825265F0: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 825265F4: 4800EB00  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825265F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825265F8 size=300
    let mut pc: u32 = 0x825265F8;
    'dispatch: loop {
        match pc {
            0x825265F8 => {
    //   block [0x825265F8..0x8252671C)
	// 825265F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825265FC: 4800EAA5  bl 0x825350a0
	ctx.lr = 0x82526600;
	sub_82535080(ctx, base);
	// 82526600: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82526604: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82526608: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 8252660C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82526610: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82526614: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82526618: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252661C: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82526620: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82526624: 4BFFE3DD  bl 0x82524a00
	ctx.lr = 0x82526628;
	sub_82524A00(ctx, base);
	// 82526628: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252662C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82526630: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82526634: 4BFD453D  bl 0x824fab70
	ctx.lr = 0x82526638;
	sub_824FAB70(ctx, base);
	// 82526638: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 8252663C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82526640: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82526644: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82526648: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8252664C: 815B0010  lwz r10, 0x10(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82526650: 82EB000C  lwz r23, 0xc(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82526654: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82526658: 409900C4  ble cr6, 0x8252671c
	if !ctx.cr[6].gt {
	pc = 0x8252671C; continue 'dispatch;
	}
	// 8252665C: 3BDB0020  addi r30, r27, 0x20
	ctx.r[30].s64 = ctx.r[27].s64 + 32;
	// 82526660: 3AC00030  li r22, 0x30
	ctx.r[22].s64 = 48;
	// 82526664: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	pc = 0x8252671C; continue 'dispatch;
            }
            0x8252671C => {
    //   block [0x8252671C..0x82526724)
	// 8252671C: 38210190  addi r1, r1, 0x190
	ctx.r[1].s64 = ctx.r[1].s64 + 400;
	// 82526720: 4800E9D0  b 0x825350f0
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82526728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82526728 size=280
    let mut pc: u32 = 0x82526728;
    'dispatch: loop {
        match pc {
            0x82526728 => {
    //   block [0x82526728..0x82526838)
	// 82526728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252672C: 4800E97D  bl 0x825350a8
	ctx.lr = 0x82526730;
	sub_82535080(ctx, base);
	// 82526730: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82526734: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82526738: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 8252673C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82526740: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82526744: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82526748: 833E0000  lwz r25, 0(r30)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252674C: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82526750: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82526754: 4BFFE2AD  bl 0x82524a00
	ctx.lr = 0x82526758;
	sub_82524A00(ctx, base);
	// 82526758: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 8252675C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82526760: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82526764: 4BFD440D  bl 0x824fab70
	ctx.lr = 0x82526768;
	sub_824FAB70(ctx, base);
	// 82526768: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252676C: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82526770: 3BABFFFF  addi r29, r11, -1
	ctx.r[29].s64 = ctx.r[11].s64 + -1;
	// 82526774: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82526778: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 8252677C: 2F1D0000  cmpwi cr6, r29, 0
	ctx.cr[6].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82526780: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82526784: 419800B4  blt cr6, 0x82526838
	if ctx.cr[6].lt {
	pc = 0x82526838; continue 'dispatch;
	}
	// 82526788: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 8252678C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82526790: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82526794: 392100A0  addi r9, r1, 0xa0
	ctx.r[9].s64 = ctx.r[1].s64 + 160;
	// 82526798: 396B0002  addi r11, r11, 2
	ctx.r[11].s64 = ctx.r[11].s64 + 2;
	// 8252679C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 825267A0: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	pc = 0x82526838; continue 'dispatch;
            }
            0x82526838 => {
    //   block [0x82526838..0x82526840)
	// 82526838: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 8252683C: 4800E8BC  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82526840(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82526840 size=304
    let mut pc: u32 = 0x82526840;
    'dispatch: loop {
        match pc {
            0x82526840 => {
    //   block [0x82526840..0x82526968)
	// 82526840: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82526844: 4800E861  bl 0x825350a4
	ctx.lr = 0x82526848;
	sub_82535080(ctx, base);
	// 82526848: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252684C: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82526850: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82526854: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82526858: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8252685C: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82526860: 82FB0000  lwz r23, 0(r27)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82526864: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82526868: 4BFFE199  bl 0x82524a00
	ctx.lr = 0x8252686C;
	sub_82524A00(ctx, base);
	// 8252686C: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82526870: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82526874: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82526878: 4BFD42F9  bl 0x824fab70
	ctx.lr = 0x8252687C;
	sub_824FAB70(ctx, base);
	// 8252687C: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82526880: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82526884: 9361005C  stw r27, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 82526888: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 8252688C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82526890: 81570010  lwz r10, 0x10(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(16 as u32) ) } as u64;
	// 82526894: 832B000C  lwz r25, 0xc(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82526898: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 8252689C: 409900CC  ble cr6, 0x82526968
	if !ctx.cr[6].gt {
	pc = 0x82526968; continue 'dispatch;
	}
	// 825268A0: 3BF70020  addi r31, r23, 0x20
	ctx.r[31].s64 = ctx.r[23].s64 + 32;
	// 825268A4: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 825268A8: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	pc = 0x82526968; continue 'dispatch;
            }
            0x82526968 => {
    //   block [0x82526968..0x82526970)
	// 82526968: 38210180  addi r1, r1, 0x180
	ctx.r[1].s64 = ctx.r[1].s64 + 384;
	// 8252696C: 4800E788  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82526970(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82526970 size=488
    let mut pc: u32 = 0x82526970;
    'dispatch: loop {
        match pc {
            0x82526970 => {
    //   block [0x82526970..0x825269F0)
	// 82526970: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82526974: 4800E721  bl 0x82535094
	ctx.lr = 0x82526978;
	sub_82535080(ctx, base);
	// 82526978: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252697C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82526980: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 82526984: 396B5754  addi r11, r11, 0x5754
	ctx.r[11].s64 = ctx.r[11].s64 + 22356;
	// 82526988: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 8252698C: 3BB3000C  addi r29, r19, 0xc
	ctx.r[29].s64 = ctx.r[19].s64 + 12;
	// 82526990: 3D208000  lis r9, -0x8000
	ctx.r[9].s64 = -2147483648;
	// 82526994: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82526998: 91730000  stw r11, 0(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 8252699C: 61290004  ori r9, r9, 4
	ctx.r[9].u64 = ctx.r[9].u64 | 4;
	// 825269A0: B1530006  sth r10, 6(r19)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[19].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 825269A4: 397D000C  addi r11, r29, 0xc
	ctx.r[11].s64 = ctx.r[29].s64 + 12;
	// 825269A8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825269AC: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 825269B0: 92F30008  stw r23, 8(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(8 as u32), ctx.r[23].u32 ) };
	// 825269B4: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 825269B8: 913D0008  stw r9, 8(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(8 as u32), ctx.r[9].u32 ) };
	// 825269BC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 825269C0: 917D0000  stw r11, 0(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825269C4: 552B003E  slwi r11, r9, 0
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825269C8: 915D0004  stw r10, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[10].u32 ) };
	// 825269CC: 83F90000  lwz r31, 0(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 825269D0: 556B00BE  clrlwi r11, r11, 2
	ctx.r[11].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 825269D4: 831F0010  lwz r24, 0x10(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825269D8: 7F0BC000  cmpw cr6, r11, r24
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[24].s32, &mut ctx.xer);
	// 825269DC: 40980024  bge cr6, 0x82526a00
	if !ctx.cr[6].lt {
	pc = 0x82526A00; continue 'dispatch;
	}
	// 825269E0: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825269E4: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 825269E8: 41980008  blt cr6, 0x825269f0
	if ctx.cr[6].lt {
	pc = 0x825269F0; continue 'dispatch;
	}
	// 825269EC: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	pc = 0x825269F0; continue 'dispatch;
            }
            0x825269F0 => {
    //   block [0x825269F0..0x82526A00)
	// 825269F0: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 825269F4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 825269F8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 825269FC: 4BF478CD  bl 0x8246e2c8
	ctx.lr = 0x82526A00;
	sub_8246E2C8(ctx, base);
	pc = 0x82526A00; continue 'dispatch;
            }
            0x82526A00 => {
    //   block [0x82526A00..0x82526B4C)
	// 82526A00: 3D60820D  lis r11, -0x7df3
	ctx.r[11].s64 = -2113077248;
	// 82526A04: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82526A08: C02B1FF8  lfs f1, 0x1ff8(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8184 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82526A0C: 4BFD4165  bl 0x824fab70
	ctx.lr = 0x82526A10;
	sub_824FAB70(ctx, base);
	// 82526A10: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82526A14: 80990008  lwz r4, 8(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82526A18: 4BFFDFE9  bl 0x82524a00
	ctx.lr = 0x82526A1C;
	sub_82524A00(ctx, base);
	// 82526A1C: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	// 82526A20: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82526A24: 9321005C  stw r25, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 82526A28: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82526A2C: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82526A30: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82526A34: 40990118  ble cr6, 0x82526b4c
	if !ctx.cr[6].gt {
	pc = 0x82526B4C; continue 'dispatch;
	}
	// 82526A38: 3A800030  li r20, 0x30
	ctx.r[20].s64 = 48;
	// 82526A3C: 3AA00040  li r21, 0x40
	ctx.r[21].s64 = 64;
	// 82526A40: 3AC00050  li r22, 0x50
	ctx.r[22].s64 = 80;
	// 82526A44: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
	pc = 0x82526B4C; continue 'dispatch;
            }
            0x82526B4C => {
    //   block [0x82526B4C..0x82526B58)
	// 82526B4C: 7E639B78  mr r3, r19
	ctx.r[3].u64 = ctx.r[19].u64;
	// 82526B50: 382101A0  addi r1, r1, 0x1a0
	ctx.r[1].s64 = ctx.r[1].s64 + 416;
	// 82526B54: 4800E590  b 0x825350e4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82526B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82526B58 size=88
    let mut pc: u32 = 0x82526B58;
    'dispatch: loop {
        match pc {
            0x82526B58 => {
    //   block [0x82526B58..0x82526BB0)
	// 82526B58: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82526B5C: 4800E55D  bl 0x825350b8
	ctx.lr = 0x82526B60;
	sub_82535080(ctx, base);
	// 82526B60: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82526B64: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82526B68: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82526B6C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82526B70: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82526B74: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82526B78: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82526B7C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82526B80: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82526B84: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82526B88: 4BF3D4B1  bl 0x82464038
	ctx.lr = 0x82526B8C;
	sub_82464038(ctx, base);
	// 82526B8C: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82526B90: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82526B94: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82526B98: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82526B9C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82526BA0: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82526BA4: 4BFFFDCD  bl 0x82526970
	ctx.lr = 0x82526BA8;
	sub_82526970(ctx, base);
	// 82526BA8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82526BAC: 4800E55C  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82526BB0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82526BB0 size=108
    let mut pc: u32 = 0x82526BB0;
    'dispatch: loop {
        match pc {
            0x82526BB0 => {
    //   block [0x82526BB0..0x82526C1C)
	// 82526BB0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82526BB4: 4800E501  bl 0x825350b4
	ctx.lr = 0x82526BB8;
	sub_82535080(ctx, base);
	// 82526BB8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82526BBC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82526BC0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82526BC4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82526BC8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82526BCC: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82526BD0: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82526BD4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82526BD8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82526BDC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82526BE0: 4BF3D459  bl 0x82464038
	ctx.lr = 0x82526BE4;
	sub_82464038(ctx, base);
	// 82526BE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82526BE8: 39600038  li r11, 0x38
	ctx.r[11].s64 = 56;
	// 82526BEC: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82526BF0: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82526BF4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82526BF8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82526BFC: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82526C00: 4BFFFD71  bl 0x82526970
	ctx.lr = 0x82526C04;
	sub_82526970(ctx, base);
	// 82526C04: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82526C08: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82526C0C: 396B5790  addi r11, r11, 0x5790
	ctx.r[11].s64 = ctx.r[11].s64 + 22416;
	// 82526C10: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82526C14: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82526C18: 4800E4EC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82526C20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82526C20 size=204
    let mut pc: u32 = 0x82526C20;
    'dispatch: loop {
        match pc {
            0x82526C20 => {
    //   block [0x82526C20..0x82526CEC)
	// 82526C20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82526C24: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82526C28: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82526C2C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82526C30: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82526C34: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82526C38: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82526C3C: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82526C40: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 82526C44: 39086BB0  addi r8, r8, 0x6bb0
	ctx.r[8].s64 = ctx.r[8].s64 + 27568;
	// 82526C48: 39296CF0  addi r9, r9, 0x6cf0
	ctx.r[9].s64 = ctx.r[9].s64 + 27888;
	// 82526C4C: 394A6D40  addi r10, r10, 0x6d40
	ctx.r[10].s64 = ctx.r[10].s64 + 27968;
	// 82526C50: 396B6D90  addi r11, r11, 0x6d90
	ctx.r[11].s64 = ctx.r[11].s64 + 28048;
	// 82526C54: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82526C58: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82526C5C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82526C60: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82526C64: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82526C68: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82526C6C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82526C70: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82526C74: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82526C78: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 82526C7C: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 82526C80: 4BFD9109  bl 0x824ffd88
	ctx.lr = 0x82526C84;
	sub_824FFD88(ctx, base);
	// 82526C84: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 82526C88: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 82526C8C: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 82526C90: 396B65F8  addi r11, r11, 0x65f8
	ctx.r[11].s64 = ctx.r[11].s64 + 26104;
	// 82526C94: 3D208252  lis r9, -0x7dae
	ctx.r[9].s64 = -2108555264;
	// 82526C98: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 82526C9C: 39086B58  addi r8, r8, 0x6b58
	ctx.r[8].s64 = ctx.r[8].s64 + 27480;
	// 82526CA0: 39296840  addi r9, r9, 0x6840
	ctx.r[9].s64 = ctx.r[9].s64 + 26688;
	// 82526CA4: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82526CA8: 394A63C0  addi r10, r10, 0x63c0
	ctx.r[10].s64 = ctx.r[10].s64 + 25536;
	// 82526CAC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82526CB0: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82526CB4: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 82526CB8: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82526CBC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82526CC0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 82526CC4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82526CC8: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 82526CCC: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 82526CD0: 4BFD90B9  bl 0x824ffd88
	ctx.lr = 0x82526CD4;
	sub_824FFD88(ctx, base);
	// 82526CD4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82526CD8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82526CDC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82526CE0: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82526CE4: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82526CE8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82526CF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82526CF0 size=76
    let mut pc: u32 = 0x82526CF0;
    'dispatch: loop {
        match pc {
            0x82526CF0 => {
    //   block [0x82526CF0..0x82526D3C)
	// 82526CF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82526CF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82526CF8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82526CFC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82526D00: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82526D04: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82526D08: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82526D0C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82526D10: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 82526D14: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82526D18: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82526D1C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82526D20: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82526D24: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82526D28: 4BFFFB19  bl 0x82526840
	ctx.lr = 0x82526D2C;
	sub_82526840(ctx, base);
	// 82526D2C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82526D30: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82526D34: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82526D38: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82526D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82526D40 size=80
    let mut pc: u32 = 0x82526D40;
    'dispatch: loop {
        match pc {
            0x82526D40 => {
    //   block [0x82526D40..0x82526D90)
	// 82526D40: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82526D44: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82526D48: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82526D4C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82526D50: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82526D54: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82526D58: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 82526D5C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82526D60: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82526D64: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82526D68: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82526D6C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82526D70: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82526D74: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82526D78: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82526D7C: 4BFFF645  bl 0x825263c0
	ctx.lr = 0x82526D80;
	sub_825263C0(ctx, base);
	// 82526D80: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82526D84: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82526D88: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82526D8C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82526D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82526D90 size=232
    let mut pc: u32 = 0x82526D90;
    'dispatch: loop {
        match pc {
            0x82526D90 => {
    //   block [0x82526D90..0x82526DBC)
	// 82526D90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82526D94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82526D98: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82526D9C: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82526DA0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82526DA4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82526DA8: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82526DAC: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82526DB0: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82526DB4: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82526DB8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82526DBC; continue 'dispatch;
            }
            0x82526DBC => {
    //   block [0x82526DBC..0x82526E78)
	// 82526DBC: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82526DC0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82526DC4: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82526DC8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82526DCC: 4200FFF0  bdnz 0x82526dbc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82526DBC; continue 'dispatch;
	}
	// 82526DD0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82526DD4: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82526DD8: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 82526DDC: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82526DE0: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82526DE4: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82526E78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82526E78 size=76
    let mut pc: u32 = 0x82526E78;
    'dispatch: loop {
        match pc {
            0x82526E78 => {
    //   block [0x82526E78..0x82526EC4)
	// 82526E78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82526E7C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82526E80: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82526E84: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82526E88: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82526E8C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82526E90: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82526E94: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82526E98: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 82526E9C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82526EA0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82526EA4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82526EA8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82526EAC: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82526EB0: 4BFFF879  bl 0x82526728
	ctx.lr = 0x82526EB4;
	sub_82526728(ctx, base);
	// 82526EB4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82526EB8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82526EBC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82526EC0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82526EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82526EC8 size=80
    let mut pc: u32 = 0x82526EC8;
    'dispatch: loop {
        match pc {
            0x82526EC8 => {
    //   block [0x82526EC8..0x82526F18)
	// 82526EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82526ECC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82526ED0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82526ED4: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82526ED8: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82526EDC: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82526EE0: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 82526EE4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82526EE8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82526EEC: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82526EF0: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82526EF4: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82526EF8: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82526EFC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82526F00: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82526F04: 4BFFF3AD  bl 0x825262b0
	ctx.lr = 0x82526F08;
	sub_825262B0(ctx, base);
	// 82526F08: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82526F0C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82526F10: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82526F14: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82526F18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82526F18 size=176
    let mut pc: u32 = 0x82526F18;
    'dispatch: loop {
        match pc {
            0x82526F18 => {
    //   block [0x82526F18..0x82526F98)
	// 82526F18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82526F1C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82526F20: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82526F24: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82526F28: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82526F2C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82526F30: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82526F34: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82526F38: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82526F3C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82526F40: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82526F44: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82526F48: 4BFFF231  bl 0x82526178
	ctx.lr = 0x82526F4C;
	sub_82526178(ctx, base);
	// 82526F4C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82526F50: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82526F54: 40980044  bge cr6, 0x82526f98
	if !ctx.cr[6].lt {
	pc = 0x82526F98; continue 'dispatch;
	}
	// 82526F58: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82526F5C: 394B08F0  addi r10, r11, 0x8f0
	ctx.r[10].s64 = ctx.r[11].s64 + 2288;
	// 82526F60: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x82526F98; continue 'dispatch;
            }
            0x82526F98 => {
    //   block [0x82526F98..0x82526FC8)
	// 82526F98: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82526F9C: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 82526FA0: 419A000C  beq cr6, 0x82526fac
	if ctx.cr[6].eq {
	pc = 0x82526FAC; continue 'dispatch;
	}
	// 82526FA4: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 82526FA8: 4BFEC499  bl 0x82513440
	ctx.lr = 0x82526FAC;
	sub_82513440(ctx, base);
	// 82526FAC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82526FB0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82526FB4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82526FB8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82526FBC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82526FC0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82526FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82526FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82526FC8 size=240
    let mut pc: u32 = 0x82526FC8;
    'dispatch: loop {
        match pc {
            0x82526FC8 => {
    //   block [0x82526FC8..0x82526FF8)
	// 82526FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82526FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82526FD0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82526FD4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82526FD8: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82526FDC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82526FE0: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82526FE4: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82526FE8: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82526FEC: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82526FF0: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82526FF4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82526FF8; continue 'dispatch;
            }
            0x82526FF8 => {
    //   block [0x82526FF8..0x825270B8)
	// 82526FF8: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82526FFC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82527000: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82527004: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82527008: 4200FFF0  bdnz 0x82526ff8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82526FF8; continue 'dispatch;
	}
	// 8252700C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82527010: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82527014: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 82527018: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8252701C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82527020: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825270B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825270B8 size=152
    let mut pc: u32 = 0x825270B8;
    'dispatch: loop {
        match pc {
            0x825270B8 => {
    //   block [0x825270B8..0x82527100)
	// 825270B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825270BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825270C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825270C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825270C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825270CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825270D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825270D4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 825270D8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 825270DC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 825270E0: 409A0020  bne cr6, 0x82527100
	if !ctx.cr[6].eq {
	pc = 0x82527100; continue 'dispatch;
	}
	// 825270E4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825270E8: 39200010  li r9, 0x10
	ctx.r[9].s64 = 16;
	// 825270EC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 825270F0: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825270F4: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 825270F8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 825270FC: 4BF3CFBD  bl 0x824640b8
	ctx.lr = 0x82527100;
	sub_824640B8(ctx, base);
	pc = 0x82527100; continue 'dispatch;
            }
            0x82527100 => {
    //   block [0x82527100..0x82527134)
	// 82527100: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82527104: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82527108: 396B6DD0  addi r11, r11, 0x6dd0
	ctx.r[11].s64 = ctx.r[11].s64 + 28112;
	// 8252710C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82527110: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82527114: 419A0020  beq cr6, 0x82527134
	if ctx.cr[6].eq {
	pc = 0x82527134; continue 'dispatch;
	}
	// 82527118: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252711C: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82527120: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82527124: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82527128: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 8252712C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82527130: 4BF3CF89  bl 0x824640b8
	ctx.lr = 0x82527134;
	sub_824640B8(ctx, base);
	pc = 0x82527134; continue 'dispatch;
            }
            0x82527134 => {
    //   block [0x82527134..0x82527150)
	// 82527134: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82527138: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252713C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82527140: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82527144: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82527148: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 8252714C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82527150(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82527150 size=136
    let mut pc: u32 = 0x82527150;
    'dispatch: loop {
        match pc {
            0x82527150 => {
    //   block [0x82527150..0x825271A8)
	// 82527150: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82527154: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82527158: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252715C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82527160: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82527164: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82527168: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 8252716C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527170: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527174: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82527178: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252717C: 4E800421  bctrl
	ctx.lr = 0x82527180;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82527180: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527184: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82527188: 419A0020  beq cr6, 0x825271a8
	if ctx.cr[6].eq {
	pc = 0x825271A8; continue 'dispatch;
	}
	// 8252718C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527190: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82527194: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82527198: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252719C: 4E800421  bctrl
	ctx.lr = 0x825271A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825271A0: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825271A4: 917F0010  stw r11, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[11].u32 ) };
            }
            0x825271A8 => {
    //   block [0x825271A8..0x825271D8)
	// 825271A8: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825271AC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825271B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825271B4: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825271B8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825271BC: 4E800421  bctrl
	ctx.lr = 0x825271C0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825271C0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825271C4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825271C8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825271CC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825271D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825271D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825271D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825271D8 size=108
    let mut pc: u32 = 0x825271D8;
    'dispatch: loop {
        match pc {
            0x825271D8 => {
    //   block [0x825271D8..0x8252722C)
	// 825271D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825271DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825271E0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 825271E4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825271E8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825271EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825271F0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825271F4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 825271F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825271FC: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82527200: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82527204: 4E800421  bctrl
	ctx.lr = 0x82527208;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82527208: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252720C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82527210: 419A001C  beq cr6, 0x8252722c
	if ctx.cr[6].eq {
	pc = 0x8252722C; continue 'dispatch;
	}
	// 82527214: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82527218: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 8252721C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527220: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82527224: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82527228: 4E800421  bctrl
	ctx.lr = 0x8252722C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x8252722C => {
    //   block [0x8252722C..0x82527244)
	// 8252722C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82527230: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82527234: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82527238: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 8252723C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82527240: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82527248(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82527248 size=140
    let mut pc: u32 = 0x82527248;
    'dispatch: loop {
        match pc {
            0x82527248 => {
    //   block [0x82527248..0x825272B4)
	// 82527248: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252724C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82527250: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82527254: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82527258: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 8252725C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82527260: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82527264: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82527268: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 8252726C: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82527270: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82527274: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527278: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252727C: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82527280: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82527284: 4E800421  bctrl
	ctx.lr = 0x82527288;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82527288: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252728C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82527290: 419A0024  beq cr6, 0x825272b4
	if ctx.cr[6].eq {
	pc = 0x825272B4; continue 'dispatch;
	}
	// 82527294: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82527298: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 8252729C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825272A0: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 825272A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825272A8: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825272AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825272B0: 4E800421  bctrl
	ctx.lr = 0x825272B4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x825272B4 => {
    //   block [0x825272B4..0x825272D4)
	// 825272B4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 825272B8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825272BC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825272C0: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 825272C4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 825272C8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825272CC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825272D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825272D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825272D8 size=32
    let mut pc: u32 = 0x825272D8;
    'dispatch: loop {
        match pc {
            0x825272D8 => {
    //   block [0x825272D8..0x825272F8)
	// 825272D8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 825272DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825272E0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 825272E4: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 825272E8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825272EC: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825272F0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825272F4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825272F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825272F8 size=4
    let mut pc: u32 = 0x825272F8;
    'dispatch: loop {
        match pc {
            0x825272F8 => {
    //   block [0x825272F8..0x825272FC)
	// 825272F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82527300(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82527300 size=32
    let mut pc: u32 = 0x82527300;
    'dispatch: loop {
        match pc {
            0x82527300 => {
    //   block [0x82527300..0x82527320)
	// 82527300: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527304: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82527308: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 8252730C: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82527310: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527314: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82527318: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252731C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82527320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82527320 size=4
    let mut pc: u32 = 0x82527320;
    'dispatch: loop {
        match pc {
            0x82527320 => {
    //   block [0x82527320..0x82527324)
	// 82527320: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82527328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82527328 size=32
    let mut pc: u32 = 0x82527328;
    'dispatch: loop {
        match pc {
            0x82527328 => {
    //   block [0x82527328..0x82527348)
	// 82527328: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252732C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82527330: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
	// 82527334: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82527338: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252733C: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82527340: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82527344: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82527348(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82527348 size=4
    let mut pc: u32 = 0x82527348;
    'dispatch: loop {
        match pc {
            0x82527348 => {
    //   block [0x82527348..0x8252734C)
	// 82527348: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82527350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82527350 size=156
    let mut pc: u32 = 0x82527350;
    'dispatch: loop {
        match pc {
            0x82527350 => {
    //   block [0x82527350..0x825273E4)
	// 82527350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82527354: 4800DD61  bl 0x825350b4
	ctx.lr = 0x82527358;
	sub_82535080(ctx, base);
	// 82527358: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252735C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82527360: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82527364: 83640000  lwz r27, 0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527368: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 8252736C: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82527370: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82527374: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82527378: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8252737C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82527380: 817B0010  lwz r11, 0x10(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527384: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527388: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 8252738C: 3960FFFF  li r11, -1
	ctx.r[11].s64 = -1;
	// 82527390: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 82527394: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527398: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 8252739C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825273A0: 4E800421  bctrl
	ctx.lr = 0x825273A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825273A4: 817F0010  lwz r11, 0x10(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825273A8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825273AC: 419A0038  beq cr6, 0x825273e4
	if ctx.cr[6].eq {
	pc = 0x825273E4; continue 'dispatch;
	}
	// 825273B0: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 825273B4: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 825273B8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 825273BC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 825273C0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 825273C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825273C8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825273CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825273D0: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 825273D4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825273D8: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 825273DC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825273E0: 4E800421  bctrl
	ctx.lr = 0x825273E4;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x825273E4 => {
    //   block [0x825273E4..0x825273EC)
	// 825273E4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 825273E8: 4800DD1C  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825273F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825273F0 size=212
    let mut pc: u32 = 0x825273F0;
    'dispatch: loop {
        match pc {
            0x825273F0 => {
    //   block [0x825273F0..0x8252746C)
	// 825273F0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825273F4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825273F8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825273FC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82527400: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82527404: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82527408: 394A57CC  addi r10, r10, 0x57cc
	ctx.r[10].s64 = ctx.r[10].s64 + 22476;
	// 8252740C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82527410: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82527414: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82527418: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 8252741C: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82527420: 915F0000  stw r10, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u32 ) };
	// 82527424: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82527428: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252742C: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82527430: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82527434: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527438: 80C50010  lwz r6, 0x10(r5)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252743C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527440: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82527444: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82527448: 81690010  lwz r11, 0x10(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252744C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82527450: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82527454: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82527458: 8128000C  lwz r9, 0xc(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252745C: 390A05A0  addi r8, r10, 0x5a0
	ctx.r[8].s64 = ctx.r[10].s64 + 1440;
	// 82527460: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527464: 409A0008  bne cr6, 0x8252746c
	if !ctx.cr[6].eq {
	pc = 0x8252746C; continue 'dispatch;
	}
	// 82527468: 390A01A0  addi r8, r10, 0x1a0
	ctx.r[8].s64 = ctx.r[10].s64 + 416;
	pc = 0x8252746C; continue 'dispatch;
            }
            0x8252746C => {
    //   block [0x8252746C..0x825274C4)
	// 8252746C: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82527470: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82527474: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82527478: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 8252747C: 7D6B48AE  lbzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82527480: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82527484: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82527488: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252748C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82527490: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82527494: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82527498: 4E800421  bctrl
	ctx.lr = 0x8252749C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 8252749C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825274A0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 825274A4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825274A8: 917F000C  stw r11, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[11].u32 ) };
	// 825274AC: 915F0010  stw r10, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 825274B0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825274B4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825274B8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825274BC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825274C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825274C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825274C8 size=88
    let mut pc: u32 = 0x825274C8;
    'dispatch: loop {
        match pc {
            0x825274C8 => {
    //   block [0x825274C8..0x82527520)
	// 825274C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825274CC: 4800DBED  bl 0x825350b8
	ctx.lr = 0x825274D0;
	sub_82535080(ctx, base);
	// 825274D0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825274D4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825274D8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825274DC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825274E0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 825274E4: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 825274E8: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 825274EC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825274F0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825274F4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 825274F8: 4BF3CB41  bl 0x82464038
	ctx.lr = 0x825274FC;
	sub_82464038(ctx, base);
	// 825274FC: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 82527500: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82527504: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82527508: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8252750C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82527510: B1630004  sth r11, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82527514: 4BFFFEDD  bl 0x825273f0
	ctx.lr = 0x82527518;
	sub_825273F0(ctx, base);
	// 82527518: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 8252751C: 4800DBEC  b 0x82535108
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82527520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82527520 size=496
    let mut pc: u32 = 0x82527520;
    'dispatch: loop {
        match pc {
            0x82527520 => {
    //   block [0x82527520..0x82527580)
	// 82527520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82527524: 4800DB85  bl 0x825350a8
	ctx.lr = 0x82527528;
	sub_82535080(ctx, base);
	// 82527528: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252752C: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527530: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 82527534: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82527538: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8252753C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82527540: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82527544: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82527548: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252754C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527550: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82527554: 4098002C  bge cr6, 0x82527580
	if !ctx.cr[6].lt {
	pc = 0x82527580; continue 'dispatch;
	}
	// 82527558: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 8252755C: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82527560: 39295814  addi r9, r9, 0x5814
	ctx.r[9].s64 = ctx.r[9].s64 + 22548;
	// 82527564: 39085804  addi r8, r8, 0x5804
	ctx.r[8].s64 = ctx.r[8].s64 + 22532;
	// 82527568: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252756C: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82527570: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82527574: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82527578: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 8252757C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82527580; continue 'dispatch;
            }
            0x82527580 => {
    //   block [0x82527580..0x82527614)
	// 82527580: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82527584: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82527588: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252758C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82527590: 9081006C  stw r4, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[4].u32 ) };
	// 82527594: 396B4F50  addi r11, r11, 0x4f50
	ctx.r[11].s64 = ctx.r[11].s64 + 20304;
	// 82527598: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252759C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 825275A0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825275A4: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 825275A8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825275AC: 815C0010  lwz r10, 0x10(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 825275B0: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825275B4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 825275B8: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 825275BC: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825275C0: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 825275C4: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 825275C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825275CC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 825275D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825275D4: 4E800421  bctrl
	ctx.lr = 0x825275D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825275D8: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 825275DC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825275E0: 419A00D4  beq cr6, 0x825276b4
	if ctx.cr[6].eq {
	pc = 0x825276B4; continue 'dispatch;
	}
	// 825275E4: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 825275E8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825275EC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825275F0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825275F4: 40980020  bge cr6, 0x82527614
	if !ctx.cr[6].lt {
	pc = 0x82527614; continue 'dispatch;
	}
	// 825275F8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825275FC: 39294FF4  addi r9, r9, 0x4ff4
	ctx.r[9].s64 = ctx.r[9].s64 + 20468;
	// 82527600: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82527604: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82527608: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8252760C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82527610: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            0x82527614 => {
    //   block [0x82527614..0x82527654)
	// 82527614: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82527618: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252761C: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82527620: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82527624: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82527628: 409A0064  bne cr6, 0x8252768c
	if !ctx.cr[6].eq {
	pc = 0x8252768C; continue 'dispatch;
	}
	// 8252762C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527630: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527634: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527638: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8252763C: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82527640: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527644: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527648: 394905A0  addi r10, r9, 0x5a0
	ctx.r[10].s64 = ctx.r[9].s64 + 1440;
	// 8252764C: 409A0008  bne cr6, 0x82527654
	if !ctx.cr[6].eq {
	pc = 0x82527654; continue 'dispatch;
	}
	// 82527650: 394901A0  addi r10, r9, 0x1a0
	ctx.r[10].s64 = ctx.r[9].s64 + 416;
	pc = 0x82527654; continue 'dispatch;
            }
            0x82527654 => {
    //   block [0x82527654..0x8252768C)
	// 82527654: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82527658: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 8252765C: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82527660: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82527664: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82527668: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 8252766C: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82527670: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82527674: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82527678: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 8252767C: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82527680: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82527684: 4E800421  bctrl
	ctx.lr = 0x82527688;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82527688: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
            }
            0x8252768C => {
    //   block [0x8252768C..0x825276B4)
	// 8252768C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527690: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82527694: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82527698: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 8252769C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 825276A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825276A4: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 825276A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825276AC: 4E800421  bctrl
	ctx.lr = 0x825276B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825276B0: 48000028  b 0x825276d8
	pc = 0x825276D8; continue 'dispatch;
            }
            0x825276B4 => {
    //   block [0x825276B4..0x825276D8)
	// 825276B4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 825276B8: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 825276BC: 419A001C  beq cr6, 0x825276d8
	if ctx.cr[6].eq {
	pc = 0x825276D8; continue 'dispatch;
	}
	// 825276C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825276C4: 80980004  lwz r4, 4(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 825276C8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 825276CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825276D0: 4E800421  bctrl
	ctx.lr = 0x825276D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825276D4: 937F0010  stw r27, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
            }
            0x825276D8 => {
    //   block [0x825276D8..0x82527708)
	// 825276D8: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 825276DC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825276E0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825276E4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825276E8: 40980020  bge cr6, 0x82527708
	if !ctx.cr[6].lt {
	pc = 0x82527708; continue 'dispatch;
	}
	// 825276EC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825276F0: 3929FE14  addi r9, r9, -0x1ec
	ctx.r[9].s64 = ctx.r[9].s64 + -492;
	// 825276F4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825276F8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825276FC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82527700: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82527704: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82527708; continue 'dispatch;
            }
            0x82527708 => {
    //   block [0x82527708..0x82527710)
	// 82527708: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 8252770C: 4800D9EC  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82527710(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82527710 size=484
    let mut pc: u32 = 0x82527710;
    'dispatch: loop {
        match pc {
            0x82527710 => {
    //   block [0x82527710..0x82527774)
	// 82527710: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82527714: 4800D991  bl 0x825350a4
	ctx.lr = 0x82527718;
	sub_82535080(ctx, base);
	// 82527718: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252771C: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527720: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 82527724: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82527728: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 8252772C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82527730: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82527734: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82527738: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 8252773C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82527740: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527744: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82527748: 4098002C  bge cr6, 0x82527774
	if !ctx.cr[6].lt {
	pc = 0x82527774; continue 'dispatch;
	}
	// 8252774C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82527750: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82527754: 39295814  addi r9, r9, 0x5814
	ctx.r[9].s64 = ctx.r[9].s64 + 22548;
	// 82527758: 39085804  addi r8, r8, 0x5804
	ctx.r[8].s64 = ctx.r[8].s64 + 22532;
	// 8252775C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82527760: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82527764: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82527768: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 8252776C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82527770: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82527774; continue 'dispatch;
            }
            0x82527774 => {
    //   block [0x82527774..0x8252781C)
	// 82527774: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82527778: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252777C: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527780: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82527784: 396B4F5C  addi r11, r11, 0x4f5c
	ctx.r[11].s64 = ctx.r[11].s64 + 20316;
	// 82527788: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 8252778C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527790: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82527794: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82527798: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8252779C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825277A0: 815C0010  lwz r10, 0x10(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 825277A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825277A8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 825277AC: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 825277B0: 9B610068  stb r27, 0x68(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u8 ) };
	// 825277B4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825277B8: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825277BC: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 825277C0: C00B8CB4  lfs f0, -0x734c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825277C4: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 825277C8: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 825277CC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 825277D0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825277D4: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825277D8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825277DC: 4E800421  bctrl
	ctx.lr = 0x825277E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825277E0: 89610068  lbz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 825277E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825277E8: 419A00D4  beq cr6, 0x825278bc
	if ctx.cr[6].eq {
	pc = 0x825278BC; continue 'dispatch;
	}
	// 825277EC: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 825277F0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825277F4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825277F8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825277FC: 40980020  bge cr6, 0x8252781c
	if !ctx.cr[6].lt {
	pc = 0x8252781C; continue 'dispatch;
	}
	// 82527800: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82527804: 39294FF4  addi r9, r9, 0x4ff4
	ctx.r[9].s64 = ctx.r[9].s64 + 20468;
	// 82527808: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 8252780C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82527810: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82527814: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82527818: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            0x8252781C => {
    //   block [0x8252781C..0x8252785C)
	// 8252781C: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82527820: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527824: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82527828: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 8252782C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82527830: 409A0064  bne cr6, 0x82527894
	if !ctx.cr[6].eq {
	pc = 0x82527894; continue 'dispatch;
	}
	// 82527834: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527838: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 8252783C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527840: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82527844: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82527848: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252784C: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527850: 394905A0  addi r10, r9, 0x5a0
	ctx.r[10].s64 = ctx.r[9].s64 + 1440;
	// 82527854: 409A0008  bne cr6, 0x8252785c
	if !ctx.cr[6].eq {
	pc = 0x8252785C; continue 'dispatch;
	}
	// 82527858: 394901A0  addi r10, r9, 0x1a0
	ctx.r[10].s64 = ctx.r[9].s64 + 416;
	pc = 0x8252785C; continue 'dispatch;
            }
            0x8252785C => {
    //   block [0x8252785C..0x82527894)
	// 8252785C: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82527860: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82527864: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82527868: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8252786C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82527870: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82527874: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82527878: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252787C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82527880: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82527884: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82527888: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252788C: 4E800421  bctrl
	ctx.lr = 0x82527890;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82527890: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
            }
            0x82527894 => {
    //   block [0x82527894..0x825278BC)
	// 82527894: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527898: 7EE8BB78  mr r8, r23
	ctx.r[8].u64 = ctx.r[23].u64;
	// 8252789C: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 825278A0: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 825278A4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 825278A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825278AC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825278B0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 825278B4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825278B8: 4E800421  bctrl
	ctx.lr = 0x825278BC;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x825278BC => {
    //   block [0x825278BC..0x825278EC)
	// 825278BC: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 825278C0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 825278C4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825278C8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825278CC: 40980020  bge cr6, 0x825278ec
	if !ctx.cr[6].lt {
	pc = 0x825278EC; continue 'dispatch;
	}
	// 825278D0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825278D4: 3929FE14  addi r9, r9, -0x1ec
	ctx.r[9].s64 = ctx.r[9].s64 + -492;
	// 825278D8: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825278DC: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825278E0: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 825278E4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825278E8: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x825278EC; continue 'dispatch;
            }
            0x825278EC => {
    //   block [0x825278EC..0x825278F4)
	// 825278EC: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 825278F0: 4800D804  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825278F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825278F8 size=456
    let mut pc: u32 = 0x825278F8;
    'dispatch: loop {
        match pc {
            0x825278F8 => {
    //   block [0x825278F8..0x82527958)
	// 825278F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825278FC: 4800D7A9  bl 0x825350a4
	ctx.lr = 0x82527900;
	sub_82535080(ctx, base);
	// 82527900: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82527904: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527908: 3B000014  li r24, 0x14
	ctx.r[24].s64 = 20;
	// 8252790C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82527910: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82527914: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82527918: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 8252791C: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82527920: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82527924: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527928: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 8252792C: 4098002C  bge cr6, 0x82527958
	if !ctx.cr[6].lt {
	pc = 0x82527958; continue 'dispatch;
	}
	// 82527930: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82527934: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82527938: 39295814  addi r9, r9, 0x5814
	ctx.r[9].s64 = ctx.r[9].s64 + 22548;
	// 8252793C: 39085804  addi r8, r8, 0x5804
	ctx.r[8].s64 = ctx.r[8].s64 + 22532;
	// 82527940: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82527944: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82527948: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252794C: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82527950: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82527954: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82527958; continue 'dispatch;
            }
            0x82527958 => {
    //   block [0x82527958..0x82527A28)
	// 82527958: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252795C: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527960: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82527964: 3B80FFFF  li r28, -1
	ctx.r[28].s64 = -1;
	// 82527968: 392A4F5C  addi r9, r10, 0x4f5c
	ctx.r[9].s64 = ctx.r[10].s64 + 20316;
	// 8252796C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82527970: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527974: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82527978: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252797C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82527980: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82527984: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82527988: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8252798C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82527990: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527994: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82527998: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 8252799C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 825279A0: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825279A4: 8388000C  lwz r28, 0xc(r8)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 825279A8: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 825279AC: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825279B0: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 825279B4: 7D4AE0AE  lbzx r10, r10, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 825279B8: 5548103E  rotlwi r8, r10, 2
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 825279BC: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 825279C0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825279C4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 825279C8: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 825279CC: 816B09AC  lwz r11, 0x9ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2476 as u32) ) } as u64;
	// 825279D0: C00A8CB4  lfs f0, -0x734c(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825279D4: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 825279D8: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 825279DC: 9B610068  stb r27, 0x68(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u8 ) };
	// 825279E0: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 825279E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825279E8: 4E800421  bctrl
	ctx.lr = 0x825279EC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825279EC: 89610068  lbz r11, 0x68(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 825279F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 825279F4: 419A0088  beq cr6, 0x82527a7c
	if ctx.cr[6].eq {
	pc = 0x82527A7C; continue 'dispatch;
	}
	// 825279F8: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 825279FC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82527A00: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527A04: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82527A08: 40980020  bge cr6, 0x82527a28
	if !ctx.cr[6].lt {
	pc = 0x82527A28; continue 'dispatch;
	}
	// 82527A0C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82527A10: 39294FF4  addi r9, r9, 0x4ff4
	ctx.r[9].s64 = ctx.r[9].s64 + 20468;
	// 82527A14: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82527A18: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82527A1C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82527A20: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82527A24: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            0x82527A28 => {
    //   block [0x82527A28..0x82527A7C)
	// 82527A28: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82527A2C: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82527A30: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82527A34: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82527A38: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527A3C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82527A40: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82527A44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82527A48: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82527A4C: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527A50: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82527A54: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82527A58: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82527A5C: 7D4AE0AE  lbzx r10, r10, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82527A60: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82527A64: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82527A68: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82527A6C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82527A70: 816B09AC  lwz r11, 0x9ac(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2476 as u32) ) } as u64;
	// 82527A74: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82527A78: 4E800421  bctrl
	ctx.lr = 0x82527A7C;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82527A7C => {
    //   block [0x82527A7C..0x82527AB8)
	// 82527A7C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82527A80: 7D58B82E  lwzx r10, r24, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82527A84: 396B228C  addi r11, r11, 0x228c
	ctx.r[11].s64 = ctx.r[11].s64 + 8844;
	// 82527A88: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82527A8C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82527A90: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527A94: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82527A98: 40980020  bge cr6, 0x82527ab8
	if !ctx.cr[6].lt {
	pc = 0x82527AB8; continue 'dispatch;
	}
	// 82527A9C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82527AA0: 3929FE14  addi r9, r9, -0x1ec
	ctx.r[9].s64 = ctx.r[9].s64 + -492;
	// 82527AA4: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82527AA8: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82527AAC: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82527AB0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82527AB4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82527AB8; continue 'dispatch;
            }
            0x82527AB8 => {
    //   block [0x82527AB8..0x82527AC0)
	// 82527AB8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82527ABC: 4800D638  b 0x825350f4
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82527AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82527AC0 size=468
    let mut pc: u32 = 0x82527AC0;
    'dispatch: loop {
        match pc {
            0x82527AC0 => {
    //   block [0x82527AC0..0x82527B20)
	// 82527AC0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82527AC4: 4800D5E5  bl 0x825350a8
	ctx.lr = 0x82527AC8;
	sub_82535080(ctx, base);
	// 82527AC8: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82527ACC: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527AD0: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 82527AD4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82527AD8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82527ADC: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82527AE0: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82527AE4: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82527AE8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82527AEC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527AF0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82527AF4: 4098002C  bge cr6, 0x82527b20
	if !ctx.cr[6].lt {
	pc = 0x82527B20; continue 'dispatch;
	}
	// 82527AF8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82527AFC: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82527B00: 39295814  addi r9, r9, 0x5814
	ctx.r[9].s64 = ctx.r[9].s64 + 22548;
	// 82527B04: 39085804  addi r8, r8, 0x5804
	ctx.r[8].s64 = ctx.r[8].s64 + 22532;
	// 82527B08: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82527B0C: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82527B10: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82527B14: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82527B18: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82527B1C: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82527B20; continue 'dispatch;
            }
            0x82527B20 => {
    //   block [0x82527B20..0x82527BB4)
	// 82527B20: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82527B24: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82527B28: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527B2C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82527B30: 9081006C  stw r4, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[4].u32 ) };
	// 82527B34: 396B4F50  addi r11, r11, 0x4f50
	ctx.r[11].s64 = ctx.r[11].s64 + 20304;
	// 82527B38: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527B3C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82527B40: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82527B44: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82527B48: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82527B4C: 815C0010  lwz r10, 0x10(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527B50: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82527B54: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82527B58: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82527B5C: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82527B60: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82527B64: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82527B68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527B6C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527B70: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82527B74: 4E800421  bctrl
	ctx.lr = 0x82527B78;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82527B78: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82527B7C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82527B80: 419A00D0  beq cr6, 0x82527c50
	if ctx.cr[6].eq {
	pc = 0x82527C50; continue 'dispatch;
	}
	// 82527B84: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82527B88: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82527B8C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527B90: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82527B94: 40980020  bge cr6, 0x82527bb4
	if !ctx.cr[6].lt {
	pc = 0x82527BB4; continue 'dispatch;
	}
	// 82527B98: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82527B9C: 39294FF4  addi r9, r9, 0x4ff4
	ctx.r[9].s64 = ctx.r[9].s64 + 20468;
	// 82527BA0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82527BA4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82527BA8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82527BAC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82527BB0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            0x82527BB4 => {
    //   block [0x82527BB4..0x82527BF4)
	// 82527BB4: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82527BB8: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527BBC: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82527BC0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82527BC4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82527BC8: 409A0064  bne cr6, 0x82527c2c
	if !ctx.cr[6].eq {
	pc = 0x82527C2C; continue 'dispatch;
	}
	// 82527BCC: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527BD0: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527BD4: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527BD8: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82527BDC: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82527BE0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527BE4: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527BE8: 394905A0  addi r10, r9, 0x5a0
	ctx.r[10].s64 = ctx.r[9].s64 + 1440;
	// 82527BEC: 409A0008  bne cr6, 0x82527bf4
	if !ctx.cr[6].eq {
	pc = 0x82527BF4; continue 'dispatch;
	}
	// 82527BF0: 394901A0  addi r10, r9, 0x1a0
	ctx.r[10].s64 = ctx.r[9].s64 + 416;
	pc = 0x82527BF4; continue 'dispatch;
            }
            0x82527BF4 => {
    //   block [0x82527BF4..0x82527C2C)
	// 82527BF4: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82527BF8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82527BFC: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82527C00: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82527C04: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82527C08: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82527C0C: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82527C10: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82527C14: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82527C18: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82527C1C: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82527C20: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82527C24: 4E800421  bctrl
	ctx.lr = 0x82527C28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82527C28: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
            }
            0x82527C2C => {
    //   block [0x82527C2C..0x82527C50)
	// 82527C2C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527C30: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82527C34: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82527C38: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82527C3C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82527C40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527C44: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527C48: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82527C4C: 4E800421  bctrl
	ctx.lr = 0x82527C50;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82527C50 => {
    //   block [0x82527C50..0x82527C8C)
	// 82527C50: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82527C54: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82527C58: 396B4B88  addi r11, r11, 0x4b88
	ctx.r[11].s64 = ctx.r[11].s64 + 19336;
	// 82527C5C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82527C60: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82527C64: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527C68: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82527C6C: 40980020  bge cr6, 0x82527c8c
	if !ctx.cr[6].lt {
	pc = 0x82527C8C; continue 'dispatch;
	}
	// 82527C70: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82527C74: 3929FE14  addi r9, r9, -0x1ec
	ctx.r[9].s64 = ctx.r[9].s64 + -492;
	// 82527C78: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82527C7C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82527C80: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82527C84: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82527C88: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82527C8C; continue 'dispatch;
            }
            0x82527C8C => {
    //   block [0x82527C8C..0x82527C94)
	// 82527C8C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82527C90: 4800D468  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82527C98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82527C98 size=428
    let mut pc: u32 = 0x82527C98;
    'dispatch: loop {
        match pc {
            0x82527C98 => {
    //   block [0x82527C98..0x82527CF4)
	// 82527C98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82527C9C: 4800D40D  bl 0x825350a8
	ctx.lr = 0x82527CA0;
	sub_82535080(ctx, base);
	// 82527CA0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82527CA4: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527CA8: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 82527CAC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82527CB0: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82527CB4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82527CB8: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82527CBC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82527CC0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527CC4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82527CC8: 4098002C  bge cr6, 0x82527cf4
	if !ctx.cr[6].lt {
	pc = 0x82527CF4; continue 'dispatch;
	}
	// 82527CCC: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82527CD0: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82527CD4: 39295814  addi r9, r9, 0x5814
	ctx.r[9].s64 = ctx.r[9].s64 + 22548;
	// 82527CD8: 39085804  addi r8, r8, 0x5804
	ctx.r[8].s64 = ctx.r[8].s64 + 22532;
	// 82527CDC: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82527CE0: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82527CE4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82527CE8: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82527CEC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82527CF0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82527CF4; continue 'dispatch;
            }
            0x82527CF4 => {
    //   block [0x82527CF4..0x82527DB0)
	// 82527CF4: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82527CF8: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527CFC: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 82527D00: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 82527D04: 392A4F50  addi r9, r10, 0x4f50
	ctx.r[9].s64 = ctx.r[10].s64 + 20304;
	// 82527D08: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82527D0C: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527D10: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82527D14: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527D18: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82527D1C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82527D20: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82527D24: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82527D28: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82527D2C: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527D30: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 82527D34: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82527D38: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527D3C: 8388000C  lwz r28, 0xc(r8)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527D40: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82527D44: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82527D48: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82527D4C: 7D4AE0AE  lbzx r10, r10, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82527D50: 5548103E  rotlwi r8, r10, 2
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82527D54: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 82527D58: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82527D5C: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82527D60: 816B09A4  lwz r11, 0x9a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82527D64: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82527D68: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82527D6C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82527D70: 4E800421  bctrl
	ctx.lr = 0x82527D74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82527D74: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82527D78: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82527D7C: 419A0084  beq cr6, 0x82527e00
	if ctx.cr[6].eq {
	pc = 0x82527E00; continue 'dispatch;
	}
	// 82527D80: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82527D84: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82527D88: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527D8C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82527D90: 40980020  bge cr6, 0x82527db0
	if !ctx.cr[6].lt {
	pc = 0x82527DB0; continue 'dispatch;
	}
	// 82527D94: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82527D98: 39294FF4  addi r9, r9, 0x4ff4
	ctx.r[9].s64 = ctx.r[9].s64 + 20468;
	// 82527D9C: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82527DA0: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82527DA4: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82527DA8: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82527DAC: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            0x82527DB0 => {
    //   block [0x82527DB0..0x82527E00)
	// 82527DB0: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82527DB4: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82527DB8: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82527DBC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82527DC0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527DC4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82527DC8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82527DCC: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82527DD0: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527DD4: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82527DD8: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82527DDC: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82527DE0: 7D4AE0AE  lbzx r10, r10, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82527DE4: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82527DE8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82527DEC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82527DF0: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82527DF4: 816B09A8  lwz r11, 0x9a8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2472 as u32) ) } as u64;
	// 82527DF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82527DFC: 4E800421  bctrl
	ctx.lr = 0x82527E00;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82527E00 => {
    //   block [0x82527E00..0x82527E3C)
	// 82527E00: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82527E04: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82527E08: 396B4B88  addi r11, r11, 0x4b88
	ctx.r[11].s64 = ctx.r[11].s64 + 19336;
	// 82527E0C: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82527E10: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82527E14: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527E18: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82527E1C: 40980020  bge cr6, 0x82527e3c
	if !ctx.cr[6].lt {
	pc = 0x82527E3C; continue 'dispatch;
	}
	// 82527E20: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82527E24: 3929FE14  addi r9, r9, -0x1ec
	ctx.r[9].s64 = ctx.r[9].s64 + -492;
	// 82527E28: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82527E2C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82527E30: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82527E34: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82527E38: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82527E3C; continue 'dispatch;
            }
            0x82527E3C => {
    //   block [0x82527E3C..0x82527E44)
	// 82527E3C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82527E40: 4800D2B8  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82527E48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82527E48 size=468
    let mut pc: u32 = 0x82527E48;
    'dispatch: loop {
        match pc {
            0x82527E48 => {
    //   block [0x82527E48..0x82527EA8)
	// 82527E48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82527E4C: 4800D25D  bl 0x825350a8
	ctx.lr = 0x82527E50;
	sub_82535080(ctx, base);
	// 82527E50: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82527E54: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527E58: 3B400014  li r26, 0x14
	ctx.r[26].s64 = 20;
	// 82527E5C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82527E60: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82527E64: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82527E68: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82527E6C: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82527E70: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82527E74: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527E78: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82527E7C: 4098002C  bge cr6, 0x82527ea8
	if !ctx.cr[6].lt {
	pc = 0x82527EA8; continue 'dispatch;
	}
	// 82527E80: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82527E84: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 82527E88: 39295814  addi r9, r9, 0x5814
	ctx.r[9].s64 = ctx.r[9].s64 + 22548;
	// 82527E8C: 39085804  addi r8, r8, 0x5804
	ctx.r[8].s64 = ctx.r[8].s64 + 22532;
	// 82527E90: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82527E94: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 82527E98: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82527E9C: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82527EA0: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82527EA4: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82527EA8; continue 'dispatch;
            }
            0x82527EA8 => {
    //   block [0x82527EA8..0x82527F3C)
	// 82527EA8: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82527EAC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82527EB0: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527EB4: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82527EB8: 9081006C  stw r4, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[4].u32 ) };
	// 82527EBC: 396B4F50  addi r11, r11, 0x4f50
	ctx.r[11].s64 = ctx.r[11].s64 + 20304;
	// 82527EC0: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527EC4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82527EC8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82527ECC: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82527ED0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82527ED4: 815C0010  lwz r10, 0x10(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527ED8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82527EDC: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82527EE0: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82527EE4: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82527EE8: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82527EEC: 91410064  stw r10, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[10].u32 ) };
	// 82527EF0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527EF4: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527EF8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82527EFC: 4E800421  bctrl
	ctx.lr = 0x82527F00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82527F00: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82527F04: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82527F08: 419A00D0  beq cr6, 0x82527fd8
	if ctx.cr[6].eq {
	pc = 0x82527FD8; continue 'dispatch;
	}
	// 82527F0C: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82527F10: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82527F14: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527F18: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82527F1C: 40980020  bge cr6, 0x82527f3c
	if !ctx.cr[6].lt {
	pc = 0x82527F3C; continue 'dispatch;
	}
	// 82527F20: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82527F24: 39294FF4  addi r9, r9, 0x4ff4
	ctx.r[9].s64 = ctx.r[9].s64 + 20468;
	// 82527F28: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82527F2C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82527F30: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82527F34: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82527F38: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            0x82527F3C => {
    //   block [0x82527F3C..0x82527F7C)
	// 82527F3C: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82527F40: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527F44: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82527F48: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82527F4C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82527F50: 409A0064  bne cr6, 0x82527fb4
	if !ctx.cr[6].eq {
	pc = 0x82527FB4; continue 'dispatch;
	}
	// 82527F54: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527F58: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527F5C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527F60: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82527F64: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82527F68: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527F6C: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527F70: 394905A0  addi r10, r9, 0x5a0
	ctx.r[10].s64 = ctx.r[9].s64 + 1440;
	// 82527F74: 409A0008  bne cr6, 0x82527f7c
	if !ctx.cr[6].eq {
	pc = 0x82527F7C; continue 'dispatch;
	}
	// 82527F78: 394901A0  addi r10, r9, 0x1a0
	ctx.r[10].s64 = ctx.r[9].s64 + 416;
	pc = 0x82527F7C; continue 'dispatch;
            }
            0x82527F7C => {
    //   block [0x82527F7C..0x82527FB4)
	// 82527F7C: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82527F80: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82527F84: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82527F88: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82527F8C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82527F90: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82527F94: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82527F98: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82527F9C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82527FA0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82527FA4: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82527FA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82527FAC: 4E800421  bctrl
	ctx.lr = 0x82527FB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82527FB0: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
            }
            0x82527FB4 => {
    //   block [0x82527FB4..0x82527FD8)
	// 82527FB4: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82527FB8: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82527FBC: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82527FC0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82527FC4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82527FC8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82527FCC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527FD0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82527FD4: 4E800421  bctrl
	ctx.lr = 0x82527FD8;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82527FD8 => {
    //   block [0x82527FD8..0x82528014)
	// 82527FD8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82527FDC: 7D5AC82E  lwzx r10, r26, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82527FE0: 396B4B88  addi r11, r11, 0x4b88
	ctx.r[11].s64 = ctx.r[11].s64 + 19336;
	// 82527FE4: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82527FE8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82527FEC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82527FF0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82527FF4: 40980020  bge cr6, 0x82528014
	if !ctx.cr[6].lt {
	pc = 0x82528014; continue 'dispatch;
	}
	// 82527FF8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82527FFC: 3929FE14  addi r9, r9, -0x1ec
	ctx.r[9].s64 = ctx.r[9].s64 + -492;
	// 82528000: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82528004: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82528008: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 8252800C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82528010: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x82528014; continue 'dispatch;
            }
            0x82528014 => {
    //   block [0x82528014..0x8252801C)
	// 82528014: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82528018: 4800D0E0  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528020(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82528020 size=428
    let mut pc: u32 = 0x82528020;
    'dispatch: loop {
        match pc {
            0x82528020 => {
    //   block [0x82528020..0x8252807C)
	// 82528020: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82528024: 4800D085  bl 0x825350a8
	ctx.lr = 0x82528028;
	sub_82535080(ctx, base);
	// 82528028: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252802C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82528030: 3B200014  li r25, 0x14
	ctx.r[25].s64 = 20;
	// 82528034: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82528038: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 8252803C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82528040: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82528044: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82528048: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252804C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82528050: 4098002C  bge cr6, 0x8252807c
	if !ctx.cr[6].lt {
	pc = 0x8252807C; continue 'dispatch;
	}
	// 82528054: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82528058: 3D008206  lis r8, -0x7dfa
	ctx.r[8].s64 = -2113536000;
	// 8252805C: 39295814  addi r9, r9, 0x5814
	ctx.r[9].s64 = ctx.r[9].s64 + 22548;
	// 82528060: 39085804  addi r8, r8, 0x5804
	ctx.r[8].s64 = ctx.r[8].s64 + 22532;
	// 82528064: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82528068: 910B000C  stw r8, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[8].u32 ) };
	// 8252806C: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82528070: 390B0010  addi r8, r11, 0x10
	ctx.r[8].s64 = ctx.r[11].s64 + 16;
	// 82528074: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82528078: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x8252807C; continue 'dispatch;
            }
            0x8252807C => {
    //   block [0x8252807C..0x82528138)
	// 8252807C: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82528080: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82528084: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 82528088: 38E0FFFF  li r7, -1
	ctx.r[7].s64 = -1;
	// 8252808C: 392A4F50  addi r9, r10, 0x4f50
	ctx.r[9].s64 = ctx.r[10].s64 + 20304;
	// 82528090: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82528094: 811E0000  lwz r8, 0(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82528098: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 8252809C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 825280A0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 825280A4: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 825280A8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 825280AC: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 825280B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 825280B4: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 825280B8: 90E10064  stw r7, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[7].u32 ) };
	// 825280BC: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 825280C0: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825280C4: 8388000C  lwz r28, 0xc(r8)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 825280C8: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 825280CC: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825280D0: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 825280D4: 7D4AE0AE  lbzx r10, r10, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 825280D8: 5548103E  rotlwi r8, r10, 2
	ctx.r[8].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 825280DC: 7D4A4214  add r10, r10, r8
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[8].u64;
	// 825280E0: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 825280E4: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 825280E8: 816B09A4  lwz r11, 0x9a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2468 as u32) ) } as u64;
	// 825280EC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 825280F0: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 825280F4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825280F8: 4E800421  bctrl
	ctx.lr = 0x825280FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825280FC: 89610054  lbz r11, 0x54(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82528100: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82528104: 419A0084  beq cr6, 0x82528188
	if ctx.cr[6].eq {
	pc = 0x82528188; continue 'dispatch;
	}
	// 82528108: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 8252810C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82528110: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82528114: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82528118: 40980020  bge cr6, 0x82528138
	if !ctx.cr[6].lt {
	pc = 0x82528138; continue 'dispatch;
	}
	// 8252811C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82528120: 39294FF4  addi r9, r9, 0x4ff4
	ctx.r[9].s64 = ctx.r[9].s64 + 20468;
	// 82528124: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82528128: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 8252812C: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 82528130: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82528134: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
            }
            0x82528138 => {
    //   block [0x82528138..0x82528188)
	// 82528138: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 8252813C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82528140: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82528144: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82528148: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252814C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82528150: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82528154: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82528158: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 8252815C: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82528160: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82528164: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82528168: 7D4AE0AE  lbzx r10, r10, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 8252816C: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82528170: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82528174: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82528178: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 8252817C: 816B09A4  lwz r11, 0x9a4(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82528180: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82528184: 4E800421  bctrl
	ctx.lr = 0x82528188;
	crate::rt::call_indirect(ctx.ctr.u32);
            }
            0x82528188 => {
    //   block [0x82528188..0x825281C4)
	// 82528188: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252818C: 7D59C02E  lwzx r10, r25, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82528190: 396B4B88  addi r11, r11, 0x4b88
	ctx.r[11].s64 = ctx.r[11].s64 + 19336;
	// 82528194: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82528198: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252819C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 825281A0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 825281A4: 40980020  bge cr6, 0x825281c4
	if !ctx.cr[6].lt {
	pc = 0x825281C4; continue 'dispatch;
	}
	// 825281A8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825281AC: 3929FE14  addi r9, r9, -0x1ec
	ctx.r[9].s64 = ctx.r[9].s64 + -492;
	// 825281B0: 912B0000  stw r9, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 825281B4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 825281B8: 390B000C  addi r8, r11, 0xc
	ctx.r[8].s64 = ctx.r[11].s64 + 12;
	// 825281BC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 825281C0: 910A0004  stw r8, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	pc = 0x825281C4; continue 'dispatch;
            }
            0x825281C4 => {
    //   block [0x825281C4..0x825281CC)
	// 825281C4: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 825281C8: 4800CF30  b 0x825350f8
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825281D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825281D0 size=108
    let mut pc: u32 = 0x825281D0;
    'dispatch: loop {
        match pc {
            0x825281D0 => {
    //   block [0x825281D0..0x8252823C)
	// 825281D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825281D4: 4800CEE1  bl 0x825350b4
	ctx.lr = 0x825281D8;
	sub_82535080(ctx, base);
	// 825281D8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825281DC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 825281E0: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 825281E4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 825281E8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 825281EC: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 825281F0: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 825281F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 825281F8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 825281FC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82528200: 4BF3BE39  bl 0x82464038
	ctx.lr = 0x82528204;
	sub_82464038(ctx, base);
	// 82528204: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82528208: 39600014  li r11, 0x14
	ctx.r[11].s64 = 20;
	// 8252820C: 7F67DB78  mr r7, r27
	ctx.r[7].u64 = ctx.r[27].u64;
	// 82528210: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82528214: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82528218: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 8252821C: B17F0004  sth r11, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u16 ) };
	// 82528220: 4BFFF1D1  bl 0x825273f0
	ctx.lr = 0x82528224;
	sub_825273F0(ctx, base);
	// 82528224: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528228: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 8252822C: 396B5828  addi r11, r11, 0x5828
	ctx.r[11].s64 = ctx.r[11].s64 + 22568;
	// 82528230: 917F0000  stw r11, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82528234: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82528238: 4800CECC  b 0x82535104
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528240(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82528240 size=204
    let mut pc: u32 = 0x82528240;
    'dispatch: loop {
        match pc {
            0x82528240 => {
    //   block [0x82528240..0x8252830C)
	// 82528240: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82528244: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82528248: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252824C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82528250: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82528254: 3D008253  lis r8, -0x7dad
	ctx.r[8].s64 = -2108489728;
	// 82528258: 3D208253  lis r9, -0x7dad
	ctx.r[9].s64 = -2108489728;
	// 8252825C: 3D408253  lis r10, -0x7dad
	ctx.r[10].s64 = -2108489728;
	// 82528260: 3D608253  lis r11, -0x7dad
	ctx.r[11].s64 = -2108489728;
	// 82528264: 390881D0  addi r8, r8, -0x7e30
	ctx.r[8].s64 = ctx.r[8].s64 + -32304;
	// 82528268: 39298310  addi r9, r9, -0x7cf0
	ctx.r[9].s64 = ctx.r[9].s64 + -31984;
	// 8252826C: 394A8360  addi r10, r10, -0x7ca0
	ctx.r[10].s64 = ctx.r[10].s64 + -31904;
	// 82528270: 396B83B0  addi r11, r11, -0x7c50
	ctx.r[11].s64 = ctx.r[11].s64 + -31824;
	// 82528274: 3BC00001  li r30, 1
	ctx.r[30].s64 = 1;
	// 82528278: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 8252827C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82528280: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82528284: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82528288: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 8252828C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82528290: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82528294: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82528298: 9BC10060  stb r30, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[30].u8 ) };
	// 8252829C: 9BC10061  stb r30, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[30].u8 ) };
	// 825282A0: 4BFD7AE9  bl 0x824ffd88
	ctx.lr = 0x825282A4;
	sub_824FFD88(ctx, base);
	// 825282A4: 3D608252  lis r11, -0x7dae
	ctx.r[11].s64 = -2108555264;
	// 825282A8: 9BC10081  stb r30, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[30].u8 ) };
	// 825282AC: 3D008252  lis r8, -0x7dae
	ctx.r[8].s64 = -2108555264;
	// 825282B0: 396B78F8  addi r11, r11, 0x78f8
	ctx.r[11].s64 = ctx.r[11].s64 + 30968;
	// 825282B4: 3D208253  lis r9, -0x7dad
	ctx.r[9].s64 = -2108489728;
	// 825282B8: 3D408252  lis r10, -0x7dae
	ctx.r[10].s64 = -2108555264;
	// 825282BC: 390874C8  addi r8, r8, 0x74c8
	ctx.r[8].s64 = ctx.r[8].s64 + 29896;
	// 825282C0: 39298020  addi r9, r9, -0x7fe0
	ctx.r[9].s64 = ctx.r[9].s64 + -32736;
	// 825282C4: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 825282C8: 394A7C98  addi r10, r10, 0x7c98
	ctx.r[10].s64 = ctx.r[10].s64 + 31896;
	// 825282CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825282D0: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 825282D4: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 825282D8: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 825282DC: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 825282E0: 91210074  stw r9, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[9].u32 ) };
	// 825282E4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825282E8: 91410078  stw r10, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[10].u32 ) };
	// 825282EC: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 825282F0: 4BFD7A99  bl 0x824ffd88
	ctx.lr = 0x825282F4;
	sub_824FFD88(ctx, base);
	// 825282F4: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 825282F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825282FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82528300: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82528304: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82528308: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528310(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82528310 size=76
    let mut pc: u32 = 0x82528310;
    'dispatch: loop {
        match pc {
            0x82528310 => {
    //   block [0x82528310..0x8252835C)
	// 82528310: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82528314: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82528318: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252831C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82528320: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82528324: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82528328: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 8252832C: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82528330: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 82528334: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82528338: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 8252833C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82528340: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82528344: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 82528348: 4BFFFCD9  bl 0x82528020
	ctx.lr = 0x8252834C;
	sub_82528020(ctx, base);
	// 8252834C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82528350: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82528354: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82528358: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82528360 size=80
    let mut pc: u32 = 0x82528360;
    'dispatch: loop {
        match pc {
            0x82528360 => {
    //   block [0x82528360..0x825283B0)
	// 82528360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82528364: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82528368: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252836C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82528370: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82528374: 7CCA3378  mr r10, r6
	ctx.r[10].u64 = ctx.r[6].u64;
	// 82528378: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 8252837C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82528380: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82528384: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82528388: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 8252838C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82528390: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82528394: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82528398: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 8252839C: 4BFFF8FD  bl 0x82527c98
	ctx.lr = 0x825283A0;
	sub_82527C98(ctx, base);
	// 825283A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825283A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825283A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825283AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825283B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825283B0 size=232
    let mut pc: u32 = 0x825283B0;
    'dispatch: loop {
        match pc {
            0x825283B0 => {
    //   block [0x825283B0..0x825283DC)
	// 825283B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825283B4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825283B8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825283BC: 9421FED0  stwu r1, -0x130(r1)
	ea = ctx.r[1].u32.wrapping_add(-304 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825283C0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 825283C4: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 825283C8: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 825283CC: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 825283D0: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 825283D4: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 825283D8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x825283DC; continue 'dispatch;
            }
            0x825283DC => {
    //   block [0x825283DC..0x82528498)
	// 825283DC: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 825283E0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 825283E4: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 825283E8: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 825283EC: 4200FFF0  bdnz 0x825283dc
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x825283DC; continue 'dispatch;
	}
	// 825283F0: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825283F4: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 825283F8: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 825283FC: 3CE08201  lis r7, -0x7dff
	ctx.r[7].s64 = -2113863680;
	// 82528400: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82528404: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82528498 size=16
    let mut pc: u32 = 0x82528498;
    'dispatch: loop {
        match pc {
            0x82528498 => {
    //   block [0x82528498..0x825284A8)
	// 82528498: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8252849C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825284A0: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 825284A4: 4BFFEEAC  b 0x82527350
	sub_82527350(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825284A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825284A8 size=76
    let mut pc: u32 = 0x825284A8;
    'dispatch: loop {
        match pc {
            0x825284A8 => {
    //   block [0x825284A8..0x825284F4)
	// 825284A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825284AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825284B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825284B4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825284B8: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 825284BC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825284C0: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 825284C4: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 825284C8: 39294BAC  addi r9, r9, 0x4bac
	ctx.r[9].s64 = ctx.r[9].s64 + 19372;
	// 825284CC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 825284D0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 825284D4: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 825284D8: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 825284DC: 99610054  stb r11, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u8 ) };
	// 825284E0: 4BFFF969  bl 0x82527e48
	ctx.lr = 0x825284E4;
	sub_82527E48(ctx, base);
	// 825284E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825284E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825284EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825284F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825284F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x825284F8 size=80
    let mut pc: u32 = 0x825284F8;
    'dispatch: loop {
        match pc {
            0x825284F8 => {
    //   block [0x825284F8..0x82528548)
	// 825284F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825284FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82528500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82528504: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82528508: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8252850C: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82528510: 39294B94  addi r9, r9, 0x4b94
	ctx.r[9].s64 = ctx.r[9].s64 + 19348;
	// 82528514: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82528518: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8252851C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82528520: C0088CB4  lfs f0, -0x734c(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-29516 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82528524: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82528528: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 8252852C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82528530: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82528534: 4BFFF58D  bl 0x82527ac0
	ctx.lr = 0x82528538;
	sub_82527AC0(ctx, base);
	// 82528538: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 8252853C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82528540: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82528544: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82528548 size=176
    let mut pc: u32 = 0x82528548;
    'dispatch: loop {
        match pc {
            0x82528548 => {
    //   block [0x82528548..0x825285C8)
	// 82528548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252854C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82528550: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82528554: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82528558: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 8252855C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82528560: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82528564: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82528568: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 8252856C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82528570: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82528574: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82528578: 4BFFEFA9  bl 0x82527520
	ctx.lr = 0x8252857C;
	sub_82527520(ctx, base);
	// 8252857C: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82528580: 7F1F5840  cmplw cr6, r31, r11
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[11].u32, &mut ctx.xer);
	// 82528584: 40980044  bge cr6, 0x825285c8
	if !ctx.cr[6].lt {
	pc = 0x825285C8; continue 'dispatch;
	}
	// 82528588: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252858C: 394B08F0  addi r10, r11, 0x8f0
	ctx.r[10].s64 = ctx.r[11].s64 + 2288;
	// 82528590: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
	pc = 0x825285C8; continue 'dispatch;
            }
            0x825285C8 => {
    //   block [0x825285C8..0x825285F8)
	// 825285C8: C01E3030  lfs f0, 0x3030(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 825285CC: FF1F0000  fcmpu cr6, f31, f0
	ctx.cr[6].compare_f64(ctx.f[31].f64, ctx.f[0].f64);
	// 825285D0: 419A000C  beq cr6, 0x825285dc
	if ctx.cr[6].eq {
	pc = 0x825285DC; continue 'dispatch;
	}
	// 825285D4: 387E3010  addi r3, r30, 0x3010
	ctx.r[3].s64 = ctx.r[30].s64 + 12304;
	// 825285D8: 4BFEAE69  bl 0x82513440
	ctx.lr = 0x825285DC;
	sub_82513440(ctx, base);
	// 825285DC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825285E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825285E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825285E8: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 825285EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825285F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825285F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825285F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825285F8 size=240
    let mut pc: u32 = 0x825285F8;
    'dispatch: loop {
        match pc {
            0x825285F8 => {
    //   block [0x825285F8..0x82528628)
	// 825285F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825285FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82528600: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82528604: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82528608: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 8252860C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82528610: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82528614: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82528618: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 8252861C: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82528620: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82528624: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	pc = 0x82528628; continue 'dispatch;
            }
            0x82528628 => {
    //   block [0x82528628..0x825286E8)
	// 82528628: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 8252862C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82528630: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82528634: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82528638: 4200FFF0  bdnz 0x82528628
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82528628; continue 'dispatch;
	}
	// 8252863C: 3D208206  lis r9, -0x7dfa
	ctx.r[9].s64 = -2113536000;
	// 82528640: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82528644: 3929FA60  addi r9, r9, -0x5a0
	ctx.r[9].s64 = ctx.r[9].s64 + -1440;
	// 82528648: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 8252864C: 39410060  addi r10, r1, 0x60
	ctx.r[10].s64 = ctx.r[1].s64 + 96;
	// 82528650: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825286E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825286E8 size=292
    let mut pc: u32 = 0x825286E8;
    'dispatch: loop {
        match pc {
            0x825286E8 => {
    //   block [0x825286E8..0x8252880C)
	// 825286E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825286EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825286F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825286F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825286F8: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 825286FC: 38800002  li r4, 2
	ctx.r[4].s64 = 2;
	// 82528700: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82528704: 4BFD7A3D  bl 0x82500140
	ctx.lr = 0x82528708;
	sub_82500140(ctx, base);
	// 82528708: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8252870C: 38800004  li r4, 4
	ctx.r[4].s64 = 4;
	// 82528710: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82528714: 4BFD7A2D  bl 0x82500140
	ctx.lr = 0x82528718;
	sub_82500140(ctx, base);
	// 82528718: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8252871C: 38800005  li r4, 5
	ctx.r[4].s64 = 5;
	// 82528720: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82528724: 4BFD7A1D  bl 0x82500140
	ctx.lr = 0x82528728;
	sub_82500140(ctx, base);
	// 82528728: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8252872C: 38800006  li r4, 6
	ctx.r[4].s64 = 6;
	// 82528730: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82528734: 4BFD7A0D  bl 0x82500140
	ctx.lr = 0x82528738;
	sub_82500140(ctx, base);
	// 82528738: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8252873C: 38800003  li r4, 3
	ctx.r[4].s64 = 3;
	// 82528740: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82528744: 4BFD79FD  bl 0x82500140
	ctx.lr = 0x82528748;
	sub_82500140(ctx, base);
	// 82528748: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8252874C: 38800007  li r4, 7
	ctx.r[4].s64 = 7;
	// 82528750: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82528754: 4BFD79ED  bl 0x82500140
	ctx.lr = 0x82528758;
	sub_82500140(ctx, base);
	// 82528758: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8252875C: 3880000C  li r4, 0xc
	ctx.r[4].s64 = 12;
	// 82528760: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82528764: 4BFD79DD  bl 0x82500140
	ctx.lr = 0x82528768;
	sub_82500140(ctx, base);
	// 82528768: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8252876C: 3880000D  li r4, 0xd
	ctx.r[4].s64 = 13;
	// 82528770: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82528774: 4BFD79CD  bl 0x82500140
	ctx.lr = 0x82528778;
	sub_82500140(ctx, base);
	// 82528778: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 8252877C: 38800013  li r4, 0x13
	ctx.r[4].s64 = 19;
	// 82528780: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82528784: 4BFD79BD  bl 0x82500140
	ctx.lr = 0x82528788;
	sub_82500140(ctx, base);
	// 82528788: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8252878C: 38800016  li r4, 0x16
	ctx.r[4].s64 = 22;
	// 82528790: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82528794: 4BFD79AD  bl 0x82500140
	ctx.lr = 0x82528798;
	sub_82500140(ctx, base);
	// 82528798: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 8252879C: 3880000A  li r4, 0xa
	ctx.r[4].s64 = 10;
	// 825287A0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825287A4: 4BFD799D  bl 0x82500140
	ctx.lr = 0x825287A8;
	sub_82500140(ctx, base);
	// 825287A8: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 825287AC: 3880000F  li r4, 0xf
	ctx.r[4].s64 = 15;
	// 825287B0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825287B4: 4BFD798D  bl 0x82500140
	ctx.lr = 0x825287B8;
	sub_82500140(ctx, base);
	// 825287B8: 38A00009  li r5, 9
	ctx.r[5].s64 = 9;
	// 825287BC: 3880000B  li r4, 0xb
	ctx.r[4].s64 = 11;
	// 825287C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825287C4: 4BFD797D  bl 0x82500140
	ctx.lr = 0x825287C8;
	sub_82500140(ctx, base);
	// 825287C8: 38A00019  li r5, 0x19
	ctx.r[5].s64 = 25;
	// 825287CC: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 825287D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825287D4: 4BFD796D  bl 0x82500140
	ctx.lr = 0x825287D8;
	sub_82500140(ctx, base);
	// 825287D8: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 825287DC: 3880001B  li r4, 0x1b
	ctx.r[4].s64 = 27;
	// 825287E0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825287E4: 4BFD795D  bl 0x82500140
	ctx.lr = 0x825287E8;
	sub_82500140(ctx, base);
	// 825287E8: 38A00018  li r5, 0x18
	ctx.r[5].s64 = 24;
	// 825287EC: 3880000E  li r4, 0xe
	ctx.r[4].s64 = 14;
	// 825287F0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 825287F4: 4BFD794D  bl 0x82500140
	ctx.lr = 0x825287F8;
	sub_82500140(ctx, base);
	// 825287F8: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 825287FC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82528800: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82528804: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82528808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528810(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82528810 size=580
    let mut pc: u32 = 0x82528810;
    'dispatch: loop {
        match pc {
            0x82528810 => {
    //   block [0x82528810..0x825288BC)
	// 82528810: 39630001  addi r11, r3, 1
	ctx.r[11].s64 = ctx.r[3].s64 + 1;
	// 82528814: 2B0B0021  cmplwi cr6, r11, 0x21
	ctx.cr[6].compare_u32(ctx.r[11].u32, 33 as u32, &mut ctx.xer);
	// 82528818: 4199023C  bgt cr6, 0x82528a54
	if ctx.cr[6].gt {
		sub_82528A54(ctx, base);
		return;
	}
	// 8252881C: 3D808253  lis r12, -0x7dad
	ctx.r[12].s64 = -2108489728;
	// 82528820: 398C8834  addi r12, r12, -0x77cc
	ctx.r[12].s64 = ctx.r[12].s64 + -30668;
	// 82528824: 5560103A  slwi r0, r11, 2
	ctx.r[0].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[0].u64 = ctx.r[0].u32 as u64;
	// 82528828: 7C0C002E  lwzx r0, r12, r0
	ctx.r[0].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[12].u32.wrapping_add(ctx.r[0].u32)) } as u64;
	// 8252882C: 7C0903A6  mtctr r0
	ctx.ctr.u64 = ctx.r[0].u64;
	// 82528830: 4E800420  bctr
	match ctx.r[11].u64 {
		0 => {
	pc = 0x825288C8; continue 'dispatch;
		},
		1 => {
	pc = 0x825288BC; continue 'dispatch;
		},
		2 => {
	pc = 0x825288D4; continue 'dispatch;
		},
		3 => {
	pc = 0x825288F8; continue 'dispatch;
		},
		4 => {
	pc = 0x82528928; continue 'dispatch;
		},
		5 => {
	pc = 0x82528904; continue 'dispatch;
		},
		6 => {
	pc = 0x82528910; continue 'dispatch;
		},
		7 => {
	pc = 0x8252891C; continue 'dispatch;
		},
		8 => {
	pc = 0x82528934; continue 'dispatch;
		},
		9 => {
	pc = 0x825288E0; continue 'dispatch;
		},
		10 => {
	pc = 0x825288EC; continue 'dispatch;
		},
		11 => {
	pc = 0x82528958; continue 'dispatch;
		},
		12 => {
	pc = 0x825289C4; continue 'dispatch;
		},
		13 => {
	pc = 0x825289DC; continue 'dispatch;
		},
		14 => {
	pc = 0x825289E8; continue 'dispatch;
		},
		15 => {
	pc = 0x82528994; continue 'dispatch;
		},
		16 => {
	pc = 0x825289F4; continue 'dispatch;
		},
		17 => {
	pc = 0x82528A18; continue 'dispatch;
		},
		18 => {
	pc = 0x82528A00; continue 'dispatch;
		},
		19 => {
	pc = 0x82528A0C; continue 'dispatch;
		},
		20 => {
	pc = 0x82528940; continue 'dispatch;
		},
		21 => {
	pc = 0x8252894C; continue 'dispatch;
		},
		22 => {
	pc = 0x82528964; continue 'dispatch;
		},
		23 => {
	pc = 0x82528970; continue 'dispatch;
		},
		24 => {
	pc = 0x8252897C; continue 'dispatch;
		},
		25 => {
	pc = 0x82528988; continue 'dispatch;
		},
		26 => {
	pc = 0x825289A0; continue 'dispatch;
		},
		27 => {
	pc = 0x825289AC; continue 'dispatch;
		},
		28 => {
	pc = 0x825289B8; continue 'dispatch;
		},
		29 => {
	pc = 0x825289D0; continue 'dispatch;
		},
		30 => {
	pc = 0x82528A24; continue 'dispatch;
		},
		31 => {
	pc = 0x82528A30; continue 'dispatch;
		},
		32 => {
	pc = 0x82528A3C; continue 'dispatch;
		},
		33 => {
	pc = 0x82528A48; continue 'dispatch;
		},
		_ => unsafe { core::hint::unreachable_unchecked() },
	}
	// 82528834: 825288C8  lwz r18, -0x7738(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30520 as u32) ) } as u64;
	// 82528838: 825288BC  lwz r18, -0x7744(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30532 as u32) ) } as u64;
	// 8252883C: 825288D4  lwz r18, -0x772c(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30508 as u32) ) } as u64;
	// 82528840: 825288F8  lwz r18, -0x7708(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30472 as u32) ) } as u64;
	// 82528844: 82528928  lwz r18, -0x76d8(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30424 as u32) ) } as u64;
	// 82528848: 82528904  lwz r18, -0x76fc(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30460 as u32) ) } as u64;
	// 8252884C: 82528910  lwz r18, -0x76f0(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30448 as u32) ) } as u64;
	// 82528850: 8252891C  lwz r18, -0x76e4(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30436 as u32) ) } as u64;
	// 82528854: 82528934  lwz r18, -0x76cc(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30412 as u32) ) } as u64;
	// 82528858: 825288E0  lwz r18, -0x7720(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30496 as u32) ) } as u64;
	// 8252885C: 825288EC  lwz r18, -0x7714(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30484 as u32) ) } as u64;
	// 82528860: 82528958  lwz r18, -0x76a8(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30376 as u32) ) } as u64;
	// 82528864: 825289C4  lwz r18, -0x763c(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30268 as u32) ) } as u64;
	// 82528868: 825289DC  lwz r18, -0x7624(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30244 as u32) ) } as u64;
	// 8252886C: 825289E8  lwz r18, -0x7618(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30232 as u32) ) } as u64;
	// 82528870: 82528994  lwz r18, -0x766c(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30316 as u32) ) } as u64;
	// 82528874: 825289F4  lwz r18, -0x760c(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30220 as u32) ) } as u64;
	// 82528878: 82528A18  lwz r18, -0x75e8(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30184 as u32) ) } as u64;
	// 8252887C: 82528A00  lwz r18, -0x7600(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30208 as u32) ) } as u64;
	// 82528880: 82528A0C  lwz r18, -0x75f4(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30196 as u32) ) } as u64;
	// 82528884: 82528940  lwz r18, -0x76c0(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30400 as u32) ) } as u64;
	// 82528888: 8252894C  lwz r18, -0x76b4(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30388 as u32) ) } as u64;
	// 8252888C: 82528964  lwz r18, -0x769c(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30364 as u32) ) } as u64;
	// 82528890: 82528970  lwz r18, -0x7690(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30352 as u32) ) } as u64;
	// 82528894: 8252897C  lwz r18, -0x7684(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30340 as u32) ) } as u64;
	// 82528898: 82528988  lwz r18, -0x7678(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30328 as u32) ) } as u64;
	// 8252889C: 825289A0  lwz r18, -0x7660(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30304 as u32) ) } as u64;
	// 825288A0: 825289AC  lwz r18, -0x7654(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30292 as u32) ) } as u64;
	// 825288A4: 825289B8  lwz r18, -0x7648(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30280 as u32) ) } as u64;
	// 825288A8: 825289D0  lwz r18, -0x7630(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30256 as u32) ) } as u64;
	// 825288AC: 82528A24  lwz r18, -0x75dc(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30172 as u32) ) } as u64;
	// 825288B0: 82528A30  lwz r18, -0x75d0(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30160 as u32) ) } as u64;
	// 825288B4: 82528A3C  lwz r18, -0x75c4(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30148 as u32) ) } as u64;
	// 825288B8: 82528A48  lwz r18, -0x75b8(r18)
	ctx.r[18].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[18].u32.wrapping_add(-30136 as u32) ) } as u64;
            }
            0x825288BC => {
    //   block [0x825288BC..0x825288C8)
	// 825288BC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825288C0: 386B5B20  addi r3, r11, 0x5b20
	ctx.r[3].s64 = ctx.r[11].s64 + 23328;
	// 825288C4: 4E800020  blr
	return;
            }
            0x825288C8 => {
    //   block [0x825288C8..0x825288D4)
	// 825288C8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825288CC: 386B5B10  addi r3, r11, 0x5b10
	ctx.r[3].s64 = ctx.r[11].s64 + 23312;
	// 825288D0: 4E800020  blr
	return;
            }
            0x825288D4 => {
    //   block [0x825288D4..0x825288E0)
	// 825288D4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825288D8: 386B5B00  addi r3, r11, 0x5b00
	ctx.r[3].s64 = ctx.r[11].s64 + 23296;
	// 825288DC: 4E800020  blr
	return;
            }
            0x825288E0 => {
    //   block [0x825288E0..0x825288EC)
	// 825288E0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825288E4: 386B5AEC  addi r3, r11, 0x5aec
	ctx.r[3].s64 = ctx.r[11].s64 + 23276;
	// 825288E8: 4E800020  blr
	return;
            }
            0x825288EC => {
    //   block [0x825288EC..0x825288F8)
	// 825288EC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825288F0: 386B5AD8  addi r3, r11, 0x5ad8
	ctx.r[3].s64 = ctx.r[11].s64 + 23256;
	// 825288F4: 4E800020  blr
	return;
            }
            0x825288F8 => {
    //   block [0x825288F8..0x82528904)
	// 825288F8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825288FC: 386B5AC8  addi r3, r11, 0x5ac8
	ctx.r[3].s64 = ctx.r[11].s64 + 23240;
	// 82528900: 4E800020  blr
	return;
            }
            0x82528904 => {
    //   block [0x82528904..0x82528910)
	// 82528904: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528908: 386B5AB4  addi r3, r11, 0x5ab4
	ctx.r[3].s64 = ctx.r[11].s64 + 23220;
	// 8252890C: 4E800020  blr
	return;
            }
            0x82528910 => {
    //   block [0x82528910..0x8252891C)
	// 82528910: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528914: 386B5AA4  addi r3, r11, 0x5aa4
	ctx.r[3].s64 = ctx.r[11].s64 + 23204;
	// 82528918: 4E800020  blr
	return;
            }
            0x8252891C => {
    //   block [0x8252891C..0x82528928)
	// 8252891C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528920: 386B5A90  addi r3, r11, 0x5a90
	ctx.r[3].s64 = ctx.r[11].s64 + 23184;
	// 82528924: 4E800020  blr
	return;
            }
            0x82528928 => {
    //   block [0x82528928..0x82528934)
	// 82528928: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252892C: 386B5A7C  addi r3, r11, 0x5a7c
	ctx.r[3].s64 = ctx.r[11].s64 + 23164;
	// 82528930: 4E800020  blr
	return;
            }
            0x82528934 => {
    //   block [0x82528934..0x82528940)
	// 82528934: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528938: 386B5A60  addi r3, r11, 0x5a60
	ctx.r[3].s64 = ctx.r[11].s64 + 23136;
	// 8252893C: 4E800020  blr
	return;
            }
            0x82528940 => {
    //   block [0x82528940..0x8252894C)
	// 82528940: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528944: 386B5A48  addi r3, r11, 0x5a48
	ctx.r[3].s64 = ctx.r[11].s64 + 23112;
	// 82528948: 4E800020  blr
	return;
            }
            0x8252894C => {
    //   block [0x8252894C..0x82528958)
	// 8252894C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528950: 386B5A30  addi r3, r11, 0x5a30
	ctx.r[3].s64 = ctx.r[11].s64 + 23088;
	// 82528954: 4E800020  blr
	return;
            }
            0x82528958 => {
    //   block [0x82528958..0x82528964)
	// 82528958: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252895C: 386B5A20  addi r3, r11, 0x5a20
	ctx.r[3].s64 = ctx.r[11].s64 + 23072;
	// 82528960: 4E800020  blr
	return;
            }
            0x82528964 => {
    //   block [0x82528964..0x82528970)
	// 82528964: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528968: 386B5A08  addi r3, r11, 0x5a08
	ctx.r[3].s64 = ctx.r[11].s64 + 23048;
	// 8252896C: 4E800020  blr
	return;
            }
            0x82528970 => {
    //   block [0x82528970..0x8252897C)
	// 82528970: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528974: 386B59E8  addi r3, r11, 0x59e8
	ctx.r[3].s64 = ctx.r[11].s64 + 23016;
	// 82528978: 4E800020  blr
	return;
            }
            0x8252897C => {
    //   block [0x8252897C..0x82528988)
	// 8252897C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528980: 386B59D4  addi r3, r11, 0x59d4
	ctx.r[3].s64 = ctx.r[11].s64 + 22996;
	// 82528984: 4E800020  blr
	return;
            }
            0x82528988 => {
    //   block [0x82528988..0x82528994)
	// 82528988: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 8252898C: 386B59BC  addi r3, r11, 0x59bc
	ctx.r[3].s64 = ctx.r[11].s64 + 22972;
	// 82528990: 4E800020  blr
	return;
            }
            0x82528994 => {
    //   block [0x82528994..0x825289A0)
	// 82528994: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528998: 386B599C  addi r3, r11, 0x599c
	ctx.r[3].s64 = ctx.r[11].s64 + 22940;
	// 8252899C: 4E800020  blr
	return;
            }
            0x825289A0 => {
    //   block [0x825289A0..0x825289AC)
	// 825289A0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825289A4: 386B5988  addi r3, r11, 0x5988
	ctx.r[3].s64 = ctx.r[11].s64 + 22920;
	// 825289A8: 4E800020  blr
	return;
            }
            0x825289AC => {
    //   block [0x825289AC..0x825289B8)
	// 825289AC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825289B0: 386B597C  addi r3, r11, 0x597c
	ctx.r[3].s64 = ctx.r[11].s64 + 22908;
	// 825289B4: 4E800020  blr
	return;
            }
            0x825289B8 => {
    //   block [0x825289B8..0x825289C4)
	// 825289B8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825289BC: 386B596C  addi r3, r11, 0x596c
	ctx.r[3].s64 = ctx.r[11].s64 + 22892;
	// 825289C0: 4E800020  blr
	return;
            }
            0x825289C4 => {
    //   block [0x825289C4..0x825289D0)
	// 825289C4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825289C8: 386B595C  addi r3, r11, 0x595c
	ctx.r[3].s64 = ctx.r[11].s64 + 22876;
	// 825289CC: 4E800020  blr
	return;
            }
            0x825289D0 => {
    //   block [0x825289D0..0x825289DC)
	// 825289D0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825289D4: 386B5948  addi r3, r11, 0x5948
	ctx.r[3].s64 = ctx.r[11].s64 + 22856;
	// 825289D8: 4E800020  blr
	return;
            }
            0x825289DC => {
    //   block [0x825289DC..0x825289E8)
	// 825289DC: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825289E0: 386B592C  addi r3, r11, 0x592c
	ctx.r[3].s64 = ctx.r[11].s64 + 22828;
	// 825289E4: 4E800020  blr
	return;
            }
            0x825289E8 => {
    //   block [0x825289E8..0x825289F4)
	// 825289E8: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825289EC: 386B5910  addi r3, r11, 0x5910
	ctx.r[3].s64 = ctx.r[11].s64 + 22800;
	// 825289F0: 4E800020  blr
	return;
            }
            0x825289F4 => {
    //   block [0x825289F4..0x82528A00)
	// 825289F4: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825289F8: 386B58F8  addi r3, r11, 0x58f8
	ctx.r[3].s64 = ctx.r[11].s64 + 22776;
	// 825289FC: 4E800020  blr
	return;
            }
            0x82528A00 => {
    //   block [0x82528A00..0x82528A0C)
	// 82528A00: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528A04: 386B58D8  addi r3, r11, 0x58d8
	ctx.r[3].s64 = ctx.r[11].s64 + 22744;
	// 82528A08: 4E800020  blr
	return;
            }
            0x82528A0C => {
    //   block [0x82528A0C..0x82528A18)
	// 82528A0C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528A10: 386B58C0  addi r3, r11, 0x58c0
	ctx.r[3].s64 = ctx.r[11].s64 + 22720;
	// 82528A14: 4E800020  blr
	return;
            }
            0x82528A18 => {
    //   block [0x82528A18..0x82528A24)
	// 82528A18: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528A1C: 386B58AC  addi r3, r11, 0x58ac
	ctx.r[3].s64 = ctx.r[11].s64 + 22700;
	// 82528A20: 4E800020  blr
	return;
            }
            0x82528A24 => {
    //   block [0x82528A24..0x82528A30)
	// 82528A24: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528A28: 386B5890  addi r3, r11, 0x5890
	ctx.r[3].s64 = ctx.r[11].s64 + 22672;
	// 82528A2C: 4E800020  blr
	return;
            }
            0x82528A30 => {
    //   block [0x82528A30..0x82528A3C)
	// 82528A30: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528A34: 386B5880  addi r3, r11, 0x5880
	ctx.r[3].s64 = ctx.r[11].s64 + 22656;
	// 82528A38: 4E800020  blr
	return;
            }
            0x82528A3C => {
    //   block [0x82528A3C..0x82528A48)
	// 82528A3C: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528A40: 386B5870  addi r3, r11, 0x5870
	ctx.r[3].s64 = ctx.r[11].s64 + 22640;
	// 82528A44: 4E800020  blr
	return;
            }
            0x82528A48 => {
    //   block [0x82528A48..0x82528A54)
	// 82528A48: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528A4C: 386B5860  addi r3, r11, 0x5860
	ctx.r[3].s64 = ctx.r[11].s64 + 22624;
	// 82528A50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528A54(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82528A54 size=8
    let mut pc: u32 = 0x82528A54;
    'dispatch: loop {
        match pc {
            0x82528A54 => {
    //   block [0x82528A54..0x82528A5C)
	// 82528A54: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82528A58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528A60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82528A60 size=144
    let mut pc: u32 = 0x82528A60;
    'dispatch: loop {
        match pc {
            0x82528A60 => {
    //   block [0x82528A60..0x82528AC8)
	// 82528A60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82528A64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82528A68: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82528A6C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82528A70: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82528A74: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82528A78: 3BC5FFE0  addi r30, r5, -0x20
	ctx.r[30].s64 = ctx.r[5].s64 + -32;
	// 82528A7C: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82528A80: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82528A84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82528A88: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82528A8C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82528A90: 4E800421  bctrl
	ctx.lr = 0x82528A94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82528A94: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82528A98: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82528A9C: 41980038  blt cr6, 0x82528ad4
	if ctx.cr[6].lt {
	pc = 0x82528AD4; continue 'dispatch;
	}
	// 82528AA0: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82528AA4: 41990030  bgt cr6, 0x82528ad4
	if ctx.cr[6].gt {
	pc = 0x82528AD4; continue 'dispatch;
	}
	// 82528AA8: 815F0014  lwz r10, 0x14(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82528AAC: 393F0020  addi r9, r31, 0x20
	ctx.r[9].s64 = ctx.r[31].s64 + 32;
	// 82528AB0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82528AB4: 409A0014  bne cr6, 0x82528ac8
	if !ctx.cr[6].eq {
	pc = 0x82528AC8; continue 'dispatch;
	}
	// 82528AB8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82528ABC: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82528AC0: 915F0018  stw r10, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[10].u32 ) };
	// 82528AC4: 48000014  b 0x82528ad8
	pc = 0x82528AD8; continue 'dispatch;
            }
            0x82528AC8 => {
    //   block [0x82528AC8..0x82528AD4)
	// 82528AC8: 38600020  li r3, 0x20
	ctx.r[3].s64 = 32;
	// 82528ACC: 917F0018  stw r11, 0x18(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(24 as u32), ctx.r[11].u32 ) };
	// 82528AD0: 48000008  b 0x82528ad8
	pc = 0x82528AD8; continue 'dispatch;
            }
            0x82528AD4 => {
    //   block [0x82528AD4..0x82528AD8)
	// 82528AD4: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	pc = 0x82528AD8; continue 'dispatch;
            }
            0x82528AD8 => {
    //   block [0x82528AD8..0x82528AF0)
	// 82528AD8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82528ADC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82528AE0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82528AE4: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82528AE8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82528AEC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528AF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82528AF8 size=204
    let mut pc: u32 = 0x82528AF8;
    'dispatch: loop {
        match pc {
            0x82528AF8 => {
    //   block [0x82528AF8..0x82528BC4)
	// 82528AF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82528AFC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82528B00: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82528B04: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528BC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82528BC8 size=20
    let mut pc: u32 = 0x82528BC8;
    'dispatch: loop {
        match pc {
            0x82528BC8 => {
    //   block [0x82528BC8..0x82528BDC)
	// 82528BC8: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82528BCC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82528BD0: 816B0014  lwz r11, 0x14(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82528BD4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82528BD8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528BE0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82528BE0 size=20
    let mut pc: u32 = 0x82528BE0;
    'dispatch: loop {
        match pc {
            0x82528BE0 => {
    //   block [0x82528BE0..0x82528BF4)
	// 82528BE0: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82528BE4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82528BE8: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82528BEC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82528BF0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528BF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82528BF8 size=76
    let mut pc: u32 = 0x82528BF8;
    'dispatch: loop {
        match pc {
            0x82528BF8 => {
    //   block [0x82528BF8..0x82528C44)
	// 82528BF8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82528BFC: 4800C4C1  bl 0x825350bc
	ctx.lr = 0x82528C00;
	sub_82535080(ctx, base);
	// 82528C00: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82528C04: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82528C08: 80640014  lwz r3, 0x14(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82528C0C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82528C10: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82528C14: 4BFE2E8D  bl 0x8250baa0
	ctx.lr = 0x82528C18;
	sub_8250BAA0(ctx, base);
	// 82528C18: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82528C1C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82528C20: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82528C24: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82528C28: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82528C2C: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82528C30: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82528C34: 4E800421  bctrl
	ctx.lr = 0x82528C38;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82528C38: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82528C3C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82528C40: 4800C4CC  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82528C48 size=68
    let mut pc: u32 = 0x82528C48;
    'dispatch: loop {
        match pc {
            0x82528C48 => {
    //   block [0x82528C48..0x82528C8C)
	// 82528C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82528C4C: 4800C471  bl 0x825350bc
	ctx.lr = 0x82528C50;
	sub_82535080(ctx, base);
	// 82528C50: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82528C54: 80630014  lwz r3, 0x14(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82528C58: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82528C5C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82528C60: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82528C64: 4BFE2E3D  bl 0x8250baa0
	ctx.lr = 0x82528C68;
	sub_8250BAA0(ctx, base);
	// 82528C68: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82528C6C: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82528C70: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82528C74: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82528C78: 816B0010  lwz r11, 0x10(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82528C7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82528C80: 4E800421  bctrl
	ctx.lr = 0x82528C84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82528C84: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82528C88: 4800C484  b 0x8253510c
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82528C90 size=84
    let mut pc: u32 = 0x82528C90;
    'dispatch: loop {
        match pc {
            0x82528C90 => {
    //   block [0x82528C90..0x82528CDC)
	// 82528C90: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528C94: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 82528C98: 396B417C  addi r11, r11, 0x417c
	ctx.r[11].s64 = ctx.r[11].s64 + 16764;
	// 82528C9C: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82528CA0: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82528CA4: 38E00009  li r7, 9
	ctx.r[7].s64 = 9;
	// 82528CA8: 394A2A68  addi r10, r10, 0x2a68
	ctx.r[10].s64 = ctx.r[10].s64 + 10856;
	// 82528CAC: 91630000  stw r11, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 82528CB0: B1230006  sth r9, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82528CB4: 91030008  stw r8, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[8].u32 ) };
	// 82528CB8: 90E3000C  stw r7, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82528CBC: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82528CC0: 90830014  stw r4, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[4].u32 ) };
	// 82528CC4: A1640004  lhz r11, 4(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(4 as u32) ) } as u64;
	// 82528CC8: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82528CCC: 419A0010  beq cr6, 0x82528cdc
	if ctx.cr[6].eq {
	pc = 0x82528CDC; continue 'dispatch;
	}
	// 82528CD0: A1640006  lhz r11, 6(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[4].u32.wrapping_add(6 as u32) ) } as u64;
	// 82528CD4: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82528CD8: B1640006  sth r11, 6(r4)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[4].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	pc = 0x82528CDC; continue 'dispatch;
            }
            0x82528CDC => {
    //   block [0x82528CDC..0x82528CE4)
	// 82528CDC: 98A3001C  stb r5, 0x1c(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[5].u8 ) };
	// 82528CE0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82528CE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82528CE8 size=960
    let mut pc: u32 = 0x82528CE8;
    'dispatch: loop {
        match pc {
            0x82528CE8 => {
    //   block [0x82528CE8..0x825290A8)
	// 82528CE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82528CEC: 4800C3A1  bl 0x8253508c
	ctx.lr = 0x82528CF0;
	sub_82535080(ctx, base);
	// 82528CF0: DBC1FF70  stfd f30, -0x90(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-144 as u32), ctx.f[30].u64 ) };
	// 82528CF4: DBE1FF78  stfd f31, -0x88(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[31].u64 ) };
	// 82528CF8: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82528CFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82528D00: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82528D04: 7CB22B78  mr r18, r5
	ctx.r[18].u64 = ctx.r[5].u64;
	// 82528D08: 7CD53378  mr r21, r6
	ctx.r[21].u64 = ctx.r[6].u64;
	// 82528D0C: 807E0014  lwz r3, 0x14(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82528D10: 4BFE2D91  bl 0x8250baa0
	ctx.lr = 0x82528D14;
	sub_8250BAA0(ctx, base);
	// 82528D14: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 82528D18: 815E0014  lwz r10, 0x14(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825290A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825290A8 size=1136
    let mut pc: u32 = 0x825290A8;
    'dispatch: loop {
        match pc {
            0x825290A8 => {
    //   block [0x825290A8..0x82529518)
	// 825290A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825290AC: 4800BFE5  bl 0x82535090
	ctx.lr = 0x825290B0;
	sub_82535080(ctx, base);
	// 825290B0: DBC1FF78  stfd f30, -0x88(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-136 as u32), ctx.f[30].u64 ) };
	// 825290B4: DBE1FF80  stfd f31, -0x80(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-128 as u32), ctx.f[31].u64 ) };
	// 825290B8: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825290BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 825290C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 825290C4: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 825290C8: 807D0014  lwz r3, 0x14(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
	// 825290CC: 4BFE29D5  bl 0x8250baa0
	ctx.lr = 0x825290D0;
	sub_8250BAA0(ctx, base);
	// 825290D0: 3D608206  lis r11, -0x7dfa
	ctx.r[11].s64 = -2113536000;
	// 825290D4: 815D0014  lwz r10, 0x14(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(20 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82529518 size=4
    let mut pc: u32 = 0x82529518;
    'dispatch: loop {
        match pc {
            0x82529518 => {
    //   block [0x82529518..0x8252951C)
	// 82529518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82529520 size=4
    let mut pc: u32 = 0x82529520;
    'dispatch: loop {
        match pc {
            0x82529520 => {
    //   block [0x82529520..0x82529524)
	// 82529520: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82529528 size=28
    let mut pc: u32 = 0x82529528;
    'dispatch: loop {
        match pc {
            0x82529528 => {
    //   block [0x82529528..0x82529544)
	// 82529528: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8252952C: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82529530: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529534: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529538: 816B001C  lwz r11, 0x1c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 8252953C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82529540: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82529548 size=40
    let mut pc: u32 = 0x82529548;
    'dispatch: loop {
        match pc {
            0x82529548 => {
    //   block [0x82529548..0x82529570)
	// 82529548: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8252954C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82529550: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82529554: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82529558: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 8252955C: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529560: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529564: 816B0020  lwz r11, 0x20(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82529568: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252956C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82529570 size=40
    let mut pc: u32 = 0x82529570;
    'dispatch: loop {
        match pc {
            0x82529570 => {
    //   block [0x82529570..0x82529598)
	// 82529570: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82529574: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82529578: 3D408206  lis r10, -0x7dfa
	ctx.r[10].s64 = -2113536000;
	// 8252957C: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82529580: 388A5B50  addi r4, r10, 0x5b50
	ctx.r[4].s64 = ctx.r[10].s64 + 23376;
	// 82529584: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529588: 80CB0000  lwz r6, 0(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252958C: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82529590: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82529594: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529598(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82529598 size=28
    let mut pc: u32 = 0x82529598;
    'dispatch: loop {
        match pc {
            0x82529598 => {
    //   block [0x82529598..0x825295B4)
	// 82529598: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 8252959C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825295A0: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825295A4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825295A8: 816B0024  lwz r11, 0x24(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 825295AC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825295B0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825295B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825295B8 size=24
    let mut pc: u32 = 0x825295B8;
    'dispatch: loop {
        match pc {
            0x825295B8 => {
    //   block [0x825295B8..0x825295D0)
	// 825295B8: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 825295BC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 825295C0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825295C4: 816B0028  lwz r11, 0x28(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 825295C8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825295CC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825295D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825295D0 size=28
    let mut pc: u32 = 0x825295D0;
    'dispatch: loop {
        match pc {
            0x825295D0 => {
    //   block [0x825295D0..0x825295EC)
	// 825295D0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825295D4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825295D8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825295DC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 825295E0: 816B002C  lwz r11, 0x2c(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 825295E4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825295E8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825295F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825295F0 size=28
    let mut pc: u32 = 0x825295F0;
    'dispatch: loop {
        match pc {
            0x825295F0 => {
    //   block [0x825295F0..0x8252960C)
	// 825295F0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 825295F4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 825295F8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 825295FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529600: 816B0030  lwz r11, 0x30(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82529604: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82529608: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529610(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82529610 size=28
    let mut pc: u32 = 0x82529610;
    'dispatch: loop {
        match pc {
            0x82529610 => {
    //   block [0x82529610..0x8252962C)
	// 82529610: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82529614: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82529618: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252961C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529620: 816B0034  lwz r11, 0x34(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82529624: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82529628: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529630(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82529630 size=180
    let mut pc: u32 = 0x82529630;
    'dispatch: loop {
        match pc {
            0x82529630 => {
    //   block [0x82529630..0x82529688)
	// 82529630: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82529634: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82529638: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 8252963C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82529640: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82529644: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82529648: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 8252964C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82529650: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82529654: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529658: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 8252965C: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82529660: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529664: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529668: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 8252966C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82529670: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82529674: 81050010  lwz r8, 0x10(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82529678: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 8252967C: 390905A0  addi r8, r9, 0x5a0
	ctx.r[8].s64 = ctx.r[9].s64 + 1440;
	// 82529680: 409A0008  bne cr6, 0x82529688
	if !ctx.cr[6].eq {
	pc = 0x82529688; continue 'dispatch;
	}
	// 82529684: 390901A0  addi r8, r9, 0x1a0
	ctx.r[8].s64 = ctx.r[9].s64 + 416;
	pc = 0x82529688; continue 'dispatch;
            }
            0x82529688 => {
    //   block [0x82529688..0x825296E4)
	// 82529688: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 8252968C: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82529690: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82529694: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82529698: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 8252969C: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 825296A0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 825296A4: 816B09A0  lwz r11, 0x9a0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2464 as u32) ) } as u64;
	// 825296A8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 825296AC: 4E800421  bctrl
	ctx.lr = 0x825296B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 825296B0: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 825296B4: 39400006  li r10, 6
	ctx.r[10].s64 = 6;
	// 825296B8: 392000FF  li r9, 0xff
	ctx.r[9].s64 = 255;
	// 825296BC: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 825296C0: 917E0000  stw r11, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[11].u32 ) };
	// 825296C4: 995F0000  stb r10, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[10].u8 ) };
	// 825296C8: 993F0002  stb r9, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[9].u8 ) };
	// 825296CC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 825296D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 825296D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 825296D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 825296DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 825296E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825296E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x825296E8 size=80
    let mut pc: u32 = 0x825296E8;
    'dispatch: loop {
        match pc {
            0x825296E8 => {
    //   block [0x825296E8..0x82529738)
	// 825296E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 825296EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 825296F0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 825296F4: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 825296F8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 825296FC: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82529700: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529704: 80CB0008  lwz r6, 8(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82529708: 80AB0004  lwz r5, 4(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 8252970C: 808B0000  lwz r4, 0(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529710: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82529714: 816B0018  lwz r11, 0x18(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82529718: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 8252971C: 4E800421  bctrl
	ctx.lr = 0x82529720;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82529720: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82529724: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82529728: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 8252972C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82529730: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82529734: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82529738 size=184
    let mut pc: u32 = 0x82529738;
    'dispatch: loop {
        match pc {
            0x82529738 => {
    //   block [0x82529738..0x825297F0)
	// 82529738: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 8252973C: 4800B975  bl 0x825350b0
	ctx.lr = 0x82529740;
	sub_82535080(ctx, base);
	// 82529740: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82529744: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82529748: 3FE08253  lis r31, -0x7dad
	ctx.r[31].s64 = -2108489728;
	// 8252974C: 3CE08253  lis r7, -0x7dad
	ctx.r[7].s64 = -2108489728;
	// 82529750: 3D008253  lis r8, -0x7dad
	ctx.r[8].s64 = -2108489728;
	// 82529754: 3D208253  lis r9, -0x7dad
	ctx.r[9].s64 = -2108489728;
	// 82529758: 99610081  stb r11, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[11].u8 ) };
	// 8252975C: 3D408253  lis r10, -0x7dad
	ctx.r[10].s64 = -2108489728;
	// 82529760: 99610082  stb r11, 0x82(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(130 as u32), ctx.r[11].u8 ) };
	// 82529764: 3F408253  lis r26, -0x7dad
	ctx.r[26].s64 = -2108489728;
	// 82529768: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 8252976C: 3F608253  lis r27, -0x7dad
	ctx.r[27].s64 = -2108489728;
	// 82529770: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82529774: 397F9528  addi r11, r31, -0x6ad8
	ctx.r[11].s64 = ctx.r[31].s64 + -27352;
	// 82529778: 3F808253  lis r28, -0x7dad
	ctx.r[28].s64 = -2108489728;
	// 8252977C: 3FA08253  lis r29, -0x7dad
	ctx.r[29].s64 = -2108489728;
	// 82529780: 3FC08253  lis r30, -0x7dad
	ctx.r[30].s64 = -2108489728;
	// 82529784: 3B5A95F0  addi r26, r26, -0x6a10
	ctx.r[26].s64 = ctx.r[26].s64 + -27152;
	// 82529788: 91610054  stw r11, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[11].u32 ) };
	// 8252978C: 39679548  addi r11, r7, -0x6ab8
	ctx.r[11].s64 = ctx.r[7].s64 + -27320;
	// 82529790: 3B7B9610  addi r27, r27, -0x69f0
	ctx.r[27].s64 = ctx.r[27].s64 + -27120;
	// 82529794: 3B9C95D0  addi r28, r28, -0x6a30
	ctx.r[28].s64 = ctx.r[28].s64 + -27184;
	// 82529798: 3BBD9630  addi r29, r29, -0x69d0
	ctx.r[29].s64 = ctx.r[29].s64 + -27088;
	// 8252979C: 3BDE96E8  addi r30, r30, -0x6918
	ctx.r[30].s64 = ctx.r[30].s64 + -26904;
	// 825297A0: 93410060  stw r26, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[26].u32 ) };
	// 825297A4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 825297A8: 39689570  addi r11, r8, -0x6a90
	ctx.r[11].s64 = ctx.r[8].s64 + -27280;
	// 825297AC: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 825297B0: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 825297B4: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 825297B8: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 825297BC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 825297C0: 93A10050  stw r29, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[29].u32 ) };
	// 825297C4: 93C1007C  stw r30, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 825297C8: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 825297CC: 39699598  addi r11, r9, -0x6a68
	ctx.r[11].s64 = ctx.r[9].s64 + -27240;
	// 825297D0: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 825297D4: 396A95B8  addi r11, r10, -0x6a48
	ctx.r[11].s64 = ctx.r[10].s64 + -27208;
	// 825297D8: 91610074  stw r11, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[11].u32 ) };
	// 825297DC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 825297E0: 99610080  stb r11, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u8 ) };
	// 825297E4: 4BFD6785  bl 0x824fff68
	ctx.lr = 0x825297E8;
	sub_824FFF68(ctx, base);
	// 825297E8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 825297EC: 4800B914  b 0x82535100
	sub_825350D0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825297F0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825297F0 size=4
    let mut pc: u32 = 0x825297F0;
    'dispatch: loop {
        match pc {
            0x825297F0 => {
    //   block [0x825297F0..0x825297F4)
	// 825297F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_825297F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x825297F8 size=4
    let mut pc: u32 = 0x825297F8;
    'dispatch: loop {
        match pc {
            0x825297F8 => {
    //   block [0x825297F8..0x825297FC)
	// 825297F8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529800(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82529800 size=4
    let mut pc: u32 = 0x82529800;
    'dispatch: loop {
        match pc {
            0x82529800 => {
    //   block [0x82529800..0x82529804)
	// 82529800: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82529808(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82529808 size=4
    let mut pc: u32 = 0x82529808;
    'dispatch: loop {
        match pc {
            0x82529808 => {
    //   block [0x82529808..0x8252980C)
	// 82529808: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


