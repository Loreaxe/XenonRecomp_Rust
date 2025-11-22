pub fn sub_82F414A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F414A8 size=76
    let mut pc: u32 = 0x82F414A8;
    'dispatch: loop {
        match pc {
            0x82F414A8 => {
    //   block [0x82F414A8..0x82F414F4)
	// 82F414A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F414AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F414B0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F414B4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F414B8: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F414BC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F414C0: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F414C4: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F414C8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F414CC: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F414D0: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F414D4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F414D8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F414DC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F414E0: 4BFFF839  bl 0x82f40d18
	ctx.lr = 0x82F414E4;
	sub_82F40D18(ctx, base);
	// 82F414E4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F414E8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F414EC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F414F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F414F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F414F8 size=76
    let mut pc: u32 = 0x82F414F8;
    'dispatch: loop {
        match pc {
            0x82F414F8 => {
    //   block [0x82F414F8..0x82F41544)
	// 82F414F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F414FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F41500: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F41504: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F41508: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F4150C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F41510: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F41514: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F41518: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F4151C: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F41520: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F41524: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F41528: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F4152C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F41530: 4BFFFB01  bl 0x82f41030
	ctx.lr = 0x82F41534;
	sub_82F41030(ctx, base);
	// 82F41534: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F41538: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4153C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F41540: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F41548(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F41548 size=176
    let mut pc: u32 = 0x82F41548;
    'dispatch: loop {
        match pc {
            0x82F41548 => {
    //   block [0x82F41548..0x82F415F8)
	// 82F41548: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4154C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F41550: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F41554: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F41558: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F4155C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F41560: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F41564: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F41568: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F4156C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F41570: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F41574: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F41578: 4BFFF419  bl 0x82f40990
	ctx.lr = 0x82F4157C;
	sub_82F40990(ctx, base);
	// 82F4157C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F41580: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F41584: 40980044  bge cr6, 0x82f415c8
	if !ctx.cr[6].lt {
	pc = 0x82F415C8; continue 'dispatch;
	}
	// 82F41588: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F4158C: 394BC5A0  addi r10, r11, -0x3a60
	ctx.r[10].s64 = ctx.r[11].s64 + -14944;
	// 82F41590: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F415F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F415F8 size=116
    let mut pc: u32 = 0x82F415F8;
    'dispatch: loop {
        match pc {
            0x82F415F8 => {
    //   block [0x82F415F8..0x82F4166C)
	// 82F415F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F415FC: 48266B71  bl 0x831a816c
	ctx.lr = 0x82F41600;
	sub_831A8130(ctx, base);
	// 82F41600: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F41604: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F41608: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4160C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F41610: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F41614: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82F41618: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F4161C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F41620: 4BF5F111  bl 0x82ea0730
	ctx.lr = 0x82F41624;
	sub_82EA0730(ctx, base);
	// 82F41624: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F41628: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4162C: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82F41630: 38E9FBE4  addi r7, r9, -0x41c
	ctx.r[7].s64 = ctx.r[9].s64 + -1052;
	// 82F41634: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F41638: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F4163C: B11F0004  sth r8, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82F41640: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82F41644: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82F41648: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82F4164C: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F41650: B0BF000C  sth r5, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[5].u16 ) };
	// 82F41654: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F41658: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82F4165C: 4804688D  bl 0x82f87ee8
	ctx.lr = 0x82F41660;
	sub_82F87EE8(ctx, base);
	// 82F41660: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F41664: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F41668: 48266B54  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F41670(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F41670 size=104
    let mut pc: u32 = 0x82F41670;
    'dispatch: loop {
        match pc {
            0x82F41670 => {
    //   block [0x82F41670..0x82F416D8)
	// 82F41670: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F41674: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F41678: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4167C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F41680: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F41684: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F41688: A17F000C  lhz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4168C: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 82F41690: 419A001C  beq cr6, 0x82f416ac
	if ctx.cr[6].eq {
	pc = 0x82F416AC; continue 'dispatch;
	}
	// 82F41694: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F41698: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F4169C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F416A0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F416A4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F416A8: 4E800421  bctrl
	ctx.lr = 0x82F416AC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F416AC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F416B0: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F416B4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F416B8: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F416BC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F416C0: 4E800421  bctrl
	ctx.lr = 0x82F416C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F416C4: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F416C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F416CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F416D0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F416D4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F416D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F416D8 size=128
    let mut pc: u32 = 0x82F416D8;
    'dispatch: loop {
        match pc {
            0x82F416D8 => {
    //   block [0x82F416D8..0x82F41758)
	// 82F416D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F416DC: 48266A91  bl 0x831a816c
	ctx.lr = 0x82F416E0;
	sub_831A8130(ctx, base);
	// 82F416E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F416E4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F416E8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F416EC: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F416F0: 38800020  li r4, 0x20
	ctx.r[4].s64 = 32;
	// 82F416F4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F416F8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F416FC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F41700: 4BF5F031  bl 0x82ea0730
	ctx.lr = 0x82F41704;
	sub_82EA0730(ctx, base);
	// 82F41704: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F41708: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4170C: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82F41710: 38E9FBE4  addi r7, r9, -0x41c
	ctx.r[7].s64 = ctx.r[9].s64 + -1052;
	// 82F41714: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F41718: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F4171C: B11F0004  sth r8, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82F41720: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82F41724: 389F0010  addi r4, r31, 0x10
	ctx.r[4].s64 = ctx.r[31].s64 + 16;
	// 82F41728: 93BF0008  stw r29, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[29].u32 ) };
	// 82F4172C: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F41730: B0BF000C  sth r5, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[5].u16 ) };
	// 82F41734: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F41738: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82F4173C: 480467AD  bl 0x82f87ee8
	ctx.lr = 0x82F41740;
	sub_82F87EE8(ctx, base);
	// 82F41740: 3C808213  lis r4, -0x7ded
	ctx.r[4].s64 = -2112684032;
	// 82F41744: 3864FC20  addi r3, r4, -0x3e0
	ctx.r[3].s64 = ctx.r[4].s64 + -992;
	// 82F41748: 907F0000  stw r3, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82F4174C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F41750: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F41754: 48266A68  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F41758(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F41758 size=304
    let mut pc: u32 = 0x82F41758;
    'dispatch: loop {
        match pc {
            0x82F41758 => {
    //   block [0x82F41758..0x82F41888)
	// 82F41758: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4175C: 482669F9  bl 0x831a8154
	ctx.lr = 0x82F41760;
	sub_831A8130(ctx, base);
	// 82F41760: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F41764: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F41768: 3B600018  li r27, 0x18
	ctx.r[27].s64 = 24;
	// 82F4176C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F41770: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F41774: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82F41778: 7D7BE02E  lwzx r11, r27, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82F4177C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F41780: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F41784: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F41788: 40980020  bge cr6, 0x82f417a8
	if !ctx.cr[6].lt {
	pc = 0x82F417A8; continue 'dispatch;
	}
	// 82F4178C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F41790: 3909FC58  addi r8, r9, -0x3a8
	ctx.r[8].s64 = ctx.r[9].s64 + -936;
	// 82F41794: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F41798: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F4179C: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F417A0: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F417A4: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F417A8: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F417AC: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82F417B0: 813F0008  lwz r9, 8(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F417B4: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82F417B8: 38E00020  li r7, 0x20
	ctx.r[7].s64 = 32;
	// 82F417BC: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F417C0: 3CC08212  lis r6, -0x7dee
	ctx.r[6].s64 = -2112749568;
	// 82F417C4: 833F0000  lwz r25, 0(r31)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F417C8: 3B010050  addi r24, r1, 0x50
	ctx.r[24].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F41888(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F41888 size=320
    let mut pc: u32 = 0x82F41888;
    'dispatch: loop {
        match pc {
            0x82F41888 => {
    //   block [0x82F41888..0x82F419C8)
	// 82F41888: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4188C: 482668CD  bl 0x831a8158
	ctx.lr = 0x82F41890;
	sub_831A8130(ctx, base);
	// 82F41890: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F41894: 836D0000  lwz r27, 0(r13)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F41898: 3B400018  li r26, 0x18
	ctx.r[26].s64 = 24;
	// 82F4189C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F418A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F418A4: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82F418A8: 7D7AD82E  lwzx r11, r26, r27
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[26].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82F418AC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F418B0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F418B4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F418B8: 40980020  bge cr6, 0x82f418d8
	if !ctx.cr[6].lt {
	pc = 0x82F418D8; continue 'dispatch;
	}
	// 82F418BC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F418C0: 3909FC58  addi r8, r9, -0x3a8
	ctx.r[8].s64 = ctx.r[9].s64 + -936;
	// 82F418C4: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F418C8: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F418CC: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F418D0: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F418D4: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F418D8: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F418DC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F418E0: 831F0000  lwz r24, 0(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F418E4: 3BBC0020  addi r29, r28, 0x20
	ctx.r[29].s64 = ctx.r[28].s64 + 32;
	// 82F418E8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F418EC: 480465FD  bl 0x82f87ee8
	ctx.lr = 0x82F418F0;
	sub_82F87EE8(ctx, base);
	// 82F418F0: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82F418F4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F418F8: 3908FF90  addi r8, r8, -0x70
	ctx.r[8].s64 = ctx.r[8].s64 + -112;
	// 82F418FC: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82F41900: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82F41904: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82F41908: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F419C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F419C8 size=364
    let mut pc: u32 = 0x82F419C8;
    'dispatch: loop {
        match pc {
            0x82F419C8 => {
    //   block [0x82F419C8..0x82F41B34)
	// 82F419C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F419CC: 4826677D  bl 0x831a8148
	ctx.lr = 0x82F419D0;
	sub_831A8130(ctx, base);
	// 82F419D0: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F419D4: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F419D8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F419DC: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F419E0: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F419E4: 83630000  lwz r27, 0(r3)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F419E8: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F419EC: 83E40000  lwz r31, 0(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F419F0: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82F419F4: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82F419F8: E86B0000  ld r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F419FC: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F41A00: EB2B0008  ld r25, 8(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F41A04: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82F41A08: EB0B0010  ld r24, 0x10(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F41A0C: 3B410080  addi r26, r1, 0x80
	ctx.r[26].s64 = ctx.r[1].s64 + 128;
	// 82F41A10: EAEB0018  ld r23, 0x18(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F41A14: 3BCA0030  addi r30, r10, 0x30
	ctx.r[30].s64 = ctx.r[10].s64 + 48;
	// 82F41A18: EACB0020  ld r22, 0x20(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F41A1C: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82F41A20: EAAB0028  ld r21, 0x28(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F41A24: 390100D0  addi r8, r1, 0xd0
	ctx.r[8].s64 = ctx.r[1].s64 + 208;
	// 82F41A28: EA8B0030  ld r20, 0x30(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F41A2C: 393F0040  addi r9, r31, 0x40
	ctx.r[9].s64 = ctx.r[31].s64 + 64;
	// 82F41A30: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F41A34: F8670000  std r3, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 82F41A38: FB270008  std r25, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[25].u64 ) };
	// 82F41A3C: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82F41A40: FB060000  std r24, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
	// 82F41A44: FAE60008  std r23, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[23].u64 ) };
	// 82F41A48: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82F41A4C: F9640008  std r11, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F41A50: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F41A54: FA840000  std r20, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82F41A58: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82F41A5C: FADA0000  std r22, 0(r26)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[26].u32.wrapping_add(0 as u32), ctx.r[22].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F41B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F41B38 size=476
    let mut pc: u32 = 0x82F41B38;
    'dispatch: loop {
        match pc {
            0x82F41B38 => {
    //   block [0x82F41B38..0x82F41D14)
	// 82F41B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F41B3C: 48266601  bl 0x831a813c
	ctx.lr = 0x82F41B40;
	sub_831A8130(ctx, base);
	// 82F41B40: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F41B44: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F41B48: 3B600018  li r27, 0x18
	ctx.r[27].s64 = 24;
	// 82F41B4C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F41B50: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F41B54: 7D7BE02E  lwzx r11, r27, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82F41B58: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F41B5C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F41B60: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F41B64: 40980020  bge cr6, 0x82f41b84
	if !ctx.cr[6].lt {
	pc = 0x82F41B84; continue 'dispatch;
	}
	// 82F41B68: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F41B6C: 3909FC58  addi r8, r9, -0x3a8
	ctx.r[8].s64 = ctx.r[9].s64 + -936;
	// 82F41B70: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F41B74: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F41B78: 392A000C  addi r9, r10, 0xc
	ctx.r[9].s64 = ctx.r[10].s64 + 12;
	// 82F41B7C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F41B80: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82F41B84: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F41B88: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82F41B8C: 90A100B4  stw r5, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[5].u32 ) };
	// 82F41B90: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82F41B94: 83E50000  lwz r31, 0(r5)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F41B98: 38AB0010  addi r5, r11, 0x10
	ctx.r[5].s64 = ctx.r[11].s64 + 16;
	// 82F41B9C: 908100B0  stw r4, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[4].u32 ) };
	// 82F41BA0: 3B010060  addi r24, r1, 0x60
	ctx.r[24].s64 = ctx.r[1].s64 + 96;
	// 82F41BA4: 3AE10070  addi r23, r1, 0x70
	ctx.r[23].s64 = ctx.r[1].s64 + 112;
	// 82F41BA8: 81240008  lwz r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F41BAC: E8AB0010  ld r5, 0x10(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F41BB0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82F41BB4: EACB0018  ld r22, 0x18(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F41BB8: 3BC90030  addi r30, r9, 0x30
	ctx.r[30].s64 = ctx.r[9].s64 + 48;
	// 82F41BBC: EA2B0000  ld r17, 0(r11)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F41BC0: 39010110  addi r8, r1, 0x110
	ctx.r[8].s64 = ctx.r[1].s64 + 272;
	// 82F41BC4: EAAB0020  ld r21, 0x20(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F41BC8: 393F0040  addi r9, r31, 0x40
	ctx.r[9].s64 = ctx.r[31].s64 + 64;
	// 82F41BCC: EA8B0028  ld r20, 0x28(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F41BD0: EA6B0030  ld r19, 0x30(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F41BD4: EA4B0038  ld r18, 0x38(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F41BD8: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F41BDC: F9670008  std r11, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F41BE0: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 82F41BE4: F8A60000  std r5, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[5].u64 ) };
	// 82F41BE8: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F41BEC: FA270000  std r17, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[17].u64 ) };
	// 82F41BF0: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82F41BF4: FAC60008  std r22, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[22].u64 ) };
	// 82F41BF8: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F41BFC: FAB80000  std r21, 0(r24)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[24].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F41D18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F41D18 size=500
    let mut pc: u32 = 0x82F41D18;
    'dispatch: loop {
        match pc {
            0x82F41D18 => {
    //   block [0x82F41D18..0x82F41F0C)
	// 82F41D18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F41D1C: 48266425  bl 0x831a8140
	ctx.lr = 0x82F41D20;
	sub_831A8130(ctx, base);
	// 82F41D20: 9421FE50  stwu r1, -0x1b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-432 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F41D24: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F41D28: 3B600018  li r27, 0x18
	ctx.r[27].s64 = 24;
	// 82F41D2C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F41D30: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F41D34: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82F41D38: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82F41D3C: 7D7BE02E  lwzx r11, r27, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82F41D40: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F41D44: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F41D48: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F41D4C: 40980020  bge cr6, 0x82f41d6c
	if !ctx.cr[6].lt {
	pc = 0x82F41D6C; continue 'dispatch;
	}
	// 82F41D50: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F41D54: 3909FC58  addi r8, r9, -0x3a8
	ctx.r[8].s64 = ctx.r[9].s64 + -936;
	// 82F41D58: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F41D5C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F41D60: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F41D64: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F41D68: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F41D6C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F41D70: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F41D74: 386B0020  addi r3, r11, 0x20
	ctx.r[3].s64 = ctx.r[11].s64 + 32;
	// 82F41D78: 48046171  bl 0x82f87ee8
	ctx.lr = 0x82F41D7C;
	sub_82F87EE8(ctx, base);
	// 82F41D7C: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F41D80: 93C100C0  stw r30, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[30].u32 ) };
	// 82F41D84: 38C100D0  addi r6, r1, 0xd0
	ctx.r[6].s64 = ctx.r[1].s64 + 208;
	// 82F41D88: 93E100C4  stw r31, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[31].u32 ) };
	// 82F41D8C: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82F41D90: 80FE0008  lwz r7, 8(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F41D94: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82F41D98: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F41D9C: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F41DA0: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82F41DA4: 39010120  addi r8, r1, 0x120
	ctx.r[8].s64 = ctx.r[1].s64 + 288;
	// 82F41DA8: EB0B0010  ld r24, 0x10(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F41DAC: EAEB0018  ld r23, 0x18(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F41DB0: EA4B0000  ld r18, 0(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F41DB4: EACB0020  ld r22, 0x20(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F41DB8: EAAB0028  ld r21, 0x28(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F41DBC: EA8B0030  ld r20, 0x30(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F41DC0: EA6B0038  ld r19, 0x38(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F41DC4: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F41DC8: 3BC70030  addi r30, r7, 0x30
	ctx.r[30].s64 = ctx.r[7].s64 + 48;
	// 82F41DCC: E8EB0008  ld r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F41DD0: 396100D0  addi r11, r1, 0xd0
	ctx.r[11].s64 = ctx.r[1].s64 + 208;
	// 82F41DD4: F8E60008  std r7, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[7].u64 ) };
	// 82F41DD8: 38E10080  addi r7, r1, 0x80
	ctx.r[7].s64 = ctx.r[1].s64 + 128;
	// 82F41DDC: FA460000  std r18, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
	// 82F41DE0: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82F41DE4: FB050000  std r24, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[24].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F41F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F41F10 size=64
    let mut pc: u32 = 0x82F41F10;
    'dispatch: loop {
        match pc {
            0x82F41F10 => {
    //   block [0x82F41F10..0x82F41F50)
	// 82F41F10: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F41F14: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F41F18: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F41F1C: 3D0082F5  lis r8, -0x7d0b
	ctx.r[8].s64 = -2097872896;
	// 82F41F20: 38EB15F8  addi r7, r11, 0x15f8
	ctx.r[7].s64 = ctx.r[11].s64 + 5624;
	// 82F41F24: 38CA1888  addi r6, r10, 0x1888
	ctx.r[6].s64 = ctx.r[10].s64 + 6280;
	// 82F41F28: 38A91D18  addi r5, r9, 0x1d18
	ctx.r[5].s64 = ctx.r[9].s64 + 7448;
	// 82F41F2C: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F41F30: 3888BB88  addi r4, r8, -0x4478
	ctx.r[4].s64 = ctx.r[8].s64 + -17528;
	// 82F41F34: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82F41F38: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F41F3C: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F41F40: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F41F44: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82F41F48: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82F41F4C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F41F50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F41F50 size=68
    let mut pc: u32 = 0x82F41F50;
    'dispatch: loop {
        match pc {
            0x82F41F50 => {
    //   block [0x82F41F50..0x82F41F94)
	// 82F41F50: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F41F54: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F41F58: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F41F5C: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F41F60: 38EB16D8  addi r7, r11, 0x16d8
	ctx.r[7].s64 = ctx.r[11].s64 + 5848;
	// 82F41F64: 38CA2260  addi r6, r10, 0x2260
	ctx.r[6].s64 = ctx.r[10].s64 + 8800;
	// 82F41F68: 38A92558  addi r5, r9, 0x2558
	ctx.r[5].s64 = ctx.r[9].s64 + 9560;
	// 82F41F6C: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F41F70: 38882138  addi r4, r8, 0x2138
	ctx.r[4].s64 = ctx.r[8].s64 + 8504;
	// 82F41F74: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82F41F78: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F41F7C: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F41F80: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F41F84: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F41F88: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82F41F8C: 99430011  stb r10, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[10].u8 ) };
	// 82F41F90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F41F98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F41F98 size=204
    let mut pc: u32 = 0x82F41F98;
    'dispatch: loop {
        match pc {
            0x82F41F98 => {
    //   block [0x82F41F98..0x82F42064)
	// 82F41F98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F41F9C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F41FA0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F41FA4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F41FA8: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F41FAC: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F41FB0: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F41FB4: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F41FB8: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F41FBC: 38EB16D8  addi r7, r11, 0x16d8
	ctx.r[7].s64 = ctx.r[11].s64 + 5848;
	// 82F41FC0: 38CA2260  addi r6, r10, 0x2260
	ctx.r[6].s64 = ctx.r[10].s64 + 8800;
	// 82F41FC4: 38A92558  addi r5, r9, 0x2558
	ctx.r[5].s64 = ctx.r[9].s64 + 9560;
	// 82F41FC8: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F41FCC: 38882138  addi r4, r8, 0x2138
	ctx.r[4].s64 = ctx.r[8].s64 + 8504;
	// 82F41FD0: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F41FD4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F41FD8: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F41FDC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F41FE0: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F41FE4: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82F41FE8: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82F41FEC: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F41FF0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82F41FF4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F41FF8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F41FFC: 4BFE3445  bl 0x82f25440
	ctx.lr = 0x82F42000;
	sub_82F25440(ctx, base);
	// 82F42000: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F42004: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F42008: 9BE10080  stb r31, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u8 ) };
	// 82F4200C: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F42010: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F42014: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 82F42018: 38CA15F8  addi r6, r10, 0x15f8
	ctx.r[6].s64 = ctx.r[10].s64 + 5624;
	// 82F4201C: 38A91888  addi r5, r9, 0x1888
	ctx.r[5].s64 = ctx.r[9].s64 + 6280;
	// 82F42020: 38881D18  addi r4, r8, 0x1d18
	ctx.r[4].s64 = ctx.r[8].s64 + 7448;
	// 82F42024: 90C10070  stw r6, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[6].u32 ) };
	// 82F42028: 3867BB88  addi r3, r7, -0x4478
	ctx.r[3].s64 = ctx.r[7].s64 + -17528;
	// 82F4202C: 90A10074  stw r5, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[5].u32 ) };
	// 82F42030: 90810078  stw r4, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[4].u32 ) };
	// 82F42034: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82F42038: 9061007C  stw r3, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 82F4203C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82F42040: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F42044: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F42048: 4BFE33F9  bl 0x82f25440
	ctx.lr = 0x82F4204C;
	sub_82F25440(ctx, base);
	// 82F4204C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F42050: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F42054: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F42058: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4205C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F42060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F42068(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F42068 size=204
    let mut pc: u32 = 0x82F42068;
    'dispatch: loop {
        match pc {
            0x82F42068 => {
    //   block [0x82F42068..0x82F42134)
	// 82F42068: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4206C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F42070: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F42074: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F42078: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4207C: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F42080: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F42084: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F42088: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F4208C: 38EB16D8  addi r7, r11, 0x16d8
	ctx.r[7].s64 = ctx.r[11].s64 + 5848;
	// 82F42090: 38CA2260  addi r6, r10, 0x2260
	ctx.r[6].s64 = ctx.r[10].s64 + 8800;
	// 82F42094: 38A92558  addi r5, r9, 0x2558
	ctx.r[5].s64 = ctx.r[9].s64 + 9560;
	// 82F42098: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F4209C: 38882138  addi r4, r8, 0x2138
	ctx.r[4].s64 = ctx.r[8].s64 + 8504;
	// 82F420A0: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F420A4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F420A8: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F420AC: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F420B0: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F420B4: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82F420B8: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82F420BC: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F420C0: 38A00004  li r5, 4
	ctx.r[5].s64 = 4;
	// 82F420C4: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F420C8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F420CC: 4BFE348D  bl 0x82f25558
	ctx.lr = 0x82F420D0;
	sub_82F25558(ctx, base);
	// 82F420D0: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F420D4: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F420D8: 9BE10080  stb r31, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u8 ) };
	// 82F420DC: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F420E0: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F420E4: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 82F420E8: 38CA15F8  addi r6, r10, 0x15f8
	ctx.r[6].s64 = ctx.r[10].s64 + 5624;
	// 82F420EC: 38A91888  addi r5, r9, 0x1888
	ctx.r[5].s64 = ctx.r[9].s64 + 6280;
	// 82F420F0: 38881D18  addi r4, r8, 0x1d18
	ctx.r[4].s64 = ctx.r[8].s64 + 7448;
	// 82F420F4: 90C10070  stw r6, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[6].u32 ) };
	// 82F420F8: 3867BB88  addi r3, r7, -0x4478
	ctx.r[3].s64 = ctx.r[7].s64 + -17528;
	// 82F420FC: 90A10074  stw r5, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[5].u32 ) };
	// 82F42100: 90810078  stw r4, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[4].u32 ) };
	// 82F42104: 38C00004  li r6, 4
	ctx.r[6].s64 = 4;
	// 82F42108: 9061007C  stw r3, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 82F4210C: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82F42110: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F42114: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F42118: 4BFE3441  bl 0x82f25558
	ctx.lr = 0x82F4211C;
	sub_82F25558(ctx, base);
	// 82F4211C: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F42120: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F42124: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F42128: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4212C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F42130: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F42138(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F42138 size=224
    let mut pc: u32 = 0x82F42138;
    'dispatch: loop {
        match pc {
            0x82F42138 => {
    //   block [0x82F42138..0x82F42218)
	// 82F42138: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4213C: 48266031  bl 0x831a816c
	ctx.lr = 0x82F42140;
	sub_831A8130(ctx, base);
	// 82F42140: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F42144: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F42148: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F4214C: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82F42150: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F42154: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F42158: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F4215C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F42160: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F42164: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F42168: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F4216C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F42170: 4200FFF0  bdnz 0x82f42160
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F42160; continue 'dispatch;
	}
	// 82F42174: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F42178: E9250050  ld r9, 0x50(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(80 as u32) ) };
	// 82F4217C: E8E50058  ld r7, 0x58(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(88 as u32) ) };
	// 82F42180: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82F42184: 38AAC5B0  addi r5, r10, -0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + -14928;
	// 82F42188: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 82F4218C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F42190: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F42218(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F42218 size=72
    let mut pc: u32 = 0x82F42218;
    'dispatch: loop {
        match pc {
            0x82F42218 => {
    //   block [0x82F42218..0x82F42260)
	// 82F42218: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4221C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F42220: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F42224: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F42228: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F4222C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F42230: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F42234: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F42238: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F4223C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F42240: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F42244: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F42248: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F4224C: 4BFFF50D  bl 0x82f41758
	ctx.lr = 0x82F42250;
	sub_82F41758(ctx, base);
	// 82F42250: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F42254: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F42258: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4225C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F42260(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F42260 size=72
    let mut pc: u32 = 0x82F42260;
    'dispatch: loop {
        match pc {
            0x82F42260 => {
    //   block [0x82F42260..0x82F422A8)
	// 82F42260: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F42264: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F42268: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4226C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F42270: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F42274: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F42278: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F4227C: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F42280: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F42284: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F42288: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F4228C: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F42290: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F42294: 4BFFF5F5  bl 0x82f41888
	ctx.lr = 0x82F42298;
	sub_82F41888(ctx, base);
	// 82F42298: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4229C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F422A0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F422A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F422A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F422A8 size=604
    let mut pc: u32 = 0x82F422A8;
    'dispatch: loop {
        match pc {
            0x82F422A8 => {
    //   block [0x82F422A8..0x82F42504)
	// 82F422A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F422AC: 48265E89  bl 0x831a8134
	ctx.lr = 0x82F422B0;
	sub_831A8130(ctx, base);
	// 82F422B0: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F422B4: 82CD0000  lwz r22, 0(r13)
	ctx.r[22].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F422B8: 3AA00018  li r21, 0x18
	ctx.r[21].s64 = 24;
	// 82F422BC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F422C0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F422C4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F422C8: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82F422CC: 7D75B02E  lwzx r11, r21, r22
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[21].u32.wrapping_add(ctx.r[22].u32)) } as u64;
	// 82F422D0: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82F422D4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F422D8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F422DC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F422E0: 40980020  bge cr6, 0x82f42300
	if !ctx.cr[6].lt {
	pc = 0x82F42300; continue 'dispatch;
	}
	// 82F422E4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F422E8: 3909FC58  addi r8, r9, -0x3a8
	ctx.r[8].s64 = ctx.r[9].s64 + -936;
	// 82F422EC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F422F0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F422F4: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F422F8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F422FC: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F42300: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F42304: 38E100B0  addi r7, r1, 0xb0
	ctx.r[7].s64 = ctx.r[1].s64 + 176;
	// 82F42308: 38C100A0  addi r6, r1, 0xa0
	ctx.r[6].s64 = ctx.r[1].s64 + 160;
	// 82F4230C: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F42310: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F42314: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F42318: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82F4231C: 837A0000  lwz r27, 0(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F42320: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82F42324: 831E0000  lwz r24, 0(r30)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F42328: E86B0000  ld r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F4232C: 3B2A0030  addi r25, r10, 0x30
	ctx.r[25].s64 = ctx.r[10].s64 + 48;
	// 82F42330: EA8B0008  ld r20, 8(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F42334: 39400002  li r10, 2
	ctx.r[10].s64 = 2;
	// 82F42338: EA6B0010  ld r19, 0x10(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F4233C: 390100E0  addi r8, r1, 0xe0
	ctx.r[8].s64 = ctx.r[1].s64 + 224;
	// 82F42340: EA4B0018  ld r18, 0x18(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F42344: 393F0040  addi r9, r31, 0x40
	ctx.r[9].s64 = ctx.r[31].s64 + 64;
	// 82F42348: EA2B0020  ld r17, 0x20(r11)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F4234C: EA0B0028  ld r16, 0x28(r11)
	ctx.r[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F42350: E9EB0030  ld r15, 0x30(r11)
	ctx.r[15].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F42354: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F42358: F8670000  std r3, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 82F4235C: FA870008  std r20, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[20].u64 ) };
	// 82F42360: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
	// 82F42364: FA660000  std r19, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82F42368: FA460008  std r18, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[18].u64 ) };
	// 82F4236C: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82F42370: FA250000  std r17, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[17].u64 ) };
	// 82F42374: FA050008  std r16, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[16].u64 ) };
	// 82F42378: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82F4237C: F9640008  std r11, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F42380: 396100B0  addi r11, r1, 0xb0
	ctx.r[11].s64 = ctx.r[1].s64 + 176;
	// 82F42384: F9E40000  std r15, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[15].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F42508(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F42508 size=76
    let mut pc: u32 = 0x82F42508;
    'dispatch: loop {
        match pc {
            0x82F42508 => {
    //   block [0x82F42508..0x82F42554)
	// 82F42508: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4250C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F42510: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F42514: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F42518: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F4251C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F42520: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F42524: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F42528: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F4252C: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F42530: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F42534: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F42538: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F4253C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F42540: 4BFFF5F9  bl 0x82f41b38
	ctx.lr = 0x82F42544;
	sub_82F41B38(ctx, base);
	// 82F42544: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F42548: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4254C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F42550: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F42558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F42558 size=76
    let mut pc: u32 = 0x82F42558;
    'dispatch: loop {
        match pc {
            0x82F42558 => {
    //   block [0x82F42558..0x82F425A4)
	// 82F42558: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4255C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F42560: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F42564: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F42568: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F4256C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F42570: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F42574: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F42578: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F4257C: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F42580: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F42584: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F42588: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F4258C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F42590: 4BFFF789  bl 0x82f41d18
	ctx.lr = 0x82F42594;
	sub_82F41D18(ctx, base);
	// 82F42594: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F42598: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4259C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F425A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F425A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F425A8 size=176
    let mut pc: u32 = 0x82F425A8;
    'dispatch: loop {
        match pc {
            0x82F425A8 => {
    //   block [0x82F425A8..0x82F42658)
	// 82F425A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F425AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F425B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F425B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F425B8: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F425BC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F425C0: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F425C4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F425C8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F425CC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F425D0: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F425D4: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F425D8: 4BFFFCD1  bl 0x82f422a8
	ctx.lr = 0x82F425DC;
	sub_82F422A8(ctx, base);
	// 82F425DC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F425E0: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F425E4: 40980044  bge cr6, 0x82f42628
	if !ctx.cr[6].lt {
	pc = 0x82F42628; continue 'dispatch;
	}
	// 82F425E8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F425EC: 394BC5A0  addi r10, r11, -0x3a60
	ctx.r[10].s64 = ctx.r[11].s64 + -14944;
	// 82F425F0: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F42658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F42658 size=104
    let mut pc: u32 = 0x82F42658;
    'dispatch: loop {
        match pc {
            0x82F42658 => {
    //   block [0x82F42658..0x82F426C0)
	// 82F42658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4265C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F42660: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F42664: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F42668: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4266C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F42670: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F42674: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82F42678: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F4267C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F42680: 4BF5E0B1  bl 0x82ea0730
	ctx.lr = 0x82F42684;
	sub_82EA0730(ctx, base);
	// 82F42684: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F42688: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82F4268C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82F42690: 38E9FC68  addi r7, r9, -0x398
	ctx.r[7].s64 = ctx.r[9].s64 + -920;
	// 82F42694: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F42698: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82F4269C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F426A0: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F426A4: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82F426A8: B0A3000C  sth r5, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u16 ) };
	// 82F426AC: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F426B0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F426B4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F426B8: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F426BC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F426C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F426C0 size=104
    let mut pc: u32 = 0x82F426C0;
    'dispatch: loop {
        match pc {
            0x82F426C0 => {
    //   block [0x82F426C0..0x82F42728)
	// 82F426C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F426C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F426C8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F426CC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F426D0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F426D4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F426D8: A17F000C  lhz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F426DC: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 82F426E0: 419A001C  beq cr6, 0x82f426fc
	if ctx.cr[6].eq {
	pc = 0x82F426FC; continue 'dispatch;
	}
	// 82F426E4: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F426E8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F426EC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F426F0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F426F4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F426F8: 4E800421  bctrl
	ctx.lr = 0x82F426FC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F426FC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F42700: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F42704: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F42708: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4270C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F42710: 4E800421  bctrl
	ctx.lr = 0x82F42714;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F42714: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F42718: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4271C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F42720: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F42724: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F42728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F42728 size=104
    let mut pc: u32 = 0x82F42728;
    'dispatch: loop {
        match pc {
            0x82F42728 => {
    //   block [0x82F42728..0x82F42790)
	// 82F42728: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4272C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F42730: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F42734: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F42738: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4273C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F42740: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F42744: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82F42748: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F4274C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F42750: 4BF5DFE1  bl 0x82ea0730
	ctx.lr = 0x82F42754;
	sub_82EA0730(ctx, base);
	// 82F42754: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F42758: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82F4275C: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82F42760: 38E9FCA4  addi r7, r9, -0x35c
	ctx.r[7].s64 = ctx.r[9].s64 + -860;
	// 82F42764: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F42768: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82F4276C: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F42770: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F42774: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82F42778: B0A3000C  sth r5, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u16 ) };
	// 82F4277C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F42780: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F42784: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F42788: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4278C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F42790(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F42790 size=444
    let mut pc: u32 = 0x82F42790;
    'dispatch: loop {
        match pc {
            0x82F42790 => {
    //   block [0x82F42790..0x82F4294C)
	// 82F42790: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F42794: 482659B1  bl 0x831a8144
	ctx.lr = 0x82F42798;
	sub_831A8130(ctx, base);
	// 82F42798: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4279C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F427A0: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F427A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F427A8: 7F4A5A14  add r26, r10, r11
	ctx.r[26].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F427AC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F427B0: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82F427B4: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F427B8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F427BC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F427C0: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F427C4: 40980020  bge cr6, 0x82f427e4
	if !ctx.cr[6].lt {
	pc = 0x82F427E4; continue 'dispatch;
	}
	// 82F427C8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F427CC: 3909FCDC  addi r8, r9, -0x324
	ctx.r[8].s64 = ctx.r[9].s64 + -804;
	// 82F427D0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F427D4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F427D8: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F427DC: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F427E0: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F427E4: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F427E8: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82F427EC: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F427F0: 815F0008  lwz r10, 8(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F427F4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F427F8: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F427FC: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82F42800: 837F0000  lwz r27, 0(r31)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F42804: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82F42808: E86B0000  ld r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F4280C: 3B8A0030  addi r28, r10, 0x30
	ctx.r[28].s64 = ctx.r[10].s64 + 48;
	// 82F42810: EB0B0008  ld r24, 8(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F42814: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F42818: EAEB0010  ld r23, 0x10(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F4281C: 390100C0  addi r8, r1, 0xc0
	ctx.r[8].s64 = ctx.r[1].s64 + 192;
	// 82F42820: EACB0018  ld r22, 0x18(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F42824: 393D0030  addi r9, r29, 0x30
	ctx.r[9].s64 = ctx.r[29].s64 + 48;
	// 82F42828: EAAB0020  ld r21, 0x20(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F4282C: EA8B0028  ld r20, 0x28(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F42830: EA6B0030  ld r19, 0x30(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F42834: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F42838: F8670000  std r3, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 82F4283C: FB070008  std r24, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[24].u64 ) };
	// 82F42840: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82F42844: FAE60000  std r23, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82F42848: FAC60008  std r22, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[22].u64 ) };
	// 82F4284C: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82F42850: FAA50000  std r21, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
	// 82F42854: FA850008  std r20, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[20].u64 ) };
	// 82F42858: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82F4285C: F9640008  std r11, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F42860: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F42864: FA640000  std r19, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F42950(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F42950 size=20
    let mut pc: u32 = 0x82F42950;
    'dispatch: loop {
        match pc {
            0x82F42950 => {
    //   block [0x82F42950..0x82F42964)
	// 82F42950: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F42954: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F42958: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F4295C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F42960: 4BFFFE30  b 0x82f42790
	sub_82F42790(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F42968(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F42968 size=616
    let mut pc: u32 = 0x82F42968;
    'dispatch: loop {
        match pc {
            0x82F42968 => {
    //   block [0x82F42968..0x82F42BD0)
	// 82F42968: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4296C: 482657E1  bl 0x831a814c
	ctx.lr = 0x82F42970;
	sub_831A8130(ctx, base);
	// 82F42970: 9421FEF0  stwu r1, -0x110(r1)
	ea = ctx.r[1].u32.wrapping_add(-272 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F42974: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F42978: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F4297C: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F42980: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82F42984: 83830000  lwz r28, 0(r3)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F42988: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F4298C: 83E40000  lwz r31, 0(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F42990: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82F42994: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F42998: E86B0000  ld r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F4299C: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82F429A0: EB4B0008  ld r26, 8(r11)
	ctx.r[26].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F429A4: 388100A0  addi r4, r1, 0xa0
	ctx.r[4].s64 = ctx.r[1].s64 + 160;
	// 82F429A8: EAEB0020  ld r23, 0x20(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F429AC: 3B6A0030  addi r27, r10, 0x30
	ctx.r[27].s64 = ctx.r[10].s64 + 48;
	// 82F429B0: EACB0028  ld r22, 0x28(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F429B4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F429B8: EB2B0010  ld r25, 0x10(r11)
	ctx.r[25].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F429BC: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82F429C0: EB0B0018  ld r24, 0x18(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F429C4: 393F0030  addi r9, r31, 0x30
	ctx.r[9].s64 = ctx.r[31].s64 + 48;
	// 82F429C8: EAAB0030  ld r21, 0x30(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F429CC: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F429D0: F8670000  std r3, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 82F429D4: FB470008  std r26, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[26].u64 ) };
	// 82F429D8: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82F429DC: FB260000  std r25, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[25].u64 ) };
	// 82F429E0: FB060008  std r24, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[24].u64 ) };
	// 82F429E4: 38C10090  addi r6, r1, 0x90
	ctx.r[6].s64 = ctx.r[1].s64 + 144;
	// 82F429E8: FAE50000  std r23, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
	// 82F429EC: FAC50008  std r22, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[22].u64 ) };
	// 82F429F0: 38A100A0  addi r5, r1, 0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + 160;
	// 82F429F4: F9640008  std r11, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F429F8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F429FC: FAA40000  std r21, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[21].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F42BD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F42BD0 size=756
    let mut pc: u32 = 0x82F42BD0;
    'dispatch: loop {
        match pc {
            0x82F42BD0 => {
    //   block [0x82F42BD0..0x82F42EC4)
	// 82F42BD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F42BD4: 48265571  bl 0x831a8144
	ctx.lr = 0x82F42BD8;
	sub_831A8130(ctx, base);
	// 82F42BD8: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F42BDC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F42BE0: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F42BE4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F42BE8: 7F6A5A14  add r27, r10, r11
	ctx.r[27].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F42BEC: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82F42BF0: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F42BF4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F42BF8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F42BFC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F42C00: 40980020  bge cr6, 0x82f42c20
	if !ctx.cr[6].lt {
	pc = 0x82F42C20; continue 'dispatch;
	}
	// 82F42C04: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F42C08: 3909FCDC  addi r8, r9, -0x324
	ctx.r[8].s64 = ctx.r[9].s64 + -804;
	// 82F42C0C: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F42C10: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F42C14: 386A000C  addi r3, r10, 0xc
	ctx.r[3].s64 = ctx.r[10].s64 + 12;
	// 82F42C18: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F42C1C: 906B0004  stw r3, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F42C20: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F42C24: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82F42C28: 90A100A4  stw r5, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[5].u32 ) };
	// 82F42C2C: 38C100E0  addi r6, r1, 0xe0
	ctx.r[6].s64 = ctx.r[1].s64 + 224;
	// 82F42C30: 908100A0  stw r4, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[4].u32 ) };
	// 82F42C34: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F42C38: 83E50000  lwz r31, 0(r5)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F42C3C: 386100D0  addi r3, r1, 0xd0
	ctx.r[3].s64 = ctx.r[1].s64 + 208;
	// 82F42C40: 3B210070  addi r25, r1, 0x70
	ctx.r[25].s64 = ctx.r[1].s64 + 112;
	// 82F42C44: 81240008  lwz r9, 8(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F42C48: E8AB0010  ld r5, 0x10(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F42C4C: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F42C50: EB0B0018  ld r24, 0x18(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F42C54: 3BA90030  addi r29, r9, 0x30
	ctx.r[29].s64 = ctx.r[9].s64 + 48;
	// 82F42C58: EA6B0000  ld r19, 0(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F42C5C: 390100C0  addi r8, r1, 0xc0
	ctx.r[8].s64 = ctx.r[1].s64 + 192;
	// 82F42C60: EAEB0020  ld r23, 0x20(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F42C64: 393F0030  addi r9, r31, 0x30
	ctx.r[9].s64 = ctx.r[31].s64 + 48;
	// 82F42C68: EACB0028  ld r22, 0x28(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F42C6C: EAAB0030  ld r21, 0x30(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F42C70: EA8B0038  ld r20, 0x38(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F42C74: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F42C78: F9670008  std r11, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F42C7C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F42C80: F8A60000  std r5, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[5].u64 ) };
	// 82F42C84: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F42C88: FA670000  std r19, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82F42C8C: 38E100E0  addi r7, r1, 0xe0
	ctx.r[7].s64 = ctx.r[1].s64 + 224;
	// 82F42C90: FB060008  std r24, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[24].u64 ) };
	// 82F42C94: 38C100D0  addi r6, r1, 0xd0
	ctx.r[6].s64 = ctx.r[1].s64 + 208;
	// 82F42C98: FAE30000  std r23, 0(r3)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[23].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F42EC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F42EC8 size=756
    let mut pc: u32 = 0x82F42EC8;
    'dispatch: loop {
        match pc {
            0x82F42EC8 => {
    //   block [0x82F42EC8..0x82F431BC)
	// 82F42EC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F42ECC: 48265279  bl 0x831a8144
	ctx.lr = 0x82F42ED0;
	sub_831A8130(ctx, base);
	// 82F42ED0: 9421FE90  stwu r1, -0x170(r1)
	ea = ctx.r[1].u32.wrapping_add(-368 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F42ED4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F42ED8: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F42EDC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F42EE0: 7F6A5A14  add r27, r10, r11
	ctx.r[27].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F42EE4: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F42EE8: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F42EEC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F42EF0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F42EF4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F42EF8: 40980020  bge cr6, 0x82f42f18
	if !ctx.cr[6].lt {
	pc = 0x82F42F18; continue 'dispatch;
	}
	// 82F42EFC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F42F00: 3909FCDC  addi r8, r9, -0x324
	ctx.r[8].s64 = ctx.r[9].s64 + -804;
	// 82F42F04: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F42F08: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F42F0C: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F42F10: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F42F14: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F42F18: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F42F1C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82F42F20: 908100A4  stw r4, 0xa4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(164 as u32), ctx.r[4].u32 ) };
	// 82F42F24: 38C100E0  addi r6, r1, 0xe0
	ctx.r[6].s64 = ctx.r[1].s64 + 224;
	// 82F42F28: 83E40000  lwz r31, 0(r4)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F42F2C: 388B0010  addi r4, r11, 0x10
	ctx.r[4].s64 = ctx.r[11].s64 + 16;
	// 82F42F30: 906100A0  stw r3, 0xa0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(160 as u32), ctx.r[3].u32 ) };
	// 82F42F34: 38A100D0  addi r5, r1, 0xd0
	ctx.r[5].s64 = ctx.r[1].s64 + 208;
	// 82F42F38: 3B210070  addi r25, r1, 0x70
	ctx.r[25].s64 = ctx.r[1].s64 + 112;
	// 82F42F3C: 81230008  lwz r9, 8(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F42F40: E88B0010  ld r4, 0x10(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F42F44: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F42F48: EB0B0018  ld r24, 0x18(r11)
	ctx.r[24].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F42F4C: 3BA90030  addi r29, r9, 0x30
	ctx.r[29].s64 = ctx.r[9].s64 + 48;
	// 82F42F50: EAEB0020  ld r23, 0x20(r11)
	ctx.r[23].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F42F54: 390100C0  addi r8, r1, 0xc0
	ctx.r[8].s64 = ctx.r[1].s64 + 192;
	// 82F42F58: EACB0028  ld r22, 0x28(r11)
	ctx.r[22].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F42F5C: 393F0030  addi r9, r31, 0x30
	ctx.r[9].s64 = ctx.r[31].s64 + 48;
	// 82F42F60: EA6B0000  ld r19, 0(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F42F64: EAAB0030  ld r21, 0x30(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F42F68: EA8B0038  ld r20, 0x38(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F42F6C: E96B0008  ld r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F42F70: F9670008  std r11, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F42F74: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F42F78: FA670000  std r19, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[19].u64 ) };
	// 82F42F7C: 38E100E0  addi r7, r1, 0xe0
	ctx.r[7].s64 = ctx.r[1].s64 + 224;
	// 82F42F80: F8860000  std r4, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[4].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F431C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F431C0 size=204
    let mut pc: u32 = 0x82F431C0;
    'dispatch: loop {
        match pc {
            0x82F431C0 => {
    //   block [0x82F431C0..0x82F4328C)
	// 82F431C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F431C4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F431C8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F431CC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F431D0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F431D4: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F431D8: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F431DC: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F431E0: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F431E4: 38EB2728  addi r7, r11, 0x2728
	ctx.r[7].s64 = ctx.r[11].s64 + 10024;
	// 82F431E8: 38CA32D8  addi r6, r10, 0x32d8
	ctx.r[6].s64 = ctx.r[10].s64 + 13016;
	// 82F431EC: 38A936D0  addi r5, r9, 0x36d0
	ctx.r[5].s64 = ctx.r[9].s64 + 14032;
	// 82F431F0: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F431F4: 38882138  addi r4, r8, 0x2138
	ctx.r[4].s64 = ctx.r[8].s64 + 8504;
	// 82F431F8: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F431FC: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F43200: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F43204: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F43208: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F4320C: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82F43210: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82F43214: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F43218: 38A00006  li r5, 6
	ctx.r[5].s64 = 6;
	// 82F4321C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F43220: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F43224: 4BFE221D  bl 0x82f25440
	ctx.lr = 0x82F43228;
	sub_82F25440(ctx, base);
	// 82F43228: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F4322C: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F43230: 9BE10080  stb r31, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[31].u8 ) };
	// 82F43234: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F43238: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F4323C: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 82F43240: 38CA2658  addi r6, r10, 0x2658
	ctx.r[6].s64 = ctx.r[10].s64 + 9816;
	// 82F43244: 38A92790  addi r5, r9, 0x2790
	ctx.r[5].s64 = ctx.r[9].s64 + 10128;
	// 82F43248: 38882EC8  addi r4, r8, 0x2ec8
	ctx.r[4].s64 = ctx.r[8].s64 + 11976;
	// 82F4324C: 90C10070  stw r6, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[6].u32 ) };
	// 82F43250: 3867BB88  addi r3, r7, -0x4478
	ctx.r[3].s64 = ctx.r[7].s64 + -17528;
	// 82F43254: 90A10074  stw r5, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[5].u32 ) };
	// 82F43258: 90810078  stw r4, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[4].u32 ) };
	// 82F4325C: 38C00006  li r6, 6
	ctx.r[6].s64 = 6;
	// 82F43260: 9061007C  stw r3, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[3].u32 ) };
	// 82F43264: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82F43268: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F4326C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F43270: 4BFE21D1  bl 0x82f25440
	ctx.lr = 0x82F43274;
	sub_82F25440(ctx, base);
	// 82F43274: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F43278: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4327C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F43280: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F43284: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F43288: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F43290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F43290 size=68
    let mut pc: u32 = 0x82F43290;
    'dispatch: loop {
        match pc {
            0x82F43290 => {
    //   block [0x82F43290..0x82F432D4)
	// 82F43290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F43294: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F43298: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4329C: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F432A0: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F432A4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F432A8: 392BF400  addi r9, r11, -0xc00
	ctx.r[9].s64 = ctx.r[11].s64 + -3072;
	// 82F432AC: 99410054  stb r10, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u8 ) };
	// 82F432B0: 7CA32B78  mr r3, r5
	ctx.r[3].u64 = ctx.r[5].u64;
	// 82F432B4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F432B8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F432BC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F432C0: 4BFFF4D1  bl 0x82f42790
	ctx.lr = 0x82F432C4;
	sub_82F42790(ctx, base);
	// 82F432C4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F432C8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F432CC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F432D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F432D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F432D8 size=72
    let mut pc: u32 = 0x82F432D8;
    'dispatch: loop {
        match pc {
            0x82F432D8 => {
    //   block [0x82F432D8..0x82F43320)
	// 82F432D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F432DC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F432E0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F432E4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F432E8: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F432EC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F432F0: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F432F4: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F432F8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F432FC: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F43300: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F43304: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F43308: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F4330C: 4BFFF485  bl 0x82f42790
	ctx.lr = 0x82F43310;
	sub_82F42790(ctx, base);
	// 82F43310: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F43314: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F43318: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4331C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F43320(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F43320 size=860
    let mut pc: u32 = 0x82F43320;
    'dispatch: loop {
        match pc {
            0x82F43320 => {
    //   block [0x82F43320..0x82F4367C)
	// 82F43320: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F43324: 48264E15  bl 0x831a8138
	ctx.lr = 0x82F43328;
	sub_831A8130(ctx, base);
	// 82F43328: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4332C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F43330: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F43334: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82F43338: 7ECA5A14  add r22, r10, r11
	ctx.r[22].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F4333C: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F43340: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F43344: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F43348: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82F4334C: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F43350: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F43354: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F43358: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F4335C: 40980020  bge cr6, 0x82f4337c
	if !ctx.cr[6].lt {
	pc = 0x82F4337C; continue 'dispatch;
	}
	// 82F43360: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F43364: 3909FCDC  addi r8, r9, -0x324
	ctx.r[8].s64 = ctx.r[9].s64 + -804;
	// 82F43368: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F4336C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F43370: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F43374: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F43378: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F4337C: 817B0008  lwz r11, 8(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F43380: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82F43384: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F43388: 815C0008  lwz r10, 8(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4338C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F43390: 83FB0000  lwz r31, 0(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F43394: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82F43398: 83D90000  lwz r30, 0(r25)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4339C: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
	// 82F433A0: 83BC0000  lwz r29, 0(r28)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F433A4: E86B0000  ld r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F433A8: 3B4A0030  addi r26, r10, 0x30
	ctx.r[26].s64 = ctx.r[10].s64 + 48;
	// 82F433AC: EAAB0008  ld r21, 8(r11)
	ctx.r[21].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) };
	// 82F433B0: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F433B4: EA8B0010  ld r20, 0x10(r11)
	ctx.r[20].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) };
	// 82F433B8: 390100C0  addi r8, r1, 0xc0
	ctx.r[8].s64 = ctx.r[1].s64 + 192;
	// 82F433BC: EA6B0018  ld r19, 0x18(r11)
	ctx.r[19].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) };
	// 82F433C0: 393F0030  addi r9, r31, 0x30
	ctx.r[9].s64 = ctx.r[31].s64 + 48;
	// 82F433C4: EA4B0020  ld r18, 0x20(r11)
	ctx.r[18].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) };
	// 82F433C8: EA2B0028  ld r17, 0x28(r11)
	ctx.r[17].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) };
	// 82F433CC: EA0B0030  ld r16, 0x30(r11)
	ctx.r[16].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) };
	// 82F433D0: E96B0038  ld r11, 0x38(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(56 as u32) ) };
	// 82F433D4: F8670000  std r3, 0(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(0 as u32), ctx.r[3].u64 ) };
	// 82F433D8: FAA70008  std r21, 8(r7)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[7].u32.wrapping_add(8 as u32), ctx.r[21].u64 ) };
	// 82F433DC: 38E10070  addi r7, r1, 0x70
	ctx.r[7].s64 = ctx.r[1].s64 + 112;
	// 82F433E0: FA860000  std r20, 0(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), ctx.r[20].u64 ) };
	// 82F433E4: FA660008  std r19, 8(r6)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[6].u32.wrapping_add(8 as u32), ctx.r[19].u64 ) };
	// 82F433E8: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82F433EC: FA450000  std r18, 0(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(0 as u32), ctx.r[18].u64 ) };
	// 82F433F0: FA250008  std r17, 8(r5)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[5].u32.wrapping_add(8 as u32), ctx.r[17].u64 ) };
	// 82F433F4: 38A10090  addi r5, r1, 0x90
	ctx.r[5].s64 = ctx.r[1].s64 + 144;
	// 82F433F8: F9640008  std r11, 8(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(8 as u32), ctx.r[11].u64 ) };
	// 82F433FC: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F43400: FA040000  std r16, 0(r4)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[4].u32.wrapping_add(0 as u32), ctx.r[16].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F43680(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F43680 size=76
    let mut pc: u32 = 0x82F43680;
    'dispatch: loop {
        match pc {
            0x82F43680 => {
    //   block [0x82F43680..0x82F436CC)
	// 82F43680: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F43684: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F43688: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4368C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F43690: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F43694: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F43698: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F4369C: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F436A0: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F436A4: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F436A8: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F436AC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F436B0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F436B4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F436B8: 4BFFF519  bl 0x82f42bd0
	ctx.lr = 0x82F436BC;
	sub_82F42BD0(ctx, base);
	// 82F436BC: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F436C0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F436C4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F436C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F436D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F436D0 size=76
    let mut pc: u32 = 0x82F436D0;
    'dispatch: loop {
        match pc {
            0x82F436D0 => {
    //   block [0x82F436D0..0x82F4371C)
	// 82F436D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F436D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F436D8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F436DC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F436E0: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F436E4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F436E8: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F436EC: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F436F0: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F436F4: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F436F8: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F436FC: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F43700: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F43704: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F43708: 4BFFF7C1  bl 0x82f42ec8
	ctx.lr = 0x82F4370C;
	sub_82F42EC8(ctx, base);
	// 82F4370C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F43710: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F43714: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F43718: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F43720(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F43720 size=176
    let mut pc: u32 = 0x82F43720;
    'dispatch: loop {
        match pc {
            0x82F43720 => {
    //   block [0x82F43720..0x82F437D0)
	// 82F43720: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F43724: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F43728: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4372C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F43730: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F43734: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F43738: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F4373C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F43740: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F43744: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F43748: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4374C: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F43750: 4BFFFBD1  bl 0x82f43320
	ctx.lr = 0x82F43754;
	sub_82F43320(ctx, base);
	// 82F43754: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F43758: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F4375C: 40980044  bge cr6, 0x82f437a0
	if !ctx.cr[6].lt {
	pc = 0x82F437A0; continue 'dispatch;
	}
	// 82F43760: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F43764: 394BC5A0  addi r10, r11, -0x3a60
	ctx.r[10].s64 = ctx.r[11].s64 + -14944;
	// 82F43768: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F437D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F437D0 size=104
    let mut pc: u32 = 0x82F437D0;
    'dispatch: loop {
        match pc {
            0x82F437D0 => {
    //   block [0x82F437D0..0x82F43838)
	// 82F437D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F437D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F437D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F437DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F437E0: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F437E4: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F437E8: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F437EC: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82F437F0: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F437F4: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F437F8: 4BF5CF39  bl 0x82ea0730
	ctx.lr = 0x82F437FC;
	sub_82EA0730(ctx, base);
	// 82F437FC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F43800: 93E30008  stw r31, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[31].u32 ) };
	// 82F43804: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82F43808: 38E9FCF0  addi r7, r9, -0x310
	ctx.r[7].s64 = ctx.r[9].s64 + -784;
	// 82F4380C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F43810: B1030004  sth r8, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82F43814: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F43818: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F4381C: B0C30006  sth r6, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82F43820: B0A3000C  sth r5, 0xc(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[5].u16 ) };
	// 82F43824: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F43828: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4382C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F43830: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F43834: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F43838(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F43838 size=112
    let mut pc: u32 = 0x82F43838;
    'dispatch: loop {
        match pc {
            0x82F43838 => {
    //   block [0x82F43838..0x82F438A8)
	// 82F43838: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4383C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F43840: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F43844: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F43848: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4384C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F43850: A17F000C  lhz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F43854: 2B0BFFFF  cmplwi cr6, r11, 0xffff
	ctx.cr[6].compare_u32(ctx.r[11].u32, 65535 as u32, &mut ctx.xer);
	// 82F43858: 419A0024  beq cr6, 0x82f4387c
	if ctx.cr[6].eq {
	pc = 0x82F4387C; continue 'dispatch;
	}
	// 82F4385C: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F43860: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F43864: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F43868: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F4386C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F43870: 4E800421  bctrl
	ctx.lr = 0x82F43874;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F43874: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82F43878: B13F000C  sth r9, 0xc(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[9].u16 ) };
	// 82F4387C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F43880: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F43884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F43888: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4388C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F43890: 4E800421  bctrl
	ctx.lr = 0x82F43894;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F43894: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F43898: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4389C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F438A0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F438A4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F438A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F438A8 size=248
    let mut pc: u32 = 0x82F438A8;
    'dispatch: loop {
        match pc {
            0x82F438A8 => {
    //   block [0x82F438A8..0x82F439A0)
	// 82F438A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F438AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F438B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F438B4: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F438B8: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F438BC: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F438C0: 7C691B78  mr r9, r3
	ctx.r[9].u64 = ctx.r[3].u64;
	// 82F438C4: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F438C8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F438CC: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F438D0: 810B000C  lwz r8, 0xc(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F438D4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F438D8: 7F0A4040  cmplw cr6, r10, r8
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F438DC: 40980020  bge cr6, 0x82f438fc
	if !ctx.cr[6].lt {
	pc = 0x82F438FC; continue 'dispatch;
	}
	// 82F438E0: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F438E4: 38E8FD28  addi r7, r8, -0x2d8
	ctx.r[7].s64 = ctx.r[8].s64 + -728;
	// 82F438E8: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F438EC: 7C8C42E6  mftb r4, 0x10c
	ctx.r[4].u64 = crate::rt::rdtsc_u64();
	// 82F438F0: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82F438F4: 908A0004  stw r4, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[4].u32 ) };
	// 82F438F8: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F438FC: 81490008  lwz r10, 8(r9)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F43900: 39600030  li r11, 0x30
	ctx.r[11].s64 = 48;
	// 82F43904: 81050008  lwz r8, 8(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F43908: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F439A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F439A0 size=496
    let mut pc: u32 = 0x82F439A0;
    'dispatch: loop {
        match pc {
            0x82F439A0 => {
    //   block [0x82F439A0..0x82F43B90)
	// 82F439A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F439A4: 482647C5  bl 0x831a8168
	ctx.lr = 0x82F439A8;
	sub_831A8130(ctx, base);
	// 82F439A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F439AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F439B0: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F439B4: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F439B8: 7F8A5A14  add r28, r10, r11
	ctx.r[28].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F439BC: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82F439C0: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F439C4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F439C8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F439CC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F439D0: 40980020  bge cr6, 0x82f439f0
	if !ctx.cr[6].lt {
	pc = 0x82F439F0; continue 'dispatch;
	}
	// 82F439D4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F439D8: 3909FD28  addi r8, r9, -0x2d8
	ctx.r[8].s64 = ctx.r[9].s64 + -728;
	// 82F439DC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F439E0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F439E4: 392A000C  addi r9, r10, 0xc
	ctx.r[9].s64 = ctx.r[10].s64 + 12;
	// 82F439E8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F439EC: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82F439F0: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F439F4: 38E00030  li r7, 0x30
	ctx.r[7].s64 = 48;
	// 82F439F8: 80640008  lwz r3, 8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F439FC: 3BE10050  addi r31, r1, 0x50
	ctx.r[31].s64 = ctx.r[1].s64 + 80;
	// 82F43A00: 390B0030  addi r8, r11, 0x30
	ctx.r[8].s64 = ctx.r[11].s64 + 48;
	// 82F43A04: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F43A08: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F43A0C: 394B0010  addi r10, r11, 0x10
	ctx.r[10].s64 = ctx.r[11].s64 + 16;
	// 82F43A10: 39690010  addi r11, r9, 0x10
	ctx.r[11].s64 = ctx.r[9].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F43B90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F43B90 size=448
    let mut pc: u32 = 0x82F43B90;
    'dispatch: loop {
        match pc {
            0x82F43B90 => {
    //   block [0x82F43B90..0x82F43D50)
	// 82F43B90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F43B94: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F43B98: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F43B9C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F43BA0: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F43BA4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F43BA8: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F43BAC: 7FEA5A14  add r31, r10, r11
	ctx.r[31].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F43BB0: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F43BB4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F43BB8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F43BBC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F43BC0: 40980020  bge cr6, 0x82f43be0
	if !ctx.cr[6].lt {
	pc = 0x82F43BE0; continue 'dispatch;
	}
	// 82F43BC4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F43BC8: 3909FD28  addi r8, r9, -0x2d8
	ctx.r[8].s64 = ctx.r[9].s64 + -728;
	// 82F43BCC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F43BD0: 7C6C42E6  mftb r3, 0x10c
	ctx.r[3].u64 = crate::rt::rdtsc_u64();
	// 82F43BD4: 390A000C  addi r8, r10, 0xc
	ctx.r[8].s64 = ctx.r[10].s64 + 12;
	// 82F43BD8: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F43BDC: 910B0004  stw r8, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F43BE0: 81650008  lwz r11, 8(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F43BE4: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82F43BE8: 80640008  lwz r3, 8(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F43BEC: 3BC10050  addi r30, r1, 0x50
	ctx.r[30].s64 = ctx.r[1].s64 + 80;
	// 82F43BF0: 394B0030  addi r10, r11, 0x30
	ctx.r[10].s64 = ctx.r[11].s64 + 48;
	// 82F43BF4: 90810080  stw r4, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u32 ) };
	// 82F43BF8: 90A10084  stw r5, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[5].u32 ) };
	// 82F43BFC: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F43C00: 81050000  lwz r8, 0(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F43D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F43D50 size=444
    let mut pc: u32 = 0x82F43D50;
    'dispatch: loop {
        match pc {
            0x82F43D50 => {
    //   block [0x82F43D50..0x82F43F0C)
	// 82F43D50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F43D54: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F43D58: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F43D5C: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F43D60: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F43D64: 39200018  li r9, 0x18
	ctx.r[9].s64 = 24;
	// 82F43D68: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82F43D6C: 7FE95214  add r31, r9, r10
	ctx.r[31].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82F43D70: 7D49502E  lwzx r10, r9, r10
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F43D74: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F43D78: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F43D7C: 7F094040  cmplw cr6, r9, r8
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F43D80: 40980020  bge cr6, 0x82f43da0
	if !ctx.cr[6].lt {
	pc = 0x82F43DA0; continue 'dispatch;
	}
	// 82F43D84: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F43D88: 38E8FD28  addi r7, r8, -0x2d8
	ctx.r[7].s64 = ctx.r[8].s64 + -728;
	// 82F43D8C: 90E90000  stw r7, 0(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F43D90: 7C6C42E6  mftb r3, 0x10c
	ctx.r[3].u64 = crate::rt::rdtsc_u64();
	// 82F43D94: 38E9000C  addi r7, r9, 0xc
	ctx.r[7].s64 = ctx.r[9].s64 + 12;
	// 82F43D98: 90690004  stw r3, 4(r9)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[9].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F43D9C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F43DA0: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F43DA4: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82F43DA8: 80EB0008  lwz r7, 8(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F43DAC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F43DB0: 394A0030  addi r10, r10, 0x30
	ctx.r[10].s64 = ctx.r[10].s64 + 48;
	// 82F43DB4: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82F43DB8: 90810084  stw r4, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[4].u32 ) };
	// 82F43DBC: 816B0000  lwz r11, 0(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F43DC0: 81040000  lwz r8, 0(r4)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F43F10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F43F10 size=20
    let mut pc: u32 = 0x82F43F10;
    'dispatch: loop {
        match pc {
            0x82F43F10 => {
    //   block [0x82F43F10..0x82F43F24)
	// 82F43F10: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F43F14: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F43F18: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F43F1C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F43F20: 4BFFF988  b 0x82f438a8
	sub_82F438A8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F43F28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F43F28 size=52
    let mut pc: u32 = 0x82F43F28;
    'dispatch: loop {
        match pc {
            0x82F43F28 => {
    //   block [0x82F43F28..0x82F43F5C)
	// 82F43F28: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F43F2C: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F43F30: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F43F34: 3D0082F5  lis r8, -0x7d0b
	ctx.r[8].s64 = -2097872896;
	// 82F43F38: 38EB37D0  addi r7, r11, 0x37d0
	ctx.r[7].s64 = ctx.r[11].s64 + 14288;
	// 82F43F3C: 38CA38A8  addi r6, r10, 0x38a8
	ctx.r[6].s64 = ctx.r[10].s64 + 14504;
	// 82F43F40: 38A93D50  addi r5, r9, 0x3d50
	ctx.r[5].s64 = ctx.r[9].s64 + 15696;
	// 82F43F44: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F43F48: 3888BB88  addi r4, r8, -0x4478
	ctx.r[4].s64 = ctx.r[8].s64 + -17528;
	// 82F43F4C: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82F43F50: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F43F54: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F43F58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F43F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F43F60 size=104
    let mut pc: u32 = 0x82F43F60;
    'dispatch: loop {
        match pc {
            0x82F43F60 => {
    //   block [0x82F43F60..0x82F43FC8)
	// 82F43F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F43F64: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F43F68: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F43F6C: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F43F70: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F43F74: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F43F78: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82F43F7C: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F43F80: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 82F43F84: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 82F43F88: 38CA37D0  addi r6, r10, 0x37d0
	ctx.r[6].s64 = ctx.r[10].s64 + 14288;
	// 82F43F8C: 38A938A8  addi r5, r9, 0x38a8
	ctx.r[5].s64 = ctx.r[9].s64 + 14504;
	// 82F43F90: 38883D50  addi r4, r8, 0x3d50
	ctx.r[4].s64 = ctx.r[8].s64 + 15696;
	// 82F43F94: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82F43F98: 3967BB88  addi r11, r7, -0x4478
	ctx.r[11].s64 = ctx.r[7].s64 + -17528;
	// 82F43F9C: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82F43FA0: 90810058  stw r4, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 82F43FA4: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82F43FA8: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82F43FAC: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82F43FB0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F43FB4: 4BFE148D  bl 0x82f25440
	ctx.lr = 0x82F43FB8;
	sub_82F25440(ctx, base);
	// 82F43FB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F43FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F43FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F43FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F43FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F43FC8 size=104
    let mut pc: u32 = 0x82F43FC8;
    'dispatch: loop {
        match pc {
            0x82F43FC8 => {
    //   block [0x82F43FC8..0x82F44030)
	// 82F43FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F43FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F43FD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F43FD4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F43FD8: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F43FDC: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F43FE0: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82F43FE4: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F43FE8: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 82F43FEC: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 82F43FF0: 38CA37D0  addi r6, r10, 0x37d0
	ctx.r[6].s64 = ctx.r[10].s64 + 14288;
	// 82F43FF4: 38A938A8  addi r5, r9, 0x38a8
	ctx.r[5].s64 = ctx.r[9].s64 + 14504;
	// 82F43FF8: 38883D50  addi r4, r8, 0x3d50
	ctx.r[4].s64 = ctx.r[8].s64 + 15696;
	// 82F43FFC: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82F44000: 3967BB88  addi r11, r7, -0x4478
	ctx.r[11].s64 = ctx.r[7].s64 + -17528;
	// 82F44004: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82F44008: 90810058  stw r4, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 82F4400C: 38C00002  li r6, 2
	ctx.r[6].s64 = 2;
	// 82F44010: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82F44014: 38A00002  li r5, 2
	ctx.r[5].s64 = 2;
	// 82F44018: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F4401C: 4BFE153D  bl 0x82f25558
	ctx.lr = 0x82F44020;
	sub_82F25558(ctx, base);
	// 82F44020: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F44024: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F44028: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4402C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F44030(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F44030 size=120
    let mut pc: u32 = 0x82F44030;
    'dispatch: loop {
        match pc {
            0x82F44030 => {
    //   block [0x82F44030..0x82F440A8)
	// 82F44030: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F44034: 48264135  bl 0x831a8168
	ctx.lr = 0x82F44038;
	sub_831A8130(ctx, base);
	// 82F44038: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4403C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F44040: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F44044: 3BA00000  li r29, 0
	ctx.r[29].s64 = 0;
	// 82F44048: 897E0031  lbz r11, 0x31(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(49 as u32) ) } as u64;
	// 82F4404C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F44050: 419A0038  beq cr6, 0x82f44088
	if ctx.cr[6].eq {
	pc = 0x82F44088; continue 'dispatch;
	}
	// 82F44054: 3BFE0012  addi r31, r30, 0x12
	ctx.r[31].s64 = ctx.r[30].s64 + 18;
	// 82F44058: 807E0008  lwz r3, 8(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4405C: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F44060: A09F0000  lhz r4, 0(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F44064: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F44068: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F4406C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F44070: 4E800421  bctrl
	ctx.lr = 0x82F44074;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F44074: 893E0031  lbz r9, 0x31(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[30].u32.wrapping_add(49 as u32) ) } as u64;
	// 82F44078: 3BBD0001  addi r29, r29, 1
	ctx.r[29].s64 = ctx.r[29].s64 + 1;
	// 82F4407C: 3BFF0004  addi r31, r31, 4
	ctx.r[31].s64 = ctx.r[31].s64 + 4;
	// 82F44080: 7F1D4800  cmpw cr6, r29, r9
	ctx.cr[6].compare_i32(ctx.r[29].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F44084: 4198FFD4  blt cr6, 0x82f44058
	if ctx.cr[6].lt {
	pc = 0x82F44058; continue 'dispatch;
	}
	// 82F44088: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4408C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F44090: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F44094: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F44098: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4409C: 4E800421  bctrl
	ctx.lr = 0x82F440A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F440A0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F440A4: 48264114  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F440A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F440A8 size=184
    let mut pc: u32 = 0x82F440A8;
    'dispatch: loop {
        match pc {
            0x82F440A8 => {
    //   block [0x82F440A8..0x82F44160)
	// 82F440A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F440AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F440B0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F440B4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F440B8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F440BC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F440C0: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F440C4: C0050004  lfs f0, 4(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F440C8: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82F440CC: C18B002C  lfs f12, 0x2c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F440D0: C1AAFD74  lfs f13, -0x28c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-652 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F440D4: ED6C0372  fmuls f11, f12, f13
	ctx.f[11].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82F440D8: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82F440DC: 40980064  bge cr6, 0x82f44140
	if !ctx.cr[6].lt {
	pc = 0x82F44140; continue 'dispatch;
	}
	// 82F440E0: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F440E4: C18B002C  lfs f12, 0x2c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F440E8: ED6C0372  fmuls f11, f12, f13
	ctx.f[11].f64 = (((ctx.f[12].f64 * ctx.f[13].f64) as f32) as f64);
	// 82F440EC: FF005800  fcmpu cr6, f0, f11
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[11].f64);
	// 82F440F0: 40980050  bge cr6, 0x82f44140
	if !ctx.cr[6].lt {
	pc = 0x82F44140; continue 'dispatch;
	}
	// 82F440F4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F440F8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F440FC: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F44100: 38800050  li r4, 0x50
	ctx.r[4].s64 = 80;
	// 82F44104: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F44108: 4BF5C629  bl 0x82ea0730
	ctx.lr = 0x82F4410C;
	sub_82EA0730(ctx, base);
	// 82F4410C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F44110: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F44114: 39000050  li r8, 0x50
	ctx.r[8].s64 = 80;
	// 82F44118: 38E9FD3C  addi r7, r9, -0x2c4
	ctx.r[7].s64 = ctx.r[9].s64 + -708;
	// 82F4411C: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F44120: B11F0004  sth r8, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82F44124: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82F44128: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82F4412C: 93DF0008  stw r30, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[30].u32 ) };
	// 82F44130: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F44134: 48007E2D  bl 0x82f4bf60
	ctx.lr = 0x82F44138;
	sub_82F4BF60(ctx, base);
	// 82F44138: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4413C: 4800000C  b 0x82f44148
	pc = 0x82F44148; continue 'dispatch;
	// 82F44140: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F44144: 48007355  bl 0x82f4b498
	ctx.lr = 0x82F44148;
	sub_82F4B498(ctx, base);
	// 82F44148: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4414C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F44150: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F44154: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F44158: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4415C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F44160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F44160 size=508
    let mut pc: u32 = 0x82F44160;
    'dispatch: loop {
        match pc {
            0x82F44160 => {
    //   block [0x82F44160..0x82F4435C)
	// 82F44160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F44164: 48263FD1  bl 0x831a8134
	ctx.lr = 0x82F44168;
	sub_831A8130(ctx, base);
	// 82F44168: 3980FF50  li r12, -0xb0
	ctx.r[12].s64 = -176;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F44360(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F44360 size=552
    let mut pc: u32 = 0x82F44360;
    'dispatch: loop {
        match pc {
            0x82F44360 => {
    //   block [0x82F44360..0x82F44588)
	// 82F44360: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F44364: 48263DD5  bl 0x831a8138
	ctx.lr = 0x82F44368;
	sub_831A8130(ctx, base);
	// 82F44368: 3980FF50  li r12, -0xb0
	ctx.r[12].s64 = -176;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F44588(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F44588 size=20
    let mut pc: u32 = 0x82F44588;
    'dispatch: loop {
        match pc {
            0x82F44588 => {
    //   block [0x82F44588..0x82F4459C)
	// 82F44588: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F4458C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F44590: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F44594: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F44598: 4BFFFDC8  b 0x82f44360
	sub_82F44360(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F445A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F445A0 size=544
    let mut pc: u32 = 0x82F445A0;
    'dispatch: loop {
        match pc {
            0x82F445A0 => {
    //   block [0x82F445A0..0x82F447C0)
	// 82F445A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F445A4: 48263B8D  bl 0x831a8130
	ctx.lr = 0x82F445A8;
	sub_831A8130(ctx, base);
	// 82F445A8: 3980FF40  li r12, -0xc0
	ctx.r[12].s64 = -192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F447C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F447C0 size=20
    let mut pc: u32 = 0x82F447C0;
    'dispatch: loop {
        match pc {
            0x82F447C0 => {
    //   block [0x82F447C0..0x82F447D4)
	// 82F447C0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F447C4: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F447C8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F447CC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F447D0: 4BFFFDD0  b 0x82f445a0
	sub_82F445A0(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F447D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F447D8 size=64
    let mut pc: u32 = 0x82F447D8;
    'dispatch: loop {
        match pc {
            0x82F447D8 => {
    //   block [0x82F447D8..0x82F44818)
	// 82F447D8: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F447DC: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F447E0: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F447E4: 3D0082F5  lis r8, -0x7d0b
	ctx.r[8].s64 = -2097872896;
	// 82F447E8: 38EB40A8  addi r7, r11, 0x40a8
	ctx.r[7].s64 = ctx.r[11].s64 + 16552;
	// 82F447EC: 38CA45A0  addi r6, r10, 0x45a0
	ctx.r[6].s64 = ctx.r[10].s64 + 17824;
	// 82F447F0: 38A94360  addi r5, r9, 0x4360
	ctx.r[5].s64 = ctx.r[9].s64 + 17248;
	// 82F447F4: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F447F8: 3888BB88  addi r4, r8, -0x4478
	ctx.r[4].s64 = ctx.r[8].s64 + -17528;
	// 82F447FC: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82F44800: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F44804: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F44808: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F4480C: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82F44810: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82F44814: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F44818(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F44818 size=104
    let mut pc: u32 = 0x82F44818;
    'dispatch: loop {
        match pc {
            0x82F44818 => {
    //   block [0x82F44818..0x82F44880)
	// 82F44818: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4481C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F44820: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F44824: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F44828: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F4482C: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F44830: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 82F44834: 38CA40A8  addi r6, r10, 0x40a8
	ctx.r[6].s64 = ctx.r[10].s64 + 16552;
	// 82F44838: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4483C: 38A945A0  addi r5, r9, 0x45a0
	ctx.r[5].s64 = ctx.r[9].s64 + 17824;
	// 82F44840: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82F44844: 38884360  addi r4, r8, 0x4360
	ctx.r[4].s64 = ctx.r[8].s64 + 17248;
	// 82F44848: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82F4484C: 3947BB88  addi r10, r7, -0x4478
	ctx.r[10].s64 = ctx.r[7].s64 + -17528;
	// 82F44850: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82F44854: 90810058  stw r4, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 82F44858: 38C00005  li r6, 5
	ctx.r[6].s64 = 5;
	// 82F4485C: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82F44860: 38A00005  li r5, 5
	ctx.r[5].s64 = 5;
	// 82F44864: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 82F44868: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F4486C: 4BFE0BD5  bl 0x82f25440
	ctx.lr = 0x82F44870;
	sub_82F25440(ctx, base);
	// 82F44870: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F44874: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F44878: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4487C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F44880(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F44880 size=160
    let mut pc: u32 = 0x82F44880;
    'dispatch: loop {
        match pc {
            0x82F44880 => {
    //   block [0x82F44880..0x82F44920)
	// 82F44880: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F44884: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F44888: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4488C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F44890: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F44894: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F44898: 39200050  li r9, 0x50
	ctx.r[9].s64 = 80;
	// 82F4489C: 394300C0  addi r10, r3, 0xc0
	ctx.r[10].s64 = ctx.r[3].s64 + 192;
	// 82F448A0: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82F448A4: 39000020  li r8, 0x20
	ctx.r[8].s64 = 32;
	// 82F448A8: 38E00010  li r7, 0x10
	ctx.r[7].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F44920(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F44920 size=148
    let mut pc: u32 = 0x82F44920;
    'dispatch: loop {
        match pc {
            0x82F44920 => {
    //   block [0x82F44920..0x82F449B4)
	// 82F44920: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F44924: 48263845  bl 0x831a8168
	ctx.lr = 0x82F44928;
	sub_831A8130(ctx, base);
	// 82F44928: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4492C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F44930: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F44934: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F44938: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F4493C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F44940: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F44944: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F44948: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F4494C: 419A0040  beq cr6, 0x82f4498c
	if ctx.cr[6].eq {
	pc = 0x82F4498C; continue 'dispatch;
	}
	// 82F44950: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82F44954: 4BF5BDDD  bl 0x82ea0730
	ctx.lr = 0x82F44958;
	sub_82EA0730(ctx, base);
	// 82F44958: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F4495C: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 82F44960: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F44964: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F44968: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F4496C: B13C0004  sth r9, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F44970: 48006AE1  bl 0x82f4b450
	ctx.lr = 0x82F44974;
	sub_82F4B450(ctx, base);
	// 82F44974: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F44978: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F4497C: 38E8F750  addi r7, r8, -0x8b0
	ctx.r[7].s64 = ctx.r[8].s64 + -2224;
	// 82F44980: 90FC0000  stw r7, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F44984: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F44988: 48263830  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82F4498C: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82F44990: 4BF5BDA1  bl 0x82ea0730
	ctx.lr = 0x82F44994;
	sub_82EA0730(ctx, base);
	// 82F44994: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82F44998: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82F4499C: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F449A0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F449A4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F449A8: 48005E01  bl 0x82f4a7a8
	ctx.lr = 0x82F449AC;
	sub_82F4A7A8(ctx, base);
	// 82F449AC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F449B0: 48263808  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F449B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F449B8 size=108
    let mut pc: u32 = 0x82F449B8;
    'dispatch: loop {
        match pc {
            0x82F449B8 => {
    //   block [0x82F449B8..0x82F44A24)
	// 82F449B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F449BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F449C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F449C4: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F449C8: 3D4082F5  lis r10, -0x7d0b
	ctx.r[10].s64 = -2097872896;
	// 82F449CC: 3D2082F5  lis r9, -0x7d0b
	ctx.r[9].s64 = -2097872896;
	// 82F449D0: 3D0082F5  lis r8, -0x7d0b
	ctx.r[8].s64 = -2097872896;
	// 82F449D4: 38CAAB70  addi r6, r10, -0x5490
	ctx.r[6].s64 = ctx.r[10].s64 + -21648;
	// 82F449D8: 38EB4920  addi r7, r11, 0x4920
	ctx.r[7].s64 = ctx.r[11].s64 + 18720;
	// 82F449DC: 38A9AD18  addi r5, r9, -0x52e8
	ctx.r[5].s64 = ctx.r[9].s64 + -21224;
	// 82F449E0: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F449E4: 3888BB88  addi r4, r8, -0x4478
	ctx.r[4].s64 = ctx.r[8].s64 + -17528;
	// 82F449E8: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F449EC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F449F0: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F449F4: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F449F8: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F449FC: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82F44A00: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F44A04: 99410061  stb r10, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[10].u8 ) };
	// 82F44A08: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F44A0C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F44A10: 4BFE0A31  bl 0x82f25440
	ctx.lr = 0x82F44A14;
	sub_82F25440(ctx, base);
	// 82F44A14: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F44A18: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F44A1C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F44A20: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F44A28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F44A28 size=1476
    let mut pc: u32 = 0x82F44A28;
    'dispatch: loop {
        match pc {
            0x82F44A28 => {
    //   block [0x82F44A28..0x82F44FEC)
	// 82F44A28: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F44A2C: 48263719  bl 0x831a8144
	ctx.lr = 0x82F44A30;
	sub_831A8130(ctx, base);
	// 82F44A30: 9421FB40  stwu r1, -0x4c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-1216 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F44A34: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F44A38: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F44A3C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F44A40: 7EEA5A14  add r23, r10, r11
	ctx.r[23].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F44A44: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F44A48: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F44A4C: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F44A50: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F44A54: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82F44A58: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F44A5C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F44A60: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F44A64: 40980020  bge cr6, 0x82f44a84
	if !ctx.cr[6].lt {
	pc = 0x82F44A84; continue 'dispatch;
	}
	// 82F44A68: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F44A6C: 3909FD84  addi r8, r9, -0x27c
	ctx.r[8].s64 = ctx.r[9].s64 + -636;
	// 82F44A70: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F44A74: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F44A78: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F44A7C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F44A80: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F44A84: C01C0018  lfs f0, 0x18(r28)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F44A88: C03F0050  lfs f1, 0x50(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F44A8C: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 82F44A90: 419A017C  beq cr6, 0x82f44c0c
	if ctx.cr[6].eq {
	pc = 0x82F44C0C; continue 'dispatch;
	}
	// 82F44A94: 817F0060  lwz r11, 0x60(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F44A98: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F44A9C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F44AA0: 409A005C  bne cr6, 0x82f44afc
	if !ctx.cr[6].eq {
	pc = 0x82F44AFC; continue 'dispatch;
	}
	// 82F44AA4: C01F0054  lfs f0, 0x54(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F44AA8: D01C0018  stfs f0, 0x18(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82F44AAC: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82F44AB0: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F44AB4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F44AB8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F44ABC: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F44AC0: 48006B21  bl 0x82f4b5e0
	ctx.lr = 0x82F44AC4;
	sub_82F4B5E0(ctx, base);
	// 82F44AC4: 81570000  lwz r10, 0(r23)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F44AC8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F44ACC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F44AD0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F44AD4: 40980020  bge cr6, 0x82f44af4
	if !ctx.cr[6].lt {
	pc = 0x82F44AF4; continue 'dispatch;
	}
	// 82F44AD8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F44ADC: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F44AE0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F44AE4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F44AE8: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F44AEC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F44AF0: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F44AF4: 382104C0  addi r1, r1, 0x4c0
	ctx.r[1].s64 = ctx.r[1].s64 + 1216;
	// 82F44AF8: 4826369C  b 0x831a8194
	sub_831A8180(ctx, base);
	return;
	// 82F44AFC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F44B00: 39410410  addi r10, r1, 0x410
	ctx.r[10].s64 = ctx.r[1].s64 + 1040;
	// 82F44B04: 813D0004  lwz r9, 4(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F44B08: 390102B0  addi r8, r1, 0x2b0
	ctx.r[8].s64 = ctx.r[1].s64 + 688;
	// 82F44B0C: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F44B10: 38A10410  addi r5, r1, 0x410
	ctx.r[5].s64 = ctx.r[1].s64 + 1040;
	// 82F44B14: 80DE0004  lwz r6, 4(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F44B18: 93A1008C  stw r29, 0x8c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), ctx.r[29].u32 ) };
	// 82F44B1C: 91610080  stw r11, 0x80(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[11].u32 ) };
	// 82F44B20: 91210084  stw r9, 0x84(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(132 as u32), ctx.r[9].u32 ) };
	// 82F44B24: 91410088  stw r10, 0x88(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(136 as u32), ctx.r[10].u32 ) };
	// 82F44B28: 90E10070  stw r7, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[7].u32 ) };
	// 82F44B2C: 93C1007C  stw r30, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[30].u32 ) };
	// 82F44B30: 90C10074  stw r6, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[6].u32 ) };
	// 82F44B34: 91010078  stw r8, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[8].u32 ) };
	// 82F44B38: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F44B3C: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 82F44B40: 482B9A49  bl 0x831fe588
	ctx.lr = 0x82F44B44;
	sub_831FE588(ctx, base);
	// 82F44B44: 817E0008  lwz r11, 8(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F44B48: 38A102B0  addi r5, r1, 0x2b0
	ctx.r[5].s64 = ctx.r[1].s64 + 688;
	// 82F44B4C: C03F0050  lfs f1, 0x50(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(80 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F44B50: 386B0040  addi r3, r11, 0x40
	ctx.r[3].s64 = ctx.r[11].s64 + 64;
	// 82F44B54: 482B9A35  bl 0x831fe588
	ctx.lr = 0x82F44B58;
	sub_831FE588(ctx, base);
	// 82F44B58: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F44B5C: 887C0016  lbz r3, 0x16(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(22 as u32) ) } as u64;
	// 82F44B60: 3B7C000C  addi r27, r28, 0xc
	ctx.r[27].s64 = ctx.r[28].s64 + 12;
	// 82F44B64: 91610140  stw r11, 0x140(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[11].u32 ) };
	// 82F44B68: 38C10150  addi r6, r1, 0x150
	ctx.r[6].s64 = ctx.r[1].s64 + 336;
	// 82F44B6C: 91610144  stw r11, 0x144(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(324 as u32), ctx.r[11].u32 ) };
	// 82F44B70: 5469E13E  srwi r9, r3, 4
	ctx.r[9].u32 = ctx.r[3].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F44B74: 88BC0014  lbz r5, 0x14(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F44B78: 5468073E  clrlwi r8, r3, 0x1c
	ctx.r[8].u64 = ctx.r[3].u32 as u64 & 0x0000000Fu64;
	// 82F44B7C: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F44B80: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82F44B84: 91210138  stw r9, 0x138(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(312 as u32), ctx.r[9].u32 ) };
	// 82F44B88: 9101013C  stw r8, 0x13c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(316 as u32), ctx.r[8].u32 ) };
	// 82F44B8C: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82F44B90: 90A10130  stw r5, 0x130(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(304 as u32), ctx.r[5].u32 ) };
	// 82F44B94: 897C0015  lbz r11, 0x15(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(21 as u32) ) } as u64;
	// 82F44B98: 835E0000  lwz r26, 0(r30)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F44B9C: 91610134  stw r11, 0x134(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(308 as u32), ctx.r[11].u32 ) };
	// 82F44BA0: 80EA0000  lwz r7, 0(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F44BA4: 81670034  lwz r11, 0x34(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F44BA8: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F44BAC: 4E800421  bctrl
	ctx.lr = 0x82F44BB0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F44BB0: 815A0000  lwz r10, 0(r26)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F44BB4: 81210130  lwz r9, 0x130(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(304 as u32) ) } as u64;
	// 82F44BB8: 38C10210  addi r6, r1, 0x210
	ctx.r[6].s64 = ctx.r[1].s64 + 528;
	// 82F44BBC: 80A10134  lwz r5, 0x134(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(308 as u32) ) } as u64;
	// 82F44BC0: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82F44BC4: 552B083C  slwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F44BC8: 810A0034  lwz r8, 0x34(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F44BCC: 7C8BDA14  add r4, r11, r27
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[27].u64;
	// 82F44BD0: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F44BD4: 4E800421  bctrl
	ctx.lr = 0x82F44BD8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F44BD8: 80BF0060  lwz r5, 0x60(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F44BDC: 38FC0020  addi r7, r28, 0x20
	ctx.r[7].s64 = ctx.r[28].s64 + 32;
	// 82F44BE0: 38C10130  addi r6, r1, 0x130
	ctx.r[6].s64 = ctx.r[1].s64 + 304;
	// 82F44BE4: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F44BE8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82F44BEC: C0250000  lfs f1, 0(r5)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F44BF0: 48005CA9  bl 0x82f4a898
	ctx.lr = 0x82F44BF4;
	sub_82F4A898(ctx, base);
	// 82F44BF4: 80810144  lwz r4, 0x144(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(324 as u32) ) } as u64;
	// 82F44BF8: 2F040000  cmpwi cr6, r4, 0
	ctx.cr[6].compare_i32(ctx.r[4].s32, 0, &mut ctx.xer);
	// 82F44BFC: 419A0010  beq cr6, 0x82f44c0c
	if ctx.cr[6].eq {
	pc = 0x82F44C0C; continue 'dispatch;
	}
	// 82F44C00: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82F44C04: 38610130  addi r3, r1, 0x130
	ctx.r[3].s64 = ctx.r[1].s64 + 304;
	// 82F44C08: 480409D1  bl 0x82f855d8
	ctx.lr = 0x82F44C0C;
	sub_82F855D8(ctx, base);
	// 82F44C0C: C01F0054  lfs f0, 0x54(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(84 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F44C10: 38A00010  li r5, 0x10
	ctx.r[5].s64 = 16;
	// 82F44C14: D01C0018  stfs f0, 0x18(r28)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82F44C18: 817D0008  lwz r11, 8(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F44C1C: C18B005C  lfs f12, 0x5c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F44C20: 815E0008  lwz r10, 8(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F44C24: C1BF0058  lfs f13, 0x58(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(88 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F44C28: 392B0040  addi r9, r11, 0x40
	ctx.r[9].s64 = ctx.r[11].s64 + 64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F44FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F44FF0 size=144
    let mut pc: u32 = 0x82F44FF0;
    'dispatch: loop {
        match pc {
            0x82F44FF0 => {
    //   block [0x82F44FF0..0x82F45080)
	// 82F44FF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F44FF4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F44FF8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F44FFC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F45000: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F45004: 817F001C  lwz r11, 0x1c(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F45008: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82F4500C: 409A0020  bne cr6, 0x82f4502c
	if !ctx.cr[6].eq {
	pc = 0x82F4502C; continue 'dispatch;
	}
	// 82F45010: 807F0014  lwz r3, 0x14(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F45014: 80BF0010  lwz r5, 0x10(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F45018: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4501C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45020: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F45024: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F45028: 4E800421  bctrl
	ctx.lr = 0x82F4502C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4502C: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F45030: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82F45034: 409A0020  bne cr6, 0x82f45054
	if !ctx.cr[6].eq {
	pc = 0x82F45054; continue 'dispatch;
	}
	// 82F45038: 807F0018  lwz r3, 0x18(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F4503C: 80BF000C  lwz r5, 0xc(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45040: 809F0010  lwz r4, 0x10(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F45044: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45048: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F4504C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F45050: 4E800421  bctrl
	ctx.lr = 0x82F45054;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F45054: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45058: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4505C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F45060: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45064: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F45068: 4E800421  bctrl
	ctx.lr = 0x82F4506C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4506C: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F45070: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F45074: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F45078: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4507C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45080(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F45080 size=4
    let mut pc: u32 = 0x82F45080;
    'dispatch: loop {
        match pc {
            0x82F45080 => {
    //   block [0x82F45080..0x82F45084)
	// 82F45080: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45088(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F45088 size=20
    let mut pc: u32 = 0x82F45088;
    'dispatch: loop {
        match pc {
            0x82F45088 => {
    //   block [0x82F45088..0x82F4509C)
	// 82F45088: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4508C: 7CE33B78  mr r3, r7
	ctx.r[3].u64 = ctx.r[7].u64;
	// 82F45090: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F45094: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F45098: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F450A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F450A0 size=28
    let mut pc: u32 = 0x82F450A0;
    'dispatch: loop {
        match pc {
            0x82F450A0 => {
    //   block [0x82F450A0..0x82F450BC)
	// 82F450A0: 81660000  lwz r11, 0(r6)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F450A4: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F450A8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F450AC: 7CC33378  mr r3, r6
	ctx.r[3].u64 = ctx.r[6].u64;
	// 82F450B0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F450B4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F450B8: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F450C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F450C0 size=4
    let mut pc: u32 = 0x82F450C0;
    'dispatch: loop {
        match pc {
            0x82F450C0 => {
    //   block [0x82F450C0..0x82F450C4)
	// 82F450C0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F450C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F450C8 size=4
    let mut pc: u32 = 0x82F450C8;
    'dispatch: loop {
        match pc {
            0x82F450C8 => {
    //   block [0x82F450C8..0x82F450CC)
	// 82F450C8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F450D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F450D0 size=4
    let mut pc: u32 = 0x82F450D0;
    'dispatch: loop {
        match pc {
            0x82F450D0 => {
    //   block [0x82F450D0..0x82F450D4)
	// 82F450D0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F450D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F450D8 size=4
    let mut pc: u32 = 0x82F450D8;
    'dispatch: loop {
        match pc {
            0x82F450D8 => {
    //   block [0x82F450D8..0x82F450DC)
	// 82F450D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F450E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F450E0 size=124
    let mut pc: u32 = 0x82F450E0;
    'dispatch: loop {
        match pc {
            0x82F450E0 => {
    //   block [0x82F450E0..0x82F4515C)
	// 82F450E0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F450E4: 90C30008  stw r6, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82F450E8: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F450EC: 392BFD90  addi r9, r11, -0x270
	ctx.r[9].s64 = ctx.r[11].s64 + -624;
	// 82F450F0: B1430006  sth r10, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F450F4: 7C8A2378  mr r10, r4
	ctx.r[10].u64 = ctx.r[4].u64;
	// 82F450F8: 91230000  stw r9, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F450FC: 8164000C  lwz r11, 0xc(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45100: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F45104: 419A0014  beq cr6, 0x82f45118
	if ctx.cr[6].eq {
	pc = 0x82F45118; continue 'dispatch;
	}
	// 82F45108: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82F4510C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45110: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F45114: 409AFFF4  bne cr6, 0x82f45108
	if !ctx.cr[6].eq {
	pc = 0x82F45108; continue 'dispatch;
	}
	// 82F45118: 9143000C  stw r10, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[10].u32 ) };
	// 82F4511C: 7CAA2B78  mr r10, r5
	ctx.r[10].u64 = ctx.r[5].u64;
	// 82F45120: 8165000C  lwz r11, 0xc(r5)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45124: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F45128: 419A0014  beq cr6, 0x82f4513c
	if ctx.cr[6].eq {
	pc = 0x82F4513C; continue 'dispatch;
	}
	// 82F4512C: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82F45130: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45134: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F45138: 409AFFF4  bne cr6, 0x82f4512c
	if !ctx.cr[6].eq {
	pc = 0x82F4512C; continue 'dispatch;
	}
	// 82F4513C: 91430010  stw r10, 0x10(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82F45140: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45144: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45148: 9143001C  stw r10, 0x1c(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(28 as u32), ctx.r[10].u32 ) };
	// 82F4514C: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45150: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45154: 91030020  stw r8, 0x20(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[8].u32 ) };
	// 82F45158: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45160(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45160 size=304
    let mut pc: u32 = 0x82F45160;
    'dispatch: loop {
        match pc {
            0x82F45160 => {
    //   block [0x82F45160..0x82F45290)
	// 82F45160: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F45164: 48263001  bl 0x831a8164
	ctx.lr = 0x82F45168;
	sub_831A8130(ctx, base);
	// 82F45168: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4516C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45170: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F45174: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F45178: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F4517C: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F45180: 38800024  li r4, 0x24
	ctx.r[4].s64 = 36;
	// 82F45184: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F45188: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4518C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82F45190: 4BF5B5A1  bl 0x82ea0730
	ctx.lr = 0x82F45194;
	sub_82EA0730(ctx, base);
	// 82F45194: 39200024  li r9, 0x24
	ctx.r[9].s64 = 36;
	// 82F45198: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F4519C: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F451A0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F451A4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F451A8: 4BFFFF39  bl 0x82f450e0
	ctx.lr = 0x82F451AC;
	sub_82F450E0(ctx, base);
	// 82F451AC: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F451B0: 811C001C  lwz r8, 0x1c(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F451B4: 2F08001D  cmpwi cr6, r8, 0x1d
	ctx.cr[6].compare_i32(ctx.r[8].s32, 29, &mut ctx.xer);
	// 82F451B8: 409A0060  bne cr6, 0x82f45218
	if !ctx.cr[6].eq {
	pc = 0x82F45218; continue 'dispatch;
	}
	// 82F451BC: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F451C0: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F451C4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F451C8: 419A0014  beq cr6, 0x82f451dc
	if ctx.cr[6].eq {
	pc = 0x82F451DC; continue 'dispatch;
	}
	// 82F451CC: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82F451D0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F451D4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F451D8: 409AFFF4  bne cr6, 0x82f451cc
	if !ctx.cr[6].eq {
	pc = 0x82F451CC; continue 'dispatch;
	}
	// 82F451DC: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F451E0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F451E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F451E8: 419A0014  beq cr6, 0x82f451fc
	if ctx.cr[6].eq {
	pc = 0x82F451FC; continue 'dispatch;
	}
	// 82F451EC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F451F0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F451F4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F451F8: 409AFFF4  bne cr6, 0x82f451ec
	if !ctx.cr[6].eq {
	pc = 0x82F451EC; continue 'dispatch;
	}
	// 82F451FC: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45200: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F45204: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F45208: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F4520C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F45210: 4E800421  bctrl
	ctx.lr = 0x82F45214;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F45214: 93DC0014  stw r30, 0x14(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(20 as u32), ctx.r[30].u32 ) };
	// 82F45218: 817C0020  lwz r11, 0x20(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F4521C: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82F45220: 409A0064  bne cr6, 0x82f45284
	if !ctx.cr[6].eq {
	pc = 0x82F45284; continue 'dispatch;
	}
	// 82F45224: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45228: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F4522C: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45230: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F45234: 419A0014  beq cr6, 0x82f45248
	if ctx.cr[6].eq {
	pc = 0x82F45248; continue 'dispatch;
	}
	// 82F45238: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82F4523C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45240: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F45244: 409AFFF4  bne cr6, 0x82f45238
	if !ctx.cr[6].eq {
	pc = 0x82F45238; continue 'dispatch;
	}
	// 82F45248: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4524C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F45250: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F45254: 419A0014  beq cr6, 0x82f45268
	if ctx.cr[6].eq {
	pc = 0x82F45268; continue 'dispatch;
	}
	// 82F45258: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F4525C: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45260: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F45264: 409AFFF4  bne cr6, 0x82f45258
	if !ctx.cr[6].eq {
	pc = 0x82F45258; continue 'dispatch;
	}
	// 82F45268: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4526C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F45270: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F45274: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F45278: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4527C: 4E800421  bctrl
	ctx.lr = 0x82F45280;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F45280: 93FC0018  stw r31, 0x18(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(24 as u32), ctx.r[31].u32 ) };
	// 82F45284: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F45288: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F4528C: 48262F28  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45290(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45290 size=236
    let mut pc: u32 = 0x82F45290;
    'dispatch: loop {
        match pc {
            0x82F45290 => {
    //   block [0x82F45290..0x82F4537C)
	// 82F45290: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F45294: 48262ED9  bl 0x831a816c
	ctx.lr = 0x82F45298;
	sub_831A8130(ctx, base);
	// 82F45298: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4529C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F452A0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F452A4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F452A8: 807F0000  lwz r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F452AC: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F452B0: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82F452B4: 409A0058  bne cr6, 0x82f4530c
	if !ctx.cr[6].eq {
	pc = 0x82F4530C; continue 'dispatch;
	}
	// 82F452B8: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F452BC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F452C0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F452C4: 419A0014  beq cr6, 0x82f452d8
	if ctx.cr[6].eq {
	pc = 0x82F452D8; continue 'dispatch;
	}
	// 82F452C8: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82F452CC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F452D0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F452D4: 409AFFF4  bne cr6, 0x82f452c8
	if !ctx.cr[6].eq {
	pc = 0x82F452C8; continue 'dispatch;
	}
	// 82F452D8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F452DC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F452E0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F452E4: 419A0014  beq cr6, 0x82f452f8
	if ctx.cr[6].eq {
	pc = 0x82F452F8; continue 'dispatch;
	}
	// 82F452E8: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F452EC: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F452F0: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F452F4: 409AFFF4  bne cr6, 0x82f452e8
	if !ctx.cr[6].eq {
	pc = 0x82F452E8; continue 'dispatch;
	}
	// 82F452F8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F452FC: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F45300: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F45304: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F45308: 4E800421  bctrl
	ctx.lr = 0x82F4530C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4530C: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45310: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45314: 2F0B001D  cmpwi cr6, r11, 0x1d
	ctx.cr[6].compare_i32(ctx.r[11].s32, 29, &mut ctx.xer);
	// 82F45318: 409A0058  bne cr6, 0x82f45370
	if !ctx.cr[6].eq {
	pc = 0x82F45370; continue 'dispatch;
	}
	// 82F4531C: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45320: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F45324: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F45328: 419A0014  beq cr6, 0x82f4533c
	if ctx.cr[6].eq {
	pc = 0x82F4533C; continue 'dispatch;
	}
	// 82F4532C: 7D655B78  mr r5, r11
	ctx.r[5].u64 = ctx.r[11].u64;
	// 82F45330: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45334: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F45338: 409AFFF4  bne cr6, 0x82f4532c
	if !ctx.cr[6].eq {
	pc = 0x82F4532C; continue 'dispatch;
	}
	// 82F4533C: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45340: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F45344: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F45348: 419A0014  beq cr6, 0x82f4535c
	if ctx.cr[6].eq {
	pc = 0x82F4535C; continue 'dispatch;
	}
	// 82F4534C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F45350: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45354: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F45358: 409AFFF4  bne cr6, 0x82f4534c
	if !ctx.cr[6].eq {
	pc = 0x82F4534C; continue 'dispatch;
	}
	// 82F4535C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45360: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F45364: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F45368: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4536C: 4E800421  bctrl
	ctx.lr = 0x82F45370;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F45370: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F45374: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F45378: 48262E44  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45380(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45380 size=144
    let mut pc: u32 = 0x82F45380;
    'dispatch: loop {
        match pc {
            0x82F45380 => {
    //   block [0x82F45380..0x82F45410)
	// 82F45380: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F45384: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F45388: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4538C: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F45390: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F45394: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F45398: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F4539C: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F453A0: 38CB5160  addi r6, r11, 0x5160
	ctx.r[6].s64 = ctx.r[11].s64 + 20832;
	// 82F453A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F453A8: 38AA50A0  addi r5, r10, 0x50a0
	ctx.r[5].s64 = ctx.r[10].s64 + 20640;
	// 82F453AC: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82F453B0: 388950C8  addi r4, r9, 0x50c8
	ctx.r[4].s64 = ctx.r[9].s64 + 20680;
	// 82F453B4: 386850D8  addi r3, r8, 0x50d8
	ctx.r[3].s64 = ctx.r[8].s64 + 20696;
	// 82F453B8: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82F453BC: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F453C0: 90810058  stw r4, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 82F453C4: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F453C8: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82F453CC: 98E10060  stb r7, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u8 ) };
	// 82F453D0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F453D4: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 82F453D8: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82F453DC: 38A0001D  li r5, 0x1d
	ctx.r[5].s64 = 29;
	// 82F453E0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F453E4: 4BFE005D  bl 0x82f25440
	ctx.lr = 0x82F453E8;
	sub_82F25440(ctx, base);
	// 82F453E8: 38C0001D  li r6, 0x1d
	ctx.r[6].s64 = 29;
	// 82F453EC: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F453F0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F453F4: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F453F8: 4BFE0049  bl 0x82f25440
	ctx.lr = 0x82F453FC;
	sub_82F25440(ctx, base);
	// 82F453FC: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F45400: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F45404: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F45408: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4540C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45410(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45410 size=84
    let mut pc: u32 = 0x82F45410;
    'dispatch: loop {
        match pc {
            0x82F45410 => {
    //   block [0x82F45410..0x82F45464)
	// 82F45410: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F45414: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F45418: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4541C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F45420: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F45424: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45428: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4542C: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F45430: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F45434: 4E800421  bctrl
	ctx.lr = 0x82F45438;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F45438: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4543C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F45440: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F45444: 81090000  lwz r8, 0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45448: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F4544C: 4E800421  bctrl
	ctx.lr = 0x82F45450;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F45450: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F45454: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F45458: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4545C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F45460: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45468(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45468 size=144
    let mut pc: u32 = 0x82F45468;
    'dispatch: loop {
        match pc {
            0x82F45468 => {
    //   block [0x82F45468..0x82F454F8)
	// 82F45468: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4546C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F45470: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F45474: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F45478: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4547C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F45480: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F45484: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F45488: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F4548C: 388AFDD4  addi r4, r10, -0x22c
	ctx.r[4].s64 = ctx.r[10].s64 + -556;
	// 82F45490: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45494: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F45498: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F4549C: 812B0004  lwz r9, 4(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F454A0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F454A4: 4E800421  bctrl
	ctx.lr = 0x82F454A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F454A8: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F454AC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F454B0: 80DF000C  lwz r6, 0xc(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F454B4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F454B8: 3888FDC8  addi r4, r8, -0x238
	ctx.r[4].s64 = ctx.r[8].s64 + -568;
	// 82F454BC: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F454C0: 8167000C  lwz r11, 0xc(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F454C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F454C8: 4E800421  bctrl
	ctx.lr = 0x82F454CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F454CC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F454D0: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F454D4: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F454D8: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F454DC: 4E800421  bctrl
	ctx.lr = 0x82F454E0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F454E0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F454E4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F454E8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F454EC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F454F0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F454F4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F454F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F454F8 size=20
    let mut pc: u32 = 0x82F454F8;
    'dispatch: loop {
        match pc {
            0x82F454F8 => {
    //   block [0x82F454F8..0x82F4550C)
	// 82F454F8: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F454FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45500: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F45504: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F45508: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45510(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F45510 size=20
    let mut pc: u32 = 0x82F45510;
    'dispatch: loop {
        match pc {
            0x82F45510 => {
    //   block [0x82F45510..0x82F45524)
	// 82F45510: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45514: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45518: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F4551C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F45520: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F45528 size=20
    let mut pc: u32 = 0x82F45528;
    'dispatch: loop {
        match pc {
            0x82F45528 => {
    //   block [0x82F45528..0x82F4553C)
	// 82F45528: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4552C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45530: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F45534: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F45538: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45540(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F45540 size=20
    let mut pc: u32 = 0x82F45540;
    'dispatch: loop {
        match pc {
            0x82F45540 => {
    //   block [0x82F45540..0x82F45554)
	// 82F45540: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45544: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45548: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F4554C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F45550: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45558(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F45558 size=20
    let mut pc: u32 = 0x82F45558;
    'dispatch: loop {
        match pc {
            0x82F45558 => {
    //   block [0x82F45558..0x82F4556C)
	// 82F45558: 8063000C  lwz r3, 0xc(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4555C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45560: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F45564: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F45568: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45570(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45570 size=260
    let mut pc: u32 = 0x82F45570;
    'dispatch: loop {
        match pc {
            0x82F45570 => {
    //   block [0x82F45570..0x82F45674)
	// 82F45570: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F45574: 48262BE5  bl 0x831a8158
	ctx.lr = 0x82F45578;
	sub_831A8130(ctx, base);
	// 82F45578: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4557C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45580: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F45584: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F45588: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F4558C: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F45590: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82F45594: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82F45598: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4559C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F455A0: 4BF5B191  bl 0x82ea0730
	ctx.lr = 0x82F455A4;
	sub_82EA0730(ctx, base);
	// 82F455A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F455A8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F455AC: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82F455B0: 38E9FDE8  addi r7, r9, -0x218
	ctx.r[7].s64 = ctx.r[9].s64 + -536;
	// 82F455B4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F455B8: B11F0004  sth r8, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82F455BC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F455C0: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82F455C4: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82F455C8: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F455CC: 837A0000  lwz r27, 0(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F455D0: 833A0008  lwz r25, 8(r26)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F455D4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82F455D8: 831B0014  lwz r24, 0x14(r27)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F455DC: 48000895  bl 0x82f45e70
	ctx.lr = 0x82F455E0;
	sub_82F45E70(ctx, base);
	// 82F455E0: 38BB0030  addi r5, r27, 0x30
	ctx.r[5].s64 = ctx.r[27].s64 + 48;
	// 82F455E4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82F455E8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F455EC: 4BF5B9BD  bl 0x82ea0fa8
	ctx.lr = 0x82F455F0;
	sub_82EA0FA8(ctx, base);
	// 82F455F0: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F455F4: 813E0010  lwz r9, 0x10(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F455F8: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F455FC: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F45600: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45604: 9341005C  stw r26, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 82F45608: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F4560C: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F45610: 392A05A0  addi r9, r10, 0x5a0
	ctx.r[9].s64 = ctx.r[10].s64 + 1440;
	// 82F45614: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 82F45618: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82F4561C: 8178000C  lwz r11, 0xc(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45620: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45624: 409A0008  bne cr6, 0x82f4562c
	if !ctx.cr[6].eq {
	pc = 0x82F4562C; continue 'dispatch;
	}
	// 82F45628: 392A01A0  addi r9, r10, 0x1a0
	ctx.r[9].s64 = ctx.r[10].s64 + 416;
	// 82F4562C: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F45630: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F45634: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F45638: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F4563C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F45640: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F45644: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82F45648: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F4564C: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F45650: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F45654: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F45658: 80E809A0  lwz r7, 0x9a0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82F4565C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F45660: 4E800421  bctrl
	ctx.lr = 0x82F45664;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F45664: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82F45668: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4566C: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82F45670: 48262B38  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45678(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45678 size=548
    let mut pc: u32 = 0x82F45678;
    'dispatch: loop {
        match pc {
            0x82F45678 => {
    //   block [0x82F45678..0x82F4589C)
	// 82F45678: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4567C: 48262ABD  bl 0x831a8138
	ctx.lr = 0x82F45680;
	sub_831A8130(ctx, base);
	// 82F45680: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F45684: 838D0000  lwz r28, 0(r13)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45688: 3B600018  li r27, 0x18
	ctx.r[27].s64 = 24;
	// 82F4568C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82F45690: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F45694: 7CB92B78  mr r25, r5
	ctx.r[25].u64 = ctx.r[5].u64;
	// 82F45698: 7CD83378  mr r24, r6
	ctx.r[24].u64 = ctx.r[6].u64;
	// 82F4569C: 7D7BE02E  lwzx r11, r27, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[27].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82F456A0: 7CF73B78  mr r23, r7
	ctx.r[23].u64 = ctx.r[7].u64;
	// 82F456A4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F456A8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F456AC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F456B0: 40980020  bge cr6, 0x82f456d0
	if !ctx.cr[6].lt {
	pc = 0x82F456D0; continue 'dispatch;
	}
	// 82F456B4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F456B8: 3909FE20  addi r8, r9, -0x1e0
	ctx.r[8].s64 = ctx.r[9].s64 + -480;
	// 82F456BC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F456C0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F456C4: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F456C8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F456CC: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F456D0: 83BE0000  lwz r29, 0(r30)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F456D4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F456D8: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F456DC: 3BFD0030  addi r31, r29, 0x30
	ctx.r[31].s64 = ctx.r[29].s64 + 48;
	// 82F456E0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F456E4: 4BF5B8C5  bl 0x82ea0fa8
	ctx.lr = 0x82F456E8;
	sub_82EA0FA8(ctx, base);
	// 82F456E8: 813E0008  lwz r9, 8(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F456EC: 38A100A0  addi r5, r1, 0xa0
	ctx.r[5].s64 = ctx.r[1].s64 + 160;
	// 82F456F0: 39690040  addi r11, r9, 0x40
	ctx.r[11].s64 = ctx.r[9].s64 + 64;
	// 82F456F4: 38C00010  li r6, 0x10
	ctx.r[6].s64 = 16;
	// 82F456F8: 388100B0  addi r4, r1, 0xb0
	ctx.r[4].s64 = ctx.r[1].s64 + 176;
	// 82F456FC: 390B0020  addi r8, r11, 0x20
	ctx.r[8].s64 = ctx.r[11].s64 + 32;
	// 82F45700: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F458A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F458A0 size=132
    let mut pc: u32 = 0x82F458A0;
    'dispatch: loop {
        match pc {
            0x82F458A0 => {
    //   block [0x82F458A0..0x82F45924)
	// 82F458A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F458A4: 482628B9  bl 0x831a815c
	ctx.lr = 0x82F458A8;
	sub_831A8130(ctx, base);
	// 82F458A8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F458AC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F458B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F458B4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F458B8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F458BC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F458C0: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F458C4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82F458C8: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F458CC: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 82F458D0: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82F458D4: 4BF5B6D5  bl 0x82ea0fa8
	ctx.lr = 0x82F458D8;
	sub_82EA0FA8(ctx, base);
	// 82F458D8: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F458DC: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82F458E0: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F458E4: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82F458E8: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82F458EC: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F458F0: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82F458F4: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82F458F8: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F458FC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F45900: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F45904: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45908: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F4590C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45910: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F45914: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F45918: 4E800421  bctrl
	ctx.lr = 0x82F4591C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4591C: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82F45920: 4826288C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45928(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45928 size=168
    let mut pc: u32 = 0x82F45928;
    'dispatch: loop {
        match pc {
            0x82F45928 => {
    //   block [0x82F45928..0x82F459D0)
	// 82F45928: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4592C: 48262835  bl 0x831a8160
	ctx.lr = 0x82F45930;
	sub_831A8130(ctx, base);
	// 82F45930: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F45934: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F45938: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F4593C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F45940: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F45944: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F45948: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4594C: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82F45950: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F45954: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82F45958: 4BF5B651  bl 0x82ea0fa8
	ctx.lr = 0x82F4595C;
	sub_82EA0FA8(ctx, base);
	// 82F4595C: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F45960: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82F45964: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F45968: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82F4596C: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82F45970: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45974: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F45978: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F4597C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F45980: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F45984: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F45988: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82F4598C: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82F45990: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45994: 392B000D  addi r9, r11, 0xd
	ctx.r[9].s64 = ctx.r[11].s64 + 13;
	// 82F45998: 552B2834  slwi r11, r9, 5
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4599C: 8128000C  lwz r9, 0xc(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F459A0: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F459A4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F459A8: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F459AC: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F459B0: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F459B4: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F459B8: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F459BC: 816809AC  lwz r11, 0x9ac(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2476 as u32) ) } as u64;
	// 82F459C0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F459C4: 4E800421  bctrl
	ctx.lr = 0x82F459C8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F459C8: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82F459CC: 482627E4  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F459D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F459D0 size=124
    let mut pc: u32 = 0x82F459D0;
    'dispatch: loop {
        match pc {
            0x82F459D0 => {
    //   block [0x82F459D0..0x82F45A4C)
	// 82F459D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F459D4: 4826278D  bl 0x831a8160
	ctx.lr = 0x82F459D8;
	sub_831A8130(ctx, base);
	// 82F459D8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F459DC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F459E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F459E4: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F459E8: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F459EC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F459F0: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F459F4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82F459F8: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F459FC: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82F45A00: 4BF5B5A9  bl 0x82ea0fa8
	ctx.lr = 0x82F45A04;
	sub_82EA0FA8(ctx, base);
	// 82F45A04: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F45A08: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82F45A0C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F45A10: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82F45A14: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82F45A18: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F45A1C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F45A20: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F45A24: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F45A28: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45A2C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F45A30: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82F45A34: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45A38: 81680010  lwz r11, 0x10(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F45A3C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F45A40: 4E800421  bctrl
	ctx.lr = 0x82F45A44;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F45A44: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82F45A48: 48262768  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45A50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45A50 size=160
    let mut pc: u32 = 0x82F45A50;
    'dispatch: loop {
        match pc {
            0x82F45A50 => {
    //   block [0x82F45A50..0x82F45AF0)
	// 82F45A50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F45A54: 48262711  bl 0x831a8164
	ctx.lr = 0x82F45A58;
	sub_831A8130(ctx, base);
	// 82F45A58: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F45A5C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F45A60: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F45A64: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F45A68: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F45A6C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F45A70: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45A74: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F45A78: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82F45A7C: 4BF5B52D  bl 0x82ea0fa8
	ctx.lr = 0x82F45A80;
	sub_82EA0FA8(ctx, base);
	// 82F45A80: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F45A84: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82F45A88: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F45A8C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82F45A90: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F45A94: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45A98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F45A9C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F45AA0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F45AA4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F45AA8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82F45AAC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82F45AB0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45AB4: 392B000D  addi r9, r11, 0xd
	ctx.r[9].s64 = ctx.r[11].s64 + 13;
	// 82F45AB8: 552B2834  slwi r11, r9, 5
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F45ABC: 8128000C  lwz r9, 0xc(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45AC0: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45AC4: 7CEB4A14  add r7, r11, r9
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F45AC8: 7D6750AE  lbzx r11, r7, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F45ACC: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F45AD0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F45AD4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F45AD8: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F45ADC: 812A09A8  lwz r9, 0x9a8(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2472 as u32) ) } as u64;
	// 82F45AE0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F45AE4: 4E800421  bctrl
	ctx.lr = 0x82F45AE8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F45AE8: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82F45AEC: 482626C8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45AF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45AF0 size=124
    let mut pc: u32 = 0x82F45AF0;
    'dispatch: loop {
        match pc {
            0x82F45AF0 => {
    //   block [0x82F45AF0..0x82F45B6C)
	// 82F45AF0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F45AF4: 4826266D  bl 0x831a8160
	ctx.lr = 0x82F45AF8;
	sub_831A8130(ctx, base);
	// 82F45AF8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F45AFC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F45B00: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F45B04: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F45B08: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F45B0C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F45B10: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45B14: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82F45B18: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F45B1C: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82F45B20: 4BF5B489  bl 0x82ea0fa8
	ctx.lr = 0x82F45B24;
	sub_82EA0FA8(ctx, base);
	// 82F45B24: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F45B28: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82F45B2C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F45B30: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82F45B34: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82F45B38: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F45B3C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F45B40: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F45B44: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F45B48: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45B4C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F45B50: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82F45B54: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45B58: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45B5C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F45B60: 4E800421  bctrl
	ctx.lr = 0x82F45B64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F45B64: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82F45B68: 48262648  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45B70 size=160
    let mut pc: u32 = 0x82F45B70;
    'dispatch: loop {
        match pc {
            0x82F45B70 => {
    //   block [0x82F45B70..0x82F45C10)
	// 82F45B70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F45B74: 482625F1  bl 0x831a8164
	ctx.lr = 0x82F45B78;
	sub_831A8130(ctx, base);
	// 82F45B78: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F45B7C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F45B80: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F45B84: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F45B88: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F45B8C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F45B90: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45B94: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F45B98: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82F45B9C: 4BF5B40D  bl 0x82ea0fa8
	ctx.lr = 0x82F45BA0;
	sub_82EA0FA8(ctx, base);
	// 82F45BA0: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F45BA4: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82F45BA8: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F45BAC: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82F45BB0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F45BB4: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45BB8: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F45BBC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F45BC0: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F45BC4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F45BC8: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82F45BCC: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82F45BD0: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45BD4: 392B000D  addi r9, r11, 0xd
	ctx.r[9].s64 = ctx.r[11].s64 + 13;
	// 82F45BD8: 552B2834  slwi r11, r9, 5
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F45BDC: 8128000C  lwz r9, 0xc(r8)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45BE0: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45BE4: 7CEB4A14  add r7, r11, r9
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F45BE8: 7D6750AE  lbzx r11, r7, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F45BEC: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F45BF0: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F45BF4: 556B103A  slwi r11, r11, 2
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F45BF8: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F45BFC: 812A09A4  lwz r9, 0x9a4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82F45C00: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F45C04: 4E800421  bctrl
	ctx.lr = 0x82F45C08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F45C08: 382100D0  addi r1, r1, 0xd0
	ctx.r[1].s64 = ctx.r[1].s64 + 208;
	// 82F45C0C: 482625A8  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45C10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45C10 size=124
    let mut pc: u32 = 0x82F45C10;
    'dispatch: loop {
        match pc {
            0x82F45C10 => {
    //   block [0x82F45C10..0x82F45C8C)
	// 82F45C10: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F45C14: 4826254D  bl 0x831a8160
	ctx.lr = 0x82F45C18;
	sub_831A8130(ctx, base);
	// 82F45C18: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F45C1C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F45C20: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F45C24: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F45C28: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F45C2C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F45C30: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45C34: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82F45C38: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F45C3C: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82F45C40: 4BF5B369  bl 0x82ea0fa8
	ctx.lr = 0x82F45C44;
	sub_82EA0FA8(ctx, base);
	// 82F45C44: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F45C48: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82F45C4C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F45C50: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82F45C54: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82F45C58: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F45C5C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F45C60: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F45C64: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F45C68: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45C6C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F45C70: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82F45C74: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45C78: 81680020  lwz r11, 0x20(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F45C7C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F45C80: 4E800421  bctrl
	ctx.lr = 0x82F45C84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F45C84: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82F45C88: 48262528  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45C90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45C90 size=272
    let mut pc: u32 = 0x82F45C90;
    'dispatch: loop {
        match pc {
            0x82F45C90 => {
    //   block [0x82F45C90..0x82F45DA0)
	// 82F45C90: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F45C94: 482624C5  bl 0x831a8158
	ctx.lr = 0x82F45C98;
	sub_831A8130(ctx, base);
	// 82F45C98: 9421FEA0  stwu r1, -0x160(r1)
	ea = ctx.r[1].u32.wrapping_add(-352 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F45C9C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45CA0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F45CA4: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82F45CA8: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F45CAC: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F45CB0: 38800010  li r4, 0x10
	ctx.r[4].s64 = 16;
	// 82F45CB4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F45CB8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F45CBC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F45CC0: 4BF5AA71  bl 0x82ea0730
	ctx.lr = 0x82F45CC4;
	sub_82EA0730(ctx, base);
	// 82F45CC4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F45CC8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F45CCC: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82F45CD0: 38E9FDE8  addi r7, r9, -0x218
	ctx.r[7].s64 = ctx.r[9].s64 + -536;
	// 82F45CD4: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F45CD8: B11F0004  sth r8, 4(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[8].u16 ) };
	// 82F45CDC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F45CE0: B0DF0006  sth r6, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[6].u16 ) };
	// 82F45CE4: 939F0008  stw r28, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[28].u32 ) };
	// 82F45CE8: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F45CEC: 837A0000  lwz r27, 0(r26)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45CF0: 833A0008  lwz r25, 8(r26)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F45CF4: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82F45CF8: 831B0014  lwz r24, 0x14(r27)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F45CFC: 48000175  bl 0x82f45e70
	ctx.lr = 0x82F45D00;
	sub_82F45E70(ctx, base);
	// 82F45D00: 38BB0030  addi r5, r27, 0x30
	ctx.r[5].s64 = ctx.r[27].s64 + 48;
	// 82F45D04: 7F24CB78  mr r4, r25
	ctx.r[4].u64 = ctx.r[25].u64;
	// 82F45D08: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F45D0C: 4BF5B29D  bl 0x82ea0fa8
	ctx.lr = 0x82F45D10;
	sub_82EA0FA8(ctx, base);
	// 82F45D10: 807D0000  lwz r3, 0(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45D14: 813E0010  lwz r9, 0x10(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F45D18: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F45D1C: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F45D20: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F45D24: 9341005C  stw r26, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[26].u32 ) };
	// 82F45D28: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F45D2C: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F45D30: 392A05A0  addi r9, r10, 0x5a0
	ctx.r[9].s64 = ctx.r[10].s64 + 1440;
	// 82F45D34: 93010050  stw r24, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[24].u32 ) };
	// 82F45D38: 90810054  stw r4, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[4].u32 ) };
	// 82F45D3C: 8178000C  lwz r11, 0xc(r24)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45D40: 8103000C  lwz r8, 0xc(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F45D44: 409A0008  bne cr6, 0x82f45d4c
	if !ctx.cr[6].eq {
	pc = 0x82F45D4C; continue 'dispatch;
	}
	// 82F45D48: 392A01A0  addi r9, r10, 0x1a0
	ctx.r[9].s64 = ctx.r[10].s64 + 416;
	// 82F45D4C: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F45D50: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F45D54: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F45D58: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F45D5C: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F45D60: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F45D64: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82F45D68: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F45D6C: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F45D70: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F45D74: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F45D78: 80E809A0  lwz r7, 0x9a0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82F45D7C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F45D80: 4E800421  bctrl
	ctx.lr = 0x82F45D84;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F45D84: 3CC08213  lis r6, -0x7ded
	ctx.r[6].s64 = -2112684032;
	// 82F45D88: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82F45D8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F45D90: 38A6FE30  addi r5, r6, -0x1d0
	ctx.r[5].s64 = ctx.r[6].s64 + -464;
	// 82F45D94: 90BF0000  stw r5, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[5].u32 ) };
	// 82F45D98: 38210160  addi r1, r1, 0x160
	ctx.r[1].s64 = ctx.r[1].s64 + 352;
	// 82F45D9C: 4826240C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45DA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45DA0 size=204
    let mut pc: u32 = 0x82F45DA0;
    'dispatch: loop {
        match pc {
            0x82F45DA0 => {
    //   block [0x82F45DA0..0x82F45E6C)
	// 82F45DA0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F45DA4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F45DA8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F45DAC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F45DB0: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F45DB4: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F45DB8: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F45DBC: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F45DC0: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F45DC4: 38CA5F80  addi r6, r10, 0x5f80
	ctx.r[6].s64 = ctx.r[10].s64 + 24448;
	// 82F45DC8: 38A95FC8  addi r5, r9, 0x5fc8
	ctx.r[5].s64 = ctx.r[9].s64 + 24520;
	// 82F45DCC: 38886018  addi r4, r8, 0x6018
	ctx.r[4].s64 = ctx.r[8].s64 + 24600;
	// 82F45DD0: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F45DD4: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82F45DD8: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F45DDC: 38EB5C90  addi r7, r11, 0x5c90
	ctx.r[7].s64 = ctx.r[11].s64 + 23696;
	// 82F45DE0: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F45DE4: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 82F45DE8: 38C0001C  li r6, 0x1c
	ctx.r[6].s64 = 28;
	// 82F45DEC: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F45DF0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F45DF4: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F45DF8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F45DFC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F45E00: 4BFDF641  bl 0x82f25440
	ctx.lr = 0x82F45E04;
	sub_82F25440(ctx, base);
	// 82F45E04: 3C6082F4  lis r3, -0x7d0c
	ctx.r[3].s64 = -2097938432;
	// 82F45E08: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F45E0C: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F45E10: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F45E14: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F45E18: 39035570  addi r8, r3, 0x5570
	ctx.r[8].s64 = ctx.r[3].s64 + 21872;
	// 82F45E1C: 38CA5A50  addi r6, r10, 0x5a50
	ctx.r[6].s64 = ctx.r[10].s64 + 23120;
	// 82F45E20: 38A95928  addi r5, r9, 0x5928
	ctx.r[5].s64 = ctx.r[9].s64 + 22824;
	// 82F45E24: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82F45E28: 38EB5B70  addi r7, r11, 0x5b70
	ctx.r[7].s64 = ctx.r[11].s64 + 23408;
	// 82F45E2C: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82F45E30: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F45E34: 90A1007C  stw r5, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 82F45E38: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82F45E3C: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82F45E40: 98810080  stb r4, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u8 ) };
	// 82F45E44: 38A0001C  li r5, 0x1c
	ctx.r[5].s64 = 28;
	// 82F45E48: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F45E4C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F45E50: 4BFDF5F1  bl 0x82f25440
	ctx.lr = 0x82F45E54;
	sub_82F25440(ctx, base);
	// 82F45E54: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F45E58: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F45E5C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F45E60: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F45E64: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F45E68: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45E70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45E70 size=272
    let mut pc: u32 = 0x82F45E70;
    'dispatch: loop {
        match pc {
            0x82F45E70 => {
    //   block [0x82F45E70..0x82F45F80)
	// 82F45E70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F45E74: 482622F1  bl 0x831a8164
	ctx.lr = 0x82F45E78;
	sub_831A8130(ctx, base);
	// 82F45E78: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F45E7C: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F45E80: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82F45E84: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F45E88: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82F45E8C: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
	// 82F45E90: E97F0020  ld r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) };
	// 82F45E94: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F45E98: E93F0028  ld r9, 0x28(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) };
	// 82F45E9C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F45EA0: E8FF0030  ld r7, 0x30(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(48 as u32) ) };
	// 82F45EA4: 3B610060  addi r27, r1, 0x60
	ctx.r[27].s64 = ctx.r[1].s64 + 96;
	// 82F45EA8: E8BF0038  ld r5, 0x38(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(56 as u32) ) };
	// 82F45EAC: E87F0000  ld r3, 0(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) };
	// 82F45EB0: F96A0000  std r11, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[11].u64 ) };
	// 82F45EB4: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F45EB8: F92A0008  std r9, 8(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(8 as u32), ctx.r[9].u64 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45F80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F45F80 size=72
    let mut pc: u32 = 0x82F45F80;
    'dispatch: loop {
        match pc {
            0x82F45F80 => {
    //   block [0x82F45F80..0x82F45FC8)
	// 82F45F80: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F45F84: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F45F88: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F45F8C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F45F90: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F45F94: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F45F98: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F45F9C: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F45FA0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F45FA4: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F45FA8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F45FAC: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F45FB0: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F45FB4: 4BFFFBBD  bl 0x82f45b70
	ctx.lr = 0x82F45FB8;
	sub_82F45B70(ctx, base);
	// 82F45FB8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F45FBC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F45FC0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F45FC4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F45FC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F45FC8 size=76
    let mut pc: u32 = 0x82F45FC8;
    'dispatch: loop {
        match pc {
            0x82F45FC8 => {
    //   block [0x82F45FC8..0x82F46014)
	// 82F45FC8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F45FCC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F45FD0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F45FD4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F45FD8: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F45FDC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F45FE0: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F45FE4: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F45FE8: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F45FEC: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F45FF0: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F45FF4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F45FF8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F45FFC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F46000: 4BFFFA51  bl 0x82f45a50
	ctx.lr = 0x82F46004;
	sub_82F45A50(ctx, base);
	// 82F46004: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F46008: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4600C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F46010: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F46018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F46018 size=224
    let mut pc: u32 = 0x82F46018;
    'dispatch: loop {
        match pc {
            0x82F46018 => {
    //   block [0x82F46018..0x82F460F8)
	// 82F46018: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4601C: 48262151  bl 0x831a816c
	ctx.lr = 0x82F46020;
	sub_831A8130(ctx, base);
	// 82F46020: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F46024: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F46028: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F4602C: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82F46030: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F46034: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F46038: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F4603C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F46040: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F46044: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F46048: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F4604C: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F46050: 4200FFF0  bdnz 0x82f46040
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F46040; continue 'dispatch;
	}
	// 82F46054: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F46058: E9250050  ld r9, 0x50(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(80 as u32) ) };
	// 82F4605C: E8E50058  ld r7, 0x58(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(88 as u32) ) };
	// 82F46060: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82F46064: 38AAC5B0  addi r5, r10, -0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + -14928;
	// 82F46068: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 82F4606C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F46070: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F460F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F460F8 size=140
    let mut pc: u32 = 0x82F460F8;
    'dispatch: loop {
        match pc {
            0x82F460F8 => {
    //   block [0x82F460F8..0x82F46184)
	// 82F460F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F460FC: 48262065  bl 0x831a8160
	ctx.lr = 0x82F46100;
	sub_831A8130(ctx, base);
	// 82F46100: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F46104: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F46108: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F4610C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F46110: 394BF400  addi r10, r11, -0xc00
	ctx.r[10].s64 = ctx.r[11].s64 + -3072;
	// 82F46114: 9BC10054  stb r30, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u8 ) };
	// 82F46118: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F4611C: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82F46120: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F46124: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F46128: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F4612C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F46130: 83FB0000  lwz r31, 0(r27)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46134: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F46138: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82F4613C: 4BF5AE6D  bl 0x82ea0fa8
	ctx.lr = 0x82F46140;
	sub_82EA0FA8(ctx, base);
	// 82F46140: 39210070  addi r9, r1, 0x70
	ctx.r[9].s64 = ctx.r[1].s64 + 112;
	// 82F46144: 9361006C  stw r27, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 82F46148: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F4614C: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82F46150: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82F46154: 811F0014  lwz r8, 0x14(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F46158: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F4615C: 91010060  stw r8, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[8].u32 ) };
	// 82F46160: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F46164: 807D000C  lwz r3, 0xc(r29)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46168: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82F4616C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46170: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46174: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46178: 4E800421  bctrl
	ctx.lr = 0x82F4617C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4617C: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82F46180: 48262030  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F46188(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F46188 size=152
    let mut pc: u32 = 0x82F46188;
    'dispatch: loop {
        match pc {
            0x82F46188 => {
    //   block [0x82F46188..0x82F46220)
	// 82F46188: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4618C: 48261FD9  bl 0x831a8164
	ctx.lr = 0x82F46190;
	sub_831A8130(ctx, base);
	// 82F46190: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F46194: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F46198: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F4619C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F461A0: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F461A4: 392AF3E8  addi r9, r10, -0xc18
	ctx.r[9].s64 = ctx.r[10].s64 + -3096;
	// 82F461A8: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F461AC: C00BBA78  lfs f0, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F461B0: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F461B4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F461B8: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F461BC: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F461C0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F461C4: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F461C8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F461CC: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82F461D0: 4BF5ADD9  bl 0x82ea0fa8
	ctx.lr = 0x82F461D4;
	sub_82EA0FA8(ctx, base);
	// 82F461D4: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82F461D8: 9381006C  stw r28, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[28].u32 ) };
	// 82F461DC: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F461E0: 91010068  stw r8, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[8].u32 ) };
	// 82F461E4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F461E8: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F461EC: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F461F0: 90610064  stw r3, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[3].u32 ) };
	// 82F461F4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F461F8: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82F461FC: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F46200: 815E000C  lwz r10, 0xc(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46204: 7D435378  mr r3, r10
	ctx.r[3].u64 = ctx.r[10].u64;
	// 82F46208: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4620C: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F46210: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F46214: 4E800421  bctrl
	ctx.lr = 0x82F46218;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46218: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82F4621C: 48261F98  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F46220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F46220 size=176
    let mut pc: u32 = 0x82F46220;
    'dispatch: loop {
        match pc {
            0x82F46220 => {
    //   block [0x82F46220..0x82F462D0)
	// 82F46220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F46224: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F46228: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4622C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F46230: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F46234: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F46238: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F4623C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F46240: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F46244: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F46248: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4624C: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F46250: 4BFFF429  bl 0x82f45678
	ctx.lr = 0x82F46254;
	sub_82F45678(ctx, base);
	// 82F46254: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46258: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F4625C: 40980044  bge cr6, 0x82f462a0
	if !ctx.cr[6].lt {
	pc = 0x82F462A0; continue 'dispatch;
	}
	// 82F46260: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F46264: 394BC5A0  addi r10, r11, -0x3a60
	ctx.r[10].s64 = ctx.r[11].s64 + -14944;
	// 82F46268: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F462D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F462D0 size=124
    let mut pc: u32 = 0x82F462D0;
    'dispatch: loop {
        match pc {
            0x82F462D0 => {
    //   block [0x82F462D0..0x82F4634C)
	// 82F462D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F462D4: 48261E8D  bl 0x831a8160
	ctx.lr = 0x82F462D8;
	sub_831A8130(ctx, base);
	// 82F462D8: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F462DC: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F462E0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F462E4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F462E8: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F462EC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F462F0: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F462F4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82F462F8: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F462FC: 38BF0030  addi r5, r31, 0x30
	ctx.r[5].s64 = ctx.r[31].s64 + 48;
	// 82F46300: 4BF5ACA9  bl 0x82ea0fa8
	ctx.lr = 0x82F46304;
	sub_82EA0FA8(ctx, base);
	// 82F46304: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F46308: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82F4630C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F46310: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82F46314: 7F47D378  mr r7, r26
	ctx.r[7].u64 = ctx.r[26].u64;
	// 82F46318: 813F0014  lwz r9, 0x14(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F4631C: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F46320: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F46324: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F46328: 807E000C  lwz r3, 0xc(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4632C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F46330: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82F46334: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46338: 81680020  lwz r11, 0x20(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F4633C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F46340: 4E800421  bctrl
	ctx.lr = 0x82F46344;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46344: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82F46348: 48261E68  b 0x831a81b0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F46350(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F46350 size=224
    let mut pc: u32 = 0x82F46350;
    'dispatch: loop {
        match pc {
            0x82F46350 => {
    //   block [0x82F46350..0x82F46430)
	// 82F46350: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F46354: 48261E15  bl 0x831a8168
	ctx.lr = 0x82F46358;
	sub_831A8130(ctx, base);
	// 82F46358: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4635C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F46360: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F46364: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82F46368: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F4636C: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82F46370: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F46374: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F46378: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F4637C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F46380: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F46384: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F46388: 4200FFF0  bdnz 0x82f46378
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F46378; continue 'dispatch;
	}
	// 82F4638C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F46390: E9260050  ld r9, 0x50(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(80 as u32) ) };
	// 82F46394: E9060058  ld r8, 0x58(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	// 82F46398: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82F4639C: 38CAC5B0  addi r6, r10, -0x3a50
	ctx.r[6].s64 = ctx.r[10].s64 + -14928;
	// 82F463A0: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82F463A4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F463A8: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F46430(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F46430 size=124
    let mut pc: u32 = 0x82F46430;
    'dispatch: loop {
        match pc {
            0x82F46430 => {
    //   block [0x82F46430..0x82F464AC)
	// 82F46430: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F46434: 48261D35  bl 0x831a8168
	ctx.lr = 0x82F46438;
	sub_831A8130(ctx, base);
	// 82F46438: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4643C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F46440: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F46444: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F46448: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4644C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F46450: 4099003C  ble cr6, 0x82f4648c
	if !ctx.cr[6].gt {
	pc = 0x82F4648C; continue 'dispatch;
	}
	// 82F46454: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F46458: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4645C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F46460: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82F46464: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46468: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4646C: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F46470: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F46474: 4E800421  bctrl
	ctx.lr = 0x82F46478;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46478: 811D0010  lwz r8, 0x10(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4647C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82F46480: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82F46484: 7F1F4000  cmpw cr6, r31, r8
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F46488: 4198FFD0  blt cr6, 0x82f46458
	if ctx.cr[6].lt {
	pc = 0x82F46458; continue 'dispatch;
	}
	// 82F4648C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46490: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F46494: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F46498: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4649C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F464A0: 4E800421  bctrl
	ctx.lr = 0x82F464A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F464A4: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F464A8: 48261D10  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F464B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F464B0 size=100
    let mut pc: u32 = 0x82F464B0;
    'dispatch: loop {
        match pc {
            0x82F464B0 => {
    //   block [0x82F464B0..0x82F46514)
	// 82F464B0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F464B4: 48261CB5  bl 0x831a8168
	ctx.lr = 0x82F464B8;
	sub_831A8130(ctx, base);
	// 82F464B8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F464BC: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F464C0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F464C4: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F464C8: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F464CC: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F464D0: 4099003C  ble cr6, 0x82f4650c
	if !ctx.cr[6].gt {
	pc = 0x82F4650C; continue 'dispatch;
	}
	// 82F464D4: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F464D8: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F464DC: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F464E0: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82F464E4: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F464E8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F464EC: 812A0024  lwz r9, 0x24(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F464F0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F464F4: 4E800421  bctrl
	ctx.lr = 0x82F464F8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F464F8: 811D0010  lwz r8, 0x10(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F464FC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82F46500: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82F46504: 7F1F4000  cmpw cr6, r31, r8
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F46508: 4198FFD0  blt cr6, 0x82f464d8
	if ctx.cr[6].lt {
	pc = 0x82F464D8; continue 'dispatch;
	}
	// 82F4650C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F46510: 48261CA8  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F46518(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F46518 size=132
    let mut pc: u32 = 0x82F46518;
    'dispatch: loop {
        match pc {
            0x82F46518 => {
    //   block [0x82F46518..0x82F4659C)
	// 82F46518: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4651C: 48261C4D  bl 0x831a8168
	ctx.lr = 0x82F46520;
	sub_831A8130(ctx, base);
	// 82F46520: DBC1FFC8  stfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-56 as u32), ctx.f[30].u64 ) };
	// 82F46524: DBE1FFD0  stfd f31, -0x30(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-48 as u32), ctx.f[31].u64 ) };
	// 82F46528: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4652C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F46530: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F46534: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82F46538: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F4653C: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F46540: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F46544: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F46548: 40990044  ble cr6, 0x82f4658c
	if !ctx.cr[6].gt {
	pc = 0x82F4658C; continue 'dispatch;
	}
	// 82F4654C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F46550: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46554: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F46558: FC40F090  fmr f2, f30
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82F4655C: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82F46560: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82F46564: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46568: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4656C: 812A0028  lwz r9, 0x28(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F46570: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F46574: 4E800421  bctrl
	ctx.lr = 0x82F46578;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46578: 811D0010  lwz r8, 0x10(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4657C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82F46580: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82F46584: 7F1F4000  cmpw cr6, r31, r8
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F46588: 4198FFC8  blt cr6, 0x82f46550
	if ctx.cr[6].lt {
	pc = 0x82F46550; continue 'dispatch;
	}
	// 82F4658C: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82F46590: CBC1FFC8  lfd f30, -0x38(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-56 as u32) ) };
	// 82F46594: CBE1FFD0  lfd f31, -0x30(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-48 as u32) ) };
	// 82F46598: 48261C20  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F465A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F465A0 size=284
    let mut pc: u32 = 0x82F465A0;
    'dispatch: loop {
        match pc {
            0x82F465A0 => {
    //   block [0x82F465A0..0x82F466BC)
	// 82F465A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F465A4: 48261BB1  bl 0x831a8154
	ctx.lr = 0x82F465A8;
	sub_831A8130(ctx, base);
	// 82F465A8: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F465AC: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F465B0: 3AE00018  li r23, 0x18
	ctx.r[23].s64 = 24;
	// 82F465B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F465B8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F465BC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F465C0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F465C4: 7D77C02E  lwzx r11, r23, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F465C8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F465CC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F465D0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F465D4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F465D8: 40980020  bge cr6, 0x82f465f8
	if !ctx.cr[6].lt {
	pc = 0x82F465F8; continue 'dispatch;
	}
	// 82F465DC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F465E0: 3909FE68  addi r8, r9, -0x198
	ctx.r[8].s64 = ctx.r[9].s64 + -408;
	// 82F465E4: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F465E8: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F465EC: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F465F0: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F465F4: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F465F8: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F465FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46600: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F46604: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46608: 4E800421  bctrl
	ctx.lr = 0x82F4660C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4660C: 811C0008  lwz r8, 8(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F46610: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F46614: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F46618: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4661C: 37C9FFFF  addic. r30, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F46620: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82F46624: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82F46628: 4180005C  blt 0x82f46684
	if ctx.cr[0].lt {
	pc = 0x82F46684; continue 'dispatch;
	}
	// 82F4662C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46630: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F46634: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46638: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F4663C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F46640: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F46644: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46648: 4E800421  bctrl
	ctx.lr = 0x82F4664C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4664C: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82F46650: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82F46654: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82F46658: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4665C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82F46660: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82F46664: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F46668: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4666C: 81090018  lwz r8, 0x18(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F46670: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F46674: 4E800421  bctrl
	ctx.lr = 0x82F46678;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46678: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F4667C: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82F46680: 4080FFAC  bge 0x82f4662c
	if !ctx.cr[0].lt {
	pc = 0x82F4662C; continue 'dispatch;
	}
	// 82F46684: 7D57C02E  lwzx r10, r23, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F46688: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4668C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46690: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F46694: 40980020  bge cr6, 0x82f466b4
	if !ctx.cr[6].lt {
	pc = 0x82F466B4; continue 'dispatch;
	}
	// 82F46698: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F4669C: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F466A0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F466A4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F466A8: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F466AC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F466B0: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F466B4: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82F466B8: 48261AEC  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F466C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F466C0 size=284
    let mut pc: u32 = 0x82F466C0;
    'dispatch: loop {
        match pc {
            0x82F466C0 => {
    //   block [0x82F466C0..0x82F467DC)
	// 82F466C0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F466C4: 48261A91  bl 0x831a8154
	ctx.lr = 0x82F466C8;
	sub_831A8130(ctx, base);
	// 82F466C8: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F466CC: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F466D0: 3AE00018  li r23, 0x18
	ctx.r[23].s64 = 24;
	// 82F466D4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F466D8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F466DC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F466E0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F466E4: 7D77C02E  lwzx r11, r23, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F466E8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F466EC: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F466F0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F466F4: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F466F8: 40980020  bge cr6, 0x82f46718
	if !ctx.cr[6].lt {
	pc = 0x82F46718; continue 'dispatch;
	}
	// 82F466FC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F46700: 3909FE68  addi r8, r9, -0x198
	ctx.r[8].s64 = ctx.r[9].s64 + -408;
	// 82F46704: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F46708: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F4670C: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F46710: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F46714: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F46718: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4671C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46720: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F46724: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46728: 4E800421  bctrl
	ctx.lr = 0x82F4672C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4672C: 811C0008  lwz r8, 8(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F46730: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F46734: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F46738: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4673C: 37C9FFFF  addic. r30, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F46740: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82F46744: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82F46748: 4180005C  blt 0x82f467a4
	if ctx.cr[0].lt {
	pc = 0x82F467A4; continue 'dispatch;
	}
	// 82F4674C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46750: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F46754: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46758: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F4675C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F46760: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F46764: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46768: 4E800421  bctrl
	ctx.lr = 0x82F4676C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4676C: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82F46770: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82F46774: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82F46778: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4677C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82F46780: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82F46784: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F46788: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4678C: 81090010  lwz r8, 0x10(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F46790: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F46794: 4E800421  bctrl
	ctx.lr = 0x82F46798;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46798: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F4679C: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82F467A0: 4080FFAC  bge 0x82f4674c
	if !ctx.cr[0].lt {
	pc = 0x82F4674C; continue 'dispatch;
	}
	// 82F467A4: 7D57C02E  lwzx r10, r23, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F467A8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F467AC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F467B0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F467B4: 40980020  bge cr6, 0x82f467d4
	if !ctx.cr[6].lt {
	pc = 0x82F467D4; continue 'dispatch;
	}
	// 82F467B8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F467BC: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F467C0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F467C4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F467C8: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F467CC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F467D0: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F467D4: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82F467D8: 482619CC  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F467E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F467E0 size=412
    let mut pc: u32 = 0x82F467E0;
    'dispatch: loop {
        match pc {
            0x82F467E0 => {
    //   block [0x82F467E0..0x82F4697C)
	// 82F467E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F467E4: 48261971  bl 0x831a8154
	ctx.lr = 0x82F467E8;
	sub_831A8130(ctx, base);
	// 82F467E8: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F467EC: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F467F0: 3AE00018  li r23, 0x18
	ctx.r[23].s64 = 24;
	// 82F467F4: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82F467F8: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F467FC: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F46800: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F46804: 7D77C02E  lwzx r11, r23, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F46808: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4680C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46810: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F46814: 40980020  bge cr6, 0x82f46834
	if !ctx.cr[6].lt {
	pc = 0x82F46834; continue 'dispatch;
	}
	// 82F46818: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4681C: 3909FE68  addi r8, r9, -0x198
	ctx.r[8].s64 = ctx.r[9].s64 + -408;
	// 82F46820: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F46824: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F46828: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F4682C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F46830: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F46834: 80790000  lwz r3, 0(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46838: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4683C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F46840: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46844: 4E800421  bctrl
	ctx.lr = 0x82F46848;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46848: 81390008  lwz r9, 8(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4684C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F46850: 9321006C  stw r25, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[25].u32 ) };
	// 82F46854: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46858: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82F4685C: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46860: 8368000C  lwz r27, 0xc(r8)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46864: 80C70008  lwz r6, 8(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F46868: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82F4686C: 4E800421  bctrl
	ctx.lr = 0x82F46870;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46870: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F46874: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82F46878: 419A00CC  beq cr6, 0x82f46944
	if ctx.cr[6].eq {
	pc = 0x82F46944; continue 'dispatch;
	}
	// 82F4687C: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F46880: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82F46884: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82F46888: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82F4688C: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F46890: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F46894: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46898: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F4689C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F468A0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F468A4: 4E800421  bctrl
	ctx.lr = 0x82F468A8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F468A8: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F468AC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F468B0: 419A0070  beq cr6, 0x82f46920
	if ctx.cr[6].eq {
	pc = 0x82F46920; continue 'dispatch;
	}
	// 82F468B4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F468B8: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F468BC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F468C0: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F468C4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F468C8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F468CC: 4E800421  bctrl
	ctx.lr = 0x82F468D0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F468D0: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82F468D4: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82F468D8: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82F468DC: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82F468E0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F468E4: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F468E8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F468EC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F468F0: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F468F4: 392A000D  addi r9, r10, 0xd
	ctx.r[9].s64 = ctx.r[10].s64 + 13;
	// 82F468F8: 552A2834  slwi r10, r9, 5
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F468FC: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F46900: 7D48D8AE  lbzx r10, r8, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82F46904: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82F46908: 7CEA4A14  add r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82F4690C: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F46910: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F46914: 814B09A8  lwz r10, 0x9a8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2472 as u32) ) } as u64;
	// 82F46918: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4691C: 4E800421  bctrl
	ctx.lr = 0x82F46920;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46920: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46924: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F46928: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4692C: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46930: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46934: 4E800421  bctrl
	ctx.lr = 0x82F46938;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46938: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F4693C: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82F46940: 409AFF3C  bne cr6, 0x82f4687c
	if !ctx.cr[6].eq {
	pc = 0x82F4687C; continue 'dispatch;
	}
	// 82F46944: 7D57C02E  lwzx r10, r23, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F46948: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4694C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46950: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F46954: 40980020  bge cr6, 0x82f46974
	if !ctx.cr[6].lt {
	pc = 0x82F46974; continue 'dispatch;
	}
	// 82F46958: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F4695C: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F46960: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F46964: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F46968: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F4696C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F46970: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F46974: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82F46978: 4826182C  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F46980(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F46980 size=292
    let mut pc: u32 = 0x82F46980;
    'dispatch: loop {
        match pc {
            0x82F46980 => {
    //   block [0x82F46980..0x82F46AA4)
	// 82F46980: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F46984: 482617CD  bl 0x831a8150
	ctx.lr = 0x82F46988;
	sub_831A8130(ctx, base);
	// 82F46988: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4698C: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46990: 3AC00018  li r22, 0x18
	ctx.r[22].s64 = 24;
	// 82F46994: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F46998: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F4699C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F469A0: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F469A4: 7D76B82E  lwzx r11, r22, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82F469A8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F469AC: 7D184378  mr r24, r8
	ctx.r[24].u64 = ctx.r[8].u64;
	// 82F469B0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F469B4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F469B8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F469BC: 40980020  bge cr6, 0x82f469dc
	if !ctx.cr[6].lt {
	pc = 0x82F469DC; continue 'dispatch;
	}
	// 82F469C0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F469C4: 3909FE68  addi r8, r9, -0x198
	ctx.r[8].s64 = ctx.r[9].s64 + -408;
	// 82F469C8: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F469CC: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F469D0: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F469D4: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F469D8: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F469DC: 807C0000  lwz r3, 0(r28)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F469E0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F469E4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F469E8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F469EC: 4E800421  bctrl
	ctx.lr = 0x82F469F0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F469F0: 811C0008  lwz r8, 8(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F469F4: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F469F8: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F469FC: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46A00: 37C9FFFF  addic. r30, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F46A04: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82F46A08: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82F46A0C: 41800060  blt 0x82f46a6c
	if ctx.cr[0].lt {
	pc = 0x82F46A6C; continue 'dispatch;
	}
	// 82F46A10: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46A14: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F46A18: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46A1C: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F46A20: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F46A24: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F46A28: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46A2C: 4E800421  bctrl
	ctx.lr = 0x82F46A30;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46A30: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82F46A34: 93810054  stw r28, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[28].u32 ) };
	// 82F46A38: 7F08C378  mr r8, r24
	ctx.r[8].u64 = ctx.r[24].u64;
	// 82F46A3C: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46A40: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82F46A44: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82F46A48: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82F46A4C: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F46A50: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46A54: 81690014  lwz r11, 0x14(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F46A58: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F46A5C: 4E800421  bctrl
	ctx.lr = 0x82F46A60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46A60: 37DEFFFF  addic. r30, r30, -1
	ctx.xer.ca = (ctx.r[30].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[30].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F46A64: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82F46A68: 4080FFA8  bge 0x82f46a10
	if !ctx.cr[0].lt {
	pc = 0x82F46A10; continue 'dispatch;
	}
	// 82F46A6C: 7D56B82E  lwzx r10, r22, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82F46A70: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46A74: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46A78: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F46A7C: 40980020  bge cr6, 0x82f46a9c
	if !ctx.cr[6].lt {
	pc = 0x82F46A9C; continue 'dispatch;
	}
	// 82F46A80: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F46A84: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F46A88: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F46A8C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F46A90: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F46A94: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F46A98: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F46A9C: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82F46AA0: 48261700  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F46AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F46AA8 size=420
    let mut pc: u32 = 0x82F46AA8;
    'dispatch: loop {
        match pc {
            0x82F46AA8 => {
    //   block [0x82F46AA8..0x82F46C4C)
	// 82F46AA8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F46AAC: 482616A5  bl 0x831a8150
	ctx.lr = 0x82F46AB0;
	sub_831A8130(ctx, base);
	// 82F46AB0: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F46AB4: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46AB8: 3AC00018  li r22, 0x18
	ctx.r[22].s64 = 24;
	// 82F46ABC: 7C781B78  mr r24, r3
	ctx.r[24].u64 = ctx.r[3].u64;
	// 82F46AC0: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F46AC4: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F46AC8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F46ACC: 7D76B82E  lwzx r11, r22, r23
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82F46AD0: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F46AD4: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46AD8: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46ADC: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F46AE0: 40980020  bge cr6, 0x82f46b00
	if !ctx.cr[6].lt {
	pc = 0x82F46B00; continue 'dispatch;
	}
	// 82F46AE4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F46AE8: 3909FE68  addi r8, r9, -0x198
	ctx.r[8].s64 = ctx.r[9].s64 + -408;
	// 82F46AEC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F46AF0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F46AF4: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F46AF8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F46AFC: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F46B00: 80780000  lwz r3, 0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46B04: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46B08: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F46B0C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46B10: 4E800421  bctrl
	ctx.lr = 0x82F46B14;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46B14: 81380008  lwz r9, 8(r24)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F46B18: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F46B1C: 9301006C  stw r24, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[24].u32 ) };
	// 82F46B20: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46B24: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82F46B28: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46B2C: 8368000C  lwz r27, 0xc(r8)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46B30: 80C70008  lwz r6, 8(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F46B34: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82F46B38: 4E800421  bctrl
	ctx.lr = 0x82F46B3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46B3C: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F46B40: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82F46B44: 419A00D0  beq cr6, 0x82f46c14
	if ctx.cr[6].eq {
	pc = 0x82F46C14; continue 'dispatch;
	}
	// 82F46B48: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F46B4C: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82F46B50: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82F46B54: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82F46B58: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F46B5C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F46B60: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46B64: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F46B68: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46B6C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46B70: 4E800421  bctrl
	ctx.lr = 0x82F46B74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46B74: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46B78: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F46B7C: 419A0074  beq cr6, 0x82f46bf0
	if ctx.cr[6].eq {
	pc = 0x82F46BF0; continue 'dispatch;
	}
	// 82F46B80: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46B84: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F46B88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F46B8C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F46B90: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F46B94: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46B98: 4E800421  bctrl
	ctx.lr = 0x82F46B9C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46B9C: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82F46BA0: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82F46BA4: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82F46BA8: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82F46BAC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82F46BB0: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46BB4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F46BB8: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F46BBC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F46BC0: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46BC4: 392A000D  addi r9, r10, 0xd
	ctx.r[9].s64 = ctx.r[10].s64 + 13;
	// 82F46BC8: 552A2834  slwi r10, r9, 5
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F46BCC: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F46BD0: 7D48D8AE  lbzx r10, r8, r27
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[27].u32)) } as u64;
	// 82F46BD4: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82F46BD8: 7D4A4A14  add r10, r10, r9
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82F46BDC: 554A103A  slwi r10, r10, 2
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F46BE0: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F46BE4: 810909AC  lwz r8, 0x9ac(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2476 as u32) ) } as u64;
	// 82F46BE8: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F46BEC: 4E800421  bctrl
	ctx.lr = 0x82F46BF0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46BF0: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46BF4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F46BF8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F46BFC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46C00: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46C04: 4E800421  bctrl
	ctx.lr = 0x82F46C08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46C08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F46C0C: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82F46C10: 409AFF38  bne cr6, 0x82f46b48
	if !ctx.cr[6].eq {
	pc = 0x82F46B48; continue 'dispatch;
	}
	// 82F46C14: 7D56B82E  lwzx r10, r22, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82F46C18: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46C1C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46C20: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F46C24: 40980020  bge cr6, 0x82f46c44
	if !ctx.cr[6].lt {
	pc = 0x82F46C44; continue 'dispatch;
	}
	// 82F46C28: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F46C2C: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F46C30: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F46C34: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F46C38: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F46C3C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F46C40: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F46C44: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82F46C48: 48261558  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F46C50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F46C50 size=296
    let mut pc: u32 = 0x82F46C50;
    'dispatch: loop {
        match pc {
            0x82F46C50 => {
    //   block [0x82F46C50..0x82F46D78)
	// 82F46C50: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F46C54: 48261501  bl 0x831a8154
	ctx.lr = 0x82F46C58;
	sub_831A8130(ctx, base);
	// 82F46C58: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F46C5C: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46C60: 3AE00018  li r23, 0x18
	ctx.r[23].s64 = 24;
	// 82F46C64: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F46C68: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F46C6C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82F46C70: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82F46C74: 7D77C02E  lwzx r11, r23, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F46C78: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82F46C7C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46C80: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46C84: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F46C88: 40980020  bge cr6, 0x82f46ca8
	if !ctx.cr[6].lt {
	pc = 0x82F46CA8; continue 'dispatch;
	}
	// 82F46C8C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F46C90: 3909FE68  addi r8, r9, -0x198
	ctx.r[8].s64 = ctx.r[9].s64 + -408;
	// 82F46C94: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F46C98: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F46C9C: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F46CA0: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F46CA4: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F46CA8: 807E0000  lwz r3, 0(r30)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46CAC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46CB0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F46CB4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46CB8: 4E800421  bctrl
	ctx.lr = 0x82F46CBC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46CBC: 811E0008  lwz r8, 8(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F46CC0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F46CC4: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F46CC8: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46CCC: 3769FFFF  addic. r27, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82F46CD0: 93C1005C  stw r30, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[30].u32 ) };
	// 82F46CD4: 91010058  stw r8, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[8].u32 ) };
	// 82F46CD8: 41800068  blt 0x82f46d40
	if ctx.cr[0].lt {
	pc = 0x82F46D40; continue 'dispatch;
	}
	// 82F46CDC: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46CE0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F46CE4: 83DF0000  lwz r30, 0(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46CE8: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F46CEC: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F46CF0: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F46CF4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46CF8: 4E800421  bctrl
	ctx.lr = 0x82F46CFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46CFC: 90610050  stw r3, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[3].u32 ) };
	// 82F46D00: 93C10054  stw r30, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[30].u32 ) };
	// 82F46D04: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F46D08: 807F0004  lwz r3, 4(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46D0C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82F46D10: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82F46D14: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F46D18: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46D1C: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46D20: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F46D24: 4E800421  bctrl
	ctx.lr = 0x82F46D28;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46D28: 88FC0004  lbz r7, 4(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[28].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46D2C: 2B070000  cmplwi cr6, r7, 0
	ctx.cr[6].compare_u32(ctx.r[7].u32, 0 as u32, &mut ctx.xer);
	// 82F46D30: 409A0010  bne cr6, 0x82f46d40
	if !ctx.cr[6].eq {
	pc = 0x82F46D40; continue 'dispatch;
	}
	// 82F46D34: 377BFFFF  addic. r27, r27, -1
	ctx.xer.ca = (ctx.r[27].u32 > (!(-1 as u32)));
	ctx.r[27].s64 = ctx.r[27].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[27].s32, 0, &mut ctx.xer);
	// 82F46D38: 3BFF0008  addi r31, r31, 8
	ctx.r[31].s64 = ctx.r[31].s64 + 8;
	// 82F46D3C: 4080FFA0  bge 0x82f46cdc
	if !ctx.cr[0].lt {
	pc = 0x82F46CDC; continue 'dispatch;
	}
	// 82F46D40: 7D57C02E  lwzx r10, r23, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F46D44: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46D48: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46D4C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F46D50: 40980020  bge cr6, 0x82f46d70
	if !ctx.cr[6].lt {
	pc = 0x82F46D70; continue 'dispatch;
	}
	// 82F46D54: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F46D58: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F46D5C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F46D60: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F46D64: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F46D68: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F46D6C: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F46D70: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82F46D74: 48261430  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F46D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F46D78 size=424
    let mut pc: u32 = 0x82F46D78;
    'dispatch: loop {
        match pc {
            0x82F46D78 => {
    //   block [0x82F46D78..0x82F46F20)
	// 82F46D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F46D7C: 482613D9  bl 0x831a8154
	ctx.lr = 0x82F46D80;
	sub_831A8130(ctx, base);
	// 82F46D80: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F46D84: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46D88: 3AE00018  li r23, 0x18
	ctx.r[23].s64 = 24;
	// 82F46D8C: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82F46D90: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F46D94: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F46D98: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F46D9C: 7D77C02E  lwzx r11, r23, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F46DA0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46DA4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46DA8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F46DAC: 40980020  bge cr6, 0x82f46dcc
	if !ctx.cr[6].lt {
	pc = 0x82F46DCC; continue 'dispatch;
	}
	// 82F46DB0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F46DB4: 3909FE68  addi r8, r9, -0x198
	ctx.r[8].s64 = ctx.r[9].s64 + -408;
	// 82F46DB8: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F46DBC: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F46DC0: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F46DC4: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F46DC8: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F46DCC: 80790000  lwz r3, 0(r25)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46DD0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46DD4: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F46DD8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46DDC: 4E800421  bctrl
	ctx.lr = 0x82F46DE0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46DE0: 81390008  lwz r9, 8(r25)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F46DE4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F46DE8: 9321006C  stw r25, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[25].u32 ) };
	// 82F46DEC: 811C0000  lwz r8, 0(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46DF0: 91210068  stw r9, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[9].u32 ) };
	// 82F46DF4: 80FF0000  lwz r7, 0(r31)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46DF8: 8348000C  lwz r26, 0xc(r8)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46DFC: 80C70008  lwz r6, 8(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F46E00: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82F46E04: 4E800421  bctrl
	ctx.lr = 0x82F46E08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46E08: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F46E0C: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82F46E10: 419A00D8  beq cr6, 0x82f46ee8
	if ctx.cr[6].eq {
	pc = 0x82F46EE8; continue 'dispatch;
	}
	// 82F46E14: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F46E18: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82F46E1C: 7FE8FB78  mr r8, r31
	ctx.r[8].u64 = ctx.r[31].u64;
	// 82F46E20: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82F46E24: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F46E28: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F46E2C: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46E30: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F46E34: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46E38: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46E3C: 4E800421  bctrl
	ctx.lr = 0x82F46E40;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46E40: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46E44: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F46E48: 419A007C  beq cr6, 0x82f46ec4
	if ctx.cr[6].eq {
	pc = 0x82F46EC4; continue 'dispatch;
	}
	// 82F46E4C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46E50: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F46E54: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F46E58: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F46E5C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F46E60: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46E64: 4E800421  bctrl
	ctx.lr = 0x82F46E68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46E68: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82F46E6C: 93C10064  stw r30, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[30].u32 ) };
	// 82F46E70: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F46E74: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82F46E78: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F46E7C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46E80: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F46E84: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F46E88: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46E8C: 392A000D  addi r9, r10, 0xd
	ctx.r[9].s64 = ctx.r[10].s64 + 13;
	// 82F46E90: 552A2834  slwi r10, r9, 5
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F46E94: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F46E98: 7D48D0AE  lbzx r10, r8, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[8].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F46E9C: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82F46EA0: 7CEA4A14  add r7, r10, r9
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82F46EA4: 54EA103A  slwi r10, r7, 2
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F46EA8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F46EAC: 814B09A4  lwz r10, 0x9a4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82F46EB0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46EB4: 4E800421  bctrl
	ctx.lr = 0x82F46EB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46EB8: 893B0004  lbz r9, 4(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46EBC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F46EC0: 409A0028  bne cr6, 0x82f46ee8
	if !ctx.cr[6].eq {
	pc = 0x82F46EE8; continue 'dispatch;
	}
	// 82F46EC4: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46EC8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F46ECC: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F46ED0: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46ED4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46ED8: 4E800421  bctrl
	ctx.lr = 0x82F46EDC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46EDC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F46EE0: 2F1EFFFF  cmpwi cr6, r30, -1
	ctx.cr[6].compare_i32(ctx.r[30].s32, -1, &mut ctx.xer);
	// 82F46EE4: 409AFF30  bne cr6, 0x82f46e14
	if !ctx.cr[6].eq {
	pc = 0x82F46E14; continue 'dispatch;
	}
	// 82F46EE8: 7D57C02E  lwzx r10, r23, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F46EEC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46EF0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46EF4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F46EF8: 40980020  bge cr6, 0x82f46f18
	if !ctx.cr[6].lt {
	pc = 0x82F46F18; continue 'dispatch;
	}
	// 82F46EFC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F46F00: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F46F04: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F46F08: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F46F0C: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F46F10: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F46F14: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F46F18: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82F46F1C: 48261288  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F46F20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F46F20 size=632
    let mut pc: u32 = 0x82F46F20;
    'dispatch: loop {
        match pc {
            0x82F46F20 => {
    //   block [0x82F46F20..0x82F47198)
	// 82F46F20: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F46F24: 4826122D  bl 0x831a8150
	ctx.lr = 0x82F46F28;
	sub_831A8130(ctx, base);
	// 82F46F28: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F46F2C: 7C972378  mr r23, r4
	ctx.r[23].u64 = ctx.r[4].u64;
	// 82F46F30: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F46F34: 92E1006C  stw r23, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[23].u32 ) };
	// 82F46F38: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F46F3C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F46F40: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82F46F44: 81770008  lwz r11, 8(r23)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F46F48: 80770000  lwz r3, 0(r23)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46F4C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82F46F50: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46F54: 812A0014  lwz r9, 0x14(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F46F58: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F46F5C: 4E800421  bctrl
	ctx.lr = 0x82F46F60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46F60: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82F46F64: 81190000  lwz r8, 0(r25)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46F68: 80E80004  lwz r7, 4(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46F6C: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F46F70: 4E800421  bctrl
	ctx.lr = 0x82F46F74;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46F74: 80D90000  lwz r6, 0(r25)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46F78: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F46F7C: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F46F80: 80A60008  lwz r5, 8(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F46F84: 7CA903A6  mtctr r5
	ctx.ctr.u64 = ctx.r[5].u64;
	// 82F46F88: 4E800421  bctrl
	ctx.lr = 0x82F46F8C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46F8C: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82F46F90: 2F1F0000  cmpwi cr6, r31, 0
	ctx.cr[6].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82F46F94: 409901FC  ble cr6, 0x82f47190
	if !ctx.cr[6].gt {
	pc = 0x82F47190; continue 'dispatch;
	}
	// 82F46F98: 7FF6FB78  mr r22, r31
	ctx.r[22].u64 = ctx.r[31].u64;
	// 82F46F9C: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F46FA0: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F46FA4: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F46FA8: 40990024  ble cr6, 0x82f46fcc
	if !ctx.cr[6].gt {
	pc = 0x82F46FCC; continue 'dispatch;
	}
	// 82F46FAC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F46FB0: 812B0000  lwz r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46FB4: 7F09D040  cmplw cr6, r9, r26
	ctx.cr[6].compare_u32(ctx.r[9].u32, ctx.r[26].u32, &mut ctx.xer);
	// 82F46FB8: 419A0018  beq cr6, 0x82f46fd0
	if ctx.cr[6].eq {
	pc = 0x82F46FD0; continue 'dispatch;
	}
	// 82F46FBC: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82F46FC0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F46FC4: 7F1F5000  cmpw cr6, r31, r10
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82F46FC8: 4198FFE8  blt cr6, 0x82f46fb0
	if ctx.cr[6].lt {
	pc = 0x82F46FB0; continue 'dispatch;
	}
	// 82F46FCC: 3BE0FFFF  li r31, -1
	ctx.r[31].s64 = -1;
	// 82F46FD0: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F46FD4: 7F49D378  mr r9, r26
	ctx.r[9].u64 = ctx.r[26].u64;
	// 82F46FD8: 7F28CB78  mr r8, r25
	ctx.r[8].u64 = ctx.r[25].u64;
	// 82F46FDC: 7EE7BB78  mr r7, r23
	ctx.r[7].u64 = ctx.r[23].u64;
	// 82F46FE0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F46FE4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F46FE8: 81640000  lwz r11, 0(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F46FEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F46FF0: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F46FF4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F46FF8: 4E800421  bctrl
	ctx.lr = 0x82F46FFC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F46FFC: 89230000  lbz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47000: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F47004: 419A010C  beq cr6, 0x82f47110
	if ctx.cr[6].eq {
	pc = 0x82F47110; continue 'dispatch;
	}
	// 82F47008: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4700C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F47010: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82F47014: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F47018: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F4701C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F47020: 4E800421  bctrl
	ctx.lr = 0x82F47024;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F47024: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82F47028: 93410064  stw r26, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[26].u32 ) };
	// 82F4702C: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82F47030: 409A00AC  bne cr6, 0x82f470dc
	if !ctx.cr[6].eq {
	pc = 0x82F470DC; continue 'dispatch;
	}
	// 82F47034: 817E0014  lwz r11, 0x14(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F47038: 3BFE000C  addi r31, r30, 0xc
	ctx.r[31].s64 = ctx.r[30].s64 + 12;
	// 82F4703C: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F47040: 556900BE  clrlwi r9, r11, 2
	ctx.r[9].u64 = ctx.r[11].u32 as u64 & 0x3FFFFFFFu64;
	// 82F47044: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F47048: 409A0010  bne cr6, 0x82f47058
	if !ctx.cr[6].eq {
	pc = 0x82F47058; continue 'dispatch;
	}
	// 82F4704C: 38800008  li r4, 8
	ctx.r[4].s64 = 8;
	// 82F47050: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F47054: 4BF5F82D  bl 0x82ea6880
	ctx.lr = 0x82F47058;
	sub_82EA6880(ctx, base);
	// 82F47058: 817F0004  lwz r11, 4(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4705C: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47060: 55691838  slwi r9, r11, 3
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F47064: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F47068: 7FA95214  add r29, r9, r10
	ctx.r[29].u64 = ctx.r[9].u64 + ctx.r[10].u64;
	// 82F4706C: 917F0004  stw r11, 4(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[11].u32 ) };
	// 82F47070: 7F49512E  stwx r26, r9, r10
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32), ctx.r[26].u32) };
	// 82F47074: 81010060  lwz r8, 0x60(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F47078: 813B0000  lwz r9, 0(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4707C: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47080: 80FC0010  lwz r7, 0x10(r28)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F47084: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82F47088: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4708C: 390A05A0  addi r8, r10, 0x5a0
	ctx.r[8].s64 = ctx.r[10].s64 + 1440;
	// 82F47090: 80DE0008  lwz r6, 8(r30)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F47094: 8129000C  lwz r9, 0xc(r9)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F47098: 409A0008  bne cr6, 0x82f470a0
	if !ctx.cr[6].eq {
	pc = 0x82F470A0; continue 'dispatch;
	}
	// 82F4709C: 390A01A0  addi r8, r10, 0x1a0
	ctx.r[8].s64 = ctx.r[10].s64 + 416;
	// 82F470A0: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F470A4: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F470A8: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82F470AC: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82F470B0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F470B4: 7D6B48AE  lbzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82F470B8: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F470BC: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F470C0: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F470C4: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F470C8: 80E809A0  lwz r7, 0x9a0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82F470CC: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F470D0: 4E800421  bctrl
	ctx.lr = 0x82F470D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F470D4: 907D0004  stw r3, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F470D8: 48000094  b 0x82f4716c
	pc = 0x82F4716C; continue 'dispatch;
	// 82F470DC: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F470E0: 57EA1838  slwi r10, r31, 3
	ctx.r[10].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F470E4: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82F470E8: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F470EC: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F470F0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82F470F4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F470F8: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F470FC: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47100: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F47104: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F47108: 4E800421  bctrl
	ctx.lr = 0x82F4710C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4710C: 48000060  b 0x82f4716c
	pc = 0x82F4716C; continue 'dispatch;
	// 82F47110: 2F1FFFFF  cmpwi cr6, r31, -1
	ctx.cr[6].compare_i32(ctx.r[31].s32, -1, &mut ctx.xer);
	// 82F47114: 419A0058  beq cr6, 0x82f4716c
	if ctx.cr[6].eq {
	pc = 0x82F4716C; continue 'dispatch;
	}
	// 82F47118: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4711C: 57FF1838  slwi r31, r31, 3
	ctx.r[31].u32 = ctx.r[31].u32.wrapping_shl(3);
	ctx.r[31].u64 = ctx.r[31].u32 as u64;
	// 82F47120: 3BBE000C  addi r29, r30, 0xc
	ctx.r[29].s64 = ctx.r[30].s64 + 12;
	// 82F47124: 7D7F5A14  add r11, r31, r11
	ctx.r[11].u64 = ctx.r[31].u64 + ctx.r[11].u64;
	// 82F47128: 7F04C378  mr r4, r24
	ctx.r[4].u64 = ctx.r[24].u64;
	// 82F4712C: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F47130: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47134: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F47138: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F4713C: 4E800421  bctrl
	ctx.lr = 0x82F47140;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F47140: 815E0010  lwz r10, 0x10(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F47144: 817E000C  lwz r11, 0xc(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F47148: 394AFFFF  addi r10, r10, -1
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	// 82F4714C: 7D0BFA14  add r8, r11, r31
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82F47150: 915E0010  stw r10, 0x10(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(16 as u32), ctx.r[10].u32 ) };
	// 82F47154: 554A1838  slwi r10, r10, 3
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(3);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F47158: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F4715C: 7CCA582E  lwzx r6, r10, r11
	ctx.r[6].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F47160: 7CCBF92E  stwx r6, r11, r31
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[6].u32) };
	// 82F47164: 80A70004  lwz r5, 4(r7)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F47168: 90A80004  stw r5, 4(r8)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[8].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F4716C: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47170: 7F44D378  mr r4, r26
	ctx.r[4].u64 = ctx.r[26].u64;
	// 82F47174: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F47178: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4717C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F47180: 4E800421  bctrl
	ctx.lr = 0x82F47184;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F47184: 36D6FFFF  addic. r22, r22, -1
	ctx.xer.ca = (ctx.r[22].u32 > (!(-1 as u32)));
	ctx.r[22].s64 = ctx.r[22].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82F47188: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82F4718C: 4082FE10  bne 0x82f46f9c
	if !ctx.cr[0].eq {
	pc = 0x82F46F9C; continue 'dispatch;
	}
	// 82F47190: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82F47194: 4826100C  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F47198(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F47198 size=496
    let mut pc: u32 = 0x82F47198;
    'dispatch: loop {
        match pc {
            0x82F47198 => {
    //   block [0x82F47198..0x82F47388)
	// 82F47198: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4719C: 48260FB5  bl 0x831a8150
	ctx.lr = 0x82F471A0;
	sub_831A8130(ctx, base);
	// 82F471A0: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F471A4: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82F471A8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F471AC: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82F471B0: 3BB7000C  addi r29, r23, 0xc
	ctx.r[29].s64 = ctx.r[23].s64 + 12;
	// 82F471B4: 7C982378  mr r24, r4
	ctx.r[24].u64 = ctx.r[4].u64;
	// 82F471B8: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F471BC: 392BFE80  addi r9, r11, -0x180
	ctx.r[9].s64 = ctx.r[11].s64 + -384;
	// 82F471C0: 9301006C  stw r24, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[24].u32 ) };
	// 82F471C4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F471C8: 93370008  stw r25, 8(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(8 as u32), ctx.r[25].u32 ) };
	// 82F471CC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F471D0: 91370000  stw r9, 0(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F471D4: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F471D8: 61450004  ori r5, r10, 4
	ctx.r[5].u64 = ctx.r[10].u64 | 4;
	// 82F471DC: 38FD000C  addi r7, r29, 0xc
	ctx.r[7].s64 = ctx.r[29].s64 + 12;
	// 82F471E0: B1170006  sth r8, 6(r23)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[23].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F471E4: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82F471E8: 90B70014  stw r5, 0x14(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82F471EC: 90F7000C  stw r7, 0xc(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82F471F0: 90D70010  stw r6, 0x10(r23)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[23].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 82F471F4: 80780000  lwz r3, 0(r24)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F471F8: 80980008  lwz r4, 8(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F471FC: 90810068  stw r4, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[4].u32 ) };
	// 82F47200: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47204: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F47208: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4720C: 4E800421  bctrl
	ctx.lr = 0x82F47210;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F47210: 7C7A1B78  mr r26, r3
	ctx.r[26].u64 = ctx.r[3].u64;
	// 82F47214: 813A0000  lwz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47218: 81090004  lwz r8, 4(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4721C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F47220: 4E800421  bctrl
	ctx.lr = 0x82F47224;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F47224: 80F70014  lwz r7, 0x14(r23)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F47228: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F4722C: 54EB00BE  clrlwi r11, r7, 2
	ctx.r[11].u64 = ctx.r[7].u32 as u64 & 0x3FFFFFFFu64;
	// 82F47230: 7F0BF000  cmpw cr6, r11, r30
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[30].s32, &mut ctx.xer);
	// 82F47234: 40980024  bge cr6, 0x82f47258
	if !ctx.cr[6].lt {
	pc = 0x82F47258; continue 'dispatch;
	}
	// 82F47238: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4723C: 7F1E5800  cmpw cr6, r30, r11
	ctx.cr[6].compare_i32(ctx.r[30].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F47240: 41980008  blt cr6, 0x82f47248
	if ctx.cr[6].lt {
	pc = 0x82F47248; continue 'dispatch;
	}
	// 82F47244: 7FCBF378  mr r11, r30
	ctx.r[11].u64 = ctx.r[30].u64;
	// 82F47248: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F4724C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F47250: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F47254: 4BF5F5A5  bl 0x82ea67f8
	ctx.lr = 0x82F47258;
	sub_82EA67F8(ctx, base);
	// 82F47258: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4725C: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82F47260: 814B0008  lwz r10, 8(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F47264: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F47268: 4E800421  bctrl
	ctx.lr = 0x82F4726C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4726C: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F47270: 2F1E0000  cmpwi cr6, r30, 0
	ctx.cr[6].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F47274: 40990108  ble cr6, 0x82f4737c
	if !ctx.cr[6].gt {
	pc = 0x82F4737C; continue 'dispatch;
	}
	// 82F47278: 7FD6F378  mr r22, r30
	ctx.r[22].u64 = ctx.r[30].u64;
	// 82F4727C: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47280: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F47284: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F47288: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82F4728C: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F47290: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F47294: 4E800421  bctrl
	ctx.lr = 0x82F47298;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F47298: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4729C: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82F472A0: 7F89E378  mr r9, r28
	ctx.r[9].u64 = ctx.r[28].u64;
	// 82F472A4: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82F472A8: 7F48D378  mr r8, r26
	ctx.r[8].u64 = ctx.r[26].u64;
	// 82F472AC: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82F472B0: 7F66DB78  mr r6, r27
	ctx.r[6].u64 = ctx.r[27].u64;
	// 82F472B4: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F472B8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F472BC: 81630004  lwz r11, 4(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F472C0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F472C4: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F472C8: 4E800421  bctrl
	ctx.lr = 0x82F472CC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F472CC: 89430000  lbz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F472D0: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F472D4: 419A0084  beq cr6, 0x82f47358
	if ctx.cr[6].eq {
	pc = 0x82F47358; continue 'dispatch;
	}
	// 82F472D8: 817D0004  lwz r11, 4(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F472DC: 815D0000  lwz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F472E0: 390B0001  addi r8, r11, 1
	ctx.r[8].s64 = ctx.r[11].s64 + 1;
	// 82F472E4: 81210060  lwz r9, 0x60(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F472E8: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F472EC: 911D0004  stw r8, 4(r29)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[29].u32.wrapping_add(4 as u32), ctx.r[8].u32 ) };
	// 82F472F0: 7FCB5214  add r30, r11, r10
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F472F4: 80FB0000  lwz r7, 0(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F472F8: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F472FC: 80DF0010  lwz r6, 0x10(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F47300: 2F060000  cmpwi cr6, r6, 0
	ctx.cr[6].compare_i32(ctx.r[6].s32, 0, &mut ctx.xer);
	// 82F47304: 8169000C  lwz r11, 0xc(r9)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F47308: 390A05A0  addi r8, r10, 0x5a0
	ctx.r[8].s64 = ctx.r[10].s64 + 1440;
	// 82F4730C: 8127000C  lwz r9, 0xc(r7)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F47310: 409A0008  bne cr6, 0x82f47318
	if !ctx.cr[6].eq {
	pc = 0x82F47318; continue 'dispatch;
	}
	// 82F47314: 390A01A0  addi r8, r10, 0x1a0
	ctx.r[8].s64 = ctx.r[10].s64 + 416;
	// 82F47318: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4731C: 7F26CB78  mr r6, r25
	ctx.r[6].u64 = ctx.r[25].u64;
	// 82F47320: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82F47324: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F47328: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82F4732C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F47330: 7D6B48AE  lbzx r11, r11, r9
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82F47334: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F47338: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F4733C: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F47340: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F47344: 80E809A0  lwz r7, 0x9a0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82F47348: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F4734C: 4E800421  bctrl
	ctx.lr = 0x82F47350;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F47350: 907E0004  stw r3, 4(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F47354: 939E0000  stw r28, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[28].u32 ) };
	// 82F47358: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4735C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F47360: 7F43D378  mr r3, r26
	ctx.r[3].u64 = ctx.r[26].u64;
	// 82F47364: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F47368: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4736C: 4E800421  bctrl
	ctx.lr = 0x82F47370;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F47370: 36D6FFFF  addic. r22, r22, -1
	ctx.xer.ca = (ctx.r[22].u32 > (!(-1 as u32)));
	ctx.r[22].s64 = ctx.r[22].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[22].s32, 0, &mut ctx.xer);
	// 82F47374: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F47378: 4082FF04  bne 0x82f4727c
	if !ctx.cr[0].eq {
	pc = 0x82F4727C; continue 'dispatch;
	}
	// 82F4737C: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82F47380: 382102D0  addi r1, r1, 0x2d0
	ctx.r[1].s64 = ctx.r[1].s64 + 720;
	// 82F47384: 48260E1C  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F47388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F47388 size=88
    let mut pc: u32 = 0x82F47388;
    'dispatch: loop {
        match pc {
            0x82F47388 => {
    //   block [0x82F47388..0x82F473E0)
	// 82F47388: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4738C: 48260DDD  bl 0x831a8168
	ctx.lr = 0x82F47390;
	sub_831A8130(ctx, base);
	// 82F47390: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F47394: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47398: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4739C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F473A0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F473A4: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F473A8: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82F473AC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F473B0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F473B4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F473B8: 4BF59379  bl 0x82ea0730
	ctx.lr = 0x82F473BC;
	sub_82EA0730(ctx, base);
	// 82F473BC: 39200038  li r9, 0x38
	ctx.r[9].s64 = 56;
	// 82F473C0: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F473C4: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F473C8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F473CC: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F473D0: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F473D4: 4BFFFDC5  bl 0x82f47198
	ctx.lr = 0x82F473D8;
	sub_82F47198(ctx, base);
	// 82F473D8: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F473DC: 48260DDC  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F473E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F473E0 size=108
    let mut pc: u32 = 0x82F473E0;
    'dispatch: loop {
        match pc {
            0x82F473E0 => {
    //   block [0x82F473E0..0x82F4744C)
	// 82F473E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F473E4: 48260D81  bl 0x831a8164
	ctx.lr = 0x82F473E8;
	sub_831A8130(ctx, base);
	// 82F473E8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F473EC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F473F0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F473F4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F473F8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F473FC: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F47400: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82F47404: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F47408: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4740C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F47410: 4BF59321  bl 0x82ea0730
	ctx.lr = 0x82F47414;
	sub_82EA0730(ctx, base);
	// 82F47414: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F47418: 39200038  li r9, 0x38
	ctx.r[9].s64 = 56;
	// 82F4741C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F47420: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F47424: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F47428: B13B0004  sth r9, 4(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F4742C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F47430: 4BFFFD69  bl 0x82f47198
	ctx.lr = 0x82F47434;
	sub_82F47198(ctx, base);
	// 82F47434: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F47438: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F4743C: 38E8F4C0  addi r7, r8, -0xb40
	ctx.r[7].s64 = ctx.r[8].s64 + -2880;
	// 82F47440: 90FB0000  stw r7, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F47444: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F47448: 48260D6C  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F47450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F47450 size=204
    let mut pc: u32 = 0x82F47450;
    'dispatch: loop {
        match pc {
            0x82F47450 => {
    //   block [0x82F47450..0x82F4751C)
	// 82F47450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F47454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F47458: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4745C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F47460: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F47464: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F47468: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F4746C: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F47470: 3D0082F4  lis r8, -0x7d0c
	ctx.r[8].s64 = -2097938432;
	// 82F47474: 38CAA9C0  addi r6, r10, -0x5640
	ctx.r[6].s64 = ctx.r[10].s64 + -22080;
	// 82F47478: 38A9AA08  addi r5, r9, -0x55f8
	ctx.r[5].s64 = ctx.r[9].s64 + -22008;
	// 82F4747C: 3888AA58  addi r4, r8, -0x55a8
	ctx.r[4].s64 = ctx.r[8].s64 + -21928;
	// 82F47480: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F47484: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82F47488: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F4748C: 38EB73E0  addi r7, r11, 0x73e0
	ctx.r[7].s64 = ctx.r[11].s64 + 29664;
	// 82F47490: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F47494: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 82F47498: 38C00008  li r6, 8
	ctx.r[6].s64 = 8;
	// 82F4749C: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F474A0: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F474A4: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F474A8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F474AC: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F474B0: 4BFDDF91  bl 0x82f25440
	ctx.lr = 0x82F474B4;
	sub_82F25440(ctx, base);
	// 82F474B4: 3C6082F4  lis r3, -0x7d0c
	ctx.r[3].s64 = -2097938432;
	// 82F474B8: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F474BC: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F474C0: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F474C4: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F474C8: 39037388  addi r8, r3, 0x7388
	ctx.r[8].s64 = ctx.r[3].s64 + 29576;
	// 82F474CC: 38CA67E0  addi r6, r10, 0x67e0
	ctx.r[6].s64 = ctx.r[10].s64 + 26592;
	// 82F474D0: 38A96AA8  addi r5, r9, 0x6aa8
	ctx.r[5].s64 = ctx.r[9].s64 + 27304;
	// 82F474D4: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82F474D8: 38EB6D78  addi r7, r11, 0x6d78
	ctx.r[7].s64 = ctx.r[11].s64 + 28024;
	// 82F474DC: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82F474E0: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F474E4: 90A1007C  stw r5, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 82F474E8: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82F474EC: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82F474F0: 98810080  stb r4, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u8 ) };
	// 82F474F4: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F474F8: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F474FC: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F47500: 4BFDDF41  bl 0x82f25440
	ctx.lr = 0x82F47504;
	sub_82F25440(ctx, base);
	// 82F47504: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F47508: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4750C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F47510: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F47514: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F47518: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F47520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F47520 size=124
    let mut pc: u32 = 0x82F47520;
    'dispatch: loop {
        match pc {
            0x82F47520 => {
    //   block [0x82F47520..0x82F4759C)
	// 82F47520: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F47524: 48260C45  bl 0x831a8168
	ctx.lr = 0x82F47528;
	sub_831A8130(ctx, base);
	// 82F47528: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4752C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F47530: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F47534: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F47538: 817D0010  lwz r11, 0x10(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4753C: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F47540: 4099003C  ble cr6, 0x82f4757c
	if !ctx.cr[6].gt {
	pc = 0x82F4757C; continue 'dispatch;
	}
	// 82F47544: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F47548: 817D000C  lwz r11, 0xc(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4754C: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F47550: 7D6BF214  add r11, r11, r30
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82F47554: 806B0004  lwz r3, 4(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F47558: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4755C: 812A001C  lwz r9, 0x1c(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F47560: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F47564: 4E800421  bctrl
	ctx.lr = 0x82F47568;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F47568: 811D0010  lwz r8, 0x10(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4756C: 3BFF0001  addi r31, r31, 1
	ctx.r[31].s64 = ctx.r[31].s64 + 1;
	// 82F47570: 3BDE0008  addi r30, r30, 8
	ctx.r[30].s64 = ctx.r[30].s64 + 8;
	// 82F47574: 7F1F4000  cmpw cr6, r31, r8
	ctx.cr[6].compare_i32(ctx.r[31].s32, ctx.r[8].s32, &mut ctx.xer);
	// 82F47578: 4198FFD0  blt cr6, 0x82f47548
	if ctx.cr[6].lt {
	pc = 0x82F47548; continue 'dispatch;
	}
	// 82F4757C: 817D0000  lwz r11, 0(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47580: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F47584: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F47588: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4758C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F47590: 4E800421  bctrl
	ctx.lr = 0x82F47594;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F47594: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F47598: 48260C20  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F475A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F475A0 size=300
    let mut pc: u32 = 0x82F475A0;
    'dispatch: loop {
        match pc {
            0x82F475A0 => {
    //   block [0x82F475A0..0x82F476CC)
	// 82F475A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F475A4: 48260BA9  bl 0x831a814c
	ctx.lr = 0x82F475A8;
	sub_831A8130(ctx, base);
	// 82F475A8: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F475AC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F475B0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F475B4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82F475B8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F475BC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F475C0: 833D0000  lwz r25, 0(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F475C4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82F475C8: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F475CC: 4BFFE8A5  bl 0x82f45e70
	ctx.lr = 0x82F475D0;
	sub_82F45E70(ctx, base);
	// 82F475D0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F475D4: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F475D8: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F475DC: 4BFD2C15  bl 0x82f1a1f0
	ctx.lr = 0x82F475E0;
	sub_82F1A1F0(ctx, base);
	// 82F475E0: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F475E4: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82F475E8: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82F475EC: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82F475F0: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F475F4: 37E9FFFF  addic. r31, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82F475F8: 418000CC  blt 0x82f476c4
	if ctx.cr[0].lt {
	pc = 0x82F476C4; continue 'dispatch;
	}
	// 82F475FC: 3AC00030  li r22, 0x30
	ctx.r[22].s64 = 48;
	// 82F47600: 3AE00040  li r23, 0x40
	ctx.r[23].s64 = 64;
	// 82F47604: 3B000050  li r24, 0x50
	ctx.r[24].s64 = 80;
	// 82F47608: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4760C: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82F47610: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 82F47614: 811D0008  lwz r8, 8(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F47618: 38EB0002  addi r7, r11, 2
	ctx.r[7].s64 = ctx.r[11].s64 + 2;
	// 82F4761C: 38C100A0  addi r6, r1, 0xa0
	ctx.r[6].s64 = ctx.r[1].s64 + 160;
	// 82F47620: 54EB2036  slwi r11, r7, 4
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F476D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F476D0 size=260
    let mut pc: u32 = 0x82F476D0;
    'dispatch: loop {
        match pc {
            0x82F476D0 => {
    //   block [0x82F476D0..0x82F477D4)
	// 82F476D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F476D4: 48260A85  bl 0x831a8158
	ctx.lr = 0x82F476D8;
	sub_831A8130(ctx, base);
	// 82F476D8: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F476DC: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F476E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F476E4: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82F476E8: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F476EC: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F476F0: 833D0000  lwz r25, 0(r29)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F476F4: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82F476F8: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F476FC: 4BFFE775  bl 0x82f45e70
	ctx.lr = 0x82F47700;
	sub_82F45E70(ctx, base);
	// 82F47700: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F47704: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F47708: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F4770C: 4BFD2AE5  bl 0x82f1a1f0
	ctx.lr = 0x82F47710;
	sub_82F1A1F0(ctx, base);
	// 82F47710: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F47714: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82F47718: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82F4771C: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82F47720: 83DF000C  lwz r30, 0xc(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F47724: 37E9FFFF  addic. r31, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82F47728: 418000A4  blt 0x82f477cc
	if ctx.cr[0].lt {
	pc = 0x82F477CC; continue 'dispatch;
	}
	// 82F4772C: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 82F47730: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47734: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82F47738: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 82F4773C: 811D0008  lwz r8, 8(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F47740: 38EB0002  addi r7, r11, 2
	ctx.r[7].s64 = ctx.r[11].s64 + 2;
	// 82F47744: 38C100A0  addi r6, r1, 0xa0
	ctx.r[6].s64 = ctx.r[1].s64 + 160;
	// 82F47748: 54EB2036  slwi r11, r7, 4
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F477D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F477D8 size=292
    let mut pc: u32 = 0x82F477D8;
    'dispatch: loop {
        match pc {
            0x82F477D8 => {
    //   block [0x82F477D8..0x82F478FC)
	// 82F477D8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F477DC: 48260975  bl 0x831a8150
	ctx.lr = 0x82F477E0;
	sub_831A8130(ctx, base);
	// 82F477E0: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F477E4: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F477E8: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82F477EC: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82F477F0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F477F4: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82F477F8: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F477FC: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F47800: 4BFFE671  bl 0x82f45e70
	ctx.lr = 0x82F47804;
	sub_82F45E70(ctx, base);
	// 82F47804: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F47808: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F4780C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F47810: 4BFD29E1  bl 0x82f1a1f0
	ctx.lr = 0x82F47814;
	sub_82F1A1F0(ctx, base);
	// 82F47814: 813A0000  lwz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47818: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82F4781C: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82F47820: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F47824: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82F47828: 811B0010  lwz r8, 0x10(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4782C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F47830: 8309000C  lwz r24, 0xc(r9)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F47834: 409900C0  ble cr6, 0x82f478f4
	if !ctx.cr[6].gt {
	pc = 0x82F478F4; continue 'dispatch;
	}
	// 82F47838: 3BDB0020  addi r30, r27, 0x20
	ctx.r[30].s64 = ctx.r[27].s64 + 32;
	// 82F4783C: 3AE00030  li r23, 0x30
	ctx.r[23].s64 = 48;
	// 82F47840: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F47900(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F47900 size=264
    let mut pc: u32 = 0x82F47900;
    'dispatch: loop {
        match pc {
            0x82F47900 => {
    //   block [0x82F47900..0x82F47A08)
	// 82F47900: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F47904: 48260851  bl 0x831a8154
	ctx.lr = 0x82F47908;
	sub_831A8130(ctx, base);
	// 82F47908: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4790C: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F47910: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F47914: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82F47918: 7CBC2B78  mr r28, r5
	ctx.r[28].u64 = ctx.r[5].u64;
	// 82F4791C: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F47920: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47924: 7CFA3B78  mr r26, r7
	ctx.r[26].u64 = ctx.r[7].u64;
	// 82F47928: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4792C: 7D194378  mr r25, r8
	ctx.r[25].u64 = ctx.r[8].u64;
	// 82F47930: 4BFFE541  bl 0x82f45e70
	ctx.lr = 0x82F47934;
	sub_82F45E70(ctx, base);
	// 82F47934: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F47938: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F4793C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F47940: 4BFD28B1  bl 0x82f1a1f0
	ctx.lr = 0x82F47944;
	sub_82F1A1F0(ctx, base);
	// 82F47944: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F47948: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82F4794C: 93A1005C  stw r29, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[29].u32 ) };
	// 82F47950: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82F47954: 831F000C  lwz r24, 0xc(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F47958: 37E9FFFF  addic. r31, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[31].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[31].s32, 0, &mut ctx.xer);
	// 82F4795C: 418000A4  blt 0x82f47a00
	if ctx.cr[0].lt {
	pc = 0x82F47A00; continue 'dispatch;
	}
	// 82F47960: 397F0002  addi r11, r31, 2
	ctx.r[11].s64 = ctx.r[31].s64 + 2;
	// 82F47964: 3AE00030  li r23, 0x30
	ctx.r[23].s64 = 48;
	// 82F47968: 556B2036  slwi r11, r11, 4
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4796C: 7FCBF214  add r30, r11, r30
	ctx.r[30].u64 = ctx.r[11].u64 + ctx.r[30].u64;
	// 82F47970: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F47A08(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F47A08 size=300
    let mut pc: u32 = 0x82F47A08;
    'dispatch: loop {
        match pc {
            0x82F47A08 => {
    //   block [0x82F47A08..0x82F47B34)
	// 82F47A08: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F47A0C: 48260741  bl 0x831a814c
	ctx.lr = 0x82F47A10;
	sub_831A8130(ctx, base);
	// 82F47A10: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F47A14: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F47A18: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82F47A1C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82F47A20: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F47A24: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82F47A28: 837C0000  lwz r27, 0(r28)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47A2C: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82F47A30: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F47A34: 4BFFE43D  bl 0x82f45e70
	ctx.lr = 0x82F47A38;
	sub_82F45E70(ctx, base);
	// 82F47A38: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F47A3C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F47A40: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F47A44: 4BFD27AD  bl 0x82f1a1f0
	ctx.lr = 0x82F47A48;
	sub_82F1A1F0(ctx, base);
	// 82F47A48: 813A0000  lwz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47A4C: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82F47A50: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82F47A54: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F47A58: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82F47A5C: 811B0010  lwz r8, 0x10(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F47A60: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F47A64: 82E9000C  lwz r23, 0xc(r9)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F47A68: 409900C4  ble cr6, 0x82f47b2c
	if !ctx.cr[6].gt {
	pc = 0x82F47B2C; continue 'dispatch;
	}
	// 82F47A6C: 3BDB0020  addi r30, r27, 0x20
	ctx.r[30].s64 = ctx.r[27].s64 + 32;
	// 82F47A70: 3AC00030  li r22, 0x30
	ctx.r[22].s64 = 48;
	// 82F47A74: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F47B38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F47B38 size=272
    let mut pc: u32 = 0x82F47B38;
    'dispatch: loop {
        match pc {
            0x82F47B38 => {
    //   block [0x82F47B38..0x82F47C48)
	// 82F47B38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F47B3C: 4826061D  bl 0x831a8158
	ctx.lr = 0x82F47B40;
	sub_831A8130(ctx, base);
	// 82F47B40: 9421FE80  stwu r1, -0x180(r1)
	ea = ctx.r[1].u32.wrapping_add(-384 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F47B44: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F47B48: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F47B4C: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82F47B50: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F47B54: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F47B58: 833C0000  lwz r25, 0(r28)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47B5C: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82F47B60: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F47B64: 4BFFE30D  bl 0x82f45e70
	ctx.lr = 0x82F47B68;
	sub_82F45E70(ctx, base);
	// 82F47B68: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F47B6C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F47B70: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F47B74: 4BFD267D  bl 0x82f1a1f0
	ctx.lr = 0x82F47B78;
	sub_82F1A1F0(ctx, base);
	// 82F47B78: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F47B7C: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82F47B80: 9381005C  stw r28, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[28].u32 ) };
	// 82F47B84: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82F47B88: 37C9FFFF  addic. r30, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[30].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[30].s32, 0, &mut ctx.xer);
	// 82F47B8C: 83FF000C  lwz r31, 0xc(r31)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F47B90: 418000B0  blt 0x82f47c40
	if ctx.cr[0].lt {
	pc = 0x82F47C40; continue 'dispatch;
	}
	// 82F47B94: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 82F47B98: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47B9C: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82F47BA0: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 82F47BA4: 811C0008  lwz r8, 8(r28)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F47BA8: 38EB0002  addi r7, r11, 2
	ctx.r[7].s64 = ctx.r[11].s64 + 2;
	// 82F47BAC: 38C100A0  addi r6, r1, 0xa0
	ctx.r[6].s64 = ctx.r[1].s64 + 160;
	// 82F47BB0: 54EB2036  slwi r11, r7, 4
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(4);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F47C48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F47C48 size=304
    let mut pc: u32 = 0x82F47C48;
    'dispatch: loop {
        match pc {
            0x82F47C48 => {
    //   block [0x82F47C48..0x82F47D78)
	// 82F47C48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F47C4C: 48260505  bl 0x831a8150
	ctx.lr = 0x82F47C50;
	sub_831A8130(ctx, base);
	// 82F47C50: 9421FE70  stwu r1, -0x190(r1)
	ea = ctx.r[1].u32.wrapping_add(-400 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F47C54: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F47C58: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82F47C5C: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82F47C60: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F47C64: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F47C68: 82FB0000  lwz r23, 0(r27)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47C6C: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F47C70: 4BFFE201  bl 0x82f45e70
	ctx.lr = 0x82F47C74;
	sub_82F45E70(ctx, base);
	// 82F47C74: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F47C78: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F47C7C: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F47C80: 4BFD2571  bl 0x82f1a1f0
	ctx.lr = 0x82F47C84;
	sub_82F1A1F0(ctx, base);
	// 82F47C84: 813A0000  lwz r9, 0(r26)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47C88: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82F47C8C: 9361005C  stw r27, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[27].u32 ) };
	// 82F47C90: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82F47C94: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82F47C98: 81170010  lwz r8, 0x10(r23)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[23].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F47C9C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F47CA0: 8329000C  lwz r25, 0xc(r9)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F47CA4: 409900CC  ble cr6, 0x82f47d70
	if !ctx.cr[6].gt {
	pc = 0x82F47D70; continue 'dispatch;
	}
	// 82F47CA8: 3BF70020  addi r31, r23, 0x20
	ctx.r[31].s64 = ctx.r[23].s64 + 32;
	// 82F47CAC: 3B000030  li r24, 0x30
	ctx.r[24].s64 = 48;
	// 82F47CB0: 39610080  addi r11, r1, 0x80
	ctx.r[11].s64 = ctx.r[1].s64 + 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F47D78(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F47D78 size=488
    let mut pc: u32 = 0x82F47D78;
    'dispatch: loop {
        match pc {
            0x82F47D78 => {
    //   block [0x82F47D78..0x82F47F60)
	// 82F47D78: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F47D7C: 482603C9  bl 0x831a8144
	ctx.lr = 0x82F47D80;
	sub_831A8130(ctx, base);
	// 82F47D80: 9421FE60  stwu r1, -0x1a0(r1)
	ea = ctx.r[1].u32.wrapping_add(-416 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F47D84: 7C731B78  mr r19, r3
	ctx.r[19].u64 = ctx.r[3].u64;
	// 82F47D88: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F47D8C: 3BB3000C  addi r29, r19, 0xc
	ctx.r[29].s64 = ctx.r[19].s64 + 12;
	// 82F47D90: 3D408000  lis r10, -0x8000
	ctx.r[10].s64 = -2147483648;
	// 82F47D94: 7C992378  mr r25, r4
	ctx.r[25].u64 = ctx.r[4].u64;
	// 82F47D98: 7CF43B78  mr r20, r7
	ctx.r[20].u64 = ctx.r[7].u64;
	// 82F47D9C: 392BFEBC  addi r9, r11, -0x144
	ctx.r[9].s64 = ctx.r[11].s64 + -324;
	// 82F47DA0: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82F47DA4: 92930008  stw r20, 8(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(8 as u32), ctx.r[20].u32 ) };
	// 82F47DA8: 7CDB3378  mr r27, r6
	ctx.r[27].u64 = ctx.r[6].u64;
	// 82F47DAC: 91330000  stw r9, 0(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F47DB0: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F47DB4: 61460004  ori r6, r10, 4
	ctx.r[6].u64 = ctx.r[10].u64 | 4;
	// 82F47DB8: 38FD000C  addi r7, r29, 0xc
	ctx.r[7].s64 = ctx.r[29].s64 + 12;
	// 82F47DBC: B1130006  sth r8, 6(r19)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[19].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F47DC0: 38A00000  li r5, 0
	ctx.r[5].s64 = 0;
	// 82F47DC4: 90D30014  stw r6, 0x14(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(20 as u32), ctx.r[6].u32 ) };
	// 82F47DC8: 90F3000C  stw r7, 0xc(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(12 as u32), ctx.r[7].u32 ) };
	// 82F47DCC: 54C4003E  slwi r4, r6, 0
	ctx.r[4].u32 = ctx.r[6].u32.wrapping_shl(0);
	ctx.r[4].u64 = ctx.r[4].u32 as u64;
	// 82F47DD0: 90B30010  stw r5, 0x10(r19)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[19].u32.wrapping_add(16 as u32), ctx.r[5].u32 ) };
	// 82F47DD4: 83F90000  lwz r31, 0(r25)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47DD8: 548B00BE  clrlwi r11, r4, 2
	ctx.r[11].u64 = ctx.r[4].u32 as u64 & 0x3FFFFFFFu64;
	// 82F47DDC: 831F0010  lwz r24, 0x10(r31)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F47DE0: 7F0BC000  cmpw cr6, r11, r24
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[24].s32, &mut ctx.xer);
	// 82F47DE4: 40980024  bge cr6, 0x82f47e08
	if !ctx.cr[6].lt {
	pc = 0x82F47E08; continue 'dispatch;
	}
	// 82F47DE8: 556B083C  slwi r11, r11, 1
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F47DEC: 7F185800  cmpw cr6, r24, r11
	ctx.cr[6].compare_i32(ctx.r[24].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F47DF0: 41980008  blt cr6, 0x82f47df8
	if ctx.cr[6].lt {
	pc = 0x82F47DF8; continue 'dispatch;
	}
	// 82F47DF4: 7F0BC378  mr r11, r24
	ctx.r[11].u64 = ctx.r[24].u64;
	// 82F47DF8: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F47DFC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F47E00: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F47E04: 4BF5E9F5  bl 0x82ea67f8
	ctx.lr = 0x82F47E08;
	sub_82EA67F8(ctx, base);
	// 82F47E08: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F47E0C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F47E10: C02B08A4  lfs f1, 0x8a4(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[1].f64 = (tmp.f32 as f64);
	// 82F47E14: 4BFD23DD  bl 0x82f1a1f0
	ctx.lr = 0x82F47E18;
	sub_82F1A1F0(ctx, base);
	// 82F47E18: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82F47E1C: 80990008  lwz r4, 8(r25)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F47E20: 4BFFE051  bl 0x82f45e70
	ctx.lr = 0x82F47E24;
	sub_82F45E70(ctx, base);
	// 82F47E24: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
	// 82F47E28: 3BDF0020  addi r30, r31, 0x20
	ctx.r[30].s64 = ctx.r[31].s64 + 32;
	// 82F47E2C: 9321005C  stw r25, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[25].u32 ) };
	// 82F47E30: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82F47E34: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F47E38: 2F180000  cmpwi cr6, r24, 0
	ctx.cr[6].compare_i32(ctx.r[24].s32, 0, &mut ctx.xer);
	// 82F47E3C: 40990118  ble cr6, 0x82f47f54
	if !ctx.cr[6].gt {
	pc = 0x82F47F54; continue 'dispatch;
	}
	// 82F47E40: 3AA00030  li r21, 0x30
	ctx.r[21].s64 = 48;
	// 82F47E44: 3AC00040  li r22, 0x40
	ctx.r[22].s64 = 64;
	// 82F47E48: 3AE00050  li r23, 0x50
	ctx.r[23].s64 = 80;
	// 82F47E4C: 39410080  addi r10, r1, 0x80
	ctx.r[10].s64 = ctx.r[1].s64 + 128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F47F60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F47F60 size=88
    let mut pc: u32 = 0x82F47F60;
    'dispatch: loop {
        match pc {
            0x82F47F60 => {
    //   block [0x82F47F60..0x82F47FB8)
	// 82F47F60: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F47F64: 48260205  bl 0x831a8168
	ctx.lr = 0x82F47F68;
	sub_831A8130(ctx, base);
	// 82F47F68: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F47F6C: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47F70: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F47F74: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F47F78: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F47F7C: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F47F80: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82F47F84: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F47F88: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F47F8C: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F47F90: 4BF587A1  bl 0x82ea0730
	ctx.lr = 0x82F47F94;
	sub_82EA0730(ctx, base);
	// 82F47F94: 39200038  li r9, 0x38
	ctx.r[9].s64 = 56;
	// 82F47F98: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F47F9C: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F47FA0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F47FA4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F47FA8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F47FAC: 4BFFFDCD  bl 0x82f47d78
	ctx.lr = 0x82F47FB0;
	sub_82F47D78(ctx, base);
	// 82F47FB0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F47FB4: 48260204  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F47FB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F47FB8 size=108
    let mut pc: u32 = 0x82F47FB8;
    'dispatch: loop {
        match pc {
            0x82F47FB8 => {
    //   block [0x82F47FB8..0x82F48024)
	// 82F47FB8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F47FBC: 482601A9  bl 0x831a8164
	ctx.lr = 0x82F47FC0;
	sub_831A8130(ctx, base);
	// 82F47FC0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F47FC4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F47FC8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F47FCC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F47FD0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F47FD4: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F47FD8: 38800038  li r4, 0x38
	ctx.r[4].s64 = 56;
	// 82F47FDC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F47FE0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F47FE4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F47FE8: 4BF58749  bl 0x82ea0730
	ctx.lr = 0x82F47FEC;
	sub_82EA0730(ctx, base);
	// 82F47FEC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F47FF0: 39200038  li r9, 0x38
	ctx.r[9].s64 = 56;
	// 82F47FF4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F47FF8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F47FFC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F48000: B13B0004  sth r9, 4(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F48004: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F48008: 4BFFFD71  bl 0x82f47d78
	ctx.lr = 0x82F4800C;
	sub_82F47D78(ctx, base);
	// 82F4800C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F48010: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F48014: 38E8FEF8  addi r7, r8, -0x108
	ctx.r[7].s64 = ctx.r[8].s64 + -264;
	// 82F48018: 90FB0000  stw r7, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F4801C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F48020: 48260194  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48028(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F48028 size=204
    let mut pc: u32 = 0x82F48028;
    'dispatch: loop {
        match pc {
            0x82F48028 => {
    //   block [0x82F48028..0x82F480F4)
	// 82F48028: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4802C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F48030: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F48034: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F48038: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4803C: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F48040: 3D4082F5  lis r10, -0x7d0b
	ctx.r[10].s64 = -2097872896;
	// 82F48044: 3D2082F5  lis r9, -0x7d0b
	ctx.r[9].s64 = -2097872896;
	// 82F48048: 3D0082F5  lis r8, -0x7d0b
	ctx.r[8].s64 = -2097872896;
	// 82F4804C: 38CA80F8  addi r6, r10, -0x7f08
	ctx.r[6].s64 = ctx.r[10].s64 + -32520;
	// 82F48050: 38A98140  addi r5, r9, -0x7ec0
	ctx.r[5].s64 = ctx.r[9].s64 + -32448;
	// 82F48054: 38888190  addi r4, r8, -0x7e70
	ctx.r[4].s64 = ctx.r[8].s64 + -32368;
	// 82F48058: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F4805C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82F48060: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F48064: 38EB7FB8  addi r7, r11, 0x7fb8
	ctx.r[7].s64 = ctx.r[11].s64 + 32696;
	// 82F48068: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F4806C: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 82F48070: 38C00014  li r6, 0x14
	ctx.r[6].s64 = 20;
	// 82F48074: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F48078: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F4807C: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F48080: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F48084: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F48088: 4BFDD3B9  bl 0x82f25440
	ctx.lr = 0x82F4808C;
	sub_82F25440(ctx, base);
	// 82F4808C: 3C6082F4  lis r3, -0x7d0c
	ctx.r[3].s64 = -2097938432;
	// 82F48090: 3D6082F4  lis r11, -0x7d0c
	ctx.r[11].s64 = -2097938432;
	// 82F48094: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F48098: 3D4082F4  lis r10, -0x7d0c
	ctx.r[10].s64 = -2097938432;
	// 82F4809C: 3D2082F4  lis r9, -0x7d0c
	ctx.r[9].s64 = -2097938432;
	// 82F480A0: 39037F60  addi r8, r3, 0x7f60
	ctx.r[8].s64 = ctx.r[3].s64 + 32608;
	// 82F480A4: 38CA77D8  addi r6, r10, 0x77d8
	ctx.r[6].s64 = ctx.r[10].s64 + 30680;
	// 82F480A8: 38A97A08  addi r5, r9, 0x7a08
	ctx.r[5].s64 = ctx.r[9].s64 + 31240;
	// 82F480AC: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82F480B0: 38EB7C48  addi r7, r11, 0x7c48
	ctx.r[7].s64 = ctx.r[11].s64 + 31816;
	// 82F480B4: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82F480B8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F480BC: 90A1007C  stw r5, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 82F480C0: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82F480C4: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82F480C8: 98810080  stb r4, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u8 ) };
	// 82F480CC: 38A00014  li r5, 0x14
	ctx.r[5].s64 = 20;
	// 82F480D0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F480D4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F480D8: 4BFDD369  bl 0x82f25440
	ctx.lr = 0x82F480DC;
	sub_82F25440(ctx, base);
	// 82F480DC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F480E0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F480E4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F480E8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F480EC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F480F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F480F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F480F8 size=72
    let mut pc: u32 = 0x82F480F8;
    'dispatch: loop {
        match pc {
            0x82F480F8 => {
    //   block [0x82F480F8..0x82F48140)
	// 82F480F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F480FC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F48100: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F48104: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F48108: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F4810C: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F48110: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F48114: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F48118: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F4811C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F48120: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F48124: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F48128: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F4812C: 4BFFFB1D  bl 0x82f47c48
	ctx.lr = 0x82F48130;
	sub_82F47C48(ctx, base);
	// 82F48130: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F48134: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F48138: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4813C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48140(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F48140 size=76
    let mut pc: u32 = 0x82F48140;
    'dispatch: loop {
        match pc {
            0x82F48140 => {
    //   block [0x82F48140..0x82F4818C)
	// 82F48140: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F48144: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F48148: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4814C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F48150: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F48154: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F48158: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F4815C: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F48160: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F48164: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F48168: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F4816C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F48170: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F48174: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F48178: 4BFFF661  bl 0x82f477d8
	ctx.lr = 0x82F4817C;
	sub_82F477D8(ctx, base);
	// 82F4817C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F48180: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F48184: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F48188: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48190(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F48190 size=224
    let mut pc: u32 = 0x82F48190;
    'dispatch: loop {
        match pc {
            0x82F48190 => {
    //   block [0x82F48190..0x82F48270)
	// 82F48190: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F48194: 4825FFD9  bl 0x831a816c
	ctx.lr = 0x82F48198;
	sub_831A8130(ctx, base);
	// 82F48198: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4819C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F481A0: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F481A4: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82F481A8: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F481AC: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F481B0: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F481B4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F481B8: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F481BC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F481C0: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F481C4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F481C8: 4200FFF0  bdnz 0x82f481b8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F481B8; continue 'dispatch;
	}
	// 82F481CC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F481D0: E9250050  ld r9, 0x50(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(80 as u32) ) };
	// 82F481D4: E8E50058  ld r7, 0x58(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(88 as u32) ) };
	// 82F481D8: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82F481DC: 38AAC5B0  addi r5, r10, -0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + -14928;
	// 82F481E0: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 82F481E4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F481E8: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48270(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F48270 size=72
    let mut pc: u32 = 0x82F48270;
    'dispatch: loop {
        match pc {
            0x82F48270 => {
    //   block [0x82F48270..0x82F482B8)
	// 82F48270: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F48274: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F48278: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4827C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F48280: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F48284: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F48288: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F4828C: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F48290: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F48294: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F48298: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F4829C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F482A0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F482A4: 4BFFF895  bl 0x82f47b38
	ctx.lr = 0x82F482A8;
	sub_82F47B38(ctx, base);
	// 82F482A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F482AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F482B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F482B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F482B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F482B8 size=76
    let mut pc: u32 = 0x82F482B8;
    'dispatch: loop {
        match pc {
            0x82F482B8 => {
    //   block [0x82F482B8..0x82F48304)
	// 82F482B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F482BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F482C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F482C4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F482C8: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F482CC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F482D0: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F482D4: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F482D8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F482DC: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F482E0: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F482E4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F482E8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F482EC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F482F0: 4BFFF3E1  bl 0x82f476d0
	ctx.lr = 0x82F482F4;
	sub_82F476D0(ctx, base);
	// 82F482F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F482F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F482FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F48300: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48308(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F48308 size=176
    let mut pc: u32 = 0x82F48308;
    'dispatch: loop {
        match pc {
            0x82F48308 => {
    //   block [0x82F48308..0x82F483B8)
	// 82F48308: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4830C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F48310: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F48314: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F48318: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F4831C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F48320: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F48324: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F48328: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F4832C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F48330: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48334: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F48338: 4BFFF269  bl 0x82f475a0
	ctx.lr = 0x82F4833C;
	sub_82F475A0(ctx, base);
	// 82F4833C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48340: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F48344: 40980044  bge cr6, 0x82f48388
	if !ctx.cr[6].lt {
	pc = 0x82F48388; continue 'dispatch;
	}
	// 82F48348: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F4834C: 394BC5A0  addi r10, r11, -0x3a60
	ctx.r[10].s64 = ctx.r[11].s64 + -14944;
	// 82F48350: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F483B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F483B8 size=152
    let mut pc: u32 = 0x82F483B8;
    'dispatch: loop {
        match pc {
            0x82F483B8 => {
    //   block [0x82F483B8..0x82F48450)
	// 82F483B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F483BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F483C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F483C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F483C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F483CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F483D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F483D4: 817F0014  lwz r11, 0x14(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F483D8: 556A0000  rlwinm r10, r11, 0, 0, 0
	ctx.r[10].u64 = ctx.r[11].u32 as u64 & 0xFFFFFFFFu64;
	// 82F483DC: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F483E0: 409A0020  bne cr6, 0x82f48400
	if !ctx.cr[6].eq {
	pc = 0x82F48400; continue 'dispatch;
	}
	// 82F483E4: 814D0000  lwz r10, 0(r13)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F483E8: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F483EC: 38C00017  li r6, 0x17
	ctx.r[6].s64 = 23;
	// 82F483F0: 809F000C  lwz r4, 0xc(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F483F4: 55651838  slwi r5, r11, 3
	ctx.r[5].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[5].u64 = ctx.r[5].u32 as u64;
	// 82F483F8: 7C69502E  lwzx r3, r9, r10
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F483FC: 4BF583B5  bl 0x82ea07b0
	ctx.lr = 0x82F48400;
	sub_82EA07B0(ctx, base);
	// 82F48400: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F48404: 57CA07FE  clrlwi r10, r30, 0x1f
	ctx.r[10].u64 = ctx.r[30].u32 as u64 & 0x00000001u64;
	// 82F48408: 392B9EAC  addi r9, r11, -0x6154
	ctx.r[9].s64 = ctx.r[11].s64 + -24916;
	// 82F4840C: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F48410: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F48414: 419A0020  beq cr6, 0x82f48434
	if ctx.cr[6].eq {
	pc = 0x82F48434; continue 'dispatch;
	}
	// 82F48418: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4841C: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F48420: 38C0001F  li r6, 0x1f
	ctx.r[6].s64 = 31;
	// 82F48424: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F48428: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4842C: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F48430: 4BF58381  bl 0x82ea07b0
	ctx.lr = 0x82F48434;
	sub_82EA07B0(ctx, base);
	// 82F48434: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F48438: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4843C: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F48440: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F48444: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F48448: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4844C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F48450 size=224
    let mut pc: u32 = 0x82F48450;
    'dispatch: loop {
        match pc {
            0x82F48450 => {
    //   block [0x82F48450..0x82F48530)
	// 82F48450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F48454: 4825FD15  bl 0x831a8168
	ctx.lr = 0x82F48458;
	sub_831A8130(ctx, base);
	// 82F48458: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4845C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F48460: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F48464: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82F48468: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F4846C: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82F48470: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F48474: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F48478: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F4847C: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F48480: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F48484: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F48488: 4200FFF0  bdnz 0x82f48478
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F48478; continue 'dispatch;
	}
	// 82F4848C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F48490: E9260050  ld r9, 0x50(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(80 as u32) ) };
	// 82F48494: E9060058  ld r8, 0x58(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	// 82F48498: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82F4849C: 38CAC5B0  addi r6, r10, -0x3a50
	ctx.r[6].s64 = ctx.r[10].s64 + -14928;
	// 82F484A0: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82F484A4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F484A8: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48530(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F48530 size=136
    let mut pc: u32 = 0x82F48530;
    'dispatch: loop {
        match pc {
            0x82F48530 => {
    //   block [0x82F48530..0x82F485B8)
	// 82F48530: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F48534: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F48538: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4853C: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F48540: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F48544: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F48548: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4854C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48550: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48554: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F48558: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4855C: 4E800421  bctrl
	ctx.lr = 0x82F48560;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F48560: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48564: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F48568: 419A0020  beq cr6, 0x82f48588
	if ctx.cr[6].eq {
	pc = 0x82F48588; continue 'dispatch;
	}
	// 82F4856C: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48570: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F48574: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F48578: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4857C: 4E800421  bctrl
	ctx.lr = 0x82F48580;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F48580: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F48584: 913F0010  stw r9, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[9].u32 ) };
	// 82F48588: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4858C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F48590: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F48594: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48598: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4859C: 4E800421  bctrl
	ctx.lr = 0x82F485A0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F485A0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F485A4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F485A8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F485AC: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F485B0: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F485B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F485B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F485B8 size=108
    let mut pc: u32 = 0x82F485B8;
    'dispatch: loop {
        match pc {
            0x82F485B8 => {
    //   block [0x82F485B8..0x82F48624)
	// 82F485B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F485BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F485C0: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F485C4: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F485C8: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F485CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F485D0: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F485D4: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F485D8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F485DC: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F485E0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F485E4: 4E800421  bctrl
	ctx.lr = 0x82F485E8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F485E8: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F485EC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F485F0: 419A001C  beq cr6, 0x82f4860c
	if ctx.cr[6].eq {
	pc = 0x82F4860C; continue 'dispatch;
	}
	// 82F485F4: 5523003E  slwi r3, r9, 0
	ctx.r[3].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82F485F8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F485FC: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48600: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F48604: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F48608: 4E800421  bctrl
	ctx.lr = 0x82F4860C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4860C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F48610: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F48614: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F48618: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4861C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F48620: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48628(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F48628 size=140
    let mut pc: u32 = 0x82F48628;
    'dispatch: loop {
        match pc {
            0x82F48628 => {
    //   block [0x82F48628..0x82F486B4)
	// 82F48628: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4862C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F48630: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F48634: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F48638: DBC1FFD8  stfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-40 as u32), ctx.f[30].u64 ) };
	// 82F4863C: DBE1FFE0  stfd f31, -0x20(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F48640: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F48644: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F48648: FFE00890  fmr f31, f1
	ctx.f[31].f64 = ctx.f[1].f64;
	// 82F4864C: FFC01090  fmr f30, f2
	ctx.f[30].f64 = ctx.f[2].f64;
	// 82F48650: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82F48654: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48658: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4865C: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F48660: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F48664: 4E800421  bctrl
	ctx.lr = 0x82F48668;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F48668: 813F0010  lwz r9, 0x10(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4866C: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F48670: 419A0024  beq cr6, 0x82f48694
	if ctx.cr[6].eq {
	pc = 0x82F48694; continue 'dispatch;
	}
	// 82F48674: 5523003E  slwi r3, r9, 0
	ctx.r[3].u32 = ctx.r[9].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82F48678: FC40F090  fmr f2, f30
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[2].f64 = ctx.f[30].f64;
	// 82F4867C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F48680: FC20F890  fmr f1, f31
	ctx.f[1].f64 = ctx.f[31].f64;
	// 82F48684: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48688: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F4868C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F48690: 4E800421  bctrl
	ctx.lr = 0x82F48694;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F48694: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F48698: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4869C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F486A0: CBC1FFD8  lfd f30, -0x28(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	ctx.f[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-40 as u32) ) };
	// 82F486A4: CBE1FFE0  lfd f31, -0x20(r1)
	ctx.f[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-32 as u32) ) };
	// 82F486A8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F486AC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F486B0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F486B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F486B8 size=12
    let mut pc: u32 = 0x82F486B8;
    'dispatch: loop {
        match pc {
            0x82F486B8 => {
    //   block [0x82F486B8..0x82F486C4)
	// 82F486B8: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F486BC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F486C0: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F486C4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F486C4 size=20
    let mut pc: u32 = 0x82F486C4;
    'dispatch: loop {
        match pc {
            0x82F486C4 => {
    //   block [0x82F486C4..0x82F486D8)
	// 82F486C4: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82F486C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F486CC: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F486D0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F486D4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F486D8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F486D8 size=4
    let mut pc: u32 = 0x82F486D8;
    'dispatch: loop {
        match pc {
            0x82F486D8 => {
    //   block [0x82F486D8..0x82F486DC)
	// 82F486D8: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F486E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F486E0 size=12
    let mut pc: u32 = 0x82F486E0;
    'dispatch: loop {
        match pc {
            0x82F486E0 => {
    //   block [0x82F486E0..0x82F486EC)
	// 82F486E0: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F486E4: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F486E8: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F486EC(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F486EC size=20
    let mut pc: u32 = 0x82F486EC;
    'dispatch: loop {
        match pc {
            0x82F486EC => {
    //   block [0x82F486EC..0x82F48700)
	// 82F486EC: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82F486F0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F486F4: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F486F8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F486FC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48700(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F48700 size=4
    let mut pc: u32 = 0x82F48700;
    'dispatch: loop {
        match pc {
            0x82F48700 => {
    //   block [0x82F48700..0x82F48704)
	// 82F48700: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48708(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F48708 size=12
    let mut pc: u32 = 0x82F48708;
    'dispatch: loop {
        match pc {
            0x82F48708 => {
    //   block [0x82F48708..0x82F48714)
	// 82F48708: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4870C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F48710: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48714(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F48714 size=20
    let mut pc: u32 = 0x82F48714;
    'dispatch: loop {
        match pc {
            0x82F48714 => {
    //   block [0x82F48714..0x82F48728)
	// 82F48714: 5563003E  slwi r3, r11, 0
	ctx.r[3].u32 = ctx.r[11].u32.wrapping_shl(0);
	ctx.r[3].u64 = ctx.r[3].u32 as u64;
	// 82F48718: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4871C: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F48720: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F48724: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F48728 size=4
    let mut pc: u32 = 0x82F48728;
    'dispatch: loop {
        match pc {
            0x82F48728 => {
    //   block [0x82F48728..0x82F4872C)
	// 82F48728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F48730 size=156
    let mut pc: u32 = 0x82F48730;
    'dispatch: loop {
        match pc {
            0x82F48730 => {
    //   block [0x82F48730..0x82F487CC)
	// 82F48730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F48734: 4825FA31  bl 0x831a8164
	ctx.lr = 0x82F48738;
	sub_831A8130(ctx, base);
	// 82F48738: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4873C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F48740: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F48744: 83640000  lwz r27, 0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48748: 3940FFFF  li r10, -1
	ctx.r[10].s64 = -1;
	// 82F4874C: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F48750: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F48754: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F48758: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82F4875C: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82F48760: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F48764: 813B0010  lwz r9, 0x10(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48768: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F4876C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48770: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82F48774: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48778: 81680020  lwz r11, 0x20(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F4877C: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F48780: 4E800421  bctrl
	ctx.lr = 0x82F48784;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F48784: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48788: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F4878C: 419A0038  beq cr6, 0x82f487c4
	if ctx.cr[6].eq {
	pc = 0x82F487C4; continue 'dispatch;
	}
	// 82F48790: 817B0018  lwz r11, 0x18(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F48794: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F48798: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4879C: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F487A0: 91410054  stw r10, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[10].u32 ) };
	// 82F487A4: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F487A8: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F487AC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F487B0: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82F487B4: 81230000  lwz r9, 0(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F487B8: 81090020  lwz r8, 0x20(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F487BC: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F487C0: 4E800421  bctrl
	ctx.lr = 0x82F487C4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F487C4: 38210090  addi r1, r1, 0x90
	ctx.r[1].s64 = ctx.r[1].s64 + 144;
	// 82F487C8: 4825F9EC  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F487D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F487D0 size=208
    let mut pc: u32 = 0x82F487D0;
    'dispatch: loop {
        match pc {
            0x82F487D0 => {
    //   block [0x82F487D0..0x82F488A0)
	// 82F487D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F487D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F487D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F487DC: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F487E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F487E4: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F487E8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F487EC: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F487F0: 9161005C  stw r11, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[11].u32 ) };
	// 82F487F4: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F487F8: 390AFF34  addi r8, r10, -0xcc
	ctx.r[8].s64 = ctx.r[10].s64 + -204;
	// 82F487FC: 90FF0008  stw r7, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82F48800: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82F48804: 911F0000  stw r8, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F48808: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82F4880C: B13F0006  sth r9, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[9].u16 ) };
	// 82F48810: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48814: 81050010  lwz r8, 0x10(r5)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48818: 81240000  lwz r9, 0(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4881C: 81450000  lwz r10, 0(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48820: 816B0008  lwz r11, 8(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F48824: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82F48828: 81630010  lwz r11, 0x10(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4882C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F48830: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82F48834: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F48838: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4883C: 392A05A0  addi r9, r10, 0x5a0
	ctx.r[9].s64 = ctx.r[10].s64 + 1440;
	// 82F48840: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48844: 409A0008  bne cr6, 0x82f4884c
	if !ctx.cr[6].eq {
	pc = 0x82F4884C; continue 'dispatch;
	}
	// 82F48848: 392A01A0  addi r9, r10, 0x1a0
	ctx.r[9].s64 = ctx.r[10].s64 + 416;
	// 82F4884C: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F48850: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F48854: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F48858: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F4885C: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82F48860: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F48864: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F48868: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4886C: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F48870: 80E809A0  lwz r7, 0x9a0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82F48874: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F48878: 4E800421  bctrl
	ctx.lr = 0x82F4887C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4887C: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82F48880: 907F000C  stw r3, 0xc(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82F48884: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F48888: 90DF0010  stw r6, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[6].u32 ) };
	// 82F4888C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F48890: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F48894: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F48898: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4889C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F488A0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F488A0 size=88
    let mut pc: u32 = 0x82F488A0;
    'dispatch: loop {
        match pc {
            0x82F488A0 => {
    //   block [0x82F488A0..0x82F488F8)
	// 82F488A0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F488A4: 4825F8C5  bl 0x831a8168
	ctx.lr = 0x82F488A8;
	sub_831A8130(ctx, base);
	// 82F488A8: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F488AC: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F488B0: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F488B4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F488B8: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F488BC: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F488C0: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82F488C4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F488C8: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F488CC: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F488D0: 4BF57E61  bl 0x82ea0730
	ctx.lr = 0x82F488D4;
	sub_82EA0730(ctx, base);
	// 82F488D4: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F488D8: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F488DC: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F488E0: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F488E4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F488E8: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F488EC: 4BFFFEE5  bl 0x82f487d0
	ctx.lr = 0x82F488F0;
	sub_82F487D0(ctx, base);
	// 82F488F0: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F488F4: 4825F8C4  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F488F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F488F8 size=496
    let mut pc: u32 = 0x82F488F8;
    'dispatch: loop {
        match pc {
            0x82F488F8 => {
    //   block [0x82F488F8..0x82F48AE8)
	// 82F488F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F488FC: 4825F85D  bl 0x831a8158
	ctx.lr = 0x82F48900;
	sub_831A8130(ctx, base);
	// 82F48900: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F48904: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48908: 3B200018  li r25, 0x18
	ctx.r[25].s64 = 24;
	// 82F4890C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F48910: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F48914: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82F48918: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82F4891C: 7D79D02E  lwzx r11, r25, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F48920: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48924: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F48928: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F4892C: 4098002C  bge cr6, 0x82f48958
	if !ctx.cr[6].lt {
	pc = 0x82F48958; continue 'dispatch;
	}
	// 82F48930: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F48934: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F48938: 38E9FF7C  addi r7, r9, -0x84
	ctx.r[7].s64 = ctx.r[9].s64 + -132;
	// 82F4893C: 38C8FF6C  addi r6, r8, -0x94
	ctx.r[6].s64 = ctx.r[8].s64 + -148;
	// 82F48940: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F48944: 90CA000C  stw r6, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F48948: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F4894C: 392A0010  addi r9, r10, 0x10
	ctx.r[9].s64 = ctx.r[10].s64 + 16;
	// 82F48950: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F48954: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82F48958: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4895C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F48960: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48964: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82F48968: 9081006C  stw r4, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[4].u32 ) };
	// 82F4896C: 390AF78C  addi r8, r10, -0x874
	ctx.r[8].s64 = ctx.r[10].s64 + -2164;
	// 82F48970: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48974: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82F48978: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F4897C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82F48980: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F48984: 809C0010  lwz r4, 0x10(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48988: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F4898C: 90810060  stw r4, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 82F48990: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F48994: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82F48998: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F4899C: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82F489A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F489A4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F489A8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F489AC: 4E800421  bctrl
	ctx.lr = 0x82F489B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F489B0: 89210054  lbz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F489B4: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F489B8: 419A00D4  beq cr6, 0x82f48a8c
	if ctx.cr[6].eq {
	pc = 0x82F48A8C; continue 'dispatch;
	}
	// 82F489BC: 7D59D02E  lwzx r10, r25, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F489C0: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F489C4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F489C8: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F489CC: 40980020  bge cr6, 0x82f489ec
	if !ctx.cr[6].lt {
	pc = 0x82F489EC; continue 'dispatch;
	}
	// 82F489D0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F489D4: 3909F834  addi r8, r9, -0x7cc
	ctx.r[8].s64 = ctx.r[9].s64 + -1996;
	// 82F489D8: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F489DC: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F489E0: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F489E4: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F489E8: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F489EC: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F489F0: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F489F4: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82F489F8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F489FC: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82F48A00: 409A0064  bne cr6, 0x82f48a64
	if !ctx.cr[6].eq {
	pc = 0x82F48A64; continue 'dispatch;
	}
	// 82F48A04: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48A08: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48A0C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48A10: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F48A14: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F48A18: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48A1C: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48A20: 392A05A0  addi r9, r10, 0x5a0
	ctx.r[9].s64 = ctx.r[10].s64 + 1440;
	// 82F48A24: 409A0008  bne cr6, 0x82f48a2c
	if !ctx.cr[6].eq {
	pc = 0x82F48A2C; continue 'dispatch;
	}
	// 82F48A28: 392A01A0  addi r9, r10, 0x1a0
	ctx.r[9].s64 = ctx.r[10].s64 + 416;
	// 82F48A2C: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F48A30: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F48A34: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F48A38: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F48A3C: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F48A40: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82F48A44: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F48A48: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F48A4C: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F48A50: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F48A54: 80E809A0  lwz r7, 0x9a0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82F48A58: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F48A5C: 4E800421  bctrl
	ctx.lr = 0x82F48A60;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F48A60: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82F48A64: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48A68: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82F48A6C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F48A70: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F48A74: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F48A78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48A7C: 814B0018  lwz r10, 0x18(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F48A80: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F48A84: 4E800421  bctrl
	ctx.lr = 0x82F48A88;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F48A88: 48000028  b 0x82f48ab0
	pc = 0x82F48AB0; continue 'dispatch;
	// 82F48A8C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48A90: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F48A94: 419A001C  beq cr6, 0x82f48ab0
	if ctx.cr[6].eq {
	pc = 0x82F48AB0; continue 'dispatch;
	}
	// 82F48A98: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48A9C: 80980004  lwz r4, 4(r24)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[24].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F48AA0: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F48AA4: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F48AA8: 4E800421  bctrl
	ctx.lr = 0x82F48AAC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F48AAC: 937F0010  stw r27, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[27].u32 ) };
	// 82F48AB0: 7D59D02E  lwzx r10, r25, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F48AB4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48AB8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F48ABC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F48AC0: 40980020  bge cr6, 0x82f48ae0
	if !ctx.cr[6].lt {
	pc = 0x82F48AE0; continue 'dispatch;
	}
	// 82F48AC4: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82F48AC8: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82F48ACC: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F48AD0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F48AD4: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F48AD8: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F48ADC: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F48AE0: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82F48AE4: 4825F6C4  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F48AE8 size=484
    let mut pc: u32 = 0x82F48AE8;
    'dispatch: loop {
        match pc {
            0x82F48AE8 => {
    //   block [0x82F48AE8..0x82F48CCC)
	// 82F48AE8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F48AEC: 4825F669  bl 0x831a8154
	ctx.lr = 0x82F48AF0;
	sub_831A8130(ctx, base);
	// 82F48AF0: 9421FF20  stwu r1, -0xe0(r1)
	ea = ctx.r[1].u32.wrapping_add(-224 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F48AF4: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48AF8: 3B200018  li r25, 0x18
	ctx.r[25].s64 = 24;
	// 82F48AFC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F48B00: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F48B04: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82F48B08: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82F48B0C: 7D79D02E  lwzx r11, r25, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F48B10: 7D174378  mr r23, r8
	ctx.r[23].u64 = ctx.r[8].u64;
	// 82F48B14: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48B18: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F48B1C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F48B20: 4098002C  bge cr6, 0x82f48b4c
	if !ctx.cr[6].lt {
	pc = 0x82F48B4C; continue 'dispatch;
	}
	// 82F48B24: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F48B28: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F48B2C: 38E9FF7C  addi r7, r9, -0x84
	ctx.r[7].s64 = ctx.r[9].s64 + -132;
	// 82F48B30: 38C8FF6C  addi r6, r8, -0x94
	ctx.r[6].s64 = ctx.r[8].s64 + -148;
	// 82F48B34: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F48B38: 90CA000C  stw r6, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F48B3C: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F48B40: 392A0010  addi r9, r10, 0x10
	ctx.r[9].s64 = ctx.r[10].s64 + 16;
	// 82F48B44: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F48B48: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82F48B4C: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F48B50: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F48B54: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48B58: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F48B5C: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48B60: 3900FFFF  li r8, -1
	ctx.r[8].s64 = -1;
	// 82F48B64: 38E9F798  addi r7, r9, -0x868
	ctx.r[7].s64 = ctx.r[9].s64 + -2152;
	// 82F48B68: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F48B6C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82F48B70: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82F48B74: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F48B78: 80DC0010  lwz r6, 0x10(r28)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48B7C: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82F48B80: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82F48B84: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82F48B88: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82F48B8C: 39010060  addi r8, r1, 0x60
	ctx.r[8].s64 = ctx.r[1].s64 + 96;
	// 82F48B90: 90E10060  stw r7, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[7].u32 ) };
	// 82F48B94: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82F48B98: 9B610068  stb r27, 0x68(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u8 ) };
	// 82F48B9C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F48BA0: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48BA4: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F48BA8: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F48BAC: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F48BB0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F48BB4: 4E800421  bctrl
	ctx.lr = 0x82F48BB8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F48BB8: 89410068  lbz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82F48BBC: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F48BC0: 419A00D4  beq cr6, 0x82f48c94
	if ctx.cr[6].eq {
	pc = 0x82F48C94; continue 'dispatch;
	}
	// 82F48BC4: 7D59D02E  lwzx r10, r25, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F48BC8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48BCC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F48BD0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F48BD4: 40980020  bge cr6, 0x82f48bf4
	if !ctx.cr[6].lt {
	pc = 0x82F48BF4; continue 'dispatch;
	}
	// 82F48BD8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F48BDC: 3909F834  addi r8, r9, -0x7cc
	ctx.r[8].s64 = ctx.r[9].s64 + -1996;
	// 82F48BE0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F48BE4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F48BE8: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F48BEC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F48BF0: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F48BF4: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F48BF8: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48BFC: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82F48C00: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F48C04: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82F48C08: 409A0064  bne cr6, 0x82f48c6c
	if !ctx.cr[6].eq {
	pc = 0x82F48C6C; continue 'dispatch;
	}
	// 82F48C0C: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48C10: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48C14: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48C18: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F48C1C: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F48C20: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48C24: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48C28: 392A05A0  addi r9, r10, 0x5a0
	ctx.r[9].s64 = ctx.r[10].s64 + 1440;
	// 82F48C2C: 409A0008  bne cr6, 0x82f48c34
	if !ctx.cr[6].eq {
	pc = 0x82F48C34; continue 'dispatch;
	}
	// 82F48C30: 392A01A0  addi r9, r10, 0x1a0
	ctx.r[9].s64 = ctx.r[10].s64 + 416;
	// 82F48C34: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F48C38: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F48C3C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F48C40: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F48C44: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F48C48: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82F48C4C: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F48C50: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F48C54: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F48C58: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F48C5C: 80E809A0  lwz r7, 0x9a0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82F48C60: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F48C64: 4E800421  bctrl
	ctx.lr = 0x82F48C68;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F48C68: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82F48C6C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48C70: 7EE8BB78  mr r8, r23
	ctx.r[8].u64 = ctx.r[23].u64;
	// 82F48C74: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82F48C78: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F48C7C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F48C80: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F48C84: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48C88: 814B0014  lwz r10, 0x14(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F48C8C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F48C90: 4E800421  bctrl
	ctx.lr = 0x82F48C94;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F48C94: 7D59D02E  lwzx r10, r25, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F48C98: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48C9C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F48CA0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F48CA4: 40980020  bge cr6, 0x82f48cc4
	if !ctx.cr[6].lt {
	pc = 0x82F48CC4; continue 'dispatch;
	}
	// 82F48CA8: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82F48CAC: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82F48CB0: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F48CB4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F48CB8: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F48CBC: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F48CC0: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F48CC4: 382100E0  addi r1, r1, 0xe0
	ctx.r[1].s64 = ctx.r[1].s64 + 224;
	// 82F48CC8: 4825F4DC  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48CD0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F48CD0 size=456
    let mut pc: u32 = 0x82F48CD0;
    'dispatch: loop {
        match pc {
            0x82F48CD0 => {
    //   block [0x82F48CD0..0x82F48E98)
	// 82F48CD0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F48CD4: 4825F47D  bl 0x831a8150
	ctx.lr = 0x82F48CD8;
	sub_831A8130(ctx, base);
	// 82F48CD8: 9421FF10  stwu r1, -0xf0(r1)
	ea = ctx.r[1].u32.wrapping_add(-240 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F48CDC: 830D0000  lwz r24, 0(r13)
	ctx.r[24].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48CE0: 3AE00018  li r23, 0x18
	ctx.r[23].s64 = 24;
	// 82F48CE4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F48CE8: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F48CEC: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F48CF0: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F48CF4: 7D77C02E  lwzx r11, r23, r24
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F48CF8: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48CFC: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F48D00: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F48D04: 4098002C  bge cr6, 0x82f48d30
	if !ctx.cr[6].lt {
	pc = 0x82F48D30; continue 'dispatch;
	}
	// 82F48D08: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F48D0C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F48D10: 38E9FF7C  addi r7, r9, -0x84
	ctx.r[7].s64 = ctx.r[9].s64 + -132;
	// 82F48D14: 38C8FF6C  addi r6, r8, -0x94
	ctx.r[6].s64 = ctx.r[8].s64 + -148;
	// 82F48D18: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F48D1C: 90CA000C  stw r6, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F48D20: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F48D24: 392A0010  addi r9, r10, 0x10
	ctx.r[9].s64 = ctx.r[10].s64 + 16;
	// 82F48D28: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F48D2C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82F48D30: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F48D34: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82F48D38: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48D3C: 3D008201  lis r8, -0x7dff
	ctx.r[8].s64 = -2113863680;
	// 82F48D40: 9061005C  stw r3, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[3].u32 ) };
	// 82F48D44: 3CE08213  lis r7, -0x7ded
	ctx.r[7].s64 = -2112684032;
	// 82F48D48: 809E0000  lwz r4, 0(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48D4C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82F48D50: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48D54: 3AC7F798  addi r22, r7, -0x868
	ctx.r[22].s64 = ctx.r[7].s64 + -2152;
	// 82F48D58: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82F48D5C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82F48D60: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48D64: C008BA78  lfs f0, -0x4588(r8)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F48D68: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82F48D6C: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F48D70: 91210054  stw r9, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u32 ) };
	// 82F48D74: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F48D78: 8384000C  lwz r28, 0xc(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48D7C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F48D80: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48D84: 386A000D  addi r3, r10, 0xd
	ctx.r[3].s64 = ctx.r[10].s64 + 13;
	// 82F48D88: 546A2834  slwi r10, r3, 5
	ctx.r[10].u32 = ctx.r[3].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F48D8C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F48D90: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F48D94: 7D4AE0AE  lbzx r10, r10, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82F48D98: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82F48D9C: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82F48DA0: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F48DA4: 7D0A5A14  add r8, r10, r11
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F48DA8: 816809AC  lwz r11, 0x9ac(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2476 as u32) ) } as u64;
	// 82F48DAC: D001008C  stfs f0, 0x8c(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(140 as u32), tmp.u32 ) };
	// 82F48DB0: D0010064  stfs f0, 0x64(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), tmp.u32 ) };
	// 82F48DB4: 92C10060  stw r22, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[22].u32 ) };
	// 82F48DB8: 9B610068  stb r27, 0x68(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[27].u8 ) };
	// 82F48DBC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F48DC0: 4E800421  bctrl
	ctx.lr = 0x82F48DC4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F48DC4: 89410068  lbz r10, 0x68(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(104 as u32) ) } as u64;
	// 82F48DC8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F48DCC: 419A0088  beq cr6, 0x82f48e54
	if ctx.cr[6].eq {
	pc = 0x82F48E54; continue 'dispatch;
	}
	// 82F48DD0: 7D57C02E  lwzx r10, r23, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F48DD4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48DD8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F48DDC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F48DE0: 40980020  bge cr6, 0x82f48e00
	if !ctx.cr[6].lt {
	pc = 0x82F48E00; continue 'dispatch;
	}
	// 82F48DE4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F48DE8: 3909F834  addi r8, r9, -0x7cc
	ctx.r[8].s64 = ctx.r[9].s64 + -1996;
	// 82F48DEC: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F48DF0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F48DF4: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F48DF8: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F48DFC: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F48E00: 815D0018  lwz r10, 0x18(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F48E04: 7F27CB78  mr r7, r25
	ctx.r[7].u64 = ctx.r[25].u64;
	// 82F48E08: 93610054  stw r27, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u32 ) };
	// 82F48E0C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82F48E10: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48E14: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F48E18: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F48E1C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F48E20: 91410050  stw r10, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[10].u32 ) };
	// 82F48E24: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48E28: 394A000D  addi r10, r10, 0xd
	ctx.r[10].s64 = ctx.r[10].s64 + 13;
	// 82F48E2C: 554A2834  slwi r10, r10, 5
	ctx.r[10].u32 = ctx.r[10].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F48E30: 7D2A5A14  add r9, r10, r11
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F48E34: 7D49E0AE  lbzx r10, r9, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82F48E38: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82F48E3C: 7D0A4A14  add r8, r10, r9
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82F48E40: 550A103A  slwi r10, r8, 2
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F48E44: 7D6A5A14  add r11, r10, r11
	ctx.r[11].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F48E48: 814B09AC  lwz r10, 0x9ac(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2476 as u32) ) } as u64;
	// 82F48E4C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F48E50: 4E800421  bctrl
	ctx.lr = 0x82F48E54;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F48E54: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F48E58: 7D57C02E  lwzx r10, r23, r24
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[23].u32.wrapping_add(ctx.r[24].u32)) } as u64;
	// 82F48E5C: 392BBA80  addi r9, r11, -0x4580
	ctx.r[9].s64 = ctx.r[11].s64 + -17792;
	// 82F48E60: 91210060  stw r9, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[9].u32 ) };
	// 82F48E64: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48E68: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F48E6C: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F48E70: 40980020  bge cr6, 0x82f48e90
	if !ctx.cr[6].lt {
	pc = 0x82F48E90; continue 'dispatch;
	}
	// 82F48E74: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82F48E78: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82F48E7C: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F48E80: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F48E84: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F48E88: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F48E8C: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F48E90: 382100F0  addi r1, r1, 0xf0
	ctx.r[1].s64 = ctx.r[1].s64 + 240;
	// 82F48E94: 4825F30C  b 0x831a81a0
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F48E98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F48E98 size=468
    let mut pc: u32 = 0x82F48E98;
    'dispatch: loop {
        match pc {
            0x82F48E98 => {
    //   block [0x82F48E98..0x82F4906C)
	// 82F48E98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F48E9C: 4825F2BD  bl 0x831a8158
	ctx.lr = 0x82F48EA0;
	sub_831A8130(ctx, base);
	// 82F48EA0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F48EA4: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48EA8: 3B200018  li r25, 0x18
	ctx.r[25].s64 = 24;
	// 82F48EAC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F48EB0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F48EB4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82F48EB8: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82F48EBC: 7D79D02E  lwzx r11, r25, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F48EC0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48EC4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F48EC8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F48ECC: 4098002C  bge cr6, 0x82f48ef8
	if !ctx.cr[6].lt {
	pc = 0x82F48EF8; continue 'dispatch;
	}
	// 82F48ED0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F48ED4: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F48ED8: 38E9FF7C  addi r7, r9, -0x84
	ctx.r[7].s64 = ctx.r[9].s64 + -132;
	// 82F48EDC: 38C8FF6C  addi r6, r8, -0x94
	ctx.r[6].s64 = ctx.r[8].s64 + -148;
	// 82F48EE0: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F48EE4: 90CA000C  stw r6, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F48EE8: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F48EEC: 392A0010  addi r9, r10, 0x10
	ctx.r[9].s64 = ctx.r[10].s64 + 16;
	// 82F48EF0: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F48EF4: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82F48EF8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F48EFC: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F48F00: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48F04: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82F48F08: 9081006C  stw r4, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[4].u32 ) };
	// 82F48F0C: 390AF78C  addi r8, r10, -0x874
	ctx.r[8].s64 = ctx.r[10].s64 + -2164;
	// 82F48F10: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48F14: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82F48F18: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F48F1C: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82F48F20: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F48F24: 809C0010  lwz r4, 0x10(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48F28: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F48F2C: 90810060  stw r4, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 82F48F30: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F48F34: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82F48F38: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F48F3C: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82F48F40: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48F44: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48F48: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F48F4C: 4E800421  bctrl
	ctx.lr = 0x82F48F50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F48F50: 89210054  lbz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F48F54: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F48F58: 419A00D0  beq cr6, 0x82f49028
	if ctx.cr[6].eq {
	pc = 0x82F49028; continue 'dispatch;
	}
	// 82F48F5C: 7D59D02E  lwzx r10, r25, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F48F60: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48F64: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F48F68: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F48F6C: 40980020  bge cr6, 0x82f48f8c
	if !ctx.cr[6].lt {
	pc = 0x82F48F8C; continue 'dispatch;
	}
	// 82F48F70: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F48F74: 3909F834  addi r8, r9, -0x7cc
	ctx.r[8].s64 = ctx.r[9].s64 + -1996;
	// 82F48F78: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F48F7C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F48F80: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F48F84: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F48F88: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F48F8C: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F48F90: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48F94: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82F48F98: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F48F9C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82F48FA0: 409A0064  bne cr6, 0x82f49004
	if !ctx.cr[6].eq {
	pc = 0x82F49004; continue 'dispatch;
	}
	// 82F48FA4: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48FA8: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F48FAC: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F48FB0: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F48FB4: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F48FB8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48FBC: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F48FC0: 392A05A0  addi r9, r10, 0x5a0
	ctx.r[9].s64 = ctx.r[10].s64 + 1440;
	// 82F48FC4: 409A0008  bne cr6, 0x82f48fcc
	if !ctx.cr[6].eq {
	pc = 0x82F48FCC; continue 'dispatch;
	}
	// 82F48FC8: 392A01A0  addi r9, r10, 0x1a0
	ctx.r[9].s64 = ctx.r[10].s64 + 416;
	// 82F48FCC: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F48FD0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F48FD4: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F48FD8: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F48FDC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F48FE0: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82F48FE4: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F48FE8: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F48FEC: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F48FF0: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F48FF4: 80E809A0  lwz r7, 0x9a0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82F48FF8: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F48FFC: 4E800421  bctrl
	ctx.lr = 0x82F49000;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F49000: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82F49004: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F49008: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82F4900C: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F49010: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F49014: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F49018: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4901C: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F49020: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F49024: 4E800421  bctrl
	ctx.lr = 0x82F49028;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F49028: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F4902C: 7D59D02E  lwzx r10, r25, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F49030: 392BC3C0  addi r9, r11, -0x3c40
	ctx.r[9].s64 = ctx.r[11].s64 + -15424;
	// 82F49034: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F49038: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4903C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F49040: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F49044: 40980020  bge cr6, 0x82f49064
	if !ctx.cr[6].lt {
	pc = 0x82F49064; continue 'dispatch;
	}
	// 82F49048: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82F4904C: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82F49050: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F49054: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F49058: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F4905C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F49060: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F49064: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82F49068: 4825F140  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49070(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F49070 size=428
    let mut pc: u32 = 0x82F49070;
    'dispatch: loop {
        match pc {
            0x82F49070 => {
    //   block [0x82F49070..0x82F4921C)
	// 82F49070: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F49074: 4825F0E5  bl 0x831a8158
	ctx.lr = 0x82F49078;
	sub_831A8130(ctx, base);
	// 82F49078: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4907C: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49080: 3B000018  li r24, 0x18
	ctx.r[24].s64 = 24;
	// 82F49084: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F49088: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F4908C: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F49090: 7D58C82E  lwzx r10, r24, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82F49094: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F49098: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4909C: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F490A0: 4098002C  bge cr6, 0x82f490cc
	if !ctx.cr[6].lt {
	pc = 0x82F490CC; continue 'dispatch;
	}
	// 82F490A4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F490A8: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F490AC: 38E9FF7C  addi r7, r9, -0x84
	ctx.r[7].s64 = ctx.r[9].s64 + -132;
	// 82F490B0: 38C8FF6C  addi r6, r8, -0x94
	ctx.r[6].s64 = ctx.r[8].s64 + -148;
	// 82F490B4: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F490B8: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F490BC: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F490C0: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82F490C4: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F490C8: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82F490CC: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F490D0: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82F490D4: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F490D8: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F490DC: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 82F490E0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82F490E4: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F490E8: 3908F78C  addi r8, r8, -0x874
	ctx.r[8].s64 = ctx.r[8].s64 + -2164;
	// 82F490EC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F490F0: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F490F4: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82F490F8: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F490FC: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F49100: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F49104: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82F49108: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F4910C: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82F49110: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F49114: 8387000C  lwz r28, 0xc(r7)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F49118: 38EA000D  addi r7, r10, 0xd
	ctx.r[7].s64 = ctx.r[10].s64 + 13;
	// 82F4911C: 54EA2834  slwi r10, r7, 5
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F49120: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F49124: 7D4AE0AE  lbzx r10, r10, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82F49128: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82F4912C: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82F49130: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F49134: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F49138: 816709A4  lwz r11, 0x9a4(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82F4913C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F49140: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82F49144: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F49148: 4E800421  bctrl
	ctx.lr = 0x82F4914C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4914C: 89410054  lbz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F49150: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F49154: 419A0084  beq cr6, 0x82f491d8
	if ctx.cr[6].eq {
	pc = 0x82F491D8; continue 'dispatch;
	}
	// 82F49158: 7D58C82E  lwzx r10, r24, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82F4915C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F49160: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F49164: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F49168: 40980020  bge cr6, 0x82f49188
	if !ctx.cr[6].lt {
	pc = 0x82F49188; continue 'dispatch;
	}
	// 82F4916C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F49170: 3909F834  addi r8, r9, -0x7cc
	ctx.r[8].s64 = ctx.r[9].s64 + -1996;
	// 82F49174: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F49178: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F4917C: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F49180: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F49184: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F49188: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F4918C: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82F49190: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82F49194: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F49198: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4919C: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F491A0: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F491A4: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82F491A8: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F491AC: 396B000D  addi r11, r11, 0xd
	ctx.r[11].s64 = ctx.r[11].s64 + 13;
	// 82F491B0: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F491B4: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F491B8: 7D69E0AE  lbzx r11, r9, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82F491BC: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F491C0: 7D0B4A14  add r8, r11, r9
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F491C4: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F491C8: 7CEB5214  add r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F491CC: 816709A8  lwz r11, 0x9a8(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2472 as u32) ) } as u64;
	// 82F491D0: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F491D4: 4E800421  bctrl
	ctx.lr = 0x82F491D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F491D8: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F491DC: 7D58C82E  lwzx r10, r24, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82F491E0: 392BC3C0  addi r9, r11, -0x3c40
	ctx.r[9].s64 = ctx.r[11].s64 + -15424;
	// 82F491E4: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F491E8: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F491EC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F491F0: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F491F4: 40980020  bge cr6, 0x82f49214
	if !ctx.cr[6].lt {
	pc = 0x82F49214; continue 'dispatch;
	}
	// 82F491F8: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82F491FC: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82F49200: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F49204: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F49208: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F4920C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F49210: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F49214: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82F49218: 4825EF90  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F49220 size=468
    let mut pc: u32 = 0x82F49220;
    'dispatch: loop {
        match pc {
            0x82F49220 => {
    //   block [0x82F49220..0x82F493F4)
	// 82F49220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F49224: 4825EF35  bl 0x831a8158
	ctx.lr = 0x82F49228;
	sub_831A8130(ctx, base);
	// 82F49228: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4922C: 834D0000  lwz r26, 0(r13)
	ctx.r[26].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49230: 3B200018  li r25, 0x18
	ctx.r[25].s64 = 24;
	// 82F49234: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F49238: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F4923C: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82F49240: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82F49244: 7D79D02E  lwzx r11, r25, r26
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F49248: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4924C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F49250: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F49254: 4098002C  bge cr6, 0x82f49280
	if !ctx.cr[6].lt {
	pc = 0x82F49280; continue 'dispatch;
	}
	// 82F49258: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4925C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F49260: 38E9FF7C  addi r7, r9, -0x84
	ctx.r[7].s64 = ctx.r[9].s64 + -132;
	// 82F49264: 38C8FF6C  addi r6, r8, -0x94
	ctx.r[6].s64 = ctx.r[8].s64 + -148;
	// 82F49268: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F4926C: 90CA000C  stw r6, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F49270: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F49274: 392A0010  addi r9, r10, 0x10
	ctx.r[9].s64 = ctx.r[10].s64 + 16;
	// 82F49278: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F4927C: 912B0004  stw r9, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82F49280: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F49284: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F49288: 83840000  lwz r28, 0(r4)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4928C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82F49290: 9081006C  stw r4, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[4].u32 ) };
	// 82F49294: 390AF78C  addi r8, r10, -0x874
	ctx.r[8].s64 = ctx.r[10].s64 + -2164;
	// 82F49298: 807F000C  lwz r3, 0xc(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4929C: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82F492A0: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F492A4: 91610068  stw r11, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[11].u32 ) };
	// 82F492A8: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F492AC: 809C0010  lwz r4, 0x10(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F492B0: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F492B4: 90810060  stw r4, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[4].u32 ) };
	// 82F492B8: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F492BC: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82F492C0: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F492C4: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82F492C8: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F492CC: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F492D0: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F492D4: 4E800421  bctrl
	ctx.lr = 0x82F492D8;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F492D8: 89210054  lbz r9, 0x54(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F492DC: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F492E0: 419A00D0  beq cr6, 0x82f493b0
	if ctx.cr[6].eq {
	pc = 0x82F493B0; continue 'dispatch;
	}
	// 82F492E4: 7D59D02E  lwzx r10, r25, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F492E8: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F492EC: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F492F0: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F492F4: 40980020  bge cr6, 0x82f49314
	if !ctx.cr[6].lt {
	pc = 0x82F49314; continue 'dispatch;
	}
	// 82F492F8: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F492FC: 3909F834  addi r8, r9, -0x7cc
	ctx.r[8].s64 = ctx.r[9].s64 + -1996;
	// 82F49300: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F49304: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F49308: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F4930C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F49310: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F49314: 817C0018  lwz r11, 0x18(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F49318: 815F0010  lwz r10, 0x10(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4931C: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82F49320: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F49324: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82F49328: 409A0064  bne cr6, 0x82f4938c
	if !ctx.cr[6].eq {
	pc = 0x82F4938C; continue 'dispatch;
	}
	// 82F4932C: 813D0000  lwz r9, 0(r29)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49330: 811E0010  lwz r8, 0x10(r30)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F49334: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49338: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F4933C: 80DF0008  lwz r6, 8(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F49340: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F49344: 8109000C  lwz r8, 0xc(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F49348: 392A05A0  addi r9, r10, 0x5a0
	ctx.r[9].s64 = ctx.r[10].s64 + 1440;
	// 82F4934C: 409A0008  bne cr6, 0x82f49354
	if !ctx.cr[6].eq {
	pc = 0x82F49354; continue 'dispatch;
	}
	// 82F49350: 392A01A0  addi r9, r10, 0x1a0
	ctx.r[9].s64 = ctx.r[10].s64 + 416;
	// 82F49354: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F49358: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F4935C: 7D6B4A14  add r11, r11, r9
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F49360: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F49364: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F49368: 7D6B40AE  lbzx r11, r11, r8
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[8].u32)) } as u64;
	// 82F4936C: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F49370: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F49374: 552B103A  slwi r11, r9, 2
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F49378: 7D0B5214  add r8, r11, r10
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F4937C: 80E809A0  lwz r7, 0x9a0(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82F49380: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F49384: 4E800421  bctrl
	ctx.lr = 0x82F49388;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F49388: 907F0010  stw r3, 0x10(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(16 as u32), ctx.r[3].u32 ) };
	// 82F4938C: 807F0010  lwz r3, 0x10(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F49390: 7F07C378  mr r7, r24
	ctx.r[7].u64 = ctx.r[24].u64;
	// 82F49394: 7FC6F378  mr r6, r30
	ctx.r[6].u64 = ctx.r[30].u64;
	// 82F49398: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F4939C: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F493A0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F493A4: 814B000C  lwz r10, 0xc(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F493A8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F493AC: 4E800421  bctrl
	ctx.lr = 0x82F493B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F493B0: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F493B4: 7D59D02E  lwzx r10, r25, r26
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[25].u32.wrapping_add(ctx.r[26].u32)) } as u64;
	// 82F493B8: 392BC3C0  addi r9, r11, -0x3c40
	ctx.r[9].s64 = ctx.r[11].s64 + -15424;
	// 82F493BC: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F493C0: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F493C4: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F493C8: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F493CC: 40980020  bge cr6, 0x82f493ec
	if !ctx.cr[6].lt {
	pc = 0x82F493EC; continue 'dispatch;
	}
	// 82F493D0: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82F493D4: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82F493D8: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F493DC: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F493E0: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F493E4: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F493E8: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F493EC: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82F493F0: 4825EDB8  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F493F8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F493F8 size=428
    let mut pc: u32 = 0x82F493F8;
    'dispatch: loop {
        match pc {
            0x82F493F8 => {
    //   block [0x82F493F8..0x82F495A4)
	// 82F493F8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F493FC: 4825ED5D  bl 0x831a8158
	ctx.lr = 0x82F49400;
	sub_831A8130(ctx, base);
	// 82F49400: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F49404: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49408: 3B000018  li r24, 0x18
	ctx.r[24].s64 = 24;
	// 82F4940C: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F49410: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F49414: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F49418: 7D58C82E  lwzx r10, r24, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82F4941C: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F49420: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F49424: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F49428: 4098002C  bge cr6, 0x82f49454
	if !ctx.cr[6].lt {
	pc = 0x82F49454; continue 'dispatch;
	}
	// 82F4942C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F49430: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F49434: 38E9FF7C  addi r7, r9, -0x84
	ctx.r[7].s64 = ctx.r[9].s64 + -132;
	// 82F49438: 38C8FF6C  addi r6, r8, -0x94
	ctx.r[6].s64 = ctx.r[8].s64 + -148;
	// 82F4943C: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F49440: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F49444: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F49448: 392B0010  addi r9, r11, 0x10
	ctx.r[9].s64 = ctx.r[11].s64 + 16;
	// 82F4944C: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F49450: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82F49454: 81430008  lwz r10, 8(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F49458: 3920FFFF  li r9, -1
	ctx.r[9].s64 = -1;
	// 82F4945C: 83A30000  lwz r29, 0(r3)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49460: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F49464: 9061006C  stw r3, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[3].u32 ) };
	// 82F49468: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82F4946C: 80FE0000  lwz r7, 0(r30)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49470: 3908F78C  addi r8, r8, -0x874
	ctx.r[8].s64 = ctx.r[8].s64 + -2164;
	// 82F49474: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49478: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F4947C: 91410068  stw r10, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[10].u32 ) };
	// 82F49480: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F49484: 815D0010  lwz r10, 0x10(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F49488: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F4948C: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82F49490: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F49494: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82F49498: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4949C: 8387000C  lwz r28, 0xc(r7)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F494A0: 38EA000D  addi r7, r10, 0xd
	ctx.r[7].s64 = ctx.r[10].s64 + 13;
	// 82F494A4: 54EA2834  slwi r10, r7, 5
	ctx.r[10].u32 = ctx.r[7].u32.wrapping_shl(5);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F494A8: 7D4A5A14  add r10, r10, r11
	ctx.r[10].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F494AC: 7D4AE0AE  lbzx r10, r10, r28
	ctx.r[10].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82F494B0: 5549103E  rotlwi r9, r10, 2
	ctx.r[9].u64 = ((ctx.r[10].u32).rotate_left(2)) as u64;
	// 82F494B4: 7D2A4A14  add r9, r10, r9
	ctx.r[9].u64 = ctx.r[10].u64 + ctx.r[9].u64;
	// 82F494B8: 552A103A  slwi r10, r9, 2
	ctx.r[10].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F494BC: 7CEA5A14  add r7, r10, r11
	ctx.r[7].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F494C0: 816709A4  lwz r11, 0x9a4(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82F494C4: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F494C8: 9B610054  stb r27, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[27].u8 ) };
	// 82F494CC: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F494D0: 4E800421  bctrl
	ctx.lr = 0x82F494D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F494D4: 89410054  lbz r10, 0x54(r1)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(84 as u32) ) } as u64;
	// 82F494D8: 2B0A0000  cmplwi cr6, r10, 0
	ctx.cr[6].compare_u32(ctx.r[10].u32, 0 as u32, &mut ctx.xer);
	// 82F494DC: 419A0084  beq cr6, 0x82f49560
	if ctx.cr[6].eq {
	pc = 0x82F49560; continue 'dispatch;
	}
	// 82F494E0: 7D58C82E  lwzx r10, r24, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82F494E4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F494E8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F494EC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F494F0: 40980020  bge cr6, 0x82f49510
	if !ctx.cr[6].lt {
	pc = 0x82F49510; continue 'dispatch;
	}
	// 82F494F4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F494F8: 3909F834  addi r8, r9, -0x7cc
	ctx.r[8].s64 = ctx.r[9].s64 + -1996;
	// 82F494FC: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F49500: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F49504: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F49508: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F4950C: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F49510: 817D0018  lwz r11, 0x18(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F49514: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82F49518: 93610064  stw r27, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[27].u32 ) };
	// 82F4951C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F49520: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49524: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F49528: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F4952C: 91610060  stw r11, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u32 ) };
	// 82F49530: 816B000C  lwz r11, 0xc(r11)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F49534: 396B000D  addi r11, r11, 0xd
	ctx.r[11].s64 = ctx.r[11].s64 + 13;
	// 82F49538: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4953C: 7D2B5214  add r9, r11, r10
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F49540: 7D69E0AE  lbzx r11, r9, r28
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[9].u32.wrapping_add(ctx.r[28].u32)) } as u64;
	// 82F49544: 5569103E  rotlwi r9, r11, 2
	ctx.r[9].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F49548: 7D0B4A14  add r8, r11, r9
	ctx.r[8].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F4954C: 550B103A  slwi r11, r8, 2
	ctx.r[11].u32 = ctx.r[8].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F49550: 7CEB5214  add r7, r11, r10
	ctx.r[7].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F49554: 816709A4  lwz r11, 0x9a4(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(2468 as u32) ) } as u64;
	// 82F49558: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F4955C: 4E800421  bctrl
	ctx.lr = 0x82F49560;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F49560: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F49564: 7D58C82E  lwzx r10, r24, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82F49568: 392BC3C0  addi r9, r11, -0x3c40
	ctx.r[9].s64 = ctx.r[11].s64 + -15424;
	// 82F4956C: 91210050  stw r9, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[9].u32 ) };
	// 82F49570: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F49574: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F49578: 7F0B4040  cmplw cr6, r11, r8
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F4957C: 40980020  bge cr6, 0x82f4959c
	if !ctx.cr[6].lt {
	pc = 0x82F4959C; continue 'dispatch;
	}
	// 82F49580: 3D20820E  lis r9, -0x7df2
	ctx.r[9].s64 = -2113011712;
	// 82F49584: 390998B4  addi r8, r9, -0x674c
	ctx.r[8].s64 = ctx.r[9].s64 + -26444;
	// 82F49588: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F4958C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F49590: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F49594: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F49598: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F4959C: 382100C0  addi r1, r1, 0xc0
	ctx.r[1].s64 = ctx.r[1].s64 + 192;
	// 82F495A0: 4825EC08  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F495A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F495A8 size=108
    let mut pc: u32 = 0x82F495A8;
    'dispatch: loop {
        match pc {
            0x82F495A8 => {
    //   block [0x82F495A8..0x82F49614)
	// 82F495A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F495AC: 4825EBB9  bl 0x831a8164
	ctx.lr = 0x82F495B0;
	sub_831A8130(ctx, base);
	// 82F495B0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F495B4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F495B8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F495BC: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F495C0: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F495C4: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F495C8: 38800014  li r4, 0x14
	ctx.r[4].s64 = 20;
	// 82F495CC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F495D0: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F495D4: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F495D8: 4BF57159  bl 0x82ea0730
	ctx.lr = 0x82F495DC;
	sub_82EA0730(ctx, base);
	// 82F495DC: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F495E0: 39200014  li r9, 0x14
	ctx.r[9].s64 = 20;
	// 82F495E4: 7F87E378  mr r7, r28
	ctx.r[7].u64 = ctx.r[28].u64;
	// 82F495E8: 7FA6EB78  mr r6, r29
	ctx.r[6].u64 = ctx.r[29].u64;
	// 82F495EC: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F495F0: B13B0004  sth r9, 4(r27)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[27].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F495F4: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F495F8: 4BFFF1D9  bl 0x82f487d0
	ctx.lr = 0x82F495FC;
	sub_82F487D0(ctx, base);
	// 82F495FC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F49600: 7F63DB78  mr r3, r27
	ctx.r[3].u64 = ctx.r[27].u64;
	// 82F49604: 38E8FF90  addi r7, r8, -0x70
	ctx.r[7].s64 = ctx.r[8].s64 + -112;
	// 82F49608: 90FB0000  stw r7, 0(r27)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[27].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F4960C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F49610: 4825EBA4  b 0x831a81b4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49618(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F49618 size=204
    let mut pc: u32 = 0x82F49618;
    'dispatch: loop {
        match pc {
            0x82F49618 => {
    //   block [0x82F49618..0x82F496E4)
	// 82F49618: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4961C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F49620: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F49624: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F49628: 9421FF60  stwu r1, -0xa0(r1)
	ea = ctx.r[1].u32.wrapping_add(-160 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4962C: 3D6082F5  lis r11, -0x7d0b
	ctx.r[11].s64 = -2097872896;
	// 82F49630: 3D4082F5  lis r10, -0x7d0b
	ctx.r[10].s64 = -2097872896;
	// 82F49634: 3D2082F5  lis r9, -0x7d0b
	ctx.r[9].s64 = -2097872896;
	// 82F49638: 3D0082F5  lis r8, -0x7d0b
	ctx.r[8].s64 = -2097872896;
	// 82F4963C: 38CA96E8  addi r6, r10, -0x6918
	ctx.r[6].s64 = ctx.r[10].s64 + -26904;
	// 82F49640: 38A99730  addi r5, r9, -0x68d0
	ctx.r[5].s64 = ctx.r[9].s64 + -26832;
	// 82F49644: 38889780  addi r4, r8, -0x6880
	ctx.r[4].s64 = ctx.r[8].s64 + -26752;
	// 82F49648: 90C10054  stw r6, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[6].u32 ) };
	// 82F4964C: 3BE00001  li r31, 1
	ctx.r[31].s64 = 1;
	// 82F49650: 90A10058  stw r5, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[5].u32 ) };
	// 82F49654: 38EB95A8  addi r7, r11, -0x6a58
	ctx.r[7].s64 = ctx.r[11].s64 + -27224;
	// 82F49658: 9081005C  stw r4, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[4].u32 ) };
	// 82F4965C: 9BE10060  stb r31, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[31].u8 ) };
	// 82F49660: 38C0001A  li r6, 0x1a
	ctx.r[6].s64 = 26;
	// 82F49664: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F49668: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F4966C: 9BE10061  stb r31, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[31].u8 ) };
	// 82F49670: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F49674: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F49678: 4BFDBDC9  bl 0x82f25440
	ctx.lr = 0x82F4967C;
	sub_82F25440(ctx, base);
	// 82F4967C: 3C6082F5  lis r3, -0x7d0b
	ctx.r[3].s64 = -2097872896;
	// 82F49680: 3D6082F5  lis r11, -0x7d0b
	ctx.r[11].s64 = -2097872896;
	// 82F49684: 9BE10081  stb r31, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[31].u8 ) };
	// 82F49688: 3D4082F5  lis r10, -0x7d0b
	ctx.r[10].s64 = -2097872896;
	// 82F4968C: 3D2082F5  lis r9, -0x7d0b
	ctx.r[9].s64 = -2097872896;
	// 82F49690: 390388A0  addi r8, r3, -0x7760
	ctx.r[8].s64 = ctx.r[3].s64 + -30560;
	// 82F49694: 38CA9070  addi r6, r10, -0x6f90
	ctx.r[6].s64 = ctx.r[10].s64 + -28560;
	// 82F49698: 38A98CD0  addi r5, r9, -0x7330
	ctx.r[5].s64 = ctx.r[9].s64 + -29488;
	// 82F4969C: 91010070  stw r8, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[8].u32 ) };
	// 82F496A0: 38EB93F8  addi r7, r11, -0x6c08
	ctx.r[7].s64 = ctx.r[11].s64 + -27656;
	// 82F496A4: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82F496A8: 38800000  li r4, 0
	ctx.r[4].s64 = 0;
	// 82F496AC: 90A1007C  stw r5, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[5].u32 ) };
	// 82F496B0: 90E10074  stw r7, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[7].u32 ) };
	// 82F496B4: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82F496B8: 98810080  stb r4, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[4].u8 ) };
	// 82F496BC: 38A0001A  li r5, 0x1a
	ctx.r[5].s64 = 26;
	// 82F496C0: 38810070  addi r4, r1, 0x70
	ctx.r[4].s64 = ctx.r[1].s64 + 112;
	// 82F496C4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F496C8: 4BFDBD79  bl 0x82f25440
	ctx.lr = 0x82F496CC;
	sub_82F25440(ctx, base);
	// 82F496CC: 382100A0  addi r1, r1, 0xa0
	ctx.r[1].s64 = ctx.r[1].s64 + 160;
	// 82F496D0: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F496D4: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F496D8: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F496DC: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F496E0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F496E8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F496E8 size=72
    let mut pc: u32 = 0x82F496E8;
    'dispatch: loop {
        match pc {
            0x82F496E8 => {
    //   block [0x82F496E8..0x82F49730)
	// 82F496E8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F496EC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F496F0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F496F4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F496F8: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F496FC: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F49700: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F49704: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F49708: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F4970C: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F49710: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F49714: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F49718: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F4971C: 4BFFFCDD  bl 0x82f493f8
	ctx.lr = 0x82F49720;
	sub_82F493F8(ctx, base);
	// 82F49720: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F49724: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F49728: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4972C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F49730 size=76
    let mut pc: u32 = 0x82F49730;
    'dispatch: loop {
        match pc {
            0x82F49730 => {
    //   block [0x82F49730..0x82F4977C)
	// 82F49730: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F49734: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F49738: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4973C: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F49740: 90C10058  stw r6, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[6].u32 ) };
	// 82F49744: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F49748: 7C8B2378  mr r11, r4
	ctx.r[11].u64 = ctx.r[4].u64;
	// 82F4974C: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F49750: 7C641B78  mr r4, r3
	ctx.r[4].u64 = ctx.r[3].u64;
	// 82F49754: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F49758: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F4975C: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F49760: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F49764: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F49768: 4BFFF909  bl 0x82f49070
	ctx.lr = 0x82F4976C;
	sub_82F49070(ctx, base);
	// 82F4976C: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F49770: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F49774: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F49778: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49780(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F49780 size=224
    let mut pc: u32 = 0x82F49780;
    'dispatch: loop {
        match pc {
            0x82F49780 => {
    //   block [0x82F49780..0x82F49860)
	// 82F49780: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F49784: 4825E9E9  bl 0x831a816c
	ctx.lr = 0x82F49788;
	sub_831A8130(ctx, base);
	// 82F49788: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4978C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F49790: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F49794: 7CE83B78  mr r8, r7
	ctx.r[8].u64 = ctx.r[7].u64;
	// 82F49798: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F4979C: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F497A0: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F497A4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F497A8: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F497AC: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F497B0: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F497B4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F497B8: 4200FFF0  bdnz 0x82f497a8
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F497A8; continue 'dispatch;
	}
	// 82F497BC: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F497C0: E9250050  ld r9, 0x50(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(80 as u32) ) };
	// 82F497C4: E8E50058  ld r7, 0x58(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[5].u32.wrapping_add(88 as u32) ) };
	// 82F497C8: 39650050  addi r11, r5, 0x50
	ctx.r[11].s64 = ctx.r[5].s64 + 80;
	// 82F497CC: 38AAC5B0  addi r5, r10, -0x3a50
	ctx.r[5].s64 = ctx.r[10].s64 + -14928;
	// 82F497D0: 3C808201  lis r4, -0x7dff
	ctx.r[4].s64 = -2113863680;
	// 82F497D4: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F497D8: 3BC10060  addi r30, r1, 0x60
	ctx.r[30].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49860(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49860 size=16
    let mut pc: u32 = 0x82F49860;
    'dispatch: loop {
        match pc {
            0x82F49860 => {
    //   block [0x82F49860..0x82F49870)
	// 82F49860: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F49864: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F49868: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F4986C: 4BFFEEC4  b 0x82f48730
	sub_82F48730(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F49870 size=72
    let mut pc: u32 = 0x82F49870;
    'dispatch: loop {
        match pc {
            0x82F49870 => {
    //   block [0x82F49870..0x82F498B8)
	// 82F49870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F49874: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F49878: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4987C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F49880: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F49884: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F49888: 390AF400  addi r8, r10, -0xc00
	ctx.r[8].s64 = ctx.r[10].s64 + -3072;
	// 82F4988C: 99210054  stb r9, 0x54(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[9].u8 ) };
	// 82F49890: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F49894: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F49898: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F4989C: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F498A0: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F498A4: 4BFFF97D  bl 0x82f49220
	ctx.lr = 0x82F498A8;
	sub_82F49220(ctx, base);
	// 82F498A8: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F498AC: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F498B0: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F498B4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F498B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F498B8 size=76
    let mut pc: u32 = 0x82F498B8;
    'dispatch: loop {
        match pc {
            0x82F498B8 => {
    //   block [0x82F498B8..0x82F49904)
	// 82F498B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F498BC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F498C0: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F498C4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F498C8: 90E10058  stw r7, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[7].u32 ) };
	// 82F498CC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F498D0: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F498D4: 3909F3E8  addi r8, r9, -0xc18
	ctx.r[8].s64 = ctx.r[9].s64 + -3096;
	// 82F498D8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F498DC: C00ABA78  lfs f0, -0x4588(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F498E0: 91010050  stw r8, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[8].u32 ) };
	// 82F498E4: D0010054  stfs f0, 0x54(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), tmp.u32 ) };
	// 82F498E8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F498EC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F498F0: 4BFFF5A9  bl 0x82f48e98
	ctx.lr = 0x82F498F4;
	sub_82F48E98(ctx, base);
	// 82F498F4: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F498F8: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F498FC: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F49900: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49908(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F49908 size=176
    let mut pc: u32 = 0x82F49908;
    'dispatch: loop {
        match pc {
            0x82F49908 => {
    //   block [0x82F49908..0x82F499B8)
	// 82F49908: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4990C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F49910: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F49914: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F49918: DBE1FFE0  stfd f31, -0x20(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-32 as u32), ctx.f[31].u64 ) };
	// 82F4991C: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F49920: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F49924: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F49928: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F4992C: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F49930: 83FE0000  lwz r31, 0(r30)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49934: C3FE3030  lfs f31, 0x3030(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(12336 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F49938: 4BFFEFC1  bl 0x82f488f8
	ctx.lr = 0x82F4993C;
	sub_82F488F8(ctx, base);
	// 82F4993C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49940: 7F1F5040  cmplw cr6, r31, r10
	ctx.cr[6].compare_u32(ctx.r[31].u32, ctx.r[10].u32, &mut ctx.xer);
	// 82F49944: 40980044  bge cr6, 0x82f49988
	if !ctx.cr[6].lt {
	pc = 0x82F49988; continue 'dispatch;
	}
	// 82F49948: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F4994C: 394BC5A0  addi r10, r11, -0x3a60
	ctx.r[10].s64 = ctx.r[11].s64 + -14944;
	// 82F49950: 397F0010  addi r11, r31, 0x10
	ctx.r[11].s64 = ctx.r[31].s64 + 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F499B8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F499B8 size=224
    let mut pc: u32 = 0x82F499B8;
    'dispatch: loop {
        match pc {
            0x82F499B8 => {
    //   block [0x82F499B8..0x82F49A98)
	// 82F499B8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F499BC: 4825E7AD  bl 0x831a8168
	ctx.lr = 0x82F499C0;
	sub_831A8130(ctx, base);
	// 82F499C0: 9421FEB0  stwu r1, -0x150(r1)
	ea = ctx.r[1].u32.wrapping_add(-336 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F499C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F499C8: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F499CC: 7D1F4378  mr r31, r8
	ctx.r[31].u64 = ctx.r[8].u64;
	// 82F499D0: 394100B0  addi r10, r1, 0xb0
	ctx.r[10].s64 = ctx.r[1].s64 + 176;
	// 82F499D4: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82F499D8: 3920000E  li r9, 0xe
	ctx.r[9].s64 = 14;
	// 82F499DC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F499E0: E92B0000  ld r9, 0(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) };
	// 82F499E4: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F499E8: F92A0000  std r9, 0(r10)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[9].u64 ) };
	// 82F499EC: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F499F0: 4200FFF0  bdnz 0x82f499e0
	ctx.ctr.u64 = ctx.ctr.u64.wrapping_sub(1);
	if ctx.ctr.u32 != 0 {
			pc = 0x82F499E0; continue 'dispatch;
	}
	// 82F499F4: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F499F8: E9260050  ld r9, 0x50(r6)
	ctx.r[9].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(80 as u32) ) };
	// 82F499FC: E9060058  ld r8, 0x58(r6)
	ctx.r[8].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	// 82F49A00: 39660050  addi r11, r6, 0x50
	ctx.r[11].s64 = ctx.r[6].s64 + 80;
	// 82F49A04: 38CAC5B0  addi r6, r10, -0x3a50
	ctx.r[6].s64 = ctx.r[10].s64 + -14928;
	// 82F49A08: 3CA08201  lis r5, -0x7dff
	ctx.r[5].s64 = -2113863680;
	// 82F49A0C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F49A10: 3BA10060  addi r29, r1, 0x60
	ctx.r[29].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49A98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49A98 size=4
    let mut pc: u32 = 0x82F49A98;
    'dispatch: loop {
        match pc {
            0x82F49A98 => {
    //   block [0x82F49A98..0x82F49A9C)
	// 82F49A98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49AA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49AA0 size=4
    let mut pc: u32 = 0x82F49AA0;
    'dispatch: loop {
        match pc {
            0x82F49AA0 => {
    //   block [0x82F49AA0..0x82F49AA4)
	// 82F49AA0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49AA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49AA8 size=24
    let mut pc: u32 = 0x82F49AA8;
    'dispatch: loop {
        match pc {
            0x82F49AA8 => {
    //   block [0x82F49AA8..0x82F49AC0)
	// 82F49AA8: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49AAC: 7CC43378  mr r4, r6
	ctx.r[4].u64 = ctx.r[6].u64;
	// 82F49AB0: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49AB4: 814B001C  lwz r10, 0x1c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F49AB8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F49ABC: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49AC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49AC0 size=40
    let mut pc: u32 = 0x82F49AC0;
    'dispatch: loop {
        match pc {
            0x82F49AC0 => {
    //   block [0x82F49AC0..0x82F49AE8)
	// 82F49AC0: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49AC4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F49AC8: 7CC53378  mr r5, r6
	ctx.r[5].u64 = ctx.r[6].u64;
	// 82F49ACC: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F49AD0: 7D274B78  mr r7, r9
	ctx.r[7].u64 = ctx.r[9].u64;
	// 82F49AD4: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F49AD8: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49ADC: 812A0020  lwz r9, 0x20(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F49AE0: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F49AE4: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49AE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49AE8 size=40
    let mut pc: u32 = 0x82F49AE8;
    'dispatch: loop {
        match pc {
            0x82F49AE8 => {
    //   block [0x82F49AE8..0x82F49B10)
	// 82F49AE8: 7CCB3378  mr r11, r6
	ctx.r[11].u64 = ctx.r[6].u64;
	// 82F49AEC: 80C40000  lwz r6, 0(r4)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49AF0: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F49AF4: 38A00008  li r5, 8
	ctx.r[5].s64 = 8;
	// 82F49AF8: 3889FFC8  addi r4, r9, -0x38
	ctx.r[4].s64 = ctx.r[9].s64 + -56;
	// 82F49AFC: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F49B00: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49B04: 810A000C  lwz r8, 0xc(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F49B08: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F49B0C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49B10(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49B10 size=24
    let mut pc: u32 = 0x82F49B10;
    'dispatch: loop {
        match pc {
            0x82F49B10 => {
    //   block [0x82F49B10..0x82F49B28)
	// 82F49B10: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49B14: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F49B18: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49B1C: 814B0024  lwz r10, 0x24(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F49B20: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F49B24: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49B28(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49B28 size=24
    let mut pc: u32 = 0x82F49B28;
    'dispatch: loop {
        match pc {
            0x82F49B28 => {
    //   block [0x82F49B28..0x82F49B40)
	// 82F49B28: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49B2C: 7CE63B78  mr r6, r7
	ctx.r[6].u64 = ctx.r[7].u64;
	// 82F49B30: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49B34: 814B0028  lwz r10, 0x28(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F49B38: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F49B3C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49B40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49B40 size=24
    let mut pc: u32 = 0x82F49B40;
    'dispatch: loop {
        match pc {
            0x82F49B40 => {
    //   block [0x82F49B40..0x82F49B58)
	// 82F49B40: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49B44: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F49B48: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49B4C: 814B002C  lwz r10, 0x2c(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(44 as u32) ) } as u64;
	// 82F49B50: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F49B54: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49B58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49B58 size=24
    let mut pc: u32 = 0x82F49B58;
    'dispatch: loop {
        match pc {
            0x82F49B58 => {
    //   block [0x82F49B58..0x82F49B70)
	// 82F49B58: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49B5C: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F49B60: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49B64: 814B0030  lwz r10, 0x30(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F49B68: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F49B6C: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49B70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49B70 size=24
    let mut pc: u32 = 0x82F49B70;
    'dispatch: loop {
        match pc {
            0x82F49B70 => {
    //   block [0x82F49B70..0x82F49B88)
	// 82F49B70: 80640000  lwz r3, 0(r4)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49B74: 7CA42B78  mr r4, r5
	ctx.r[4].u64 = ctx.r[5].u64;
	// 82F49B78: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49B7C: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F49B80: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F49B84: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49B88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F49B88 size=176
    let mut pc: u32 = 0x82F49B88;
    'dispatch: loop {
        match pc {
            0x82F49B88 => {
    //   block [0x82F49B88..0x82F49C38)
	// 82F49B88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F49B8C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F49B90: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F49B94: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F49B98: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F49B9C: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82F49BA0: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F49BA4: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F49BA8: 806B0000  lwz r3, 0(r11)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49BAC: 808B0004  lwz r4, 4(r11)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F49BB0: 80AB0008  lwz r5, 8(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F49BB4: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F49BB8: 81030000  lwz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49BBC: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49BC0: 80E50010  lwz r7, 0x10(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F49BC4: 81250000  lwz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49BC8: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82F49BCC: 8168000C  lwz r11, 0xc(r8)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F49BD0: 390905A0  addi r8, r9, 0x5a0
	ctx.r[8].s64 = ctx.r[9].s64 + 1440;
	// 82F49BD4: 814A000C  lwz r10, 0xc(r10)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F49BD8: 409A0008  bne cr6, 0x82f49be0
	if !ctx.cr[6].eq {
	pc = 0x82F49BE0; continue 'dispatch;
	}
	// 82F49BDC: 390901A0  addi r8, r9, 0x1a0
	ctx.r[8].s64 = ctx.r[9].s64 + 416;
	// 82F49BE0: 556B2834  slwi r11, r11, 5
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(5);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F49BE4: 7D6B4214  add r11, r11, r8
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[8].u64;
	// 82F49BE8: 7D6B50AE  lbzx r11, r11, r10
	ctx.r[11].u64 = unsafe { crate::rt::load_u8(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F49BEC: 556A103E  rotlwi r10, r11, 2
	ctx.r[10].u64 = ((ctx.r[11].u32).rotate_left(2)) as u64;
	// 82F49BF0: 7D4B5214  add r10, r11, r10
	ctx.r[10].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F49BF4: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F49BF8: 7D2B4A14  add r9, r11, r9
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[9].u64;
	// 82F49BFC: 810909A0  lwz r8, 0x9a0(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(2464 as u32) ) } as u64;
	// 82F49C00: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F49C04: 4E800421  bctrl
	ctx.lr = 0x82F49C08;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F49C08: 38E00006  li r7, 6
	ctx.r[7].s64 = 6;
	// 82F49C0C: 38C000FF  li r6, 0xff
	ctx.r[6].s64 = 255;
	// 82F49C10: 907E0000  stw r3, 0(r30)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[30].u32.wrapping_add(0 as u32), ctx.r[3].u32 ) };
	// 82F49C14: 98FF0000  stb r7, 0(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82F49C18: 387E0010  addi r3, r30, 0x10
	ctx.r[3].s64 = ctx.r[30].s64 + 16;
	// 82F49C1C: 98DF0002  stb r6, 2(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(2 as u32), ctx.r[6].u8 ) };
	// 82F49C20: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F49C24: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F49C28: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F49C2C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F49C30: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F49C34: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49C38(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F49C38 size=80
    let mut pc: u32 = 0x82F49C38;
    'dispatch: loop {
        match pc {
            0x82F49C38 => {
    //   block [0x82F49C38..0x82F49C88)
	// 82F49C38: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F49C3C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F49C40: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F49C44: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F49C48: 7CBF2B78  mr r31, r5
	ctx.r[31].u64 = ctx.r[5].u64;
	// 82F49C4C: 80C30008  lwz r6, 8(r3)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F49C50: 80A30004  lwz r5, 4(r3)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F49C54: 80830000  lwz r4, 0(r3)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49C58: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49C5C: 7D635B78  mr r3, r11
	ctx.r[3].u64 = ctx.r[11].u64;
	// 82F49C60: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49C64: 812A0018  lwz r9, 0x18(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F49C68: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F49C6C: 4E800421  bctrl
	ctx.lr = 0x82F49C70;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F49C70: 387F0010  addi r3, r31, 0x10
	ctx.r[3].s64 = ctx.r[31].s64 + 16;
	// 82F49C74: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F49C78: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F49C7C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F49C80: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F49C84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49C88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F49C88 size=184
    let mut pc: u32 = 0x82F49C88;
    'dispatch: loop {
        match pc {
            0x82F49C88 => {
    //   block [0x82F49C88..0x82F49D40)
	// 82F49C88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F49C8C: 4825E4E1  bl 0x831a816c
	ctx.lr = 0x82F49C90;
	sub_831A8130(ctx, base);
	// 82F49C90: 9421FF50  stwu r1, -0xb0(r1)
	ea = ctx.r[1].u32.wrapping_add(-176 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F49C94: 3D4082F5  lis r10, -0x7d0b
	ctx.r[10].s64 = -2097872896;
	// 82F49C98: 3D2082F5  lis r9, -0x7d0b
	ctx.r[9].s64 = -2097872896;
	// 82F49C9C: 3D0082F5  lis r8, -0x7d0b
	ctx.r[8].s64 = -2097872896;
	// 82F49CA0: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 82F49CA4: 394A9B58  addi r10, r10, -0x64a8
	ctx.r[10].s64 = ctx.r[10].s64 + -25768;
	// 82F49CA8: 39299B70  addi r9, r9, -0x6490
	ctx.r[9].s64 = ctx.r[9].s64 + -25744;
	// 82F49CAC: 39089B40  addi r8, r8, -0x64c0
	ctx.r[8].s64 = ctx.r[8].s64 + -25792;
	// 82F49CB0: 91410060  stw r10, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[10].u32 ) };
	// 82F49CB4: 38E79B88  addi r7, r7, -0x6478
	ctx.r[7].s64 = ctx.r[7].s64 + -25720;
	// 82F49CB8: 91210064  stw r9, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[9].u32 ) };
	// 82F49CBC: 3CC082F5  lis r6, -0x7d0b
	ctx.r[6].s64 = -2097872896;
	// 82F49CC0: 9101005C  stw r8, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[8].u32 ) };
	// 82F49CC4: 3CA082F5  lis r5, -0x7d0b
	ctx.r[5].s64 = -2097872896;
	// 82F49CC8: 90E10050  stw r7, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[7].u32 ) };
	// 82F49CCC: 3C8082F5  lis r4, -0x7d0b
	ctx.r[4].s64 = -2097872896;
	// 82F49CD0: 3FE082F5  lis r31, -0x7d0b
	ctx.r[31].s64 = -2097872896;
	// 82F49CD4: 3FC082F5  lis r30, -0x7d0b
	ctx.r[30].s64 = -2097872896;
	// 82F49CD8: 3FA082F5  lis r29, -0x7d0b
	ctx.r[29].s64 = -2097872896;
	// 82F49CDC: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F49CE0: 38C69C38  addi r6, r6, -0x63c8
	ctx.r[6].s64 = ctx.r[6].s64 + -25544;
	// 82F49CE4: 38A59AA8  addi r5, r5, -0x6558
	ctx.r[5].s64 = ctx.r[5].s64 + -25944;
	// 82F49CE8: 99610081  stb r11, 0x81(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(129 as u32), ctx.r[11].u8 ) };
	// 82F49CEC: 38849AC0  addi r4, r4, -0x6540
	ctx.r[4].s64 = ctx.r[4].s64 + -25920;
	// 82F49CF0: 90C1007C  stw r6, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[6].u32 ) };
	// 82F49CF4: 395F9AE8  addi r10, r31, -0x6518
	ctx.r[10].s64 = ctx.r[31].s64 + -25880;
	// 82F49CF8: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82F49CFC: 393E9B10  addi r9, r30, -0x64f0
	ctx.r[9].s64 = ctx.r[30].s64 + -25840;
	// 82F49D00: 90810068  stw r4, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[4].u32 ) };
	// 82F49D04: 391D9B28  addi r8, r29, -0x64d8
	ctx.r[8].s64 = ctx.r[29].s64 + -25816;
	// 82F49D08: 99610082  stb r11, 0x82(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(130 as u32), ctx.r[11].u8 ) };
	// 82F49D0C: 38E00001  li r7, 1
	ctx.r[7].s64 = 1;
	// 82F49D10: 91610078  stw r11, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[11].u32 ) };
	// 82F49D14: 91610058  stw r11, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[11].u32 ) };
	// 82F49D18: 38C0FFFF  li r6, -1
	ctx.r[6].s64 = -1;
	// 82F49D1C: 9141006C  stw r10, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[10].u32 ) };
	// 82F49D20: 38A0FFFF  li r5, -1
	ctx.r[5].s64 = -1;
	// 82F49D24: 91210070  stw r9, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[9].u32 ) };
	// 82F49D28: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F49D2C: 91010074  stw r8, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[8].u32 ) };
	// 82F49D30: 98E10080  stb r7, 0x80(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(128 as u32), ctx.r[7].u8 ) };
	// 82F49D34: 4BFDB8ED  bl 0x82f25620
	ctx.lr = 0x82F49D38;
	sub_82F25620(ctx, base);
	// 82F49D38: 382100B0  addi r1, r1, 0xb0
	ctx.r[1].s64 = ctx.r[1].s64 + 176;
	// 82F49D3C: 4825E480  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49D40(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49D40 size=4
    let mut pc: u32 = 0x82F49D40;
    'dispatch: loop {
        match pc {
            0x82F49D40 => {
    //   block [0x82F49D40..0x82F49D44)
	// 82F49D40: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49D48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49D48 size=4
    let mut pc: u32 = 0x82F49D48;
    'dispatch: loop {
        match pc {
            0x82F49D48 => {
    //   block [0x82F49D48..0x82F49D4C)
	// 82F49D48: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49D50(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49D50 size=4
    let mut pc: u32 = 0x82F49D50;
    'dispatch: loop {
        match pc {
            0x82F49D50 => {
    //   block [0x82F49D50..0x82F49D54)
	// 82F49D50: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49D58(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49D58 size=4
    let mut pc: u32 = 0x82F49D58;
    'dispatch: loop {
        match pc {
            0x82F49D58 => {
    //   block [0x82F49D58..0x82F49D5C)
	// 82F49D58: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49D60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49D60 size=4
    let mut pc: u32 = 0x82F49D60;
    'dispatch: loop {
        match pc {
            0x82F49D60 => {
    //   block [0x82F49D60..0x82F49D64)
	// 82F49D60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49D68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49D68 size=40
    let mut pc: u32 = 0x82F49D68;
    'dispatch: loop {
        match pc {
            0x82F49D68 => {
    //   block [0x82F49D68..0x82F49D90)
	// 82F49D68: 81440010  lwz r10, 0x10(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F49D6C: 81640014  lwz r11, 0x14(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F49D70: 7F0B5000  cmpw cr6, r11, r10
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[10].s32, &mut ctx.xer);
	// 82F49D74: 4098001C  bge cr6, 0x82f49d90
	if !ctx.cr[6].lt {
		sub_82F49D90(ctx, base);
		return;
	}
	// 82F49D78: 8144000C  lwz r10, 0xc(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F49D7C: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F49D80: 392A0030  addi r9, r10, 0x30
	ctx.r[9].s64 = ctx.r[10].s64 + 48;
	// 82F49D84: 91640014  stw r11, 0x14(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82F49D88: 9124000C  stw r9, 0xc(r4)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(12 as u32), ctx.r[9].u32 ) };
	// 82F49D8C: 4800004C  b 0x82f49dd8
	sub_82F49DD8(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49D90(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F49D90 size=72
    let mut pc: u32 = 0x82F49D90;
    'dispatch: loop {
        match pc {
            0x82F49D90 => {
    //   block [0x82F49D90..0x82F49DD8)
	// 82F49D90: 81440008  lwz r10, 8(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F49D94: 2F0B0001  cmpwi cr6, r11, 1
	ctx.cr[6].compare_i32(ctx.r[11].s32, 1, &mut ctx.xer);
	// 82F49D98: 396A0030  addi r11, r10, 0x30
	ctx.r[11].s64 = ctx.r[10].s64 + 48;
	// 82F49D9C: 4099002C  ble cr6, 0x82f49dc8
	if !ctx.cr[6].gt {
	pc = 0x82F49DC8; continue 'dispatch;
	}
	// 82F49DA0: 81240014  lwz r9, 0x14(r4)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F49DA4: 3929FFFF  addi r9, r9, -1
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	// 82F49DA8: C00B001C  lfs f0, 0x1c(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F49DAC: C1AA001C  lfs f13, 0x1c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F49DB0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82F49DB4: 40990008  ble cr6, 0x82f49dbc
	if !ctx.cr[6].gt {
	pc = 0x82F49DBC; continue 'dispatch;
	}
	// 82F49DB8: 7D6A5B78  mr r10, r11
	ctx.r[10].u64 = ctx.r[11].u64;
	// 82F49DBC: 3529FFFF  addic. r9, r9, -1
	ctx.xer.ca = (ctx.r[9].u32 > (!(-1 as u32)));
	ctx.r[9].s64 = ctx.r[9].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F49DC0: 396B0030  addi r11, r11, 0x30
	ctx.r[11].s64 = ctx.r[11].s64 + 48;
	// 82F49DC4: 4082FFE4  bne 0x82f49da8
	if !ctx.cr[0].eq {
	pc = 0x82F49DA8; continue 'dispatch;
	}
	// 82F49DC8: C003001C  lfs f0, 0x1c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F49DCC: C1AA001C  lfs f13, 0x1c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F49DD0: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82F49DD4: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49DD8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49DD8 size=132
    let mut pc: u32 = 0x82F49DD8;
    'dispatch: loop {
        match pc {
            0x82F49DD8 => {
    //   block [0x82F49DD8..0x82F49E5C)
	// 82F49DD8: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49E5C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F49E5C size=8
    let mut pc: u32 = 0x82F49E5C;
    'dispatch: loop {
        match pc {
            0x82F49E5C => {
    //   block [0x82F49E5C..0x82F49E64)
	// 82F49E5C: D0040004  stfs f0, 4(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[4].u32.wrapping_add(4 as u32), tmp.u32 ) };
	// 82F49E60: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49E68(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49E68 size=20
    let mut pc: u32 = 0x82F49E68;
    'dispatch: loop {
        match pc {
            0x82F49E68 => {
    //   block [0x82F49E68..0x82F49E7C)
	// 82F49E68: 3D6082F5  lis r11, -0x7d0b
	ctx.r[11].s64 = -2097872896;
	// 82F49E6C: 3D408339  lis r10, -0x7cc7
	ctx.r[10].s64 = -2093416448;
	// 82F49E70: 396B9D68  addi r11, r11, -0x6298
	ctx.r[11].s64 = ctx.r[11].s64 + -25240;
	// 82F49E74: 916AA2C4  stw r11, -0x5d3c(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(-23868 as u32), ctx.r[11].u32 ) };
	// 82F49E78: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49E80(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49E80 size=8
    let mut pc: u32 = 0x82F49E80;
    'dispatch: loop {
        match pc {
            0x82F49E80 => {
    //   block [0x82F49E80..0x82F49E88)
	// 82F49E80: 8063001C  lwz r3, 0x1c(r3)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F49E84: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49E88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49E88 size=12
    let mut pc: u32 = 0x82F49E88;
    'dispatch: loop {
        match pc {
            0x82F49E88 => {
    //   block [0x82F49E88..0x82F49E94)
	// 82F49E88: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F49E8C: 38630014  addi r3, r3, 0x14
	ctx.r[3].s64 = ctx.r[3].s64 + 20;
	// 82F49E90: 4C9A0020  bnelr cr6
	if !ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49E94(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49E94 size=8
    let mut pc: u32 = 0x82F49E94;
    'dispatch: loop {
        match pc {
            0x82F49E94 => {
    //   block [0x82F49E94..0x82F49E9C)
	// 82F49E94: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F49E98: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49EA0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49EA0 size=8
    let mut pc: u32 = 0x82F49EA0;
    'dispatch: loop {
        match pc {
            0x82F49EA0 => {
    //   block [0x82F49EA0..0x82F49EA8)
	// 82F49EA0: 38600000  li r3, 0
	ctx.r[3].s64 = 0;
	// 82F49EA4: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49EA8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49EA8 size=16
    let mut pc: u32 = 0x82F49EA8;
    'dispatch: loop {
        match pc {
            0x82F49EA8 => {
    //   block [0x82F49EA8..0x82F49EB8)
	// 82F49EA8: 81630014  lwz r11, 0x14(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F49EAC: 38640001  addi r3, r4, 1
	ctx.r[3].s64 = ctx.r[4].s64 + 1;
	// 82F49EB0: 7F035800  cmpw cr6, r3, r11
	ctx.cr[6].compare_i32(ctx.r[3].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F49EB4: 4D980020  bltlr cr6
	if ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49EB8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49EB8 size=8
    let mut pc: u32 = 0x82F49EB8;
    'dispatch: loop {
        match pc {
            0x82F49EB8 => {
    //   block [0x82F49EB8..0x82F49EC0)
	// 82F49EB8: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82F49EBC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49EC0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49EC0 size=36
    let mut pc: u32 = 0x82F49EC0;
    'dispatch: loop {
        match pc {
            0x82F49EC0 => {
    //   block [0x82F49EC0..0x82F49EE4)
	// 82F49EC0: 8163000C  lwz r11, 0xc(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F49EC4: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F49EC8: 81230010  lwz r9, 0x10(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F49ECC: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F49ED0: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F49ED4: 7C8A482E  lwzx r4, r10, r9
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[9].u32)) } as u64;
	// 82F49ED8: 80E80014  lwz r7, 0x14(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F49EDC: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F49EE0: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49EE8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49EE8 size=16
    let mut pc: u32 = 0x82F49EE8;
    'dispatch: loop {
        match pc {
            0x82F49EE8 => {
    //   block [0x82F49EE8..0x82F49EF8)
	// 82F49EE8: 81630018  lwz r11, 0x18(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49EF8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49EF8 size=28
    let mut pc: u32 = 0x82F49EF8;
    'dispatch: loop {
        match pc {
            0x82F49EF8 => {
    //   block [0x82F49EF8..0x82F49F14)
	// 82F49EF8: 7C6A1B78  mr r10, r3
	ctx.r[10].u64 = ctx.r[3].u64;
	// 82F49EFC: 7C832378  mr r3, r4
	ctx.r[3].u64 = ctx.r[4].u64;
	// 82F49F00: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F49F04: 7C6B1B78  mr r11, r3
	ctx.r[11].u64 = ctx.r[3].u64;
	// 82F49F08: 810A001C  lwz r8, 0x1c(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(28 as u32) ) } as u64;
	// 82F49F0C: 2F080000  cmpwi cr6, r8, 0
	ctx.cr[6].compare_i32(ctx.r[8].s32, 0, &mut ctx.xer);
	// 82F49F10: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49F14(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F49F14 size=52
    let mut pc: u32 = 0x82F49F14;
    'dispatch: loop {
        match pc {
            0x82F49F14 => {
    //   block [0x82F49F14..0x82F49F48)
	// 82F49F14: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82F49F18: 80EA0018  lwz r7, 0x18(r10)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(24 as u32) ) } as u64;
	// 82F49F1C: 39290001  addi r9, r9, 1
	ctx.r[9].s64 = ctx.r[9].s64 + 1;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49F48(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F49F48 size=168
    let mut pc: u32 = 0x82F49F48;
    'dispatch: loop {
        match pc {
            0x82F49F48 => {
    //   block [0x82F49F48..0x82F49FF0)
	// 82F49F48: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F49F4C: 4825E211  bl 0x831a815c
	ctx.lr = 0x82F49F50;
	sub_831A8130(ctx, base);
	// 82F49F50: 9421FD70  stwu r1, -0x290(r1)
	ea = ctx.r[1].u32.wrapping_add(-656 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F49F54: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F49F58: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F49F5C: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F49F60: 37A5FFFF  addic. r29, r5, -1
	ctx.xer.ca = (ctx.r[5].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[5].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82F49F64: 41800084  blt 0x82f49fe8
	if ctx.cr[0].lt {
	pc = 0x82F49FE8; continue 'dispatch;
	}
	// 82F49F68: 3D605555  lis r11, 0x5555
	ctx.r[11].s64 = 1431633920;
	// 82F49F6C: 3B200003  li r25, 3
	ctx.r[25].s64 = 3;
	// 82F49F70: 617A5556  ori r26, r11, 0x5556
	ctx.r[26].u64 = ctx.r[11].u64 | 21846;
	// 82F49F74: 817B0020  lwz r11, 0x20(r27)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F49F78: 38A10050  addi r5, r1, 0x50
	ctx.r[5].s64 = ctx.r[1].s64 + 80;
	// 82F49F7C: A3DC0000  lhz r30, 0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F49F80: 815B0024  lwz r10, 0x24(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F49F84: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F49F88: 7D3ECBD6  divw r9, r30, r25
	ctx.r[9].s32 = ctx.r[30].s32 / ctx.r[25].s32;
	// 82F49F8C: 810B0010  lwz r8, 0x10(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F49F90: 5527103A  slwi r7, r9, 2
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shl(2);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F49F94: 7C87502E  lwzx r4, r7, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[7].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F49F98: 80C80014  lwz r6, 0x14(r8)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F49F9C: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82F49FA0: 4E800421  bctrl
	ctx.lr = 0x82F49FA4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F49FA4: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F49FA8: 67C43F00  oris r4, r30, 0x3f00
	ctx.r[4].u64 = ctx.r[30].u64 | 1056964608;
	// 82F49FAC: 7D65D096  mulhw r11, r5, r26
	ctx.r[11].s64 = ((ctx.r[5].s32 as i64 * ctx.r[26].s32 as i64) >> 32);
	// 82F49FB0: 556A0FFE  srwi r10, r11, 0x1f
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shr(31);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F49FB4: 37BDFFFF  addic. r29, r29, -1
	ctx.xer.ca = (ctx.r[29].u32 > (!(-1 as u32)));
	ctx.r[29].s64 = ctx.r[29].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[29].s32, 0, &mut ctx.xer);
	// 82F49FB8: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F49FBC: 3B9C0002  addi r28, r28, 2
	ctx.r[28].s64 = ctx.r[28].s64 + 2;
	// 82F49FC0: 556A083C  slwi r10, r11, 1
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F49FC4: 7D6B5214  add r11, r11, r10
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[10].u64;
	// 82F49FC8: 7D6B2850  subf r11, r11, r5
	ctx.r[11].s64 = ctx.r[5].s64 - ctx.r[11].s64;
	// 82F49FCC: 394B0002  addi r10, r11, 2
	ctx.r[10].s64 = ctx.r[11].s64 + 2;
	// 82F49FD0: 55492036  slwi r9, r10, 4
	ctx.r[9].u32 = ctx.r[10].u32.wrapping_shl(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F49FF0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F49FF0 size=68
    let mut pc: u32 = 0x82F49FF0;
    'dispatch: loop {
        match pc {
            0x82F49FF0 => {
    //   block [0x82F49FF0..0x82F4A034)
	// 82F49FF0: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F49FF4: D0230010  stfs f1, 0x10(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.f32 = (ctx.f[1].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), tmp.u32 ) };
	// 82F49FF8: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F49FFC: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4A000: 39000001  li r8, 1
	ctx.r[8].s64 = 1;
	// 82F4A004: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F4A008: 38C00013  li r6, 0x13
	ctx.r[6].s64 = 19;
	// 82F4A00C: B1030006  sth r8, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[8].u16 ) };
	// 82F4A010: 38ABBCD8  addi r5, r11, -0x4328
	ctx.r[5].s64 = ctx.r[11].s64 + -17192;
	// 82F4A014: 90E30008  stw r7, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[7].u32 ) };
	// 82F4A018: 388AFFFC  addi r4, r10, -4
	ctx.r[4].s64 = ctx.r[10].s64 + -4;
	// 82F4A01C: 90C3000C  stw r6, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F4A020: 3969FFD8  addi r11, r9, -0x28
	ctx.r[11].s64 = ctx.r[9].s64 + -40;
	// 82F4A024: 90A30014  stw r5, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[5].u32 ) };
	// 82F4A028: 90830000  stw r4, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[4].u32 ) };
	// 82F4A02C: 91630014  stw r11, 0x14(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(20 as u32), ctx.r[11].u32 ) };
	// 82F4A030: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A038(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4A038 size=244
    let mut pc: u32 = 0x82F4A038;
    'dispatch: loop {
        match pc {
            0x82F4A038 => {
    //   block [0x82F4A038..0x82F4A12C)
	// 82F4A038: FBE1FFF8  std r31, -8(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[31].u64 ) };
	// 82F4A03C: 3D608201  lis r11, -0x7dff
	ctx.r[11].s64 = -2113863680;
	// 82F4A040: 3D008212  lis r8, -0x7dee
	ctx.r[8].s64 = -2112749568;
	// 82F4A044: 39460010  addi r10, r6, 0x10
	ctx.r[10].s64 = ctx.r[6].s64 + 16;
	// 82F4A048: 39200000  li r9, 0
	ctx.r[9].s64 = 0;
	// 82F4A04C: C00BBA78  lfs f0, -0x4588(r11)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(-17800 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4A050: D0060000  stfs f0, 0(r6)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[6].u32.wrapping_add(0 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A130(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4A130 size=240
    let mut pc: u32 = 0x82F4A130;
    'dispatch: loop {
        match pc {
            0x82F4A130 => {
    //   block [0x82F4A130..0x82F4A220)
	// 82F4A130: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4A134: 4825E021  bl 0x831a8154
	ctx.lr = 0x82F4A138;
	sub_831A8130(ctx, base);
	// 82F4A138: DBE1FFA8  stfd f31, -0x58(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-88 as u32), ctx.f[31].u64 ) };
	// 82F4A13C: 9421FD30  stwu r1, -0x2d0(r1)
	ea = ctx.r[1].u32.wrapping_add(-720 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4A140: 3D608212  lis r11, -0x7dee
	ctx.r[11].s64 = -2112749568;
	// 82F4A144: 81430028  lwz r10, 0x28(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F4A148: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F4A14C: 7CB72B78  mr r23, r5
	ctx.r[23].u64 = ctx.r[5].u64;
	// 82F4A150: 3B230028  addi r25, r3, 0x28
	ctx.r[25].s64 = ctx.r[3].s64 + 40;
	// 82F4A154: 3B000000  li r24, 0
	ctx.r[24].s64 = 0;
	// 82F4A158: C3EB3EF8  lfs f31, 0x3ef8(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16120 as u32) ) };
	ctx.f[31].f64 = (tmp.f32 as f64);
	// 82F4A15C: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82F4A160: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F4A164: 409900A8  ble cr6, 0x82f4a20c
	if !ctx.cr[6].gt {
	pc = 0x82F4A20C; continue 'dispatch;
	}
	// 82F4A168: 3B630024  addi r27, r3, 0x24
	ctx.r[27].s64 = ctx.r[3].s64 + 36;
	// 82F4A16C: 3B430020  addi r26, r3, 0x20
	ctx.r[26].s64 = ctx.r[3].s64 + 32;
	// 82F4A170: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F4A174: 3BE00000  li r31, 0
	ctx.r[31].s64 = 0;
	// 82F4A178: 817A0000  lwz r11, 0(r26)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A17C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F4A180: 815B0000  lwz r10, 0(r27)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A184: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F4A188: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4A18C: 7C9F502E  lwzx r4, r31, r10
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[31].u32.wrapping_add(ctx.r[10].u32)) } as u64;
	// 82F4A190: 81090014  lwz r8, 0x14(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F4A194: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F4A198: 4E800421  bctrl
	ctx.lr = 0x82F4A19C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4A19C: 80E30000  lwz r7, 0(r3)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A1A0: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F4A1A4: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F4A1A8: 80C70030  lwz r6, 0x30(r7)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(48 as u32) ) } as u64;
	// 82F4A1AC: 7CC903A6  mtctr r6
	ctx.ctr.u64 = ctx.r[6].u64;
	// 82F4A1B0: 4E800421  bctrl
	ctx.lr = 0x82F4A1B4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4A1B4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A220(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4A220 size=420
    let mut pc: u32 = 0x82F4A220;
    'dispatch: loop {
        match pc {
            0x82F4A220 => {
    //   block [0x82F4A220..0x82F4A3C4)
	// 82F4A220: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4A224: 4825DF31  bl 0x831a8154
	ctx.lr = 0x82F4A228;
	sub_831A8130(ctx, base);
	// 82F4A228: 9421FD50  stwu r1, -0x2b0(r1)
	ea = ctx.r[1].u32.wrapping_add(-688 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4A22C: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A230: 3B000018  li r24, 0x18
	ctx.r[24].s64 = 24;
	// 82F4A234: 7C771B78  mr r23, r3
	ctx.r[23].u64 = ctx.r[3].u64;
	// 82F4A238: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F4A23C: 7CBD2B78  mr r29, r5
	ctx.r[29].u64 = ctx.r[5].u64;
	// 82F4A240: 7CDC3378  mr r28, r6
	ctx.r[28].u64 = ctx.r[6].u64;
	// 82F4A244: 7D78C82E  lwzx r11, r24, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82F4A248: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4A24C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4A250: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F4A254: 40980020  bge cr6, 0x82f4a274
	if !ctx.cr[6].lt {
	pc = 0x82F4A274; continue 'dispatch;
	}
	// 82F4A258: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4A25C: 3909003C  addi r8, r9, 0x3c
	ctx.r[8].s64 = ctx.r[9].s64 + 60;
	// 82F4A260: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F4A264: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F4A268: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F4A26C: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F4A270: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F4A274: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F4A278: 3B40FFFF  li r26, -1
	ctx.r[26].s64 = -1;
	// 82F4A27C: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82F4A280: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F4A284: 917C0040  stw r11, 0x40(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82F4A288: 815F0028  lwz r10, 0x28(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F4A28C: 2F0A0000  cmpwi cr6, r10, 0
	ctx.cr[6].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F4A290: 409900C4  ble cr6, 0x82f4a354
	if !ctx.cr[6].gt {
	pc = 0x82F4A354; continue 'dispatch;
	}
	// 82F4A294: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F4A298: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F4A29C: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4A2A0: 419A0048  beq cr6, 0x82f4a2e8
	if ctx.cr[6].eq {
	pc = 0x82F4A2E8; continue 'dispatch;
	}
	// 82F4A2A4: 80DF0020  lwz r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F4A2A8: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F4A2AC: 38E60010  addi r7, r6, 0x10
	ctx.r[7].s64 = ctx.r[6].s64 + 16;
	// 82F4A2B0: 409A0008  bne cr6, 0x82f4a2b8
	if !ctx.cr[6].eq {
	pc = 0x82F4A2B8; continue 'dispatch;
	}
	// 82F4A2B4: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F4A2B8: 809D0024  lwz r4, 0x24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F4A2BC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F4A2C0: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F4A2C4: 38610051  addi r3, r1, 0x51
	ctx.r[3].s64 = ctx.r[1].s64 + 81;
	// 82F4A2C8: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A2CC: 7D0BF02E  lwzx r8, r11, r30
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F4A2D0: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A2D4: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F4A2D8: 4E800421  bctrl
	ctx.lr = 0x82F4A2DC;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4A2DC: 89030000  lbz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A2E0: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82F4A2E4: 419A005C  beq cr6, 0x82f4a340
	if ctx.cr[6].eq {
	pc = 0x82F4A340; continue 'dispatch;
	}
	// 82F4A2E8: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F4A2EC: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F4A2F0: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F4A2F4: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F4A2F8: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4A2FC: 7C8AF02E  lwzx r4, r10, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F4A300: 81090014  lwz r8, 0x14(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F4A304: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F4A308: 4E800421  bctrl
	ctx.lr = 0x82F4A30C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4A30C: 7C671B78  mr r7, r3
	ctx.r[7].u64 = ctx.r[3].u64;
	// 82F4A310: 7F86E378  mr r6, r28
	ctx.r[6].u64 = ctx.r[28].u64;
	// 82F4A314: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F4A318: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F4A31C: 7CE43B78  mr r4, r7
	ctx.r[4].u64 = ctx.r[7].u64;
	// 82F4A320: 81670000  lwz r11, 0(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A324: 814B0020  lwz r10, 0x20(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F4A328: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4A32C: 4E800421  bctrl
	ctx.lr = 0x82F4A330;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4A330: 89210050  lbz r9, 0x50(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(80 as u32) ) } as u64;
	// 82F4A334: 2B090000  cmplwi cr6, r9, 0
	ctx.cr[6].compare_u32(ctx.r[9].u32, 0 as u32, &mut ctx.xer);
	// 82F4A338: 419A0008  beq cr6, 0x82f4a340
	if ctx.cr[6].eq {
	pc = 0x82F4A340; continue 'dispatch;
	}
	// 82F4A33C: 7F7ADB78  mr r26, r27
	ctx.r[26].u64 = ctx.r[27].u64;
	// 82F4A340: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F4A344: 3B7B0001  addi r27, r27, 1
	ctx.r[27].s64 = ctx.r[27].s64 + 1;
	// 82F4A348: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82F4A34C: 7F1B5800  cmpw cr6, r27, r11
	ctx.cr[6].compare_i32(ctx.r[27].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F4A350: 4198FF48  blt cr6, 0x82f4a298
	if ctx.cr[6].lt {
	pc = 0x82F4A298; continue 'dispatch;
	}
	// 82F4A354: 817C0040  lwz r11, 0x40(r28)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(64 as u32) ) } as u64;
	// 82F4A358: 2F1AFFFF  cmpwi cr6, r26, -1
	ctx.cr[6].compare_i32(ctx.r[26].s32, -1, &mut ctx.xer);
	// 82F4A35C: 396BFFFF  addi r11, r11, -1
	ctx.r[11].s64 = ctx.r[11].s64 + -1;
	// 82F4A360: 917C0040  stw r11, 0x40(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(64 as u32), ctx.r[11].u32 ) };
	// 82F4A364: 419A0010  beq cr6, 0x82f4a374
	if ctx.cr[6].eq {
	pc = 0x82F4A374; continue 'dispatch;
	}
	// 82F4A368: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F4A36C: 556A103A  slwi r10, r11, 2
	ctx.r[10].u32 = ctx.r[11].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F4A370: 7F4AE12E  stwx r26, r10, r28
	unsafe { crate::rt::store_u32(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[28].u32), ctx.r[26].u32) };
	// 82F4A374: 7D58C82E  lwzx r10, r24, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82F4A378: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4A37C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4A380: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F4A384: 40980020  bge cr6, 0x82f4a3a4
	if !ctx.cr[6].lt {
	pc = 0x82F4A3A4; continue 'dispatch;
	}
	// 82F4A388: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F4A38C: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F4A390: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F4A394: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F4A398: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F4A39C: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F4A3A0: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F4A3A4: 397A0001  addi r11, r26, 1
	ctx.r[11].s64 = ctx.r[26].s64 + 1;
	// 82F4A3A8: 7EE3BB78  mr r3, r23
	ctx.r[3].u64 = ctx.r[23].u64;
	// 82F4A3AC: 7D6A0034  cntlzw r10, r11
	ctx.r[10].u64 = if ctx.r[11].u32 == 0 { 32 } else { ctx.r[11].u32.leading_zeros() as u64 };
	// 82F4A3B0: 5549DFFE  rlwinm r9, r10, 0x1b, 0x1f, 0x1f
	ctx.r[9].u64 = ctx.r[10].u32 as u64 & 0x0000001Fu64;
	// 82F4A3B4: 69280001  xori r8, r9, 1
	ctx.r[8].u64 = ctx.r[9].u64 ^ 1;
	// 82F4A3B8: 99170000  stb r8, 0(r23)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[23].u32.wrapping_add(0 as u32), ctx.r[8].u8 ) };
	// 82F4A3BC: 382102B0  addi r1, r1, 0x2b0
	ctx.r[1].s64 = ctx.r[1].s64 + 688;
	// 82F4A3C0: 4825DDE4  b 0x831a81a4
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A3C8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4A3C8 size=344
    let mut pc: u32 = 0x82F4A3C8;
    'dispatch: loop {
        match pc {
            0x82F4A3C8 => {
    //   block [0x82F4A3C8..0x82F4A520)
	// 82F4A3C8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4A3CC: 4825DD8D  bl 0x831a8158
	ctx.lr = 0x82F4A3D0;
	sub_831A8130(ctx, base);
	// 82F4A3D0: 9421FD40  stwu r1, -0x2c0(r1)
	ea = ctx.r[1].u32.wrapping_add(-704 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4A3D4: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A3D8: 3B000018  li r24, 0x18
	ctx.r[24].s64 = 24;
	// 82F4A3DC: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4A3E0: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F4A3E4: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F4A3E8: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F4A3EC: 7D78C82E  lwzx r11, r24, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82F4A3F0: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4A3F4: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4A3F8: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F4A3FC: 40980020  bge cr6, 0x82f4a41c
	if !ctx.cr[6].lt {
	pc = 0x82F4A41C; continue 'dispatch;
	}
	// 82F4A400: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4A404: 3909003C  addi r8, r9, 0x3c
	ctx.r[8].s64 = ctx.r[9].s64 + 60;
	// 82F4A408: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F4A40C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F4A410: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F4A414: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F4A418: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F4A41C: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F4A420: 3B800000  li r28, 0
	ctx.r[28].s64 = 0;
	// 82F4A424: 2F0B0000  cmpwi cr6, r11, 0
	ctx.cr[6].compare_i32(ctx.r[11].s32, 0, &mut ctx.xer);
	// 82F4A428: 409900C0  ble cr6, 0x82f4a4e8
	if !ctx.cr[6].gt {
	pc = 0x82F4A4E8; continue 'dispatch;
	}
	// 82F4A42C: 3BC00000  li r30, 0
	ctx.r[30].s64 = 0;
	// 82F4A430: 817D0024  lwz r11, 0x24(r29)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F4A434: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4A438: 419A0048  beq cr6, 0x82f4a480
	if ctx.cr[6].eq {
	pc = 0x82F4A480; continue 'dispatch;
	}
	// 82F4A43C: 80DF0020  lwz r6, 0x20(r31)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F4A440: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F4A444: 38E60010  addi r7, r6, 0x10
	ctx.r[7].s64 = ctx.r[6].s64 + 16;
	// 82F4A448: 409A0008  bne cr6, 0x82f4a450
	if !ctx.cr[6].eq {
	pc = 0x82F4A450; continue 'dispatch;
	}
	// 82F4A44C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F4A450: 809D0024  lwz r4, 0x24(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F4A454: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F4A458: 817F0024  lwz r11, 0x24(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F4A45C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F4A460: 81440000  lwz r10, 0(r4)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A464: 7D0BF02E  lwzx r8, r11, r30
	ctx.r[8].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[11].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F4A468: 812A0000  lwz r9, 0(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A46C: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F4A470: 4E800421  bctrl
	ctx.lr = 0x82F4A474;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4A474: 89030000  lbz r8, 0(r3)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A478: 2B080000  cmplwi cr6, r8, 0
	ctx.cr[6].compare_u32(ctx.r[8].u32, 0 as u32, &mut ctx.xer);
	// 82F4A47C: 419A0058  beq cr6, 0x82f4a4d4
	if ctx.cr[6].eq {
	pc = 0x82F4A4D4; continue 'dispatch;
	}
	// 82F4A480: 817F0020  lwz r11, 0x20(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(32 as u32) ) } as u64;
	// 82F4A484: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F4A488: 815F0024  lwz r10, 0x24(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(36 as u32) ) } as u64;
	// 82F4A48C: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F4A490: 812B0010  lwz r9, 0x10(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4A494: 7C8AF02E  lwzx r4, r10, r30
	ctx.r[4].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[30].u32)) } as u64;
	// 82F4A498: 81090014  lwz r8, 0x14(r9)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F4A49C: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F4A4A0: 4E800421  bctrl
	ctx.lr = 0x82F4A4A4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4A4A4: 80FB0008  lwz r7, 8(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4A4A8: 9361006C  stw r27, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[27].u32 ) };
	// 82F4A4AC: 7F46D378  mr r6, r26
	ctx.r[6].u64 = ctx.r[26].u64;
	// 82F4A4B0: 93810064  stw r28, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[28].u32 ) };
	// 82F4A4B4: 38A10060  addi r5, r1, 0x60
	ctx.r[5].s64 = ctx.r[1].s64 + 96;
	// 82F4A4B8: 90610060  stw r3, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[3].u32 ) };
	// 82F4A4BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F4A4C0: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82F4A4C4: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A4C8: 814B0010  lwz r10, 0x10(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(16 as u32) ) } as u64;
	// 82F4A4CC: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4A4D0: 4E800421  bctrl
	ctx.lr = 0x82F4A4D4;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4A4D4: 817F0028  lwz r11, 0x28(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(40 as u32) ) } as u64;
	// 82F4A4D8: 3B9C0001  addi r28, r28, 1
	ctx.r[28].s64 = ctx.r[28].s64 + 1;
	// 82F4A4DC: 3BDE0004  addi r30, r30, 4
	ctx.r[30].s64 = ctx.r[30].s64 + 4;
	// 82F4A4E0: 7F1C5800  cmpw cr6, r28, r11
	ctx.cr[6].compare_i32(ctx.r[28].s32, ctx.r[11].s32, &mut ctx.xer);
	// 82F4A4E4: 4198FF4C  blt cr6, 0x82f4a430
	if ctx.cr[6].lt {
	pc = 0x82F4A430; continue 'dispatch;
	}
	// 82F4A4E8: 7D58C82E  lwzx r10, r24, r25
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82F4A4EC: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4A4F0: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4A4F4: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F4A4F8: 40980020  bge cr6, 0x82f4a518
	if !ctx.cr[6].lt {
	pc = 0x82F4A518; continue 'dispatch;
	}
	// 82F4A4FC: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F4A500: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F4A504: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F4A508: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F4A50C: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F4A510: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F4A514: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F4A518: 382102C0  addi r1, r1, 0x2c0
	ctx.r[1].s64 = ctx.r[1].s64 + 704;
	// 82F4A51C: 4825DC8C  b 0x831a81a8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A520(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4A520 size=8
    let mut pc: u32 = 0x82F4A520;
    'dispatch: loop {
        match pc {
            0x82F4A520 => {
    //   block [0x82F4A520..0x82F4A528)
	// 82F4A520: 3863FFEC  addi r3, r3, -0x14
	ctx.r[3].s64 = ctx.r[3].s64 + -20;
	// 82F4A524: 48000004  b 0x82f4a528
	sub_82F4A528(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A528(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4A528 size=124
    let mut pc: u32 = 0x82F4A528;
    'dispatch: loop {
        match pc {
            0x82F4A528 => {
    //   block [0x82F4A528..0x82F4A5A4)
	// 82F4A528: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4A52C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4A530: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4A534: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4A538: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4A53C: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F4A540: 397F0014  addi r11, r31, 0x14
	ctx.r[11].s64 = ctx.r[31].s64 + 20;
	// 82F4A544: 409A0008  bne cr6, 0x82f4a54c
	if !ctx.cr[6].eq {
	pc = 0x82F4A54C; continue 'dispatch;
	}
	// 82F4A548: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4A54C: 3D408213  lis r10, -0x7ded
	ctx.r[10].s64 = -2112684032;
	// 82F4A550: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F4A554: 390ABCD8  addi r8, r10, -0x4328
	ctx.r[8].s64 = ctx.r[10].s64 + -17192;
	// 82F4A558: 38E99EAC  addi r7, r9, -0x6154
	ctx.r[7].s64 = ctx.r[9].s64 + -24916;
	// 82F4A55C: 548607FE  clrlwi r6, r4, 0x1f
	ctx.r[6].u64 = ctx.r[4].u32 as u64 & 0x00000001u64;
	// 82F4A560: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F4A564: 90FF0000  stw r7, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F4A568: 2B060000  cmplwi cr6, r6, 0
	ctx.cr[6].compare_u32(ctx.r[6].u32, 0 as u32, &mut ctx.xer);
	// 82F4A56C: 419A0020  beq cr6, 0x82f4a58c
	if ctx.cr[6].eq {
	pc = 0x82F4A58C; continue 'dispatch;
	}
	// 82F4A570: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A574: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4A578: 38C00027  li r6, 0x27
	ctx.r[6].s64 = 39;
	// 82F4A57C: A0BF0004  lhz r5, 4(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[31].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4A580: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4A584: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4A588: 4BF56229  bl 0x82ea07b0
	ctx.lr = 0x82F4A58C;
	sub_82EA07B0(ctx, base);
	// 82F4A58C: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4A590: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4A594: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4A598: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4A59C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4A5A0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A5A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4A5A8 size=144
    let mut pc: u32 = 0x82F4A5A8;
    'dispatch: loop {
        match pc {
            0x82F4A5A8 => {
    //   block [0x82F4A5A8..0x82F4A638)
	// 82F4A5A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4A5AC: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4A5B0: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4A5B4: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4A5B8: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A638(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4A638 size=148
    let mut pc: u32 = 0x82F4A638;
    'dispatch: loop {
        match pc {
            0x82F4A638 => {
    //   block [0x82F4A638..0x82F4A6CC)
	// 82F4A638: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4A63C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4A640: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4A644: 9421FF70  stwu r1, -0x90(r1)
	ea = ctx.r[1].u32.wrapping_add(-144 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4A648: 39600010  li r11, 0x10
	ctx.r[11].s64 = 16;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A6D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4A6D0 size=84
    let mut pc: u32 = 0x82F4A6D0;
    'dispatch: loop {
        match pc {
            0x82F4A6D0 => {
    //   block [0x82F4A6D0..0x82F4A724)
	// 82F4A6D0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4A6D4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4A6D8: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4A6DC: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4A6E0: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4A6E4: 7CAB2B78  mr r11, r5
	ctx.r[11].u64 = ctx.r[5].u64;
	// 82F4A6E8: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F4A6EC: 7D645B78  mr r4, r11
	ctx.r[4].u64 = ctx.r[11].u64;
	// 82F4A6F0: 807F0008  lwz r3, 8(r31)
	ctx.r[3].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4A6F4: 81430000  lwz r10, 0(r3)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A6F8: 812A0004  lwz r9, 4(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4A6FC: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F4A700: 4E800421  bctrl
	ctx.lr = 0x82F4A704;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4A704: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4A708: 88E80004  lbz r7, 4(r8)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[8].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4A70C: 98FF0004  stb r7, 4(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(4 as u32), ctx.r[7].u8 ) };
	// 82F4A710: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4A714: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4A718: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4A71C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4A720: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A728(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4A728 size=4
    let mut pc: u32 = 0x82F4A728;
    'dispatch: loop {
        match pc {
            0x82F4A728 => {
    //   block [0x82F4A728..0x82F4A72C)
	// 82F4A728: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A730(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4A730 size=8
    let mut pc: u32 = 0x82F4A730;
    'dispatch: loop {
        match pc {
            0x82F4A730 => {
    //   block [0x82F4A730..0x82F4A738)
	// 82F4A730: 2B030000  cmplwi cr6, r3, 0
	ctx.cr[6].compare_u32(ctx.r[3].u32, 0 as u32, &mut ctx.xer);
	// 82F4A734: 4D9A0020  beqlr cr6
	if ctx.cr[6].eq { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A738(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4A738 size=20
    let mut pc: u32 = 0x82F4A738;
    'dispatch: loop {
        match pc {
            0x82F4A738 => {
    //   block [0x82F4A738..0x82F4A74C)
	// 82F4A738: 81630000  lwz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A73C: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4A740: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A744: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4A748: 4E800420  bctr
	crate::rt::call_indirect(ctx.ctr.u32);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A74C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4A74C size=4
    let mut pc: u32 = 0x82F4A74C;
    'dispatch: loop {
        match pc {
            0x82F4A74C => {
    //   block [0x82F4A74C..0x82F4A750)
	// 82F4A74C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A750(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4A750 size=32
    let mut pc: u32 = 0x82F4A750;
    'dispatch: loop {
        match pc {
            0x82F4A750 => {
    //   block [0x82F4A750..0x82F4A770)
	// 82F4A750: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82F4A754: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A770(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4A770 size=20
    let mut pc: u32 = 0x82F4A770;
    'dispatch: loop {
        match pc {
            0x82F4A770 => {
    //   block [0x82F4A770..0x82F4A784)
	// 82F4A770: C0030018  lfs f0, 0x18(r3)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(24 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4A774: FF000800  fcmpu cr6, f0, f1
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[1].f64);
	// 82F4A778: 409A000C  bne cr6, 0x82f4a784
	if !ctx.cr[6].eq {
		sub_82F4A784(ctx, base);
		return;
	}
	// 82F4A77C: D0430018  stfs f2, 0x18(r3)
	tmp.f32 = (ctx.f[2].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
	// 82F4A780: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A784(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4A784 size=32
    let mut pc: u32 = 0x82F4A784;
    'dispatch: loop {
        match pc {
            0x82F4A784 => {
    //   block [0x82F4A784..0x82F4A7A4)
	// 82F4A784: 3D408201  lis r10, -0x7dff
	ctx.r[10].s64 = -2113863680;
	// 82F4A788: 39630020  addi r11, r3, 0x20
	ctx.r[11].s64 = ctx.r[3].s64 + 32;
	// 82F4A78C: C00A9534  lfs f0, -0x6acc(r10)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(-27340 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4A790: D0030018  stfs f0, 0x18(r3)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(24 as u32), tmp.u32 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A7A8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4A7A8 size=240
    let mut pc: u32 = 0x82F4A7A8;
    'dispatch: loop {
        match pc {
            0x82F4A7A8 => {
    //   block [0x82F4A7A8..0x82F4A898)
	// 82F4A7A8: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4A7AC: 4825D9B9  bl 0x831a8164
	ctx.lr = 0x82F4A7B0;
	sub_831A8130(ctx, base);
	// 82F4A7B0: 9421FF40  stwu r1, -0xc0(r1)
	ea = ctx.r[1].u32.wrapping_add(-192 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4A7B4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4A7B8: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4A7BC: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4A7C0: 392B0050  addi r9, r11, 0x50
	ctx.r[9].s64 = ctx.r[11].s64 + 80;
	// 82F4A7C4: 7C9E2378  mr r30, r4
	ctx.r[30].u64 = ctx.r[4].u64;
	// 82F4A7C8: 90DF0008  stw r6, 8(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(8 as u32), ctx.r[6].u32 ) };
	// 82F4A7CC: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F4A7D0: B15F0006  sth r10, 6(r31)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[31].u32.wrapping_add(6 as u32), ctx.r[10].u16 ) };
	// 82F4A7D4: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F4A7D8: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4A7DC: 839E0000  lwz r28, 0(r30)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A7E0: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A7E4: 80BB0008  lwz r5, 8(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4A7E8: 809E0008  lwz r4, 8(r30)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4A7EC: 4BF56875  bl 0x82ea1060
	ctx.lr = 0x82F4A7F0;
	sub_82EA1060(ctx, base);
	// 82F4A7F0: 811D000C  lwz r8, 0xc(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4A7F4: 2F080004  cmpwi cr6, r8, 4
	ctx.cr[6].compare_i32(ctx.r[8].s32, 4, &mut ctx.xer);
	// 82F4A7F8: 38C10050  addi r6, r1, 0x50
	ctx.r[6].s64 = ctx.r[1].s64 + 80;
	// 82F4A7FC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F4A800: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F4A804: 387F000C  addi r3, r31, 0xc
	ctx.r[3].s64 = ctx.r[31].s64 + 12;
	// 82F4A808: 409A000C  bne cr6, 0x82f4a814
	if !ctx.cr[6].eq {
	pc = 0x82F4A814; continue 'dispatch;
	}
	// 82F4A80C: 4803994D  bl 0x82f84158
	ctx.lr = 0x82F4A810;
	sub_82F84158(ctx, base);
	// 82F4A810: 48000008  b 0x82f4a818
	pc = 0x82F4A818; continue 'dispatch;
	// 82F4A814: 4803975D  bl 0x82f83f70
	ctx.lr = 0x82F4A818;
	sub_82F83F70(ctx, base);
	// 82F4A818: 397F0020  addi r11, r31, 0x20
	ctx.r[11].s64 = ctx.r[31].s64 + 32;
	// 82F4A81C: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F4A820: 7FCAF378  mr r10, r30
	ctx.r[10].u64 = ctx.r[30].u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A898(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4A898 size=164
    let mut pc: u32 = 0x82F4A898;
    'dispatch: loop {
        match pc {
            0x82F4A898 => {
    //   block [0x82F4A898..0x82F4A93C)
	// 82F4A898: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4A89C: 4825D8C9  bl 0x831a8164
	ctx.lr = 0x82F4A8A0;
	sub_831A8130(ctx, base);
	// 82F4A8A0: 9421FF30  stwu r1, -0xd0(r1)
	ea = ctx.r[1].u32.wrapping_add(-208 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4A8A4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4A8A8: 83640000  lwz r27, 0(r4)
	ctx.r[27].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A8AC: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F4A8B0: 80A40008  lwz r5, 8(r4)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4A8B4: 7CDE3378  mr r30, r6
	ctx.r[30].u64 = ctx.r[6].u64;
	// 82F4A8B8: 7CFD3B78  mr r29, r7
	ctx.r[29].u64 = ctx.r[7].u64;
	// 82F4A8BC: 839F0000  lwz r28, 0(r31)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A8C0: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4A8C4: 4BF5679D  bl 0x82ea1060
	ctx.lr = 0x82F4A8C8;
	sub_82EA1060(ctx, base);
	// 82F4A8C8: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F4A8CC: 38C10060  addi r6, r1, 0x60
	ctx.r[6].s64 = ctx.r[1].s64 + 96;
	// 82F4A8D0: 7F65DB78  mr r5, r27
	ctx.r[5].u64 = ctx.r[27].u64;
	// 82F4A8D4: 7F84E378  mr r4, r28
	ctx.r[4].u64 = ctx.r[28].u64;
	// 82F4A8D8: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F4A8DC: 4803C36D  bl 0x82f86c48
	ctx.lr = 0x82F4A8E0;
	sub_82F86C48(ctx, base);
	// 82F4A8E0: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82F4A8E4: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4A8E8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82F4A8EC: 39200020  li r9, 0x20
	ctx.r[9].s64 = 32;
	// 82F4A8F0: C001005C  lfs f0, 0x5c(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4A940(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4A940 size=556
    let mut pc: u32 = 0x82F4A940;
    'dispatch: loop {
        match pc {
            0x82F4A940 => {
    //   block [0x82F4A940..0x82F4AB6C)
	// 82F4A940: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4A944: 4825D80D  bl 0x831a8150
	ctx.lr = 0x82F4A948;
	sub_831A8130(ctx, base);
	// 82F4A948: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4A94C: 82ED0000  lwz r23, 0(r13)
	ctx.r[23].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A950: 3AC00018  li r22, 0x18
	ctx.r[22].s64 = 24;
	// 82F4A954: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F4A958: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F4A95C: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82F4A960: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82F4A964: 7D56B82E  lwzx r10, r22, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82F4A968: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4A96C: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4A970: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F4A974: 4098002C  bge cr6, 0x82f4a9a0
	if !ctx.cr[6].lt {
	pc = 0x82F4A9A0; continue 'dispatch;
	}
	// 82F4A978: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4A97C: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F4A980: 38E9009C  addi r7, r9, 0x9c
	ctx.r[7].s64 = ctx.r[9].s64 + 156;
	// 82F4A984: 38C80094  addi r6, r8, 0x94
	ctx.r[6].s64 = ctx.r[8].s64 + 148;
	// 82F4A988: 90EB0000  stw r7, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F4A98C: 90CB000C  stw r6, 0xc(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(12 as u32), ctx.r[6].u32 ) };
	// 82F4A990: 7CAC42E6  mftb r5, 0x10c
	ctx.r[5].u64 = crate::rt::rdtsc_u64();
	// 82F4A994: 386B0010  addi r3, r11, 0x10
	ctx.r[3].s64 = ctx.r[11].s64 + 16;
	// 82F4A998: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F4A99C: 906A0004  stw r3, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[3].u32 ) };
	// 82F4A9A0: 7D56B82E  lwzx r10, r22, r23
	ctx.r[10].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[22].u32.wrapping_add(ctx.r[23].u32)) } as u64;
	// 82F4A9A4: 812A000C  lwz r9, 0xc(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4A9A8: 816A0004  lwz r11, 4(r10)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4A9AC: 7F0B4840  cmplw cr6, r11, r9
	ctx.cr[6].compare_u32(ctx.r[11].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F4A9B0: 40980020  bge cr6, 0x82f4a9d0
	if !ctx.cr[6].lt {
	pc = 0x82F4A9D0; continue 'dispatch;
	}
	// 82F4A9B4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4A9B8: 39090088  addi r8, r9, 0x88
	ctx.r[8].s64 = ctx.r[9].s64 + 136;
	// 82F4A9BC: 910B0000  stw r8, 0(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F4A9C0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F4A9C4: 38AB000C  addi r5, r11, 0xc
	ctx.r[5].s64 = ctx.r[11].s64 + 12;
	// 82F4A9C8: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F4A9CC: 90AA0004  stw r5, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F4A9D0: 386101E0  addi r3, r1, 0x1e0
	ctx.r[3].s64 = ctx.r[1].s64 + 480;
	// 82F4A9D4: 83DC0000  lwz r30, 0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A9D8: 83BA0000  lwz r29, 0(r26)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4A9DC: 80BA0008  lwz r5, 8(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4A9E0: 809C0008  lwz r4, 8(r28)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4A9E4: 4BF5667D  bl 0x82ea1060
	ctx.lr = 0x82F4A9E8;
	sub_82EA1060(ctx, base);
	// 82F4A9E8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F4A9EC: 893B0016  lbz r9, 0x16(r27)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(22 as u32) ) } as u64;
	// 82F4A9F0: 88BB0014  lbz r5, 0x14(r27)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(20 as u32) ) } as u64;
	// 82F4A9F4: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F4A9F8: 891B0015  lbz r8, 0x15(r27)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[27].u32.wrapping_add(21 as u32) ) } as u64;
	// 82F4A9FC: 5527E13E  srwi r7, r9, 4
	ctx.r[7].u32 = ctx.r[9].u32.wrapping_shr(4);
	ctx.r[7].u64 = ctx.r[7].u32 as u64;
	// 82F4AA00: 91610070  stw r11, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[11].u32 ) };
	// 82F4AA04: 552B073E  clrlwi r11, r9, 0x1c
	ctx.r[11].u64 = ctx.r[9].u32 as u64 & 0x0000000Fu64;
	// 82F4AA08: 91410074  stw r10, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[10].u32 ) };
	// 82F4AA0C: 3BFB000C  addi r31, r27, 0xc
	ctx.r[31].s64 = ctx.r[27].s64 + 12;
	// 82F4AA10: 90E10068  stw r7, 0x68(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(104 as u32), ctx.r[7].u32 ) };
	// 82F4AA14: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F4AA18: 90A10060  stw r5, 0x60(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[5].u32 ) };
	// 82F4AA1C: 38C10080  addi r6, r1, 0x80
	ctx.r[6].s64 = ctx.r[1].s64 + 128;
	// 82F4AA20: 9161006C  stw r11, 0x6c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(108 as u32), ctx.r[11].u32 ) };
	// 82F4AA24: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4AA28: 91010064  stw r8, 0x64(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(100 as u32), ctx.r[8].u32 ) };
	// 82F4AA2C: 815E0000  lwz r10, 0(r30)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4AA30: 812A0034  lwz r9, 0x34(r10)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F4AA34: 7D2903A6  mtctr r9
	ctx.ctr.u64 = ctx.r[9].u64;
	// 82F4AA38: 4E800421  bctrl
	ctx.lr = 0x82F4AA3C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4AA3C: 811D0000  lwz r8, 0(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4AA40: 80E10060  lwz r7, 0x60(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(96 as u32) ) } as u64;
	// 82F4AA44: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F4AA48: 80A10064  lwz r5, 0x64(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(100 as u32) ) } as u64;
	// 82F4AA4C: 38C10140  addi r6, r1, 0x140
	ctx.r[6].s64 = ctx.r[1].s64 + 320;
	// 82F4AA50: 54EB083C  slwi r11, r7, 1
	ctx.r[11].u32 = ctx.r[7].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4AA54: 81480034  lwz r10, 0x34(r8)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[8].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F4AA58: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82F4AA5C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4AA60: 4E800421  bctrl
	ctx.lr = 0x82F4AA64;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4AA64: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F4AA68: 38C101E0  addi r6, r1, 0x1e0
	ctx.r[6].s64 = ctx.r[1].s64 + 480;
	// 82F4AA6C: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F4AA70: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F4AA74: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F4AA78: 4803C1D1  bl 0x82f86c48
	ctx.lr = 0x82F4AA7C;
	sub_82F86C48(ctx, base);
	// 82F4AA7C: 81210074  lwz r9, 0x74(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(116 as u32) ) } as u64;
	// 82F4AA80: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82F4AA84: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F4AA88: 419A0010  beq cr6, 0x82f4aa98
	if ctx.cr[6].eq {
	pc = 0x82F4AA98; continue 'dispatch;
	}
	// 82F4AA8C: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4AA90: 38610060  addi r3, r1, 0x60
	ctx.r[3].s64 = ctx.r[1].s64 + 96;
	// 82F4AA94: 4803AB45  bl 0x82f855d8
	ctx.lr = 0x82F4AA98;
	sub_82F855D8(ctx, base);
	// 82F4AA98: 2F190000  cmpwi cr6, r25, 0
	ctx.cr[6].compare_i32(ctx.r[25].s32, 0, &mut ctx.xer);
	// 82F4AA9C: 409A006C  bne cr6, 0x82f4ab08
	if !ctx.cr[6].eq {
	pc = 0x82F4AB08; continue 'dispatch;
	}
	// 82F4AAA0: 39410050  addi r10, r1, 0x50
	ctx.r[10].s64 = ctx.r[1].s64 + 80;
	// 82F4AAA4: 813C0008  lwz r9, 8(r28)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4AAA8: 39000010  li r8, 0x10
	ctx.r[8].s64 = 16;
	// 82F4AAAC: C01E0010  lfs f0, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4AAB0: C1A1005C  lfs f13, 0x5c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(92 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4AAB4: 39600020  li r11, 0x20
	ctx.r[11].s64 = 32;
	// 82F4AAB8: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82F4AABC: C17D0010  lfs f11, 0x10(r29)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4AAC0: 3CE08200  lis r7, -0x7e00
	ctx.r[7].s64 = -2113929216;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4AB70(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4AB70 size=420
    let mut pc: u32 = 0x82F4AB70;
    'dispatch: loop {
        match pc {
            0x82F4AB70 => {
    //   block [0x82F4AB70..0x82F4AD14)
	// 82F4AB70: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4AB74: 4825D5E9  bl 0x831a815c
	ctx.lr = 0x82F4AB78;
	sub_831A8130(ctx, base);
	// 82F4AB78: 9421FD90  stwu r1, -0x270(r1)
	ea = ctx.r[1].u32.wrapping_add(-624 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4AB7C: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4AB80: 3B800018  li r28, 0x18
	ctx.r[28].s64 = 24;
	// 82F4AB84: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F4AB88: 7C9A2378  mr r26, r4
	ctx.r[26].u64 = ctx.r[4].u64;
	// 82F4AB8C: 7CD93378  mr r25, r6
	ctx.r[25].u64 = ctx.r[6].u64;
	// 82F4AB90: 7D7CE82E  lwzx r11, r28, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82F4AB94: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4AB98: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4AB9C: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F4ABA0: 40980020  bge cr6, 0x82f4abc0
	if !ctx.cr[6].lt {
	pc = 0x82F4ABC0; continue 'dispatch;
	}
	// 82F4ABA4: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4ABA8: 3909FD84  addi r8, r9, -0x27c
	ctx.r[8].s64 = ctx.r[9].s64 + -636;
	// 82F4ABAC: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F4ABB0: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F4ABB4: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F4ABB8: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F4ABBC: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F4ABC0: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F4ABC4: 83DB0000  lwz r30, 0(r27)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4ABC8: 83FA0000  lwz r31, 0(r26)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4ABCC: 80BA0008  lwz r5, 8(r26)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4ABD0: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4ABD4: 4BF5648D  bl 0x82ea1060
	ctx.lr = 0x82F4ABD8;
	sub_82EA1060(ctx, base);
	// 82F4ABD8: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4ABDC: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F4ABE0: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F4ABE4: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82F4ABE8: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F4ABEC: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F4ABF0: 409A000C  bne cr6, 0x82f4abfc
	if !ctx.cr[6].eq {
	pc = 0x82F4ABFC; continue 'dispatch;
	}
	// 82F4ABF4: 48039565  bl 0x82f84158
	ctx.lr = 0x82F4ABF8;
	sub_82F84158(ctx, base);
	// 82F4ABF8: 48000008  b 0x82f4ac00
	pc = 0x82F4AC00; continue 'dispatch;
	// 82F4ABFC: 48039375  bl 0x82f83f70
	ctx.lr = 0x82F4AC00;
	sub_82F83F70(ctx, base);
	// 82F4AC00: 8961005A  lbz r11, 0x5a(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 82F4AC04: 39400001  li r10, 1
	ctx.r[10].s64 = 1;
	// 82F4AC08: 88810059  lbz r4, 0x59(r1)
	ctx.r[4].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82F4AC0C: 39000000  li r8, 0
	ctx.r[8].s64 = 0;
	// 82F4AC10: 88A10058  lbz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F4AC14: 5566E13E  srwi r6, r11, 4
	ctx.r[6].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[6].u64 = ctx.r[6].u32 as u64;
	// 82F4AC18: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82F4AC1C: 914100C0  stw r10, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[10].u32 ) };
	// 82F4AC20: 90C100B8  stw r6, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[6].u32 ) };
	// 82F4AC24: 38C100D0  addi r6, r1, 0xd0
	ctx.r[6].s64 = ctx.r[1].s64 + 208;
	// 82F4AC28: 906100BC  stw r3, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[3].u32 ) };
	// 82F4AC2C: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F4AC30: 908100B4  stw r4, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[4].u32 ) };
	// 82F4AC34: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F4AC38: 90A100B0  stw r5, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[5].u32 ) };
	// 82F4AC3C: 910100C4  stw r8, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[8].u32 ) };
	// 82F4AC40: 817E0000  lwz r11, 0(r30)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4AC44: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F4AC48: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4AC4C: 4E800421  bctrl
	ctx.lr = 0x82F4AC50;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4AC50: 813F0000  lwz r9, 0(r31)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4AC54: 810100B0  lwz r8, 0xb0(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82F4AC58: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82F4AC5C: 80A100B4  lwz r5, 0xb4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82F4AC60: 38C10190  addi r6, r1, 0x190
	ctx.r[6].s64 = ctx.r[1].s64 + 400;
	// 82F4AC64: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F4AC68: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4AC6C: 80E90034  lwz r7, 0x34(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F4AC70: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F4AC74: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F4AC78: 4E800421  bctrl
	ctx.lr = 0x82F4AC7C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4AC7C: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82F4AC80: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F4AC84: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F4AC88: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F4AC8C: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82F4AC90: 4803BFB9  bl 0x82f86c48
	ctx.lr = 0x82F4AC94;
	sub_82F86C48(ctx, base);
	// 82F4AC94: 7D7CE82E  lwzx r11, r28, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82F4AC98: 80CB000C  lwz r6, 0xc(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4AC9C: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4ACA0: 7F0A3040  cmplw cr6, r10, r6
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[6].u32, &mut ctx.xer);
	// 82F4ACA4: 40980020  bge cr6, 0x82f4acc4
	if !ctx.cr[6].lt {
	pc = 0x82F4ACC4; continue 'dispatch;
	}
	// 82F4ACA8: 3D208201  lis r9, -0x7dff
	ctx.r[9].s64 = -2113863680;
	// 82F4ACAC: 3909BADC  addi r8, r9, -0x4524
	ctx.r[8].s64 = ctx.r[9].s64 + -17700;
	// 82F4ACB0: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F4ACB4: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F4ACB8: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F4ACBC: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F4ACC0: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F4ACC4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82F4ACC8: 409A0028  bne cr6, 0x82f4acf0
	if !ctx.cr[6].eq {
	pc = 0x82F4ACF0; continue 'dispatch;
	}
	// 82F4ACCC: C01E0010  lfs f0, 0x10(r30)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4ACD0: 3D608200  lis r11, -0x7e00
	ctx.r[11].s64 = -2113929216;
	// 82F4ACD4: C1A1006C  lfs f13, 0x6c(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(108 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4ACD8: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82F4ACDC: C17F0010  lfs f11, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4ACE0: C00B08A4  lfs f0, 0x8a4(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(2212 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4ACE4: ED4C5828  fsubs f10, f12, f11
	ctx.f[10].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 82F4ACE8: FF0A0000  fcmpu cr6, f10, f0
	ctx.cr[6].compare_f64(ctx.f[10].f64, ctx.f[0].f64);
	// 82F4ACEC: 40980020  bge cr6, 0x82f4ad0c
	if !ctx.cr[6].lt {
	pc = 0x82F4AD0C; continue 'dispatch;
	}
	// 82F4ACF0: 81790000  lwz r11, 0(r25)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4ACF4: 7F23CB78  mr r3, r25
	ctx.r[3].u64 = ctx.r[25].u64;
	// 82F4ACF8: 7F64DB78  mr r4, r27
	ctx.r[4].u64 = ctx.r[27].u64;
	// 82F4ACFC: 7F45D378  mr r5, r26
	ctx.r[5].u64 = ctx.r[26].u64;
	// 82F4AD00: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4AD04: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4AD08: 4E800421  bctrl
	ctx.lr = 0x82F4AD0C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4AD0C: 38210270  addi r1, r1, 0x270
	ctx.r[1].s64 = ctx.r[1].s64 + 624;
	// 82F4AD10: 4825D49C  b 0x831a81ac
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4AD18(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4AD18 size=740
    let mut pc: u32 = 0x82F4AD18;
    'dispatch: loop {
        match pc {
            0x82F4AD18 => {
    //   block [0x82F4AD18..0x82F4AFFC)
	// 82F4AD18: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4AD1C: 4825D439  bl 0x831a8154
	ctx.lr = 0x82F4AD20;
	sub_831A8130(ctx, base);
	// 82F4AD20: 9421FCF0  stwu r1, -0x310(r1)
	ea = ctx.r[1].u32.wrapping_add(-784 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4AD24: 832D0000  lwz r25, 0(r13)
	ctx.r[25].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4AD28: 3B000018  li r24, 0x18
	ctx.r[24].s64 = 24;
	// 82F4AD2C: 7C7D1B78  mr r29, r3
	ctx.r[29].u64 = ctx.r[3].u64;
	// 82F4AD30: 7C9C2378  mr r28, r4
	ctx.r[28].u64 = ctx.r[4].u64;
	// 82F4AD34: 7CBA2B78  mr r26, r5
	ctx.r[26].u64 = ctx.r[5].u64;
	// 82F4AD38: 7CD73378  mr r23, r6
	ctx.r[23].u64 = ctx.r[6].u64;
	// 82F4AD3C: 7D78C82E  lwzx r11, r24, r25
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[24].u32.wrapping_add(ctx.r[25].u32)) } as u64;
	// 82F4AD40: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4AD44: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4AD48: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F4AD4C: 40980020  bge cr6, 0x82f4ad6c
	if !ctx.cr[6].lt {
	pc = 0x82F4AD6C; continue 'dispatch;
	}
	// 82F4AD50: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4AD54: 3909FD84  addi r8, r9, -0x27c
	ctx.r[8].s64 = ctx.r[9].s64 + -636;
	// 82F4AD58: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F4AD5C: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F4AD60: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F4AD64: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F4AD68: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F4AD6C: 38610100  addi r3, r1, 0x100
	ctx.r[3].s64 = ctx.r[1].s64 + 256;
	// 82F4AD70: 83DD0000  lwz r30, 0(r29)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4AD74: 83FC0000  lwz r31, 0(r28)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4AD78: 80BC0008  lwz r5, 8(r28)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4AD7C: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4AD80: 4BF562E1  bl 0x82ea1060
	ctx.lr = 0x82F4AD84;
	sub_82EA1060(ctx, base);
	// 82F4AD84: 817F000C  lwz r11, 0xc(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4AD88: 38C10100  addi r6, r1, 0x100
	ctx.r[6].s64 = ctx.r[1].s64 + 256;
	// 82F4AD8C: 7FE5FB78  mr r5, r31
	ctx.r[5].u64 = ctx.r[31].u64;
	// 82F4AD90: 2F0B0004  cmpwi cr6, r11, 4
	ctx.cr[6].compare_i32(ctx.r[11].s32, 4, &mut ctx.xer);
	// 82F4AD94: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F4AD98: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F4AD9C: 409A000C  bne cr6, 0x82f4ada8
	if !ctx.cr[6].eq {
	pc = 0x82F4ADA8; continue 'dispatch;
	}
	// 82F4ADA0: 480393B9  bl 0x82f84158
	ctx.lr = 0x82F4ADA4;
	sub_82F84158(ctx, base);
	// 82F4ADA4: 48000008  b 0x82f4adac
	pc = 0x82F4ADAC; continue 'dispatch;
	// 82F4ADA8: 480391C9  bl 0x82f83f70
	ctx.lr = 0x82F4ADAC;
	sub_82F83F70(ctx, base);
	// 82F4ADAC: 8961005A  lbz r11, 0x5a(r1)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(90 as u32) ) } as u64;
	// 82F4ADB0: 3B600000  li r27, 0
	ctx.r[27].s64 = 0;
	// 82F4ADB4: 88A10058  lbz r5, 0x58(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(88 as u32) ) } as u64;
	// 82F4ADB8: 38C10160  addi r6, r1, 0x160
	ctx.r[6].s64 = ctx.r[1].s64 + 352;
	// 82F4ADBC: 88E10059  lbz r7, 0x59(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[1].u32.wrapping_add(89 as u32) ) } as u64;
	// 82F4ADC0: 5563073E  clrlwi r3, r11, 0x1c
	ctx.r[3].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82F4ADC4: 83FD0000  lwz r31, 0(r29)
	ctx.r[31].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4ADC8: 5568E13E  srwi r8, r11, 4
	ctx.r[8].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[8].u64 = ctx.r[8].u32 as u64;
	// 82F4ADCC: 9061014C  stw r3, 0x14c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(332 as u32), ctx.r[3].u32 ) };
	// 82F4ADD0: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F4ADD4: 93610150  stw r27, 0x150(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(336 as u32), ctx.r[27].u32 ) };
	// 82F4ADD8: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4ADDC: 90A10140  stw r5, 0x140(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(320 as u32), ctx.r[5].u32 ) };
	// 82F4ADE0: 91010148  stw r8, 0x148(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(328 as u32), ctx.r[8].u32 ) };
	// 82F4ADE4: 90E10144  stw r7, 0x144(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(324 as u32), ctx.r[7].u32 ) };
	// 82F4ADE8: 93610154  stw r27, 0x154(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(340 as u32), ctx.r[27].u32 ) };
	// 82F4ADEC: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4ADF0: 83DC0000  lwz r30, 0(r28)
	ctx.r[30].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4ADF4: 814B0034  lwz r10, 0x34(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F4ADF8: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4ADFC: 4E800421  bctrl
	ctx.lr = 0x82F4AE00;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4AE00: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4AE04: 81010140  lwz r8, 0x140(r1)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(320 as u32) ) } as u64;
	// 82F4AE08: 39610050  addi r11, r1, 0x50
	ctx.r[11].s64 = ctx.r[1].s64 + 80;
	// 82F4AE0C: 80A10144  lwz r5, 0x144(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(324 as u32) ) } as u64;
	// 82F4AE10: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F4AE14: 550A083C  slwi r10, r8, 1
	ctx.r[10].u32 = ctx.r[8].u32.wrapping_shl(1);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F4AE18: 38C10220  addi r6, r1, 0x220
	ctx.r[6].s64 = ctx.r[1].s64 + 544;
	// 82F4AE1C: 80E90034  lwz r7, 0x34(r9)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[9].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F4AE20: 7C8A5A14  add r4, r10, r11
	ctx.r[4].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F4AE24: 7CE903A6  mtctr r7
	ctx.ctr.u64 = ctx.r[7].u64;
	// 82F4AE28: 4E800421  bctrl
	ctx.lr = 0x82F4AE2C;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4AE2C: 38E100C0  addi r7, r1, 0xc0
	ctx.r[7].s64 = ctx.r[1].s64 + 192;
	// 82F4AE30: 38C10100  addi r6, r1, 0x100
	ctx.r[6].s64 = ctx.r[1].s64 + 256;
	// 82F4AE34: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F4AE38: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4AE3C: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 82F4AE40: 4803BE09  bl 0x82f86c48
	ctx.lr = 0x82F4AE44;
	sub_82F86C48(ctx, base);
	// 82F4AE44: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82F4AE48: 409A00EC  bne cr6, 0x82f4af34
	if !ctx.cr[6].eq {
	pc = 0x82F4AF34; continue 'dispatch;
	}
	// 82F4AE4C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F4AE50: 388100C0  addi r4, r1, 0xc0
	ctx.r[4].s64 = ctx.r[1].s64 + 192;
	// 82F4AE54: 38610140  addi r3, r1, 0x140
	ctx.r[3].s64 = ctx.r[1].s64 + 320;
	// 82F4AE58: 4803A8C9  bl 0x82f85720
	ctx.lr = 0x82F4AE5C;
	sub_82F85720(ctx, base);
	// 82F4AE5C: 38A10070  addi r5, r1, 0x70
	ctx.r[5].s64 = ctx.r[1].s64 + 112;
	// 82F4AE60: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82F4AE64: 809D0008  lwz r4, 8(r29)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4AE68: 4BF5C099  bl 0x82ea6f00
	ctx.lr = 0x82F4AE6C;
	sub_82EA6F00(ctx, base);
	// 82F4AE6C: C01F0010  lfs f0, 0x10(r31)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4AE70: C1A10090  lfs f13, 0x90(r1)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(144 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4AE74: ED8D0028  fsubs f12, f13, f0
	ctx.f[12].f64 = (((ctx.f[13].f64 - ctx.f[0].f64) as f32) as f64);
	// 82F4AE78: C17E0010  lfs f11, 0x10(r30)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(16 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4AE7C: C15A0004  lfs f10, 4(r26)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F4AE80: EC0C5828  fsubs f0, f12, f11
	ctx.f[0].f64 = (((ctx.f[12].f64 - ctx.f[11].f64) as f32) as f64);
	// 82F4AE84: D0010090  stfs f0, 0x90(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(144 as u32), tmp.u32 ) };
	// 82F4AE88: FF005000  fcmpu cr6, f0, f10
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[10].f64);
	// 82F4AE8C: 409800A8  bge cr6, 0x82f4af34
	if !ctx.cr[6].lt {
	pc = 0x82F4AF34; continue 'dispatch;
	}
	// 82F4AE90: 39610060  addi r11, r1, 0x60
	ctx.r[11].s64 = ctx.r[1].s64 + 96;
	// 82F4AE94: FC000050  fneg f0, f0
	ctx.f[0].u64 = ctx.f[0].u64 ^ 0x8000_0000_0000_0000u64;
	// 82F4AE98: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82F4AE9C: 3941005C  addi r10, r1, 0x5c
	ctx.r[10].s64 = ctx.r[1].s64 + 92;
	// 82F4AEA0: C1BF0010  lfs f13, 0x10(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(16 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4AEA4: 39210080  addi r9, r1, 0x80
	ctx.r[9].s64 = ctx.r[1].s64 + 128;
	// 82F4AEA8: 39010070  addi r8, r1, 0x70
	ctx.r[8].s64 = ctx.r[1].s64 + 112;
	// 82F4AEAC: FD806850  fneg f12, f13
	ctx.f[12].u64 = ctx.f[13].u64 ^ 0x8000_0000_0000_0000u64;
	// 82F4AEB0: D181005C  stfs f12, 0x5c(r1)
	tmp.f32 = (ctx.f[12].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), tmp.u32 ) };
	// 82F4AEB4: 38E10130  addi r7, r1, 0x130
	ctx.r[7].s64 = ctx.r[1].s64 + 304;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B000(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4B000 size=508
    let mut pc: u32 = 0x82F4B000;
    'dispatch: loop {
        match pc {
            0x82F4B000 => {
    //   block [0x82F4B000..0x82F4B1FC)
	// 82F4B000: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4B004: 4825D155  bl 0x831a8158
	ctx.lr = 0x82F4B008;
	sub_831A8130(ctx, base);
	// 82F4B008: 9421FD80  stwu r1, -0x280(r1)
	ea = ctx.r[1].u32.wrapping_add(-640 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4B00C: 7CBB2B78  mr r27, r5
	ctx.r[27].u64 = ctx.r[5].u64;
	// 82F4B010: 83860000  lwz r28, 0(r6)
	ctx.r[28].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4B014: 7C791B78  mr r25, r3
	ctx.r[25].u64 = ctx.r[3].u64;
	// 82F4B018: 80A60008  lwz r5, 8(r6)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4B01C: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F4B020: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F4B024: 7CF83B78  mr r24, r7
	ctx.r[24].u64 = ctx.r[7].u64;
	// 82F4B028: 83BB0000  lwz r29, 0(r27)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4B02C: 7D1E4378  mr r30, r8
	ctx.r[30].u64 = ctx.r[8].u64;
	// 82F4B030: 809B0008  lwz r4, 8(r27)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4B034: 4BF5602D  bl 0x82ea1060
	ctx.lr = 0x82F4B038;
	sub_82EA1060(ctx, base);
	// 82F4B038: 3BFF000C  addi r31, r31, 0xc
	ctx.r[31].s64 = ctx.r[31].s64 + 12;
	// 82F4B03C: 3B400000  li r26, 0
	ctx.r[26].s64 = 0;
	// 82F4B040: 7FA3EB78  mr r3, r29
	ctx.r[3].u64 = ctx.r[29].u64;
	// 82F4B044: 934100C0  stw r26, 0xc0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(192 as u32), ctx.r[26].u32 ) };
	// 82F4B048: 38C100D0  addi r6, r1, 0xd0
	ctx.r[6].s64 = ctx.r[1].s64 + 208;
	// 82F4B04C: 934100C4  stw r26, 0xc4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(196 as u32), ctx.r[26].u32 ) };
	// 82F4B050: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4B054: 897F000A  lbz r11, 0xa(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(10 as u32) ) } as u64;
	// 82F4B058: 88BF0008  lbz r5, 8(r31)
	ctx.r[5].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4B05C: 895F0009  lbz r10, 9(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(9 as u32) ) } as u64;
	// 82F4B060: 5569E13E  srwi r9, r11, 4
	ctx.r[9].u32 = ctx.r[11].u32.wrapping_shr(4);
	ctx.r[9].u64 = ctx.r[9].u32 as u64;
	// 82F4B064: 5568073E  clrlwi r8, r11, 0x1c
	ctx.r[8].u64 = ctx.r[11].u32 as u64 & 0x0000000Fu64;
	// 82F4B068: 912100B8  stw r9, 0xb8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(184 as u32), ctx.r[9].u32 ) };
	// 82F4B06C: 910100BC  stw r8, 0xbc(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(188 as u32), ctx.r[8].u32 ) };
	// 82F4B070: 90A100B0  stw r5, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[5].u32 ) };
	// 82F4B074: 914100B4  stw r10, 0xb4(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(180 as u32), ctx.r[10].u32 ) };
	// 82F4B078: 80FD0000  lwz r7, 0(r29)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4B07C: 81670034  lwz r11, 0x34(r7)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[7].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F4B080: 7D6903A6  mtctr r11
	ctx.ctr.u64 = ctx.r[11].u64;
	// 82F4B084: 4E800421  bctrl
	ctx.lr = 0x82F4B088;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4B088: 815C0000  lwz r10, 0(r28)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[28].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4B08C: 812100B0  lwz r9, 0xb0(r1)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(176 as u32) ) } as u64;
	// 82F4B090: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F4B094: 80A100B4  lwz r5, 0xb4(r1)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(180 as u32) ) } as u64;
	// 82F4B098: 38C10190  addi r6, r1, 0x190
	ctx.r[6].s64 = ctx.r[1].s64 + 400;
	// 82F4B09C: 552B083C  slwi r11, r9, 1
	ctx.r[11].u32 = ctx.r[9].u32.wrapping_shl(1);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4B0A0: 810A0034  lwz r8, 0x34(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(52 as u32) ) } as u64;
	// 82F4B0A4: 7C8BFA14  add r4, r11, r31
	ctx.r[4].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82F4B0A8: 7D0903A6  mtctr r8
	ctx.ctr.u64 = ctx.r[8].u64;
	// 82F4B0AC: 4E800421  bctrl
	ctx.lr = 0x82F4B0B0;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4B0B0: 38E10060  addi r7, r1, 0x60
	ctx.r[7].s64 = ctx.r[1].s64 + 96;
	// 82F4B0B4: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F4B0B8: 7F85E378  mr r5, r28
	ctx.r[5].u64 = ctx.r[28].u64;
	// 82F4B0BC: 7FA4EB78  mr r4, r29
	ctx.r[4].u64 = ctx.r[29].u64;
	// 82F4B0C0: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82F4B0C4: 4803BB85  bl 0x82f86c48
	ctx.lr = 0x82F4B0C8;
	sub_82F86C48(ctx, base);
	// 82F4B0C8: 80E100C4  lwz r7, 0xc4(r1)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(196 as u32) ) } as u64;
	// 82F4B0CC: 2F070000  cmpwi cr6, r7, 0
	ctx.cr[6].compare_i32(ctx.r[7].s32, 0, &mut ctx.xer);
	// 82F4B0D0: 419A0010  beq cr6, 0x82f4b0e0
	if ctx.cr[6].eq {
	pc = 0x82F4B0E0; continue 'dispatch;
	}
	// 82F4B0D4: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4B0D8: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82F4B0DC: 4803A4FD  bl 0x82f855d8
	ctx.lr = 0x82F4B0E0;
	sub_82F855D8(ctx, base);
	// 82F4B0E0: 7FC5F378  mr r5, r30
	ctx.r[5].u64 = ctx.r[30].u64;
	// 82F4B0E4: 38810060  addi r4, r1, 0x60
	ctx.r[4].s64 = ctx.r[1].s64 + 96;
	// 82F4B0E8: 386100B0  addi r3, r1, 0xb0
	ctx.r[3].s64 = ctx.r[1].s64 + 176;
	// 82F4B0EC: 4803A635  bl 0x82f85720
	ctx.lr = 0x82F4B0F0;
	sub_82F85720(ctx, base);
	// 82F4B0F0: 80FB0008  lwz r7, 8(r27)
	ctx.r[7].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[27].u32.wrapping_add(8 as u32) ) } as u64;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B200(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4B200 size=292
    let mut pc: u32 = 0x82F4B200;
    'dispatch: loop {
        match pc {
            0x82F4B200 => {
    //   block [0x82F4B200..0x82F4B324)
	// 82F4B200: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4B204: 4825CF59  bl 0x831a815c
	ctx.lr = 0x82F4B208;
	sub_831A8130(ctx, base);
	// 82F4B208: 9421FEC0  stwu r1, -0x140(r1)
	ea = ctx.r[1].u32.wrapping_add(-320 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4B20C: 83AD0000  lwz r29, 0(r13)
	ctx.r[29].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4B210: 3B800018  li r28, 0x18
	ctx.r[28].s64 = 24;
	// 82F4B214: 7C7B1B78  mr r27, r3
	ctx.r[27].u64 = ctx.r[3].u64;
	// 82F4B218: 7C9F2378  mr r31, r4
	ctx.r[31].u64 = ctx.r[4].u64;
	// 82F4B21C: 7CBE2B78  mr r30, r5
	ctx.r[30].u64 = ctx.r[5].u64;
	// 82F4B220: 7CDA3378  mr r26, r6
	ctx.r[26].u64 = ctx.r[6].u64;
	// 82F4B224: 7D7CE82E  lwzx r11, r28, r29
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[28].u32.wrapping_add(ctx.r[29].u32)) } as u64;
	// 82F4B228: 7CF93B78  mr r25, r7
	ctx.r[25].u64 = ctx.r[7].u64;
	// 82F4B22C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4B230: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4B234: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F4B238: 40980020  bge cr6, 0x82f4b258
	if !ctx.cr[6].lt {
	pc = 0x82F4B258; continue 'dispatch;
	}
	// 82F4B23C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4B240: 3909FD84  addi r8, r9, -0x27c
	ctx.r[8].s64 = ctx.r[9].s64 + -636;
	// 82F4B244: 910A0000  stw r8, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[8].u32 ) };
	// 82F4B248: 7CEC42E6  mftb r7, 0x10c
	ctx.r[7].u64 = crate::rt::rdtsc_u64();
	// 82F4B24C: 38AA000C  addi r5, r10, 0xc
	ctx.r[5].s64 = ctx.r[10].s64 + 12;
	// 82F4B250: 90EA0004  stw r7, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F4B254: 90AB0004  stw r5, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[5].u32 ) };
	// 82F4B258: 386100C0  addi r3, r1, 0xc0
	ctx.r[3].s64 = ctx.r[1].s64 + 192;
	// 82F4B25C: 80BE0008  lwz r5, 8(r30)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4B260: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4B264: 4BF55DFD  bl 0x82ea1060
	ctx.lr = 0x82F4B268;
	sub_82EA1060(ctx, base);
	// 82F4B268: 815F0000  lwz r10, 0(r31)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4B26C: 813E0000  lwz r9, 0(r30)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[30].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4B270: 396100C0  addi r11, r1, 0xc0
	ctx.r[11].s64 = ctx.r[1].s64 + 192;
	// 82F4B274: 811F0008  lwz r8, 8(r31)
	ctx.r[8].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4B278: C01A0004  lfs f0, 4(r26)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[26].u32.wrapping_add(4 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4B27C: D0010060  stfs f0, 0x60(r1)
	tmp.f32 = (ctx.f[0].f64 as f32);
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), tmp.u32 ) };
	// 82F4B280: 91610050  stw r11, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[11].u32 ) };
	// 82F4B284: 38C10070  addi r6, r1, 0x70
	ctx.r[6].s64 = ctx.r[1].s64 + 112;
	// 82F4B288: 91410058  stw r10, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[10].u32 ) };
	// 82F4B28C: 38A10080  addi r5, r1, 0x80
	ctx.r[5].s64 = ctx.r[1].s64 + 128;
	// 82F4B290: 9121005C  stw r9, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[9].u32 ) };
	// 82F4B294: 389B000C  addi r4, r27, 0xc
	ctx.r[4].s64 = ctx.r[27].s64 + 12;
	// 82F4B298: 91010054  stw r8, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[8].u32 ) };
	// 82F4B29C: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F4B2A0: 4803C839  bl 0x82f87ad8
	ctx.lr = 0x82F4B2A4;
	sub_82F87AD8(ctx, base);
	// 82F4B2A4: 2F030000  cmpwi cr6, r3, 0
	ctx.cr[6].compare_i32(ctx.r[3].s32, 0, &mut ctx.xer);
	// 82F4B2A8: 409A0044  bne cr6, 0x82f4b2ec
	if !ctx.cr[6].eq {
	pc = 0x82F4B2EC; continue 'dispatch;
	}
	// 82F4B2AC: 39610070  addi r11, r1, 0x70
	ctx.r[11].s64 = ctx.r[1].s64 + 112;
	// 82F4B2B0: 81590000  lwz r10, 0(r25)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[25].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4B2B4: 39210090  addi r9, r1, 0x90
	ctx.r[9].s64 = ctx.r[1].s64 + 144;
	// 82F4B2B8: 93E100B0  stw r31, 0xb0(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(176 as u32), ctx.r[31].u32 ) };
	// 82F4B2BC: 39010080  addi r8, r1, 0x80
	ctx.r[8].s64 = ctx.r[1].s64 + 128;
	// 82F4B2C0: 38E100A0  addi r7, r1, 0xa0
	ctx.r[7].s64 = ctx.r[1].s64 + 160;
	// 82F4B2C4: 38810090  addi r4, r1, 0x90
	ctx.r[4].s64 = ctx.r[1].s64 + 144;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B328(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4B328 size=80
    let mut pc: u32 = 0x82F4B328;
    'dispatch: loop {
        match pc {
            0x82F4B328 => {
    //   block [0x82F4B328..0x82F4B378)
	// 82F4B328: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4B32C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4B330: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4B334: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4B338: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4B33C: 7C852378  mr r5, r4
	ctx.r[5].u64 = ctx.r[4].u64;
	// 82F4B340: 387F0030  addi r3, r31, 0x30
	ctx.r[3].s64 = ctx.r[31].s64 + 48;
	// 82F4B344: 809F0008  lwz r4, 8(r31)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4B348: 48036BC9  bl 0x82f81f10
	ctx.lr = 0x82F4B34C;
	sub_82F81F10(ctx, base);
	// 82F4B34C: 817F0000  lwz r11, 0(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4B350: 38800001  li r4, 1
	ctx.r[4].s64 = 1;
	// 82F4B354: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4B358: 814B0000  lwz r10, 0(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4B35C: 7D4903A6  mtctr r10
	ctx.ctr.u64 = ctx.r[10].u64;
	// 82F4B360: 4E800421  bctrl
	ctx.lr = 0x82F4B364;
	crate::rt::call_indirect(ctx.ctr.u32);
	// 82F4B364: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4B368: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4B36C: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4B370: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4B374: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B378(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4B378 size=16
    let mut pc: u32 = 0x82F4B378;
    'dispatch: loop {
        match pc {
            0x82F4B378 => {
    //   block [0x82F4B378..0x82F4B388)
	// 82F4B378: 89230032  lbz r9, 0x32(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(50 as u32) ) } as u64;
	// 82F4B37C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F4B380: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F4B384: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B388(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4B388 size=40
    let mut pc: u32 = 0x82F4B388;
    'dispatch: loop {
        match pc {
            0x82F4B388 => {
    //   block [0x82F4B388..0x82F4B3B0)
	// 82F4B388: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82F4B38C: 39630036  addi r11, r3, 0x36
	ctx.r[11].s64 = ctx.r[3].s64 + 54;
	// 82F4B390: A0EB0000  lhz r7, 0(r11)
	ctx.r[7].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4B394: 7F074040  cmplw cr6, r7, r8
	ctx.cr[6].compare_u32(ctx.r[7].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F4B398: 419A0018  beq cr6, 0x82f4b3b0
	if ctx.cr[6].eq {
		sub_82F4B3B0(ctx, base);
		return;
	}
	// 82F4B39C: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F4B3A0: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F4B3A4: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F4B3A8: 4198FFE8  blt cr6, 0x82f4b390
	if ctx.cr[6].lt {
	pc = 0x82F4B390; continue 'dispatch;
	}
	// 82F4B3AC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B3B0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4B3B0 size=12
    let mut pc: u32 = 0x82F4B3B0;
    'dispatch: loop {
        match pc {
            0x82F4B3B0 => {
    //   block [0x82F4B3B0..0x82F4B3BC)
	// 82F4B3B0: 7D445378  mr r4, r10
	ctx.r[4].u64 = ctx.r[10].u64;
	// 82F4B3B4: 38630030  addi r3, r3, 0x30
	ctx.r[3].s64 = ctx.r[3].s64 + 48;
	// 82F4B3B8: 480379C0  b 0x82f82d78
	sub_82F82D78(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B3C0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4B3C0 size=16
    let mut pc: u32 = 0x82F4B3C0;
    'dispatch: loop {
        match pc {
            0x82F4B3C0 => {
    //   block [0x82F4B3C0..0x82F4B3D0)
	// 82F4B3C0: 89230032  lbz r9, 0x32(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(50 as u32) ) } as u64;
	// 82F4B3C4: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4B3C8: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F4B3CC: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B3D0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4B3D0 size=36
    let mut pc: u32 = 0x82F4B3D0;
    'dispatch: loop {
        match pc {
            0x82F4B3D0 => {
    //   block [0x82F4B3D0..0x82F4B3F4)
	// 82F4B3D0: 39430036  addi r10, r3, 0x36
	ctx.r[10].s64 = ctx.r[3].s64 + 54;
	// 82F4B3D4: A10A0000  lhz r8, 0(r10)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[10].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4B3D8: 2B08FFFF  cmplwi cr6, r8, 0xffff
	ctx.cr[6].compare_u32(ctx.r[8].u32, 65535 as u32, &mut ctx.xer);
	// 82F4B3DC: 419A0018  beq cr6, 0x82f4b3f4
	if ctx.cr[6].eq {
		sub_82F4B3F4(ctx, base);
		return;
	}
	// 82F4B3E0: 396B0001  addi r11, r11, 1
	ctx.r[11].s64 = ctx.r[11].s64 + 1;
	// 82F4B3E4: 394A0008  addi r10, r10, 8
	ctx.r[10].s64 = ctx.r[10].s64 + 8;
	// 82F4B3E8: 7F0B4800  cmpw cr6, r11, r9
	ctx.cr[6].compare_i32(ctx.r[11].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F4B3EC: 4198FFE8  blt cr6, 0x82f4b3d4
	if ctx.cr[6].lt {
	pc = 0x82F4B3D4; continue 'dispatch;
	}
	// 82F4B3F0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B3F4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4B3F4 size=16
    let mut pc: u32 = 0x82F4B3F4;
    'dispatch: loop {
        match pc {
            0x82F4B3F4 => {
    //   block [0x82F4B3F4..0x82F4B404)
	// 82F4B3F4: 556B1838  slwi r11, r11, 3
	ctx.r[11].u32 = ctx.r[11].u32.wrapping_shl(3);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4B3F8: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82F4B3FC: B08B0036  sth r4, 0x36(r11)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[11].u32.wrapping_add(54 as u32), ctx.r[4].u16 ) };
	// 82F4B400: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B408(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4B408 size=20
    let mut pc: u32 = 0x82F4B408;
    'dispatch: loop {
        match pc {
            0x82F4B408 => {
    //   block [0x82F4B408..0x82F4B41C)
	// 82F4B408: 89230032  lbz r9, 0x32(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(50 as u32) ) } as u64;
	// 82F4B40C: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F4B410: 7CEA3B78  mr r10, r7
	ctx.r[10].u64 = ctx.r[7].u64;
	// 82F4B414: 2F090000  cmpwi cr6, r9, 0
	ctx.cr[6].compare_i32(ctx.r[9].s32, 0, &mut ctx.xer);
	// 82F4B418: 4C990020  blelr cr6
	if !ctx.cr[6].gt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B41C(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4B41C size=40
    let mut pc: u32 = 0x82F4B41C;
    'dispatch: loop {
        match pc {
            0x82F4B41C => {
    //   block [0x82F4B41C..0x82F4B444)
	// 82F4B41C: 5488043E  clrlwi r8, r4, 0x10
	ctx.r[8].u64 = ctx.r[4].u32 as u64 & 0x0000FFFFu64;
	// 82F4B420: 39630034  addi r11, r3, 0x34
	ctx.r[11].s64 = ctx.r[3].s64 + 52;
	// 82F4B424: A0CB0002  lhz r6, 2(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(2 as u32) ) } as u64;
	// 82F4B428: 7F064040  cmplw cr6, r6, r8
	ctx.cr[6].compare_u32(ctx.r[6].u32, ctx.r[8].u32, &mut ctx.xer);
	// 82F4B42C: 419A0018  beq cr6, 0x82f4b444
	if ctx.cr[6].eq {
		sub_82F4B444(ctx, base);
		return;
	}
	// 82F4B430: 394A0001  addi r10, r10, 1
	ctx.r[10].s64 = ctx.r[10].s64 + 1;
	// 82F4B434: 396B0008  addi r11, r11, 8
	ctx.r[11].s64 = ctx.r[11].s64 + 8;
	// 82F4B438: 7F0A4800  cmpw cr6, r10, r9
	ctx.cr[6].compare_i32(ctx.r[10].s32, ctx.r[9].s32, &mut ctx.xer);
	// 82F4B43C: 4198FFE8  blt cr6, 0x82f4b424
	if ctx.cr[6].lt {
	pc = 0x82F4B424; continue 'dispatch;
	}
	// 82F4B440: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B444(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4B444 size=12
    let mut pc: u32 = 0x82F4B444;
    'dispatch: loop {
        match pc {
            0x82F4B444 => {
    //   block [0x82F4B444..0x82F4B450)
	// 82F4B444: 98EB0000  stb r7, 0(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(0 as u32), ctx.r[7].u8 ) };
	// 82F4B448: 98EB0001  stb r7, 1(r11)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[11].u32.wrapping_add(1 as u32), ctx.r[7].u8 ) };
	// 82F4B44C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B450(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4B450 size=68
    let mut pc: u32 = 0x82F4B450;
    'dispatch: loop {
        match pc {
            0x82F4B450 => {
    //   block [0x82F4B450..0x82F4B494)
	// 82F4B450: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4B454: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4B458: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4B45C: 9421FFA0  stwu r1, -0x60(r1)
	ea = ctx.r[1].u32.wrapping_add(-96 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4B460: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4B464: 4BFFF345  bl 0x82f4a7a8
	ctx.lr = 0x82F4B468;
	sub_82F4A7A8(ctx, base);
	// 82F4B468: 3D608213  lis r11, -0x7ded
	ctx.r[11].s64 = -2112684032;
	// 82F4B46C: 39400000  li r10, 0
	ctx.r[10].s64 = 0;
	// 82F4B470: 392B00A8  addi r9, r11, 0xa8
	ctx.r[9].s64 = ctx.r[11].s64 + 168;
	// 82F4B474: 7FE3FB78  mr r3, r31
	ctx.r[3].u64 = ctx.r[31].u64;
	// 82F4B478: 913F0000  stw r9, 0(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(0 as u32), ctx.r[9].u32 ) };
	// 82F4B47C: 915F0030  stw r10, 0x30(r31)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[31].u32.wrapping_add(48 as u32), ctx.r[10].u32 ) };
	// 82F4B480: 38210060  addi r1, r1, 0x60
	ctx.r[1].s64 = ctx.r[1].s64 + 96;
	// 82F4B484: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4B488: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4B48C: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4B490: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B498(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4B498 size=156
    let mut pc: u32 = 0x82F4B498;
    'dispatch: loop {
        match pc {
            0x82F4B498 => {
    //   block [0x82F4B498..0x82F4B534)
	// 82F4B498: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4B49C: 4825CCCD  bl 0x831a8168
	ctx.lr = 0x82F4B4A0;
	sub_831A8130(ctx, base);
	// 82F4B4A0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4B4A4: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4B4A8: 39400014  li r10, 0x14
	ctx.r[10].s64 = 20;
	// 82F4B4AC: 7CDF3378  mr r31, r6
	ctx.r[31].u64 = ctx.r[6].u64;
	// 82F4B4B0: 7C7E1B78  mr r30, r3
	ctx.r[30].u64 = ctx.r[3].u64;
	// 82F4B4B4: 7C9D2378  mr r29, r4
	ctx.r[29].u64 = ctx.r[4].u64;
	// 82F4B4B8: 2B1F0000  cmplwi cr6, r31, 0
	ctx.cr[6].compare_u32(ctx.r[31].u32, 0 as u32, &mut ctx.xer);
	// 82F4B4BC: 7C6A582E  lwzx r3, r10, r11
	ctx.r[3].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4B4C0: 38A0001F  li r5, 0x1f
	ctx.r[5].s64 = 31;
	// 82F4B4C4: 419A0048  beq cr6, 0x82f4b50c
	if ctx.cr[6].eq {
	pc = 0x82F4B50C; continue 'dispatch;
	}
	// 82F4B4C8: 38800080  li r4, 0x80
	ctx.r[4].s64 = 128;
	// 82F4B4CC: 4BF55265  bl 0x82ea0730
	ctx.lr = 0x82F4B4D0;
	sub_82EA0730(ctx, base);
	// 82F4B4D0: 7C7C1B78  mr r28, r3
	ctx.r[28].u64 = ctx.r[3].u64;
	// 82F4B4D4: 39200080  li r9, 0x80
	ctx.r[9].s64 = 128;
	// 82F4B4D8: 7FE6FB78  mr r6, r31
	ctx.r[6].u64 = ctx.r[31].u64;
	// 82F4B4DC: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F4B4E0: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F4B4E4: B13C0004  sth r9, 4(r28)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[28].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F4B4E8: 4BFFF2C1  bl 0x82f4a7a8
	ctx.lr = 0x82F4B4EC;
	sub_82F4A7A8(ctx, base);
	// 82F4B4EC: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F4B4F0: 38E00000  li r7, 0
	ctx.r[7].s64 = 0;
	// 82F4B4F4: 38C800A8  addi r6, r8, 0xa8
	ctx.r[6].s64 = ctx.r[8].s64 + 168;
	// 82F4B4F8: 7F83E378  mr r3, r28
	ctx.r[3].u64 = ctx.r[28].u64;
	// 82F4B4FC: 90DC0000  stw r6, 0(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(0 as u32), ctx.r[6].u32 ) };
	// 82F4B500: 90FC0030  stw r7, 0x30(r28)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[28].u32.wrapping_add(48 as u32), ctx.r[7].u32 ) };
	// 82F4B504: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F4B508: 4825CCB0  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
	// 82F4B50C: 38800030  li r4, 0x30
	ctx.r[4].s64 = 48;
	// 82F4B510: 4BF55221  bl 0x82ea0730
	ctx.lr = 0x82F4B514;
	sub_82EA0730(ctx, base);
	// 82F4B514: 39200030  li r9, 0x30
	ctx.r[9].s64 = 48;
	// 82F4B518: 38C00000  li r6, 0
	ctx.r[6].s64 = 0;
	// 82F4B51C: B1230004  sth r9, 4(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[9].u16 ) };
	// 82F4B520: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F4B524: 7FC4F378  mr r4, r30
	ctx.r[4].u64 = ctx.r[30].u64;
	// 82F4B528: 4BFFF281  bl 0x82f4a7a8
	ctx.lr = 0x82F4B52C;
	sub_82F4A7A8(ctx, base);
	// 82F4B52C: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F4B530: 4825CC88  b 0x831a81b8
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B538(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4B538 size=64
    let mut pc: u32 = 0x82F4B538;
    'dispatch: loop {
        match pc {
            0x82F4B538 => {
    //   block [0x82F4B538..0x82F4B578)
	// 82F4B538: 3D6082F5  lis r11, -0x7d0b
	ctx.r[11].s64 = -2097872896;
	// 82F4B53C: 3D4082F5  lis r10, -0x7d0b
	ctx.r[10].s64 = -2097872896;
	// 82F4B540: 3D2082F5  lis r9, -0x7d0b
	ctx.r[9].s64 = -2097872896;
	// 82F4B544: 3D0082F5  lis r8, -0x7d0b
	ctx.r[8].s64 = -2097872896;
	// 82F4B548: 38EBB498  addi r7, r11, -0x4b68
	ctx.r[7].s64 = ctx.r[11].s64 + -19304;
	// 82F4B54C: 38CAAB70  addi r6, r10, -0x5490
	ctx.r[6].s64 = ctx.r[10].s64 + -21648;
	// 82F4B550: 38A9AD18  addi r5, r9, -0x52e8
	ctx.r[5].s64 = ctx.r[9].s64 + -21224;
	// 82F4B554: 90E30000  stw r7, 0(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F4B558: 3888BB88  addi r4, r8, -0x4478
	ctx.r[4].s64 = ctx.r[8].s64 + -17528;
	// 82F4B55C: 90C30004  stw r6, 4(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[6].u32 ) };
	// 82F4B560: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4B564: 90A30008  stw r5, 8(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[5].u32 ) };
	// 82F4B568: 9083000C  stw r4, 0xc(r3)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[3].u32.wrapping_add(12 as u32), ctx.r[4].u32 ) };
	// 82F4B56C: 99630010  stb r11, 0x10(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(16 as u32), ctx.r[11].u8 ) };
	// 82F4B570: 99630011  stb r11, 0x11(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(17 as u32), ctx.r[11].u8 ) };
	// 82F4B574: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B578(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4B578 size=104
    let mut pc: u32 = 0x82F4B578;
    'dispatch: loop {
        match pc {
            0x82F4B578 => {
    //   block [0x82F4B578..0x82F4B5E0)
	// 82F4B578: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4B57C: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4B580: 9421FF90  stwu r1, -0x70(r1)
	ea = ctx.r[1].u32.wrapping_add(-112 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4B584: 3D4082F5  lis r10, -0x7d0b
	ctx.r[10].s64 = -2097872896;
	// 82F4B588: 3D2082F5  lis r9, -0x7d0b
	ctx.r[9].s64 = -2097872896;
	// 82F4B58C: 3D0082F5  lis r8, -0x7d0b
	ctx.r[8].s64 = -2097872896;
	// 82F4B590: 3CE082F5  lis r7, -0x7d0b
	ctx.r[7].s64 = -2097872896;
	// 82F4B594: 38CAB498  addi r6, r10, -0x4b68
	ctx.r[6].s64 = ctx.r[10].s64 + -19304;
	// 82F4B598: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4B59C: 38A9AB70  addi r5, r9, -0x5490
	ctx.r[5].s64 = ctx.r[9].s64 + -21648;
	// 82F4B5A0: 90C10050  stw r6, 0x50(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(80 as u32), ctx.r[6].u32 ) };
	// 82F4B5A4: 3888AD18  addi r4, r8, -0x52e8
	ctx.r[4].s64 = ctx.r[8].s64 + -21224;
	// 82F4B5A8: 99610060  stb r11, 0x60(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(96 as u32), ctx.r[11].u8 ) };
	// 82F4B5AC: 3947BB88  addi r10, r7, -0x4478
	ctx.r[10].s64 = ctx.r[7].s64 + -17528;
	// 82F4B5B0: 90A10054  stw r5, 0x54(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(84 as u32), ctx.r[5].u32 ) };
	// 82F4B5B4: 90810058  stw r4, 0x58(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(88 as u32), ctx.r[4].u32 ) };
	// 82F4B5B8: 38C00001  li r6, 1
	ctx.r[6].s64 = 1;
	// 82F4B5BC: 9141005C  stw r10, 0x5c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(92 as u32), ctx.r[10].u32 ) };
	// 82F4B5C0: 38A00001  li r5, 1
	ctx.r[5].s64 = 1;
	// 82F4B5C4: 99610061  stb r11, 0x61(r1)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[1].u32.wrapping_add(97 as u32), ctx.r[11].u8 ) };
	// 82F4B5C8: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F4B5CC: 4BFD9E75  bl 0x82f25440
	ctx.lr = 0x82F4B5D0;
	sub_82F25440(ctx, base);
	// 82F4B5D0: 38210070  addi r1, r1, 0x70
	ctx.r[1].s64 = ctx.r[1].s64 + 112;
	// 82F4B5D4: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4B5D8: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4B5DC: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B5E0(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4B5E0 size=120
    let mut pc: u32 = 0x82F4B5E0;
    'dispatch: loop {
        match pc {
            0x82F4B5E0 => {
    //   block [0x82F4B5E0..0x82F4B658)
	// 82F4B5E0: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4B5E4: 9181FFF8  stw r12, -8(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(-8 as u32), ctx.r[12].u32 ) };
	// 82F4B5E8: FBC1FFE8  std r30, -0x18(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-24 as u32), ctx.r[30].u64 ) };
	// 82F4B5EC: FBE1FFF0  std r31, -0x10(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-16 as u32), ctx.r[31].u64 ) };
	// 82F4B5F0: 9421FF00  stwu r1, -0x100(r1)
	ea = ctx.r[1].u32.wrapping_add(-256 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4B5F4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4B5F8: 90810070  stw r4, 0x70(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(112 as u32), ctx.r[4].u32 ) };
	// 82F4B5FC: 90A10074  stw r5, 0x74(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(116 as u32), ctx.r[5].u32 ) };
	// 82F4B600: 38610080  addi r3, r1, 0x80
	ctx.r[3].s64 = ctx.r[1].s64 + 128;
	// 82F4B604: 90C10078  stw r6, 0x78(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(120 as u32), ctx.r[6].u32 ) };
	// 82F4B608: 7CFE3B78  mr r30, r7
	ctx.r[30].u64 = ctx.r[7].u64;
	// 82F4B60C: 80A50008  lwz r5, 8(r5)
	ctx.r[5].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4B610: 80840008  lwz r4, 8(r4)
	ctx.r[4].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4B614: 817F0008  lwz r11, 8(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4B618: 9161007C  stw r11, 0x7c(r1)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[1].u32.wrapping_add(124 as u32), ctx.r[11].u32 ) };
	// 82F4B61C: 4BF55A45  bl 0x82ea1060
	ctx.lr = 0x82F4B620;
	sub_82EA1060(ctx, base);
	// 82F4B620: 7FC9F378  mr r9, r30
	ctx.r[9].u64 = ctx.r[30].u64;
	// 82F4B624: 391F0020  addi r8, r31, 0x20
	ctx.r[8].s64 = ctx.r[31].s64 + 32;
	// 82F4B628: 38FF0030  addi r7, r31, 0x30
	ctx.r[7].s64 = ctx.r[31].s64 + 48;
	// 82F4B62C: 38DF000C  addi r6, r31, 0xc
	ctx.r[6].s64 = ctx.r[31].s64 + 12;
	// 82F4B630: 38A10058  addi r5, r1, 0x58
	ctx.r[5].s64 = ctx.r[1].s64 + 88;
	// 82F4B634: 38810050  addi r4, r1, 0x50
	ctx.r[4].s64 = ctx.r[1].s64 + 80;
	// 82F4B638: 38610070  addi r3, r1, 0x70
	ctx.r[3].s64 = ctx.r[1].s64 + 112;
	// 82F4B63C: 480448E5  bl 0x82f8ff20
	ctx.lr = 0x82F4B640;
	sub_82F8FF20(ctx, base);
	// 82F4B640: 38210100  addi r1, r1, 0x100
	ctx.r[1].s64 = ctx.r[1].s64 + 256;
	// 82F4B644: 8181FFF8  lwz r12, -8(r1)
	ctx.r[12].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[1].u32.wrapping_add(-8 as u32) ) } as u64;
	// 82F4B648: 7D8803A6  mtlr r12
	ctx.lr = ctx.r[12].u64;
	// 82F4B64C: EBC1FFE8  ld r30, -0x18(r1)
	ctx.r[30].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-24 as u32) ) };
	// 82F4B650: EBE1FFF0  ld r31, -0x10(r1)
	ctx.r[31].u64 = unsafe { crate::rt::load_u64( base as *const u8, ctx.r[1].u32.wrapping_add(-16 as u32) ) };
	// 82F4B654: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B658(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    let mut ea: u32 = 0;
    // ---- function 0x82F4B658 size=532
    let mut pc: u32 = 0x82F4B658;
    'dispatch: loop {
        match pc {
            0x82F4B658 => {
    //   block [0x82F4B658..0x82F4B86C)
	// 82F4B658: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4B65C: 4825CB05  bl 0x831a8160
	ctx.lr = 0x82F4B660;
	sub_831A8130(ctx, base);
	// 82F4B660: 9421FEE0  stwu r1, -0x120(r1)
	ea = ctx.r[1].u32.wrapping_add(-288 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4B664: 816D0000  lwz r11, 0(r13)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[13].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4B668: 39400018  li r10, 0x18
	ctx.r[10].s64 = 24;
	// 82F4B66C: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4B670: 7FCA5A14  add r30, r10, r11
	ctx.r[30].u64 = ctx.r[10].u64 + ctx.r[11].u64;
	// 82F4B674: 7CFC3B78  mr r28, r7
	ctx.r[28].u64 = ctx.r[7].u64;
	// 82F4B678: 7D6A582E  lwzx r11, r10, r11
	ctx.r[11].u64 = unsafe { crate::rt::load_u32(base as *const u8, ctx.r[10].u32.wrapping_add(ctx.r[11].u32)) } as u64;
	// 82F4B67C: 812B000C  lwz r9, 0xc(r11)
	ctx.r[9].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(12 as u32) ) } as u64;
	// 82F4B680: 814B0004  lwz r10, 4(r11)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(4 as u32) ) } as u64;
	// 82F4B684: 7F0A4840  cmplw cr6, r10, r9
	ctx.cr[6].compare_u32(ctx.r[10].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F4B688: 4098002C  bge cr6, 0x82f4b6b4
	if !ctx.cr[6].lt {
	pc = 0x82F4B6B4; continue 'dispatch;
	}
	// 82F4B68C: 3D208213  lis r9, -0x7ded
	ctx.r[9].s64 = -2112684032;
	// 82F4B690: 3D008213  lis r8, -0x7ded
	ctx.r[8].s64 = -2112684032;
	// 82F4B694: 38E900E0  addi r7, r9, 0xe0
	ctx.r[7].s64 = ctx.r[9].s64 + 224;
	// 82F4B698: 3868F8A4  addi r3, r8, -0x75c
	ctx.r[3].s64 = ctx.r[8].s64 + -1884;
	// 82F4B69C: 90EA0000  stw r7, 0(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(0 as u32), ctx.r[7].u32 ) };
	// 82F4B6A0: 906A000C  stw r3, 0xc(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(12 as u32), ctx.r[3].u32 ) };
	// 82F4B6A4: 7D2C42E6  mftb r9, 0x10c
	ctx.r[9].u64 = crate::rt::rdtsc_u64();
	// 82F4B6A8: 38EA0010  addi r7, r10, 0x10
	ctx.r[7].s64 = ctx.r[10].s64 + 16;
	// 82F4B6AC: 912A0004  stw r9, 4(r10)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[10].u32.wrapping_add(4 as u32), ctx.r[9].u32 ) };
	// 82F4B6B0: 90EB0004  stw r7, 4(r11)
	unsafe { crate::rt::store_u32( base as *mut u8, ctx.r[11].u32.wrapping_add(4 as u32), ctx.r[7].u32 ) };
	// 82F4B6B4: C0060050  lfs f0, 0x50(r6)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(80 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4B6B8: 3BBF0018  addi r29, r31, 0x18
	ctx.r[29].s64 = ctx.r[31].s64 + 24;
	// 82F4B6BC: C1BF0018  lfs f13, 0x18(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(24 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4B6C0: FF0D0000  fcmpu cr6, f13, f0
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[0].f64);
	// 82F4B6C4: 419A00F4  beq cr6, 0x82f4b7b8
	if ctx.cr[6].eq {
	pc = 0x82F4B7B8; continue 'dispatch;
	}
	// 82F4B6C8: 81640008  lwz r11, 8(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4B6CC: 38600010  li r3, 0x10
	ctx.r[3].s64 = 16;
	// 82F4B6D0: 81450008  lwz r10, 8(r5)
	ctx.r[10].u64 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[5].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4B6D4: 38E10050  addi r7, r1, 0x50
	ctx.r[7].s64 = ctx.r[1].s64 + 80;
	// 82F4B6D8: 392B0040  addi r9, r11, 0x40
	ctx.r[9].s64 = ctx.r[11].s64 + 64;
	// 82F4B6DC: C1BF002C  lfs f13, 0x2c(r31)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[31].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4B6E0: C1860004  lfs f12, 4(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(4 as u32) ) };
	ctx.f[12].f64 = (tmp.f32 as f64);
	// 82F4B6E4: 3B610050  addi r27, r1, 0x50
	ctx.r[27].s64 = ctx.r[1].s64 + 80;
	// 82F4B6E8: C0060058  lfs f0, 0x58(r6)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[6].u32.wrapping_add(88 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4B6EC: FF0D6000  fcmpu cr6, f13, f12
	ctx.cr[6].compare_f64(ctx.f[13].f64, ctx.f[12].f64);
	// 82F4B6F0: C14B005C  lfs f10, 0x5c(r11)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[11].u32.wrapping_add(92 as u32) ) };
	ctx.f[10].f64 = (tmp.f32 as f64);
	// 82F4B6F4: 390A0040  addi r8, r10, 0x40
	ctx.r[8].s64 = ctx.r[10].s64 + 64;
	// 82F4B6F8: C16A005C  lfs f11, 0x5c(r10)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[10].u32.wrapping_add(92 as u32) ) };
	ctx.f[11].f64 = (tmp.f32 as f64);
	// 82F4B6FC: 3B410060  addi r26, r1, 0x60
	ctx.r[26].s64 = ctx.r[1].s64 + 96;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4B870(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4B870 size=792
    let mut pc: u32 = 0x82F4B870;
    'dispatch: loop {
        match pc {
            0x82F4B870 => {
    //   block [0x82F4B870..0x82F4BB88)
	// 82F4B870: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4B874: 4825C8DD  bl 0x831a8150
	ctx.lr = 0x82F4B878;
	sub_831A8130(ctx, base);
	// 82F4B878: DBC1FF98  stfd f30, -0x68(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[30].u64 ) };
	// 82F4B87C: DBE1FFA0  stfd f31, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 82F4B880: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4BB88(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4BB88 size=832
    let mut pc: u32 = 0x82F4BB88;
    'dispatch: loop {
        match pc {
            0x82F4BB88 => {
    //   block [0x82F4BB88..0x82F4BEC8)
	// 82F4BB88: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4BB8C: 4825C5C5  bl 0x831a8150
	ctx.lr = 0x82F4BB90;
	sub_831A8130(ctx, base);
	// 82F4BB90: DBA1FF90  stfd f29, -0x70(r1)
	ctx.fpscr.disable_flush_mode_unconditional();
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-112 as u32), ctx.f[29].u64 ) };
	// 82F4BB94: DBC1FF98  stfd f30, -0x68(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-104 as u32), ctx.f[30].u64 ) };
	// 82F4BB98: DBE1FFA0  stfd f31, -0x60(r1)
	unsafe { crate::rt::store_u64( base as *mut u8, ctx.r[1].u32.wrapping_add(-96 as u32), ctx.f[31].u64 ) };
	// 82F4BB9C: 3980FF80  li r12, -0x80
	ctx.r[12].s64 = -128;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4BEC8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4BEC8 size=12
    let mut pc: u32 = 0x82F4BEC8;
    'dispatch: loop {
        match pc {
            0x82F4BEC8 => {
    //   block [0x82F4BEC8..0x82F4BED4)
	// 82F4BEC8: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F4BECC: 99630004  stb r11, 4(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(4 as u32), ctx.r[11].u8 ) };
	// 82F4BED0: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4BED8(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut tmp: PPCRegister = Default::default();
    // ---- function 0x82F4BED8 size=28
    let mut pc: u32 = 0x82F4BED8;
    'dispatch: loop {
        match pc {
            0x82F4BED8 => {
    //   block [0x82F4BED8..0x82F4BEF4)
	// 82F4BED8: 89630008  lbz r11, 8(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(8 as u32) ) } as u64;
	// 82F4BEDC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4BEE0: 419A0014  beq cr6, 0x82f4bef4
	if ctx.cr[6].eq {
		sub_82F4BEF4(ctx, base);
		return;
	}
	// 82F4BEE4: C004001C  lfs f0, 0x1c(r4)
	ctx.fpscr.disable_flush_mode_unconditional();
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[4].u32.wrapping_add(28 as u32) ) };
	ctx.f[0].f64 = (tmp.f32 as f64);
	// 82F4BEE8: C1A3002C  lfs f13, 0x2c(r3)
	tmp.u32 = unsafe { crate::rt::load_u32( base as *const u8, ctx.r[3].u32.wrapping_add(44 as u32) ) };
	ctx.f[13].f64 = (tmp.f32 as f64);
	// 82F4BEEC: FF006800  fcmpu cr6, f0, f13
	ctx.cr[6].compare_f64(ctx.f[0].f64, ctx.f[13].f64);
	// 82F4BEF0: 4C980020  bgelr cr6
	if !ctx.cr[6].lt { return; }
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4BEF4(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4BEF4 size=44
    let mut pc: u32 = 0x82F4BEF4;
    'dispatch: loop {
        match pc {
            0x82F4BEF4 => {
    //   block [0x82F4BEF4..0x82F4BF20)
	// 82F4BEF4: 39630010  addi r11, r3, 0x10
	ctx.r[11].s64 = ctx.r[3].s64 + 16;
	// 82F4BEF8: 39400010  li r10, 0x10
	ctx.r[10].s64 = 16;
	// 82F4BEFC: 39200001  li r9, 1
	ctx.r[9].s64 = 1;
	// 82F4BF00: 99230008  stb r9, 8(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(8 as u32), ctx.r[9].u8 ) };
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4BF20(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4BF20 size=64
    let mut pc: u32 = 0x82F4BF20;
    'dispatch: loop {
        match pc {
            0x82F4BF20 => {
    //   block [0x82F4BF20..0x82F4BF60)
	// 82F4BF20: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4BF24: 548A103A  slwi r10, r4, 2
	ctx.r[10].u32 = ctx.r[4].u32.wrapping_shl(2);
	ctx.r[10].u64 = ctx.r[10].u32 as u64;
	// 82F4BF28: 99630022  stb r11, 0x22(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(34 as u32), ctx.r[11].u8 ) };
	// 82F4BF2C: 89230021  lbz r9, 0x21(r3)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(33 as u32) ) } as u64;
	// 82F4BF30: 552B103E  rotlwi r11, r9, 2
	ctx.r[11].u64 = ((ctx.r[9].u32).rotate_left(2)) as u64;
	// 82F4BF34: 7D6B1A14  add r11, r11, r3
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[3].u64;
	// 82F4BF38: 7D0A1A14  add r8, r10, r3
	ctx.r[8].u64 = ctx.r[10].u64 + ctx.r[3].u64;
	// 82F4BF3C: 38EBFFFC  addi r7, r11, -4
	ctx.r[7].s64 = ctx.r[11].s64 + -4;
	// 82F4BF40: A0CBFFFC  lhz r6, -4(r11)
	ctx.r[6].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-4 as u32) ) } as u64;
	// 82F4BF44: 7CCA1B2E  sthx r6, r10, r3
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[10].u32.wrapping_add(ctx.r[3].u32), ctx.r[6].u16) };
	// 82F4BF48: A0ABFFFE  lhz r5, -2(r11)
	ctx.r[5].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[11].u32.wrapping_add(-2 as u32) ) } as u64;
	// 82F4BF4C: B0A80002  sth r5, 2(r8)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[8].u32.wrapping_add(2 as u32), ctx.r[5].u16 ) };
	// 82F4BF50: 89630021  lbz r11, 0x21(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(33 as u32) ) } as u64;
	// 82F4BF54: 388B00FF  addi r4, r11, 0xff
	ctx.r[4].s64 = ctx.r[11].s64 + 255;
	// 82F4BF58: 98830021  stb r4, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[4].u8 ) };
	// 82F4BF5C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4BF60(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4BF60 size=52
    let mut pc: u32 = 0x82F4BF60;
    'dispatch: loop {
        match pc {
            0x82F4BF60 => {
    //   block [0x82F4BF60..0x82F4BF94)
	// 82F4BF60: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4BF64: B1630002  sth r11, 2(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(2 as u32), ctx.r[11].u16 ) };
	// 82F4BF68: B1630006  sth r11, 6(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(6 as u32), ctx.r[11].u16 ) };
	// 82F4BF6C: B163000A  sth r11, 0xa(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(10 as u32), ctx.r[11].u16 ) };
	// 82F4BF70: B163000E  sth r11, 0xe(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(14 as u32), ctx.r[11].u16 ) };
	// 82F4BF74: B1630012  sth r11, 0x12(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(18 as u32), ctx.r[11].u16 ) };
	// 82F4BF78: B1630016  sth r11, 0x16(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(22 as u32), ctx.r[11].u16 ) };
	// 82F4BF7C: B163001A  sth r11, 0x1a(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(26 as u32), ctx.r[11].u16 ) };
	// 82F4BF80: B163001E  sth r11, 0x1e(r3)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[3].u32.wrapping_add(30 as u32), ctx.r[11].u16 ) };
	// 82F4BF84: 99630021  stb r11, 0x21(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(33 as u32), ctx.r[11].u8 ) };
	// 82F4BF88: 99630020  stb r11, 0x20(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(32 as u32), ctx.r[11].u8 ) };
	// 82F4BF8C: 99630022  stb r11, 0x22(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(34 as u32), ctx.r[11].u8 ) };
	// 82F4BF90: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4BF98(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    let mut ea: u32 = 0;
    // ---- function 0x82F4BF98 size=128
    let mut pc: u32 = 0x82F4BF98;
    'dispatch: loop {
        match pc {
            0x82F4BF98 => {
    //   block [0x82F4BF98..0x82F4C018)
	// 82F4BF98: 7D8802A6  mflr r12
	ctx.r[12].u64 = ctx.lr;
	// 82F4BF9C: 4825C1D1  bl 0x831a816c
	ctx.lr = 0x82F4BFA0;
	sub_831A8130(ctx, base);
	// 82F4BFA0: 9421FF80  stwu r1, -0x80(r1)
	ea = ctx.r[1].u32.wrapping_add(-128 as u32);
	unsafe { crate::rt::store_u32(base as *mut u8, ea, ctx.r[1].u32) };
	ctx.r[1].u32 = ea;
	// 82F4BFA4: 7C7F1B78  mr r31, r3
	ctx.r[31].u64 = ctx.r[3].u64;
	// 82F4BFA8: 7CDD3378  mr r29, r6
	ctx.r[29].u64 = ctx.r[6].u64;
	// 82F4BFAC: 8BDF0021  lbz r30, 0x21(r31)
	ctx.r[30].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(33 as u32) ) } as u64;
	// 82F4BFB0: 2B1E0008  cmplwi cr6, r30, 8
	ctx.cr[6].compare_u32(ctx.r[30].u32, 8 as u32, &mut ctx.xer);
	// 82F4BFB4: 41990058  bgt cr6, 0x82f4c00c
	if ctx.cr[6].gt {
	pc = 0x82F4C00C; continue 'dispatch;
	}
	// 82F4BFB8: 7FA5EB78  mr r5, r29
	ctx.r[5].u64 = ctx.r[29].u64;
	// 82F4BFBC: 7FE4FB78  mr r4, r31
	ctx.r[4].u64 = ctx.r[31].u64;
	// 82F4BFC0: 38610050  addi r3, r1, 0x50
	ctx.r[3].s64 = ctx.r[1].s64 + 80;
	// 82F4BFC4: 48000055  bl 0x82f4c018
	ctx.lr = 0x82F4BFC8;
	sub_82F4C018(ctx, base);
	// 82F4BFC8: 89630000  lbz r11, 0(r3)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[3].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4BFCC: 2B0B0000  cmplwi cr6, r11, 0
	ctx.cr[6].compare_u32(ctx.r[11].u32, 0 as u32, &mut ctx.xer);
	// 82F4BFD0: 409A003C  bne cr6, 0x82f4c00c
	if !ctx.cr[6].eq {
	pc = 0x82F4C00C; continue 'dispatch;
	}
	// 82F4BFD4: 2F1E0008  cmpwi cr6, r30, 8
	ctx.cr[6].compare_i32(ctx.r[30].s32, 8, &mut ctx.xer);
	// 82F4BFD8: 40980034  bge cr6, 0x82f4c00c
	if !ctx.cr[6].lt {
	pc = 0x82F4C00C; continue 'dispatch;
	}
	// 82F4BFDC: 57CB103A  slwi r11, r30, 2
	ctx.r[11].u32 = ctx.r[30].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4BFE0: A15D0000  lhz r10, 0(r29)
	ctx.r[10].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4BFE4: 7FC3F378  mr r3, r30
	ctx.r[3].u64 = ctx.r[30].u64;
	// 82F4BFE8: 7D2BFA14  add r9, r11, r31
	ctx.r[9].u64 = ctx.r[11].u64 + ctx.r[31].u64;
	// 82F4BFEC: 7D4BFB2E  sthx r10, r11, r31
	unsafe { crate::rt::store_u16(base as *mut u8, ctx.r[11].u32.wrapping_add(ctx.r[31].u32), ctx.r[10].u16) };
	// 82F4BFF0: A11D0002  lhz r8, 2(r29)
	ctx.r[8].u64 = unsafe { crate::rt::load_u16( base as *const u8, ctx.r[29].u32.wrapping_add(2 as u32) ) } as u64;
	// 82F4BFF4: B1090002  sth r8, 2(r9)
	unsafe { crate::rt::store_u16( base as *mut u8, ctx.r[9].u32.wrapping_add(2 as u32), ctx.r[8].u16 ) };
	// 82F4BFF8: 897F0021  lbz r11, 0x21(r31)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[31].u32.wrapping_add(33 as u32) ) } as u64;
	// 82F4BFFC: 38EB0001  addi r7, r11, 1
	ctx.r[7].s64 = ctx.r[11].s64 + 1;
	// 82F4C000: 98FF0021  stb r7, 0x21(r31)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[31].u32.wrapping_add(33 as u32), ctx.r[7].u8 ) };
	// 82F4C004: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F4C008: 4825C1B4  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
	// 82F4C00C: 3860FFFF  li r3, -1
	ctx.r[3].s64 = -1;
	// 82F4C010: 38210080  addi r1, r1, 0x80
	ctx.r[1].s64 = ctx.r[1].s64 + 128;
	// 82F4C014: 4825C1A8  b 0x831a81bc
	sub_831A8180(ctx, base);
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C018(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C018 size=76
    let mut pc: u32 = 0x82F4C018;
    'dispatch: loop {
        match pc {
            0x82F4C018 => {
    //   block [0x82F4C018..0x82F4C064)
	// 82F4C018: 89640021  lbz r11, 0x21(r4)
	ctx.r[11].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[4].u32.wrapping_add(33 as u32) ) } as u64;
	// 82F4C01C: 354BFFFF  addic. r10, r11, -1
	ctx.xer.ca = (ctx.r[11].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[11].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F4C020: 41800038  blt 0x82f4c058
	if ctx.cr[0].lt {
	pc = 0x82F4C058; continue 'dispatch;
	}
	// 82F4C024: 554B103A  slwi r11, r10, 2
	ctx.r[11].u32 = ctx.r[10].u32.wrapping_shl(2);
	ctx.r[11].u64 = ctx.r[11].u32 as u64;
	// 82F4C028: 89250000  lbz r9, 0(r5)
	ctx.r[9].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C02C: 7D6B2214  add r11, r11, r4
	ctx.r[11].u64 = ctx.r[11].u64 + ctx.r[4].u64;
	// 82F4C030: 890B0000  lbz r8, 0(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(0 as u32) ) } as u64;
	// 82F4C034: 7F084840  cmplw cr6, r8, r9
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[9].u32, &mut ctx.xer);
	// 82F4C038: 409A0014  bne cr6, 0x82f4c04c
	if !ctx.cr[6].eq {
	pc = 0x82F4C04C; continue 'dispatch;
	}
	// 82F4C03C: 890B0001  lbz r8, 1(r11)
	ctx.r[8].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[11].u32.wrapping_add(1 as u32) ) } as u64;
	// 82F4C040: 88E50001  lbz r7, 1(r5)
	ctx.r[7].u64 = unsafe { crate::rt::load_u8( base as *const u8, ctx.r[5].u32.wrapping_add(1 as u32) ) } as u64;
	// 82F4C044: 7F083840  cmplw cr6, r8, r7
	ctx.cr[6].compare_u32(ctx.r[8].u32, ctx.r[7].u32, &mut ctx.xer);
	// 82F4C048: 419A001C  beq cr6, 0x82f4c064
	if ctx.cr[6].eq {
		sub_82F4C064(ctx, base);
		return;
	}
	// 82F4C04C: 354AFFFF  addic. r10, r10, -1
	ctx.xer.ca = (ctx.r[10].u32 > (!(-1 as u32)));
	ctx.r[10].s64 = ctx.r[10].s64 + -1;
	ctx.cr[0].compare_i32(ctx.r[10].s32, 0, &mut ctx.xer);
	// 82F4C050: 396BFFFC  addi r11, r11, -4
	ctx.r[11].s64 = ctx.r[11].s64 + -4;
	// 82F4C054: 4080FFDC  bge 0x82f4c030
	if !ctx.cr[0].lt {
	pc = 0x82F4C030; continue 'dispatch;
	}
	// 82F4C058: 39600000  li r11, 0
	ctx.r[11].s64 = 0;
	// 82F4C05C: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82F4C060: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


pub fn sub_82F4C064(ctx: &mut crate::recompiler::ppc_context::PPCContext, base: *mut u8) {
    // ---- function 0x82F4C064 size=12
    let mut pc: u32 = 0x82F4C064;
    'dispatch: loop {
        match pc {
            0x82F4C064 => {
    //   block [0x82F4C064..0x82F4C070)
	// 82F4C064: 39600001  li r11, 1
	ctx.r[11].s64 = 1;
	// 82F4C068: 99630000  stb r11, 0(r3)
	unsafe { crate::rt::store_u8( base as *mut u8, ctx.r[3].u32.wrapping_add(0 as u32), ctx.r[11].u8 ) };
	// 82F4C06C: 4E800020  blr
	return;
            }
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}


